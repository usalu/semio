"""Public top-level item set (exact `pub`) of the original root vs root + lines-kind children now; plus the pub-use heads of the root."""
import collections, importlib, json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from tgmlib import *
import split as S


def pubset(text):
    lines, depths, inlit, items = S.analyse(text)
    out = collections.Counter()
    for it in items:
        hs = lines[it["head_idx"]].lstrip()
        if re.match(r"^pub\s", hs) and not re.match(r"^pub\s*\(", hs):
            if it["kind"] == "impl":
                continue
            out[(it["kind"], it["name"] or re.sub(r"\s+", " ", hs)[:80])] += 1
    return out


def main():
    res = {}
    for name, t in TARGETS.items():
        spec = S.load_spec(name)
        base = os.path.dirname(t["root"])
        old = pubset(read(os.path.join(snap_dir(name), t["root"])))
        # inline kind: original public names inside inline modules are not top-level; keep them out by comparing only top-level
        new = pubset(read(os.path.join(ROOT, t["root"])))
        for ch in spec["children"]:
            if ch.get("kind", "lines") == "lines":
                new += pubset(read(os.path.join(ROOT, base, ch["folder"], "🦀️.rs")))
        # mounts add `pub use x::*` / pub mod lines; drop those from new, and the `pub mod` of inline children from both sides
        for k in list(new):
            if k[0] == "use" and re.match(r"^pub use \w+::\*;", k[1]):
                gl = k[1]
                for ch in spec["children"]:
                    if gl.startswith(f"pub use {ch['mod']}::*"):
                        del new[k]
                        break
        missing = old - new
        extra = new - old
        res[name] = dict(old=sum(old.values()), new=sum(new.values()), missing=[list(k) for k in missing], extra=[list(k) for k in extra])
    print(json.dumps(res, ensure_ascii=False, indent=1))
    write(F + "/🗑️generated/tgm-pubsurface.json", json.dumps(res, ensure_ascii=False, indent=1))


main()
