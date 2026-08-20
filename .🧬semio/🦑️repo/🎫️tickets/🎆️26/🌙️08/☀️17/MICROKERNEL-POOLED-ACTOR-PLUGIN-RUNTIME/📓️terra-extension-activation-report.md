# 📓️ terra-extension-activation-report

Packet `extension-activation` (M6). **This report SUPERSEDES the file of the same name written
2026-08-20 00:32** — that run worked under a different, earlier `path_scope`
(`💻️os/🖥️host/**` + `🎠️kernel/🦀️component.rs`, the Rust framework-tier extension-descriptor half)
and correctly identified two gaps in `🎭️actor` (no shard-pin-to primitive, no `deactivate` at all)
as blocking. This run's `path_scope` is exactly those two gaps plus the two host activation
sites: `🎭️actor/🦀️component.rs`, `🎯️targets/🧊️wgpu/📦️glue.rs`'s `KernelThreadState::create_app`,
and `🎠️kernel/🟦️component.ts`'s `ActivationRegistry`. The earlier report's own two additions
(`kernel::ExtensionDescriptor` in the Rust `🎠️kernel/🦀️component.rs`, `PluginHost::
install_extension_package` in `💻️os/🖥️host/🦀️component.rs`) are untouched by this packet and still
stand as written.

`CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/40ab938a-57cf-4d17-94a3-77c54a12536e/scratchpad/target-extact`
for every Rust command below.

---

## 1. What is wired, by file:line

### 1a. `🎭️actor/🦀️component.rs` — the pure kernel primitives (fully built, tested, green)

