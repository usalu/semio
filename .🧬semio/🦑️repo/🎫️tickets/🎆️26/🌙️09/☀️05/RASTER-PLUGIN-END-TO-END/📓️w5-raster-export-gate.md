# W5 — the `verify interactivity tool-jobs` raster export law, rewritten for real exporters

Scope: `📜️script.ts` only (the repo-root shared script). No file under `✏️s/🔌️plugins/🖨️raster` was
touched. All paths relative to `/Users/ueli/Documents/semio`. Line numbers are the CURRENT tree.

This closes the item W3 flagged for the coordinator in `📓️w3-raster-io.md` §4 ("⚠️ Pre-existing repo-gate
breakage this run did NOT touch (needs a decision)").

---

## 1. The gate, end to end

One function owns it: `toolJobRasterEnvelopeCallerRetainedExact(store, raster, editor, wasm, plugin)`
(`📜️script.ts:3982`). It has **no doc comment** — the intent has to be reconstructed from the failure
sentence it raises (`📜️script.ts:10367`) and from the shape of the clauses.

Its `raster` argument is a **concatenation of five sources** built at `📜️script.ts:10280`:

1. `…/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
2. `🗿️artifacts/🖨️raster/🦀️.rs`
3. `…/✳️any/🧬️schema/🦀️component.rs` — **does not exist**; `policyReadFileSafe` yields `""` (see §5)
4. `…/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
5. `rasterMountedOutputSerializers` — the eight binary export leaves, joined (`📜️script.ts:10264-10280`)

The single assertion is `📜️script.ts:10367`; the self-tests that exercise it live inside
`toolJobCoverageSelfTests()` (`📜️script.ts:7084-8912`) — positive fixture `retainedRasterCodec`
(`:7982-8163`), positive check `:8163`, hostile mutations `:8164-8312`.

### What the law protected

The clauses on the eight mounted leaves were:

```ts
rasterMountedOutputGuards  === 8   // occurrences of `snapshot.require_empty_output_shell().map_err(str::to_owned)?;`
rasterMountedOutputCallers === 8   // occurrences of `Ok(<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())`
```

Read against the pre-ticket tree, where all eight binary "exporters" were `print_dsl`-under-a-foreign-
extension writers, the invariant is: **a mounted output serializer that has no real encoder must not
silently emit a populated document through its empty shell.** Every one of the eight had to be the
*same* fake writer, and every one of them had to refuse a populated snapshot first. It is the leaf-level
peer of the same "populated output is forbidden" rule the snapshot module enforces at
`…/🧬️schema/📸️snapshot/🦀️.rs:47,261,496,536,546` (those clauses are untouched here and still hold).

### Why it could never pass

Three of the eight listed paths did not resolve, so `policyReadFileSafe` returned `""` for them and the
counters could reach at most **5**, never 8:

| listed (stale) | on disk |
|---|---|
| `🖼️bmp/🔖️v3/✳️any` | `🪟️bmp/🔖️v3/✳️any` |
| `🌳️pdf/🔖️1.4/✳️any` | `📖️pdf/🔖️1.4/✳️any` |
| `📷️jpg/🔖️jfif-1.01/✳️any` | `📸️jpg/🔖️jfif-1.01/♾️any` |

And after W3 the law contradicts the tree outright: real encoders must do the *opposite* of what it demands.

---

## 2. Old law vs new law

| | old | new |
|---|---|---|
| mounted leaves counted | 8, all identical fake writers | 8, split 6 real + 2 typed declines |
| `snapshot.require_empty_output_shell().map_err(str::to_owned)?;` | `=== 8` | `=== 0` |
| `Ok(<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())` | `=== 8` | `=== 0` |
| uniform entry point | (not checked) | `pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {` `=== 8` |
| document composite reached | (not checked) | `raster_composite_image(snapshot)` `=== 5` |
| stdio dialect hop | (not checked) | `semio_image_to_format(&image, ` `=== 5` |
| real byte encoder reached | (not checked) | `semio_s_plugin_stdio::artifacts::…io::encode_<fmt>(` `=== 5` |
| vector container path | (not checked) | `raster_document_json_to_svg(snapshot)` `=== 1` |
| honest decline | (not checked) | `Err(RASTER_(PDF\|DWG)_EXPORT_UNSUPPORTED.to_string())` `=== 2`, plus both `pub const RASTER_*_EXPORT_UNSUPPORTED: &str =` declared |
| missing path | silently `""` | `throw` naming the path |

The new law is closed: `5` pixel leaves + `1` vector leaf + `2` declines `= 8` entry points, so no leaf can
be silently dropped, retargeted, or turned back into a DSL printer without one of the eight counts moving.

The five pixel encoders it pins are the ones actually on disk — `encode_bmp`, `encode_png`, `encode_tiff`,
`encode_jpg`, and gif's `…::gif::standards::v87a::subsets::any::io::encode_gif` — all in `🗄️stdio`, none in
this plugin, which is exactly the property the law is meant to keep.

---

## 3. Changes (file:line)

All in `📜️script.ts`. `git diff --stat` → `40 insertions(+), 12 deletions(-)`.

**Counters** — `📜️script.ts:4010-4015` (six new `const`s, inserted immediately above the two retained ones
at `:4016-4017`, whose regexes are unchanged; only their expected values move).

**Law clauses** — `📜️script.ts:4127-4136`, replacing the two lines `rasterMountedOutputGuards === 8 &&` /
`rasterMountedOutputCallers === 8 &&`:

```ts
    rasterMountedOutputEntryPoints === 8 &&
    rasterMountedPixelComposites === 5 &&
    rasterMountedPixelDialects === 5 &&
    rasterMountedStdioEncoders === 5 &&
    rasterMountedVectorComposites === 1 &&
    rasterMountedTypedDeclines === 2 &&
    raster.includes("pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str =") &&
    raster.includes("pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =") &&
    rasterMountedOutputGuards === 0 &&
    rasterMountedOutputCallers === 0 &&
```

**Stale paths + loud failure** — `📜️script.ts:10268-10270` (the three renames above) and
`📜️script.ts:10274-10278`, where `.map((file) => policyReadFileSafe(root, file))` became a mapper that
throws `[verify interactivity tool-jobs] mounted Raster export serializer "<path>" is missing or empty; the
export law cannot be evaluated against a path that does not resolve.`

**Failure sentence** — `📜️script.ts:10367`, extended with `…, or its eight mounted export serializers are
not the six real format encoders plus two typed RASTER_*_EXPORT_UNSUPPORTED declines`.

**Positive self-test fixture** — `📜️script.ts:8143-8152`, replacing the
`...Array.from({ length: 8 }, () => "pub async fn serialize_bytes(…) { snapshot.require_empty_output_shell()…
print_dsl(snapshot).into_bytes() }")` block with eight distinct bodies that mirror the real leaves: four
templated pixel formats (`bmp`/`png`/`tiff`/`jpg`), the gif 87a-via-89a body, the svg body, and the two
`RASTER_*_EXPORT_UNSUPPORTED` const + decline pairs.

**Hostile self-tests** — `📜️script.ts:8293-8298`. The one obsolete test
(`Raster-mounted-exporter-populated-output-reachability-restoration`) is replaced by six:

| mutation | must be rejected because |
|---|---|
| `…encode_png(&target)` → the old `print_dsl(snapshot).into_bytes()` | encoders 5→4 **and** callers 0→1 |
| `let image = raster_composite_image(snapshot)?;` → `SemioImage::default();` | composites 5→4 (encoder fed something other than the document) |
| `raster_document_json_to_svg(snapshot)?` → a literal `<svg/>` | vector composites 1→0 |
| `Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string())` → `Ok(Vec::new())` | declines 2→1 (a silent empty file instead of a reason) |
| `pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =` → `const DWG_UNSUPPORTED: &str =` | the reason constant is no longer the published one |
| `pub fn serialize_bytes(…) {` → `fn unmounted_serialize_bytes(…) {` | entry points 8→7 (a leaf silently unmounted) |

The four surviving `require_empty_output_shell` self-tests at `📜️script.ts:8299-8302` are untouched — they
guard the snapshot module's own DSL/pack preflights, which are still load-bearing (W3 §4).

`toolJobCoverageSelfTests()`'s reported total keeps its historical literal `398` (`📜️script.ts:8902`); it is
already not a count of the 480 `self-test` throws in that function, so it was left alone rather than guessed at.

---

## 4. Verbatim output

### 4a. The new law against the LIVE tree — `🟦️w5-gate-probe.ts` (kept in this ticket folder)

`policyReadFileSafe` collapses the whole gate into one boolean, so the probe extracts
`toolJobRasterEnvelopeCallerRetainedExact` (and its helper `toolJobStoreInitializerRetainedExact`) out of
`📜️script.ts`, splits the `return (…)` on its per-line `&&`, and evaluates all 195 clauses individually
against the same five sources the real run feeds it.

```
$ bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🟦️w5-gate-probe.ts"
[mounted] 1525 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️.rs
[mounted] 979 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs
[mounted] 727 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs
[mounted] 1083 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs
[mounted] 1155 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs
[mounted] 1123 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs
[mounted] 1019 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs
[mounted] 1278 bytes  …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs

[probe] clauses=195 passing=179 failing=16
  FAIL  raster.includes("fn serialize_empty_owned_map<S: serde::Serializer, V>")
  FAIL  rasterOwnedMapSerdeGuards === 3
  FAIL  wasm.includes("pub struct RasterEnvelopeLoadHandle")
  FAIL  wasm.includes("fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle")
  FAIL  wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)")
  FAIL  wasm.includes("source: &js_sys::Uint8Array")
  FAIL  wasm.includes("let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]")
  FAIL  preflight >= 0
  FAIL  construct > preflight
  FAIL  copy > construct
  FAIL  wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())")
  FAIL  wasm.includes("app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")
  FAIL  wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())")
  FAIL  wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())")
  FAIL  wasm.includes("cancel_artifact_envelope_load(handle.runtime_handle())")
  FAIL  wasm.includes("close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")
```

**All eight mounted paths resolve, and every clause of the new export law passes against the live tree.**
The 16 residual failures are all outside this slice — see §5.

Independent count of the same concatenation (`grep -o … | wc -l`), matching the law's expected values:

```
serialize_bytes-sig: 8
composite:           5
semio_image_to_format: 5
encode calls:        5
  semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::io::encode_gif(
  semio_s_plugin_stdio::artifacts::tiff::io::encode_tiff(
  semio_s_plugin_stdio::artifacts::bmp::io::encode_bmp(
  semio_s_plugin_stdio::artifacts::jpg::io::encode_jpg(
  semio_s_plugin_stdio::artifacts::png::io::encode_png(
json_to_svg:         1
declines:            2
print_dsl callers:   0
mounted guards:      0
```

### 4b. The self-tests that exercise the gate

`toolJobCoverageSelfTests()` runs the raster fixture + all 7 raster export self-tests at `📜️script.ts:8163`
and `:8293-8298`, i.e. inside `bun ./📜️script.ts verify interactivity tool-jobs`, which reaches them at
`:10309` before anything else in the run. **They pass**: the run proceeds past them and dies ~750 lines
later inside `toolJobScalarConfigCohortSelfTests` on an unrelated peer-in-flight rename (see §5):

```
$ bun "./📜️script.ts" verify interactivity tool-jobs
2689 |   const files = new Map<string, string>(fixture.sources.map((file: string) => [file, readFileSync(join(WORKSPACE_ROOT, file), "utf8")]));
                                                                                            ^
ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
    path: "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
 syscall: "open",
   errno: -2,
    code: "ENOENT"

      at <anonymous> (/Users/ueli/Documents/semio/📜️script.ts:2689:86)
      at map (1:11)
      at toolJobScalarConfigCohortSelfTests (/Users/ueli/Documents/semio/📜️script.ts:2689:57)
      at toolJobCoverageSelfTests (/Users/ueli/Documents/semio/📜️script.ts:8910:24)
      at toolJobCoverageRun (/Users/ueli/Documents/semio/📜️script.ts:10309:21)
      at runToolJobCoverage (/Users/ueli/Documents/semio/📜️script.ts:10927:20)
```

The identical stack (same file, same line 2689) is what the run produced **before** any W5 edit, so nothing
here regressed it.

The `--self-test` sub-mode is a *different*, earlier code path (`📜️script.ts:10888-10920`) and never reaches
`toolJobCoverageSelfTests()` at all — it dies first on a pre-existing `📕️norm` / `🖍️draw` scan:

```
$ bun "./📜️script.ts" verify interactivity tool-jobs --self-test
10919 |       if (activation.failures.length > 0) throw new Error(`[verify interactivity tool-jobs] app activation factory proof scan: ${JSON.stringify(activation)}`);
                                                            ^
error: [verify interactivity tool-jobs] app activation factory proof scan: {"owners":50,"customRows":593,"genericRows":0,"failures":["forged bounded reducer factory or compiler witness ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/…"]}
```

### 4c. Negative controls — proving the raster self-tests are load-bearing, not vacuous

Two deliberate, reverted breakages. Both were run in full; both produced the expected failure.

**Control A — the positive fixture is actually checked.** Temporarily shrank the fixture's pixel-format list
from `["bmp", "png", "tiff", "jpg"]` to `["bmp", "png", "tiff"]` (7 entry points instead of 8):

```
error: [verify interactivity tool-jobs] self-test retained-Raster-envelope-route was falsely rejected.
```

**Control B — a hostile mutation is actually checked.** Temporarily weakened
`rasterMountedTypedDeclines === 2` to `>= 1`:

```
error: [verify interactivity tool-jobs] self-test Raster-mounted-exporter-silent-empty-output-instead-of-typed-decline was falsely accepted.
```

Both edits were reverted; `git diff` for `📜️script.ts` contains neither.

---

## 5. Every other `📜️script.ts` gate that touches raster

`grep -c -i raster 📜️script.ts` → 524 hits. Grouped by enclosing top-level function:

| function | hits | touches raster export leaves / `require_empty_output_shell`? |
|---|---|---|
| `toolJobCoverageSelfTests` | 242 | **yes** — the fixture + self-tests for the gate below. Updated by W5. |
| `toolJobRasterEnvelopeCallerRetainedExact` | 202 | **yes** — the gate itself. Updated by W5. |
| `toolJobCoverageRun` | 17 | **yes** — builds the inputs and raises the failure. Updated by W5. |
| `interactivityPreparedRasterProducerFailures` / `…SelfTests` | 56 | **no** — the framework GPU texture-upload pipeline (`PREPARED_RASTER_PAGE_BYTES`, canvas image uploads). Different "raster" entirely; never reads `✏️s/🔌️plugins/🖨️raster`. |
| `interactivityPuzzleFillPreviewJsonSelfTests` | 9 | no — puzzle preview rasterization. |
| `interactivityAuditRun` | 8 | no. |
| `interactivityAllAppDiscoverySelfTests` | 5 | no. |
| `policyDeclarativeRegistrationBreaches`, `policyPascalAppStructName`, `policyDslCompleteTypeNames`, `policyStdioCodecIdUniquenessBreaches`, `policyMutationVocabularyBreaches`, `policyInferenceImplPresenceBreaches`, `policyInferenceFamilyBreaches` | 11 | no — naming/registration policies. `📜️script.ts:26836` allowlists raster's DSL type names (`DwgColor`, …, `rasterize_svg_to_png_base64`); unrelated to the export law. |

Cross-check by literal path: the only lines in the whole script naming `✏️s/🔌️plugins/🖨️raster/…` are
`📜️script.ts:10264-10282` — the block W5 fixed. **There is no second gate to update.**

### Residual failures in this same gate — NOT W5's slice, flagged for the coordinator

The probe (§4a) shows 16 clauses still failing. None is an export-leaf clause; all are pre-existing rot from
other tickets:

1. **12 `wasm.…` clauses + the 3 `preflight`/`construct`/`copy` ordering clauses.** `rasterWasm` is read from
   `…/✏️editor/🌉️wasm/🦀️component.rs` (`📜️script.ts:10282`), which **does not exist**. The file on disk is
   `…/✏️editor/🌉️wasm/🦀️.rs`, and it is a tombstone: *"The wasm-bindgen `RasterArtifactVcs`/
   `RasterEnvelopeLoadHandle` bridge that used to live here was deleted … see
   `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`."* So the path is stale **and** the
   law is dead — those 15 clauses describe code that was deliberately removed and cannot come back.
2. **`fn serialize_empty_owned_map<S: serde::Serializer, V>` and `rasterOwnedMapSerdeGuards === 3`.** The
   serde attributes moved to `…/✳️any/🧬️schema/🦀️.rs:25` as `#[cfg_attr(test, serde(serialize_with = …))]` —
   a file this gate does not read — during the serde-elimination sweep.
3. **`…/✳️any/🧬️schema/🦀️component.rs` (`📜️script.ts:10280`) does not exist either**, so one of the five
   concatenated sources is permanently `""`. It contributes no clause today, but it is silent rot of the same
   kind; left as-is because the loud-failure requirement was scoped to `rasterMountedOutputSerializers`.

Deciding what the post-deletion wasm law should say (delete the clauses, or point them at the surviving
`🗺️surface` bridge) belongs to whoever owns that 09/01 ticket's follow-up, not to this ticket's export slice.

### Unrelated blockers that stop `verify interactivity tool-jobs` from reaching a verdict today

Both are other sessions' in-flight work, both reproduce identically before and after the W5 edits:

- `📜️script.ts:2689` hard-codes `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`;
  a peer renamed that subset `✳️any` → `🌐️any` mid-run, so `readFileSync` throws.
- `--self-test` mode: `toolJobFactoryProofActivationScan` reports 48 "forged bounded reducer factory or
  compiler witness" rows across `📕️norm`'s 16 standards and `🖍️draw`.

---

## 6. Exact files touched by W5

Updated:

- `📜️script.ts` (only file changed; +40 −12 — regions `:4010-4015`, `:4127-4136`, `:8143-8152`,
  `:8293-8298`, `:10268-10278`, `:10367`)

Created (kept, per the ticket rules — it is an input/verification script, not generated output):

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🟦️w5-gate-probe.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️w5-raster-export-gate.md` (this file)

Not touched: everything under `✏️s/🔌️plugins/🖨️raster` (W1/W2/W3 own those), and every non-raster region of
`📜️script.ts`.
