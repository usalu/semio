# GIF Emoji Repair

## Scope

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif`

The initial strict audit reported 728 governed entries and 132 findings: 105 missing emojis, 11 presentation violations, and 16 sibling-duplicate violations.

## Handpicked identities

- Standards: `87a → 7️⃣87a`, `89a → 9️⃣89a`.
- 89a subsets: `application → 🧩️application`, `base → 🧱️base`, `comment → 💬️comment`, `graphic-control → 🎛️graphic-control`.
- Local oracle contributions: each subset-local `oracle → 🔮️oracle`.
- Window options: each governed `options → ☑️options`.
- Dancing fixtures: `dancing-87a → 💃️dancing-87a`, `dancing-87a-large → 🐘️dancing-87a-large`.
- Role leaves: every mutation pair is `⬅️before.gif` and `➡️after.gif`; the pattern corpus leaf is `🎬️pattern-strip.gif`; the independent reader source is `📖️reader_main.rs`; both snapshot sidecars are `🧬️.schema.json`.
- 87a fixture directories: `➕️insert-image`, `🔀️move-image`, `⏸️no-mutation`, `➖️remove-image`, `🖌️set-background-color-index`, `🎨️set-global-color-table`, `📐️set-image-geometry`, `🪜️set-image-interlace`, `🎞️set-image-pixels`, `⚖️set-pixel-aspect-ratio`, `🖥️set-screen-size`, `📸️set-snapshot`.
- 89a application fixtures: `🧩️add-app-extension`, `🧹️remove-app-extension`, `🔁️set-loop-count-applied`.
- 89a base fixtures: `➕️insert-frame-applied`, `🔀️move-frame-applied`, `⏸️no-mutation-no-op`, `📼️pattern-strip`, `➖️remove-frame-applied`, `🖌️set-background-color-index-applied`, `📐️set-frame-geometry-applied`, `🪜️set-frame-interlace-applied`, `🧱️set-frame-pixels-applied`, `🌈️set-global-color-table-applied`, `⚖️set-pixel-aspect-ratio`, `🖥️set-screen-size-applied`, `📸️set-snapshot-applied`, `🪞️set-snapshot-no-op`.
- 89a comment fixtures: `💬️insert-comment`, `🗑️remove-comment`.
- 89a graphic-control fixtures: `⏱️set-frame-delay-applied`, `♻️set-frame-disposal-applied`, `👻️set-frame-transparency-applied`, `🕹️set-frame-user-input-applied`.
- Mutation/schema presentation identities were reconciled to the physical semantic owner names, including image/frame pixel, background-index, insert/remove image or frame, delay, disposal, and user-input owners.

All selections were made individually from entry semantics. No emoji chooser, rename planner, migration script, or Git-mutating command was used.

## Reference reconciliation

- Reconciled GIF paths in local oracles, generators, Cargo manifests, shared Stdio registries, the dependency inventory, and the plugin policy allowlist.
- Added the exact 89a subset directory roster and the five GIF oracle-directory overrides to the central taxonomy.
- Updated the 89a generator with an explicit recipe-to-subset fixture routing table, so base, application, comment, and graphic-control manifests resolve and generation defaults target the committed physical subset roots.
- Preserved concurrent DWG/PDF changes in shared Stdio and taxonomy files.

## Verification

- Strict ticket audit: 457 files, 288 directories, 728 governed entries; `missing=0`, `generic=0`, `presentation=0`, `spacing=0`, `duplicate=0`, `multiple=0`, `reserved-emoji=0`, `oracle=0`.
- Central taxonomy validation: `[]`.
- JSON parse: every GIF JSON file, `🔒️dependencies.json`, and `✏️s/🔌️plugins/🔒️policy-allowlist.json` passed `jq empty`.
- Oracle coordinates: all 69 `../🧫️fixtures/...` paths in the five GIF oracle manifests resolve to existing files.
- Generator manifests: 87a `manifests`; 89a `manifests`, `build-manifests`, `extensions-manifests`, and `aspect-manifests` all exited 0.
- 89a generator engine: `cargo test` passed 7 tests, 0 failed; both generator binaries compiled.
- 87a reader and 89a extension-reader: both `cargo test` commands exited 0.
- Stdio `test-quick` reached Rust compilation but is presently blocked outside GIF by the concurrently renamed Semio drawing DXF deserializer path `🧿️semio/.../✳️drawing/.../🖊️dxf/🔖️r12/✳️any/🦀️.rs` not existing. No GIF diagnostic was emitted.