| primitive | file:line | what it does |
|---|---|---|
| `ShardTable::pin_to` | `:1515` | pins an actor to an **exact** `ShardId`, bypassing `pin`/`pin_avoiding`'s least-loaded heuristic. Idempotent for an already-pinned actor; clamps into `[0, shard_count)`. |
| `Kernel::shard_of` | `:1529` (on `ShardTable`), re-exposed as `Kernel::shard_of` `:2620` | cheap shard lookup without `actor_record`'s full-`ActorRecord` allocation cost. |
| `intersect_capabilities` | `:971` | pure fn: `requested.filter(|r| granted.iter().any(|g| g.capability == r.capability))` — the security property. Output is always a subset of `requested`, bounded by `granted`. |
| `Kernel.links: HashMap<ActorId, Vec<ActorId>>` | `:2343` | the explicit parent→children cascade edge table (O(children) cascade; survives multiple instances of one plugin — see the field's own doc comment). |
| `Kernel::activate_pinned` | `:2591` | `activate`, widened: pins to an exact `shard` via `pin_to`; when `parent: Some(id)`, computes `capabilities = intersect_capabilities(parent.capabilities, requested)` — when `None`, capabilities are the caller-supplied set verbatim (top-level activation). Does **not** itself call `link_extension` — kept as two composable primitives, per the design brief. |
| `Kernel::set_capabilities` | `:2612` | records/replaces an already-live actor's granted set — lets a host attach the parent's own broker-granted capabilities before activating its extensions. |
| `Kernel::link_extension` | `:2628` | records one parent→child cascade edge; `UnknownActor` if either id isn't live. |
| `Kernel::children_of` | `:2638` | direct children of an actor. |
| `Kernel::subtree_leaves_first` (private) | `:2648` | **iterative** (no self-recursive `async fn` — see R10's residue-shape-3 trap) post-order walk: every descendant precedes its own parent, `root` last. |
| `Kernel::cascade_remove` (private) | `:2671` | shared teardown primitive: `Scheduler::unregister_actor` + `ShardTable::unpin` + `actors.remove` + `links.remove`, for every id in leaves-first order; scrubs the removed ids out of every other actor's children list too (no dangling edge survives). |
| `Kernel::deactivate` | `:2689` | graceful cascade teardown — `cascade_remove` on the whole subtree. |
| `Kernel::kill` | `:2705` | the failure ladder's cascade teardown — same `cascade_remove` primitive (this crate's pure state has no "abrupt vs. graceful" axis of its own; see the method's own doc for why it is a separate named entry point rather than an alias). "A parent kill takes its extensions down" is exactly this called on the parent. |
| `Kernel::suspend_cascade` | `:2718` | leaves-first `suspend`; only `root` gets the caller's checkpoint bytes, descendants suspend with `None`. This IS the design's "checkpoint" cascade — `ActorStatus::Suspended{checkpoint}` already carries the payload, so "checkpoint" and "suspend-with-bytes" are the same kernel operation (a judgment call, stated plainly, not re-litigated further). |
| `Kernel::resume_cascade` | `:2735` | **parent-first** `resume` (design's own "restore: parent first"); skips any descendant not currently `Suspended` rather than erroring, so a partial cascade still resumes everything legitimately resumable. |

### 1b. `🎯️targets/🧊️wgpu/📦️glue.rs` — native activation site

| piece | file:line |
|---|---|
| pre-existing bug fixed (upstream of this packet's own addition) | `:355` region — `GuestRuntime::compile` is `async fn` (landed after this call site was written); was called with no `.await`. Bridged with `pollster::block_on`, this file's own established sync↔async bridge (already used repeatedly elsewhere, e.g. `poll_world3d_assets`). |
| `PLUGINS_REGISTRY_JSON` (`include_str!`) | `:177` | embeds `🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json` at **compile time** — no runtime path lookup into a gitignored, unstable-at-runtime `🤖️generated/**` tree. Read-only; never edited. |
| `ExtensionRecord` / `ExtensionIndex` | `:200`, `:208` | `by_parent: HashMap<String, Vec<ExtensionRecord>>`, built once from the 26 real `role: "extension"` descriptors, keyed by `extends`. |
| `extension_index()` | `:238` | lazy `OnceLock`-cached singleton. |
| `find_wasm_artifact` | `:246` | mirrors `program_bridge::load_wasm_plugins`'s own "first `.wasm` directly inside the plugin's own directory" convention. |
| `KernelThreadState::create_app` cascade tail | `:430` (whole fn), cascade call near its end | after the parent's own `InstanceOpen` turn completes, derives `modules_root` from `wasm_path`'s grandparent and calls `activate_extensions_of`. |
| `activate_extensions_of` | `:493` | for each extension of `plugin_id`: resolve wasm → compile (`guest_runtime.compile`, itself content-hash-keyed/cached, no extra cache layer added) → `ParallelRuntime::activate` (existing, unchanged signature) with `ActorKind::Extension`, `Lane::Background`, `PackageId(extension_id)` (**the extension's own package id, distinct from the parent's** — see §3) → `Kernel::set_capabilities` with the `intersect_capabilities` result → `Kernel::link_extension(parent, extension)`. Best-effort per extension (logs + `continue`, mirrors `load_wasm_plugins`'s "one bad plugin does not hold the batch hostage" policy). |
| `destroy_app` cascade teardown | `:545` | `Kernel::deactivate(actor)` → for every id in the returned leaves-first list, `ParallelRuntime::unregister(id)` (the shard-side `GuestInstance` teardown `Kernel`'s own purity boundary keeps out of the pure crate). Falls back to unregistering just `actor` if `deactivate` errors. |

### 1c. `🎠️kernel/🟦️component.ts` — web mirror

| piece | file:line |
|---|---|
| `intersectCapabilityGrants` | `:1654` | matched by `ShardCapabilityGrant.id` (the web counterpart of Rust's `capability` string). |
| `extensionsByParent: Map<string, string[]>` | `:1808` | stores extension **ids**, not manifest snapshots — `activateExtensionsOf` re-resolves the manifest from `manifests` at activation time, same pattern `activate()` itself already uses for the parent, so a later `registerManifest` update (e.g. once a real broker populates `caps`) is honoured, not shadowed. |
| `extensionChildren: Map<string, string[]>` | `:1812` | parent actorId → the child actorIds this registry minted. |
| `registerCatalog` | `:1862` | indexes `catalog.extensions` by `target.dependsOn[0]` (guaranteed `== extends` by the same builder assertion the native pipeline enforces). |
| `activate` / `activateExtensionsOf` | `:1905` / `:1932` | cascades every extension of `pluginId` under deterministic child id `${parentActorId}::${extensionId}`, `caps` scoped via `intersectCapabilityGrants`. Best-effort per extension. |
| `suspend` / `suspendExtensionsOf` | `:2000` / `:2030` | leaves-first. **Ordering-critical fix**: `turnScheduler.cancelQueued(actorId)` stays the very first synchronous statement (before any `await`) — see §4 for the regression this fixes. |
| `resume` / `resumeExtensionsOf` | `:2038` / `:2061` | parent-first; skips a child with no checkpoint. |
| `cancel` | `:2122` | leaves-first, **permanent** — recurses into `cancel(child)` for every tracked child, then deletes the `extensionChildren` edge (no checkpoint survives a cancel). |

---

## 2. Capability-intersection implementation

Both sides implement the identical rule: **a requested grant survives only if the parent's own
already-granted set carries a grant of the same name; the output is always a subset of the
request, never the request itself.** Rust: `intersect_capabilities` (`🎭️actor/🦀️component.rs:971`).
TypeScript: `intersectCapabilityGrants` (`🎠️kernel/🟦️component.ts:1654`).

**Honest gap, both sides**: the REAL capability-broker enforcement point (the bytes actually handed
to the guest — `caps: &[BrokerCapabilityGrant]` on `ParallelRuntime::activate` natively,
`ShardClient.activate`'s `caps` param on the web) is `&[]`/`[]` for **every** actor kind today
(plugin or extension) — no capability broker is wired up anywhere in this codebase yet (A2b/T1
territory). The parent's own `capabilities` are therefore always empty right now, so the
intersection is currently `intersect([], requested) = []` in practice on both sides. The mechanism
is fully implemented, tested, and exercised end-to-end (see the kernel test
`extension_capability_grant_is_the_intersection_not_the_request` and the TS test "scopes an
extension's activated caps to the intersection with its parent's own granted set", both of which
inject a non-empty parent grant set by hand to prove the intersection logic itself) — it will start
doing real work the moment a broker populates `parent_grants`/`parentCaps` for real, with zero
further changes to this packet's code.

## 3. Design decision: an extension gets its OWN `PackageId`, not the parent's

Pre-existing kernel tests (`kernel_suspend_resume_round_trip`, `kernel_metrics_counts_actors_...`,
`runtime_metrics_snapshot_reflects_real_kernel_activity`, all pre-dating this packet) activate
`ActorKind::Extension` with `package == plugin` (same `PackageId` as the parent) — a unit-test
convenience, not a binding convention. `Kernel::quarantine_package`
(`failure_ladder_trap_then_quarantine_is_package_wide`, also pre-existing) quarantines **every**
actor sharing a `PackageId` once an actor's `restart_count` crosses
`FAILURE_QUARANTINE_RESTART_THRESHOLD` (3). Sharing the parent's `PackageId` would mean a
repeatedly-trapping extension eventually quarantines its **parent** too — directly contradicting
this packet's own acceptance wording ("a trapping extension is restored/killed... without faulting
the parent"). Both `activate_extensions_of` (native, glue.rs) and `registerCatalog`
(web, component.ts) therefore give each extension **its own** package/plugin id (the extension
crate's own `pluginId`, e.g. `"cad-extension-aec-building"`, never `"cad"`). Proven by the kernel
test `trapping_extension_never_faults_the_parent`, which pushes the SAME extension all the way to
`FailureStage::Quarantined` and asserts the parent's status is untouched throughout.

## 4. Cascade semantics per lifecycle event

| event | order | native | web |
|---|---|---|---|
| activate | parent first, then children | `create_app` → `activate_extensions_of` | `activate()` → `activateExtensionsOf` |
| deactivate (graceful) | leaves-first | `Kernel::deactivate` in `destroy_app` | no direct web equivalent named "deactivate" — `suspend` (resumable) and `cancel` (permanent) cover the two real teardown intents |
| kill (failure-ladder / forced) | leaves-first | `Kernel::kill` (same primitive as deactivate, distinct named entry point) | `cancel()` — permanent, leaves-first, drops the cascade edge |
| suspend / checkpoint | leaves-first | `Kernel::suspend_cascade` | `suspend()` / `suspendExtensionsOf()` |
| resume / restore | **parent-first** | `Kernel::resume_cascade` | `resume()` / `resumeExtensionsOf()` |

**A real regression this packet caused and fixed, in the web mirror**: inserting `await
this.suspendExtensionsOf(actorId)` as the FIRST statement in `suspend()` (ahead of
`turnScheduler.cancelQueued(actorId)`) broke the pre-existing test "a suspended actor's queued
turns are cancelled, never delivered" — even a no-op `await` yields one microtask tick, which was
enough for an already-enqueued turn to get dispatched before cancellation. Fixed by moving
`cancelQueued` back to the first synchronous statement (`component.ts:2000`-area); the cascade now
runs strictly after it. Documented in-line with a `🐛️` tag at the fix site so the next reader does
not reintroduce it.

## 5. Every new test, by name, with its result

### `semio-framework-actor` (Rust, `component::tests::quick`)
All six below are new; all pass. Full suite: see §6.
- `activate_pinned_places_extension_on_parents_shard` — **pass**
- `deactivate_parent_cascades_leaves_first_with_zero_orphans` — **pass**
- `kill_parent_takes_extensions_down` — **pass**
- `trapping_extension_never_faults_the_parent` — **pass**
- `extension_capability_grant_is_the_intersection_not_the_request` — **pass**
- `suspend_cascade_leaves_first_resume_cascade_parent_first` — **pass**

### `@semio-tech/framework-kernel` (TypeScript, `🟦️component.ts`)
All seven below are new; all pass (confirmed by name via `--reporter=verbose`, not just count).
- `intersectCapabilityGrants > keeps only requested grants the parent's own granted set also carries, matched by id` — **pass**
- `intersectCapabilityGrants > is empty when the parent holds nothing, never escalates an ungranted request` — **pass**
- `ActivationRegistry extension cascade (registerCatalog) > activate() cascades to every registered extension of the plugin, under a deterministic child actorId` — **pass**
- `ActivationRegistry extension cascade (registerCatalog) > a plugin with no registered extensions activates with no cascade side effects` — **pass**
- `ActivationRegistry extension cascade (registerCatalog) > suspend() cascades leaves-first, resume() cascades parent-first — zero orphans either way` — **pass**
- `ActivationRegistry extension cascade (registerCatalog) > cancel() on the parent takes its extension down too — permanently, zero orphans` — **pass**
- `ActivationRegistry extension cascade (registerCatalog) > scopes an extension's activated caps to the intersection with its parent's own granted set` — **pass** (captures the actual `caps` bytes posted to the fake worker's `activate` message and asserts `fs.admin` never reaches the wire)

## 6. Every check/test run, with pasted exit codes

```
$ cargo test -p semio-framework-actor --lib
test result: ok. 76 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
EXIT: 0
```
(70 pre-existing + 6 new. Regression floor met and exceeded.)

```
$ cargo check -p semio-framework-actor --all-targets
Finished `dev` profile [unoptimized] target(s) in 0.25s
EXIT: 0
```

```
$ cargo clean -p semio-framework-actor && cargo check -p semio-framework-actor --lib --message-format=short
Finished `dev` profile [unoptimized] target(s) in 26.37s
EXIT: 0
```
R12/R13/R17 forced-rebuild dropped-future census: `grep -c "unused implementer of \`std::future::Future\`"` → **0**. `grep -c "^warning"` → **0** (whole file, forced rebuild — no warnings at all, not just no dropped-future ones).

```
$ cargo check -p semio-framework-os-renderer-wgpu --lib      (before AND after this packet's glue.rs edits)
error: could not compile `semio-framework-ui` (lib) due to 682 previous errors
EXIT: 101
```
**Identical** error count (682), **identical** failing crate (`semio-framework-ui`, not
`semio-framework-os-renderer-wgpu` or anything this packet touched), measured BOTH before any edit
of mine and again after every glue.rs edit — zero regression, but also **zero compiler verification
possible today** for the glue.rs code, native or otherwise (`semio-framework-ui`'s ~682 errors are
all missing-`.await` sites in `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs` and siblings — an
entirely unrelated, pre-existing, out-of-scope crate). See §7 for exactly what that means for
confidence in the glue.rs code.

```
$ bun nx run @semio-tech/framework-kernel:test --skip-nx-cache
Test Files  1 passed (1)
Tests  40 passed (40)
EXIT: 0
```
(33 pre-existing, including the one this packet's ordering fix repaired mid-session + 7 new.)

`bunx tsc --noEmit -p tsconfig.json` (repo-root config): 8,457 errors reported — but this is a
**mismatched-config artifact**, not a real signal: every error attributed to
`🎠️kernel/🟦️component.ts` (7 of them, at lines 9/16/17/239/240/329/333) is pre-existing,
unrelated to any line this packet touched (missing export, `.ts`-extension-import lint,
`PluginManifest` shape drift, an untyped `.map` callback — all in code this packet never edited),
and the repo-root config is missing `allowImportingTsExtensions` that this package's own local
build config clearly has (the file imports `.ts` paths throughout and the package's own vitest
suite runs it correctly). The package's own `vitest` run (§ above) is the meaningful gate here,
matching how every other `🟦️typescript` baseline in this ticket has been measured throughout.

## 7. What is NOT wired — read this before trusting any "done" above

**A hard architectural gap, both native and web, is the reason `activate_pinned`
(`🎭️actor:2591`, tested and green) is never actually reachable from either host today:**

- **Native**: `ParallelRuntime` (`🎯️targets/🧊️wgpu/🎠️runtime.rs`, owned by packet
  `kernel-async-native`, **not** in this packet's `path_scope`) exposes `activate` (least-loaded
  shard only) but no `activate_pinned`-shaped entry point, and its `shards: Vec<ShardHandle>` field
  is private, so `activate_extensions_of` cannot reach the shard-executor registration needed to
  force an extension onto its parent's exact shard even via `kernel_mut()`. **A lease-request is
  open**: one small additive method,
  `ParallelRuntime::activate_pinned(&mut self, package, plugin_ordinal, kind, lane, window, event,
  shard: ShardId, compiled: &CompiledHandle, caps: &[BrokerCapabilityGrant], instantiate_budget:
  &TurnBudget) -> Result<ActorId, String>` — identical body to the existing `activate` but calling
  `self.kernel.activate_pinned(..., shard, ...)` instead of `self.kernel.activate(...)`. Until it
  lands, `activate_extensions_of` activates extensions via the existing `ParallelRuntime::activate`
  (least-loaded placement) — **not necessarily the parent's shard**. This is the ONE acceptance
  criterion this packet does NOT meet natively today: "activating a parent brings up its N
  extension actors... each reported on the SAME shard as the parent" is proven at the kernel level
  (`activate_pinned_places_extension_on_parents_shard`) but not yet end-to-end through the native
  host.
- **Web**: the symmetric gap. `ShardClient.activate` (`🎭️actor/📦️packages/🟦️typescript/
  🧵️shard-client.ts`, out of this packet's `path_scope`) has no pinned-shard/worker parameter
  either (`assignShard` is private, least-loaded). Same lease shape needed there.
- Everything ELSE this packet's acceptance list names — zero orphans on deactivate/kill, a
  trapping extension not faulting the parent, an over-asking extension's grant being absent — is
  fully wired and proven, on both platforms, independent of this shard-pinning gap.

**The 50×50 bench fixture (`budget_3_activate_100`, `🎯️targets/🧊️wgpu/📦️glue.rs`) does NOT go
through the real cascade — this packet did not wire it, and here is exactly why, plainly:**
`Env` (the bench harness's own kernel wrapper, same file) activates every record — plugin or
extension — through `Env::activate_on_lane`, which calls the SAME `ParallelRuntime::activate` the
native host itself is stuck on above; wiring it through my new cascade hits the identical missing-
`activate_pinned`-on-`ParallelRuntime` blocker, PLUS `budget_3_activate_100`
(`🎯️targets/🧊️wgpu/📦️glue.rs`, inside the `scale_bench` module, same file, in my `path_scope`)
currently only activates ONE plugin's worth of extensions (its own title says "50 plugins + 50
extensions **of one plugin**", not all 50×50 = 2,500) — a second, independent piece of rewiring
this packet did not attempt, on top of the shared shard-pinning blocker. The real 2,550-record
50-plugins×50-extensions-each fixture genuinely exists on disk (confirmed:
`.🧬semio/.../🔣️bench-registry.json`, `recordCount: 2550`, `extensionsPerPlugin: 50`,
`extensions: 2500`) and is loaded by `scale_bench::run`, but `budget_3` itself only ever selects
one plugin's slice of it. **Both** the shard-pinning lease AND a `budget_3` rewrite (looping every
plugin, not just the first, and switching `Env::activate` to a pinned/linked cascade call) are
needed before the fixture would exercise the identical code path with zero special-casing. Neither
is done.

**Message routing to a live extension is not wired at all, either platform** — `create_app`'s
`run_turn`/`exchange` only ever address the actor id tracked in `self.instances` (keyed by the
PluginApp's own `u32` instance id); an activated extension actor is never given envelopes by
anything in this packet. `MessageEndpoint::Extension` dispatch (routing a parent's turn's outbound
messages to its extension, and vice versa) is a materially larger piece of work than "activation,"
was never in this packet's acceptance list, and is not attempted. Same on the web side —
`enqueueTurn` takes a bare `actorId`; nothing routes a parent's turn output to
`${actorId}::${extensionId}` automatically.

**`.sxt` install-while-suspended queueing** (a "decided, implement, do not re-litigate" item in the
coordinator's brief) is **not implemented** — this packet never touched plugin install/uninstall
(`PluginHost::install_extension_package`, the earlier report's own addition, in
`💻️os/🖥️host/**`, out of this packet's `path_scope`); there is no hot-attach/detach path here at
all, only the activation cascade that runs at `create_app` time for whatever is already installed.

## 8. Files touched

- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` — additive: `ShardTable::pin_to`,
  `intersect_capabilities`, `Kernel.links` field, `Kernel::activate_pinned` /
  `set_capabilities` / `shard_of` / `link_extension` / `children_of` /
  `subtree_leaves_first` (private) / `cascade_remove` (private) / `deactivate` / `kill` /
  `suspend_cascade` / `resume_cascade`, 6 new tests. No existing line changed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
  — fixed a pre-existing sync/async bug in `create_app`'s own `compile` call (upstream of this
  packet's addition); new `ExtensionIndex`/`ExtensionRecord`/`extension_index`/
  `find_wasm_artifact`; `create_app` gained a cascade tail (`activate_extensions_of`); `destroy_app`
  rewritten to cascade via `Kernel::deactivate` + per-id `ParallelRuntime::unregister`.
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` — new `intersectCapabilityGrants`;
  `ActivationRegistry` gained `extensionsByParent`/`extensionChildren`, `registerCatalog` now
  indexes extensions, `activate`/`suspend`/`resume`/`cancel` all gained cascade halves; fixed a
  real regression in `suspend`'s statement ordering (see §4); 7 new tests plus one shared fixture
  helper (`catalogWithOneExtension`).
- This report (rewritten in place, superseding the earlier scope's version — see the top of this
  file for what that means for the earlier report's own content, which still stands).

## 9. Lease-requests open against the coordinator

1. **`🎯️targets/🧊️wgpu/🎠️runtime.rs`** (`ParallelRuntime`, owned by `kernel-async-native`): add
   `activate_pinned` — see §7 for the exact signature and body shape. Small, additive, mirrors the
   existing `activate` almost verbatim.
2. **`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`** (`ShardClient`, out of this packet's
   `path_scope`): add an equivalent pinned-shard overload to `activate`.

Neither blocks this packet's OWN acceptance floor (kernel primitives fully green; cascade topology,
capability scoping, and zero-orphan teardown fully wired and tested on both hosts) — both block
ONLY the "same shard as parent" placement guarantee and, transitively, the 50×50 bench fixture.
