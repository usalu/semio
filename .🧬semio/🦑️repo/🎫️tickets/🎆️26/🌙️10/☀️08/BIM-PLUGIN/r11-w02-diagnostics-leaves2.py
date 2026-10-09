import sys
S = sys.argv[1]
def patch(path, pairs):
    s = open(path, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (path, old, s.count(old))
        s = s.replace(old, new)
    open(path + ".new", 'w', encoding='utf-8', newline='').write(s)
patch(S + "/🚪️io/📤️export/📊️csv/🦀️.rs", [
("""/// 📋️ The report of a model:""", """/// ⚖️ The canonical JSON table `"<position>|<slug>|<elements>" → { severity, storey, missing, message_en, message_de }` of a diagnostics table read back with the decoder: the table the third-party oracle reads from the same file.
pub fn findings_json(text: &str) -> String {
    use std::collections::BTreeMap;
    let records = codec::read_records(text);
    let table: BTreeMap<String, BTreeMap<&str, &str>> = records
        .iter()
        .skip(1)
        .enumerate()
        .map(|(position, record)| (format!("{position:04}|{}|{}", record[1], record[3]), BTreeMap::from([("severity", record[0].as_str()), ("storey", record[2].as_str()), ("missing", record[4].as_str()), ("message_en", record[5].as_str()), ("message_de", record[6].as_str())])))
        .collect();
    semio_framework_pack_json::to_json_string(&table)
}

/// 📋️ The report of a model:"""),
])
