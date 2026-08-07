# P6 spec wiring status (2026-08-07)

Ticket: `HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT` (P6 support — shrink `POLICY_SPEC_WIRING_*` exemptions).

## Policy exemption sets

| Set | Before | After |
| --- | ---: | ---: |
| `POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS` | 220 facet `🦀️component.rs` paths | **0** (empty) |
| `POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS` | 44 artifact roots | **0** (empty) |

Updated in `📜️script.ts` after wiring completed.

## What was wired

### Facet `include_str!` (220 paths)

For every exempt facet under `✏️s/🔌️plugins/**/🗿️artifacts/**`:

- **Text facets** (`🗣️dsl`, `🔧️op`, `🔺️diff`): `COMPONENT_GRAMMAR_SEMIO` + `COMPONENT_GRAMMAR_PATH` via `include_str!("📖️component.grammar.semio")` (lowpoly/dag pilot pattern).
- **Binary facets** (`🎒️pack`, `📡️spr`): `COMPONENT_PROTOCOL_SEMIO` + `COMPONENT_PROTOCOL_PATH` via `include_str!("📡️component.protocol.semio")`.

Derive emission on facets was **not** removed (owned by another agent).

### `register_language` / `register_pilot_languages` (44 artifacts)

Each artifact with facet specs now registers five `dsl::LanguageSpec` roles (`Document`, `Ops`, `Diff`, `Pack`, `Spr`) using:

- Language ids and extensions parsed from sibling `📖️component.grammar.semio` / `📡️component.protocol.semio` (`grammar …` / `protocol …` lines).
- `dsl::passthrough_hooks(id)` and cross-facet protocol carry (document↔pack, op↔spr) matching lowpoly/dag/en1992 pilots.

**Special case — `🪐️space` / `🏠️home`:** no `⚙️engine` facet; `register_pilot_languages()` lives on `🏠️home/🦀️component.rs` and is invoked from `register_s_exports()` in the space plugin glue.

### Post-pass fixes

Bulk wiring initially resolved some Rust module names via the outer `pub mod artifacts` wrapper. A follow-up pass re-targeted `crate::artifacts::<mod>::…` paths using each artifact’s `🦀️component.rs` anchor in plugin glue (22 engines corrected). `🧱️block` `🖐️5d` was pointed at `block5d` instead of `block3d`.

## Tooling (ticket folder)

| Script | Role |
| --- | --- |
| `🔧️p6-wire-specs.mjs` | Bulk include + engine registration + empty exemption sets |
| `🔧️p6-fix-engine-mods.mjs` | Correct `crate::artifacts::<mod>` paths in engines |
| `🧪p6-wire-specs-log.json` | Machine log from initial bulk run |

## Verification

- `bun ./📜️script.ts policy` — **no** `handcrafted-grammar/spec-wiring-include` or `spec-wiring-register` breaches (filtered `spec-wiring`).

## Out of scope (unchanged)

- `POLICY_GENERIC_CODEC_DERIVE_*` / derive emission removal (other P6 agent).
- Handcrafted codec implementation beyond registration and embed.

## Artifact coverage (44 register exemptions → wired)

`mathematical`, `procedural2d`, `procedural3d`, `flow`, `gisterrain`, `gismap`, `vcs`, `present`, `shooting`, `sequence`, `program`, `process3d`, `wires`, `forms`, `layout`, all `📕️norm` variants in the exemption list (`iso16757`, `vdi3805`, `din4108`, `din16798`, `en1990`–`en1999`, `din18599`), `playbook`, `imperative`, `remodel`, `rewrite`, `jack`, `draw`, `raster`, `puzzle2d` / `puzzle3d` / `puzzle5d`, `block2d` / `block3d` / `block5d`, `home` (space), `curate`.
