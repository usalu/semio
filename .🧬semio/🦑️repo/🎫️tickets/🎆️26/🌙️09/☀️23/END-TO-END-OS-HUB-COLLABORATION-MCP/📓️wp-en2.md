# WP EN2 — Energy epJSON, Draw Verb Descriptions, glTF/obj/bcf/docx Reader Pipeline

Session 14, slice EN2 (Claude Code fleet, Opus executor), ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP. Coordinator = `main`.
Continues `📓️wp-en1.md` (LB-F2 energy + draw verbs) and `📓️wp-gf1.md` (F10b remainder). Ports 8100–8109 / 6600–6609 (none used yet).
Guest-linked edits = prepared patches (`wp-en2/*.py`, dry run default, `--write`) + overlay proofs during the GUEST FREEZE; landed
compile-atomic in window 3. Durable data: `.🧬semio/🌐hub/s14-en2-*/`.

## Session 14

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Energy epJSON oracle 28/32 → 32/32 (LB-F2) | 4 root causes found; EnergyPlus prototype 4/4 exact; patch sets `energy-epjson` + `test-raw-routing` written; overlay proof in progress | `generated/proto-1-shadow1.txt`, `generated/bun-routing-laws-1.txt` |
| 2 | Draw `editFill`/`editPath`/`editSelection` en/de descriptions, descriptor, AJV oracle | **patch prepared** (`draw-verb-descriptions`, dry run clean on live: 2 hunks / 0 problems); projection proof: audit 3 → **0**, Rust = AJV (3 = 3 → 0 = 0); descriptor regeneration + native/wasm32 at window 3 (Codex peer active in the file) | `generated/audit-draw-{base,patched}-1.txt`, `generated/oracle-draw-*-1.txt` |
| 3 | glTF/obj/bcf/docx reader pipeline 88/96 → 96/96 (F10b-1, F10b-2, ♾️any, bcf, docx) | F10b-1 + F10b-2 patches prepared (dry runs clean), overlay parity queued; ♾️any adapter found uncompilable (fixed in patch); bcf ×3/docx: plan measured, not built | `payload/gltf-*`, `s14-en2-logs/overlay-gltf-parity-1.txt` (pending) |

### Log

- 18:3x started; read AGENTS.md, preambles 12/13/14, fleet-14 roster/log (no CHAIN LAUNCHED yet), EN1/GF1/LB/T13 reports,
  window-3 inventory, session-13 coordinator log from 14:00.
- 18:4x EN1's captures re-read: EN1 had found the orchestrator fold (below) and hand-tested epJSON variants (A=1 infiltration,
  layered glazing, daily shadows) without reaching 4/4.
- 18:5x **LB-F2 root causes (measured, EnergyPlus 25.2.0 via the oracle's own `_validate/_run/_results`,
  `wp-en2/en2-eplus-probe.py`, data `.🧬semio/🌐hub/s14-en2-eplus/`):**
  1. **Test platform:** `⚖️parity/📋️orchestration/🟦️.ts` `subjectRawInputs()` folds every passed subject result of a case
     into ONE implementation → rawPath map, so every `@oracle-input-subject-raw` oracle scenario decodes the LAST subject
     scenario's bytes (EN1's finding) — also affects architect `mutate-program-1`/`export-program-xlsx` (Rust oracle), lowpoly
     `io-lowpoly-png-1` (Python) and statutes `breach-cache-envelope` (TS).
  2. **Infiltration:** the engine runs `ScheduledAch`/`PerExteriorArea` as a constant design flow (coefficients unused), the
     exporter copies the model's unused zeros into `A..D` → EnergyPlus simulates NO infiltration (600 heating −30.7 %).
  3. **Glazing:** the model binds every window to the two-pane `Double Clear Glazing` (`glazing_construction_id`, "that
     layered stack IS the glazing"), the exporter ignores it and writes simple glazing.
  4. **First vertex:** the document declares `UpperLeftCorner` but writes rings lower-left first; EnergyPlus derives a
     surface's height/width/in-plane frame from vertices 1–3 → 600FF peak −1.3 K (bisect: `generated/bisect-600ff-{1,2}.txt`;
     floor view factor, run-period year: no effect).
  5. (convention) `ShadowCalculation` default 20 days vs the reference's daily updates: 900FF peak −0.41 K.
  The committed `🔮️energyplus.json` are honeybee `native` (NREL BESTEST-GSR encoding, `📓️w2-oracle-toolchain.md` §6), and
  EnergyPlus on EN1's converted native IDF reproduces them exactly (`bisect-600ff-1.txt`).
