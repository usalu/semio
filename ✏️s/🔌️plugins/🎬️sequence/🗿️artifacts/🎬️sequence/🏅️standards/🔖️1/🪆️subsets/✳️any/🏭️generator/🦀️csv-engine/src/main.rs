//! 📊️ Third-party CSV fixture generator for `s.sequence.sequence@1/✳️any`.
//!
//! Row shape mirrors `SequenceIntoCsv` exactly — `has_header: false`, one record per step,
//! `[id, kind, JSON-encoded params]` with the params column quoted. Every fixture is an authored
//! `(before.csv, after.csv)` PAIR for exactly one mutation kind: the pair IS the expectation, the
//! `csv` crate writes it and `csv-rfc4180-reader` re-derives the row set from the bytes. Nothing here
//! applies one of our mutations, which is what keeps this a READER oracle and not a predicting one.
//!
//! Only the four ROW-LEVEL kinds are covered. `connect-steps`, `disconnect-steps`, `move-step` and
//! `change-step-collapsed` are NOT: the serializer never writes `edges` (a flat grid has no edge
//! concept) and the hop is declared `IoFidelity::Lossy`, so no carrier records them. A reader cannot
//! witness what no carrier records.

type Row = (&'static str, &'static str, &'static str);

fn base() -> Vec<Row> {
    vec![
        ("step-0", "source", r#"{"path":"in.csv"}"#),
        ("step-1", "filter", r#"{"where":"x > 0"}"#),
        ("step-2", "sink", r#"{"path":"out.csv"}"#),
    ]
}

/// 🧪️ `(kind, before, after)` — one authored pair per row-level mutation kind.
fn corpus() -> Vec<(&'static str, Vec<Row>, Vec<Row>)> {
    let mut created = base();
    created.push(("step-3", "map", r#"{"expr":"x * 2"}"#));

    let mut deleted = base();
    deleted.remove(1);

    let mut duplicated = base();
    duplicated.insert(2, ("step-1-copy", "filter", r#"{"where":"x > 0"}"#));

    let mut edited = base();
    edited[1] = ("step-1", "filter", r#"{"where":"x > 10"}"#);

    vec![
        ("create-step", base(), created),
        ("delete-step", base(), deleted),
        ("duplicate-step", base(), duplicated),
        ("edit-step-params", base(), edited),
    ]
}

fn render(rows: &[Row]) -> Vec<u8> {
    let mut writer = csv::WriterBuilder::new().has_headers(false).quote_style(csv::QuoteStyle::Necessary).from_writer(Vec::new());
    for (id, kind, params) in rows {
        writer.write_record([*id, *kind, *params]).expect("write record");
    }
    writer.into_inner().expect("flush")
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| { eprintln!("usage: generate <out-dir>"); std::process::exit(2) });
    let mut written = 0usize;
    for (kind, before, after) in corpus() {
        assert_ne!(before, after, "{kind}: a fixture pair whose halves are equal proves nothing");
        let dir = std::path::Path::new(&out).join(kind);
        std::fs::create_dir_all(&dir).expect("fixture directory");
        for (name, rows) in [("before.csv", &before), ("after.csv", &after)] {
            std::fs::write(dir.join(name), render(rows)).expect("write");
            written += 1;
        }
        println!("{kind}");
    }
    eprintln!("{written} file(s)");
}
