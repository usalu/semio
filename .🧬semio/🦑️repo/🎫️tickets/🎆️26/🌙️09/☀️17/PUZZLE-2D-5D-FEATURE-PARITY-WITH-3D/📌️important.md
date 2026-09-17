# 📌️ Important

## 2026-09-17 20:4x — I5: ROOT CAUSE of `schema = "puzzle.3d.fixture"` + "examples load empty" (BOTH crates)

Someone (not a wave-1 slice — the mtimes are 19:23–19:37, inside the 16:45–19:50 outage) rewrote
`document_json()` in **all seven** puzzle example modules (2d ×2, 3d ×2, 5d ×3) from the committed,
DSL-derived form

```rust
let projection = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(DSL_TEXT)…;
dsl::json::to_json_string(&projection)
```

to `include_str!("🖼️assets/…/📄️document.json")`, and dropped hand-made, **UNTRACKED** `📄️document.json`
files next to each asset. For 2d and 5d those JSON files are wrong:

- `5d/…/🌲️concrete-forest/…/📄️document.json` → a **puzzle 3d** document (`"schema":"puzzle.3d.fixture"`, `objects`, `attractions`).
- `5d/…/🏗️nakagin-capsule-tower/…/📄️document.json` → likewise a 3d document (158 KB).
- `5d/…/🌙️capsule-dream/…/📄️document.json` → a 121-byte EMPTY stub.
- 2d has the same two files (`git status --porcelain` shows them as `??`).

`document_from_json` swallows the serde error and answers `empty_document()`, so this surfaces as
"seeded document has schema puzzle.3d.fixture / 0 parts" in ~30 shared unit tests, and as
`clipboard_verbs_cover_every_part_of_the_largest_example  left: 0, right: 2880`.
None of the `📄️document.json` files is tracked by git, so this also breaks a fresh checkout.

**I5 fixed the 5d half (20:45):** the three 5d example modules are back on the DSL-derived
`document_json()`, and the three bogus 5d `📄️document.json` files are deleted (copies kept in
`🗑️generated/I5/salvaged-example-json/` in case anyone wants to look at them).

**I2 — do the same for 2d** (`◻️2d/…/📚️examples/{🌲️concrete-forest,🏗️nakagin-capsule-tower}/🦀️.rs`,
`git diff` shows the exact two-line revert) and delete the two untracked `📄️document.json` files.

**Coordinator — 3d has it too** (`🧊️3d/…/📚️examples/{🌲️concrete-forest,🏗️nakagin-capsule-tower}/🦀️.rs`).
3d's JSON happens to hold 3d data so 3d still "works", but it is untracked and drifts from the DSL.
Nobody in this ticket owns 3d; it needs the same two-line revert.

## Companion break: `#[dsl(table)]` cannot be absent

`FieldKind::VecTable`/`VecList` in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:1021,1044`
answer `expected List, found Absent` — a table field is NEVER optional, `#[value(default)]` or not.
3d has always shipped an explicit empty `target-volumes [id:TEXT origin:CRD orientation:TUPLE
scale:LIST hidden:BOOL locked:BOOL] {\n}` in its example assets for exactly this reason.

**I5 fixed 5d by mirroring 3d** — appended that empty table to all three
`🖐️5d/…/📚️examples/*/🖼️assets/*/🗣️.dsl.semio` assets (the field is last in `Puzzle5dSnapshot`,
so it goes at the end of the file). The shared derive was NOT changed.

**I2**: `target_regions` in 2d needs the same treatment — append the empty
`target-regions [...] {}` table to the 2d example assets rather than touching the shared derive.

## 2026-09-17 21:0x — I2: the 2d half of both items above is DONE

- `◻️2d/…/📚️examples/{🌲️concrete-forest,🏗️nakagin-capsule-tower}/🦀️.rs` — `document_json()` back on the
  DSL projection; the two bogus `📄️document.json` deleted (copies in `🗑️generated/I2/salvaged-example-json/`).
  The intruder's three extra tests were removed too: `regenerate_document_json_fixture` +
  `document_json_fixture_matches_dsl_projection` in `🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs`, and
  `export_concrete_forest_document_json_fixture` in `…/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs`.
  **Every one of those files is now byte-identical to HEAD again** (`git diff HEAD` is empty for them).
