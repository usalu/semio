# Wave B43 — `segmented-download-worker-limit`: the fault is a TYPE check, not a limit

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12 · battery `🗑️generated/probe-2026-09-12T08-28-10.md` (wasm #56).
Written incrementally. Every command ran in the FOREGROUND with its tail quoted.

Inputs read first: `📓️2026-09-12-wave-B38-probe-recipes-export-segments.md` §2 (the whole segmented lane),
the served jco module `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/semio_s_plugin_puzzle_component.js`.

---

## 0 Headline

**No limit is exceeded.** Nakagin's 145 714 B export is 36 chunks of ≤ 4 096 B against a 64-slot table
(`PUZZLE_COMMAND_OUTPUT_BYTES / CHUNK_BYTES = 262 144 / 4 096`) — inside every declared bound, chunk
bytes, slot count and total alike. The guard that throws is a four-way `||` whose FIRST arm is a type
discriminator, and that arm is what fires: the bridge hands the worker jco's **tagged option object**
`{ tag: "some", val: Uint8Array }` instead of a `Uint8Array`, because it is the one guest export in
`pluginComponentBridgeSource` that does not pass its result through the bridge's own `unwrapOption`.

Consequences, both fatal: chunk **0** faults the shard worker (no chunk is ever delivered), and the
terminal sentinel `{ tag: "none" }` is not `undefined` either — so even with the type arm relaxed the
drain could never terminate.

---

## 1 The limit, and where it fires — `file:line`

| hop | where | what it declared | what it did |
|---|---|---|---|
| 0 | `✏️editor/🎮️commands/📤️export-fixture/🦀️.rs:55` `puzzle3d_export_segmented` | slices by `ArtifactOutputChunks::CHUNK_BYTES` (4 096), caps at `PUZZLE_COMMAND_OUTPUT_BYTES` (262 144) | **correct.** 145 714 B → 36 pages, last 2 354 B; 36 of 64 slots |
| 1 | `🔌️plugin/🦀️.rs:12952` `ARTIFACT_OUTPUT_CHUNK_BYTES = 4_096` and `:13044` `push` | rejects an empty/oversized chunk with `interactive-job.segmented-output-limit` | **never fired** |
| 2 | `🔌️plugin/🦀️.rs:27138` `take_segmented_download_chunk` | `take_chunk()` pops ONE chunk (the outstanding table retires as it is read), terminal `None` removes the registry entry | **correct.** The native law drains all 36 and reassembles byte-for-byte |
| 3 | **`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:732`** at HEAD, `:785` now (`pluginComponentBridgeSource`'s `createActorApi`) | — | **THE DEFECT.** `takeSegmentedDownloadChunk: async (…) => jobs.takeSegmentedDownloadChunk(…)` — the guest value handed on RAW |
| 4 | `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:538` at HEAD, `:582` now (`shardWorkerSource`) | `if (chunk !== undefined && chunk !== null && (toString.call(chunk) !== "[object Uint8Array]" \|\| byteLength === 0 \|\| byteLength > 4096)) throw new Error("segmented-download-worker-limit")` | **fires on its FIRST arm** — the type discriminator, not any byte or slot bound |

### 1.1 Why the type arm fires: jco lifts `option<t>` as a TAGGED variant

`jobs.take-segmented-download-chunk` returns `result<option<list<u8>>, plugin-error>`. In the served
module (`🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/semio_s_plugin_puzzle_component.js`, wasm #56's own bytes):

- `:10469` `trampoline23` lifts that return with `_liftFlatResult({ ok: _liftFlatOption({ some: _liftFlatList(Uint8Array) }) })`;
- `:3632` `_liftFlatOption(meta)` is `_liftFlatVariant(meta)` **verbatim** — unlike `:3619` `_liftFlatEnum`,
  which reduces its result to `.tag`, the option lifter does not unwrap anything;
- `:3434`/`:3446` `_liftFlatVariant` builds `val = { tag }` or `val = { tag, val: newVal }`;
- `:9210` the export wrapper unwraps ONE level (`taskRes.tag === "ok" → taskRes.val`), so the inner option
  survives as a tagged object.

So the guest export answers `{ tag: "some", val: Uint8Array(4096) }` and, at the end,
`{ tag: "none" }`. `Object.prototype.toString.call({…})` is `[object Object]`.

The bridge already knows this — its OWN helper says so at `🟦️.ts:686`:

> `jco lifts option<t> as a tagged { tag: "none" | "some" } variant; every host-side reader below wants the bare value (or nothing)`

…and `poll` routes `nextWake`, `lifecycleReceipt`, `uiPatchReceipt` and `commandIngress` through
`unwrapOption`. `takeSegmentedDownloadChunk` was the one guest export that did not.

Two consequences, both fatal, and both consistent with the battery:

1. chunk **0** faults the shard worker — no chunk is ever delivered, so `export-only` /
   `export-names-the-example` stay `download=none`;
2. `{ tag: "none" }` is not `undefined` either, so even with the type arm relaxed the drain could never
   see its terminator and would run to its own cap instead.

### 1.2 Why the native law was green

`export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture` (B38 §2.5)
drains through `Puzzle3dSettled.downloads` → the Rust `take_chunk` directly. jco's lifting is not on that
path at all. Every HOST-side law of this lane (`ShardClient segmented-download transport`, the seven drain
laws) stubs the worker's REPLY with an already-unwrapped `Uint8Array` — so the guest's real wire shape
appeared in no law in the repo.

---

## 2 The fix — ONE declared chunk contract, four enforcing hops

### 2.1 The contract (new, schema-first)

`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📤️segmented-download/` — a zero-import leaf beside the
transport that owns the wire:

| file | what |
|---|---|
| `🧬️schema/🔣️.json` | `https://semio.tech/schema/framework/actor/shard-client/segmented-download/schema.json#/$defs/SegmentedDownloadContract` |
| `🧫️fixtures/🔣️.json` | `contract`: `chunkBytes 4096`, `maximumOutstandingChunks 8192`, `maximumTotalBytes 33554432`, `maximumOperationId "18446744073709551615"`; plus the six `refusals` codes |
| `🟦️.ts` | `SEGMENTED_DOWNLOAD_CONTRACT`, `SEGMENTED_DOWNLOAD_REFUSAL`, `admitSegmentedDownloadChunk`, `admitSegmentedDownloadOperationId` |

It is a leaf and not part of `📮️shard-client/🟦️.ts` because the DRAIN (an OS renderer element) must read
it too, and `🎠️kernel/🟦️.ts` imports `ShardClient` — so the contract cannot live anywhere `ShardClient`
itself cannot reach without a cycle.

The four hops, each of which used to carry its own `4_096` (and its own 32 MiB):

| hop | before | after |
|---|---|---|
| guest producer | `🔌️plugin/🦀️.rs:12952` `ARTIFACT_OUTPUT_CHUNK_BYTES = 4_096`, no total cap at all | same constant + new `ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES`, both held equal to the fixture by a Rust law; `ArtifactOutputChunks::{MAXIMUM_TOTAL_BYTES, admit_maximum}` refuse an over-cap budget AT CONSTRUCTION |
| generated worker | `🟦️.ts:241` `const MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4096` (a literal of the worker's own) | `const SEGMENTED_DOWNLOAD_CHUNK_BYTES = ${SEGMENTED_DOWNLOAD_CHUNK_BYTES}` — interpolated from the package's declared mirror, exactly as `PROGRESS_HEARTBEAT_INTERVAL_MS` already was |
| host transport | `📮️shard-client/🟦️.ts:362-363` two literals + hand-rolled checks | `admitSegmentedDownloadOperationId` + `admitSegmentedDownloadChunk` |
| host drain | `📤️SegmentedDownload/🟦️.ts:6-7` `MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES` / `MAX_SEGMENTED_DOWNLOAD_BYTES` + `MAX_U64` | imports the contract; `admitSegmentedDownloadChunk` per item, `maximumTotalBytes` as the default cap, and a NEW `maximumOutstandingChunks` loop bound |
| Layout plugin | `📤️export/🦀️.rs:41,45` its own `32 << 20` and `4_096` (B38 §2.4 left this derivable) | `ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES` / `::CHUNK_BYTES` |

### 2.2 The defect fix

`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` `pluginComponentBridgeSource`:

```js
takeSegmentedDownloadChunk: async (instanceId, operationId) => unwrapOption(await jobs.takeSegmentedDownloadChunk(instanceId, operationId)),
```

### 2.3 One refusal per bound, never one "limit" for four violations

The worker's single `segmented-download-worker-limit` becomes `segmented-download-chunk-type` /
`-chunk-empty` / `-chunk-over-cap`; the transport's `segmented-download-transport-type` /
`-transport-limit` and the drain's `segmented-download-chunk-limit` / `-total-limit` collapse into the
SAME six declared codes (`…-chunk-type`, `…-chunk-empty`, `…-chunk-over-cap`,
`…-outstanding-over-cap`, `…-total-over-cap`, `…-authority-invalid`). This is why the live battery read
as an exceeded byte cap while nothing was over any cap at all.

### 2.4 Retirement on drain

Unchanged in mechanism, now asserted: `ArtifactOutputChunks::take_chunk` pops the queue (so each take
retires one outstanding slot) and the terminal `None` removes the registry entry
(`🔌️plugin/🦀️.rs:27138`). The drain's new `maximumOutstandingChunks` bound means a producer that never
answers `None` is refused instead of spun on forever — the `while (true)` had no iteration bound at all.

### 2.5 Over-cap is a notice, never a fault

`✏️editor/🎮️commands/📤️export-fixture/🦀️.rs`: new `puzzle3d_export_segmented_budget_bytes()` =
`ArtifactOutputChunks::admit_maximum(PUZZLE_COMMAND_OUTPUT_BYTES)`, new
`Puzzle3dExportPublication::Refused(String)` and one shared `puzzle3d_export_refusal(...)` message, so
BOTH arms answer identically: the interactive arm publishes `Complete(Emit::effect(Effect::Notify …))`
(`✏️editor/🦀️.rs:3904`), the leftover arm pushes the same notice.

---

## 3 Laws

### 3.1 The bridge adaptation — the REAL generated bytes, not a transcription

`🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts`, inside the existing
`ShardClient segmented-download transport` describe:

> `unwraps the guest's tagged option in the generated bridge bytes, so no hop ever sees [object Object]`

It writes `pluginComponentBridgeSource("guest", "guest.wasm")` to a temp dir beside a stub guest module
whose `jobs.takeSegmentedDownloadChunk` returns exactly what jco's `_liftFlatVariant` builds
(`{ tag: "some", val: Uint8Array(4096) }`, then `{ tag: "none" }`), imports the real bridge and calls
`createActorApi(...).takeSegmentedDownloadChunk(1, 91n)`. Asserts a plain `Uint8Array` out, `undefined`
for the terminator, and — as the independent oracle for the same bytes — that the RAW tagged value is
what `admitSegmentedDownloadChunk` refuses.

**Real coverage, proven** (only the bridge line reverted to `jobs.takeSegmentedDownloadChunk(...)`):

```
× ShardClient segmented-download transport > unwraps the guest's tagged option in the generated bridge bytes, so no hop ever sees [object Object]
  → expected '[object Object]' to be '[object Uint8Array]' // Object.is equality
  Received: "[object Object]"
      Tests  1 failed | 235 skipped (236)
```

`[object Object]` — the live symptom, reproduced from the generated bridge itself.

### 3.2 The worker contract — 145 KB and `maximumTotalBytes - 1`, plus one refusal per bound

> `streams a 145 KB and a maximumTotalBytes-1 download through the generated worker and names each violated bound`

`shardWorkerSource()` runs in a `node:vm` context (the same harness the liveness law uses) and is driven
through its own dispatcher: `145 714` B → 36 chunks, `33 554 431` B → 8 192 chunks, every one a real
`Uint8Array` ≤ `chunkBytes`, totals exact, chunk counts `≤ maximumOutstandingChunks`; then the three
poisoned answers are refused as `…-chunk-type` / `…-chunk-empty` / `…-chunk-over-cap`. It also asserts
`SEGMENTED_DOWNLOAD_CHUNK_BYTES === SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes`.

### 3.3 The mirror law, extended

`mirrors the schema-owned policy record in every consumer of it` now also validates the contract fixture
against its schema (Ajv, strict), asserts `SEGMENTED_DOWNLOAD_CONTRACT` ≡ `contract`, the package's
declared `SEGMENTED_DOWNLOAD_CHUNK_BYTES` ≡ `contract.chunkBytes`,
`chunkBytes × maximumOutstandingChunks === maximumTotalBytes`, the refusal codes ≡ the fixture's
`refusals`, and that `shardWorkerSource()` really interpolates that number.

### 3.4 The guest half

`🔌️plugin/🧪️tests/📤️segmented-download/🦀️.rs` (new, mounted from `🔌️plugin/🦀️.rs`), reading the SAME
fixture through `include_str!`:

- `segmented_download_constants_mirror_the_schema_owned_contract`
- `segmented_download_admits_exactly_the_declared_total_budget`
- `segmented_download_slot_table_retires_every_chunk_as_it_is_taken` — 145 714 B → 36 slots, and after
  every `take_chunk` the outstanding count is `expected - taken`, ending at 0 with the bytes reassembled

---

## 4 Verification — every command in the FOREGROUND, tails quoted

### 4.1 `@semio-tech/framework-actor` vitest (the contract + transport + worker + bridge laws)

```
SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts --testNamePattern='segmented-download|schema-owned policy'
 ✓ ShardClient segmented-download transport > preserves operation identity and last-Some then None ordering
 ✓ ShardClient segmented-download transport > propagates unknown-operation errors without manufacturing a terminal None
 ✓ ShardClient segmented-download transport > rejects oversized or empty response items
 ✓ ShardClient segmented-download transport > rejects an in-flight read when actor disposal cancels its transport ownership
 ✓ ShardClient segmented-download transport > preserves the complete u64 operation authority through structured clone
 ✓ ShardClient segmented-download transport > rejects zero and overflowing operation authorities
 ✓ ShardClient segmented-download transport > unwraps the guest's tagged option in the generated bridge bytes, so no hop ever sees [object Object]
 ✓ ShardClient segmented-download transport > streams a 145 KB and a maximumTotalBytes-1 download through the generated worker and names each violated bound  1937ms
 ✓ ShardClient liveness watchdog (schema-owned timelines) > mirrors the schema-owned policy record in every consumer of it
      Tests  9 passed | 227 skipped (236)
```

Baseline before the wave: the same describe reported `Tests 6 passed | 228 skipped (234)`.

Full lane: `Test Files 6 failed | 5 passed (11)`, `Tests 15 failed | 221 passed (236)`. **None of the 15 is
new.** The two that touch shard-client are proven pre-existing against `HEAD`:

- `beats at every generated-worker activation boundary…` asserts the generated worker source contains no
  `@vite-ignore` after one replacement, but the template has had **two** since `armGuestRuntimeDiagnostics`
  landed — `git show HEAD:…🔌️plugin/📦️packages/🟦️typescript/🟦️.ts | rg -c "@vite-ignore"` → `2`
  (worktree: also `2`);
- `ShardClient … expected [42 fields] to deeply equal [40]` is the field inventory, and
  `git show HEAD:…📮️shard-client/🟦️.ts | rg -c "actorsPastFirstTurn"` → `4`.

The remaining 13 are in `📤️return/`, `🚪️lifetime/`, `🪪️activation/` — files this wave never touched.

### 4.2 `@semio-tech/framework-renderer-react` vitest (the drain)

```
… --testNamePattern='segmented'
 ✓ 🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts > segmented download marker > accepts only exact version-one markers and canonical u64 operation ids
 ✓ … > segmented download drain > fails closed when the browser exposes no real streaming file sink
 ✓ … > segmented download drain > awaits chunks sequentially and preserves identity ordering
 ✓ … > segmented download drain > decodes base64 across producer boundaries without reordering
 ✓ … > segmented download drain > aborts the sink when cancellation lands during an awaited producer read
 ✓ … > segmented download drain > rejects an unknown-operation error before the required None terminator
 ✓ … > segmented download drain > fails closed on per-chunk and total-cap overflow
 ✓ … > segmented download assembled sink > assembles every chunk in order and delivers the payload once
 ✓ … > segmented download assembled sink > delivers nothing when the drain aborts
      Tests  9 passed | 219 skipped (228)
```

Full lane: `Test Files 12 failed | 18 passed (30)`, `Tests 4 failed | 251 passed (255)`. All 12 file
failures are `Failed to resolve import "…🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/…"` /
`"…🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts"` — a peer's in-flight relocation of
`🎯️targets` (`git status`: `?? 🧰️framework/🔨️modules/🖱️ui/🎯️targets/`, and
`R …🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts -> …🧊️wgpu/🐚️plugin-bridge/🟦️.ts`). The 4 test
failures are `TypeError: themeColorVar is not a function` (×2), `Cannot find module
'../📦️deployment/🟦️.ts'` from `📇️registry/🤖️generated/🧩️plugins/🟦️.ts`, and two plugin-runtime wire shape
mismatches — none mentions the segmented lane.

### 4.3 `tsc --noEmit` (scoped to the touched modules)

```
bun node_modules/typescript/bin/tsc --noEmit --strict … 📤️segmented-download/🟦️.ts 📤️SegmentedDownload/🟦️.ts
(no output)
```

`📮️shard-client/🟦️.ts` + its law, `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`, the ShellHelpers component law
and `🛠️ShellHelpers/🟦️.tsx` were run the same way and report **zero** diagnostics naming the segmented
lane or any of the new exports (this loose invocation does surface pre-existing repo-wide `ImportMeta.dir`
/ implicit-`any` noise, none of it in the changed lines).

### 4.4 `semio-framework-plugin` Rust laws

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib segmented -- --test-threads=1
test component::app::artifact_fixed_registry_tests::duplicate_id_never_drops_active_media_snapshot_or_segmented_download_ownership ... ok
test component::app::artifact_media_export_credit_tests::segmented_output_accepts_exact_cap_and_rejects_plus_one_or_foreign_authority ... ok
test component::app::artifact_media_export_credit_tests::segmented_output_preallocates_exact_former_growth_boundary_and_drains_terminal_storage_to_zero ... ok
test component::app::artifact_media_export_credit_tests::segmented_output_seal_is_linearly_ordered_with_push ... ok
test component::plugin_runtime::plugin_builder_contract_tests::segmented_download_remains_addressable_until_terminal_none_is_observed ... FAILED
test component::segmented_download_contract::segmented_download_admits_exactly_the_declared_total_budget ... ok
test component::segmented_download_contract::segmented_download_constants_mirror_the_schema_owned_contract ... ok
test component::segmented_download_contract::segmented_download_slot_table_retires_every_chunk_as_it_is_taken ... ok
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 673 filtered out; finished in 0.02s
```

The one red is **pre-existing and whole-module**: it panics in
`🏪️store/🦀️.rs:17917 "artifact store reached Drop without its exact terminal-empty shallow-shell witness"`,
and `cargo test -p semio-framework-plugin --lib plugin_builder_contract_tests` shows **every** test in that
module FAILED on the same store-drop witness — a peer's store change, nothing to do with chunk bounds
(that law calls `ArtifactOutputChunks::new(4)` directly and never touches `admit_maximum`).

### 4.5 `cargo check`

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly   → 0 errors (110 warnings)
cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2                    → Finished `dev` profile, 0 errors
cargo check -p semio-s-artifact-layout-layout                                  → 0 errors
```

### 4.6 `semio-s-artifact-puzzle-3d` export laws — B38's still green, plus this wave's refusal

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib export -- --test-threads=1
test editor::puzzle3d::component::unit_tests::export_fixture_downloads_round_trippable_json ... ok
test editor::puzzle3d::component::unit_tests::export_fixture_names_the_download_after_the_active_example ... ok
test editor::puzzle3d::component::unit_tests::export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture ... ok
test editor::puzzle3d::component::unit_tests::export_refuses_a_payload_above_the_declared_segmented_budget_with_a_notice ... ok
test editor::puzzle3d::component::unit_tests::exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity ... ok
test editor::puzzle3d::component::unit_tests::import_fixture_reproduces_the_exported_document ... ok
test editor::puzzle3d::component::unit_tests::leftover_export_fixture_downloads_the_boot_example_json ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 729 filtered out; finished in 23.56s
```

B38's `export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture` is green with
the budget now routed through `ArtifactOutputChunks::admit_maximum`, and the new
`export_refuses_a_payload_above_the_declared_segmented_budget_with_a_notice` passes beside it (6 → 7 in this
filter).

🧯 This run took four attempts across ~40 minutes: `semio-framework-graph`'s gitignored generated registry
(`🧰️framework/🔨️modules/🕸️graph/🤖️generated/📇️registry/…`) kept vanishing and reappearing leaf by leaf under a
peer's regeneration, and its own `framework-graph:generate` was red in between
(`error: graph output path is not an exact safe identity`). Polling until the test build compiled was the
only way through; nothing about it was attributable to this change.

---

## 5 Live on :6013 — restaged, and blocked by the host entry, not by the fix

The fix splits across two delivery mechanisms, and `:6013` serves the **release** staging dir
(`🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules`, proven by byte-matching the served
`semio_s_plugin_puzzle_component.js` at 404 567 B against that directory and not the two other candidates):

| half | mechanism | state |
|---|---|---|
| `📮️shard-client` transport + `📤️SegmentedDownload` drain + the new contract leaf | vite-live from source | live |
| `🟨️shard-worker.js` | materialized by `support-{dev,release}` | **restaged**, both profiles |
| `🌉️bridge.js` | materialized by `materialize-{dev,release}` | **restaged**, both profiles |

```
bun nx run @semio-tech/framework-plugin-web:support-release --skip-nx-cache        → EXIT=0
… dist/release/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js:393
          if (Object.prototype.toString.call(chunk) !== "[object Uint8Array]") throw new Error("segmented-download-chunk-type");

bun ./📜️script.ts materialize release --manifest ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml  → EXIT=0
curl … /🔌️plugin-modules/🧩️puzzle/🌉️bridge.js
162:    takeSegmentedDownloadChunk: async (instanceId, operationId) => unwrapOption(await jobs.takeSegmentedDownloadChunk(instanceId, operationId)),
```

**No wasm rebuild was needed** — `materialize` re-transpiles the EXISTING component artifact, and the guest
Rust the export lane executes is unchanged for Nakagin (`puzzle3d_export_segmented_budget_bytes()` returns
the same `PUZZLE_COMMAND_OUTPUT_BYTES = 262 144`; only the over-cap branch is new, and 145 714 B does not
reach it). So the served guest is still **#56's wasm**, with the bridge and worker fixed.

The verdicts still could not be measured: the app does not boot on `:6013` at all right now, before any
gesture, and not because of this change.

```
bun 🔍️browser-probe.ts --only=example-switch,export-import --port=6013
[15.8s] boot waiting… windows=0 canvases=0 faults=0
…
[181.2s] done booted=false faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=0
## console tail
error: Failed to load resource: the server responded with a status of 500 (Internal Server Error)
```

The 500 is the app's own entry module:

```
curl http://127.0.0.1:6013/🟦️.ts  → 500
{"message":"Failed to resolve import \"@semio-tech/framework-renderer-react\" from \"../../🟦️.ts\". Does the file exist?"}
```

The package and its `exports["."] → ./🟦️.tsx` both exist on disk; the `node_modules/@semio-tech/framework-renderer-react`
symlink was **recreated at 11:31** by a peer's install, while this vite server has been up since **07:35** —
it is holding a stale resolution. The same relocation is what fails 12 files in §4.2. Two probe runs
(`probe-2026-09-12T10-20-55.md`, `probe-2026-09-12T10-27-44.md`, both in `🗑️generated`) are identical: the
first predates the release restage, so the restage is not the cause.

**Handover recipe** (nothing else is required — the staging dirs already carry the fix):

1. restart the `:6013` vite server (this wave must not kill a shared process);
2. `bun T/🔍️browser-probe.ts --only=example-switch,export-import --port=6013`;
3. expect `export-only` and `export-names-the-example` to PASS on Nakagin. If a chunk is still refused, the
   error now names WHICH bound (`…-chunk-type` / `…-chunk-empty` / `…-chunk-over-cap` /
   `…-outstanding-over-cap` / `…-total-over-cap`) instead of one undifferentiated "limit".

A full #57 (a guest rebuild) is separately blocked: `@semio-tech/puzzle-plugin:component-dev` fails with
`error[E0425]: cannot find type DocumentArchivePack in crate protocol` at `🔌️plugin/🦀️.rs:11876` under
`--target wasm32-wasip2` + the component features — a peer's `protocol` refactor, and not needed for this
proof (`🗑️generated/b43-materialize-dev.txt`).

---

## 6 Verdicts

| claim | verdict |
|---|---|
| the limit that fires | **none.** `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:538` (HEAD) fired on its TYPE arm; 145 714 B is 36 chunks of ≤ 4 096 B in a 64-slot table, inside every declared bound |
| the cause | `…🟦️.ts:732` (HEAD) `pluginComponentBridgeSource` handed jco's tagged `option<list<u8>>` (`{ tag, val }`) to the shard worker instead of routing it through the bridge's own `unwrapOption`, as all its other guest options already are |
| fixed | **yes** — one line, plus the contract that makes the whole lane read one number and one refusal vocabulary |
| the contract is ONE declared record | **yes** — `📮️shard-client/📤️segmented-download/` schema + fixture, mirrored by the drain, the transport, the generated worker (interpolated) and the Rust producer (asserted by a law), with the Layout plugin's two duplicate literals now derived too |
| retirement on drain | **yes**, and now asserted; the drain's unbounded `while (true)` gained the contract's `maximumOutstandingChunks` bound |
| over-cap refuses with a notice | **yes** — `ArtifactOutputChunks::admit_maximum` refuses at construction, `Puzzle3dExportPublication::Refused` carries the message through both arms |
| worker-level law | **green** (145 KB and `maximumTotalBytes - 1` both stream; each bound named) |
| bridge law with real coverage | **green**, and RED (`[object Object]`) with the one line reverted |
| B38's guest law still green | **green**, alongside the new over-cap refusal law — `7 passed; 0 failed` (§4.6) |
| live `export-only` / `export-names-the-example` | **UNPROVEN** — `:6013` returns 500 on its own entry module from a stale `@semio-tech/framework-renderer-react` resolution (§5). Both halves ARE restaged; the guest rides #56's unchanged wasm |

---

## 7 Files

**New**

- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📤️segmented-download/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📤️segmented-download/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📤️segmented-download/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📤️segmented-download/🦀️.rs`

**Changed**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` — the bridge `unwrapOption` (the defect), `SEGMENTED_DOWNLOAD_CHUNK_BYTES` declared + interpolated, the worker's guard split into three named refusals
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` — two literals removed, contract imported and re-exported, `takeSegmentedDownloadChunk` admits through it, test-deps updated
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` — two new laws, mirror law extended, refusal codes updated
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📤️SegmentedDownload/🟦️.ts` — contract-driven, `maximumOutstandingChunks` loop bound
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — re-exports the contract instead of the two `MAX_*`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts` — the drain laws read the contract and assert per-bound codes
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — one comment reference
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — the two constants with their contract docs, `ArtifactOutputChunks::{MAXIMUM_TOTAL_BYTES, admit_maximum}`, the new test mount
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🎮️commands/📤️export-fixture/🦀️.rs` — `puzzle3d_export_segmented_budget_bytes`, `puzzle3d_export_refusal`, `Puzzle3dExportPublication::Refused`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` — the `Refused` arm publishes a notice
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the over-cap refusal law
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/…/⚙️engine/📤️export/🦀️.rs` — its two duplicate literals derived from the contract
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` + `📓️schema-catalog.md` — regenerated by `bun ./📜️script.ts schema generate` for the new schema module (`framework.actor.shard-client.segmented-download`)
