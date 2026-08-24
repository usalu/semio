//! 🧪️ Throwaway verification for the AP214 test-vector fix (IFCLENGTHMEASURE -> LENGTH_MEASURE).
//!
//! The real test lives in `📐️step/…/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`, but the repository
//! workspace does not compile today: `semio-framework-job` and `semio-framework-ui-contract` are
//! mid-refactor in another session, and `semio-s-plugin-stdio` cannot be built through them. The
//! `StepValue` codec functions below are copied VERBATIM from
//! `📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (`enc_value`,
//! `dec_value`, `enc_value_bin`, `dec_value_bin` and their string primitives) — the exact code the
//! mutation module's `print_op`/`parse_op` and `encode_op`/`decode_op` reach for a `TypedValue`.

#[derive(Clone, Debug, PartialEq)]
enum StepValue {
    Unset,
    Derived,
    Integer(i64),
    Real(f64),
    String(String),
    Enum(String),
    Reference(u64),
    Aggregate(Vec<StepValue>),
    TypedValue { type_name: String, value: Box<StepValue> },
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

fn enc_value(v: &StepValue) -> String {
    match v {
        StepValue::Unset => "U[]".to_string(),
        StepValue::Derived => "D[]".to_string(),
        StepValue::Integer(i) => format!("I[{i}]"),
        StepValue::Real(r) => format!("R[{r}]"),
        StepValue::String(s) => format!("S[{}]", enc_str(s)),
        StepValue::Enum(s) => format!("E[{}]", enc_str(s)),
        StepValue::Reference(id) => format!("F[{id}]"),
        StepValue::Aggregate(items) => format!("A[{}]", items.iter().map(enc_value).collect::<Vec<_>>().join(",")),
        StepValue::TypedValue { type_name, value } => format!("T[{},{}]", enc_str(type_name), enc_value(value)),
    }
}
fn dec_value(s: &str) -> Result<StepValue, String> {
    if s.len() < 3 {
        return Err(format!("step value: too short {s:?}"));
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "U" => Ok(StepValue::Unset),
        "D" => Ok(StepValue::Derived),
        "I" => Ok(StepValue::Integer(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(StepValue::Real(inner.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?)),
        "S" => Ok(StepValue::String(dec_str(inner)?)),
        "E" => Ok(StepValue::Enum(dec_str(inner)?)),
        "F" => Ok(StepValue::Reference(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "A" => {
            let items = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?;
            Ok(StepValue::Aggregate(items))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [type_name, value] = parts.as_slice() else { return Err(format!("typed value: expected 2 fields, got {}", parts.len())) };
            Ok(StepValue::TypedValue { type_name: dec_str(type_name)?, value: Box::new(dec_value(value)?) })
        }
        other => Err(format!("step value: unknown tag {other:?}")),
    }
}

fn write_varint_u64(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let byte = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}
fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, String> {
    let mut out = 0u64;
    let mut shift = 0u32;
    loop {
        let byte = *bytes.get(*pos).ok_or("varint: out of bytes")?;
        *pos += 1;
        out |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(out);
        }
        shift += 7;
    }
}
fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_bin(bytes: &[u8], pos: &mut usize) -> Result<String, String> {
    let len = read_varint_u64(bytes, pos)? as usize;
    let slice = bytes.get(*pos..*pos + len).ok_or("string: out of bytes")?;
    *pos += len;
    String::from_utf8(slice.to_vec()).map_err(|e| e.to_string())
}
fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn read_f64_bin(bytes: &[u8], pos: &mut usize) -> Result<f64, String> {
    let slice = bytes.get(*pos..*pos + 8).ok_or("f64: out of bytes")?;
    *pos += 8;
    Ok(f64::from_le_bytes(slice.try_into().map_err(|_| "f64: bad width")?))
}

fn enc_value_bin(v: &StepValue, out: &mut Vec<u8>) {
    match v {
        StepValue::Unset => out.push(0),
        StepValue::Derived => out.push(1),
        StepValue::Integer(i) => {
            out.push(2);
            write_varint_u64(out, ((*i << 1) ^ (*i >> 63)) as u64);
        }
        StepValue::Real(r) => {
            out.push(3);
            write_f64_bin(out, *r);
        }
        StepValue::String(s) => {
            out.push(4);
            write_str_bin(out, s);
        }
        StepValue::Enum(s) => {
            out.push(5);
            write_str_bin(out, s);
        }
        StepValue::Reference(id) => {
            out.push(6);
            write_varint_u64(out, *id);
        }
        StepValue::Aggregate(items) => {
            out.push(7);
            write_varint_u64(out, items.len() as u64);
            for item in items {
                enc_value_bin(item, out);
            }
        }
        StepValue::TypedValue { type_name, value } => {
            out.push(8);
            write_str_bin(out, type_name);
            enc_value_bin(value, out);
        }
    }
}
fn dec_value_bin(bytes: &[u8], pos: &mut usize) -> Result<StepValue, String> {
    let tag = *bytes.get(*pos).ok_or("tag: out of bytes")?;
    *pos += 1;
    match tag {
        0 => Ok(StepValue::Unset),
        1 => Ok(StepValue::Derived),
        2 => {
            let zig = read_varint_u64(bytes, pos)?;
            Ok(StepValue::Integer(((zig >> 1) as i64) ^ -((zig & 1) as i64)))
        }
        3 => Ok(StepValue::Real(read_f64_bin(bytes, pos)?)),
        4 => Ok(StepValue::String(read_str_bin(bytes, pos)?)),
        5 => Ok(StepValue::Enum(read_str_bin(bytes, pos)?)),
        6 => Ok(StepValue::Reference(read_varint_u64(bytes, pos)?)),
        7 => {
            let count = read_varint_u64(bytes, pos)?;
            let items = (0..count).map(|_| dec_value_bin(bytes, pos)).collect::<Result<Vec<_>, String>>()?;
            Ok(StepValue::Aggregate(items))
        }
        8 => {
            let type_name = read_str_bin(bytes, pos)?;
            let value = Box::new(dec_value_bin(bytes, pos)?);
            Ok(StepValue::TypedValue { type_name, value })
        }
        other => Err(format!("step value binary: unknown tag {other}")),
    }
}

/// 🧪️ The nine-argument `insert-entity` vector both AP214 round-trip tests build, with the wrapper's
/// type name under test.
fn demo_args(type_name: &str) -> Vec<StepValue> {
    vec![
        StepValue::Unset,
        StepValue::Derived,
        StepValue::Integer(-42),
        StepValue::Real(3.5),
        StepValue::String("s".into()),
        StepValue::Enum("T".into()),
        StepValue::Reference(9),
        StepValue::Aggregate(vec![StepValue::Integer(1), StepValue::Real(2.0)]),
        StepValue::TypedValue { type_name: type_name.into(), value: Box::new(StepValue::Real(3000.0)) },
    ]
}

fn main() {
    let mut failures = 0usize;
    for type_name in ["LENGTH_MEASURE", "IFCLENGTHMEASURE", "X"] {
        for value in demo_args(type_name) {
            let text = enc_value(&value);
            match dec_value(&text) {
                Ok(back) if back == value => {}
                Ok(back) => {
                    failures += 1;
                    println!("[FAIL] text round-trip {type_name}: {value:?} -> {text:?} -> {back:?}");
                }
                Err(e) => {
                    failures += 1;
                    println!("[FAIL] text decode {type_name}: {text:?} -> {e}");
                }
            }
            let mut bytes = Vec::new();
            enc_value_bin(&value, &mut bytes);
            let mut pos = 0usize;
            match dec_value_bin(&bytes, &mut pos) {
                Ok(back) if back == value && pos == bytes.len() => {}
                Ok(back) => {
                    failures += 1;
                    println!("[FAIL] binary round-trip {type_name}: {value:?} -> {back:?} (consumed {pos}/{})", bytes.len());
                }
                Err(e) => {
                    failures += 1;
                    println!("[FAIL] binary decode {type_name}: {e}");
                }
            }
        }
        let wrapper = StepValue::TypedValue { type_name: type_name.into(), value: Box::new(StepValue::Real(3000.0)) };
        println!("[ok] {type_name}: text={} bytes={}", enc_value(&wrapper), {
            let mut b = Vec::new();
            enc_value_bin(&wrapper, &mut b);
            b.len()
        });
    }
    println!("[result] failures={failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
