"""🗂️ Registry edits: workspace members, profile rows, build owners, launch routes and lock entries (pure functions over live file text)."""
import json, re
from rlib import *
import rcrates as C

PROFILE_ANCHOR = "[profile.wasm-prototype.package.semio-framework-replication]\nopt-level = 2\n"

def members_edit(text):
    m = re.search(r"members = \[\n(.*?)\n\]", text, re.S)
    lines = m.group(1).split("\n")
    pre = '    "🔨️modules/📡️replication/'
    for k in C.KEYS:
        entry = '    "' + C.pkgdir(k)[len("🧰️framework/"):] + '",'
        if entry in lines: continue
        idx = None
        for i, l in enumerate(lines):
            if l.startswith(pre) and l.encode() > entry.encode():
                idx = i; break
        if idx is None:
            idx = max(i for i, l in enumerate(lines) if l.startswith(pre)) + 1
        lines.insert(idx, entry)
    return text[:m.start(1)] + "\n".join(lines) + text[m.end(1):]

def profile_rows():
    rows = ""
    for k in C.KEYS:
        rows += "\n# 🎚️ `" + C.short_id(k) + "` — carved out of `semio-framework-replication`; keeps the optimisation level the replication crate had on the same hop.\n[profile.wasm-prototype.package." + C.cname(k) + "]\nopt-level = 2\n"
    return rows

def profile_edit(text):
    assert text.count(PROFILE_ANCHOR) == 1
    if "[profile.wasm-prototype.package.semio-framework-replication-ids]" in text: return text
    return text.replace(PROFILE_ANCHOR, PROFILE_ANCHOR + profile_rows())

def owners_edit(text):
    rows = []
    for k in C.KEYS:
        n = C.project_name(k)
        assert ('"name": "' + n + '"') not in text, n
        rows.append((n, '    {\n      "name": "' + n + '",\n      "root": "' + C.pkgdir(k) + '",\n      "manifest": "' + C.pkgdir(k) + '/📋️project.json",\n      "packages": []\n    },\n'))
    entries = list(re.finditer(r'    \{\n      "name": "([^"]+)",', text))
    out = text
    for n, row in sorted(rows, key=lambda x: x[0].encode(), reverse=True):
        entries = list(re.finditer(r'    \{\n      "name": "([^"]+)",', out))
        pos = None
        for e in entries:
            if e.group(1).encode() > n.encode():
                pos = e.start(); break
        assert pos is not None, n
        out = out[:pos] + row + out[pos:]
    json.loads(out)
    return out

ROUTES = [("build", "🏗️build", "4_build", "prototype", "🛠️prototype"), ("build", "🏗️build", "4_build", "release", "🚀️release"),
          ("check", "🔎️check", "4_gate", "prototype", "🛠️prototype"), ("check", "🔎️check", "4_gate", "release", "🚀️release"),
          ("test", "🧪️test", "4_test", "prototype", "🛠️prototype"), ("test", "🧪️test", "4_test", "release", "🚀️release")]
BAND = {"4_build": "900.0", "4_gate": "922.0", "4_test": "903.5"}

def pick_order_base(texts):
    used = set()
    for t in texts: used |= set(re.findall(r'"order": ([0-9.]+)', t))
    norm = {str(float(x)) for x in used}
    spec = {"4_build": range(63, 100), "4_gate": range(50, 100), "4_test": range(90, 100)}
    base = {}
    n = len(C.KEYS)
    for group, codes in spec.items():
        for c in codes:
            cand = [BAND[group] + "%02d" % c + "%02d" % (2 * idx - 2 + rel) for idx in range(1, n + 1) for rel in (1, 2)]
            if not any(x in used or str(float(x)) in norm for x in cand):
                base[group] = "%02d" % c; break
        else:
            raise RuntimeError("no free order band " + group)
    return base

