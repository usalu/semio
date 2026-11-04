# ✅️ Validation: Per-Artifact Acceptance Harness and Baseline (T06)

## Harness

Root: `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/` (schema first, fixture driven, no per-artifact code).

| Part | Path (under the root) | Role |
| --- | --- | --- |
| Schema | `🧬️schema/🔣️.json` | JSON Schema 2020-12 `semio.stdio.artifact-acceptance/v1`: row = batch, artifact, kind, plugin, extension, oracle, surface, edit pin, export format, invalid input (hex) |
| Fixture | `🧫️fixtures/🔣️.json` | 38 artifact rows (batches A-H) plus `nonArtifacts` (`📇️inventory`, `🔣️.json`). Generated once by `<ticket>/build-acceptance-fixture.py`, then hand-owned |
| Oracles | `🔬️oracles/🟦️.ts` | Third-party judges: `JSON.parse`, fast-xml-parser, pngjs, fflate (zip, OOXML, zlib), published magic for the other formats |
| Loader | `📇️matrix/🟦️.ts` | Typed rows, `candidateSamples` (skips `✂️truncated`, `🪄️wrong-magic`, `invalid/truncated/corrupt/malformed/broken` names), `resolveSample` (first candidate the oracle accepts) |
| Contract (offline) | `🧪️contract/🟦️.ts` | bun test; 155 tests: schema, coverage of every artifact directory, per row definition agreement, shipped editor+viewer in the family component and generated catalog, sample accepted by the oracle, invalid input refused by the oracle |
| Browser journey | `🏃️browser/🟦️.ts` | Playwright journey through the real OS shell per row: open (command palette spawn row), render, edit (`mutateUndoRedo` pins from the fixture), save (`#s-checkin` count moves), close (dock tab), reopen, reload+reopen, export (rail row, download judged by the oracle), invalid import (file picker with the invalid bytes must raise a refusal notice and no pageerror). Writes `journey.json` and `table.md` per tag |

It reuses the existing harness helpers of `🧑‍💻dev` program-matrix (`@semio-tech/framework-os-dev/testing/program-matrix`: `withDevServe`, `readShell`, `mutateUndoRedo`, `unfoldActionsRail`, ...) and the repo's Chromium launcher; it adds no dependency.

## Run commands

Offline contract (no server; the operand needs the `./` prefix, the repo guard refuses it otherwise):

```bash
cd /home/deck/semio
repo tools exec bun -- test "./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts"
```

Browser journey (needs the dev server T05 records in `🛠️runtime.md`, default `http://127.0.0.1:6070/`; `withDevServe` reuses a live server):

```bash
repo tools exec bun -- "./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts" run --serve http://127.0.0.1:6070/ [--batch A,B] [--only txt,json] [--locale en|de] [--tag baseline] [--out <dir>]
```

Output goes to `🤖️generated/<tag>/{journey.json,table.md,exports/}` inside the harness root; move it under the ticket `🗑️generated/` or delete it after use (tool output, not a deliverable).

## Status of what ran

| Item | Status | Evidence |
| --- | --- | --- |
| Contract test, first run | RAN: 143 pass, 12 fail of 155 | see baseline below |
| Contract test after sample resolver and `semio` oracle fix | partially re-run: only idml and idf still failed in a direct probe (`resolveSample` returned `NO SAMPLE` for exactly those two); full rerun is QUEUED behind the capacity gate (swap above ceiling, 5 running, 6 ahead). Log `🗑️generated/[DEBUG]contract.txt` |
| Browser journey | NOT RUN: no dev server yet (`🛠️runtime.md` absent), and the TS compile check (`bun build`) of `🏃️browser/🟦️.ts` timed out in the capacity queue, so that file is written but UNCOMPILED and UNRUN. Expect first-run selector fixes |
| Consolidated cargo plan | below; nothing started by me |
| Generators (`composition-prepare`, `native-codec-projection`) | NOT RUN, waiting for batch definitions to settle |
| `natural-file-lifecycle` reconciliation | NOT STARTED (see plan) |

## Baseline: offline contract, per artifact (first run)

Pass for all rows on: definition agreement, shipped editor+viewer in the family component and the generated catalog, and invalid-input refusal. Failures:

