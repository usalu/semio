# 📓️ S5 gates census (measurement only, bun/python gates, no cargo)

Agent: S5-GATES-CENSUS. Date: 2026-10-04 (23:2x onward, times from `date`). Machine load 50–70 during the runs (a detached activation build holds the cargo lease). Nothing was fixed or edited besides this report and the raw outputs under `🗑️generated/s5-gates/`.
Paths: `T` = this ticket folder; `OUT` = `T/🗑️generated/s5-gates/`; `REPO` = repo root; `TESTMOD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.
Last-recorded numbers come from `📓️s4-gates-report.md`, `📓️s2-agnostic-report.md` (S4.1/S4.8/S4.12) and `📓️status.md`.

## 1. Gate table (all commands run from the repo root `/Users/ueli/Documents/semio`)

| # | Gate | Command | Exit | Headline | Last recorded | Delta |
| --- | --- | --- | ---: | --- | --- | --- |
| G1 | history-closure | `bun ./📜️script.ts verify history-closure` | 1 | 9 (bracket-verb 9, all other rules 0; self-test 34 cases pass) | bracket-verb 11 only (10-04 07:27-12:50) | -2 |
| G2 | mutation outcome law | `bun ./📜️script.ts verify mutation-outcome-law` | 1 | 8 breaches, all print chart diff | 11 at ~08:55, then green | +8 vs green |
| G3 | labels | `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-labels --json` | 0 | 0 findings (3024 labels) | 0 (3052 labels, 10-04 12:28) | 0 |
| G4a | payloads, repo-wide | `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-payloads --json` | 1 | 11 (gis 7, lowpoly 4) over 3072 leaves | 31 (10-04 13:04) | -20 |
| G4b | payloads, stdio | `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-payloads --json --under "✏️s/🔌️plugins/🗄️stdio"` | 0 | 0 findings over 1031 leaves | 0 / 1031 | 0 |
| G5a | fault notices, `history-editing` | `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema fault-notices --scope history-editing --json` | 1 | 434 (anonymous 358, framework 39, descriptor 13, missing 11, syntax 7, invalid 6) | 448 (10-04 ~20:30) | -14 |
| G5b | fault notices, all scopes | `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema fault-notices` | 1 | 3252 findings; 206 labelled of 1167 codes; 2272 anonymous | 3308; 135 of 1126; 2308 anonymous | -56 |
| G5c | raw tool-mismatch census | `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s4-gates-tool-mismatch.ts` | 0 | 33 raw codes / 84 sites / 8 owners | 45 / 96 / 14 | -12 / -12 / -6 |
| G6 | inputs (UI descriptors) | `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-inputs --json` | 1 | 255 (176 catalogue staleness, 79 real) | 261 (10-04 12:45) | -6 |
| G7 | editability | `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-editability --json` | 1 | 8 (cad 8; jack, rewriting, layout 0) over 3071 leaves | 23 (jack 8, rewriting 6, cad 8, layout 1; 10-04 13:01) | -15 |
| G8a | docstring emoji-unique, touched files | `bun ./📜️script.ts verify docstrings emoji-unique --files-from .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/s3-gates/ticket-files.txt` | 1 | 489 files / 15222 repeats (1914 files) | 488 / 15203 | +1 / +19 |
| G8b | docstring emoji-unique, F14 files | same with `--files-from …/🗑️generated/s4-gates/f14-files.txt` | 1 | 176 repeats in 4 of 7 files (tool-run 68, tool-machine 67, flow editor 21, stdio patch 20) | 177 (69, 67, 21, 20) | -1 |
| G9 | taxonomy, new module dirs (scope-limited) | `bun ./📜️script.ts verify taxonomy report --scopes-from <file with the 10 dirs>` | 0 | 10 scopes: 9 clean, `⏯️tool-run` 2 errors | no per-scope record | n/a |
| G10 | debug tags (extra) | `bun ./📜️script.ts verify debug-tags` | 1 | 708 lines / 277 files | 611 / 244 (10-04 03:55) | +97 / +33 |
| G11 | bun suites (13 files) | see G11 | 0 | all green: source-census 30/0, goal-gate 28/0, fault-notices-gate 3/0, time-travel conformance 20/0, tool-machine conformance 34/0, gesture-drive 8/0 x30, node-graph-row-ownership 2/0, outcome-law-gate 25/0, history-gates 43/0, payload-parity 43/0, manifest inputs 92/0, manifest fault-notices 5/0, text-splice 69/0 | same counts (history-gates 39/0 -> 43/0) | 0 (+4 tests) |

Run times (from `date`): G1 23:29-23:31, G2 23:31-23:36, G3 23:37, G4 23:37-23:40, G5 23:40-23:43, G6 23:44, G7 23:45-23:48, G8 23:49-23:50, tests 23:51-23:53, G9 23:53-23:55, G10 23:55-23:56. The tree changed under the runs (peers edit concurrently); a later run may differ.

Reading guide: 727 findings are open across the owner-routable gates (G1, G2, G4, G5, G6, G7, G9); 198 of them are staleness classes (central catalogue, stale hub descriptors) that clear without source edits, 529 are real. Biggest movers since 20:30: editability 23 -> 8, payloads 31 -> 11, fault notices 448 -> 434, raw tool-mismatch 45 -> 33. New regressions: outcome-law 8 (print chart diff, a peer's in-flight work) and lowpoly payload 4 (`meshState: null`).


## Per-gate findings

(sections are appended per gate in run order)

### G1. `verify history-closure` (run 23:29:40–23:31:09, exit 1)

Raw: `OUT/history-closure.txt`. Self-test: **passed, 34 cases**. Findings **9**, all rule `bracket-verb`: census `amend-emit=0 amend-last=0 coalesce-key=0 preview-contract=0 bracket-verb=9 host-snapshot-bracket=0 edit-literal=0 release-plain-commit=0 footprint-hand=0 footprint-default=0`.
Last recorded: bracket-verb 11 only (status 10-04 07:27 / 12:50; `footprint-default` 6 -> 0 by S4-PUZZLE, `coalesce-key` 14 -> 0 by S4-BUMP). Delta: **-2** (bracket-verb 11 -> 9), every other rule still 0.

Findings by owner tree (all in generated hub composition descriptors under `🌎️hub/🧩️compositions/<app>/🔣️.json`; none in hand-written source):

| Owner tree | Count | Findings |
| --- | ---: | --- |
| 💠️lowpoly (hub composition descriptor) | 5 | `🌎️hub/🧩️compositions/💠️lowpoly/🔣️.json:1882` bracket-verb `transformBegin`; `:1938` `transformEnd`; `:2047`, `:3512` `paintStrokeEnd`; `:5092` `paintStrokeBegin` |
| 🏗️fem (hub composition descriptor) | 2 | `🌎️hub/🧩️compositions/🏗️fem/🔣️.json:12468` `transformBegin`; `:12514` `transformEnd` |
| 🎪️demonstrator (hub composition descriptor) | 2 | `🌎️hub/🧩️compositions/🎪️demonstrator/🔣️.json:25911` `transformBegin`; `:25957` `transformEnd` |

Scoping artefact: all 9 are stale generated descriptors (the `describe` output still lists bracket verbs the sources no longer declare; status 10-04 03:53/07:18 already classed these "stale descriptors -> describe wave"). They clear with a re-describe of lowpoly, fem and demonstrator, not with source edits. Verified 23:3x: `grep -rl transformBegin|transformEnd|paintStroke{Begin,End}` over `✏️s/🔌️plugins/{💠️lowpoly,🏗️fem,🎪️demonstrator}` (rs/ts/tsx) has 0 hits; the verbs survive only in `🌎️hub/🧩️compositions/<app>/🔣️.json` (and lowpoly's `🛂️.descriptor.semio`).

### G2. `verify mutation-outcome-law` (run 23:31:39–23:36:39, exit 1, 5.0 min)

Raw: `OUT/mutation-outcome-law.txt`. **8 breaches**, all `mutation-migration/message-code`, all in ONE new tree: the print framework product's chart-diff leaf (peer ticket `UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O`, files mtime 23:12-23:13, i.e. still being written).
Last recorded: 11 breaches at 10-04 ~08:55 (jack `mutation.child-refused` x8 + stdio png/bmp/pptx), then "outcome law green" (status 10-04 ~08:20-12:xx; `test outcome-law-gate` 25/0). Delta: **+8 vs the last green state** (the earlier 11 are gone; these 8 are new).

| Owner tree | Count | Representative findings |
| --- | ---: | --- |
| `🧰️framework/🛍️products/📓️print` (chart diff / chart mutations) | 8 | `🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🦀️.rs:69` message-code: outcome code `print.chart.path` is not a `mutation.apply.<detail>` apply-rejection code; `…/🔀️diff/🦀️.rs:77` `print.chart.parent`; `…/🔀️diff/🦀️.rs:84,96` `print.chart.precondition`; `:94` `print.chart.index`; `:105` `print.chart.parent`; `:114` `print.chart.schema`; `🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs:42` `print.chart.path` at error "is not in the frozen outcome vocabulary" |

Not a scoping artefact: `MutationApplyError::new("print.chart.*", …)` and `MutationOutcome::error("print.chart.path", …)` use free-form app codes where the frozen vocabulary requires `mutation.apply.<detail>` (apply rejections) or one of the 9 framework outcome codes. Fix: rename the 6 distinct codes to `mutation.apply.{path,parent,precondition,index,schema}` (the diff file) and route the `🧬️mutations/🦀️.rs:42` error through the frozen vocabulary.

### G3. `schema mutation-labels` (run 23:37:25–23:37:43, exit 0)

Raw: `OUT/mutation-labels.txt`. **0 findings**; 35 owners, 3024 leaf labels = 2982 native + 42 forwarding. Last recorded: 0 findings, 3052 labels (3010 native + 42 forwarding), 10-04 12:28. Delta: **0 findings** (28 fewer native labels = leaves removed since, e.g. the §20.15 conversions). No owner groups (nothing to attribute).

### G4. `schema mutation-payloads` (repo-wide run 23:37:55–23:38:45, exit 1; stdio-scoped run 23:39:18–23:40:11, exit 0)

Raw: `OUT/mutation-payloads.txt`, `OUT/mutation-payloads-stdio.txt`. Repo-wide: **11 findings** over 3072 leaves (3072 witnessed), 3568 fixtures, 200 negatives, 14 invariants, 1526 rows, 33 owners. Stdio-scoped: **0 findings over 1031 leaves** (1031 witnessed, 635 fixtures, 1446 rows), identical to the last record (0 / 1031).
Last recorded repo-wide: 31 at 10-04 13:04 (S4.12: pptx 16, gisterrain 7, raster 4, mathematical 2, semio graph `set-snapshot` 2). Delta: **-20** (pptx 16, raster 4, math 2, semio graph 2 all gone) but **+4 new** (lowpoly), gisterrain 7 unchanged.

| Owner tree | Count | Representative findings |
| --- | ---: | --- |
| `✏️s/🔌️plugins/🌍️gis` (gisterrain `change-imported-features`) | 7 | schema `…/🏔️gisterrain/…/🧬️mutations/📥change-imported-features/🧬️schema/🔣️.json`: `opaque` at `imported-map.json#/definitions/map/properties/positions/items`, `…/routes/items`, `…/regions/items` and `#/definitions/intrinsic` ("an object schema declares no members"); fixture `…/🧫️fixtures/🧬️mutations/📥change-imported-features/📥️imports/🦠️mutation/🔣️.json`: `undescribed` `/newImportedMap/positions/0/{id,lat,lon}` |
| `✏️s/🔌️plugins/💠️lowpoly` (`create-object`, `create-mesh`) | 4 | fixtures `…/🧫️fixtures/🧬️mutations/🌱️create-object/⛵️inserts/🦠️mutation/🔣️.json` and `…/🕸️create-mesh/🕸️attaches/🦠️mutation/🔣️.json`: `unresolved` + `aggregate` "Unexpected null, expected schema in properties". Cause located: the leaf schemas carry a literal JSON `null` as a property schema, `🌱️create-object/🧬️schema/🔣️.json` `/properties/object/properties/meshState` and `🕸️create-mesh/🧬️schema/🔣️.json` `/properties/meshState` (mtime 18:27, the merge) |

