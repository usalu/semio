# Flow Eval Session Retirement — The Unpaid-Frontier Family, Fixed At Its Owning Layer (2026-09-13)

Lane `flow-eval-session-retirement` (Opus). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
`repo` and `semio` MCP both failed to connect all session (`repo`: -32602 invalid initialize params;
`semio`: CONNECTION_CLOSED) — no ticket lifecycle call, no `📓️status.md` / `🎫️ticket.json` edit, no
modifying git command, no dev server started or stopped, no restage, and nothing another lane created
under `🗑️generated` was read-modified or deleted.

## TL;DR

`FlowEvalSession::close_step` was never the defect. The defect is one commit, `5b6f77afcf`
(`git log --date=iso` → **2026-09-13 11:27:07 +0200**), turning three byte-retirement frontiers from
**payload drawdown** into **all-or-nothing physical release** that answers `Blocked` — never an error
— to any grant smaller than one owner's backing. Every driver in the tree hands a fixed page (1, 64,
4096) and only ever closes, so each of them became a silent infinite spinner. The
language-agnostic contract those frontiers are pinned by says the opposite, in so many words
(`🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts`: `while (left) { released = Math.min(grant, left) }`).

Fixed at the three owning layers; every driver above them needed no change.

| suite (foreground, `--test-threads=1`) | before | after |
|---|---|---|
| `semio-framework-os-flow --lib -- retirement` | **5 passed / 2 failed** (peer-measured, §5.1) | **8 passed / 0 failed** |
| `semio-framework-os-kernel-neural-engine --lib` | **HANGS** — infinite loop, sampled (§1) | **54 passed / 0 failed**, 0.10 s |
| `semio-framework-replication --lib` (the `protocol::value::ordered` owner) | not separately measured | **273 passed / 0 failed** |
| `semio-framework-os-flow --lib` (whole suite) | **HANGS** at test 51 of 220 | **162 passed / 58 failed**, 1.58 s, runs to completion |
| generation3d featured `--lib -- unit_tests::` | **68 passed / 6 failed** (`📓️disposer-teardown-ownership`) | **70 passed / 4 failed**, 42 s |
| generation3d featured `--lib` (FULL) | **266 passed / 117 failed, then ABORTED** (`📓️editor-verbs-cancel-undo` §7.1) | **427 passed / 11 failed**, 65.8 s, **no abort** |

The four `unit_tests::` reds that remain are exactly the four the coordinator named as other owners
(`nodeGraphViewport`, `module.vcs` remote-merge guard, `generation3d-publication.contended`, and the
preview-transient law). The keyboard-fixture law is green again (its own lane fixed it).

## 1. Root cause, with the witness

Commit `5b6f77afcf` rewrote `Owner::Bytes` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs` from

```rust
Owner::Bytes(mut bytes) => {
    released_bytes = maximum_bytes.min(bytes.len()); bytes.truncate(bytes.len() - released_bytes);
    if !bytes.is_empty() { self.owners.push_front(Owner::Bytes(bytes)); }
}
```

to an all-or-nothing `capacity()` release that returns `Blocked` when `capacity > maximum_bytes`, and
made the same change in two sibling frontiers:

- `protocol::value::ordered::Retirement::advance` — `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:442`
  (ordered-map KEYS; a 5-byte key blocks a 1-byte grant)
- `retained::FlowRetirement` via `release_backing` —
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:47`

**The witness.** My first act was to run the neural crate's own `--lib` suite. It never finished.
`sample 85546` on the live 19-minute hang (`🗑️generated/flow-eval-retirement/before-neural-hang-stack.txt`):

```
component::tests::dictionary_owned_cursor_preserves_order_and_nested_ownership
  → neural_engine::component::retirement::retire_value_cold
    → ValueRetirement::close_step
      → LinkedList<Owner>::push_front          ← the Blocked branch, re-pushing forever
```

`retire_value_cold` is `while !matches!(owner.close_step(1, 4096), Complete) {}`. It is called from
`ColdDictionaryBuilder::insert`, `ColdValueOwner::drop` and every `ColdRetire` impl — so **any cold
dictionary teardown holding a string over 4 KiB hangs the process**. That is the same shape three
lanes reported today at three different drivers, and it is a superset of them.

The contract that settles which side is wrong is language-agnostic and predates the commit —
`🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts`:

