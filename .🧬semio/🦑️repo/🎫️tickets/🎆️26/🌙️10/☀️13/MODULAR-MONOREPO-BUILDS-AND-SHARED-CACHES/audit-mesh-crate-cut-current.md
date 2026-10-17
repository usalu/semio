# Mesh Crate Cut Independent Audit

No actionable findings in the reviewed first crate cut. This is a read-only Source/evidence audit; it does not qualify Rust compilation or runtime behavior.

## Scope and Evidence

- Root AGENTS.md and the applicable 3D AGENTS.md were read. No nested mesh instruction file exists. Only this audit Markdown is authored; no Source, Git state, launcher, Cargo lock, or report evidence was changed.
- Report SHA-256 before and after: `6ed814fbec37238dd5b5b42233e3c016f3ee4574091fcef00c9ff3d46f0570e2` (matches requested `6ed814fbec37238dd5b5b42233e3c016f3ee4574091fcef00c9ff3d46f0570e2`).
- All 16 listed Source files and all 14 retained logs match their frozen hashes on both observations. No observed shared workspace/launcher coedit needs attribution.
- Original Clip body and retirement incidence were compared against read-only HEAD Source; this is a comparison baseline, not a fabricated Original run. Twelve original dynamic buffers remain in both retirement and birth-byte ledgers.

## Ownership and Semantics

Canonical Vec3, ids, enums, error/result, soup, source vertex and ranked edge definitions live in the contract crate. The mesh aggregate reexport is intentional. HalfedgeMesh remains with its private mutation fields and inherent methods; no foreign inherent impl or facade-return edge was introduced. The clipping crate owns its real retained phases and private buffers; Bevel imports MeshClipJob and passes its actual soup. Bevel phases 7–8 consume each completed face/corner/weight allocation once and query vertex sources using Build.origin_vertices. Completion closes advance; each transfer bit is separate and rejected before completion or after its own transfer. The original half-plane predicate, f32 interpolation and f64 provenance promotion are preserved. Deduplication remains map-owned, with at most an extra constant-size candidate calculation. Parent Work/snapshot/retirement paths remain present.

The direct manifest edges are 3D → clipping, 3D → contract, clipping → contract, plus existing firstparty Value/MeshEngine/derive dependencies. A read-only traversal of production path/workspace dependency tables reachable from the two new roots found 82 firstparty manifests, no production cycle, and no return to semio-framework-3d. This is literal manifest evidence; no Cargo metadata/resolution was executed. Dev-dependency cycles are outside that production-edge claim.

New contract closure: 1 files / 6848 bytes / 140 nonblank lines. New clipping closure including authored cfg(test) child: 2 files / 11726 bytes / 153 nonblank lines. Both retain 262144-byte / 32-file / 4000-line limits.

The old 3D three-file lower bound remains **340210 bytes**, exceeding 262144 by **78066**; its full mounted closure is larger. No full no-giant-crate success is established. Rust remains **uncompiled**, authored Rust laws **unrun**, and Cargo.lock **unrefreshed**. Tree-sitter/TypeScript evidence cannot establish type resolution, macro expansion, allocation bounds, compiler memory or native runtime behavior. Empty types3 logs support empty diagnostics only; terminal exit collection is stated in the author report and cannot be independently reconstructed from those empty files alone.

Green6 prototype/release retained logs each contain 3 pass / 0 fail / 80 assertions, and the separate prerequisite 10 pass / 216 assertions. RED1 retains output-cap failure; RED2 retains absent-cut failure; intermediate failures are not rewritten as passes. No tests, compiler, native provider, cache, network, Cargo or build action was run for this audit.

## Source and Retained Log Inventory