| Batch | Artifact | Failing check | Cause / work for the batch owner |
| --- | --- | --- | --- |
| B | idml | catalog/surface; sample | No editor, viewer, family component or activation; no valid `.idml` anywhere in the tree (B owner) |
| F | idf | catalog/surface; sample | Same: no surface, no valid `.idf` sample (F owner) |
| C | bmp | sample | Smallest `.bmp` was the demo asset `📚️examples/🎬️demo/🖼️assets/🧪️example/🖼️.bmp` containing the text `hello` (placeholder, not a bitmap). Valid candidates exist; replace the placeholder (C owner) |
| D | dxf | sample | Demo asset `🖊️.dxf` is the text `Hello, stdio.txt!` (D owner) |
| E | stl | sample | Demo asset `🧊️.stl` is the same placeholder text (E owner) |
| E | gltf, ply | sample | Smallest candidates were `natural/invalid.gltf`/`invalid.ply` (deliberately broken, now excluded by the resolver); not real failures |
| F | bcf | sample | Smallest candidate `natural-open/truncated.bcf` (deliberately broken, now excluded) |
| H | deflate | sample | Demo asset `🗜️.zz` header `78bb...` has the FDICT flag: fflate refuses it ("need dictionary"). Either the sample is a preset-dictionary stream (then the oracle needs the dictionary) or it is malformed (H owner to decide) |
| H | semio | sample | Oracle fix: demo `.pack.semio` is a binary pack (`89 53 45 4d 0d 0a 1a 0a` magic), now accepted |

After the resolver and oracle fixes the only hard row failures left are idml and idf; bmp/dxf/stl/deflate pass only because other valid files exist, and their placeholder demo assets remain findings for the owners.

Other findings:

1. The generated catalog `🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json` lists only 7 `on-artifact-kind:` events for plugin `stdio` (txt xml json csv md tsv html); the other families are separate plugins with their own events (contract checks this per row).
2. Existing harnesses (`verify matrix`, hub `io-matrix`, hub `document-sweep`) pin only five stdio kinds (txt md html json xml) and the import/export pins list no stdio program at all (`io-matrix.json` has `programs: []`). The new journey is the first per-stdio-kind end-to-end flow.

## Consolidated cargo plan (for the coordinator)

Constraints: each artifact is its own Cargo workspace, so `-p` flags combine only within one artifact directory; the capacity gate admits about 2 GB per job, so chain sequentially in ONE background shell:

1. One shared-store invocation per workspace, several `-p` per call: the 36 shipped artifacts' `*-tests`, `*-editor`, `*-viewer` crates of one artifact in a single `repo cargo --store=shared --configuration=prototype -- test -p <a> -p <b> --lib`.
2. Order to unblock most workers: (a) `🌎️hub/🧩️compositions/🗄️stdio` family component assembly and `editor-catalog`/`shipped-fleet` laws (all batches depend on them), (b) txt/md/json/xml/html/csv/tsv (A, B text), (c) image family (C), (d) mesh+cad (D, E), (e) bim/office/media/binary (B, F, G, H).
3. After batch definitions stop changing: `repo run @semio-tech/stdio-plugin:composition-prepare` then `:native-codec-projection`, once each.
4. TS: contract test above, then `repo run @semio-tech/stdio-js:test`; then the browser journey once.

Cancellation request: every worker's separate narrow `-p` run that is already queued for a crate also covered by a later combined call can be dropped; I have not cancelled anything (I own no worker's jobs). Please have workers stop adding new narrow runs and send crate lists to me instead.

## Blockers

1. No dev server recorded (`🛠️runtime.md` missing), so no browser baseline; no real data for open/edit/save/reopen/export per artifact yet.
2. Capacity gate backlog: the contract rerun and the harness compile check are queued; the browser file is therefore unverified.
3. The browser harness assumes rail ids matching `/export/i` and `/import/i`, the palette row `spawn.<plugin>.<appId>` and `#s-checkin`; Details-pane-only editors (17 windows without edit actions per `🎟️capability-audit.md`) will fail `edit` until T16/batches decide.
4. Pending coordinator notes not yet done: take valid/invalid samples from `🧫️fixture-manifest.json` instead of the name heuristic (currently both folder names and basenames are rejected; migrating to the manifest needs it checked in under the harness with paths under 256 characters), and the single-pass `natural-file-lifecycle` reconciliation.

## Update 2 (T06)

