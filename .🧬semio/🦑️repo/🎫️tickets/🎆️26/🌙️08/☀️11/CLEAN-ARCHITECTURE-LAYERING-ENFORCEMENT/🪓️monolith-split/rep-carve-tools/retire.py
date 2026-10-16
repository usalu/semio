"""♻️ Distributes the retirement integration impls to the files that define their types."""
import re, collections
from rlib import *
import rcrates as C

RET = R + "🧬️retirement/🦀️.rs"
NAMES = ["RetireOwned", "RetirementCursor", "sequence", "leaf", "deferred", "deferred_birth_bytes_for", "sequence_birth_bytes"]

def def_index():
    out = collections.defaultdict(list)
    for key in C.KEYS:
        for f in crawl(C.libroot(key)):
            if "🧪️tests" in f or f.endswith("🧪️tests/🦀️.rs"): continue
            for name in top_defs(read_stage_or_base(f)):
                out[name].append((key, f))
    return out

def collect(extra_items):
    t = read_base(RET)
    out = []
    for it in items(t):
        if it["kind"] == "use": continue
        txt = re.sub(r"\bcrate::", "", it["text"])
        if it["kind"] == "impl":
            m = re.search(r"\bfor\s+(\w+)", txt)
            out.append((m.group(1), txt))
            continue
        m = re.search(r"semio_framework_value::artifact_retire_(struct|leaf)!\((.*)\);\s*$", txt, re.S)
        assert m, txt[:100]
        kind, body = m.group(1), m.group(2)
        if kind == "leaf":
            for ty in [x.strip() for x in body.split(",") if x.strip()]:
                out.append((ty, "semio_framework_value::artifact_retire_leaf!(" + ty + ");"))
        else:
            ty = re.match(r"\s*(\w+)", body).group(1)
            out.append((ty, "semio_framework_value::artifact_retire_struct!(" + body.strip() + ");"))
    for txt in extra_items:
        m = re.search(r"\bfor\s+(\w+)", txt) if txt.lstrip().startswith("impl") else None
        if m: ty = m.group(1)
        else: ty = re.search(r"!\(\s*(\w+)", txt).group(1)
        out.append((ty, txt.strip()))
    return out

def distribute(extra_items, report):
    idx = def_index()
    by_file = collections.defaultdict(list)
    for ty, txt in collect(extra_items):
        d = idx.get(ty)
        assert d, ("no definition for retired type", ty)
        assert len(d) == 1, ("ambiguous definition", ty, d)
        key, f = d[0]
        by_file[f].append((ty, txt))
        report.setdefault(key, []).append(ty)
    for f, entries in by_file.items():
        body = "\n".join(txt for _, txt in entries)
        used = bare_ids(body)
        imports = [n for n in NAMES if n in used]
        head = "mod owned_retirement {\n    use super::*;\n"
        if imports: head += "    use semio_framework_value::retirement::{" + ", ".join(imports) + "};\n"
        inner = "\n".join(("    " + l if l.strip() else l) for l in body.split("\n"))
        t = read_stage_or_base(f)
        assert "mod owned_retirement" not in t, f
        write(f, t.rstrip("\n") + "\n\n" + head + inner + "\n}\n")
    return by_file
