//! 🧾️ Canonical JSON: the byte-stable writer behind goldens, content hashes and cross-language comparison.

use value::{DslValue, Number, ToValue};

/// 🧾️ Writes `value` as canonical JSON: object keys sorted bytewise, no whitespace, integers bare, floats with a decimal point, `-0` as `0.0`.
pub fn canonical_json<T: ToValue + ?Sized>(value: &T) -> String {
    let mut out = String::new();
    write_value(&value.to_value(), &mut out);
    out
}

fn write_value(value: &DslValue, out: &mut String) {
    match value {
        DslValue::Null | DslValue::Bytes(_) => out.push_str("null"),
        DslValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        DslValue::Number(Number::UInt(v)) => out.push_str(&v.to_string()),
        DslValue::Number(Number::Int(v)) => out.push_str(&v.to_string()),
        DslValue::Number(Number::Float(v)) => write_float(*v, out),
        DslValue::String(s) => write_string(s, out),
        DslValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(item, out);
            }
            out.push(']');
        }
        DslValue::Object(entries) => {
            let mut sorted: Vec<_> = entries.iter().collect();
            sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
            out.push('{');
            for (i, (key, item)) in sorted.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(key, out);
                out.push(':');
                write_value(item, out);
            }
            out.push('}');
        }
    }
}

fn write_float(v: f64, out: &mut String) {
    if !v.is_finite() {
        out.push_str("null");
    } else if v == 0.0 {
        out.push_str("0.0");
    } else {
        let text = format!("{v:?}");
        out.push_str(&text);
    }
}

fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}
