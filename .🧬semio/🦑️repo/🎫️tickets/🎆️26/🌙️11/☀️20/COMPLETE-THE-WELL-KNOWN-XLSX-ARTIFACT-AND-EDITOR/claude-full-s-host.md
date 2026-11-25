# Full S host (port 6070): native bring-up, state and recipe

Status at the time of writing: the S host serves and the shell boots, but the Space and Stdio wasm components were not yet built, so no editor journey (English or German) and no Hub I/O matrix ran. No journey verdict exists. This file records exactly what was done, what works, what blocks, and the commands to finish.

## Managed route: tried once, stalled

`repo run @semio-tech/plugin-registry:session-s --configuration=prototype` was started at 19:59 and stopped after about 13 minutes. It never left the graph slice load (`checking the file inventory of the tree`, 77,696 distinct source observations, owner check `0 of 600`; the graph-plan grants held 4.3 GB). This is the same endless restart on the live tree that `browser-hosts.md` documents over eight attempts, so no `session-s` build error could be read. The native route below replaces it. Log: `🗑️generated/claude-full-s/session-s.log`.

## Native recipe (no graph, no issued invocation)

Component stages normally need a repo-issued `ScriptInvocation`. The lower-level real functions do not, so each stage is reproduced by a ticket script (all in this folder):

