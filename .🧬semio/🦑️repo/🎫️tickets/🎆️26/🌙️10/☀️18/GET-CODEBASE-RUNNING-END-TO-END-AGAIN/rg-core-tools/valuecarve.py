import os, re, sys, json
sys.path.insert(0, os.path.dirname(__file__))
import rgl, deps, valuemap
R = rgl.ROOT
V = R + "/🧰️framework/🔨️modules/🌱️value"
GLUE = V + "/📦️packages/🦀️rust/🦀️.rs"
ROOTF = V + "/🦀️.rs"
FWWS = R + "/🧰️framework/Cargo.toml"

BASE = {
 "retained_clone": "semio_framework_value_retained_clone::retained_clone",
 "ValueError": "semio_framework_value_control_contract::ValueError",
 "ValueRefusalKind": "semio_framework_value_control_contract::ValueRefusalKind",
 "RetirementDemand": "semio_framework_value_retirement::RetirementDemand",
 "retirement": "semio_framework_value_retirement::retirement",
}

def manifest(name, libname, workspace_up, deps_lines, dev_lines, desc, ident):
    return f'''[package]
workspace = "{workspace_up}"
name = "{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{desc}"

[package.metadata.semio]
role = "framework"
id = "{ident}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
name = "{libname}"
path = "🦀️.rs"

[dependencies]
{deps_lines}

[dev-dependencies]
{dev_lines}
'''

def swap(path, old, new, cnt=1):
    s = rgl.read(path)
    assert s.count(old) == cnt, (path[-50:], old[:70], s.count(old))
    rgl.write(path, s.replace(old, new))

def drop_mount(path, mount_rel, modname):
    s = rgl.read(path)
    pat = re.compile(r'#\[path\s*=\s*"' + re.escape(mount_rel) + r'"\]\s*(?:pub )?mod ' + modname + r'\s*;\n?')
    s2, c = pat.subn("", s)
    assert c == 1, (modname, c)
    rgl.write(path, s2)
