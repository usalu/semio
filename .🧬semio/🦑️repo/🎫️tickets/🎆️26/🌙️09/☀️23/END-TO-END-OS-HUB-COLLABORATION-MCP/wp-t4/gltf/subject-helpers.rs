    //#region 🔖️Params
    /// 🔢️ A non-negative integer payload member.
    fn num(params: &Json, key: &str) -> Result<usize, String> {
        match params.get(key) {
            Some(Json::Number(value)) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
            _ => Err(format!("missing or non-integer `{key}`")),
        }
    }
    /// 🔢️ A `u32` payload member.
    fn num_u32(params: &Json, key: &str) -> Result<u32, String> {
        u32::try_from(num(params, key)?).map_err(|_| format!("`{key}` exceeds u32"))
    }
    /// 🔢️ An index list payload member.
    fn order(params: &Json, key: &str) -> Result<Vec<usize>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(value) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
                    _ => Err(format!("`{key}` must hold only non-negative integers")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    /// 🔢️ A number list payload member.
    fn floats(params: &Json, key: &str) -> Result<Vec<f64>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(value) => Ok(*value),
                    _ => Err(format!("`{key}` must hold only numbers")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    /// 💾️ A byte list payload member.
    fn bytes(params: &Json, key: &str) -> Result<Vec<u8>, String> {
        order(params, key)?.into_iter().map(|byte| u8::try_from(byte).map_err(|_| format!("`{key}` holds a value above 255"))).collect()
    }
    /// 📝️ A string payload member.
    fn text(params: &Json, key: &str) -> Result<String, String> {
        match params.get(key) {
            Some(Json::String(value)) => Ok(value.clone()),
            _ => Err(format!("missing or non-string `{key}`")),
        }
    }
    /// 📝️ A nullable string payload member.
    fn optional_text(params: &Json, key: &str) -> Result<Option<String>, String> {
        match params.get(key) {
            Some(Json::String(value)) => Ok(Some(value.clone())),
            Some(Json::Null) | None => Ok(None),
            _ => Err(format!("`{key}` must be a string or null")),
        }
    }
    /// 📝️ A string list payload member.
    fn texts(params: &Json, key: &str) -> Result<Vec<String>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::String(value) => Ok(value.clone()),
                    _ => Err(format!("`{key}` must hold only strings")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    //#endregion 🔖️Params

    //#region 🔖️Presence
    /// 🌱️ The snapshot's own JSON value for one spec value.
    fn gltf_json(value: &Json) -> GltfJson {
        match value {
            Json::Null => GltfJson::Null,
            Json::Bool(flag) => GltfJson::Bool(*flag),
            Json::Number(number) => GltfJson::Number(*number),
            Json::String(text) => GltfJson::String(text.clone()),
            Json::Array(items) => GltfJson::Array(items.iter().map(gltf_json).collect()),
            Json::Object(entries) => GltfJson::Object(entries.iter().map(|(key, item)| (key.clone(), gltf_json(item))).collect()),
        }
    }
    /// 🧩️ `data: {state: present, value}` as `Some(value)`, `data: {state: absent}` as `None`.
    fn presence(params: &Json) -> Result<Option<GltfJson>, String> {
        let data = params.get("data").ok_or("missing `data`")?;
        match data.get("state") {
            Some(Json::String(state)) if state == "absent" => Ok(None),
            Some(Json::String(state)) if state == "present" => data.get("value").map(|value| Some(gltf_json(value))).ok_or_else(|| "present data carries no `value`".to_string()),
            _ => Err("`data.state` must be `present` or `absent`".to_string()),
        }
    }
    //#endregion 🔖️Presence

    //#region 🔖️Transform
    /// 🧮️ `transform: {kind: matrix, matrix} | {kind: trs, translation?, rotation?, scale?}`.
    fn transform<T>(params: &Json, matrix: impl Fn([f64; 16]) -> T, trs: impl Fn(Option<[f64; 3]>, Option<[f64; 4]>, Option<[f64; 3]>) -> T) -> Result<T, String> {
        fn fixed<const N: usize>(value: &Json, key: &str) -> Result<Option<[f64; N]>, String> {
            match value.get(key) {
                None | Some(Json::Null) => Ok(None),
                Some(Json::Array(items)) if items.len() == N => {
                    let mut out = [0.0; N];
                    for (slot, item) in out.iter_mut().zip(items) {
                        *slot = match item {
                            Json::Number(number) => *number,
                            _ => return Err(format!("`{key}` must hold only numbers")),
                        };
                    }
                    Ok(Some(out))
                }
                _ => Err(format!("`{key}` must hold {N} numbers")),
            }
        }
        let value = params.get("transform").ok_or("missing `transform`")?;
        match value.get("kind") {
            Some(Json::String(kind)) if kind == "matrix" => Ok(matrix(fixed::<16>(value, "matrix")?.ok_or("missing `matrix`")?)),
            Some(Json::String(kind)) if kind == "trs" => Ok(trs(fixed::<3>(value, "translation")?, fixed::<4>(value, "rotation")?, fixed::<3>(value, "scale")?)),
            _ => Err("`transform.kind` must be `matrix` or `trs`".to_string()),
        }
    }
    //#endregion 🔖️Transform