Not scoping artefacts: gis is a real schema gap (4 object schemas without members, carried over from 13:04); lowpoly is a real invalid schema (null where a schema belongs). Fix: gis declares `properties` for `positions/routes/regions` items and `intrinsic` in `imported-map.json`; lowpoly replaces both `meshState: null` with the mesh-state schema ref.

### G5. Fault-notice gate: `schema fault-notices` (scoped run 23:40:22–23:40:38 exit 1; all-scope 23:42:3x exit 1; scoped census exit 0)

Raw: `OUT/fault-notices-history-editing.json` (diagnostics + census), `OUT/fault-notices-history-census.tsv`, `OUT/fault-notices-all.txt`, `OUT/fault-notices-owner-groups.md` (generated grouping with located lines), `OUT/tool-mismatch.txt`.
The diagnostics carry the file but not the line; lines below were located by grepping the literal/code in the file (first hit, "+N more" = further occurrences in the same file).

**Headline.**
- `--scope history-editing`: **434 findings** (last recorded 448 after triage, 10-04 ~20:30). Delta **-14**.
  Classes: faultAnonymous 358, faultNoticeFramework 39, faultNoticeDescriptor 13, faultNoticeMissing 11, faultNoticeSyntax 7, faultNoticeInvalid 6 (the last class is new vs the 448 residue: 6 trinity rewriting rows).
