"""📦 OSD: writes the five new manifests and project files and rewrites the facade manifest (idempotent: refuses when a target exists with other content)."""
import os, re
ROOT = "/Users/ueli/Documents/semio"
M = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
OLD = M + "/📦️packages/🦀️rust"
old_text = open(OLD + "/Cargo.toml", encoding="utf-8").read()

def ext_dir(line_path):
    return os.path.normpath(os.path.join(OLD, line_path))

EXT = {}
for m in re.finditer(r'^(\S+) = \{ path = "([^"]+)"(?:, package = "([^"]+)")?(?:, [^}]*)? \}$', old_text, re.M):
    EXT[m.group(1)] = (ext_dir(m.group(2)), m.group(3))
EXT_SERDE = 'serde = { version = "1.0.219", features = ["derive"] }'
EXT_SERDE_JSON = 'serde_json = "1.0.140"'

PKGS = {
    "browser-actor": dict(node=M + "/🧬️schema/🌐️browser-actor", desc="Directory browser actor identities (leaf of the wire contract)", deps=["semio-framework-value", "protocol", "semio-framework-value-derive"]),
    "schema": dict(node=M + "/🧬️schema", desc="Directory wire contract core and its canonical text IO", deps=["semio-framework-value", "protocol", "semio-framework-value-derive", "semio-framework-artifact-reference", "semio-framework-os-channel", "semio-framework-os-identity", "semio-framework-plugin-component-receipt", "semio-framework-hash", "semio-framework-pack-json", "os-directory-browser-actor", "SERDE"]),
    "check-in": dict(node=M + "/🧬️schema/📌️document-check-in-v1", desc="Directory document Check In wire contract and its canonical text IO", deps=["semio-framework-value", "semio-framework-value-derive", "semio-framework-pack-json", "os-directory-schema", "SERDE"]),
    "binary": dict(node=M + "/🚪️io/🧱️binary", desc="Directory native binary boundaries: artifact hash, descriptor digest and the canonical checkpoint pair", deps=["semio-framework-hash", "os-directory-schema"]),
    "access-policy": dict(node=M + "/🛡️access-policy", desc="Directory declared access policy and its decision authority", deps=["SERDE", "SERDE_JSON"]),
}
NEWDEP = {"os-directory-browser-actor": "browser-actor", "os-directory-schema": "schema"}

def pkg_dir(key):
    return PKGS[key]["node"] + "/📦️packages/🦀️rust"

def depth(path):
    return len(os.path.relpath(path, ROOT).split(os.sep))

def rel(frm, to):
    return os.path.relpath(to, frm)

def manifest(key):
    P = PKGS[key]
    d = pkg_dir(key)
    up = "/".join([".."] * depth(d))
    lines = [
        "[package]", f'workspace = "{up}"', f'name = "semio-framework-os-directory-{key}"', 'version = "0.1.0"', 'edition = "2021"', "rust-version.workspace = true", f'description = "{P["desc"]}"', "",
        "[package.metadata.semio]", 'role = "framework"', f'id = "os-directory-{key}"', "",
        "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
        "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "", "[dependencies]",
    ]
    for dep in P["deps"]:
        if dep == "SERDE":
            lines.append(EXT_SERDE)
        elif dep == "SERDE_JSON":
            lines.append(EXT_SERDE_JSON)
        elif dep in NEWDEP:
            lines.append(f'semio-framework-{dep} = {{ path = "{rel(d, pkg_dir(NEWDEP[dep]))}" }}')
        elif dep == "protocol":
            target, package = EXT[dep]
            lines.append(f'protocol = {{ path = "{rel(d, target)}", package = "{package}" }}')
        else:
            target, _ = EXT[dep]
            lines.append(f'{dep} = {{ path = "{rel(d, target)}" }}')
    return "\n".join(lines) + "\n"

PROJECT = '''{
  "name": "@semio-tech/os-directory-%s-rs",
  "tags": [
    "lang:rust",
    "role:framework",
    "family:os-directory"
  ]
}
'''

def write_new(path, text):
    assert os.path.realpath(path).startswith(os.path.realpath(M) + os.sep)
    if os.path.exists(path):
        assert open(path, encoding="utf-8").read() == text, f"exists with other content: {path}"
        return
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)

for key in PKGS:
    write_new(pkg_dir(key) + "/Cargo.toml", manifest(key))
    write_new(pkg_dir(key) + "/📋️project.json", PROJECT % key)

facade = '''[package]
workspace = "../../../../../../.."
name = "semio-framework-os-directory"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Directory event projection and the public facade over the directory crates"

[package.metadata.semio]
role = "framework"
id = "os-directory"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[features]
default = []

[dependencies]
'''
for dep in ["semio-framework-value", "semio-framework-async"]:
    target, _ = EXT[dep]
    facade += f'{dep} = {{ path = "{rel(OLD, target)}" }}\n'
for key in ["schema", "check-in", "binary", "access-policy"]:
    facade += f'semio-framework-os-directory-{key} = {{ path = "{rel(OLD, pkg_dir(key))}" }}\n'
cur = open(OLD + "/Cargo.toml", encoding="utf-8").read()
assert cur == old_text
open(OLD + "/Cargo.toml", "w", encoding="utf-8").write(facade)
for key in PKGS:
    print("---", key); print(manifest(key))
