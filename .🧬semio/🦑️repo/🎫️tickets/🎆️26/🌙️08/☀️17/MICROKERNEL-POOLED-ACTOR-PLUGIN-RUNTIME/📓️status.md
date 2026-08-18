# 📓️ Status — coordinator log (sol only, append-only)

## 2026-08-17 — W0 open

- Ticket opened: `🎫microkernelpooledactorpluginruntime`, goal `🎯r2602🎯runningsketchpad`, issue #2567. Registry `llm` enum has no `opus-5`; coordinator model is Claude Opus 5, recorded in the prompt.
- **Disk**: was 100 % full (4.5 GiB free) — cargo could not run. User approved removing the `🎯️target` build caches of the two CLOSED tickets `☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` and `☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` (verified as cargo caches: `CACHEDIR.TAG` / `.rustc_info.json`, no sources). After removal: **377 GiB free** (APFS reclaimed more than the two dirs measured). Nothing else deleted. Root `target/` (340 G) and all open tickets' caches left untouched.
- Scaffolding written: `📋️master.md`, `📌️important.md`, `📓️design-runtime.md`, `📓️design-abi.md`, `📓️design-workforce.md`.
- Dispatched W0 luna audits (`L0-imports`, `L0-consumers`) + `A1-actor` + `A3-kernel-types`.

### Design refinement at dispatch: A3 split, tree never goes red

The plan put channel v12 inside `A3-kernel-types`. Landing it there would have left the workspace uncompilable from the moment A3 finished until every W3 plugin packet completed — potentially days, with other sessions live in this tree. Split instead:

- **`A3-kernel-types`** (dispatched now): new `Effect`/`Event`/`UiPatch`/`Budget`/`TurnResult`/`Broker`/`Quota` + descriptor types, **plus the atomic mechanical `HostEffect` → `Effect` rename across all ~330 call sites in the same packet**. Purely additive + a rename, so the workspace stays green. Hard requirement stated in the packet: if Part 2 cannot land green, revert Part 2 (by editing, never git) and keep Part 1.
- **`A4-channel`** (new packet, queued): channel v12 — removing `AppFrame::{Welcome,UiSection,Effects,Events}` and `AppCommand::{Hello,Bye,AttachBackbone,DetachBackbone,RefreshUi}` and adding revisioned `ui-patch`. This genuinely cannot be atomic on its own, so it is dispatched **together with** `A2-abi-sdk` and `B1-host-native`, confining the red window to the W1→G1 gate instead of spanning all of W3.

`A1-actor` was told to treat `Effect`/`Event`/`UiPatch` as opaque pack bytes so it has no dependency on A3 — the two run fully in parallel with disjoint files.

### 21:05 — peer-ticket collision found; A2/B1 held

Pre-dispatch churn check on the hot files (`git log --date=iso` + `git diff --stat HEAD` + mtimes, per `📌️important.md`) found a **live** peer session: ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`, slice **W1-D**, whose report file was written at 21:02 — three minutes before the check. It holds ~1 660 uncommitted lines across `📜️world.wit` (+37), `🔌️plugin/🖥️host/🦀️component.rs` (+459), `🔌️plugin/🦀️component.rs` (+1259/−96) and `🎠️kernel/🟦️component.ts` (+341, region `🔖️IoRouter`). Details and the absorb-don't-delete mapping are recorded in `📌️important.md`.

Actions taken:
1. **`A2-abi-sdk` and `B1-host-native` held** — not dispatched. Rewriting the WIT, the guest SDK and the plugin host while a peer is mid-flight in all three would destroy unrecoverable work.
2. **`A3` amended in flight** (SendMessage): surgical region-scoped edits only, never a full-file rewrite, and the peer's `🔖️IoRouter` region in `🎠️kernel/🟦️component.ts` must come out byte-identical; it must report the region's line count before/after under a new `## peer-coexistence` section.
3. `A1` unaffected (its files do not exist yet anywhere else).

Escalated to the user: the whole critical path runs through A2/B1, so the two tickets need an explicit sequencing decision rather than a race.

### 21:10 — user decision: proceed, absorbing the peer's state

Hold lifted. `A2-abi-sdk` and `B1-host-native` dispatched with the working tree (not `HEAD`) as their baseline and an explicit absorb-don't-delete contract for the peer's io mechanism, plus: never rewrite a file wholesale, re-read from disk before every edit, and a mandatory `## peer-coexistence` section proving the peer's route-resolution semantics survive. Both were also told to report `blocked-on-A1`/`blocked-on-A3` rather than inventing duplicate types if the contract crates are still in flight.

W0 recon complete: `📓️luna-imports-audit.md` (576 lines) and `📓️luna-consumers-audit.md` (429 lines).

Measured numbers that refine the plan:
- **297** `HostEffect` occurrences (the plan estimated ~330), `LoadDocument` 112 / `ReplayShellCommand` 37 / `DownloadMediaExport` 36 / `DispatchAction` 27.
- Migration weight: **4 heavy** (🪐️space 81, 🧩️puzzle 21, 📏️layout 20, 🏗️fem 19), 20 medium, 9 small — this is the batching basis for W3.
- **No plugin calls a raw `host_*` import**; everything goes through framework-level effects. The W3 migration is therefore a rename plus semantic declaration work, not a per-plugin ABI rewrite — materially cheaper than planned.
- `WasmPluginRuntime` has 72 consumer sites; `exchange` has 40+; `pollster::block_on` has 19 sites of which only 3 are plugin-blocking (the rest are GPU/init and can stay).
- `LeasePool` has 11 consumers, only 4 plugin-specific — confirms the relocation of the generic helper rather than deletion.

Four terra executors now live: A1, A3, A2, B1.

### 21:2x — registrar: actor crate registered

`🎭️actor`'s `Cargo.toml` appeared, so as registrar I added two lines to the root `Cargo.toml` (the only root edit so far):
- member `"🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust"` after the `🕸️graph` member;
- `[workspace.dependencies]` alias `semio-framework-actor = { path = … }` so B1/A3 can consume it.

`cargo metadata --no-deps` exits 0 — the workspace stays valid for the peer sessions live in this tree. A1 notified; also flagged to it that `[lib] path = "📦️glue.rs"` needs that file to exist before any `cargo check` can succeed, and that `cargo-features` in a member manifest is inert (root-only).

### A1-actor — coordinator-verified (executor could not self-verify)

A1 ended its turn three times waiting on background cargo jobs, which do not survive a subagent's turn in this harness — a wake/idle loop that burned ~338k tokens without ever collecting a result. I stopped the loop, took the acceptance runs into the main session, and redirected A1 to write its report only. **Worth remembering for every future packet: executors must run builds in the foreground, never backgrounded.**

Verified by me, not taken from the executor's word:

| check | result |
|---|---|
| `cargo check -p semio-framework-actor --all-targets` | `Finished dev profile in 2m 38s` — zero warnings, zero errors |
| `cargo test -p semio-framework-actor` | **52 passed, 0 failed** |
| `cargo check --target wasm32-unknown-unknown` | in flight (13 concurrent cargo processes across sessions) |

Deliverable: `🦀️component.rs` 2 642 lines, `📦️glue.rs` 95, plus `🟦️component.ts`, `📜️script.ts`, `📋️project.json`.

Test coverage checked against the packet's required properties — all present, by name:
`drr_fairness_plugin_with_50_actors_does_not_starve_plugin_with_1` (the exact hierarchical-fairness property the design exists for), `deadline_preemption_runs_before_background_drr_deficit`, `mailbox_coalesces_latest_wins_older_dropped`, `mailbox_backpressure_rejected_when_full_and_nothing_lower_priority`, `mailbox_pop_next_honors_lane_priority_over_fifo`, `failure_ladder_escalates_and_decays_back_to_healthy`, `failure_ladder_trap_then_quarantine_is_package_wide`, `scene_revision_is_monotonic_and_reuses_snapshot_on_empty_commit`, `scene_ui_node_quota_truncates_and_signals`, `kernel_suspend_resume_round_trip`, `kernel_request_exclusive_then_release`, plus `actor_id` bit-packing and pack round-trips.

A1 later confirmed `cargo check --target wasm32-unknown-unknown` also exits 0, and filed honest gaps: `CapabilityGrant` is a local stand-in (depending on the os-product-tier type would invert framework/os layering); `🤖️generated/🟦️actor.ts` not yet emitted; the TS package dir is packet H2's `ShardClient` work; `SceneStore` can cap node counts but not structurally truncate patches, since `UiPatch` is opaque to the crate by design. **A1 accepted.**

**Purity constraint verified directly** (this is what keeps mobile open, so it is not taken on trust): `grep` for `wasm_bindgen|web_sys|winit|tokio|rayon|std::thread|SystemTime|Instant::now|std::fs|std::net` in `🦀️component.rs` matches only the header doc comment. `ThreadTransport` is correctly behind `#[cfg(not(target_arch = "wasm32"))]`; every `wasm_bindgen` use is confined to `📦️glue.rs` behind `#[cfg(target_arch = "wasm32")]` and passes byte buffers only.

### A3-kernel-types — accepted; three leases applied by registrar

A3 completed both parts and, like A1, then got stuck waiting on a background `cargo check --workspace`. Its own checks are green: `cargo check -p semio-framework --all-targets` and `-p semio-framework-os-kernel --all-targets` both exit 0.

**Scope delivered:** new regions `🔖️Effect` (22 renamed + 23 new variants), `🔖️Event`, `🔖️ActivationEvent`, `🔖️UiPatch`, `🔖️Budget`, `🔖️TurnResult`, `🔖️Broker` in `🎠️kernel/🦀️component.rs`; `🔖️PackageDescriptor` in `🛂️manifest/🦀️component.rs`; both hand-written TS twins updated to match. The mechanical rename covered **135 files** (132 edited directly, 3 registrar-only) via a word-boundary pass plus one manual fix the regex could not reach.

**Peer safety held.** The peer's `🔖️IoRouter` region in `🎠️kernel/🟦️component.ts` is byte-identical before and after — 240 lines (559–798), SHA-256 `ddb2ce7f…36a7`. The mid-flight amendment worked exactly as intended.

**Two judgement calls A3 made that I endorse rather than override:**
- It found an existing `kernel::CapabilityGrant` (the action/window capability model) with live consumers in `📦️glue.rs` and three puzzle editor components, so it named the broker type **`BrokerCapabilityGrant`** instead of colliding, and did not delete `CapabilityRequirement`/`Rights`/`Scope` for the same reason. **Registrar ruling: the `Broker`-prefixed name stays for now.** Collapsing the two capability models is real design work belonging to whichever packet lands the broker end-to-end, not a rename.
- It refused to silently no-op the `invokeExtension` redispatch, flagging it as a behaviour change for someone who owns the file to decide.

**Leases applied by me:**
1. `📜️script.ts` — `POLICY_HOST_EFFECT_CONSTRUCT_RE` → `/\bEffect::(\w+)\b/g`. A3's best catch: after the rename the old regex matched nothing, turning a real capability-parity lint into an always-passing no-op without any compile error.
2. `Shell/🧊️component.rs` — 22 renames plus `, ..` on both `RequestMediaFrames` patterns (1701, 2982), which would otherwise fail once `req` became mandatory. Zero `HostEffect` remain.
3. `ShellHost/🟦️component.tsx` — mechanical renames applied. For `invokeExtension` I did **not** take the silent-no-op stopgap: the branch now takes `req`, still performs the invoke, and emits an explicit `console.error` naming the missing completion path, with a comment tying it to H1-react. **H1-react now carries wiring this to a `req`-correlated completion as required work** — recorded so a loud placeholder cannot quietly become permanent.

**Open follow-ups A3 identified:** retype `Event::InstanceOpen.actor` from its `String` placeholder to `RuntimeActorId` (it was correctly barred from depending on the concurrently-created `🎭️actor` crate); and give `ContributionSet`'s placeholder `Vec<DescriptorEntry>` categories (menus, file types, panels, themes, inference/mutation services, io/composer entries) their typed shapes in packet `E1-describe` — no typed model for those exists anywhere in the codebase yet, so inventing one was correctly out of an additive-only charter.

### A1 fully confirmed; A2 structure landed; B1 unblocked

`cargo check -p semio-framework-actor --target wasm32-unknown-unknown` → **exit 0** (23m 28s — the wall time is lock contention across four concurrent sessions, not compile cost). A1's table above is now complete on every row.

**Registrar fix the executors missed.** After A3's rename I swept the tree myself for surviving `HostEffect` references. Everything left was doc-comment prose except one real one: `📺️renderer/…/🎯️targets/⚛️react/📦️index.tsx:331` still did `type HostEffect,`, which would have broken the React renderer's build. It was not in A3's registrar list and not in its 132 edited files. Fixed. Lesson recorded: after any atomic rename, the coordinator re-greps rather than trusting the executor's file count.

**A2 → B1 cross-packet bug, resolved through the tree.** B1's `cargo check -p semio-framework-plugin-host` failed to parse A2's new WIT package at `📜️effects.wit:44`: the field `stream: bool` uses `stream`, a **WIT-reserved keyword**, which breaks `wasmtime::component::bindgen!` for the entire package. B1 correctly reported it as out-of-scope rather than editing A2's files. A2 had already fixed it in-tree by the time I checked — renamed to `streaming`, with a comment crediting B1's compiler output as the discovery; the Rust-side `Effect::HttpRequest` field keeps the name `stream`. Same reserved-word class as the peer W1-D ticket's own `from`/`into` finding. B1 resumed with the blocker cleared.

**A2 structural progress** (verified on disk, packet still running): the WIT is fully split into all 12 specified interface files (`types`, `pure`, `capabilities`, `effects`, `events`, `ui`, `documents`, `jobs`, `checkpoint`, `reactor`, `describe`, `world`), and the guest SDK is decomposed as designed into `⚛️reactor/{🦀️component.rs, 🧵️executor, 📮️requests, 🩹️patches, 💼️jobs, 📸️checkpoint}` plus `🌐host/`.

**Standing process rule added to every future brief:** executors run cargo in the **foreground** within a single turn. All three finished executors independently stalled in wake/idle loops on backgrounded builds that cannot survive a subagent turn boundary — roughly 1.1M tokens spent collecting no result. Verification now runs from the coordinator session.

### Build-system saturation and a THIRD live peer ticket

`ps` shows **54 cargo processes**. Attribution (not inferred from files — read off the actual command lines):

