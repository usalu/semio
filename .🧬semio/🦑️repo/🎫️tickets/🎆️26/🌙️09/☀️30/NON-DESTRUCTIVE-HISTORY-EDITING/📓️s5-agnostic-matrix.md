# 📓️ S5-AGNOSTIC matrix — is history editing artifact-agnostic?

Generated 2026-10-06 03:22:08 by `🧪️s5-agnostic-matrix.py` (S5-AGNOSTIC; evidence and method: `📓️s2-agnostic-report.md` § Session 5).
One row per plugin artifact crate that wires `history_edit_acceptance_law!` (G12), `composed_reload_law!` (reload) or
`composed_child_history_law!` (child). A verdict is a run of this WP on the tree of the stated time; nothing is inferred from
other reports. Rows before 06:14 ran on the pre-wave-B (channel 21) tree and say so. "Bare inputs" = the inputs gate's
`inferred` + `refused` rows among the artifact's document-lane leaf inputs (every depth); the gate reads catalogued leaves only.

## Headline

- Acceptance law over 96 artifact crates / 34 plugins: FAIL (mechanism 1, registry 1, harness 1) 1, FAIL (plugin) 3, NOT RUN 85, PASS 7.
- Lib-test compiles: not built by this WP 82, yes 14.
- Document-lane leaves 2483, withdraw-only (`editable: false`) 133; bare inputs 3481 of 18255 document-lane inputs.
- Owners of the non-passing rows: S5-FLOWCAD: flow-flow; S5-FLOWCAD (+ S5-RUNTIME: see report S5.3): cad-cad; S5-GRAPHS-WIRES: dag-dag; S5-STORE (mechanism); S5-UI + S5-STROKES-NORM (registry); S5-AGNOSTIC (harness, fixed v9): raster-raster.
- Census over the crates that ran law v3: 167 of 316 editable leaves exercised end to end by a case (the rest are named with their skip reason in the family logs).
- Payload-law failures (derive-emitted `semio_payload_law_*`, not the acceptance law): stdio-pdf → S5-TEXT-STDIO: op 44 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1; stdio-gltf → S5-TEXT-STDIO: op 76 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1.

## What has actually run (honest state)

