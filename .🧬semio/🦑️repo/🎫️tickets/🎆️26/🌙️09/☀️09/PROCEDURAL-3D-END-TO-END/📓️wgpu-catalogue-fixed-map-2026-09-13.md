# 🛍️ wgpu catalogue `UiFixedMap` — JSON object key order was never a contract

Lane **wgpu-catalogue-fixed-map** · 2026-09-13 · ticket `26/09/09/PROCEDURAL-3D-END-TO-END`

---

## 1. TL;DR

| | |
|---|---|
| **Symptom** | On every wgpu boot the app-static catalogue document failed to parse: `renderDocument result parse failed: UiFixedMap requires at most 32 ascending unique entries at line 1 column 21173`. The node-graph palette/spotlight got no operators. React rendered the same document fine. (`📓️wgpu-input-hit-runtime-2026-09-13.md` §3.2 named it and left it.) |
| **It was never about the cap** | The map had exactly 32 entries — its capacity, not one over. The refusal came from the **other** half of `try_push`'s precondition: keys must arrive in **ascending** order. |
| **Root cause** | A `UiFixedMap` is a sorted map whose wire form is a JSON **object**, and object key order carries no meaning. The wgpu bridge's `renderDocument` hands the guest document to `JSON.stringify` (`🐚️plugin-bridge/🟦️.ts:1369`), and every ECMAScript engine emits **array-index-like keys first, in ascending numeric order**, then the remaining string keys. The carrier's keys are `"01".."32"`, so `"10".."32"` (23 of them) jump ahead of `"01".."09"` — and the 24th entry the decoder saw was `"01"` after `"32"`. |
| **Why React was immune** | Its reader already sorts: `packedTextLeaf` (`🔌️PluginRuntime/🧳️packed-text/🟦️.ts`) iterates `Object.keys(dataAttributes).sort()`. The TypeScript half of the contract had the right answer; the Rust half did not. |
| **Fix (owning layer)** | `UiFixedMap` decode is now order-independent: `try_insert` places each entry at its **sorted** position, `contains` separates duplication from capacity, and both `Deserialize` and `FromValue` use them. The contract's invariant (a sorted, ≤32-entry, unique-keyed map) is unchanged — only the false dependency on wire order is gone. No renderer-side cap was raised; no projection was paged. |
| **Laws** | Rust **8 passed** (`--test catalogue_carrier_map`), TypeScript twin **9 checks** — over one language-neutral fixture holding a real catalogue-shaped carrier leaf and the key order a JS engine actually emits. |
| **Runtime proof (6118)** | `[DEBUG] wgpu shell app catalogue ready bytes=111557` at 6 255 ms. Zero `parse failed`, zero `UiFixedMap requires`, zero reassembly failures across 75 s. Boot does **not** wedge — frames advance to generation 16. §6. |

---

## 2. Root cause, with file:line

### 2.1 The producer

`semio_framework_plugin::app::section_text_chunks`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:419–458`) packs the ~108 kB catalogue payload
(`flow_app_catalogue_json`, `🌊️flow/🗂️catalogue/🦀️.rs:171`) into text leaves: `1 + UI_FIXED_LIST_ITEMS`
= 33 slices of `UI_TEXT_MAX_BYTES` each, the first as `TextProps.value` and the remaining **32** as
`data_attributes` keyed `"{offset:02}"` — `"01".."32"`. That is exactly `UiFixedMap`'s capacity, and
the producer admits it: it builds the map in ascending order with `try_push`.

### 2.2 The precondition that actually broke

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:587–592` (pre-fix):

```rust
pub fn try_push(&mut self, key: UiText, value: V) -> Result<(), (UiText, V)> {
    if self.entries.len().checked_sub(1).and_then(|index| self.entries.get(index)).is_some_and(|(last, _)| last >= &key) {
        return Err((key, value));
    }
    self.entries.try_push((key, value))
}
```

Two refusals, **one message**. The decoders at `:641–649` (`Deserialize`) and `:664–679` (`FromValue`)
turned either one into `UiFixedMap requires at most 32 ascending unique entries`, which read like a
capacity overflow and was not one.

### 2.3 The boundary that reorders

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:1369`:

```ts
renderDocument: (instanceId, surfaceId, bodyKey, viewState) =>
  renderSurface(instanceId, surfaceId, bodyKey, viewState)
    .then((result) => JSON.stringify({ document: result.document, effects: jsonEffects(result.effects) })),