- all scopes: **3252 findings**, 206 labelled of 1167 guest fault codes, 154 declared notices, 2272 anonymous faults (last recorded 3308 findings, 135 labelled of 1126 codes, 2308 anonymous). Delta **-56 findings**, +71 labelled, +41 codes.
- Raw `*-tool-mismatch` census (`bun T/🧪️s4-gates-tool-mismatch.ts`, all-scope sites): **33 distinct raw codes at 84 sites across 8 owners** (last recorded 45 / 96 / 14). Delta **-12 / -12 / -6**. Remaining owners: stdio 21 codes at 72 sites (`stdio-example-tool-mismatch` alone is 52 sites, the other 20 are 1 each), puzzle 4 (`puzzle{2d,3d,5d}-command-tool-mismatch`, `puzzle2d-command-tool-unmapped`), block 3, and 1 each animate, demonstrator, cad, norm, sourcing. Converted since the record (0 raw codes now): wfc, trinity jack, process, writer, vcs, raster.

**Per owner (scoped, 434).** Counts first, then up to 3 representatives (`file:line` — rule: code or text).

- **SDK 🔌️plugin (framework): 109 {'faultNoticeFramework': 39, 'faultAnonymous': 70}**
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25890` — faultNoticeFramework: artifact-envelope.load-stale-handle (faultCode)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25859 (+2 more occurrences)` — faultNoticeFramework: artifact-envelope.stale-handle (faultCode)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25853 (+1 more occurrences)` — faultNoticeFramework: artifact-store.replacement-stale-handle (faultCode)
- **🌊️flow: 82 {'faultNoticeMissing': 2, 'faultNoticeSyntax': 3, 'faultAnonymous': 77}**
  - `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:30` — faultNoticeMissing: flow.add-widget.child-delta-invalid (faultCode)
  - `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2296` — faultNoticeMissing: flow.retained.legacy-dispatch (faultCode)
  - `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1994` — faultNoticeSyntax: flow.child-projection (faultCode)
- **🎬️sequence: 81 {'faultAnonymous': 78, 'faultNoticeMissing': 2, 'faultNoticeSyntax': 1}**
  - `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2319` — faultNoticeMissing: sequence.retained.config-command (faultCode)
  - `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2534` — faultNoticeMissing: sequence.retained.example-command (faultCode)
  - `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3057` — faultNoticeSyntax: sequence.child-projection (faultCode)
- **🗄️stdio: 73 {'faultNoticeSyntax': 3, 'faultAnonymous': 69, 'faultNoticeMissing': 1}**
  - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio/🦀️.rs:472` — faultNoticeMissing: stdio.wav.audio-tool-mismatch (faultHelper)
  - `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:1231` — faultNoticeSyntax: bounded-native-edit.tool-mismatch (faultHelper)
  - `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:1534 (+1 more occurrences)` — faultNoticeSyntax: snapshot-edit.command-mismatch (faultHelper)
- **💡️reasoning: 16 {'faultAnonymous': 16}**
  - `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:718` — faultAnonymous: Fault::from("wires-canvas-window-context-required") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:627` — faultAnonymous: Fault::from("wires-command-payload-too-large") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:399 (+1 more occurrences)` — faultAnonymous: Fault::from("wires-drag-camera-invalid") carries no code — name the refusal and declare its notice
- **hub descriptors: 13 {'faultNoticeDescriptor': 13}**
  - `🌎️hub/🧩️compositions/🕸️dag/🔣️.json` (no line, generated) — faultNoticeDescriptor: describe owed — unpublished ["dag.layout-run.layered-layout","dag.layout-run.layered-load","dag.layout-run.layered-result","dag.layout-run.start"], no longer declared []
  - `🌎️hub/🧩️compositions/🖍️draw/🔣️.json` (no line, generated) — faultNoticeDescriptor: describe owed — unpublished ["drawing.gesture.closing","drawing.gesture.command","drawing.gesture.owner","drawing.gesture.point-capacity","drawing.gesture.query-capacity","drawing.gesture.query-output-capacity","drawing.gesture.query-owner","drawing.gesture.retained-route","drawing.gesture.saturated"], no longer declared []
  - `🌎️hub/🧩️compositions/🏗️fem/🔣️.json` (no line, generated) — faultNoticeDescriptor: describe owed — unpublished ["fem.canvas.window-kind","fem.canvas.window-required","fem.gumball-flag.window-context-required","fem.gumball-flag.window-kind","fem.gumball-flag.window-required","fem.gumball-flag.window-stale","fem.gumball.phase-unknown","fem.gumball.transient-context-required"], no longer declared []