- 19:0x **Prototype of fixes 2–5 on the committed documents (`wp-en2/en2-epjson-prototype.py`): 4/4, deviation 0.0000 on every
  metric** (600 heat/cool, 600FF min/max/mean, 900 heat/cool, 900FF min/max/mean; `generated/proto-1-shadow1.txt`); without
  daily shadows 4/4 with 900FF max −0.41 K (`proto-1.txt`).
- 19:0x Codex peer (coordinator heads-up) owns live draw work (ticket 26/09/26 COMPLETE-DRAW-VECTOR-EDITING-EXPERIENCE, editor
  `🦀️.rs` modified 18:41); its notes never mention the three verbs' descriptions → EN2 prepares them, re-diffs at window 3.
- 19:0x overlay `.🧬semio/🌐hub/s14-en2-overlay` (tracked-file mirror, `wp-en2/en2-overlay-sync.py`, 106 736 files, 509 s) +
  patch tool `wp-en2/en2-patch.py` (begin → edit overlay → make hunks → `apply` dry run / `--write`, never whole-file).
- 19:1x patch set **`energy-epjson`** written in the overlay (exporter: `ShadowCalculation` daily, `upper_left_first`,
  `glazing_stack`, constant-design-flow coefficients, `dangling-glazing-construction` diagnostic; importer: `ShadowCalculation`
  known, layered windows bound + fallback optics, sill/height along the host's up for any start corner,
  `coefficients-dropped` diagnostic; 3 new exporter laws + 1 importer law + stronger window round-trip law; feature/adapter
  rationale text) and **`test-raw-routing`** (TS `SubjectRawInputs` per scenario + `subjectRawInputsByScenario` +
  `AdapterContext.subjectRawBytes`; Python host + Rust protocol/runner read by `scenario.id`; statutes oracle uses the
  accessor). TS: tsc on the overlay adds 0 errors (only the pre-existing TS7016 `🟨️.mjs` + an overlay-only missing generated
  playground file; live baseline `generated/tsc-routing-live-baseline.txt`); **new routing laws 2/2 PASS** (bun, overlay,
  `generated/bun-routing-laws-1.txt`, 916 s — the file's module-level repo scan).
- 19:26 overlay energy build 1 failed before the energy crate: overlay lacks gitignored generated sources
  (`🖱️ui/🎨️styling/🔤️tokens/🦀️.rs`) → copying the ignored generated sources into the overlay, rerun queued.
- 19:5x item 2 prepared: `.describe(LocalizedLabel::native(en, de))` + `.use_when([...])` on the three `bounded_catalog` actions
  (texts derived from the command handlers: editSelection's 13 operations, editPath's 8 `PathEdit` kinds, editFill's 7
  `FillEdit` kinds; the bridge gives editSelection no default selection, so the text says "the given layers").
  `python3 wp-en2/en2-patch.py apply draw-verb-descriptions` → 2 hunks pending / 0 problems (live, 19:5x). Projection
  (`wp-en2/en2-draw-projection.py`: LB's landed-source projection + the LIVE draw descriptor, ± the three texts):
  `semio-os-mcp audit` (LB's 10:34 binary) base **3** description findings (exactly the three), patched **0**, exit 0;
  AJV twin (`wp-d1/d1-oracle.ts`) rust=ajv 3=3 → 0=0.
- 20:0x **Item 3 — F10b-1 (lossy inverses):** the camera/skin/animation/material/asset subjects never ran the production
  inverse — they replayed a hand-written `inverse_spec` (skin/animation have none for `delete-*`) or COPIED the deleted
  collection back from the before-snapshot (`undo_delete_*`). Patch set **`gltf-production-inverse`**: production test
  bridges in the gltf mutation root (`gltf_mutated_document`, `gltf_inverse_restored_document`: `(kind, params)` row →
  `GltfMutation` wire form → `Mutation::diff(..).apply_to`, then every step of `Mutation::inverse(base)`; `.glb` or
  `.gltf` in and out; a refused step is an error) + the five subjects' `inverse` on the bridge (hand-written copies
  removed). Dry run on live: 17 hunks / 7 files / 0 problems.
- 20:0x **F10b-2:** the generator recipe `create-material` inserted `{"name":"createdMaterial"}` WITHOUT reindexing
  `primitive.material` (production inserts `GltfMaterial::default()` and remaps refs ≥ 1). Recipe fixed
  (`createReindexed(..., {}, remapMaterialRefs)`); `bun ♾️any/🏭️generator/📜️script.ts generate --only create-material-applied`
  in the overlay: `before.gltf` byte-identical, `after.gltf` = default `{}` at 1, primitives `[[0],[2],[2]]`. Sets
  **`gltf-create-material-fixture`** (generator + after.gltf) and **`gltf-create-material-manifest`** (fixture-manifest
  sha256 `4308e565…` / 16 540 bytes + notes). Both dry runs clean.
- 20:1x **♾️any `🧊️mutate-gltf-2-0` is uncompilable** (its subject imports `GltfMutationLeafDescriptor`/`DESCRIPTOR`, which
  no longer exist in the gltf crate — `/usr/bin/grep` finds them only in the adapter): subject rewritten onto the two
  bridges (in `gltf-production-inverse`), narrative docstring corrected. Its feature still declares the three reader
  oracle with no committed afters (the census gap).
- 20:1x bcf ×3 + docx measured: each case declares the TS reader oracle but its Rust adapter hosts the cross-semio
  `zip-quick-xml-*` oracle; re-tagging to that is NOT allowed (`cross-semio-implementation` is supplemental, an `s.stdio.*`
  format owes a qualifying third-party reference — `QUALIFYING_ORACLE_KINDS`). Their generators build pairs from their own
  synthetic base (bcf markup: 8 applied pairs incl. no-mutation, no `set-snapshot`), the rows mutate the real review →
  real-input afters need an XML-level jszip/fast-xml-parser mutation per kind in each generator. Not built this session
  (see Open).
- 20:2x overlay proofs queued in the overlay lane: energy job (test-host tests, fixture regeneration, epJSON laws, then
  parity `🏛️export-epjson-runs-in-energyplus` + `🗜️breach-cache-envelope`; `s14-en2-logs/overlay-energy-2.txt`) and gltf
  parity job (camera, skin, animation, material, asset, ♾️any; `overlay-gltf-parity-1.txt`); `SEMIO_PYTHON` = repo `.venv`
  (jsonschema 4.26.0), `SEMIO_ORACLE_OPENSTUDIO_ROOT` = the verified OpenStudio 3.11.0 tree. Lane: t14 holding since 19:26.
- 20:2x Python host routing verified directly (`🖥️host/🐍️.py` `Context.subject_raw_bytes`, overlay copy): s1 → own bytes,
  s2 → own bytes, s3 refused with the platform's message — 3/3 (`generated/python-host-routing-1.txt`).
- 20:3x `zsh wp-en2/en2-land.sh dry` (all six sets on the live tree): test-raw-routing 16 hunks, energy-epjson 28,
  draw-verb-descriptions 2, gltf-production-inverse 17, gltf-create-material-fixture 4, gltf-create-material-manifest 2 —
  **all pending, 0 problems**. `en2-land.sh write` / `native` = the window-3 runbook (apply all, then one native-lane hold:
  test-host tests, energy `--lib --tests` check + epJSON laws, draw check, gltf check).
- 20:3x overlay lane: t14 holds since 19:26 (one 107-crate cold `--keep-going` check, rustc children progressing, 17 rustc
  machine-wide); 7 holds ahead of EN2. Asked main (20:1x) for priority or a pre-freeze landing of test-raw-routing.

### Open (EN2)

- **♾️any / bcf ×3 / docx reader wiring (the 5 `no-oracle-adapter` census rows).** Each case declares its TS reader oracle
  (required: `s.stdio.*` owes a qualifying third-party reference; the Rust adapters' `zip-quick-xml-*`/`json-rust-*` are
  supplemental cross-semio oracles and may not replace it) but mutates a REAL input whose afters were never committed.
  Honest completion = committed real-input afters from each owner's generator, then T13's reader pattern (feature step
  naming the after, TS `committedArtifact` oracle, Rust subject `actual-*` artifact, production inverse through the
  bridge): ♾️any — generator GLB mode over `🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb` (7 afters ≈ 2 MB), the
  `gltf-import` probe must hand GLB to `GLTFLoader.parse` as an ArrayBuffer (today it decodes every input as text, so a
  `.glb` cannot import despite its header comment), `gltfFixtureOutputPaths` admits only `.gltf`; bcf/docx — XML-level
  jszip + fast-xml-parser edits per kind over the real review / `example-readme.docx` (their generators only build from a
  synthetic `TopicRecipe`/docx base, which cannot carry the real inputs' content). Not started: owner decision on ≈2 MB
  of GLB fixtures and on the generator scope; EN2 spent the session on items 1–2 and F10b-1/-2.
