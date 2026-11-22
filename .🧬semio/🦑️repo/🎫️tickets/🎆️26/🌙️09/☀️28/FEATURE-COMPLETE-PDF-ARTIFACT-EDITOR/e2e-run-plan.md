# 🧭 E2E Run Plan: PDF Artifact Editor

Prepared read-only, then the harness references were repaired (see `e2e-harness-repair.md`). No build, browser, or server was started. Goal IDs (G#) refer to `pdf-goal-gap-audit.md`.

Status at planning time: no mounted PDF journey has a passing run on the rebuilt component (G8), and the Stdio companion component is missing (G9a). Expect steps 1 and 2 to be blocked until G9a, G1 and G2 are resolved. The plan is still the correct order to run once they are.

## 0. Conventions and preflight

Run everything from the repository root, because the browser scripts resolve `🧰️framework/...` and the native and browser fixtures relative to `process.cwd()`. Use the `repo` binary only. Do not use `repo dashboard`.

```sh
REPO=/Users/ueli/Documents/semio
T="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR"
S="$REPO/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures"
cd "$REPO"
bun() { repo tools exec bun -- "$@"; }
py()  { repo tools exec --light uv -- run --offline --no-project --python 3.12 --with pymupdf==1.26.5 --with pillow==13.0.0 -- python "$@"; }
```

Preflight:

```sh
command -v repo && repo --version
ls "$REPO/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"   # generated catalog (must exist)
ls "$S/✏️edit-existing-pdf/2️⃣two-pages.pdf" "$S/📄️natural-file-edit/🔣️.json"
lsof -nP -iTCP:6260 -sTCP:LISTEN || echo "6260 free"               # suite refuses to start if taken
```

Pass signal: the catalog and both fixtures exist, and the dev-host port is free. Gap: G9e (compiler storage headroom) can block every build, so check free disk before step 1.

Port: 6260 (`reactPort` of the `stdio-pdf-1-7` variant, `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧪️tests/🚪️route/🧫️fixtures/🔣️.json`). The suite and every ticket script read it through `📜️e2e-harness.ts`. Pins, also in the harness: `pymupdf==1.26.5` (locked in the staffage oracles `pyproject.toml` and `uv.lock`) and `pillow==13.0.0` (the lock's Python 3.12 resolution). See F7 and F10.

## 1. Build and publish components

Use the authored scripts rather than the raw targets. The scripts set the prototype profile, the capability envelope and the `--exclude-dependencies` selection. They must run in this order: catalog, then session, then components, then serve.

1a. Regenerate the catalog.

```sh
bun "$T/📜️pdf-browser-direct.ts" catalog
```

Pass: exit 0, `🤖️generated/🔌️plugins.json` is rewritten and contains the `stdio-pdf-1-7` app. Gap: G9d if Hub assembly fails later.

1b. Republish the session receipt.

```sh
bun "$T/📜️pdf-browser-direct.ts" session
```

Pass: exit 0, a two-plugin receipt (`stdio`, `stdio-pdf`) is written. Target: `@semio-tech/plugin-registry:session-stdio-pdf-1-7`, constructed by naming convention (flag F8).

1c. Build and publish the Stdio companion, then the PDF component (the PDF playground loads Stdio as a runtime dependency).

```sh
bun "$T/📜️pdf-browser-direct.ts" components-stdio   # currently expected to fail: G9a
bun "$T/📜️pdf-browser-direct.ts" components-pdf
# or, both at once:
bun "$T/📜️pdf-browser-direct.ts" components
```

Pass: both phases exit 0 and the `prototype` artifacts (Editor, Surface, NodeGraph, font, Stdio, PDF) are staged, as shown by the artifact reader messages. Gap: G9a (Stdio key `ff2dca26f35d2cadeb93c740` missing from product components) blocks this step. G9c (Cargo registry drift) and G9d (E0460 on the alternate Hub lane) can also block it.

Warm single-component alternative for the PDF component only (it cannot publish Stdio):

```sh
repo run semio-hub-stdio-pdf-component:component --configuration=prototype
```

Pass: linked `.wasm` for the PDF component. Use only after the scoped build has warmed the store.

1d. Native runtime (needed for step 4 only).

```sh
bun "$T/📜️native-wgpu-run.ts" host      # @semio-tech/framework-renderer-wgpu:native-build, gap G9b
bun "$T/📜️native-wgpu-run.ts" modules  # publishes native modules, gap G9a
```

Pass: `[DEBUG] Verified signed native executable <size> <mtime> <path>` in the log. Gap: G9b (WGPU link is unconfirmed after E0425).

Alternative full activation (slow; use only if the component steps above cannot be scoped):

```sh
S_LOCAL_ONLY=1 repo run @semio-tech/framework-os-dev:activate-stdio-pdf-1-7-react-prototype --output=stream
```

Pass: the activation receipt is written and `http://127.0.0.1:6260/` serves once step 2 starts. The `s` shell alternative is `activate-s-react-prototype`, which the mounted report says takes hours (flag F8).

## 2. Start the dev host

The mounted suite starts its own serve process in step 3. Run this manually only for the browser-only checks (inspection, the `image-authoring-cancel-browser.ts` run, the Hub check-in).

```sh
S_LOCAL_ONLY=1 S_OS_PORT=6260 S_DATA_DIR="$T/🗑️generated" SEMIO_VITE_HMR=0 \
  repo tools exec bun -- "$T/📜️pdf-browser-direct.ts" serve
```

Pass: `curl -fsS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:6260/` prints `200`. The suite gives up after 300 s and reports `Browser activation did not start; see serve.log`. Gap: `serve` runs with `--exclude-dependencies`, so it fails when step 1 has not staged the components.


## 3. Mounted journeys (browser, Chromium, real shell)

### 3a. Recommended single entry: the mounted suite

```sh
bun "$T/📜️pdf-browser-mounted-suite.ts" run-$(date +%Y%m%d-%H%M)              # all 15 cases
bun "$T/📜️pdf-browser-mounted-suite.ts" run-$(date +%Y%m%d-%H%M) starter rotation   # subset
```

The run name must be fresh (the output folder is created with `mkdirSync` and must not exist). Output goes to `$T/🗑️generated/mounted-<run>/`. Each case writes `<case>.log` (the `wx` log) and `serve.log`. Pass signal: the suite exits 0. Each `[DEBUG] <name> exit 0` line is a pass, and each `<name>-oracle exit 0` line is an independent check that passed. The final failure count is printed. The suite now generates and verifies its own fixtures (F1 is fixed).

The table below lists each case, the command it runs, the independent check and what it proves. Case names are the suite's names. They can also be run one at a time through `3b`.

| Case (journey) | Input | Smoke mode | Independent check after download | Proves gap | Expected pass signal |
|---|---|---|---|---|---|
| `starter` (Open) | `2️⃣two-pages.pdf` | `starter` | `📜️pdf-starter-export-oracle.py pdf-mounted-starter.pdf` | G1, G2, G3, G8 | Shell opens the file, export is a 2-page PDF |
| `rotation` (default rotation, Save, reopen) | `2️⃣two-pages.pdf` | default | `📜️mounted-download-oracle.py <original> pdf-mounted.pdf` (page 2 at 90, page 1 at 0, text/title/author kept) | G1, G2, G8, G10 | Oracle prints a pass, page 2 rotation = 90 |
| `content` (edit-content: text, vector X, image width, title, CropBox, TrimBox, undo/redo) | `pdf-mounted-content-input.pdf` (oracle-generated) | `edit-content` | `📜️mounted-content-edit-oracle.py <input> pdf-mounted-content.pdf` | G8, G10, G15 (partial) | Oracle prints `MuPDF verified the saved text, rectangle, RGB image, page boxes and metadata` |
| `style` (font size, vector fill, language, subject) | same content input | `style-properties` | `📜️pdf-style-properties-oracle.py pdf-mounted-style-properties.pdf` | G8, G12 (partial) | Oracle passes; author and page-2 raster unchanged |
| `page-history` (insert, move, remove page, six undo/redo transitions) | `pdf-page-history-source.pdf` (`📜️pdf-page-history-oracle.py build`) | `page-history` | `📜️pdf-page-history-oracle.py verify pdf-mounted-page-history.pdf` | G15, G8, G10 | Oracle checks page order, 216x216 and 240x260 sizes, text, untouched raster |
| `png` (image authoring, see 3c) | `png/image-source.pdf` (from `--self-test`) | `image-authoring-browser.ts` | `📜️pdf-raw-png-history-oracle.py png --check` | G14, G8 | Oracle checks shared-resource isolation, pixels and alpha |
| `large-vector` (49,279 bytes, 48 pages) | `large-vector-pdf.pdf` (`generate` by the suite) | `large-vector` | `📜️large-vector-pdf-oracle.py verify` of the input (the case saves no download) | G5, G6, G4, G8 | Page 48 selectable and inspector visible, saved output opens |
| `large-text` (33,038 bytes, 48 pages) | `large-mounted-pdf.pdf` (`generate`) | `large-text` | `📜️large-mounted-pdf-oracle.py verify` of the input (no download) | G5, G6, G8 | Text `Text: Page 48 of 48` readable in Text Layer |
| `large-images` (38,096 bytes, 4 pages) | `🎞️controlled-images.pdf` (`🎞️fixture.py`) | `large-images` | `🎞️fixture.py pdf-mounted-large-images.pdf` (verify mode) | G5, G6, G14 (partial), G8 | Oracle verifies four pages, images and positions |
| `cancel` (cancel-large of the 48-page vector file) | `large-vector-pdf.pdf` | `cancel-large` (uses `🧪️mounted-cancel.json`) | `verify` of the input plus `📜️mounted-notice-oracle.py` on `pdf-mounted-cancel-console.json` (`shell.documentTransfer.open-cancelled`) | G16, G4, G8 | Notice `shell.documentTransfer.open-cancelled`, original windows unchanged, retry opens the file |
| `reject-invalid` (malformed PDF) | `invalid.pdf` (`📜️pdf-invalid-open-oracle.py generate`) | `reject-invalid` | `verify` (MuPDF refuses it) plus the notice oracle on `pdf-mounted-reject-invalid-console.json` (`file-open-failed`, no `import-and-retirement-failed`) | G8, G11 | Notice `shell.documentTransfer.file-open-failed` |
| `image-cancel` (live PNG decode cancel and retry, see 3c) | `image-source.pdf` (derived by the script) | `image-authoring-cancel-browser.ts` | `📜️image-cancel-oracle.py` (three saves byte-equal, MuPDF structure) | G14, G16, G8 | Before, after and undone saves identical |
| `first-archive` (G17, see 3e) | `2️⃣two-pages.pdf` | `📜️pdf-first-archive-browser.ts` | `📜️first-archive-oracle.py` (FNV64, framing, SHA-256 folder, SQLite) | G17, G7 | Exactly one new `events.semio` decodes |
| `hub-check-in` (two Browser actors, see 3d) | generated PNG | `📜️pdf-hub-runtime.ts` -> `📜️pdf-hub-image-check-in.ts` | `📜️pdf-raw-png-history-oracle.py --hub-export` | G7, G8 | One painted `SemioImage` with exact RGB and alpha |
| `segmented-source` (extra, not in the user list) | `pdf-segmented-source.pdf` (`📜️pdf-segmented-source-oracle.py generate`) | `segmented-source` | `📜️pdf-segmented-source-oracle.py verify <source> pdf-mounted-segmented-source.pdf` | G5, G10 | Oracle checks rotation law |

### 3b. Run one journey on its own

Use these only to rerun a single journey after the suite. Each follows the same pattern and needs the dev host from step 2 on port 6260. Create the inputs first, as the suite does (`generate` refuses to replace an existing file):

```sh
py "$T/📜️mounted-content-edit-oracle.py" "$T/🗑️generated/pdf-mounted-content-input.pdf"
py "$T/📜️pdf-page-history-oracle.py" build                       # writes 🗑️generated/pdf-page-history-source.pdf
py "$T/📜️large-mounted-pdf-oracle.py" generate "$T/🗑️generated/large-mounted-pdf.pdf"
py "$T/📜️large-vector-pdf-oracle.py" generate "$T/🗑️generated/large-vector-pdf.pdf" "$S/📄️natural-file-edit/🔣️.json"
py "$T/🎞️fixture.py"                                               # writes 🗑️generated/🎞️controlled-images.pdf
py "$T/📜️pdf-segmented-source-oracle.py" generate "$T/🗑️generated/pdf-segmented-source.pdf"
py "$T/📜️pdf-invalid-open-oracle.py" generate "$T/🗑️generated/invalid.pdf"
```

Then the journeys:

```sh
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$S/✏️edit-existing-pdf/2️⃣two-pages.pdf" "$T/🗑️generated/manual-open" starter
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$S/✏️edit-existing-pdf/2️⃣two-pages.pdf" "$T/🗑️generated/manual-rotation"
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/pdf-mounted-content-input.pdf" "$T/🗑️generated/manual-content" edit-content
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/pdf-mounted-content-input.pdf" "$T/🗑️generated/manual-style" style-properties
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/pdf-page-history-source.pdf" "$T/🗑️generated/manual-history" page-history
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/large-vector-pdf.pdf" "$T/🗑️generated/manual-large-vector" large-vector
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/large-mounted-pdf.pdf" "$T/🗑️generated/manual-large-text" large-text
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/🎞️controlled-images.pdf" "$T/🗑️generated/manual-large-images" large-images
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/large-vector-pdf.pdf" "$T/🗑️generated/manual-cancel" cancel-large
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/invalid.pdf" "$T/🗑️generated/manual-reject" reject-invalid
bun "$T/📜️pdf-browser-smoke.ts" http://127.0.0.1:6260/ "$T/🗑️generated/pdf-segmented-source.pdf" "$T/🗑️generated/manual-segmented" segmented-source
```

Pass signal: exit 0, and the case writes `pdf-mounted-<case>.pdf` (where a download is expected), `pdf-mounted-<case>-console.json` and, for `cancel-large`, the notices. Note: the `reject-invalid` input path shown above must be created first by `📜️pdf-invalid-open-oracle.py`. The suite writes it to `mounted-<run>/invalid.pdf`.

### 3c. Image authoring (PNG insert, replace, cancel, retry)

The suite runs both the PNG picker journey (`png`) and the cancel journey (`image-cancel`).

Prepare the sources (the same way the suite does it):

```sh
py "$T/📜️pdf-raw-png-history-oracle.py" "$T/🗑️generated/png" --self-test
cp "$T/🗑️generated/png/oracle-proof/image-source.pdf" "$T/🗑️generated/png/image-source.pdf"
```

Pass: the self-test prints its checks and `large-picked.png` is written in `png/`.

Insert, replace, undo, redo, inline and archive journey:

```sh
bun "$T/image-authoring-browser.ts" http://127.0.0.1:6260/ "$T/🗑️generated/png" "$T/🗑️generated/png/large-picked.png"
py "$T/📜️pdf-raw-png-history-oracle.py" "$T/🗑️generated/png" --check
```

Pass: `image-inserted.pdf`, `image-replaced.pdf`, `image-undone.pdf`, `image-redone.pdf`, `image-inline.pdf`, `image-cancelled.pdf` and the archive reload files exist, and the `--check` oracle exits 0. Gap: G14, G8.

Cancel and retry journey. `image-authoring-cancel-browser.ts` derives `image-source.pdf` itself with the PNG history oracle self-test when the file is missing, so no copy step is needed:

```sh
bun "$T/image-authoring-cancel-browser.ts" --fixture-only "$T/🗑️generated/image-cancel"    # no browser, writes decode-cancellation.png
bun "$T/image-authoring-cancel-browser.ts" http://127.0.0.1:6260/ "$T/🗑️generated/image-cancel"
py "$T/📜️image-cancel-oracle.py" "$T/🗑️generated/image-cancel"
```

Pass: `decode-cancellation-before.pdf`, `-after.pdf` and `-undone.pdf` are byte-equal (script and oracle). Gap: G14 (decode path), G16 (cancel of a live decode), G8.

### 3d. Hub check-in (two Browser actors, Hub checkpoint)

The Hub local start command is `repo run os-hub:dev` (port 8787, `🌎️hub/📦️packages/🦀️rust/📋️project.json`). The authenticated two-user lane is `repo run os-hub:dev-collaboration`, and `📜️pdf-hub-runtime.ts` runs that lane in process: it starts the Hub, provisions two credentials, starts two shell dev servers on scanned ports, and then runs `📜️pdf-hub-image-check-in.ts` against them. The suite case `hub-check-in` wires it, so no manual Hub or shells are needed and it does not need the 6260 server (it is skipped when only `hub-check-in` is selected).

```sh
bun "$T/📜️pdf-browser-mounted-suite.ts" run-hub-$(date +%Y%m%d-%H%M) hub-check-in
# manual form:
py "$T/📜️pdf-raw-png-history-oracle.py" "$T/🗑️generated/hub-png"
bun "$T/📜️pdf-hub-runtime.ts" "$T/🗑️generated/hub-checkin" "$T/🗑️generated/hub-png/large-picked.png"
py "$T/📜️pdf-raw-png-history-oracle.py" "$T/🗑️generated/hub-png" --hub-export "$T/🗑️generated/hub-checkin/hub-reopened.pdf"
```

Pass: `hub-checkin/hub-reopened.pdf` and `hub-original-media.json` are written and the `--hub-export` oracle confirms one painted `SemioImage` with exact RGB samples and the alpha mask. Gap: G8, G7 (Hub persistence). It still needs the Hub bootstrap bundle and a built Hub host, so G9d can block it.

### 3e. First archive (new-document persistence, G17)

The suite case `first-archive` is the mounted driver. `📜️pdf-first-archive-browser.ts` opens `2️⃣two-pages.pdf` in the mounted shell, waits for the editor window, and lists the new `<S_DATA_DIR>/os/local-documents/<sha256(document id)>/.semio/events.semio` files. `📜️first-archive-oracle.py` then decodes them independently (physical FNV64, kind 4, archive framing, folder equals SHA-256 of the record key, 4 MiB limit, SQLite readback). It is expected to fail until the G17 prerequisites (shared guest first SPR producer, dev/native folder authority) are integrated. The unmounted laws stay available:

```sh
bun "$T/📜️pdf-browser-mounted-suite.ts" run-fa-$(date +%Y%m%d-%H%M) first-archive
bun "$T/browser-first-archive-law.ts"
bun "$T/browser-first-archive-http-law.ts"
py "$T/browser-first-archive-oracle.py"
```

Pass: the driver prints `Mounted open wrote one first-genesis folder archive` and the oracle prints one `holds one valid kind-4 record` line. Invocation arguments were not verified for the three `native-*` checks (`native-first-archive-disk-check.py`, `native-first-genesis-check.py`, `native-archive-readback-check.py`); read each file's `argv` before running. Gap: G17, G7.

### 3f. Shared Stdio natural-file acceptance row (optional)

```sh
bun "$REPO/✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts" run --serve http://127.0.0.1:6260/ --entry natural-file --locale en --only pdf --out "$T/🗑️generated/artifact-acceptance"
```

Pass: every selected row passes every step (the script exits non-zero otherwise). Requires the dev host from step 2 on 6260. Gap: G8, G10 (natural-file Open/Save/reopen). Flag F7 applies.

## 4. Native desktop smoke

Gap: G18. Also G9a and G9b. Run the wrapper phases in order (1d), then the smoke.

```sh
repo run @semio-tech/framework-os-dev:run-stdio-pdf-1-7-native-prototype -- --smoke
# equivalent wrapper form, after host and modules have run:
bun "$T/📜️native-wgpu-run.ts" smoke
```

Pass: the process exits 0 without an error notice. The exact success string was not verified in the ticket (F9). The wrapper sets `S_DATA_DIR` to `$T/🗑️generated/native-wgpu-save/desktop-data`. Prerequisite: a materialized native runtime and renderer. The mounted report states none is staged on this host.

## 5. Independent oracles that validate outputs

| Oracle | Runner | Validates | Gap |
|---|---|---|---|
| `📜️mounted-download-oracle.py` | `py` (PyMuPDF 1.26.5) | Rotation download vs an independently rotated reference | G8, G10 |
| `📜️mounted-content-edit-oracle.py` | `py` | Content edit download: text, rectangle, RGB image samples, boxes, author | G8 |
| `📜️pdf-style-properties-oracle.py` | `py` | Style properties download | G8, G12 |
| `📜️pdf-page-history-oracle.py build/verify` | `py` | Page order, sizes, text, untouched raster | G15 |
| `📜️pdf-raw-png-history-oracle.py` (`--self-test`, `--check`, `--hub-export`) | `py` (PyMuPDF and Pillow) | PNG history pixels, alpha, placement, Hub export | G14, G8, G7 |
| `📜️pdf-starter-export-oracle.py` | `py` | Open-and-export of the starter PDF | G1, G2 |
| `📜️pdf-invalid-open-oracle.py` | `py` | Creates the malformed input for `reject-invalid` | G11 |
| `📜️pdf-segmented-source-oracle.py` | `py` | Segmented source generate and verify | G5, G10 |
| `📜️large-mounted-pdf-oracle.py` | `py` | Creates and self-checks the 33,038-byte, 48-page large-text fixture | G5 |
| `📜️large-vector-pdf-oracle.py` | `py` | Creates and self-checks the 49,279-byte, 48-page vector fixture (F1) | G5, G16 |
| `🎞️fixture.py` | `py` | Creates the controlled-images PDF and verifies a saved file | G14 |
| `📜️pdf-base14-mupdf-oracle.py` | `py` | Base-14 outline check (54 glyphs) | G12 |
| `browser-first-archive-oracle.py`, `native-first-archive-disk-check.py`, `native-first-genesis-check.py`, `native-archive-readback-check.py` | `py` | First-archive and native archive readback | G17, G7 |
| `large-reopen-check.py` | `py` | Bounded archive ingress, 5.3 MB fixture (ingress only) | G7 |
| Native Rust tests (lopdf reference) | `repo cargo` / registered test | Native registered PDF test | G19 |

Native and browser-side Rust and TypeScript laws (`natural-pdf-*-law.mjs`, `pdf-*-law.rs`) are self-checks and do not replace the oracles above. Run them with `repo tools exec bun -- <file>`.

Pass signal for oracles: each prints its pass line and exits 0. Any failed assertion stops the run and must not be treated as a pass.

## 6. Flags: broken or missing references

F1. FIXED. `📜️large-vector-pdf-oracle.py` now has explicit `generate <file> <spec>` (exclusive create, then verify) and `verify <file> <spec>` modes. `📜️large-mounted-pdf-oracle.py` and `📜️pdf-invalid-open-oracle.py` follow the same `generate`/`verify` contract, so no oracle can overwrite a file it is asked to check.

F2. FIXED. The suite now runs oracle steps for `large-vector`, `large-text`, `cancel` and `reject-invalid`. These smoke modes save no download, so the oracles verify the independent input fixture and, for `cancel` and `reject-invalid`, the recorded shell notices through `📜️mounted-notice-oracle.py`. No saved-output oracle exists for them because the browser saves none.

F3. FIXED. `image-cancel` (with `📜️image-cancel-oracle.py`), `hub-check-in` and `first-archive` are suite cases.

F4. FIXED. `image-authoring-cancel-browser.ts` derives `image-source.pdf` itself through the PNG history oracle self-test when it is missing.

F5. RESOLVED. Hub local start: `repo run os-hub:dev` (8787); the authenticated two-user lane is `os-hub:dev-collaboration`, run in process by `📜️pdf-hub-runtime.ts`, which the suite calls (3d). Not executed; the Hub host build can still hit G9d.

F6. FIXED as a driver, UNPROVEN as a run: `📜️pdf-first-archive-browser.ts` plus `📜️first-archive-oracle.py` (3e). G17 itself stays open until it passes.

F7. FIXED. One port: 6260, read from the `stdio-pdf-1-7` variant fixture by `📜️e2e-harness.ts`. The 6070 `s` shell references were removed; the natural-file acceptance row now targets 6260. The suite refuses to start if 6260 is already served.

F8. Target names. `serve-stdio-pdf-1-7-react-prototype`, `session-stdio-pdf-1-7`, `activate-stdio-pdf-1-7-react-prototype`, `dev-stdio-pdf-1-7-react-prototype` and `run-stdio-pdf-1-7-native-prototype` are built from naming conventions (`activate-<variant>-react-prototype`, `run-<variant>-native-<mode>`) and are not literal strings in tracked config. `dev-s-wgpu-prototype` (cited in the mounted report) also has no literal match. The MCP `repo show_targets` call failed with `no sealed generation`. Verify with `repo show project <name>` or `repo run ... --graph-policy=fresh` before step 1.

F9. Native smoke success string. The pass text of `run-stdio-pdf-1-7-native-prototype -- --smoke` was not found in the ticket. Use exit code 0 plus the absence of error notices as the signal until the native run is observed.

F10. FIXED. One pin in `📜️e2e-harness.ts`: `pymupdf==1.26.5` (staffage oracles `pyproject.toml`/`uv.lock`, the only repo lock with PyMuPDF for Python 3.12 besides the authoring package's `2.1`) and `pillow==13.0.0` (that lock's resolution for Python >= 3.11). The mounted report's 1.24.14 runs predate this; re-run the oracles on the pin and treat version-dependent mismatches as oracle questions, not passes. `🔒️dependencies.json` and the root `pyproject.toml`/`uv.lock` carry no PyMuPDF.

F11. The `serve` phase of `📜️pdf-browser-direct.ts` runs with `--exclude-dependencies`. If step 1 did not stage the components, it fails after 300 s with `Browser activation did not start`.

F12. Clean references (checked, present): all oracles and fixtures listed in the suite, `🔣️mounted-*.json`, `🧪️mounted-cancel.json`, `🎞️fixture.json`, `image-authoring.json`, `📜️pdf-browser-smoke.ts`, `📜️pdf-browser-direct.ts`, `📜️pdf-browser-mounted-suite.ts`, `image-authoring-browser.ts`, `image-authoring-cancel-browser.ts`, `📜️pdf-hub-image-check-in.ts`, `📜️pdf-hub-runtime.ts`, `📜️e2e-harness.ts`, `📜️pdf-first-archive-browser.ts`, `📜️native-wgpu-run.ts`, and the Stdio fixtures (`2️⃣two-pages.pdf`, `📄️natural-file-edit/🔣️.json` with `largeVector` 49,279 bytes). Gap-audit names without the emoji prefix (for example `pdf-base14-mupdf-oracle.py`, `pdf-close-store-work.ts`, `native-pdf-open-and-history-gate.md`) resolve to the prefixed files in the ticket.

## 7. Gap coverage map

| Gap | Covered by | Status |
|---|---|---|
| G1 Open traps | `starter`, `rotation` | Not run |
| G2 Lifecycle refusal | `starter`, `rotation`, `cancel` | Not run |
| G3 Small-file stall | `starter` | Not run; trace via `SEMIO_PDF_TRACE_OPEN=1` |
| G4 Natural-source large path | `large-vector`, `cancel`, `segmented-source` | Not run |
| G5 Contiguous decoder | `large-vector`, `large-text`, `large-images`, `segmented-source` | Not run |
| G6 Reserved-job ceilings | `large-vector`, `large-text`, `large-images` | Not run |
| G7 Archive above 4 MiB | `3d` (Hub), `3e` (first archive, laws only) | Not run |
| G8 No accepted mounted run | All mounted cases | Not run |
| G9 Build blockers | Steps 1a to 1d | Blocking |
| G10 Save | `rotation`, `content`, `page-history`, `segmented-source` | Not run |
| G11 Import barriers | `reject-invalid` (malformed only) | Not run |
| G12 Base-14 fonts | `style` (partial), base-14 oracle | Not run |
| G13 Placeholder appearance | None | No journey |
| G14 Large images | `png`, `3c` cancel, `large-images` (partial) | Not run |
| G15 Page structure and history | `page-history`, `edit-content` (partial) | Not run |
| G16 Native and browser cancel | `cancel`, `3c` cancel (decode) | Not run |
| G17 First archive | `first-archive` suite case, `3e` laws | Driver written, not run |
| G18 Native smoke | Step 4 | Blocked by G9a and G9b |
| G19 Native durable and reopen | Step 5, native Rust tests | Not run |
| G20 Canvas harness | None | No journey |
| G21 Retirement regression | None | No test |