| Path | Bytes | Before SHA-256 | After SHA-256 |
| --- | ---: | --- | --- |
| `🧰️framework/Cargo.toml` | 40677 | `ac530aa5668fd4524340cb3f2d4646ffdd8823d57f2379e247266f91ad616508` | `ac530aa5668fd4524340cb3f2d4646ffdd8823d57f2379e247266f91ad616508` |
| `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | 1981 | `7ca87a03d766b95cc2d001b262a0967e2a04ae6283d69d171fb6d14b104a28a3` | `7ca87a03d766b95cc2d001b262a0967e2a04ae6283d69d171fb6d14b104a28a3` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` | 79675 | `0ee0b72c2a5873bee7dd68e95b7e58a579635059cf405912ede0201845a6fee1` | `0ee0b72c2a5873bee7dd68e95b7e58a579635059cf405912ede0201845a6fee1` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` | 257755 | `1dc05ff7fc3114e0ec869edf1b7439e1e433a8169e27856bc6cbe11007b1a563` | `1dc05ff7fc3114e0ec869edf1b7439e1e433a8169e27856bc6cbe11007b1a563` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️contract/🦀️.rs` | 6848 | `6e0a60c777562b5e655a3d21750fe371110d42545e1c88eb87110133b5741732` | `6e0a60c777562b5e655a3d21750fe371110d42545e1c88eb87110133b5741732` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️contract/📦️packages/🦀️rust/Cargo.toml` | 607 | `68ac7810849489612c2527029b726f869ecbe83131f4d08d1394e96c3a10da0f` | `68ac7810849489612c2527029b726f869ecbe83131f4d08d1394e96c3a10da0f` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🦀️.rs` | 9777 | `e21618b7bb6c5351d220227413e01658c07465a96f61330b75b3abf326b86f8b` | `e21618b7bb6c5351d220227413e01658c07465a96f61330b75b3abf326b86f8b` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/📦️packages/🦀️rust/Cargo.toml` | 709 | `694d62f9589a89da796d81e2e64ef6036067518d1b3bb66a8ca318cead364570` | `694d62f9589a89da796d81e2e64ef6036067518d1b3bb66a8ca318cead364570` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🟦️.ts` | 917 | `87a1f34b25e4ca15c6ac50cfa940c68434cf907f9cd14a92cf680a582139c76f` | `87a1f34b25e4ca15c6ac50cfa940c68434cf907f9cd14a92cf680a582139c76f` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧬️schema/🔣️.json` | 1855 | `de3b05b64debf00be48c68df37e67c56261e5d3e07957b30a2b8279815aa1e11` | `de3b05b64debf00be48c68df37e67c56261e5d3e07957b30a2b8279815aa1e11` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🔣️.json` | 2241 | `3f5324ddcbf4d2c8e01db1fe3c7ba5bd540147652ab6c686de99353586567279` | `3f5324ddcbf4d2c8e01db1fe3c7ba5bd540147652ab6c686de99353586567279` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts` | 5902 | `0c9a79c5e2fce6c1a0aa71fa953485c9c128a9509373bd0e8c33a389b214cfc4` | `0c9a79c5e2fce6c1a0aa71fa953485c9c128a9509373bd0e8c33a389b214cfc4` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🦀️.rs` | 1949 | `a4ffb8c6008f61ab1f8293cdf5d6f765c22d83788a2c9caf23f1d7347c87ca1c` | `a4ffb8c6008f61ab1f8293cdf5d6f765c22d83788a2c9caf23f1d7347c87ca1c` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | 88000 | `3a08d6d0310063c43cb932efac5920af524721e0f469d654843425a5398e9029` | `3a08d6d0310063c43cb932efac5920af524721e0f469d654843425a5398e9029` |
| `.vscode/launch.json` | 8733161 | `bb0bf04a644a3f71a55a69e74d162a9e7adfff9582f45d06d937accf95e53f40` | `bb0bf04a644a3f71a55a69e74d162a9e7adfff9582f45d06d937accf95e53f40` |
| `.vscode/🧩️launch.seed.jsonc` | 6624444 | `e9efe9e0f8ff9d871af207fc11a13292a88ebeaccd812f63cc03ee6de457f85a` | `e9efe9e0f8ff9d871af207fc11a13292a88ebeaccd812f63cc03ee6de457f85a` |
| `🗑️generated/mesh-crate-green1-prototype.log` | 2709 | `894dc864f9035fdcc71294e1955628b1f81b9e0891883af03a763102f8f53427` | `894dc864f9035fdcc71294e1955628b1f81b9e0891883af03a763102f8f53427` |
| `🗑️generated/mesh-crate-green2-prototype.log` | 2806 | `0ffe2c24824d9453bf420627ac08181aa96c41b54bb60e124cc4d3caaa6dd0b1` | `0ffe2c24824d9453bf420627ac08181aa96c41b54bb60e124cc4d3caaa6dd0b1` |
| `🗑️generated/mesh-crate-green3-prototype.log` | 3789 | `fabbfc595345aedcbfe4c6e8b7abe0431551af486431769be48f7b6c3606bff4` | `fabbfc595345aedcbfe4c6e8b7abe0431551af486431769be48f7b6c3606bff4` |
| `🗑️generated/mesh-crate-green4-prototype.log` | 8502 | `0b5a2bd30d33d178fc7b4bc037bdf8b8b199d8d64b9c77a1504c80a9c00b99e0` | `0b5a2bd30d33d178fc7b4bc037bdf8b8b199d8d64b9c77a1504c80a9c00b99e0` |
| `🗑️generated/mesh-crate-green5-prototype.log` | 2808 | `18410b33de9a2f439a956498ccd48e331b65c47214f7ca8c4dc16f696d4eafa2` | `18410b33de9a2f439a956498ccd48e331b65c47214f7ca8c4dc16f696d4eafa2` |
| `🗑️generated/mesh-crate-green5-release.log` | 2807 | `03e4f70103b89c6c9f3e351b9e8ad93f06e5b8106b69b72669cd8753af219072` | `03e4f70103b89c6c9f3e351b9e8ad93f06e5b8106b69b72669cd8753af219072` |
| `🗑️generated/mesh-crate-green6-prototype.log` | 2806 | `227312ee16d995750811c7f8f111120136a9e6865de518ab3c6a8fad90263a6e` | `227312ee16d995750811c7f8f111120136a9e6865de518ab3c6a8fad90263a6e` |
| `🗑️generated/mesh-crate-green6-release.log` | 2809 | `793765715750fd16a4e8ed911720d2ef9ad54fe9568c54e2cad2bfb401a504ed` | `793765715750fd16a4e8ed911720d2ef9ad54fe9568c54e2cad2bfb401a504ed` |
| `🗑️generated/mesh-crate-red1-prototype.log` | 5939 | `c57a8afe3d5979e51ed202de601e9c26e5df102e666b23761a11bd1362b0db7e` | `c57a8afe3d5979e51ed202de601e9c26e5df102e666b23761a11bd1362b0db7e` |
| `🗑️generated/mesh-crate-red2-prototype.log` | 7713 | `316e12a728772d543cfdd1713598bb89cf411babb909f5806f799a082213c12a` | `316e12a728772d543cfdd1713598bb89cf411babb909f5806f799a082213c12a` |
| `🗑️generated/mesh-crate-types1-prototype.log` | 2372 | `07da52f4aecd820c26e26e8046acf4878fc3a16b226a013540b4180b0762e875` | `07da52f4aecd820c26e26e8046acf4878fc3a16b226a013540b4180b0762e875` |
| `🗑️generated/mesh-crate-types2-prototype.log` | 3537 | `7df71369309dd0ec948fa4c03efee3a1e3b603504638aff25b55f3adddc119b9` | `7df71369309dd0ec948fa4c03efee3a1e3b603504638aff25b55f3adddc119b9` |
| `🗑️generated/mesh-crate-types3-prototype.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `🗑️generated/mesh-crate-types3-release.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

