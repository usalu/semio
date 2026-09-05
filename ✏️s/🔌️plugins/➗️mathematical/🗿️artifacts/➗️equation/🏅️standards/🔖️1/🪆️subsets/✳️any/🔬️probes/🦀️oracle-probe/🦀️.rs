//! 🔬️ Third-party carrier reader for `s.mathematical.equation@1/✳️any`.
//!
//! This binary READS. It parses the exported CSV grid with the `csv` crate — an already-approved
//! `test-oracle` entry (registered in `🔒️dependencies.json` for the `sequence` subset's own
//! `csv-rfc4180-reader`, the same shape this file repeats for a different owner) — and reports what
//! that library recovered from the bytes. It knows nothing about this repository's mutation
//! vocabulary and it is never asked what a mutation should produce: an oracle that predicts the
//! expected result is this repository's own semantics wearing a third-party name, which is the exact
//! shape the `reimplementation-registered-as-third-party` gate blocks.
//!
//! The carrier decides what is checkable. This subset's own `EquationIntoCsv` serializer writes
//! exactly one row per graph node — `id,label(quoted),x,y`, no header
//! (`…➗️equation/…/📊️csv/🔖️rfc4180/✳️any/🦀️.rs:22-33`) — so this reader only ever answers
//! questions about node identity, label text and position. It has no column for `edges`,
//! `directed`, `algorithm`, point-cloud geometry or the `equation` AST, and it never fabricates one.
//!
//! Usage — one probe per invocation, one JSON body on stdout:
//!   semio-equation-oracle-probe csv-rows    --input a.csv
//!   semio-equation-oracle-probe csv-compare --input expected.csv --input actual.csv
//!   semio-equation-oracle-probe gate-inputs --out <dir>
//!   semio-equation-oracle-probe fixtures    --out <dir>
//!
//! @see ../📜️script.ts — the wrapper that stamps the ProbeReport envelope around this output
//! @see ../../🔣️oracle.json — the oracle, probe and pipeline registrations

//#region 🧾️Json
/// 🧾️ The smallest JSON value this probe needs. Hand-rolled on purpose: `serde_json` is a production
/// runtime dependency of this repository, so reaching for it inside an independence-critical probe
/// would put production code on the measurement path.
enum J {
    S(String),
    N(f64),
    B(bool),
    A(Vec<J>),
    O(Vec<(String, J)>),
}

// 🚫️async: pure formatting helper, no I/O.
fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

// 🚫️async: pure formatting helper, no I/O.
fn render(value: &J, out: &mut String) {
    match value {
        J::S(text) => {
            out.push('"');
            out.push_str(&esc(text));
            out.push('"');
        }
        J::N(number) => {
            use std::fmt::Write as _;
            if number.is_finite() {
                let _ = write!(out, "{number}");
            } else {
                out.push_str("null");
            }
        }
        J::B(flag) => out.push_str(if *flag { "true" } else { "false" }),
        J::A(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                render(item, out);
            }
            out.push(']');
        }
        J::O(fields) => {
            out.push('{');
            for (index, (key, item)) in fields.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(&esc(key));
                out.push_str("\":");
                render(item, out);
            }
            out.push('}');
        }
    }
}

// 🚫️async: pure formatting helper, no I/O.
fn emit(status: &str, measurements: J, diagnostics: Vec<(String, String, String)>) {
    let mut fields = vec![("status".to_string(), J::S(status.to_string())), ("measurements".to_string(), measurements)];
    if !diagnostics.is_empty() {
        fields.push((
            "diagnostics".to_string(),
            J::A(
                diagnostics
                    .into_iter()
                    .map(|(severity, message, detail)| J::O(vec![("severity".to_string(), J::S(severity)), ("message".to_string(), J::S(message)), ("detail".to_string(), J::S(detail))]))
                    .collect(),
            ),
        ));
    }
    let mut text = String::new();
    render(&J::O(fields), &mut text);
    println!("{text}");
}

// 🚫️async: pure formatting helper, no I/O.
fn unsupported(reason: &str) {
    emit("unsupported", J::O(vec![("reason".to_string(), J::S(reason.to_string()))]), Vec::new());
}

// 🚫️async: pure formatting helper, no I/O.
fn failed(message: &str, detail: &str) {
    emit("failed", J::O(Vec::new()), vec![("error".to_string(), message.to_string(), detail.to_string())]);
}
//#endregion 🧾️Json