- **🧩️puzzle: 11 {'faultAnonymous': 11}**
  - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:5322` — faultAnonymous: Fault::from("puzzle2d import-media admits a decoded media value, never a wire payload") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:5325` — faultAnonymous: Fault::from("puzzle2d import-media requires media input") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:5196` — faultAnonymous: Fault::from("puzzle2d-command-tool-mismatch") carries no code — name the refusal and declare its notice
- **🔱️trinity: 6 {'faultNoticeInvalid': 6}**
  - `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (line not located: variable text or declared-table row) — faultNoticeInvalid: syntax rewriting.child-projection in TrinityRewritingPlayApp
  - `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:559` — faultNoticeInvalid: syntax rewriting.child-refused in TrinityRewritingPlayApp
  - `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (line not located: variable text or declared-table row) — faultNoticeInvalid: syntax rewriting.graph-window-required in TrinityRewritingPlayApp
- **🌀️procedural: 4 {'faultAnonymous': 4}**
  - `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1552` — faultAnonymous: Fault::from("generation2d import-media admits a decoded media value, never a wire payload") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1555` — faultAnonymous: Fault::from("generation2d import-media requires media input") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:195 (+4 more occurrences)` — faultAnonymous: Fault::from("generation3d-eval-session-closing") carries no code — name the refusal and declare its notice
- **🎞️animate: 4 {'faultAnonymous': 4}**
  - `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:997` — faultAnonymous: Fault::from("animate-presentation-command-payload-too-large") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:994` — faultAnonymous: Fault::from("animate-presentation-command-tool-mismatch") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1036` — faultAnonymous: Fault::from("presentation import-media admits a decoded media value, never a wire payload") carries no code — name the refusal and declare its notice
- **💠️lowpoly: 4 {'faultAnonymous': 4}**
  - `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1021` — faultAnonymous: Fault::from("lowpoly-retained-checkpoint-tool") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:983 (+1 more occurrences)` — faultAnonymous: Fault::from("lowpoly-retained-command-capacity") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🦀️.rs` (line not located: variable text or declared-table row) — faultAnonymous: Fault::from(text) carries no code — name the refusal and declare its notice
- **🖨️raster: 4 {'faultAnonymous': 4}**
  - `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1340` — faultAnonymous: Fault::from("raster import-media admits a decoded media value, never a wire payload") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1343` — faultAnonymous: Fault::from("raster import-media requires media input") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️paint-stroke/🦀️.rs:427` — faultAnonymous: Fault::from("raster-paint-stroke-route-mismatch") carries no code — name the refusal and declare its notice
- **🀄️wfc: 3 {'faultAnonymous': 2, 'faultNoticeMissing': 1}**
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:863` — faultNoticeMissing: wfc3d.node-graph.row (faultCode)
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:988 (+1 more occurrences)` — faultAnonymous: Fault::from("wfc-bitmap-command-unmapped") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:682` — faultAnonymous: Fault::from("wfc.grid3d.retained.extent") carries no code — name the refusal and declare its notice
- **🏗️fem: 3 {'faultAnonymous': 3}**
  - `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:278 (+1 more occurrences)` — faultAnonymous: Fault::from("fem2d.gumball-flag.window-context-required") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:863` — faultAnonymous: Fault::from("fem3d-command-payload-too-large") carries no code — name the refusal and declare its notice
- **🧱️block: 3 {'faultAnonymous': 3}**
  - `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:481` — faultAnonymous: Fault::from("block2d-retained-command-tool-mismatch") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:822` — faultAnonymous: Fault::from("block3d-retained-command-tool-mismatch") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:448` — faultAnonymous: Fault::from("block5d-retained-command-tool-mismatch") carries no code — name the refusal and declare its notice
- **➗️mathematical: 2 {'faultAnonymous': 2}**
  - `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1131` — faultAnonymous: Fault::from("equation-command-capacity") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:448` — faultAnonymous: Fault::from("equation-work-replay-drift") carries no code — name the refusal and declare its notice
- **🎪️demonstrator: 2 {'faultAnonymous': 2}**
  - `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:395 (+1 more occurrences)` — faultAnonymous: Fault::from("playground-command-payload-too-large") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:392` — faultAnonymous: Fault::from("playground-command-tool-mismatch") carries no code — name the refusal and declare its notice
- **📏️layout: 2 {'faultAnonymous': 2}**
  - `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1384` — faultAnonymous: Fault::from("layout import-media admits a decoded media value, never a wire payload") carries no code — name the refusal and declare its notice
  - `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1387` — faultAnonymous: Fault::from("layout import-media requires media input") carries no code — name the refusal and declare its notice
- **🕸️dag: 2 {'faultNoticeMissing': 2}**
  - `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:59` — faultNoticeMissing: dag.node-graph-edit.malformed (faultCode)
  - `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:100` — faultNoticeMissing: dag.node-graph-edit.unsupported (faultCode)
- **🌍️gis: 1 {'faultAnonymous': 1}**
  - `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:531 (+1 more occurrences)` — faultAnonymous: Fault::from("gis3d-command-payload-too-large") carries no code — name the refusal and declare its notice
- **🎥️shooting: 1 {'faultNoticeMissing': 1}**
  - `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:709` — faultNoticeMissing: shooting.retained.extent (faultCode)
- **🏛️architect: 1 {'faultAnonymous': 1}**
  - `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1629` — faultAnonymous: Fault::from("architect-window-command-mismatch-or-capacity") carries no code — name the refusal and declare its notice
- **🏭️process: 1 {'faultAnonymous': 1}**
  - `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1431` — faultAnonymous: Fault::from("process3d-retained-work-extent-overflow") carries no code — name the refusal and declare its notice
- **📐️cad: 1 {'faultAnonymous': 1}**
  - `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2093` — faultAnonymous: Fault::from("cad-retained-command-tool-mismatch") carries no code — name the refusal and declare its notice
- **📕️norm: 1 {'faultAnonymous': 1}**
  - `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs:2245` — faultAnonymous: Fault::from("norm-command-tool-mismatch") carries no code — name the refusal and declare its notice
- **📸️remodel: 1 {'faultAnonymous': 1}**
  - `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:907` — faultAnonymous: Fault::from("remodeling.reconstruction.provisional-count") carries no code — name the refusal and declare its notice
- **🔋️energy: 1 {'faultNoticeMissing': 1}**
  - `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2188` — faultNoticeMissing: energy.model.retained.extent (faultCode)
- **🖍️draw: 1 {'faultNoticeMissing': 1}**
  - `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1710` — faultNoticeMissing: drawing.canvas.window-required (faultCode)
- **🪵️sourcing: 1 {'faultAnonymous': 1}**
  - `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:983` — faultAnonymous: Fault::from("sourcing-curation-command-tool-mismatch") carries no code — name the refusal and declare its notice

**Verifier-scoping notes (what is and is not a real defect).**
- `hub descriptors` 13 (`faultNoticeDescriptor`, "describe owed"): generated `🌎️hub/🧩️compositions/<app>/🔣️.json` lists codes the sources no longer declare or lacks new ones. Artefact of stale generated output, clears with the describe wave; no source edit. Apps: dag, draw, fem, note, playbook, procedural, process, raster, reasoning, remodel, trinity, vcs, wfc.
- SDK `🔌️plugin` 70 `plugin.internal (faultHelper)`: the `plugin_sdk_fault(...)` helper (364 call sites in `🦀️.rs`, 9 in `⏯️tool-run`, 4 in `⏪️time-travel`) is a fixed-code catch-all; the gate counts each call that sits in an in-scope flow fn. Real, not an artefact: the S4 staging (`sdk-naming.patch`, 49 framework rows) has NOT landed (grep of the SDK for `timeTravel.snapshot-retirement`, `toolRun.publication-handoff`, `transaction.group-history-dialect` = 0 hits), so these refusals still carry no code. The 39 `faultNoticeFramework` findings are SDK-raised codes (`timeTravel.*`, `toolRun.*`, `transaction.*`, `interactive-job.*`, `toolTransaction.shape`, `artifact-envelope.*`, `window-config.*`, `ledger-not-replayable`) that have no row in `FRAMEWORK_FAULT_NOTICE_LABELS`; the staged rows cover 32 of them. `ledger-not-replayable` still lives as a single-segment code at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:38500` (rename to `history.ledger-not-replayable` owed).
- `faultNoticeSyntax` (7) / `faultNoticeInvalid` (6): a notice code needs at least three kebab segments; `flow.child-projection`, `flow.delete-selection-empty`, `flow.widget-id-unavailable`, `sequence.child-projection` and the six trinity rewriting declared rows (`rewriting.{child-refused,child-projection,window-required,graph-window-required,lod,viewport}`) have two. Real renames. The stdio three (`bounded-native-edit.tool-mismatch`, `snapshot-edit.tool-mismatch`, `snapshot-edit.command-mismatch`) are helper-built codes in `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:1231,1534`.
- Anonymous `import-media admits a decoded media value, never a wire payload` / `requires media input` (puzzle2d 5322/5325, generation2d 1552/1555, raster 1340/1343, layout 1384/1387, animate 1036): in scope because they sit in `build_reserved_tool_job`, a tool-job builder fn (read at `🧩️puzzle/…/◻️2d/…/✏️editor/🦀️.rs:5309-5326`). Real refusals of a tool flow (the same two messages in five plugins), so one shared framework code/notice pair would clear 10 findings.
- flow (77) and sequence (78) anonymous faults are real refusals of their retained steps (M6 decision); the sequence/flow findings that read `Fault::from(text)` (variable text, no literal) are counted as anonymous too, their exact line is not reported by the gate.


