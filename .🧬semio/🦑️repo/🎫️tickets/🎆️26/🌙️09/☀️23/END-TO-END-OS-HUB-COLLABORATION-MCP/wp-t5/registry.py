"""Per-leaf registry codecs: each leaf's literal tag const becomes a derivation from the aggregate 📡️.protocol.semio record."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from wire import render
root = "/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith("--")]
survey = json.load(open(root + ".tmp-ticket/wp-t5/generated/survey.json"))
KEBAB = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$")
LIT = re.compile(r"(?m)^pub const (TAG|BINARY_TAG): (u8|u32) = (\d+);")
for o in survey:
    if only and not any(x in o["proto"] for x in only): continue
    proto = root + o["proto"]; vocab = os.path.dirname(os.path.dirname(proto))
    leaves = {}
    for d in os.listdir(vocab):
        f = f"{vocab}/{d}/💾️binary/🦀️.rs"
        if KEBAB.search(d) and os.path.exists(f):
            m = LIT.search(open(f, encoding="utf-8").read())
            if m: leaves[KEBAB.search(d).group(0)] = (f, m)
    if not leaves: continue
    agg = open(vocab + "/💾️binary/🦀️.rs", encoding="utf-8").read() if os.path.exists(vocab + "/💾️binary/🦀️.rs") else ""
    used = "REGISTRY" in agg or "BINARY_CODECS" in agg or "BINARY_TAG_REGISTRY" in agg
    agg_all = agg + "".join(open(root + f, encoding="utf-8").read() for f in o["impl"])
    tags = {k: int(m.group(3)) for k, (f, m) in leaves.items()}
    problems = []
    if set(tags) != set(o["kinds"]): problems.append(f"kinds {sorted(set(o['kinds']) - set(tags))} untagged, {sorted(set(tags) - set(o['kinds']))} extra")
    if len(set(tags.values())) != len(tags): problems.append("duplicate tags")
    print(len(tags), "used" if used else "UNUSED", o["proto"].split("🗿️artifacts/")[-1][:70], "OK" if not problems else problems)
    declared = {k: int(v) for k, v in re.findall(r"(?m)^\s*record\s+([a-z][a-z0-9-]*)\s+tag=(\d+)", open(proto, encoding="utf-8").read())}
    keep_protocol = "--consts-only" in sys.argv
    if keep_protocol and declared != tags: problems.append(f"leaf consts disagree with the records: {sorted(set(tags.items()) ^ set(declared.items()))[:4]}")
    if problems: print("   ", problems)
    if problems or not apply or not (used or keep_protocol): continue
    for k, (f, m) in leaves.items():
        text = open(f, encoding="utf-8").read()
        rel = os.path.relpath(proto, os.path.dirname(f))
        text = text.replace(m.group(0), f'pub const {m.group(1)}: {m.group(2)} = dsl::protocol_record::tag_{m.group(2)}(include_str!("{rel}"), "{k}");', 1)
        open(f, "w", encoding="utf-8").write(text)
    if keep_protocol: continue
    fmt = "OP_BINARY_FORMAT" in agg_all
    header = [("format", "u8"), ("tag", "u8")] if fmt else [("tag", "u8")]
    note = ["Real op frame: " + ("`format u8` (`OP_BINARY_FORMAT`) then " if fmt else "") + "`tag u8`, then the kind's own payload (its leaf's `💾️binary` codec).",
            "Each record is one mutation kind at its wire tag. This file is the only source of those tags: every leaf",
            "codec derives its tag from its record at compile time (`dsl::protocol_record::tag_u8`)."]
    render(proto, header, sorted(tags.items(), key=lambda kv: kv[1]), note)
