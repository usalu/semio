# 🧱️ Free-item inventory of a staging crate: every column-0 item that is not an `impl ShellState`, with lines.
import os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import shellsrc as S, stg
def free_items(crate_dir):
    out = []
    for p in stg.files(crate_dir):
        raw = open(p, encoding="utf-8").read()
        for it in S.outline(raw):
            if it["kind"] == "impl" and "ShellState" in it["head"] and " for " not in it["head"]: continue
            if it["kind"] == "use": continue
            it["path"] = p; it["slug"] = os.path.basename(os.path.dirname(p)); out.append(it)
    return out
if __name__ == "__main__":
    for d in sys.argv[1:]:
        its = free_items(d)
        tot = collections.Counter()
        for it in its: tot[it["slug"]] += it["end"] - it["start"] + 1
        print("==", d, sum(tot.values()), dict(tot))
        for it in its:
            n = it["end"] - it["start"] + 1
            if n >= int(os.environ.get("MIN", "20")): print("   %-30s %4d %-7s %s" % (it["slug"], n, it["kind"], it["name"] or it["head"][:60]))