### G6. `schema mutation-inputs` (run 23:44:28–23:44:57, exit 1; the "UI descriptors / x-semio-ui" input gate)

Raw: `OUT/mutation-inputs.txt` (+ `.byowner.json` grouping). **255 findings**: malformed 102, leafUncatalogued 74, labelMissing 66, optionLabelMissing 7, refUnresolved 3, uiInvalid 2, widgetIncompatible 1.
Last recorded: 261 at 10-04 12:45 (S4.12; catalogue staleness 167 of them). Delta **-6**.

**Scoping artefact, 176 of 255 (69 %): the central leaf catalogue is stale.**
- `malformed` 102 = `"<path> is not JSON"`: the catalogued leaf paths no longer exist. Checked: 0 of the 102 files exist on disk (§20.15 conversions, stdio renames).
- `leafUncatalogued` 74 = leaf schemas on disk whose `$id` names no catalogued mutation-leaf scope.
- Both classes clear with the central `schema generate` (coordinator action, S4.14 item 1), not with plugin edits. Per owner: stdio malformed 20 + uncatalogued 61; dag malformed 17; trinity malformed 13; reasoning 12; flow 10; energy 8 + 8; playbook 8; sequence 8; imperative 4; mathematical 1 + 2; raster 3 uncatalogued; fem 1.

**Real findings, 79 (plugin-side UI metadata): labelMissing 66, optionLabelMissing 7, refUnresolved 3, uiInvalid 2, widgetIncompatible 1.**

| Owner tree | Total (all classes) | Real | Representative findings |
| --- | ---: | ---: | --- |
| `✏️s/🔌️plugins/🔱️trinity` (rewriting typed `workingGraph`) | 67 | 54 | `🔱️trinity/🗿️artifacts/♻️rewriting/…/🧬️mutations/🖼️edit-before-fixture/🧬️schema/🔣️.json` labelMissing `/newWorkingGraph`, `/newWorkingGraph/schema`; optionLabelMissing `/newWorkingGraph/manifest/{nodeKinds,edgeKinds}/-/properties/-/kind`; `…/♻️rewriting/…/🧬️mutations/🔧️change-parameter/🧬️schema/🔣️.json` refUnresolved `/newValue` (no document with `$id …/framework/graph/manifest/property-value.json`), same for `…/👉️edit-rhs/…` `/newRhs/set/-/value` (3 refUnresolved in all) |
| `✏️s/🔌️plugins/🗄️stdio` (semio graph subset) | 97 | 16 | `…/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔌add-node-port/🧬️schema/🔣️.json` labelMissing `/port/category`; `…/🌉️create-edge/…` labelMissing `/source_port`; `…/➖remove-node-property/…` widgetIncompatible `/node_id` (a reference is a string or integer id) and uiInvalid `/key` (a target names its ref.kind); optionLabelMissing `/ports/-/kind`, `/snapshot/nodes/-/ports/-/kind` |
| `✏️s/🔌️plugins/🌍️gis` (gisterrain) | 4 | 4 | `…/🏔️gisterrain/…/🧬️mutations/📥change-imported-features/🧬️schema/🔣️.json` labelMissing `/newImportedMap` (+3) |
| `✏️s/🔌️plugins/📏️layout` | 3 | 3 | `…/📏️layout/…/🧬️mutations/🧾change-data-fields/🧬️schema/🔣️.json` uiInvalid `/newFields` ("widget is not a declared widget"); labelMissing `/newFields/entries` (+1) |
| `✏️s/🔌️plugins/💠️lowpoly` | 2 | 2 | `…/💠️lowpoly/…/🧬️mutations/🕸️create-mesh/🧬️schema/🔣️.json` labelMissing `/meshState` (the same property carries the literal `null` schema seen in G4) |

The other owners (dag, energy, reasoning, flow, playbook, sequence, imperative, mathematical, raster, fem) have only catalogue-staleness findings.


### G7. `schema mutation-editability` (run 23:45:45–23:48:24, exit 1, 2.7 min)

Raw: `OUT/mutation-editability.txt` (+ `.byowner.json`). **8 findings**, all `parentLeafReadsChild`, all cad. Census: 38 owners, 219 aggregates, 3071 leaves (3067 editable, 4 foreign, 140 inert), 111 hand-written aggregates.
Last recorded: **23** at 10-04 13:01 (jack 8, rewriting 6, cad 8, layout 1; S4.12). Delta **-15**: jack 8 -> 0, rewriting 6 -> 0, layout 1 -> 0 (the "fixed later" claims hold), cad 8 -> 8 unchanged.

