# JSON 512-byte truncation — `Unterminated string in JSON at position 512`

Lane: shell/typed-operation completion path, 512-byte cap. Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.

## Symptom (boot #6b, `📓️runtime-verification-2026-09-09.md`)

```
[DEBUG] render failed [unknown] no-code SyntaxError: Unterminated string in JSON at position 512
[DEBUG] typed-operation completion effects failed SyntaxError: Unterminated string in JSON at position 512
```

Both window bodies replaced by the error card. `position 512 (line 1 column 513)` means the parser
consumed exactly 512 characters and hit end-of-input inside a string — i.e. the payload handed to
`JSON.parse` was an exact 512-**byte** prefix of a longer ASCII JSON document.

## Root cause

Not a producer cap at all: **the reader drops 32 of every 33 payload slices.**

* Producer — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:391` `section_text_chunks`
  (via `paged_text_carrier` `:439`, `section_component_tree` `:461`). It splits the section payload
  into `UI_TEXT_MAX_BYTES` (= 512, `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:18`)
  slices and **packs `1 + UI_FIXED_LIST_ITEMS` = 33 slices per text leaf**: slice 0 in
  `TextProps.value`, slices 1..32 as `data_attributes` keyed `"01".."32"`. This packing exists so a
  Nakagin-scale payload stays inside `UI_DOCUMENT_NODES`.
* Reader — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1353`
  `sectionValueFromBuiltNode`. Its walk concatenated **only** `component.value` and ignored
  `component.dataAttributes` — even though its own docstring claimed it was "the same walk"
  as the Interpreter's `surfaceSceneLaneText`
  (`🧱️elements/🗣️Interpreter/🟦️.tsx:508`), which **does** append the sorted `dataAttributes`.

So any reserved refresh section (`engagements` / `measures` / `tools` / `catalogue`,
`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1151`) whose canonical JSON exceeds 512 bytes reached
`JSON.parse` as its first 512 bytes only. generation3d's `catalogue` section (the whole registered
operator catalogue) and its `measures` section are both far past 512 bytes, so `refreshUi` threw on
every call — which is why the SAME `SyntaxError` surfaced from both the session-refresh effect
(`ShellHost/🟦️.tsx:4388` `render failed`) and the typed-operation completion effect pass
(`ShellHost/🟦️.tsx:4961` → `applyHostEffects` → `refreshUi`): both funnel into `refreshUi`.