| session | command | target dir |
|---|---|---|
| this ticket | `cargo check -p semio-framework-os-renderer-wgpu --all-targets` | our `🎯️target` |
| this ticket (A2) | `cargo check -p semio-framework-plugin --lib` | our `🎯️target` |
| peer `CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` | `cargo nextest run -p semio-framework-os --features os-host-full` | its own `🎯️target` |
| peer **`SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`** | `cargo test -p semio-framework-os-kernel --lib --features sync` (W0 baseline **and** W1 gate) | its own `🎯️target` |
| unattributed | `cargo check --workspace` | root `target/` |

Two consequences:

1. **There are three live tickets in this tree, not two.** `SHARED-PRESENCE-…` was not visible in the earlier churn check because it had not written to our hot files. Peers use the shared root `target/`; we use our per-ticket dir, so the separation holds, but the cargo package-cache lock is global — hence a 23-minute wasm check that compiles in seconds.

2. **A3's edits land inside a crate that peer is actively gating on.** `SHARED-PRESENCE-…` is running `cargo test -p semio-framework-os-kernel --lib` as its own W0 baseline and W1 gate, and A3 rewrote the `Effect` region of exactly that crate. A3 verified `cargo check -p semio-framework-os-kernel --all-targets` exits 0, so it compiles — but **compilation is not the same as its test suite still passing**, and a regression here would surface in that peer's gate and be misattributed to them. Added to the G1 checklist: run `cargo test -p semio-framework-os-kernel --lib --features sync` from this session and compare against that peer's own recorded baseline before declaring A3 finished. Not duplicating the run right now — the peer is executing that exact command and the lock is already saturated.

**No new load will be added until A2 and B1 return.**

### A2-abi-sdk — honest partial; W1 does not close here

A2 reported without fabricating anything, which is exactly the behaviour the packet contract asks for: **zero acceptance commands observed to complete** — every check sat blocked on the saturated build lock.

**Done:** the WIT split — all 12 files, `pure` confirmed as the only import interface, and every mandated deletion (`plugin-world`, `extension-world`, `contributor`, `host`, `exchange`, `manifest`, `instantiate-app`, `clear-instance-guard`, `activate`/`deactivate`/`invoke`) verified absent from the WIT tree. `LocalExecutor` and `RequestRegistry` + async `host::*` with unit tests. Jobs, including the absorbed peer io mechanism. Deletions confirmed: `INSTANCE_GUARD`, `host_port`, `component::host_*`, `install_io_fallback_dispatcher`, `extension_component`.

**Partial / not started — carried into a follow-up packet `A2b`:**
- the `poll` turn loop is written with full variant coverage but **has never been compiled**;
- checkpoint/restore and UI patch diffing are full-body only — no `view_state`/`ephemeral` capture, no structural diff;
- `describe()` exists but the builder methods that populate it do not;
- `Emit.tasks` not started — so the async-follow-up path that replaces `InvokeExtension{response_action}` has no guest-side counterpart yet;
- `plugin_exchange` deliberately **kept** (reused internally by the turn loop) rather than deleted — reasonable, but it means `📌️important.md`'s "must not exist" list is not yet satisfied and the exit check must re-examine it;
- **the backbone channel was deleted with no replacement.** A2 flagged this rather than silently dropping it. This is a real gap: `EffectBackbone` (per-instance, replacing the process-global `set_host_backbone_channel`) is specified in `📓️design-abi.md` §4 but not implemented, so guest↔store sync currently has no path. **Registrar decision needed before W2** — it is on the critical path for both renderer packets.

**Assessment.** W1 is **not** closed. A1 and A3 are genuinely complete and verified. A2 delivered its contract surface (the WIT, which unblocked B1) but only part of the SDK, and none of it is compiler-verified. B1 is mid-implementation. G1 therefore cannot be declared; the gate is now: `cargo check -p semio-framework-plugin --lib` green, then `-p semio-framework-plugin-host`, then `cargo test -p semio-framework-os-kernel --lib --features sync` diffed against the `SHARED-PRESENCE-…` peer baseline, then a `🗒️note` turn.

**Throughput correction applied.** The blocking was partly self-inflicted: our own packets share one `🎯️target`, so parallel builds serialize against each other on top of the global package-cache lock. All executor builds are stopped; verification now runs one at a time from this session. The rule is recorded in `📌️important.md` for W2 onward — parallel editing is free, parallel building is not.

### G1 gate run — two real bugs found by the coordinator, one peer breakage

Ran the gate myself once all executors were stopped and the lock freed.

**1. `cargo check -p semio-framework-plugin --lib` → exit 0** (5 benign warnings). But A2 was right that this proves little: the bare build skips the `cfg`-gated component module, so the turn loop is never compiled.

**2. `--target wasm32-wasip2 --features component-guest` → FAILED.** ~30 errors, all cascading from one root cause the grep-for-`error` output completely hid:

```
error: failed to resolve directory while parsing WIT for path […/📜️wit]
       --> 📜️effects.wit:241:5
```

`result` is a **WIT-reserved keyword**, and `respond-effect.result` / `completed-event.result` / `job-completed-event.result` used it as a field name. `wit_bindgen::generate!` therefore expanded to nothing, which is why every downstream error read as "cannot find `exports` in `component`" — a misleading cascade with no mention of WIT.

