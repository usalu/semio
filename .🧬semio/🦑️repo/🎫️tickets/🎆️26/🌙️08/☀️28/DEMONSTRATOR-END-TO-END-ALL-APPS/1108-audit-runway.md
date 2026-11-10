# 1108 Audit: runway (gateway to e2e PASS, still executable as written?)

Read-only audit, 2026-11-08. Lane `runway`. No builds, no cargo, no git writes, no edits except this file. Scratch index created during the check was deleted.

Method: read `1107-runbook.md`, `1107-audit-paneready.md`, `1107-audit-acttargets.md`, `1108-brief.md`, and every script named by the runbook. Crate names were resolved against a grep index of all tracked and untracked `Cargo.toml` `name = "..."` lines. Manifest and test targets were checked with `ls` and by reading `[[test]]` blocks. Demonstrator targets were read from `♻️mit-bestand/🧺️demonstrator/📋️project.json`. Nothing was run.

## Verdict

Executable with 4 corrections. Every crate, manifest, port, receipt path and script path the gateway-to-e2e chain names exists. Stale parts: 3 pane smoke commands are bare (missing required args), 1 pane command is missing, 1 Nx target does not exist, and 1 audit claim is contradicted by code. Activation and serve outputs are absent, as expected before step 30.

## 1. Stale or missing references

| # | file:line | reference as written | status | current / note |
|---|---|---|---|---|
| 1 | `1107-runbook.md:56` (step 14) | `koordinator/tst.sh` (bare) | STALE | needs `<label> <manifest> <wasm\|native> <cargo test args>`. Use `koordinator/tst.sh W-koordinator-<n> ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml native -p semio-s-artifact-cad-cad --test cad_pane_smoke -- --nocapture --test-threads=1`. Manifest and `[[test]] cad_pane_smoke` exist. |
| 2 | `1107-runbook.md:56` (step 14) | `aussuchen/smoke.sh` (bare) | STALE | needs `<label>` as `$1`. Script and test `aussuchen_pane_smoke` exist. |
| 3 | `1107-runbook.md:56` (step 14) | `bearbeiten/smoke.sh` (bare) | STALE | needs `<n>` (`W-bearbeiten-<n>`). Script and test exist. |
| 4 | `1107-runbook.md:56` (step 14) | `verfolgen/tst.sh` (bare, "exact test not recorded") | MISSING | `tst.sh` is a generic wrapper needing 4 args, and no verfolgen smoke command exists. Closest working form is `panes.sh:8`: `runtime/tst.sh W-pane-gis-<n> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen --nocapture --test-threads=1`. The manifest exists. The `--lib` path `../../🦀️.rs` resolves. |
| 5 | `1107-runbook.md:51` (step 10) | `repo run @semio-tech/framework:generate` | MISSING | project `@semio-tech/framework` (`🧰️framework/📦️packages/🟦️typescript/📋️project.json`) has no `generate` target. Generate targets exist only on `@semio-tech/framework-rs`, `-graph`, `-schema`. The runbook already says the name is unknown. |
| 6 | `1107-runbook.md:103` (trap) | `🏗️builder/🌐️vite/🟦️.ts` | STALE (base) | the file exists only as `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts`. Its `viteCacheDirectory` is at line 80 in the cited file. |
| 7 | `1107-runbook.md:56` (step 14) | "each law loops PANE_SMOKE_LOCALES (en, de)" | OK (note) | the pane-smoke tests loop `semio_plugin::artifact_app_laws::PANE_SMOKE_LOCALES` (e.g. `…/generation3d/🧪️tests/🪧️pane-smoke/🦀️.rs:20`). The per-pane scripts set `SEMIO_LANGUAGE` only once, so the loop is inside the Rust test. |
| 8 | `1107-audit-paneready.md`, generator row | "no (smoke.sh sets SEMIO_LANGUAGE, test never reads it)" for de | CONTRADICTED | generator pane-smoke loops `PANE_SMOKE_LOCALES` at `…/generation3d/🧪️tests/🪧️pane-smoke/🦀️.rs:20`. Whether de is actually exercised is still unverified. |
| 9 | `1107-audit-paneready.md`, section 1 cites | `pane-smoke-law/🦀️.rs:42:106`, `:393:5`, `:16:144`, `…/🦀️.rs:320`, `aussuchen … 21:5`, `statik … 50:90` | STALE (line drift) | the audit itself records that these cites no longer match. Verdicts are stale too: no pane has a test-level green since the 11-06 runs. |
| 10 | `1107-audit-acttargets.md`, section 1 "Catalog gate", 4th bullet | `🧰️framework/🛍️products/💻️os/🔨️modules/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` (wasm :1266, :1274) | MISSING | no such path. Nearest existing: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/📦️packages/🦀️rust/📋️project.json`. Its `wasm` targets were not re-checked. |
| 11 | `1107-audit-acttargets.md`, section 1 producer rule | `🧰️framework/🔨️modules/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs` | OK (the audit's full path is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`, which exists) | only my shortened form was missing; no fix needed. |
| 12 | `1028-result-cad.md:13,31,34` (historical, not in the chain) | `semio-framework-os-store-engine`, `-canonical-json`, `-replay` | STALE | now `semio-os-store-engine`, `semio-os-store-canonical-json`, `semio-os-store-replay` (all exist). No active script, runbook or audit uses the old names. |

