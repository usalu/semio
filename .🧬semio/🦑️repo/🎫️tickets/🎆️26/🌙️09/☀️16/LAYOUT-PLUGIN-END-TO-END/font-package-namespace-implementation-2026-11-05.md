# Font Package Namespace

Actual WGPU product74049 failed the selected-unit precompile guard on a font package name. The 256 UTF-16 bound remains unchanged. All20 font asset Rust packages now use the concise domain namespace semio-font-{family}-{subset}-{style}; longest package name42, predicted browser codegen path250 at the canonical checkout fixture. Family, subset, style and source directories remain explicit. No runtime alias, compatibility layer, migration script, private target directory or Cargo layout change was added.

Before implementation, authored20 actual-manifest expectations with Bun/smol-toml agreement in the existing WGPU compiler suite, plus3 portable old/new/worst-font path vectors covered by the independent Python pathlib oracle. Product74049 is the real causal RED; the new focused tests await first postchange execution and are not claimed RED or GREEN.

Updated existing manifests, source imports and all affected lockfiles together, preserving concurrent file contents outside exact package names.

Files:

- Cargo.lock
- ✏️s/Cargo.lock
- ✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock
- ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.lock
- ✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.lock
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.lock
- ✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.lock
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.lock
- ✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.lock
- ✏️s/🔌️plugins/🏭️process/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.lock
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.lock
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.lock
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.lock
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.lock
- ✏️s/🔌️plugins/💡️reasoning/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.lock
- ✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.lock
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.lock
- ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.lock
- ✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.lock
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.lock
- ✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/Cargo.lock
- ✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.lock
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.lock
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/Cargo.lock
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.lock
- ✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.lock
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.lock
- ✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.lock
- ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/Cargo.lock
- ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.lock
- ✏️s/🧑‍💻dev/🌊️flow/Cargo.lock
- ✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/Cargo.lock
- ✏️s/🧑‍💻dev/💡️services/Cargo.lock
- ✏️s/🧑‍💻dev/📐️cad/Cargo.lock
- ✏️s/🧑‍💻dev/🧩️composition/Cargo.lock
- ✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock
- 🌎️hub/Cargo.lock
- 🧰️framework/Cargo.lock
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/📇️catalog/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/📇️catalog/🦀️.rs
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🧫️fixtures/📦️specimens.json
- 🧰️framework/🔨️modules/🖱️ui/🔤️typography/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖱️ui/🔤️typography/🧩️component/🔠️text-faces-and-kerning/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/🔤️typography/🧩️component/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/⌨️share-tech-mono/🏛️latin/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🌍️regions/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🌿️nature/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🍽️food/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🎯️activities/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔗️joined-forms/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔣️symbols/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/😀️faces/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🚩️flags/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🧑️people/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🧰️objects/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🧳️travel/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🪉️supplement/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🧮️math/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/➕️latin-ext/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/🏛️latin/📖️regular/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/🪆️cyrillic/📖️regular/📦️packages/🦀️rust/Cargo.toml

## Validation

The first focused run reached the new actual-manifest test but refused its checkout-store path through the materialization guard. The test now reads the real asset manifests and feeds their names to the pure selected-path model for both POSIX and Windows; it performs no store access. Fresh normal repo Bun run exited 0: six tests, 181 assertions, including the independent Python pathlib corpus and selected-unit control/cancellation law. The actual WGPU prototype product owner has been restarted; no product pass is claimed.