| Owner tree | Count | Findings |
| --- | ---: | --- |
| `✏️s/🔌️plugins/📐️cad` (`📐️cad/…/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>`) | 8 | parent-lane leaves of the composed parent `CadSnapshot` that read the owned child's content: `🆕create-object`, `❌delete-object`, `🚚move-objects`, `🌀rotate-objects`, `⚖️scale-objects` through `cad_pane_local_scene` (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs:169`); `✋️drag-selection`, `🔄️rotate-selection`, `🔍️scale-selection` through `cad_selection_inverse_objects` (`…/🧬️mutations/🦀️.rs:256`). Rule text: "composed content is edited only on the child lane (design §20.15)" |

Not a scoping artefact: these are real unconverted §20.15 parent leaves (the same conversion that cleared dag/wires/mathematical/jack/rewriting). Fix: move the five object leaves and three selection leaves to child-lane (`workingGraph`-style) leaves, the same pattern rewriting used.


### G8. `verify docstrings emoji-unique` over the ticket-touched files (run 23:49:05–23:49:22 exit 1; F14 list run exit 1; refreshed-list run 23:50:2x exit 1)

Raw: `OUT/docstrings-emoji-unique-touched.txt`, `OUT/docstrings-emoji-unique-f14.txt`, `OUT/docstrings-emoji-unique-touched-refreshed.txt`, list `OUT/ticket-files-refreshed.txt`.
Commands use the same file list as the last record (`🗑️generated/s3-gates/ticket-files.txt`, 1914 Rust/TS sources; read only) so the numbers compare.
- Touched list (1914 files): **489 files reuse an emoji, 15222 repeats**. Last recorded 488 files / 15203 repeats (S4 M7). Delta **+1 file, +19 repeats**. The gate prints only the top 200 rows (14588 of the repeats); grouping below is over those 200.
- Refreshed list (re-derived from the 111 ticket markdown files today by `🧪️s3-gates-ticket-files.py`, 2259 paths, 1963 Rust/TS): **518 files, 15698 repeats** (72 paths added, 5 dropped). The added files that reuse emojis are mostly new work: procedural generation3d editor 62, wgpu `🖌️paint` 50, cad editor 41, ui `🧩️component` contract 40, process3d editor 32, procedural generation2d editor 27.
- Whole tree (acceptance record, last recorded 41951 repeats in 7744 of 34398 sources) was NOT re-run (`verify docstrings emoji-unique` without a list publishes the repo-wide record and walks every source).
- F14 named files (7 files): **176 repeats in 4 files**. `⏯️tool-run/🦀️.rs` **68** (was 69), `🛠️tool-machine/🦀️.rs` **67** (67), flow editor `🌊️flow/…/✳️any/✏️editor/🦀️.rs` **21** (21), stdio `🩹️patch/🦀️.rs` **20** (20). Playbook root, wires root and flow root stay at 0.

By owner tree (top-200 rows, repeats | reusing files | worst file):

| Owner tree | Repeats | Files | Worst file (repeats) |
| --- | ---: | ---: | --- |
| framework `💻️os/…/📺️renderer` (wgpu Shell, React hosts) | 3122 | 31 | `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (1155) |
| framework `💻️os/…/🔌️plugin` (SDK) | 2381 | 13 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (1673); `⏯️tool-run/🦀️.rs` (68) |
| framework `💻️os/…/🏪️store` | 1171 | 5 | `🏪️store/🦀️.rs` (842) |
| framework `🛂️manifest` | 616 | 3 | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (510) |
| repo `📚️library` | 613 | 4 | `📚️library/🔍️discovery/🟦️.ts` (395) |
| framework `♾️infinite` | 611 | 8 | `♾️infinite/🌍️world/🦀️.rs` (250) |
| framework `🖱️ui` | 577 | 13 | `🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` (92) |
| repo root `📜️script.ts` | 427 | 1 | `📜️script.ts` (427) |
| repo `🧪️test` | 385 | 2 | `🧪️test/🟦️.ts` (337) |
| framework `🎠️kernel` | 322 | 2 | `🎠️kernel/🟦️.ts` (210) |
| plugin 🧩️puzzle | 311 | 5 | `🧊️3d/…/✏️editor/🦀️.rs` (141) |
| plugin 🗄️stdio | 220 | 13 | `🔮️oracles/📃️document/🦀️.rs` (40) |

Not a verifier artefact (the rule is AGENTS.md "start all docstrings with a unique … emoji", held per file), but a debt class, not a runtime defect: the repeats are concentrated in hot shared files (the two largest files, the SDK `🔌️plugin/🦀️.rs` 1673 and the wgpu Shell 1155, hold 19 % of the top-200 repeats), where hand editing needs one owner per file. Listed for completeness; it does not block the contract gates above.


### G9. `verify taxonomy report` over the ticket's new framework module dirs (scope-limited; run 23:53:53–23:54:32 and 23:54:43–23:55:30, exit 0 both: report mode never fails)

Raw: `OUT/taxonomy-scoped.txt`, `OUT/taxonomy-scoped-renderer.txt`, scope lists `OUT/taxonomy-scopes.txt`, `OUT/taxonomy-scopes-renderer.txt`. Used the multi-scope route `--scopes-from <file>` (one repository capture for all scopes; S4 M3 proved each multi verdict equals the single `--scope` verdict, 4/4 SAME) instead of ten separate `--scope` runs. The repo-wide taxonomy report was not run.
- 10 scopes: **9 clean, 1 dirty (2 errors, 0 warnings)**.
- Clean (0 errors, 0 warnings): `🧰️framework/🔨️modules/⏪️time-travel`, `🧰️framework/🔨️modules/🛠️tool-machine`, `…/💻️os/🔨️modules/🔌️plugin/⏯️tool-run`, `…/🔌️plugin/⏪️time-travel`, `…/🔌️plugin/🛠️tool-machine`, `…/🔌️plugin/🧫️fixtures/⏯️tool-run`, and the three renderer dirs (`🛠️ShellHelpers/⏪️time-travel`, `🐚️Shell/🧪️tests/🧪️wgpu-time-travel`, `🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel`).
- Last recorded: no per-scope record for these dirs (the S4 M2 first taxonomy chunk, 10-04 04:12-04:28, listed `reference-preimage-unreadable` 2 for the renderer's wgpu time-travel dirs, "files changed during the run"; now clean). Delta: wgpu time-travel preimage errors 2 -> 0.

| Owner tree | Count | Findings |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/⏯️tool-run` (framework module) | 2 | `🧰️framework/🔨️modules/⏯️tool-run/🧪️tests/🧩️conformance` directory-kind-unresolved ("Directory has no registered semantic kind"); `🧰️framework/🔨️modules/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy` directory-kind-unresolved |