```ts
for (const grant of fixture.grants) {          // 1, 64, 4096
  let remaining = row.expectedBytes; let released = 0;
  while (remaining) { const step = Math.min(grant, remaining); remaining -= step; released += step; }
  assert.equal(released, row.expectedBytes);
}
```

and the session twin, `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:14-16,36-40`, which
sums `Buffer.byteLength` over the owners and asserts `fixture.text.reservedCapacity >
Buffer.byteLength(text)` — i.e. it states explicitly that a String's **reserved capacity beyond its
payload is not released bytes**. That is precisely what
`empty_reserved_text_does_not_require_capacity_sized_credit` pins in Rust.

## 2. The fix

Three owning layers, one idea: **payload is drawn down `min(grant, left)` and the allocation is freed
when the charge is settled; allocations are never charged against the caller's payload grant.**

### 2.1 `neural::ValueRetirement` (`🧠️neural/⚙️engine/🧵️retirement/🦀️.rs`)

- `Owner::Bytes(Vec<u8>)` → `Owner::Bytes { values, remaining_bytes }` (`byte_owner` seeds
  `remaining_bytes = values.len()`); `close_step` charges `min(maximum_bytes, remaining)` per turn and
  drops the whole buffer — reserved capacity included — on the turn the charge reaches zero.
- The emptied element-vector backings (`Strings`, `Channels`, `Fields`) are freed in one turn under
  any positive grant, reporting **zero** payload bytes. `capacity * size_of::<T>()` is machine-width
  dependent (`size_of::<String>()` is 24 native, 12 on `wasm32`), so it can never be part of a
  cross-language released-byte total. It stays counted by `allocated_bytes()`, which is what the
  terminal proofs and `FlowRetirement`'s reservation actually need.
- `next_close_byte_demand()` collapses to `0/1`: one byte of credit per turn is all a close needs.
- `close_step` is now **total**: `Blocked` means the caller offered no credit, never "this owner is
  too big for your page".

### 2.2 `protocol::value::ordered::Retirement` (`🌱️value/🗂️ordered/🦀️.rs`)

Same change for map KEYS: `Owner::Bytes { values, remaining_bytes }`, `min(grant, left)` drawdown,
demand `0/1`. `advance` now blocks only on a zero grant.

### 2.3 `retained::FlowRetirement` (`🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`)

- New `root_backing_credit` field + `release_root_backing`: `FlowOwner::Bytes` is charged down by
  payload; an emptied `Vec<Widget>` / `Vec<SynapseSpec>` / … is freed in one turn reporting 0 payload
  bytes (its retained `capacity` is an allocation, not payload — that was a real spinner: a graph
  that once held 21 widgets keeps a >4 KiB `Vec` capacity after `pop`ing every one of them).
- The empty-page release pays its own allocation demand
  (`frontier.next_release_allocation_bytes()`) instead of failing under the caller's page, and reports
  `released_bytes: 0` for the same currency reason.
- New `pub fn close_page(maximum_items, maximum_bytes)` — the ONE entry point a retained driver
  should use. It pays this RESERVE-then-CLOSE frontier's outstanding page reservation, then closes.
  `retire_cold` is documented as its cold twin.

### 2.4 Drivers routed to the paid entry point

Every remaining pure-close driver of a `FlowRetirement` (§4) now calls `close_page`, and every
`terminal_is_empty()` witness that was still spelled `is_empty()` was moved — after commit 617 a
frontier can be logically empty while still holding **allocated** pages, and `FlowRetirement::drop`
panics on exactly that (the lesson `📓️disposer-teardown-ownership-2026-09-13.md` §3 states).

