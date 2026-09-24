"""Keyword-table codecs (tag = index into OP_KEYWORDS/KINDS): tags move onto 📡️.protocol.semio records."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from wire import const_name, protocol_include, render
root = "/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
survey = json.load(open(root + ".tmp-ticket/wp-t5/generated/survey.json"))

def kebab(name):
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "-", name).lower()

def span(text, fn):
    m = re.search(r"fn " + fn + r"\s*\([^)]*\)[^{]*\{", text)
    depth, i = 1, m.end()
    while depth and i < len(text):
        depth += {"{": 1, "}": -1}.get(text[i], 0); i += 1
    return (m.start(), i)

for o in survey:
    if not o["impl"] or not o["kinds"]: continue
    path = root + o["impl"][0]
    text = open(path, encoding="utf-8").read()
    lookup = re.search(r"let keyword = (OP_KEYWORDS|KINDS)\.get\(tag as usize\)\.ok_or_else\(\|\| protocol::ProtocolError::Malformed \{ what: \"op tag\", offset: 1, detail: format!\(\"tag \{tag\} out of range for \{\} declared variants\", (?:OP_KEYWORDS|KINDS)\.len\(\)\) \}\)\?;", text)
    if not lookup: continue
    table_name = lookup.group(1)
    enum = re.search(r"impl\s+(?:protocol::)?OpBinary\s+for\s+(\w+)", text).group(1)
    src = text if table_name == "OP_KEYWORDS" else open(root + o["impl"][0], encoding="utf-8").read()
    table_m = re.search(r"(?:pub )?const " + table_name + r": (?:\[&str; \d+\]|&\[&str\]) = &?\[(.*?)\];", src, re.S)
    table = re.findall(r'"([A-Za-z0-9-]+)"', table_m.group(1))
    helper = re.search(r"fn (\w+)\(\w+: &" + enum + r"\) -> u8", text).group(1)
    hs, he = span(text, helper)
    arms = [(m.start(2) , m.end(2), m.group(1), int(m.group(2))) for m in re.finditer(enum + r"::(\w+)\([^)]*\)\s*=>\s*(\d+)", text[hs:he])]
    problems = []
    tags = {}
    for a, b, variant, tag in arms:
        k = kebab(variant)
        if tag >= len(table): problems.append(f"{variant}={tag} outside the table")
        tags[k] = tag
    if set(tags) != set(o["kinds"]): problems.append(f"kinds mismatch {sorted(set(o['kinds']) ^ set(tags))}")
    by_tag = {t: k for k, t in tags.items()}
    text_keywords = [(by_tag[i], table[i]) for i in range(len(table)) if i in by_tag]
    aliased = any(k != w for k, w in text_keywords)
    print(len(tags), enum, table_name, "aliased" if aliased else "", "OK" if not problems else problems[:3])
    if problems or not apply: continue
    for a, b, variant, tag in sorted(arms, reverse=True):
        text = text[:hs + a] + const_name(kebab(variant)) + text[hs + b:]
    if aliased:
        assert table_name == "OP_KEYWORDS"
        lookup_line = 'let kind = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;\n        let keyword = TEXT_KEYWORDS.iter().find(|(record, _)| *record == kind).map(|(_, keyword)| *keyword).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("record {kind} has no text keyword") })?;'
        text = text.replace(lookup.group(0), lookup_line)
    text = text.replace(lookup.group(0), 'let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;')
    if table_name == "OP_KEYWORDS":
        full = re.search(r"(?:///[^\n]*\n|//[^\n]*\n)*const OP_KEYWORDS: \[&str; \d+\] = \[.*?\];\n", text, re.S)
        replacement = ""
        if aliased:
            replacement = "/// 🧾️ Each record kind's text-grammar keyword, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.\n"
            replacement += f"const TEXT_KEYWORDS: [(&str, &str); {len(text_keywords)}] = [\n" + "".join(f'    ("{k}", "{w}"),\n' for k, w in text_keywords) + "];\n"
        text = text[:full.start()] + replacement + text[full.end():]
        text = text.replace("pub const KINDS: &[&str] = &OP_KEYWORDS;", "pub const KINDS: &[&str] = &[" + ", ".join(f'"{k}"' for k, _ in sorted(tags.items(), key=lambda kv: kv[1])) + "];")
        assert "OP_KEYWORDS" not in text.replace("[`OP_KEYWORDS`]", ""), path
    proto = root + o["proto"]
    include = "COMPONENT_PROTOCOL_SEMIO" if os.path.dirname(path) == os.path.dirname(proto) else protocol_include(path, proto)
    consts = ["//#region 🏷️WireTags", f"/// 🏷️ Op tags of `{enum}`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.", f"const WIRE_PROTOCOL: &str = {include};"]
    consts += [f'const {const_name(k)}: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "{k}");' for k, _ in sorted(tags.items(), key=lambda kv: kv[1])]
    consts += ["//#endregion 🏷️WireTags", ""]
    m = re.search(r"(?m)^(?:///[^\n]*\n|// 🚫️async[^\n]*\n)*fn " + helper + r"\(", text)
    text = text[:m.start()] + "\n".join(consts) + "\n" + text[m.start():]
    open(path, "w", encoding="utf-8").write(text)
    note = [f"Real `{enum}` op frame: `format u8` (`OP_BINARY_FORMAT`) then `tag u8`, then the kind's `key=value ...` text arguments.",
            "Each record is one mutation kind at its wire tag. This file is the only source of those tags: the codec",
            "derives every tag from these records (`dsl::protocol_record`), and decodes a tag to its record's kind."]
    render(proto, [("format", "u8"), ("tag", "u8")], sorted(tags.items(), key=lambda kv: kv[1]), note)