Not a scoping artefact (the mutation-case pairing effect of S4 M4 does not apply to test-suite directories): the sibling modules name the same suites `🧪️tests/🧪️conformance` (time-travel, tool-machine) and `🧪️tests/🧪️<law-name>` (tool-machine `🧪️gesture-drive-law`, `🧪️history-closure-policy`). Fix: rename `🧩️conformance` -> `🧪️conformance` and `🔬️interactivity-tool-run-policy` -> `🧪️interactivity-tool-run-policy` (with their references). I did not read the registered-kind table, so the rename target is inferred from the siblings.

### G10. `verify debug-tags` (run 23:55:46–23:56:45, exit 1) — extra, not in the requested list

Raw: `OUT/debug-tags.txt`. **708 lines in 277 tracked sources** carry `[DEBUG] `. Last recorded 611 lines / 244 files (S4 M2, 10-04 03:55). Delta **+97 lines, +33 files**. The gate prints only 200 rows (sorted by path, so plugin trees first); the owner split below covers those 200 only: plugin 🖍️draw 87 rows in 24 files (worst `🖍️drawing/…/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs` 13), 🌊️flow 34 in 3 (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs` 22), 🌀️procedural 33 in 9, 💡️reasoning 25 in 2 (`…/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs` 18), 🗄️stdio 10 in 6, 🎪️demonstrator 4, wfc 2, gis 2, trinity 2, sequence 1. Almost all sit in unit-test files. Real debt (temporary logs, AGENTS.md rule), not a verifier artefact; the framework/os/repo rows are beyond the 200-row cap and unattributed here. The gate writes an acceptance record only when `SEMIO_ACCEPTANCE_RESULT` is set (it was not), so the run changed nothing.

### G11. bun test suites (all run from the repo root with `./`-prefixed paths, one at a time)

Raw: `OUT/test-<name>.txt`.

| Suite | Command (repo root) | Result | Last recorded |
| --- | --- | --- | --- |
| source census | `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧪️tests/🧮️source-census/🟦️.ts` | 30 pass / 0 fail (46 expects) | 30/0 |
| goal gate | `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧪️tests/🎯️goal-gate/🟦️.ts` | 28 pass / 0 fail | 28/0 |
| fault-notices gate | `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` | 3 pass / 0 fail | 3/0 |
| time-travel conformance | `bun test ./🧰️framework/🔨️modules/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts` | 20 pass / 0 fail | 20/0 |
| tool-machine conformance | `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` | 34 pass / 0 fail | 34/0 |
| gesture-drive corpus | `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️gesture-drive-law/🟦️.ts` | 8 pass / 0 fail; **30 sequential runs: 30 x (8/0)** (`OUT/test-gesture-drive-law-x30.txt`; `--rerun-each 30` was not used because bun still reported 8 total) | 8/0 x30 |
| node-graph row ownership | `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️node-graph-row-ownership/🟦️.ts` | 2 pass / 0 fail | 2/0 |
| outcome-law gate | `bun test ./🧪️tests/🧪️outcome-law-gate/🟦️.ts` | 25 pass / 0 fail | 25/0 |
| mutation history gates | `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts` | 43 pass / 0 fail | 39/0 (S4.1) |
| mutation payload parity | `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-payload-parity/🟦️.ts` | 43 pass / 0 fail | 43/0 |
| manifest mutation inputs | `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts` | 92 pass / 0 fail | 92/0 |
| manifest fault notices | `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` | 5 pass / 0 fail | 5/0 |
| ui text splice | `bun test ./🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.test.ts` | 69 pass / 0 fail | 69/0 |

Every self-test suite is green and unchanged (mutation-history-gates gained 4 tests since 39). The gates above that fail do so on the tree's content, not on their own implementation.


## Routing: open findings per owner tree

Columns: **All** = every finding of the gates G1-G2, G4-G7, G9 above (fault notices counted in the `history-editing` scope; docstring and `[DEBUG]` debt of G8/G10 excluded because they are not owner-split to the file everywhere); **Real** = All minus the two verifier/staleness classes (stale generated hub descriptors, stale central catalogue `malformed`/`leafUncatalogued`). Sorted by Real.

| Owner tree | All | Real | Breakdown | Single most valuable fix |
| --- | ---: | ---: | --- | --- |
| framework `💻️os/…/🔌️plugin` (SDK) | 109 | 109 | fault notices 70 anonymous `plugin.internal` + 39 framework codes | Land the staged S4 package (`🗑️generated/s4-gates/stage-r45/`: 49 en/de framework rows + schema pattern + `sdk-naming.patch`); clears about 56 of 109. The other 53 are the S4-LOAD document-load refusals (47 anonymous + 5 coded) and the `ledger-not-replayable` rename |
| 🗄️stdio | 170 | 89 | fault notices 73; inputs 16 real + 81 catalogue | Switch every raw `*-tool-mismatch` (21 codes, 72 sites; `stdio-example-tool-mismatch` alone is 52 subset editors) to `app.command.tool-mismatch`; clears about 70 of the 73 fault notices. Then the semio-graph input labels (16 inputs) |
| 🌊️flow | 92 | 82 | fault notices 77 anonymous + 3 syntax + 2 missing; 10 catalogue | Name the 77 `flow-retained-*` refusals in one `fault_notices()` table (en/de) and rename the 3 two-segment codes (`flow.child-projection`, `flow.delete-selection-empty`, `flow.widget-id-unavailable`) |
| 🎬️sequence | 89 | 81 | fault notices 78 anonymous + 2 missing + 1 syntax; 8 catalogue | Same as flow for the editor's 78 anonymous retained refusals (+ `sequence.child-projection` rename, `sequence.retained.{config,example}-command` rows) |
| 🔱️trinity | 74 | 60 | inputs 54 real + 13 catalogue; fault notices 6 invalid; 1 descriptor | Give the typed `newWorkingGraph` input of rewriting `edit-before-fixture` its `x-semio-ui` labels (46 labelMissing + 5 optionLabelMissing) and publish `framework/graph/manifest/property-value.json` (3 refUnresolved); then rename the 6 two-segment declared rewriting codes |
| 💡️reasoning (wires) | 29 | 16 | fault notices 16 anonymous; 12 catalogue; 1 descriptor | Name the 16 `wires-*` anonymous refusals (`wires-canvas-window-context-required`, `wires-command-payload-too-large`, `wires-drag-camera-invalid`, ...) |
| 🌍️gis | 12 | 12 | payloads 7 + inputs 4 + fault notice 1 | Declare `properties` for `imported-map.json` `positions/routes/regions` items and `#/definitions/intrinsic` (clears the 7 payload findings); label `/newImportedMap` (4) |
| 🧩️puzzle | 11 | 11 | fault notices 11 anonymous | Convert the four raw `puzzle{2d,3d,5d}-command-tool-mismatch` / `-tool-unmapped` codes to `app.command.tool-mismatch`, name the two `import-media` refusals, name the rest |
| 💠️lowpoly | 15 | 10 | payloads 4 + inputs 2 + fault notices 4; 5 stale closure descriptors | Replace the literal `null` property schema `meshState` in `create-object` and `create-mesh` (clears 4 payload + 2 label findings); re-describe lowpoly for the 5 bracket-verb rows |
| 📐️cad | 9 | 9 | editability 8 + fault notice 1 | Convert the 8 parent-lane leaves (`create/delete/move/rotate/scale-objects`, `drag/rotate/scale-selection`) to child-lane leaves (§20.15) |
| framework `📓️print` (chart diff) | 8 | 8 | outcome law 8 | Rename the 6 `print.chart.*` outcome codes to `mutation.apply.<detail>` and route `🧬️mutations/🦀️.rs:42` through the frozen vocabulary |
| 📏️layout | 5 | 5 | inputs 3 + fault notices 2 | Declare the widget of `change-data-fields` `/newFields` and label `/newFields/entries` (3 input findings) |
| 🖨️raster | 8 | 4 | fault notices 4; 3 catalogue; 1 descriptor | Name the 4 anonymous refusals (`raster import-media …` x2, `raster-paint-stroke-route-mismatch`, ...) |
| 🌀️procedural | 5 | 4 | fault notices 4; 1 descriptor | Name the 4 `generation2d/3d` anonymous refusals (`generation3d-eval-session-closing` is 5 occurrences in one file) |
| 🎞️animate | 4 | 4 | fault notices 4 | Name the 4 `animate-presentation-*` / `presentation import-media` refusals |
| 🏗️fem | 7 | 3 | fault notices 3; 2 stale closure descriptors; 1 catalogue; 1 descriptor | Name the 3 `fem2d/fem3d` gumball/payload refusals |
| 🀄️wfc | 4 | 3 | fault notices 3; 1 descriptor | Add `wfc3d.node-graph.row` and name the 2 `wfc-bitmap-*` / `wfc.grid3d.retained.extent` refusals |
| 🧱️block | 3 | 3 | fault notices 3 | Convert `block{2d,3d,5d}-retained-command-tool-mismatch` to `app.command.tool-mismatch` |
| framework `⏯️tool-run` | 2 | 2 | taxonomy 2 | Rename `🧩️conformance` and `🔬️interactivity-tool-run-policy` under `🧪️tests` to the `🧪️<name>` form |
| 🎪️demonstrator | 4 | 2 | fault notices 2; 2 stale closure descriptors | Convert `playground-command-{tool-mismatch,payload-too-large}` |
| ➗️mathematical | 5 | 2 | fault notices 2; 3 catalogue | Name `equation-command-capacity`, `equation-work-replay-drift` |
| 🕸️dag | 20 | 2 | fault notices 2 missing; 17 catalogue; 1 descriptor | Add `dag.node-graph-edit.{malformed,unsupported}` rows |
| others (1 real each) | | 1 each | energy `energy.model.retained.extent`, shooting `shooting.retained.extent`, draw `drawing.canvas.window-required`, architect, process, remodel, norm, sourcing | Add the single missing row / name the single refusal |
| central catalogue (coordinator) | 176 | 0 | inputs `malformed` 102 + `leafUncatalogued` 74 | Run the central `schema generate`; clears 176 of the 255 input findings |
| describe wave (coordinator) | 22 | 0 | 13 fault-notice descriptors + 9 bracket-verb descriptors | Re-describe dag, draw, fem, note, playbook, procedural, process, raster, reasoning, remodel, trinity, vcs, wfc (notices) and lowpoly, fem, demonstrator (bracket verbs) |

Totals: G1 9 + G2 8 + G4 11 + G5 434 + G6 255 + G7 8 + G9 2 = **727 open findings** across the owner-routable gates, of which 176 (catalogue) + 22 (stale descriptors) = 198 are staleness classes and **529 are real**. Not in this total: G8 docstring debt (15222 repeats in 489 touched files), G10 `[DEBUG]` lines (708 in 277 files).

## Gates that could not be run, or were not run

No gate failed to start; every run in G1-G11 produced output. Not run:
- `bun ./📜️script.ts test multi-scope-verification` (library, taxonomy multi-scope law): not attempted. S4 M4 recorded about 21 min at load 70 (budget `exhaustive`), above the 10-minute cap. Record: exceeds 10 min.
- Kernel vitest for `🧪️framework-notices` / `🧪️history-notices` (last: 2 files / 5-6 tests): runs through the kernel owner script with an explicit test policy (vitest, not `bun test`) and its exact command is not in the reports; not attempted.
- `verify dependencies` (ratchet; last 17.6 min) and `verify taxonomy report` repo-wide: outside the brief and above the cap.
- Repo-wide `verify docstrings emoji-unique` (no list): publishes the whole-tree acceptance record and walks 34k sources; the touched-file form was run instead.
- Everything that needs cargo/wasm: the G12 acceptance batches, the derived payload laws' cap/cap+1 `mutation.too-large` runs, kernel/plugin Rust law suites (`cargo test` of `framework_notices`, `fault_params`, `history_*`), and any `describe`/`materialize`/`activate`/`generate` target. These are listed as owed in the S4 reports and were not touched here (the detached activation holds the cargo lease).

Stopped by me: one ad-hoc `find .` over the repository (an attempt to find recently modified files after the debug-tags run) hit the 120 s tool limit and was moved to the background by the shell; I stopped it. It was a read-only `find`, not a gate. The `verify debug-tags` run itself leaves no file behind (acceptance records are written only when `SEMIO_ACCEPTANCE_RESULT` is set).
