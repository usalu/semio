"""variants_binary codecs: records = the DslVariants keyword order (the ordinal the wire carried); codecs move to encode_tagged_op."""
import json, os, re, sys, glob
sys.path.insert(0, os.path.dirname(__file__))
from wire import render
root = "/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith("--")]
survey = json.load(open(root + ".tmp-ticket/wp-t5/generated/survey.json"))

def kebab(name):
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "-", name).lower()

def enum_variants(src, name):
    m = re.search(r"(?:pub(?:\([^)]*\))? )?enum " + name + r"\b[^{]*\{", src)
    if not m: return None
    depth, i = 1, m.end()
    while depth and i < len(src):
        depth += {"{": 1, "}": -1}.get(src[i], 0); i += 1
    body = src[m.end():i - 1]
    out, depth, j, attrs = [], 0, 0, []
    for line in body.splitlines():
        s = line.strip()
        if s.startswith("//"): continue
        if depth == 0:
            if s.startswith("#[dsl("):
                attrs.append(s)
            else:
                v = re.match(r"([A-Z]\w*)\s*[({,]?", s)
                if v and not s.startswith("//"):
                    key = None
                    for a in attrs:
                        k = re.search(r'(?:key|keyword)\s*=\s*"([^"]+)"', a)
                        if k: key = k.group(1)
                    out.append((v.group(1), key or kebab(v.group(1))))
                    attrs = []
        depth += s.count("{") + s.count("(") - s.count("}") - s.count(")")
    return out

def crate_sources(path):
    d = os.path.dirname(path)
    while d.startswith(root) and not os.path.exists(d + "/📦️packages/🦀️rust/Cargo.toml"): d = os.path.dirname(d)
    return glob.glob(d + "/**/*.rs", recursive=True)

rows = []
for o in survey:
    if "variants" not in o["shape"] or not o["kinds"]: continue
    if only and not any(x in o["proto"] for x in only): continue
    proto = root + o["proto"]
    files = [root + f for f in o["impl"]]
    calls = []
    for f in files:
        t = open(f, encoding="utf-8").read()
        for m in re.finditer(r"impl\s+(?:protocol::)?OpBinary\s+for\s+(\w+)\s*\{", t):
            seg = t[m.end():m.end() + 800]
            c = re.search(r"variants_binary::encode_op\((&?[\w.()]+)\)", seg)
            if c: calls.append((f, m.group(1), c.group(1)))
    types = {ty for _, ty, arg in calls if arg in ("self", "&self")}
    problems, order = [], None
    if len(types) != 1: problems.append(f"types {sorted(types)}")
    else:
        ty = next(iter(types))
        for src in [files[0]] + crate_sources(files[0]):
            t = open(src, encoding="utf-8").read()
            if re.search(r"derive\([^)]*(?:DslOps|DslEnum)[^)]*\)\]\s*(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))? )?enum " + ty + r"\b", t):
                order = enum_variants(t, ty); break
        if not order: problems.append(f"enum {ty} not found")
    if order:
        keys = [k for _, k in order]
        if set(keys) != set(o["kinds"]): problems.append(f"kinds≠keywords: -{sorted(set(o['kinds']) - set(keys))[:4]} +{sorted(set(keys) - set(o['kinds']))[:4]}")
    rows.append((o, calls, order, problems))
    print(len(order or []), sorted(types), o["proto"].split("🗿️artifacts/")[-1][:60], "OK" if not problems else problems)
    if problems or not apply: continue
    for f in {c[0] for c in calls}:
        t = open(f, encoding="utf-8").read()
        rel = os.path.relpath(proto, os.path.dirname(f))
        lit = "COMPONENT_PROTOCOL_SEMIO" if os.path.dirname(f) == os.path.dirname(proto) else f'include_str!("{rel}")'
        t = re.sub(r"dsl::variants_binary::encode_op\((self|&self)\)", r"dsl::variants_binary::encode_tagged_op(" + lit.replace("\\", "\\\\") + r", \1)", t)
        t = re.sub(r"dsl::variants_binary::decode_op\((bytes)\)", r"dsl::variants_binary::decode_tagged_op(" + lit.replace("\\", "\\\\") + r", \1)", t)
        open(f, "w", encoding="utf-8").write(t)
    note = ["Real op frame (`dsl::variants_binary::encode_tagged_op`): `format u8` (`OP_BINARY_FORMAT`) then `tag varint`,",
            "then the kind's framework record body (`os_pack::encode_record_body`). Each record is one mutation kind at its",
            "wire tag; this file is the only source of those tags, which the codec looks up by the kind's keyword."]
    render(proto, [("format", "u8"), ("tag", "varint")], [(k, i) for i, (_, k) in enumerate(order)], note)