//#region 📊️Csv
/// 📊️ One row as the `csv` crate recovered it. `x`/`y` are parsed as `f64` so a coordinate change is
/// measured as a numeric delta rather than a string inequality — the same reasoning
/// `semio@v1/drawing`'s own SVG comparator applies to path coordinates.
#[derive(Clone, Debug)]
struct MathRow {
    id: String,
    label: String,
    x: Option<f64>,
    y: Option<f64>,
    raw_x: String,
    raw_y: String,
}

// 🚫️async: pure parsing helper.
fn read_rows(path: &str) -> Result<Vec<MathRow>, String> {
    let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(path).map_err(|error| format!("{path}: {error}"))?;
    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|error| format!("{path}: {error}"))?;
        if record.len() < 4 {
            return Err(format!("{path}: row has {} field(s), expected 4 (id,label,x,y)", record.len()));
        }
        let raw_x = record.get(2).unwrap_or_default().to_string();
        let raw_y = record.get(3).unwrap_or_default().to_string();
        rows.push(MathRow { id: record.get(0).unwrap_or_default().to_string(), label: record.get(1).unwrap_or_default().to_string(), x: raw_x.parse::<f64>().ok(), y: raw_y.parse::<f64>().ok(), raw_x, raw_y });
    }
    Ok(rows)
}

/// ⚖️ The measured difference between two CSV carriers, all of it read out of the two files. This
/// grid has no tessellation freedom at all — `x`/`y` are the coordinates, not a re-sampleable curve —
/// so the gate is near-exact on the parsed numeric value, tolerant only of DECIMAL FORMATTING (`10`
/// against `10.0`), never of the value itself.
// 🚫️async: pure comparison over already-parsed values.
fn compare_rows(expected: &[MathRow], actual: &[MathRow]) -> J {
    let row_count_equal = expected.len() == actual.len();
    let mut max_coordinate_deviation = 0.0f64;
    let mut ids_equal = true;
    let mut labels_equal = true;
    let mut differing: Vec<J> = Vec::new();

    let paired = expected.len().min(actual.len());
    for index in 0..paired {
        let (before, after) = (&expected[index], &actual[index]);
        let mut reasons: Vec<String> = Vec::new();
        if before.id != after.id {
            ids_equal = false;
            reasons.push(format!("id differs: {:?} vs {:?}", before.id, after.id));
        }
        if before.label != after.label {
            labels_equal = false;
            reasons.push(format!("label differs: {:?} vs {:?}", before.label, after.label));
        }
        match (before.x, after.x) {
            (Some(bx), Some(ax)) => {
                let delta = (bx - ax).abs();
                if delta > max_coordinate_deviation {
                    max_coordinate_deviation = delta;
                }
                if delta > 0.0 {
                    reasons.push(format!("x differs by {delta}"));
                }
            }
            _ => reasons.push(format!("x not numeric on one side: {:?} vs {:?}", before.raw_x, after.raw_x)),
        }
        match (before.y, after.y) {
            (Some(by), Some(ay)) => {
                let delta = (by - ay).abs();
                if delta > max_coordinate_deviation {
                    max_coordinate_deviation = delta;
                }
                if delta > 0.0 {
                    reasons.push(format!("y differs by {delta}"));
                }
            }
            _ => reasons.push(format!("y not numeric on one side: {:?} vs {:?}", before.raw_y, after.raw_y)),
        }
        if !reasons.is_empty() {
            differing.push(J::O(vec![("rowIndex".to_string(), J::N(index as f64)), ("id".to_string(), J::S(before.id.clone())), ("reasons".to_string(), J::A(reasons.into_iter().map(J::S).collect()))]));
        }
    }

    let equal = row_count_equal && differing.is_empty();
    J::O(vec![
        ("equal".to_string(), J::B(equal)),
        ("rowCountEqual".to_string(), J::B(row_count_equal)),
        ("expectedRowCount".to_string(), J::N(expected.len() as f64)),
        ("actualRowCount".to_string(), J::N(actual.len() as f64)),
        ("idsEqual".to_string(), J::B(ids_equal)),
        ("labelsEqual".to_string(), J::B(labels_equal)),
        ("maxCoordinateDeviation".to_string(), J::N(max_coordinate_deviation)),
        ("differingRows".to_string(), J::A(differing)),
    ])
}
//#endregion 📊️Csv

