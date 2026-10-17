"""🧪 RI3 scan: for every renderer crate with a `testkit` feature, checks that it forwards `testkit` to every first-party renderer dependency that has one, and which crates use `feature = "testkit"` in code without declaring the feature."""
import os, re, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from owners import *

def mini_toml(text):
    out, sec = {}, None
    for line in text.split("\n"):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        m = re.match(r"^\[(.+)\]$", line)
        if m:
            sec = m.group(1)
            out.setdefault(sec, {})
            continue
        m = re.match(r"^([A-Za-z0-9_\-\"\.]+)\s*=\s*(.*)$", line)
        if m and sec is not None:
            out[sec][m.group(1).strip('"')] = m.group(2)
    return out

PFX = "semio-framework-os-renderer-wgpu-"


def load(folder):
    return mini_toml(read(os.path.join(T, folder, "📦️packages/🦀️rust/Cargo.toml")))


def main():
    tbl, cf, idx = build()
    man = {s: load(f) for s, f in tbl.items()}
    has_tk = {s for s, m in man.items() if "testkit" in m.get("features", {})}
    for s, m in man.items():
        if "testkit" in m.get("features", {}):
            m["features"]["testkit"] = re.findall(r'"([^"]+)"', m["features"]["testkit"])
    used_tk = set()
    for s, files in cf.items():
        for f in files:
            try:
                if 'feature = "testkit"' in read(f):
                    used_tk.add(s)
                    break
            except Exception:
                pass
    print("crates using feature=testkit in code without declaring it:", sorted(used_tk - has_tk))
    bad = []
    for s in sorted(has_tk):
        m = man[s]
        deps = set()
        for secname, sec in m.items():
            if secname == "dependencies" or (secname.startswith("target.") and secname.endswith(".dependencies")):
                for k in sec:
                    if k.startswith(PFX):
                        deps.add(k[len(PFX):])
        fw = set()
        for e in m["features"]["testkit"]:
            e2 = e[4:] if e.startswith("dep:") else e
            if "/" in e2:
                d, feat = e2.split("/", 1)
                d = d[len(PFX):] if d.startswith(PFX) else d
                if feat == "testkit":
                    fw.add(d)
        for d in sorted(deps):
            if d in has_tk and d not in fw:
                bad.append((s, d))
    print("missing forwards (consumer testkit lacks owner testkit):")
    for s, d in bad:
        print("   ", s, "->", d)
    json.dump(dict(bad=bad, has_tk=sorted(has_tk), used_tk=sorted(used_tk)), open(GEN + "/ri3-testkit-forward.json", "w"))


if __name__ == "__main__":
    main()


def closure_check(owner="shell-history", consumer="shell-state"):
    tbl, cf, idx = build()
    man = {s: load(f) for s, f in tbl.items()}
    for s, m in man.items():
        if "testkit" in m.get("features", {}):
            m["features"]["testkit"] = re.findall(r'"([^"]+)"', m["features"]["testkit"])
    def deps_of(s):
        m, out = man[s], set()
        for secname, sec in m.items():
            if secname in ("dependencies",) or (secname.startswith("target.") and secname.endswith(".dependencies")):
                for k in sec:
                    if k.startswith(PFX):
                        out.add(k[len(PFX):])
        return out
    def trans(s):
        seen, st = set(), [s]
        while st:
            x = st.pop()
            for d in deps_of(x) if x in man else []:
                if d not in seen:
                    seen.add(d)
                    st.append(d)
        return seen
    def tk_closure(s):
        seen, st = set(), [s]
        while st:
            x = st.pop()
            if x in seen or x not in man:
                continue
            seen.add(x)
            for e in man[x].get("features", {}).get("testkit", []):
                e2 = e[4:] if e.startswith("dep:") else e
                if "/" in e2:
                    d, feat = e2.split("/", 1)
                    d = d[len(PFX):] if d.startswith(PFX) else d
                    if feat == "testkit":
                        st.append(d)
        return seen
    out = []
    for s in sorted(man):
        if "testkit" not in man[s].get("features", {}):
            continue
        E = tk_closure(s)
        if owner in E and (consumer in trans(s) or s == consumer) and consumer not in E:
            out.append(s)
    return out


if __name__ == "__main__":
    print("closure check history->state:", closure_check())
