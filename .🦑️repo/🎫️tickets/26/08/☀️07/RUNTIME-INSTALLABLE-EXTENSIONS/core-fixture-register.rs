        if kind == "core.image" {
            let value = input.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
            return Ok(channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(value.into())))));
        }
        if kind == "math.add" {
            let a = input.get("a").or_else(|| input.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("a".into()))?;
            let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(channel_output("sum", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(a + b)))));
        }
        if kind == "math.passThrough" {
            let n = input.get("number").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number".into()))?;
            return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(n)))));
        }
        if kind == "core.variable" {
            let name = input.get("name").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).ok_or_else(|| EvalError::MissingInput("name".into()))?;
            let payload = input.get(name).and_then(|v| v.as_dictionary()).cloned().ok_or_else(|| EvalError::MissingInput(name.into()))?;
            return Ok(channel_output(name, payload));
        }
        Err(EvalError::UnknownKind(kind.into()))
    }

    fn fixture_kind_infos_json() -> String {
        let mut registry = Registry::new();
        flow_extension_core::register(&mut registry);
        flow_extension_math::register(&mut registry);
        flow_extension_brep::register(&mut registry);
        serde_json::to_string(&registry.operator_catalogue()).unwrap_or_else(|_| "[]".into())
    }

    fn test_kind_infos_json() -> String {
        serde_json::to_string(&[
            NeuronKindInfo {
                id: "math.add".into(),
                extension: "math".into(),
                name: "Add".into(),
                abbreviation: "Add".into(),
                icon: "emoji:➕️".into(),
                summary: "Sums two numbers".into(),
                inputs: vec![InputSpec::number("a", NUMBER_OPS), InputSpec::number_default("b", 0.0, NUMBER_OPS)],
                outputs: vec![InputSpec::named("S", "Sum", "sum", "Sum")],
                ..Default::default()
            },
            NeuronKindInfo {
                id: "math.passThrough".into(),
                extension: "math".into(),
                name: "PassThrough".into(),
                abbreviation: "Pass".into(),
                icon: "emoji:➡️".into(),
                summary: "Forwards a number".into(),
                inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
                outputs: vec![InputSpec::named("N", "Num", "number", "Number")],
                ..Default::default()
            },
        ])
        .unwrap()
    }

    fn host_with_test_bridge() -> FlowHost {
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(test_math_bridge));