Crate renames: no active tooling uses `semio-framework-os-store-*`. The remaining hits are in historical result notes only.

## 2. Pane smoke commands and script existence

Pane ids are from `e2eready/expect.json` and match the 8 brand pane ids in `🪧️brand.ts`. The lane names `proc` and `gis` are mapped to their pane ids.

| pane (lane) | smoke command | smoke script exists | test target exists | in `panes.sh` |
|---|---|---|---|---|
| generator (proc) | `1105b-tools/proc/smoke.sh <label> [en\|de]` | yes | `[[test]] pane-smoke` in generation3d manifest | no |
| koordinator (cad) | `1105b-tools/koordinator/tst.sh <label> <manifest> native -p semio-s-artifact-cad-cad --test cad_pane_smoke …` | no `smoke.sh`; `tst.sh` wrapper only (needs args) | yes | yes (:4) |
| aggregator | `1105b-tools/aggregator/smoke.sh <label>` | yes | `[[test]] editor-gestures` | no |
| energie | `1105b-tools/energie/go102.sh <n>` (waits for `lock-sync-1107-ok`, present) | no `smoke.sh`; runner only | mounted-tests lib | yes (:6) |
| aussuchen | `1105b-tools/aussuchen/smoke.sh <label>` | yes | `[[test]] aussuchen_pane_smoke` | no |
| bearbeiten | `1105b-tools/bearbeiten/smoke.sh <n>` | yes | `[[test]] process3d_bearbeiten_pane_smoke` | yes (:5, process3d, via panes.sh) |
| verfolgen (gis lane) | none. Only generic `verfolgen/tst.sh`. See table row 4 | no | `--lib -- verfolgen` (laws crate) | yes (:8, gis) |
| statik | `1105b-tools/statik/smoke.sh` (no args); `statik/retry.sh <n>` for transient refusals | yes | `[[test]] statik_pane_smoke` | yes (:7, fem3d) |
| gis | same as verfolgen; `1105b-tools/gis/` holds only port and before-files, no smoke | no | as verfolgen | as verfolgen |
| proc | same as generator | yes | as generator | as generator |

`panes.sh` covers 5 of 8 panes (cad, process3d, energy, fem3d, gis). It does not cover generator, aggregator or aussuchen.

## 3. Ordered command list, gateway to e2e (runbook step numbers)

Gateway (brief): `"$C/1105b-tools/chk.sh" W-gateway-<n> Cargo.toml native --tests -p semio-plugin-testing-fixtures -p semio-os-store-testing-doubles -p semio-plugin-testing-laws` — OK (root `Cargo.toml` exists; all 3 crates resolve).

