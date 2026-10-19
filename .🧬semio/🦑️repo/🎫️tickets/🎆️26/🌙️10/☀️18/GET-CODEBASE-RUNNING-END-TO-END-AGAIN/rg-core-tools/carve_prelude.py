import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl, deps, repoint
R = rgl.ROOT
TF = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing"
FM = TF + "/📦️packages/🦀️rust"
N = TF + "/🧭️prelude/📦️packages/🦀️rust"
os.makedirs(N, exist_ok=True)
m = rgl.read(FM + "/Cargo.toml")
dep = re.search(r'\[dependencies\]\n(.*?)(?=\n\[|\Z)', m, re.S).group(1)
lines = []
for l in dep.split("\n"):
    if not l.strip(): continue
    pm = re.search(r'path = "([^"]+)"', l)
    if pm:
        absd = os.path.normpath(os.path.join(FM, pm.group(1)))
        l = l.replace(pm.group(1), os.path.relpath(absd, N))
    lines.append(l)
ws = os.path.relpath(R + "/🧰️framework", N)
rgl.write(N + "/Cargo.toml", f'''[package]
workspace = "{ws}"
name = "semio-framework-os-store-testing-prelude"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Store test prelude: the Store vocabulary every companion test crate imports."
publish = false

[package.metadata.semio]
role = "framework"
id = "os-store-testing-prelude"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
name = "semio_framework_os_store_testing_prelude"
path = "../../🦀️.rs"

[dependencies]
''' + "\n".join(lines) + "\n")
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/os-store-testing-prelude-rs"))
root = TF + "/🦀️.rs"
s = rgl.read(root)
old = '#[path = "🧭️prelude/🦀️.rs"]\npub mod prelude;\n'
assert old in s
s = s.replace(old, 'use semio_framework_os_store_testing_prelude as prelude;\n')
rgl.write(root, s)
t = rgl.read(FM + "/Cargo.toml")
t = t.replace("[dependencies]\n", '[dependencies]\nsemio-framework-os-store-testing-prelude = { path = "../../🧭️prelude/📦️packages/🦀️rust" }\n', 1)
rgl.write(FM + "/Cargo.toml", t)
rgl.register(FM, "semio-framework-os-store-testing-fixtures", N, "semio-framework-os-store-testing-prelude", "@semio-tech/os-store-testing-fixtures-rs", "@semio-tech/os-store-testing-prelude-rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🧭️prelude/📦️packages/🦀️rust", dep_ws=[])
import subprocess
files = [f for f in subprocess.check_output(["git", "-C", R, "grep", "-l", "os_store_testing_fixtures", "--", "*.rs", ":!.🧬semio"]).decode().split("\n") if f]
for f in files:
    p = R + "/" + f
    t = rgl.read(p)
    t2 = t.replace("semio_framework_os_store_testing_fixtures::prelude", "semio_framework_os_store_testing_prelude")
    t2 = re.sub(r'semio_framework_os_store_testing_fixtures::\{([^}]*?)\bprelude\b(?:::\*)?\s*,?\s*([^}]*)\}', lambda m: "semio_framework_os_store_testing_fixtures::{" + (m.group(1) + m.group(2)).strip().strip(",") + "};\nuse semio_framework_os_store_testing_prelude::*;" if False else m.group(0), t2)
    if t2 != t:
        rgl.write(p, t2)
        mm = deps.manifest_with_anchor(p, "semio-framework-os-store-testing-fixtures")
        print(os.path.relpath(mm, R)[-60:] if mm else "NO-OWNER", f[-50:], deps.add_dep(mm, "semio-framework-os-store-testing-fixtures", "semio-framework-os-store-testing-prelude", N) if mm else "")