| Stage | Script | What it does |
| --- | --- | --- |
| S session | `claude-fulls-session.ts` | calls the real `renderPlaygroundSessionTypeScript("s", projection)` over the generated catalog (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`), restricted to `space,stdio,stdio-office` (`FULLS_PLUGINS` overrides), and writes `.cache/b/p/m/browser/prototype/sessions/s/🎮️playground-session/🟦️.ts` |
| Wasm component | `claude-fulls-cargo.sh` | `cargo build -p semio-hub-space-component --lib --target wasm32-wasip2 --profile wasm-prototype` in `🌎️hub`, sharing the standalone-pane agent's `CARGO_TARGET_DIR` (`🗑️generated/claude-pane/native`) so dependencies compile once. It is a copy of `claude-pane-cargo.sh` with another `-p`. Stdio and Stdio Office are built by the pane agent's `claude-pane-cargo.sh` |
| Browser module | `claude-fulls-publish.ts <plugin>=<wasm>…` | real `transpilePluginComponentModulesAsync` (native transpiler), host shim, preview2 import rewrite, JSPI `describe()` probe under node, `finalizePluginDescriptor`, bridge, `encodeComponentReceiptV1`, `.semio-artifact.json`; publishes to `.cache/b/p/m/browser/prototype/modules/<dir>`. Receipt `sourceTreeDigest` and `provenanceSha256` are derived hashes, not cargo-observed. Verified for `stdio-office` from the earlier captured component (38 files published, describe probe accepted) |
| Activation | `claude-fulls-activate.ts s <plugin>…` | real `collectActivationArtifactsV1`, `nextActivationReceipt`, `publishActivationReceipt` for `runtime/react/s/activation/🔣️receipt.json` (support digest excludes the fonts artifact) |
| Serve | `claude-fulls-serve.sh s 6070` | vite directly from the dev package with the gis contribution config (the `s` playground row uses `✏️s/🧑‍💻dev/🎭️variants/🌍️gis`), env `SEMIO_PLUGIN=s SEMIO_RENDERER=react SEMIO_BUILD_MODE=prototype S_OS_PORT=6070 S_LOCAL_ONLY=1 S_HUB_URL= SEMIO_VITE_HMR=0` and `SEMIO_VITE_CACHE_DIRECTORY=.cache/b/p/m/vite/state/s-react-prototype` |
| Probe | `claude-fulls-probe.ts <url> <locale> <seconds>` | headless repo-managed Chromium (`.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`), prints body text, console errors, failed requests, screenshot |
| Journey | `claude-fulls-journey.ts <en\|de> <url> <out>` | per locale and per dialect (Transitional, Strict) and per entry (Open File picker, OS drop): open, editor mount (A1 shows the original value, owner app id `s.stdio.xlsx@ecma-376/<dialect>#editor`), edit A1, save (download name must equal the opened name, content read back with the independent XLSX reader), reopen the kept document and check the edit; refusals for a foreign docx renamed `.xlsx`, a non-zip `.xlsx` and an unknown extension through both entries, with the localized text checked against `pickedFileRefusalTextV1` |

The generated session is a reduced S session (three plugins, not 74). The full 74-plugin build is not needed for XLSX and is not feasible on this host; this is a deviation from "full S" and is stated here on purpose. Engines (`surface`, `editor`, `graph`) were published by the pane agent (`claude-pane-engines*.{sh,ts}`) into `wasm-packages`; the Puzzle engine (S `play` facet) was not built.

## Verified so far

- Vite starts in about 6 s on 6070 with the activation receipt for `stdio-office`; `GET /` answers 200.
- The shell boots in headless Chromium (title `semio · os`, Plugin Recovery, Settings, Marketplace, Tasks, Command rows). Space then fails with `plugin module space is unavailable ... descriptor answered HTTP 404`, which is the expected state before the Space module is published. Screenshot: `🗑️generated/claude-full-s/probe-en-US.png`.
- `lazy-activate` prefetch lines (`Original parent Script process invocation required`) are harmless noise: the host-playground prefetch tries to build absent components through the build driver and is refused without an invocation.

## Defects fixed in source (found while building Space for the browser)

Both are stale call sites after an in-flight framework refactor (native snapshot decode takes an `output` slot; operation enums need owned receiving derives and the preparation factory needs `begin_copy_byte_demand`). They broke `cargo build -p semio-hub-space-component`:

1. `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: `decode_sqlite_snapshot_native` now has the four-parameter signature (`output: &mut Option<SnapshotReceivingOutput<Self>>`) and returns `SnapshotReceivingOutcome`, passing `native_owner` and `output` to `decode_sqlite_snapshot_record_native` like the framework Space artifact (`🧰️framework/.../🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`).
2. `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs` (reached through the demo example crate): `HomeConfigMutation`, `RetireLocalStudio` and `ListLocalStudio` gained `RetainedClone`, `NativeReceive` and `DslOwnedReceiving`, and `HomeConfigPreparationFactory` gained `begin_copy_byte_demand`, mirroring the Raster and Process3D configs.

Neither fix has a new test: both are compile-contract migrations covered by the existing store and mutation suites once the crate compiles.

## What blocks completion

1. Space wasm is not built. Attempt history: the managed-gate build (`build-space-stdio.log`) died with the machine restart after 857 compiler artifacts; native attempts 1 to 3 hit the two source defects above; attempt 3 was stuck for 14 minutes in `ld` inside an `access()` call while linking the `dsl_derive` proc-macro dylib (killed by me, cargo failed, no source cause), then attempt 4 queued behind the pane agent's rebuild on the shared target lock. Logs: `🗑️generated/claude-full-s/space-cargo{,2,3,4}.log`.
2. Stdio and Stdio Office wasm come from the pane agent's build (same target directory). Until `semio_hub_stdio_component.wasm` exists, `stdio` cannot be published and the XLSX editors cannot mount. The older captured Office component (`🗑️generated/office-codec-current/stdio-office.wasm`, 21 Nov) can substitute for `stdio-office` only.
3. The products image was detached by the machine restart and later reattached; `modules/` was empty and had to be republished (the Office module I published earlier is gone).

## Finish recipe (once both wasm builds are done)

```sh
T=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR
N=$T/🗑️generated/claude-pane/native/target/wasm32-wasip2/wasm-prototype
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
BUN=$(repo tools list | awk '$1=="bun"{print $NF}')
$BUN $T/claude-fulls-session.ts
$BUN $T/claude-fulls-publish.ts space=$N/semio_hub_space_component.wasm stdio=$N/semio_hub_stdio_component.wasm stdio-office=$N/semio_hub_stdio_office_component.wasm
$BUN $T/claude-fulls-activate.ts s space stdio stdio-office
(cd $T && ./claude-fulls-serve.sh s 6070 &)
$BUN $T/claude-fulls-journey.ts en http://127.0.0.1:6070/
$BUN $T/claude-fulls-journey.ts de http://127.0.0.1:6070/
```

Then the existing gates against the same host: `repo ./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts run --serve http://127.0.0.1:6070/ --entry natural-file --locale en|de --only xlsx --out <dir>` and, from `🌎️hub/📦️packages/🦀️rust`, `repo ./📜️script.ts io-matrix --serve http://127.0.0.1:6070/ --locale en --only stdio-office/xlsx --out <dir>`.

## Journey verdicts

Run 2026-11-25 17:10 (`🗑️generated/claude-full-s/journey-en.log`, `journey-de.log`) against 6070 with the Space, Stdio and Stdio Office wasm published and activated. The shell boots (`ready:s`), the plugin catalog lists, and the stdio-office wasm of that run predates the XML namespace fix, so every workbook open fails with `xlsx: xml in xl/workbook.xml: unbound XML namespace prefix in attribute xmlns:r`. A re-run after the rebuild is pending (see `claude-standalone-pane.md`, blockers).

| Journey | en | de |
| --- | --- | --- |
| Shell boot and locale | pass | pass |
| Open File picker (Transitional, Strict) | fail: no `openArtifactFile` rail row without an open document | fail: same |
| OS file drop (Transitional, Strict) | fail: file-open-failed (xmlns:r defect, fixed in source, wasm pending) | same |
| Subset editors mount | not reached | not reached |
| Edit cell, save keeps name, reopen shows edit | not reached | not reached |
| Refusal notice for a foreign docx, a non-zip and an unknown extension (drop entry) | pass (3 of 3 localized) | pass (3 of 3 localized, German text) |
| Refusal notice (picker entry) | fail: no `openArtifactFile` rail row | fail: same |
| Stdio artifact-acceptance browser gate | not run | not run |
| Hub I/O matrix `stdio-office/xlsx` | not run | n/a (matrix runs `en`) |

## Files

Created: `claude-fulls-inspect.ts`, `claude-fulls-session.ts`, `claude-fulls-publish.ts`, `claude-fulls-activate.ts`, `claude-fulls-serve.sh`, `claude-fulls-probe.ts`, `claude-fulls-journey.ts`, `claude-fulls-cargo.sh` (all in this ticket folder), this file. Source changed: the two Rust files above.
