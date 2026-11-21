# GLB metadata object access staging audit

Attempt 28 (driver `wxXXO4R`) reached the mesh engine and reported missing `Object::member_at` and `Object::swap_members` in the captured JSON dependency.

The shared JSON object owner now exposes both methods with the signatures used by GLB metadata. `member_at` borrows a vector entry by index. `swap_members` validates both indices and exchanges existing vector entries in place. The mesh engine's direct JSON dependency resolves to the manifest that mounts the authoritative JSON source. That source publicly exports the object module. [JSON object owner](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🧱️object-and-value/🦀️.rs:34)

The metadata cursor checks a grant for one item, sufficient copy bytes and depth, and one work unit before it advances. Each turn inspects at most one indexed member of a long object, checks a fixed set of recognized fields, and swaps at most one pair of slots. Known fields are partitioned into the reader's bounded prefix. The scene forwards the actual swap receipt together with its other retained progress. [GLB metadata cursor](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏗️mesh-engine/🚪️io/💾️binary/🌳️glb-scene/🗂️metadata/🦀️.rs:66)

No source edits or competing Rust builds were needed for this audit. Fresh Note guest staging will verify Rust linkage against the updated JSON crate.