//#region 🏭️Fixtures
/// 🏭️ Writes the four gate-validation inputs directly with the `csv` crate's own `Writer` — never by
/// calling this repository's `encode_csv`. `good-a`/`good-b` are the SAME five values written with
/// different but equally legal decimal formatting (`5` vs `5.0`); `bad-label`/`bad-coordinate` each
/// change exactly one cell.
fn write_row(writer: &mut csv::Writer<std::fs::File>, id: &str, label: &str, x: &str, y: &str) -> Result<(), String> {
    writer.write_record([id, label, x, y]).map_err(|error| error.to_string())
}

fn gate_inputs(out: &str) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(out).map_err(|error| format!("{out}: {error}"))?;
    let mut written = Vec::new();
    let make = |file: &str, rows: &[(&str, &str, &str, &str)]| -> Result<(), String> {
        let target = format!("{out}/{file}");
        let mut writer = csv::WriterBuilder::new().has_headers(false).terminator(csv::Terminator::Any(b'\n')).from_path(&target).map_err(|error| format!("{target}: {error}"))?;
        for (id, label, x, y) in rows {
            write_row(&mut writer, id, label, x, y)?;
        }
        writer.flush().map_err(|error| format!("{target}: {error}"))?;
        Ok(())
    };
    make("good-a.csv", &[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10", "5")])?;
    make("good-b.csv", &[("n1", "Alpha", "0.0", "0.0"), ("n2", "Beta", "10.00", "5.0")])?;
    make("bad-label.csv", &[("n1", "Alpha", "0", "0"), ("n2", "Betaa", "10", "5")])?;
    make("bad-coordinate.csv", &[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10.05", "5")])?;
    for name in ["good-a.csv", "good-b.csv", "bad-label.csv", "bad-coordinate.csv"] {
        written.push(name.to_string());
    }
    Ok(written)
}

/// 🏭️ Writes the five mutation-recipe BEFORE/AFTER pairs, one per witnessable kind, directly with the
/// `csv` crate — this is what a correct `EquationIntoCsv` export SHOULD contain before and after
/// each mutation, authored independently of that serializer and written through a different writer.
fn fixtures(out: &str) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(out).map_err(|error| format!("{out}: {error}"))?;
    let mut written = Vec::new();
    let recipe = |dir: &str, before: &[(&str, &str, &str, &str)], after: Option<&[(&str, &str, &str, &str)]>| -> Result<(), String> {
        let recipe_dir = format!("{out}/{dir}");
        std::fs::create_dir_all(&recipe_dir).map_err(|error| format!("{recipe_dir}: {error}"))?;
        let write = |name: &str, rows: &[(&str, &str, &str, &str)]| -> Result<(), String> {
            let target = format!("{recipe_dir}/{name}");
            let mut writer = csv::WriterBuilder::new().has_headers(false).terminator(csv::Terminator::Any(b'\n')).from_path(&target).map_err(|error| format!("{target}: {error}"))?;
            for (id, label, x, y) in rows {
                write_row(&mut writer, id, label, x, y)?;
            }
            writer.flush().map_err(|error| format!("{target}: {error}"))?;
            Ok(())
        };
        write("before.csv", before)?;
        if let Some(after) = after {
            write("after.csv", after)?;
        }
        Ok(())
    };

    recipe(
        "create-node-adds-a-row",
        &[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10", "5")],
        Some(&[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10", "5"), ("n3", "Gamma", "20", "15")]),
    )?;
    recipe(
        "delete-node-removes-a-row",
        &[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10", "5"), ("n3", "Gamma", "20", "15")],
        Some(&[("n1", "Alpha", "0", "0"), ("n3", "Gamma", "20", "15")]),
    )?;
    recipe(
        "delete-nodes-removes-two-rows",
        &[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10", "5"), ("n3", "Gamma", "20", "15"), ("n4", "Delta", "30", "25")],
        Some(&[("n1", "Alpha", "0", "0"), ("n4", "Delta", "30", "25")]),
    )?;
    recipe(
        "change-node-label-rewrites-the-quoted-cell",
        &[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10", "5")],
        Some(&[("n1", "Alpha", "0", "0"), ("n2", "Beta \"B\" Renamed", "10", "5")]),
    )?;
    recipe(
        "move-node-rewrites-the-coordinate-cells",
        &[("n1", "Alpha", "0", "0"), ("n2", "Beta", "10", "5")],
        Some(&[("n1", "Alpha", "0", "0"), ("n2", "Beta", "12.5", "7.25")]),
    )?;

    for name in [
        "🟢️create-node-adds-a-row/⬅️before.csv",
        "🟢️create-node-adds-a-row/➡️after.csv",
        "❌️delete-node-removes-a-row/⬅️before.csv",
        "❌️delete-node-removes-a-row/➡️after.csv",
        "🗑️delete-nodes-removes-two-rows/⬅️before.csv",
        "🗑️delete-nodes-removes-two-rows/➡️after.csv",
        "🏷️change-node-label-rewrites-the-quoted-cell/⬅️before.csv",
        "🏷️change-node-label-rewrites-the-quoted-cell/➡️after.csv",
        "🕹️move-node-rewrites-the-coordinate-cells/⬅️before.csv",
        "🕹️move-node-rewrites-the-coordinate-cells/➡️after.csv",
    ] {
        written.push(name.to_string());
    }
    Ok(written)
}
//#endregion 🏭️Fixtures

//#region 🚀️Entry
fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let probe = argv.first().cloned().unwrap_or_default();
    let mut inputs: Vec<String> = Vec::new();
    let mut out = String::new();
    let mut at = 1usize;
    while at < argv.len() {
        if argv[at] == "--input" && at + 1 < argv.len() {
            inputs.push(argv[at + 1].clone());
            at += 2;
        } else if argv[at] == "--out" && at + 1 < argv.len() {
            out = argv[at + 1].clone();
            at += 2;
        } else {
            at += 1;
        }
    }
    let extension = |path: &str| path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    let need = |count: usize| -> bool {
        if inputs.len() < count {
            failed("missing inputs", &format!("{probe} needs {count} --input path(s), got {}", inputs.len()));
            false
        } else {
            true
        }
    };

    match probe.as_str() {
        "csv-rows" => {
            if !need(1) {
                return;
            }
            if extension(&inputs[0]) != "csv" {
                return unsupported(&format!(".{} is not a CSV carrier", extension(&inputs[0])));
            }
            match read_rows(&inputs[0]) {
                Err(error) => failed("csv parse failed", &error),
                Ok(rows) => {
                    let listed: Vec<J> = rows
                        .iter()
                        .map(|row| {
                            J::O(vec![
                                ("id".to_string(), J::S(row.id.clone())),
                                ("label".to_string(), J::S(row.label.clone())),
                                ("x".to_string(), row.x.map(J::N).unwrap_or(J::S(row.raw_x.clone()))),
                                ("y".to_string(), row.y.map(J::N).unwrap_or(J::S(row.raw_y.clone()))),
                            ])
                        })
                        .collect();
                    emit("ok", J::O(vec![("parsed".to_string(), J::B(true)), ("rowCount".to_string(), J::N(rows.len() as f64)), ("rows".to_string(), J::A(listed))]), Vec::new());
                }
            }
        }
        "csv-compare" => {
            if !need(2) {
                return;
            }
            for input in inputs.iter().take(2) {
                if extension(input) != "csv" {
                    // ✘️CSV is the only exported carrier of this subset. `edges`, `directed`,
                    // `algorithm`, point-cloud geometry and the `equation` AST have no column here at
                    // all — answering `ok` with an empty row set would let a mutation to one of those
                    // fields pass against evidence that was never in the file.
                    return unsupported(&format!(".{} is not a CSV carrier", extension(input)));
                }
            }
            match (read_rows(&inputs[0]), read_rows(&inputs[1])) {
                (Err(error), _) | (_, Err(error)) => failed("csv parse failed", &error),
                (Ok(expected), Ok(actual)) => emit("ok", compare_rows(&expected, &actual), Vec::new()),
            }
        }
        "gate-inputs" => {
            if out.is_empty() {
                return failed("missing --out", "gate-inputs needs --out <dir>");
            }
            match gate_inputs(&out) {
                Err(error) => failed("gate input generation failed", &error),
                Ok(written) => emit("ok", J::O(vec![("written".to_string(), J::A(written.into_iter().map(J::S).collect())), ("out".to_string(), J::S(out))]), Vec::new()),
            }
        }
        "fixtures" => {
            if out.is_empty() {
                return failed("missing --out", "fixtures needs --out <dir>");
            }
            match fixtures(&out) {
                Err(error) => failed("fixture generation failed", &error),
                Ok(written) => emit("ok", J::O(vec![("written".to_string(), J::A(written.into_iter().map(J::S).collect())), ("out".to_string(), J::S(out))]), Vec::new()),
            }
        }
        other => failed("unknown probe", &format!("{other:?} — known: csv-rows, csv-compare, gate-inputs, fixtures")),
    }
}
//#endregion 🚀️Entry