## Literal Caller Inventory

Independent repository literal census reproduces all 46 Rust caller files, without claiming resolved dispatch or cfg-selected compilation.

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🌗️mesh-shading/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔢️math-values/🧪️tests/🎟️original/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✅️analysis-check/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🚀️original-field/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🚀️original-field/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✏️mesh-edit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧪️tests/🧰️oracle-support/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/💎️value/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔌️inputs/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🎚️mesh-component/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/↔️mesh-transform/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🥽️mesh-primitive/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🧪️tests/🧰️mesh-harness/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔎️mesh-inspect/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🗺️mesh-uv/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🎟️original/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🎟️original/📐️read/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔀️mesh-convert/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry/📼️mesh-interchange/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔷️mesh-edit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧵️uv/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔌️mounted/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧭️normal-channels/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🕸️mesh/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs`

## Traversed Production Edges

- `semio-framework-3d-mesh-clipping` → `semio-framework-3d-mesh-contract`
- `semio-framework-3d-mesh-clipping` → `semio-framework-mesh-engine`
- `semio-framework-3d-mesh-clipping` → `semio-framework-value`
- `semio-framework-3d-mesh-contract` → `semio-framework-value`
- `semio-framework-3d-mesh-contract` → `semio-framework-value-derive`
- `semio-framework-artifact-reference` → `semio-framework-artifact-reference-borrow`
- `semio-framework-artifact-reference` → `semio-framework-dsl-record`
- `semio-framework-artifact-reference` → `semio-framework-pack-json`
- `semio-framework-artifact-reference` → `semio-framework-schema-registry`
- `semio-framework-artifact-reference` → `semio-framework-value`
- `semio-framework-async` → `semio-framework-trace`
- `semio-framework-async` → `semio-framework-value`
- `semio-framework-deflate` → `semio-framework-value`
- `semio-framework-diagnostic` → `semio-framework-pack-json`
- `semio-framework-diagnostic` → `semio-framework-value`
- `semio-framework-dsl` → `semio-framework-diagnostic`
- `semio-framework-dsl` → `semio-framework-dsl-lexical`
- `semio-framework-dsl` → `semio-framework-dsl-token`
- `semio-framework-dsl` → `semio-framework-value`
- `semio-framework-dsl-lexical` → `semio-framework-diagnostic`
- `semio-framework-dsl-lexical` → `semio-framework-dsl-token`
- `semio-framework-dsl-lexical` → `semio-framework-value-control-contract`
- `semio-framework-dsl-notation` → `semio-framework-diagnostic`
- `semio-framework-dsl-notation` → `semio-framework-dsl`
- `semio-framework-dsl-notation` → `semio-framework-value-control-contract`
- `semio-framework-dsl-record` → `semio-framework-diagnostic`
- `semio-framework-dsl-record` → `semio-framework-dsl`
- `semio-framework-dsl-record` → `semio-framework-dsl-notation`
- `semio-framework-dsl-record` → `semio-framework-dsl-record-emission`
- `semio-framework-dsl-record` → `semio-framework-dsl-record-model`
- `semio-framework-dsl-record` → `semio-framework-io-control`
- `semio-framework-dsl-record` → `semio-framework-pack-json`
- `semio-framework-dsl-record` → `semio-framework-value`
- `semio-framework-dsl-record-emission` → `semio-framework-diagnostic`
- `semio-framework-dsl-record-emission` → `semio-framework-dsl`
- `semio-framework-dsl-record-emission` → `semio-framework-dsl-notation`
- `semio-framework-dsl-record-emission` → `semio-framework-dsl-record-model`
- `semio-framework-dsl-record-emission` → `semio-framework-value`
- `semio-framework-dsl-record-model` → `semio-framework-diagnostic`
- `semio-framework-dsl-record-model` → `semio-framework-dsl`
- `semio-framework-dsl-record-model` → `semio-framework-io-control`
- `semio-framework-dsl-record-model` → `semio-framework-pack-json`
- `semio-framework-dsl-record-model` → `semio-framework-value`
- `semio-framework-dsl-token` → `semio-framework-diagnostic`
- `semio-framework-io-control` → `semio-framework-value`
- `semio-framework-io-schema` → `semio-framework-artifact-reference`
- `semio-framework-io-schema` → `semio-framework-diagnostic`
- `semio-framework-io-schema` → `semio-framework-dsl-record`
- `semio-framework-io-schema` → `semio-framework-schema-registry`
- `semio-framework-io-schema` → `semio-framework-value`
- `semio-framework-io-schema` → `semio-framework-value-derive`
- `semio-framework-io-sqlite-model` → `semio-framework-value`
- `semio-framework-io-sqlite-snapshot` → `semio-framework-io-control`
- `semio-framework-io-sqlite-snapshot` → `semio-framework-io-sqlite-model`
- `semio-framework-io-sqlite-snapshot` → `semio-framework-value`
- `semio-framework-job` → `semio-framework-async`
- `semio-framework-job` → `semio-framework-diagnostic`
- `semio-framework-job` → `semio-framework-dsl-record`
- `semio-framework-job` → `semio-framework-trace`
- `semio-framework-job` → `semio-framework-value`
- `semio-framework-job` → `semio-framework-value-shared-issuance`
- `semio-framework-mesh-component-references` → `semio-framework-pack-json`
- `semio-framework-mesh-component-references` → `semio-framework-value`
- `semio-framework-mesh-engine` → `pack`
- `semio-framework-mesh-engine` → `protocol`
- `semio-framework-mesh-engine` → `semio-framework-dsl-record`
- `semio-framework-mesh-engine` → `semio-framework-io-base64`
- `semio-framework-mesh-engine` → `semio-framework-job`
- `semio-framework-mesh-engine` → `semio-framework-mesh-component-references`
- `semio-framework-mesh-engine` → `semio-framework-pack-json`
- `semio-framework-mesh-engine` → `semio-framework-value`
- `semio-framework-mesh-engine` → `semio-framework-value-derive`
- `semio-framework-mutation-diff` → `semio-framework-value-intrinsic`
- `semio-framework-mutation-diff` → `semio-framework-value-retirement`
- `semio-framework-native-build-compiler-resources` → `semio-framework-hash`
- `semio-framework-pack` → `semio-framework-diagnostic`
- `semio-framework-pack` → `semio-framework-dsl-record`
- `semio-framework-pack` → `semio-framework-hash`
- `semio-framework-pack` → `semio-framework-io-schema`
- `semio-framework-pack` → `semio-framework-io-sqlite-snapshot`
- `semio-framework-pack` → `semio-framework-pack-async`
- `semio-framework-pack` → `semio-framework-pack-error`
- `semio-framework-pack` → `semio-framework-pack-format`
- `semio-framework-pack` → `semio-framework-pack-format-catalog`
- `semio-framework-pack` → `semio-framework-pack-format-retained`
- `semio-framework-pack` → `semio-framework-pack-http`
- `semio-framework-pack` → `semio-framework-pack-intrinsic`
- `semio-framework-pack` → `semio-framework-pack-io`
- `semio-framework-pack` → `semio-framework-pack-json`
- `semio-framework-pack` → `semio-framework-pack-record`
- `semio-framework-pack` → `semio-framework-pack-record-encode`
- `semio-framework-pack` → `semio-framework-pack-record-preflight`
- `semio-framework-pack` → `semio-framework-pack-record-retained`
- `semio-framework-pack` → `semio-framework-pack-record-schema`
- `semio-framework-pack` → `semio-framework-replication-codec`
- `semio-framework-pack` → `semio-framework-value`
- `semio-framework-pack-async` → `semio-framework-async`
- `semio-framework-pack-async` → `semio-framework-pack-error`
- `semio-framework-pack-async` → `semio-framework-replication-codec`
- `semio-framework-pack-async` → `semio-framework-value`
- `semio-framework-pack-error` → `semio-framework-diagnostic`
- `semio-framework-pack-error` → `semio-framework-value`
- `semio-framework-pack-format` → `semio-framework-hash`
- `semio-framework-pack-format` → `semio-framework-pack-error`
- `semio-framework-pack-format` → `semio-framework-pack-source`
- `semio-framework-pack-format` → `semio-framework-replication-codec`
- `semio-framework-pack-format` → `semio-framework-value`
- `semio-framework-pack-format-catalog` → `semio-framework-hash`
- `semio-framework-pack-format-catalog` → `semio-framework-pack-error`
- `semio-framework-pack-format-catalog` → `semio-framework-pack-format`
- `semio-framework-pack-format-catalog` → `semio-framework-pack-format-retained`
- `semio-framework-pack-format-catalog` → `semio-framework-pack-source`
- `semio-framework-pack-format-catalog` → `semio-framework-replication-codec`
- `semio-framework-pack-format-catalog` → `semio-framework-value`
- `semio-framework-pack-format-retained` → `semio-framework-pack-error`
- `semio-framework-pack-format-retained` → `semio-framework-pack-format`
- `semio-framework-pack-format-retained` → `semio-framework-pack-source`
- `semio-framework-pack-format-retained` → `semio-framework-replication-codec`
- `semio-framework-pack-format-retained` → `semio-framework-value`
- `semio-framework-pack-http` → `semio-framework-async`
- `semio-framework-pack-http` → `semio-framework-pack-async`
- `semio-framework-pack-http` → `semio-framework-pack-error`
- `semio-framework-pack-http` → `semio-framework-replication-codec`
- `semio-framework-pack-intrinsic` → `semio-framework-hash`
- `semio-framework-pack-intrinsic` → `semio-framework-pack-error`
- `semio-framework-pack-intrinsic` → `semio-framework-pack-format`
- `semio-framework-pack-intrinsic` → `semio-framework-pack-format-catalog`
- `semio-framework-pack-intrinsic` → `semio-framework-pack-format-retained`
- `semio-framework-pack-intrinsic` → `semio-framework-pack-record`
- `semio-framework-pack-intrinsic` → `semio-framework-pack-record-retained`
- `semio-framework-pack-intrinsic` → `semio-framework-replication-codec`
- `semio-framework-pack-intrinsic` → `semio-framework-value`
- `semio-framework-pack-io` → `semio-framework-pack-error`
- `semio-framework-pack-io` → `semio-framework-pack-format`
- `semio-framework-pack-io` → `semio-framework-pack-source`
- `semio-framework-pack-io` → `semio-framework-replication-codec`
- `semio-framework-pack-io` → `semio-framework-value`
- `semio-framework-pack-json` → `semio-framework-value`
- `semio-framework-pack-record` → `semio-framework-async`
- `semio-framework-pack-record` → `semio-framework-diagnostic`
- `semio-framework-pack-record` → `semio-framework-dsl-record`
- `semio-framework-pack-record` → `semio-framework-pack-error`
- `semio-framework-pack-record` → `semio-framework-pack-format`
- `semio-framework-pack-record` → `semio-framework-pack-record-encode`
- `semio-framework-pack-record` → `semio-framework-pack-record-schema`
- `semio-framework-pack-record` → `semio-framework-replication-codec`
- `semio-framework-pack-record` → `semio-framework-value`
- `semio-framework-pack-record-encode` → `semio-framework-dsl-record`
- `semio-framework-pack-record-encode` → `semio-framework-pack-error`
- `semio-framework-pack-record-encode` → `semio-framework-pack-format`
- `semio-framework-pack-record-encode` → `semio-framework-pack-record-schema`
- `semio-framework-pack-record-encode` → `semio-framework-replication-codec`
- `semio-framework-pack-record-encode` → `semio-framework-value`
- `semio-framework-pack-record-preflight` → `semio-framework-dsl-record`
- `semio-framework-pack-record-preflight` → `semio-framework-hash`
- `semio-framework-pack-record-preflight` → `semio-framework-pack-format`
- `semio-framework-pack-record-preflight` → `semio-framework-pack-record-schema`
- `semio-framework-pack-record-preflight` → `semio-framework-replication-codec`
- `semio-framework-pack-record-preflight` → `semio-framework-value`
- `semio-framework-pack-record-retained` → `semio-framework-dsl-record`
- `semio-framework-pack-record-retained` → `semio-framework-pack-error`
- `semio-framework-pack-record-retained` → `semio-framework-pack-format-catalog`
- `semio-framework-pack-record-retained` → `semio-framework-pack-format-retained`
- `semio-framework-pack-record-retained` → `semio-framework-pack-record-schema`
- `semio-framework-pack-record-retained` → `semio-framework-replication-codec`
- `semio-framework-pack-record-retained` → `semio-framework-replication-wire-core`
- `semio-framework-pack-record-retained` → `semio-framework-value`
- `semio-framework-pack-record-schema` → `semio-framework-dsl-record`
- `semio-framework-pack-record-schema` → `semio-framework-hash`
- `semio-framework-pack-record-schema` → `semio-framework-pack-error`
- `semio-framework-pack-record-schema` → `semio-framework-pack-json`
- `semio-framework-pack-record-schema` → `semio-framework-pack-schema-index`
- `semio-framework-pack-record-schema` → `semio-framework-replication-codec`
- `semio-framework-pack-record-schema` → `semio-framework-value`
- `semio-framework-pack-schema-index` → `semio-framework-dsl-record`
- `semio-framework-pack-schema-index` → `semio-framework-value`
- `semio-framework-pack-source` → `semio-framework-pack-error`
- `semio-framework-replication` → `semio-framework-deflate`
- `semio-framework-replication` → `semio-framework-io-base64`
- `semio-framework-replication` → `semio-framework-mutation-diff`
- `semio-framework-replication` → `semio-framework-pack-error`
- `semio-framework-replication` → `semio-framework-replication-causal`
- `semio-framework-replication` → `semio-framework-replication-codec`
- `semio-framework-replication` → `semio-framework-replication-command-ingress`
- `semio-framework-replication` → `semio-framework-replication-conflict`
- `semio-framework-replication` → `semio-framework-replication-crypto`
- `semio-framework-replication` → `semio-framework-replication-edit`
- `semio-framework-replication` → `semio-framework-replication-error`
- `semio-framework-replication` → `semio-framework-replication-format`
- `semio-framework-replication` → `semio-framework-replication-frames`
- `semio-framework-replication` → `semio-framework-replication-history`
- `semio-framework-replication` → `semio-framework-replication-history-index`
- `semio-framework-replication` → `semio-framework-replication-ids`
- `semio-framework-replication` → `semio-framework-replication-interaction`
- `semio-framework-replication` → `semio-framework-replication-local-interaction`
- `semio-framework-replication` → `semio-framework-replication-mutation`
- `semio-framework-replication` → `semio-framework-replication-mutation-messages`
- `semio-framework-replication` → `semio-framework-replication-presence`
- `semio-framework-replication` → `semio-framework-replication-wire-core`
- `semio-framework-replication` → `semio-framework-value`
- `semio-framework-replication-causal` → `semio-framework-replication-codec`
- `semio-framework-replication-causal` → `semio-framework-replication-edit`
- `semio-framework-replication-causal` → `semio-framework-replication-error`
- `semio-framework-replication-causal` → `semio-framework-replication-ids`
- `semio-framework-replication-causal` → `semio-framework-replication-mutation`
- `semio-framework-replication-causal` → `semio-framework-replication-wire-core`
- `semio-framework-replication-causal` → `semio-framework-value`
- `semio-framework-replication-codec` → `semio-framework-deflate`
- `semio-framework-replication-codec` → `semio-framework-diagnostic`
- `semio-framework-replication-codec` → `semio-framework-io-base64`
- `semio-framework-replication-codec` → `semio-framework-pack-error`
- `semio-framework-replication-codec` → `semio-framework-replication-error`
- `semio-framework-replication-codec` → `semio-framework-value`
- `semio-framework-replication-command-ingress` → `semio-framework-diagnostic`
- `semio-framework-replication-command-ingress` → `semio-framework-trace`
- `semio-framework-replication-command-ingress` → `semio-framework-value`
- `semio-framework-replication-command-ingress` → `semio-framework-value-derive`
- `semio-framework-replication-conflict` → `semio-framework-diagnostic`
- `semio-framework-replication-conflict` → `semio-framework-hash`
- `semio-framework-replication-conflict` → `semio-framework-replication-causal`
- `semio-framework-replication-conflict` → `semio-framework-replication-ids`
- `semio-framework-replication-conflict` → `semio-framework-replication-mutation`
- `semio-framework-replication-conflict` → `semio-framework-replication-wire-core`
- `semio-framework-replication-conflict` → `semio-framework-value`
- `semio-framework-replication-crypto` → `semio-framework-replication-error`
- `semio-framework-replication-edit` → `semio-framework-hash`
- `semio-framework-replication-edit` → `semio-framework-pack-json`
- `semio-framework-replication-edit` → `semio-framework-replication-crypto`
- `semio-framework-replication-edit` → `semio-framework-replication-ids`
- `semio-framework-replication-edit` → `semio-framework-replication-mutation`
- `semio-framework-replication-edit` → `semio-framework-replication-wire-core`
- `semio-framework-replication-edit` → `semio-framework-value`
- `semio-framework-replication-error` → `semio-framework-diagnostic`
- `semio-framework-replication-error` → `semio-framework-pack-error`
- `semio-framework-replication-error` → `semio-framework-value`
- `semio-framework-replication-format` → `semio-framework-hash`
- `semio-framework-replication-format` → `semio-framework-pack-error`
- `semio-framework-replication-format` → `semio-framework-pack-source`
- `semio-framework-replication-format` → `semio-framework-replication-codec`
- `semio-framework-replication-format` → `semio-framework-replication-crypto`
- `semio-framework-replication-format` → `semio-framework-replication-error`
- `semio-framework-replication-format` → `semio-framework-replication-wire-core`
- `semio-framework-replication-frames` → `semio-framework-hash`
- `semio-framework-replication-frames` → `semio-framework-replication-causal`
- `semio-framework-replication-frames` → `semio-framework-replication-error`
- `semio-framework-replication-frames` → `semio-framework-replication-ids`
- `semio-framework-replication-frames` → `semio-framework-replication-wire-core`
- `semio-framework-replication-history` → `semio-framework-replication-causal`
- `semio-framework-replication-history` → `semio-framework-replication-crypto`
- `semio-framework-replication-history` → `semio-framework-replication-edit`
- `semio-framework-replication-history` → `semio-framework-replication-error`
- `semio-framework-replication-history` → `semio-framework-replication-history-index`
- `semio-framework-replication-history` → `semio-framework-replication-ids`
- `semio-framework-replication-history` → `semio-framework-replication-wire-core`
- `semio-framework-replication-history` → `semio-framework-value`
- `semio-framework-replication-history-index` → `semio-framework-pack-json`
- `semio-framework-replication-history-index` → `semio-framework-value`
- `semio-framework-replication-ids` → `semio-framework-pack-json`
- `semio-framework-replication-ids` → `semio-framework-value`
- `semio-framework-replication-interaction` → `semio-framework-pack-json`
- `semio-framework-replication-interaction` → `semio-framework-value`
- `semio-framework-replication-local-interaction` → `semio-framework-replication-interaction`
- `semio-framework-replication-local-interaction` → `semio-framework-value`
- `semio-framework-replication-mutation` → `semio-framework-diagnostic`
- `semio-framework-replication-mutation` → `semio-framework-mutation-diff`
- `semio-framework-replication-mutation` → `semio-framework-pack-json`
- `semio-framework-replication-mutation` → `semio-framework-replication-ids`
- `semio-framework-replication-mutation` → `semio-framework-replication-wire-core`
- `semio-framework-replication-mutation` → `semio-framework-schema-state`
- `semio-framework-replication-mutation` → `semio-framework-value`
- `semio-framework-replication-mutation-messages` → `semio-framework-diagnostic`
- `semio-framework-replication-mutation-messages` → `semio-framework-mutation-diff`
- `semio-framework-replication-mutation-messages` → `semio-framework-replication-conflict`
- `semio-framework-replication-mutation-messages` → `semio-framework-replication-mutation`
- `semio-framework-replication-mutation-messages` → `semio-framework-value`
- `semio-framework-replication-presence` → `semio-framework-io-base64`
- `semio-framework-replication-presence` → `semio-framework-replication-error`
- `semio-framework-replication-presence` → `semio-framework-replication-interaction`
- `semio-framework-replication-presence` → `semio-framework-replication-wire-core`
- `semio-framework-replication-presence` → `semio-framework-value`
- `semio-framework-replication-wire-core` → `semio-framework-diagnostic`
- `semio-framework-replication-wire-core` → `semio-framework-hash`
- `semio-framework-replication-wire-core` → `semio-framework-pack-error`
- `semio-framework-replication-wire-core` → `semio-framework-pack-json`
- `semio-framework-replication-wire-core` → `semio-framework-replication-codec`
- `semio-framework-replication-wire-core` → `semio-framework-replication-crypto`
- `semio-framework-replication-wire-core` → `semio-framework-replication-error`
- `semio-framework-replication-wire-core` → `semio-framework-value`
- `semio-framework-schema-state` → `semio-framework-value`
- `semio-framework-value` → `semio-framework-io-base64`
- `semio-framework-value` → `semio-framework-schema-registry`
- `semio-framework-value` → `semio-framework-value-allocation-return`
- `semio-framework-value` → `semio-framework-value-clone-source`
- `semio-framework-value` → `semio-framework-value-control-contract`
- `semio-framework-value` → `semio-framework-value-derive`
- `semio-framework-value` → `semio-framework-value-factory`
- `semio-framework-value` → `semio-framework-value-intrinsic`
- `semio-framework-value` → `semio-framework-value-native-decoding`
- `semio-framework-value` → `semio-framework-value-native-encoding`
- `semio-framework-value` → `semio-framework-value-ordered`
- `semio-framework-value` → `semio-framework-value-original-aliases`
- `semio-framework-value` → `semio-framework-value-paged-list`
- `semio-framework-value` → `semio-framework-value-paged-text`
- `semio-framework-value` → `semio-framework-value-retained-clone`
- `semio-framework-value` → `semio-framework-value-retained-map`
- `semio-framework-value` → `semio-framework-value-retirement`
- `semio-framework-value` → `semio-framework-value-retirement-queue`
- `semio-framework-value` → `semio-framework-value-shared-issuance`
- `semio-framework-value-allocation-return` → `semio-framework-value-control-contract`
- `semio-framework-value-clone-source` → `semio-framework-value-control-contract`
- `semio-framework-value-clone-source` → `semio-framework-value-retirement`
- `semio-framework-value-clone-source` → `semio-framework-value-shared-issuance`
- `semio-framework-value-derive` → `semio-framework-hash`
- `semio-framework-value-derive` → `semio-framework-native-build-compiler-resources`
- `semio-framework-value-factory` → `semio-framework-value-control-contract`
- `semio-framework-value-factory` → `semio-framework-value-derive`
- `semio-framework-value-factory` → `semio-framework-value-retirement`
- `semio-framework-value-intrinsic` → `semio-framework-io-base64`
- `semio-framework-value-intrinsic` → `semio-framework-value-control-contract`
- `semio-framework-value-intrinsic` → `semio-framework-value-native-decoding`
- `semio-framework-value-intrinsic` → `semio-framework-value-native-encoding`
- `semio-framework-value-intrinsic` → `semio-framework-value-ordered`
- `semio-framework-value-intrinsic` → `semio-framework-value-paged-list`
- `semio-framework-value-intrinsic` → `semio-framework-value-retirement`
- `semio-framework-value-native-decoding` → `semio-framework-value-control-contract`
- `semio-framework-value-native-decoding` → `semio-framework-value-native-encoding`
- `semio-framework-value-native-decoding` → `semio-framework-value-retirement`
- `semio-framework-value-native-encoding` → `semio-framework-value-control-contract`
- `semio-framework-value-native-encoding` → `semio-framework-value-retirement`
- `semio-framework-value-ordered` → `semio-framework-value-control-contract`
- `semio-framework-value-ordered` → `semio-framework-value-native-decoding`
- `semio-framework-value-ordered` → `semio-framework-value-retirement`
- `semio-framework-value-original-aliases` → `semio-framework-value-control-contract`
- `semio-framework-value-original-aliases` → `semio-framework-value-retirement`
- `semio-framework-value-paged-list` → `semio-framework-value-allocation-return`
- `semio-framework-value-paged-list` → `semio-framework-value-control-contract`
- `semio-framework-value-paged-text` → `semio-framework-value-control-contract`
- `semio-framework-value-paged-text` → `semio-framework-value-native-decoding`
- `semio-framework-value-paged-text` → `semio-framework-value-native-encoding`
- `semio-framework-value-paged-text` → `semio-framework-value-paged-list`
- `semio-framework-value-paged-text` → `semio-framework-value-retirement`
- `semio-framework-value-retained-clone` → `semio-framework-value-clone-source`
- `semio-framework-value-retained-clone` → `semio-framework-value-control-contract`
- `semio-framework-value-retained-clone` → `semio-framework-value-intrinsic`
- `semio-framework-value-retained-clone` → `semio-framework-value-paged-list`
- `semio-framework-value-retained-clone` → `semio-framework-value-retirement`
- `semio-framework-value-retained-clone` → `semio-framework-value-shared-issuance`
- `semio-framework-value-retained-map` → `semio-framework-value-control-contract`
- `semio-framework-value-retained-map` → `semio-framework-value-derive`
- `semio-framework-value-retained-map` → `semio-framework-value-intrinsic`
- `semio-framework-value-retained-map` → `semio-framework-value-retained-clone`
- `semio-framework-value-retained-map` → `semio-framework-value-retirement`
- `semio-framework-value-retirement` → `semio-framework-value-allocation-return`
- `semio-framework-value-retirement` → `semio-framework-value-control-contract`
- `semio-framework-value-retirement` → `semio-framework-value-paged-list`
- `semio-framework-value-retirement-queue` → `semio-framework-value-control-contract`
- `semio-framework-value-retirement-queue` → `semio-framework-value-paged-list`
- `semio-framework-value-retirement-queue` → `semio-framework-value-retirement`
- `semio-framework-value-shared-issuance` → `semio-framework-value-control-contract`
- `semio-framework-value-shared-issuance` → `semio-framework-value-retirement`
