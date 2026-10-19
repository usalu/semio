import os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
from mkcomp import rebase, dep_manifest, lib_ident_of, used_in, rebase_fixtures
write = "--write" in sys.argv
PL = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
INST = PL + "/🧵️instance"
IMAN = INST + "/📦️packages/🦀️rust"
T = PL + "/♻️lifecycle/🏪️stores/🧪️tests"
D = T + "/📦️packages/🦀️rust"
WS = R
NAME = "semio-framework-plugin-store-stage-laws"
t = rd(T + "/🦀️.rs")
iman = rd(IMAN + "/Cargo.toml")
sec = re.search(r"(?ms)^\[dependencies\]\n(.*?)(?=^\[|\Z)", iman).group(1)
deps = []
for l in sec.split("\n"):
    m = re.match(r"^([\w\-]+)(\.workspace)?\s*=", l)
    if not m or not l.strip(): continue
    key = m.group(1)
    names = {key.replace("-", "_")}
    pk = re.search(r'package = "([^"]+)"', l)
    if pk: names.add(pk.group(1).replace("-", "_"))
    dm = dep_manifest(l, IMAN, WS, key)
    if dm and os.path.exists(dm): names.add(lib_ident_of(dm))
    if any(used_in(n, t) for n in names): deps.append(rebase(l, IMAN, D))
# dev-dependencies of the owner used by the file
dsec = re.search(r"(?ms)^\[dev-dependencies\]\n(.*?)(?=^\[|\Z)", iman)
dev = []
if dsec:
    for l in dsec.group(1).split("\n"):
        m = re.match(r"^([\w\-]+)(\.workspace)?\s*=", l)
        if not m or not l.strip(): continue
        key = m.group(1); names = {key.replace("-", "_")}
        pk = re.search(r'package = "([^"]+)"', l)
        if pk: names.add(pk.group(1).replace("-", "_"))
        dm = dep_manifest(l, IMAN, WS, key)
        if dm and os.path.exists(dm): names.add(lib_ident_of(dm))
        if any(used_in(n, t) for n in names) and key not in [d.split(" ")[0] for d in deps]: dev.append(rebase(l, IMAN, D))
deps = [d.replace(", optional = true", "") for d in deps]
deps.append(rebase(re.search(r"(?m)^semio-framework-os-kernel = .*$", iman).group(0), IMAN, D))
print("deps", [d.split(" ")[0] for d in deps], "dev", [d.split(" ")[0] for d in dev])
man = '''[package]
workspace = "%s"
name = "%s"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Companion laws of the plugin store lifecycle: the original disposer body and its final box release share the neutral close corpus."
publish = false

[package.metadata.semio]
role = "framework"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
%s
[dev-dependencies]
%s
''' % (os.path.relpath(WS, D), NAME, "\n".join(deps) + "\n", "\n".join(dev) + ("\n" if dev else ""))
head = "//! 🧪️ Actual original disposer body and final Box release share the neutral close corpus.\n"
assert t.startswith(head)
t2 = head + "#![cfg(test)]\n\nextern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as store;\nextern crate semio_framework_os_kernel as pack;\n" + t[len(head):]
iroot = rd(INST + "/🦀️.rs")
mount = '#[cfg(any(test, feature = "artifact-app-testing"))]\n#[path = "../♻️lifecycle/🏪️stores/🧪️tests/🦀️.rs"]\npub mod original_store_stage_tests;\n'
assert mount in iroot
if not write:
    print(man); print(t2[:700]); sys.exit(0)
wr(T + "/🦀️.rs", t2)
os.makedirs(D, exist_ok=True); assert not os.path.exists(D + "/Cargo.toml")
open(D + "/Cargo.toml", "w", encoding="utf8").write(man)
rebase_fixtures([T + "/🦀️.rs"], INST + "/📦️packages/🦀️rust", D)
wr(INST + "/🦀️.rs", iroot.replace(mount, "", 1))
add_member(WS, D, IMAN)
add_owner(NAME, D)
print("done")
