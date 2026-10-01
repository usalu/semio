# 📓️ Resume audit — evidence lanes (read-only)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, auditor: Sonnet state-reconstruction (read-only), measured 2026-10-01 11:35–12:30.
Scope: every lane that rolled out design §6 / §11 / §14 (`x-semio-ui`, schema-first payload schemas, committed wire witnesses, the 9-code
outcome vocabulary, payload round-trip laws): W1-D, W2-R*, W2-S*, W2-W*, W3-CODES, W3-TAX, W3-GLTF, W3-STDIO-CASES(-2), norm-1/2/3,
media, energy. Method: read the design, plan, status, all listed reports and every newer `🗑️generated/<lane>/` file; ran only bun/python
scripts (no cargo, no nx, no servers, no modifying git). Raw outputs: `🗑️generated/resume-evidence/` (index in §9).
Aliases: `S` = `✏️s/🔌️plugins`, `STD` = `S/🗄️stdio/🗿️artifacts`, `NORM` = `S/📕️norm/🗿️artifacts`, `T` = the ticket folder.

## 0. Bottom line

1. The evidence rule is essentially met on disk. At 11:35: `schema mutation-payloads` 5077/5078 fixture rows clean, **3066/3066 leaves witnessed**,
   191 negative witnesses (9 by declared `x-semio-invariant`), 2 findings; `schema mutation-inputs` 5424/5427 inputs of 3025 leaves declared, 18 findings.
2. The 9-code outcome vocabulary is enforced: `verify mutation-outcome-law` rule 1 (leaf diffs reference a vocabulary code) = **0** (the 47 norm breaches of 02:31 are gone),
   rules 3-7 = 0, rule 2 (code and level at every outcome position) = **6** (six `"level": "warn"` fixtures in layout, W3-T-LAYOUT leaves written 02:32).
3. Norm path budget (design §14, option B) is **done**: 9,961 norm files, 0 over 240 bytes (max 240). The Python oracle of all 15 norm mutate cases passes in-process (see §4.3).
4. What is NOT done is verification, not authoring: every Rust compile/test/parity run after ~07:25 is missing. Cause: peer breakage (non-fleet `ArtifactSqliteSnapshot` rollout, writer `project.json`)
   and, since ~12:00, a repo-wide rename (REPO-PATH-BUDGET `🧪️apply.py --apply`, PID 81513 alive at 12:26, gone by 12:37, 4,366 renames, outside norm; its `rewrite.py` is dated 12:27, so a reference-rewrite phase may follow). Any non-norm cargo result is unreliable until it finishes.
5. The census is a moving target: at 12:24 the same two lints read inputs 41 findings / payloads 8 findings (3064/3066 witnessed). The extra findings are concurrent renames
   (energy and procedural catalog rows point at renamed dirs, glTF kind-tag drift in two fixtures) and new W3-T leaves, not regressions of the evidence lanes (see §3).
6. Real half-done work found by mtime/log forensics: din4108 outcome-class step (1 failing crate test), W3-CODES report truncated plus 6 `warn` fixtures,
   W3-STDIO-CASES red cases (xml 1/14, dxf 0/7, dwg 4/6 x2, bcf viewpoint 6/8) with no report, W3-STDIO-CASES-2 report placeholders, media follow-up applied but never run,
   energy Rust verification, 8 leaf-dir/kind identity mismatches.

## 1. Global blockers and concurrency (read before launching anything)

| id | what | evidence | effect |
|---|---|---|---|
| B1 | REPO-PATH-BUDGET (other session `⚪5dba80e6…`) renames over-budget dirs outside norm and rewrites references | `.🧬semio/…/☀️01/REPO-PATH-BUDGET/🧪️apply.py --apply` running at 12:26 (ps 81513; gone 12:37), `rename-map.txt` 4,366 lines, glTF: 548 tracked deletions + 96 new dirs | every case/fixture path in the old reports is stale for energy, stdio, architect, puzzle, remodel, fem, procedural, block, shooting, gis, wfc, … ; do not edit evidence trees outside norm until it lands |
| B2 | non-fleet `ArtifactSqliteSnapshot` rollout (stdio, wfc, os, note) | `🪶️sqlite` codec `.rs` in 42 artifact dirs, edits as late as 11:50 (glTF 282 files incl. 239 TS twins re-pointed to `🧰️framework/…/🪶️sqlite-snapshot/🔢️ieee754`, `🗄️.sql`) | stdio/wfc crates and energy (via stdio semio) did not compile at 02:40 and 07:48 (`E0599 ArtifactSqliteSnapshotCodec::of`, `E0277 DocxSnapshot`); current health unknown |
| B3 | root `bun ./📜️script.ts …` crashes in routing | `Invalid owned command …/✒️writer/…/📋️project.json:graph-generate` (`workspaceCommand: ["graph-generate"]` has 1 word, router needs >= 2); reproduced 11:36 and 12:25 | `verify mutation-outcome-law` cannot run from the root; run `bun T/🧪️w2-r-energy-outcome-law.ts` (all 7 rules, 2.5 min) instead. Test-module commands (`cwd 🧪️test`) are unaffected |
| B4 | PAGED-ARTIFACT-HISTORY-LEDGER and PER-VIEWER-ALTERNATIVE-HEAD sub-tickets (other sessions) edit store/vcs/plugin | `🏪️store`, `🌿️vcs/🦀️.rs`, `🔌️plugin/🦀️.rs` mtimes 12:00 | framework crate churn; plugin crate may not build |
| B5 | `schema generate` (central catalog) is coordinator-gated and not run since 09-30 | `test schema` baseline: 667 `schema-catalog-stale`, 5 `schema-catalog-malformed`, inputs lint 14-25 `leafUncatalogued` | the lint cannot read new leaves until it runs |
| B6 | taxonomy validity flickers (peer edits) | invalid at 07:40-08:16 (3 `generatorContracts` order errors), valid at 11:36 and 12:00 | wrap taxonomy-dependent commands with the `loadCatalogTaxonomy()` poll used in `T/🧪️w3-stdio-run-cases.sh` |
| B7 | auto-commit at 11:16 (`4e36b2b5012`, "665") absorbed all session-1 work | `git diff` against HEAD shows only post-11:16 churn | interrupted-edit detection must use mtimes and logs, not `git diff` |

Fleet rules for successors: `T/📌️important/📝️.md` (items 21-24: successor duty, new crate-manifest layout under `🌎️hub/🧩️compositions/<plugin>/📦️packages/🦀️rust/`, no `describe`/activation).

## 2. Lane reconstruction

Format per lane: last assignment / done with evidence / unverified / remaining checklist / interrupted-edit suspicion / blockers / successor brief.

### 2.1 W1-D input descriptors (agent a6587486…)

