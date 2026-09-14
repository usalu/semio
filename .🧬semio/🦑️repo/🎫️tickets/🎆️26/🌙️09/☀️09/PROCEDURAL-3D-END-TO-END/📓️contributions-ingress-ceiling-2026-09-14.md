# 📥️ Command ingress has no page ceiling — lane `contributions-ingress-ceiling` (2026-09-14)

Opus lane on port **6027** (`📜️serve-generation3d-react-6027.sh`, new). Closes the coordinator's
19:03 BLOCKER: the procedural plugin's 272 089-char contributions pack could not reach the guest, so
`No loaded plugin contributes the flow extension "brep"` → preview `phase: faulted`, every example
dead on both renderers.

Runtime logs: `🗑️generated/ingress/`.

---

## 1. TL;DR

**Two ceilings, both 262 144 bytes, both written as if 64 pages of 4 KiB were a law of nature. They
are gone, and nothing replaced them with a bigger number — both are now DERIVED from the one declared
guest linear-memory budget.**

1. **Transport.** `📡️spr/🧵️channel/🦀️.rs` declared `COMMAND_MAXIMUM_PAGES = 64`, so a command past
   262 144 B was refused before it started: `[DEBUG] command ingress exceeds 64 pages`
   (`📮️shard-client/🟦️.ts:130`). The 64 was not arbitrary — it was the largest page count whose
   assembly buffer the guest could still be asked for, because `CommandPageSet` stored each
   `FixedCommandPage` INLINE, making the buffer ONE contiguous block of `declared * 4098` bytes.
   **Fixed by indirection, not by arithmetic**: a page now holds its 4 KiB block behind a pointer, so
   a page authority reserves a spine of 16-byte slots and every page is its own routine 4 KiB request.
2. **Tool contract.** With the transport open, all 67 pages crossed and the guest's own tool factory
   refused the assembled command: `tool factory 's.procedural.generation3d@1/*#editor/setContributions'
   rejected 273136 raw bytes before decoding; maximum is 262144`. That maximum was
   `PUBLIC_INVOCATION_BODY_BYTES` — the JSON entry point's body cap — declared on a route whose payload
   does not go through the JSON entry point at all (it crosses whole, pack-encoded). **Fixed by naming
   the bound that actually binds**: `COMMAND_MAXIMUM_BYTES`.

Measured on 6027 after both fixes, one boot:

```
[DEBUG] command ingress crossed {"actionId":null,"bytes":273731,"pages":67,"ms":696}
[DEBUG] command ingress settled status=command-complete observed=command-complete
[DEBUG] contributions publish {"plugin":"procedural","instanceId":1,"outcome":{"status":"installed","chars":272089,…}}
```

No `exceeds` line, no `rejected … raw bytes` line, contributions **installed**, and the preview left
`faulted`/`flow.extension-not-contributed` for `computing`.

**The preview still does not reach `idle` with meshes — for a different, fleet-wide, peer-owned
reason** this lane neither caused nor fixed: see §7.

---

## 2. Root cause, with the exact cap

