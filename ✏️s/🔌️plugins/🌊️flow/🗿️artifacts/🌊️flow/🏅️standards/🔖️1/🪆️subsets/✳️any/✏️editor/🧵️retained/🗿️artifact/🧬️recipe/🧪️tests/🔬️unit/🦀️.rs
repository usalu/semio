
use super::*;
use store::SnapshotRetirementStep;

pub(in super::super) fn source(label: &str) -> FlowWorkingScene {
    FlowWorkingScene {
        widgets: ["a", "b", "c"].into_iter().map(|id| Widget::InputSlider { id: id.into(), label: if id == "b" { label.into() } else { id.into() }, value: 1.0, min: 0.0, max: 10.0, step: 1.0 }).collect(),
        synapses: [("ab", "a", "b"), ("bc", "b", "c"), ("ac", "a", "c")]
            .into_iter()
            .map(|(id, from, to)| semio_framework_artifact_flow_flow::SynapseSpec { id: id.into(), from: from.into(), from_port: "value".into(), to: to.into(), to_port: "value".into() })
            .collect(),
        layout: [("b".to_owned(), WidgetLayout { x: 1.0, y: 2.0 })].into_iter().collect(),
    }
}

fn close(recipe: &mut Recipe, grant: store::ArtifactStoreOneItemGrant) -> usize {
    recipe.begin_close();
    let mut bytes = 0;
    for _ in 0..500_000 {
        match recipe.close_step(grant).unwrap() {
            SnapshotRetirementStep::Complete => {
                assert!(recipe.terminal_is_empty());
                return bytes;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes);
                bytes += released_bytes;
            }
            SnapshotRetirementStep::Blocked => panic!("positive recipe close grant cannot block"),
        }
    }
    panic!("Flow recipe did not reach terminal emptiness");
}

fn retire(scene: FlowWorkingScene, inverse: Vec<FlowMutation>, grant: store::ArtifactStoreOneItemGrant) {
    let mut retirement = Retirement::default();
    retirement.push(Owner::Scene(scene));
    retirement.push(Owner::Mutations(inverse));
    for _ in 0..500_000 {
        if store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, grant.maximum_bytes).unwrap() == SnapshotRetirementStep::Complete {
            return;
        }
    }
    panic!("Flow recipe output did not retire");
}

fn apply_inverse(post: &mut serde_json::Value, inverse: &serde_json::Value) {
    for mutation in inverse.as_array().unwrap() {
        match mutation["mutation"].as_str().unwrap() {
            "createWidget" => post["widgets"].as_array_mut().unwrap().insert(mutation["index"].as_u64().unwrap() as usize, mutation["widget"].clone()),
            "connectWidgets" => {
                let mut edge = mutation.as_object().unwrap().clone();
                edge.remove("mutation");
                let index = edge.remove("index").unwrap().as_u64().unwrap() as usize;
                post["synapses"].as_array_mut().unwrap().insert(index, serde_json::Value::Object(edge));
            }
            "moveWidgets" => {
                for entry in mutation["entries"].as_array().unwrap() {
                    let id = entry["id"].as_str().unwrap();
                    if entry["layout"].is_null() {
                        post["layout"].as_object_mut().unwrap().remove(id);
                    } else {
                        post["layout"][id] = entry["layout"].clone();
                    }
                }
            }
            "replaceWidget" => {
                let widget = post["widgets"].as_array_mut().unwrap().iter_mut().find(|widget| widget["id"] == mutation["id"]).unwrap();
                *widget = mutation["widget"].clone();
            }
            _ => panic!("unexpected recipe inverse"),
        }
    }
}

#[test]
fn retained_recipes_match_immer_fixture_and_exact_inverse_at_one_and_production_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️fixtures/🧬️artifact-recipes.json")).unwrap();
    let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
    for bytes in fixture["grants"].as_array().unwrap() {
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes.as_u64().unwrap() as usize };
        for row in fixture["cases"].as_array().unwrap() {
            let root = Arc::new(source(&label));
            let baseline = serde_json::Value::from(dsl::ToValue::to_value(&*root));
            let weak = Arc::downgrade(&root);
            let mutation: FlowMutation = dsl::FromValue::from_value(dsl::DslValue::from(row["mutation"].clone())).unwrap();
            let mut recipe = Recipe::new(root, Arc::new(mutation));
            for _ in 0..500_000 {
                if recipe.complete() {
                    break;
                }
                assert!(recipe.advance(grant).unwrap().unwrap() <= grant.maximum_bytes);
            }
            assert!(recipe.complete());
            let (post, inverse) = recipe.take().unwrap();
            let mut json = serde_json::Value::from(dsl::ToValue::to_value(&post));
            let inverse_json = serde_json::Value::from(dsl::ToValue::to_value(&inverse));
            assert_eq!(json["widgets"].as_array().unwrap().iter().map(|widget| widget["id"].clone()).collect::<Vec<_>>(), *row["widgets"].as_array().unwrap());
            assert_eq!(json["synapses"].as_array().unwrap().iter().map(|edge| edge["id"].clone()).collect::<Vec<_>>(), *row["synapses"].as_array().unwrap());
            assert_eq!(inverse_json.as_array().unwrap().iter().map(|mutation| mutation["mutation"].clone()).collect::<Vec<_>>(), *row["inverseKinds"].as_array().unwrap());
            apply_inverse(&mut json, &inverse_json);
            assert_eq!(json, baseline);
            close(&mut recipe, grant);
            assert!(weak.upgrade().is_none());
            retire(post, inverse, grant);
        }
    }
}

#[test]
fn recipe_cancellation_retires_every_partial_frontier_without_losing_original_root() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️fixtures/🧬️artifact-recipes.json")).unwrap();
    let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
    for bytes in fixture["grants"].as_array().unwrap() {
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes.as_u64().unwrap() as usize };
        for after in fixture["cancelAfterSteps"].as_array().unwrap() {
            let root = Arc::new(source(&label));
            let weak = Arc::downgrade(&root);
            let mutation = dsl::FromValue::from_value(dsl::DslValue::from(fixture["cases"][0]["mutation"].clone())).unwrap();
            let mut recipe = Recipe::new(root, Arc::new(mutation));
            for _ in 0..after.as_u64().unwrap() {
                if recipe.complete() {
                    break;
                }
                recipe.advance(grant).unwrap();
            }
            assert!(weak.upgrade().is_some());
            let retired = close(&mut recipe, grant);
            assert!(weak.upgrade().is_none());
            if after.as_u64().unwrap() == 0 {
                assert_eq!(retired, 4849);
            }
        }
    }
}