- Both 2d example DSL assets gained the empty table, mirroring 3d:
  `target-regions [id:TEXT x:NUM y:NUM width:NUM height:NUM label:TEXT hidden:BOOL locked:BOOL] {\n}`.
  Header = one `key:SHAPE` per field in DECLARATION order (`shape_type_name`), parsing is key-driven
  so it goes at the end of the file.

## 2026-09-17 21:0x — I2: `setActiveTool`/`setActiveUtility` must be GENERATED tool ids, never factory keys

65 of 268 red 2d laws died on `interactive-job.catalog-authority`: 2d aliased
`OpBinary::TOOL_JOB_IDS = PUZZLE2D_RETAINED_TOOL_IDS`, and that one list also keys
`Puzzle2dRetainedCommandJobFactory`. `validate_tool_job_rows`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12837`) intersects `TOOL_JOB_IDS` with the
migrated manifest rows, so the two host-configuration verbs were missing from `expected` and every
registry-backed app test failed in `app_with_registry()`. Adding them to the retained list would have
been wrong the other way — a registered factory makes the GENERIC proof's `registered.is_none()`
false. Fixed with a const-eval concat (`PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS` +
`PUZZLE2D_TOOL_JOB_IDS`, `◻️2d/…/✏️editor/🦀️.rs` just after `PUZZLE2D_RETAINED_TOOL_IDS`); 5d already
carries the identical shape at `🖐️5d/…/✏️editor/🦀️.rs:4134`, so only 2d was missing it. The
retained-jobs fixture `toolIds` stays the FACTORY list and must NOT gain the two verbs.

## 2026-09-17 21:0x — I2: `publication-authority-audit` is GREEN again for all three owners

`ownerOracle`'s 2d branch (`🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts:289`) still pinned the old
`(addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))` literal that 2D replaced
with the early-return + proximity budget. Re-pinned to the new shape (the refusal AND the
`saturating_add(connects)` line). `bun ./📜️script.ts publication-authority-audit` → exit 0,
`windowOwnershipCases=7`.

## 2026-09-17 21:1x — I5: the descriptor-probe guest panic is `Puzzle5dScale`'s DSL shape, FIXED

`materialize-dev`'s probe aborted with
`concrete-forest example dsl parses: table column 'scale' has a non-self-delimiting shape (TUPLE) and
cannot be a table column at 1:1`. Root cause, not the asset text:
`impl dsl::DslField for Puzzle5dScale` (`🖐️5d/🦀️.rs:148`) bound `Shape::Tuple(Float, None)` —
legal while `scale` was only ever reached through the nested keyed `part-3d` record, illegal the moment
5G made `target_volumes` a `#[dsl(table)]` whose `scale` is a BARE positional column. Puzzle 3d hit this
exact wall when it added `targetVolumes` and documents it at `🧊️3d/🦀️.rs:137-143`; 5d now binds the same
bracketed `Shape::List(Float)` (`scale=[2]` uniform / `scale=[2 3 4]` per-axis), self-delimiting at any
item count. No shipped 5d asset carries a `scale=` literal, so no asset text changed for this.
(`orientation` stays `TUPLE` and is fine — `[f64; 4]` is a BOUNDED tuple, which IS self-delimiting.)
Stale cross-reference left alone on purpose: `🧊️3d/🦀️.rs:139` still says 5d "uses Tuple"; editing 3d
would have forced a 3d + 5d rebuild for a comment.

## 2026-09-17 21:10 — Coordinator: the 3d half is DONE
`🧊️3d/…/📚️examples/{🌲️concrete-forest,🏗️nakagin-capsule-tower}/🦀️.rs` are byte-identical to HEAD again (DSL-derived `document_json()`); the two untracked `📄️document.json` moved to `🗑️generated/main/salvaged-3d-example-json/`.
