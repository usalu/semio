# Geometry Hand Review

Scope: `🧰️framework/🔨️modules/📐️geometry`. No nested `AGENTS.md` exists; shared framework instructions apply. Root, engine, random generator, package, and Rust source names are already individually meaningful and sibling-unique.

One actual missing-emoji leaf was found: `📦️packages/🦀️rust/tests/first_party_geometry.rs`. Its five integration tests compare first-party points/vectors, affine transforms, primitive and adaptive paths, and Bézier bounds with the independent Kurbo implementation. The handpicked filename is `📐️first_party_geometry.rs`, preserving every existing stem character and source byte. The logical Cargo test identity stays `first_party_geometry` through an explicit `[[test]]` path and `autotests = false`, avoiding an emoji-derived Rust crate identifier. Literal `tests` and `Cargo.toml` remain tool-reserved names.

The exact move is applied. Before and after SHA-256: `49c66ad19868980955f2fbfd9ba9305167aa99b3935b01a0966b84fb474b87a6`. The audit covers 10 physical entries, 8 governed entries, zero naming violations, and zero unresolved directory roles. No geometry API, algorithm, or other agent's source work changed.

The actual explicitly mounted Cargo integration target passed all five tests, including independent Kurbo comparisons. The broader package test build fails on 49 preexisting E0277 errors in `🎲️random/🦀️.rs` tests that await synchronous `Rng`, `SplitMix64`, integer, and float return values. These API/test inconsistencies are not rename failures and were not weakened or rewritten. The production library and the renamed integration test compiled successfully.
