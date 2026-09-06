# W5b — the 16 residual clauses of `toolJobRasterEnvelopeCallerRetainedExact`, rewritten for the tree that exists

Scope: `📜️script.ts` only. No file under `✏️s/🔌️plugins/🖨️raster` was touched (W4 owns it). All paths
relative to `/Users/ueli/Documents/semio`; line numbers are the tree at the end of this run.

Continues `📓️w5-raster-export-gate.md` §5 ("Residual failures in this same gate — NOT W5's slice"). W5
rewrote the eight export-leaf clauses; this note closes the remaining 16 plus the stale
`🧬️schema/🦀️component.rs` concat source.

---

## 1. What the residuals turned out to be

Two independent pieces of rot, from two different repo-wide sweeps:

**(a) The serde sweep.** `serialize_empty_owned_map` — the free function whose whole job was to emit an
*empty* map for a `RasterOwnedMap` field under `#[serde(serialize_with = …)]` — no longer exists
anywhere in the repo (`grep -rn "fn serialize_empty_owned_map" --include='*.rs'` → 0 hits). Its
first-party successor is in `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:337-352`:

```rust
impl<V> dsl::ToValue for RasterOwnedMap<V> {
    fn to_value(&self) -> dsl::DslValue {
        assert!(self.is_empty(), "Populated Raster owned map serialization is forbidden; …");
        dsl::DslValue::Object(Vec::new())
    }
}
impl<V> dsl::FromValue for RasterOwnedMap<V> { … Err(dsl::ValueError::new("Raster maps require the retained page decoder")) … }
```

The `=== 3` was the count of `#[serde(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map")]`
attributes. `git grep -n … ede955d5a2` shows the three sites it covered — one per `RasterOwnedMap`-typed
**field** in a derived type:

| then (`🦀️component.rs`, commit `ede955d5a2`) | now |
|---|---|
| `🧬️schema/🦀️component.rs:23` | `🧬️schema/🦀️.rs:26` `pub assets: RasterOwnedMap<RasterAssetChild>` (`RasterArtifact`) |
| `📸️snapshot/🦀️component.rs:40` | `📸️snapshot/🦀️.rs:40` `pub assets: RasterOwnedMap<RasterAssetChild>` (`RasterSnapshot`) |
| `🦀️component.rs:503` | `🦀️.rs:495` `params: RasterOwnedMap<dsl::DslValue>` (`RasterLayerNode::Adjustment`) |

So the invariant is unchanged — *every* owned-map-typed field reaches output only through a guard that
refuses a populated map — but it is now carried by one blanket impl plus the derives, not by three
per-field attributes. The count that still means something is the number of owned-map field sites.

**(b) The `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` sweep.** The
wasm-bindgen bridges were deleted from raster, gismap, presentation, writer and jack; the file the gate
reads (`…/✏️editor/🌉️wasm/🦀️component.rs`) never existed under that name anyway, and the file that does
exist (`…/✏️editor/🌉️wasm/🦀️.rs`) is a five-line tombstone.

**The paged-ingress law itself is very much alive**, so none of those clauses may simply be deleted:

* `store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` = 4 096 (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:7753`).
* `begin_/preflight_/construct_and_admit_/admit_/seal_/advance_/cancel_artifact_envelope_load`,
  `acknowledge_artifact_store_replacement`, `maintenance_step`, `close_step` all still exist on
  `PluginApp`/`VcsArtifactApp` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19801-19830`, …).