- **Last assignment:** design §6 end to end plus follow-ups §6 (unions, cycle guard, integer refs, collecting lint, hidden stop, color, vector facets), §7 (os.store split, `s.stdio.registry` runtime), §8 audit fixes (Apply accessors, generic payload law, nullable, host roster, glossary), §9 (law cold retirement). Last status line: 11:40 resume, §9 written 17:05.
- **Done (evidence):** `T/📓️w1-d-report.md` §9: 103 test binaries, 199 `semio_payload_law_*` tests, 0 failures (102 crates + kernel + plugin); TS mutation-inputs 80/80; Python 200 verdicts; reader fault class empty today (my 11:35 and 12:24 runs show no `readerFault`).
- **Unverified:** the law sweep predates the sqlite rollout (B2) and REPO-PATH-BUDGET (B1); glTF law rerun was blocked (report §9.2).
- **Remaining:**
  - [ ] `os.config.mutation.attach-local-folder` `/folder`: `widgetIncompatible` ("widget text cannot edit this value"). Leaf `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧬️schema/🔣️.json` (W2-B follow-up 4, 09-30 22:52): `folder` is `$ref LocalFolderRef` (path | handle union) with `widget: text`. Make it `widget: hidden` or a discriminated union the reader supports.
  - [ ] hand-written and generic aggregates get no emitted `semio_payload_law_*` (report §9.2); list them with `grep "impl Mutation"` and add the law call by hand or give them the derive.
  - [ ] plugin descriptors: 21 of 34 `🌎️hub/🧩️compositions/<plugin>/🔣️.json` are dated 04:24-04:27, i.e. before W1-D's 07:10 and 08:55 follow-ups (vector `snaps`, `nullable`, `Reference.kinds`); central `describe` regeneration is the coordinator's (fleet rule 23).
  - [ ] wgpu color/reference/dial recipes (W1-E/W2-C scope, not evidence).
- **Interrupted edits:** none found.
- **Blockers:** B1, B2 for the law sweep.
- **Brief:** treat W1-D as complete; only the attach-local-folder widget and the law coverage gap remain, both small.

### 2.2 W2-R* input-UI rollout (stdio-a, stdio-b, norm, energy, architect+remodel, mid, design, tail)

