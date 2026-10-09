import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
rep('''    let kind = kind_holding(snapshot, id).map_or("wall", |row| row.kind);
    let name = kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id)).unwrap_or_else(|| id.to_string());
    DslValue::object([''', '''    let holder = kind_holding(snapshot, id);
    let name = holder.and_then(|row| (row.name)(snapshot, id)).unwrap_or_else(|| id.to_string());
    let granularity = holder.map(|row| ("interactionGranularityId".to_string(), DslValue::String(row.kind.to_string())));
    DslValue::object([''')
rep('''        ("interactionId".to_string(), DslValue::String(id.to_string())),
        ("interactionGranularityId".to_string(), DslValue::String(kind.to_string())),
    ])''', '''        ("interactionId".to_string(), DslValue::String(id.to_string())),
    ].into_iter().chain(granularity))''')
open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
