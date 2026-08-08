# Wave 4 Report — Norm Plugin (Operations → Mutations)

## Gate

| Command | Result | Log |
|---------|--------|-----|
| `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-s-plugin-norm` | **PASS** | `🧪wave4-norm-check.txt` |

Default features omit `cross-fem` so the crate gates without waiting on `semio-s-plugin-fem` Wave 4. Enable `cross-fem` to compile EN 1992/1993 FEM bridge paths.

## Scope

All fifteen norm artifacts under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/`:

`iso16757`, `vdi3805`, `din4108`, `din16798`, `din18599`, `en1990`–`en1999`.

## Pattern (thin SetDocument)

Each artifact shares `crate::document::SetDocumentMutation<Document>`:

```
🧬️mutations/
  🦀️component.rs          # <Artifact>Mutation = SetDocumentMutation<Document>
  🟦️component.ts           # stub facade
  📤️set-document/
    🦠️mutation/ 🔺️diff/ ↩️inverse/   # leaf stubs + apply/inverse helpers
🔧️op/                     # re-exports <Artifact>Mutation; OpText on shared generic
⚙️engine/                 # <Artifact>Engine: ArtifactEngine + register_pilot_languages
```

## Kernel / shared

- `📄️document/🦀️component.rs`: `SetDocumentMutation`, `NormFamily::Mutation`, `NormHost::apply` → `vcs::apply_mutation`
- `🎚️config/🦀️component.rs`: `NormConfigMutation`, `Mutation` trait, `NormHost` re-export
- `🖥️app-surface/🦀️component.rs`: `Emit::mutations` / `commit` with `SetDocumentMutation`
- Fifteen play apps: `DocumentApp::Mutation` / `ConfigMutation` / `DraftMutation`, `app_commands!` wired to `*Mutation` types

## Glue / TS / grammars

- `📦️glue.rs`: `mutations` facet + `set_document` leaves per artifact; `setup::register_norm_exports`
- `📦️packages/🟦️typescript/📦️index.ts`: `*_mutations` exports alongside `*_op`
- Op grammars: `start mutation` / `mutation =` (was `operation`)
- `en1992` SPR protocol: `schema norm.en1992.mutation`

## Collateral (norm-only)

- `fem` dependency **optional**; feature `cross-fem` gates EN 1992/1993 FEM helpers
- `vdi3805` / `iso16757` engines: `crate::dsl::…` to avoid glob-import shadowing of `dsl`
- Restored `🔌️plugin/🔧️setup/🦀️component.rs` `register_norm_exports` (calls all `register_pilot_languages`)

## Kept (Op brand)

`🔧️op`, `*.op.semio`, `OpText`, `OpBinary`, `print_op` / `parse_op`, `LanguageRole::Ops`.

## Generator / logs (ticket folder)

- `wave4-norm-gen.sh` (initial generator; superseded by inline Python in agent run)
- `🧪wave4-norm-check.txt`