- **Last assignment:** annotate every leaf input (`x-semio-ui`), then negatives rollout (design §11): fem 16 and remodel 7 (mid), energy 149 (energy).
- **Done:** all eight groups reported lint 0 for their scope (reports 05:05-05:25 and 16:46, 02:59 for energy); today only 4 W2-R-owned items remain (below). Negative witnesses: fem 21 (4 by invariant), remodel 5, energy 134 (4 by invariant), puzzle 18, norm 5, layout 3, lowpoly 2 (payload census).
- **Unverified:** Rust compile of the W2-R edits (schemas are `include_str!`'d by the leaf derives): energy report §9.5/§11.5 "WRITTEN BUT UNVERIFIED" (peer breaks), remodel 7 failing lib tests inferred pre-existing, "Rust subject and parity phases of `📸️mutate-remodeling-1` not run".
- **Remaining:**
  - [ ] `s.stdio.semio.v1.kit.mutation.set-snapshot` `/snapshot/schema` labelMissing (leaf `STD/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/📸️set-snapshot`, refs `kit/snapshot.json`, which was rewritten 06:32 by the sqlite peer: `schema` is `{type: string, x-semio-state: artifact}`). Fix: `x-semio-ui {widget: hidden}` on that property or an en/de glossary row (glossary is W1-D's file).
  - [ ] energy 4 identity mismatches (leaf directory slug != descriptor `semanticKind`): `change-humidistat-dehumidifying-setpoint` vs `…-schedule`, `change-ideal-loads-system-max-heating-supply` vs `…-air-temp`, `change-ideal-loads-system-min-cooling-supply` vs `…-air-temp`, `change-humidistat-dehumidifying-throttle` vs `…-range` (W3-TAX §8.3). Decide per leaf: rename kind (wire change: schema, Rust, fixtures, features) or directory. REPO-PATH-BUDGET is renaming exactly these energy leaves now (§1 B1): coordinate, do not duplicate.
  - [ ] 14 stub-schema leaves flagged in W2-R-mid §5.2 are no longer opaque (payload census `opaque` 0), nothing to do.
- **Interrupted edits:** none.
- **Brief:** W2-R is complete; remaining items are the kit label, the energy identity decision, and Rust verification of energy/remodel.

### 2.3 W2-S schema/payload parity (S, S-A norm, S-B1 glTF, S-B2 stdio, S-C design, S-E layout+tail)

- **Last assignment:** F1-F17 (aggregates schema-first, lints inside `test schema`, wire-witness rule, negative-witness rule, `x-semio-invariant`). Last status: W2-S 11:55 (F17), S-A 16:45, S-B1 08:30, S-B2 09:10, S-C 18:55, S-E 17:00.
- **Done:** both lints are wired into `test schema` (`🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1008-1012`); payload census clean for every plugin (§3). glTF 0/0 (375/375 rows, 121/121 leaves).
- **Unverified:** S-E "fixture tests -> W3-G rerun" (layout), S-C raster parity and `descriptor_is_fresh` (needs `describe`), S-A `semio-s-plugin-norm` tests not run.
- **Remaining:** none owned by W2-S except the 6 layout `warn` fixtures (below) and what §3 lists per class.
  - [ ] layout: 6 fixtures `🧫️fixtures/🧬️mutations/{✋️drag-frames,🔃️rotate-frames,🗜️scale-frames}/{⏸️keeps-a-zero-offset|angle|unit-factors,⚠️skips-a-locked-and-a-missing-frame}/🎯️outcome/🔣️.json` carry `"level": "warn"`; gate rule 2 wants `"warning"`. Change the string (and the owning Rust/Python expectations if they spell `warn`); the files were authored 02:32, after W3-CODES' 70-fixture sweep.
- **Interrupted edits:** none.
- **Brief:** complete; hand the 6 fixtures to the layout/CODES successor.

### 2.4 W2-W stdio conversions: bim, document, text, office, geometry

- **Last assignment:** F10 conversion (feature rows to wire params, generic `from_payload_value`, payload-only witnesses). Done 17:35-18:50 with lints 0; their red cases were routed to W3-STDIO-CASES(-2) (below).
- **Done:** bim ifc 145 + step 42 witnessed, 14/14 cases parity, lib 135+148; document pdf 149, semio 241, docx 32 witnessed; text 8 artifacts, lib 487 tests; office 11 cases green; geometry 94 -> 0, lib 350.
- **Unverified:** all of it after the sqlite rollout (B2).
- **Remaining (their own open lists, still open):**
  - [ ] json/i-json leaf schemas type `JsonValue` fields as `{"description": "any JSON value"}` (7 leaves: `🧾️json/…/🧱️base/…/{✏️set-member,📥️insert-array-element,🔢️set-scalar}`, `…/🛜️i-json/…/{➕upsert-member,🌳set-top-level,📥insert-array-element,📸️set-snapshot}`); should `$ref` `json/rfc8259/base/snapshot.json#/$defs/JsonValue` with `lexeme/items/members` labels so `mutation-inputs` stays 0.
  - [ ] `deserialize_double_option` is duplicated in 7 crates (html, bcf, docx, xml, pptx, gltf diff, semio model); move to `S/🗄️stdio/📇️registry/🧬️contract`.
  - [ ] `no-mutation` dead branches remain in `S/🗄️stdio/🔮️oracles/📃️document/🦀️.rs` (lines ~985, 1112, 1576, 1586), `…/⚖️law/🧪️tests/🔬️unit/🦀️.rs:73`, and the generator scripts of gif, avi, dxf; `params_are_wire` law itself landed (03:04).
  - [ ] csv and json oracle catalogs list `patch-snapshot` in `kinds` with no `mutationManifests` entry (W3-TAX §6.5).
  - [ ] obj reader-oracle corpus still has the `no-mutation-no-op` identity recipe (geometry §4.5).
- **Interrupted edits:** none; the 07:56-08:38 stdio mtimes (avi, las, step, pdf) are the sqlite peer.
- **Brief:** the lints are done; the successor owns the three cleanups above after B1/B2 settle.

### 2.5 W2-W-media (agent a4bb11c3…)

- **Last assignment (follow-up, resumed 03:21):** decide and document the bmp/gif raster divergences, declare the undeclared snapshot-editing kinds (`set-snapshot`, `patch-snapshot`, png `patch-pixels`), wav oracle phase, refresh inventories.
- **Done on disk (07:26-07:54, never reported):** bmp `replace-pixel-data` vectors re-derived from the decided BITMAPINFOHEADER rule (`🧪️w2w-media-bmp-vectors.py`, 07:26); gif 87a/89a features rewritten with `@mode-error` outcome (`🧪️w2w-media-gif-decisions.py`, applied 07:41, `@mode-error` present in both features); snapshot-edit rows for png/jpg document/tiff document/bmp and owner manifests + before/after fixtures (`🧪️w2w-media-snapshot-rows.py` 07:49, `…-manifests.py` 07:50, files 07:50:30-07:50:47); payload lint per artifact at 07:50-07:54: gif 31/31, png 18/18, jpg 21/21, wav 6/6, tiff 16/16, bmp 6/6 witnessed, 0 findings (`🗑️generated/w2w-media/p-fu-*.json`).
- **Unverified:** everything Rust/Python-executed after 22:32: last test attempt `fu-bmp-test.txt` (07:44) died on 25 peer compile errors in `🔌️plugin/⏪️time-travel`; no parity run after the follow-up; the original report (22:32) still says "14 of 17 cases green, 3 known divergences".
- **Remaining:**
  - [ ] `cargo test --lib` for bmp, gif, png, tiff, jpg (and wav, mp4, avi) crates; then `parity exhaustive --case` for `🪟️mutate-bmp-v3`, `🖼️mutate-gif-87a`, `🎞️mutate-gif-89a`, `🔀️mutate-png-1-2`, `🖼️mutate-tiff-6-0`, `📸️mutate-jpg-jfif-1-01` plus baseline subsets (expect the three divergences gone).
  - [ ] bridge inventory refresh (`test inventory --artifact … --standard …`) for bmp, png, tiff (both subsets), jpg (both subsets): cache still 09-25; production offers `set-snapshot`/`patch-snapshot`/`patch-pixels` that catalogs now declare.
  - [ ] wav oracle `patch-snapshot` is never executed (`@no-oracle-frozen-hound-pcm16`); avi oracle unit-test target did not compile (dxf smoke `include_bytes!` path from a peer rename).
  - [ ] append "Session 2" section to `T/📓️w2-w-media-report.md` (follow-up items are unreported).
  - [ ] wav `set-snapshot` witness `…/📸️set-snapshot/🔊️resamples…/🦠️mutation/🔣️.json` is invalid (`.snapshot.chunkOrder[2]`: `{"kind":"other","value":"0"}` fails 3 subschemas); file mtime 11:19, i.e. the sqlite peer, not media: check against the leaf schema (`value` type) with the peer before touching.
- **Interrupted edits:** low suspicion (scripts are idempotent, lints green after).
- **Brief:** media is authored and lint-clean; run the Rust/Python verification chain once B1/B2 are quiet, refresh the six inventories, then write the Session-2 report section.

### 2.6 W2-W-norm-1 (en1991 + en1990) (agent a6f7dd4b…)

- **Last assignment:** §8 follow-up (done 22:42): target-missing recoding, 17 emoji-lost en1991 leaves restored, scenario names shortened, outcome classes derived, shared binary codec, inventory refresh. Idle after 02:23 (option B applied by norm-2 for its one over-long en1991 kind).
- **Done (evidence):** en1991 80/80 and en1990 30/30 witnessed; `cargo test --lib` en1991 160/160, en1990 140/140 at 22:3x, and again en1990 151/151 and en1991 160/160 at 07:59 (`T/🗑️generated/w2w-norm-2/test-six.log`); parity 322/322 and 122/122; contract breaches 137 -> 0; bridge inventory 80/80 and 30/30. Today: payload/inputs 0 findings; oracle probe 161/161 and 72/72; path budget 0 over.
- **Unverified:** nothing after 07:59.
- **Remaining:** [ ] re-run crate tests + `parity exhaustive` for both cases after the option-B renames (cheap, part of WP-1).
- **Interrupted edits:** none (its two `🐍️.py` adapters rewritten 07:44:35 by the norm-2 case renderer; oracle probe passes).
- **Brief:** complete; fold its verification into the norm closing WP.

### 2.7 W2-W-norm-2 (en1996, din16798, din18599, din4108) (agent a8153e59…)

- **Last assignment (resumed 21:47):** din18599 folder renames, din4108 delta renames, binary wire records, path-budget analysis, inventory refresh; then (02:23) apply option B; then the "outcome law" step (`T/🧪️w2w-norm-2-outcomes.py`: `refusals|classes|sync|suite`): witness each leaf's duplicate/missing/no-op/clamp/rule outcome with a committed vector and keep descriptor `outcomeClasses` truthful.
- **Done (evidence):** din18599 folder renames and 4 `🚫rule` negative vectors (`07:44:14`, leaf schema bound added 07:44:51); delta renames (din4108 `📈️change-element-delta-uf|ug|ur`); path budget 0 over; catalog scenarios == bundles for all 15 artifacts (checked 12:0x); crate suites at 07:59 (`test-six.log`): din16798 92/92, din18599 119/119, en1990 151/151, en1991 160/160, en1996 147/147, **din4108 141/142**; oracle probe 100% (din18599 now 43/43 vs 39 earlier).
- **Interrupted at 07:59 (confirmed):** din4108 step. Failing test `…/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs:169 committed_vectors_are_this_implementations_answer`: descriptor classes declare outcomes the committed vectors never reach: `change-airtightness-n50`, `change-rh-int`, `change-t-int-c` declare `{applied,no-op,rejected}` (descriptors from 09-26) but reach `{applied,no-op}`; `insert-element`, `insert-thermal-bridge`, `insert-zone` declare `{applied,rejected}` but reach `{applied}`. din4108 has 0 refusal/no-op vectors (en1990 has 22, din18599 10, din16798 12, en1996 8 …). 07:59:05 also re-wrote the demo and `🎬️failing-thin-insulation` DSL/pack twins (4 files same second): confirm they still round-trip.
- **Remaining:**
  - [ ] din4108: `python3 T/🧪️w2w-norm-2-outcomes.py refusals 🧱️din4108 insert-zone=dupe insert-element=dupe insert-thermal-bridge=dupe change-…=noop|rule`, then `classes`, `sync 🧱️din4108 <catalog-id> 🧱️mutate-din4108-1`, `suite`; re-run `cargo test -p semio-s-artifact-norm-din4108 --lib` (expect 142+N).
  - [ ] din18599: 9 leaf dirs flagged "do not render their kind" (`🏷️use-class`, `🧮method`, …): renames reported done (07:44 batch); confirm the taxonomy no longer reports `mutation-payload-schema-authority-invalid` (my descriptor scan: dir slug == kind for all 569 norm leaves except en1992 `🧷change-anchor-a-s`).
  - [ ] binary-protocol-drift for en1996, din18599 (norm-2 §5.2): resumed 21:47, no report; run the scoped contract with the binary drift rule.
  - [ ] runtime inventory (bridge) for en1996, din16798, din18599, din4108 was refreshed 22:46-23:21, before the outcome vectors/renames: refresh again.
  - [ ] append Session-2 section to `T/📓️w2-w-norm-2-report.md` (last written 21:45; omits renames, option B, outcome vectors, inventories).
- **Blockers:** cargo (B2 not applicable to norm crates at 07:59: they compiled in 8 m 51 s).
- **Brief:** finish the din4108 outcome vectors and class reconciliation, re-run the four crates, refresh inventories, report.

### 2.8 W2-W-norm-3 (en1992/1993/1994/1995/1997/1998/1999, iso16757, vdi3805, results) (agent a0b1aae9…)

- **Last assignment:** (21:32) rename 4 over-long en1998 kinds (done), en1998 inventory refresh (done 22:20), restore red cases en1992/1993/1994/1995/1997/1999/iso16757/vdi3805; (22:20) en1992 renames `change-action-vk/nk` (done: dirs exist); (02:23) option B + en1995 renames; (07:25-07:55) en1995 conversion (`plan-en1995.json` 07:25, 419 files 07:23-07:55, `settle-en1995` 07:55, last edit `…/🪵️en1995/…/🧬️mutations/🦀️.rs` 07:55:30).
- **Done (evidence):** all 10 scopes lint 0/0 (`🗑️generated/w2w-norm3/lint-*` 07:38-07:44); en1998 59/59 x3 phases (report), en1992/1993/1997/1999 crate tests 100/161/99/80; oracle probe today 59/59, 115/115, 51/51, **135/135 (en1995, was 121/135)**, 44/44, 68/68, 38/38, 59/59, 39/39; catalog scenarios == bundles for all; `#[path]` check of 1,531 attributes in files touched 07:00-08:40: 0 dangling; no `[DEBUG]`/`debug_settle_vectors` left in norm; path budget 0 over (en1995 max 240 bytes).
- **Unverified:** no Rust test, subject or parity run for en1992-en1999, iso16757, vdi3805 since 03:3x; the en1995 settle output was never followed by a test run; its report (21:28) documents only en1998 and the witnesses.
- **Remaining:**
  - [ ] en1992 `🧷change-anchor-a-s` vs descriptor `change-anchor-as` (only remaining norm identity mismatch): one rename across leaf dir, fixtures mirror, canonical test dir, `#[path]`, catalog, taxonomy fixture `📇️mutation-leaf-taxonomy-v1` (regenerate with `mutation-leaf-taxonomy-generate`).
  - [ ] cargo: `cargo test --lib` for en1992, en1993, en1994, en1995, en1997, en1998, en1999, iso16757, vdi3805 + `semio-s-artifact-norm-contract` (`--test config_mutation` too); wasm32-wasip2 check; `semio_payload_law_*`.
  - [ ] phases: `oracle|subject|parity exhaustive` for the 9 mutate cases (oracle already 100% in-process).
  - [ ] bridge inventory for en1992, en1993, en1994, en1995, en1997, en1999, iso16757, vdi3805 (never refreshed since 09-25 per its §6; en1998 refreshed).
  - [ ] binary-protocol-drift en1992, en1999 (norm-3 §7.3).
  - [ ] Session-2 section in `T/📓️w2-w-norm-3-report.md` (en1995 conversion, renames, option B).
- **Interrupted edit:** en1995 conversion finished on disk (consistent), verification missing.
- **Brief:** run the Rust and case verification for all nine scopes, resolve the anchor identity, refresh inventories, report.

### 2.9 W3-CODES (agent a5a85a54…)

- **Last assignment:** classify and remap every non-vocabulary outcome code repo-wide, one protocol table (`protocol::OUTCOME_CODES`), gate rule 2; follow-up: drop the `warn` level alias (70 fixtures), generation3d/workflow checked-apply adapters propagate vocabulary outcomes (+ law), remodel oracle, deferred reruns.
- **Done:** vocabulary table + fixture + TS twin + i18n + persistence (`T/📓️w3-codes-report.md` §1-§5); gate negatives 13/13; rule 2 0 breaches (03:xx); workflow-run checked-apply law (21 tests ok 03:37); 41 Error->Fatal level fixes; today full rule run: rule 1 **0**, rules 3-7 0, rule 2 **6**.
- **Unverified:** the deferred cargo reruns (glTF, zip, wfc-bitmap, layout, media crates, energy-model; command in report §5) never ran: `check-followup5` (07:48) stopped at `semio-s-artifact-stdio-las` (`E0599 ArtifactSqliteSnapshotCodec::of`), `check-kernel-wait` (07:52) and `check-os-run` (07:55) were clean.
- **Interrupted (confirmed):** report cites "§9" twice (§7 items 2 and 3) but the file ends at §8 (mtime 03:04); the claimed alias drop missed the 6 layout fixtures (§2.3).
- **Remaining:**
  - [ ] fix the 6 layout `warn` -> `warning`, re-run `bun T/🧪️w3-codes-outcome-law.ts --with-leaves` (rule 2 + rule 1).
  - [ ] write the missing §9 (alias drop, checked-apply adapters, remodel oracle result `275/275 exhaustive` seen in `oracle-remodel-ex.txt`).
  - [ ] the deferred reruns listed in report §5, after B1/B2.
  - [ ] report §7 open items 4-6 (peer fixture `🏪️store/🧫️fixtures/🧫️command-rejection`, dxf generator prose, TS twin of snapshot-edit mapping).
- **Brief:** tiny: finish the report, flip six strings, rerun the deferred crates.

### 2.10 W3-TAX (agent a3f5f2ca…)

- **Last assignment:** register `🧾️wire-witness` and `🩹️patch-snapshot`; follow-up 1: structural mutation-leaf identity (replace per-name lists); decision 07:24: restore the sealed `📽️nested-cargo-package-projection` catalog; 05:50 decision: re-target the `🔬️workspace-contract` package-move coverage onto a synthetic sealed fixture workspace (after the structural rule).
- **Done:** `mutation-wire-witness` kind, 1,800 leaf names removed from `members-of-schema` (44 left), 2,433 `directory-kind-unresolved` removed, 0 added (75,885-directory census); tests `🧪️mutation-leaf-identity` 8/8 (07:27) and `🧪️mutation-wire-witness` 11/11 (07:29); `validateTaxonomy` passes at 11:36.
- **Unverified / interrupted:** last activity 08:07/08:13: subject cases `🔤️rename-casings` 8/8 and `🚚️file-folder-move` 6/6 (package-move re-target): no report entry; completion unknown. Inventory-level before/after diff "WRITTEN BUT NOT RUN" (§8.6). Report last written 07:32.
- **Remaining:**
  - [ ] confirm/finish the workspace-contract package-move re-target; run `bun test …/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`.
  - [ ] launch rows: `.vscode/launch.json` has `test-mutation-wire-witness` and `test-mutation-case-pair` but **not** `test-mutation-leaf-identity` (coordinator regeneration).
  - [ ] re-run scoped `verify taxonomy report` for wav and json (blocked by the taxonomy break at the time; ~20 min each) and the norm artifacts (expect `path-too-long` 0, `directory-kind-unresolved` 0).
  - [ ] 8 identity mismatches (§3.3).
  - [ ] optional hardening: sealed-evidence tamper as a finding instead of a throw; library test that every sealed document still matches its digest.
- **Brief:** finish the package-move coverage, add the launch row, rerun the scoped taxonomy verifies after B1 lands.

### 2.11 W3-GLTF (agent a754a7f5…)

- **Last assignment:** glTF TS twins and two-level leaf taxonomy. Done 02:31 (status 02:35).
- **Done:** tsc 2,857 -> 0 over 260 files; 66 -> 2 parser-missing (the 2 left are generated surface twins); TS witness 486 pass; lints 0/0; `cargo test` 265 pass at 17:05; engine tests registered.
- **Concurrent rewrite risk:** since 11:45 the sqlite peer re-points 239 glTF TS twins (`import type {Binary64} from "../…/🪶️sqlite-snapshot/🔢️ieee754"`) and REPO-PATH-BUDGET shortens glTF case dirs (e.g. `✅️required-extension/➕️add/🔬️t041` -> new dirs under `🌲️scene-root/…`, `🌳️node/⚖️change/…`); at 12:24 two glTF fixtures show kind-tag drift (`…/🔤️primitive/🔀️reorder/🔬️t059` says `reorderPrimitiveAttributes`, aggregate says `reorderPrimitives`; `…/🚚️move/🔬️t060` likewise).
- **Remaining:** none owned; after B1/B2 settle: re-run `bun nx run @semio-tech/stdio-gltf:test`, `cargo test -p semio-s-artifact-stdio-gltf --lib`, `🧪️w3-gltf-twins.ts --check`, `🧪️w3-gltf-cases.py --check`, `🧪️w3-gltf-catalogs.ts --check` (the scripts embed old paths).
- **Interrupted edits:** none from the fleet.

### 2.12 W3-STDIO-CASES (first; no root report) (agent adb239aa…)

- **Last assignment (status 17:50, 18:50, 17:55):** xml base wiring, dwg versions, bcf viewpoint round trip, dxf input + comparison outputs, md end-list marker, `params_are_wire` law, oracle-crate dxf smoke blocker, bridge/no-mutation consolidation.
- **Reconstructed outcome** (`T/🗑️generated/w3-stdio-cases/`, runner `T/🧪️w3-stdio-run-cases.sh`, final battery 08:14-08:16, "[w3-stdio] done" written at 08:16):
  - md `mutate-set-snapshot` fixed: 22/22, parity 11/11 (02:55).
  - `params_are_wire` law landed (`🔮️oracles/⚖️law/🦀️.rs:239`, 03:04); oracle crate lib tests 400 passed / 2 ignored (07:43, 524 s); `check-stdio` clean (07:29, 6 m 05 s).
  - bcf base 34/34 parity 17/17; bcf snapshot 4/4; **bcf viewpoint 14/16, parity 6/8**: `mutate-insert-viewpoint` and `inverse-insert-viewpoint`: "no artifact for role(s) actual-bcf" (adapter emits no `actual-bcf` for these rows; also `decode_bcf` drops viewpoint camera/components, geometry §4.2).
  - **xml base: 28/28 executed, parity 1/14**: `xml-1-0-quick-xml-compare-v1` stage 1 "exited 0 without a valid report: probe report is not an object" for every row (stage 0 import has no report; adapters emit `expected-xml`/`actual-xml` in `…/🧪️tests/📰️mutate-xml-1-0/🦀️.rs` but the probe output is not an object): wiring still red.
  - **dxf r12 header: 14/14, parity 0/7**: "bothImport is not equal to the declared value; dxf-compare reported no measurement `equal`" (R2000 `🚏️bus-shelter/🖊️.dxf` asset vs R12 rows, linetypes normal form, geometry §4.3).
  - **dwg ac1024 and ac1018: 12 executed, 10 passed, parity 4/6 each**: only `mutate-set-snapshot` and `inverse-set-snapshot` differ (1 difference); `set-version-info` rows now pass, so the AC1032/AC1018 decision is resolved.
- **Interrupted:** no report was ever written; the lane was mid-battery. dxf/xml/bcf/dwg files in scope show no mtimes after 07:07-07:50 except the sqlite peer (dwg 38 files until 08:38).
- **Remaining:**
  - [ ] xml: diagnose the probe (`xml-1-0-quick-xml-compare-v1`), emit the pipeline artifacts correctly, target parity 14/14.
  - [ ] dxf: re-home the rows to the R12 example (or regenerate expectations on R2000), make the `semantic-dxf-r12-v1` compare pipeline see both imports equal (linetypes), target 7/7.
  - [ ] dwg: the `set-snapshot` row differs by one field (see `parity-🖊️mutate-dwg-ac1024.txt`), target 6/6 in both subsets.
  - [ ] bcf viewpoint: emit `actual-bcf` for insert-viewpoint rows and fix the `decode_bcf` loss of camera/components, target 8/8.
  - [ ] consolidation: per-aggregate decode bridges into the contract crate; delete dead `no-mutation` branches (§2.4).
  - [ ] write `T/📓️w3-stdio-cases-report.md` from this section plus the runs.
- **Blockers:** B2 for every cargo-backed step (parity runs build the stdio hosts).

### 2.13 W3-STDIO-CASES-2 (pdf encoder, docx base, semio drawing) (agent id not recorded in status.md)

- **Last assignment:** fix pdf 1.7 base and six conformance subsets (typed lanes overwriting COS edits, catalog re-statement, dropped trailer entries), docx base judge, semio drawing census/inverse.
- **Done (evidence):** pdf base 66/66, parity 33/33 (23:55); subsets `♿️mutate-pdf-1-7-ua` 46/46 parity 23/23 (07:50) and `⚕️mutate-pdf-1-7-h` 42/42 parity 21/21 (08:13). Product fixes described in the report §2-§4 (writer `io::reconcile`, `carry_graph_edit`, docx XML-logical digests, `inverse_unflatten_node`).
- **Unverified / interrupted:** report §1, §5, §6 are literal placeholders ("filled from the runs"); subsets `🧾️vt`, `🗄️a`, `🖨️x`, `📐️e` last ran 02:29 and failed on a peer's catalog import (`../../../🧬️schema/✅️validation/🟦️.ts` missing), never rerun; no docx base or semio drawing parity run exists after the fixes (last logs 18:36-18:47).
- **Remaining:**
  - [ ] `parity exhaustive --case` for `🧾️mutate-pdf-1-7-vt`, `🗄️mutate-pdf-1-7-a`, `🖨️mutate-pdf-1-7-x`, `📐️mutate-pdf-1-7-e`, docx base/strict/transitional (`mutate-set-snapshot`/`inverse-set-snapshot`), semio drawing cases.
  - [ ] pdf, docx, semio crate `--lib` tests (pins in `🚪️io/🧪️tests/🔬️unit/🦀️.rs`).
  - [ ] fill report §1/§5/§6.
- **Blockers:** B2 (docx/pdf/semio snapshots are in the rollout list).

### 2.14 energy (W2-R-energy follow-ups, agent a7f3d627…)

- **Last assignment:** vocabulary extension (done 22:14), outcome-law gate speed + symlink safety (done, 95 s verdict), energy reds (zones-window tests, `identity-round-trip` Python oracle). Parked at 07:23 with 2 reds + oracle.
- **Done:** energy 738/738 inputs; 291 leaves witnessed; negatives 134 (4 by invariant); gate 7 rules on one git inventory; Python oracle `🏛️mutate-energy-model-1` 1149/1149 (02:43); zones-window tests rewritten to read merged table lanes (report §11.4).
- **Unverified:** all Rust: `cargo-check-energy.txt` (04:05) failed with 26 errors in `semio-s-artifact-stdio-semio` (sqlite); the zones-window and viewer twins never ran; baseline 6297 pass / 3 fail (zones x2, timing flake), parity 2297/2298.
- **Remaining:** [ ] `cargo test -p semio-s-artifact-energy-model --lib` (private target), `parity exhaustive --case 🏛️mutate-energy-model-1`; [ ] the 4 identity mismatches (§2.2, with B1); [ ] append Session-2 note.
- **Brief:** blocked by B1 (renames in its tree) and B2 (stdio semio compile).

## 3. Current census (merged lints, outcome law)

### 3.1 Per plugin (11:35 snapshot; `T/🗑️generated/resume-evidence/{inputs,payloads}.json`, table in `census-table.md`)

Commands (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): `bun ./📜️script.ts schema mutation-inputs --census --json` (12 s), `schema mutation-payloads --census --json` (67 s). Strict runs: `inputs-strict.txt`, `payloads-strict.txt`.
Totals: inputs 3025 leaves / 5427 inputs / 5424 declared / 18 findings; payloads 3066 leaves, 3066 witnessed, 3476 fixtures, 191 negatives (9 invariant), 1602 wire rows, 5077/5078 clean, 2 findings. (The payload universe is 41 leaves larger: per owner payload minus inputs is os +43 (framework test-fixture leaves under `🔌️plugin`, `🏪️store`, `📡️spr` are not catalogued for the inputs lint), gis +6, process -1, framework -1, window -6.)

Findings by class and artifact (all other plugins 0 / 0):

| plugin / artifact | inputs findings | payload findings | outcome-law |
|---|---|---|---|
| wfc (2d `✋️drag-slots`, `🎯️set-slot-positions`; 3d same two; bitmap `✍️paint-input-stroke`) | `leafUncatalogued` 5 | 0 | 0 |
| procedural (gen2d `🎚️change-slider-value`, `🚚️move-nodes`; gen3d `✋️drag-transforms`, `🎚️change-slider-value`, `📏️scale-transforms`, `🔃️rotate-transforms`, `🚚️move-nodes`) | `leafUncatalogued` 7 | 0 | 0 |
| fem 3d `🧭️move-selection` | `leafUncatalogued` 1 | 0 | 0 |
| lowpoly `🖌️apply-paint-stroke` | `leafUncatalogued` 1 | 0 | 0 |
| process3d `⏱️change-cursor` | `malformed` 1 (catalog row; leaf dir deleted in commit 665) | 0 | 0 |
| os `📎️attach-local-folder` | `widgetIncompatible` 1 (`/folder`) | 0 | 0 |
| draw `📍️drag-path-points` | `widgetIncompatible` 1 (`/targets`, array of structured items with `role: target`); gone at 12:24 (someone fixed it, unattributed) | 0 | 0 |
| stdio semio kit `📸️set-snapshot` | `labelMissing` 1 (`/snapshot/schema`) | 0 | 0 |
| stdio wav `📸️set-snapshot` witness `🔊️resamples-16-khz-doubles-pcm16-amplitude` | 0 | `invalid` 1 + `aggregate` 1 (`chunkOrder[2]`) | 0 |
| layout (`✋️drag-frames`, `🔃️rotate-frames`, `🗜️scale-frames` fixtures) | 0 | 0 | rule 2: 6 (`warn`) |

The 14 uncatalogued leaves all carry `x-semio-ui` on every property and have mirror witnesses (my script: props with no `x-semio-ui` = 0; mirrorFix 1-7 each), so `schema generate` should clear them without annotation work (unverified until run).

### 3.2 Same lints at 12:24 (`inputs-strict-2.txt`, `payloads-strict-2.txt`)

Inputs 5406/5408 of 3037 leaves, 41 findings: `leafUncatalogued` 25, `malformed` 13 (energy 7 catalog rows whose leaf dirs were just renamed: `change-humidistat-humidifying-setpoint-schedule`, `…-outdoor-air-per-area`, `…-per-person`, `…-infiltration-temperature-term-coefficient`, `…-velocity-squared-term-coefficient`, `…-fan-delta-pressure`, `…-fan-total-efficiency`; procedural editor-lane leaves 5: `set-generation-preview` x2, `set-selected-generation`, `set-preview-camera`, `set-preview-eval`; process 1), `widgetIncompatible` 1 (os), `labelMissing` 1. Draw `drag-path-points` was fixed in between.
Payloads 5073/5078, 3064/3066 witnessed, 8 findings: wav 2 (same fixture, dir now `🔊️resamples`), glTF 2 (kind-tag drift t059, t060), gen3d `🧹️delete-widget/🧹️unpins` says `deleteWidgetPosition` and leaf `🧹️delete-widget-position` is unwitnessed, shooting `🏷️set-camera-draft-label` unwitnessed and its fixture says `SetCameraDraftLabel`.
Third snapshot 12:33-12:37 (`inputs-strict-3.txt`, `payloads-strict-3.txt`): inputs unchanged (41); payloads 5073/5085 rows clean, 3064/3066 witnessed, **15 findings** (the 8 above plus 6 new puzzle 3d fixtures `✋️drag-selection/{⛓️drags-the-attracted-subtree,🎯️drags-object-and-volume,🪢️re-derives-the-crossing-attractions}`, `🔄️rotate-selection/{⛓️turns-the-attracted-subtree,🎯️turns-object-and-volume}`, `🔍️scale-selection/{⛓️scales-without-re-solving,🎯️scales-object-and-volume}` reported "the aggregate wire value is not an object (`Puzzle3dMutation`)": new since 11:35, written by a concurrent session (W3-T puzzle or the REPO-PATH-BUDGET rewrite), not by an evidence lane). At 12:37 no `apply.py` process was alive any more, but `REPO-PATH-BUDGET/rewrite.py` was written 12:27: that ticket may still be in its reference-rewrite phase.
Reading: stable remainder = os widget, kit label, wav fixture, 14 uncatalogued; everything else is in-flight renames (B1) or new W3-T leaves/fixtures. Re-measure before trusting any number.

### 3.3 Outcome law and identity (`outcome-law-rules.txt`, run 12:0x via `T/🧪️w2-r-energy-outcome-law.ts`, 2 m 33 s)

`policyMutationOutcomeBreaches` 0; `policyMutationMessageCodeBreaches` 6 (layout, above); NoCrdt 0; NoValidateOverride 0; SeverityInfo 0; MergePolicyParity 0; DeriveGlueMount 0. The root command is blocked by B3.
Identity (leaf dir slug vs descriptor `semanticKind`, scan of 2,679 descriptors): 8 mismatches: en1992 `🧷change-anchor-a-s`->`change-anchor-as`; energy x4 (§2.2); framework `🔁️set-state`->`set-interaction-state` (`🧰️framework/…/🔌️plugin/🕹️interaction/🧬️mutations`); plugin fixture `📝️set-test-count`->`set-count`; (one more hit, `4️⃣add-counter-four-times`, is a digit-prefix artifact of my regex, not a defect).

### 3.4 Adjacent `test schema` baseline (not part of the evidence rule; 6 min, `test-schema-all.json`)

4,811 findings repo-wide: `schema-export-incomplete` 1900, `schema-export-parser-missing` 993, `schema-catalog-stale` 667, `schema-owner-ineligible` 479, `schema-dialect-not-draft-07` 228, `schema-fixture-defines-schema` 202, `schema-ref-unresolved` 147, `schema-placement-outside-module` 105, `schema-export-unknown` 53, `schema-mutation-input-ui` 17, `schema-placement-forbidden-filename` 8, `schema-export-format-undeclared` 5, `schema-catalog-malformed` 5, `schema-mutation-payload-parity` 2. By plugin: stdio 1492, framework 1149, architect 700, **norm 321 (245 parser-missing, 32 owner-ineligible, 31 catalog-stale, 13 export-incomplete)**, procedural 111, puzzle 104, wfc 80, layout 80, fem 71, remodel 68, block 63. Mutation-leaf-path slice: 255 (ref-unresolved 60 mostly semio/docx/xlsx/forms cross-document refs, fixture-defines-schema 76, parser-missing 39, export-incomplete 14, placement-outside-module 30, input-ui 17, catalog-stale 16).

## 4. Cross-cutting measurements

### 4.1 Norm path budget (design §14 closed)
`norm-path-budget-now.txt`: 9,961 norm files, 0 over 240 bytes in every artifact (max exactly 240 in en1995, din4108, en1996; maxima 233-239 elsewhere). Option B is therefore applied norm-wide, including en1995 and en1996.
Repo-wide (`path-budget-repo-now.txt`): 6,808 files over 240 outside tickets (energy 1,251, stdio 1,173, architect 971, puzzle 415, remodel 302, fem 281, procedural 268, block 208, shooting 170, gis 162, wfc 158): REPO-PATH-BUDGET's scope.

### 4.2 Descriptor-vs-directory and path-attribute checks
All 569 norm leaf descriptors: dir slug == kind except en1992 anchor. `#[path]` attributes in the 1,279 files modified 07:00-08:40: 1,531 checked, 0 dangling (`mtime-0700-0840.txt`).

### 4.3 Python oracle, in-process (`oracle-probe-now.txt`, script `oracle-probe.py` = `T/🧪️w2-w-norm-3-oracle-probe.py` with its work dir moved into resume-evidence)
en1990 72/72, en1991 161/161, en1992 59/59, en1993 115/115, en1994 51/51, en1995 135/135, en1996 121/121, en1997 44/44, en1998 68/68, en1999 38/38, din16798 89/89, din18599 43/43, din4108 87/87, iso16757 59/59, vdi3805 39/39. (Last norm-3 probe 03:44: en1995 121/135, everything else equal or smaller.) This proves the oracle side only; subject and parity phases need cargo.

### 4.4 Misc
`[DEBUG]` scan (excluding sqlite): 34 files, none in norm, energy, wfc, layout or stdio evidence dirs (hits are renderer/test/remodel video/other tickets). `.vscode/launch.json`: `schema-mutation-inputs`, `schema-mutation-payloads`, `mutation-payload-parity`, `test-mutation-wire-witness`, `test-mutation-case-pair`, `test-oracle-source` present; `test-mutation-leaf-identity` absent. Colour widget used by 4 inputs only; snaps on 123 inputs (90 with `snapSource`); `x-semio-invariant` on 61 leaves.

## 5. Proposed work packages (at most 6, exclusive ownership)

Sequencing: WP-1 and WP-6 can start now (norm is excluded from B1 and compiled at 07:59). WP-2, WP-3, WP-4 start after REPO-PATH-BUDGET reports done (or at least after their trees are renamed) and a one-crate `cargo check` proves the sqlite state. WP-5 runs last, alone. Every WP: re-read before edit, private `CARGO_TARGET_DIR=…/target-nde-<wp>`, report as a new dated section in the lane report.

| WP | name | exclusive ownership | checklist |
|---|---|---|---|
| WP-1 | NORM-CLOSE | `S/📕️norm/**` (artifacts, shared `🔮️oracles/🏃️execution/🐍️.py`, `📇️registry`, `🧫️fixtures`, `🏭️bridge`, `🧪️tests`) except every `🟦️.ts` (WP-6) | (1) din4108 outcome vectors + class reconciliation (§2.7); (2) en1992 anchor identity (§2.8); (3) `cargo test --lib` for all 15 crates + norm contract (+ `--target wasm32-wasip2` check); (4) oracle/subject/parity exhaustive for 15 mutate cases; (5) bridge inventory refresh for 15 and contract phase; (6) binary-protocol-drift scan en1992, en1999, en1996, din18599; (7) scoped `verify taxonomy report` for each artifact; (8) `bun ./📜️script.ts oracle-source` suite; (9) Session-2 sections in the three norm reports. Exit: crate tests green incl. `semio_payload_law_*`, 15/15 cases in 3 phases, payload/inputs 0 |
| WP-2 | STDIO-DOCUMENT-TEXT-OFFICE | `STD/{📖️pdf,📜️docx,📕️xlsx,📽️pptx,🎒️zip,🗜️deflate,💾️binary,🧿️semio,📰️xml,🧾️json,📊️csv,📑️tsv,🌐️html,📝️md,🔤️txt,🎨️svg}/**`, `S/🗄️stdio/🔮️oracles/**`, `S/🗄️stdio/📇️registry/**` (excluding `🪶️sqlite` dirs and glTF) | xml compare pipeline 14/14; pdf subsets vt/a/x/e + docx + semio drawing parity; crate tests pdf/docx/xlsx/pptx/zip/semio; json `JsonValue` refs (7 leaves) with labels; kit `set-snapshot` label; `deserialize_double_option` into contract; `no-mutation` dead-branch cleanup; csv/json `patch-snapshot` manifest rows; finalize STDIO-CASES-2 report (§1/§5/§6); `contract exhaustive --owner 🗄️stdio` (9.5 min) at the end |
| WP-3 | STDIO-MEDIA-GEOMETRY-BIM | `STD/{🎞️gif,📷️png,📸️jpg,🖼️tiff,🪟️bmp,📼️avi,🎥️mp4,🎵️mp3,🔊️wav,🖊️dwg,🖋️dxf,💬️bcf,🗽️obj,🧱️ply,🔺️stl,☁️las,🌦️epw,🏗️ifc,📐️step}/**` (excluding `🪶️sqlite` dirs and glTF) | media follow-up verification (§2.5: 5-8 crate tests, 6+ parity cases, 6 inventories, wav oracle phase, avi oracle test); dwg set-snapshot 6/6 x2; dxf 7/7; bcf viewpoint 8/8 + `decode_bcf`; obj recipe; wav witness (with the sqlite peer); write `T/📓️w3-stdio-cases-report.md` and the media Session-2 section. Asks WP-2 for any shared-oracle/contract change |
| WP-4 | RESIDUALS-AND-VERIFY (non-norm, non-stdio) | leaf schemas/fixtures/outcome docs only under `S/{🔋️energy,📏️layout,🖍️draw,🀄️wfc,🌀️procedural,💠️lowpoly,🏗️fem,🏭️process}/**` plus `🧰️framework/…/🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder` and `🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state`; never tool/editor code | 6 layout `warn`->`warning` + `T/🧪️w3-codes-outcome-law.ts --with-leaves` 0; attach-local-folder widget; draw `drag-path-points` re-check; 4 energy + `set-state` + `set-test-count` identity decisions (coordinate with REPO-PATH-BUDGET); energy crate tests and case (§2.14); `semio_payload_law_*` sweep over the 102 crates (W1-D `🗑️generated/w1d/law_batch.sh` pattern) after B1/B2; finish W3-CODES report §9 and deferred reruns; remodel/fem/layout/wfc-bitmap lib tests listed as unverified; process stale catalog row (confirm after generate) |
| WP-5 | CENTRAL-GATES (last, coordinator-gated) | `📚️library/🔣️schema-catalog.json`, `📓️schema-catalog.md`, `🔣️taxonomy.json` (Edit tool only), `.vscode/launch*`, `🌎️hub/🧩️compositions/*/{🔣️.json,🛂️.descriptor.semio}` | after WP-1..4 and B1 finish: `schema generate`; `describe` regeneration (coordinator); launch.json (`test-mutation-leaf-identity`); resolve B3 (writer `project.json` `workspaceCommand`) then root `verify mutation-outcome-law` = 0; `test schema` slice for mutation-leaf paths (255) triaged, evidence classes (`catalog-stale`, `catalog-malformed`, `input-ui`, `payload-parity`) = 0; wav/json taxonomy verifies; W3-TAX package-move coverage + sealed-digest guard test; final `schema mutation-inputs` and `mutation-payloads` strict = 0 and a final census table |
| WP-6 | NORM-TS-TWINS | every `NORM/**/🟦️.ts` (leaf, aggregate, snapshot, diff twins) | 245 `schema-export-parser-missing` + 13 `schema-export-incomplete` (norm slice of `test schema`): generator-first like `T/🧪️w3-gltf-twins.ts` (`--check` = 0 drift), TS witness test over every committed `🦠️mutation` and snapshots (Ajv `semioSchemaAjvV1` + byte-equal re-encode), tsc strict over norm twins; `bun nx run` check target if present |

Ownership conflicts to avoid: WP-1 and WP-6 split by file kind (`🟦️.ts`); WP-2 and WP-3 split by artifact; WP-4 never touches stdio or norm; WP-5 touches only generated/hot shared files and runs when no other WP is active. The wfc, procedural, lowpoly, fem, draw and layout leaves added by the W3-T/W3-T2 tool lanes (14-25 uncatalogued, the 12:24 generation3d/shooting drift) belong to the tool-conversion WPs in `T/📓️resume-tools.md`: WP-4 only verifies their evidence classes and never edits tool/editor code; hand every finding in those leaves to the tool WP that owns the plugin.

## 6. Successor brief (one paragraph)

The evidence rule is on disk: every one of 3,066 mutation leaves has a committed wire witness that passes its leaf schema, 5,424/5,427 inputs have `x-semio-ui` (en/de), the outcome vocabulary is one table with a gate, norm paths fit 240 bytes, and all 15 norm Python oracles agree with their vectors. What is left is (a) verification that nobody could run since ~07:25 because of the sqlite rollout, the writer `project.json` routing crash and the REPO-PATH-BUDGET rename now in flight; (b) a handful of genuinely unfinished pieces: din4108 outcome vectors (1 failing test), en1992 `change-anchor-a-s` identity, 6 layout `warn` fixtures, os `attach-local-folder` widget, kit `set-snapshot` label, red stdio cases (xml 1/14, dxf 0/7, dwg 4/6, bcf-viewpoint 6/8), unrun media/pdf-subset/docx/semio/energy verification, and the missing or placeholder reports of W3-CODES, W3-STDIO-CASES(-2), norm-2/3 and media; (c) central chores (`schema generate`, descriptors, launch row, writer routing). Start WP-1 and WP-6 immediately, hold the stdio and non-norm WPs until REPO-PATH-BUDGET lands and a one-crate `cargo check` is green, run WP-5 alone at the end, and re-measure with the two lint commands in §3.1 (they are cheap: 12 s and 67 s) before and after every WP because the numbers move with concurrent renames.

## 7. Not verified by this audit

Any cargo/nx result, taxonomy reports (20 min per scope), `contract`/`parity`/`subject` phases, the bridge inventories, whether the 12:24 findings have converged (rename in flight), whether `describe`d descriptors match current schemas beyond the mtime comparison, and the claim that `schema generate` clears the 14 uncatalogued leaves (inferred from their annotations and fixtures).

## 8. Interrupted-edit table (summary)

| lane | file/step | evidence | judgement |
|---|---|---|---|
| norm-2 | din4108 outcome classes | test-six.log 07:59 failure, descriptors 09-26 vs 0 refusal vectors | half-done (repair, §2.7) |
| norm-2 | din4108 asset twins re-encoded 07:59:05 | 4 files same second | likely complete; confirm round trip |
| norm-3 | en1995 conversion/settle | last edit 07:55:30, 68 bundles == catalog == 135 oracle rows, 0 dangling `#[path]`, no debug module | complete, unverified |
| W3-CODES | report §9, 6 layout fixtures | file ends at §8, gate output | half-done (repair, §2.9) |
| W3-TAX | package-move coverage | last logs 08:07/08:13 only | unknown, verify |
| W3-STDIO-CASES | xml/dxf/dwg/bcf | summary.txt 08:14-08:16 red rows | unfinished, no report |
| W3-STDIO-CASES-2 | report §1/§5/§6 | placeholder text | unfinished report; subsets unrerun |
| media | follow-up scripts 07:26-07:50 | lints green, no Rust/Python run | complete on disk, unverified |
| energy | zones-window tests, oracle | parked 07:23 | unverified |
| W1-D, W2-R*, W2-S*, W2-W bim/doc/text/office/geometry, W3-GLTF, norm-1 | none | no mtimes after their last reports except peers | none |

## 9. Evidence files (`T/🗑️generated/resume-evidence/`)

`inputs.json`, `payloads.json`, `census-table.md`; `inputs-strict.txt`, `payloads-strict.txt` (11:35 findings); `inputs-strict-2.txt`, `payloads-strict-2.txt` (12:24); `outcome-law-rules.txt` (7 rules, 12:0x), `outcome-law.txt` and `outcome-law-root-retry.txt` (root crash, B3); `oracle-probe.py`, `oracle-probe-now.txt`, `probe-work/` (run output); `norm-path-budget-now.txt`, `path-budget-repo-now.txt`; `test-schema-all.json` and `test-schema-norm.txt` (adjacent baseline); `mtime-0700-0840.txt` (1,279 files), `mtime-plugins-after-0816.txt` (991 files), `debug-tag-scan.txt`.
