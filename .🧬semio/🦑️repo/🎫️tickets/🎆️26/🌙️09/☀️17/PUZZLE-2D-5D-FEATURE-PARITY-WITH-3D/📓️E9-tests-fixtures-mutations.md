# E9 — Non-UI Parity Audit: Mutations, Fixtures, Test Infrastructure, Examples, Editor Laws

Read-only audit. Scope: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}` at `🏅️standards/🔖️1/🪆️subsets/✳️any`, plus plugin-root `🧪️tests`. Vocabulary map used throughout: **Node=Object=Part**, **Handle=Vortex=Grip**, **Edge=Attraction=Fastener**, **Wire=Cable=Rope**.

---

## 1. Mutation kinds — side by side

Counts exclude the non-mutation siblings `💾️binary`, `📝️text`, `🧪️tests` that also live under each `🧬️mutations/` dir.

| Total mutation kinds | 2d | 3d | 5d |
|---|---|---|---|
| count | **26** | **35** | **28** |

### Shared across all three (vocabulary-normalized)
create / delete / move (node|object|part) · change-kind · change-locked (partial, see gaps below) · add/remove/replace-handle(vortex/grip) · connect/disconnect-handles(vortices/grips) · connect/disconnect-kind-compatibility · replace-kind-catalogs · replace-\<edge\>-geometry (edge/attraction/fastener geometry) · change-domain (2d gap, see below)

### 3d-only, no 2d/5d analogue (13 mutations — the `target-volume` and `reference` families are 3d-exclusive concepts)
`create/delete/move/rotate/scale/change-locked/change-hidden-target-volume` (7) and `create/delete/move/resize/change-hidden/change-locked/replace-source-reference` (6, minus overlap already counted). These two concepts (bounding target volume, external reference/import) have **zero** counterpart mutation in 2d or 5d's mutation vocabulary.

Also 3d-only within the edge/attraction family: 3d has **no** `change-attraction-kind`, `change-attraction-visible`, or `change-attraction-locked` — attractions in 3d only get `replace-attraction-geometry`. (2d has all four: kind/visible/locked/tips on edges; 5d has fastener-kind + fastener-geometry but no fastener-visible/locked.)

### 2d-only, no 3d/5d analogue
`change-node-root` (promote to root), `change-manifest-id`, `change-edge-tips` — 3 mutations with no 3d or 5d equivalent.

### 5d-only, no 2d/3d analogue
`change-description`, `rename-puzzle5d` — 2 mutations, both document-level metadata edits.

### Locked-mutation gap
2d has `change-edge-locked`; **neither 3d nor 5d has an attraction/fastener-locked mutation.** 5d additionally lacks `change-part3d-locked` (only `change-part2d-locked` exists) — so a 5d part's 3D representation cannot be locked independently of its 2D representation via a dedicated mutation.

### Icon gap
2d has `change-node-icon`; 5d has `change-part2d-icon`; **3d has no `change-object-icon`** (3d objects only get `change-object-mesh`, no separate icon mutation) — asymmetric but by design (3d has no 2D icon glyph concept). Flag only if icon-based catalogue rendering is later required for 3d.

## 2. Fixture vector counts per mutation

Counted as **case subdirectories** (each case dir bundles ~5 files: before/mutation/after/etc.) — file counts alone are misleading since 2d cases bundle the same 5 files as 3d/5d cases.

| Artifact | mutations | total fixture cases | cases/mutation |
|---|---|---|---|
| 2d | 26 | **75** | mostly 3, a few 2 (`change-manifest-id`, `replace-node-handle`, `connect-kind-compatibility`) |
| 3d | 35 | **35** | exactly 1 every mutation |
| 5d | 28 | **28** | exactly 1 every mutation |

**Finding:** 3d itself — the reference implementation — has only single-case fixture coverage per mutation, same depth as 5d. 2d is the only artifact with multi-scenario fixture depth (2–3 cases/mutation). So 5d is not behind 3d here; both 3d and 5d are thin relative to 2d's practice.

## 3. Test infrastructure

### Cargo `[[test]]` / feature gates
- **No `[[test]]` declarations exist anywhere** in the puzzle plugin tree (`grep '\[\[test\]\]'` across all 4 `Cargo.toml` files returns nothing) — every artifact mounts its tests via `#[path]` into the crate's single lib target, so there is nothing to declare.
- `component-app-assembly` feature gate is present and structurally consistent across all three `Cargo.toml`s (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/📦️packages/🦀️rust/Cargo.toml`); 5d's gate additionally forwards `semio-s-artifact-puzzle-3d/component-app-assembly`, confirming 5d's fill/precompute delegates to the 3d engine (see §5).

### `#[path]` mount integrity
Wrote a resolver script checking every `#[path = "..."]` mount against disk, relative to its containing file. **Result: 0 stale mounts** — 2d (399 mounts), 3d (432), 5d (371), whole-plugin total 1204 mounts, all resolve. No stale renames found; this dimension is currently clean.

### Second-language implementations & oracle registrations
Each artifact has exactly one `mutate-puzzle-Nd-1` suite (Rust `.rs` + Python `.py` second implementation + `.feature`):
- 2d: `🧪️tests/◻️mutate-puzzle-2d-1/` (617-line `.py`)
- 3d: `🧪️tests/🧊️mutate-puzzle-3d-1/` (471-line `.py`)
- 5d: `🧪️tests/🖐️mutate-puzzle-5d-1/` (510-line `.py`)

**Critical gap: 2d additionally has two dedicated third-party-oracle suites that 3d and 5d entirely lack:**
- `🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts` (448 lines)
- `🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py` (632 lines)

2d's `🔮️oracles/🔣️.json` registers **7 third-party libraries** (networkx, graphology, shapely, jsonschema-py, jsonpatch-py/deepdiff, jsonschema-js, fast-json-patch-js) plus the python second implementation, each keyed to specific mutation ids. 3d's and 5d's `🔮️oracles/🔣️.json` register **only** `puzzle-{3d,5d}-python-independent` — no third-party library entries at all, even though `parry3d` (3d) and `rstar` (5d) are present as `dev-dependencies` and used directly in Rust tests. Those crates are used but **not declared in the oracle registry**, and neither artifact has a matching dedicated third-party test-suite directory the way 2d does. Per AGENTS.md's "at least one language-agnostic test per feature validated by a third-party library" bar, 3d and 5d currently rely on undeclared, ad hoc third-party usage inside the Rust suite rather than a registered, language-agnostic oracle suite.

## 4. Editor law files (`✏️editor/🧪️tests`)

| Artifact | test-dir topics | `#[test]` fns in editor tree | `#[test]` fns whole artifact |
|---|---|---|---|
| 2d | 20 | **169** | 691 |
| 3d | 15 | **239** | 534 |
| 5d | 15 | **57** | 307 |

5d's editor unit-test count (57) is **24% of 3d's** (239) and **34% of 2d's** (169), despite having a similar number of test-dir topics (15, matching 3d) — 5d's law files are present but much thinner per topic.

**3d-only law suites, zero 2d/5d counterpart** (confirmed via `grep` for the topic name across 2d and 5d trees — 0 hits both):
- `example-switch` (editor root, line 8701)
- `mutation-latency` (editor root, line 8705)
- `selection-scale` (editor root, line 8709)

These sit alongside the universal `unit` topic at editor root. 2d and 5d only have `unit` at editor root (5d additionally has its own `puzzle5d-retained-retirement-laws`, which is a genuinely 5d-specific capability with no 3d/2d analogue by design, not a gap).

**Structural topic gaps 3d/5d have that 2d lacks:** `⏳️precompute/📐️geometry/🧪️tests` (3d, 5d have it as `🧠️precompute/📐️geometry`; 2d's fill lives under `⚙️engine`, no geometry-precompute test dir). **Topic 2d has that 3d/5d lack:** dedicated command-level test dirs `🎮️commands/⚖️set-brush-kind-weights` and `🎮️commands/🧮️set-fill-count`, and tool/option-level dirs `🎭️modes/✏️edit/🛠️tools/🪣️fill` + `🎭️modes/✏️edit/☑️options/🖌️brush`. 3d and 5d fold brush/fill command testing into the coarser `⏳️precompute/🖌️brush` and `⏳️precompute/🪣️fill` (or top-level `🎭️modes/✏️edit`) suites instead of per-command dirs.

**5d-unique window split:** 5d has separate `🎭️modes/✏️edit/🪟️windows/◻️2d/🧪️tests` and `.../🧊️3d/🧪️tests` (reflecting its dual 2D/3D representation), which 2d/3d don't need.

## 5. Plugin-root tests (`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/…`)

| Suite | Covers |
|---|---|
| `🔬️interactivity-puzzle-fill-p4e` (237 lines) | **3d only** — literal-source law-checks against `Puzzle3dPrecomputeSession`/`FillBuilder` (objects/attractions/target_volumes, vortex/cable kind catalogs) |
| `🔬️interactivity-puzzle-fill-run-job` (221 lines) | **3d only** — same FillBuilder/precompute source family |
| `🔬️interactivity-puzzle-fill-trace` (29 lines) | **3d only** |
| `🔬️tool-job-puzzle-reserved-routes` (39 lines) | **5d only** — literal-source law-checks against `Puzzle5dCopyJobFactory`/`Puzzle5dCutJobFactory`/`Puzzle5dPasteJob`/`Puzzle5dImportJob` reserved-wire routes (`PUZZLE5D_RESERVED_PAGE_BYTES`) |
| `🔬️surface` | shared framework-level surface test, not artifact-specific |

**Finding:** none of these four artifact-targeted self-tests has a counterpart for the other two artifacts. The fill/precompute law family is validated only against 3d's source; the reserved-routes/clipboard law family is validated only against 5d's source. 2d has neither a fill-precompute self-test here nor a reserved-routes self-test (2d has no clipboard/reserved-job feature yet per its fixtures). This mirrors the fixtures-level asymmetry: `🥽️brush-mesh-upload` fixture is 3d-only, `📋️clipboard-maximum` fixture is 5d-only — each artifact has one exclusive job-fixture family with its own exclusive plugin-root self-test, and none of the three currently has all three families.

## 6. Examples (`📚️examples`)

| Artifact | examples |
|---|---|
| 2d | `🌲️concrete-forest`, `🏗️nakagin-capsule-tower` |
| 3d | `🌲️concrete-forest`, `🏗️nakagin-capsule-tower` |
| 5d | `🌲️concrete-forest`, `🏗️nakagin-capsule-tower`, **`🌙️capsule-dream`** (5d-exclusive, no 2d/3d counterpart) |

Directory shape (`🖼️assets/`, `.ts`, `.rs`, `🧪️tests/🧩️example`) is identical across all three for the two shared examples.

### `kind_catalogs` / inference fallback (the item flagged in the ticket)
- **3d's `concrete-forest`** is the only example anywhere that builds an explicit `kind_catalogs` payload in its test fixture (`kind_catalogs` string literal found once, in `🧊️3d/…/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs`).
- **2d has no example with an explicit `kind_catalogs`**, but compensates with an `inferred_node_kind_rows` fallback function that exists in exactly two places: `◻️2d/…/✏️editor/🦀️.rs` and `◻️2d/…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs`. `grep` for `inferred_*_kind_rows` in 3d and 5d trees: **0 hits both.**
- **5d has neither.** Its catalogue panel (`🖐️5d/…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:97`) reads kind rows directly off the document: `let catalogs = envelope.document.kind_catalogs.clone().unwrap_or(json!({}));` with no inference fallback. Since none of 5d's three examples populate `kind_catalogs` (all three greps for `kind_catalogs` in 5d's `📚️examples` return 0), the catalogue panel and fill precompute will render/derive **empty part/grip/fastener/rope rows for every 5d example** unless the running document was mutated via `replace-kind-catalogs` at runtime. This is the concrete parity gap the ticket description anticipated — **5d needs either its own `inferred_part_kind_rows` fallback (mirroring 2d) or example fixtures that carry real `kind_catalogs`.**

### Fixture sizes vs known caps
| File | 2d | 3d | 5d |
|---|---|---|---|
| `nakagin` `.dsl.semio` source | 93,682 B | 128,755 B | **168,262 B** |
| `concrete-forest` `.dsl.semio` source | 2,129 B | 7,640 B | 3,093 B |
| `capsule-dream` `.dsl.semio` source | — | — | **3,035,107 B (~2.96 MB)** |
| `capsule-dream` golden `.json` | — | — | 860,636 B |

`capsule-dream`'s test (`🖐️5d/…/📚️examples/🌙️capsule-dream/🧪️tests/🧩️example/🦀️.rs`) asserts `projection.parts.len() == 2880` and `fasteners.len() == 2864`. These `.dsl.semio` files are uncompiled declarative source (not the on-wire document), so they don't directly violate the 32 KiB surface-doc / 64 KiB contiguous-request caps by themselves, but **the part count is a real capacity risk**: 5d's precompute tree defines no `DOCUMENT_*_SLOTS` constants of its own (`grep` returns nothing), and 5d's `component-app-assembly` feature forwards to 3d's (`semio-s-artifact-puzzle-3d/component-app-assembly`), whose `DOCUMENT_OBJECT_SLOTS = (NAKAGIN_DOCUMENT_OBJECTS(180) + DOCUMENT_FILL_HEADROOM_SLOTS(1024)).next_power_of_two() = 2048`. If 5d's fill precompute genuinely inherits that 2048-object cap, **`capsule-dream`'s 2880 parts exceed it by ~40%** — worth a direct capacity check before treating `capsule-dream` as a safe fill-precompute exercise (it may currently only be exercised through the golden-flatten/DSL-roundtrip tests, not through an actual fill run).

2d has its own unrelated 8,192-item caps (`PUZZLE2D_FORCE_UNITS_PER_STEP`, `PUZZLE2D_REDRAW_MAX_HANDLES`), not directly comparable to 3d/5d's object-slot cap; no 384 KiB board-descriptor constant was found by name in any of the three artifacts (grep for `393216`/`384 * 1024`/`BOARD_DESCRIPTOR` returned nothing) — if that cap exists it is likely defined at the framework/plugin layer, outside this audit's scope, or under a different constant name not yet located.

---

## OWED — prioritized

1. **5d catalogue/fill kind-row fallback missing** (§6). `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:97` — add an `inferred_part_kind_rows`-style fallback mirroring `◻️2d/…/✏️editor/🦀️.rs` and `◻️2d/…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs`, or give 5d's examples real `kind_catalogs` payloads. Highest priority: currently produces empty catalogue sections for every 5d example.

2. **`capsule-dream` fill-precompute capacity risk** (§6). `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/` — 2880 parts vs. inherited 2048-object `DOCUMENT_OBJECT_SLOTS` cap from 3d. Verify whether a fill run against this example is ever exercised, and if so whether it needs its own headroom constant.

3. **3d/5d missing third-party-oracle test suites** (§3). Add `🌐️third-party-puzzle-{3d,5d}-1` / `🕸️third-party-puzzle-{3d,5d}-1` suites analogous to `◻️2d/…/🧪️tests/🌐️third-party-puzzle-2d-1` and `🕸️third-party-puzzle-2d-1`, and register `parry3d` (3d) / `rstar` (5d) — already dev-dependencies — in `🔮️oracles/🔣️.json` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json`) instead of leaving them undeclared, ad hoc dev-dependencies.

4. **5d editor law-file thinness** (§4). 57 `#[test]` fns vs 3d's 239 across the same 15 topic dirs. No single missing topic identified, but per-topic depth should be audited against 3d's `⏳️precompute/🖌️brush`, `⏳️precompute/🪣️fill`, `📌️panels/🔍️inspection` for gaps once the catalogue fallback (item 1) is fixed (which will itself need new tests).

5. **3d-only editor law suites with zero 2d/5d counterpart** (§4): `example-switch`, `mutation-latency`, `selection-scale` (`🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:8701,8705,8709`). Decide per-suite whether the underlying behavior (example switching, mutation latency budget, selection-driven scale) is meant to be universal — if so, port the law files to 2d/5d; if 3d-specific, document why.

6. **3d-only mutation families with no 2d/5d analogue** (§1): `target-volume` (7 mutations) and `reference` (6 mutations) concepts entirely absent from 2d/5d's mutation vocabulary, plus attraction/fastener-locked and attraction-kind/visible mutations missing in 3d itself. Lower priority — likely intentional (bounding-volume + external-reference are 3d-specific concepts) but worth an explicit decision recorded somewhere rather than silent absence.

7. **Fixture case depth for 3d/5d** (§2): both sit at exactly 1 fixture case per mutation (35 and 28 respectively) vs 2d's 2–3. Since 3d itself is thin here, this is a "raise the floor for all three" item, not strictly a 2d/5d-vs-3d parity gap — deprioritized relative to items 1–6.

8. **Plugin-root self-test coverage is siloed per artifact** (§5): fill-precompute self-tests only exercise 3d source, reserved-routes self-tests only exercise 5d source, and 2d has neither family's self-test. If fill and reserved-routes are meant to be cross-artifact laws, each artifact needs its own literal-source self-test coverage; if they're intentionally artifact-specific capabilities, no action needed — flagging for a decision.