| step | command | status |
|---|---|---|
| 1 | `ls $G`, slot dirs, `uptime`, `ps` | OK |
| 2 | `bash "$C/1105b-tools/value/gate.sh" <n>` (hub.sh, ext.sh) | OK (21 ext manifests and `-p` names exist; hub manifest `🌎️hub/Cargo.toml` and 13 `semio-hub-*` crates exist) |
| 3 | `cat "$G/value-<n>-gate.rc"` | OK |
| 4 | `1105b-tools/hostcheck.sh <label>` (requires `<label>`) | OK (19 `semio-plugin-host*` names, runtime-ops, instance-app all resolve) |
| 5 | `cd 1107-a11y && python3 -I core.py; python3 -I localize.py` (dry) | OK (files exist; not run) |
| 6 | `python3 -I core.py --write` | OK (file exists) |
| 7 | `python3 -I localize.py --write` | OK (file exists) |
| 8 | `python3 -I check.py` | OK (file exists) |
| 9 | hand edits (A) | n/a (manual) |
| 10 | `repo run @semio-tech/framework:generate` | MISSING (table row 5) |
| 11 | `repo dependencies lock --all`, `sync --all` | OK (tools `dependencies_lock`, `dependencies_sync` are in the repo catalogue) |
| 12 | `chk.sh a11y-<n>-<name> <Cargo.toml> wasm -p <crate>` | NOT CHECKED (crate names not resolved this pass) |
| 13 | toolsplit: `chk.sh toolsplit-102 Cargo.toml native -p semio-plugin-tool-machine -p semio-plugin-tool-run -p semio-plugin-tool-run-operations -p semio-plugin-tool-run-turns`, wasm twin, `runtime/tst.sh toolsplit-103/104 …` | OK (all 6 crates resolve; `--lib` test names not checked) |
| 14a | `proc/smoke.sh <label> en` | OK |
| 14b | `koordinator/tst.sh` (bare) | STALE (row 1) |
| 14c | `aggregator/smoke.sh <label>` | OK |
| 14d | `energie/go102.sh <n>` | OK |
| 14e | `aussuchen/smoke.sh` (bare) | STALE (row 2) |
| 14f | `bearbeiten/smoke.sh` (bare) | STALE (row 3) |
| 14g | `verfolgen/tst.sh` (bare) | MISSING (row 4) |
| 14h | `statik/smoke.sh` | OK |
| 15 | remove `aussuchen_apply_probe` (manual) | n/a (manual; file still present per audit) |
| 16 | `git grep -n -I -F '[DEBUG]' -- '*.rs' '*.ts' '*.tsx' ':!.🧬semio'` | OK |
| 17 | debug removal (A) | n/a |
| 18 | `bash "$C/1105b-tools/prefreeze.sh"` | OK (every path it checks exists: channel-pin files, profile schema and consumption fixture, `actinfra/namedinputs.ts`, demonstrator runtime script) |
| 19 | lock and sync again | as step 11 (OK) |
| 20 | `prefreeze.sh` pre | OK |
| 21 | `printf 1 > freeze`; `slots.k`=0; `slots.wextra`=1 | OK (both slot files exist; current values 8 and 2, matching the runbook's restore value) |
| 22 | `slot.sh W-canary-act-2 canary/activate.sh canary-act-2` | OK (target `@semio-tech/framework-os-dev:activate-energy-react-prototype` is inferred, not literal in project.json; audit verdict "present") |
| 23 | `canary/serve.sh start` | OK (`🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` exists; port 6106 not re-verified) |
| 24 | `canary/probe.sh http://127.0.0.1:6106/?plugin=energy` | OK (`probe.mjs` exists) |
| 25 | `canary/serve.sh stop` | OK |
| 26 | `slot.sh W-act-150 env … repo run workspace:components -- describe energy --profile prototype` | OK (`workspace` project has `components` target) |
| 27 | describe 32 ids, label `W-act-200` | OK (command shape; ids not checked) |
| 28 | `grep -L '"appChannelVersion": 25'` | OK |
| 29 | `prefreeze.sh` go | OK |
| 30 | `slot.sh W-act-160 … repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype` | OK (target exists; script `🔨️modules/🧩️runtime/📜️script.ts activate`; `--tool=bazel` flag not verified) |
| 31 | `serve.sh start` (6029) | OK (script exists; receipts `generator|energy|fem3d` and `dist/♻️activation/prototype` are absent until step 30, expected) |
| 32 | `e2eready/run.sh --check` | OK (`probe.ts` exists; `expect.json` ids equal brand ids) |
| 33 | `e2eready/run.sh http://127.0.0.1:6029/ --panes generator,…,statik --languages en,de` | OK (flags `--panes`, `--languages`, `--no-landing`, `--out` exist; all 8 pane ids, action cases and case-file keys resolve) |
| 34 | `slot.sh W-e2e-160 … repo run @semio-tech/mit-bestand-demonstrator:test-e2e` | OK (target exists; `test-e2e` → `serve-e2e` → `prepare-e2e` → `activate-prototype`; script `🔨️modules/🧪️e2e/📜️script.ts`) |
| 35 | `repo playwright test --config "$D/🧪️tests/🎭️acceptance/🧭️behaviours/🎚️config/🟦️.ts" --grep @behaviour` | NOT CHECKED (config exists; `playwright` is a declared repo tool; command shape not run) |
| 36 | `slot.sh W-build-160 … repo run @semio-tech/mit-bestand-demonstrator:build` | OK (target and `🔨️modules/📦️site/📜️script.ts` exist) |
| 37 | teardown (`serve.sh stop`, slot files, `mv freeze`) | OK |
| 38 | close-out, `ticket_close` with explicit path | n/a (manual) |

Totals for the chain: 36 executable steps checked. STALE: 3 (14b, 14e, 14f). MISSING: 2 (10, 14g). NOT CHECKED: 2 (12, 35).

## 4. Notes for the coordinator

- Fix rows 1 to 5 in the runbook before step 14 and step 10. Rows 1 to 4 are one-line command edits. Row 5 needs the real generate project name from the a11y result.
- The runbook's step 14 language claim (de via `PANE_SMOKE_LOCALES`) is consistent with the code. The paneready audit's de column is wrong for generator and should be corrected.
- Pane verdicts are not part of this audit. Per the paneready audit, no pane has a green test-level result since the fix wave.
- Nothing here changes the activation order risk recorded in the runbook trap at line 97.
