# 📓️ S5 — every editor: faults by owner

Generated 2026-10-06 03:22:08 by `🧪️s5-agnostic-matrix.py` (re-run by the unattended acceptance batch after every family). One line per crate that ran and did not pass, grouped by owner: class — first failing `file:line` — first message. A class marked `auto` was assigned by pattern while no agent was reading; `MECHANISM CANDIDATE` rows are the ones to review first. Classes: mechanism (the framework's time-travel / store path), plugin (a leaf, a fixture, an action or an example of that plugin), harness (the law's own driver — S5-AGNOSTIC), compile-red (the lib-test does not build), peer-in-flight (a peer's uncommitted edit).

## S5-FLOWCAD

- `flow-flow` — FAIL (plugin) — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:1176` — 19:13 G12 ok (leafless), inputs ok, CHILD LAW ok (`/dx = 26`, row "Drag 1 node by (25, 5)", seeded / overwritten / alternative documents reload identically). `documents_reload_identically`: flow's own example route `setActiveExample{exampleId: demo}` faults on its own asset — `app.message` "expected LBrace, found Ident 'x' at 11:7" (`📚️examples/🎬️demo/🖼️assets/🎮️.cmd.semio`)

## S5-FLOWCAD (+ S5-RUNTIME: see report S5.3)

- `cad-cad` — FAIL (plugin) — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:826` — 19:13 inputs ok (19 schemas). G12: none of 0 committed and 30 derived cases exercises an edit — 6 leaves hand out owned children (`create-energy-model`, `create-node`, `create-structure-classic-model`, `create-building-model`, `create-drawing`, `replace-references`): applied through the store they leave a document whose archive the loader refuses (`plugin.internal.document-archive-replacement.closure-rejected`, Incom

## S5-GRAPHS-WIRES

- `dag-dag` — FAIL (plugin) — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:1332` — 19:13 reload law ok (1 document). `child_history_edits_end_to_end`: the seed gesture `addNode` faults when settled with the framework's own fixture protocol — `plugin.internal`: "typed-operation emitted a store lane absent from its exact factory publication contract": dag's `addNode` writes the composed child lane but its declared publication contract does not list it (wires' `addNode` and sequence's / flow's `nodeGr

## S5-STORE (mechanism); S5-UI + S5-STROKES-NORM (registry); S5-AGNOSTIC (harness, fixed v9)

- `raster-raster` — FAIL (mechanism 1, registry 1, harness 1) — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:1414` — 01:17 (1) MECHANISM — `history_edits_end_to_end` panics in raster's Drop witness (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:311`, "Raster owned map reached Drop before every entry and page backing was explicitly retired") because the kernel store drops the replayed working snapshot by drop glue on a REJECTED apply: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` `replay_mutations` (:22147) returns `Err

## S5-TEXT-STDIO

- `stdio-pdf` — payload law (derive-emitted, not the acceptance law) — plugin — op 44 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1
- `stdio-gltf` — payload law (derive-emitted, not the acceptance law) — plugin — op 76 (patch-snapshot): answers 128 inverse row(s) where its leaf schema declares 1

