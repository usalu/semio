# Font Package Namespace Inventory

Recommend `semio-font-{family}-{subset}-{style}` for all 20 asset font packages. Remove only the redundant `framework` segment; retain family, subset and regular style verbatim. Longest proposed name is `semio-font-noto-emoji-joined-forms-regular` (42 characters), within the requested45-character package bound; former longest is52. All names are unique. This reduces identical Rust codegen stems by10 characters. Actual <=256 selected-path validation must still run under the unchanged compiler lane; this inventory does not claim a product pass.

Use direct package/dependency/import/fixture/lock replacements for all rows, including underscore Rust names. No aliases, migrations, source-directory changes, cap or Cargo layout changes. Other font logic/catalog/canvas/PDF package names are listed in JSON and remain outside the asset namespace rename.

| Old | Proposed | Length |
|---|---|---|
| semio-framework-font-share-tech-mono-latin-regular | semio-font-share-tech-mono-latin-regular | 50→40 |
| semio-framework-font-noto-emoji-regions-regular | semio-font-noto-emoji-regions-regular | 47→37 |
| semio-framework-font-noto-emoji-nature-regular | semio-font-noto-emoji-nature-regular | 46→36 |
| semio-framework-font-noto-emoji-food-regular | semio-font-noto-emoji-food-regular | 44→34 |
| semio-framework-font-noto-emoji-activities-regular | semio-font-noto-emoji-activities-regular | 50→40 |
| semio-framework-font-noto-emoji-joined-forms-regular | semio-font-noto-emoji-joined-forms-regular | 52→42 |
| semio-framework-font-noto-emoji-symbols-regular | semio-font-noto-emoji-symbols-regular | 47→37 |
| semio-framework-font-noto-emoji-faces-regular | semio-font-noto-emoji-faces-regular | 45→35 |
| semio-framework-font-noto-emoji-flags-regular | semio-font-noto-emoji-flags-regular | 45→35 |
| semio-framework-font-noto-emoji-people-regular | semio-font-noto-emoji-people-regular | 46→36 |
| semio-framework-font-noto-emoji-objects-regular | semio-font-noto-emoji-objects-regular | 47→37 |
| semio-framework-font-noto-emoji-travel-regular | semio-font-noto-emoji-travel-regular | 46→36 |
| semio-framework-font-noto-emoji-supplement-regular | semio-font-noto-emoji-supplement-regular | 50→40 |
| semio-framework-font-anta-latin-ext-regular | semio-font-anta-latin-ext-regular | 43→33 |
| semio-framework-font-anta-latin-regular | semio-font-anta-latin-regular | 39→29 |
| semio-framework-font-anta-symbols-regular | semio-font-anta-symbols-regular | 41→31 |
| semio-framework-font-anta-math-regular | semio-font-anta-math-regular | 38→28 |
| semio-framework-font-kelly-slab-latin-ext-regular | semio-font-kelly-slab-latin-ext-regular | 49→39 |
| semio-framework-font-kelly-slab-latin-regular | semio-font-kelly-slab-latin-regular | 45→35 |
| semio-framework-font-kelly-slab-cyrillic-regular | semio-font-kelly-slab-cyrillic-regular | 48→38 |

## Affected Tracked Files

Search used tracked `git grep` for both hyphen package and underscore crate prefixes, excluding ticket records and AGENTS. It found 86 files, including every matching tracked Cargo.lock. Historical ticket records are evidence, not rename inputs.

- `Cargo.lock`
- `✏️s/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.lock`
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.lock`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.lock`
- `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.lock`
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.lock`
- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.lock`
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.lock`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.lock`
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.lock`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.lock`
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/Cargo.lock`
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.lock`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/Cargo.lock`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.lock`
- `✏️s/🧑‍💻dev/🌊️flow/Cargo.lock`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/Cargo.lock`
- `✏️s/🧑‍💻dev/💡️services/Cargo.lock`
- `✏️s/🧑‍💻dev/📐️cad/Cargo.lock`
- `✏️s/🧑‍💻dev/🧩️composition/Cargo.lock`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock`
- `🌎️hub/Cargo.lock`
- `🧰️framework/Cargo.lock`
- `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/📇️catalog/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/📇️catalog/🦀️.rs`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🧫️fixtures/📦️specimens.json`
- `🧰️framework/🔨️modules/🖱️ui/🔤️typography/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🔤️typography/🧩️component/🔠️text-faces-and-kerning/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🔤️typography/🧩️component/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/⌨️share-tech-mono/🏛️latin/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🌍️regions/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🌿️nature/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🍽️food/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🎯️activities/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔗️joined-forms/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔣️symbols/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/😀️faces/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🚩️flags/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🧑️people/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🧰️objects/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🧳️travel/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🪉️supplement/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🧮️math/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/➕️latin-ext/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/🏛️latin/📖️regular/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/🪆️cyrillic/📖️regular/📦️packages/🦀️rust/Cargo.toml`
