"""Moves every hand-written op tag literal of a codec onto consts derived from its 📡️.protocol.semio records."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from wire import const_name, protocol_include, render
root = "/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith("--")]
survey = json.load(open(root + ".tmp-ticket/wp-t5/generated/survey.json"))

def kebab(name):
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "-", name).lower()

def span(text, fn):
    m = re.search(r"fn " + fn + r"\s*\([^)]*\)[^{]*\{", text)
    if not m: return None
    depth, i = 1, m.end()
    while depth and i < len(text):
        depth += {"{": 1, "}": -1}.get(text[i], 0); i += 1
    return (m.end(), i)

ENC = [r"=>\s*(\d+)\s*,", r"(?:write_u8|push)\(\s*(\d+)\s*\)", r"OP_BINARY_FORMAT,\s*(\d+)\s*\]", r"(?m)^\s*(\d+)\s*$", r"=>\s*(\d+)\s*$"]

def encode_edits(text, s, e, enum):
    edits, seg_text = {}, text[s:e]
    hits = [(m.start(), m.group(1)) for m in re.finditer(r"\b" + enum + r"::(\w+)", seg_text)]
    for idx, (pos, variant) in enumerate(hits):
        if variant in edits: continue
        end = hits[idx + 1][0] if idx + 1 < len(hits) else len(seg_text)
        seg = seg_text[pos:end]
        for pattern in ENC:
            m = re.search(pattern, seg)
            if m:
                edits[variant] = (s + pos + m.start(1), s + pos + m.end(1), int(m.group(1)))
                break
    return edits

def decode_edits(text, s, e, enum):
    seg_text = text[s:e]
    marks = [(m.start(1), m.end(1), int(m.group(1)), len(m.group(0)) - len(m.group(0).lstrip())) for m in re.finditer(r"(?m)^[ \t]*(\d+)\s*=>", seg_text)]
    if not marks: return {}
    indent = marks[0][3]
    marks = [mk for mk in marks if mk[3] == indent]
    edits = {}
    for idx, (a, b, tag, _) in enumerate(marks):
        end = marks[idx + 1][0] if idx + 1 < len(marks) else len(seg_text)
        found = re.search(r"\b" + enum + r"::(\w+)", seg_text[a:end])
        if found and found.group(1) not in edits: edits[found.group(1)] = (s + a, s + b, tag)
    return edits

report = []
for o in survey:
    if o["shape"] != ["literal"] or not o["impl"] or not o["kinds"]: continue
    if only and not any(x in o["proto"] for x in only): continue
    path = root + o["impl"][0]
    text = open(path, encoding="utf-8").read()
    enum = re.search(r"impl\s+(?:protocol::|store::)?OpBinary\s+for\s+(\w+)", text).group(1)
    es, ee = span(text, "encode_op"); ds, de = span(text, "decode_op")
    enc, dec = encode_edits(text, es, ee, enum), decode_edits(text, ds, de, enum)
    helper = re.search(r"fn (\w+)\(\w+: &" + enum + r"\) -> u8", text)
    if not enc and helper:
        hs, he = span(text, helper.group(1)); enc = encode_edits(text, hs, he, enum)
    problems = []
    tags = {}
    for v in sorted(set(enc) | set(dec)):
        values = {x[2] for x in (enc.get(v), dec.get(v)) if x}
        if len(values) != 1: problems.append(f"{v}: encode/decode disagree {values}")
        if v not in enc: problems.append(f"{v}: no encode tag")
        if v not in dec: problems.append(f"{v}: no decode tag")
        tags[kebab(v)] = min(values)
    kinds = set(o["kinds"])
    if set(tags) != kinds: problems.append(f"kinds {sorted(kinds - set(tags))} untagged, {sorted(set(tags) - kinds)} not kinds")
    if len(set(tags.values())) != len(tags): problems.append("duplicate tags")
    clash = [n for n in ["WIRE_PROTOCOL"] + [const_name(k) for k in tags] if re.search(r"\b" + n + r"\b", text)]
    if clash: problems.append(f"name clash {clash[:3]}")
    report.append((o["proto"].split("🗿️artifacts/")[-1][:60], enum, len(tags), problems))
    if problems or not apply: continue
    edits = sorted([(a, b, const_name(kebab(v))) for v, (a, b, _) in enc.items()] + [(a, b, const_name(kebab(v))) for v, (a, b, _) in dec.items()], reverse=True)
    for a, b, name in edits:
        text = text[:a] + name + text[b:]
    proto = root + o["proto"]
    include = "COMPONENT_PROTOCOL_SEMIO" if os.path.dirname(path) == os.path.dirname(proto) else protocol_include(path, proto)
    consts = ["//#region 🏷️WireTags", f"/// 🏷️ Op tags of `{enum}`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.", f"const WIRE_PROTOCOL: &str = {include};"]
    consts += [f'const {const_name(k)}: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "{k}");' for k, _ in sorted(tags.items(), key=lambda kv: kv[1])]
    consts += ["//#endregion 🏷️WireTags", ""]
    m = re.search(r"(?m)^(?:///[^\n]*\n)*impl\s+(?:protocol::|store::)?OpBinary\s+for", text)
    text = text[:m.start()] + "\n".join(consts) + "\n" + text[m.start():]
    open(path, "w", encoding="utf-8").write(text)
    enc_text = text[span(text, "encode_op")[0]:span(text, "encode_op")[1]]
    header = [("format", "u8"), ("tag", "u8")] if "OP_BINARY_FORMAT" in enc_text or "FORMAT" in enc_text else [("tag", "u8")]
    note = [f"Real `{enum}` op frame: " + ("`format u8` (`OP_BINARY_FORMAT`) then " if len(header) == 2 else "") + "`tag u8`, then the kind's own payload.",
            "Each record is one mutation kind at its wire tag. This file is the only source of those tags: the codec",
            "derives every tag from these records at compile time (`dsl::protocol_record::tag_u8`)."]
    declared = dict(re.findall(r"(?m)^\s*record\s+([a-z][a-z0-9-]*)\s+tag=(\d+)", open(proto, encoding="utf-8").read()))
    if {k: str(v) for k, v in tags.items()} != declared:
        render(proto, header, sorted(tags.items(), key=lambda kv: kv[1]), note)
for r in report: print(r[2], r[1], r[0], "OK" if not r[3] else r[3])