`FlowEvalSession::close_step` itself needed **no code change** — under the restored contract it was a
correct driver all along. It carries a new docstring saying why (its `eval_json`, status JSON and mesh
packs are routinely larger than the framework's 4 KiB close page). The evidence it is fixed: the
authority string `"Generation3d evaluation session awaits its exact close grant"`, which
`📓️disposer-teardown-ownership` §5.1 named as the blocker, now appears **0 times** in the
generation3d suite (`grep -c` on `🗑️generated/flow-eval-retirement/after-gen3d-unit-2.txt`).

## 3. Byte-contract changes, one by one

**No fixture changed.** `🧹️session-retirement/🔣️.json` still says `releasedBytes: 42405`,
`🔣️value-retirement.json`, `🗃️cache-retirement`, `🧮️evaluation-owners` are untouched, and both
`🧪️source-contract/🟦️.ts` oracles are untouched — the fix restores the numbers they always stated.

Three Rust laws that commit 617 added or rewrote to pin the all-or-nothing behaviour had to change,
because they contradict those fixtures and the TS oracles:

1. `neural_physical_retirement_direct_string_and_vector_backings_release_exact_capacity_once` →
   **`…_direct_vector_backings_free_whole_under_any_positive_grant`**. Its `Owner::Bytes` case moved
   out into its own law (2). What changed: `next_close_byte_demand` on an emptied element vector goes
   `capacity * size_of::<T>()` → `1`; `close_step(1, capacity - 1)` goes `Blocked` → not asserted;
   the release turn reports `released_bytes: capacity` → `0`. `allocated_bytes()` still reports the
   capacity before and `0` after — the physical accounting is unchanged, only the *currency it is
   charged in*.
2. New `neural_physical_retirement_charges_byte_buffers_down_and_frees_the_whole_reservation` — an
   8193-capacity / 8000-payload buffer releases 8000 under a **one-byte** grant, and a
   `String::with_capacity(8193)` with no payload releases **0**. That second assertion is the Rust
   twin of the TS `assert(fixture.text.reservedCapacity > Buffer.byteLength(text))`.
3. `neural_physical_retirement_delegates_ordered_key_capacity_without_clamping` →
   **`…_charges_a_nested_ordered_key_down_under_a_one_byte_grant`**: a nested key with capacity 8193
   and payload 10 releases **10**, not 8193, and one byte of grant suffices.
4. `ordered_physical_retirement_uses_inline_frontier_and_releases_exact_key_capacity` →
   **`…_and_charges_the_key_payload_down`**: same three changes at the ordered layer.

## 4. Tree-wide audit of pure-close drivers

Every site that drives a reserve-then-close / drawdown frontier. "Paid" = it can finish under a fixed
page grant after this lane's change.

### 4.1 `neural::ValueRetirement` — all paid by §2.1, none needed a code change

| site | note |
|---|---|
| `🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:197` `retire_value_cold` | fixed 4096 — **was the hang in §1** |
| `🧠️neural/⚙️engine/🧊️cold/🦀️.rs:51` `ColdRetire for NeuralCache` | fixed 4096 |
| `🧠️neural/⚙️engine/🦀️.rs:1697` `NeuralCacheRetirement::close_step` | forwards the caller's grant |
| `🧠️neural/⚙️engine/📔️registry/🦀️.rs:62` `RegistryRetirement::close_step` | forwards |
| `🌊️flow/🖥️host/🦀️.rs:3512` **`FlowEvalSession::close_step`** | the lane's named target |
| `🌊️flow/🖥️host/🦀️.rs` (`close_page`, `state.neural`) | forwards |
| `🌊️flow/📔️registry/🦀️.rs:256` `retire_flow_extension_registries_step` + `:85` the test drain | rotates on `Blocked`, so it was a *silent leak* rather than a spin when only one version was queued |
| `🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:362` `FlowRetirement::neural` | forwards |
| `✏️s/🔌️plugins/🎬️sequence/…/✏️editor/🦀️.rs:1570, 2101, 2451, 2473` `release_one` ×4 | fixed `maximum_bytes` |
| `🌱️value/🗂️ordered/🦀️.rs:459 retire_cold`, `:466 close_cold`, `🧺️set/🦀️.rs:32` | `close_cold` used a bare 4096 |

### 4.2 `retained::FlowRetirement` — these DID need the paid entry point

| site | before | now |
|---|---|---|
| `🌊️flow/🖥️host/🦀️.rs` `FlowHostRetirement::close_page` | paid the reservation (peer, `d8dce87ca0`), not the backing | `state.domain.close_page(1, maximum_bytes)` |
| `🌊️flow/🖥️host/🦀️.rs:2541` `FlowHost::retire_cold` | **confirmed infinite loop** (`📓️editor-verbs-cancel-undo` §8.1) | terminates |
| `🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:636` `FlowFixtureRetirement::close_step` | bare `close_step`, witness `is_empty()` | `close_page`, witness `terminal_is_empty()` |
| `…/🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs:40,69` `FlowMutationRetirementFrontier` | same | same |
| `🌊️flow/🌿️vcs/🦀️.rs:816, 925` `close_operation_step` / `close_retired_step` (+ the `:967` witness) | same | same |
| `🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:403` (+ the `:396` witness) | same | same |
| `✏️s/🔌️plugins/🌀️procedural/…/💾️binary/🦀️.rs:541` `generation3d_close_flow_frontier` | refused a page demand larger than the caller's grant (`Some(demand) if demand > maximum_bytes => Blocked`) — the peer's own documented caveat, and **the single biggest generation3d red class** once the ladder got far enough to reach it | delegates to `FlowRetirement::close_page` |
| `✏️s/🔌️plugins/🌊️flow/…/✏️editor/👥️presence/♻️retirement/🦀️.rs:35` | listed by `📓️disposer-teardown-ownership` §6 | **not touched** — it drives its own owner type, not a `FlowRetirement`; unverified |

## 5. Runs — all in the foreground of my own Bash calls

Logs under `🗑️generated/flow-eval-retirement/`.

| log | command | result |
|---|---|---|
| `before-neural.txt`, `before-neural-sample.txt`, `before-neural-hang-stack.txt` | neural `--lib`, sampled after 19 min | **HANG**, stack in §1 |
| `after-neural.txt` | after the `Owner::Bytes` drawdown | 49 passed / 5 failed |
| `after-neural-2.txt` | after the ordered-key drawdown | 51 / 3 |
| `after-neural-3.txt`, `final-neural.txt` | after the vector-backing + law changes | **54 / 0** and **273 / 0** (replication) |
| `after-flow-retirement.txt` | `-p semio-framework-os-flow --lib -- retirement` | 7 / 1 (the 1 was a peer's law, edited mid-run) |
| `after-flow-session-retirement.txt`, `after-flow-retirement-2.txt`, `final-flow-retirement.txt` | same filter | **8 / 0** |
| `after-flow-lib.txt` | whole flow `--lib` | hung at test 51 (`add_input_port_inserts_variadic_slot`), sampled → `FlowMutationRetirementFrontier` |
| `after-flow-lib-2/-3/-4` | intermediate | hang moved to `retained_vcs_256_plus_one…`, then to the page release |
| `after-flow-lib-6.txt`, `final-flow-lib.txt` | whole flow `--lib` | **162 / 58**, 1.58 s, runs to completion |
| `after-gen3d-unit.txt` | generation3d `unit_tests::` before the §4.2 generation3d fix | 46 / 28 |
| `after-gen3d-unit-2.txt` | after it | **70 / 4**, 42.4 s |
| `after-gen3d-full.txt` | generation3d FULL featured `--lib` | **427 / 11**, 65.8 s, **no abort** |
| `debug-1…6.txt`, `flow-lib-hang-*.txt` | `[DEBUG]` probes and `sample` output | all instrumentation removed (verified by grep) |

### 5.1 On the `5 passed / 2 failed` baseline

I did not re-measure it myself: the flow crate would not have reached those laws without the neural
fix landing first. It is the peer-measured `🗑️generated/disposer-teardown/flowhost-laws.txt` from
`📓️disposer-teardown-ownership-2026-09-13.md` §4, same command, same tree, ~18:00. What I did measure
myself is that both named laws — `empty_reserved_text_does_not_require_capacity_sized_credit` and
`session_semantic_bytes_larger_than_production_grant_retire_exactly_across_workers` — are green.

### 5.2 The 58 flow `--lib` reds, classified

They are a **different family**, made visible by the suite now reaching them (it previously hung at
test 51 of 220, so nothing after `host::tests::add_input_port…` had ever been measured today):

- **27 × `ordered-map root must be explicitly retired before drop`** and
  **13 × `final Dictionary ownership must be explicitly retired or owned by a cold boundary`** — tests
  that build a `FlowFixture` / `FlowRetainedVcs` and DROP it without driving any close ladder.
  Backtraces confirm it: `host::tests::fixture_json_round_trip` panics in
  `<OrderedMap<WidgetLayout> as Drop>::drop` called directly from the test body, and
  `retained_vcs_malformed_sources_…` panics from `drop_glue::<FlowVcsDocument>` with the ladder never
  entered (`debug-3.txt`, `debug-4.txt`). Test-side ownership debt, not a frontier defect.
- **7 × value assertions** unrelated to bytes: `Integer(2)` vs `Decimal(2.0)`, `"flow"` vs
  `"flow.flow"`, a bundler output diff, an empty `layout` map.
- **2 × `FlowEvalSession must finish explicit close before drop`**, **2 × extrude solid output**, and
  six singletons (engine edge selection, `mem::replace` guard, VCS bridge progress, oracle source law,
  `flow.registry-retirement-full`, retained operator fault).

I did not fix these: ~58 sites across three lanes' live areas, and none of them is the frontier
family this lane owns. `flow.registry-retirement-full` is the one I would look at first if it is
mine — a slower drain fills `RETIRED_REGISTRY_CAPACITY` sooner — but I did not establish that and
make **no claim** either way.

### 5.3 The 11 generation3d FULL reds

Four are the `unit_tests::` four the coordinator named as other owners. The other seven:
`work_capacity::every_bounded_retained_route_answers_an_admissible_extent`
(`flow.registry-retirement-full`), `viewer::…eval_chain_tests::a_late_contributions_install…`
(`P3 production envelope load did not reach terminal`), and five
`viewer::…preview::component::tests` tessellation laws ("the first render of a fresh document must
actually tessellate", "at least one preview mesh", "wireframe must drop the shaded triangle
channels", "at least one instance", "nodeGraphViewport decodes from its own action id"). None is a
close-ladder failure. They are in the viewer/wgpu lanes' live area and I did not investigate them.

## 6. Peer breakage I fixed forward, and peer work I did not touch

- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1212` — a peer's 20:15 edit derived `Eq` on a struct holding
  `RequestOutcome` without deriving `Eq` on the enum; `semio-framework` did not compile for the whole
  workspace. I waited 3 minutes, then added `Eq` to `RequestOutcome` (its payloads are `Vec<u8>`).
  One token. Noted here so its author knows.
- `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs` grew a new uncommitted region
  `🖐️GestureHistoryRetirement` (`a_gesture_that_changed_nothing_retires_its_history_baseline`) while I
  was running. It is green in the final run. I changed nothing in it.
- I reverted no hunk of any peer's work, and I did not touch the keyboard fixture, the
  `nodeGraphViewport` bridge, the `module.vcs` remote-merge guard or the
  `generation3d-publication.contended` lease.

## 7. What is NOT claimed

- **Not fully green.** 58 of 220 flow `--lib` laws and 11 of 438 generation3d laws still fail. §5.2
  and §5.3 classify them; none is of this family, and I established a root cause for none of them.
- **The `5 / 2` flow baseline is peer-measured**, not mine (§5.1).
- **No production or browser claim.** Everything here is the native `--lib` harness. I started no dev
  server, restaged nothing, and booted no plugin. The user-path argument for `FlowEvalSession` —
  every flow evaluation session in the procedural 3d editor closes through it — is a code-path
  argument, not a measurement.
- **`allocated_bytes()` semantics are unchanged**, but they are now the ONLY place a machine-width
  backing capacity is counted. If any consumer was relying on `released_bytes` to sum to a physical
  total, that consumer is now wrong; I found none, and I did not audit the wasm/TS bridges for it.
- **`✏️s/🔌️plugins/🌊️flow/…/👥️presence/♻️retirement/🦀️.rs:35`** (named by
  `📓️disposer-teardown-ownership` §6) is **unverified** — it drives its own owner type.
- **`🧵️retained/🧪️tests/🧵️retained/🦀️.rs` still does not compile** (it calls
  `FlowRetirement::next_push_allocation_bytes` / `reserve_push_allocation`, which the lib has never
  had). That is a live peer refactor, reported by two earlier lanes, and I did not touch it — so the
  `semio-framework-artifact-flow-flow` package's OWN test target was not run by me.
- **Ticket lifecycle untouched**, per the coordinator's instruction to this lane.

## 8. Files

Source:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs` — `Owner::Bytes` drawdown, `byte_owner`, vector-backing release, `next_close_byte_demand`, `close_step` docstring.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs` — the three laws in §3.
- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs` — ordered-key drawdown.
- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧪️tests/🗂️ordered/🦀️.rs` — its law.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` — `root_backing_credit`, `release_root_backing`, paid page release, `close_page`, demand.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs` — `FlowFixtureRetirement`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs` — `FlowMutationRetirementFrontier`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs` — selected-copy disposer.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `FlowHostRetirement::close_page`, `FlowEvalSession::close_step` docstring.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs` — operation/session close + witness.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` — `generation3d_close_flow_frontier` delegates.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — one-token peer fix-forward (§6).

Ticket: this report and my own logs under `🗑️generated/flow-eval-retirement/`.
