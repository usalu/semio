"""🕳️ OHH undeclared-crate-head scan (read only, no rustc): for every crate of a family, every path head (`use X::…`, `X::path`) that names the lib name of a repo package but is neither a declared
dependency (normal/dev/build; renames honoured), the crate itself, nor a local module / alias / type, is reported with its first use site. cargo fails each with E0433/E0432.
usage: undecl.py <family-json> [<family-json>...]"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/pos")
import posunion as U
from poslib import *
from posstub import mask
mans = Manifests(Source(None))
mans.build_index()
LIB = collections.defaultdict(set)
for n in mans.by_name:
    mm = mans.manifest(n)
    if mm:
        LIB[mans.lib_name(mm).replace("-", "_")].add(n)
HEAD = re.compile(r"(?<![\w$.:])(?:::)?\b([a-z][a-z0-9_]*)\s*::")
tot = 0
for fam in sys.argv[1:]:
    spec = json.load(open(fam, encoding="utf-8"))
    pk = spec.get("own", spec["crates"])
    crates = U.prepare(Source(None), mans, pk, True)
    lines = []
    for c in crates:
        declared = set(c.externs) | set(c.aliases) | {c.id}
        seen = {}
        for rel in sorted(c.code):
            tx = c.files[rel].decode("utf-8", "replace")
            mt = mask(tx)
            local = set(re.findall(r"\b(?:mod|enum|struct|trait|type|fn|union|const|static|let|mut|use\s+[\w:]*?\bas)\s+(\w+)", mt))
            local |= set(re.findall(r"\buse\s+[^;]*?\b(\w+)\s*(?:,|;|\})", mt))
            local |= set(re.findall(r"\|\s*(\w+)\s*[:|,]", mt))
            local |= set(re.findall(r"\b(\w+)\s*:\s*[^:]", mt))
            for m in HEAD.finditer(mt):
                h = m.group(1)
                if h in declared or h in local or h not in LIB or h in seen or h in ("self", "super", "crate", "std", "core", "alloc"):
                    continue
                seen[h] = (rel, tx.count("\n", 0, m.start()) + 1)
        for h, (rel, ln) in sorted(seen.items()):
            lines.append(f"{c.pkg}: undeclared `{h}` ({'/'.join(sorted(LIB[h]))}) first at {rel[-80:]}:{ln}")
    tot += len(lines)
    print(os.path.basename(fam), len(lines))
    for l in lines:
        print("   ", l)
