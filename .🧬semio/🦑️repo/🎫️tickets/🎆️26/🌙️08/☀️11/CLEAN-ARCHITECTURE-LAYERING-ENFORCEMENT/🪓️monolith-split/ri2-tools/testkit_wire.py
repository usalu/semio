#!/usr/bin/env python3
# 🧪️ Wires the `testkit` feature of the Shell crates that testkitify touched: [features] forwarding to first-party dependencies that have it, and the top crate's dev-dependencies.
import sys, re, json, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import flip as F
from model import *
done = json.load(open(F.HERE + "/testkit-crates.json"))
base = {"contracts", "ui-engine", "ui-engine-dump", "ui-engine-focus"}
tk = base | set(done)
table = F.TABLE
for c in done:
    man = T + "/" + table[c] + "/📦️packages/🦀️rust/Cargo.toml"
    text = read(man)
    deps = set(re.findall(r'^semio-framework-os-renderer-wgpu-([a-z0-9-]+)\s*=', text, re.M))
    fwd = sorted(d for d in deps if d in tk and d != c)
    line = "testkit = [" + ", ".join(f'"semio-framework-os-renderer-wgpu-{d}/testkit"' for d in fwd) + "]"
    if "[features]" in text:
        if re.search(r"(?m)^testkit\s*=", text):
            keep = re.findall(r'"(dep:[^"]+)"', re.search(r"(?m)^testkit\s*=.*$", text).group(0))
            full = line[:-1] + "".join(', "%s"' % k for k in keep) + "]" if keep else line
            text = re.sub(r"(?m)^testkit\s*=.*$", lambda m: full, text)
        else:
            text = text.replace("[features]\n", "[features]\n" + line + "\n", 1)
    else:
        text = text.replace("\n[dependencies]\n", "\n[features]\n" + line + "\n\n[dependencies]\n", 1)
    write(man, text)
top = T + "/📦️packages/🦀️rust/Cargo.toml"
tt = read(top)
for c in sorted(done):
    key = f"semio-framework-os-renderer-wgpu-{c}"
    if re.search(r"(?m)^\[dev-dependencies\][\s\S]*^" + re.escape(key) + r"\s*=", tt):
        continue
    newline = f'{key} = {{ path = "../../{table[c]}/📦️packages/🦀️rust", features = ["testkit"] }}'
    i = tt.index("[dev-dependencies]\n") + len("[dev-dependencies]\n")
    j = tt.find("\n[", i)
    j = len(tt) if j < 0 else j
    lines = tt[i:j].strip("\n").split("\n") + [newline]
    lines.sort(key=lambda l: l.split("=")[0].strip())
    tt = tt[:i] + "\n".join(lines) + "\n" + tt[j:]
write(top, tt)
print("wired", sorted(done))

# the test bins build their Shell dependencies with the testkit variants
import glob
tkset = base | set(done)
for c in [c for c in table if c.startswith("shell-tests-")] + ["ui-engine-tests"]:
    man = T + "/" + table[c] + "/📦️packages/🦀️rust/Cargo.toml"
    text = read(man)
    def fix(m):
        name = m.group(1)
        if name in tkset and 'features = ["testkit"]' not in m.group(0):
            return m.group(0)[:-2] + ', features = ["testkit"] }'
        return m.group(0)
    new = re.sub(r'(?m)^semio-framework-os-renderer-wgpu-([a-z0-9-]+) = \{ path = "[^"]*" \}', lambda m: fix(m).replace(m.group(1), m.group(1), 1) if False else (m.group(0)[:-2] + ', features = ["testkit"] }' if m.group(1) in tkset and 'features' not in m.group(0) else m.group(0)), text)
    if new != text:
        write(man, new)
print("bins wired")
