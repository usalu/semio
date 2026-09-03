# 🗿️ Unified Artifact Naming and Deduplication — Summary

## What was asked
Artifacts unified, finalized, cleanly implemented; artifact names must be clear **nouns** rather than
verbs auto-derived from plugin names (`present` → `presentation`); and every semio-scoped artifact
defined exactly **once**.

## Deduplication — investigated, nothing to remove
Every `s.stdio.semio.<subset>` schema id is declared **exactly once**, in
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️<subset>/🧬️schema/`.
The apparent duplicates in other plugins are **compositions, not redefinitions**: they reference the
canonical subsets through `#[child(kind = "s.stdio.semio.<subset>")]` (e.g.
`PresentationArtifact.presentation: PresentationChild`). No second definition existed, so none was removed.

## Renames landed (9 artifacts)

| plugin | before | after | id change |
|---|---|---|---|
| 🎞️animate | `🎬️present` | `🎬️presentation` | `s.animate.present` → `s.animate.presentation` |
| 🖍️draw | `🖍️draw` | `🖍️drawing` | `draw.document` → `drawing.document` |
| 🪵️sourcing | `🗂️curate` | `🗂️curation` | `sourcing.curate` → `sourcing.curation` |
| 🔱️trinity | `♻️rewrite` | `♻️rewriting` | `text.♻️rewrite` → `text.rewriting` |
| 📸️remodel | `📸️remodel` | `📸️remodeling` | `3d.remodel` → `3d.remodeling` |
| ➗️mathematical | `➗️mathematical` (adj.) | `➗️equation` | `computation.mathematical` → `computation.equation` |
| 🌀️procedural | `🌀️procedural2d` (adj.) | `🌀️generation2d` | `2d.procedural` → `2d.generation` |
| 🌀️procedural | `🧊️procedural3d` (adj.) | `🧊️generation3d` | `3d.procedural` → `3d.generation` |
| 📜️imperative | `📜️imperative` (adj.) | `📜️procedure` | `computation.imperative` → `computation.procedure` |

## Hygiene fixed
- **Emoji leaked into machine ids** (an id must be ASCII): `data.🔋️model` → `data.model`,
  `data.🏛️program` → `data.program`, `text.♻️rewrite` → `text.rewriting`. A sweep of all 92 artifact
  roots found no other non-ASCII in `id`/`schema`/`source_format`/`component_kind`.
- **Missing emoji variation selector U+FE0F** in directory names, against the taxonomy SSOT which
  declared the correct spellings: `◻2d` → `◻️2d` (fem, puzzle, block), `📄txt` → `📄️txt`,
  `📰xml` → `📰️xml` (stdio). ~700 in-scope references corrected by exact-codepoint matching, plus 13
  stragglers in `📜️script.ts`, `package.json`, `🔒️policy-allowlist.json`, two `📋️project.json`
  workspace paths, and doc comments.
- **Taxonomy SSOT** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` updated
  centrally (kept off-limits to the parallel agents to avoid write collisions).
- **Doc-comment drift** in `🧿️semio/🦀️.rs`: claimed "13 schema-owning domain subsets" and
  "19 … (14 domain + text + base)"; actual is 18 domain subsets + `✳️base` = 19. Corrected.
- One real rename fallout found and fixed by the verification pass: `🧹️normalization/🟦️.ts:1323`
  hardcoded the pre-rename literal `"🖍️draw"`, which crashed `parseTaxonomy`.

## Verification

| gate | result |
|---|---|
| taxonomy registry vs disk | **87 = 87, zero drift** (exact set equality) |
| `#[path]` resolution, repo-wide | **17,712 / 17,713 resolve**; the 1 failure is in `🌊️flow`, untouched here |
| rename-corruption sweep | **0** (`presentationation`, `drawinging`, `curationd`, … all zero) |
| errors naming a renamed token as missing | **0** |
| `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2` | exit 0, clean |
| `cargo check -p semio-s-plugin-animate` | 0 errors |

## Workspace is red — but not from this ticket
A full `cargo check` of the 14 touched crates reports 7,574 errors. Attribution, established by
evidence rather than assumption:
- Dominant families are **E0308 (3,227)** and **E0053 (2,160)** — `method 'empty' / 'from_snapshot' /
  'from_text' has an incompatible type for trait`, i.e. an **async-signature migration** on a shared
  codec trait. Rename breakage would surface as E0432/E0433, which total only 306.
- `🏛️architect` shows 2,456 errors although the only change here was one string literal; its schema
  file is **unmodified** (`git status` clean on it) and still declares `async fn empty() -> Self`.
- The `could not find 'mutation' in create_tile` family is **pre-existing**: at commit `a807c0706c`,
  before this ticket, the file already read
  `use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;`. That ticket is
  converting mutation dirs to a `🦠️mutation/` layout and has done `draw` but not `animate`.
- Doubled-emoji doc headers (`//! 🀄️ 🀄️ …`) are byte-identical at `a807c0706c` — pre-existing.

These belong to two other in-flight tickets (`SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS…`
and the `value_derive`/async migration). Left untouched deliberately.

## Deliberately not done
- **`🧿️semio` standard dir `🔖️v1` → `🔖️1`.** 58 plugin artifacts use `🔖️1`, semio alone uses `🔖️v1`,
  so it is a genuine outlier. Not changed because (a) format standards legitimately use real version
  names (`🔖️rfc8259`, `🔖️ecma-376`, `🔖️1.7`), so semio's semantic standard is a different class, and
  (b) it is a very large mechanical change to the exact path the request quoted. **Flagged for a
  decision rather than changed silently.**
- `✒️writer`, `🎥️shooting`, `📋️forms` left alone — already nouns, not verbs.
