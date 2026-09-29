# WP-FH1 — Fault Localization Helper: Families A (Framework Catalog) + H (11 Small Plugins)

Slice: FH1 (session 14c, 2026-09-29), helper of S20's fault-localization set (`📓️fault-localization-api.md` §3/§4).
Works ONLY in the faults overlay `.🧬semio/🌐hub/s14-s20-overlay-faults/` (APFS clone owned by S20). Scripts:
`.tmp-ticket/wp-fh1/`; captures: `.🧬semio/🌐hub/s14-fh1-*`. No ticket open/close.

## Session 14

| # | Item | State | Evidence |
|---|---|---|---|
| H | family H: every raise site `app_fault("…")` (+ `.with_parameter`), every code declared en/de on the app definitions that raise it | **census 0 violations 11:5x** (162 raises, 135 declarations); crate checks queued behind "framework green" | `wp-fh1/fh1-census.ts H`, `.🧬semio/🌐hub/s14-fh1-census/family-H.json` |
| A | family A: framework catalog `{code, en, de}` for every framework-raised code; computed-code + framework-declaration sites | **official `verify faults framework` 12:0x: 1 violation = S20's `📡️replication/🎮️mutation:1018` (hands off, coordinator decision pending); catalog 613 entries en/de (12:1x + S20's `plugin.action.argument-missing` {action}); framework-declaration 0; computed-code 0 of mine** | `.🧬semio/🌐hub/s14-fh1-census/verify-AH-1.txt`, `wp-fh1/catalog_texts.py`, `wp-fh1/fh1-catalog.py`, `wp-fh1/a/*.py` |
| C | crate checks (`check --offline --lib --tests`) for H plugins + touched framework/hub/gis/cad/stdio crates | **not run** — waits for S20's "framework green" (coordinator paused FH1 12:0x) | — |

### Resume checklist (when S20 reports "framework green")

1. Re-run the census: `cd .tmp-ticket/wp-fh1 && GIT_DIR=/Users/ueli/Documents/semio/.git GIT_WORK_TREE=<overlay> NX_DAEMON=false bun fh1-census.ts A H`
   (writes `.🧬semio/🌐hub/s14-fh1-census/{family-A,family-H,framework-raised}.json`); if new framework codes appear, add their
   en/de to `wp-fh1/catalog_texts.py` and regenerate with `python3 fh1-catalog.py` (writes exactly the raised codes).
2. Crate checks on the overlay lane (foreground, one batch): `zsh .tmp-ticket/wp-s20/s20-f1/overlay-cargo.sh fh1-H check --offline --lib --tests`
   with `-p` for the 11 H plugin crates (writer, remodel, energy, animate, sourcing, block, dag, shooting, demonstrator, imperative +
   its 5 extensions — wasm32-only `extension-entry` code is NOT compiled natively, re-read it by eye —, playbook + procedural module),
   plus the touched non-H crates: semio-framework-plugin (+tests), the kernel, hub (`🌎️hub/💡️inference`), gis, cad aec-building, stdio gltf.
   Find package names with `/usr/bin/grep -m1 '^name' <crate>/Cargo.toml`.
3. Expected compile follow-ups: nested modules that use bare `app_fault` without importing it (energy `args_bridge` already fixed);
   `FaultCode` vs `&str` comparisons of `ArtifactInferenceExecutionError.code` (gis + plugin wire tests fixed; others would show);
   the plugin-assembly fault now carries `plugin.assembly-failed`.
4. Laws to run after green: energy editor unit laws (35 re-pointed asserts), remodel add-stream + qc-report laws, plugin
   `🔬️plugin-runtime-runtime-cleanup-fault-vector` (fault_code arms == vector table), jobs unit laws, inference service/wire laws,
   gis gismap + native-codecs laws.
5. Then record rc + capture paths in the table above and relay to S20 + main.

### Family H per plugin (census on the overlay, 11:5x)

