#!/usr/bin/env python3
"""Apply from the repo root in a patch window. store-cursor's lib test target compiles `cfg(test)` serde derives on
ArtifactCursorOwners, but HistoryPageStack's serde impls are gated on vcs `testkit` (only enabled by cursor's own `testkit`
feature, not under plain `cargo test -p`). Fail-closed: exactly one [dev-dependencies] anchor."""
p="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧭️cursor/📦️packages/🦀️rust/Cargo.toml"
s=open(p).read()
A='[dev-dependencies]\nserde = { version = "1.0.219", features = ["derive"] }'
N='[dev-dependencies]\nsemio-framework-os-vcs = { path = "../../../../🌿️vcs/📦️packages/🦀️rust", features = ["testkit"] }\nserde = { version = "1.0.219", features = ["derive"] }'
assert s.count(A)==1 and 'features = ["testkit"] }' not in s.split('[dev-dependencies]')[1]
open(p,'w').write(s.replace(A,N))
