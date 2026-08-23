//! Derives the IANA TSV form of the committed real CSV fixture
//! (✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv) using the `csv`
//! crate as the reader (same convention as this repo's own csv oracle: has_headers(false),
//! flexible(true), so the header row survives as a plain record and per-record field counts stay
//! real information). Output is a byte-exact tab-join of the same real cell values with LF line
//! endings and a trailing newline. Since IANA text/tab-separated-values has no quoting mechanism,
//! any field containing a literal tab or newline is unrepresentable; this program scans for both
//! and reports what it finds before writing anything.

use std::env;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let input = args.next().expect("usage: tsv-iana-any-fixture-gen <input.csv> <output.tsv>");
    let output = args.next().expect("usage: tsv-iana-any-fixture-gen <input.csv> <output.tsv>");

    let bytes = fs::read(&input).expect("read input csv");
    let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_reader(bytes.as_slice());
    let records: Vec<Vec<String>> = reader.records().map(|result| result.expect("read csv record").iter().map(|cell| cell.to_string()).collect()).collect();

    println!("read {} record(s) from {}", records.len(), input);

    let mut tab_offenders = 0usize;
    let mut newline_offenders = 0usize;
    for (row_index, row) in records.iter().enumerate() {
        for (field_index, cell) in row.iter().enumerate() {
            if cell.contains('\t') {
                tab_offenders += 1;
                println!("  [TAB]     record {row_index} field {field_index}: {cell:?}");
            }
            if cell.contains('\n') || cell.contains('\r') {
                newline_offenders += 1;
                println!("  [NEWLINE] record {row_index} field {field_index}: {cell:?}");
            }
        }
    }
    println!("tab-bearing fields: {tab_offenders}, newline-bearing fields: {newline_offenders}");
    if tab_offenders > 0 || newline_offenders > 0 {
        println!("NOTE: IANA TSV has no quoting mechanism; the offending byte(s) cannot be represented and would need a deliberate policy (e.g. stripped/substituted) before writing.");
    } else {
        println!("no tab or newline bytes found in any field — every cell is representable verbatim as TSV.");
    }

    let column_count = records.first().map(|row| row.len()).unwrap_or(0);
    for (row_index, row) in records.iter().enumerate() {
        assert_eq!(row.len(), column_count, "record {row_index} has {} fields, expected {column_count} (ragged input)", row.len());
    }

    // 📤️ Written back out through the SAME `csv` crate, reconfigured for IANA TSV: tab delimiter,
    // LF terminator, and quoting disabled entirely (`QuoteStyle::Never`) — TSV has no quoting
    // mechanism, so the writer must never invent one, matching the byte-exact split/rejoin this
    // repository's own `encode_tsv` performs.
    let mut writer = csv::WriterBuilder::new().delimiter(b'\t').terminator(csv::Terminator::Any(b'\n')).quote_style(csv::QuoteStyle::Never).from_writer(Vec::new());
    for record in &records {
        writer.write_record(record).expect("write tsv record");
    }
    let bytes = writer.into_inner().expect("finish tsv writer");

    fs::write(&output, &bytes).expect("write output tsv");
    println!("wrote {} bytes ({} record(s) x {} column(s)) to {}", bytes.len(), records.len(), column_count, output);
}
