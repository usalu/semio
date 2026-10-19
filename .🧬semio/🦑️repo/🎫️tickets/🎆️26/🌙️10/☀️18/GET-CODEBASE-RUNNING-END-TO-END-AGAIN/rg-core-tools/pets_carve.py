import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl
R = rgl.ROOT
P = R + "/🧰️framework/🛍️products/🐾️pets"
M = P + "/🔨️modules"
A_MODS = ["📝️draft", "📏️spacing", "🗓️schedule", "👀️attention", "🚶️locomotion"]
NAME = {"📝️draft": "draft", "📏️spacing": "spacing", "🗓️schedule": "schedule", "👀️attention": "attention", "🚶️locomotion": "locomotion"}
B_MODS = ["💞️sociability", "🎯️choice", "👥️population", "🕰️clock", "🎥️projection", "🎪️stage"]
A_LIB = "semio_framework_pets_staging_draft"

def edit(p, old, new, cnt=1):
    s = rgl.read(p)
    assert s.count(old) == cnt, (p, old, s.count(old))
    rgl.write(p, s.replace(old, new))

A = M + "/📝️draft/📦️packages/🦀️rust"
os.makedirs(A, exist_ok=True)
rgl.write(A + "/Cargo.toml", '''[package]
workspace = "../../../../../../.."
name = "semio-framework-pets-staging-draft"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Pets staging draft: the working state of a stage with the spacing, schedule, attention and locomotion rules that edit it. Pure functions only, bit for bit the results of the TypeScript twin @semio-tech/pets."

[package.metadata.semio]
role = "product"
id = "pets-staging-draft"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
semio-framework-pets-character = { path = "../../../🎭️character/📦️packages/🦀️rust" }
semio-framework-pets-contract = { path = "../../../📜️contract/📦️packages/🦀️rust" }
semio-framework-pets-kinematics = { path = "../../../⚙️kinematics/📦️packages/🦀️rust" }
''')
rgl.write(A + "/📋️project.json", rgl.project_json("@semio-tech/pets-staging-draft-rs"))
rgl.write(A + "/🦀️.rs", "//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.\n\n" + "".join(f'#[path = "../../{"" if m == "📝️draft" else "../" + m + "/"}🦀️.rs"]\npub mod {NAME[m]};\n\n' for m in A_MODS).rstrip("\n") + "\n")

for m in A_MODS:
    f = f"{M}/{m}/🦀️.rs"
    s = rgl.read(f)
    s2 = s.replace('#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;', "")
    s2 = s2.replace("crate::schema::", "semio_framework_pets_contract::schema::")
    if s2 != s:
        rgl.write(f, s2.rstrip("\n") + "\n")

B = M + "/🎬️staging"
root = B + "/🦀️.rs"
s = rgl.read(root)
for m in A_MODS:
    pat = re.compile(r'#\[path = "\.\./' + re.escape(m) + r'/🦀️\.rs"\]\n(?:pub )?mod ' + NAME[m] + r';\n\n?')
    s, c = pat.subn("", s)
    assert c == 1, m
tests = "".join(f'#[cfg(test)]\n#[path = "../{m}/🧪️tests/🔬️unit/🦀️.rs"]\nmod {NAME[m]}_tests;\n\n' for m in ["📝️draft", "🗓️schedule", "👀️attention", "🚶️locomotion"])
s = s.rstrip("\n") + "\n\n" + tests.rstrip("\n") + "\n"
rgl.write(root, s)

mods_alt = "|".join(NAME.values())
rx = re.compile(r'crate::(' + mods_alt + r')::')
touched = []
for m in B_MODS:
    for f in [f"{M}/{m}/🦀️.rs", f"{M}/{m}/🧪️tests/🔬️unit/🦀️.rs"]:
        if not os.path.exists(f): continue
        t = rgl.read(f)
        t2 = rx.sub(lambda mo: f"{A_LIB}::{mo.group(1)}::", t)
        if t2 != t: rgl.write(f, t2); touched.append(f)
for m in ["📝️draft", "🗓️schedule", "👀️attention", "🚶️locomotion"]:
    f = f"{M}/{m}/🧪️tests/🔬️unit/🦀️.rs"
    t = rgl.read(f)
    t = rx.sub(lambda mo: f"{A_LIB}::{mo.group(1)}::", t)
    assert t.count("use super::*;") == 1, f
    t = t.replace("use super::*;", f"use {A_LIB}::{NAME[m]}::*;")
    rgl.write(f, t)
    touched.append(f)
for m in B_MODS:
    f = f"{M}/{m}/🧪️tests/🔬️unit/🦀️.rs"
print("touched", len(touched))

edit(B + "/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-pets-character = {', 'semio-framework-pets-staging-draft = { path = "../../../📝️draft/📦️packages/🦀️rust" }\nsemio-framework-pets-character = {')
edit(P + "/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-pets-staging = {', 'semio-framework-pets-staging-draft = { path = "../../🔨️modules/📝️draft/📦️packages/🦀️rust" }\nsemio-framework-pets-staging = {')
edit(P + "/🦀️.rs", "pub use semio_framework_pets_staging::spacing;", "pub use semio_framework_pets_staging_draft::spacing;")
edit(P + "/📦️packages/🦀️rust/📜️script.ts", '"semio-framework-pets-staging", "semio-framework-pets"]', '"semio-framework-pets-staging-draft", "semio-framework-pets-staging", "semio-framework-pets"]')
rgl.register(B + "/📦️packages/🦀️rust", "semio-framework-pets-staging", A, "semio-framework-pets-staging-draft", "@semio-tech/pets-staging-rs", "@semio-tech/pets-staging-draft-rs", "🧰️framework/🛍️products/🐾️pets/🔨️modules/📝️draft/📦️packages/🦀️rust", dep_ws=[])
