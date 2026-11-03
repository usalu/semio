"""🧪 Applies the en1992 `test-support` serde pattern to every artifact model crate and enables it in the dev-dependencies of every crate of the artifact workspace that uses serde."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, sys, tomllib
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
ARTS = NORM + "/🗿️artifacts"
SNAP = "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot"
rows = []
for art in sorted(os.listdir(ARTS)):
    root = ARTS + "/" + art
    mt = root + SNAP + "/📦️packages/🦀️rust/Cargo.toml"
    if not os.path.exists(mt):
        continue
    name = tomllib.loads(rd(mt))["package"]["name"]
    for dp, dn, fn in os.walk(root + SNAP):
        dn[:] = [d for d in dn if d not in ("target", "node_modules")]
        for f in fn:
            if f.endswith(".rs"):
                p = dp + "/" + f
                s = rd(p)
                n = s.replace("cfg_attr(test, ", 'cfg_attr(any(test, feature = "test-support"), ')
                if n != s:
                    wr(p, n)
    s = rd(mt)
    if "test-support" not in s:
        s = s.replace("[dependencies]\n", "[dependencies]\nserde = { workspace = true, optional = true }\n", 1)
        if "[features]" in s:
            s = s.replace("[features]\n", '[features]\ntest-support = ["dep:serde"]\n', 1)
        else:
            s = s.replace("\n[dev-dependencies]", '\n[features]\ntest-support = ["dep:serde"]\n\n[dev-dependencies]', 1)
            assert "test-support" in s, mt
        wr(mt, s)
    MODS = re.compile(r'((?:#\[path\s*=\s*"([^"]+)"\]\s*)?)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')
    INC = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
    USE = re.compile(r'serde_json::(to_value|to_string|from_value|from_str|to_vec|from_slice|to_string_pretty|to_writer)')

    def closure(entry):
        seen, stack = set(), [entry]
        while stack:
            p = os.path.normpath(stack.pop())
            if p in seen or not os.path.isfile(p):
                continue
            seen.add(p)
            t = rd(p)
            d = os.path.dirname(p)
            for m in MODS.finditer(t):
                if m.group(2):
                    stack.append(os.path.join(d, m.group(2)))
                else:
                    base = os.path.splitext(os.path.basename(p))[0]
                    stack += [os.path.join(d, m.group(3) + ".rs"), os.path.join(d, m.group(3), "mod.rs")] if base in ("lib", "main", "mod") else [os.path.join(d, base, m.group(3) + ".rs")]
            for m in INC.finditer(t):
                stack.append(os.path.join(d, m.group(1)))
        return seen

    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in ("target", "node_modules")]
        if "Cargo.toml" not in fn or dp == os.path.dirname(mt):
            continue
        p = dp + "/Cargo.toml"
        t = rd(p)
        try:
            c = tomllib.loads(t)
        except Exception as e:
            continue
        if "package" not in c or c["package"]["name"] == name:
            continue
        entries = [c.get("lib", {}).get("path", "src/lib.rs")] + [x["path"] for x in c.get("test", []) if "path" in x]
        files = set()
        for e in entries:
            files |= closure(os.path.join(dp, e))
        hit = [f for f in files if USE.search(rd(f))]
        dev = c.get("dev-dependencies", {})
        ts = isinstance(dev.get(name), dict) and "test-support" in dev[name].get("features", [])
        line = '%s = { workspace = true, features = ["test-support"] }\n' % name
        if not hit:
            if line in t and not ts is False:
                pass
            if line in t:
                wr(p, t.replace(line, "", 1))
                print("reverted", c["package"]["name"])
            continue
        if ts:
            continue
        if "[dev-dependencies]" in t:
            t = t.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + line, 1)
        else:
            t = t.rstrip("\n") + "\n\n[dev-dependencies]\n" + line
        wr(p, t)
        rows.append((c["package"]["name"], hit[0][len(ARTS):]))
for r in rows:
    print(r)