Existing coverage missed it because the only TS test over this reader
(`🧪️tests/🔌️plugin-runtime/🟦️.tsx`, "projects a section's chunked text carrier back into the
shell's own measures map") splits a 17-byte payload across two separate leaf **nodes** and never
produces a packed leaf. The Rust side is fully covered
(`🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:4124`) and is correct.

Ruled out as unrelated to this fault:
`ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES = 512` (`🔌️plugin/🧵️retained-command/🦀️.rs:10`) is a
binary checkpoint with a 48-byte header — it never carries JSON and can never yield a 512-byte JSON
prefix; `PluginRuntime/🟦️.tsx:1283`'s `% 512` is a log cadence; `RESOLVED_CONFLICT_CAP` and
`try_reserve_exact(512)` are not on this path.

## Fix

1. `sectionValueFromBuiltNode` appends `component.dataAttributes` in sorted key order after
   `component.value`, exactly inverting `section_text_chunks`' pack — the carrier's slices are
   complete on the wire, so nothing had to be re-budgeted or re-paged.
2. A non-parsable section payload is no longer a bare `SyntaxError`: it raises a typed
   `SemioFaultError` `plugin-ui.section-payload-not-json` whose `scope.bodyKey` names the reserved
   section, whose `scope.pluginId`/`scope.instanceId` name the producing actor/instance, and whose
   message carries the byte length and the parser's own reason.
3. Both call sites (`retainedUiRefreshResponse`, `ownedUiRefreshResponse`'s `projectSections`) pass
   the producing actor/instance so the fault can name it.

## Tests

Language-neutral fixture `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🔬️paged-text-carrier/🔣️.json`
declares the packing law (`textMaxBytes`, `packSlices`, `attributeKeyFormat`) and one payload past a
single slice together with its exact slice split. Both languages are pinned against it.

## Reproduction (native, no browser)

The neutral fixture's own packed leaf, walked the OLD way (`value` only) versus the fixed way:

```
OLD reader: Unterminated string in JSON at position 512 (line 1 column 513)
NEW reader bytes: 1226 parses: true identical: true
```

Byte-for-byte the live console string. The fixture payload is 1 226 bytes → slices `[512, 512, 202]`
→ one packed leaf whose `value` is slice 0 and whose `dataAttributes` `01`/`02` carry slices 1 and 2.

## Concurrency note

Between the diagnosis and the edit, a sibling lane landed the same reassembly repair: it extracted
`packedTextLeaf` into `🔌️PluginRuntime/packed-text.ts` (05:37) and pointed BOTH readers at it —
`sectionValueFromBuiltNode` and the Interpreter's `surfaceSceneLaneText`. That half is therefore
theirs, and the two readers now share one implementation. This lane's own additions are the typed
fault, the producer attribution at both call sites, and the language-neutral fixture plus the Rust
and TypeScript tests that pin the pack against it.

One follow-up for whoever owns the taxonomy gate: `packed-text.ts` is an ASCII filename inside an
emoji-named taxonomy (`📓️taxonomy-violations-audit-2026-09-10.md`'s subject) — nothing in this lane
depends on where it lands, but it will need a taxonomy-conforming home.

## Results

All runs foreground/`nohup`-polled, `CARGO_TARGET_DIR=$S/target-j512` seeded by `cp -Rc` from
`target/debug`, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`. Tails in `🗑️generated/j512-*.txt`.

| run | result |
|---|---|
| `cargo check -p semio-framework-plugin --all-targets --keep-going` | `Finished dev profile in 41.68s`, **0 errors**, 93 warnings (the warning count is the proof the `lib test` unit — which `include!`s the new test — actually type-checked) — `j512-2.txt` |
| `cargo test -p semio-framework-plugin --lib packed_section_carrier -- --nocapture` | `test result: ok. 1 passed; 0 failed` · `[DEBUG] neutral fixture pinned: 1226 bytes packed into 1 value slice and 2 dataAttributes slices` — `j512-3.txt` |
| `cargo check -p semio-s-plugin-procedural --keep-going` (native) | `Finished dev profile in 1m 38s`, **0 errors** — `j512-4.txt` |
| `cargo check -p semio-s-plugin-procedural --keep-going --target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | `Finished wasm-dev profile in 3m 14s`, **0 errors** — `j512-5.txt` |
| `SEMIO_TEST_LEVEL=long vitest run` (`@semio-tech/framework-renderer-react`) | **753 passed / 753**; 19 of 20 suites pass — `j512-ts-long.txt` |
| new TS case `reassembles a PACKED carrier leaf …` | ✓ 17 ms · `[DEBUG] packed carrier leaf reassembled 1226 bytes from 1 value slice and 2 dataAttributes slices` — `j512-ts-packed.txt` |
| new TS case `raises a typed fault naming the reserved section …` | ✓ 39 ms · `[DEBUG] truncated carrier raised plugin-ui.section-payload-not-json naming framework.section.measures and its producer instead of a bare SyntaxError` — `j512-ts-fault.txt` |

Pre-existing, unrelated, NOT caused by this lane: the suite
`🧪️tests/🧩️package-integration/🟦️.ts` fails to LOAD with `ReferenceError: self is not defined` from
`🎯️targets/🧊️wgpu/…/🐚️plugin-bridge.ts:159` — a worker-scope module (`self.addEventListener`) that
the jsdom suite imports. That file is clean in the working tree (committed, wgpu lane's).

### Environment hazard hit during this lane

The box ran **out of disk** at 05:38 (`/` at 100 %, 117 MiB free): `Write` and even the harness's own
Bash output files failed with `ENOSPC`. It recovered to 15–40 GiB as peers freed space, but free
space oscillated by tens of GiB throughout. `$S` alone held ~300 GiB across fourteen sibling lane
`target-*` dirs. Nothing was deleted from any peer's directory by this lane; the coordinator should
schedule a sweep of the retired lanes' target dirs.

## Files changed

* `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🔬️paged-text-carrier/🔣️.json` (new, language-neutral)
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
