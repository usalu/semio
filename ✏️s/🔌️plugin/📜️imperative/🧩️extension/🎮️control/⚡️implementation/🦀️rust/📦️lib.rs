//! 🔀️ Imperative control module: catalogue-only control-flow step kinds.

pub fn catalogue_json() -> String {
    serde_json::to_string(&serde_json::json!({
        "schema": "imperative.catalogue",
        "sections": [{
            "id": "control",
            "title": "Control",
            "items": [
                {
                    "kind": "control.if",
                    "name": "If",
                    "abbreviation": "If",
                    "icon": "emoji:🔀️",
                    "summary": "Runs the then or else body based on a boolean scope key.",
                    "module": "control",
                    "inputs": [{ "name": "key", "code": "S" }],
                    "bodies": ["then", "else"],
                },
                {
                    "kind": "control.while",
                    "name": "While",
                    "abbreviation": "Whl",
                    "icon": "emoji:🔁️",
                    "summary": "Repeats the body while a boolean scope key is true.",
                    "module": "control",
                    "inputs": [{ "name": "key", "code": "S" }],
                    "bodies": ["body"],
                },
                {
                    "kind": "control.repeat",
                    "name": "Repeat",
                    "abbreviation": "Rpt",
                    "icon": "emoji:🔁️",
                    "summary": "Repeats the body a fixed number of times.",
                    "module": "control",
                    "inputs": [{ "name": "count", "code": "N" }],
                    "bodies": ["body"],
                },
            ],
        }],
    }))
    .unwrap_or_else(|_| "{}".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_includes_control_kinds() {
        let raw = catalogue_json();
        assert!(raw.contains("control.if"));
        assert!(raw.contains("control.while"));
        assert!(raw.contains("control.repeat"));
    }
}
