// #region 🔖️EvalBridge
#[cfg(target_arch = "wasm32")]
fn parse_bridge_dictionary_json(result_json: &str) -> Result<Dictionary, EvalError> {
    if let Ok(dict) = serde_json::from_str::<Dictionary>(result_json) {
        return Ok(dict);
    }
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(result_json) {
        if let Some(err) = value.get("error").and_then(|v| v.as_str()) {
            return Err(EvalError::InvalidInput(err.into()));
        }
    }
    Err(EvalError::InvalidInput("invalid bridge response".into()))
}

#[cfg(target_arch = "wasm32")]
struct EvalBridge {
    cb: js_sys::Function,
}

#[cfg(target_arch = "wasm32")]
impl EvalBridge {
    fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        use wasm_bindgen::JsValue;
        let input_json = serde_json::to_string(input).map_err(|e| EvalError::InvalidInput(e.to_string()))?;
        let result = self.cb.call2(&JsValue::NULL, &JsValue::from_str(kind_id), &JsValue::from_str(&input_json)).map_err(|_| EvalError::InvalidInput("bridge call failed".into()))?;
        let result_json = result.as_string().ok_or_else(|| EvalError::InvalidInput("bridge did not return string".into()))?;
        parse_bridge_dictionary_json(&result_json)
    }
}

/// 🔌️ Native eval-bridge callback: operator kind id + input dictionary in, evaluated dictionary or `EvalError` out.
#[cfg(not(target_arch = "wasm32"))]
type EvalBridgeFn = dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Send;

#[cfg(not(target_arch = "wasm32"))]
struct EvalBridge {
    cb: Box<EvalBridgeFn>,
}

#[cfg(not(target_arch = "wasm32"))]
impl EvalBridge {
    fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        (self.cb)(kind_id, input)
    }
}
// #endregion 🔖️EvalBridge

// #region 🔖️ChannelEval
fn neural_value_to_json(value: &NeuralValue) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

fn dictionary_to_json_object(dict: &Dictionary) -> serde_json::Map<String, serde_json::Value> {
    dict.keys().map(|key| (key.clone(), neural_value_to_json(dict.get(key).expect("key came from dict.keys(), so get(key) cannot miss")))).collect()
}

fn input_ports_json(dict: &Dictionary, kind_info: Option<&OperatorInfo>) -> serde_json::Map<String, serde_json::Value> {
    let mut ports = serde_json::Map::new();
    if let Some(info) = kind_info {
        if let Some(variadic) = &info.variadic_input {
            if let Some(slots) = dict.get(&variadic.slot_key).and_then(|value| value.as_dictionary()) {
                for key in slots.keys() {
                    if let Some(value) = slots.get(key) {
                        ports.insert(key.clone(), neural_value_to_json(value));
                    }
                }
            }
        }
        for port in &info.inputs {
            if port.name == "*" {
                continue;
            }
            if let Some(value) = dict.get(&port.name) {
                ports.insert(port.name.clone(), neural_value_to_json(value));
            }
        }
        return ports;
    }
    dictionary_to_json_object(dict)
}

fn output_ports_json(dict: &Dictionary) -> serde_json::Map<String, serde_json::Value> {
    let mut ports = serde_json::Map::new();
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            ports.insert(key.clone(), serde_json::to_value(value).unwrap_or(serde_json::Value::Null));
        }
    }
    ports
}

fn outputs_from_channel_eval_json(json: &str) -> HashMap<String, Dictionary> {
    let Ok(parsed) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json) else {
        return HashMap::new();
    };
    let mut outputs = HashMap::new();
    for (widget_id, entry) in parsed {
        let Some(out_ports) = entry.get("out").and_then(|value| value.as_object()) else {
            continue;
        };
        let mut dict = Dictionary::new();
        for (key, val) in out_ports {
            if let Ok(value) = serde_json::from_value::<NeuralValue>(val.clone()) {
                dict = dict.insert(key.clone(), value);
            }
        }
        if !dict.is_empty() {
            outputs.insert(widget_id, dict);
        }
    }
    outputs
}

fn inputs_from_channel_eval_json(json: &str) -> HashMap<String, Dictionary> {
    let Ok(parsed) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json) else {
        return HashMap::new();
    };
    let mut inputs = HashMap::new();
    for (widget_id, entry) in parsed {
        let Some(in_ports) = entry.get("in").and_then(|value| value.as_object()) else {
            continue;
        };
        let mut dict = Dictionary::new();
        for (key, val) in in_ports {
            if let Ok(value) = serde_json::from_value::<NeuralValue>(val.clone()) {
                dict = dict.insert(key.clone(), value);
            }
        }
        if !dict.is_empty() {
            inputs.insert(widget_id, dict);
        }
    }
    inputs
}

fn preview_dict_from_connection(src: &Dictionary, from_port: &str, to_port: &str) -> Dictionary {
    let payload = if from_port.is_empty() {
        src.clone()
    } else if let Some(dict) = src.get(from_port).and_then(|value| value.as_dictionary()) {
        dict.clone()
    } else if let Some(value) = src.get(from_port) {
        Dictionary::new().insert(from_port, value.clone())
    } else {
        Dictionary::new()
    };
    if to_port.is_empty() {
        payload
    } else {
        Dictionary::new().insert(to_port, NeuralValue::Dictionary(payload))
    }
}

fn widget_operator_info(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> Option<OperatorInfo> {
    match widget {
        Widget::Neuron { neuron_kind, .. } => kind_infos.get(neuron_kind).cloned(),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            let info = OperatorInfo {
                id: "core.variable".into(),
                extension: "core".into(),
                name: name.clone(),
                abbreviation: name.chars().take(3).collect(),
                icon: "emoji:🔣️".into(),
                summary: "Named typed dictionary".into(),
                inputs: inputs.iter().map(|port| ChannelSpec::requires(&port.id, &[schema.as_str()])).collect(),
                outputs: outputs.iter().map(|port| ChannelSpec::provides(&port.id, vec![schema.clone()])).collect(),
                ..Default::default()
            };
            Some(info)
        }
        Widget::Cluster { id, name, tree, .. } => Some(cluster_operator_info(id, if name.is_empty() { "Cluster" } else { name }, tree)),
        _ => None,
    }
}

fn widget_to_inner_neuron(widget: &Widget) -> Option<Neuron> {
    match widget {
        Widget::Neuron { id, neuron_kind, params, .. } => Some(Neuron::with_kind(id, neuron_kind, params.clone())),
        Widget::InputSlider { id, value, .. } => Some(Neuron::with_kind(id, "core.number", Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(*value))))),
        Widget::InputNote { id, text } => Some(Neuron::with_kind(id, "core.text", Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))))),
        Widget::InputImage { id, src } => Some(Neuron::with_kind(id, "core.image", Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))))),
        Widget::Variable { id, name, schema } => Some(Neuron::with_kind(id, "core.variable", Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))).insert("schema", NeuralValue::Atom(Atom::String(schema.clone()))))),
        _ => None,
    }
}

fn contract_boundary_params(channel: &str, schema: &str) -> Dictionary {
    Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String(channel.into()))).insert("operators", NeuralValue::Atom(Atom::String(schema.into())))
}

fn boundary_schema_from_params(params: &Dictionary) -> String {
    params.get("operators").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("dictionary").to_string()
}

fn variable_widget_meta(widgets: &[Widget], id: &str) -> Option<(String, String)> {
    widgets.iter().find_map(|widget| match widget {
        Widget::Variable { id: widget_id, name, schema } if widget_id == id => Some((name.clone(), schema.clone())),
        _ => None,
    })
}