| # | layer | site | the cap | why it existed |
|---|-------|------|---------|----------------|
| 1 | framework (Rust) | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` `COMMAND_MAXIMUM_PAGES` | `64` pages × `COMMAND_PAGE_MAXIMUM_BYTES` 4 096 = **262 144 B** | `CommandPageSet`'s `VecDeque<FixedCommandPage>` held each page's 4 096-byte array INLINE, so a `declared`-page authority was `declared * size_of::<FixedCommandPage>()` = `declared * 4 098` CONTIGUOUS bytes. 64 pages is already 262 272 B — four times `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, and the existing law `a_command_page_authority_reserves_only_the_pages_its_command_declares` ASSERTED that ("the 64-page ceiling is over that bound — which is why it may not be reserved for every command"). Any larger ceiling would have made every command's reservation worse, so the ceiling could not move while the storage stayed inline. |
| 2 | framework (TS) | `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` `SHARD_COMMAND_MAXIMUM_PAGES` | `64` | Hand-mirrored the Rust constant; threw `[DEBUG] command ingress exceeds 64 pages` from `createShardCommandIngressPages` before a single page was posted. This is the line the coordinator measured. |
| 3 | app (Rust) | `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` `GENERATION3D_CONTRIBUTIONS_RAW_BYTES` | `PUBLIC_INVOCATION_BODY_BYTES` = **262 144 B** | Its docstring assumed the pack crosses as a JSON page run capped by `PUBLIC_INVOCATION_STRING_BYTES`. It does not: `ShellHost`'s contributions publisher sends ONE pack-encoded crossing (`encoding: "pack"`, `pageCount: 1`), because paging the envelope at app level cost 99 guest turns / ~202 s. So the route declared a cap from a protocol it does not use, and the real 273 136-byte wire was refused at `ToolDispatchError::RawWireLimit`. |
| 4 | app (Rust) | `✏️s/…/🧊️generation3d/…/👁️viewer/🦀️.rs` `GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES` | same | Same declaration on the viewer role; would have refused the identical push the moment the viewer became the receiver. |

**What was already right, and stayed right.** The ingress is ALREADY a multi-page / multi-turn stream
with a bounded per-turn budget and exact reassembly — that part needed no invention:

- the host writer cuts at `ACTOR_BYTE_PAGE_BYTES` and stamps `pageIndex`/`pageCount` per page;
- `PluginRuntime`'s `runQueuedTurn` (React) and `runQueuedTurnSerialized` (wgpu) submit **one page per
  actor turn** and then poll continuations until `command-complete`;
- the reactor admits **one page per turn** (`⚛️reactor/🔄️turn/🦀️.rs`), parks the partial command in
  `CommandIngressOwner::GenericAssembly` across turns, and refuses anything that is not the next page
  (`cursor.page_index as usize != pages.len()` → `plugin.command-page-order`);
- `PagedCommandReader` decodes by walking pages and RELEASING each as it is consumed, so the assembled
  command is drained rather than copied flat.

The defect was never the stream. It was that the stream's buffer was one contiguous block, and a
ceiling had been chosen to make that block survivable.

---

## 3. The design

### 3.1 A page slot is a handle, never the block

```rust
pub struct FixedCommandPage {
    bytes: Box<[u8; COMMAND_PAGE_MAXIMUM_BYTES]>,
    len: u16,
}
```

