use super::*;

#[semio_framework_async_macros::async_test]
async fn interaction_spec_parses_box_asset() {
    let raw = include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📦️box.json");
    let spec: InteractionSpec = protocol::json::from_json_str(raw).expect("📦️box.json parses as InteractionSpec");
    assert_eq!(spec.id, "primitive.box");
    assert_eq!(spec.machine.initial, "idle");
    assert!(spec.state("first_corner").is_some());
    assert!(spec.state("ready").is_some());
    assert_eq!(spec.commit.operation.action, "primitive.createBoxFromCorners");
    assert!(spec.commit.operation.params.contains_key("cornerA"));
    assert!(spec.commit.operation.params.contains_key("cornerB"));
    assert!(spec.commit.operation.params.contains_key("height"));
    assert_eq!(spec.commit.from_states, vec!["ready".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn interaction_spec_parses_sphere_asset_with_command_finish() {
    let raw = include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌐️sphere.json");
    let spec: InteractionSpec = protocol::json::from_json_str(raw).expect("🌐️sphere.json parses as InteractionSpec");
    assert_eq!(spec.id, "solid.sphere");
    assert_eq!(spec.commit.operation.action, "command.finish");
    assert!(spec.display.states.iter().any(|s| s.state == "radius"));
}

#[semio_framework_async_macros::async_test]
async fn interaction_spec_parses_all_energy_and_structure_classic_assets() {
    let sources = [
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🧱️constructBasePlate.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🚧️constructExternalWall.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🚢️constructHull.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🏠️constructRoof.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🪟️constructWindows.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🧱️constructOneWayRe-72a083.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🏛️constructReinforc-411bd6.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🛡️constructReinforc-c38891.json"),
        include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🚧️constructReinforc-e8fc67.json"),
    ];
    for raw in sources {
        let spec: InteractionSpec = protocol::json::from_json_str(raw).expect("asset parses as InteractionSpec");
        assert!(spec.commit.operation.action.ends_with("From2PointsAndHeight") || spec.commit.operation.action.ends_with("FromSurface"));
        assert!(spec.commit.operation.params.contains_key("pointA"));
        assert!(spec.commit.operation.params.contains_key("pointB"));
        assert!(spec.commit.operation.params.contains_key("height"));
        assert!(spec.commit.operation.params.contains_key("typology"));
    }
}

/// Regression guard: every `interaction/*.json` asset in the tree must parse as
/// `InteractionSpec` — catches schema drift between the JSON assets and these Rust types.
#[semio_framework_async_macros::async_test]
async fn every_interaction_asset_on_disk_parses_as_interaction_spec() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions");
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".json")) && path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()) == Some("🕹️interactions") {
                out.push(path);
            }
        }
    }
    let mut files = Vec::new();
    walk(&root, &mut files);
    assert!(files.len() >= 40, "expected at least 40 interaction assets, found {}", files.len());
    let mut failures = Vec::new();
    for file in &files {
        let raw = std::fs::read_to_string(file).expect("read asset");
        if let Err(err) = protocol::json::from_json_str::<InteractionSpec>(&raw) {
            failures.push(format!("{}: {}", file.display(), err));
        }
    }
    assert!(failures.is_empty(), "{} interaction assets failed to parse:\n{}", failures.len(), failures.join("\n"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_expr_supports_path_const_var_and_boolean_combinators() {
    let mut context = std::collections::HashMap::new();
    context.insert("height".to_string(), DslValue::float(2.5));
    context.insert("origin".to_string(), DslValue::Array(vec![DslValue::float(0.0), DslValue::float(0.0), DslValue::float(0.0)]));
    let env = ExprEnv { context: &context, event: None };
    let vars = std::collections::HashMap::new();

    let path_expr = Expr::Path { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "height".into() }] };
    assert_eq!(evaluate_expr(&path_expr, &env, &vars), DslValue::float(2.5));

    let exists_expr = Expr::Exists { target: ExprPathTarget { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "origin".into() }] } };
    assert_eq!(evaluate_expr(&exists_expr, &env, &vars), DslValue::Bool(true));

    let missing_exists_expr = Expr::Exists { target: ExprPathTarget { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "missing".into() }] } };
    assert_eq!(evaluate_expr(&missing_exists_expr, &env, &vars), DslValue::Bool(false));

    let binop_expr = Expr::Binop { operation: ">".into(), left: Box::new(path_expr.clone()), right: Box::new(Expr::Const { value: DslValue::float(1.0) }) };
    assert_eq!(evaluate_expr(&binop_expr, &env, &vars), DslValue::Bool(true));

    let all_expr = Expr::All { args: vec![exists_expr, binop_expr] };
    assert_eq!(evaluate_expr(&all_expr, &env, &vars), DslValue::Bool(true));

    let let_expr = Expr::Let {
        bindings: vec![ExprBinding { name: "h".into(), value: Box::new(path_expr) }],
        body: Box::new(Expr::Binop { operation: "*".into(), left: Box::new(Expr::Var { name: "h".into() }), right: Box::new(Expr::Const { value: DslValue::float(2.0) }) }),
    };
    assert_eq!(evaluate_expr(&let_expr, &env, &vars), DslValue::float(5.0));
}

#[semio_framework_async_macros::async_test]
async fn interaction_spec_guard_evaluates_against_context() {
    let raw = include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🚧️constructExternalWall.json");
    let spec: InteractionSpec = protocol::json::from_json_str(raw).expect("parses");
    let mut context = std::collections::HashMap::new();
    let env_without = ExprEnv { context: &context, event: None };
    assert!(!spec.guard("hasConstructMode", &env_without));
    context.insert("constructMode".to_string(), DslValue::String("2PointsAndHeight".to_string()));
    let env_with = ExprEnv { context: &context, event: None };
    assert!(spec.guard("hasConstructMode", &env_with));
}
