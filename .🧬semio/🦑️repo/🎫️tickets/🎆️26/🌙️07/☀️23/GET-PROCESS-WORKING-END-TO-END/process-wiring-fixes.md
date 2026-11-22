# Process Plugin Wiring Fixes

Base `P` = `✏️s/🔌️plugins/🏭️process`, `S` = `P/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

- F1: `S/🔮️oracles/🔣️.json` 16 `payloadSchema` values set to `🧬️schema/🔣️.json` (the canonical form of trinity/jack oracles); the two prose mentions of `🧬️.schema.json` updated too.
- F2: `mutationDirectoryName` of rename-machine set to `🔖rename-machine`.
- F3: authored `S/🧬️schema/🧬️mutations/📏resize-stock/🧪️tests/🟰️keeps-width/🦀️.rs` (committed JSON fixtures already existed under `S/🧪️tests/🧫️fixtures`). The sibling harness only knew status `applied`; the committed outcome (also asserted by the Python oracle) is `no-op`, so a `no-op` arm was added. Registered as `process3d_resize_stock` in `S/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`.
- F4: not a defect, no change. `directoryName` is the `deployment-directory` of each extension's `🧩️component/📦️packages/🦀️rust/Cargo.toml` (an emoji-prefixed deployment name, independent of the source folder), and `🗂️sources.json` and `🔣️taxonomy.json` (the allowed-name list and `dist/dev/🔌️plugin-modules` paths) agree with it. Renaming would ripple into the taxonomy; the generated file was not hand-edited.
- F5: four extension `📦️packages/🦀️rust/📋️project.json` `nativeSources` second glob replaced with `<ext>/**/*.{toml,json,semio,wit}`.
- F6: oracle rationale path fixed to `../🧪️tests/🏭️mutate-process3d-1/🐍️.py`.
- F7: `P/📦️packages/🟦️typescript/package.json` devDependencies gained `@semio-tech/plugin-registry` (`workspace:*`) and `smol-toml` (`1.6.1`, the root pin).

## Validation

- `repo test @semio-tech/process-process3d-rs` hung 3h42m in graph reading and then failed with `BUILD_SOURCE_UNPARSEABLE` (missing module in the repo tool's own bootstrap sources, unrelated to these edits). `repo conformance` lists only build-system scenarios, not oracle checks.
- Native fallback: oracle JSON parses; every oracle mutation directory, scenario test `🦀️.rs` and fixture outcome exists (0 missing).
- Rust was not compiled; the new test is a copy of the change-stock-label harness plus the `no-op` arm.
