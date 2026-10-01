# 📓️ W3-CODES — Frozen Outcome-Code Vocabulary, Repo-Wide

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W3-CODES, 2026-09-30 → 10-01. Binding contract: `📋️design.md`
§11 ("Negative witnesses", "Outcome-code vocabulary extension"). Coordinator decisions applied: (1) FEM `id-mismatch` was
folded by the energy executor; (2) energy editor faults are mine after energy finished; (3) the repo product's GraphQL
resolver ids (`Mutation.*`-style `mutation.goalCreate` …) are GraphQL terminology and stay untouched; the gate checks
outcome positions only, never arbitrary strings; (4) every non-vocabulary outcome code in ANY namespace and every level
mismatch is class (a).

## 1. Result

- Every `MutationMessage`/outcome code the repository produces, declares or expects is now one of the nine vocabulary
  codes or `mutation.apply.<kebab-detail>`, at the one level the code fixes. Persistence (`🏪️store`
  `expected_mutation_message_level`) can no longer be handed an unpersistable message by a leaf.
- The vocabulary is ONE protocol table (`protocol::OUTCOME_CODES`, `APPLY_OUTCOME_CODE_PREFIX`, `outcome_code_level`,
  `MutationOutcome::refuse`) with a language-agnostic twin document; persistence, the TypeScript twin, the gate, i18n
  and tests all read or are tested against it.
- `verify mutation-outcome-law` rule 2 was rewritten: it checks code AND level at every outcome position in Rust,
  TypeScript, Python, Gherkin and committed `🎯️outcome` documents. Repo-wide: **0 breaches** (15 s). A planted-violation
  proof reports all 13 positions. (Energy then moved all 7 rules onto one git inventory: full gate verdict in ~95 s;
  the only remaining breaches are 47 rule-1 leaves in 📕️norm, see §7.)

## 2. Vocabulary (single source)

| code | level | meaning |
|---|---|---|
| `mutation.target-missing` | error | addressed target does not exist |
| `mutation.target-referenced` | error | target still referenced |
| `mutation.target-mismatch` | error | payload inconsistent with the target's current state |
| `mutation.no-op` / `partial` / `clamped` | warning | — |
| `mutation.duplicate-id` | fatal | identity already exists |
| `mutation.invariant` | fatal | payload violates a leaf-schema bound or `x-semio-invariant` |
| `mutation.cascade` | info | dependent content changed |
| `mutation.apply.<kebab>` | fatal | apply-time rejection (`MutationOutcome::apply_to` persists it) |