| Plugin | Raise sites | Codes | Declared | Violations | Declared on |
|---|---|---|---|---|---|
| ✒️writer | 23 | 9 | 9 | 0 | editor (9), viewer (2) |
| 🎞️animate | 17 | 14 | 14 | 0 | editor (14), viewer (1) |
| 🎥️shooting | 7 | 7 | 7 | 0 | editor |
| 🎪️demonstrator | 5 | 4 | 4 | 0 | playground editor |
| 📖️playbook | 5 | 5 | 5 | 0 | editor (3), procedural module app (2) |
| 📜️imperative | 9 | 5 | 5 | 0 | editor (5, incl. the 5 extensions' `extension.evaluate`), viewer (1) |
| 📸️remodel | 18 | 16 | 16 | 0 | editor |
| 🔋️energy | 45 | 31 | 31 | 0 | editor (30), viewer (7) |
| 🕸️dag | 9 | 9 | 9 | 0 | editor (9), viewer (1) |
| 🧱️block | 11 | 10 | 10 | 0 | 2d editor (2), 5d editor (2), 3d editor (7) |
| 🪵️sourcing | 13 | 12 | 12 | 0 | editor (12), viewer (1) |

Crate check (`cargo check --lib --tests`): not run yet — waits for S20's "framework green".

### Decisions (family H)

- Existing well-formed codes kept (`writer-command-tool-mismatch`, `remodeling.qc-report.missing`, `dag.unhandled-action`, …).
- Codes whose one code carried several meanings were split so each code has ONE text and ONE parameter set:
  energy `mutation.invalid-payload` / `mutation.target-missing` / `mutation.target-in-use` (editor layer) →
  `energy.model.<entity>.missing {id}` (9, via a `ModelEntity` enum), `energy.model.zone.in-use-by-{space,surface,thermostat} {id}`,
  `energy.model.property.unknown {property}`, `energy.model.property.value {property, value}`, `energy.model.zone.volume`,
  `energy.model.surface.class-unknown {class}`, `energy.model.site.location`, `energy.model.run-period.invalid`,
  `energy.model.example.unknown {example}`, `energy.model.simulation-settings.range`, `energy.model.result-field.unknown {field}`,
  `energy.model.construction.layer-material {material}`; energy camera `app.command.invalid-payload` → `energy.model.camera.invalid`;
  sourcing `sourcing.invalid-payload: …` → `sourcing.document-json.limit` / `.schema`; shooting export → `nothing-to-export` + `no-shots`.
  The energy MUTATION fixtures (`🧫️fixtures/🧬️mutations/…/⛔️refuses`) keep `mutation.*` — that is the reducer layer, not a Fault raise.
- True duplicates merged: energy viewer `energy.model.3d.viewer.window-{required,kind}` → the editor's `energy.model.3d.window-*`;
  `energy.model.viewer.retained.tool-mismatch` → `energy.model.retained.tool-mismatch`; writer's 10 window-command stubs → `writer.main-window-required`.
- Unknown-action refusals: `app.command.unsupported {action}` (one text in every H plugin); existing `dag./block3d./playbook./imperative.unhandled-action` kept with `{action}`.
- Entity kinds are codes, never parameters (a parameter is shown verbatim in both locales); parameters carry ids, user values and property keys only.
- Animate's video export error answers its own refusal (`PresentationVideoExportError::fault()` replaces the computed `code()`).
- Laws updated to the new codes (tests assert codes, never text): energy editor unit laws (35 asserts incl. the out-of-range loop now
  pairs command → code), remodel add-stream law (code instead of message), remodel qc-report law (sentence assert dropped).
- Declared text of remodel `exportQcReport` action description corrected ("without a report the export is refused").

### Log

- 10:48 start; read preamble 1–27, API brief, census families A/H. Framework compile queued by S20 (overlay lane).
- 10:5x census runner `wp-fh1/fh1-census.ts` (imports the overlay's `runFaultCensus`, writes family A/H subsets to
  `.🧬semio/🌐hub/s14-fh1-census/`); baseline A 758 violations (uncatalogued 704, framework-declaration 41, computed-code 13), H 219.
- 11:0x found the F2 double crate path `semio_framework_plugin::semio_framework_plugin::app_fault(` in ~38 files (all families) →
  relayed to S20, who swept all 103 sites.
- 11:0x–11:5x family H: codemod `wp-fh1/fh1-raise.py` (every `Fault::new(<origin>, FaultCode::new("x"), msg)` → `app_fault("x")`,
  import tidy), `wp-fh1/fh1-tidy.py`, then per-plugin handcrafted scripts `wp-fh1/h/<plugin>.py` with the en/de table `wp-fh1/texts.py`.
  Census H → 0.
- Census gap noted for S20: `Fault::from` used as a PATH (`.map_err(Fault::from)`, no parenthesis) is not read by the census
  (39 files repo-wide); the compiler will list them once `From<String> for Fault` is gone.
- 11:1x–12:1x family A: catalog written from `wp-fh1/catalog_texts.py` (handcrafted per code; homogeneous close/cleanup/
  capacity invariant groups share one phrasing built from an area named in both languages with German number agreement) by
  `wp-fh1/fh1-catalog.py` for exactly the codes the census says framework crates raise (literal + const, `framework-raised.json`).
- framework-declaration 41 → 0: store schema-JSON helper `fault`→`diagnose` (35 calls + 3 fns), renderer asset probe `fault`→`refuse`
  (`wp-fh1/a/rename-diagnose.py`) — they build decode diagnostics / probe steps, never a declared Fault.
- computed-code → literal codes (S20 approved the file list 11:2x): builtin jobs (`BuiltinJobKind` with literal code tables, helpers
  take `FaultCode`; `a/jobs.py`), `ArtifactInferenceExecutionError.code: FaultCode` (framework 35 + hub 2 + gis 10 / cad 1 / stdio 3
  constructor sites wrapped, gis + plugin wire tests compare `.code.as_str()`; `a/inference-error.py`), host effects/imports
  `fault_bytes`/`emit_completed_err` take `FaultCode` (48 callers; `a/host-fault-bytes.py`), `DocumentLinkStatus::fault_code`,
  `ImportStagingRefusal::fault_code`, tool-run trace rejection match, window-config reject closure + load-diagnostic match,
  intent bridge `reject`, `protocol_fault`, `transaction_fault` (`a/computed-codes.py`), path `Fault::from` → literal codes
  (`a/fault-from-paths.py`), `RuntimeCleanupFault::fault_code` (19 arms beside the vector table), `push_os_fault(FaultCode)`,
  plugin assembly failure → `plugin.assembly-failed` (`a/plugin-runtime-codes.py`).
- Census bug found + fixed by S20 (11:4x): nested `#[cfg(test)]` over-blanking hid 💼️jobs/💡️infer (27 raises) and 🖥️host
  `surface.*` (4); tool `wp-fh1/fh1-blanking.ts`.
- 12:0x official verb run (A + H owners) → 1 violation (S20's); relayed to S20 + main. Coordinator paused FH1 until "framework green".
- 12:1x S20 request: catalogued `plugin.action.argument-missing` {action} (raised twice in the SDK, both with `action`); catalog 613.
  The census run before the regeneration showed it (and one other) uncatalogued; generator now writes all 613 raised codes, 0 missing.
