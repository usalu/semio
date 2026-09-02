# 🗿️ Unified Artifact Naming and Deduplication — Plan

## Verified ground truth (checked directly, not assumed)

- Artifact dirs are wired into Rust via hand-written `#[path = "../../🗿️artifacts/<emoji><name>/…/🦀️.rs"]`
  chains in each plugin's `📦️packages/🦀️rust/🦀️.rs`. No codegen. Renaming a dir therefore requires
  editing those `#[path]` strings plus the `mod` identifier.
- Directory-name policy after emoji stripping: `/^[a-z][a-z0-9]*$/u`. All proposed nouns pass.
- Every `s.stdio.semio.<subset>` schema id is declared **exactly once**, in
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️<subset>/🧬️schema/`.
  There is **no** duplicate *definition* of a semio-scoped artifact.
- Plugin artifacts already **compose** semio subsets via `#[child(kind = "s.stdio.semio.<subset>")]`
  (e.g. `PresentArtifact.presentation: PresentationChild`). They are not redefinitions.
- `artifacts::present` references: 95 of 97 files are inside `✏️s/🔌️plugins/🎞️animate` (1 in
  `🧰️framework/🛍️products/💻️os`, 1 in ticket notes). Renames are plugin-local.
- Standard version dirs: `🔖️1` used 58×; semio alone uses `🔖️v1`. Real format standards
  (`🔖️rfc8259`, `🔖️ecma-376`, …) are intentional and stay.

## Wave 1 — verb/derived artifact names → nouns

| plugin | current | new | id change |
|---|---|---|---|
| 🎞️animate | `🎬️present` | `🎬️presentation` | `s.animate.present` → `s.animate.presentation`, `animate.present` → `animate.presentation` |
| 🖍️draw | `🖍️draw` | `🖍️drawing` | `draw.document` → `drawing.document` |
| 🪵️sourcing | `🗂️curate` | `🗂️curation` | `sourcing.curate` → `sourcing.curation` |
| 🔱️trinity | `♻️rewrite` | `♻️rewriting` | `text.♻️rewrite` → `text.rewriting` (also strips leaked emoji) |
| 📸️remodel | `📸️remodel` | `📸️remodeling` | `remodel.scene` → `remodeling.scene`, `3d.remodel` → `3d.remodeling` |

## Wave 2 — adjective artifact names → nouns (pending recon)

`➗️mathematical`, `🌀️procedural2d`, `🧊️procedural3d`, `📜️imperative`, `✒️writer`, `🎥️shooting`.

## Wave 3 — hygiene

- Emoji leaked into machine ids: `data.🔋️model` → `data.model`, `data.🏛️program` → `data.program`.
- Directory names missing U+FE0F after the emoji: `📄txt`, `📰xml`, `◻2d` (×3 plugins).
- Strays: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️.json` (file next to the `🔣️json` dir),
  `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🦀️.rs`.
- `🧿️semio` standard dir `🔖️v1` → `🔖️1` to match the other 58.
- Doc-comment drift in `🧿️semio/🦀️.rs` ("13 schema-owning domain subsets" / "19 … (14 domain + text + any)"
  vs the actual 19).

## Verification gates

```
cargo check -p <plugin-crate> --target wasm32-wasip2
bun ./📜️script.ts verify taxonomy enforce
bun nx run-many -t check --all --exclude workspace
```
