# 🧪️ 1030 Audit: E2E readiness of the mit-Bestand demonstrator (auditor e2eaudit)

Status: NO SOURCE BLOCKER. Remaining blockers are operational: the browser install is still running, and activation has not produced its union receipt yet. Read-only audit; no source edited, no activate/prepare/serve/test-e2e/cargo run, no sub-agents. Measured 10/30 after `deps-browsers.txt` was last written (06:36).

## 1. Vite config vs `semioActivationVitePlugin` (PASS)

- `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts:102` passes `receiptDirectory`, `moduleRoot`, `installRoot`, `components`. This matches `ActivationVitePluginOptions` at `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🚀️activation-vite-plugin/🧬️options/🟦️.ts:5`.
- `demonstratorUnionReceiptVitePlugin` (line 101) is registered before the activation plugin, as its doc requires.
- Every relative named import in the vite config, the acceptance suite, the activation module and the e2e modules resolves to an export (static checker, `1030-tools/e2eaudit/imports.py`). Negative control: a bogus symbol is reported as MISSING.
- Package-specifier imports were not checked statically. The demonstrator `node_modules` symlink now resolves (it dangled in 1029).

## 2. Acceptance PANE_CASES vs `DEMONSTRATOR_PANES` (PASS)

- `🧪️tests/🎭️acceptance/🟦️.ts:195-276` lists paneIds in order: generator, koordinator, aggregator, energie, aussuchen, bearbeiten (273), verfolgen (274), statik (276).
- `🪧️brand.ts:912` declares ids in the same order: generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik.
- The consistency test at `🎭️acceptance/🟦️.ts:452` reads the brand ids as text, so drift fails there.

## 3. Activation gate lanes vs `📋️project.json` targets (PASS on targets; gate not yet run)

- Lanes: `activate-lanes-prototype` (`📋️project.json:278`) depends on `activate-energy`, `activate-fem3d`, `activate-generator`. This matches `demonstratorActivationLanes()` in `🔨️modules/🧩️runtime/♻️activation/🟦️.ts`: energy and fem3d lie outside the demonstrator closure (`depends-on` cad, gis, procedural, process, puzzle, sourcing, flow-*), and the other variants are covered by it.
- Target existence, from the current graph (`mcp__repo__show_targets`, `graph_policy=current`, 6710 names, saved to `🗑️generated/1030/e2eaudit-targets.txt`): all 9 `prepare-{aggregator,aussuchen,bearbeiten,energy,fem3d,generation3d,generator,koordinator,verfolgen}-react-prototype` and `activate-{energy,fem3d,generator}-react-prototype` exist. The names are inferred per playground variant in `📥️inference/🎮️playground-targets/🟨️.mjs`, and every variant has a Cargo `variant =` declaration under `🌎️hub/🧩️compositions/🎪️demonstrator` and `…/🔋️energy`, `…/🏗️fem`, `…/🌀️procedural`.
- `deps-browsers`, `prepare-e2e`, `test-e2e` also exist as names.
- Caveat: the target list is flat, so the owning project of each name was not confirmed. The demonstrator's `@semio-tech/framework-os-dev:` prefix is assumed.
- Gate union: `mergeDemonstratorActivationReceipts` throws if the three lane receipts disagree on any `artifactSha256`, or if the union is not exact. Nothing has been produced yet (no `dist/` under the demonstrator), so the gate is unproven.

## 4. Playwright output path (PASS with 1 char headroom)

- Guard: `🔨️modules/🧪️e2e/📁️artifacts/🟦️.ts:5-9`, `output.length + reservedSuffixChars(102) > maxPathChars(256)`. `.length` is UTF-16 units.
- With `SEMIO_TICKET_DIR` = ticket folder: output = 153 units, plus 102 = 255. Passes with 1 unit headroom only.
- With `SEMIO_TICKET_DIR` unset: output = `<repo>/dist/reports/e/<32 hex>`, about 76 units, comfortable. Leave the variable unset and move the output into `🗑️generated/1029` or `1030` afterwards, as 1029 recommended.

## 5. TS imports of renamed or removed Rust/WIT symbols (PASS)

- `🧰️framework/…/🔌️plugin/🧬️schema/📜️reactor.wit` no longer exports `reserve-poll-input`, `write-poll-input`, `close-poll-input`, `actor-poll-input-call` or `poll-input-page` (HEAD still had them). It exports `stage-command-page` and `stage-cold-pair-page` instead.
- No TS/TSX/MJS file in the repo references the removed names. The `publication-authority` hits are script and fixture names, not imported symbols.

## Blockers (concrete, with fix)

| # | Blocker | Where | Fix |
|---|---|---|---|
| B1 | Chromium not installed: `tools/ms-playwright` does not exist. `workspace:deps-browsers` run is still going (`🗑️generated/1030/deps-browsers.txt`, last write 06:36, stage `build:project-assembly`). The 10/28 run failed on `Exact original descendant creator identity required` at `🧰️framework/…/native-build/💾️storage/🟦️.ts:1155-1162`. | `📋️project.json` `prepare-e2e.dependsOn` (`workspace:deps-browsers`); `🔨️modules/🧪️e2e/🟦️.ts:23` | Let the running `deps-browsers` finish and read its rc. If rc≠0, diagnose the creator-claim sidecar on the partition that holds `tools/ms-playwright` (1029-audit-e2e-ready section 3). |
| B2 | Activation not run: no `♻️mit-bestand/🧺️demonstrator/dist/♻️activation/prototype`. The union gate (`📋️project.json` `activate-prototype`) and the dev union watcher need it. | `🔨️modules/🧩️runtime/♻️activation/🟦️.ts` `DEMONSTRATOR_UNION_RECEIPT_DIRECTORY` | Coordinator runs `activate-prototype` once the 1028 compile frontier is green. Verify the three lane receipts and the merged union. |
| B3 | Output path has 1 char headroom if `SEMIO_TICKET_DIR` is set. | `📁️artifacts/🟦️.ts:5-9`; `🔣️.json` `reservedSuffixChars` | Leave `SEMIO_TICKET_DIR` unset (as in 1029). Do not raise the limit. |

## Non-blockers and notes

- The demonstrator `node_modules` symlink resolves now (it dangled in 1029).
- `serve` on 6029 and `test-e2e` use different ports, so they do not conflict.
- Energy window kind vs the stale descriptor (`✏️s/🔌️plugins/🔋️energy/🔣️.json`) and `statik` example choice remain runtime risks from 1029 section 5, not checked here.

## Tools

- `1030-tools/e2eaudit/imports.py` (static import checker, with negative control).
- `🗑️generated/1030/e2eaudit-targets.txt` (target name list from `repo show_targets`, graph_policy=current).
