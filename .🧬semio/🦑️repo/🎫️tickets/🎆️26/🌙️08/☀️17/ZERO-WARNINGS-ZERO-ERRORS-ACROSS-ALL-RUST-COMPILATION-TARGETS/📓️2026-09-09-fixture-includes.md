# Fixture Include Repairs

Native926 found includes targeting previous fixture locations. Resolved every replacement against an existing authored fixture and parsed the Rust sources before guarded writes. No fixtures were copied or regenerated.

- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs: `../🪪️mutation-leaf-descriptor/🧫️fixtures/🔣️.json` → `../../🧫️fixtures/🪪️mutation-leaf-descriptor/🔣️.json`
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🦀️.rs: `🧫️fixtures/🔣️.json` → `../../🧫️fixtures/🧬️mutation-laws/🔣️.json`


WASI retry941 and root scope942 stopped before compilation because the scale fixture package moved from os/🧫️fixtures to os/🧪️testkit while Cargo.toml still referenced the old directory twice. Validated the new manifest's package identity and updated the workspace member and workspace dependency paths in one guarded write.


Root verification scope945 passed after the scale Cargo path repair: three target vectors, two rejection vectors, and the independent Cargo catalog comparison covering 160 WASI packages.
