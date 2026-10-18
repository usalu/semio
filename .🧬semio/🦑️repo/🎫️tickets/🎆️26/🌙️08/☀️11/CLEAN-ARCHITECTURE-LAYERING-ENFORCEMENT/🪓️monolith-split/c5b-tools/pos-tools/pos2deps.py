"""🧾 POS2 undeclared-dependency scan (read only, no rustc). For every crate of a family: every `semio_*::` head used in its source closure that names a package of the repo
but is neither a declared dependency / dev-dependency, nor the crate itself, nor an alias or a local module, is reported with its first use site. cargo would fail on each of them with E0432/E0433.
usage: pos2deps.py <family> [<family> ...]      (writes GEN/pos2-deps-<family>.txt)"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import posunion as U
from poslib import *
from posstub import mask

HEAD = re.compile(r"(?<![\w$.:])(?:::)?\b(semio_[a-z0-9_]+)\s*::|\buse\s+(?:::)?(semio_[a-z0-9_]+)\s*(?:as\s+\w+\s*)?;")
mans = Manifests(Source(None))
mans.build_index()
by_ident = {n.replace("-", "_"): n for n in mans.by_name}
tot = 0
for fam in sys.argv[1:]:
    spec = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "families", fam + ".json"), encoding="utf-8"))
    rev = os.environ.get("POS2_REV")
    crates = U.prepare(Source(rev) if rev else Source(None), mans, spec["base"]["crates"] if rev else spec["crates"], True)
    lines = []
    for c in crates:
        declared = set(c.externs) | set(c.aliases) | {c.id}
        seen = {}
        for rel in sorted(c.code):
            tx = c.files[rel].decode("utf-8", "replace")
            mt = mask(tx)
            local = set(re.findall(r"\bmod\s+(\w+)\b", mt))
            for m in HEAD.finditer(mt):
                h = m.group(1) or m.group(2)
                if h in declared or h in local or h not in by_ident or h in seen:
                    continue
                seen[h] = (rel, tx.count("\n", 0, m.start()) + 1)
        for h, (rel, ln) in sorted(seen.items()):
            lines.append(f"{c.pkg}: undeclared `{h}` ({by_ident[h]}) first used at {rel[-90:]}:{ln}")
    tot += len(lines)
    log_write(f"{GEN}/{PREFIX}-deps-{fam}{'-base' if rev else ''}.txt", "\n".join(lines) + "\n")
    print(f"{fam}: {len(lines)} undeclared")
    for l in lines:
        print("   ", l)
print("total", tot)