* Two live shapes satisfy it today:
  * **browser bridge** — `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🌉️wasm/🦀️.rs` and the 5d twin
    still carry the exact `preflight → construct_and_admit → js_sys::Uint8Array` route the old raster
    clauses described, verbatim.
  * **editor cohort** — plugins with no browser bridge exercise the same route from their own
    `✏️editor/🦀️.rs` test cohort: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🦀️.rs:1103-1155`
    (`admit_gis_map_envelope` / `drive_gis_map_live_load` / submit-pump-ack / cancel). Jack, Writer,
    Process3d, Generation2d/3d and Drawing have the same.

Raster has **neither**. Its editor contains no `begin_artifact_envelope_ingress` at all. Per the
instruction not to weaken a law just because raster stopped satisfying it, the 15 clauses were
**retargeted at the editor cohort** (the shape a bridge-less plugin must use) and now fail loudly —
see §5, gap 2.

---

## 2. Clause-by-clause

| # | old clause | what it protected | where that lives today | new clause |
|---|---|---|---|---|
| 1 | `raster.includes("fn serialize_empty_owned_map<S: serde::Serializer, V>")` | one dedicated output routine for owned maps that can only emit an empty map | `🦀️.rs:337` `impl<V> dsl::ToValue for RasterOwnedMap<V>` (assert + `DslValue::Object(Vec::new())`) | `raster.includes("impl<V> dsl::ToValue for RasterOwnedMap<V>")` **+** `raster.includes("impl<V> dsl::FromValue for RasterOwnedMap<V>")` (the decode half, previously unpinned) |
| 2 | `rasterOwnedMapSerdeGuards === 3` | all three owned-map-typed fields route through that guard; a fourth field or a bypassed one moves the count | the same three fields, now guarded by the blanket impl through `#[derive(dsl::ToValue, dsl::FromValue)]` | `rasterOwnedMapFieldSites === 3`, counting `/^\s*(?:pub )?[a-z_]+: RasterOwnedMap</gm` — plus a **new** dangling-oracle clause `(rasterOwnedMapSerdeGuards === 0 \|\| raster.includes("fn serialize_empty_owned_map<S: serde::Serializer, V>"))`: the test-only serde oracle may keep its `serialize_with` route only while the function it names exists |
| 3 | `wasm.includes("pub struct RasterEnvelopeLoadHandle")` | the caller holds a **typed** ingress handle, not a raw u64 | gismap `…/✏️editor/🦀️.rs:1103`, whose admit helper returns `semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle` | `editor.includes("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle")` |
| 4 | `wasm.includes("fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle")` | the handle carries the **generation**, so a stale load cannot publish | gismap `…:1129` `assert_eq!(handle.generation, base_generation);` | `editor.includes("assert_eq!(handle.generation, base_generation)")` |
| 5 | `wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)")` | page/byte credits are reserved **before** any byte is admitted | gismap `…:1105` `app.begin_artifact_envelope_ingress(pages, wire.len().max(1))` | `credits = editor.indexOf("begin_artifact_envelope_ingress(pages, ")`, clause `credits >= 0` |
| 6 | `wasm.includes("source: &js_sys::Uint8Array")` | bytes arrive through a **bounded** page, never as an owned growable buffer | gismap `…:1109` `store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len())` | `construct = editor.indexOf("store::ArtifactEnvelopeDecodePage::try_from_array(bytes, ")` |
| 7 | `wasm.includes("let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]")` | the page buffer is a fixed-size array | identical line, gismap `…:1107` | unchanged text, now read from `editor` (`page` cursor) |
| 8 | `preflight >= 0` | credit check happens | folded into #5 | `credits >= 0` |
| 9 | `construct > preflight` | page construction only after the credit check | ordering of the same four cursors | `page > credits` |
| 10 | `copy > construct` | bytes are copied only into an already-admitted fixed page | — | `construct > page` **and** `admit > construct` (`admit = editor.indexOf("admit_artifact_envelope_ingress_page(handle, page)")`) — the whole wire is also required to be paged: `editor.includes("chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")` |
| 11 | `wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())")` | ingress is sealed before the load is driven | gismap `…:1112` | `editor.includes("seal_artifact_envelope_ingress(handle)")` |
| 12 | `wasm.includes("app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")` | the load advances one bounded maintenance turn at a time | gismap `…:1118` — identical | unchanged text, read from `editor` |
| 13 | `wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())")` | the caller polls rather than blocking | gismap `…:1119` | `editor.includes("advance_artifact_envelope_load(handle)")` |
| 14 | `wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())")` | the store swap is acknowledged **exactly once** | gismap `…:1136-1137` | `editor.includes("acknowledge_artifact_store_replacement(handle)")` **+ new** `editor.includes("duplicate Raster load acknowledgement is a no-op")` (the "exactly once" half the old clause could not see) |
| 15 | `wasm.includes("cancel_artifact_envelope_load(handle.runtime_handle())")` | cancellation goes through the retained route | gismap `…:1152` | `editor.includes("cancel_artifact_envelope_load(handle)")` |
| 16 | `wasm.includes("close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")` | owners are released one bounded grant at a time | gismap `…:1087` (fixture envelope retirement) | unchanged text, read from `editor` |
| — | *(new)* | the cohort actually exists as named tests, not as dead helpers | gismap `…:1126`/`…:1141` | `editor.includes("raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed")`, `editor.includes("raster_live_envelope_cancel_closes_retained_pages_without_publication")` |
| 17 | `!wasm.includes("envelope_json: &str" \| "reject_whole_buffer_artifact_envelope_ingress" \| "ArtifactStore::new" \| "source.to_vec()" \| "while let" \| "loop {")` | no whole-buffer bypass, no store construction outside the initializer, no unbounded loop | same prohibitions, now against the file that hosts the route | six `!editor.includes(…)` clauses (all six verified 0 occurrences in raster's editor today, so they are live constraints, not vacuous ones) |
| 18 | concat source `…/✳️any/🧬️schema/🦀️component.rs` (`policyReadFileSafe` → `""`) | the schema file was supposed to contribute the owned-map field guards | renamed to `…/✳️any/🧬️schema/🦀️.rs` | path fixed **and** all twelve retained sources routed through a throwing reader (below) |

**Nothing was deleted.** The only clause that changed meaning rather than location is #2's serde half,
which became conditional — see §5, gap 1, for why that is the correct current law and not a weakening.

### Loud inputs

`policyReadFileSafe` silently yields `""`, which is how #18 rotted unnoticed. All twelve retained Raster
sources (four schema/codec + eight mounted export leaves) now go through one throwing reader
(`📜️script.ts:10282-10286`):

```
[verify interactivity tool-jobs] retained Raster envelope source "<path>" is missing or empty; the Raster envelope law cannot be evaluated against a path that does not resolve.
```

Verified by control D (§4d).

### The `wasm` parameter is gone

With every positive clause retargeted at the editor and every negative clause with it, the fourth
parameter had no reader left. `toolJobRasterEnvelopeCallerRetainedExact` is now
`(store, raster, editor, plugin)`; the `rasterWasm` input and the `retainedRasterWasm` self-test fixture
were deleted, and the 85 self-test call sites plus 2 multi-line ones were updated mechanically (the
substitution was diffed against the pre-edit file to prove nothing else moved).

---

## 3. Changes (`📜️script.ts`, the only file changed — `git diff --stat` → `+205 −152`)

| region | change |
|---|---|
| `:3982` | signature drops `wasm: string` |
| `:3997-4000` | `preflight`/`construct`/`copy` cursors → `credits`/`page`/`construct`/`admit`, read from `editor` |
| `:4004` | `rasterOwnedMapSerdeGuards` regex relaxed to the `cfg_attr(test, serde(…))` form actually on disk |
| `:4005` | new `rasterOwnedMapFieldSites` counter |
| `:4105-4109` | owned-map output/decode authority + field-site count + dangling-oracle clause |
| `:4215-4230` | the 15 retargeted ingress clauses (+ 2 new cohort-name clauses, + the duplicate-ack clause) |
| `:4236-4241` | six negatives retargeted to `editor` |
| `:8112-8117` | positive fixture: serde lines → `impl<V> dsl::ToValue/FromValue` + the three owned-map field lines |
| `:8172-8190` | `retainedRasterEditor` grown into the full ingress cohort; `retainedRasterWasm` deleted |
| `:8204-8215` | 12 hostile mutations rewritten/added for the editor cohort |
| `:8225-8229` | 5 hostile mutations retargeted (page route, whole-buffer bypass, exact ack, cancel, bulk close) |
| `:8274-8277` | the obsolete `Raster-serde-derived-map-guard-removal` test replaced by 4: unguarded map-field escape, output-authority removal, decode-authority removal, dangling serde oracle |
| all self-test call sites | `retainedRasterWasm` argument removed (85 single-line + 2 multi-line) |
| `:10282-10307` | throwing reader `rasterRetainedSource`; concat path `🦀️component.rs` → `🦀️.rs`; `rasterWasm` deleted |
| `:10392` | failure sentence rewritten (`preflight-before-copy` → editor-hosted credit-before-page cohort; owned-map authority added) |

Hostile self-tests now covering this slice (name → what it must reject):

| self-test | rejects |
|---|---|
| `Raster-page-admitted-without-ingress-credits` | pages admitted with no credit reservation |
| `Raster-page-admitted-before-bounded-construction` | admission before the bounded page is built |
| `Raster-fixed-page-owner-removal` | growable `Vec` page buffer |
| `Raster-generation-handle-erasure` | handle no longer checked against the base generation |
| `Raster-untyped-ingress-handle` | admit helper stops returning the typed handle |
| `Raster-single-whole-wire-page` | the wire stops being chunked into fixed pages |
| `Raster-unsealed-ingress-submit` | submit without seal |
| `Raster-unpumped-load-progress` | load driven without bounded maintenance turns |
| `Raster-unpolled-load-terminal` | terminal reached without polling |
| `Raster-repeatable-load-acknowledgement` | the duplicate-ack no-op law dropped |
| `Raster-live-submit-fixture-missing` / `Raster-live-cancel-fixture-missing` | either cohort test renamed away |
| `Raster-dynamic-page-route` | `try_from_vec(bytes.to_vec(), …)` instead of the fixed array |
| `Raster-whole-buffer-bypass` | an `envelope_json: &str` route appearing |
| `Raster-terminal-without-exact-ack` / `Raster-cancel-owner-drop` / `Raster-bulk-close` | unchanged intent, editor-hosted |
| `Raster-unguarded-map-field-escape` | an owned-map field replaced by a bare `HashMap` |
| `Raster-owned-map-output-authority-removal` / `…-decode-authority-removal` | the `ToValue`/`FromValue` impls moved off `RasterOwnedMap` |
| `Raster-dangling-serde-oracle-route` | a `serialize_with` attribute naming a function that does not exist |

---

## 4. Verbatim output

### 4a. Clause-by-clause against the LIVE tree — `🟦️w5b-gate-probe.ts` (kept in this ticket folder)

```
$ bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🟦️w5b-gate-probe.ts"
[source] 280537 bytes  …/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs
[source] 37838 bytes  ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs
[source] 21740 bytes  …/✳️any/🧬️schema/🦀️.rs
[source] 27325 bytes  …/✳️any/🧬️schema/📸️snapshot/🦀️.rs
[source] 1525 bytes  …/🧵️serializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️.rs
[source] 979 bytes  …/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs
[source] 727 bytes  …/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs
[source] 1083 bytes  …/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs
[source] 1155 bytes  …/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs
[source] 1123 bytes  …/🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs
[source] 1019 bytes  …/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs
[source] 1278 bytes  …/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs
[source] 100399 bytes  …/✳️any/✏️editor/🦀️.rs

[probe] clauses=199 passing=182 failing=17
  FAIL  (rasterOwnedMapSerdeGuards === 0 || raster.includes("fn serialize_empty_owned_map<S: serde::Serializer, V>"))
  FAIL  editor.includes("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle")
  FAIL  editor.includes("assert_eq!(handle.generation, base_generation)")
  FAIL  editor.includes("chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")
  FAIL  credits >= 0
  FAIL  page > credits
  FAIL  construct > page
  FAIL  admit > construct
  FAIL  editor.includes("seal_artifact_envelope_ingress(handle)")
  FAIL  editor.includes("app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")
  FAIL  editor.includes("advance_artifact_envelope_load(handle)")
  FAIL  editor.includes("acknowledge_artifact_store_replacement(handle)")
  FAIL  editor.includes("duplicate Raster load acknowledgement is a no-op")
  FAIL  editor.includes("cancel_artifact_envelope_load(handle)")
  FAIL  editor.includes("close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)")
  FAIL  editor.includes("raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed")
  FAIL  editor.includes("raster_live_envelope_cancel_closes_retained_pages_without_publication")

[oracle gis-map editor] ingress clauses=20 passing=15 failing=5
  ONLY-RASTER-NAMED  editor.includes("raster_envelope_decode_owner_bundle()")
  ONLY-RASTER-NAMED  editor.includes("raster_document_store_initialization_job(envelope, operation, generation)")
  ONLY-RASTER-NAMED  editor.includes("duplicate Raster load acknowledgement is a no-op")
  ONLY-RASTER-NAMED  editor.includes("raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed")
  ONLY-RASTER-NAMED  editor.includes("raster_live_envelope_cancel_closes_retained_pages_without_publication")
```

Reading of that output:

* **All 182 clauses that passed before still pass**, including everything W5 installed — adding
  `🧬️schema/🦀️.rs` to the concatenation broke no negative clause and moved no count, and the new
  `rasterOwnedMapFieldSites === 3` and both owned-map authority clauses hold against the live tree.
* **17 clauses fail, and every one of them is a real raster gap** (§5), not gate rot: 1 is the dangling
  serde oracle, 16 are the missing retained ingress cohort. It is honestly *not* possible to show this
  gate green against the live tree — raster does not satisfy it, and weakening it to green was the one
  thing this task forbade.
* **The oracle pass is the proof the rewritten clauses are right.** Feeding the *GIS Map* editor — the
  live plugin that already carries the post-bridge cohort — into the same clauses passes 15 of 20; the
  5 that do not are exactly the five that name Raster identifiers. So every structural clause I wrote
  matches code that exists in this repo today, character for character, rather than a shape invented
  for the gate.

### 4b. The self-tests that exercise the gate

`toolJobCoverageSelfTests()` runs the raster fixture (`📜️script.ts:8191`) and all ~130 raster hostile
mutations (`:8192-8380`) inside `bun ./📜️script.ts verify interactivity tool-jobs`. **They pass**: the
run proceeds past them and dies ~550 lines later inside `toolJobScalarConfigCohortSelfTests`
(`:8928 → :2689`) on an unrelated peer-in-flight rename — the same pre-existing blocker W5 recorded:

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
      at toolJobCoverageSelfTests (/Users/ueli/Documents/semio/📜️script.ts:8928:24)
      at toolJobCoverageRun (/Users/ueli/Documents/semio/📜️script.ts:10334:21)
      at runToolJobCoverage (/Users/ueli/Documents/semio/📜️script.ts:10952:20)
```

(The `✳️any` → `🌐️any` FEM subset rename is flapping under a peer session: one run during this ticket
got past line 2689 and reached the raster self-tests directly — that run is what caught the obsolete
`Raster-serde-derived-map-guard-removal` test, now fixed.)

`--self-test` is a different, earlier code path (`:10944`) that never reaches
`toolJobCoverageSelfTests()`; it still dies first on the pre-existing `📕️norm`/`🖍️draw` factory-proof
scan, unchanged and untouched by this slice:

```
$ bun "./📜️script.ts" verify interactivity tool-jobs --self-test
error: [verify interactivity tool-jobs] app activation factory proof scan: {"owners":49,"customRows":575,"genericRows":0,"failures":["forged bounded reducer factory or compiler witness ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/…","… 🖍️draw/🗿️artifacts/🖍️drawing/… DrawingBoundedCommandJobFactory …"]}
      at runToolJobCoverage (/Users/ueli/Documents/semio/📜️script.ts:10944:53)
```

### 4c. Negative controls — the self-tests are load-bearing, not vacuous

Three deliberate, reverted breakages; each was run in full and produced exactly the expected failure.

**Control A — the positive fixture is really checked.** Removed one of the three
`"    pub assets: RasterOwnedMap<RasterAssetChild>,"` fixture lines (field sites 3 → 2):

```
error: [verify interactivity tool-jobs] self-test retained-Raster-envelope-route was falsely rejected.
```

**Control B — the owned-map field law is really checked.** Weakened `rasterOwnedMapFieldSites === 3` to
`>= 2`:

```
error: [verify interactivity tool-jobs] self-test Raster-unguarded-map-field-escape was falsely accepted.
```

**Control C — the new ingress ordering law is really checked.** Weakened `admit > construct` to
`admit >= 0`:

```
error: [verify interactivity tool-jobs] self-test Raster-page-admitted-before-bounded-construction was falsely accepted.
```

All three edits were reverted; `git diff` for `📜️script.ts` contains none of them.

### 4d. Control D — a missing input is now loud instead of silent

Temporarily restored the stale concat path (`🧬️schema/🦀️.rs` → `🧬️schema/🦀️component.rs`):

```
error: [verify interactivity tool-jobs] retained Raster envelope source "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs" is missing or empty; the Raster envelope law cannot be evaluated against a path that does not resolve.
```

Before this change the same path produced `""` and the gate simply evaluated a smaller corpus. Reverted.

---

## 5. Raster gaps for the coordinator

### Gap 1 — `🧬️schema/🦀️.rs:25` names a function that no longer exists (breaks `cargo test`)

```rust
#[derive(… )]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct RasterArtifact {
    …
    #[cfg_attr(test, serde(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map"))]
    pub assets: RasterOwnedMap<RasterAssetChild>,
```

`crate::artifacts::raster::serialize_empty_owned_map` was removed by the serde-elimination sweep
(`grep -rn "fn serialize_empty_owned_map" --include='*.rs'` → 0 hits repo-wide; the only surviving
mention is the doc comment at `🦀️.rs:334` that calls the `dsl::ToValue` impl its "first-party analog").
Under `cfg(test)` the derived `Serialize` for `RasterArtifact` expands to a call to that path, so this
is a compile error in the raster crate's test profile, not merely dead text. `RasterOwnedMap` has no
`Serialize` impl either (the gate still asserts it must not), so deleting the attribute alone will not
compile — the two clean fixes are (a) drop the test-only `Serialize`/`Deserialize` derive from
`RasterArtifact`, or (b) add a `#[cfg(test)] fn serialize_empty_owned_map<S: serde::Serializer, V>` back
to `🗿️artifacts/🖨️raster/🦀️.rs` next to the `dsl::ToValue` impl. The gate stays agnostic: it demands
only that no `serialize_with` route name a function that is absent.

**Owner: W4** (raster source). One clause, one line.

### Gap 2 — raster has no retained envelope-ingress caller at all

`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` declares the two
build hooks the gate already checks (`build_envelope_decode_owner_bundle`,
`build_document_store_initialization_job`) but **never drives the paged load**: zero occurrences of
`begin_artifact_envelope_ingress`, `admit_artifact_envelope_ingress_page`,
`seal_artifact_envelope_ingress`, `advance_artifact_envelope_load`,
`acknowledge_artifact_store_replacement` or `cancel_artifact_envelope_load`. The wasm bridge that used
to hold them was deleted on 09/01 and nothing replaced it, so since then nothing has proven that
raster's decode-owner bundle and initialization job actually work end to end under bounded page credits.

The fix is a straight port of the GIS Map cohort
(`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1064-1155`) into
raster's editor test module, with these names (which the gate now pins):

* `fn raster_envelope_wire() -> Vec<u8>` — build the envelope, retire it through
  `close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)`.
* `fn admit_raster_envelope(app, wire) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle`
  — `begin_artifact_envelope_ingress(pages, …)`, `for chunk in wire.chunks(PAGE_BYTES)`,
  `let mut bytes = [0; PAGE_BYTES]`, `ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len())`,
  `admit_artifact_envelope_ingress_page(handle, page)`, `seal_artifact_envelope_ingress(handle)` — in
  that order (the gate checks the ordering, not just the presence).
* `fn drive_raster_live_load(app, handle)` — `app.maintenance_step(1, PAGE_BYTES)` then
  `advance_artifact_envelope_load(handle)`.
* `async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed()` — asserts
  `handle.generation == base_generation`, poll `Ready`, generation `+ 1`, first
  `acknowledge_artifact_store_replacement(handle)` true and the second false with the expect string
  `"duplicate Raster load acknowledgement is a no-op"`.
* `async fn raster_live_envelope_cancel_closes_retained_pages_without_publication()` —
  `cancel_artifact_envelope_load(handle)`, poll `Fault`, generation unchanged.

**Owner: W4 / coordinator.** 16 of the gate's 199 clauses stay red until this lands.

### Not raster, not touched — flagged only

* **Same rot in four peer gates.** `toolJobPresentationEnvelopeCallerRetainedExact` (`:3660`),
  `…Writer…` (`:3716`), `…Jack…` (`:3765`) and `…GisMap…` (`:3903`) still read
  `…/✏️editor/🌉️wasm/🦀️component.rs`, which does not exist for any of them, and still assert 8-14
  `wasm.includes(…)` clauses against `""`. GIS Map's gate already grew editor-cohort clauses
  (`:3956-3958`) beside its dead wasm ones, so the migration was started and abandoned. Those four
  gates need the same treatment this note applies to raster; puzzle 3d/5d keep their bridges and need
  none.
* **Another stale `🦀️component.rs`**: `📜️script.ts:11779`
  `INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE` points at
  `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/🧬️schema/🦀️component.rs`; the directory holds `🦀️.rs`.
* **`toolJobScalarConfigCohortSelfTests` (`:2689`)** hard-codes the FEM2D editor path a peer is renaming
  `✳️any` → `🌐️any`; it throws ENOENT roughly half the time. Unrelated to raster.
* **`--self-test` factory-proof scan** — 📕️norm's 16 standards plus 🖍️draw report "forged bounded reducer
  factory or compiler witness". Pre-existing, unchanged.

---

## 6. Exact files touched

Updated:

- `📜️script.ts` (only file changed; `+205 −152`, every hunk inside the Raster gate, the Raster
  self-test block, or the Raster input/failure lines — verified with `git diff -U0 | grep '^@@'`)

Created (kept — verification input, not generated output):

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🟦️w5b-gate-probe.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️w5b-raster-gate-residuals.md` (this file)

Not touched: everything under `✏️s/🔌️plugins/🖨️raster`, every non-Raster region of `📜️script.ts`, the
peer gates named above, and `🗑️generated/`.
