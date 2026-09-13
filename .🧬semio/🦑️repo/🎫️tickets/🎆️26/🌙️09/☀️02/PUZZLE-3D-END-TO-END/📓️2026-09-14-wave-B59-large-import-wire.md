# Wave B59 — the large-import wire

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-13 · wasm **#61** on `:6013`; the fixes here ride **#62**.
Written incrementally. Every command ran in the FOREGROUND with its tail quoted.

Inputs read first: `📓️2026-09-14-wave-B57-export-recipe-show-restore.md` §2.3 (the red verdict),
`🔌️PluginRuntime/🟦️.tsx:2900-2922` (`performInvocation`, the `importFixture ingress` tap) and `:2350-2400`
(`runQueuedTurn`'s `createShardCommandIngressPages` loop), `📮️shard-client/🟦️.ts:114-150`
(`SHARD_COMMAND_MAXIMUM_PAGES`, `ACTOR_BYTE_PAGE_BYTES`), `📡️spr/🧵️channel/🦀️.rs:137-139,400-437`
(`COMMAND_PAGE_MAXIMUM_BYTES`, `read_bounded_bytes`), `🔌️plugin/🦀️.rs:34271-34360`
(`PluginCommandIngress`), `🎮️commands/📥️import-fixture/🦀️.rs`, `🎮️commands/📤️export-fixture/🦀️.rs`
(the OUTBOUND lane this wave mirrors), `✏️editor/🦀️.rs:7113-7170` (`PUZZLE3D_IMPORT_RAW_BYTES`,
`Puzzle3dRetainedCommandJobFactory`), `🎮️commands/🧵️retained/🦀️.rs` (the retained command job),
`🛠️ShellHelpers/🟦️.tsx:774-785` (`dispatchOpenedFiles`).

---

## 0 Headline

The import is **not** dropped on the host→guest wire. Every host hop carries the 145 924-byte payload
correctly and the guest ADMITS it. The drop is inside the guest's own retained command job: its wire
ladder advanced **one byte per step** and published a checkpoint on each of those steps whose
`input_hash` **re-folded the whole buffer**, so assembling a 160 314-byte command cost O(bytes²) and the
operation never reached its `Decode` phase. `import_fixture` therefore never ran — no document edit, no
history row, and (because nothing refused) no notice. That is exactly `paneObjects=180→180`.

Root cause, file:line (pre-fix): `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:510-515`
(`PuzzleCommandPhase::WireBytes`, `raw_scan_cursor.saturating_add(1)`) together with `:411`
(`checkpoint_state`'s `puzzle_checkpoint_hash([&self.raw[..self.raw_len]])`).

## 1 The hop table

Measured natively (§2) and read off B57's live run. `payload` = the 145 924-char JSON text.

| # | hop | file:line | carries the payload? | bound | verdict |
|---|---|---|---|---|---|
| 0 | file chooser → import args | `🛠️ShellHelpers/🟦️.tsx:783` `dispatchOpenedFiles` | yes, `{payload, name}` in ONE arg | none declared | passes, unbounded by construction |
| 1 | renderer edge | `🔌️PluginRuntime/🟦️.tsx:2909` `importFixture ingress` tap | yes — live `payloadLen=141574` | none | passes |
| 2 | pack encode | `🔌️PluginRuntime/🟦️.tsx:2916` `encodePackValue(invocation)` | yes, one contiguous string field | none | passes |
| 3 | command pages | `📮️shard-client/🟦️.ts:128` `createShardCommandIngressPages` | yes, `ceil(bytes/4096)` pages | `SHARD_COMMAND_MAXIMUM_PAGES = 64` → 262 144 B | passes (≈40 pages) |
| 4 | guest ingress assembly | `🔌️plugin/🦀️.rs:34327` `PluginCommandIngress::step` | yes | `COMMAND_INGRESS_MOVES_PER_TURN = 64+8` | passes |
| 5 | `AppCommand::Command` field decode | `📡️spr/🧵️channel/🦀️.rs:1689` `read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)` | yes | `COMMAND_MAXIMUM_BYTES = 262 144` | passes |
| 6 | tool-job admission | `✏️editor/🦀️.rs:7163` `input.declared_bytes() > contract.max_raw_wire_bytes` | yes | `PUZZLE3D_IMPORT_RAW_BYTES = 262 144` | passes — 160 314 B admitted |
| 7 | retained wire pages → `raw` | `🧵️retained/🦀️.rs:500-508` `WirePages` | yes | one page per step | passes, 40 steps |
| 8 | **retained wire ladder** | `🧵️retained/🦀️.rs:512` `WireBytes` | — | **one BYTE per step, each publishing an O(n) checkpoint** | **FAILS: never completes** |
| 9 | `decode_op` → `Puzzle3dCommand` | `🧵️retained/🦀️.rs:521` | never reached | — | unreached |
| 10 | `import_fixture` arm | `🎮️commands/📥️import-fixture/🦀️.rs:9` | never reached | — | unreached |
| 11 | refusal / notice | `import_fixture`'s `ctx.notice(import_invalid)` | never reached | — | **no notice exists for an over-budget or never-assembled import** |

The two ceilings the brief named are NOT the cause: 8 KiB is the SPR page size (hop 3/5 page the payload
correctly), and `PUZZLE3D_IMPORT_RAW_BYTES`/`COMMAND_MAXIMUM_BYTES` are both 262 144, above the payload.
`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` (65 536) IS violated, at hop 7 — `raw: vec![0; declared_bytes]`
(`🧵️retained/🦀️.rs:369`) is one 160 314-byte contiguous guest allocation — but that violation is silent,
not the drop.

## 2 The native reproduction

New law `a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs`): load Nakagin (180 objects), add one, export that document's own JSON,
return to Nakagin, import the 181-object text. Every import law before it fed a payload of a few hundred
bytes, so the product's size class was never under test in this crate.

Before the fix, with a temporary `[DEBUG] retained wire-byte scan cursor=…` tap in the `WireBytes` arm:

```
[DEBUG] retained wire-byte scan cursor=0 of 95          ← setActiveExample, 95 B: invisible
[DEBUG] B59 import payload bytes=145924 seeded=180
[DEBUG] retained wire-byte scan cursor=0 of 160314
[DEBUG] retained wire-byte scan cursor=8192 of 160314
[DEBUG] retained wire-byte scan cursor=16384 of 160314
[DEBUG] retained wire-byte scan cursor=24576 of 160314
panicked at …🔬️unit/🦀️.rs:395: puzzle3d app never quiesced: pending typed operations outlived the settle budget
test result: FAILED. 0 passed; 1 failed; … finished in 63.85s
```

28 672 of 160 314 bytes in 60 s — the ladder is quadratic, and the settle budget (1 048 576 turns) is not
the shortfall. The tap has been removed.

---

## 3 The design

Two defects, two fixes. The first makes the import COMPLETE; the second stops it asking the fixed guest
heap for a block it cannot grow — which, measured at hop 5, no single-command import can avoid.

### 3.1 The ladder: one PAGE per step, and an O(1) checkpoint

`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs`

- `PUZZLE_COMMAND_WIRE_SCAN_STRIDE_BYTES` (new, = `semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES`,
  derived): the `WireBytes` ladder advances by one WIRE PAGE per step instead of one byte. The ladder exists
  to make the scan resumable at the granularity the pages arrived in, not to spend a host turn per byte —
  160 314 steps became 40. The stage id says what it does now: `puzzle-command-wire-page-scan`.
- `raw_hash` (new field) + `puzzle_fold_checkpoint_hash` (new): the checkpoint's `input_hash` is folded
  ONCE per admitted page as the pages land, instead of re-folding `raw[..raw_len]` on every checkpoint.
  FNV-1a is a sequential fold, so the value is bit-identical to `retained_input_hash`'s walk over the same
  pages and the resume authority (`validate_wire_checkpoint`) is unchanged.

This is the whole fix for the silent drop, and it is shared by every retained puzzle command — puzzle 2D
and 5D carried the same quadratic ladder.

### 3.2 The lane: a chunked INBOUND import, mirroring the segmented DOWNLOAD

The outbound direction has had a real lane since B38/B43: `puzzle3d_export_publication` picks INLINE at or
under `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, SEGMENTED up to `PUZZLE_COMMAND_OUTPUT_BYTES`, and a
NOTICE above it. Inbound had nothing: one contiguous string, no budget, no refusal, no notice.

**Host** — `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx`

- `IMPORT_CHUNK_BYTES = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2` (derived): half the ceiling leaves the
  other half for the JSON-escaped op envelope the chunk rides in (measured 1.10× on the Nakagin export).
- `importPayloadChunks(payload)`: slices by **UTF-8 extent**, never by code units — the guest measures
  `text.len()` in bytes and the Nakagin fixture has `·`/`ō` labels — and never splits a code point.
- `dispatchOpenedFiles` dispatches one call per chunk, in order, awaited one at a time (the guest refuses a
  gap, so a concurrent fan-out would cost the whole file). A file that fits one chunk dispatches exactly
  ONE call whose args are the pre-B59 `{payload, name}` plus `chunk: 0, chunkCount: 1`.

**Guest** — `✏️editor/🎮️commands/📥️import-fixture/🦀️.rs`, modelled on the `registerBrushMesh` page run
(`✏️editor/⏳️precompute/🦀️.rs`'s `stage_brush_mesh_page`), which is this repo's existing inbound paged lane.

- `PUZZLE3D_IMPORT_CHUNK_BYTES` (= the host's, both derived from the framework ceiling, so the two chunkers
  cannot drift), `PUZZLE3D_IMPORT_TOTAL_BYTES` (= `PUZZLE_COMMAND_OUTPUT_BYTES` — an import carries exactly
  what an export may stream, so a file this app wrote is always a file it can read back), and
  `PUZZLE3D_IMPORT_MAXIMUM_CHUNKS` (the budget at the chunk extent, never a second literal).
- `stage_import_chunk` + a fixed-slot staging area: a run keyed by `(name, chunkCount)`, a retransmitted
  chunk acknowledged at the cursor it stands on, a skipped chunk dropping the run, a sweep for abandoned
  runs. `pages: Vec<String>` IS the paged owner — one page per chunk, each bounded by the chunk extent, so
  the reassembled document never exists as one contiguous block.
- `puzzle3d_import_root_value(pages)`: rebuilds the root JSON object from the paged owner with a byte
  cursor that never joins the pages. Each root member is captured on its own, and an ARRAY member's
  elements are parsed **one at a time** into a growing `Vec<Value>` — objects one by one. The largest
  contiguous request the whole import makes is ONE element.
- `Puzzle3dImportFault { Envelope, Chunk, Gap, Capacity, Element, Payload }` with stable `code()`s, and
  every one of them becomes a localized notice (`import_too_large`, `import_incomplete`, `import_invalid` —
  new labels authored in English and German, no default language). A chunk that does not close its run
  stages and aborts: no document edit and no history row for a partial import, and never silence.

---

## 4 The laws

### 4.1 Guest — `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (4 new)

1. `a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle` — Nakagin (180) + 1 → its
   own 145 924-byte export → back to Nakagin → import through `puzzle3d_import_chunks`. Asserts the census
   moves 180 → 181, exactly ONE history row lands across the whole run, a STAGED chunk records ZERO rows and
   moves nothing, and nothing is refused.
2. `an_unchunked_over_ceiling_import_refuses_with_a_notice` — the shape the live verdict actually saw (one
   command, 145 924 B): exactly one notice, no history row, document untouched, and NOT a fault.
3. `a_one_hundred_forty_five_kilobyte_import_never_asks_for_a_block_above_the_guest_contiguous_ceiling` —
   the contiguous-ceiling law. Both halves measured: every chunk ≤ the chunk extent, and the widest
   contiguous value the paged reassembly captures. It also proves the law is not vacuous by asserting the
   widest root MEMBER is itself over the ceiling, and that the reassembly is byte-identical to the document
   that was chunked.
4. `an_out_of_order_import_chunk_is_refused_and_a_retransmitted_one_is_acknowledged` — the gap discipline.

### 4.2 Host — `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (2 new, 2 updated)

- `dispatchOpenedFiles (B59): a file above one chunk is dispatched in order as chunks that each stay inside
  the guest contiguous ceiling` — chunk indices 0..n-1, one `chunkCount`, one name, every chunk's UTF-8
  extent inside the cap, and the run rejoins to the original text.
- `importPayloadChunks (B59): slices by UTF-8 extent so a non-ASCII label can never overrun the chunk cap,
  and never splits a code point` — plus the empty-payload one-chunk answer.
- The two pre-existing D3 laws now assert the one-chunk envelope beside `{payload, name}`.

### 4.3 Outputs (foreground, tails quoted)

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib import -- --test-threads=1 --nocapture`

```
test …::a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle ... [DEBUG] B59 import payload bytes=145924 chunks=5 seeded=180
test …::a_one_hundred_forty_five_kilobyte_import_never_asks_for_a_block_above_the_guest_contiguous_ceiling ... [DEBUG] B59 ceiling law chunks=5 widestMember=125958 widestElement=2212
test …::an_out_of_order_import_chunk_is_refused_and_a_retransmitted_one_is_acknowledged ... ok
test …::an_unchunked_over_ceiling_import_refuses_with_a_notice ... [DEBUG] B59 unchunked refusal notice=["That file is larger than one import may carry"]
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 745 filtered out; finished in 1.70s
```

Read those two numbers together: the widest root member of the Nakagin document is **125 958 B**, 1.9× the
65 536-byte ceiling, while the widest contiguous request the paged reassembly makes is **2 212 B** — one
object. And the whole import suite now finishes in **1.70 s** where the single failing law alone burned
**63.85 s** without ever completing.

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`

```
warning: `semio-s-artifact-puzzle-3d` (lib) generated 95 warnings (run `cargo fix …`)
    Finished `dev` profile [unoptimized] target(s) in 0.96s
```

(0 errors; the 95 warnings are the crate's pre-existing set — warnings ARE emitted, so expansion completed.)

`cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2`

```
    Finished `dev` profile [unoptimized] target(s) in 2m 05s
```

`SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts --testNamePattern "dispatchOpenedFiles|importPayloadChunks"`

```
 Test Files  1 passed | 34 skipped (35)
      Tests  4 passed | 1071 skipped (1075)
   Duration  33.38s
```

`bun x tsc --noEmit`, filtered to the three files this wave touched (`🛠️ShellHelpers`, `🔬️engine-contract`,
`🎯️targets/⚛️react`): **no diagnostics**. (Unfiltered it reports the pre-existing `storybook-static/**`
`TS1127` corpus, which no file of this wave is part of.)

### 4.4 Regression sweep, and the baseline that reads it honestly

The whole `semio-s-artifact-puzzle-3d` lib suite single-threaded fails **11** laws WITH this wave's changes
and **12** WITHOUT them (the same set plus `set_fill_count_clamps_to_available_and_no_longer_dispatches_
catch_up`), so none of them belongs to this wave:

```
with B59      test result: FAILED. 749 passed; 11 failed; … finished in 710.54s
without B59   test result: FAILED. 744 passed; 12 failed; …   4 filtered out; finished in 601.73s
              (--skip a_one_hundred_forty_five --skip an_unchunked_over_ceiling --skip an_out_of_order_import)
```

Every one of the 11 passes in isolation and in its own group, e.g.

```
cargo test … --lib panels::catalogue -- --test-threads=1
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 756 filtered out; finished in 0.09s
```

They are whole-suite cross-test contamination through this crate's process-wide statics (worker-session
admission, the app session cache, the brush-mesh staging area) — the failure list is dominated by exactly
those (`an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app`,
`a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh`). Worth its own wave; it is not
this one's, and the baseline above is the proof.

`semio-s-artifact-puzzle-2d` (156 failed) and `-5d` (71 failed) fail in
`🧬️schema/🧬️mutations/*/🧪️tests/*` fixture laws this wave touches nothing of; the shared retained-command
change is proven to compile for all three artifacts by the `wasm32-wasip2` check above.

---

## 5 Verdicts

| claim | verdict | evidence |
|---|---|---|
| the payload crosses every host→guest hop | **confirmed, was never the defect** | §1 hops 0-7 |
| the drop is the guest's retained wire ladder (per-byte + O(n) checkpoint) | **root cause, reproduced and fixed** | §2, §3.1 |
| a 145 KB distinct fixture imports 180 → 181 with one history row | **PASS** | §4.3 law 1 |
| an over-budget/unchunked import refuses with a named notice, never silence | **PASS** | §4.3 law 2 |
| no single contiguous guest request above 65 536 B during a 145 KB import | **PASS** (widest 2 212 B against a 125 958 B member) | §4.3 law 3 |
| the host chunker and the guest chunker agree, and slice by UTF-8 extent | **PASS** | §4.2 |
| `import-same-file-idempotent`'s live PASS is unfalsifiable (B57 §2.3) | **still true** | the probe is unchanged by this wave |

## 6 What rides #62

Everything guest-side: the retained ladder (§3.1) and the whole import lane (§3.2) are Rust, so the browser
only sees them once the coordinator builds wasm **#62**. The host half (`importPayloadChunks`,
`dispatchOpenedFiles`) is vite-live and is already on `:6013` — which means **between #61 and #62 the live
import is refused with the `import_too_large` notice rather than silently dropped**, because #61's guest
does not read the chunk envelope and receives only chunk 0. That is the correct intermediate state (an
answer instead of silence), but it will read as a red `import-distinct` until #62 lands.

