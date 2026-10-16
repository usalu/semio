"""🗂️ Registry edits: workspace members, profile rows, build owners and launch routes (pure functions over the live file text)."""
import json, re
from lib import *
from crates import *

RECORD_PROJECT = "@semio-tech/framework-pack-record-rs"

def project_name(key):
    return RECORD_PROJECT if key == "record" else "@semio-tech/" + short_id(key) + "-rs"

def members_edit(text):
    """Inserts the new producer packages into the framework workspace member list, byte-sorted among the pack lines."""
    m = re.search(r"members = \[\n(.*?)\n\]", text, re.S)
    body = m.group(1)
    lines = body.split("\n")
    pre = '    "🔨️modules/🎒️pack/'
    new = []
    for k, *_ in CRATES:
        rel = pkgdir(k)[len("🧰️framework/"):]
        entry = '    "' + rel + '",'
        if entry not in lines: new.append(entry)
    for entry in new:
        idx = None
        for i, l in enumerate(lines):
            if l.startswith(pre) and l.encode() > entry.encode():
                idx = i; break
        if idx is None:
            last = max(i for i, l in enumerate(lines) if l.startswith(pre))
            idx = last + 1
        lines.insert(idx, entry)
    return text[:m.start(1)] + "\n".join(lines) + text[m.end(1):]

PROFILE_ANCHOR = "[profile.wasm-prototype.package.semio-framework-pack]\nopt-level = 2\n"

def profile_edit(text):
    assert text.count(PROFILE_ANCHOR) == 1
    rows = ""
    for k, *_ in CRATES:
        row = "\n# 🎚️ `" + short_id(k) + "` — the pack codec split out of `semio-framework-pack`; keeps the optimisation level the pack crate had on the same hop.\n[profile.wasm-prototype.package." + cname(k) + "]\nopt-level = 2\n"
        if row.strip() in text: continue
        rows += row
    return text.replace(PROFILE_ANCHOR, PROFILE_ANCHOR + rows)

def owners_edit(text):
    anchor = '    {\n      "name": "@semio-tech/pack-source-rs",'
    assert text.count(anchor) == 1
    rows = []
    for k, *_ in CRATES:
        if k == "record": continue
        n = project_name(k)
        rows.append((n, '    {\n      "name": "' + n + '",\n      "root": "' + pkgdir(k) + '",\n      "manifest": "' + pkgdir(k) + '/📋️project.json",\n      "packages": []\n    },\n'))
    rows.sort()
    for n, r in rows:
        assert ('"name": "' + n + '"') not in text
    out = text.replace(anchor, "".join(r for _, r in rows) + anchor)
    json.loads(out)
    return out

ROUTES = [("build", "🏗️build", "4_build", "900.0", "prototype", "🛠️prototype"), ("build", "🏗️build", "4_build", "900.0", "release", "🚀️release"),
          ("check", "🔎️check", "4_gate", "922.0", "prototype", "🛠️prototype"), ("check", "🔎️check", "4_gate", "922.0", "release", "🚀️release"),
          ("test", "🧪️test", "4_test", "903.5", "prototype", "🛠️prototype"), ("test", "🧪️test", "4_test", "903.5", "release", "🚀️release")]

def route_blocks(order_base):
    """Returns the launch route objects (text) for every crate; order keys are `<band><order_base><crate index><release>`."""
    out = []
    for idx, (k, *_) in enumerate(CRATES, start=1):
        for target, label, group, band, conf, cl in ROUTES:
            rel = 1 if conf == "prototype" else 2
            tail = "%02d" % (2 * idx - 2 + rel)
            band_prefix = {"4_build": "900.0", "4_gate": "922.0", "4_test": "903.5"}[group]
            order = band_prefix + order_base[group] + tail
            out.append('    {\n      "name": "🦀️ ' + cname(k) + ' ' + label + ' ' + cl + '",\n      "type": "node-terminal",\n      "request": "launch",\n'
                       '      "command": "bun repo run ' + project_name(k) + ':' + target + ' --configuration=' + conf + ' --parallel=1 -- --lib --offline",\n'
                       '      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "' + group + '",\n        "order": ' + order + '\n      }\n    },\n')
    return out

def pick_order_base(texts):
    used = set()
    for t in texts:
        used |= set(re.findall(r'"order": ([0-9.]+)', t))
    norm = {str(float(x)) for x in used}
    spec = {"4_build": ("900.0", range(16, 100)), "4_gate": ("922.0", range(46, 100)), "4_test": ("903.5", range(86, 100))}
    base = {}
    for group, (prefix, codes) in spec.items():
        for c in codes:
            cand = [prefix + "%02d" % c + "%02d" % (2 * idx - 2 + rel) for idx in range(1, len(CRATES) + 1) for rel in (1, 2)]
            if not any(x in used or str(float(x)) in norm for x in cand):
                base[group] = "%02d" % c; break
        else:
            raise RuntimeError("no free order band " + group)
    return base

def launch_edit(text, anchor_name, base):
    blocks = "".join(route_blocks(base))
    m = re.search(r'    \{\n      "name": "' + re.escape(anchor_name) + r'",.*?\n    \},?\n', text, re.S)
    assert m, anchor_name
    seg = m.group(0)
    if not seg.rstrip().endswith("},"):
        seg = seg.rstrip("\n")[:-1] + "},\n" if False else seg
    ins = m.end()
    new = text[:ins] + blocks + text[ins:]
    return new
