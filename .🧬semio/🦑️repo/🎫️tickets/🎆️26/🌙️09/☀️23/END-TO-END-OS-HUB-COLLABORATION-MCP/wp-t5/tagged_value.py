"""Text/JSON-as-binary codecs: move onto the real tagged frame (`dsl::tagged_value_binary`) with 📡️.protocol.semio records."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from wire import render
from variants import enum_variants, kebab
root = "/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
names = [a for a in sys.argv[1:] if not a.startswith("--")]
survey = json.load(open(root + ".tmp-ticket/wp-t5/generated/survey.json"))

def fn_span(text, start, fn):
    m = re.compile(r"fn " + fn + r"\s*\([^)]*\)[^{]*\{").search(text, start)
    depth, i = 1, m.end()
    while depth:
        depth += {"{": 1, "}": -1}.get(text[i], 0); i += 1
    return m.end(), i - 1

for o in survey:
    if not any("/" + n + "/" in o["proto"] for n in names) or not o["impl"]: continue
    path = root + o["impl"][0]; proto = root + o["proto"]
    text = open(path, encoding="utf-8").read()
    impl = re.search(r"impl\s+(?:protocol::)?OpBinary\s+for\s+(\w+)\s*\{", text)
    enum = impl.group(1)
    src = next(open(f, encoding="utf-8").read() for f in [path] + [os.path.join(dp, f) for dp, _, fs in os.walk(os.path.dirname(os.path.dirname(proto))) for f in fs if f == "🦀️.rs"] if re.search(r"pub enum " + enum + r"\b", open(f, encoding="utf-8").read()))
    attrs = src[max(0, src.index("pub enum " + enum) - 400):src.index("pub enum " + enum)]
    tagged = re.search(r'#\[value\([^\]]*tag = "(\w+)"', attrs)
    order = [kebab(v) for v, _ in enum_variants(src, enum)]
    vocab = os.path.dirname(os.path.dirname(proto)); kinds = set(o["kinds"])
    described = set()
    for d in os.listdir(vocab):
        for leaf in [f"{vocab}/{d}"] + ([f"{vocab}/{d}/{c}" for c in os.listdir(f"{vocab}/{d}")] if os.path.isdir(f"{vocab}/{d}") else []):
            f = leaf + "/🔣️.json"
            if os.path.isfile(f):
                try: j = json.load(open(f, encoding="utf-8"))
                except Exception: continue
                if isinstance(j, dict) and j.get("schemaVersion") == 1 and "semanticKind" in j: described.add(j["semanticKind"])
    if described: o["kinds"] = sorted(described)
    problems = [] if set(order) == set(o["kinds"]) else [f"kinds≠variants -{sorted(set(o['kinds']) - set(order))} +{sorted(set(order) - set(o['kinds']))}"]
    tagging = f'dsl::tagged_value_binary::VariantTag::Field("{tagged.group(1)}")' if tagged else "dsl::tagged_value_binary::VariantTag::Key"
    print(len(order), enum, tagging.split("::")[-1], "OK" if not problems else problems)
    if problems or not apply: continue
    rel = os.path.relpath(proto, os.path.dirname(path))
    lit = "COMPONENT_PROTOCOL_SEMIO" if os.path.dirname(path) == os.path.dirname(proto) else f'include_str!("{rel}")'
    es, ee = fn_span(text, impl.end(), "encode_op")
    ds, de = fn_span(text, impl.end(), "decode_op")
    text = text[:ds] + f"\n        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, {tagging}, bytes)\n    " + text[de:]
    text = text[:es] + f"\n        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, {tagging}, self)\n    " + text[ee:]
    head = re.search(r"(?m)^(?:///[^\n]*\n)*impl\s+(?:protocol::)?OpBinary\s+for\s+" + enum, text)
    block = f"//#region 🏷️WireTags\n/// 🏷️ `{enum}`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.\nconst WIRE_PROTOCOL: &str = {lit};\n//#endregion 🏷️WireTags\n\n"
    text = text[:head.start()] + block + text[head.start():]
    open(path, "w", encoding="utf-8").write(text)
    note = [f"Real `{enum}` op frame (`dsl::tagged_value_binary`): `format u8` (`OP_BINARY_FORMAT`) then `tag varint`, then",
            "the variant's `ToValue` tree without its variant name, as one framework wire value (`pack_rt::encode_wire_value`).",
            "Each record is one mutation kind at its wire tag; this file is the only source of those tags."]
    render(proto, [("format", "u8"), ("tag", "varint")], [(k, i) for i, k in enumerate(order)], note)
