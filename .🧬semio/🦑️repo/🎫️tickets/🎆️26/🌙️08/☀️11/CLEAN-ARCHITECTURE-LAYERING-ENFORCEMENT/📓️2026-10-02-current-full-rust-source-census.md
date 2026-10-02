# Current Full Rust Source Census

Normal registered `@semio-tech/repo-lib:lint-rust-source-direction` completed RED in Nx9m31s, with 86 strict violations and 12 census problems across 24135 files/61363 authored references. This is a terminal authored-source verdict, not a deadline refusal. Original policy/selection/severity remain intact; no cached authority, ignored-owner waiver or deadline increase was used. No Cargo/native compiler was started by this gate.

## Exact strict rule groups

| Rule | Actual edges |
| --- | ---: |
| framework-modules-no-products | 7 |
| plugin-no-extension-or-artifact-📕️norm | 1 |
| plugin-no-extension-or-artifact-🗄️stdio | 72 |
| plugin-no-extension-or-artifact-🧩️puzzle | 4 |
| plugin-no-extension-or-artifact-🪐️space | 2 |

## Current General Framework To Product Edges

- `🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️standalone/🦀️.rs:44` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (include_str).
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs:13` → `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs` (path).
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs:129` → `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json` (include_str).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs:14` → `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖼️scene-raster-ownership/🔣️.json` (include_str).
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️component-unit/🦀️.rs:168` → `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎛️inline-tree-controls/🔣️.json` (include_str).
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:54` → `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧫️fixtures/🔣️.json` (include_str).
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:438` → `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧫️fixtures/🔣️.json` (include_str).

## Exact Unresolved Inputs

- `unsupported-expression` — `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🔮️oracles/🧪️tests/🔬️unit/🦀️.rs`: Unsupported Rust compile expression at line 10: include_str
- `unsupported-expression` — `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧪️tests/🔬️unit/🦀️.rs`: Unsupported Rust compile expression at line 177: include_str
- `unsupported-expression` — `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`: Unsupported Rust compile expression at line 596: include_str
- `unresolved-template-scope` — `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs`:19: Finite Rust macro requires an exclusive live lexical scope: committed; definitionOffset=1129
- `unresolved-target` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs`: Rust source dependency requires manifest provenance: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs:167
- `unresolved-template-scope` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🦀️.rs`:48: Finite Rust macro requires an exclusive live lexical scope: vector; definitionOffset=2024
- `unresolved-target` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: Rust source dependency requires manifest provenance: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:39753
- `unresolved-target` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs`: Rust source dependency requires manifest provenance: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs:2
- `unresolved-target` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`: Rust source dependency requires manifest provenance: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:43
- `unresolved-target` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️subset-macro/🦀️.rs`: Rust source dependency requires manifest provenance: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️subset-macro/🦀️.rs:22
- `unresolved-target` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`: Rust source dependency requires manifest provenance: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:876
- `unresolved-target` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🔁️document-replacement/🦀️.rs`: Rust source dependency requires manifest provenance: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🔁️document-replacement/🦀️.rs:6

The physical participation/input provider now retains denials rather than granting a missing context; the full count exceeds the previous narrow Framework-only edge audit because current strict plugin rules are also evaluated. The seven remaining Framework product edges include the held 2D engine ownership and specific fixture ownership moves. Twelve unresolved inputs require their actual owning source/module/template/manifest corrections; none was waived here.

The owned standalone root suite separately passed3/55 under45s; this whole-tree RED does not undo that helper receipt or imply the remaining source boundaries are resolved. The read-only one-second host sample near five minutes observed2.1GB footprint without decoded JS function attribution; it does not prove a particular parser/function bottleneck.