- Contract test now 194 tests: 191 pass, 3 fail (`B/idml` surface and sample, `F/idf` surface). Every other artifact passes: definition, shipped editor and viewer, catalog activation, valid sample accepted, invalid samples refused.
- Samples now come from the schema-defined manifest checked into the harness: `🧫️fixtures/📇️samples.json` with schema `🧬️schema/📇️samples.json` (copied from `🧫️fixture-manifest.json`; longest path 153 characters). The name heuristic is gone. Invalid samples are asserted refused where the oracle can tell: `🪄️wrong-magic` for magic-based oracles, `✂️truncated` for full parsers (png, zip, ooxml, bcf, zlib, json, xml, svg). Oracle gaps fixed on the way: zip family now requires the local header magic, mp3 requires an MPEG-1 Layer III frame sync, mp4 requires a sane `ftyp` box size.
- Edit path: the fixture schema gained `edit.path` (`main` or `details`) and `edit.field`; the browser journey builds the Details control `[data-ui-node-key="edit"][aria-label^="<field>:"]` and the verb `setSnapshotValue` for `details` rows. No row declares `details` yet: the concrete field for each of the 17 read-only main-window editors needs the T16/T18 host map (not available to me), so those rows still use `scan` and will fail `edit` until their owners fill `path` and `field`.
- `natural-file-lifecycle` reconciled: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs` now has named per-batch sets A to H (48+16+3 apps) and asserts the declared app ids equal the union of those sets exactly (fixture holds 67 unique ids). Not compiled yet; the test file is not listed as a `[[test]]` of `semio-framework-plugin` (only autodiscovery by another owner), so the owning crate must be confirmed by T10.
- Generators: `composition-prepare` then `native-codec-projection` are running chained in one background shell (logs `🗑️generated/[DEBUG]composition-prepare.log`, `[DEBUG]native-codec-projection.log`), currently waiting in the capacity queue (load 37-43, swap above ceiling). No failure yet. The trusted-catalog publish and the per-workspace cargo chain start only after both generators exit 0.
- Browser journey: still unrun (needs the dev server from T05, whose activation was stuck in dependency sync at last report) and uncompiled.

## Update 3 (T06)

- Evidence: `🗑️generated/[DEBUG]contract.txt` (bun test of the contract, 194 tests: 192 pass, 2 fail; the earlier 191/194 run is superseded by it). Remaining failures: `B/idml > ships an editor and a viewer` and `B/idml > has a natural-file sample` only.
- idf row encodes the coordinator contract: `plugin: energy`, `ownerApp: s.energy.model@1/*#editor`; the contract asserts that app exists in a family manifest and in the `natural-file-lifecycle` fixture with a `s.stdio.idf@...` format kind, and that the energy IDF sample (in `📇️samples.json`) is accepted. It passes.
- Details edit rows now declared (`edit.path: details`, verb `setSnapshotValue`, `field` from `🎟️capability-audit.md`): las, gif, mp4, avi, mp3, svg, jpg, tiff, stl, ply, dxf, dwg, obj, gltf, step, ifc. The `field` strings are the audit's pointers or names, and the values are my placeholders; both are UNCONFIRMED against each editor's Details tree (the journey matches `aria-label^="<field>:"`) and must be corrected on the first browser run by the batch owners.
- idml: needs a valid minimal IDML sample (deterministic, validated with Python `zipfile`) added by batch B to `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧫️fixtures/📇️samples.json` (`samples.idml.valid`), and the surface appears after `composition-prepare` regenerates the roster. I could not message sonnet-editors-b from here; the coordinator should relay.
- `natural-file-lifecycle` is mounted by `semio-framework-plugin-tests-07` (per `📦️crate-names.md`); not yet compiled. Generators remain queued behind the capacity gate.

## Update 4 (T06)

Chain after the generators (script `run-generator-chain.sh`, waits for `native-codec-projection` to write its `rc=`): `repo run workspace:components describe stdio --profile prototype` (regenerates the component descriptors/captures after T04's `stdio.semio.<subset>` schema ids), then one shared-store cargo call in the hub stdio workspace running `editor_catalog`, `shipped_fleet`, `format_resolution`, `semio_natural_dispatch`, `native_openable_provider`, `component_assembly`. Logs: `🗑️generated/[DEBUG]components-describe-stdio.log`, `[DEBUG]hub-stdio-laws.log`. The trusted-catalog publish has no project target (hand-edited integrity pointers, `os-hub trusted-catalog publish`) and is NOT included; it follows once the generators exit 0. Nothing in the chain has completed yet.

## Transcribed evidence (T20, before the `🗑️generated` deletion)

Every result the coverage report relies on, copied from its log so it survives deletion. Log paths are kept for reference; the logs themselves are deleted at close. Exit codes are given only where the log records one.

### E1. Offline acceptance contract (Update 3)

- Command: `cd /home/deck/semio && repo tools exec bun -- test "./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts"` (command from "Run commands" above).
- Log: `🗑️generated/[DEBUG]contract.txt` (1,928 bytes).
- Exit code: not recorded in the log. `bun test` exits non-zero when any test fails, so a non-zero exit is expected here. [U]
- Header: `bun test v1.4.2 (744846f84)`; capacity admitted 1 job, 2,048 MB.
- Result lines (verbatim): `192 pass`, `2 fail`, `381 expect() calls`, `Ran 194 tests across 1 file. [2.66s]`.
- Failing tests (verbatim): `(fail) B/idml > ships an editor and a viewer in its family component and the generated catalog [11.19ms]` and `(fail) B/idml > has a natural-file sample accepted by its third-party oracle [0.37ms]`.
- Key failure lines: `Expected: "shipped"` / `Received: "missing"` (idml family component), and `Expected: > 0` / `Received: 0` (idml sample candidates).
- Passing rows: all 37 artifacts other than idml (idf passes, per Update 3).
- Not editor e2e. This is an offline contract; it does not open an editor.

### E2. Registry format-resolution law (os-io crate)

- Log: `🗑️generated/t01-os-io-format-resolution.log`. Final line `EXIT 0` (recorded by the wrapper).
- Command: `repo cargo --store=shared` for the package `semio-framework-os-io` (the log shows `cargo:test`; the argv is not echoed; test target `🧪️tests/🧭️format-resolution/🦀️.rs`). [U: exact argv]
- Result (verbatim): `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`.
- Tests: `a_revision_is_picked_by_its_declared_header_and_otherwise_the_owner_opens` ok; `a_claim_held_by_two_formats_is_refused_instead_of_guessed` ok; `every_fixture_case_resolves_with_the_frozen_precedence` ok; `declared_extension_order_is_the_canonical_export_precedence` ok; `single_extension_names_agree_with_the_standard_library_path_oracle` ok.
- Scope: registry law only. No editor row is an editor test.

### E3. Host vitest (reported, no log)

- `🖥️host.md` §5: 2 files, 29 tests passed (`🚪️replay-open-artifact`, `🛰️framework-plugin-runtime`).
- `🖥️host.md` §6: 3 files, 55 tests passed (plus `📂️picked-file-opening`).
- `🖥️host.md` §7: 7 files, 94 tests passed (after the twin dedupe, saved-state and a11y changes). Command: vitest with the temporary `vitest.host.config.mjs` (jsdom).
- No log under `🗑️generated/` records these numbers. They are cited from the reports only. [U: re-run needed for a log]

### E4. Registry vitest (reported, no log)

- `📇️registry.md` Addendum 5: `vitest run --config <ticket>/vitest.registry.config.mjs`: 4 tests passed (control-names 1, format-resolution 3). No log. [U]

### E5. Failed cargo starts (root workspace selection)

Each of these is a failure, not a pass. Exit status 1 is recorded in the `OwnedCommandFailure` lines.

| Log | Command (as logged) | Verbatim failure |
| --- | --- | --- |
| `h-zip-editor-test.log` | `cargo test -p semio-s-artifact-stdio-zip-editor …` from the repo root | `error: package ID specification \`semio-s-artifact-stdio-zip-editor\` did not match any packages`; `OwnedCommandFailure: repository:command failed (1)` |
| `e-obj-editor-test.log` | `cargo test -p semio-s-artifact-stdio-obj editor:: …` from the repo root | `error: package ID specification \`semio-s-artifact-stdio-obj\` did not match any packages`; `OwnedCommandFailure` |
| `e-semio-s-artifact-stdio-obj-editor::obj.log` | `cargo test -p semio-s-artifact-stdio-obj --lib editor::obj …` from the repo root | `error: package ID specification \`semio-s-artifact-stdio-obj\` did not match any packages`; `exited with status 1`; `OwnedCommandFailure` |
| `e-transform-test.log` | `cargo test -p semio-s-artifact-stdio-contract-editing transform …` | `error: package ID specification \`semio-s-artifact-stdio-contract-editing\` did not match any packages`; `exited with status` (status 1) |

Root cause (recorded in `📦️crate-names.md` and `📘️commands.md`): the root workspace excludes the artifact-level workspaces. Re-run from the owning artifact directory.

### E6. Runs with no result yet (transcribed as state)

- `d-dxf-editor-shared.log`: 461 `Compiling` lines, 0 `error[E…]`, warnings only. No test result.
- `d-dxf-editor-baseline.log`: last line `Cancelled (SIGTERM); leases released.` No result.
- `a-txt-editor-test.log`, `f-batch.log`, `h-zip-editor-test2.log`, `t13-wav-codec.log`, `batch-g-mp3.log`, `b08-idml-run.txt`, `show-stdio-plugin.txt`, `[DEBUG]composition-prepare.log`, `t04-semio-test.txt`, `t01-ui-validation.log`: capacity-wait lines only. No result and no error.
- `activate-s.log`: dependency sync, no dev server. No result.

## Re-run evidence with exit codes (T20, light suites and contract)

Logs are in `🗑️generated/` and are deleted at close; the transcriptions below keep the counts and rc values. All runs were started from `/home/deck/semio`.

### E1 (updated): acceptance contract re-run, with exit code

- Command: `repo tools exec bun -- test "./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts" > 🗑️generated/contract-rerun.log 2>&1; echo "rc=$?" >> …`
- Log: `🗑️generated/contract-rerun.log`. **rc=1.**
- Counts: `192 pass`, `2 fail`, `Ran 194 tests across 1 file. [2.26s]`.
- Failing tests, with owner: `B/idml > ships an editor and a viewer in its family component and the generated catalog` and `B/idml > has a natural-file sample accepted by its third-party oracle`. **Owner: batch B (T08, sonnet-editors-b).** Key lines: `Expected: "shipped"` / `Received: "missing"`; `Expected: > 0` / `Received: 0`.
- The earlier run (`[DEBUG]contract.txt`, E1 above) gave the same counts. Both runs agree: 192 pass, 2 fail, idml only.

### E3 (updated): host vitest, with a log, one suite does not run

- Command: `cd /home/deck/semio && HOST_TEST_ROOT=/home/deck/semio HOST_TEST_ENV=jsdom node node_modules/.bin/vitest run --config <ticket>/vitest.host.config.mjs > <ticket>/🗑️generated/host-vitest.log 2>&1; echo "rc=$?" >> …`
- Required env: `HOST_TEST_ROOT` (the glob root; set to the repo root), `HOST_TEST_ENV=jsdom` (the config defaults to `node`, and several included files are `.tsx`). `HOST_TEST_TREE` unset (the optional `⌨️roving-tabindex.tsx` is not included).
- Log: `🗑️generated/host-vitest.log`. Vitest `v4.1.12`. **rc=1.**
- Counts (verbatim): `Test Files 1 failed | 8 passed (9)`; `Tests 96 passed (96)`; `Duration 39.90s`.
- Failing suite: `🛰️framework-plugin-runtime/🟦️.ts` (0 tests ran). Verbatim: `Transform failed with 1 error: …/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx:120:349: ERROR: Expected ")" but found ";"`. The esbuild error is a syntax error in the `openNaturalFile` / import code. **Owner: T02 (sonnet-host).** The file is the scope file that T02 edited in phase 3 (single-flight import, `updateSpacePanelFrom`).
- Consequence: the previously reported 94 tests (E3 above, `🖥️host.md` §7) cannot be reproduced now. The suites that ran pass (96 tests in 8 files). The `🛰️framework-plugin-runtime` suite (29 tests in §5, part of the 94) is blocked by the syntax error. [U: whether the 94 count was taken before the syntax error was introduced]

### E4 (updated): registry vitest, with a log

- Command: `cd /home/deck/semio && HOST_TEST_ROOT=/home/deck/semio HOST_TEST_ENV=jsdom node node_modules/.bin/vitest run --config <ticket>/vitest.registry.config.mjs > <ticket>/🗑️generated/registry-vitest.log 2>&1; echo "rc=$?" >> …`
- Log: `🗑️generated/registry-vitest.log`. **rc=0.**
- Counts: `Test Files 2 passed (2)`; `Tests 4 passed (4)`. The config fixes `environment: "node"`, so `HOST_TEST_ENV` has no effect here.
- Suites: `♿️control-names` (1 test) and `🧭️format-resolution` (3 tests).

### E7 (new): media vitest configs, with logs

- `vitest.media.config.mjs` (`⏱️time-and-keys`, node environment): log `🗑️generated/media-vitest.log`, **rc=0**, `Test Files 1 passed (1)`, `Tests 20 passed (20)`. Command as in E4 with `--config <ticket>/vitest.media.config.mjs`.
- `vitest.media-react.config.mjs` (`MediaTransportHost` lifecycle, jsdom set in the config): log `🗑️generated/media-react-vitest.log`, **rc=0**, `Test Files 1 passed (1)`, `Tests 10 passed (10)`.
- Owner: batch G (T13). These cover the media transport view and host lifecycle. They are unit tests, not editor e2e.

### Summary of light-suite results

| Suite | Log | rc | Files | Tests | Failing |
| --- | --- | --- | --- | --- | --- |
| Acceptance contract (bun) | `contract-rerun.log` | 1 | 1 | 192 pass, 2 fail (of 194) | idml, owner batch B (T08) |
| Host vitest (9 files) | `host-vitest.log` | 1 | 8 pass, 1 failed to transform (of 9) | 96 pass | `🛰️framework-plugin-runtime`, owner T02 (syntax error in `🔶️scope/🟦️.tsx:120`) |
| Registry vitest | `registry-vitest.log` | 0 | 2 pass | 4 pass | none |
| Media vitest | `media-vitest.log` | 0 | 1 pass | 20 pass | none |
| Media React vitest | `media-react-vitest.log` | 0 | 1 pass | 10 pass | none |

## Update 5 (T06): pass 2 queued

State at this write: chain 1 has not produced results; `composition-prepare` is running (last phase `source.served`, 890 s) and the later steps wait for it. Nothing has finished, so there are no per-batch results yet.

Pass 2 is scripted in `run-pass2-chain.sh` (started now, waits for chain 1's `hub-stdio-laws.log` to write `rc=`), strictly sequential: `composition-prepare`, `native-codec-projection`, `components describe stdio --profile prototype`, then `repo cargo --store=shared --configuration=prototype -- test --workspace --no-fail-fast` once per artifact workspace (38 workspaces under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`, covering every batch's crates listed in `📦️crate-names.md`), then the contract test. Logs: `🗑️generated/[DEBUG]p2-*.log`, each ending in `rc=`. Caveat: the script cannot know when batch B (idml composition, office registration), F (ifc 4x3 port, `sqliteSnapshot` law key), G (real wav hash) and T04 (semio derived ids) have landed; if chain 1 finishes before they do, pass 2 must be killed (`pkill -f run-pass2-chain.sh`) and restarted by the coordinator once the sources are in. Per-batch results (A to H) will be added here from those logs.

## Update 6 (T06): pass 2 replaced by a lease-friendly pass 3

- Killed `run-pass2-chain.sh` and `run-generator-chain.sh` and the shell wrapper that would have started `native-codec-projection` right after `composition-prepare`. `composition-prepare` (PID 939199) was left running; it still holds the JS dependency lease.
- `run-pass3-chain.sh` now waits for BOTH: the file `🗑️generated/GATE-T05-activation-done` (T05 creates it after `dependencies ensure` plus activation, so the dev server can come up first) and the exit of PID 939199. Only then does it run `native-codec-projection`, `components describe stdio --profile prototype`, the artifact workspaces and the hub stdio laws, then the contract test.
- Workspace order: most-changed first (count of `git status --short` entries per artifact workspace), and any workspace with a `repo cargo` already running in that directory is skipped and recorded in `🗑️generated/[DEBUG]p3-skipped.log` instead of being rebuilt. The script cannot see a finished owner run, so the coordinator should stop it (`pkill -f run-pass3-chain` is NOT safe from a shell whose own command contains that text; use the PID from `ps`) or delete workspaces from the loop if owners report results first.
- Still nothing completed; no batch results to post.
