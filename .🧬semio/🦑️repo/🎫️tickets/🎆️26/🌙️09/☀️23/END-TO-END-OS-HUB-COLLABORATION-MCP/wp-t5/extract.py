"""Reads the op tags each codec really writes, from its own encode and decode bodies, and cross-checks both sides."""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
survey = json.load(open(root + ".tmp-ticket/wp-t5/generated/survey.json"))

def kebab(name):
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "-", name).lower()

def body(text, fn):
    m = re.search(r"fn " + fn + r"\s*\([^)]*\)[^{]*\{", text)
    if not m: return ""
    depth, i = 1, m.end()
    while depth and i < len(text):
        depth += {"{": 1, "}": -1}.get(text[i], 0); i += 1
    return text[m.end():i]

def arms(segment, enum):
    return [(m.start(), m.group(1)) for m in re.finditer(r"\b" + enum + r"::(\w+)", segment)]

def decode_side(dec, enum):
    out = {}
    marks = [(m.start(), int(m.group(1))) for m in re.finditer(r"(?m)^\s*(\d+)\s*=>", dec)]
    for idx, (pos, tag) in enumerate(marks):
        end = marks[idx + 1][0] if idx + 1 < len(marks) else len(dec)
        found = arms(dec[pos:end], enum)
        if found: out.setdefault(found[0][1], tag)
    return out

def encode_side(enc, enum):
    out = {}
    hits = arms(enc, enum)
    for idx, (pos, variant) in enumerate(hits):
        end = hits[idx + 1][0] if idx + 1 < len(hits) else len(enc)
        seg = enc[pos:end]
        m = re.search(r"=>\s*(?:\{\s*)?(\d+)\s*[,}]", seg) or re.search(r"(?:write_u8|push)\(\s*(\d+)\s*\)", seg) or re.search(r"OP_BINARY_FORMAT,\s*(\d+)\s*\]", seg)
        if m and variant not in out: out[variant] = int(m.group(1))
    return out

results = []
for o in survey:
    if "literal" not in o["shape"] or not o["impl"]: continue
    text = "".join(open(root + f, encoding="utf-8").read() for f in o["impl"])
    enum = re.search(r"impl\s+(?:protocol::|store::)?OpBinary\s+for\s+(\w+)", text).group(1)
    dec = decode_side(body(text, "decode_op"), enum)
    enc = encode_side(body(text, "encode_op"), enum)
    kinds = set(o["kinds"])
    variants = sorted(set(dec) | set(enc))
    rows, problems = [], []
    for v in variants:
        d, e = dec.get(v), enc.get(v)
        if d is not None and e is not None and d != e: problems.append(f"{v}: encode {e} != decode {d}")
        tag = d if d is not None else e
        k = kebab(v)
        if k not in kinds: problems.append(f"{v}: kind {k} has no leaf directory")
        rows.append((k, tag, "both" if d is not None and e is not None else ("decode" if d is not None else "encode")))
    missing = sorted(kinds - {r[0] for r in rows})
    if missing: problems.append("kinds without a tag: " + ", ".join(missing))
    results.append({"proto": o["proto"], "enum": enum, "records": rows, "problems": problems})
json.dump(results, open(root + ".tmp-ticket/wp-t5/generated/extract-literal.json", "w"), ensure_ascii=False, indent=1)
for r in results:
    print(len(r["records"]), r["enum"], "OK" if not r["problems"] else r["problems"][:4])