- 10 of 96 artifact crates have run the acceptance laws on the live tree (build B2, channel 23, 2026-10-05 17:25–19:13), each in a private cargo folder, one cargo at a time: 7 PASS (puzzle 2d, 3d, 5d; stdio pdf, gltf; sequence; wires), 3 FAIL, all classified PLUGIN (dag, flow, cad). No failure was a framework mechanism failure. 4 more crates (raster, drawing, layout, note) BUILT their lib-test but their laws were not executed. 82 crates were never built by this WP.
- A PASS proves, through the generic verbs only: withdraw → accept → restore leaves zero trace; the preview right after Begin equals the document as of the edited mutation (downstream not applied); a schema-valid input change → accept → Report replay → finalize as overwrite AND as a new alternative, each head a fresh fold of the edited log, both reloads alike, row label en/de; one passing scenario per input-control kind the editors offer; no committed case that breaks the mechanism. For a composed plugin (sequence, wires, flow's child lane) the same session runs on the member store (`store` argument) and the seeded, overwritten and alternative documents reload identically.
- A PASS does NOT prove every leaf: the census column says how many editable leaves a case exercised end to end (154 of 294 so far); the others are named with their skip reason in `🗑️generated/s5-agnostic/family-*.test.txt` (fixture content: a seed that does not apply on the shipped document, an index input that blocks the replay, no changeable input). Preview-as-of and withdraw/restore are not asserted on the child lane.
- Every `NOT RUN` is owed, not presumed green.

## How to run

One family per call. The runner builds in the private folder `🗑️generated/s5-agnostic/target` (both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR`, `CARGO_BUILD_JOBS=4`, `RUST_MIN_STACK=268435456`), starts nothing while `🗑️generated/coord/activation.flag` exists or below 12 GiB free (exit 5 — owed), and keeps the folder between families until it exceeds 12 GiB. Re-issue the same call when the 10-minute cap of a Bash call moves it to the background (it waits, reports, and is the disk watchdog). Measured: 15 min cold for the first family (3 GiB), 5–8 min for a following one; the folder reaches 12 GiB after about 15 crates — set `S5_DROP_DIR=1` on the last family. A family whose build ended with less than 12 GiB free is NOT executed: free disk and re-issue the call (the build is then a no-op).

## Faults by owner (every row that ran and did not pass)

- **S5-FLOWCAD** — `flow-flow` FAIL (plugin): 19:13 G12 ok (leafless), inputs ok, CHILD LAW ok (`/dx = 26`, row "Drag 1 node by (25, 5)", seeded / overwritten / alternative documents reload identically). `documents_reload_identically`: flow's own example route `setActiveExample{exampleId: demo}` faults on its own asset — `app.message` "expected LBrace, found Ident 'x' at 11:7" (`📚️examples/🎬️demo/🖼️assets/🎮️.cmd.semio`)
- **S5-FLOWCAD (+ S5-RUNTIME: see report S5.3)** — `cad-cad` FAIL (plugin): 19:13 inputs ok (19 schemas). G12: none of 0 committed and 30 derived cases exercises an edit — 6 leaves hand out owned children (`create-energy-model`, `create-node`, `create-structure-classic-model`, `create-building-model`, `create-drawing`, `replace-references`): applied through the store they leave a document whose archive the loader refuses (`plugin.internal.document-archive-replacement.closure-rejected`, Incomplete) BEFORE any edit (harness control); the other leaves name entities the initial document lacks (`mutation.target-missing`). cad commits no case with a document and wires no child law
- **S5-GRAPHS-WIRES** — `dag-dag` FAIL (plugin): 19:13 reload law ok (1 document). `child_history_edits_end_to_end`: the seed gesture `addNode` faults when settled with the framework's own fixture protocol — `plugin.internal`: "typed-operation emitted a store lane absent from its exact factory publication contract": dag's `addNode` writes the composed child lane but its declared publication contract does not list it (wires' `addNode` and sequence's / flow's `nodeGraphEdit` publish fine)
- **S5-STORE (mechanism); S5-UI + S5-STROKES-NORM (registry); S5-AGNOSTIC (harness, fixed v9)** — `raster-raster` FAIL (mechanism 1, registry 1, harness 1): 01:17 (1) MECHANISM — `history_edits_end_to_end` panics in raster's Drop witness (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:311`, "Raster owned map reached Drop before every entry and page backing was explicitly retired") because the kernel store drops the replayed working snapshot by drop glue on a REJECTED apply: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` `replay_mutations` (:22147) returns `Err(VcsError::Rejected{..})` (and `?` on `encode_op` / `diff.apply`) without `retire_replayed_projection(snapshot)` / `retire_scratch_operations(forwards|inverse)` — its own `InverseRefused` branch retires them. Backtrace: drop_glue::<RasterSnapshot> ← replay_mutations ← apply_comma
- **S5-TEXT-STDIO** — `stdio-pdf` payload law (derive-emitted, not the acceptance law): op 44 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1
- **S5-TEXT-STDIO** — `stdio-gltf` payload law (derive-emitted, not the acceptance law): op 76 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1

## Remaining families — exact commands (every crate without a PASS; one call per line, in this order of value)

```
cd /Users/ueli/Documents/semio
T=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING
zsh $T/🧪️s5-agnostic-run-family.sh raster semio-s-artifact-raster-raster
zsh $T/🧪️s5-agnostic-run-family.sh draw semio-s-artifact-draw-drawing
zsh $T/🧪️s5-agnostic-run-family.sh layout semio-s-artifact-layout-layout
zsh $T/🧪️s5-agnostic-run-family.sh note semio-s-artifact-note-note
zsh $T/🧪️s5-agnostic-run-family.sh space semio-s-artifact-space-home semio-s-artifact-space-space
zsh $T/🧪️s5-agnostic-run-family.sh mathematical semio-s-artifact-mathematical-equation
zsh $T/🧪️s5-agnostic-run-family.sh imperative semio-s-artifact-imperative-procedure
zsh $T/🧪️s5-agnostic-run-family.sh writer semio-s-artifact-writer-writer
zsh $T/🧪️s5-agnostic-run-family.sh vcs semio-s-artifact-vcs-vcs
zsh $T/🧪️s5-agnostic-run-family.sh trinity semio-s-artifact-trinity-rewriting semio-s-artifact-trinity-jack
zsh $T/🧪️s5-agnostic-run-family.sh remodel semio-s-artifact-remodel-remodeling
zsh $T/🧪️s5-agnostic-run-family.sh process semio-s-artifact-process-process3d
zsh $T/🧪️s5-agnostic-run-family.sh wfc semio-s-artifact-wfc-2d semio-s-artifact-wfc-grid2d semio-s-artifact-wfc-bitmap semio-s-artifact-wfc-3d semio-s-artifact-wfc-grid3d
zsh $T/🧪️s5-agnostic-run-family.sh fem semio-s-artifact-fem-2d semio-s-artifact-fem-3d
zsh $T/🧪️s5-agnostic-run-family.sh lowpoly semio-s-artifact-lowpoly-lowpoly
zsh $T/🧪️s5-agnostic-run-family.sh shooting semio-s-artifact-shooting-shooting
zsh $T/🧪️s5-agnostic-run-family.sh energy semio-s-artifact-energy-model
zsh $T/🧪️s5-agnostic-run-family.sh forms semio-s-artifact-forms-forms
zsh $T/🧪️s5-agnostic-run-family.sh gis semio-s-artifact-gis-gisterrain semio-s-artifact-gis-gismap
zsh $T/🧪️s5-agnostic-run-family.sh procedural semio-s-artifact-procedural-generation2d semio-s-artifact-procedural-generation3d
zsh $T/🧪️s5-agnostic-run-family.sh playbook semio-s-artifact-playbook-playbook
zsh $T/🧪️s5-agnostic-run-family.sh norm-1 semio-s-artifact-norm-en1990 semio-s-artifact-norm-din18599 semio-s-artifact-norm-en1997 semio-s-artifact-norm-din16798 semio-s-artifact-norm-en1991 semio-s-artifact-norm-en1992 semio-s-artifact-norm-vdi3805 semio-s-artifact-norm-iso16757 semio-s-artifact-norm-en1993
zsh $T/🧪️s5-agnostic-run-family.sh norm-2 semio-s-artifact-norm-en1994 semio-s-artifact-norm-din4108 semio-s-artifact-norm-en1996 semio-s-artifact-norm-en1995 semio-s-artifact-norm-en1999 semio-s-artifact-norm-en1998
zsh $T/🧪️s5-agnostic-run-family.sh stdio-1 semio-s-artifact-stdio-las semio-s-artifact-stdio-html semio-s-artifact-stdio-epw semio-s-artifact-stdio-zip semio-s-artifact-stdio-gif semio-s-artifact-stdio-mp4 semio-s-artifact-stdio-svg semio-s-artifact-stdio-mp3 semio-s-artifact-stdio-ifc
zsh $T/🧪️s5-agnostic-run-family.sh stdio-2 semio-s-artifact-stdio-bcf semio-s-artifact-stdio-binary semio-s-artifact-stdio-csv semio-s-artifact-stdio-step semio-s-artifact-stdio-tsv semio-s-artifact-stdio-xlsx semio-s-artifact-stdio-docx semio-s-artifact-stdio-md semio-s-artifact-stdio-xml
zsh $T/🧪️s5-agnostic-run-family.sh stdio-3 semio-s-artifact-stdio-png semio-s-artifact-stdio-jpg semio-s-artifact-stdio-avi semio-s-artifact-stdio-pptx semio-s-artifact-stdio-wav semio-s-artifact-stdio-txt semio-s-artifact-stdio-stl semio-s-artifact-stdio-dwg semio-s-artifact-stdio-dxf
zsh $T/🧪️s5-agnostic-run-family.sh stdio-4 semio-s-artifact-stdio-tiff semio-s-artifact-stdio-deflate semio-s-artifact-stdio-obj semio-s-artifact-stdio-ply semio-s-artifact-stdio-json semio-s-artifact-stdio-semio semio-s-artifact-stdio-bmp
zsh $T/🧪️s5-agnostic-run-family.sh block semio-s-artifact-block-2d semio-s-artifact-block-5d semio-s-artifact-block-3d
zsh $T/🧪️s5-agnostic-run-family.sh animate semio-s-artifact-animate-presentation
zsh $T/🧪️s5-agnostic-run-family.sh architect semio-s-artifact-architect-program
zsh $T/🧪️s5-agnostic-run-family.sh demonstrator semio-s-artifact-demonstrator-playground
zsh $T/🧪️s5-agnostic-run-family.sh sourcing semio-s-artifact-sourcing-curation
zsh $T/🧪️s5-agnostic-run-family.sh dag semio-s-artifact-dag-dag
zsh $T/🧪️s5-agnostic-run-family.sh flow semio-s-artifact-flow-flow
zsh $T/🧪️s5-agnostic-run-family.sh cad semio-s-artifact-cad-cad
python3 $T/🧪️s5-agnostic-matrix.py
```

## Matrix

| Plugin | Artifact crate | Laws | Lib-test compiles | Acceptance law | G12 / inputs / reload / child | Conflict (cascade, law v8) | Leaves exercised (census) | Control kinds proven | Payload laws ok·fail | Leaves | Withdraw-only | Undeclared inputs (gate) | Input-less editable leaves (gate) | Bare inputs (inferred + refused of total) | Owner WP | First failing file:line — reason |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ✒️writer | `writer-writer` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 5 | 0 | 0 | 0 | 0 + 0 of 9 | S5-TEXT-STDIO |  |
| ➗️mathematical | `mathematical-equation` | G12+reload | not built by this WP | NOT RUN |  |  |  |  |  | 18 | 0 | 0 (+3 catalogue-stale) | 0 | 0 + 0 of 48 | S5-GRAPHS-WIRES |  |
| 🀄️wfc | `wfc-2d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 17 | 0 | 0 | 0 | 0 + 0 of 48 | S5-STROKES-NORM |  |
| 🀄️wfc | `wfc-grid2d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 14 | 0 | 0 | 0 | 0 + 0 of 105 | S5-STROKES-NORM |  |
| 🀄️wfc | `wfc-bitmap` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 11 | 0 | 0 | 0 | 0 + 0 of 37 | S5-STROKES-NORM |  |
| 🀄️wfc | `wfc-3d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 17 | 0 | 0 | 0 | 0 + 0 of 58 | S5-STROKES-NORM |  |
| 🀄️wfc | `wfc-grid3d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 14 | 0 | 0 | 0 | 0 + 0 of 42 | S5-STROKES-NORM |  |
| 🌀️procedural | `procedural-generation2d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 16 | 0 | 0 | 0 | 0 + 0 of 43 | S5-TOOLS |  |
| 🌀️procedural | `procedural-generation3d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 20 | 0 | 2 | 0 | 1 + 5 of 65 | S5-TOOLS |  |
| 🌊️flow | `flow-flow` | G12+child+reload | yes (19:13) | FAIL (plugin) | ok / ok / FAIL / ok | not run (pre-v8) |  |  | 0·0 | 0 | 0 | 0 (+10 catalogue-stale) | 0 | 0 of 0 (no catalogued leaf) | S5-FLOWCAD | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:1176` — 19:13 G12 ok (leafless), inputs ok, CHILD LAW ok (`/dx = 26`, row "Drag 1 node by (25, 5)", seeded / overwritten / alternative documents reload identically). `documents_reload_identically`: flow's own example route `setActiveExample{exampleId: demo}` faults on |
| 🌍️gis | `gis-gisterrain` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 2 | 0 | 4 | 0 | 3 + 4 of 8 | S5-TOOLS |  |
| 🌍️gis | `gis-gismap` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 12 | 0 | 6 | 0 | 18 + 0 of 27 | S5-TOOLS |  |
| 🌿️vcs | `vcs-vcs` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 6 | 0 | 0 | 0 | 0 + 0 of 6 | S5-TEXT-STDIO |  |
| 🎞️animate | `animate-presentation` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 9 | 0 | 22 | 0 | 27 + 0 of 44 | main (no S5 owner) |  |
| 🎥️shooting | `shooting-shooting` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 31 | 0 | 0 | 0 | 0 + 0 of 93 | S5-TOOLS |  |
| 🎪️demonstrator | `demonstrator-playground` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 1 | 0 | 0 | 0 | 0 + 0 of 1 | main (no S5 owner) |  |
| 🎬️sequence | `sequence-sequence` | G12+child+reload | yes (19:13) | PASS | ok / ok / ok / ok | not run (pre-v8) |  |  | 0·0 | 0 | 0 | 0 (+8 catalogue-stale) | 0 | 0 of 0 (no catalogued leaf) | S5-GRAPHS-WIRES |  |
| 🏗️fem | `fem-2d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 30 | 0 | 1 | 0 | 0 + 0 of 112 | S5-TOOLS |  |
| 🏗️fem | `fem-3d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 30 | 0 | 1 | 0 | 0 + 0 of 128 | S5-TOOLS |  |
| 🏛️architect | `architect-program` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 0 | 0 | 2 | 0 | 2 + 0 of 6271 | main (no S5 owner) |  |
| 🏭️process | `process-process3d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 15 | 0 | 14 | 0 | 18 + 0 of 67 | S5-STROKES-NORM |  |
| 💠️lowpoly | `lowpoly-lowpoly` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 21 | 0 | 8 | 0 | 24 + 0 of 94 | S5-TOOLS |  |
| 💡️reasoning | `reasoning-wires` | child+reload | yes (19:13) | PASS | - / - / ok / ok | not run (pre-v8) |  |  | 2·0 | 0 | 0 | 0 (+12 catalogue-stale) | 0 | 0 of 0 (no catalogued leaf) | S5-GRAPHS-WIRES |  |
| 📋️forms | `forms-forms` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 13 | 0 | 1 | 0 | 0 + 7 of 124 | S5-TOOLS |  |
| 📏️layout | `layout-layout` | G12 | yes (03:22, build only) | NOT RUN |  |  |  |  |  | 48 | 0 | 0 | 0 | 1 + 0 of 190 | S5-TOOLS | `🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:62` — its lib-test BUILT on the B2 tree (19:14–19:30, `cargo build --tests --keep-going` exit 0, 15 min 55 s), but the law run was refused by the disk floor (11 < 12 GiB) and I then deleted the 12 GiB private folder by mistake before executing the binaries; nothing  |
| 📐️cad | `cad-cad` | G12 | yes (19:13) | FAIL (plugin) | FAIL / ok / - / - | not run (pre-v8) |  |  | 1·0 | 19 | 10 | 0 (+8 catalogue-stale) | 4 | 0 + 0 of 49 | S5-FLOWCAD (+ S5-RUNTIME: see report S5.3) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:826` — 19:13 inputs ok (19 schemas). G12: none of 0 committed and 30 derived cases exercises an edit — 6 leaves hand out owned children (`create-energy-model`, `create-node`, `create-structure-classic-model`, `create-building-model`, `create-drawing`, `replace-refere |
| 📕️norm | `norm-en1990` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 30 | 0 | 0 | 0 | 13 + 0 of 98 | S5-STROKES-NORM |  |
| 📕️norm | `norm-din18599` | G12+reload | not built by this WP | NOT RUN |  |  |  |  |  | 19 | 0 | 0 | 0 | 3 + 0 of 66 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1997` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 20 | 0 | 0 | 0 | 5 + 0 of 75 | S5-STROKES-NORM |  |
| 📕️norm | `norm-din16798` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 41 | 0 | 0 | 0 | 4 + 0 of 110 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1991` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 80 | 0 | 0 | 0 | 7 + 0 of 129 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1992` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 28 | 0 | 0 | 0 | 11 + 0 of 143 | S5-STROKES-NORM |  |
| 📕️norm | `norm-vdi3805` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 19 | 0 | 0 | 0 | 11 + 0 of 121 | S5-STROKES-NORM |  |
| 📕️norm | `norm-iso16757` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 29 | 0 | 0 | 0 | 20 + 0 of 170 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1993` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 49 | 0 | 0 | 0 | 74 + 0 of 413 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1994` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 25 | 0 | 0 | 0 | 8 + 0 of 133 | S5-STROKES-NORM |  |
| 📕️norm | `norm-din4108` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 43 | 0 | 0 | 0 | 10 + 0 of 158 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1996` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 58 | 0 | 0 | 0 | 8 + 0 of 212 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1995` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 66 | 0 | 0 | 0 | 6 + 0 of 222 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1999` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 18 | 0 | 0 | 0 | 33 + 0 of 171 | S5-STROKES-NORM |  |
| 📕️norm | `norm-en1998` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 29 | 0 | 0 | 0 | 18 + 0 of 171 | S5-STROKES-NORM |  |
| 📖️playbook | `playbook-playbook` | G12+child+reload | not built by this WP | NOT RUN |  |  |  |  |  | 1 | 0 | 0 (+8 catalogue-stale) | 0 | 1 + 0 of 1 | S5-TOOLS |  |
| 📜️imperative | `imperative-procedure` | child+reload | not built by this WP | NOT RUN |  |  |  |  |  | 0 | 0 | 0 (+4 catalogue-stale) | 0 | 0 of 0 (no catalogued leaf) | S5-GRAPHS-WIRES |  |
| 📸️remodel | `remodel-remodeling` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 36 | 8 | 0 | 0 | 17 + 0 of 160 | S5-STROKES-NORM |  |
| 🔋️energy | `energy-model` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 297 | 2 | 6 (+16 catalogue-stale) | 0 | 6 + 0 of 730 | S5-TOOLS |  |
| 🔱️trinity | `trinity-rewriting` | G12+child+reload | not built by this WP | NOT RUN |  |  |  |  |  | 9 | 1 | 3 (+5 catalogue-stale) | 0 | 0 + 0 of 46 | S5-TEXT-STDIO |  |
| 🔱️trinity | `trinity-jack` | G12+child+reload | not built by this WP | NOT RUN |  |  |  |  |  | 1 | 0 | 0 (+8 catalogue-stale) | 0 | 0 + 0 of 1 | S5-TEXT-STDIO |  |
| 🕸️dag | `dag-dag` | child+reload | yes (19:13) | FAIL (plugin) | - / - / ok / FAIL | not run (pre-v8) |  |  | 2·0 | 0 | 0 | 0 (+17 catalogue-stale) | 0 | 0 of 0 (no catalogued leaf) | S5-GRAPHS-WIRES | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:1332` — 19:13 reload law ok (1 document). `child_history_edits_end_to_end`: the seed gesture `addNode` faults when settled with the framework's own fixture protocol — `plugin.internal`: "typed-operation emitted a store lane absent from its exact factory publication co |
| 🖍️draw | `draw-drawing` | G12 | yes (02:28, build only) | NOT RUN |  |  |  |  |  | 22 | 0 | 2 (+1 catalogue-stale) | 0 | 2 + 0 of 80 | S5-TOOLS | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🦀️.rs:44` — its lib-test BUILT on the B2 tree (19:14–19:30, `cargo build --tests --keep-going` exit 0, 15 min 55 s), but the law run was refused by the disk floor (11 < 12 GiB) and I then deleted the 12 GiB private folder by mistake before executing the binaries; nothing  |
| 🖨️raster | `raster-raster` | G12 | yes (02:03) | FAIL (mechanism 1, registry 1, harness 1) | ok / FAIL / - / - | resolved (withdraw) | 13 of 22 (5 on a shipped document) | number ✓ boolean ✓ option ✓ vector ✓ text ✓ | 1·0 | 22 | 0 | 0 (+3 catalogue-stale) | 0 | 0 + 0 of 118 | S5-STORE (mechanism); S5-UI + S5-STROKES-NORM (registry); S5-AGNOSTIC (harness, fixed v9) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:1414` — 01:17 (1) MECHANISM — `history_edits_end_to_end` panics in raster's Drop witness (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:311`, "Raster owned map reached Drop before every entry and page backing was explicitly retired") because the kernel store drop |
| 🗄️stdio | `stdio-las` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 15 | 2 | 25 (+1 catalogue-stale) | 0 | 69 + 0 of 106 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-html` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 9 | 1 | 7 | 0 | 9 + 0 of 28 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-epw` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 13 | 1 | 1 (+1 catalogue-stale) | 0 | 2 + 0 of 64 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-zip` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 14 | 4 | 60 (+1 catalogue-stale) | 0 | 120 + 0 of 202 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-gif` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 33 | 2 | 36 (+2 catalogue-stale) | 0 | 42 + 0 of 110 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-mp4` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 10 | 1 | 91 | 0 | 175 + 0 of 264 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-svg` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 32 | 4 | 12 (+3 catalogue-stale) | 0 | 32 + 0 of 152 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-mp3` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 5 | 1 | 14 (+1 catalogue-stale) | 0 | 33 + 0 of 58 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-ifc` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 32 | 5 | 11 (+2 catalogue-stale) | 0 | 43 + 0 of 290 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-bcf` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 14 | 2 | 2 (+1 catalogue-stale) | 0 | 29 + 0 of 121 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-binary` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 4 | 1 | 2 | 0 | 3 + 0 of 11 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-csv` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 6 | 1 | 0 | 0 | 0 + 0 of 19 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-step` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 39 | 7 | 29 (+1 catalogue-stale) | 0 | 103 + 0 of 281 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-tsv` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 7 | 1 | 0 (+1 catalogue-stale) | 0 | 1 + 0 of 13 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-xlsx` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 25 | 5 | 4 (+2 catalogue-stale) | 0 | 38 + 0 of 138 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-pdf` | G12 | yes (18:46) | PASS | ok / ok / - / - | not run (pre-v8) | 20 of 60 (44 on a shipped document) | number ✓ boolean ✓ option ✓ vector ✓ text ✓ | 9·1 | 152 | 16 | 131 (+3 catalogue-stale) | 0 | 565 + 0 of 953 | S5-TEXT-STDIO | payload law (derive-emitted, not the acceptance law) FAILS: op 44 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1 |
| 🗄️stdio | `stdio-docx` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 33 | 5 | 21 (+1 catalogue-stale) | 0 | 36 + 0 of 242 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-md` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 5 | 1 | 0 | 0 | 1 + 0 of 14 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-xml` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 17 | 2 | 0 (+2 catalogue-stale) | 0 | 14 + 0 of 74 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-png` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 5 | 1 | 0 (+15 catalogue-stale) | 0 | 2 + 0 of 16 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-jpg` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 22 | 3 | 20 (+1 catalogue-stale) | 0 | 90 + 0 of 148 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-avi` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 13 | 1 | 49 (+1 catalogue-stale) | 0 | 50 + 0 of 90 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-pptx` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 25 | 5 | 9 (+1 catalogue-stale) | 0 | 32 + 0 of 182 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-wav` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 6 | 2 | 9 | 0 | 25 + 0 of 40 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-txt` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 6 | 1 | 0 (+1 catalogue-stale) | 0 | 1 + 0 of 7 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-stl` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 7 | 1 | 3 (+1 catalogue-stale) | 0 | 3 + 0 of 16 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-dwg` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 3 | 1 | 0 (+1 catalogue-stale) | 0 | 0 + 0 of 3 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-dxf` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 19 | 1 | 9 (+1 catalogue-stale) | 0 | 20 + 0 of 66 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-tiff` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 17 | 4 | 4 (+3 catalogue-stale) | 0 | 14 + 0 of 52 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-deflate` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 4 | 1 | 1 | 0 | 5 + 0 of 14 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-obj` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 22 | 1 | 30 (+1 catalogue-stale) | 0 | 57 + 0 of 104 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-gltf` | G12 | yes (18:27) | PASS | ok / ok / - / - | not run (pre-v8) | 44 of 121 | number ✓ boolean ✓ option ✓ vector — text ✓ | 0·1 | 0 | 0 | 20 | 1 | 244 + 0 of 429 | S5-TEXT-STDIO | payload law (derive-emitted, not the acceptance law) FAILS: op 76 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1 |
| 🗄️stdio | `stdio-ply` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 10 | 1 | 1 (+1 catalogue-stale) | 0 | 10 + 0 of 32 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-json` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 15 | 1 | 0 | 0 | 7 + 0 of 34 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-semio` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 271 | 25 | 312 (+29 catalogue-stale) | 0 | 1220 + 16 of 1692 | S5-TEXT-STDIO |  |
| 🗄️stdio | `stdio-bmp` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 4 | 1 | 0 (+8 catalogue-stale) | 0 | 0 of 0 (no catalogued leaf) | S5-TEXT-STDIO |  |
| 🗒️note | `note-note` | G12 | yes (19:31, build only) | NOT RUN |  |  |  |  |  | 33 | 0 | 0 | 0 | 0 + 0 of 76 | S5-TOOLS | its lib-test BUILT on the B2 tree (19:14–19:30, `cargo build --tests --keep-going` exit 0, 15 min 55 s), but the law run was refused by the disk floor (11 < 12 GiB) and I then deleted the 12 GiB private folder by mistake before executing the binaries; nothing  |
| 🧩️puzzle | `puzzle-2d` | G12 | yes (17:25) | PASS | ok / ok / - / - | not run (pre-v8) | 30 of 36 | number ✓ boolean ✓ option ✓ vector — text ✓ | 1·0 | 36 | 0 | 0 | 0 | 0 + 0 of 220 | S5-PUZZLE |  |
| 🧩️puzzle | `puzzle-5d` | G12 | yes (18:27) | PASS | ok / ok / - / - | not run (pre-v8) | 31 of 39 | number ✓ boolean ✓ option ✓ vector ✓ text ✓ | 1·0 | 39 | 0 | 0 | 0 | 0 + 0 of 219 | S5-PUZZLE |  |
| 🧩️puzzle | `puzzle-3d` | G12 | yes (17:25) | PASS | ok / ok / - / - | not run (pre-v8) | 29 of 38 | number ✓ boolean ✓ option ✓ vector ✓ text ✓ | 1·0 | 38 | 0 | 0 | 0 | 0 + 0 of 203 | S5-PUZZLE |  |
| 🧱️block | `block-2d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 26 | 0 | 0 | 0 | 0 + 0 of 58 | main (no S5 owner) |  |
| 🧱️block | `block-5d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 41 | 0 | 0 | 0 | 0 + 0 of 102 | main (no S5 owner) |  |
| 🧱️block | `block-3d` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 37 | 0 | 0 | 0 | 0 + 0 of 89 | main (no S5 owner) |  |
| 🪐️space | `space-home` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 1 | 0 | 0 | 0 | 0 + 0 of 1 | S5-GRAPHS-WIRES |  |
| 🪐️space | `space-space` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 4 | 0 | 0 | 0 | 0 + 0 of 19 | S5-GRAPHS-WIRES |  |
| 🪵️sourcing | `sourcing-curation` | G12 | not built by this WP | NOT RUN |  |  |  |  |  | 3 | 0 | 2 | 0 | 3 + 0 of 6 | main (no S5 owner) |  |
