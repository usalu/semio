//! One-off derivation of the real XLSX ECMA-376 mutation fixture. Reads the real 50-row, 12-column
//! European building-component reuse-marketplace survey (already committed as
//! ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv, itself derived from
//! ♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex) with the `csv` reader and
//! writes a genuine two-sheet ECMA-376 workbook with the `rust_xlsxwriter` reference writer: sheet 1
//! is the real table verbatim, sheet 2 is a real per-country tally computed from column 3 (`Land`) of
//! the same real data. Repeated real values (country names, access categories, platform channels)
//! naturally deduplicate into `rust_xlsxwriter`'s own shared-string table, so the committed fixture
//! carries a genuine multi-entry SST built from real content, not synthesised strings.

use rust_xlsxwriter::Workbook;
use std::collections::BTreeMap;

fn main() {
    let src = std::env::args().nth(1).expect("usage: xlsx-fixture-derive <source.csv> <dest.xlsx>");
    let dst = std::env::args().nth(2).expect("usage: xlsx-fixture-derive <source.csv> <dest.xlsx>");

    let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(&src).expect("open source csv");
    let records: Vec<Vec<String>> = reader.records().map(|r| r.expect("read csv record").iter().map(|f| f.to_string()).collect()).collect();
    assert!(records.len() > 1, "expected a header row plus real data rows");
    let header = &records[0];
    let data = &records[1..];
    eprintln!("source: {} header field(s), {} real data row(s)", header.len(), data.len());

    let land_col = header.iter().position(|h| h == "Land").expect("csv must carry a 'Land' column");

    let mut workbook = Workbook::new();

    // 🌱 Sheet 1 — the real 50x12 survey table, verbatim.
    let marketplaces = workbook.add_worksheet();
    marketplaces.set_name("Marktplätze").expect("set sheet 1 name");
    for (col, field) in header.iter().enumerate() {
        marketplaces.write_string(0, col as u16, field.as_str()).expect("write header cell");
    }
    for (row_idx, record) in data.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        for (col, field) in record.iter().enumerate() {
            marketplaces.write_string(row, col as u16, field.as_str()).expect("write data cell");
        }
    }

    // 🌱 Sheet 2 — a real per-country tally derived from column `Land` of the same real rows,
    // first-appearance order (never fabricated: every count is a genuine occurrence tally).
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    for record in data {
        let land = record.get(land_col).cloned().unwrap_or_default();
        if !counts.contains_key(&land) {
            order.push(land.clone());
        }
        *counts.entry(land).or_insert(0) += 1;
    }
    let countries = workbook.add_worksheet();
    countries.set_name("Länderübersicht").expect("set sheet 2 name");
    countries.write_string(0, 0, "Land").expect("write countries header 0");
    countries.write_string(0, 1, "Anzahl").expect("write countries header 1");
    for (row_idx, land) in order.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        countries.write_string(row, 0, land.as_str()).expect("write country name");
        countries.write_number(row, 1, f64::from(counts[land])).expect("write country count");
    }
    eprintln!("derived: {} distinct countr(y/ies) from the real 'Land' column", order.len());

    workbook.save(&dst).expect("save derived xlsx");
    let written = std::fs::metadata(&dst).expect("stat dest").len();
    eprintln!("wrote {written} bytes to {dst}");
}