After #62, the live proof is `--only=example-switch,export-import --port=6013`:

- `import-distinct` should move `paneObjects=180→181` with one history row, and the console should carry
  five `[DEBUG] performInvocation … actionId: "importFixture"` settles rather than one;
- `import-same-file-idempotent` should be RE-CUT first (B57 §2.3: on the document it describes, an import
  that applies and an import that is dropped are the same census) — the chunk lane gives it a witness the
  identity cannot fake, since a dropped import now publishes a notice.

## 7 Files

- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` — page-granular ladder, folded checkpoint hash.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-fixture/🦀️.rs` — the chunked inbound lane, the paged owner, the paged reassembly, the fault taxonomy.
- `…/✏️editor/🗣️terminology/🦀️.rs` — `import_too_large`, `import_incomplete` (en + de).
- `…/✏️editor/🦀️.rs` — `retire_abandoned_import_runs()` on slot retirement.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — 4 new laws.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `IMPORT_CHUNK_BYTES`, `importPayloadChunks`, chunked `dispatchOpenedFiles`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx` — barrel exports.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — 2 new + 2 updated host laws.

No temporary `[DEBUG]` tap survives: the wire-byte scan tap of §2 was removed with the fix, and the three
`[DEBUG]` lines that remain are law witnesses printed by the laws themselves.

### 7.1 Final re-verification after the retirement hook

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
  warning: `semio-s-artifact-puzzle-3d` (lib) generated 96 warnings …
      Finished `dev` profile [unoptimized] target(s) in 1.32s        (grep -c "^error" → 0)

cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Checking semio-s-plugin-puzzle v0.1.0 (…/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust)
      Finished `dev` profile [unoptimized] target(s) in 21.45s

RUST_MIN_STACK=134217728 cargo test … --lib import -- --test-threads=1
  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 745 filtered out; finished in 5.32s

RUST_MIN_STACK=134217728 cargo test … --lib retained -- --test-threads=1
  test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 732 filtered out; finished in 8.83s
```
