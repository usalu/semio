//! 🔎️ Drawing retained mutation laws over lookups: borrowed lookup, rename preparation, owned rename preparation, rename inverse and owned lookup measurements.

use super::*;

#[test]
fn paged_native_drawing_snapshot_borrowed_lookup_measures_frames_and_alias_closure() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{retained_clone::{RetainedCloneSource, RetainedCloneStep}, paged::PagedUtf8};
    use semio_s_artifact_draw_drawing_model::snapshot::lookup::{DrawingLayerLookupCursor, DrawingLayerLookupStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    let duplicate = { let mut value = input["layers"][1]["children"][0].clone(); value["name"] = "Later Duplicate".into(); value };
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    let mut deep = input["layers"][0].clone();
    deep["id"] = long.clone().into();
    deep["name"] = "Long UTF8".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut group = input["layers"][1].clone();
        group["id"] = format!("nested-{depth}").into();
        group["children"] = serde_json::json!([deep]);
        deep = group;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let mut cases = law["lookup"]["cases"].as_array().unwrap().clone();
    let deep_path = std::iter::once(source.borrow().get().layers.len() - 1).chain(std::iter::repeat_n(0usize, law["lookup"]["deep"].as_u64().unwrap() as usize)).collect::<Vec<_>>();
    cases.push(serde_json::json!({"target":long,"name":"Long UTF8","path":deep_path}));
    cases.push(serde_json::json!({"target":"missing-after-deep-tree","name":null,"path":null}));
    for case in cases {
        let target = RetainedCloneSource::from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(case["target"].as_str().unwrap()).unwrap()), ());
        for pause in law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).chain([usize::MAX]) {
            let (mut cursor, allocation) = observe(|| DrawingLayerLookupCursor::new(source.borrow(), target.borrow()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(Default::default()).unwrap());
            assert!(matches!(zero, DrawingLayerLookupStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(grant));
                let step = result.unwrap();
                let (done, progress) = match step { DrawingLayerLookupStep::Pending(progress) => (false, progress), DrawingLayerLookupStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let (length, allocation) = observe(|| cursor.path_len());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                let expected_path = case["path"].as_array();
                assert_eq!(length, expected_path.map(Vec::len));
                if let Some(path) = expected_path {
                    for (index, ordinal) in path.iter().enumerate() {
                        let (actual, allocation) = observe(|| cursor.path_index(index));
                        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                        assert_eq!(actual, Some(ordinal.as_u64().unwrap() as usize));
                    }
                    assert!(cursor.path_index(path.len()).is_none());
                }
                let output = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                assert!(cursor.path_len().is_none());
                assert!(cursor.path_index(0).is_none());
                assert_eq!(output.map(|node| semio_s_artifact_draw_drawing_model::layer_base(node.get()).name.to_string_owner()), case["name"].as_str().map(str::to_owned));
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            for turn in 0..100000 {
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.close_granted(grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(cursor.terminal_is_empty());
            let (_, allocation) = observe(|| drop(cursor));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        }
    }
    assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
    eprintln!("[DEBUG] Drawing borrowed depth90 tree lookup preserves first duplicate/long UTF8/NUL, zero-allocation ordinal paths, and admits every real frame birth, body and cancellation release within4096");
}

#[test]
fn paged_native_drawing_snapshot_rename_preparation_preserves_borrowed_inverse_before_disposition() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::retained_clone::{RetainedCloneSource, RetainedCloneStep};
    use semio_s_artifact_draw_drawing_mutations::rename_prepare::{DrawingRenamePreparationCursor, DrawingRenamePreparationStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    input["layers"][1]["children"][0]["name"] = long.clone().into();
    let mut long_identity = input["layers"][0].clone();
    long_identity["id"] = long.clone().into();
    long_identity["name"] = "Long Identity".into();
    input["layers"].as_array_mut().unwrap().push(long_identity);
    let mut duplicate = input["layers"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let mut deep = input["layers"][0].clone();
    deep["id"] = "deep-rename".into();
    deep["name"] = "Deep Original".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut parent = input["layers"][1].clone();
        parent["id"] = format!("rename-nested-{depth}").into();
        parent["children"] = serde_json::Value::Array(vec![deep]);
        deep = parent;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    assert!(std::mem::size_of::<DrawingRenamePreparationCursor>() <= budget);
    for case in law["rename"]["cases"].as_array().unwrap() {
        let name = if case["repeatNewName"].as_bool().unwrap_or(false) { format!("{}{}{}", case["newName"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize), if case["disposition"] == "no-op" || case["suffix"].is_string() { "\0" } else { "" }, case["suffix"].as_str().unwrap_or("")) } else { case["newName"].as_str().unwrap().to_owned() };
        let target = if case["repeatTarget"].as_bool().unwrap_or(false) { long.as_str() } else { case["target"].as_str().unwrap() };
        let mutation = RetainedCloneSource::from_authority(std::sync::Arc::new(semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer { layer_id: target.into(), new_name: name.into() }), ());
        let mutation_before = serde_json::to_value(mutation.borrow().get()).unwrap();
        for pause in [0usize, 1, 3, 7, 17, 64, 256, 1024, usize::MAX] {
            let (mut cursor, allocation) = observe(|| DrawingRenamePreparationCursor::new(source.borrow(), mutation.borrow()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(Default::default()).unwrap());
            assert!(matches!(zero, DrawingRenamePreparationStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(grant));
                let step = result.unwrap();
                let (done, progress) = match step { DrawingRenamePreparationStep::Pending(progress) => (false, progress), DrawingRenamePreparationStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let plan = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                assert_eq!(plan.disposition.as_str(), case["disposition"].as_str().unwrap());
                let expected = case["inverseName"].as_str().map(|name| if name == "long" { long.as_str() } else { name });
                assert_eq!(plan.inverse_name().map(|name| name.get().to_string_owner()), expected.map(str::to_owned));
                assert_eq!(plan.payload.get(), mutation.borrow().get());
                let (_, allocation) = observe(|| drop(plan));
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            for turn in 0..100000 {
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.close_granted(grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(cursor.terminal_is_empty());
            let (_, allocation) = observe(|| drop(cursor));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
            assert_eq!(serde_json::to_value(mutation.borrow().get()).unwrap(), mutation_before);
        }
    }
    eprintln!("[DEBUG] Drawing borrowed rename preserves first identity/old inverse, empty and long UTF8/NUL names; every real lookup/comparison/cancellation birth+release <=4096 and constructor0");
}

#[test]
fn paged_native_drawing_snapshot_owned_rename_preparation_preserves_original_binding_and_root_projection() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::retained_clone::{RetainedCloneSource, RetainedCloneStep};
    use semio_s_artifact_draw_drawing_mutations::rename_prepare::owned::{DrawingOwnedRenamePreparationCursor, DrawingOwnedRenamePreparationStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    input["layers"][1]["children"][0]["name"] = long.clone().into();
    let mut long_identity = input["layers"][0].clone();
    long_identity["id"] = long.clone().into();
    long_identity["name"] = "Long Identity".into();
    input["layers"].as_array_mut().unwrap().push(long_identity);
    let mut duplicate = input["layers"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let mut deep = input["layers"][0].clone();
    deep["id"] = "deep-rename".into();
    deep["name"] = "Deep Original".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut parent = input["layers"][1].clone();
        parent["id"] = format!("rename-nested-{depth}").into();
        parent["children"] = serde_json::Value::Array(vec![deep]);
        deep = parent;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    fn assert_static<T: Send + Sync + 'static>() {}
    assert_static::<DrawingOwnedRenamePreparationCursor>();
    assert!(std::mem::size_of::<DrawingOwnedRenamePreparationCursor>() <= budget);
    for case in law["rename"]["cases"].as_array().unwrap() {
        let name = if case["repeatNewName"].as_bool().unwrap_or(false) { format!("{}{}{}", case["newName"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize), if case["disposition"] == "no-op" || case["suffix"].is_string() { "\0" } else { "" }, case["suffix"].as_str().unwrap_or("")) } else { case["newName"].as_str().unwrap().to_owned() };
        let target = if case["repeatTarget"].as_bool().unwrap_or(false) { long.as_str() } else { case["target"].as_str().unwrap() };
        let mutation = RetainedCloneSource::from_authority(std::sync::Arc::new(semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer { layer_id: target.into(), new_name: name.into() }), ());
        let mutation_before = serde_json::to_value(mutation.borrow().get()).unwrap();
        let foreign = RetainedCloneSource::from_authority(std::sync::Arc::new(mutation.borrow().get().clone()), ());
        for pause in [0usize, 1, 3, 7, 17, 64, 256, 1024, usize::MAX] {
            let (mut cursor, allocation) = observe(|| DrawingOwnedRenamePreparationCursor::new(source.project_owned(0, |snapshot| snapshot)).unwrap());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(mutation.borrow(), Default::default()).unwrap());
            assert!(matches!(zero, DrawingOwnedRenamePreparationStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(mutation.borrow(), grant));
                let step = result.unwrap();
                let (done, progress) = match step { DrawingOwnedRenamePreparationStep::Pending(progress) => (false, progress), DrawingOwnedRenamePreparationStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if turn == 0 {
                    let (zero, allocation) = observe(|| cursor.advance(foreign.borrow(), Default::default()).unwrap());
                    assert!(matches!(zero, DrawingOwnedRenamePreparationStep::Pending(progress) if progress == Default::default()));
                    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                    assert!(cursor.advance(foreign.borrow(), grant).is_err());
                }
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let mut plan = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                let (checked, allocation) = observe(|| plan.check_original(mutation.borrow()));
                checked.unwrap();
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                assert!(plan.check_original(foreign.borrow()).is_err());
                assert_eq!(plan.disposition.as_str(), case["disposition"].as_str().unwrap());
                let expected = case["inverseName"].as_str().map(|name| if name == "long" { long.as_str() } else { name });
                assert_eq!(plan.inverse_name().unwrap().map(|name| name.get().to_string_owner()), expected.map(str::to_owned));
                assert_eq!(plan.path_len(), case["path"].as_array().map(Vec::len));
                if let Some(expected) = case["path"].as_array() {
                    for (index, ordinal) in expected.iter().enumerate() {
                        let (actual, allocation) = observe(|| plan.path_index(index));
                        assert_eq!(actual, Some(ordinal.as_u64().unwrap() as usize));
                        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                    }
                }
                let (_, allocation) = observe(|| plan.begin_close());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                let (zero, allocation) = observe(|| plan.close_granted(Default::default()).unwrap());
                assert_eq!(zero.progress(), Default::default());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                assert!(plan.path_len().is_none());
                for turn in 0..100000 {
                    let grant = drawing_snapshot_heap_grant(turn, budget);
                    let (result, allocation) = observe(|| plan.close_granted(grant));
                    let step = result.unwrap();
                    drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                    if matches!(step, RetainedCloneStep::Complete(_)) { break; }
                }
                assert!(plan.terminal_is_empty());
                let (_, allocation) = observe(|| drop(plan));
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            for turn in 0..100000 {
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.close_granted(grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(cursor.terminal_is_empty());
            let (_, allocation) = observe(|| drop(cursor));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
            assert_eq!(serde_json::to_value(mutation.borrow().get()).unwrap(), mutation_before);
        }
    }
    eprintln!("[DEBUG] Drawing static owned rename preserves first identity/old inverse, empty and long UTF8/NUL names; every real lookup/comparison/cancellation birth+release <=4096 and constructor0");
}

#[test]
fn paged_native_drawing_snapshot_rename_inverse_measures_every_owned_page_and_retirement() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{retained_clone::{RetainedCloneSource, RetainedCloneStep}, retirement::controlled::ControlledRetirement};
    use semio_s_artifact_draw_drawing_mutations::rename_prepare::inverse::DrawingRenameInverseCursor;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    input["layers"][1]["children"][0]["name"] = long.clone().into();
    let mut long_identity = input["layers"][0].clone();
    long_identity["id"] = long.clone().into();
    long_identity["name"] = "Long Identity".into();
    input["layers"].as_array_mut().unwrap().push(long_identity);
    let mut duplicate = input["layers"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let mut deep = input["layers"][0].clone();
    deep["id"] = "deep-rename".into();
    deep["name"] = "Deep Original".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut parent = input["layers"][1].clone();
        parent["id"] = format!("rename-nested-{depth}").into();
        parent["children"] = serde_json::Value::Array(vec![deep]);
        deep = parent;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    fn assert_static<T: Send + 'static>() {}
    assert_static::<DrawingRenameInverseCursor>();
    assert!(std::mem::size_of::<DrawingRenameInverseCursor>() <= budget);
    for case in law["rename"]["cases"].as_array().unwrap() {
        let name = if case["repeatNewName"].as_bool().unwrap_or(false) { format!("{}{}{}", case["newName"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize), if case["disposition"] == "no-op" || case["suffix"].is_string() { "\0" } else { "" }, case["suffix"].as_str().unwrap_or("")) } else { case["newName"].as_str().unwrap().to_owned() };
        let target = if case["repeatTarget"].as_bool().unwrap_or(false) { long.as_str() } else { case["target"].as_str().unwrap() };
        let mutation = RetainedCloneSource::from_authority(std::sync::Arc::new(semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer { layer_id: target.into(), new_name: name.into() }), ());
        let mutation_before = serde_json::to_value(mutation.borrow().get()).unwrap();
        for pause in [0usize, 1, 3, 7, 17, 64, 256, 1024, 4096, usize::MAX] {
            let (mut cursor, allocation) = observe(|| DrawingRenameInverseCursor::new(source.project_owned(0, |snapshot| snapshot)).unwrap());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.advance(mutation.borrow(), Default::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(mutation.borrow(), grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                let output = cursor.take().unwrap();
                assert!(cursor.take().is_none());
                let expected = case["inverseName"].as_str().map(|name| if name == "long" { long.as_str() } else { name });
                assert_eq!(output.len(), usize::from(expected.is_some()));
                if let Some(name) = expected {
                    let DrawingMutation::RenameLayer(payload) = output.get(0).unwrap() else { panic!("wrong native inverse variant") };
                    assert!(payload.layer_id.eq_str(target));
                    assert!(payload.new_name.eq_str(name));
                }
                let (result, allocation) = observe(|| ControlledRetirement::new(output));
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                let mut owner = result.unwrap();
                let (zero, allocation) = observe(|| owner.step(Default::default()).unwrap());
                assert_eq!(zero.progress(), Default::default());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                for turn in 0..100000 {
                    let grant = drawing_snapshot_heap_grant(turn, budget);
                    let (result, allocation) = observe(|| owner.step(grant));
                    let step = result.unwrap();
                    drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                    if matches!(step, RetainedCloneStep::Complete(_)) { break; }
                }
                assert!(owner.terminal_is_empty());
                let (_, allocation) = observe(|| drop(owner));
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            for turn in 0..100000 {
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.close_granted(grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(cursor.terminal_is_empty());
            let (_, allocation) = observe(|| drop(cursor));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
            assert_eq!(serde_json::to_value(mutation.borrow().get()).unwrap(), mutation_before);
        }
    }
    eprintln!("[DEBUG] Drawing native rename inverse preserves missing/no-op/first-target old name; each real ID/name clone, literal/page birth, partial cancellation and returned owner release admitted within4096, constructor0");
}

#[test]
fn paged_native_drawing_snapshot_owned_lookup_measures_root_alias_and_native_pages() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{paged::PagedUtf8, retained_clone::{RetainedCloneSource, RetainedCloneStep}};
    use semio_s_artifact_draw_drawing_model::snapshot::lookup::owned::{DrawingOwnedLayerLookupCursor, DrawingOwnedLayerLookupStep};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let mut duplicate = input["layers"][1]["children"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().push(duplicate);
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    let mut deep = input["layers"][0].clone();
    deep["id"] = long.clone().into();
    deep["name"] = "Long UTF8".into();
    for depth in 0..law["lookup"]["deep"].as_u64().unwrap() {
        let mut group = input["layers"][1].clone();
        group["id"] = format!("nested-{depth}").into();
        group["children"] = serde_json::json!([deep]);
        deep = group;
    }
    input["layers"].as_array_mut().unwrap().push(deep);
    let source = RetainedCloneSource::from_authority(std::sync::Arc::new(serde_json::from_value::<DrawingSnapshot>(input).unwrap()), ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    let budget = law["bodyBytes"].as_u64().unwrap() as usize;
    let deep_path = std::iter::once(source.borrow().get().layers.len() - 1).chain(std::iter::repeat_n(0usize, law["lookup"]["deep"].as_u64().unwrap() as usize)).collect::<Vec<_>>();
    let mut cases = law["lookup"]["cases"].as_array().unwrap().clone();
    cases.push(serde_json::json!({"target":long,"name":"Long UTF8","path":deep_path}));
    cases.push(serde_json::json!({"target":"missing-after-deep-tree","name":null,"path":null}));
    fn static_owner<T: Send + Sync + 'static>() {}
    static_owner::<DrawingOwnedLayerLookupCursor>();
    assert!(std::mem::size_of::<DrawingOwnedLayerLookupCursor>() <= budget);
    for case in cases {
        let target = RetainedCloneSource::from_authority(std::sync::Arc::new(PagedUtf8::<{usize::MAX}>::try_from_str(case["target"].as_str().unwrap()).unwrap()), ());
        for pause in law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).chain([usize::MAX]) {
            let (created, allocation) = observe(|| DrawingOwnedLayerLookupCursor::new(source.project_owned(0, |snapshot| snapshot)));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut cursor = created.unwrap();
            let (zero, allocation) = observe(|| cursor.advance(target.borrow(), Default::default()).unwrap());
            assert!(matches!(zero, DrawingOwnedLayerLookupStep::Pending(progress) if progress == Default::default()));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let mut complete = false;
            for turn in 0..100000 {
                if turn == pause { break; }
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.advance(target.borrow(), grant));
                let (done, progress) = match result.unwrap() { DrawingOwnedLayerLookupStep::Pending(progress) => (false, progress), DrawingOwnedLayerLookupStep::Complete { progress, .. } => (true, progress) };
                drawing_snapshot_heap_admission(progress, grant, allocation);
                if done { complete = true; break; }
            }
            if pause == usize::MAX {
                assert!(complete);
                assert_eq!(cursor.path_len(), case["path"].as_array().map(Vec::len));
                if let Some(path) = case["path"].as_array() { for (index, ordinal) in path.iter().enumerate() { assert_eq!(cursor.path_index(index), Some(ordinal.as_u64().unwrap() as usize)); } }
                let (output, allocation) = observe(|| cursor.take().unwrap());
                assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                assert!(cursor.take().is_none() && cursor.path_len().is_none());
                assert_eq!(output.as_ref().map(|node| semio_s_artifact_draw_drawing_model::layer_base(node.borrow().unwrap().get()).name.to_string_owner()), case["name"].as_str().map(str::to_owned));
                if let Some(mut node) = output {
                    let (step, allocation) = observe(|| node.close_step(1).unwrap());
                    assert_eq!(step, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                    assert!(node.terminal_is_empty());
                    let (_, allocation) = observe(|| drop(node));
                    assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
                }
            }
            let (_, allocation) = observe(|| cursor.begin_close());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            let (zero, allocation) = observe(|| cursor.close_granted(Default::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            for turn in 0..100000 {
                let grant = drawing_snapshot_heap_grant(turn, budget);
                let (result, allocation) = observe(|| cursor.close_granted(grant));
                let step = result.unwrap();
                drawing_snapshot_heap_admission(step.progress(), grant, allocation);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(cursor.terminal_is_empty());
            let (_, allocation) = observe(|| drop(cursor));
            assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
            assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
        }
    }
    eprintln!("[DEBUG] owned Drawing lookup holds actual immutable root projections, reborrows target each turn, preserves first duplicate/depth90/UTF8/NUL and admits every native page birth/release within4096; constructor/alias close0heap");
}
