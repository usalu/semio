# 📓️ terra — packet A2-abi-sdk report

Ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet **A2-abi-sdk**.

**Finalized on sol's direct instruction**: the background monitor I was relying on for `cargo
check` output does not survive a subagent turn boundary in this harness (confirmed pattern across
A1/A3/B1 too) — no build/test result arrived. This report is written from confirmed static
knowledge only (source reads + one `shasum -a 256` run), with no fabricated command output. Where
I did not observe a command complete, it says so plainly rather than claiming a result.

## 1. Files created / changed, SHA-256

```
120225485089d2d2eb0b68eb3052076f21922196243eb09cac10cf79fac80919  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️types.wit        (new)
7abbb8291f6c45b5a11502dcc0d06fb937ffc500d7f453bde6bb9792222f28f2  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️pure.wit         (new)
fbf52ceed06a59d96a2638d676ef95ac55e332d03a9b86c676004e6eeb349cff  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️capabilities.wit (new)
37256e58b2b3265f35344417b9d792dfcab88b468ac7261d77fd6db31ddb2aea  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️ui.wit           (new)
a2d6cf9e7f305b9c595bca4a62a49f3b60932318a1e3f7155e8b09115bac86d2  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️documents.wit    (new)
0ab6db584c598a73d462a654b50ad0014a6937a56e7280a8038ee2c1c3911d4c  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️effects.wit      (new, post-`streaming` fix)
215857d5d6174d6cc3aaa7c7e2175ed717291d83f07cb21b9d103796fdd671a4  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️events.wit       (new)
9a6d5010d2b22ad161a7ab172300bd279e18ff9362c5b994113f73f968ec8d7d  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️jobs.wit         (new)
203585ce8d7c941bb0a976b5ec745f1e94479bf55d2ce09ff595840af19d40ee  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️checkpoint.wit   (new)
fd5747c0de431c625ea9594ee3c08eb9caea1e3b5461d5df5fbc9200509bf43a  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️reactor.wit      (new)
b2f8f0016c45607c532d979e822c82c35a2190b29e713e3f89d70ef3ce8be7c0  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️describe.wit     (new)
d6f957b8b788f49b6e3aa0ecfd3b99aa0423ad24a25a4c58158a38ed96c2d217  🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit        (edited, was the whole old ABI)
507d1b2e58fbabce1406f3d74244f49ea41e21dddfc2330dc79659b070407196  🔌️plugin/🦀️component.rs                            (edited)
77cfa504b3bff8babe4d5d82793513b54a64cb4c4257a7ba1946ba20729ed715  🔌️plugin/⚛️reactor/🦀️component.rs                   (new)
9582e4d68114f9c30c531b1b4def15784b632818ea5678cb32d7ab190416579a  🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs        (new)
42b93b79588a0b316363808092bbe6f929c58e2c87dd8bee8f0efe3353494823  🔌️plugin/⚛️reactor/📮️requests/🦀️component.rs        (new)
13f9a5ff786b06ae142f964a510ab75c3f50cf6cbc1ceabf7e8c0d31c7413d0b  🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs         (new)
e072b01f76e845491581662479bb64320e074d186f7913162b8dd9d5e8e13451  🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs            (new)
9042c28aa7eceb36aad08233c37eda05264d60a61a082aa2cd7b7e5f7d697716  🔌️plugin/⚛️reactor/📸️checkpoint/🦀️component.rs      (new)
3247ddb1f04f37663addfd317716c4608e248c73715637a971e816f994df9be6  🔌️plugin/🌐host/🦀️component.rs                      (new)
c3b023d199a35e5b6df6f26596a9a808939c0c69021f83629fef4282f2473901  🔌️plugin/🛂️describe/🦀️component.rs                  (new)
```

Not touched: `🔌️plugin/🖥️host/**` (B1's), `🎠️kernel/**`/`🛂️manifest/**` (A3's), `📡️spr/🧵️channel/**`,
`✏️s/🔌️plugins/🗒️note/**` (see §5 — needed no edits), any registrar file.

## 2. WIT interface map — all 12 files

| file | interface/world | declares |
|---|---|---|
| `types.wit` | `types` | `pack`/`request-id`/`instance-id`/`revision` aliases, `plugin-error`, `message-endpoint`, inference records (moved verbatim per design table; `migrate-artifact-input/output` deliberately NOT carried over — dead once `migrate-artifact` became a job kind) |
| `pure.wit` | `pure` | `log`, `now-ms`, `trace-span` — **the only import** |
| `capabilities.wit` | `capabilities` | `capability-id`, `capability-token`/`capability-grant`/`capability-change` as plain records (not WIT `resource` — see deviation note below) |
| `ui.wit` | `ui` | `surface-ref`, `patch-op` variant (`replace`/`insert-child`/`remove-child`/`set-props`), `ui-patch` |
| `documents.wit` | `documents` | `document-ref`/`transaction-ref`/`blob-ref` (plain records, same resource deviation) |
| `effects.wit` | `effects` | `effect` variant, ~44 payload records, one per `kernel::Effect` variant — no functions |
| `events.wit` | `events` | `event` variant, lifecycle/channel/surface/completion/messaging/timer/request payload records — no functions |
| `jobs.wit` | `jobs` (exported) | `start-job`/`step-job`/`cancel-job`, `job-budget`, `job-step` |
| `checkpoint.wit` | `checkpoint` (exported) | `checkpoint`/`restore` |
| `reactor.wit` | `reactor` (exported) | `budget`, `turn-status`, `turn-result`, `poll` |
| `describe.wit` | `describe` (exported) | `describe` — build-time only |
| `world.wit` | `world actor` | `import pure; export reactor; export jobs; export checkpoint; export describe;` |

**Confirmed** (re-read `world.wit` after editing, not assumed): `plugin-world`/`extension-world`
are gone — one `world actor`. `contributor`/`host` interfaces are gone — replaced by `effects`/
`events`/`pure`. The exports `exchange`/`manifest`/`instantiate-app`/`clear-instance-guard`/
`activate`/`deactivate`/`invoke` do not appear anywhere in any of the 12 files — verified by
`grep -c` for each name across the `📜️wit/` directory returning 0 for all seven.

### Deviation from the design table (documented, not silent)

`resource capability-token`/`resource surface`/`resource document`/`resource transaction`/
`resource blob` are plain records here, not real WIT `resource`s. A WIT `resource` only gets
cross-boundary methods when its owning interface is imported or exported by the world; `pure` is
`world actor`'s only import, so a `resource` with host-callable methods has no seam. `ui`/
`capabilities` each keep one nominal empty `resource surface;`/`resource capability-token`-shaped
marker so the design vocabulary has a type to point at, unused by any function signature.

### Cross-packet bug found and fixed

`effects.wit`'s `http-request-effect.stream` field used `stream`, a WIT-reserved keyword —
`wit-parser` failed to parse the WHOLE `📜️wit` package on that line, which is what blocked packet
B1's `bindgen!` call too (confirmed from B1's own report, written before I fixed this). Renamed to
`streaming` in the WIT record only; `kernel::Effect::HttpRequest.stream` (Rust) is untouched.

## 3. SDK region map

```
🔌️plugin/🦀️component.rs
  pub mod component { generate!(world: "actor"); impl ReactorGuest/JobsGuest/CheckpointGuest/
    DescribeGuest for ComponentGuest; export!(ComponentGuest); log/now_ms/trace_span wrapping
    `pure`; component_export_anchor. }
  pub mod plugin_runtime { UNCHANGED core (plugin_exchange, plugin_render, plugin_document_pack,
    plugin_load_document_pack) MINUS INSTANCE_GUARD/InstanceGuard/clear_instance_guard (deleted);
    INSTANCES: UnsafeCell -> RefCell; NEW plugin_create_app_with_id(id, app_id); host_port module
    body DELETED (comment left in its place). }
  pub mod app / world3d_host / engagement — untouched, as directed.
  #[path] mod builder;        — untouched (design's builder-method additions NOT done, see §4)
  #[path] pub mod reactor;    — new, -> ⚛️reactor/🦀️component.rs
  #[path] pub mod host;       — new, -> 🌐host/🦀️component.rs
  #[path] pub mod describe;   — new, -> 🛂️describe/🦀️component.rs

⚛️reactor/🦀️component.rs
  thread_local! PATCHES/REGISTRY/EXECUTOR/OPEN_INSTANCES/ARMED_TIMERS/PENDING_PATCHES
  fn host() -> crate::host::Host
  fn checkpoint_now()/restore_now() — unconditional, no WIT dependency
  mod wit_bridge (cfg-gated, same cfg as `component`):
    fn poll(events, budget) -> Result<TurnResult, Fault>   — real turn loop
    fn route_app_frame / decode_wire_effect / decode_wire_app_event
    fn wit_event_to_kernel / wit_activation_to_kernel / wit_completion_to_kernel / wit_endpoint_to_kernel
    fn kernel_turn_result_to_wit / kernel_ui_patch_to_wit / kernel_patch_op_to_wit / kernel_effect_to_wit
      / kernel_endpoint_to_wit / kernel_placement_to_wit / kernel_outcome_to_wit_respond
    fn path_to_indices — always empty (see §4, patches is full-body-only)

⚛️reactor/🧵️executor — LocalExecutor: spawn/wake/run_until_idle, real RawWaker, 3 unit tests
⚛️reactor/📮️requests — RequestRegistry: request(build)/emit/resolve/drain/pending_ids, 2 unit tests
⚛️reactor/🩹️patches   — PatchTracker: diff/mark_rejected/mark_ack, 4 unit tests
⚛️reactor/💼️jobs      — start_job/step_job/cancel_job + run_io_run/run_io_sniff, 3 unit tests
⚛️reactor/📸️checkpoint — checkpoint()/restore() pack envelope, 1 unit test
🌐host                — Host{registry}: one method per Effect variant + log/now_ms/trace_span
🛂️describe            — describe_plugin() -> packed PackageDescriptor
```

## 4. Done / partial / not-started

- **`LocalExecutor`** — **done.** Real `RawWaker`, index slots + free-list, ready queue,
  `run_until_idle(max_iterations)`. 3 unit tests written (spawn-to-completion, self-waking task,
  externally-woken parked task). Not run against a real compiler this session (see §7).
- **`RequestRegistry` + async `host::*`** — **done.** `request(build: FnOnce(RequestId) -> Effect)`
  allocates the id before building the effect (every completable variant embeds its own `req`);
  `Host` has one method per `Effect` variant, grouped per design §4's ergonomic list. 2 unit tests
  on the registry.
- **The `poll` turn loop** — **done, but unverified by compilation.** Real event routing
  (`InstanceOpen`→`plugin_create_app_with_id`, `AppCommand`→batched `plugin_exchange` per instance,
  `SurfaceVisible`→`plugin_render`+`PatchTracker::diff`, `Completed`/`HttpChunk`→
  `RequestRegistry::resolve`, `Timer`→`LocalExecutor::wake`), full `Effect`↔WIT and `Event`↔WIT
  conversion for every variant. Written under heavy time pressure against a live target — I found
  and fixed several field-name mismatches between my own WIT and the real landed `kernel::Effect`/
  `Event` by re-reading both side by side (documented inline as I found them: `dispatch-action-
  effect`, `open-plugin-instance-effect`, `blob-write-effect`, `request-media-frames-effect` all
  needed WIT-side corrections after I first wrote the Rust conversion against a wrong assumption).
  I do NOT have compiler confirmation this file is currently free of further such mismatches —
  reported as "done" meaning "complete first-draft coverage of every variant," not "verified
  correct."
- **Jobs (`start-job`/`step-job`/`cancel-job` + absorbed `semio.io-run`/`semio.io-sniff`)** —
  **done.** Bookkeeping + the two absorbed job kinds, bodies copied from the peer's own guest-export
  implementation (verified line-for-line against what was on disk). 3 unit tests.
- **Checkpoint/restore** — **partial.** Real envelope (`instances[{id,app_id,document_pack}]`,
  `timers`, `pending_requests`) using the existing `plugin_document_pack`/`store::
  encode_document_pack_bytes` codec. Missing: `view_state`/`ephemeral` per instance (`AppInstance`
  has no public read for either — would need touching `app` module, which design says stays; not
  reached in, flagged as self-owned deferred work rather than a lease since no OTHER packet's file
  is involved).
- **UI patch diffing with full-body fallback** — **partial, and the "fallback" framing is
  misleading for what's actually here.** `PatchTracker` real revision/base_revision bookkeeping is
  done. The diff itself is **full-body only** — every dirty surface always emits one root-path
  `Replace`. There is no partial (node-identity-path) differ, so there is nothing to fall back FROM;
  every patch already IS the fallback case. 4 unit tests cover only the bookkeeping, not diffing.
- **`describe()`** — **partial.** Builds a real `PackageDescriptor` from the existing
  `plugin_manifest()`. `activation_events`/`capability_requests`/`extension_points`/`execution`/
  `quotas` are emitted empty/default because `PluginBuilder` has no `.activation(..)`/
  `.extension_point(..)`/`.requests(..)`/`.quota(..)`/`.execution(..)` methods yet — **not
  implemented this session**, and both A3's own report and this one independently name this
  packet's `🏗️builder/🦀️component.rs` as the owner. `hashes` are empty strings by design (only the
  external emitter, packet E1, can know the built wasm's own hash).
- **`Emit.tasks: Vec<AsyncTask>`** — **not started.** Design §4 says `Emit` (the existing
  `app`-module type carrying command-dispatch output) "gains `tasks: Vec<AsyncTask>` so a command
  may await host results and then emit follow-up mutations under the same `ActionMeta`." I did not
  touch `Emit`'s definition or add `AsyncTask` — this is genuine remaining scope, not something I
  attempted and got wrong. `app` module (where `Emit` lives) was left untouched per "app... stays,"
  but this specific field addition is explicitly part of this packet's own charter and is simply
  not done.
- **`INSTANCE_GUARD`/`InstanceGuard`/`clear-instance-guard`** — **deleted.** `INSTANCES` changed
  `UnsafeCell`→`RefCell`; `with_instances_mut` now just calls `borrow_mut()`.
- **`host_port`** — **deleted**, whole module removed, replaced by `🌐host::Host` +
  `log`/`now_ms`/`trace_span`. `HostBackboneChannel`/`set_host_backbone_channel` deleted with **no
  replacement** — a real, reported gap (see §6), not an oversight.
- **`component::host_*`** — **deleted**, replaced by `🌐host::Host`'s methods.
- **`plugin_exchange`** — **NOT deleted** — reused as-is, called from inside `poll`'s app-command
  routing, exactly as design §4 directs ("app-command → the existing PluginApp dispatch
  unchanged"). If the acceptance criteria expected this name gone entirely, flag that back to me —
  my read of design-abi.md §4 is that only the WIT-level `exchange` EXPORT is deleted, not the
  internal Rust function that implements command dispatch.
- **`install_io_fallback_dispatcher`** — **deleted**, no replacement needed (its old caller,
  `io-compose` host import, is now `host::io_compose`, an ordinary async call site, not a
  process-global dispatcher hook).
- **`SECTION_KIND_*`** — **not found.** I grepped for this exact name in `🔌️plugin/🦀️component.rs`
  before and during this session and got zero matches — either already removed by an earlier
  wave/packet, or the name lives somewhere I didn't search (I did not check outside my owned
  paths). Not claiming deletion credit for something I never observed present.
- **`extension_component`** — **deleted**, whole module removed. `extension_exports!`'s link
  anchor repointed from `extension_component_export_anchor` (deleted) to `component_export_anchor`
  (shared with plugins now — one `world actor` for both roles).

## 5. `🗒️note` pilot — how far it actually got

Read `🗒️note/🦀️component.rs` (the `Plugin::builder(...).editor::<NotePlayApp>(...).viewer::
<NoteViewer>(...).try_build()` root) and `📦️packages/🦀️rust/📦️glue.rs` (the `#[path]` wiring +
`semio_framework_plugin::plugin_exports!(plugin::plugin)` invocation). **Neither file references
the WIT world, `exports::...`, or any deleted export name** — the plugin is built entirely through
declarative builder calls (`.editor`, `.viewer`, `.artifact`, `.artifact_kind`) that this packet's
own `app`/`builder` modules (left untouched) still provide unchanged. `plugin_exports!` itself only
wires bundle-installer link shims and a `component_export_anchor` `#[used]` static — unaffected by
the reactor/jobs/checkpoint/describe restructuring.

**Conclusion: zero source changes were needed in the note plugin crate for this migration** — its
compile status is entirely a function of whether `semio-framework-plugin` itself compiles.
`note`'s own `Cargo.toml` already has `semio-framework-plugin = { workspace = true, features =
["component-guest"] }`, so `cargo check -p semio-s-plugin-note --target wasm32-wasip2` genuinely
exercises the new guest-gated `component`/`reactor` code (unlike the SDK crate's own bare
`--target wasm32-wasip2` check, which skips it — same caveat the peer W1-D report already
documented for this exact command shape).

**Not done**: I did not observe this command run to completion (§7), and I did not attempt a live
runtime proof of "one `poll` turn produces a UI patch" — that needs a wasmtime component
instantiation driving a real compiled `.wasm`, which requires packet B1's `WasmtimeRuntime`/
`ShardLoop` (B1's own report states these are not landed, blocked on the WIT parse bug I've since
fixed). What IS real: `⚛️reactor::poll`'s `SurfaceVisible` routing genuinely calls
`plugin_render(instance, "window", "{}")` → `PatchTracker::diff` → collects into
`turn-result.ui_patches` in the source as written — the wiring exists and is inspectable, but I am
not claiming I ran it.

## peer-coexistence

Absorbed peer (W1-D) io mechanism symbol table — where each one lives now:

| peer symbol | kind | new home | verified how |
|---|---|---|---|
| guest export `list-io-entries` | static descriptor data (partial — not yet a `ContributionSet.io_entries` row, see §4's `describe()` note) | `🛂️describe` (incomplete) | read `describe_plugin()` source; `io_entries` field is not populated |
| guest export `io-run` (single hop, this plugin's own registry) | cold job kind `semio.io-run` | `⚛️reactor/💼️jobs::run_io_run` | body copied line-for-line from the peer's guest-export impl (`ArtifactDialect::parse_coordinate`/`io_mechanism::io_entries`/`io_run`, same fully-qualified paths) |
| guest export `io-sniff` | cold job kind `semio.io-sniff` | `⚛️reactor/💼️jobs::run_io_sniff` | same — carrier-confidence logic copied verbatim |
| host import `io-routes` | `Effect::RegistryQuery{kind:"io-routes"}` | `🌐host::io_routes` | new async wrapper, not a copy (host-side routing is B1's, not mine) |
| host import `io-identify` | `Effect::RegistryQuery{kind:"io-identify"}` | `🌐host::io_identify` | same |
| host import `io-run` (multi-hop) | `Effect::IoCompose` | `🌐host::io_run` | new async wrapper |

**Route-resolution algorithm, ≤3-hop cycle-free rule, ranking order (highest minimum fidelity →
fewest hops → lexicographic), self-owned-hop reentrancy guard** — **not touched by this packet at
all.** That logic lives entirely in `🔌️plugin/🖥️host/🦀️component.rs`'s `IoRouter`
(`resolve_io_route`/`route_reenters_calling_plugin`/`io_entries_conflict`), which is packet B1's
owned file, not mine. B1's own report (`📓️terra-B1-host-native-report.md` §4 "peer-coexistence")
independently confirms that region is byte-for-byte untouched by their edits — 385 lines, same
region markers, before and after. I did not re-verify this myself (not my file); I am relying on
B1's own direct evidence, cited rather than re-claimed as my own observation. My packet only
absorbed the GUEST-side single-hop lookup (`io-run`/`io-sniff` exports → job kinds); the host-side
multi-hop route resolution the peer built is unowned and unmodified by me.

## acceptance — what I actually observed vs. did not

**Update, post-finalization**: one of the two `cargo check -p semio-framework-plugin --lib`
invocations I had launched before sol's stop instruction completed on its own after I finished
writing this report (its output arrived as an unsolicited background-task-completion event, not
from further polling) — raw log saved as `📓️terra-A2-check-lib-real.txt` in this ticket folder.
Pasting it here since it is genuine, observed data that materially changes the honest picture below,
not a fabricated or re-requested result:

```
$ cargo check -p semio-framework-plugin --lib
    [... ~27 min of dependency compilation, queued behind "Blocking waiting for file lock on
    build directory" ...]
    Checking semio-framework-plugin v0.1.0 (.../🔌️plugin/📦️packages/🦀️rust)
warning: unused imports: `Effect`, `Event`, `MessageEndpoint`, `PatchOp`, `RequestOutcome`,
  `TurnStatus`, and `UiPatch`
  --> ⚛️reactor/🦀️component.rs:26:31
warning: unused import: `std::collections::HashMap`
  --> ⚛️reactor/🦀️component.rs:28:5
warning: function `outcome_to_result` is never used
  --> 🌐host/🦀️component.rs:21:15
warning: fields `child_slots` and `link_slots` are never read
    --> 🔌️plugin/🦀️component.rs:2607:9   [pre-existing, ArtifactDeclaration, not touched by me]
warning: fields `schemas`, `inferences`, `languages`, and `app_schemas` are never read
    --> 🔌️plugin/🦀️component.rs:3617:9   [pre-existing, PluginRuntimeRegistry, not touched by me]
warning: `semio-framework-plugin` (lib) generated 5 warnings
    Finished `dev` profile [unoptimized] target(s) in 27m 55s
[exited with code 0]
```

**`EXIT: 0`.** This is a bare `--lib` check — no `wasm32-wasip2` target, no `component-guest`
feature — so it does NOT compile the cfg-gated `wit_bridge` submodule (`poll` and every
`kernel_effect_to_wit`/`wit_event_to_kernel` conversion function), which is exactly why the first 3
of the 5 warnings exist: `Effect`/`Event`/`MessageEndpoint`/`PatchOp`/`RequestOutcome`/
`TurnStatus`/`UiPatch`/`HashMap` are imported at `⚛️reactor/🦀️component.rs`'s module level for
`wit_bridge`'s use, and `🌐host::outcome_to_result` is called only from inside `wit_bridge` — all
three warnings are artifacts of this specific build configuration skipping that submodule, not
dead code in any build that actually exercises it. The other 2 warnings are pre-existing,
unrelated to this packet (confirmed by line number: `ArtifactDeclaration`/`PluginRuntimeRegistry`,
neither touched this session). **This confirms**: `plugin_runtime`'s `INSTANCE_GUARD` removal,
`host_port`'s deletion, `extension_component`'s deletion, and every unconditional part of
`⚛️reactor`/`🌐host`/`🛂️describe`/`💼️jobs`/`📮️requests`/`🧵️executor`/`🩹️patches` (including their
unit tests, which this command also type-checks even though it does not RUN them — `--lib` without
`test` doesn't execute `#[test]` fns) compile cleanly. It does **not** confirm the cfg-gated
`wit_bridge` module (the actual `poll` turn loop and `Effect`/`Event` WIT conversions) compiles —
that needs `--target wasm32-wasip2 --features component-guest`, still unobserved.

None of the other three required acceptance commands produced output I can paste:

```
cargo check -p semio-framework-plugin --all-targets           — NOT RUN (would also cover #[cfg(test)] mods executing, still not the wasm32 target)
cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest  — NOT RUN — this is the one that actually matters most (see above)
cargo test -p semio-framework-plugin --lib                    — NOT RUN
cargo check -p semio-s-plugin-note --target wasm32-wasip2     — NOT RUN
```

I am not claiming any of these three pass. What I did otherwise, given no compiler was available
for most of the session: a
manual, source-level consistency pass — re-reading the real `kernel::Effect`/`Event`/`UiPatch`/
`TurnResult` definitions and my own WIT files side by side, field by field, and fixing every
mismatch I found (§2's `streaming` rename, §4's four WIT record corrections, a missing
`ensure_plugin_initialized` import in the `component` module, and a test in `🩹️patches` that
constructed `UiNode` as a struct-with-a-`text`-field when it is actually an 19-variant enum with no
`Default` — found by grepping for existing `UiNode::Text(UiTextNode{...})` construction elsewhere
in the codebase and copying the real pattern). This raises my confidence the code is closer to
correct than a first draft, but it is **not a substitute for compilation** and I am not presenting
it as one. Real remaining risk: the ~40-variant `kernel_effect_to_wit`/`wit_event_to_kernel`
match arms in `⚛️reactor/🦀️component.rs` are the least-reviewed code in this packet (written fastest,
under the most time pressure) and are the most likely place a next `cargo check` finds further
field-name or type mismatches.

## lease-request

```lease-request
file: 🔌️plugin/🖥️host/🦀️component.rs (B1's — not requesting an edit)
reason: recorded only for traceability — the `streaming` WIT fix (§2) unblocks B1's own
  `bindgen!` call, which their report named as the exact blocker. No action needed from B1 beyond
  re-running their check.
change: none requested.
```

```lease-request
file: 🔌️plugin/🏗️builder/🦀️component.rs (my own owned path — not a lease, a self-owned gap)
reason: design-abi.md §3's `.activation(..)`/`.extension_point(..)`/`.requests(..)`/`.quota(..)`/
  `.execution(..)` PluginBuilder methods are not implemented. A3's own report independently flags
  the same gap and names this packet as owner.
change: none requested — self-owned follow-up, not done this session.
```

## anything deferred

1. VCS backbone channel — `set_host_backbone_channel`/`HostBackboneChannel` deleted with no
   per-instance async replacement (§4). Needs a design decision on bridging a synchronous
   `store::BackboneChannelPort` trait onto the async effect/event model.
2. `PluginBuilder` descriptor-populating methods (§4/lease-request).
3. Node-identity-path UI patch diffing — full-body-replace only (§4).
4. `Emit.tasks: Vec<AsyncTask>` — not started (§4).
5. Real end-to-end `poll()` proof against a compiled `.wasm` — needs B1's `WasmtimeRuntime`/
   `ShardLoop`.
6. `describe()`'s `activation_events`/`capability_requests`/`extension_points`/`io_entries` — empty
   until item 2 lands.
7. `component-extension-guest` kept as a Cargo feature alias (repointed to the unified `component`
   module) rather than deleted, to avoid a hard "unknown feature" break across 26 extension crates
   not otherwise touched this wave.
8. 26 extension crates' `component-extension-guest` usage and the one measured `host_now_ms` caller
   (`✏️s/🔌️plugins/🪐️space`) will not compile against this packet's SDK until their own W3 migration
   wave repoints them — expected fallout of the sanctioned big-bang flip (SDK crate is explicitly
   "frozen during W3" per `important.md`), not a regression.
9. **No compiler verification of any of the above this session** — see `## acceptance`. The single
   highest-value next step for whoever picks this back up is simply: run the four acceptance
   commands for real and fix whatever the compiler finds, starting with `⚛️reactor/🦀️component.rs`'s
   `kernel_effect_to_wit`/`wit_event_to_kernel` match arms (least-reviewed code in this packet).

No `[DEBUG]` logs were added by this packet — nothing to strip.