```

`result.document` is a **JavaScript object**. ECMAScript `[[OwnPropertyKeys]]` orders integer-index
keys first, ascending numerically, then string keys in insertion order — so `JSON.stringify` emits:

```
{"10":…,"11":…,…,"32":…,"01":…,"02":…,…,"09":…}
```

Verified directly (`node -e`), and on a second engine (bun/JSC) by the twin:

```
JSON.stringify(JSON.parse('{"01":1,"02":2,"09":3,"10":4,"32":5}'))
→ {"10":4,"32":5,"01":1,"02":2,"09":3}
```

The Rust decoder then read `"32"` followed by `"01"`, refused, and the whole
`BrowserRenderEnvelope` parse died — taking every node of the document with it. Column 21 173 is where
the first leaf's `data_attributes` ends; the isolated fixture reproduces the identical message at
column 15 071 (§5).

### 2.4 Why this is the contract's defect, not the renderer's

The coordinator's two framings were *raise the renderer's bound* or *page the projection at the
contract level*. Neither applies: the bound was never reached, and the projection is already paged.
The real defect is that **decode depended on something JSON does not promise**. The contract's own
TypeScript reader (`🧳️packed-text/🟦️.ts`) had already conceded that point by sorting; the Rust
decoder is what was out of step. Fixing decode fixes every `UiFixedMap` on every wire at once, not
just this carrier.

---

## 3. The fix

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs`

* **`UiFixedMap::try_insert`** — binary-searches the insert position, refuses a duplicate key, reserves
  the list's page grant, then walks ONE displaced payload through the tail with `std::mem::swap` and
  pushes the last one into the reserved slot. No temporary buffer, no second allocation: admission
  costs exactly what `try_push` cost.
* **`UiFixedMap::contains`** — lets a decoder that can only return a string tell duplication from
  capacity.
* **`Deserialize::visit_map`** and **`FromValue::from_value`** now use `contains` + `try_insert`, and
  the expectation string is `"a bounded fixed UI map of unique keys, in any order"`.
* `try_push` is **unchanged** — the producer's ascending fast path keeps enforcing its invariant.

A peer lane landed `UiFixedMap::refusal()` on top of this while it was in flight, splitting the
refusal text into *at capacity* vs *out of page grant*; both decoder arms now report through it. Kept
as-is — it is the same defect's other half (an unhelpful message) and it composes cleanly.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
`refresh_app_catalogue` had traces on **both** failure arms and none on success, so a working
catalogue was indistinguishable from one that never ran. Added the symmetric line:
`[DEBUG] wgpu shell app catalogue ready bytes=<n>`.

---

