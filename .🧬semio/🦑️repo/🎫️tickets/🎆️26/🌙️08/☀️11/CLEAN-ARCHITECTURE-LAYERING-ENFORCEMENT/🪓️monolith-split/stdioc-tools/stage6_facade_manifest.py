#!/usr/bin/env python3
# 📦️ Stage 6: rewrites the facade manifest over the producer crates and registers every producer as a member of the stdio workspace.
import os
import re
import sys

sys.path.insert(0, os.path.dirname(__file__))
from lib import *  # noqa: E402,F401,F403
from stage4_manifests import DEPS, DEV, spec_line, sibling_line  # noqa: E402

pkgdir = C + "/📦️packages/🦀️rust"
facade_deps = ["semio-framework-diagnostic", "semio-framework-os-kernel", "semio-framework-pack", "semio-framework-ui-locale", "semio-framework-value"]
lines = [
    "[package]",
    'workspace = "../../../.."',
    f'name = "{FACADE}"',
    "version.workspace = true",
    "edition.workspace = true",
    "rust-version.workspace = true",
    'description = "Shared schema, assembly, and native codec contract for stdio artifacts"',
    "",
    "[package.metadata.semio]",
    'role = "s-module"',
    "",
    "[lints]",
    "workspace = true",
    "",
    "[lib]",
    'path = "../../🦀️.rs"',
    "",
    "[features]",
    f'testing = ["dep:{BY_KEY["testing"]["package"]}"]',
    "",
    "[dependencies]",
]
rows = [(name, spec_line(pkgdir, name, DEPS[name])) for name in facade_deps]
for crate in CRATES:
    if crate["key"] == "testing":
        rows.append((crate["package"], sibling_line(pkgdir, crate).replace(" }", ", optional = true }")))
    else:
        rows.append((crate["package"], sibling_line(pkgdir, crate)))
rows.sort(key=lambda row: row[0])
lines.extend(line for _, line in rows)
lines += ["", "[[test]]", 'name = "details_arena_headroom"', 'path = "../../🧪️tests/🎟️details-arena-headroom/🦀️.rs"', "", "[[test]]", 'name = "table_arena_headroom"', 'path = "../../🧪️tests/📊️table-arena-headroom/🦀️.rs"', "", "[dev-dependencies]"]
tokens = set()
for rel in ("🧪️tests/🎟️details-arena-headroom/🦀️.rs", "🧪️tests/📊️table-arena-headroom/🦀️.rs"):
    tokens |= external_tokens(read(C + "/" + rel))
dev_rows = []
for token in sorted(tokens):
    spec = DEPS.get(token) or DEV.get(token)
    assert spec, token
    dev_rows.append(spec_line(pkgdir, token, spec, spec.get("features")))
lines.extend(dev_rows)
lines.append("")
write(pkgdir + "/Cargo.toml", "\n".join(lines))
print("facade dev-deps:", sorted(tokens))

workspace = read(STDIO + "/Cargo.toml")
match = re.search(r"(?ms)^members = \[\n(.*?)^\]\n", workspace)
members = re.findall(r'"([^"]+)"', match.group(1))
for crate in CRATES:
    entry = os.path.relpath(crate["pkgdir"], STDIO)
    if entry not in members:
        members.append(entry)
members.sort(key=lambda value: value.encode("utf8"))
block = "members = [\n" + "".join(f'    "{value}",\n' for value in members) + "]\n"
workspace = workspace[:match.start()] + block + workspace[match.end():]
write(STDIO + "/Cargo.toml", workspace)
print("members", len(members))