Every collection on this path (`CommandPageSet`, `PagedCommand`, `CommandEnvelopeSet`, `CommandBatch`,
`PresenceCommandCursor`, the reactor's retained `CommandIngressOwner`) is a deque of these, so all of
them shrank by the same construction with no call-site churn. The block itself is reserved fallibly
(`try_reserve_command_page_block`, `plugin.command-page-allocation`) — an exhausted guest heap answers
with a `Fault` the host can display, never `handle_alloc_error` → `unreachable`.

Measured (`the_command_ingress_authorities_are_derived_from_the_guest_memory_budget`, `--nocapture`):

```
[DEBUG] command ingress authorities: pages=2048 bytes=8388608 slot=16 B spine=32768 B ceiling=65536 B
```

A page slot costs **16 B** natively (8 on wasm32), so the WHOLE ceiling's spine is 32 768 B — half the
contiguous request ceiling. Before: 64 slots × 4 098 B = 262 272 B, four times over it.

### 3.2 Both authorities are read off the memory budget, not chosen

```rust
pub const COMMAND_MAXIMUM_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES;
pub const COMMAND_MAXIMUM_PAGES: usize = COMMAND_MAXIMUM_BYTES / COMMAND_PAGE_MAXIMUM_BYTES;
```

A command IS an assembled host answer, so it is bound by the budget that already says what a guest may
be handed for one outstanding request before it answers with a typed fault instead of allocating
(`⏱️trace/🧮️memory/🧬️schema/🔣️.json`'s `hostAnswerCeilingBytes`). `semio-framework-trace` is a
zero-dependency, wasm32-wasip2-safe crate the guest plugin crate already links; it is now a dependency
of `semio-framework-os-kernel` for exactly these two names.

The TypeScript writer reads the same two numbers rather than mirroring a literal:

```ts
export const SHARD_COMMAND_MAXIMUM_PAGES = GUEST_HOST_ANSWER_CEILING_BYTES / ACTOR_BYTE_PAGE_BYTES;
```

`INVOCATION_RESULT_PACK_MAXIMUM_BYTES` follows `COMMAND_MAXIMUM_BYTES` on both sides, as it already did.

**There is no page ceiling left to raise.** The only bound is the guest's own declared budget, which is
what "keep the 64 KiB contiguous guest request ceiling" asked for: every per-page request is 4 KiB,
every per-command spine request is at most 32 768 B, and both are under 65 536 B **by a law**, not by a
constant someone picked.

### 3.3 No fixed reservation left on the batch path

`CommandEnvelopeSet::try_new` used to reserve `COMMAND_MAXIMUM_PAGES` page slots up front — 262 272 B
per batch, whatever the batch carried. It now reserves its command slots only, and `try_push` reserves
exactly the page slots the command it is admitting declares (`plugin.command-batch-page-allocation`).
Same principle `CommandPageSet` already followed: reserve what is DECLARED, never the ceiling.

### 3.4 The tool contract names the bound that binds

```rust
const GENERATION3D_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;
```

Widening this costs nothing at runtime: `RetainedToolWireInput::try_new` reserves per DECLARED extent
(`declared_bytes / TOOL_WIRE_PAGE_BYTES` slots), never per maximum — the maximum is a refusal bound
only. The viewer route now reads the same constant.

### 3.5 Crossing by reference — considered, deliberately NOT landed

The brief asked whether a content-addressed pack cache in the shard should replace the 272 KB crossing
on every example switch. **It should not, because that crossing does not happen.**
`ShellHost`'s `createContributionsPublisher` already installs keyed by `(instanceId, content)` and
joins on an unmoved registry generation, so the pack crosses ONCE per instance per closure — a plain
example switch re-crosses nothing. This is visible in the runtime logs: the boot shows exactly one
`contributions publish … installed` and one 67-page crossing, and the 23-step journey (all eight
examples, three roles) shows **no second 67-page crossing at all**.

A shard-side content-addressed cache would therefore add a second cache under an existing one, with a
second invalidation rule, to avoid a crossing that already happens once. The place a by-reference push
would pay is a NEW instance of the same plugin (spawned app, viewer role) re-crossing a pack an already
resident actor holds — a real but different optimisation, and one that belongs with the publisher that
owns the identity, not with the transport. Flagged, not built.

---

## 4. Laws

Language-agnostic: one shared fixture, a Rust reader and a TypeScript twin, plus two laws through the
REAL reactor.

**Shared fixture** — `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📥️command-ingress-pages/🔣️.json`
(new): `pageBytes`, `commandMaximumBytes`, `maximumPages`, seven `rows` (1 B … 8 MiB, including
`former-fixed-ceiling` 262 144 B/64 pages, `former-fixed-ceiling-plus-one-byte` 262 145 B/65 pages and
`contributions-pack-2026-09-14` 272 089 B/67 pages), two `refusals`, four `disorder` rows.

| law | where | what it pins |
|---|---|---|
| `the_command_ingress_authorities_are_derived_from_the_guest_memory_budget` | `📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` | `COMMAND_MAXIMUM_BYTES == GUEST_HOST_ANSWER_CEILING_BYTES`; pages = bytes / page extent; the ceiling's spine ≤ `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`; a page slot is smaller than a page |
| `every_command_ingress_row_reassembles_to_the_exact_bytes_it_was_cut_from` | same | every row cuts into exactly its declared pages, every nonterminal page is full, `PagedCommandReader` reads back the EXACT body byte for byte, the reader is terminal-empty afterwards and the next read is `plugin.command-decode-truncated` |
| `every_command_ingress_refusal_answers_a_fault_and_keeps_its_page` | same | an over-declaration and an over-push are `Fault`s, and a refused page is handed BACK |
| `a_command_page_that_is_not_the_next_one_is_refused_by_its_order` | `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` (new) | through the REAL `poll_kernel`: duplicate first page, skipped middle page, reversed tail and replayed middle page each answer `plugin.command-page-order` at the exact delivery the fixture names, and leave no retained ingress owner |
| `a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` | same (extended) | `INGRESS_PAGE_SETS` now `[1, 2, 4, 8, 67, 80]` — the budget park/resume proof: a command PAST the old ceiling still costs exactly one turn per page |
| `a_command_page_authority_reserves_only_the_pages_its_command_declares` | same (corrected) | its final assertion was `reservation_bytes(COMMAND_MAXIMUM_PAGES) > ceiling`; it is now `<=`, with the old shape named as the cause |
| `contributions_route_declares_a_reachable_wire_ceiling` | `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (rewritten) | the route's `max_raw_wire_bytes` IS `COMMAND_MAXIMUM_BYTES` and is strictly wider than `PUBLIC_INVOCATION_BODY_BYTES`, and the MEASURED 272 089-char pack fits |
| `command ingress pages` (13 cases) | `🎭️actor/📮️shard-client/🧪️tests/📥️command-ingress-pages/🟦️.ts` (new) | the TS twin: the writer derives its ceiling from the budget, every fixture row cuts into its declared pages with exact per-page cursors, the pages concatenate back byte for byte, and both refusals throw |
| `wgpu contributions command ingress` | `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts` (updated) | the wgpu fixture's `maximumPages` equals `SHARD_COMMAND_MAXIMUM_PAGES` and `maximumPages * pageBytes` equals the host-answer ceiling; the fat view is re-sized so the slim-view law still bites |

### 4.1 Counts, run in the foreground

```
cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::
  → 83 passed, 2 failed (both PRE-EXISTING, §7.1)
cargo test -p semio-framework-os-kernel --lib -- tests::the_command_ingress tests::every_command_ingress
  → 3 passed, 0 failed
RUST_MIN_STACK=33554432 cargo test -p semio-framework-plugin --lib -- command_page a_command_page_that_is_not_the_next_one
  → 4 passed, 0 failed
  [DEBUG] command page set turns: 1→1 2→2 4→4 8→8 67→67 80→80
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- contributions_route_declares
  → 1 passed, 0 failed
bun ./📜️script.ts test -t "command ingress pages"      (@semio-tech/framework-actor)
  → 13 passed
bun ./📜️script.ts test long 🔬️wgpu-extension-dispatch  (@semio-tech/framework-renderer-react)
  → 8 passed
```

### 4.2 Each law was RED first

- Rust, with `COMMAND_MAXIMUM_BYTES` put back to `4096 * 64`:
  `the_command_ingress_authorities…` FAILED (`left: 262144`), and
  `every_command_ingress_row_reassembles…` FAILED at
  `row former-fixed-ceiling-plus-one-byte declares a page authority: Fault { code: FaultCode("plugin.command-page-count") }`
  — the browser's refusal reproduced exactly, in a unit test.
- TypeScript, with `SHARD_COMMAND_MAXIMUM_PAGES` put back to `64`: **4 failed**, including
  `cuts contributions-pack-2026-09-14 into 67 pages…` and `cuts assembled-maximum into 2048 pages…`.
- `a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` could not even be WRITTEN for
  67 or 80 pages before this lane: `CommandPageSet::try_new(67)` refused.
- `contributions_route_declares_a_reachable_wire_ceiling` asserted the old
  `PUBLIC_INVOCATION_BODY_BYTES` equality, so it was green while the browser was refusing the pack —
  that is the law failing at its job, and it now names the measured 272 089-char pack.
- The tool-factory refusal itself is the second fix's failing-first evidence, measured live on 6027
  with only the transport fixed: `🗑️generated/ingress/boot/console.txt:45`.

---

## 5. Runtime, on 6027

Restage (both, foreground, `EXIT=0`): `🗑️generated/ingress/restage-react.txt`,
`🗑️generated/ingress/restage-react-2.txt`, `🗑️generated/ingress/restage-wgpu.txt`.
Serve: `📜️serve-generation3d-react-6027.sh` under `screen -dmS g3dreact6027`, 200 after 10 s.

| stage | measurement |
|---|---|
| scoped pack | `chars: 272089`, 16 operator kinds, 8 examples |
| assembled command | **273 731 B** (pack-encoded invocation incl. envelope) |
| pages crossed | **67** (`⌈273731 / 4096⌉`), one per actor turn |
| ms spent crossing | **696 ms** warm (`🗑️generated/ingress/boot2`, `boot3`); 3 168 ms on the first boot after the restage (`🗑️generated/ingress/boot`), cold jco/module load included |
| ingress status | `command-complete`, observed statuses `command-complete` only — no backpressure, no fault, no continuation past the pages |
| contributions | `[DEBUG] contributions publish … "status":"installed","chars":272089` |
| crossings per session | **1** — the 23-step journey over all eight examples and three roles contains no second 67-page crossing |
| preview before | `phase: "faulted"`, `fault.code: "flow.extension-not-contributed"`, `extensionId: "brep"`, `contributed: []`, `data-meshes-json` empty |
| preview after | `phase: "computing"`, `unitsDone 5/7`, `inFlight 2` — the brep extension IS contributed and the graph evaluates |
| `exceeds` lines | **0** (boot probe and 23-step journey) |
| `rejected … raw bytes` lines | **0** |

Per-page cost, from the same boot: ordinary one-page commands cross in 2–57 ms each, and the 67-page
command in 696 ms — **~10 ms per page**, i.e. the crossing is one actor round trip per page with no
extra stall, matching the unit law's `67→67` turns.

`🐍️journey-probe.mjs` → `DONE steps 23 meshSteps 0`: all 23 steps ran, none converged to meshes, every
one of them for the reason in §7.2 — `meshes 0 vs oracle N`, with zero ingress faults in the log.

---

## 6. Files

**Created**

- `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📥️command-ingress-pages/🔣️.json`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/📥️command-ingress-pages/🟦️.ts`
- `<ticket>/📜️serve-generation3d-react-6027.sh`
- `<ticket>/📓️contributions-ingress-ceiling-2026-09-14.md` (this report)

**Updated**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` — boxed page block, derived
  authorities, exact per-admission batch page reservation, refusal messages
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` — three new laws;
  the hostile-field-length vector re-cut to the derived cap
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — `semio-framework-trace` dependency
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` — derived `SHARD_COMMAND_MAXIMUM_PAGES`,
  twin registration, `SHARD_COMMAND_MAXIMUM_PAGES` exported to the test dependency bundle
- `🧰️framework/🛍️products/💻️os/🟦️.ts` — `INVOCATION_RESULT_PACK_MAXIMUM_BYTES` derived
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  — page-order law, `INGRESS_PAGE_SETS` extended past the old ceiling, reservation law corrected
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
  — `[DEBUG] command ingress crossed {bytes, pages, ms}` per command
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔬️wgpu-extension-dispatch/🔣️.json`
  and its `🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js`
  — regenerated (`generate-frame-worker`) so the wgpu door carries the derived constant
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  and `…/👁️viewer/🦀️.rs` — contributions wire ceiling derived from the transport
- `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the reachability law, rewritten around the
  measured pack

---

## 7. What is NOT claimed

### 7.1 Two pre-existing `os_spr::channel` failures are not mine

`paged_generic_decoder_admits_document_config_and_projection_commands_used_during_browser_boot` and
`paged_recursive_archive_crosses_pages_and_decoded_owner_closes_one_field_per_grant` fail. Both are
`LoadDocumentArchive` decode-phase failures (`decoded == None` after the law's four `step()` calls),
and both fail IDENTICALLY with `COMMAND_MAXIMUM_BYTES` put back to its old value — so they are not
caused by anything here. They belong with whoever is moving `DocumentArchive`.

### 7.2 The preview still does not reach `idle` with meshes — and the reason is NOT the ingress

Every extension dispatch now fails with
`SemioFaultError: targeted window transient capture requires an exact ViewModel roster`
(`🔌️plugin/🦀️.rs:25876` — `meta.view_state` is `None` on the turn the extension completion drives),
so `flow-extension-math evaluate` and `flow-extension-brep evaluate` never complete and the preview
parks at `unitsDone 5/7, inFlight 2`. This is **fleet-wide and not this lane's**: the same fault is in
three other lanes' console dumps taken at 19:42–19:43 on their own ports
(`🗑️generated/react-s5/boot-now/`, `🗑️generated/flow-inline/console-boot/`,
`🗑️generated/react-interact/interact/`). Nothing in this lane touches view-state capture. It is the
next blocker and wants its own owner — it looks like the known "action view state needs window
instances" shape (the host must carry a `ViewModel` roster on the turn that drives an extension
completion, the way `ShellHost`'s base dispatch view state does).

Consequently: `journey-probe` is **23 steps run, 0 mesh steps green**, not 23/23. The brief's `idle`
+ meshes and 23/23 gates are NOT met, and this lane does not claim them. What it does claim is that
**no gate is blocked on command ingress any more**: zero `exceeds` lines, zero raw-wire refusals, and
contributions `installed` in every run.

### 7.3 Other things not claimed

- **No wgpu browser run.** The wgpu guest was restaged and its frame-worker bundle regenerated with the
  derived constant, but no `6118`-class browser verification was taken here.
- **`generation2d`'s contributions ceiling was not touched.** It derives from
  `PUBLIC_INVOCATION_STRING_BYTES * ESCAPE_PAIR_WIRE_FACTOR` (~8 KiB) and genuinely uses the paged JSON
  route, so the §2 row 3 reasoning does not transfer. If its pack ever grows past that, the fix is the
  same reasoning applied to ITS actual crossing shape, not a copy of this constant.
- **The hostile-field-length bound widened with the command ceiling.**
  `APP_COMMAND_FIELD_MAXIMUM_BYTES` is `COMMAND_MAXIMUM_BYTES` and so moved from 262 144 to 8 388 608:
  a malformed length now reserves (fallibly, `plugin.command-field-allocation`) up to 8 MiB before
  being found truncated. Bounding a field by the bytes its own command still carries is strictly
  better and was tried — it changes the retained-close ladder three existing decode laws pin, so it
  was reverted rather than reshaped blind. **Follow-up, not a regression this lane introduced beyond
  the widening.**
- **`semio-framework-os-run`'s BIN does not compile** (`store::Media: Serialize`/`Deserialize` not
  satisfied, `🏗️bootstrap/🦀️.rs:43`). Peer-owned `🏪️store` churn, unrelated; the crate's LIB and every
  crate this lane touched compile.
- **Six unrelated `@semio-tech/framework-actor` suites fail** (return framing, lifetime fixtures, the
  `@vite-ignore` assertion on the generated shard worker). None is in the command-ingress path; the
  shard-worker source file itself carries no page cap and needed no rebuild.
- **The `[DEBUG] command ingress crossed` line is a permanent instrument, not a temporary log.** It is
  one `performance.now()` pair per command and it is what made the 67-page/696 ms number readable.
- **No claim about any other app's command sizes.** The transport change is app-neutral by
  construction and the framework laws cover it, but only generation3d was exercised in a browser.