## 4. Fixture (language-neutral)

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🛍️catalogue-carrier-map.json` — regenerated by
`<ticket>/🐍️carrier-map-fixture.mjs`, which **refuses to write a fixture whose keys the engine did not
reorder**, so the law can never quietly go vacuous.

| field | meaning |
|---|---|
| `value` | slice 0 — the leaf's `TextProps.value` |
| `ascendingKeys` / `javascriptKeys` | `"01".."32"` vs the order `JSON.stringify` emits |
| `dataAttributesJson` | the wire **bytes**, stored as a string so no JSON reader in either language can re-sort them |
| `payload` | the 16 896-byte catalogue payload both readers must recover |

The payload is catalogue-shaped: operator records with `kind`/`label`/`module`/`inputs`/`outputs`,
quote-dense, including the `"default":"{…\"angle\":1.5707963267948966}","cardinality":"!"` and
`"outputs":[{"code":"C","abbreviation":"Crv"…` text that the failing boot window quoted verbatim.

---

## 5. Laws

### 5.1 Rust — 8 passed

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🛍️catalogue-carrier-map/🦀️.rs`, registered as
`[[test]] name = "catalogue_carrier_map"` in the crate manifest (emoji path needs the explicit target).

```
cargo test -p semio-framework-ui-contract --test catalogue_carrier_map -- --nocapture
```

| law | what it pins |
|---|---|
| `the_catalogue_carrier_fixture_pins_a_reordered_javascript_object` | the fixture really is reordered — first key `10`, last `09` |
| `a_javascript_ordered_carrier_map_decodes_into_the_sorted_map` | **the defect** — 32 entries, decoded into ascending order |
| `a_javascript_ordered_carrier_leaf_recovers_its_payload` | 16 896 bytes recovered byte-for-byte |
| `a_carrier_map_still_refuses_duplicate_keys` | unordered ≠ permissive |
| `a_carrier_map_still_refuses_one_entry_past_capacity` | 33 entries still refused |
| `a_carrier_map_refusal_names_capacity_and_page_grant_apart` | peer lane's law over `refusal()` |
| `a_javascript_ordered_carrier_map_decodes_through_from_value` | the pack codec the guest speaks carries the same law |
| `an_ascending_carrier_leaf_still_round_trips_through_json` | the producer's own path is untouched |

**Pre-fix**, the four decode laws failed with the runtime's exact text:

```
Error("UiFixedMap requires at most 32 ascending unique entries", line: 1, column: 15071)
```

**Post-fix** (see §7 for the run):

```
[DEBUG] catalogue-carrier-map javascript-order first=10 last=09
[DEBUG] catalogue-carrier-map decoded entries=32
[DEBUG] catalogue-carrier-map payload-bytes=16896
[DEBUG] catalogue-carrier-map from-value entries=32
[DEBUG] catalogue-carrier-map ascending-round-trip json-bytes=20760
```

### 5.2 TypeScript twin — 9 checks

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🛍️catalogue-carrier-map/🟦️.ts`, wired into the crate's
`📜️script.ts` `TestScript` beside the other twins:

```
[DEBUG] catalogue-carrier-map-twin checks=9
```

The **engine itself is the third-party oracle**: the twin re-runs `JSON.parse`/`JSON.stringify` and
asserts the emitted key order equals the pinned one, that sorting recovers the payload, and — the
negative half — that reading the wire in **arrival** order would corrupt it. It passes under bun/JSC
as well as the node/V8 that generated the fixture, so the reordering is engine-independent, not a V8
quirk.

---

## 6. Runtime proof on 6118

`<ticket>/🐍️wgpu-catalogue-probe.mjs` → `🗑️generated/wgpu-catalogue/after-fix/`
(`console.txt`, `report.json`, `shell.png`), after
`bunx nx run @semio-tech/framework-renderer-wgpu:wasm` (exit 0) and a reload.

```json
{ "url": "http://127.0.0.1:6118/?plugin=generation3d", "seconds": 75, "lines": 406,
  "parseFailed": [], "reassemblyFailed": [], "fixedMapRefusals": [],
  "ready": ["6255 log [DEBUG] wgpu shell app catalogue ready bytes=111557"],
  "catalogueRenders": 3 }
```

From the same console:

```
5150  wgpu-shell render begin surface=framework.panel.catalogue body=procedural.play.catalogue
5923  wgpu-shell render leave  surface=framework.panel.catalogue 775 ms resident-roots=4 resident-bytes=1200300/33554432
6166  wgpu-bridge renderSurface surface=framework.section.catalogue turn=0 …
6253  wgpu-bridge renderSurface surface=framework.section.catalogue turn=2 …
6255  [DEBUG] wgpu shell app catalogue ready bytes=111557
```

* The 111 557-byte app-static catalogue **parses, reassembles and publishes** into the node-graph
  hosts — one fetch, three turns, no refusal.
* The Catalogue panel body renders and retains a real document (`resident-roots=4`).
* Boot is **not** wedged: `frame build admitted generation=16`, `world3d … draws=1`, and a dock plan at
  `canvas=1594x936` at 25 s, still live at 75 s.
* Pre-fix comparison — `🗑️generated/wgpu-input/boot-generate-final/console.txt:501` carries the
  refusal on the same document at the same column.

**Not claimed: the screenshot.** `shell.png` is a uniform dark frame — this headless Chromium has no
WebGPU adapter, so the worker's `OffscreenCanvas` never presents into the captured page, even though
frames are being built. The catalogue evidence above is console-only, and the visual listing of
operators in the panel is therefore **not** visually confirmed by this lane.

---

## 7. Commands

```
node   <ticket>/🐍️carrier-map-fixture.mjs
cargo  test -p semio-framework-ui-contract --test catalogue_carrier_map -- --nocapture
bun    -e 'import { catalogueCarrierMapSelfTests } from ".../🧪️tests/🛍️catalogue-carrier-map/🟦️.ts"; …'
CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bunx nx run @semio-tech/framework-renderer-wgpu:wasm
SEMIO_PROBE_SECONDS=75 SEMIO_PROBE_OUT=after-fix bun <ticket>/🐍️wgpu-catalogue-probe.mjs
```

### 7.1 Fleet note — the shared cargo build dir filled up

The first renderer wasm build failed with `No space left on device` (354 MiB free; the build dir was
358 GB, of which ~108 GB was `incremental/`). Pruned every `incremental/` directory under
`⚡️cache/cargo/build` → 74 GiB free, and ran all later builds with `CARGO_INCREMENTAL=0`. Nothing but
incremental caches was removed; they regrow on demand. Peers building at that moment may have seen one
spurious failure.

---

## 8. Left open (named, not fixed)

**`UiMap` has the identical defect.** `UiMapBuilder::push` (same file, `pub struct UiMapBuilder`)
rejects any key not strictly greater than `last_key`, and `Deserialize for UiMap` reports it as
`UiMap requires at most N ascending unique entries`. Any `UiValue` map with numeric-ish keys crossing
the same `JSON.stringify` boundary — an action binding's `args`, an extension's props — will fail the
same way. It is **not** fixed here because `UiMap` is arena-backed with page/credit budgets and
cursors: sorted insertion has to rewrite arena pages rather than swap one payload through a tail, and
that is a different owner's change. A background task has been filed with the full reproduction recipe
and a pointer to `UiFixedMap::try_insert` as the reference.

---

## 9. Files

**Changed**
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs` — `try_insert`, `contains`, `search`; both decoders
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` — `[[test]] catalogue_carrier_map`
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` — twin registered in `TestScript`
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — catalogue success trace

**Added**
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🛍️catalogue-carrier-map/🦀️.rs`
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🛍️catalogue-carrier-map/🟦️.ts`
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🛍️catalogue-carrier-map.json`
* `<ticket>/🐍️carrier-map-fixture.mjs`, `<ticket>/🐍️wgpu-catalogue-probe.mjs`
