# Current Raw Rust Writer Ingress

Read-only permanent-script audit over all eight actual monorepo domain roots, with direct Rust writer call pattern matching. This audit does not prove every dynamic call or inherited environment is bounded. A first search included nonexistent `semio` and exited2; only the successor over actual roots is complete for this pattern.

Actual remaining direct writer call sites in framework:

- Dashboard Rust package battle journey/load uses `Bun.spawnSync(cargo test --no-run)` and chooses ambient `CARGO_TARGET_DIR` or old cargo/dashboard-journeys. This path bypasses the new declared compiler/materializer helper and is synchronous.
- OS config mutation bridge uses `spawnSync(cargo run --offline)` without a scoped original process envelope or explicit bounded target/dependency/tool homes. It also exits directly.
- OS shell typegen and schema-check use `runCmdStatus(cargo test)`; preview-generated creates a system temporary directory and its own target under it, then `Bun.spawnSync(cargo test)`. Source generation output and temporary compiler custody need genuine current owned scopes.

The caching script match is `bun test` whose file path contains cargo-provenance, not a direct Cargo child. Existing runRepositoryCargoTests and runCargo helpers are not classified by this narrow literal pattern. Those helpers and battle/config/shell adapters must be audited through their actual callers and Original lifetimes before any all-writers or whole-checkout ceiling claim. No Source or shared reader generation was modified or retired by this audit.


Complete literal-pattern successor exited0: 123 matching lines across 90 permanent script files. Matches include comments and path strings; this is a review queue, not an exact unsafe-writer count. Distinct files by domain: ✏️s=82, 🌎️hub=4, 🧰️framework=4.

| Script | Matching Lines |
|---|---:|
| ✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🌍️gis/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🌿️vcs/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🎞️animate/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🎥️shooting/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🎬️sequence/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🏗️fem/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🏙️bim/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🏛️architect/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🏭️process/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/💡️reasoning/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/📋️forms/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/📏️layout/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/📐️cad/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/📕️norm/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/📖️playbook/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/📜️imperative/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/📸️remodel/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🔋️energy/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🔱️trinity/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🕸️dag/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🖍️draw/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🖨️raster/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/📜️script.ts | 3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/📜️script.ts | 3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/📜️script.ts | 3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/📜️script.ts | 3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/📜️script.ts | 3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/📜️script.ts | 4 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/📜️script.ts | 2 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗒️note/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🧱️block/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🪐️space/🏭️bridge/📜️script.ts | 1 |
| ✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/📜️script.ts | 1 |
| 🌎️hub/📦️packages/🟦️typescript/📜️script.ts | 1 |
| 🌎️hub/📦️packages/🦀️rust/📜️script.ts | 7 |
| 🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/📜️script.ts | 2 |
| 🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts | 4 |
| 🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/📜️script.ts | 1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📜️script.ts | 3 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts | 1 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts | 1 |