def route_blocks(base):
    out = []
    for idx, k in enumerate(C.KEYS, start=1):
        for target, label, group, conf, cl in ROUTES:
            rel = 1 if conf == "prototype" else 2
            order = BAND[group] + base[group] + "%02d" % (2 * idx - 2 + rel)
            out.append('    {\n      "name": "🦀️ ' + C.cname(k) + ' ' + label + ' ' + cl + '",\n      "type": "node-terminal",\n      "request": "launch",\n'
                       '      "command": "bun repo run ' + C.project_name(k) + ':' + target + ' --configuration=' + conf + ' --parallel=1 -- --lib --offline",\n'
                       '      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "' + group + '",\n        "order": ' + order + '\n      }\n    },\n')
    return out

LAW_NAME = "🧪️test📡️replication🏛️crate-partition"
LAW_PROJECT = "@semio-tech/framework-replication-rs"

def law_block(order):
    return ('    {\n      "name": "' + LAW_NAME + '",\n      "type": "node-terminal",\n      "request": "launch",\n'
            '      "command": "bun repo run ' + LAW_PROJECT + ':test-crate-partition --skip-cache --exclude-dependencies",\n'
            '      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "9_gates",\n        "order": ' + order + '\n      },\n'
            '      "env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}\n    },\n')

def pick_law_order(texts):
    used = set()
    for t in texts: used |= set(re.findall(r'"order": ([0-9.]+)', t))
    norm = {str(float(x)) for x in used}
    for i in range(1, 1000):
        cand = "900.0581" + "%03d" % (200 + i)
        if cand not in used and str(float(cand)) not in norm: return cand
    raise RuntimeError("no free law order")

def launch_edit(text, anchor_name, base, law_order):
    blocks = "".join(route_blocks(base)) + law_block(law_order)
    m = re.search(r'    \{\n      "name": "' + re.escape(anchor_name) + r'",.*?\n    \},?\n', text, re.S)
    assert m, anchor_name
    assert "semio-framework-replication-ids 🏗️build" not in text
    return text[:m.end()] + blocks + text[m.end():]

def dep_names(manifest_text, dev=True):
    names = set(); sect = None
    for l in manifest_text.split("\n"):
        s = l.strip()
        if s.startswith("["): sect = s; continue
        if sect in (("[dependencies]", "[dev-dependencies]") if dev else ("[dependencies]",)) and s and not s.startswith("#"):
            m = re.match(r'^([A-Za-z0-9_\-]+)(?:\.workspace)?\s*=', s)
            if m: names.add(m.group(1))
    return sorted(names)

def block(name, deps):
    return '[[package]]\nname = "%s"\nversion = "0.1.0"\ndependencies = [\n%s]\n' % (name, "".join(' "%s",\n' % d for d in deps))

def new_blocks(dev=True):
    out = {}
    for k in C.KEYS:
        out[C.cname(k)] = block(C.cname(k), dep_names(read_stage_or_base(C.pkgdir(k) + "/Cargo.toml"), dev))
    out["semio-framework-replication"] = block("semio-framework-replication", dep_names(read_stage_or_base(FACADE_PKG + "/Cargo.toml"), dev))
    return out

def apply_lock(text, blocks, check_deps=True):
    parts = text.split("\n[[package]]\n")
    head = parts[0]; pkgs = ["[[package]]\n" + p for p in parts[1:]]
    pname = lambda b: re.search(r'name = "([^"]+)"', b).group(1)
    have = {pname(b): i for i, b in enumerate(pkgs)}
    if check_deps:
        for n, b in blocks.items():
            for d in re.findall(r'^ "([^"]+)",$', b, re.M):
                assert d in have or d in blocks, ("lock lacks dependency", n, d)
    for n, b in blocks.items():
        b = b.rstrip("\n") + "\n"
        if n in have:
            old = pkgs[have[n]]
            if "\nsource = " in old or "\nchecksum" in old: continue
            pkgs[have[n]] = b + ("\n" if old.endswith("\n\n") else "")
        else:
            pkgs.append(b + "\n")
    pkgs.sort(key=lambda b: pname(b).encode())
    pkgs = [p if p.endswith("\n\n") else p.rstrip("\n") + "\n\n" for p in pkgs]
    pkgs[-1] = pkgs[-1].rstrip("\n") + "\n"
    return head + "\n" + "".join(pkgs)
