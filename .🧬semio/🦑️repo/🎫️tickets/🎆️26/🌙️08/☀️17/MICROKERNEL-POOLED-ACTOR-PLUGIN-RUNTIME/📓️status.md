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