This is the **third** instance of the same bug class in this tree today: A2 hit `stream`, the peer W1-D ticket hit `from`/`into`, and now `result`. Fixed by renaming all three fields to `outcome` (aligning with A3's Rust-side `RequestOutcome`) plus the three Rust conversion sites in `⚛️reactor/🦀️component.rs`. Swept the whole WIT tree for the full reserved-word set afterwards — clean.

**Rule for W2 onward:** any packet touching WIT greps its files for reserved words before claiming completion, and never reports a WIT-consuming build as "blocked" without reading the *first* error — the cascade is actively misleading.

**3. Peer breakage now blocking the wasm build (not ours).** With the WIT fixed, the build fails earlier on an unrelated crate:
`semio-framework-ui-styling/📦️glue.rs:5` → `couldn't read 🤖️generated.rs: No such file or directory`.
`git status` shows a peer session mid-restructure of `🖱️ui/🎨️styling`: `🔣️tokens.json`, `🎨️theme/`, `🎨️tailwind/`, `🟦️vite-elements-assets.ts` all deleted from `📦️packages/🦀️rust/` and re-added one level up, with `📜️script.ts` modified and the generated file not yet re-emitted at the new location. **Not our file, not our regression — left untouched.** It sits in the plugin crate's dependency graph, so the wasip2 gate cannot complete until that peer finishes. Falling back to `cargo check -p semio-framework-plugin-host`, which parses the same WIT through `bindgen!` on a different dependency path, to confirm the fix independently.

**Both verification paths blocked.** `cargo check -p semio-framework-plugin-host --all-targets` fails on the same missing `semio-framework-ui-styling/🤖️generated.rs`. Every crate that parses our WIT reaches `ui-styling` through `ui_wgpu`, so the reserved-word fix is **applied but not yet compiler-verified**. Deliberately NOT worked around: regenerating that file would write it at the old path the peer is actively moving away from, which would interfere with their restructure. `wasm-tools` is not installed, so no standalone WIT validation is available without adding a toolchain install under an already-saturated build system.

**Honest position at this point in W1:**

| packet | state | verified by |
|---|---|---|
| `A1-actor` | **complete** | coordinator: check (native + wasm32-unknown-unknown) + 52/52 tests, purity grep |
| `A3-kernel-types` | **complete** | coordinator: two named checks exit 0; peer region byte-identical |
| `A2-abi-sdk` | **partial** | `--lib` exit 0; component-guest path **unverified** (blocked) |
| `B1-host-native` | **in progress** | resumed after WIT unblock; not yet verified |
| `A4-channel` | queued | — |

G1 is **not** met and will not be claimed until the component-guest build and `cargo test -p semio-framework-os-kernel --lib --features sync` (diffed against the `SHARED-PRESENCE-…` peer's baseline) both run clean.

### Coordinator debugging pass — four more real bugs fixed, WIT now parses

"Finish end to end" instruction. Took the component-guest gate apart myself rather than round-tripping to agents.

**The WIT source of truth moved mid-session.** A2 (or the repo's single-file consolidation tooling) collapsed the 12 split interface files into ONE `🔌️plugin/🧬️schema/📜️component.wit` (821 lines) and deleted `📦️packages/🦀️rust/📜️wit/`. My earlier `result`→`outcome` fix had been applied to the old directory — I re-verified it survived the consolidation (3 occurrences present) rather than assuming.

**Fourth reserved-keyword bug**, found by B1 and confirmed by me: `from: message-endpoint` in `request-event`. Renamed to `origin` plus its Rust conversion site. A full reserved-word sweep of the consolidated file is now **clean**. That is four instances of this one bug class in a single day (`stream`, `result`, `from`, and the peer ticket's `from`/`into`) — the rule in `📌️important.md` stands.

**Then the WIT parsed, and the real bugs surfaced** — ordinary Rust defects in code that had never been compiled:
1. Wrong generated-binding module path. The crate root mounts the owner file as `crate::component`, and the WIT macro sits in `pub mod component` inside it, so everything is at `crate::component::component::…`. Fixed 25 sites in the reactor plus the three `pure` calls in `🌐host`.
2. `let pack = |value: &impl serde::Serialize|` — `impl Trait` is illegal in a closure parameter. Replaced with a local generic `fn`.
3. Three `instance.0 as u32` casts on a `PluginInstanceId(pub String)` — non-primitive cast. Now `parse::<u32>().unwrap_or(0)`.

**Remaining, dispatched as `A2b-bridge-green`:** ~80 references alias the exported `reactor` module for types that actually live in the sibling `effects`/`events`/`ui` interfaces. rustc offers no import suggestion for them, so the packet is instructed to discover the real path empirically rather than guess, then reconcile the genuine shape mismatches between the WIT and A3's kernel types — with the **kernel Rust types as SSOT**.

This is the honest state: the ABI contract is sound and parses, the guest bridge does not yet compile, and the fix is mechanical but large.

### ✅ A2b-bridge-green — VERIFIED. The reactor ABI compiles for wasm.

Coordinator-run, not taken on the executor's word:

```
cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
    Finished `dev` profile [unoptimized] target(s) in 3.47s     ← zero errors
cargo check -p semio-framework-plugin --lib
    Finished `dev` profile [unoptimized] target(s) in 3.54s     ← zero errors
```

**This is the milestone W1 existed for.** The effect-only reactor ABI — `poll(events, budget) -> TurnResult`, jobs, checkpoint, describe, `pure`-only imports — now compiles as a real `wasm32-wasip2` component. Every sync host import is gone from the guest contract.

A2b's module-path answer, verified empirically rather than guessed: `world actor` exports only `reactor`/`jobs`/`checkpoint`/`describe`, so wit-bindgen aliases under `…::exports::semio::framework::<interface>` **only** the types named directly in those interfaces' own signatures. Nested payload types declared in `effects`/`events`/`ui`/`types` sit one level shallower at `…::semio::framework::<interface>`, with no `exports::` prefix. ~90 call sites repointed behind four aliases.

Two genuine WIT↔kernel mismatches were resolved on the WIT side, correctly honouring the kernel as SSOT: `request-file-open-effect` gained the missing `import-action: string`, and `request-media-frames-effect.payload` narrowed from `option<pack>` to `option<string>`.

A2b also hit a live fleet-wide breakage in `🛂️manifest/🦀️component.rs` (a serde tag/field-name collision from a peer), correctly refused to touch it, flagged it, waited it out, and resumed — exactly the peer-coexistence behaviour the packet contract asks for.

### Contention wall

`cargo test -p semio-framework-os-kernel --lib --features sync` — the check that would diff our kernel edits against the `SHARED-PRESENCE-…` peer's recorded **996/996** baseline — is queued behind that peer's own instance of the identical command, which has been running since 21:34 (2 h 25 m elapsed, 20 rustc processes). Ours is blocked on the build-directory lock, not on anything in our code.

**Not waiting on it further this session.** It is the one outstanding G1 item; everything it gates is a regression check, not new work, and the peer's own run will publish the same number.

### W1 scoreboard (coordinator-verified only)

| packet | state | evidence |
|---|---|---|
| `A1-actor` | ✅ complete | native + wasm32-unknown-unknown checks, 52/52 tests, purity grep |
| `A3-kernel-types` | ✅ complete | 2 crate checks exit 0; 135-file rename; peer region byte-identical |
| `A2-abi-sdk` + `A2b` | ✅ **contract green** | component-guest wasip2 build `Finished`, 0 errors |
| `B1-host-native` | ⏳ partial | trait + mock + engine primitives + real `impl GuestRuntime` written; `ShardLoop`, router relay and the legacy deletions outstanding |
| `A4-channel` | queued | channel v12 |
| kernel regression diff | ⏳ blocked | queued behind peer's 2.5 h run |

## 2026-08-18 — W2 dispatched, per-packet target dirs

Standing instruction is to finish end to end, so W2 goes out now that the guest contract is green.

**Build-contention fix applied.** Every W2 packet gets its **own** `CARGO_TARGET_DIR` (`🎯️target-b1b`, `-e1`, `-a4`, `-f1`) instead of sharing one. Parallel editing was always free; parallel *building* was serializing us against ourselves on a single build-directory lock, on top of the global package-cache lock the three peer tickets contend for. Disk is 304 GiB free, so the duplicate-cache cost is affordable — this is exactly what prior workforce tickets did with `🎯️target-w3-cad` / `🎯️target-verify`.

Four packets dispatched, file-disjoint:

| packet | scope | target dir |
|---|---|---|
| `B1b-host-complete` | `ShardLoop`, post-turn router relay, delete `WasmPluginRuntime`/`ExtensionRuntime`/both `ProgramSupervisorState`/`PLUGIN_FUEL_BUDGET`, `🏃️run` onto `GuestRuntime` | `🎯️target-b1b` |
| `E1-describe` | emitter crate, `descriptor_is_fresh()` macro test, registry reads descriptors, typed `ContributionSet` shapes | `🎯️target-e1` |
| `A4-channel` | channel v12: drop `Hello`/`RefreshUi`/`UiSection`/`Effects`/`Events`/`Welcome`, add revisioned `UiPatch` | `🎯️target-a4` |
| `F1-scale-fixture` | 50×50 fixture crate + seeded deterministic generator, 2550 records | `🎯️target-f1` |

Two instructions carried into every brief from the W1 post-mortem: **foreground builds only**, and the specific coverage rule for B1b — rewrite the ~510-line real-wasm `IoRouter` test block against `GuestRuntime` rather than deleting it, since it is the only real-wasm coverage of cross-plugin routing.

`A4-channel` will make the tree red between its landing and the renderer packets; that is planned, confined to this gate, and its brief requires it to enumerate every out-of-scope consumer with the exact edit needed so the renderer packets can pick them up.

### ✅ A4-channel landed; 2 of 7 leases applied by registrar

Channel v12 is in: `CHANNEL_VERSION` 11→12, `AppCommand::{Hello,Bye,AttachBackbone,DetachBackbone,RefreshUi}` and `SectionProbe` removed, `AppFrame::{Welcome,UiSection,Effects,Events}` removed, `AppFrame::{UiPatch,UiSnapshotEnd}` added mirroring `kernel::UiPatch` field-for-field and reusing `kernel::PatchOp` rather than redefining it. Tags renumbered contiguously with no legacy gaps — correct for a greenfield repo that forbids compatibility layers. TS twin mirrored, including retiring `hello()`/`refreshUi()`/`attachBackbone()`/`detachBackbone()`/`drain()` from `AppChannelClient`.

A4 made one judgment call worth endorsing: it edited the 6 cross-language JSON fixtures under `💻️os/🧫️fixtures/📡️channel/**`, one level outside its literal owned prefix, because they are this codec's own byte-parity pins and have no other owner — and it documented the call rather than doing it silently. Its 4 cross-language vector tests confirm Rust/TS byte-parity on the new wire.

**Leases applied by me (same crate, mechanical):**
1. `📡️spr/🦀️component.rs` — dropped the dead `SectionProbe` from the barrel re-export.
2. `📡️spr/🧪️testkit/🦀️component.rs` — 8 test-only uses of `AppCommand::Hello`/`Bye` and `AppFrame::Welcome`, which were arbitrary "any two variants" round-trip samples with no semantic dependency on those variants. Swapped for `ConfigCommand`/`ReadConflicts`/`Done`. **Coverage preserved, not deleted** — the round-trip laws still run against real surviving variants.

**Leases routed, not applied (5).** These need real rewrites, not renames, so they go to the packets that own the surrounding redesign:
- `🔌️plugin/🦀️component.rs` + `⚛️reactor/🦀️component.rs` — the old `plugin_exchange` dispatcher's `Hello`/`RefreshUi`/`UiSection`/`AttachBackbone`/`Bye`/`Effects` handling, and `route_app_frame`'s three now-dead match arms. A4's diagnosis is right that the `UiSection` arm should become a real `AppFrame::UiPatch` → `kernel::UiPatch` passthrough (fields match field-for-field; `ops` needs `decode_wire_value::<Vec<PatchOp>>`).
- `🔌️plugin/🖥️host/🦀️component.rs` — the `Hello` handshake at 4 sites; instance bring-up is now `instance-open` at the `GuestRuntime` level. **Routed to `B1b`, which is live in that file.**
- `ProgramBridge/🧊️component.rs` and the React `📦️index.tsx`/`🧪️index.test.ts` — routed to the renderer packets, which is exactly where `📓️design-abi.md` §2 says this work belongs.

### ✅ Kernel regression check CLEARED — the last open G1 item

```
cargo check -p semio-framework-os-kernel --all-targets   → Finished in 13.99s, 0 errors
cargo test  -p semio-framework-os-kernel --lib           → 1003 passed; 0 failed
```

Peer `SHARED-PRESENCE-…`'s recorded pre-work baseline was **996/996**. We are at **1003/0** — **no regressions, +7 tests** (A4's new channel-v12 codec tests, including the 4 cross-language Rust/TS byte-parity vectors).

This was the risk I flagged when A3 rewrote a crate that a peer ticket was actively gating on: compiling is not the same as its suite still passing. It passes, and it grew. Earlier the check was queued behind that peer's 2.5-hour run; giving A4 its own `🎯️target-a4` dir dropped it to seconds, which validates the per-packet-target-dir decision on its own.

**G1 is now met.** Every W1 contract is landed and verified:

| | evidence |
|---|---|
| actor kernel | native + wasm32-unknown-unknown checks, 52/52 tests, purity grep |
| kernel/manifest contracts | 2 crate checks clean, 135-file rename, peer region byte-identical |
| reactor ABI (guest) | `wasm32-wasip2 --features component-guest` → `Finished`, 0 errors |
| channel v12 | os-kernel green, 1003/0 tests, Rust/TS byte-parity vectors pass |

### ✅ F1-scale-fixture landed — the 50×50 proof asset exists

The fixture that makes this ticket's central claim measurable rather than asserted.

Coordinator-verified after applying its leases:
```
cargo check -p semio-framework-os-scale-fixture --all-targets            → Finished in 3m 14s, 0 errors
cargo check -p … --target wasm32-wasip2 --features component-guest      → Finished, 0 errors  (F1's run)
cargo test  -p … --lib                                                  → 12 passed; 0 failed
cargo metadata --no-deps                                                → workspace parses
```
Determinism proven properly, not claimed: same seed → **byte-identical** `registry.json` (`8acb5ef9…`) and `catalog.json` (`f7623e7c…`) across two runs; a **different seed produces different output**, which is what actually proves the seed drives generation rather than the result being constant. Record count exactly **2550** = 50 plugins + 2500 extensions.

F1 also independently verified the taxonomy claim I had asserted in the design — `pluginAreas` is `["✏️s/🔌️plugins"]` and `🧫️fixtures` is a legal owner-root data dir — so the fixture is invisible to the production registry, dev catalog and `launch.json`. It confirmed `role = "testkit"` was declared but unused. Good instinct to check rather than trust the brief.

**Four leases applied by me:**
1. Root `Cargo.toml` — member + `[workspace.dependencies]` alias, **and** removed the crate's temporary `[workspace]`/`[workspace.lints]` opt-out tables plus its crate-local `Cargo.lock`, which F1 correctly warned would conflict with real membership. Verified the crate builds as a member afterwards.
2. Root `📜️script.ts` — `generate scale-fixture` branch + a `scale-fixture check` router entry. Verified: `scale-fixture` now appears in the usage line.
3. Root `📋️project.json` — `scale-fixture-check` target; JSON re-validated.
4. `.vscode/🧩️launch.seed.jsonc` — `🛠️dev🦀️os-plugins🧫️scale-fixture` (3_dev, 386.7) and `📦️verify🧫️scale-fixture🚦check` (4_build, **209.6** — F1 found the design's suggested 209.3 already taken by `📦️new🧩️taxonomy🗿️artifact` and picked the next free slot, correctly checking rather than colliding). Seed re-validated as JSON; one name I had written as escape sequences was normalized to literal emoji for consistency with the rest of the file.

**Known gap, honestly recorded:** the fixture's `describe()` returns an empty placeholder — a real packed `PackageDescriptor` is `E1-describe`'s job, and nothing in F1's acceptance or the generated registry depends on it. No wasmtime instantiation was exercised (that is `V1-bench`'s job; nobody has yet instantiated this ABI under wasmtime).

**Third peer breakage found and routed, not patched:** `.storybook/scopes.ts:146` still imports `🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts`, which the ui-styling restructure moved up a level. That session already fixed its own `glue.rs` but missed this one. Spawned as a separate task rather than edited, since it belongs to their in-flight move. Two similar dangling-import bugs F1 hit were likewise routed (one already fixed by the spawned session mid-turn).

### ✅ A5-sdk-channel — guest SDK consumes channel v12

Coordinator-verified:
```
cargo check -p semio-framework-plugin --lib                                    → Finished, 0 errors
cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest → Finished, 0 errors
```

`plugin_exchange` now returns `PluginExchangeOutput { frames, effects, events }` instead of `Vec<Vec<u8>>` — the right call, since `AppFrame::Effects`/`Events` no longer exist as wire variants it could construct; effects and events are carried as raw wire bytes and decoded in `poll`. The dead `Hello`/`RefreshUi`/`SECTION_KIND_*`/`AttachBackbone`/`DetachBackbone`/`Bye` arms are gone, and `route_app_frame`'s `UiSection` arm became the real `AppFrame::UiPatch → kernel::UiPatch` passthrough that A4 predicted, decoding `ops` through `decode_wire_value` → `from_dsl_value::<Vec<PatchOp>>`.

Two decisions worth recording:
- A5 **checked the caller count with `rg` before changing a public signature** and found exactly one. I re-verified independently: `plugin_exchange` appears only in the plugin crate's own two files (plus a name mention in the OS TS twin). Packet B1b, live in `🔌️plugin/🖥️host/**`, is unaffected — no cross-packet break.
- It ported actor-id recording forward from the deleted `Hello` handshake into `poll`'s `Event::InstanceOpen`, since `InstanceOpen.actor` carries the same data — but deliberately did **not** port `Hello`'s optional inline config-load, because `AppCommand::LoadConfig` already covers it and the brief said remove rather than adapt. Correct reading of the no-compatibility-layers rule.

**Both halves of the ABI now compile against channel v12.** The guest contract and the wire are consistent.

### ✅ E1-describe — descriptor pipeline in; and a real flaw in MY design that E1 caught

**Delivered and self-verified by E1:** the wasmtime emitter crate `semio-framework-plugin-describe` (compiles clean, 5/5 tests, and proven against a real built plugin wasm where it fails *correctly* with a wasi-import error because no plugin has migrated to the new ABI yet — that is the honest result, not a fudge); the `descriptor_is_fresh()` test inside `plugin_exports!`/`extension_exports!`, proven with a throwaway fixture crate; registry `parsePluginCargo` reading `🔣️descriptor.json` with a transitional Cargo-metadata fallback (0/59 plugins migrated so far) plus a `check` gate on id/extends/extension-point/hash consistency, `plugin-registry:check` exiting 0; and typed `ContributionSet` shapes grounded in existing types — deliberately leaving `menus`/`themes` untyped because nothing in the codebase declares them yet. Correct restraint: inventing shapes for contributions no plugin makes would be fabrication.

It also found and fixed a genuine pre-existing bug: `📇️registry/📜️script.ts`'s own import path had 8 `../` instead of 6, so `plugin-registry:check`/`:generate` **could not run at all** before this packet.

#### Registrar decision: descriptors move out of `🤖️generated/`

E1's most valuable finding is a flaw in my own design doc. `📓️design-abi.md` §3 specifies the descriptor as *checked-in* at `<crate>/🤖️generated/🛂️descriptor.semio`, but I verified: `.gitignore` lines 87–88 ignore `**/🤖️generated/` and `**/🤖️generated.*`, and `git ls-files` shows **zero** tracked files under any `🤖️generated`. A descriptor there could never survive a commit — the freshness test would have no baseline on a clean checkout and the registry would have nothing to read. The design was unimplementable as written.

Ruling: **descriptors live at the plugin/extension owner root** as `🛂️descriptor.semio` + `🔣️descriptor.json`, siblings of the already-tracked `🛂️manifest.json`. This is not an ad-hoc workaround — the taxonomy's own `_languageNeutralityComment` states that language-neutral assets *including generated output* belong "at the owner root as siblings of 📦️packages — never inside 📦️packages/<lang>/". So the correct location was already specified; my design doc contradicted it.

Applied as registrar:
- root `Cargo.toml` — member + `[workspace.dependencies]` alias for the describe crate;
- `🔣️taxonomy.json` — `rootDataDirNames` += `📇️describe`, `rootDataFileNames` += `🛂️descriptor.semio`, `🔣️descriptor.json`.

`📓️design-abi.md` §3's stated path is now superseded by this entry.

**Not started, carried forward:** the `PluginBuilder`/`ExtensionBundle` descriptor-populating methods (`.activation`, `.extension_point`, `.requests`, `.quota`, `.execution`). E1 judged it unsafe to rush them in its remaining budget and said so rather than half-landing them — the right call, and they are a prerequisite for W3, since a plugin cannot declare an activation event without them.

### ✅ B1b-host-complete — legacy runtime deleted, ShardLoop in

Coordinator-verified after two registrar fixes:
```
cargo check -p semio-framework-plugin-host --all-targets → Finished, 0 errors
cargo test  -p semio-framework-plugin-host --lib         → 67 passed; 0 failed
cargo check -p semio-framework-os-run --all-targets      → Finished, 0 errors
```

**The legacy runtime is gone.** `WasmPluginRuntime`, `ExtensionRuntime`, `PLUGIN_FUEL_BUDGET` and the orphaned old `HostState` are deleted — I verified every surviving mention of those names in the tree is a doc comment, not code. `ShardLoop` exists at `🖥️host/🧵️shard/` driving a real `ShardTransport`, and B1b found that `GuestRuntime` was missing `start_job` despite `jobs.wit` declaring three functions — a genuine gap in B1's own trait, caught by implementing against it.

B1b's best find: B1's `bindgen!` had `additional_derives: [Clone, Debug]`, but wasmtime-wit-bindgen always hand-writes `Debug` for WIT records — requesting it again caused **~91 of the original 112 errors** as `E0119` conflicts. One line.

**Coverage was rewritten, not dropped**, as instructed: pure tests kept verbatim, the real-wasm compose test replaced with honest compile/instantiate-rejects coverage plus new `MockGuestRuntime`-backed cross-plugin routing tests. Two blocks were deleted because they tested subsystems (`ExtensionRuntime`, the old `exchange` ABI) that have no `world actor` equivalent — correct, and stated rather than hidden.

**Two fixes I made as registrar:**
1. `🎚️config/🧬️schema/🧬️mutations/🦀️component.rs` — B1b's filed blocker. `DefaultApp` is declared at the schema level, so the test module needed one more `super` than the file-level import. One line; plugin-host tests could not run at all before it.
2. B1b's own new gap-documenting test was failing. It asserted the "resolution succeeds, dispatch not yet wired" path but built the **inverse** `IoKey` orientation to the one `register_plugin` actually derives from a `(writes, reads)` pair — the Export route is keyed on the READ dialect with the WRITE dialect as its format. So it failed on resolution and never reached the gap it existed to pin down. **I fixed the test, not the router**: that route-derivation is the peer ticket's algorithm, which `📌️important.md` requires be preserved byte-for-byte. The orientation looks semantically inverted to me (exporting cad-as-step keys on step, not cad) and that is worth the peer's attention, but it is theirs to judge, not mine to silently rewrite.

**Honest gaps:** `PluginHost.supervisor` → `KernelMetrics` read view not attempted (no live `Kernel` is instantiated anywhere yet); `IoRouter::compose` for the OLD IoKey mechanism resolves ownership then reports "not yet wired" rather than guessing an undocumented wire format — no job kind exists for it in the WIT, so refusing to invent one is right.

### ⏳ H3-wgpu-native — substantial partial; blocked by a peer's presence refactor

**Delivered:** a real `KernelClient` + dedicated kernel thread in `📦️glue.rs` driving `semio_framework_actor::Kernel` + `ShardLoop` + `WasmtimeRuntime` over a real `ThreadTransport`, replacing the deleted `Arc<WasmPluginRuntime>` backend. `ProgramBridge` rewritten onto channel v12 — `UiPatch`/`PatchOp` applied to a retained per-surface tree with `base_revision`-mismatch desync handling, effects read from `TurnResult.effects`. `load_wasm_plugins` no longer eagerly instantiates: it scans for a build-time `🔣️descriptor.json` and compiles only on `create_app`. 12 surgical hunks in the shared 12.7k-line `Shell/🧊️component.rs`, each listed by line range.

**H3's block_on claim, checked:** it identified the 3 plugin-blocking sites out of 19 occurrences and converted 1 (`spawn_app_task`) to a genuinely non-blocking task pool drained from `about_to_wait`. The other 2 still park, and H3 explained why rather than papering over it — they run inside an already-active `Rc<RefCell<AppRuntime>>` borrow, so removing the park needs a `ShellState` ownership refactor beyond this packet. The wasm execution itself moved off-thread regardless. That is a real constraint honestly reported, not an excuse.

**Its "pre-existing, not mine" claim — verified independently, and it holds.** 5 errors remain in `Shell/🧊️component.rs`: unresolved `store_sync::PresencePoint`/`PresenceViewport`, a non-exhaustive `ArtifactEvent::Session` match, `PresencePeer` missing `cursor`/`viewport`, and a `DockStackTab`-vs-`String` mismatch. I confirmed `PresencePoint`/`PresenceViewport` **no longer exist anywhere** — the peer `SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION` ticket replaced them with `PresenceUi`/`PresenceWindowView`/`PresencePeer` in `📡️spr/📡️wire/🦀️component.rs` (the same rename A3 recorded in its peer-coexistence section) and has not yet updated `Shell`. Last commit on that file is theirs, 01:34 today, with 33 further uncommitted insertions.

So the wgpu renderer cannot compile until that peer finishes. **Not our regression and not ours to fix** — touching their half-done presence rename would be exactly the "chasing a moving target" failure both tickets' rules forbid. H3 accepted as a verified partial; its own crate's warnings were cleaned to zero.

**Honest follow-ups H3 named:** no DRR `tick()`/multi-shard scheduling yet (single shard, direct dispatch), and `attach_backbone`/`detach_backbone`/`ephemeral_snapshot`/`context_menu`/`window_engagements`/`window_measures` are explicit error/empty stubs tied to the still-open backbone-replacement gap — it refused to invent a wire format for a mechanism that was retired without a replacement.

### ✅ E2-builder-descriptor — a real descriptor exists for a real plugin

The W3 prerequisite is met: plugins can now declare what the descriptor carries.

**Verified by me:** `🗒️note` has a committed `🛂️descriptor.semio` (64 KB) + `🔣️descriptor.json` (269 KB) **at its owner root**, confirmed NOT gitignored, containing real data — `role: plugin`, `activationEvents: [{onArtifactKind: {kind: "2d.note"}}]`, `execution: isolated`, `capabilityRequests: [{id: "documents.write", scope: "plugin", reason: "persist note edits to the open document"}]`. That is the end-to-end proof the packet was asked for, not a placeholder.

E2 resolved the design fork per-role and justified it: `ExtensionManifest` got the fields directly; plugins use a `PluginDescriptorExtras` side-channel populated atomically in `try_build()`, avoiding a cascade through ~10 `PluginManifest` construction sites in files it did not own. Reasonable — and it kept a single build-time source of truth rather than a drifting parallel registry.

**It found a real gap in E1's work**, which only surfaced because E2 is the first packet ever to commit a real descriptor: `descriptor_is_fresh()` still pointed at the superseded `🤖️generated` location, and its hash fields could never match a committed descriptor. My registrar ruling propagated correctly *because* someone exercised it end to end.

#### Registrar fixes and one mistake of mine, corrected

`ActionArgDef.control` no longer exists — a peer redesign turned it into a `control()` method derived from `ArgSchema`. Two construction sites blocked the whole plugin test binary.

My first fix was **wrong**: I replaced `ActionArgDef { control: Select { options: vec![] }, ..text(..) }` with plain `text(..)`, reasoning they were equivalent. They are not — an empty Select was *deliberately invalid input*, constructed by two tests to prove the validator rejects it. My change made the tests pass nothing and they failed. Caught it by running the suite rather than trusting the compile.

Reading `control()` settled it: it yields `ActionArgControl::Select` **only** when `options` is non-empty. So "a Select with no options" is now **structurally unrepresentable**, the `validate_arg_defs` assertion for it can never fire, and both tests assert an impossible rejection. I removed the two tests with a `🪦️` tombstone recording exactly why, and kept the defensive asserts as a tripwire in case `control()` ever widens. This is not dropped coverage — making an illegal state unconstructible is strictly stronger than validating it.

#### Honest test position

`cargo test -p semio-framework-plugin --lib` → **239 passed, 5 failed.** Attribution, checked rather than assumed:
- **4 fail deterministically in isolation** — exactly matching the pre-existing baseline A2 recorded ("230 run / 226 pass / 4 fail") before any of our work. Not ours.
- **1** (`a_child_survives_…_through_the_channel_frames`) **passes alone but fails in the suite** — shared-global-state interference in the SDK's process-global registries, still failing with `--test-threads=1`. I cannot cleanly attribute this one: the tree has moved too far to reproduce a true pre-change baseline. Recorded as unattributed rather than claimed as pre-existing.

Also of note: my E2 brief named `cargo check -p semio-s-plugin-note --target wasm32-wasip2 --features component-guest` as acceptance, but that crate has no `component-guest` feature — my error in the brief, not the packet's.

## 2026-08-18 morning — renderer blocker cleared

### ✅ wgpu renderer compiles

`cargo check -p semio-framework-os-renderer-wgpu --lib` → **Finished in 26s, 0 errors.**

The 5 errors H3 correctly attributed to the peer's presence/dock refactor were still there this morning. **Liveness re-checked before touching anything** (the rule is "don't chase a moving target", not "never touch"): `Shell` mtime 02:51, peer ticket's newest file 22:14 yesterday, zero `.rs` edits repo-wide in the last 60 minutes, one cargo process on the box. That session has been stopped for ~12 hours and the auto-commit bot has since committed the broken state — so the blockage was permanent, not transient, and it blocked every session touching the renderer.

Fixed by adapting the call sites to the peer's **own new types**, inventing nothing:
- `use store_sync::{PresencePeer, PresencePoint, PresenceViewport}` → `use store_sync::PresencePeer` (the latter two no longer exist anywhere).
- Dropped the `cursor`/`viewport` initializers — those fields are gone from `PresencePeer`; that state moved into the presence packet.
- Added the four new required fields (`color`, `surface`, `views`, `ui`) as `None`/empty **with a comment explaining why**: the wgpu shell does not publish colour/surface/view/ui presence, the sync actor assigns `color` from the hub's `Session` frame, and the rich payloads belong to the presence packet. Left empty rather than fabricated so nothing invents presence state the shell never observes.
- `DockNode::Stack { windows }` now holds `DockStackTab`, not `String` — `tab.window_id.clone()`.
- Added an explicit `ArtifactEvent::Session { .. } => {}` arm (the sync actor already stamps session identity onto outbound heartbeats) rather than a catch-all, so a future variant still fails the build instead of being silently swallowed.

A backup of the pre-edit file is at `/tmp/shell.bak` should the peer want to diff.

### ⛔️ W3 blocked: a LIVE peer half-landed a presence refactor into the guest SDK

`M0-stdio` cannot pass acceptance because `semio-framework-plugin` itself stopped compiling. Commit **`abd29c08d0`, 10:38 today** (twenty minutes before this stretch began) changed `🔌️plugin/🦀️component.rs`:

- `ArtifactApp` gained `adopt_presence(own_color, peers, now_ms)`, but `VcsArtifactApp` does not implement it;
- `ephemeral_snapshot`'s trait return type became `EphemeralSnapshot` while the impl still returns `(Vec<u8>, u64, u64)`;
- `EphemeralSnapshot` is defined (line 9509) but out of scope at one construction site.

That crate was **verified green by me earlier today** (A5 and E2 acceptance), so this is new breakage, landed by an active session, and it is theirs to finish. **I stopped editing that file.** Completing another ticket's presence feature — deciding the `EphemeralSnapshot` shape and what `adopt_presence` does to the peer maps — is their design call, not mine.

**Distinguish this from the renderer unblock earlier today**, which looked superficially similar but was the opposite case. There, the peer had been stopped ~12 hours, the tree was quiet, and the auto-commit bot had already committed the broken state: permanent breakage, safe to adapt to their own new types. Here the peer committed twenty minutes ago and is mid-edit. Same evidence test (`git log --date=iso`, mtimes, process check), opposite answer. The rule is "don't chase a moving target" — the first target had stopped; this one has not.

**Two edits I did land in that file before stopping**, both required by our own channel v12 and both written against the peer's *documented* contract rather than guessed semantics:
1. `AppFrame::Ephemeral` gained the `interaction` field (channel v12) — passed `Vec::new()` with a comment recording that the guest's interaction slice travels via `PeerPresence`, not this drain path.
2. An `AppCommand::Presence { seq, own_color, peers }` arm that decodes each wire peer and calls their `adopt_presence`. The trait's own doc names that command as "the ONLY plugin ingress for peers" and says the call is the roster's single source of truth, not a diff — so the arm passes the whole roster and skips individually-malformed peers rather than failing the batch.

Those two will compile the moment the peer finishes their impl.

**M0's own work looks sound** from inspection: `OnArtifactKind` activation events per artifact kind (binary/txt/json/xml/csv/md/…), matching the shape `🗒️note` proved. It could not emit a descriptor because the SDK it links against is red.

### ✅ Peer finished — SDK green again; M0 accepted as partial

`cargo check -p semio-framework-plugin --lib` → **Finished in 4.16s**. The peer completed their `adopt_presence`/`EphemeralSnapshot` implementation, and my two v12 edits (the `interaction` field and the `AppCommand::Presence` arm written against their documented contract) compiled against it unchanged. Waiting rather than finishing their feature was the right call — it cost about an hour and zero rework.

`cargo check -p semio-s-plugin-stdio --target wasm32-wasip2` → **Finished in 42.74s.**

**M0-stdio — accepted as a verified partial.** What it delivered:
- **36 activation events**, one per genuine artifact kind, each read live from that format's own `artifact_kind().id` function rather than hardcoded — exactly the right instinct, and the standard now written into M1's brief.
- `ExecutionMode::Isolated`, one honest `documents.write` capability request covering ~90 editor surfaces, and **no quotas** because it found no measured need. Declaring speculative quotas would have been worse than declaring none.
- Items 2 and 3 confirmed as genuine no-ops by grep (zero `HostEffect::` usages, zero self-tick loops) rather than assumed.
- **Two real wiring gaps found outside its literal brief**, without which none of the rest would have mattered: stdio's `Cargo.toml` never requested the `component-guest` feature (so the actor world was never exported for wasm), and `📦️glue.rs` never called `plugin_exports!` (so no `descriptor_is_fresh()` test existed). Both fixed against the `🗒️note` reference.

**Descriptor not committed, and that is the correct outcome.** Emission is blocked by a pre-existing SDK validation rule (`🔌️plugin/🦀️component.rs:2568`): a definition must declare a capability whose claim set exactly equals its runtime claims, and up to 35 of stdio's 36 formats fail it. M0 generated a descriptor via a native fixture, saw it carried `pluginId: "assembly-failed"` placeholder data, and **deleted it rather than check garbage in**. That judgment is worth more than a green checkbox — a committed placeholder descriptor would have poisoned the registry and the freshness test simultaneously.

That capability-claim mismatch belongs to stdio's own artifact declarations, not this ticket. Recorded as a follow-up; `🗒️note` proves the path works where declarations are consistent.

**`M1-small-plugins` dispatched** (🖍️draw, 📋️forms, ➗️mathematical, 📏️layout, 🖨️raster) carrying M0's two wiring findings and the "read the value from the code, never hardcode" standard, plus an explicit instruction not to commit placeholder descriptors if they hit the same claim-set rule.

### ✅ H1-react — React renderer on the actor kernel; W2 renderers complete

Verified by me:
- **My loud placeholder is gone** — zero matches for `"invokeExtension completion not yet deliverable"`. It is replaced by a real `req`-correlated completion: the branch now calls `completeExtensionInvoke(instanceId, req, { ok })`, delivering `Event::Completed` to resume the guest SDK's parked `RequestRegistry` future, exactly as `📓️design-abi.md` §2 specifies. The temporary marker I left in A3's wave did not become permanent, which is the whole reason it was made loud.
- Every surviving `evictPluginModule`/`acquirePluginModule`/`PluginModuleLease` reference in `ShellHost` is a **doc comment** explaining the removal — no live call sites.

**Landed:** `PluginRuntime.loadPluginModule` drives a real actor through `ActivationRegistry` + `ShardClient` instead of the deleted per-plugin Worker lease; `exchange()` submits `app-command` events and demuxes `TurnResult.effects` back to `AppFrame` bytes; a new, independently unit-tested `applyUiPatchToRetained` backs a rewritten `refreshUi`, so window bodies come from `Event::SurfaceVisible` + `TurnResult.uiPatches` rather than a blocking round trip — the UI thread no longer waits on a plugin.

**A real gap H1 found and closed rather than papered over:** H2's deletion of `withSerializedPluginWasmHandle` left nothing enforcing one-turn-at-a-time per actor on the host side — the shard worker *rejects* overlapping turns rather than queueing them. H1 added `serializePerActor` to restore that invariant. That is precisely the class of silent correctness hole that only surfaces under the concurrency this ticket exists to enable.

Also fixed, all verified by targeted `tsc` rather than assumed: a missed `HostEffect`→`Effect` rename in `PluginRuntime.tsx`, three `ShellHost` call sites still using H2-deleted `evictPluginModule`, and stray dead imports in the React entry.

**Acceptance:** `framework-renderer-react` 321/336 (15 pre-existing failures identified by name and root cause); the packet's own tests isolated → **14/14**; `framework-os` baseline reconfirmed **322/324** — the same 2 pre-existing wasm-artifact failures, zero regression.

**W2 is now complete**: H1 (React), H2 (web shard pool), H3 (wgpu native, verified partial), H4 folded into H2/H3's TypeScript. All four renderer paths are off one-worker-per-plugin and onto the actor kernel.

### M1-small-plugins — 3 of 5 green; 2 blocked upstream

Coordinator-run `cargo check -p <crate> --lib`:

| crate | result |
|---|---|
| `semio-s-plugin-draw` | **Finished**, 0 errors |
| `semio-s-plugin-mathematical` | **Finished**, 0 errors |
| `semio-s-plugin-raster` | **Finished**, 0 errors |
| `semio-s-plugin-layout` | blocked — stale `semio_s_plugin_stdio::artifacts::dwg::DwgDecodeStatus` import and `DwgSnapshot` has no field `bytes` |
| `semio-s-plugin-forms` | blocked — `BlockListScene has no field domain_id` in `📖️playbook`, a live peer edit (uncommitted, ~9-min-old mtime when M1 saw it) |

**M1 corrected me, with evidence, and was right.** I read "2 `.activation(` per crate" off a grep and told it that looked thin next to stdio's 36. It checked and showed each of these five owns exactly **one** artifact kind — so one activation event is the honest number, and my grep had counted a doc-comment mention. Pushing back with a citation rather than padding the count to match my expectation is exactly the behaviour I want.

It also verified M0's two wiring findings did **not** apply here — all five already request `component-guest` and already call `plugin_exports!` — instead of assuming the previous packet's bugs were universal.

**Two fixes I made as registrar** (pre-existing, not from this ticket): `➗️mathematical`'s and `🖍️draw`'s `#[cfg(test)]` modules used `EditorApp`/`App` without importing them, while the working `🗒️note` reference imports both explicitly. Adding the imports took draw and mathematical from failing to green — their test code had simply never compiled.

**The two blocked crates are both upstream, neither ours.** `📏️layout` carries the same stale-`DwgSnapshot`-field problem E2 already fixed inside `🗒️note`; `📋️forms` is blocked by an actively-edited peer file. Recorded rather than patched — `📖️playbook` is outside this ticket's scope and was live.

**Descriptors: none emitted for the five.** M1 ran out of budget before emission after its own build got backgrounded. Correctly, it committed nothing rather than a placeholder.

## W3 full fan-out — five packets, all 33 plugins + 26 extensions covered

User direction: finish end to end **in conjunction with the other ongoing work**, without stopping. That changes one standing rule — packets are now **authorized to fix pre-existing and peer-owned breakage that blocks them**, subject to: re-read from disk immediately before each edit, never rewrite a whole file, no git-modifying commands, list every out-of-scope file touched with its reason, and **still leave anything under active edit** (checked via `git log --date=iso` + mtime + running cargo) and report it instead. The "don't chase a moving target" rule survives; the "don't touch anything that isn't yours" rule is relaxed to "fix what's dead, report what's live".

| packet | crates | target dir |
|---|---|---|
| `M2` | 📐️cad +4 ext, 🪵️sourcing +3 ext | `🎯️target-m2` |
| `M3` | 🌊️flow +9 ext (largest family) | `🎯️target-m3` |
| `M4` | 📜️imperative +5, 📖️playbook +1, 🏭️process +4 | `🎯️target-m4` |
| `M5` | 🧩️puzzle, 🌀️procedural, 🌍️gis, 💠️lowpoly, 📸️remodel | `🎯️target-m5` |
| `M6` | 14 remaining plugins + repair of 📏️layout | `🎯️target-m6` |

Design decisions carried into the briefs so they are not re-derived: flow's 9 and imperative's 5 extensions are `Linked` (same publisher, real evaluators); process's 4, sourcing's 3, cad's 4 and playbook's 1 are `Declarative` (no handlers → no wasm ever runs) — each brief requires verifying that by grepping `.handler(` rather than trusting the classification. Long-running work moves to jobs: puzzle's WFC, remodel's SfM, fem's solver, flow/brep's tessellation (exclusive placement). `HostEffect::InvokeExtension`'s `response_action` is gone — flow and procedural must use `host::extensions::invoke(...).await` resolved by a `req`-correlated `Event::Completed`, not a reintroduced callback id. 🪐️space is the only crate calling `host_now_ms` directly (~81 effect sites, heaviest in the repo).

Two blockers routed into the packets that own the surrounding code rather than patched centrally: `📖️playbook`'s `BlockListScene { domain_id }` against a struct with no such field (→ M4, with a liveness re-check first, since it was a live edit when M1 hit it), and `📏️layout`'s stale `DwgSnapshot`/`DwgDecodeStatus` drift against stdio's evolved API (→ M6, same class E2 already fixed inside 🗒️note).

### 🚨 Repo-wide workspace breakage found by M2 and fixed by registrar

`cargo metadata --no-deps` was exiting **101** — the entire workspace was unloadable, so *every* cargo command on this machine failed, for every session, not just this ticket:

```
error: failed to load manifest for workspace member
       `✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust`
       referenced by workspace at `/Cargo.toml`
```

Cause: a peer session moved the two `🔄️fsm` crates out of `✏️s/🔌️plugins/🖍️draw/🔄️fsm/` into the command co-location path `…/✏️editor/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/` — `git status` shows the renames staged (`R` entries) — but root `Cargo.toml` still listed the old paths. Classic half-landed move: the files went, the registry entry didn't.

Repointed both member entries (root `Cargo.toml` is registrar-owned, so this was mine to do). `cargo metadata --no-deps` now exits **0**. This unblocked all five in-flight W3 packets *and* every peer session on the box.

**M2 gets the credit and behaved correctly:** it hit the failure, diagnosed it precisely to the moved path, refused to fabricate acceptance results while the workspace was unloadable, and reported rather than guessing. It did set a background Monitor to poll for the fix — which cannot survive its turn boundary here — so I told it to stop polling and run acceptance in the foreground now.

This is the third distinct class of peer half-landed change this ticket has absorbed: a moved generated file (ui-styling), a renamed type set (presence), and now a moved crate. All three shared the same signature — the artifact moved, its registration did not.

### W3 declaration sweep — 32 of 33 plugins now declare activation

Measured directly across the fleet (`grep -c '\.activation('` per plugin root):

**32 of 33 plugins carry activation declarations.** 🗄️stdio 37, 📕️norm 16, 🧩️puzzle 4, 🧱️block 4, 🗒️note/🌀️procedural/🏗️fem/🔱️trinity/🪐️space 3, the rest 1–2. Only `🎪️demonstrator` is untouched, which is correct — it was always sequenced last because it bundles panes from six other plugins.

That is the substantive half of W3: every plugin now tells the registry *when it needs to exist*, which is the precondition for "installed packages consume no runtime resources". The remaining half is descriptor emission.

**Descriptors committed: 1 of 33** (`🗒️note`). The blocker is uniform and already diagnosed: `try_library()` fails with `no declared <kind> capability owns the runtime claims` (SDK `🔌️plugin/🦀️component.rs:2568`) whenever a definition's declared capability claim-set does not exactly equal its runtime claims. M0 measured ~35 of stdio's 36 formats failing it. Every packet was instructed — and each complied — to emit **no** descriptor rather than commit placeholder `assembly-failed` data.

That rule is pre-existing and orthogonal to this ticket's ABI work: `🗒️note` proves the emission path is sound where a plugin's declarations are internally consistent. Bringing the other 32 into consistency is a per-plugin data-correctness task, not a runtime one, and it is the single largest remaining item in W3.

### ⚠️ M5 finding: the jobs / timers / async-invoke surface is NOT wired in the guest SDK

The most consequential report of this wave, and it corrects both the plan and my own brief.

1. **It overturned the audit's WFC attribution.** I told M5 (following `📓️luna-imports-audit.md`) that 🧩️puzzle carried wave-function-collapse precompute. M5 checked and found the real 10 930-LOC WFC solver lives in `🌀️procedural/🗿️artifacts/🧩️assembly` and is currently **unmounted/dormant** — puzzle does not contain it. It verified against the code instead of trusting the audit or the brief, which is exactly right; the audit was a `rg` census and this is what censuses miss.

2. **`Effect::SpawnJob`, timer-driven ticks, and async `host::extensions::invoke` are all blocked by missing shared-SDK plumbing**, with file:line evidence in `📓️terra-M5-report.md`. The WIT declares `start-job`/`step-job`/`cancel-job` and A2 built the executor and request registry, but the guest-side path from a plugin's code to a spawned job is not connected. No W3 packet can move long-running work off the turn loop until that lands.

**Consequence, stated plainly:** the "long computations become resumable jobs" half of W3 is **not achievable by the migration packets** and was mis-scoped in `📓️design-abi.md` §6 and in every M-packet brief I wrote. The declaration half (activation events, execution modes, capability requests) *is* landing — 32 of 33 plugins now carry it. The jobs half needs a new SDK packet first.

This is a gap in our own W1/W2 work, not a peer's: A2 delivered the WIT and the executor, A2b made the bridge compile, but nothing exercised `spawn_job` end to end, and no acceptance gate covered it. The lesson matches the WIT reserved-keyword episode — a contract that compiles is not a contract that runs, and nothing in W1/W2 ever ran a job.

**Follow-up packet required before the jobs work can proceed:** wire `Effect::SpawnJob` → host `start_job`/`step_job` → `Event::JobCompleted` end to end, with a test that actually spawns, steps and completes a job — the same "prove it runs, not just compiles" bar the scale fixture sets.

### Throughput ceiling reached — 160 concurrent cargo processes

Six packets (M2–M6, J1) building at once on six separate target dirs is 6× the compilation work plus the sibling peer tickets, all contending for one global package-cache lock. My own verification sweep has been queued behind them for ~20 minutes without emitting a line. Several executors have again fallen into wake/idle loops waiting on backgrounded builds, despite explicit instruction — at this contention level a foreground build can exceed any patience threshold, which is what makes the trap so persistent.

**Correction to my earlier "parallel editing is free, parallel building is not" rule:** per-packet target dirs removed the *lock* contention between our packets but multiplied the *total* work, because each dir rebuilds the same shared dependency graph from scratch. The right shape is per-packet dirs with a **cap of ~3 concurrent builders**, not one dir per packet with unlimited concurrency. Recorded for the next wave.

### Consolidated position

**Verified complete (coordinator-run, not from reports):**
- W1: actor kernel (52/52, purity enforced by grep), contract types (135-file atomic rename, peer region byte-identical), reactor ABI compiling as a real `wasm32-wasip2` component, channel v12 (os-kernel **1003/0** against the peer's recorded 996 baseline), guest SDK on v12, native host with `WasmPluginRuntime`/`ExtensionRuntime`/both `ProgramSupervisorState`/`PLUGIN_FUEL_BUDGET` **deleted** (67/0).
- W2: all four renderer paths off one-worker-per-plugin — React (321/336, own tests 14/14), web shard pool (316/318 baseline held exactly), wgpu native (kernel thread + v12 ProgramBridge + lazy descriptor scan), wgpu web.
- Infrastructure: 2550-record scale fixture (byte-identical across runs, different seed → different output), descriptor pipeline, one real committed plugin descriptor.

**W3, honestly:** 32 of 33 plugins declare activation events, execution modes and capability requests — the half that makes "installed packages consume no runtime resources" real. Two halves outstanding: descriptor emission, blocked fleet-wide by a **pre-existing** capability-claim rule (`🗒️note` proves the path works where declarations are internally consistent); and jobs, blocked by **our own** unwired `Effect::SpawnJob` path, now dispatched as `J1`.

**W4 not started.** The 50×50 claim remains *measurable but not measured* — the fixture exists, the bench does not yet run.

**Repo-wide breakage absorbed this session** (none of it ours, all of it blocking everyone): a moved generated file (ui-styling), a renamed presence type set, a moved crate whose workspace member entry was left behind (this one made `cargo metadata` fail for every session on the machine), plus pre-existing broken `#[cfg(test)]` imports in two plugins.

### ⛔️ Wave halted by self-inflicted contention — next session starts here

Load kept climbing (160 → 174 cargo processes) rather than draining: six migration packets plus `J1`, each with its own target dir, each rebuilding the shared dependency graph, on top of the live peer tickets. No verification command of mine has completed in the last ~40 minutes. **This is my scheduling error, not the executors'** — I dispatched six builders at once after explicitly recording, one wave earlier, that parallel building does not scale here.

**Do this first next session, before dispatching anything:**
1. Let the queue drain to < 12 cargo processes.
2. Re-run the consolidated sweep: `cargo check -p semio-s-plugin-<name> --lib` across the 32 declared plugins, **≤3 concurrent**, and record the per-crate result table that this wave never got.
3. Then resume `J1-jobs-end-to-end` — it is the true critical path, since the jobs half of W3 cannot proceed without it.

**Standing rule, corrected twice now and final:** per-packet `🎯️target-*` dirs remove lock contention but multiply total compile work. Cap concurrent *builders* at 3 regardless of how many packets are editing. Editing is free; building is not; separate target dirs do not change that, they only move the bottleneck from the lock to the CPU.

**State of the work is unchanged by the halt** — everything below is already verified and committed to the tree; only the remaining verification runs are outstanding:

| area | state |
|---|---|
| W1 contracts (actor kernel, kernel/manifest types, reactor ABI, channel v12, guest SDK) | ✅ verified |
| W1 native host, legacy runtime deleted | ✅ verified (67/0) |
| W2 all four renderer paths | ✅ verified |
| Scale fixture (2550 records, deterministic) | ✅ verified |
| Descriptor pipeline + 1 real descriptor | ✅ verified |
| W3 activation declarations | ✅ 32/33 plugins |
| W3 descriptor emission | ⛔️ pre-existing capability-claim rule, fleet-wide |
| W3 jobs migration | ⛔️ our own unwired `SpawnJob`, `J1` dispatched |
| W4 bench / parity / task manager / process shards | ⬜️ not started |

**The 50×50 claim is measurable but still unmeasured.** That sentence should stay in every summary until the bench actually runs.

## 2026-08-18 W4 — session resumed after reboot cleared the contention halt

### ✅ S0 consolidated plugin sweep — 33/33 GREEN, zero red

The per-crate table the halted wave never produced. `cargo check -p <crate> --lib`, 3-wide, three rotating target dirs (`🎯️target-sweep0/1/2`), driver `w4-sweep/run.sh`, raw logs `w4-sweep/<crate>.txt`.

| crate | rc | secs |
|---|---|---|
| semio-s-plugin-animate | 0 | 444 |
| semio-s-plugin-architect | 0 | 339 |
| semio-s-plugin-block | 0 | 322 |
| semio-s-plugin-cad | 0 | 285 |
| semio-s-plugin-dag | 0 | 276 |
| semio-s-plugin-demonstrator | 0 | 289 |
| semio-s-plugin-draw | 0 | 61 |
| semio-s-plugin-energy | 0 | 26 |
| semio-s-plugin-fem | 0 | 153 |
| semio-s-plugin-flow | 0 | 123 |
| semio-s-plugin-forms | 0 | 72 |
| semio-s-plugin-gis | 0 | 65 |
| semio-s-plugin-imperative | 0 | 61 |
| semio-s-plugin-layout | 0 | 76 |
| semio-s-plugin-lowpoly | 0 | 42 |
| semio-s-plugin-mathematical | 0 | 22 |
| semio-s-plugin-norm | 0 | 31 |
| semio-s-plugin-note | 0 | 59 |
| semio-s-plugin-playbook | 0 | 23 |
| semio-s-plugin-procedural | 0 | 47 |
| semio-s-plugin-process | 0 | 42 |
| semio-s-plugin-puzzle | 0 | 86 |
| semio-s-plugin-raster | 0 | 64 |
| semio-s-plugin-reasoning-mindmap | 0 | 81 |
| semio-s-plugin-remodel | 0 | 30 |
| semio-s-plugin-sequence | 0 | 21 |
| semio-s-plugin-shooting | 0 | 23 |
| semio-s-plugin-sourcing | 0 | 25 |
| semio-s-plugin-space | 0 | 58 |
| semio-s-plugin-stdio | 0 | 44 |
| semio-s-plugin-trinity | 0 | 64 |
| semio-s-plugin-vcs | 0 | 22 |
| semio-s-plugin-writer | 0 | 24 |

**Both crates M1 reported blocked are now green**: `📏️layout` (was: stale `DwgSnapshot`/`DwgDecodeStatus` drift against stdio's evolved API) 76s, and `📋️forms` (was: `BlockListScene` missing `domain_id`, a live peer edit at the time) 72s. M6's repair and the peer's own completion both landed. `🎪️demonstrator` — sequenced last because it bundles panes from six other plugins — also compiles clean at 289s.

Total wall time ~13 min at 3-wide on a freshly rebooted machine, versus the previous wave's 40+ minutes producing nothing at 174 concurrent cargo processes. The corrected standing rule (cap concurrent BUILDERS at 3 regardless of packet count) is now evidenced, not just asserted.

### ✅ V1a registrar scaffolding — `bench` verb + `verify rust-warnings` landed

Registrar-owned files, so sol edited them directly rather than leasing:

- Root `📜️script.ts`: new `//#region 🔖️BenchScript` after `🔖️TestScript`, registered as the `bench` verb. Thin router to `@semio-tech/framework-os-dev:bench` — the budgets and harness belong beside the fixture generator that emits the registry they read, so the numbers have exactly one home.
- Root `📜️script.ts`: `verify rust-warnings --target <triple> [-p <crate>…]` in `🔖️VerifyScript`, plus helpers `pluginCrateNames` / `rustWarningTargetScope`.
- Root `📋️project.json`: targets `bench-plugins` and `verify-rust-warnings` (both `cache: false`, `forwardAllArgs`).

**Two design decisions worth recording, both taken from evidence in the tree rather than invented:**

1. **Deny-on-warnings is clippy's trailing `-- -D warnings`, never `RUSTFLAGS`.** `runCargoLint`'s own docstring in the repo library states why: `RUSTFLAGS` REPLACES rather than merges with `.cargo/config.toml`'s rustflags (`-Z threads=8`, the wasm32 `getrandom_backend` cfg, mold), which would break every wasm build. Following the existing rule instead of reaching for the obvious env var avoided reintroducing a defect the repo had already solved once.
2. **`--all-targets` is native-only.** The wasm triples get `--lib` (plus `--features component-guest` for wasip2): plugin test harnesses are native-only, so a wasm `--all-targets` clippy would fail on code that target never ships. Target→crate-set resolution: `wasm32-wasip2` → the 33 plugin crates; `wasm32-unknown-unknown` → `semio-framework-actor` (the purity-critical one); native → framework + kernel + the fleet.

Verified without spending a build slot: `bench` routes (reaches `BenchScript`, rejects a missing subcommand), `verify rust-warnings --target bogus-triple` rejects with the expected message, `📋️project.json` parses with 65 targets, and the plugin-crate discovery resolves **33** crates — byte-matching the S0 sweep list.

Launch-seed entries deliberately deferred: they must name commands that `T1`/`P1`/`V1b` are still creating, and the seed is regenerated (`plugin-registry:generate`), so adding entries for not-yet-existing verbs would bake a broken launch.json.

### ⚠️ Peer liveness re-check before the descriptor wave — the D-wave scope must bend

Checked before dispatching any descriptor-emission packet, per the standing "live predicate, not derived artifact" rule.

`26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` is **open and active ~40 min ago** (`📓️status.md` 14:38, `📓️w4-animate-report.md` 14:36). Its W4 is migrating plugins from the old `.artifact(declaration())` channel to the new `.declare_artifact(artifact())` tree, **batch 1 = 11 plugins, structurally converted and committed (`dirty=0`)**: 🎬️sequence, 🌿️vcs, 📋️forms, 🗒️note, 🪵️sourcing, 🕸️dag, ➗️mathematical, ✒️writer, 🖍️draw, 💡️reasoning, 🎞️animate. Further batches are queued behind its own concurrency cap. Its log also records three other live interactive sessions in this tree.

**This corrects the D-wave plan, and the correction matters more than the plan did.** An explorer established that the "capability-claim rule blocks emission fleet-wide" line in this log is stale — the rule only fires on the 21 old-channel plugins, and the two worst-cited cases (🗄️stdio, 📐️cad) were already brought into consistency by peer tickets. But the same investigation named the strategically cheaper path as "finish the peer's migration", and **that path is not ours to take**: the peer is actively walking it, plugin by plugin, right now.

Binding decisions for the D-wave:

1. **Never touch a declaration channel.** No `.artifact(…)` → `.declare_artifact(…)` conversions, no `definition()` row deletions. That is the peer's ticket, mid-flight.
2. **Descriptor emission is orthogonal and stays ours** — it reads whatever surface a plugin registers, through either channel.
3. **A descriptor emitted for a plugin the peer is about to migrate goes stale on their next commit**, and with D0's hardening that stale file becomes a RED test for every session in this tree. So emission is ordered by peer state: the 11 committed batch-1 plugins are safe; the rest are emitted only if the peer has not opened them, and are otherwise recorded as deferred WITH the attribution, not silently skipped.
4. Repeating this ticket's own hard-won rule: **don't chase a moving target** — but "don't touch what's live" is not "don't finish what's dead". The distinction is evidence, re-measured per plugin at dispatch time, never inferred from this table later.

### ✅ K1-suspend-resume-placement — accepted, plus a registrar fix that invalidates part of J1's claim

**Landed by the packet** (all in `🖥️host/🧵️shard/🦀️component.rs`): `ShardLoop::pump` now dispatches `Payload::Suspend` → `GuestRuntime::checkpoint`, `Resume` → `restore`, `Cancel` → cancel-all-jobs + unregister, replacing the blanket `Fault` arm; three new struct variants `ShardOutcome::{Checkpoint{actor,state}, Resumed{actor}, Cancelled{actor}}` carry results back over the transport; `Effect::SpawnJob`'s `placement` is captured into a `job_placement` map (it was destructured away with `..` and silently discarded) and `Exclusive` jobs are stepped first via a stable sort. Four tests added.

**Two judgement calls I endorse rather than override:**
- `Payload::Cancel(u64)` has **no doc comment and no other construction or match site anywhere in the tree** — the packet grepped for one before writing. Rather than invent a meaning for the bare `u64`, it implemented actor-level teardown (the variant sits beside `Suspend`/`Resume`, while per-job cancellation already exists as `Effect::CancelJob{job}` and per-envelope cancellation as `Envelope::cancel_of`), left the `u64` unconsumed, and flagged it for whoever documents it. Deriving intent from the enum's own neighbourhood beats guessing.
- Cross-shard `Exclusive` routing needs `Kernel`/`ShardTable` wiring a `ShardLoop` cannot reach, so it implemented the honest in-shard approximation, labelled it as such, and filed a lease-request instead of faking placement.

#### 🐛️ The real find: J1's "jobs proven end to end" did not cover the completion path

K1 reported a failing test it correctly refused to fix (3 files outside its scope). I reproduced it in isolation before acting, rather than taking the report's word:

```
Running -> Ok("{\"kind\":\"running\",\"progress\":[1]}")
Done    -> Err(Error("cannot serialize tagged newtype variant JobStep::Done containing a sequence"))
Failed  -> Err(Error("cannot serialize tagged newtype variant JobStep::Failed containing a sequence"))
```

**J1 fixed `Running` and stopped.** Its own doc comment states the rule as "serde cannot serialize a newtype variant whose payload is itself an `Option`" — but the actual rule is *any sequence*, and `Vec<u8>` is a sequence. So **every successful job completion failed to serialize**, on the single path a job must survive to be worth having. J1's resumability test drove three `step_job` calls and asserted on in-process `JobStep` values rather than on bytes that had crossed `send_outcome` — so the completion path was proven in memory and never on the wire.

Registrar fix applied across the files K1 could not touch: `JobStep::Done{output}` / `Failed{error}` as struct variants, plus all 12 construction/match sites in `🖥️host/🦀️component.rs` and `🧵️shard/🦀️component.rs`. The WIT-generated `JobStep` (guest SDK, scale fixture) is untouched — component-model variants are not serde-tagged, so those newtypes are correct where they are.

**The generalisable lesson, recorded because this ticket has now been bitten by it twice:** fixing one variant of a defect is not fixing the defect. When a rule is discovered through one symptom, re-derive the rule and re-check every sibling — J1 patched the variant that failed rather than the class that was broken, and a green test suite hid the other two for a full wave.

Acceptance (coordinator-run, after the fix):
```
cargo check -p semio-framework-plugin-host --all-targets  → Finished, 0 errors, 0 warnings in this crate
cargo test  -p semio-framework-plugin-host --lib          → ok. 74 passed; 0 failed
```
K1 had reported 72 passed / 1 failed; the fix takes it to **74/0** (its own two suspend/resume tests now also exercise a serializable completion). Also cleared an `unused MutexGuard` warning in `MockGuestRuntime::instantiate` (`drop(self.queue_for(actor))` — the entry-creating side effect was the point, the guard was not). One warning remains in `semio-framework-os-kernel` (`unused_assignments`), left for Z1.

### 🔓️ Registrar unblock: descriptor emission was blocked by TWO defects, neither of them the capability rule

D0 hit a wall running the note describe round-trip. Diagnosing it cleared the fleet-wide blocker this ticket has carried since W3 — and neither cause was the capability-claim rule the log blamed for a full wave.

**Defect 1 — WASI Preview 2 was never wired into any linker in this repo.**

```
component imports instance `wasi:io/poll@0.2.9`, but a matching implementation was not found in the linker
```

A precise, already-verified lease-request for exactly this was sitting unactioned in our own ticket folder — `📓️lease-from-LLM-FIRST-OS-P7b-wasi-linker.md`, filed by the `P7-headless-workspace` packet of a peer ticket, addressed to "whoever owns `🔌️plugin/🖥️host/**`", i.e. us. It had already established the root cause (`world actor` declares only `pure`; the Rust `wasm32-wasip2` target pulls WASI in transitively, so every real component needs a full WASI linker regardless of its own WIT), confirmed `wasmtime-wasi = "22.0.1"` was a declared-but-never-used dependency, and written the fix against the pinned crate source. **It blocked that peer's entire headless path, our descriptor emission, and our bench — and it sat unread in our folder while we attributed the blockage to something else.** A lease-request nobody reads is indistinguishable from a bug nobody found.

Applied in both places that build a `Linker` (the describe CLI has its own, which the lease did not cover):
- `🖥️host/🦀️component.rs`: `ActorHostState` gains `wasi_ctx`/`resource_table`, `impl WasiView`, `wasmtime_wasi::add_to_linker_sync` beside the existing `pure::add_to_linker`.
- `📇️describe/📦️glue.rs`: same shape on `DescribeHostState`, plus `wasmtime-wasi` added to that crate's `Cargo.toml`. Its dependency comment had explicitly reasoned *"no `wasmtime-wasi`: the world declares no wasi import"* — a conclusion drawn from the WIT and disproved the first time a real component was instantiated. Comment replaced with the measured reality.
- Both ctxs are the sandboxed default (no inherited stdio/fs/network/env), matching the crate's capability-gated stance.

**Defect 2 — the describe fuel cap was ~18× too small, and failed without saying so.**

With WASI linked, instantiation succeeded and execution trapped *inside* the component at `AppBuilder::try_build_definition` with a bare "error while executing" — no mention of fuel. `DESCRIBE_FUEL_BUDGET` was `5_000_000`, documented as "generous for a pure struct-building function": an estimate made against the function's *shape*, never against a real build. Measured actual consumption for `🗒️note`:

```
[DEBUG] describe fuel remaining after call: Ok(19907672227) of 20000000000   → 92_327_773 consumed
```

Raised to `2_000_000_000` (~21× headroom over the measured figure, still bounding a runaway to seconds) with the measurement written into the docstring and an instruction to **re-measure, not re-estimate**, if a larger plugin trips it. Probe instrumentation removed.

**Round-trip proof — the thing that was missing:**
```
cargo run -p semio-framework-plugin-describe -- describe <note.wasm> --out <probe>  → exit 0
regenerated 🔣️descriptor.json vs committed ✏️s/🔌️plugins/🗒️note/🔣️descriptor.json  → sha256 IDENTICAL (266 725 bytes, structural diff empty)
```

The emission path is now proven reproducible end to end against a real `wasm32-wasip2` component. **Both defects were environmental, not per-plugin data problems** — which is why 32 plugins "failed the capability rule": most of them never got far enough to reach it.

### ✅ C1 census cleanup accepted — exit-checklist item 9 measured clean

C1 deleted the last live `ProgramSupervisorState` (`💻️os/🖥️host/🦀️component.rs`, the enum + its ~10 consumers, replaced by the actor kernel's real `ActorStatus`/`FailureStage`) and the process-global `set_host_backbone_channel` (`🏪️store/🦀️component.rs`). Both crate checks finished (`semio-framework-os` 4m53s, `semio-framework-os-kernel`), and I re-ran the census myself rather than accepting the report — the standing lesson is that an executor's file count is not proof.

Full "Replace, never wrap — these must not exist at exit" census, doc-comment prose excluded:

| symbol | live hits | verdict |
|---|---|---|
| `PluginWorkerClient` (both copies) | 0 | ✅ gone |
| `PluginModuleLease` | 0 | ✅ gone |
| `ExtensionRuntime` | 0 | ✅ gone |
| `ProgramSupervisorState` (both defs) | 0 | ✅ **C1** |
| `PLUGIN_FUEL_BUDGET` | 0 | ✅ gone |
| `PLUGIN_WORKER_UNRESPONSIVE_MS` | 0 | ✅ gone |
| `INSTANCE_GUARD` / `clear-instance-guard` | 0 | ✅ gone |
| `install_io_fallback_dispatcher` | 0 | ✅ gone |
| `set_host_backbone_channel` | 0 | ✅ **C1** |
| `runSerialized` | 0 | ✅ gone |
| `loadPluginModuleUncached` | 0 | ✅ gone |
| `LeasePool` | 14 | ✅ **correct** — all 14 in `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`, exactly the relocation `📌️important.md` prescribes for the generic pool's 3 non-plugin users. Zero in the kernel. |
| `WasmPluginRuntime` | 1 | ⚠️ one error-message string in `🏃️run/🦀️component.rs:1363` naming what was removed — R1 is replacing that code path |
| `host_port` | 3 | ✅ false positives — a URL-parsing local in `Shell/🧊️component.rs` and testcontainers' `get_host_port_ipv4`; unrelated to the deleted WIT import |
| `exchange(` | 37 | ✅ false positives — see below |

**Naming hazard worth recording, because it will re-fire on every future census.** The banned `exchange` is the WIT-level plugin ABI replaced by `poll(events,budget)→TurnResult`. In `📜️component.wit` the word now survives ONLY in prose describing what it replaced ("the old `exchange(id, cmds)` collapses into this"), and the real export is `poll` — verified. But the host keeps an unrelated internal `TransactionCoordinator::exchange(plugin_id, instance_id, AppCommand) -> Vec<AppFrame>` (`🖥️host/🦀️component.rs:3379` + 36 call sites): a different mechanism that happens to share the word. A grep-based census cannot tell them apart, so **anyone re-running item 9 will see 37 hits and must not read that as the ABI surviving.** Either the census pattern gets narrowed to the WIT surface, or the host method gets a name that is not a banned word.

Item 9 ("none of the must-not-exist symbols remain") is therefore **met**, with the one genuine remainder (`WasmPluginRuntime` in a string) owned by R1.

### ✅ T1 metrics + task manager — accepted, with one honest gap left standing

**(a) Metrics.** `ActorMetricsSample`/`ShardMetricsSample`/`RuntimeMetricsSnapshot` + a `runtime_metrics_due` 2Hz gate added to `🎭️actor/🦀️component.rs` with the clock **always caller-injected**; `RuntimeMetricsPublisher` (native host) and `startRuntimeMetricsPublisher` (web `ActivationRegistry`) both reuse the pre-existing `Origin::Bus`/`Payload::Event` primitive rather than inventing a second bus.

**The gap T1 declared rather than hid, and it is the right call:** nothing in this codebase — native or web — yet drives a live `Kernel` thread or tracks topic subscribers, so the publication path is **correct but unreachable**. It is wired, tested and provably shaped right, and no running process currently calls it. Claiming "metrics published at 2Hz" would have been false; claiming nothing would have lost the work. Recorded as belonging to whoever lands a live kernel thread.

**(b) Task manager.** New `TaskManager` element reuses the existing dual-rendered `SurfaceKind::Table` scene pipeline (it studied `Table`/`Interpreter` first) instead of inventing a new surface kind — so React+wgpu parity is genuine and no out-of-scope renderer file was touched. Plus an i18n'd (en/de, no default language) accessible `TaskManagerPanel`.

**Coordinator-verified, not taken from the report:**
```
purity grep on 🎭️actor/🦀️component.rs → only match is the doc comment asserting purity (crate core clean; this is what keeps mobile open)
🔖️IoRouter region in 🎠️kernel/🟦️component.ts → 240 lines, sha256 ddb2ce7f… — byte-identical to the hash A3 recorded in its own wave
```
The peer-region invariant has now survived two independent packets editing the same file, which is the whole point of measuring it each time rather than assuming it.

Reported green: actor 57/57 (was 52/52), plugin-host 75/75, shard-client vitest 30/30, kernel vitest 14/14, TaskManager vitest 9/9. **Note a number disagreement to re-measure, not average:** my own post-K1 run of plugin-host was 74/0. Two measurements of the same suite differ, so the suite gets re-run rather than either figure being quoted.

**Follow-up dispatched:** T1 wrote its suspend/resume/cancel actions as "correctly shaped but not yet dispatched (K1's pending work)" — but K1 landed mid-flight. T1 resumed to wire the three actions through to the real `Payload::Suspend/Resume/Cancel` → `checkpoint`/`restore`/`cancel` paths, so the buttons stop being inert.

**Lease filed and pending with registrar:** `terra-T1-lease-typegen.md` — `🎭️actor/🤖️generated/` is **empty**; ts_rs typegen for this crate has apparently never been run. Until it does, the TS side uses hand-authored stand-in interfaces (the same pattern `🧵️shard-client.ts` already used for `ShardBudget`). Not a hard blocker; queued behind the bench for build-slot reasons.

### ✅ D0 descriptor plumbing accepted + registrar lease actioned — the gate is now honest

**What D0 landed:** one canonical descriptor path (the plugin/extension **owner root**, sibling of the tracked `🛂️manifest.json`), a shared `describePluginComponent()` in `📇️describe/📜️script.ts` registered as a `describe` command + nx target on **all 33** plugin crates, and a hardened `descriptor_is_fresh()` using an explicit per-crate opt-in ratchet (`DESCRIPTOR_MIGRATED_PLUGINS = ["note"]`) so a missing descriptor hard-fails for a listed crate while the unmigrated fleet stays green. `cargo test -p semio-s-plugin-note --lib` → **115/115** including `descriptor_is_fresh`.

**Two findings worth keeping:**
- It fixed a real pre-existing bug in `ensureBuiltBin()`, which hardcoded `target/debug` and ignored `CARGO_TARGET_DIR` — under this ticket's mandatory ticket-scoped target dirs that would have exec'd a stale or absent binary from the wrong tree.
- **The committed `🗒️note` descriptor was genuinely stale**: the live peer ticket had migrated note's declaration channel since E2 committed it. Re-running `describe` is the intended maintenance action, and this is the first evidence that descriptors decay when a peer changes declarations — exactly the coupling that makes emitting descriptors for peer-held plugins a bad idea right now.

**Lease actioned by registrar** (`📇️registry/📜️script.ts`, `DESCRIPTOR_JSON_REL_PATH`): dropped the `🤖️generated/` segment. The old constant reasoned by analogy with `🎭️actor`'s generated TS bindings — but that analogy points at a directory holding *regenerable build output*, while a descriptor is a *tracked artifact* whose entire purpose is to be the committed, static answer the registry reads without instantiating wasm. `🤖️generated/**` is globally gitignored, so the old path could never hold a committed descriptor at all.

**The failure mode this had been producing is worth naming:** the gate and the freshness test were reading *different files*, and **both reported green**. `plugin-registry:check` said note had no descriptor while a real, fresh one sat at the owner root and `descriptor_is_fresh()` passed against it. Two green signals, disagreeing, neither wrong on its own terms.

Verified after the lease landed:
```
bun nx run @semio-tech/plugin-registry:check    → exit 1: "plugin registry catalog is stale" (the gate could finally SEE descriptor data)
bun nx run @semio-tech/plugin-registry:generate → exit 0
bun nx run @semio-tech/plugin-registry:check    → exit 0: "descriptor gate: 1/59 crates have a 🔣️descriptor.json"
```
Warnings now name the correct owner-root path and a command that actually exists. **1/59 is the true number** — previously the gate reported 0 while the answer was 1, and would have kept reporting 0 no matter how many descriptors were emitted.

Pre-existing gap D0 found and did not paper over: `🔋️energy` declares no `crate-type` in its `Cargo.toml`, so no wasm artifact is produced for it at all — it cannot be described until that is fixed.

### 🔧️ Bench blocked by a third half-landed peer refactor — fixed, same signature as the other two

The first native bench run died compiling the wgpu renderer:
```
error[E0063]: missing field `color` in initializer of `PresencePeerRow`
  --> 🧱️elements/Shell/🧊️component.rs:316
```
`PresencePeerRow` (`🖱️ui/🧱️elements/👥️PresenceBar`) gained `color: Option<u8>` — "hub-assigned session-color palette index (contract freeze §C7.5)" — and this inbound mapping site was not updated. `Shell/🧊️component.rs` is registrar-owned (shared with live presence/hover tickets), so this was mine.

**Resolved by reading the data flow, not by filling the field to make it compile.** The wire `PresencePeer` (`📡️spr/📡️wire/🦀️component.rs:859`) has **no colour field at all** — checked the whole struct, not just the neighbourhood: actor, connected_at_ms, label, presence_pack, user_id, role, drag_ghost_json, interaction. The hub assigns session colours out of band through its `Session` frame, and this shell keeps no such roster. So `None` is the only truthful value, it is the row's own documented "no hub connection" default (renders as palette index 0), and it matches the identical decision already recorded 2 000 lines away in this same file for the OUTBOUND heartbeat.

Recorded as a real consequence rather than a silent default: **remote peers all render at palette index 0 in wgpu** until either the wire carries the colour or the shell tracks Session frames. That is the presence ticket's call, not this ticket's.

`cargo check -p semio-framework-os-renderer-wgpu --lib` → **Finished in 2m 01s**, 0 errors.

**This is the fourth distinct half-landed peer change this ticket has absorbed**, and all four share one signature — *the artifact moved, its registration did not*: a moved generated file (ui-styling), a renamed presence type set, a moved crate whose workspace member entry was left behind (that one broke `cargo metadata` for every session on the machine), and now a struct that gained a field its call sites did not. The pattern is stable enough to be worth a check rather than a lesson: after adding a required field to a shared struct, grep its initializers before considering the change landed.

### ✅ T1 follow-up — the task manager's buttons now do something

Dispatched after K1 landed mid-flight, which turned T1's "correctly shaped but not dispatched" actions into closeable work.

- `🎠️kernel/🟦️component.ts` (`ActivationRegistry`): new `cancel(actorId)` disposes the worker instance and forgets the actor, mirroring K1's native semantics (cancel running jobs + unregister) — so a later `resume()` correctly throws rather than silently resurrecting a dead actor.
- `TaskManager/🟦️component.tsx`: `createTaskManagerDispatcher(registry)` maps suspend/resume/cancel onto `ActivationRegistry`'s real methods, which call through to a real `ShardClient`.
- 3 new tests build a real `ActivationRegistry` + real `ShardClient` (only `Worker` is faked), render a real panel, click a real button, and assert real state changed — the same bar `AgentApprovals`' own dispatch tests set.

**Native side deliberately left alone, with the right reason**: `Kernel::suspend`/`resume` and `Payload::Cancel` are the correct calls, but nothing drives a live `Kernel` thread natively yet — the same root cause as the metrics publisher gap. It filed **no** lease, correctly observing there is no concrete landable diff to request, only missing infrastructure. A lease-request for "someone should build a thing" would have been noise.

**The number disagreement resolved by re-measurement, not by averaging.** I had plugin-host at 74/0, T1 first reported 75/75. Re-run fresh: **75/75, 0 failed** — and the explanation is that T1's own follow-up added a test, so both earlier figures were correct at the moment they were taken and neither is now. This is the second time today two green measurements of the same suite disagreed; both times the answer was to re-run rather than to pick one.

Also re-verified: actor 57/57, shard-client 30/30, kernel vitest 17/17, TaskManager 12/12, and the peer's `🔖️IoRouter` region **still** byte-identical at 240 lines (md5 `222db26f…`) after a second round of edits to that file.

## 🎯️ THE BENCH RAN — and immediately earned its keep

First real execution of the 50×50 scale bench through `Kernel` + `ShardLoop` + `WasmtimeRuntime` against the real `wasm32-wasip2` fixture component. `bun ./📜️script.ts bench plugins --renderer native --count 50 --extensions 50` → exit 0, report at `terra-v1b-bench-native.json`.

**Run 1 result: `1:pass 2:fail 3:fail 4:fail 5:skipped 6:fail 7:fail 8:fail`**

| # | budget | measured |
|---|---|---|
| 1 | registry 2550 records, 0 instantiations, <150 ms | ✅ **4.78 ms**, recordCount 2550, instantiations 0 |
| 2 | cold boot ≤1.5 s native | ❌ `instance count too high at 2` |
| 3 | activate 100 actors across K shards | ❌ same |
| 4 | native RSS ≤1.5 GiB | ❌ same |
| 5 | interactive p95 ≤8 ms | ⏭️ skipped — budget 4 failed before it could run |
| 6 | hang killed, shard rebuilt, pause ≤250 ms | ❌ same |
| 7 | stateful suspend/resume, identical hash | ❌ same |
| 8 | capability revoked at runtime | ❌ same |

**Six failures, one cause — and it is not a scaling limit.** `wasmtime: resource limit exceeded: instance count too high at 2`. `BudgetLimiter::default()` set `max_instances: 1` (and `max_tables`/`max_memories: 8`). Those are **core-module** numbers guarding a **component** store: the component model instantiates one core instance per module in the component graph (guest module + the `wasm32-wasip2` adapter + whatever `wit-bindgen` composes), so a real plugin dies at its second core instance.

**What that actually means: `WasmtimeRuntime` — the production native host — could never instantiate ANY real component.** Not a slow path, not a scaling ceiling: zero. The whole native runtime this ticket built was, in this one respect, non-functional.

**Why nothing caught it for three waves.** B1 landed `BudgetLimiter` as one of the four pieces that did not depend on A1/A3, and it compiled. W1's acceptance was `cargo check`/`cargo test`. The unit tests use `MockGuestRuntime`, which never instantiates wasm. The ONE path that had ever instantiated a real component end to end — the `📇️describe` emitter — builds its own `Store` and **installs no limiter at all**, so it sailed past. Two independent green signals, neither touching the broken line.

This is the third instance today of the same class, and the sharpest: **a contract that compiles is not a contract that runs, and a test that passes against a mock is not a test of the runtime.** J1's jobs, the WASI linker, and now the resource limiter were each "verified" by a check that structurally could not observe the defect. The bench is the first artifact on this ticket that runs the real thing, and it found this on its first execution.

Raised to `max_instances: 256`, `max_tables`/`max_memories: 128` — bounded against a hostile component, room for ordinary composition, with the instruction to re-measure rather than re-guess. Re-run in flight.

### 📝️ Z1 backlog item found while the bench queued: `[DEBUG] ` used for permanent diagnostics

`📌️important.md` rule 8 reserves the `[DEBUG] ` prefix for **temporary** logs, removed before a packet reports done — that is what makes a later sweep safe to run blind. The new bench code uses it for permanent operator-facing diagnostics instead: `🧊️wgpu/📦️glue.rs` **29**, dev `📜️script.ts` **51**, wgpu `📜️script.ts` **4**. Examples are real error paths (`"scale-bench: failed to read {}"`, `"engine build failed"`, `"wrote <report path>"`) that SHOULD survive.

The hazard is precise: a future sweep that deletes every `[DEBUG] ` line — exactly what the rule licenses — would strip the bench's entire error reporting and leave it failing silently. Repo-wide the count is 312+, so the prefix has already lost its meaning as a marker.

Z1 should re-prefix permanent diagnostics (or drop the marker) rather than delete them, and the distinction is worth stating in `📌️important.md`: `[DEBUG] ` means *delete me*, not *this is a log line*.

### 📊️ Bench run 2 — real actors ran, and the shard pool turned out not to be sharding

With the limiter corrected, actors instantiate and execute. **`1:pass 2:fail 3:fail 4:fail 5:skipped 6:fail 7:PASS 8:fail`** — and the failures are now *measurements* rather than one blocked line.

| # | measured | verdict |
|---|---|---|
| 1 | 2.76 ms, 2550 records, 0 instantiations (≤150 ms) | ✅ pass |
| 2 | **718 ms** cold boot (≤1500 ms) — but 143 actors live and **29 guest traps** | ❌ timing well inside budget; faults fail it |
| 3 | activeActors **100/100**, shardsReported 8 — but `perShardCounts {"0": 100}`, maxShardLoad **100** vs ceiling **14** | ❌ **all actors on one shard** |
| 4 | `maximum concurrent limit of 1000 for core instances reached` | ❌ second pool limit |
| 5 | — | ⏭️ skipped, budget 4 failed first |
| 6 | trap `cannot enter component instance`, killed=false, siblingsRestored=true | ❌ |
| 7 | checkpoint `395f1136…` **identical** after suspend→resume, through the REAL `ShardLoop::pump` → `checkpoint`/`restore` path | ✅ **pass** |
| 8 | `capabilityRequested: false`, survived revoke turn, status Idle | ❌ actor never requested the capability |

**Budget 7 passing is K1's vindication**: suspend → resume → re-checkpoint produces byte-identical state through the production dispatch path that was faulting out this morning. The bench measured the thing the packet claimed, independently.

#### 🐛️ The headline defect: `ShardTable::pin` never distributed anything

Budget 3 activated 100 actors across 8 configured shards and put **all 100 on shard 0**.

`pin` was `ShardId((actor.0 % pool as u64) as u16)`. `ActorId` is bit-packed `plugin_ordinal:u16 | kind:u2 | ordinal:u32 | generation:u14` — **generation occupies the low bits**, and `Kernel::activate` mints every actor at generation 0. So `actor.0 % 8` was `0` for every actor that has ever existed. The pooled-shard multiplexing that is this ticket's entire reason for being was a no-op, in the one line that implements it.

**Why three waves of tests missed it:** the existing coverage asserts pin/pack round-trips and that a pinned actor resolves to a shard. Every one of those passes when the answer is always shard 0. There was no test of the *property* — that N actors occupy more than one shard.

Replaced with **least-loaded** placement, not a hash: budget 3's `no shard > ceil(actors/K)+1` is a hard bound that hash variance breaks well before 100 actors, and exact balancing gives it by construction. Least-loaded also refills the gaps `unpin` leaves, which a round-robin counter strides past. Ties break on lowest id — deterministic, no clock, no RNG, crate stays pure. (I wrote the hash version first, and the new distribution test failed it — the test caught my fix, which is the point of asserting the property rather than the mechanism.)

Three tests added that would have caught the original: `pin_spreads_actors_of_one_plugin_across_the_pool` (8/8 shards occupied, none over ceiling), `pin_is_idempotent_for_the_same_actor`, `pin_refills_the_gap_left_by_unpin`. Actor crate **60 passed / 0 failed**.

#### 🐛️ Second pool limit, same class as the first

Budget 4 died on `maximum concurrent limit of 1000 for core instances reached` while the component pool still had thousands free. `build_shared_engine` set `total_component_instances` and nothing else — but the pooling allocator meters core instances, memories and tables from **separate pools that each default to 1000**, and one component consumes several of each.

Sized off the component budget so raising one knob cannot silently leave the others behind — but **not uniformly**, and that distinction cost a build to learn: multiplying `total_memories` by the core factor makes the reservation `total_memories × max_memory_size` = tens of TiB, the allocator refuses outright, and `build_shared_engine` **silently falls back to on-demand** — losing the entire pooling design while every test still passes. Caught by `build_shared_engine_defaults_to_pooling` flipping to FAILED. Core instances ×8, tables ×4, memories ×1: core/table slots are bookkeeping, memory slots are address space. Host **86 passed / 0 failed**.

### 📊️ Bench run 3 — shard fix confirmed; and the bench was failing itself on the fixture working correctly

**`1:pass 2:fail 3:fail 4:fail 5:skipped 6:PASS 7:PASS 8:PASS`** — four passing.

**Budget 3's distribution is fixed, measured:** `perShardCounts {"0":13,"1":13,"2":13,"3":13,"4":12,"5":12,"6":12,"7":12}`, maxShardLoad **13** against ceiling **14**, all 8 shards occupied, 100/100 actors active. From all-100-on-shard-0 to textbook balance. **Budget 6 and 8 also flipped to pass** — 8's `capabilityRequested` went `false` → `true` because actors now actually reach their `InstanceOpen` turn.

#### 🔍️ But budgets 2 and 3 were failing on a criterion that is not in the spec

Both still reported fail — budget 2 at **669 ms against a 1500 ms threshold**, budget 3 with every specified quantity correct. The only unmet condition in each was `faults == 0`.

That condition is **not in `📓️design-workforce.md` §4**. Budget 2 is a deadline plus "only `on-startup-finished` actors live"; budget 3 is actor count, shard count and per-shard ceiling. Neither mentions faults. And the fixture ships `hang` (393 records) and `crash` (343) **specifically so the watchdog and failure ladder have something to catch** — 29% of the catalog. Requiring zero faults across a random sample of it is requiring the crash profile not to crash.

Measured proof it was that and not a runtime defect: 29 faults across 143 boot actors ≈ 20%, against a 29% hang+crash share of the catalog — the right order for a random draw, and the fuel hypothesis was already excluded because the harness overrides fuel to 200M.

Corrected to count **unexpected** faults only: a trap from an `idle`/`cpu`/`ui`/`io`/`stateful` actor is still a real failure and still fails the budget; a trap from `hang`/`crash` is the fixture doing its job. Implemented as one shared `unexpected_faults()` helper so both budgets use the same rule.

**Worth stating as its own lesson**: this is the mirror image of every other defect found today. Those were runtime bugs hidden by a too-weak check; this was a correct runtime failed by a too-strong one. A red result deserves the same scrutiny as a green one — the wrong response would have been to "fix" the runtime until the crash profile stopped crashing.

Also fixed, same class as the earlier pool bug: budget 4's error moved from `core instances` to `maximum concurrent GC heap limit of 1000 reached` — a THIRD pooling sub-pool with its own 1000 default.

### ✅ P1 process shards — accepted, with the runtime proof the packet was actually for

`ProcessTransport`/`StdioTransport` (length-prefixed, tagged framing) in the plugin-host crate, plus a `semio-shard` `[[bin]]` hosting a real `ShardLoop` + `WasmtimeRuntime` over stdio. The `🎭️actor` crate is untouched — purity grep still matches only its own doc comment, so the mobile-keeping constraint holds and the transport lives host-side exactly as `🚚️ShardTransport`'s doc prescribes.

**It proved the thing rather than compiling the thing.** Built a real `wasm32-wasip2` component (verified component-model magic bytes), spawned two `semio-shard` children, ran real turns, then `kill -9`'d child A **from outside the process** and watched a native port of `ShardClient.checkHeartbeats` detect the EOF, rebuild shard A with a fresh child, and confirm untouched shard B still responsive. Coordinator-verified from its log:

```
[semio-shard] pid=1664 package=scale-fixture-a actor=1 ready
[semio-shard] pid=1665 package=scale-fixture-b actor=2 ready
[semio-shard] pid=1681 package=scale-fixture-a actor=3 ready     ← rebuilt after kill -9
test process_shard_kill_is_detected_and_the_shard_rebuilds_while_a_sibling_shard_stays_healthy ... ok
```

Three distinct PIDs is the evidence: two spawned, one respawned. This is the first genuine process-isolation demonstration on the ticket, and it mirrors the web `ShardClient`'s semantics rather than inventing different ones.

`cargo check -p semio-framework-plugin-host --all-targets` and `cargo test --lib` both exit 0, **86 passed / 0 failed** — and its arithmetic reconciles with mine exactly (74 baseline + 12 new).

Gaps it flagged rather than glossed: no live scheduler wires `ShardRuntimeKind` yet (pre-existing, grep-confirmed), no multi-actor bootstrap over the child's CLI, no checkpoint-restore across the kill boundary. Launch-entry lease recorded for the registrar.

## ✅ THE 50×50 CLAIM IS NOW MEASURED — 6 of 8 budgets pass

`bun ./📜️script.ts bench plugins --renderer native --count 50 --extensions 50` → exit 0.
**`1:pass 2:pass 3:pass 4:fail 5:skipped 6:pass 7:pass 8:pass`**

| # | budget | measured | threshold |
|---|---|---|---|
| 1 | registry parse, zero instantiations | **2.71 ms**, 2550 records, **0 instantiations** | ≤150 ms ✅ |
| 2 | cold boot, only startup actors live | **764 ms**, 143/143 startup actors, **0 unexpected faults** | ≤1500 ms ✅ |
| 3 | 100 actors across K shards | **100/100** active, 8/8 shards, per-shard **13,13,13,13,12,12,12,12**, max 13 | ceiling 14 ✅ |
| 4 | memory ≤ K×512 MiB + headroom | `maximum concurrent GC heap limit of 1000 reached` | ⛔️ 4th pooling sub-pool |
| 5 | interactive p95 ≤8 ms | — | ⏭️ depends on budget 4's fleet |
| 6 | hang killed, shard rebuilt, siblings restored | trap caught, siblings restored, pause 0 ms | ≤250 ms ✅ |
| 7 | stateful suspend/resume identical hash | `395f1136…` **identical** both sides | ✅ |
| 8 | capability revoked at runtime | requested ✓, survived revoke + follow-up turn, status Idle | ✅ |

Every one of these is a real number from real `wasm32-wasip2` components executing through the real `Kernel` + `ShardLoop` + `WasmtimeRuntime`. The sentence that has stood in every summary since W4 was scoped — *"the 50×50 claim is measurable but still unmeasured"* — can finally come out.

**Budget 3 is the one that matters most**, because it is the ticket's thesis stated as a number: 100 actors, 8 shards, 12-13 actors per shard. This morning that same measurement was `{"0": 100}`.

### The honest remainder

**Budget 4 is genuinely unmet**, and it is a real constraint rather than a bug: full 2550-actor concurrency hits wasmtime's pooling allocator caps. Four separate sub-pools have now been found this way — component instances, core instances, memories/tables, and GC heaps — **each with its own 1000 default, and each invisible until the scale exceeds it**. They surface strictly one run at a time: fixing one lets the bench run far enough to hit the next. `total_gc_heaps` is now configured too; whether that clears budget 4 or reveals a fifth cap is the next run's answer, not a prediction.

**Budget 5 is skipped, not passed** — it needs budget 4's full fleet. **Web renderers (react/wgpu) were never run** and their rows say `skipped` with a reason, never a fabricated pass. So the measured claim today is precisely: *50 plugins × 50 extensions activate, shard-balance, checkpoint, survive revocation and get their hangs caught — on native, at 100-actor and 143-actor scale.* Not the full 2550 concurrently, and not on the web renderers.

### 🚀️ Budget 4 breakthrough: **all 2550 actors live at 391 MB RSS**

With `total_gc_heaps` configured (the fourth sub-pool), the full-scale run got through:

```
"activatedCount": 2550, "activeActors": 2550, "rssBytes": 410009600   (391 MB)
```

**2550 actors — the entire 50×50 catalog — instantiated and live simultaneously in 391 MB.** The budget's own ceiling is 4.25 GiB, so memory comes in at under a tenth of it. This is the "K shards ⇒ ceiling independent of package count" claim from `📋️master.md`'s baseline table, measured: the runtime it replaces managed ~20 plugins before exhausting a 4 GiB-per-module guard region.

Budget 4 still reported `fail` on `faultCount: 516` — the same too-strict criterion I had corrected for budgets 2 and 3 but not this one. 516/2550 ≈ 20%, again the hang+crash share. Same `by_design` filter now applied; re-running.

### ⚠️ Budget 5 ran for the first time — and its failure needs reading carefully

`p95Ms: 237.99` against an 8 ms native threshold, 0 round faults, 30 rounds. But look at the samples: `229.905, 229.957, 229.970, 229.972, 229.973, 229.982, 229.982…` — **thirty samples inside a 0.1 ms band**. That is not contention jitter, it is a constant.

The cause is the harness's own documented limitation, which V1b stated up front rather than hiding: **one physical `ShardLoop` backs all K shard labels**, so `pump()` runs every actor serially on one thread. The interactive turn queues behind 40 `cpu` actors busy-looping their declared milliseconds — ~230 ms of strictly serial work, reproduced to within a tenth of a millisecond every round.

**So budget 5 is measuring the harness, not the kernel.** The kernel's design point is that K shards execute in parallel; a single-threaded driver cannot exercise that no matter how the number comes out. Recording it as a **failure with a known-invalid instrument** — not as evidence the design misses its latency target, and not quietly as "skipped" either. It becomes measurable once a real multi-shard executor drives it (P1's `ProcessTransport` and the thread-shard path are both candidates); until then the row stays red and honest.

This is the counterpart to the `faults == 0` correction: there, a good runtime failed a bad criterion; here, an untested property fails a good criterion measured with the wrong instrument. Both are reasons to read a red row rather than react to it.
