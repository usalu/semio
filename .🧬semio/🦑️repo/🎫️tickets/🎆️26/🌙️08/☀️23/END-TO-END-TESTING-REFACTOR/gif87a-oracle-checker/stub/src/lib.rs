#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
            _ => None,
        }
    }
    pub fn str(&self, key: &str) -> String {
        match self.get(key) {
            Some(Json::String(value)) => value.clone(),
            _ => String::new(),
        }
    }
    pub fn array(&self, key: &str) -> Vec<Json> {
        match self.get(key) {
            Some(Json::Array(items)) => items.clone(),
            _ => Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            Json::Number(value) => out.push_str(&format!("{}", *value as i64)),
            Json::String(value) => out.push_str(&format!("{:?}", value)),
            Json::Array(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { out.push(','); }
                    item.write(out);
                }
                out.push(']');
            }
            Json::Object(entries) => {
                out.push('{');
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 { out.push(','); }
                    out.push_str(&format!("{:?}", k));
                    out.push(':');
                    v.write(out);
                }
                out.push('}');
            }
        }
    }
}