- Rust: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` — `OUTCOME_CODES`, `APPLY_OUTCOME_CODE_PREFIX`,
  `outcome_code_level`, `MutationOutcome::refuse(code, message, target)` (level picked from the table; used by runtime-coded
  guards: raster validators, glTF rejections, stdio snapshot-edit patches).
- Store: `expected_mutation_message_level` now delegates to `protocol::outcome_code_level` (no second table).
- Document: `🎮️mutation/🧫️fixtures/🧫️outcome-code/🔣️.json` + schema `🎮️mutation/🧬️schema/🔣️outcome-code/🔣️.json`
  (codes, levels, apply pattern + accepted examples, rejected examples incl. every retired code).
- TS twin: `📡️replication/🟦️.ts` — `OUTCOME_CODES`, `APPLY_OUTCOME_CODE_PREFIX`, `outcomeCodeLevel`.
- Labels: en/de for all nine codes (energy) plus the new `mutation.apply.*` family label (`ui.mutation.code.apply`,
  `📚️I18n` type, both bundles, `ShellHost` `mutationCodeLabelKey`, time-travel `history_code_text`).

## 3. Census and classification

Method: `git ls-files` literal census of `"mutation.*"` (all languages, tickets excluded) — 551 (file, code) pairs outside
the vocabulary; plus a census of every Rust builder argument (`MutationOutcome::error|fatal`, `MutationMessage::*`,
`MutationApplyError`), every TS `refuse(level, code)`/`{level, code}`, every Python `(level, code)` tuple and every
`🎯️outcome` document — which surfaced outcome codes in OTHER namespaces (`wfc3d.*`, `stdio.*`, `forms.*`, `gltf.*`,
`s.home.*`, bare apply codes) and level mismatches. Raw outputs: `🗑️generated/w3-codes/`.

### 3.1 Class (a) — outcome codes remapped (Rust + TS twin + Python + fixtures + features + tests together)

| owner | was | now |
|---|---|---|
| wfc bitmap / grid3d | `missing-target` (fatal), `unknown-palette-color` (fatal) | `target-missing` (error) |
| wfc bitmap | `malformed-payload` region not base64 / base buffer corrupt; `colour-in-use` | `invariant`; `apply.invalid-base`; `target-referenced` (error) |
| wfc 3d | `wfc3d.{tile,slot,edge,rule}.missing`, `…duplicate-id`, `non-positive-weight`, `degenerate-box`, `edge.self-loop`, `unknown-tile`, `unknown-pinned-tile`, `*-unchanged`/`pin-absent`/`already-connected`, `*-cascaded` | `target-missing`, `duplicate-id`, `invariant` (+ schema `exclusiveMinimum` on weight/width/height/depth, `x-semio-invariant no-self-loop`), `target-missing` (was fatal), `no-op`, `cascade` |
| norm en1992 / iso16757 | `mutation.missing` (fatal), `missing-id`, `mutation.duplicate` | `target-missing` (error), `duplicate-id` |
| draw | `blend-mode-invalid`, `invalid-text-size`, `invalid-geometry`; `text-missing`/`path-missing`/`target-invalid` | `invariant` (schema enum/bounds exist); split `target-missing` vs `target-mismatch` (wrong layer kind) |
| gis gismap | `invalid-number` | `invariant` |
| layout | `mutation.duplicate` (error); `invariant`@error ×3 | `duplicate-id` (fatal); `target-mismatch` (not a text frame, locked frame), `target-referenced` (style used by a story) |
| cad + stdio semio kit/object | `child-identity` (diff) | `invariant` + `x-semio-invariant child-identity` on 11 leaf schemas |
| forms, dag, semio object, sourcing curation | `child-identity` (`MutationApplyError`) | `mutation.apply.child-identity` |
| forms | `forms.missing-response`, `invalid-response`, `duplicate-response` | `target-missing`, `invariant` (+ `x-semio-invariant unique-answer-questions`), `duplicate-id`; shared events vectors + TS twin throw the vocabulary code |
| remodel | `referenced`; `invalid-asset-payload`, `invalid-content-chunk`; `content-gap/-kind-mismatch/-conflict/-capacity`, `incomplete-mesh`, `invalid-reconstruction-*` | `target-referenced`; `invariant` @fatal (+ base64 `pattern` on create-asset `data` and append-content chunks, so the create-asset negative witness fails its schema); `target-mismatch` |
| raster | `*-conflict`, `adjustment-capacity`; `asset-missing`; `transform-invalid`; `adjustment-invalid`; `invariant`@error | `target-mismatch`; `target-missing`; `invariant` (+ `x-semio-invariant invertible-transform`, `bounded-mask-area`); split invariant vs target-mismatch; level fixed via `MutationOutcome::refuse` |
| space home | `s.home.local-studio-tombstone-refused`; `s.home.directory-event-page-invalid`/`frontier-race` | split `invariant` (inadmissible id) vs `target-mismatch` (ceiling); `invariant` vs `target-mismatch` (fixture updated) |
| flow, os plugin time-travel test | `target-missing`@fatal | @error |
| stdio apply errors (las, stl, obj, ifc 4 + 2x3, step, dxf, ply, zip) | bare `invalid-{add,modify,remove,upsert}-{index,target}`, `duplicate-base-target`, `invalid-instance-order`, `stdio.zip.serialization.invalid-state` | `mutation.apply.<same>` / `mutation.apply.invalid-state` |
| stdio apply helpers (41 + 12 media) | `MutationOutcome::error(error.code …)` for an apply rejection | `fatal` (apply family is Fatal) |
| stdio semio base | `mutation.absorb.kind-mismatch`, `mutation.inverse.kind-mismatch` | `mutation.apply.kind-mismatch` |
| stdio zip base | anchor `apply.missing-target`@error (Rust + TS twin + test) | `target-missing` |
| stdio zip iso21320 | `stdio.zip.iso21320.mutation-outside-profile` | `duplicate-id` (name taken), `target-missing` (anchor) |
| stdio step cc1–6, ifc cobie/cv20/sav, svg tiny/basic, xml valid, xlsx, docx | `…mutation-rejected`, `…outside-profile/-subset`, `canonical-edit.invalid`, `xml-address.invalid` | `target-mismatch` |
| stdio pdf (9 leaves) | `stdio.pdf.<verb>.invalid-target` | `target-missing` (remove-page of the last page: `target-mismatch`) |
| stdio png/wav | `patch-pixels/patch-data.invalid-range`, wav `serialization.*` | `target-mismatch` (glTF-style code kept in the message) |
| stdio patch-snapshot (csv, json, png, jpg, wav, tiff, mp4) | raw `snapshot-edit.*` | `SnapshotEditError::outcome_code()` (contract crate) → target-missing / duplicate-id / target-mismatch / invariant via `refuse` |
| stdio i-json | `target-missing`@fatal | @error |
| stdio glTF | substring heuristic; fixtures carried `gltf.mutation.*` as outcome code | explicit `rejection_outcome_code` table (stale/node-cycle/permutation/arity/overflow → `target-mismatch`); 4 rejected fixtures now `code` = vocabulary, `rejection` = glTF code; carrier oracle +1 mismatch row |
| energy (after energy finished) | 12 multi-line `error(\n "mutation.invariant"` | `fatal` |
| framework | presence store `presence.store.closed/local.owner`; reasoning `example.unparsable`; test fixtures `job-test.value-overflow`, `test-snapshot.declared-child-uri`; store tests `mutation.duplicate/conflict`; engine-contract `mutation.targetMissing`; stdio law test `missing-target` | `mutation.apply.{store-closed,local-owner,unparsable-example,value-overflow,declared-child-uri}`; vocabulary codes |

### 3.2 Class (b) — moved out of the `mutation.` namespace

| was | now | where |
|---|---|---|
| `mutation.rejected` (channel/app Fault) | `app.command.rejected` (const `COMMAND_REJECTED_FAULT_CODE`) | os mcp (workspace, dispatch, channel + fixtures/tests), plugin, run, spr channel, kernel TS, os TS, ShellHost, Shell, wgpu shell + tests, AgentBridge test, ShellHelpers test |
| `mutation.fixture-rejected` | `app.command.fixture-rejected` | mcp first-party-codecs fixture |
| `Diagnostic::error("mutation.apply", …)` (ArtifactBuilder diagnostic) | `build.apply` | 46 aggregate `🧬️schema/🦀️.rs` (incl. one literal in energy, announced) |
| procedural retained owner catalog `mutation.<kind>` | `mutations.<kind>` (matches `🧬️mutations` facet) | generation2d/3d binary, mounted-registry tests, owner-catalog-law fixtures |
| energy editor/viewer Faults `mutation.invalid-payload/kind-unavailable/target-in-use` | `app.command.*` | energy editor + unit tests + zones TS, fem docstring |
| GraphQL resolver ids, `mutation.json` filename, `ui.mutation.*` label keys | untouched (not outcome codes) | — |

## 4. Enforcement

- `📜️script.ts` `//#region 🔧️PolicyRuleMutationOutcomeMergePolicy`: `POLICY_MUTATION_FROZEN_CODES` replaced by
  `policyOutcomeVocabulary(repoRoot)` (reads the fixture; malformed ⇒ gate fails), rule 1 uses it, rule 2
  `policyMutationMessageCodeBreaches` rewritten: Rust builders (+level), chainable `.info/.warn` inside outcome bodies
  (single masked brace pass — the old per-fn `policyExtractFnBody` scan was quadratic and made the gate run 40+ min),
  `MutationApplyError` codes (apply pattern), `mutation.` literals in `/🧬️mutations/` and `/🔺️diff/` sources and Python
  oracles, TS `refuse(level, code)` and `{level, code}`, Python `(level, code)` tuples, Gherkin tokens (file names and
  `mutation.apply.<detail>` placeholders excluded), `🎯️outcome` documents (code + level, canonical level names only — see §9).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🚶️file-walk/🟦️.ts`: dot-named / skip-dir symlinks are
  skipped like dot directories (the repo-root `.tmp-ticket` link crashed every walking rule).
- Rust tests: `🎮️mutation/🧪️tests/🧪️outcome-code/🦀️.rs` (table == document, apply family, rejected codes, `refuse`
  levels); store `persisted_messages_admit_exactly_the_outcome_vocabulary` (every code persists at exactly its level and
  is refused at every other; every rejected code refused).
- TS tests: `📡️replication/🧪️tests/🧪️outcome-code/🟦️.ts` (Ajv validates document vs schema; twin == document);
  `📚️I18n/🧪️tests/🔬️translation-totality/🟦️.ts` (en + de, normal + beginner, distinct, for every code + apply family).
- Ticket proofs: `🧪️w3-codes-outcome-law.ts` (rule 2 alone, `--with-leaves` adds rule 1), `🧪️w3-codes-gate-negatives.ts`
  (scratch git repo, one planted violation per position, expects exactly 13).

## 5. Verification (run, observed)

| check | result |
|---|---|
| rule 2 (`🧪️w3-codes-outcome-law.ts`) | 0 breaches, 14–25 s |
| gate negatives | 13 reported, missing [], unexpected [] (also re-run green by energy after its inventory refactor) |
| full `verify mutation-outcome-law` (energy, after shared inventory) | 47 breaches, all rule 1, all 📕️norm (§7) |
| `cargo test --lib -p semio-framework-replication -- outcome_code` | 3/3 |
| `cargo test --lib -p semio-framework-os-kernel` (vocabulary walk, energy's spr round trip, ledger, conflict retirement) | 4/4 |
| `cargo test --lib -p semio-framework-plugin -- time_travel job_test test_app history_code` | 40/40 |
| `cargo check` framework (replication, os-kernel, plugin, mcp+tests, renderer-wgpu+tests) | clean |
| `cargo check` 30 touched s crates (remodel, raster, draw, layout, wfc-bitmap, home, energy, fem-3d, 22 stdio) | clean |
| lib tests green | wfc-3d 187, wfc-bitmap 153, gismap 159, norm-en1992 100, iso16757 314, forms 230, dag 214, stdio semio 2682, step 148, ifc 135, json 90, las 46, obj 44, ply 46, stl 44, zip 68, dxf 36, contract 62, csv 41, docx 76, pdf 521, svg 94, xlsx 75, xml 69, home 29 |
| Python oracles | wfc bitmap 10 vectors / 0 failures; wfc3d 15 vectors / 0 problems |
| TS | replication outcome-code 2/2; forms response events twin pass; remodel suite 1373/1373; ui-react typecheck 0 errors, translation-totality 2/2, check-chrome-i18n clean |

Failures observed and NOT caused by this work (code strings / levels play no part in them):
- draw: example parse ("expected Bool, found Absent"), kinds catalog 17≠18, retained route dispositions — peer edits.
- layout: 2 editor tests (locked frame visibility, page row action) — listed pre-existing in status.
- grid3d: create-tile schema self-containment (peer).
- cad: concrete-forest demo asset digest (2); flow: 3 editor command refusal-by-name tests; reasoning: reorganize undo
  entry; curation: 12 editor window tests; remodel: 9 timing/editor/window tests; raster: 10 store-initializer /
  retention / media-export tests ("document store close awaits a retained reader or owner", initializer step budget).
- WRITTEN BUT UNVERIFIED (blocked): glTF lib tests after the stale-restore expectation update and the carrier-oracle row,
  and the media lane (png/jpg/wav/tiff/mp4/gif/mp3/avi/bmp/pptx) + layout lib tests — first blocked by a peer's
  `dsl::ArtifactCodec::bare` (plugin 🦀️.rs:3635/3648/3666/41860), then by the root workspace manifest
  (`🌎️hub/🧩️compositions/🗄️stdio` inheriting `semio-s-plugin-stdio`). All 30 crates passed `cargo check` before those
  breaks; rerun: `cargo test --no-fail-fast --lib -p semio-s-artifact-stdio-gltf -p semio-s-artifact-stdio-zip -p
  semio-s-artifact-wfc-bitmap -p semio-s-artifact-layout-layout -p semio-s-artifact-stdio-{png,jpg,wav,tiff,mp4,gif,mp3,avi,bmp,pptx}`
  and `-p semio-s-artifact-energy-model`. The remodel Python oracle (code substitutions only) runs only inside the repo
  test case runner against the Rust subject — not run.

## 6. Ticket-folder inputs

`🧪️w3-codes-remap.py` (lanes wfc3d, forms, apply-codes, remodel, raster, profiles), `🧪️w3-codes-schema-bounds.py`,
`🧪️w3-codes-child-identity-invariant.py`, `🧪️w3-codes-apply-level.py`, `🧪️w3-codes-feature-tables.py`,
`🧪️w3-codes-outcome-law.ts`, `🧪️w3-codes-gate-negatives.ts`. Outputs: `🗑️generated/w3-codes/` (census, builds, tests).

## 7. Follow-ups (not done, with reason)

1. 📕️norm: 47 rule-1 `🔺️diff` leaves never emit a vocabulary code (insert-* without duplicate-id, remove-* without
   target-missing, change-* without no-op) — norm groups own these files; routed to the coordinator.
2. DONE in §9: `🎯️outcome` level spelling normalised to `warning`, gate alias deleted.
3. DONE in §9: the checked-apply adapters (generation3d, workflow run) propagate the vocabulary outcome unchanged, with a law.
4. Peer fixture `🏪️store/🧫️fixtures/🧫️command-rejection` (untracked, active) uses `mutation.note` as decode-test data.
5. dxf generator `notes` prose still names bare details (`invalid-add-target`) — prose only.
6. Per-plugin TS twins for the snapshot-edit outcome mapping do not exist (no TS consumer produces those outcomes).

## 8. Files changed (by group)

- Framework: `📡️replication/🎮️mutation/🦀️.rs`, `📡️replication/🟦️.ts`, new `🎮️mutation/{🧬️schema/🔣️outcome-code,
  🧫️fixtures/🧫️outcome-code,🧪️tests/🧪️outcome-code}`, new `📡️replication/🧪️tests/🧪️outcome-code/🟦️.ts`; `🏪️store/🦀️.rs`
  (table delegation, presence apply codes), `🏪️store/🧪️tests/🔬️unit/🦀️.rs`; `🔌️plugin/🦀️.rs`, `🔌️plugin/⏪️time-travel/🦀️.rs`,
  plugin tests (time-travel, job-test-mutations + fixture, test-app-mutations-document); `🌉️mcp` workspace/dispatch/channel
  (+ tests, fixtures); `🏃️run`, `📡️spr/🧵️channel`, `🎠️kernel/🟦️.ts`, `💻️os/🟦️.ts`; renderer ShellHost, Shell (tsx + wgpu +
  tests), ProgramBridge wgpu, AgentBridge test, ShellHelpers test, engine-contract test; `🖱️ui` `📚️I18n/🟦️.tsx`,
  `🌐️i18n/🟦️.ts`, translation-totality test; repo library file-walk; root `📜️script.ts` (rule region).
- Plugins: wfc (bitmap, grid3d, 3d incl. schemas/python/feature/fixtures), norm (en1992, iso16757), draw, gis gismap,
  layout, cad (+ 5 schemas), forms (+ schema, events fixture, TS twin), dag, sourcing curation, remodel (diffs, TS twin,
  python, feature, fixtures, tests, 2 schemas), raster (+ 3 schemas), space home (config, transient, fixture, tests),
  flow, procedural gen2d/gen3d, reasoning wires, energy (editor/viewer faults, 12 diff levels, 1 builder literal), fem 3d
  docstring, 46 aggregate `🧬️schema/🦀️.rs` (`build.apply`), stdio (semio base/kit/object + 6 schemas, contract editing,
  zip base/iso21320 + TS twin + editor test, las, stl, obj, ifc, step, dxf, ply, html, epw, svg, bcf, binary, csv, tsv,
  xlsx, docx, md, xml, pptx, dwg, deflate, json, pdf ×9, gltf mapping/fixtures/tests, media png/jpg/wav/tiff/mp4/gif/mp3/
  avi/bmp, law oracle test).

## Session 2 — 2026-10-01

Successor S2-CODES (coordinator `⚪552b484a…`). Scratch: `🗑️generated/s2-codes/`. Rules 1–24 obeyed (no git writes, no ticket
tools, foreground, private `target-nde-s2-codes`).

### S2.1 Repair (rule 21)

The session-1 report promised a "§9" that was never written (cut ~08:00). Disk truth, verified file by file:
- `🧪️w3-codes-warning-level.py` had run (all `🎯️outcome` docs, Python twins, remodel TS twin, per-case Rust readers,
  features, `level_of` normaliser deleted) — EXCEPT the 23 layout files W3-T-LAYOUT authored afterwards (6 outcomes, 17
  readers), still `"warn"`.
- Workflow run checked seam (`apply_run_operation_checked` + law `checked_run_admission_matches_the_typed_diff_rejection`)
  and generation3d (`apply_generation3d_mutation` + law `checked_apply_propagates_the_vocabulary_outcome_unchanged`):
  source-complete and committed (auto-commit 11:16); workflow-run lib had passed 21/21 at 03:37.

### S2.2 One level spelling (`warning`), everywhere

- Re-ran `🧪️w3-codes-warning-level.py`: the 23 layout files; dry re-run = 0. (A peer's §14 short-slug rename of the layout and
  remodel case directories landed minutes later and carried these contents; verified no `"warn"` remains.)
- Generators that would re-emit `"warn"`: `🧪️w1-f-author-selection-vectors.py`, `🧪️w3-t-puzzle-author-vectors.py`,
  `🧪️w3-t-layout-author-vectors.py` (outcome dicts + the Rust reader template) → `warning`.
- Census (`git grep`, tracked + untracked, `🗑️generated/s2-codes/warn-census.txt`): no `"warn"` outcome level left in any
  fixture, decoder, TS or Python twin. Remaining `"warn"` hits are other vocabularies (trace record levels, hub
  observability log levels, lint severities, `console.warn`, go output types) and the Rust builder method names
  `MutationMessage::warn` / `MutationOutcome::warn` (an API name, not a wire spelling; the gate maps it to `warning`).
- Gate now refuses the alias instead of skipping it (`📜️script.ts` rule-2 region): TS `refuse(..)`/`{level, code}` and
  Python tuple patterns match `warn` so it is reported (`… at warn is fixed at warning, not warn`); new Gherkin
  `{level: …, code: mutation.…}` message check (code + level; `mutation.apply.<detail>` placeholders excluded); new Rust
  hand-decoder check `"warn" => …Severity::Warning` (`outcome level "warn" is a retired spelling of "warning"`).
  `🎯️outcome` documents already compared levels exactly. Proof: `🧪️s2-codes-level-alias-negatives.ts`.
- Second level tables removed: per-case readers derived a rejected outcome's level as `invariant ⇒ Fatal, else Error`
  (wrong for `duplicate-id` and `mutation.apply.*`). Now `protocol::outcome_code_level(code)`:
  17 layout readers + the layout author template, the lowpoly paint-stroke laws (`🧪️s2-codes-vocabulary-level-readers.py`).
- `🏪️store/🧫️fixtures/🧫️command-rejection` (session-1 follow-up 4): decode data `mutation.note` → `mutation.cascade` (info).

### S2.3 Checked-apply adapters

- generation2d `apply_generation2d_mutation` had the generation3d defect: it applied a refused diff's empty delta and
  answered `Ok` (refusal silently dropped). Now the same checked shape: `Result<(), Vec<MutationMessage>>`, the diff's own
  messages unchanged on Error/Fatal, apply-time rejection joined as the `Fatal` `mutation.apply.*` message. Stale unit test
  `delete_widget_on_unknown_id_is_a_noop_with_no_inverse` (it only passed because of the defect) →
  `…_is_rejected_with_no_inverse`; new law `checked_apply_propagates_the_vocabulary_outcome_unchanged` (target-missing
  ×4 kinds, invariant on a non-finite move, no-op). All callers are tests using `.expect`/`.is_ok()` (incl. the peer's new
  `🧪️gesture-leaves`).
- Workflow run + generation3d: unchanged (S2.1).

### S2.4 Remodel oracle

`bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts oracle exhaustive --case 📸️mutate-remodeling-1`
(the Python second implementation, all 36 kinds, incl. every remapped refusal code): **273/275 passed, 0 failed, 2 errored**
— both errors are `unresolved fixture …/🛠️update-camera/🔍️refines-the-cam-0eaef0/…`: a peer's in-flight §14 short-slug
rename of the remodel case directories (746 paths moved at 12:16, feature rows not yet re-pointed). `quick` plans 0 scenarios
(level-gated). Oracle-vs-Rust parity (`parity exhaustive`) needs the remodel Rust subject built: see S2.6.

### S2.5 Gate verdict (c)

| check | result |
|---|---|
| seven rules, one inventory (`🧪️s2-codes-outcome-law-bundle.ts`, same bundle + `high` filter as `verify mutation-outcome-law`) | **0 breaches** (all 7 rules 0), 162 s under load |
| rule 2 alone (`🧪️w3-codes-outcome-law.ts`) | 0 breaches, 118 s |
| `🧪️w3-codes-gate-negatives.ts` | 13 reported, missing [], unexpected [] |
| `🧪️s2-codes-level-alias-negatives.ts` | 6 reported (Rust decoder, TS refuse, TS object, Python, Gherkin, outcome doc), 0 canonical controls reported |
| `bun ./📜️script.ts verify mutation-outcome-law` | CRASHES before the gate: the `verify` router's owned-command table throws `Invalid owned command ✒️writer/📦️packages/🦀️rust/📋️project.json:graph-generate` (`workspaceCommand` of one word; also `graph-wire-check`) — peer writer graph work, committed 11:16; not this WP's tree |

Rule-1 breaches by owner: **none remain**. The 47 📕️norm `🔺️diff` leaves of session 1 are fixed (norm-2/norm-3).

