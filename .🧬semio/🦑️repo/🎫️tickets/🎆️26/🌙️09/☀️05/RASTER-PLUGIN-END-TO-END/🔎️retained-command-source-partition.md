# Retained Command Source Partition

Actual artifact-app-testing plus unit closure is 294,358 bytes against 262,144, requiring at least 32,214 bytes removed from this real closure. Production is 251,267 bytes and artifact-app-testing 258,967 bytes, so the latter has only 3,177 bytes of headroom. The test-only closure delta is 35,391 bytes; moving a single test file cannot solve it.

Largest test-only owners and their included fixtures are:

- 🧪️tests/🔬️unit/🦀️.rs: 13,229 bytes; two included fixtures total 3,370 bytes.
- ♻️metadata/🧪️tests/🦀️.rs: 5,212 bytes; fixture 952 bytes.
- 📸️checkpoint/👣️cursor/🧪️tests/🦀️.rs: 4,683 bytes; fixture 699 bytes.
- 🎟️admission/🧪️tests/🦀️.rs: 1,419 bytes; fixture 1,537 bytes.
- 🏭️factory/🧪️tests/🦀️.rs: 1,098 bytes; fixture 3,192 bytes.

All paths are relative to 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command. The primary unit suite imports private checkpoint header, checkpoint type, encoder, decoder, and fault-size constant; admission tests call pub(super) admit_limits/admit_raw; factory tests call pub(super) quote. Directly mounting those tests in a companion crate will fail unless the real ownership boundary is refactored or an intentional test-feature interface is provided. Metadata laws primarily exercise public declaration-view and controlled-retirement types; checkpoint cursor is already public. Those two suites and fixtures alone remove only about 11.5 KB.

The existing declaration-app companion in plugin/📜️declaration/🧩️app/🧪️tests/🥣️companion has its own Cargo.toml and dev-depends on the real owner, mounts original laws with #[path], and installs the first-party HeapWitness as global allocator. That is a concrete companion registration pattern, but almost all five suites must be moved to fit this closure unless an independent production owner is also repartitioned. Keep all moved laws and their fixture access executable. No cap change or native build was made.
