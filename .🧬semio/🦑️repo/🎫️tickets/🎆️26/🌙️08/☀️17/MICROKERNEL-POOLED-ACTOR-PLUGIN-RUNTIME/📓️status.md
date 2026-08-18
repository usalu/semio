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

## 🏁️ FINAL BENCH — 7 of 8 budgets pass, 2550 actors concurrent

`bench plugins --renderer native --count 50 --extensions 50`, seed 1, K=8 shards. Archived at `🔣️bench-native-FINAL.json`.
**`1:pass 2:pass 3:pass 4:PASS 5:fail 6:pass 7:pass 8:pass`**

| # | budget | measured | verdict |
|---|---|---|---|
| 1 | registry 2550 records, 0 instantiations, <150 ms | **17.9 ms**, 2550 records, **0 instantiations** | ✅ |
| 2 | cold boot ≤1500 ms, only startup actors live | **742 ms**, 143/143, 0 unexpected faults | ✅ |
| 3 | 100 actors, K shards, ≤ceil(100/K)+1 per shard | **100/100**, 8/8 shards, **13,13,13,13,12,12,12,12** | ✅ |
| 4 | memory ceiling at full scale | **2550/2550 actors live, 390 MB RSS**, 0 unexpected faults | ✅ |
| 5 | interactive p95 ≤8 ms under 40 cpu actors | 295 ms — **invalid instrument**, see below | ❌ |
| 6 | hang killed ≤2× budget, siblings restored | trap caught, siblings restored, 0 ms pause | ✅ |
| 7 | stateful suspend→resume identical hash | `395f1136…` identical both sides | ✅ |
| 8 | capability revoked at runtime | requested ✓, survived revoke + follow-up, Idle | ✅ |

### What is now actually proven

**Budget 4 is the ticket's headline claim, measured: all 2550 records — 50 plugins × 50 extensions + parents — live simultaneously in 390 MB.** The baseline table in `📋️master.md` records the runtime this replaces as capped at roughly 20 plugins by a 4 GiB-per-module guard region. The ceiling is now set by the shard pool, not by package count, exactly as `📓️design-runtime.md` claims.

**Budget 3 is the thesis as a number**: 100 actors distributed 12-13 per shard across 8 shards. The same measurement this morning read `{"0": 100}`.

### Budget 5 — the one honest failure, and why it is not a design result

`p95 = 295 ms` against 8 ms. But the 30 samples sit in a **0.1 ms band** (229.903, 229.948, 229.950, 229.964, 229.973…). A p95 that reproduces to a tenth of a millisecond under "saturating background load" is not measuring contention — it is measuring a constant.

The harness runs **one physical `ShardLoop` behind all K shard labels** (V1b documented this up front). `pump()` therefore executes every actor serially on one thread, and the interactive turn queues behind 40 `cpu` actors busy-looping their declared milliseconds. The kernel's entire design point for this budget is that K shards run in parallel; a single-threaded driver cannot exercise it whatever number it prints.

**Recorded as a failure with a known-invalid instrument** — not as "the design misses its latency target", which the data does not support, and not quietly as "skipped", which would hide a real gap. It becomes measurable when a genuine multi-shard executor drives it; P1's `ProcessTransport` (proven today with a real kill→rebuild across OS processes) and the thread-shard path are both candidates. **That is the single highest-value follow-up on this ticket.**

### Scope of the claim, stated precisely

Measured today: **native only**, seed 1, K=8. Web renderers (react/wgpu) were never run and their rows say `skipped` with a reason — never a fabricated pass. So: *50×50 activates, shard-balances, checkpoints, survives capability revocation and gets its hangs caught, at full 2550-actor concurrency in 390 MB, on native.* Interactive latency under load is **unmeasured**, not passed.

### ✅ R1 native manifest — work accepted, verification was NOT done by the packet

**Its design decision is right and unusually well-evidenced.** Prefer the committed `🛂️descriptor.semio` (zero instantiations); live `describe()` is a designed-but-unwired fallback. It cited five independent sources that all converge: `📓️design-abi.md` §3 twice (freshness gate treats the descriptor as the artifact of record; the emitter runs once at build time), `📓️design-runtime.md` §3 (`ActivationRegistry` seeded from build-time descriptors), `📜️component.wit`'s own `describe` doc ("build-time only, never called at runtime"), and this ticket's founding premise. Reading the descriptor by instantiating the wasm would have reintroduced exactly the per-package instantiation this ticket exists to remove.

Wired in `🏃️run/🦀️component.rs`: `read_committed_descriptor` (same `pack_rt::decode_wire_value` + `dsl::from_dsl_value` decode the emitter uses, applied to committed bytes), and `load_runtime_recursive` now performs the full walk the old `NOT YET WIRED` comment specified — compile → decode → recurse dependencies → mint actor → register with io/mutation/inference routers, plugin graph and app router → `owned_surface_gaps` gate.

**It also declined to do the wrong thing for the right reason**: wiring the live-`describe()` fallback needed either a `GuestRuntime::describe` seam (out of scope, and P1/T1 were live in that file) or a second hand-rolled wasmtime+WASI linker inside `🏃️run` — which would put raw `wasmtime` calls outside the `GuestRuntime` interface that CLAUDE.md requires external libraries to stay behind. It filed a lease instead. Correct call.

**But its report shipped with `## Commands + exit codes` → `(filled in below)`, never filled in** — a direct violation of binding rule 7, and the third packet today to end a turn before verifying. Coordinator-run instead:
```
CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo check -p semio-framework-os-run --all-targets  → Finished in 5m 55s, 0 errors
```
Genuine finding it surfaced: `ContributionSet.io_entries` cannot be mapped to `io_schema::IoEntryDescriptor` because **two different types share that name** — the manifest crate's (owner/counterpart/direction) and the io crate's (from/into/fidelity/sniffs) — and the descriptor schema carries no fidelity/sniffs data. A name collision that a compiler cannot warn about, in the same family as the `exchange` census hazard recorded earlier today.

Native smoke across all 33 plugin ids remains gated on descriptor emission (10/33 at time of writing), not on this wiring.

## 📌️ Session consolidation — 2026-08-18 W4

### Accepted (coordinator-verified, never taken from a report)

| packet | result |
|---|---|
| S0 sweep | **33/33 plugin crates green, 0 red** — incl. `layout` and `forms`, both previously blocked |
| V1a | root `bench` verb + `verify rust-warnings --target` + 2 nx targets |
| C1 | `ProgramSupervisorState` + `set_host_backbone_channel` at 0 live; **exit item 9 met** |
| K1 | suspend/resume/cancel dispatch + placement capture; 4 tests |
| D0 | one canonical descriptor path, `describe` on all 33 crates, opt-in ratchet; gate honest at 1/59 |
| T1 | metrics sampling + publisher, `TaskManager` dual-renderer, actions wired to real `ActivationRegistry`→`ShardClient` |
| P1 | `ProcessTransport` + `semio-shard` bin; **real `kill -9` → detect → rebuild, sibling survives** (3 PIDs logged) |
| V1b | bench harness; **7/8 budgets measured passing** |
| R1 | committed-descriptor path wired into the native runner |
| D1 | descriptor emission in flight, 10/33 |

### Registrar fixes landed by sol

`JobStep` serde variants · WASI p2 into both linkers · describe fuel 5M→2G · `PresencePeerRow.color` · `ShardTable::pin` least-loaded + 3 property tests · 4 pooling sub-pools · `Mailbox.lanes` `ts(skip)` (first-ever actor typegen) · registry `DESCRIPTOR_JSON_REL_PATH` · bench `unexpected_faults` criterion · `MockGuestRuntime` guard drop

### NOT done — stated plainly

- **V2 parity 58/58 both renderers: not started.** The harness exists and works; the run needs dev servers and hours.
- **Z1 zero warnings: dispatched, unfinished.** Known non-zero on all three targets.
- **Descriptor emission: 10 of 33.** Deliberately limited to the peer-stable set; the rest wait on `CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`'s migration batches.
- **Native smoke 33/33: blocked** on the above, not on R1's wiring.
- **Bench budget 5: unmeasured**, not passed — needs a real multi-shard executor.
- **Web renderer benches: never run.**
- **Launch-seed entries** for task-manager / process-shards / bench gates: not added (leases pending).

**The ticket must NOT be closed on this session's work.** Exit checklist items 1-8 are not all met; `📌️important.md` is not emptied. What changed today is that the runtime now demonstrably runs — it did not this morning — and the headline claim has real numbers behind it for the first time.

### 🐛️ Eighth defect, found by the zero-warnings gate: the wasm bindings did not compile

Coordinator ran `verify rust-warnings --target wasm32-unknown-unknown` to hand Z1 a baseline. That target was not merely warning — **it did not compile**:

```
error[E0308]: mismatched types
  --> 🎭️actor/📦️packages/🦀️rust/📦️glue.rs:73
   self.inner.complete(actor, result, now_ms)   // Kernel::complete now takes &TurnResult
```

`Kernel::complete`'s signature had become `complete(&mut self, actor, result: &TurnResult, now_ms)` — almost certainly during T1's metrics work — and the wasm-bindgen glue still passed by value.

**`📦️glue.rs` is `#[cfg(target_arch = "wasm32")]`-gated, so no native `cargo check` or `cargo test` ever compiles it.** Whoever changed the signature got a green native build AND a green 60/60 test suite while the web bindings were broken. Same shape as everything else today — the verification that ran could not observe the defect — but with a sharper edge, because this file is invisible to the entire native toolchain by construction.

**The zero-warnings gate is the first thing on this ticket that compiles that file at all.** That is an argument for keeping `verify rust-warnings` in the routine gate rather than saving it for the exit checklist: it is not a tidiness pass, it is the only cross-target compile coverage the repo has. Recorded as a recommendation for `📌️important.md` on the next wave.

Z1 fixed the E0308 itself (credited to it, not to me). Remaining on that target: one clippy `needless_pass_by_value` at `📦️glue.rs:85`, denied by `-D warnings`; exit still 101, target not yet clean. Logs: `w4-z1-wasm-unknown.txt`, `w4-z1-wasm-unknown2.txt`.

### ✅ R1 finalized — and a systematic naming hazard now has four instances

`cargo check` and `cargo test -p semio-framework-os-run` both exit 0, **16/16 tests**. `🗒️note` verified loading natively end to end with `--nocapture` evidence — and when the D0-built wasm artifact it had relied on vanished mid-run, it **rebuilt the component itself rather than downgrading the row to "skipped"**. Exactly the right instinct.

Honest scope from its own report: only `note` was built and tested here; the other 32 are blocked on descriptor emission and wasm builds, **not on this packet's wiring**. 10 descriptors committed and climbing.

**Its mid-implementation find completes a pattern this ticket keeps paying for.** `GuestRuntime::instantiate`'s `Budget` is `semio_framework::kernel::Budget`, not `semio_framework_actor::Budget` — two distinct types, same name. That is now the **fourth** same-name-different-type collision recorded:

| name | the two types |
|---|---|
| `ActorId` | `kernel::ActorId` (presence/collab) vs the runtime actor id — already in `📌️important.md` as a naming hazard, aliased `RuntimeActorId` |
| `exchange` | the deleted WIT plugin ABI vs the live host `TransactionCoordinator::exchange` — makes the must-not-exist census (exit item 9) report 37 false positives |
| `IoEntryDescriptor` | manifest crate (owner/counterpart/direction) vs io crate (from/into/fidelity/sniffs) — blocks `ContributionSet.io_entries` mapping (R1) |
| `Budget` | `kernel::Budget` vs `actor::Budget` (R1) |

Three of the four cost real debugging time today. The compiler catches the type collisions eventually; it never catches the `exchange` one, which is why that census needs a narrowed pattern rather than a grep for a banned word. **Recommendation for the next wave**: treat a same-name-different-type pair across crates as a defect to be renamed at the point of discovery, not a curiosity to be documented — `RuntimeActorId` is the precedent that already worked.

### ✅ D1 descriptor emission — the pipeline is closed end to end, 10 plugins ratcheted

`DESCRIPTOR_MIGRATED_PLUGINS` now reads `["note", "sequence", "vcs", "forms", "sourcing", "dag", "mathematical", "writer", "reasoning-mindmap", "animate"]` — precisely the peer-stable batch-1 set the packet was scoped to, with nothing outside it. Each entry means that plugin's `descriptor_is_fresh()` now **hard-fails** on a missing or stale descriptor instead of silently passing.

Registrar verification of the whole loop:
```
plugin-registry:check     → exit 1: "plugin registry catalog is stale" (the 9 new descriptors changed real catalog data)
plugin-registry:generate  → exit 0
plugin-registry:check     → exit 0: "descriptor gate: 10/59 crates have a 🔣️descriptor.json"
```

**That stale→regenerate→green cycle is the proof the pipeline is genuinely connected.** This morning the gate read the gitignored `🤖️generated/` path, so it reported 0 and would have kept reporting 0 no matter how many descriptors were emitted — the number could not move. It now moves with reality: emit descriptors, the catalog goes stale, regenerate, the count rises.

**10 of 59, and the remainder is a sequencing decision rather than a failure.** The other 22 top-level plugins plus extensions are held back deliberately: the peer ticket `CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` is open and migrating declaration channels in batches, and D0 demonstrated that a peer's migration silently invalidates a committed descriptor (note's own was stale for exactly that reason). Emitting for a plugin the peer is about to touch would put a red `descriptor_is_fresh` in every session's tree. The right sequence is: their batch commits, then ours emits.

Note for whoever runs the exit checklist: `plugin-registry:generate` must be re-run **after** the final descriptor lands, or the catalog will be stale at close.

## 🧾️ Exit checklist — measured position (design-workforce §6)

| # | item | state |
|---|---|---|
| 1 | `verify gate` exit 0 | ⬜️ not run this session |
| 2 | `verify` and `test long` exit 0 | ⬜️ not run |
| 3 | Parity 58/58 both renderers across 4 shards | ⛔️ **not started** — harness exists and works, run needs dev servers + hours |
| 4 | Native smoke all 33 plugin ids exit 0 | 🟡 wiring done + `🗒️note` proven end to end; **10/33 have descriptors**, rest blocked on emission, not on wiring |
| 5 | Bench green on react/wgpu/native, JSON in `bench/` | 🟡 **native 7/8 measured**, JSON archived `🔣️bench-native-FINAL.json`; **web never run**; budget 5 needs a multi-shard executor |
| 6 | Zero rust warnings on native + wasip2 + wasm32-unknown-unknown | 🔄 Z1 in flight; wasm32-unknown-unknown had a real **compile error** (fixed), one clippy lint remains |
| 7 | `plugin-registry:check` fresh, `launch.json` regenerated, no stray `[DEBUG] ` | 🟡 registry **fresh, exit 0, gate 10/59**; launch seed entries **not added** (leases pending); `[DEBUG] ` misuse catalogued, Z1 owns |
| 8 | Task manager shows live actors in both renderers | 🟡 built dual-renderer, actions dispatch through real `ActivationRegistry`→`ShardClient`; **no live `Kernel` thread exists natively**, so the metrics publisher is correct-but-unreachable |
| 9 | Census: 0 sync host imports, no must-not-exist symbols | ✅ **met** — every symbol 0 live; `LeasePool`'s 14 are the prescribed relocation; `exchange`'s 37 are the documented name collision |
| 10 | `📌️important.md` emptied, `ticket_close` with explicit path | ⛔️ **not done, and must not be** |

**Two items met, six partial, three not started. The ticket does not close on this session.**

### What a closing session needs to do, in order

1. Let the peer ticket's remaining declaration batches land, then emit the other 23 descriptors and ratchet each — **re-run `plugin-registry:generate` after the last one** or the catalog is stale at close.
2. Finish Z1 across all three targets. Keep `verify rust-warnings` in the ROUTINE gate afterwards: it is currently the only thing in this repo that compiles the `#[cfg(target_arch = "wasm32")]` glue at all.
3. Drive bench budget 5 with a real multi-shard executor (P1's `ProcessTransport` is proven and available) — today's 295 ms is a single-threaded harness artifact, not a design result.
4. Run the web benches and the 58×2 parity sweep.
5. Wire a live `Kernel` thread natively so T1's metrics publisher and the task manager's native actions stop being unreachable.
6. Then items 1, 2, 10.

### 🔬️ Ratchet spot-check — the descriptor gate is real, not decorative

A ratchet entry is only worth anything if the test it arms actually runs and passes. Coordinator-run against two of the ten ratcheted plugins:

```
cargo test -p semio-s-plugin-note --lib descriptor_is_fresh → test descriptor_is_fresh ... ok   (1 passed, 114 filtered out)
cargo test -p semio-s-plugin-vcs  --lib descriptor_is_fresh → test descriptor_is_fresh ... ok   (1 passed,  58 filtered out)
```

(A third, `🕸️dag`, was cut off by my own 10-minute command timeout — the two above are the verified ones. Not claiming a third pass I did not see.)

So for these plugins the chain is now closed and enforced end to end: **committed descriptor at the owner root → `descriptor_is_fresh` hard-fails if it goes missing or stale → `plugin-registry:check` counts it → the generated catalog carries its data.** Before today every link in that chain existed and none of them met.

D1 has continued past its original peer-stable scope — **12/33 and climbing** at time of writing.

### ✅ D1 went past the safe set — correctly, and the ratchet is what makes that safe

14 descriptors committed, but the ratchet still lists **only the 10 peer-stable ones**. The four extras — `🌊️flow`, `🎥️shooting`, `🏛️architect`, `🏭️process` — are old-channel plugins the peer's future batches will migrate, and they are committed **without** a ratchet entry.

That is precisely the right split, and it is worth naming because the two halves have different failure modes:
- **Committed descriptor** → the registry counts it and the generated catalog carries its data. Value now.
- **Ratchet entry** → `descriptor_is_fresh` HARD-FAILS on drift. Value now, red tree for every session later if the peer migrates that plugin.

Emitting broadly while ratcheting narrowly gets the first without buying the second. Peer liveness re-checked at the same moment: **zero plugin `.rs` files modified in the last 30 minutes**, so nothing was being raced.

**Residual risk, stated so it is not forgotten:** an unratcheted descriptor that goes stale still feeds *wrong data* into the generated catalog — silently, since no test guards it. That is a data-correctness problem rather than a broken-build problem, and the mitigation is the one already recorded for the closing session: re-run `plugin-registry:generate` after the peer's migration completes, and ratchet each plugin only once its declarations have settled.

### ✅ Z1 zero-warnings — one target clean, and it found a bug in the gate itself (mine)

| target | end state |
|---|---|
| `wasm32-unknown-unknown` | ✅ **clean** — 13 clippy errors → 0 in `semio-framework-actor` (plus the E0308 compile error my earlier run exposed) |
| `wasm32-wasip2` | 🔄 **was blocked 100% of the time by a bug in my own verb**; now unblocked, see below |
| `native` | 🟡 `semio-framework-actor` clean; the other 35 crates unreachable through the aggregate gate behind two out-of-scope crates (`semio-framework-os-kernel-dsl-derive` 2 errors, `semio-framework-mesh-engine` 13) — exact fixes documented, lease-requested |

#### 🐛️ The gate bug was mine, and I had already been told

`rustWarningTargetScope` passed `--features component-guest` for the wasip2 target. **Plugin crates declare no `[features]` section at all** — `component-guest` is a *dependency* feature each one enables unconditionally on `semio-framework-plugin` (`features = ["component-guest"]` on that dep line). `cargo -p <plugin> --features component-guest` therefore fails with "does not contain this feature", blocking that target on every invocation.

**D0 reported this hours earlier**, in its own report, with the reasoning spelled out — it had hit the same wall building its `describe` command and correctly omitted the flag. I wrote the verb without applying its finding, and it took Z1 hitting the identical wall independently for it to land. Verified before fixing (`grep '\[features\]'` on `🗒️note`'s manifest returns nothing; the only `component-guest` is on the dependency line), then removed the flag.

After the fix the target genuinely runs and reaches real lints in dependency crates (`semio-framework-graph`'s build script: `map_or` simplifications, `chunks_exact` with constant size). Same shape as native: the gate is reachable, and what it now reveals is out-of-scope crates.

**The lesson is about the workforce, not the flag**: a finding buried in one packet's report does not propagate to a sibling packet or to the coordinator by itself. D0's discovery was correct, written down, and still cost Z1 a full blocked target. Cross-packet findings need to be lifted into `📌️important.md` or a coordinator message at the moment they are read, not left in a report to be re-discovered.

#### What Z1 did well
- **Caught and fixed its own regression** (an import removal that broke `--tests`).
- **`[DEBUG] ` cleanup done by judgement, not by sweep**: 29+4+51 occurrences across the three in-scope files, all assessed as permanent operator diagnostics and **re-prefixed rather than deleted** — exactly the distinction that would have stripped the bench's error reporting.
- **Spawned 4 follow-ups for "gap, not dead code" findings** (`PluginRuntimeRegistry` fields, stdio zip mtime/ISO21320 stub, stdio STL ascii export, wgpu retained-node/fuel-quota) instead of blanket-`#[allow]`-ing them.
- **Found `semio-s-plugin-puzzle` has 176 pre-existing test compile errors**, confirmed not its own, flagged prominently and left untouched.

## 2026-08-18 W5 — async-first rewrite opens (plan: /Users/ueli/.claude/plans/rewrite-everything-async-where-wondrous-leaf.md)

New user directive: *"Rewrite everything async where it makes sense, no matter the effort"* — extend/refactor clean mechanisms, plan exhaustively for a parallel agent workforce, end to end for all non-legacy technology. Planning was done in a separate Fable 5 session; this session coordinates as sol (Opus 5).

**Ticket NOT reopened — it was never closed.** `repo://tickets` shows `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` `status: OPEN`, which is correct: last session's own exit checklist said it must not close. The async waves therefore land as **W5+** of this ticket rather than a new one, and this ticket's outstanding exit items (parity sweep, web benches, Z1 tail, 23 descriptors, live native `Kernel` thread) are absorbed into W9 — running the parity/web benches against the pre-async runtime would be work thrown away twice.

### Session-start checks (all measured, not assumed)

| check | result |
|---|---|
| disk | **241 GiB free** (74 % used) — no action needed; the 100 %-full incident of 08/17 has not recurred |
| cargo processes at start | **1** peer: `cargo build -p semio-s-plugin-demonstrator --target wasm32-wasip2` (~4 min in). Finished by 18:35, so the machine was quiet before this wave |
| repo-wide source churn | **zero** `.rs`/`.ts`/`.tsx` modified in the preceding 60 min |
| prior session | Z1 wrote `📓️terra-Z1-report.md` + `w4-z1-wasip2-note.txt` at 18:06–18:08 and stopped; no attributable process since. Treated as finished, not live |
| live peer ticket | `LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` is OPEN and explicitly "builds on the landing state (actor Kernel, reactor ABI, Broker, PackageDescriptor)" of this ticket — the same session that filed the WASI-linker lease we actioned. Its consumers of `Effect`/`GuestRuntime` are why W5 must not churn those shapes casually |

### User decisions binding this wave

1. **ABI: full WASI 0.3 native backend** — WIT authored in 0.3 semantics as SSOT, wasmtime component-model-async as the primary native path, with the proven `poll(events,budget)` reactor kept as the lowering/compat backend (web via jco, and fallback wherever the 0.3 toolchain gate fails). Even under native async: **one kernel task owns one Store**; no concurrent reentrant component calls (wasmtime still documents parts of that as incomplete).
2. **Async runtime: tokio, but only behind an interface in host crates** — exactly the pattern wasmtime already sits behind `GuestRuntime`. `🎭️actor` stays pure. Scheduling authority stays ours (DRR/lanes/quotas); tokio only parks and wakes I/O.
3. **Scope includes the whole TypeScript/web side**, not just the Rust runtime.

### Toolchain facts measured on this machine (they change what is buildable)

- `rustc 1.99.0-nightly (2026-07-06)`; `wasm32-wasip3` **is** in `--print target-list`, but rustup has no prebuilt std for it → wasip3 needs `-Z build-std`.
- `~/.cargo/registry/cache` already holds **wasmtime 34.0.2** and **wit-bindgen 0.57.1** beside the pinned 22.0.1 / 0.36.0 — an upgrade path exists offline.
- `wasm-tools`, `jco`, `wac` are **not installed**. So no standalone WIT validation is available today; component-model claims must be proven by real host instantiation instead.

### 📐️ Registrar ruling 1 — the WIT does NOT get re-split into 12 files

`📓️design-abi.md` §1 specifies one file per interface under `📦️packages/🦀️rust/📜️wit/`. Measured reality: that directory is **gone**, and all 12 interfaces live as `interface` blocks inside a single consolidated `🧬️schema/📜️component.wit` (822 lines) — the repo's single-file consolidation tooling collapsed them mid-session on 08/17, as this log already records.

Re-splitting would (a) fight a tool that has already demonstrated it re-collapses such splits, and (b) delete the exact path both `bindgen!` and `wit_bindgen::generate!` now point at. So the async worlds and the params refactor land **inside the consolidated file**, using its `interface` blocks as the organizing structure. Same class of correction as the descriptor-path ruling: the design doc's stated path is superseded by what the taxonomy tooling actually enforces.

### 📐️ Registrar ruling 2 — `cancel-request` is deferred out of W0

The plan put a new `Effect::CancelRequest{req}` variant in W0 alongside the params refactor. Adding an `Effect` variant breaks **every exhaustive match** on that enum across the fleet — including the TS twin and two registrar-owned files (`Shell/🧊️component.rs`, `ShellHost/🟦️component.tsx`) — which is precisely the "tree goes red for days" shape the A3 wave was split to avoid.

The params refactor on its own is genuinely contained: plugins never touch WIT types (measured in W0's own audit — everything goes through framework-level effects), so only the reactor and host conversion glue move. `cancel-request` therefore moves to the packet that lands the async worlds, where guest-side future-drop actually consumes it and the fallout can be paid once.

### W5 dispatch — 5 packets, file-disjoint

| packet | scope | builds cargo | target dir |
|---|---|---|---|
| `S1-spike` | go/no-go gate: real wasip3/component-async component executing `async func` + an async host import + a **host-written `stream<u32>`** under candidate wasmtime; certifies exact versions and `bindgen!` options | yes | `🎯️target-s1` |
| `R1-async-iface` | new PURE crate `semio-framework-async`: `OperationContext`, tri-state `CancelToken` with `child()`, `Scope`/`ScopeDrainReport`, `ChannelPolicy`, pure `ThreadPlan`+`ThreadBudget`, trait `HostAsyncRuntime` | yes | `🎯️target-r1` |
| `W0-params` | 26 `req`-carrying effect records → reusable `*-params` records (the SSOT mechanism that stops the async and poll worlds ever hand-maintaining two payload copies); Rust `Effect` stays flat | yes | `🎯️target-w0` |
| `T-P1-async-glue` | `createBoundedMailbox` (TS twin of the Rust `Mailbox`), `retryWithJitteredBackoff`, `latestWins`, `fetchWithTimeout`, `waitForEvent` in `🟦️glue.ts` | no | — |
| `T-P8-dev-server` | bounded-parallel materialize (cargo stays serial), sqlite handle cache, stale `🟨️host-shim.js` sweep, SSE keepalive | no | — |

Builder cap held at 3 concurrent cargo packets, per the standing rule corrected twice in W3/W4 (per-packet target dirs remove lock contention but multiply total compile work; the cap is on *builders*, not packets).

Every brief carries the W1–W4 post-mortem rules explicitly: foreground builds only, no git-modifying commands, lease-requests instead of registrar edits, no `ticket_close`, paste output + exit codes, read the FIRST error of a WIT-consuming build, and prove it RUNS rather than compiles.

### ✅ D1 descriptor wave complete — 20/59 gate green, 10 ratcheted, every failure classified

Registrar-run after its final descriptor (D1 correctly did not run `generate` itself):
```
plugin-registry:generate → exit 0
plugin-registry:check    → exit 0: "descriptor gate: 20/59 crates have a 🔣️descriptor.json"
on disk: 20 — matches the gate exactly
```

**Ratcheted: 10** (`note` + D1's 9 peer-stable batch-1 additions — sequence, vcs, forms, sourcing, dag, mathematical, writer, reasoning-mindmap, animate), each with its own `descriptor_is_fresh ... ok`. D1 also reported that 4 of those crates carry unrelated pre-existing test failures elsewhere (forms 2, dag 1, mathematical 18, reasoning 2) and verified none touch descriptor freshness — reporting the surrounding noise rather than hiding behind a filtered pass.

**Emitted but deliberately unratcheted: 10** — flow, shooting, architect, process, lowpoly, cad, norm, remodel, raster, space. Genuine non-placeholder descriptors, outside batch-1, so no `descriptor_is_fresh` guard. Exactly the split asked for.

**Failures classified, nothing fabricated, zero `assembly-failed` placeholders committed:**
| class | plugins |
|---|---|
| weak-linkage duplicate symbols | draw, imperative |
| capability-claim rule | fem, layout, playbook, trinity, stdio, puzzle, block (7) |
| **real cross-plugin dialect collision** on `s.stdio.dwg@ac1018/*` | procedural, gis |
| downstream `kit.catalog` conflict | demonstrator — confirming why it was always sequenced last |
| pre-existing no-`crate-type` gap | energy |

So the capability-claim rule is real for **7 of 33**, not the fleet-wide blocker this log asserted for a full wave. The rest are four distinct, separately-named causes.

**On the disk warning**: it reclaimed **76 GB** by deleting regenerable `incremental/` caches (52 GB wasm + 24 GB native) and top-level `.wasm` files, then held ~244 GB free for the remainder — never approaching the 60 GB stop line. Acting on the constraint rather than stopping at it.

**Residual risk restated in its own words**: the 10 unratcheted descriptors have no freshness guard, so if their declarations change the descriptor goes stale **silently** — data-correctness, nothing turns red — until someone ratchets them the way batch-1 was done.

It also observed a sibling packet (Z1) concurrently hardening `register_composer_entries`/`register_subset_validator` in the same shared `component.rs`, which plausibly explains why several capability/dialect collisions surfaced as loud errors during its run — and confirmed its own edit there was a single isolated line via `git diff --cached`. Attribution checked rather than assumed.

### ✅ T-P1 async glue accepted — and its self-flagged risk turned out to be a latent wire bug

Coordinator-re-run acceptance (not taken from the report): `cd 🧰️framework/📦️packages/🟦️typescript && bun ./📜️script.ts test` → **182 passed, exit 0**.

Delivered: `createBoundedMailbox` (TS twin of the Rust `📬️Mailbox`), `retryWithJitteredBackoff` (full jitter, abort-aware), `latestWins` (single-flight + one trailing coalesced run), `fetchWithTimeout`, `waitForEvent` — all in `🟦️glue.ts`, with a locally-declared response interface so no ambient external type leaks through a public signature.

T-P1 also reported its own bug honestly rather than quietly fixing it: its first `latestWins` deferred `run()` into a microtask, which broke synchronous single-flight and failed its own new test. Caught by the test it wrote, which is the point of writing it.

#### 🐛️ The valuable part: `Lane` casing diverges across languages, and `Backpressure` repeats the `JobStep` defect

T-P1 flagged "a future name-collision risk between my TS twins and the actor module's generated types". Investigating that found two real defects, neither of them a mere naming clash.

**1. `Lane` has a silent cross-language wire mismatch.** `🎭️actor/🤖️generated/🟦️actor.ts` **now exists** (10 KB, written 16:17 today — the T1 typegen lease was actioned after T-P1's brief was written, so this was new information mid-flight). It declares `Lane = "Interactive" | "UserVisible" | "Background" | "Maintenance"`. Checked against the Rust source: `Lane` (`🎭️actor/🦀️component.rs:347-352`) derives `Serialize, Deserialize` with **no `#[serde(rename_all)]`**, so PascalCase is genuinely the wire form and the generated mirror is right. T-P1's `glue.ts:216` had camelCase. Nothing had broken yet only because nothing crosses that boundary yet — and `T-P4` is the packet that makes it cross. Ordered aligned to the Rust/serde form.

Deliberately NOT "fixed" on the Rust side: A4 pinned cross-language byte-parity vectors on this wire, and Rust is the declared SSOT. But the crate is internally inconsistent — `Lane` carries no `rename_all` while `Backpressure` (L734-740) declares `rename_all = "camelCase"` — which deserves a deliberate ruling rather than a drive-by change from either direction. Tracked, not silently patched.

**2. `Backpressure::Dropped(Lane)` is the `JobStep` bug again, in a different enum.** `Backpressure` is `#[serde(tag = "kind")]` (internally tagged) with a **newtype** variant carrying a `Lane`, which serializes as a string. serde cannot serialize an internally-tagged newtype variant whose payload is not a map — it fails at RUNTIME, compiles clean. That is character-for-character the defect that made **every successful job completion** fail to serialize earlier in this ticket (`JobStep::Done(Vec<u8>)`), whose recorded lesson was: *fixing one variant is not fixing the defect — re-derive the rule and re-check every sibling.*

**That sibling sweep was never done.** The generated mirror exposes the tell in four more places — impossible intersection types that no value can satisfy: `Origin` → `{"kind":"actor"} & ActorId`, `Payload` → `{"kind":"event"} & Array<number>` and `{"kind":"cancel"} & bigint`, `FailureSignal` → `{"kind":"trap"} & string`, plus `Backpressure` → `{"kind":"dropped"} & Lane`. So the generated mirror is **not usable as a wire contract for those variants**, independent of whether serde is in the path. A luna audit (`📓️luna-serde-newtype-audit.md`) is running the full repo-wide inventory and, crucially, a per-variant liveness verdict — whether each is actually serde-serialized somewhere (live bug) or only travels through the repo's hand-rolled `pack` codec, which bypasses serde's tagged-enum path entirely (latent trap). The distinction decides whether this is urgent or merely load-bearing debt.

#### 📐️ Registrar ruling 3 — the mailbox twin moves next to its original

CLAUDE.md: *"If code is repeated, it MUST be close to each other."* The mailbox is the TS twin of an actor-crate type and its main consumer is `T-P4`'s `🧵️turn-scheduler.ts`, which lives in the actor TS package. Keeping it in the generic framework glue also parks it in a package that cannot import the actor module's generated types without inverting the layering.

So `createBoundedMailbox` + its types + tests move to `🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts` (beside `🧵️shard-client.ts`), consuming the generated `Lane`/`CoalesceKey` as SSOT while keeping a correctly-shaped local `Backpressure` with a docstring recording why the mirror's version is unusable. The four genuinely generic utilities stay in `🟦️glue.ts`, which is where their many unrelated consumers reach them.

### ✅ T-P1 follow-up verified — and the mailbox twin can no longer drift from the wire

Coordinator-re-run, both green:
```
🧰️framework/📦️packages/🟦️typescript      → Tests 174 passed (174), exit 0
🎭️actor/📦️packages/🟦️typescript          → Tests  38 passed  (38), exit 0
```

T-P1 solved the `Lane` divergence better than I specified. I asked it to align its values to PascalCase; it instead **stopped declaring the types at all** — `📬️mailbox.ts` now does `import type { Lane, CoalesceKey } from "../../🤖️generated/🟦️actor.ts"` and re-exports them, so the twin is structurally incapable of drifting from the Rust wire again. Fixing the mechanism beats fixing the value.

It kept its own correctly-shaped `Backpressure` (`{kind:"dropped"; lane: Lane}`) with a docstring recording why the generated one is unusable, and it verified by name that the 4 relocated mailbox tests actually execute rather than silently skipping — the package's `🧪️vitest.config.ts` used **explicit filename arrays**, not globs, in `include`/`coverage.include`/`includeSource`, so a new sibling file would otherwise have been invisible while still reporting green. Worth remembering for every future file added to that package.

### 🔬️ Audit correction — the 6 serde findings are LATENT TRAPS, not live bugs

`📓️luna-serde-newtype-audit.md` inventoried 6 internally-tagged newtype variants and graded `Payload::Event` **CRITICAL / "every actor communication path"**. **That severity is wrong, and I checked rather than propagating it.**

The contradiction that prompted the check: the W4 bench ran **2550 actors executing real turns over `ThreadTransport`** and passed 7/8 budgets. If `Payload::Event`'s serde path were live, nothing would have run at all.

Four independent pieces of evidence, all measured:

| evidence | result |
|---|---|
| `🎭️actor` crate dependencies | **no `serde_json` at all** — only `serde` (derive), `thiserror`, optional `ts-rs`. `grep -c serde_json 🦀️component.rs` = **0** |
| transport encoding | `📦️glue.rs:433` `pack_envelope` → `envelope.pack_encode(&mut bytes)` — the hand-rolled pack codec, which never enters serde's tagged-enum path |
| the call sites the audit cited as proof | `📦️glue.rs:353,360,667` are `Payload::Event(serde_json::to_vec(&event)…)` — serde_json serializes the **inner `Event`** and the resulting `Vec<u8>` is then wrapped. `Payload` itself is never serde-serialized. The audit read "serde_json appears next to `Payload::Event`" as "`Payload` is serde-serialized" |
| repo-wide grep for `serde_json::to_vec/to_string` on any of the 6 types | **zero** genuine hits; every match was a different type whose name merely contains "Payload"/"Origin" (`IoPayload`, `MutationOrigin`, Gltf payloads) |

So the correct grading is: **all 6 are latent traps.** They break the instant anyone puts serde in an `Envelope` path (a JSON framing for `ProcessTransport`, a debug dump), and they compile clean until that day.

This is the mirror of the `faults == 0` episode recorded in W4: there a correct runtime was failed by a too-strict criterion; here a sound runtime was graded critical by a misread call site. **A red row earns the same scrutiny as a green one** — the wrong move would have been an emergency refactor of the entire `Envelope` wire on a false alarm.

#### What IS real, and where it lands

Two genuine consequences survive the correction:

1. **The generated TS mirror is unusable for those 6 variants** — `{"kind":"event"} & Array<number>`, `{"kind":"actor"} & bigint`, `{"kind":"cancel"} & bigint`, `{"kind":"faulted"} & Array<number>`, `{"kind":"trap"} & string`, `{"kind":"dropped"} & Lane`. An object intersected with a string/bigint/array is a type no value can satisfy. This is not cosmetic: packet **`T-P4b`** must carry `Envelope`/`Payload` to the web shard worker, and it cannot type that wire from the mirror as it stands.
2. The audit did confirm (and this validates ruling 3) that **ts-rs v10 honours serde renames**, so the generated mirror is authoritative for casing wherever it and the serde form agree — which is why importing from it is the right fix rather than hand-aligning values.

**Assigned to `R3-shard-grants` as required work**, because R3 already owns the `ShardFrame` wire and the actor `🔖️ThreadTransport` region: convert the 6 newtype variants to **struct** variants (the same fix the registrar applied to `JobStep::Done{output}`/`Failed{error}`), re-run the actor typegen, and assert the regenerated mirror contains **no** `} & ` intersection. Doing it in R3 lands it before the web adopts the wire, and pays the match-site fallout once. Greenfield rules apply: no compatibility shim, fix the shape.

**Standing rule this reinforces, now stated as a check rather than a lesson:** after fixing one variant of a serde-shape defect, grep every sibling `#[serde(tag = …)]` enum in the tree for newtype variants. The `JobStep` fix recorded exactly this instruction in W4 and nobody executed it; these 6 sat undetected for a full session as a result.

### 🎯️ S1 spike — VERDICT: GO, but on wasmtime **47.0.3**, and 34.0.2 is a trap

All four gates pass. Real `wasm32-wasip2` component under wasmtime 47.0.3: `ping(41) = 42`, the async host import `echo` observed being called **from inside the guest's await**, and `run(events: stream<u32>)` summing a **host-written** stream to 21. That last one is the gate that mattered — a scalar-only async proof would not have told us whether the real `run(events: stream<event>)` ingress shape works.

**The finding that saves the slice: wasmtime 34.0.2 exposes the whole async API and does not implement it.** My own W5 notes pointed at 34.0.2 because it was already in the local registry cache. S1 inspected the source before spending a build and found `concurrent.rs` is ~**35 bare `todo!()` bodies** and `StreamReader<T>` has **zero trait impls** — then confirmed with a real compile error. Had the upgrade packet targeted 34.0.2 on my say-so, it would have compiled, linked, and panicked at runtime, and we would have discovered it two packets later.

That is a new variant of this ticket's most expensive recurring lesson. The old form was *a contract that compiles is not a contract that runs*. The new form: **an API that exists is not an API that is implemented.** Version availability is not feature availability.

Other measured answers (full detail in `📓️terra-S1-report.md`):
- `Config::wasm_component_model_async(true)` + the **new** `Config::concurrency_support(true)`; `Config::async_support` is now a **deprecated no-op**.
- `bindgen!` has **no `async: true` option** in 47 — async-ness is derived from the WIT's own `async func` syntax. Imports go through a `HasSelf<T>`/`Accessor` pattern; exports are called as `instance.call_x(accessor, args).await` inside `store.run_concurrent(...)`. So the async backend is **not** a "sprinkle `.await`" port; it is a different call model.
- Guest side: `wit-bindgen 0.57.1`, plain `generate!({ path, world })`, plain `async fn` + `StreamReader::next().await`.
- **Route A (`wasm32-wasip3` + `-Z build-std`) hard-fails** — `std`'s own `os::wasi` module is gated to `target_env` p1/p2 only in this nightly. A genuine upstream `std` gap, not something to engineer around. **Route B works**: `wasm32-wasip2` + the async canonical ABI via wit-bindgen 0.57.1. The plan's fallback route is now the primary route.
- Host-side stream writing works via a `StreamProducer` trait; `Vec<T>` has a built-in one-shot impl. Incremental `poll_produce` is **untested** — flagged, not claimed.
- S1 installed `wasm-tools` 1.256.0 to decode the component and confirm the `async func`/`stream<u32>` shapes verbatim, which closes the "no WIT validation tooling on this machine" gap from the session-start notes.

### ✅ W0 params refactor — accepted, and the "+2 regression" was a stale baseline, proven not assumed

23 of the 26 `req`-carrying effect records extracted into reusable `*-params` records; 3 deliberately left unwrapped (`link-resolve`, `respond`, `completed-event`) because their single field is already a nominal type and wrapping it would add a layer that buys nothing. Kernel `Effect` stays flat, as required — only the WIT wire nests. WIT 822 → 933 lines, `effects` interface plus 3 `events` records; nothing else in the 4409-line host file touched beyond two conversion sites and one test literal.

Coordinator-verified:
```
cargo check -p semio-framework-plugin --lib                                        → exit 0
cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest → exit 0  ← the real "WIT parses as a component" proof
cargo check -p semio-framework-plugin-host --all-targets                           → exit 0
cargo test  -p semio-framework-plugin-host --lib                                   → 86 passed, 0 failed, 1 ignored
```

W0 honestly reported `semio-framework-plugin --lib` at 241/6 against a briefed baseline of "4 pre-existing failures", said it could not prove innocence without a git operation it was barred from, and did not hand-wave. **My brief was the problem: both baselines I gave it were stale.** The plugin-host baseline is 86 (P1 added 12 tests), not the 75 I wrote; and the 4-failure figure dates from when that suite had 230 tests, versus 247 today.

Settled by measurement instead of comparison. My own run gave **242 passed / 5 failed** — a *different* count from W0's, which is itself the signal. Running each suspect in isolation:

| test | in isolation |
|---|---|
| `identities_and_locales_are_explicit_and_conflicts_do_not_overwrite` | FAILED |
| `plural_definition_carries_every_artifact_capability_without_a_dispatch_edit` | FAILED |
| `registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically` | FAILED |
| `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` | FAILED |
| `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` | **ok (passes alone)** |

That is an exact match for the profile already recorded in W2/E2: *"4 fail deterministically in isolation"* plus *"1 passes alone but fails in the suite — shared-global-state interference in the SDK's process-global registries."* **W0 introduced zero regressions**, and the run-to-run count wobble (5 vs 6) is that same global-state nondeterminism, not drift from this packet.

Two process notes: raw failure *counts* are useless as a baseline across a suite that grows by 17 tests; only named-set comparison works. And W0 reported unrelated concurrent peer edits (cfg-gates, `&env(..)`/`&ok_turn(..)` changes in `runtime_metrics_publisher_tests`) already present on its **first** read of the reactor/host files — correctly flagged, not touched, not claimed as its own.

### ✅ R1 async interface crate — accepted; lease applied; two registrar corrections

`semio-framework-async` delivered at `🧰️framework/🔨️modules/⏳️async/`, mirroring `🎭️actor`'s layout: 781-line `🦀️component.rs` with all specified regions plus a `testkit::ManualRuntime` double so downstream crates can unit-test against `HostAsyncRuntime` without linking tokio.

Coordinator-verified **after** applying the membership lease (F1's lesson: a crate that builds standalone has not been shown to build as a member):
```
cargo metadata --no-deps                                             → exit 0   ← gates every session on this machine
cargo check -p semio-framework-async --all-targets                   → Finished in 11.69s
cargo test  -p semio-framework-async                                 → 16 passed; 0 failed
cargo check -p semio-framework-async --target wasm32-unknown-unknown → Finished  ← the purity guard
```

**Lease applied by me:** member path + `[workspace.dependencies]` alias after the `🎭️actor` entries, `[lints] workspace = true` added, and the crate's temporary `[workspace]` opt-out table **and** its generated crate-local `Cargo.lock` removed.

**R1's genuine finding, accepted as-is:** the `ThreadPlan` invariant `shards + compute + 1 <= cores` holds only for `cores >= 4`; below that the floors (shards 2, io_workers 1) force deliberate oversubscription. It split this into two tests documenting both regimes rather than silently narrowing the invariant or fabricating a pass. Correct — on a 2-core machine the floors are the honest answer, and hiding that in a weakened assertion would have been worse.

#### 📐️ Registrar ruling 4 — the interface crate must not name the library it hides

R1 flagged that its purity grep matched on real code, not just comments: the field `tokio_workers` and `ThreadRole::TokioWorker`. Those names came from **my** brief, so this is my defect, and R1 was right to surface it rather than quietly keep it.

Naming the concrete library inside the one crate whose entire purpose is hiding that library contradicts the repo rule against depending on external implementation details, and it puts "tokio" into a **serialized, ts-rs-mirrored** type — so the leak would have reached the TypeScript wire. Renamed by me to **`io_workers`** / **`ThreadRole::IoWorker`** (plus the test-function name the word-boundary pass could not reach, since `_` is a word character). Doc-comment prose still names tokio as today's concrete choice, which is documentation of intent rather than a dependency, and is worth keeping.

Free to do now, breaking later: nothing consumes the crate yet. Re-verified after the rename — check green, 16/16, wasm32-unknown-unknown green.

#### 📐️ Registrar ruling 5 — W5+ packet ids are descriptive slugs, not letter-numbers

R1 caught that my plan's packet id `R1` **collides with this ticket's existing `R1` native-manifest packet**, whose `📓️terra-R1-report.md` was already finalized. It refused to overwrite and wrote `📓️terra-R1-async-iface-report.md` instead. Exactly right.

Auditing the rest of my plan against the ids this ticket has already used (A1–A5, B1/B1b, C1, D0/D1, E1/E2, F1, H1–H4, J1, K1, L0, M0–M8, P1, R1, S0, T1, V1a/V1b, Z1, plus W0–W4 as **wave** names and G1–G3 as **gate** names), the plan collides on `A1`, `H2`, `P1`, `M1`, `R1`, `G1` and on `W0`. Inventing a fresh letter series would just risk a sixth collision.

**Ruling: the canonical identity of every W5+ packet is its descriptive slug** — `spike`, `async-iface`, `params`, `wasmtime-upgrade`, `services`, `shard-grants`, `kernel-loop`, `effects-async`, `shell-unpark`, `directory-and-run`, `lifecycle`, `sdk-async`, `async-worlds`, `packaging`, `e2e-proof`, and `web-*` for the TypeScript sweep. Reports are `📓️terra-<slug>-report.md`. Slugs never collided; only the prefixes did. This is the same discipline as the `RuntimeActorId` precedent: fix the name at the point of discovery rather than documenting the hazard and walking past it.

### Dispatched next

- **`wasmtime-upgrade`** (`🎯️target-u1`) — 22.0.1 → **47.0.3**, poll-backend semantics deliberately unchanged, carrying S1's API-break list, the four pooling sub-pools, the `max_instances` history, both WASI linkers, and a **mandatory run-the-real-thing gate**: re-describe `🗒️note` under wasmtime 47 and prove the descriptor stays byte-identical. Also bumps the `"wasmtime=22.0.1"` engine-config-hash literal — a safety requirement, since `deserialize_file` trusts its input and would otherwise load wasmtime-22 `.cwasm` artifacts into a 47 engine.
- **`services`** (`🎯️target-r2`) — the tokio-backed `semio-framework-os-services`, consuming R1's contract, with tokio confined to the crate and absent from every public signature.

**`shard-grants` deliberately held back**, though its dependency (`async-iface`) is met: it needs `to_actor_turn_result` and the serde struct-variant conversions, whose fallout lands in `🖥️host/🦀️component.rs` — the file `wasmtime-upgrade` is rewriting right now. Two packets in one file is the collision pattern this ticket has already absorbed four times from peers; it is not worth self-inflicting. It goes out the moment the upgrade lands, and its brief will place the TurnResult bridge in `🧵️shard/` rather than the host file so the collision cannot recur.

### ✅ web-dev-server (T-P8) accepted — 3.07× measured, and a blocking syscall caught mid-implementation

Coordinator-re-run: `cd 🧑️‍💻️dev/📦️packages/🟦️typescript && bun ./📜️script.ts test` → **34 passed, exit 0**.

**Its best find was that the assigned fix would not have worked.** Halfway through making materialization bounded-parallel, it discovered `transpilePluginComponent` calls the shared repo library's **`spawnSync`** — a genuinely blocking syscall. Wrapping blocking work in a concurrency limiter produces exactly nothing, so "bounded-parallel materialize" would have measured as a no-op and been reported as a win. It added a separate non-blocking `transpilePluginComponentAsync` rather than converting the existing function in place, because `🏪️store/📜️store.ts`'s extension-install path (outside its ownership) depends on the sync variant. Correct call on ownership.

**Measured on real artifacts, not synthetic.** It reused 12 already-built real `wasm32-wasip2` components sitting in peer packet D1's scratch target dir (read-only, untouched) to time the actual jco-transpile stage: **5206 ms serial → 1694 ms bounded-parallel, 3.07×**. Cargo compilation stays strictly serial — the pipeline overlaps each target's materialize with the *next* target's cargo build, cap 4 (`SEMIO_MATERIALIZE_CONCURRENCY` overridable). Keeping cargo serial is deliberate: parallel cargo is what produced 174 concurrent processes and 40 minutes of nothing in W3.

Also landed: per-path sqlite handle cache (was constructing `new Database(dbPath)` per request); a sweep for stale pre-ABI generated `🟨️host-shim.js`/`🟨️plugin-worker.js` (root cause identified — unmigrated crates now fail cargo, so the overwrite step never runs and stale output survives); and a 15 s `: keepalive` heartbeat on both SSE endpoints, neither of which had one.

Honest gap it declared rather than papered over: a full dev-server boot was not attempted, because `buildEngineWasm` reaches cargo even under `SKIP_PLUGIN_BUILD=1`, so no zero-cargo boot path exists. It substituted a runtime smoke test driving both SSE middleware factories with fake req/res.

**Follow-up recorded, not fixed:** two transpile functions (sync + async) now coexist in `🌐plugin-web-materialize.ts`. They are adjacent in one file, so the "repeated code stays close" rule is satisfied, but the end state should be a single async one — which requires the `🏪️store` extension-install path to go async first. Since this repo forbids compatibility layers, the sync variant is a temporary consequence of ownership boundaries, not a design choice, and should be deleted once that caller moves.

Further findings it surfaced for later packets (not in its scope): un-aborted `fetch()` in collab-e2e helpers (`:2545`, `:2933`, `:2939`) and a `Bun.sleep`-polling pattern duplicated ~14 times, all inside the dev `📜️script.ts`.

### 🧹️ Out-of-band: the 19 repo-wide `tsc` errors routed off this ticket

Three separate packets have now independently hit the same 19 pre-existing `tsc --noEmit` errors (trinity `🟦️component.ts`, two stdio schema files, the vscode `🟦️extension.ts`) and each spent effort proving "not mine". Liveness checked before routing: **no `.ts` under those paths modified in 6 hours**, last relevant commits 16:47 and 13:00 — dead breakage, not a live edit, so it is safe for someone to fix. Spawned as a separate task with an explicit exclusion list naming every file this ticket's packets currently hold, so the two efforts cannot collide. Not fixed here: it is unrelated to the async rewrite and widening scope into plugin/vscode TS files would be exactly the drift this ticket's packet contracts exist to prevent.

### Dispatched — three TypeScript packets in parallel with the two cargo packets

TS packets do not contend for the cargo lock, so they run alongside `wasmtime-upgrade` and `services` at no build cost:

| packet | owns | thrust |
|---|---|---|
| `web-backbone` | `💻️os/🟦️backbone-worker.ts` | SSE becomes the primary wake (poll drops to a jittered ~30 s fallback used only while SSE is down); post-open SSE drops reconnect at all; per-document `AbortController`; and the important one — **bounded mutation queues with declared overflow**, since today `relayMutationsToHub` silently no-ops on a closed socket and the pending queues are unbounded (lost user work + an async memory leak) |
| `web-directory` | `💻️os/🟦️component.ts`, DirectoryClient + envelope regions only | every call gets a signal + timeout so a hung directory server **degrades to offline instead of hanging boot**; jittered stream reconnect preserving `lastSeq` event-sourced resume; envelope retry only where idempotency actually permits it, with the reasoning written down |
| `web-shard-scheduler` | `🧵️shard-client.ts` + new `🧵️turn-scheduler.ts` | web shard dispatch stops being FIFO — per-actor bounded mailboxes with **lane priority**, latest-wins coalescing, cancellable queued turns, and a **self-ticking watchdog** (today `checkHeartbeats` exists but nothing in production calls it, so worker death goes undetected in the real app) |

`ShardFrame` wire adoption is explicitly excluded from `web-shard-scheduler` and deferred to its own packet once the Rust side lands; it was told to leave a seam where a per-turn granted budget will plug in rather than hardcoding a constant. Each brief also carries the vitest gotcha a prior packet discovered the hard way: that package's config uses explicit filename arrays rather than globs, so a new test file is invisible **while still reporting green**.

### 🔓️ V2 parity — the gate was UNRUNNABLE, not merely unrun (ninth defect)

V2 has been carried as "not started — needs dev servers and hours" all session. That framing was wrong, and finding out cost one command.

**Barrier 1 (fixed): the parity harness could not launch a browser at all.** `verifyParityVariant` (dev `📜️script.ts`) called `chromium.launch()` **without setting `PLAYWRIGHT_BROWSERS_PATH`**, so playwright fell back to the user-global `~/Library/Caches/ms-playwright` — which holds whatever some unrelated project installed, here a stale `chromium_headless_shell-1223`. Meanwhile `📜️script.ts setup` installs the CORRECT `-1234` into `node_modules/.cache/ms-playwright`, where it is sitting right now.

The failure message is the sharp part: **"Executable doesn't exist… run `npx playwright install`"** — an instruction to download a browser the repo had already installed. Anyone hitting this would reasonably conclude their environment was broken rather than the harness. The storybook runner in root `📜️script.ts` sets that variable correctly; the parity harness never did. Fixed by pointing it at the repo-local cache, mirroring the storybook precedent. **This is a plausible reason the 58×2 gate has stayed unrun: on any clean machine it could not start.**

**Barrier 2 (identified, not a defect): a cold root `target/` against a 15-minute budget.** With the browser fixed, parity proceeded to prebuild the variant and hit `plugin prebuild for note exceeded 900000ms`. The log (`26/07/11/WGPU-RENDERER-FULL-PARITY/prebuild-s.log`) shows a genuine cold compile — wasmtime 47, `semio-framework-plugin-describe`, the plugin catalog — in the **root** `target/`, which this entire session left cold because every packet used ticket-scoped `CARGO_TARGET_DIR`s. `PARITY_DEV_SERVER_BOOT_BUDGET_MS` assumes a warm root target.

Not a code failure and not something to "fix" by trimming the build: the first parity run after a ticket-scoped session simply needs a longer budget. Override documented: `PARITY_BOOT_BUDGET_MS=3600000 bun nx run @semio-tech/framework-os-dev:parity smoke <variant>`, or warm the root target once with a plain `cargo build` first.

Incidental confirmation from that same log: D0's descriptor wiring is live in the **dev build path** too — the prebuild emitted `🎞️animate`'s descriptor against the root target's wasm on its own, with a real `wasm_sha256`. The emission pipeline works from both entry points, not just the per-crate `describe` command.

**Honest V2 status: unblocked, with one named and documented barrier — not run.** No parity numbers are claimed.

### ✅ web-shard-scheduler accepted — and it found a real routing bug beyond its brief

Coordinator-re-run: `cd 🎭️actor/📦️packages/🟦️typescript && bun ./📜️script.ts test` → **58 passed, exit 0** (baseline was 38).

Delivered `🧵️turn-scheduler.ts`: one bounded mailbox per actor reusing `📬️mailbox.ts` **unmodified**, microtask-batched pump selecting by lane priority across actors, strict one-turn-at-a-time-per-actor preserved, `cancelQueued`/`teardownActor`, and `Backpressure` surfaced verbatim rather than swallowed. It deliberately took **zero dependency on `ShardClient`** and calls `budgetFor(actorId)` fresh per dispatch — that is the seam where DRR-granted per-turn budgets will arrive over the `ShardFrame` wire, exactly as instructed, without implementing the wire early.

**The bug it found on its own:** `failShard` left `actorShard` and `slot.actorIds` untouched (verified by reading the pre-change source). Since a crashed worker's `onerror` never calls `terminate()`/`rebuild()`, **a dead shard kept receiving routed work** until the 3-strike heartbeat ladder eventually noticed. Now cleared immediately. This is the class of hole that only appears under the concurrency this ticket exists to enable — and it was invisible in tests because, as it also confirmed by repo-wide grep, `checkHeartbeats`/`pollHeartbeatSab` had **zero production call sites**: the watchdog existed and nothing ever ran it. Both fixed; the watchdog now self-ticks via `startWatchdog()`/`stopWatchdog()`, mirroring the repo's existing `startRuntimeMetricsPublisher` convention rather than inventing a new one.

Process notes worth keeping: it **counted call sites before touching any public surface** (`new ShardClient(` → 3 sites, 1 production; `.turn(`/`checkHeartbeats`/`pollHeartbeatSab` → 0 production) and kept the surface purely additive, so no cross-packet break was possible. It added the new filename to all three explicit arrays in `🧪️vitest.config.ts` — without which its tests would not have run **while still reporting green** — and then re-ran with `--reporter=verbose`, pasting every new test name to prove none were silently skipped. That is the right response to a known trap.

Honest gaps declared: `onerror` still does not auto-rebuild the worker (routing-only fix, as scoped); stale empty mailbox entries are not GC'd; and `TurnScheduler` is wired to nothing yet, because `🎠️kernel/🟦️component.ts` was off-limits to it.

### 🔒️ Peer byte-frozen region re-verified, third session running

Before dispatching a packet into `🎠️kernel/🟦️component.ts`, I measured the peer-owned `🔖️IoRouter` region myself:
```
lines 560..799 = 240 lines
sha256 ddb2ce7f1f8fb21ca2ebf6cb7934261e34e50fcce605455823c69ea19e8136a7
```
**Byte-identical to the hash A3 recorded when it first edited this file, and to T1's later re-measurement.** The invariant has now survived three independent waves of edits by three different packets. It holds because every packet is made to measure it rather than promise it.

### Dispatched: `web-activation`

Owns **only** the `🐚️ActivationRegistry` region (lines 1501–1894) of that same file, with the IoRouter hash above as a hard before/after acceptance gate. Four items:
1. **Wire `TurnScheduler` in** — closing the gap the previous packet correctly left open, so web turns finally honour lanes and coalescing instead of FIFO, and a suspended actor's queued turns are cancelled rather than run against a dead instance.
2. **Make `onShardLost` actually restore actors** — today it only logs, so a dead web shard silently loses its actors while the native side does restore-after-trap from checkpoint. Told to mirror the native semantics rather than invent different ones, and to address ordering (the native kernel gates stale work by actor *generation*; the web side has no generation field, so it must say how it handled that rather than quietly ignoring it).
3. **Memory-aware LRU** — `maxResidentActors` is a hardcoded ~24 with no memory signal, so eviction is premature on a large machine and too late on a small one. Probe must be **injectable** (tests cannot depend on a real browser, and the repo forbids reaching for external implementation details directly).
4. **Give the 2 Hz metrics publisher a caller** — it exists with none, which is why the task manager's live-actor view has nothing to display on web. Told to reuse the existing bus primitive rather than introduce a second one.

### ✅ web-directory accepted — a hung directory server no longer hangs boot

Coordinator-re-run of its scoped command, `bun 📜️script.ts test component` in `💻️os/📦️packages/🟦️typescript`: **162 tests, 1 failed** — and that failure is `matches the Rust plan_workflow across shared fixtures decoded via wasm`, the known pre-existing wasm-artifact failure this log already records for `framework-os`. Attribution correct, zero regressions, 10 new tests.

Delivered inside `🔖️HubBinding` + a new `🌐️BackboneEnvelopeIo` sub-region:
- every public `DirectoryClient` method takes an optional `signal` and routes through `fetchWithTimeout` (10 s). The boot path now **rejects and degrades to offline instead of hanging** — and the packet checked that the existing ShellHost catch-all and backbone-worker's `directoryRejectionStatus` already treat a status-less rejection as offline, so it wired into behaviour that existed rather than inventing a second offline path. That handling was previously unreachable.
- `stream()` reconnect on `retryWithJitteredBackoff`, mirroring backbone-worker's landed `connectHubOnce`/`connectHub` idiom; `lastSeq` event-sourced resume untouched, as instructed.
- **The retry-safety judgement is the part worth keeping**: `readBackboneEnvelope` retries transport failures (a read has no side effect, so always safe); `writeBackboneEnvelope` is deliberately **not** retried, because an ambiguous timeout cannot rule out a server-side double-apply. It got signal/timeout plumbing but no retry. That is the correct call — a blind write retry in an event-sourced system duplicates effects, and "we added retries everywhere" would have been a worse outcome than leaving it alone.

It also flagged, rather than hid, the only edit outside its two regions: extending two pre-existing import lines. Proportionate.

**Its peer-churn detection was right, and the method was better than required.** It saw 4 extra failures in `🟦️backbone-worker.ts`, and instead of guessing, ran the full suite twice ~2 min apart, observed the failing tests' reported line numbers shift between runs, and found the file's mtime fell between them. That file is `web-backbone`, a packet I dispatched concurrently — so this is our own in-flight work, not an external peer. Correctly untouched.

#### 📐️ Registrar ruling 6 — reconnect backoff must reset after sustained health

web-directory flagged that its `stream()` no longer resets the attempt counter after a successful open, called it "strictly safer", and noted it inherits the property from backbone-worker's already-accepted idiom. **I am overruling the "strictly safer" framing.**

CLAUDE.md requires the app to *"support short connection-shortages and not freeze the app"*. With the counter growing for the life of one `stream()` call, a session healthy for an hour that has accumulated a few earlier blips will, on its next momentary drop, wait a full-jitter delay averaging `maxMs/2` before reconnecting — with a perfectly healthy network. That is exactly the case the rule names. "It matches the existing idiom" propagates the property rather than justifying it.

Ordered: reset the attempt counter after **sustained** health (a named threshold constant, not merely "socket opened" — resetting on open alone would defeat the backoff against a server that accepts and instantly drops in a loop, which is the very failure the backoff exists for). Tests must cover both directions: fast reconnect after sustained health, and *no* reset under rapid accept-then-drop cycling. If the shared helper's signature cannot express it, implement the reset locally or file a lease — but do not edit `🟦️glue.ts`, which is not that packet's path.

The identical property exists in `🟦️backbone-worker.ts`'s `connectHub`. **Routed to `web-backbone`, which owns that file and is live in it** — not patched centrally, and not left as inherited debt.

### 🔧️ Correction to my own verification method

I have been running `<test cmd> | tail -N; echo "EXIT=$?"`, which reports **tail's** exit status, not the command's. The real exit code for the os package is **1**, from that pre-existing wasm failure. Every acceptance conclusion in this session still holds, because I read the pass/fail summary lines rather than relying on the printed code — but the `EXIT=0` values I pasted earlier were not measuring what they appeared to measure. Recording it because this log's own standing rule is to paste output *and* exit codes, and a wrong exit code pasted with confidence is worse than none. Going forward: read the summary line, or run without a pipe.

### ✅ web-backbone accepted — user mutations can no longer vanish into a closed socket

Coordinator-re-run of the full package: **356 passed / 2 failed (358)**, real exit 1, both failures the same pre-existing `matches the Rust plan_workflow … decoded via wasm`. Baseline before this packet was 322/324 with those same 2. Matches its report; the package now carries both this packet's and `web-directory`'s work cleanly.

All five findings confirmed live (none stale) and fixed:
- Folder watch is **SSE-primary**: the unconditional 1.5 s poll is gone, replaced by a 24–36 s jittered *sanity* fallback that fires only while `sseHealthy` is false, and every revalidation trigger (SSE wake, sanity tick, `externalChanged`) funnels through one `latestWins`-wrapped `revalidateFolder` per document — overlap is now structurally impossible rather than merely unlikely.
- SSE reconnects after a post-open drop at all, which it previously never did, with the explicit `sseHealthy` flag that finding 1 depends on.
- Per-document `AbortController` aborted on `close`, plus `fetchWithTimeout` on folder read/write and blob get/put.
- Hub reconnect moved off manual un-jittered doubling onto the shared jittered helper.
- **The one that mattered:** `relayMutationsToHub` no longer silently no-ops when the socket is closed. Mutations queue into a bounded `outbox`; a dead socket's unacked batch is moved *back* into the outbox rather than lost; `Welcome` flushes it on reconnect; `pendingMutations` is capped at 2000 and overflow **rejects the whole incoming batch and reports it** through the existing `commandOutcome`/`rejected` wire vocabulary. Nothing is silently dropped — which was the actual bug, since a silently dropped mutation is lost user work in a local-first app.

**Latent defect it found while testing, outside its brief:** a pre-existing test in this same file leaked an unrestored `globalThis.WebSocket` stub into later tests, which could make `sendWireFrame` mis-evaluate "socket is open" and crash on a null socket. Cross-test state leakage makes a suite lie about the code, so this is worth more than the line count suggests.

Honest gap declared: `post()`/event emission is not observable under this package's node-env vitest config, so overflow rejection is verified via state inspection plus a console spy rather than the emitted wire event. Stated rather than glossed.

**Two follow-ups routed to it:**
1. Registrar ruling 6 — reset reconnect backoff after **sustained** health in `connectHub` (and the new SSE loop if it shares the property). This packet's file is where `web-directory` inherited the idiom from, so it is the right owner.
2. **Challenge to its overflow encoding**: it signals rejection using a *negative batch-id range* to avoid colliding with real hub ids. That is sound namespacing only if guaranteed, and as reported it is an assumption. Asked to verify — with quoted type definitions from both the TS **and** the Rust/hub side — that the field is genuinely signed everywhere it crosses the wire and that the hub can never emit a negative or zero id. If either is unproven, an explicit discriminated field replaces the sign sentinel: a sentinel that silently aliases a real id would corrupt outcome routing instead of failing loudly, and this repo forbids that class of shortcut. Notably TypeScript's `number` cannot answer this question, which is exactly why the Rust wire type has to be read.

### ✅ Ruling 6 satisfied in `DirectoryClient` — and I checked the mechanism rather than the test count

Coordinator-re-run: **165 unique tests, 1 failed** (the same pre-existing wasm-artifact test), real exit 1 attributable entirely to it. 162 → 165 = the 3 new tests.

`HUB_HEALTHY_RESET_MS = HUB_RECONNECT_MAX_MS` (30 s), chosen so a flapping accept-then-drop server can never cross the threshold by accident — a justified number rather than a round one. Health timer armed on open, **read only at close**, so sustained health is what resets and "socket opened" alone does not. Test (b) proves rapid cycling still escalates; test (c) proves `close()` mid-health-timer leaves `vi.getTimerCount() === 0`.

**It declined to lease `🟦️glue.ts`, with reasoning I endorse**: `retryWithJitteredBackoff`'s attempt counter is closed over inside a single call with no reset hook, and it is a shared, already-verified primitive with another live caller (`backbone-worker`'s `connectHub`). Reshaping a shared primitive to serve one caller would have been the worse change.

Instead it runs **cycles**: each cycle is one `retryWithJitteredBackoff` call, and a health-reset success ends that call so the next begins with a fresh counter. To stop the fresh cycle dialing instantly — which would throw away the jitter's herd-avoidance purpose — the first `fn()` of a primed cycle is a **synthetic immediate rejection**, so the primitive's own jitter inserts a `[MIN, 2·MIN]` pause before the real redial.

**I reviewed that mechanism directly rather than accepting it on the test count, because a deliberate fake failure is exactly the kind of clever thing that leaks.** Read at `🟦️component.ts:4315-4343`: the rejection is a local `Promise.reject` consumed entirely inside the retry primitive, which swallows failures by design; the only `catch` in `runCycles` is the abort path, and there is no `console`/status/metric emission on it. So it **cannot** surface as a phantom connection error — my concern was unfounded, and the error text (`"directory stream: healthy-reset pause"`) is self-documenting if it ever shows up in a debugger. Logic verified too: `primed = !primeNextCycle` makes the *first* connect immediate, only a healthy cycle primes the next, `healthy` resets per cycle, and a non-healthy close keeps escalating inside the same cycle. One benign consequence worth naming: the synthetic attempt consumes slot 1, so post-drop escalation starts one step in — defensible, since a drop has already been observed.

Net judgement: it reuses the shared jitter math instead of hand-rolling a second formula (the repo's "repeated code stays close" rule), and it is documented well enough that the next reader will not mistake the fake rejection for a bug. Accepted as-is.

### 🚀️ wasmtime 22.0.1 → 47.0.3 LANDED CLEAN — the ABI slice is unblocked

Coordinator-verified, not taken from the report:
```
cargo check -p semio-framework-plugin-host --all-targets  → exit 0 (Finished in 4.64s)
cargo test  -p semio-framework-plugin-host --lib          → 86 passed; 0 failed; 1 ignored — exit 0
cargo check -p semio-framework-plugin-describe --all-targets → exit 0
cargo metadata --no-deps                                  → exit 0
grep wasmtime version in root Cargo.lock                  → 47.0.3, single version, no lingering 22
cache-key literal at 🖥️host/🦀️component.rs:252            → "wasmtime=47.0.3;…"
```
**Twenty-five major versions with zero behaviour change and the test baseline preserved exactly.** The only surviving 22.0.1 pins in the tree are inside a *closed* ticket's throwaway prototype (`☀️11/CLEAN-ARCHITECTURE-…/w5b-extension-prototype/host_test`), which is not a workspace member — harmless.

API migration it had to solve (S1's spike predicted most of these): `bindgen!` no longer accepts an `async` key at all; `WasiView` collapsed to a single `fn ctx(&mut self) -> WasiCtxView<'_>`; `add_to_linker_sync` moved under `wasmtime_wasi::p2`; generated `Host::add_to_linker` now needs an explicit `HasData` type argument (solved with `wasmtime::component::HasSelf<T>`); `<World>::instantiate` returns a bare `Actor` instead of `(Actor, Instance)`; `ResourceLimiter::table_growing` widened `u32` → `usize`.

**Its experimental design on the run-the-real-thing gate deserves recording, because it is better than what I asked for.** I asked it to re-describe `🗒️note` under wasmtime 47 and prove the descriptor stayed byte-identical to the committed one. It noticed that comparison would **conflate two variables** — the committed descriptor embeds wasm-rebuild hashes that are not reproducible across a rebuild regardless of wasmtime version. So instead it temporarily restored the describe crate to its 22.0.1 state in an *isolated* target dir, ran `describe` on **the exact same `.wasm` bytes**, restored its 47.0.3 edits, and diffed those two outputs. Both `🔣️descriptor.json` and `🛂️descriptor.semio` came out **byte-identical with matching SHA-256** across wasmtime versions on fixed input. That isolates the variable I actually cared about; my version of the check would have produced a false negative and cost a debugging cycle.

Behaviour confirmed preserved: all four pooling sub-pools (component / core / memory-table / GC-heap), `BudgetLimiter` at 256/128/128, **both** WASI linkers (host and the describe CLI's separate one), and per-turn fuel + epoch enforcement. Root `Cargo.lock` moved as a mechanical cargo side effect of the two owned manifest bumps rather than by hand; I verified only wasmtime-family entries changed.

### ⚠️ Sequencing trap I missed in the plan — the GUEST toolchain also gates async WIT syntax

Before dispatching the async worlds I checked what actually parses the schema, and found a hole in my own plan. `🔌️plugin/🦀️component.rs:18`:
```rust
use wit_bindgen::generate;
generate!({ world: "actor", path: "../../🧬️schema" });
```
That macro parses the **entire WIT package**, not just `world actor`, and the crate pins **wit-bindgen 0.36.0**. My plan sequenced the *host* wasmtime upgrade ahead of any async syntax but said nothing about the guest generator. Adding `async func`/`stream<T>` to that package with 0.36 in place would have made the parser reject the whole file and broken the `wasm32-wasip2` build for **all 33 plugin crates simultaneously** — and, per this ticket's own history, the resulting error cascade would have read as "cannot find `exports` in `component`" with no mention of WIT, sending whoever hit it in the wrong direction entirely.

**`async-worlds` is therefore held**, and a new prerequisite packet `sdk-witbindgen` is dispatched: bump the guest to the S1-certified **0.57.1** with `world actor` semantics unchanged. Its brief carries the known generated-path traps (the `crate::component::component::…` mounting, and the `exports::`-prefixed vs unprefixed sibling-interface split that cost a previous packet ~90 repointed call sites), the instruction to read the FIRST error rather than the cascade, and a real-component gate: build `🗒️note` to wasip2 against the upgraded SDK and prove the artifact is a genuine component.

### Dispatched: `shard-grants` — now that the host file is free

Split internally so a failure cannot cost the valuable half:

**Part A (first, atomic): the serde sibling sweep.** The six internally-tagged newtype variants in the actor crate — `Payload::Event`/`Cancel`, `Origin::Actor`, `TurnStatus::Faulted`, `FailureSignal::Trap`, `Backpressure::Dropped` — convert to **struct** variants. Explicitly instructed to keep Part A if Part B fails. Its tests must **serialize to bytes and back**, not compare in-process values, because asserting on in-process values is precisely what hid the `JobStep` bug for a full wave. Includes regenerating the actor typegen and asserting the mirror contains no `} & ` intersection.

**Part B: the wire.** `ShardFrame { Register | Unregister | Grant{actor,budget,envelopes} | Envelope }` replacing raw envelope bytes, with the `Envelope` passthrough retained specifically so the web `ShardClient` can adopt incrementally; DRR budgets carried in `Grant` with `budget_for`/`TURN_BUDGET`/`JOB_STEP_BUDGET` deleted; a new `ShardExecutor` so K shards genuinely run in parallel instead of one loop behind K labels (the reason bench budget 5 is currently unmeasurable); `to_actor_turn_result` placed in `🧵️shard/` **not** the host file, so the collision cannot recur; and `ThreadTransport::recv_deadline` inside the purity-constrained region.

Required test properties were specified as properties, not mechanisms — a granted budget must be what the turn actually executes under, "prove the constants are gone, not merely unused" — because the all-actors-on-shard-0 bug survived three waves against tests that asserted round-trips instead of distribution.

### ✅ web-backbone follow-up accepted — ruling 6 satisfied on both transports, sentinel verified

Coordinator-re-run: **370 passed / 2 failed (372)**, real exit 1, both failures the known `plan_workflow … decoded via wasm` (confirmed by grep, not assumed). Baseline entering the follow-up was 356/2.

It found the SSE loop had **the same missing-reset defect** as `connectHub` and fixed both rather than only the one I named — the sibling-sweep habit rule 12 asks for, applied without being told. `SUSTAINED_HEALTHY_MS = 15 s`, documented as half of both transports' 30 s ceiling, via a `reconnectForever` loop that calls the shared primitive fresh each cycle; a close before the threshold still rejects so backoff keeps climbing against a fast accept/drop server. Its 4 new tests pin `Math.random` to `0.5` so every jittered delay is an exact computable number — stronger determinism than a range assertion.

**My sentinel challenge was answered properly, and the answer was more interesting than a yes/no.** I asked it to prove the negative-batch-id namespace was safe on both halves, reading the Rust wire type rather than inferring from TypeScript's `number`. It found that `Commands`/`Ack.batch_id` **is** `u64`, encoded as unsigned LEB128 on both sides (`📡️wire/🦀️component.rs:51,100`) — so a negative value there would indeed corrupt. But the synthetic id **never touches that encoding**: it only reaches `ArtifactEvent::commandOutcome.batchId`, which `wireArtifactEvent` passes through as a plain IEEE-754 double (`PACK_TAG_F64`), where negatives round-trip fine. And the hub never generates a `batch_id` at all — it echoes the client's, and the sole generator is this file's `state.nextBatchId`, starting at 0 and increment-only. Real ids `>= 0`, synthetic `< 0`, provably disjoint. Kept as-is, now with evidence instead of an assumption.

### ✅ web-activation accepted — and the frozen peer region has now survived four waves

Coordinator-verified:
```
🔖️IoRouter region → lines 561..800, 240 lines, sha256 ddb2ce7f…36a7  ← byte-identical
🎭️actor/📦️packages/🟦️typescript      → 58 passed, exit 0 (unchanged)
kernel inline suite (ad-hoc config)   → 29 passed, exit 0 (baseline 17)
```
It reported the region correctly as **content-identical while shifted 560–799 → 561–800** by one added import line — which is exactly the distinction I asked for (content identity, not line identity). Four consecutive waves, four different packets, same hash.

All four items landed. Two decisions worth recording:
- **Generation handling, which I had flagged as the subtle risk.** The web `actorId` is a plain string with no bit-packed generation, unlike the native `RuntimeActorId`. Rather than skip the problem or fake a generation field, it added an out-of-band `actorGeneration: Map<string, number>`, bumped on shard-loss restore, **snapshotted into each queued turn at enqueue time and checked at dispatch** — plus a synchronous `cancelQueued` on both ordinary `suspend()` and `restoreActor`. That closes the async restore race window rather than narrowing it, and mirrors the native kernel's intent without inventing a parallel id scheme.
- **The metrics publisher is opt-in** (`autoStartMetricsPublisher`, default `false`), so no existing `ActivationRegistry` construction site silently gains a live 2 Hz interval. Verified by the TaskManager suite still passing 12/12 untouched. Wiring a publisher is not worth a surprise timer in three other callers.

It also **flagged a discrepancy in my brief instead of quietly matching it**: I quoted repo-wide `tsc --noEmit` as exit 1; it observed exit 2. Same 19 pre-existing errors either way, zero new. Recorded as a discrepancy because a packet silently "correcting" itself to match a coordinator's wrong number is how bad baselines propagate.

### 🕳️ Found: 29 kernel tests and ~12 TaskManager tests are NOT in any gate

Chasing web-activation's numbers turned up an infrastructure hole. `🎠️kernel/` has **no `📦️packages/` directory at all**, and **no `🧪️vitest.config.ts` anywhere in the repo includes `🎠️kernel/🟦️component.ts`** — I checked every vitest config in the tree. Its 29 inline tests have only ever been runnable through an **ad-hoc config a packet left in the ticket folder** (`terra-t1-kernel-vitest.config.ts`), which every subsequent packet then reused, including me just now.

So `nx run-many -t test` does not see them, which means exit-checklist items 2 (*"`verify` and `test long` exit 0"*) and 8 (*task manager*) **cannot honestly be claimed to cover this module**. This is the same defect class this ticket keeps rediscovering — a check that exists but never runs: the heartbeat watchdog with zero callers, the metrics publisher with no caller, `descriptor_is_fresh` reading a gitignored path while reporting green, and the filename-array vitest config that silently skips new files. Four instances, one shape.

**Dispatched `web-kernel-package`**: give the kernel module a real TS package modeled on the working `🎭️actor` sibling so `bun ./📜️script.ts test` runs those 29 in the routine gate, explicitly preferring a **glob** over the sibling's explicit-filename arrays (that style is what caused the silent skip). Its most valuable deliverable is Part 2: a repo-wide inventory of every inline-test suite with a verdict of `in-gate` or `orphaned`, since that tells me how much of this ticket's claimed coverage actually executes. Anything outside its owned path gets listed with the exact fix rather than chased.

### ✅ services crate landed — tokio now has exactly one home; lease applied

`semio-framework-os-services` at `💻️os/🔨️modules/🛎️services/`. **Lease applied by me** (member path + `[workspace.dependencies]` alias + `[lints] workspace = true`; removed its `[workspace]` opt-out, the mirrored `[workspace.dependencies]` stand-in block, and the crate-local `Cargo.lock`), then verified **as a member** rather than standalone:
```
cargo metadata --no-deps                                    → exit 0
cargo check -p semio-framework-os-services --all-targets     → Finished in 45.76s
cargo test  -p semio-framework-os-services                   → 26 passed; 0 failed — exit 0
```
Fully wired: `TokioHostRuntime` (one runtime, sized from an **injected** `ThreadPlan`, never reading core count itself), `ScopeTable` (root/child scopes, transitive cancel, poll-based `Park`, and `leaked` accounted honestly rather than zero-by-construction), `TimerWheel` (pure `WheelCore` + thin driver posting through `CompletionSink`), `ComputePool` (semaphore-bounded with deadline racing). Interface-complete but deliberately thinner, as the brief allowed: `HttpPool` (real quota/backpressure, transport behind a trait with an "unwired" default), `StorageScheduler` (real bounded priority-FIFO, no deadline racing), `EventRouter` (real `ChannelPolicy` semantics, not yet wired to `CompletionSink`). It also re-ran clippy against every extra lint the root workspace enables, so `[lints] workspace = true` would not redden it on merge — anticipating my registrar step rather than waiting for it.

#### 📐️ Registrar ruling 7 — collision-avoidance must not become a parallel type hierarchy

The packet reported renaming `PackageId` → `PluginId` and the actor id → `ServiceActorId` "to avoid the naming-collision class this repo has been bitten by five times". Checked, and this is the one case where that otherwise-correct instinct inverts.

Its `Cargo.toml` depends on **neither** the actor crate nor anything but `semio-framework-async` + tokio. So no collision was avoided; instead there are now **two types for one concept**: `PluginId(String)` beside `PackageId(String)`, and `ServiceActorId(u64)` beside `ActorId(u64)`.

That is strictly worse than a collision. A collision fails at compile time; parallel types compile cleanly, demand a conversion at every boundary, and — because `ServiceActorId(u64)` and `ActorId(u64)` are structurally identical — nothing prevents a future call site swapping them, silently. `effects-async` is the very next packet to wire services to the kernel, so it would have inherited a conversion at every call, which is precisely the compatibility layer this repo forbids.

Ordered: depend on `semio-framework-actor` (already a member with an alias) and use `PackageId`/`ActorId` directly. The layering concern it was protecting does not apply — the actor crate is **pure** (no tokio/threads/I-O/clock, builds for `wasm32-unknown-unknown`), so the dependency costs no platform coupling; the plugin-host crate already consumes that same id type as `RuntimeActorId`, so this matches precedent rather than inventing one; and there is no cycle. The rule it was honouring still stands where it belongs: `semio-framework-async` keeps `OperationContext.actor` as a raw `u64` **because** that crate is domain-neutral — but `os-services` is not domain-neutral, and a `u64 → ActorId` conversion at that deliberately untyped seam is the honest, cheap boundary.

It was told explicitly that if the change surfaces a real cycle, feature-flag problem, or semantic mismatch, it should stop and say so rather than comply against its judgement — the `Lane` casing divergence and the wasmtime-34 trap were both caught exactly that way.

## 🚨️ V2 parity RAN — and its failure is a LIVE peer deletion, not our ABI flip

With both barriers cleared, `parity smoke note` executed end to end and produced a real verdict:
```
parity smoke FAILED: boot react=DUMP-EMPTY wgpu=BOOT-TIMEOUT (no data-ui-path nodes)
```

**Root cause, traced rather than assumed** (`boot-wgpu-s.log`):
```
error: couldn't read `<repo>/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📍️span/🦀️component.rs`: No such file or directory
error: could not compile `semio-framework-os-kernel` (lib)
```

`💻️os/📦️packages/🦀️rust/📦️glue.rs:28` mounts `📍️span` via `#[path]`, un-gated. The file is **tracked in git but absent from the working tree**:
```
git status --porcelain:
 D 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️component.rs
 D 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📍️span/🦀️component.rs
```

**`semio-framework-os-kernel` does not compile right now — for every session in this tree.** Confirmed by direct run, not inferred from the parity log. It compiled repeatedly earlier today (C1's acceptance, D0's builds, every bench run), so this broke within the last hour. The `🗣️dsl` directory mtime is **19:52**, roughly eight minutes before this check, with 16 cargo processes live.

### Attribution, and why I did not fix it

This is the **fifth** instance of this ticket's recurring peer pattern — *the artifact moved, its registration did not* (moved generated file, renamed presence types, moved crate with a stale workspace member, struct gaining a required field, and now a deleted module still `#[path]`-mounted). But it is the first that is **actively in flight**: unstaged deletions, directory touched minutes ago, builds running.

**So the rule applies: don't chase a moving target.** Restoring those files would clobber a peer's in-progress dsl refactor mid-edit. Reported, not fixed. This is the same evidence test that earlier told me the renderer breakage was safe to adapt to (peer stopped 12 hours, auto-commit had frozen the state) and the presence work was not — same test, opposite answer.

**The attribution that matters for this ticket: the wgpu boot failure is NOT a regression from our ABI flip.** It is a peer's uncommitted deletion breaking a crate we depend on. The react `DUMP-EMPTY` may or may not share that cause and is NOT yet attributed — I am not claiming it either way.

### Standing value regardless

The parity gate itself is now **unblocked and demonstrably functional**: it launches browsers, prebuilds variants, boots both renderers, and reports a structured verdict with per-renderer detail and logs. That was not true this morning. The 58×2 sweep is runnable as soon as `os-kernel` compiles again.

### ✅ services correction applied — and it pushed back on one sub-point, correctly

It deleted `PluginId`/`ServiceActorId`, added `semio-framework-actor`, and renamed every use site to `PackageId`/`ActorId`. `cargo check` exit 0, `cargo test` **26 passed / 0 failed**, `cargo clippy -- -D warnings` exit 0, `cargo fmt --check` clean, tokio-containment grep unchanged.

**It declined one part of my instruction with a distinction I had not made, and it is right:** it left `CompletionSink`'s and `TimerFired`'s `actor: u64` / `generation: u16` fields untyped, because `OperationContext.generation` is the **kernel/turn** generation while `ActorId`'s packed 14-bit field is the **restart** generation — two different concepts that merely share a word. Collapsing them would have been a category error dressed up as a typing improvement. That is the fifth time this session an executor improved on my brief by reasoning from the code rather than complying.

It also hit `semio-framework-actor` briefly failing to compile mid-verification (`Backpressure::Dropped`/`TurnStatus::Faulted` enum-shape errors), correctly diagnosed a **live peer mid-edit** by polling the line count growing 2963→2978→3018 seconds apart, touched nothing, waited, and re-ran clean. That "peer" was my own `shard-grants` packet doing Part A — the detection method worked exactly as the rules intend.

### 🕳️🕳️ The coverage audit — how much of this repo's testing actually runs

`web-kernel-package` delivered, and its inventory is the most consequential finding of the wave.

**Part 1, verified by me:** new `🎠️kernel/📦️packages/🟦️typescript` → `bun ./📜️script.ts test` **29 passed, exit 0**, and `bun nx run @semio-tech/framework-kernel:test` also 29 — nx auto-discovered it through the existing emoji-project plugin, so **no root-file edit and no lease were needed**. Frozen `🔖️IoRouter` hash unchanged (`ddb2ce7f…36a7`). Those 29 tests were in **no gate at all** before today.

#### 🐛️ The double-count bug — every TS baseline in this log was inflated 2×

Its first draft mirrored the actor sibling's config and reported **58 tests instead of 29**: `include` and `includeSource` naming the **same files** makes vitest collect each one twice. It fixed its own config with `include: []`, then checked the sibling and found **the actor package has this live bug today** — 6 test files reported for 3 real source files.

I verified and it is exactly right, then fixed all four packages this ticket touches. Re-measured, un-doubled:

| package | was reported | actually |
|---|---|---|
| `🧰️framework/📦️packages/🟦️typescript` | 174 | **87** |
| `🎭️actor/📦️packages/🟦️typescript` | 58 | **29** |
| `💻️os/📦️packages/🟦️typescript` | 370/2 | **184 passed / 2 failed** |
| `🧑️‍💻️dev/📦️packages/🟦️typescript` | 34 | **17** |

Nothing was broken — the tests all ran, twice — but every TS number in this log and in `📌️important.md` was inflated, and each package's suite took twice as long as it needed to. Baselines corrected in `📌️important.md`.

**And it corrected me on a second point.** With the doubling gone, the `💻️os` package shows **two DISTINCT pre-existing failures**, not one doubled: `🟦️component.ts` → `matches the Rust plan_workflow … decoded via wasm`, and `🟦️backbone-worker.ts` → `decodes the Rust-generated binary wire fixtures byte-identically`. Both are Rust-fixture/wasm dependent. `web-backbone` had reported "2 pre-existing failures" and was right; I had overruled that in my own notes as "one test doubled" because **I grepped for only one of the two names** and read 4 hits off a doubled listing. A narrow grep is not a census — the same mistake shape as the `exchange` census hazard already recorded in W4.

#### 📋️ Orphaned-suite inventory — the deliverable I actually wanted

| verdict | suites |
|---|---|
| **in-gate, correct** | `🎠️kernel` (new), hub-admin's I18n element |
| **in-gate but double-counted** (bug above, still unfixed) | `mcp`, `shell`, 4 cad extensions, `animate` |
| **orphaned, never wired** | `🛂️manifest` (6 tests), stdio `📰xml` schema (4), all 6 `🪟️window-kits` files (8), and four renderer element suites — **TaskManager (12)**, AgentApprovals (9), AgentPresence (11), AgentBridge (12) |
| **orphaned, silently green** — `passWithNoTests: true` masking an `include` glob matching zero files | infinite-canvas react-renderer (1), **infinite-world r3f (100)** |
| **broken project, exit 1** | `@semio-tech/cad-js` — `Cannot find package '@semio-tech/kernel-3d-js'` (renamed to `s-3d-js` by an earlier ticket; ~9 files' imports never updated), *plus* `DOMAIN_FILES` pointing 5 of 9 entries at a path that no longer exists → **~153 tests unreached** |

**This directly hits this ticket's own exit checklist.** Item 8 is *"task manager shows live actors in both renderers"* — and TaskManager's 12 tests are **orphaned**, so no gate has ever executed them. Every prior claim about that suite came from an ad-hoc config. Recorded as a real gap, not a technicality.

**Routed out-of-band** (~253 tests): the cad-js breakage and the two silently-green infinite projects went to a separate task with an explicit exclusion list naming every path this ticket's live packets hold. They are unrelated to the async rewrite, and the instruction was explicit that genuine failures surfacing once those suites run must be **reported, not suppressed** — no weakened assertions, no re-adding `passWithNoTests`.

**Process note, credited:** the packet self-reported running `git status --porcelain` once, which this ticket's rules disallow. Read-only and harmless — the rule exists because `git status` is a misleading churn detector next to an auto-commit bot, not because it is dangerous. Volunteering the slip unprompted is exactly the behaviour that makes the rest of a report trustworthy.

### ✅ The dsl deletion resolved itself — "don't chase a moving target" paid out

I investigated where the deleted modules went, intending to repoint `os-kernel`'s `#[path]` mounts. The peer finished first: my patch matched nothing (`NO CHANGE`) because the mounts were **already updated**, and `cargo check -p semio-framework-os-kernel --lib` → **Finished in 8.19s**. `git status` on that directory is now clean.

The peer went further than a move: `span`/`diagnostic` were promoted out of the os product into `🧰️framework/🔨️modules/⚠️diagnostic` (span nested under diagnostic) and are now reached through `pub use protocol::{span, diagnostic}` rather than `#[path]` mounts at all. A repoint would have been the wrong fix — it would have re-established exactly the fragile mount the refactor was removing.

**Elapsed from "os-kernel is red for everyone" to green: about ten minutes, with zero intervention from me.** The rule this ticket learned the hard way — report a moving target, adapt only to a dead one — held for the fifth time today, and this instance is the cleanest evidence for it: acting would have produced a worse tree than waiting.

Consequence for parity: the wgpu `BOOT-TIMEOUT` traced to this breakage, so the earlier parity verdict is void. Re-running against a green tree.

### 🔒️ `verify gate` now compiles the wasm bindings — closing the blind spot that hid defect #8

Added to `runGate()` (root `📜️script.ts`, registrar):
```
console.log("[verify] actor kernel wasm32 bindings…");
this.runRustWarnings(["--target", "wasm32-unknown-unknown"]);
```

**Rationale, from evidence rather than tidiness.** Native `cargo check` and `cargo test` never compile `#[cfg(target_arch = "wasm32")]` blocks. That is how `🎭️actor/📦️packages/🦀️rust/📦️glue.rs` sat with a hard **E0308** — `Kernel::complete` had gained a `&TurnResult` parameter while the glue still passed by value — behind a fully green native build **and** a green 60/60 test suite. Nothing in the repo would ever have compiled that file; Z1's clippy run against the real triple was the first thing that did, and only because I had built the verb that morning.

**Scoped deliberately to `wasm32-unknown-unknown`/`semio-framework-actor`**: one small, fast, purity-critical crate whose wasm glue every renderer depends on. The fleet-wide `wasm32-wasip2` (33 crates) and `native` (36 crates) sweeps stay opt-in through `verify rust-warnings --target <triple>` — they are far too slow for a pre-close gate, and a gate people skip protects nothing.

Root script re-parsed clean after the edit. This is the durable fix for the defect class; the E0308 itself was a symptom.

### ✅ sdk-witbindgen — the fleet-wide risk turned out to be ONE line

Guest generator bumped **0.36.0 → 0.57.1**. Coordinator-verified:
```
pin at 🔌️plugin/📦️packages/🦀️rust/Cargo.toml:32          → wit-bindgen 0.57.1
cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest → Finished in 7.98s, exit 0
```
**No source edits were needed at all** — every generated-path alias A2b painstakingly discovered under 0.36.0 (`exports::semio::framework::{reactor,jobs,checkpoint,describe}`, the unprefixed `semio::framework::{effects,events,types,ui}`, `types::PluginError`) resolves **identically** under 0.57.1. The ~90-call-site repointing that the 0.36 migration cost did not repeat. I briefed this packet for a hard migration; the honest outcome was a one-line change, and it proved that rather than manufacturing work.

Its test comparison did exactly what rule 11 asks: `242 passed / 5 failed`, and the five are **the same named set** as the recorded baseline — no new failures, count wobble ignored as the known global-state artifact.

Real-component proof: built `semio-s-plugin-note` for `wasm32-wasip2`, confirmed the magic bytes `00 61 73 6d 0d 00 01 00` mark a genuine **component** (not a core module), then ran the describe binary against it under wasmtime 47.0.3 end to end. So the new guest generator, the new host runtime, and a real plugin all agree.

**It corrected my brief, and the correction is embarrassing in a useful way:** I told it to build note with `--features component-guest`, a feature that crate does not have. This log already records E2's brief making *the identical mistake* earlier in this ticket. I reintroduced a documented error by copying a plausible-looking command instead of checking the manifest. Worth stating because the fix is mechanical: acceptance commands in a brief must be read off the actual manifest, not pattern-matched from a sibling.

It also weathered a live peer consolidating `🗣️dsl`/`🎒️pack`/`📡️spr🎮️command` through the registrar-only `📦️glue.rs`, which broke `semio-framework-os-kernel` for ~30 minutes. It proved the breakage unrelated with an isolated `cargo check -p semio-framework-os-kernel --lib` (same error, no target or feature involved), edited nothing, waited, and re-ran clean. Correct application of "fix what's dead, report what's live".

### 🎯️ `async-worlds` dispatched — every prerequisite is now met and verified

The WASI 0.3 schema goes in with all three gates behind it: **wasmtime 47.0.3** (host), **wit-bindgen 0.57.1** (guest), and the **`*-params` refactor** that lets an async import and a poll effect share one payload shape by construction.

Scope: `interface host-async` (async funcs over the existing `*-params` records — no new payload shapes), `http-fetch` returning a `stream<u8>` body (which also closes a real gap: the poll bridge currently **discards non-final chunks** at `⚛️reactor/🦀️component.rs:143-147`), `blob-read` as a stream, ONE `emit(effect)` door for all ~24 fire-and-forget variants rather than a function each, `interface runner` with `run(events: stream<event>)`, and `world actor-async` alongside an untouched `world actor`.

**The dominant risk is stated in the brief as the acceptance criterion**: both generators parse the *entire* package, so if async syntax fails under either, all 33 plugin crates stop building simultaneously — and the failure will present as "cannot find `exports` in `component`" with no mention of WIT. It was told to read the FIRST error, and that a truthful "this does not parse, here is why" is a complete result. The poll backend works for the whole fleet today and is not to be broken chasing this.

Deliberately excluded: `cancel-request` (still deferred to the adapter packet that will actually consume guest future-drop), any change to `world actor`, the Rust `Effect` enum, the guest SDK, or any plugin. The parity test goes in a NEW file rather than `🖥️host/🦀️component.rs`, so it cannot collide with the packets live in that file.

### 🔍️ `🎪️demonstrator` (M8) — the last plugin, and its blocker is architectural, not mechanical

Traced read-only (no build added, three packets were already competing for slots). `kit.catalog` is declared as an artifact kind by **four separate plugins**:

| plugin | sites |
|---|---|
| 🧩️puzzle | 23 |
| 🧱️block | 23 |
| 🪵️sourcing | 11 |
| 🗄️stdio | 4 |

Each emits its own descriptor without complaint — `🧱️block` and `🪵️sourcing` both have committed descriptors right now. The conflict is **purely compositional**: `demonstrator` bundles panes from six foreign plugins (cad, gis, procedural, process, puzzle, sourcing), so assembling it registers `kit.catalog` several times over and the definition registry rejects the duplicate.

**This is the same class as D3's `s.stdio.dwg@ac1018/*` collision but with four claimants, and it is a genuine ownership question**: should `kit.catalog` be owned by exactly one plugin and *referenced* by the others, or is a shared vocabulary kind meant to be declarable by many? The codebase currently does the latter and the registry forbids it at composition time — so one of the two has to change, and which one is a design decision with consequences for every kit-consuming plugin.

**Not forcing it.** I have no evidence about intended ownership, and picking a winner arbitrarily across four plugins would be exactly the "fabricate a fix" failure this ticket has avoided all session. Recorded for the closing session as the single remaining M8 blocker, with the claimant table above as the starting evidence. It also explains, concretely, why `🎪️demonstrator` was sequenced last from the very first plan — the plan was right about the risk without knowing its shape.

### ✅ shard-grants — BOTH parts landed; the sibling sweep is finally done

Coordinator-verified:
```
cargo test  -p semio-framework-actor                                        → 69 passed / 0 failed (baseline 60 + 9 new), exit 0
cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity     → 100 passed / 0 failed / 1 ignored, exit 0 (baseline 86 + 14 new)
grep -c '} & ' 🎭️actor/🤖️generated/🟦️actor.ts                                → 0
```

**Part A — the sweep W4 wrote down and nobody ran.** All six internally-tagged newtype variants (`Payload::Event`/`Cancel`, `Origin::Actor`, `TurnStatus::Faulted`, `FailureSignal::Trap`, `Backpressure::Dropped`) are now struct variants, every construction and match site fixed repo-wide, and the regenerated TypeScript mirror contains **zero** `} & ` intersections — so the previously un-typeable variants are now expressible on the web side, which is what unblocks the `ShardFrame` web adoption later. Its six new tests **serialize to bytes and back** rather than comparing in-process values; that distinction is the entire reason the identical `JobStep` defect hid for a full wave.

**Part B — the scheduler's decisions now actually reach the shard.** `ShardFrame { Register | Unregister | Grant{actor,budget,envelopes} | Envelope }` with a pack round-trip; `TURN_BUDGET`, `JOB_STEP_BUDGET` and the `budget_for` closure are **deleted**, with DRR budgets arriving in `Grant` and remembered per actor. `ShardExecutor` lives in a new `🧵️shard/🏃️executor.rs` parking on `ThreadTransport::recv_deadline`, and `to_actor_turn_result` went into `🧵️shard/` rather than the host file — so the collision I sequenced around cannot recur. It mounted the new module with a relative `#[path]` inside `🧵️shard/🦀️component.rs`, needing **no** edit to `🖥️host/🦀️component.rs`'s module tree at all.

#### A discrepancy I chased before believing either side

My first plugin-host run showed **100 passed / 4 failed**, against its reported 100/0. The four were `schema_parity::tests::*` — a module created **two minutes earlier** by the concurrently-running `async-worlds`, in a crate it shares. Schema mtime 20:53, test dir 20:55, my run 20:57. Re-running with `--skip schema_parity` gives **100 / 0**, confirming shard-grants' number was accurate when taken.

Worth noting what those in-flight failures are asserting, since `async-worlds` will have to resolve it: its parity test demands `world actor` import **only** `pure`, while `wit-parser` reports `{capabilities, effects, events, pure, types, ui}`. That is almost certainly the test being too strict rather than a capability leak — a `use` of another interface's *types* makes that interface appear as an import, which is not the same as importing host *functions*. The "only `pure`" claim in this ticket's design has always meant functions.

#### 🔧️ Lease resolved by dispatching the packet that owns the file

`pump()` losing its budget-closure parameter broke **two call sites in the wgpu target's `📦️glue.rs`** (≈363 and ≈684), leaving that crate red. shard-grants correctly filed a lease instead of editing outside its authorization.

**Registrar decision: per-actor budgets survive by travelling in `ShardFrame::Grant`; they do not flatten to a Maintenance default.** The bench exists to measure behaviour under specific budgets, so flattening them would quietly change what it measures — and sending real `Grant` frames makes the bench exercise the production path instead of a test-only shortcut.

I did not hand-patch it. The fix needs `Grant` frames and the `Budget`→`TurnBudget` conversion inside a 2700-line file that the **next** packet rewrites anyway, and I would have been editing half-understood code while a sibling packet was live in the same crate. Instead `kernel-loop` is dispatched with un-redding wgpu as an explicit **step 0**, to be reported with its exit code *before* the larger work begins — minimising the window in which other sessions trip over it.

### 🎯️ `kernel-loop` dispatched — the packet budget 5 has been waiting for

Scope: a real kernel loop (`submit` → `tick` → `Grant` → drain → `to_actor_turn_result` → **`Kernel::complete`** → `commit_frame`), **K parallel `ShardExecutor` threads** replacing the single-shard servant, `Kernel::new(Thread, K, 2, 64)` with K taken from `thread_plan(cores).shards` rather than a fresh ad-hoc formula, `EventLoopProxy` wake plus a `MainThreadBridge` for main-thread-only operations.

Until now the DRR scheduler, failure ladder and metrics this ticket built have been **inert natively** — nothing calls `tick` or `complete` outside tests and benches. That is why budget 5's last measurement put 30 samples inside a 0.1 ms band: a constant, not a latency. With K real shard threads the instrument becomes valid for the first time.

The brief is explicit that **a valid-instrument failure is a publishable result**: report the number measured, do not tune the harness to produce a pass, and do not claim a row that was not run. `ControlFlow::Poll → Wait` is deliberately excluded (per-frame asset polls depend on `Poll`), as are the `Shell` block_on parks and the async effect executor.

### 🔄️ D3 corrected the classification — the DWG collision was INTRA-plugin, not cross-plugin

D1 classified `s.stdio.dwg@ac1018/*` as a collision **between** `🌀️procedural` and `🌍️gis`, and I repeated that in D3's brief. D3 root-caused it from real build output and found it is **intra**-plugin in both cases: `procedural2d` vs `procedural3d`, and `gismap` vs `gisterrain`, each pair declaring identical literal composer claims for dwg/json/png inside a single plugin.

Materially different defect, materially different fix — and it got there by reading the compiler instead of trusting a brief that two prior agents had already agreed on. **The classification was wrong at two levels of the chain (D1's report, then my brief) and survived because it was plausible.**

**Ownership rules it applied, each backed by a distinguishing signal rather than a coin flip:**
- `procedural3d` keeps DWG — it has a real `mesh_dwg_bridge` host-media handler; `procedural2d` has none.
- `gismap` keeps all three — it is the independently-activated top-level artifact; `gisterrain` is a composed child that is never independently activated, so it sheds them.
- json/png between procedural2d/3d: **explicitly flagged as a documented tie-break with no distinguishing signal.** I told it to keep that caveat exactly as-is rather than dress it up as a justification. An honest arbitrary choice, labelled, beats a fabricated rationale.
- **Import capability untouched in every case** — only the export/composer claim was removed from the non-owning artifact. Minimal and reversible.

**Other fixes:** `🖍️draw`'s duplicate symbol was a genuinely dead weak-linkage shim (`semio_plugin_bundle_installer_link_shim`) colliding with the strong symbol from its own `plugin_exports!` — removed, **105/105 tests pass**, descriptor committed and **ratcheted** (11 now). `📜️imperative`'s was five `#[path]`-mounted extension modules each unconditionally calling `extension_exports!` inside imperative's crate — five strong definitions of one symbol — fixed with an `extension-entry` feature mirroring procedural's existing `plugin-entry` pattern, i.e. reusing an established mechanism rather than inventing one. `🔋️energy` gained the missing `crate-type = ["cdylib", "rlib"]`.

It ended its turn on background builds (the seventh occurrence of that trap) and has been resumed to verify in the foreground.

### ⏱️ Scheduling note — parity and the descriptor packets want the same fleet

Parity's prebuild compiles **every plugin** into the dev catalog (`built program block`, `built program cad`, …). D2 and D3 are simultaneously building those same plugin crates for descriptor emission. All three serialize on one cargo build-directory lock, so parity's log shows `Blocking waiting for file lock on build directory` while making real progress in between.

Parity started 20:18 with a 60-minute budget; at 21:05 it was still prebuilding. It may exhaust that budget purely on lock waits rather than on work.

**Not intervening, deliberately.** Killing D2/D3 to hurry parity would discard descriptor repairs mid-flight; killing parity wastes a prebuild that is warming the exact catalog a later run needs. The correct sequence is the one already in motion: let the descriptor packets finish, then re-run parity against a warm catalog — where it should complete in a fraction of the time.

**The generalisable rule, third time it has bitten today:** on this machine, *scheduling* is the binding constraint far more often than correctness. Per-packet target dirs removed lock contention between our own builds but multiplied total compile work; capping builders at 3 fixed that; and now two packet families that each need the whole plugin fleet cannot productively overlap at all, regardless of the cap. **Packets that build the same fleet should be sequenced, not parallelised** — the cap on concurrent builders is necessary but not sufficient.

### 🎉️ async-worlds — WASI 0.3 syntax is in the schema AND the guest generator accepts it

`interface host-async` (schema line 887), `interface runner` (961), `world actor-async` (1044), 28 `async func`/`stream<` constructs, `world actor` untouched.

**Gate 1 — the fleet-wide risk — PASSES, coordinator-run:**
```
cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
  → Finished in 3m 57s, exit 0
```
That is the result this wave existed to get. Both generators parse the **entire** package, so async syntax that 0.36 could not read would have stopped all 33 plugin crates at once — behind a "cannot find `exports` in `component`" cascade that names no WIT. It parses. The 0.3-shaped ABI and the proven poll ABI now coexist in one file, sharing every record.

**Gates 2–4 are honestly unrun**, and the packet said so plainly rather than implying coverage: host `bindgen!` (`plugin-host --all-targets`), the run that would actually **execute** its 4 parity tests (`plugin-host --lib`), and `plugin-describe --all-targets`. Mine to finish.

#### The design question it settled, correctly

Its own parity test asserted `world actor` imports **only** `pure`, while `wit-parser` reports `{capabilities, effects, events, pure, types, ui}`. It agreed with my read and then did better than loosen the assertion: it re-read each of the five interface blocks, confirmed **none declares a single `func`** (so no `Host` trait method exists for any of them), and rewrote the check as `functional_import_names` — interfaces carrying ≥1 `func`, which must equal `{pure}` for `actor` and `{pure, host-async}` for `actor-async`. It then added a **positive** assertion that `actor` really does have type-only implicit imports, so the distinction cannot silently become vacuous.

That is the right resolution: "only `pure`" in this ticket's design has always meant *host functions*, and it structurally cannot mean raw interface names while `reactor`/`events` share record types across interfaces. A test that asserts something which cannot be true is worse than no test.

#### Two process failures worth recording — one its, one mine

It **ended two consecutive turns idling on backgrounded builds**, the wake/idle trap that cost ~1.4M tokens in W1 and is binding rule 5 in its own brief. I stopped it, handed it my verified gate-1 result so it would not rebuild, and took the remaining gates. It also flubbed capturing an exit code through a `tee` pipe — the same shape as my own `| tail; echo $?` error earlier today, which is now rule 10.

**Mine is the more consequential:** I dispatched `kernel-loop` and `effects-async` while `async-worlds` was still finishing, so five-plus cargo processes contended and my gate-2 run died at a 10-minute timeout. W3 recorded this exact error — *"I dispatched six builders at once after explicitly recording, one wave earlier, that parallel building does not scale here"* — and I repeated it. The cap is on **builders**, not packets, and it must count sibling packets I have already dispatched, not just peers. I am now waiting for the queue to drain rather than adding a sixth.

### ✅ GATE 2 PASSES TOO — both generators accept the WASI 0.3 package

```
cargo check -p semio-framework-plugin-host --all-targets   → exit 0, 14m 41s, zero errors
```
So `wit_bindgen::generate!` (guest, my run) **and** `wasmtime::component::bindgen!` (host, its run) both parse the schema with `interface host-async`, `interface runner` and `world actor-async` present. The dual-backend ABI this wave set out to build is real and neither toolchain rejects it. `--all-targets` also **compiled** the 4 parity tests, though it did not execute them.

The packet had reported this check as abandoned; the notification arrived after its message and it came back to correct the record rather than leaving a pessimistic claim standing. It also found and fixed 2 warnings of its own (`///` doc comments on `let` statements are invalid — should be `//`), without a rebuild against the fix.

**Still genuinely open, and it listed them rather than implying coverage:** `cargo test -p semio-framework-plugin-host --lib` — the run that would actually **execute** those parity tests — plus `plugin-describe --all-targets` (its own separate `bindgen!` call) and the 86/0/1 baseline re-diff. Per this ticket's most expensive recurring lesson, *a contract that compiles is not a contract that runs*, so gate 3 is the one that decides whether the parity mechanism works. Mine to run once the machine is free.

### ⏳️ Deliberately NOT adding build load

Contention went **up** to 15 cargo processes after my last mistake, not down — two sibling Rust packets of mine plus a peer session. Running gate 3 now would repeat the error that already cost me one 10-minute timeout, so I set a bounded background waiter that fires when builders drop below 3, and spent the wait on work that costs **zero** build capacity.

### 🌐️ `web-shardframe` dispatched — the Rust side just unblocked it

The web adoption of the `ShardFrame` wire became possible only now, and specifically because of Part A of `shard-grants`: converting those six newtype variants regenerated a mirror **without impossible `object & string` intersections**, which is what previously made `Payload`/`Envelope`/`Origin` un-typeable in TypeScript. The wire could not have been adopted on the web before that fix, which is a good illustration of why Part A was worth doing even though every one of those variants was *latent* rather than live.

Scope: mirror the `ShardFrame` union in TypeScript, land `Envelope` passthrough **first** then `Grant` (the incremental path the Rust variant was designed for), and make DRR-granted per-turn budgets reach the shard worker — filling the `budgetFor` seam that `web-shard-scheduler` deliberately left open — with the standing requirement to **prove the old constant is gone, not merely unused**.

One decision was handed to the packet rather than pre-decided: the Rust transports pack `ShardFrame` with a hand-rolled binary codec while the web worker uses `postMessage` structured clone. Shape-only adoption is very likely right for now, but it must **say so and justify it**, and add a cross-language variant/field-name parity test — this session already caught `Lane` casing diverging silently between the two languages, and a repeat is exactly what such a test prevents.

## 🚨️ THREE PLACEHOLDER DESCRIPTORS WERE COMMITTED — caught, deleted, and made unrepresentable

**Correction to my own earlier report first.** I reported "the hard classes are cracking: procedural, draw and block all landed" based on `ls | wc -l`. **Two of those three were `assembly-failed` placeholders.** I counted filenames and never read contents. The real total was 21, not 24.

Audit of every committed descriptor found three with `manifest.pluginId == "assembly-failed"`:

| plugin | size | emitted by |
|---|---|---|
| 🌀️procedural | 713 B | D3 |
| 🏗️fem | 707 B | D2 |
| 🧱️block | 684 B | D2 |

Against `🖍️draw`'s 164 712 B and `🗒️note`'s 266 725 B — **the size alone was the tell, and I had been reporting counts past it for over an hour.** `procedural`'s even carried its unresolved error in the label: `dialect:s.stdio.dwg@ac1018/* is already registered by s.procedural2d.composer.dwg`, i.e. D3's ownership fix had not taken effect when it emitted.

All six files deleted (`.json` + `.semio`). **None had been ratcheted**, so `descriptor_is_fresh` never went red — but they would have fed the generated registry catalog with fabricated contributions on the next `generate`. `🏗️fem` was then re-emitted as a placeholder a second time while I was cleaning up, which is what settled the diagnosis: cleanup is not a fix when the producer is still running.

### Why the rule failed, and the structural repair

Every packet was told never to commit a placeholder, and every packet agreed. The rule held right up until the agent **stalled on a background build before reaching its verification step** — the eighth and ninth occurrences of that trap today. **A rule that depends on an agent reaching a later step enforces nothing when it doesn't get there.**

So the fix went to the writer, not the process. `describe_component` (`📇️describe/📦️glue.rs`) now refuses:
```rust
if descriptor.manifest.plugin_id == ASSEMBLY_FAILED_PLUGIN_ID {
    return Err(DescribeError(format!("refusing to write a placeholder descriptor for {}: plugin assembly failed — {}", wasm_path.display(), descriptor.manifest.label)));
}
```
The error surfaces the real assembly failure instead of burying it in a file that looks like success. `ASSEMBLY_FAILED_PLUGIN_ID` is defined **once**, in `🛂️manifest/🦀️component.rs` beside `PackageDescriptor` — the single crate that both the guest SDK which mints the stub and the host emitter which rejects it depend on, so the two cannot drift.

### Two lessons I am treating as load-bearing

1. **A descriptor count is not a descriptor audit.** I reported progress from `ls | wc -l` for hours. The content check is one command and belongs in every count. This is the same failure as "a contract that compiles is not a contract that runs", one level up: *an artifact that exists is not an artifact that is valid.*
2. **The background-build stall stopped being an efficiency problem and became a correctness one.** Eight prior occurrences cost tokens; these two produced **incorrect committed state**. That reclassifies it.

### 🤝️ Cross-session coordination — a peer on this same ticket, and the `kit.catalog` question answered with evidence

A peer session ("Plugin architecture for extensions") messaged with shared-tree changes and one cross-ticket design question. Most of what it listed is already recorded above from the W4 session (WASI p2 into both linkers, `BudgetLimiter` 1→256, `ShardTable::pin` least-loaded, `JobStep` struct variants, `verify gate` compiling the wasm32 bindings). **Two items were new to me and are worth having:**
- The `describe` emitter now **refuses to write a descriptor whose `pluginId` is `assembly-failed`** — three such placeholders had been committed; they parse as JSON and would feed fabricated contributions into the generated catalog. The refusal message carries the real assembly error.
- The dev parity harness now sets `PLAYWRIGHT_BROWSERS_PATH` to the repo-local cache. Without it `parity` died claiming a browser was missing that `📜️script.ts setup` had already installed — so any past "parity is broken" impression was that.

#### `kit.catalog` — grepped rather than opined on

The question: four plugins declare the artifact kind `kit.catalog`, `🎪️demonstrator` bundles them, and the registry rejects the duplicate claim — should one plugin own it, or is a shared vocabulary kind meant to be multiply declarable?

**Answer: single declarer, everyone else references — and the registry check should stay strict.** An artifact kind is a *schema*; N declarations are N sources of truth for one contract, and the first divergence between them would be silently undetectable. Resolving demonstrator by relaxing the duplicate rejection would delete the only mechanism that catches this.

The evidence is one-sided once you separate *declaration* (an `artifact_kinds` entry) from *reference* (a port's `kind_id`):

| plugin | role | evidence |
|---|---|---|
| `🧱️block` | **declarer + producer** | `const KIT_CATALOG_ARTIFACT_ID` in two artifact trees (`🧊️3d`, `🖐️5d`), asserts the kind is in `definition.artifact_kinds`, emits `catalog:out` |
| `🧩️puzzle` | consumer that **also** declares | `🗿️artifacts/🧊️3d/🦀️component.rs:510` says it outright — "the `kit.catalog` artifact kind puzzle3d's `kit:in` port consumes — **declared here too (harmless**". It is not harmless; that is the duplicate blocking demonstrator |
| `🪵️sourcing` | pure consumer | one `kind_id` on a port |
| `🗄️stdio` | does **not** declare it | but two docstrings state the intent — "three-block's *separately-declared* `kit.catalog`" and "**Absorbs the duplicated** `kit.catalog`" |

So the minimal correct fix is to drop the declarations from the consumers, leaving `🧱️block` as owner. Whether ownership should ultimately migrate to `🗄️stdio` — the shared-vocabulary plugin everything depends on, whose own comments say it means to absorb this — I deliberately did **not** decide: the declaration channel belongs to the peer's ticket, nothing has actually absorbed it yet, and "block declares, others reference" unblocks demonstrator without pre-empting that call. Enforcement is mine and stays strict; vocabulary ownership is theirs.

I also warned them off the three files my packets hold live (`🖥️host/🦀️component.rs`, `🧵️shard/**` + wgpu glue, the actor TS package) and told them about the wasmtime 22→47 / wit-bindgen 0.36→0.57.1 jump, the deliberately-cold `.cwasm` cache key, the 2× vitest inflation still present in their packages, and the ~253 silently-unreached tests.

### ⚠️ Third packet today ended a turn idling on a backgrounded build

`effects-async` did what `async-worlds` did twice: wrote its module, backgrounded `cargo check`, and ended the turn waiting. **The root cause is mine.** At 18 cargo processes a two-minute build takes fifteen-plus, which is exactly the pressure that makes detaching look reasonable — and I created that pressure by dispatching siblings past my own cap. Rule 5 keeps being violated because the condition that provokes it keeps being recreated by me.

Redirected it to produce a report with **no further builds**, marking every acceptance command UNRUN rather than omitting it, and to answer one question above all others: **has the module ever compiled even once?** It lives in the 264 KB shared `🖥️host/🦀️component.rs`, and if it has never compiled I need that in writing before the peer's plugin-fleet build trips over it. Acceptance moves to me, as it has for every packet this wave.

Current load is mostly the peer's fleet prebuild plus `kernel-loop`'s wgpu check (22 min and counting — that crate is genuinely slow, not hung). Holding all new Rust dispatch until the queue drains.

## 🤝️ Cross-session coordination — `kit.catalog` resolved, and three of my claims corrected

A peer session (`semio-fb`, coordinating the W5+ async-first rewrite on this same ticket) replied to my coordination message with evidence, corrections, and a set of live-file claims. Acting on all of it.

### `kit.catalog` — RESOLVED: `🧱️block` declares, consumers reference. Landed.

The peer's deciding argument: **an artifact kind is a schema, so N declarations are N sources of truth for one contract, and the first divergence between them is undetectable.** Relaxing the registry's duplicate-claim rejection to unblock demonstrator would have destroyed a check doing exactly its job — fixing the alarm instead of the fire.

Verified their evidence rather than taking it on trust; it held on every point. The clincher was **puzzle's own docstring**:

> "The `kit.catalog` artifact kind puzzle3d's `kit:in` port consumes — **declared here too (harmless** if a producer, e.g. block3d, declares an identical spec)…"

Not harmless: that line was the last thing blocking `🎪️demonstrator`. **A comment asserting its own safety, wrong for months, with nothing able to contradict it until a six-plugin bundle forced both declarations into one registry.** The same shape as the `📇️describe` `Cargo.toml` comment that reasoned "no `wasmtime-wasi`: the world declares no wasi import" — a confident inline claim, derived from the design rather than from a run, wrong the moment anything real exercised it.

Change (one line of behaviour): removed `.artifact_kind(…kit_catalog_artifact_kind())` from puzzle3d's editor builder — its single registration site. **Kept** the spec function and every `kind_id: Some("kit.catalog")` port reference; consumers reference, only the declaration went. Docstring replaced with the real reasoning, naming block as owner via `catalog:out`/`KIT_CATALOG_ARTIFACT_ID`. `🪵️sourcing` needed nothing (pure consumer); `🗄️stdio` declares nothing.

**Ownership deliberately NOT moved to `🗄️stdio`.** Its docstrings describe absorbing this kind, but nothing has acted on that; making the move now would pre-empt a decision with no evidence. Recorded as open.

### Three of my own claims corrected by the peer

1. **My test counts may be inflated ~2×.** Naming the same file in both vitest `include` and `includeSource` collects it twice. Fixed in `🧰️framework`/`💻️os`/`🧑️‍💻️dev`/`🎭️actor`; still live in `mcp`, `shell`, the 4 cad extensions, `animate`. **Every suite figure I quoted this session ("322/324", "86/0", "60/0") is suspect and needs re-measuring, not restating.**
2. **~253 tests never run at all** — `@semio-tech/cad-js` fails outright on a stale `kernel-3d-js` import (renamed `s-3d-js`, ~9 files, ~153 tests), and two infinite-canvas projects report green while matching zero files under `passWithNoTests: true` (~101 tests). My parity work covers the cad variant, so this is directly load-bearing.
3. **wasmtime 22.0.1 → 47.0.3 and wit-bindgen 0.36 → 0.57 landed today.** I had attributed my 10-minute builds and cold artifact cache purely to lock contention; a large part was a by-design full rebuild (the `.cwasm` cache key was bumped so stale 22-era artifacts cannot deserialize into a 47 engine).

### Live-file boundaries now agreed

The peer has packets live in `🔌️plugin/🖥️host/🦀️component.rs` (`effects-async`), `🖥️host/🧵️shard/**` + wgpu `📦️glue.rs` (`kernel-loop`), and `🎭️actor/📦️packages/🟦️typescript/**` (`web-shardframe`). **I had already edited two of those today, before their message** — WASI wiring, `BudgetLimiter`, the four pooling sub-pools, `scale_bench`, `unexpected_faults`. All landed and verified beforehand; I have committed to no further edits there without pinging first, and offered to rebase my side if theirs conflicts.

### ✅ web-shardframe — the web speaks the Grant wire; verified 40/40

Coordinator-re-run: `🎭️actor/📦️packages/🟦️typescript` → **40 passed / 0 failed, exit 0** (29 baseline + 11 new).

`ShardFrame`/`ShardEnvelope`/`ShardOrigin` mirror the Rust variants field-for-field; `ShardClient` gains `envelope()` (Grant-less passthrough) and `grant()` (budget travels with lane-sorted envelopes) while `turn()`/`activate()` stay untouched, so **both wires coexist** exactly as the incremental-adoption design intended. The generated worker handles a `"frame"` message, tracks last-granted budgets per actor, falls back to a Maintenance floor mirroring `lane_defaults::budget_for`, and **acks unknown frame kinds instead of throwing** — forward-compatible with Rust-side variants that do not exist yet.

**The two things that make this more than a type mirror:**
- Its parity test **reads `🖥️host/🧵️shard/🦀️component.rs` fresh on every run** and diffs it against a runtime descriptor of the TS union, so Rust-side drift fails loudly rather than silently. That is the direct answer to `Lane`'s casing having diverged unnoticed earlier this session.
- It wrote an ad-hoc Node smoke test that **executes the actual generated worker source** against a faked bridge, confirming Grant→budget→`poll()` threading, lane ordering, granted-budget reuse, the Maintenance fallback and unknown-frame tolerance in the real generated text — not just in the TypeScript mirror. A template string cannot `import`, so the worker's logic is hand-transcribed; testing the mirror alone would have proven nothing about what actually ships.

Encoding decision, made and justified rather than assumed: **shape-only over structured clone**, not Rust's byte pack-codec, with `payload` keeping the already-decoded envelope shape rather than opaque pack bytes nothing on the web can decode yet. The future byte-unification seam is written up.

Honest gap: no real caller yet — `ActivationRegistry`'s `defaultBudget` constant sits in `🎠️kernel/🟦️component.ts`, outside its owned paths.

### 🔴️ effects-async — HONEST failure, and the systemic cause found

It reported **zero confirmed compiles**: all three acceptance commands UNRUN, every one marked as such rather than omitted. Unverified code now sits in the shared 264 KB `🖥️host/🦀️component.rs` plus a new `⚡️effects/` module, so *is the tree red* became my highest-priority unknown.

With no build available it did a line-by-line self-review and **caught three real defects**, one of them a hard compile error: `dispatch_http`/`dispatch_router_effect` moved `scope` into an `async move` block while still needing it afterwards for `spawn_scoped(&scope, …)`; a capability-revocation test asserted a non-revoked op completes `Ok` while the handler under test always returned `Err`, making the assertion wrong independent of the logic; and an ambiguous slice-vs-array `PartialEq` simplified to a plain `Vec` comparison. It flagged `Send`/`'static` propagation through nested `Arc<dyn Trait>` as its lowest-confidence area. Self-review is not a build, and it said so.

Its `🔀️PostTurnRelay` verdict: **KEPT**, with grep evidence that it is a job-execution primitive `IoRouter`/`ArtifactInferenceRouter` still depend on rather than a live per-turn dispatcher — none of which exists anywhere outside tests. Correct call under the brief's "retire only if you can prove nothing depends on it".

#### 🧷️ Root cause of the wake/idle trap, finally identified

The **Bash tool auto-backgrounds at a ~120 s default timeout.** Three packets today did not choose to detach — the harness detached them, and they then idled across a turn boundary where the result can never arrive. My own runs succeeded all wave because I pass explicit long timeouts without having noticed that was the load-bearing difference. Added as rule 19: executors set the maximum timeout on every cargo command and report a build unrun rather than detaching; coordinator background tasks *do* survive turn boundaries and notify, which is a second reason acceptance belongs to the coordinator.

My compile check timed out twice at 10 minutes against 19 then 14 concurrent builders (mostly the peer's fleet prebuild), so I have moved it to a coordinator background task and will report the verdict when it lands.

### ⏳️ Guard status — compiles, NOT yet fired (stated precisely)

`cargo check -p semio-framework-plugin-describe` → **Finished in 28m 51s** (the wasmtime 47 rebuild), so the placeholder guard compiles. **It has not yet been empirically fired**: my check produced no binary, and the pre-existing `describe` binaries in the D2/D3 target dirs predate the guard, so running those would prove nothing.

Build + fire against `🏗️fem`'s known-`assembly-failed` wasm is running now. Until it reports: the guard is **verified to compile, not verified to work** — the exact distinction this ticket has enforced on every packet all session, and it applies to my own change no differently.

### 🛑️ D3 stopped after three stalls

D3 ended its turn waiting on a background build **three times**, across 333k tokens and 226 tool calls, including twice after an explicit instruction naming the trap. Stopped rather than resumed a fourth time. Its landed work stands and is real (`draw` fixed + ratcheted at 105/105, `imperative`'s five-`extension_exports!` duplication root-caused, `energy`'s `crate-type`, the intra-plugin classification correction) — but `gis`, `energy` and `imperative` were never verified by it, and `procedural` emitted a placeholder that I deleted.

**Nine stalls across the session, and the last two produced incorrect committed state rather than merely wasted tokens.** Briefs naming the trap explicitly did not prevent it. The durable mitigations are the ones now in place: the emitter refuses invalid output regardless of whether an agent reaches its verification step, and `📌️important.md` rule 9 carries the instruction for future waves.

### 🔍️ The tree-red scare was a peer mid-move, not our code — checked before reacting

My background compile of `effects-async` failed, but **not on its code**:
```
error: couldn't read `💻️os/📦️packages/🦀️rust/./../../🔨️modules/🎒️pack/🦀️component.rs`: No such file or directory
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error
```
That is `📦️glue.rs:97`'s `#[path]` pointing at a file that was momentarily absent — the peer's `🗣️dsl`/`🎒️pack`/`📡️spr🎮️command` consolidation, the same one `sdk-witbindgen` hit and correctly waited out earlier today.

**Before touching anything I checked whether it was dead or live, and it was live by one minute.** `ls` on that directory shows `🦀️component.rs` present with mtime **22:05**, siblings `🔢️value` at 22:02 and `🧪️testkit` at 22:05 — my compile ran at ~22:04, inside the window where the file did not exist. So this is a genuinely transient mid-move state, not a half-landed change to repair. Had I "fixed" the `#[path]` or recreated the file, I would have fought a session that was actively creating it.

Worth naming the near-miss: this ticket's most-repeated peer failure signature is *the artifact moved, its registration did not*, and it has absorbed four instances. The fifth candidate turned out to be a live edit caught mid-flight. **The evidence test is the same either way; only the timestamps differ** — which is precisely why the rule is "measure liveness, then decide" rather than "recognise the pattern and act".

A cheap methodological note: my `find -newermt '-45 minutes'` returned nothing while `ls -la` showed a one-minute-old file in the same directory. The `ls` evidence is direct; the `find` (with emoji `-not -path` patterns) silently under-reported. **A find that returns nothing is not evidence of nothing** — the same lesson as the too-narrow grep earlier today, and it would have led me to conclude the peer was idle.

Compile re-queued as a coordinator background task now that the file is back. `effects-async`'s code remains **unverified** until it returns; nothing about the earlier failure implicates it.

### ⏸️ Peer consolidation is progressive — stopped chasing it

Re-run failed again, on a **different** file: first `🎒️pack/🦀️component.rs` was absent, now `🎒️pack/⏳️async/🦀️component.rs`. The peer is moving subunits **one at a time**, so every re-run simply catches the next one mid-flight. Each doomed attempt also consumes the build-directory lock that the peer needs to finish, making the wait longer for everyone — chasing here is not merely useless, it is counterproductive.

Replaced the retry loop with a **cheap file-existence gate**: poll the `#[path = "…"]` targets declared in `💻️os/📦️packages/🦀️rust/📦️glue.rs` until every one resolves, and only then spend a cargo build. Bounded at 70 polls / ~35 minutes. Reading the module registration to decide when the tree is coherent costs nothing and asks nothing of the lock, where a speculative `cargo check` costs several minutes and takes the lock from the session that is trying to make it green.

`effects-async` stays **unverified** — and I want to be precise about what is and is not known: it has never compiled, its own author said so, and three separate build attempts have now been blocked *before reaching its code* by an unrelated peer restructure. Nothing so far is evidence either way about its correctness.

### 🔁️ Sixth half-landed change — `🎒️pack` modules deleted while still mounted

`os-kernel` red again for every session, hours after the `🗣️dsl` instance resolved:
```
error: couldn't read `…/💻️os/🔨️modules/🎒️pack/⏳️async/🦀️component.rs`: No such file or directory
 D  💻️os/🔨️modules/🎒️pack/⏳️async/🦀️component.rs
 D  💻️os/🔨️modules/🎒️pack/🌐️http/🦀️component.rs
 D  💻️os/🔨️modules/🎒️pack/📐️format/🦀️component.rs
```
All three still `#[path]`-mounted from `💻️os/📦️packages/🦀️rust/📦️glue.rs` (the `⏳️async` one at line 101). The `⏳️async` name matches the peer's live `effects-async` packet, which they explicitly told me is in that area right now.

**Not touched, and notified instead** — the same call as last time, and last time it was demonstrably correct: I was about to repoint the `🗣️dsl` mounts when the peer finished by removing them entirely in favour of `pub use protocol::*`. My "fix" would have re-established the exact fragile mechanism they were deleting.

Blocks my `🧩️puzzle` verification for the `kit.catalog` change. Waiting rather than racing.

**Six instances now, one signature: the artifact moved or went, its registration did not.** Moved generated file (ui-styling) · renamed presence type set · moved crate with a stale workspace member entry (broke `cargo metadata` machine-wide) · struct gaining a required field its call sites missed · `🗣️dsl` promotion · these `🎒️pack` deletions.

**Proposed cheap guard, offered to the peer rather than imposed**: a test in the owning crate that walks every `#[path]` in `glue.rs` and asserts its target exists. That converts "os-kernel is red for every session in the tree" into one named failing test in the crate that owns the mount table. Six occurrences in one ticket is enough evidence that the mount table is a live hazard during this rewrite, not a stable structure.

### ✅ D3 accepted — one plugin verified, four fixed-but-honestly-unratcheted

It came back with real exit codes after the third nudge, and the report is sound.

**`🖍️draw` fully verified**: describe exit 0, `cargo test --lib` **105/105** including `descriptor_is_fresh`, ratcheted. The duplicate symbol was a genuinely dead `semio_plugin_bundle_installer_link_shim` weak-linkage stub colliding with the strong symbol from draw's own `plugin_exports!`.

**`procedural`/`gis`/`energy`/`imperative`: fixes applied and reviewed, explicitly NOT ratcheted** pending re-verification once the tree is green. That is the right discipline — a ratchet entry on an unverified plugin is a red tree for every session.

**It independently confirmed the `🎒️pack` blocker I had just reported to the peer**, reaching the same conclusion by a different route: `REAL EXIT: 101` reproduced identically across **7 separate retries**, root-caused to the stale `#[path]` mount, confirmed via `git status` showing the files deleted-but-unstaged at the old location and present at a new top-level one — then **flagged rather than touched** because it was outside path_scope. Two independent agents, same evidence, same restraint.

**Its classification correction now carries stronger proof than when it first raised it.** Beyond "procedural2d vs procedural3d, gismap vs gisterrain", it established that the definition registry is **freshly instantiated per plugin build**, and that `process3d` claims the same literal dwg string with **zero conflict** — which is decisive: if the collision were cross-plugin, process3d would collide too. D1's cross-plugin framing (which I repeated in the brief) could not have survived that test, and nobody had run it.

**Second pre-existing blocker found and flagged, not fixed**: `procedural`'s own native `cargo test --lib` carries **44 compile errors** from `ArtifactStore::new()`'s `Result`-unwrap fallout, spanning framework files. Out of scope, routed to a follow-up task.

**It also reported its own mess**, unprompted: stray duplicate background processes, and an incremental-cache deletion that corrupted one of its own running builds. Self-reporting a self-inflicted failure is worth more than the clean narrative it could have written instead — and it is the honest explanation for part of the contention I was attributing elsewhere.

### ✅ D2 accepted — 0/7 converted, reported as 0/7, and it corrects both D1's taxonomy and me

**It leads with "0/7 fully converted".** Row-level fixes for 6/7, mechanically confirmed for 2/7, stdio untouched. After ~7 hours and 461k tokens, opening with the honest headline rather than the six things it did accomplish is the right instinct.

**Attribution correction — mine to own.** I recorded `🧱️block`'s placeholder descriptor as "emitted by D2". It was **D1's**, committed in the earlier wave and left stale. D2 inherited it. My audit found the placeholder correctly but I guessed at its author from which packet happened to be running, and guessed wrong. The deletion was still right; the blame was not.

**Third reclassification of D1's taxonomy, and the class keeps shrinking.** `🏗️fem`'s rows were genuinely broken AND fixed AND mechanically confirmed (`cargo test -p semio-s-plugin-fem --test dbg_capability_claim_diff` → 2 passed, exit 0) — and then `describe` ran clean and surfaced a *different* real bug underneath:
```
dialect:s.stdio.csv@rfc4180/* is already registered by s.fem2d.composer.csv
```
fem2d and fem3d share one plugin and both register composers for the same five stdio dialects. **That is the intra-plugin dialect collision D3 independently identified in procedural and gis — not the capability-claim class D1 filed it under.** Three plugins now, one pattern: sibling artifacts inside a single plugin claiming identical dialects. D1's "capability-claim class (7 plugins)" is measurably smaller than 7, and the real recurring defect is sibling-artifact dialect ownership.

**Independent third confirmation of the `🎒️pack` blocker**: 6 build attempts failing on *different files* in that directory, one surfacing 44 real compile errors — which it used to distinguish "genuinely mid-refactor" from "a race I could retry past". That is the right test, and it matches what D3 (7 retries) and I (direct compile) each found separately.

**Hygiene it did unprompted**: removed every temporary diff-test file and reverted the `pilot_languages()` visibility bumps it had needed to support them. No scaffolding left in the tree.

Where it stopped short and said so: playbook/trinity/puzzle/block rows were fixed **by hand cross-reference** against the real Rust `EXTENSION`/`DOCUMENT_SCHEMA`/`WRITES` constants because the build never came back — correct method, unconfirmed result. `🗄️stdio`'s broken artifact among ~24 remaining was never isolated. Both stated plainly rather than rounded up.

## 🏁️ Session close — state, and what a next session picks up

### Tree status at close: RED, and not ours
`semio-framework-os-kernel` has **17 compile errors** (was 1 when first noticed), from the peer's live `🎒️pack` module rewrite — files deleted while still `#[path]`-mounted in `💻️os/📦️packages/🦀️rust/📦️glue.rs`. Independently confirmed three times tonight: by me (direct compile), D3 (7 retries), D2 (6 builds failing on different files, one with 44 errors). Reported to the peer, **never touched**, on the rule that proved correct earlier today when my intended fix for the identical `🗣️dsl` case would have re-established the exact mechanism they were removing.

**Nothing further can be verified until that lands.** The placeholder guard is therefore **compiles, not fired** — my binary build failed on the same red kernel. That distinction stands unresolved and is stated as such.

### Verified and standing
| | |
|---|---|
| Bench, 50×50 native | **7/8 budgets**; 2550 actors concurrent in 390 MB; 100 actors 12-13 per shard across 8 |
| Descriptors | **21 committed, 11 ratcheted, 0 placeholders** (audited by content, not count) |
| Census (exit item 9) | met — every must-not-exist symbol 0 live |
| Process shards | real `kill -9` → detect → rebuild, sibling survives (3 PIDs logged) |
| `kit.catalog` | resolved with the peer: `🧱️block` declares, consumers reference |
| Parity gate | unblocked (was unrunnable: no `PLAYWRIGHT_BROWSERS_PATH`); runs, not yet green |
| Guards added | `describe` refuses `assembly-failed`; `verify gate` compiles wasm32 bindings |

### Ten defects, one shape
Job completions that could not serialize · WASI never linked · fuel cap 18× low · limiter blocking ALL component instantiation · shard pool putting every actor on shard 0 · four pooling sub-pools defaulting to 1000 · an out-of-spec bench criterion failing a correct runtime · wasm bindings that did not compile · a parity gate that could not launch a browser · three placeholder descriptors committed.

**Every one was invisible to `cargo check` plus mock-backed tests.** Two of them — the limiter and the shard pool — meant the system's two central claims were false while every gate showed green.

### Three taxonomy corrections, each by the agent closest to the evidence
D1 classified the descriptor failures into five causes. **D3 corrected "cross-plugin dialect collision" to intra-plugin** (decisive proof: `process3d` claims the same literal dwg string with zero conflict). **D2 then found `🏗️fem`'s real blocker was that same intra-plugin collision**, not the capability-claim class it was filed under. The genuine recurring defect — **sibling artifacts inside one plugin claiming identical dialects** — had no name this morning and now has three confirmed instances.

### What the next session picks up, in order
1. Wait for the peer's `🎒️pack` rewrite; re-run `cargo check -p semio-framework-os-kernel --lib` to confirm green.
2. **Fire the placeholder guard** against `🏗️fem`'s wasm and confirm it refuses + writes nothing. It is unverified.
3. Re-verify and ratchet the five plugins with applied-but-unconfirmed fixes: procedural, gis, energy, imperative (D3), layout (D2).
4. Resolve the sibling-artifact dialect ownership class (fem2d/fem3d, and re-check procedural/gis) — now a named pattern, not scattered failures.
5. Re-run parity on a quiet tree; then the 58×2 sweep.
6. **Re-measure every test count in this log** — the peer found vitest double-collection inflating figures ~2×, and ~253 tests that never run at all.
7. Then exit items 1, 2, 10.

### 🔧️ Corrections from the `SERVER-FRAMEWORK-PRODUCT` session — including one of mine

**os-kernel is GREEN again**: `cargo test -p semio-framework-os-kernel --lib` **778 passed**, wasm `--lib` clean.

**My attribution was wrong.** I recorded the `🎒️pack` deletions as the peer's `effects-async` packet. They were a *different* session's (`26/08/18/SERVER-FRAMEWORK-PRODUCT`) promotion of the `.spk` container — `📐️format`/`🔌️io`/`⏳️async`/`🌐️http` — out of `💻️os/🔨️modules/🎒️pack` into a product-neutral `🧰️framework/🔨️modules/🎒️pack` (`semio-framework-pack`). I saw `⏳️async` in the deleted paths, matched it to the one live packet I already knew of, and wrote the inference down as fact.

**Second misattribution today** — I also pinned `🧱️block`'s placeholder descriptor on D2 when it was D1's. Both were inferred from an adjacent signal (a matching directory name; which packet happened to be running). **A plausible attribution derived from a name is not evidence** — the same rule this ticket already learned for liveness (`git log` over derived artifacts) applies to authorship, and I did not carry it across.

Also confirmed: the fix was again to **delete** the `#[path]` mounts rather than repoint them (`os_pack` now re-exports). That is twice in one day my intended repair would have re-established the exact mechanism being removed. Not touching a moving target has now paid out three times.

### ✅ The `#[path]` guard I proposed is LANDED — by them, in the owning crate

`every_path_mount_in_this_glue_resolves_to_an_existing_file` in `💻️os/📦️packages/🦀️rust/📦️glue.rs` walks every `#[path]` literal and asserts the target exists. **The sixth-occurrence framing was what decided it belonged in the owning crate rather than a repo-wide lint.** Six identical incidents finally became one named test.

Implementation note worth keeping: resolve against `env!("CARGO_MANIFEST_DIR")`, **not** `file!()`'s parent — cargo runs tests with CWD at the package root while `file!()` is workspace-root-relative; their first version flagged all 40 mounts as missing.

### 📉️ Two more corrections to my measurements

- **`bun install` is broken repo-wide**: root `package.json:249` patches `@electron-forge/core-utils@7.11.2` but `patches/` does not exist, so any clean-box install dies. My "parity is unblocked" claim therefore holds **only on this box** — on a fresh machine the 58×2 sweep is still blocked, environmentally rather than by a parity defect.
- **`💻️os` is 184/185, not the "322/324" in my log**, and the one failure is a missing build artifact (`pkg/semio_framework_os.js` never built), not a regression. Combined with the vitest double-collection finding, **every suite figure in this log is provisional** and must be re-measured rather than restated.

## ✅ THE PLACEHOLDER GUARD FIRED — verified, not merely compiled

Run against `🏗️fem`'s known-`assembly-failed` wasm on the peer's green window:

```
semio-framework-plugin-describe describe: refusing to write a placeholder descriptor for
  …/🎯️target-d2/wasm32-wasip2/debug/semio_s_plugin_fem.wasm: plugin assembly failed —
  dialect:s.stdio.csv@rfc4180/* is already registered by s.fem2d.composer.csv

files written: 0   (0 = guard held)
```

Three things it did, all of them the point:
1. **Refused** — no descriptor written.
2. **Wrote nothing at all** — not a partial file, not a `.semio` without its `.json`.
3. **Surfaced the real error** instead of burying it. The old behaviour produced a 707-byte file that parsed as JSON and read as success while carrying `pluginId: "assembly-failed"`; the new behaviour names the actual defect on stderr.

**And the error it surfaced is D2's finding, independently reproduced**: `fem2d` and `fem3d` both registering a composer for `s.stdio.csv@rfc4180/*` — the intra-plugin sibling-artifact dialect collision, now confirmed by a third route (D3's build output, D2's describe run, this guard).

This closes the last item I had standing as **"compiles but has never fired"**. It is now compiled, fired, and observed to hold — the same three-step bar this ticket has applied to every other claim, applied to my own change without exception.

Sequence worth noting: the guard was written *because* three placeholders slipped through, and the very first thing it caught was a fourth attempt at the same plugin. The defect it prevents was not hypothetical.

### 📨️ A cross-session message that was NOT for us — and two things worth keeping from it

A message arrived addressed to a `SERVER-FRAMEWORK-PRODUCT` session ("if that was not you, please ignore"). It was not us and we acted on none of its requests. Three items are still worth recording, because they resolve open unknowns:

**1. The `🎒️pack` mover is a THIRD session.** Three of my builds died inside `semio-framework-os-kernel` on `#[path]` targets vanishing mid-move, and I could not attribute them. It is `SERVER-FRAMEWORK-PRODUCT` promoting `🎒️pack` — not the peer I had been corresponding with, and not our `effects-async` packet (that peer had itself misattributed the deletions to our packet, then corrected it unprompted). Attribution now settled; we remain gated on their move and are editing nothing of theirs.

**2. The `plan_workflow` failure has a real root cause, and it is not a code defect.** They traced it to `cannot resolve …/pkg/semio_framework_os.js` — **`pkg/` was never built**. This log has carried that failure for a full day as a vague "pre-existing wasm-artifact failure", which is a classification, not a diagnosis. It is a **missing build artifact**, therefore fixable, and it should stop being excused in every baseline. Added to the W9 list rather than fixed now (it needs a wasm build on a machine at 19 cargo processes).

Their figure was `184/185`; mine, measured minutes earlier, is **184 passed / 2 failed** — two *distinct* failures in different files (`🟦️component.ts`'s `plan_workflow…decoded via wasm` and `🟦️backbone-worker.ts`'s `decodes the Rust-generated binary wire fixtures byte-identically`). Both plausibly share the missing-`pkg/` cause. I told them so, including that **I made the mirror-image error earlier today** — grepped one of the two names, saw hits, and recorded "one failure, doubled".

**3. A zone-overlap risk I pushed back on.** Their stated zone — `🔌️plugin/**`, wgpu, `🎭️actor`, `✏️s/🔌️plugins/**`, root `📜️script.ts` — is almost exactly where our live packets are (`effects-async` in the host file, `kernel-loop` in wgpu glue, `web-shardframe` and `web-plugin-runtime` in the TS trees). That statement may have been aimed at the server session rather than at us, so I enumerated our live files precisely and asked them to name theirs if the overlap is real, offering to sequence around them. Two sessions in one file is the failure this ticket has absorbed four times from others; inflicting it on ourselves would be worse.

**Also flagged for us:** a `DslValue` promotion in flight touches `dsl::from_dsl_value`, which sits on our descriptor path in `📇️describe/📦️glue.rs` and `🏃️run/🦀️component.rs`. No action yet — recorded so a later failure there is not mistaken for our own.

A note on method, since both sessions hit it today: they twice inferred authorship from an adjacent signal (a matching directory name, whichever packet happened to be running) and were twice wrong, and said so unprompted. Our equivalent was copying an acceptance command from a sibling packet without reading the manifest, reintroducing an error this ticket had already documented once. **A plausible attribution derived from a name is not evidence** — the same sentence covers both mistakes.

### 🧩️ `kit.catalog` change — landed, NOT verified (7th blocked window)

`cargo check -p semio-s-plugin-puzzle --lib` → `error[E0432]: unresolved import `crate::os_dsl::schema::WireNode``.

**Not mine**: my edit removed one `.artifact_kind(crate::artifacts::puzzle3d::kit_catalog_artifact_kind())` line, which cannot produce a schema-type import error. This is the third session's `DslValue` promotion out of `🗣️dsl/🧬️schema`, which they warned me about by name — arriving between my check starting and finishing.

So the `kit.catalog` resolution is **applied and reviewed, not compile-verified**. Stated as such rather than assumed green because the change is small: the ticket's own standard is that a small change is not a verified change.

**Seventh instance of the tree going red under a moving refactor today** — dsl (span/diagnostic), pack (×3 files), now schema (WireNode). Three separate sessions, one shared tree. The `#[path]`-mount guard the server session landed will catch the first class; import-surface promotions like this one it will not.

### 📊️ os figures corrected again — by measurement, not by me

`sol` measured `💻️os/📦️packages/🟦️typescript` at **184 passed / 2 failed** — two distinct failures in different files (`🟦️component.ts` → `plan_workflow … decoded via wasm`; `🟦️backbone-worker.ts` → `decodes the Rust-generated binary wire fixtures byte-identically`). I had "184/185" second-hand from the server session and was one command from recording it.

My `pkg/`-never-built diagnosis plausibly explains **both**, since both are artifact-dependent — but plausibly-explains is not measured, so it is recorded as **2 failures, both suspected missing-artifact, unconfirmed**.

**Two sessions made mirror-image counting errors on the same suite within an hour** (they grepped one of two names and recorded "one, doubled"; I took a second-hand figure). That is the strongest argument yet for the standing rule: re-run, never restate.

### 🔧️ Two corrections from the `SERVER-FRAMEWORK-PRODUCT` session — one of them mine again

**1. "Three separate sessions" was wrong — it is TWO.** I recorded the dsl promotion, the `🎒️pack` move, the `#[path]` guard and the `DslValue` promotion as evidence of three concurrent sessions churning the tree. **All of it is one ticket** (`26/08/18/SERVER-FRAMEWORK-PRODUCT`), plus `sol`'s async-first W5+ work. Third attribution error today, same mechanism as the first two: I inferred a distinct actor from a distinct-looking symptom. The tree was never as chaotic as my log said — one session was executing a coherent promotion sequence and I read each step as an unrelated event.

**2. My descriptor path survived `DslValue`, and they measured it rather than assuring me:**
```
cargo check -p semio-framework-plugin-describe → Finished (clean)
cargo check -p semio-framework-os-run          → Finished (clean)
```
`dsl::from_dsl_value`/`to_dsl_value` still resolve at exactly `📇️describe/📦️glue.rs:140,151,155` and `🏃️run/🦀️component.rs:31`. The mechanism is a facade: `DslValue` and the 716-line serde bridge moved to `🧰️framework/🔨️modules/🌱️value`, mounted once by the replication crate, with `os_dsl::schema` re-exporting — so every historical path keeps resolving. **The schema was red for about four minutes**, not the long window I had braced for.

### 📊️ The os TS figure resolved — and the resolution is more interesting than either number

Three sessions produced three counts for one suite: 322/324 (mine, stale), 184/185 (theirs), 184/2 (`sol`'s). **All three were correct for the tree state they sampled.** Their 184/185 predated moving the wire-fixture test out of `🟦️backbone-worker.ts`; it has since moved to the replication package (`🧫️fixtures/wire`, 20 frames, passing there).

As they put it: **the suite identity moved under us** — which is worse than one party being wrong, because every reading was defensible. Final recorded state: **one genuine remaining failure** (`plan_workflow … decoded via wasm`), root cause confirmed independently by both of us as `🖥️host/📦️packages/🦀️rust/pkg/` never having been built in this tree. **A missing build artifact, not a code defect** — so it is fixable rather than something to keep excusing in every baseline.

### Zones — no overlap after all
They have not been in `🎭️actor` (or its TS package), wgpu, `🖥️host/🧵️shard`, `📇️describe`, `🛂️manifest`, or anything under `✏️s/`. Only shared interest is `📇️describe/📦️glue.rs`, read-direction only: they keep it compiling, they do not edit it.

**Their remaining sequence, for planning**: `store::pack_rt` out of `🏪️store`, then `db_engine`'s `vcs_integration` via the `🕸️version-graph` seam, then relocating `db`. `🏪️store/🦀️component.rs` is the one likely to sit on someone else's path. Same protocol: short red windows, mounts deleted rather than repointed.

### ✅ kernel-loop — wgpu GREEN, a real parallel kernel, and the bench blocked by our own async syntax

**Step 0 achieved:** `cargo check -p semio-framework-os-renderer-wgpu --lib` → **exit 0**. It did not merely repair the two broken `pump` call sites; it replaced them with the real mechanism.

Delivered `🎯️targets/🧊️wgpu/🎠️runtime.rs` — `ParallelRuntime`: `Kernel::submit` → `tick` → dispatch to **K real `ShardExecutor` OS threads** → collect outcomes over an aggregated multiplexed channel → **`Kernel::complete`**, including synthesizing a `Faulted` result for trap outcomes **so the failure ladder finally observes them**. Both native call sites — the winit interactive host and the scale-bench harness — now run through that one engine, so "K shards run in parallel" is the same real mechanism in production and in the bench rather than two lookalike paths. K comes from `semio_framework_async::thread_plan`, not a fresh ad-hoc formula. `Kernel::new(Thread, K, 2, 64)` replaces the single-shard `(…, 1, 0, …)`.

Coordinator-relevant numbers from its run: `cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity` → **113 passed / 0 failed** (was 100). `--all-targets` fails only on a pre-existing `Dock` test-module break from another session's localization/presence work, which it never touched.

**Its load-bearing finding, which would have been a silent disaster:** `Kernel::activate` takes **no per-actor budget** — it always applies `lane_defaults::budget_for(lane)`. Dispatching `tick`'s own computed budget verbatim would have **fuel-starved almost every real turn** (the lane default's 2M fuel against the 50M/200M these hosts actually need), and the symptom would have been mysterious mid-turn traps, not an obvious error. Fixed with a caller-supplied budget resolver; DRR's turn/shard *selection* is preserved and throttle-scaling of the budget itself is left as a documented gap rather than faked.

#### 🐛️ The bench is blocked — by `async-worlds`, through a generator I failed to enumerate

The scale-fixture guest wasm no longer compiles: a WIT parse error on `async func` syntax. kernel-loop confirmed it across two runs 15 minutes apart, diagnosed it read-only, never edited the file, and reported budget 5 as **unmeasured rather than fabricated**. Exactly right.

Root cause, found by me: **`💻️os/🧫️fixtures/🔌️scale` pins `wit-bindgen 0.36.0` and has its OWN `generate!`** over the same `🧬️schema` package — deliberately not depending on the SDK, as its own header comment explains. When I sequenced the guest-generator bump ahead of the async syntax I checked *the SDK's* `generate!` and missed this second one. My grep for `wit_bindgen::generate!` under-reported it because this crate, like the SDK, writes `use wit_bindgen::generate;` then bare `generate!({…})`.

**That is the third time today a too-narrow query produced a confidently wrong picture** — after the grep that made me record two distinct test failures as one doubled failure, and a `find -newermt` that returned nothing while `ls` showed a one-minute-old file. Bumped the fixture to 0.57.1 with a comment stating the invariant: *any crate with an independent `generate!` against that schema must move in lockstep, because there is no per-world parsing.* Rebuild in flight.

The fleet itself was never at risk — the 33 plugin crates go through the SDK, which is on 0.57.1 and verified green. Only this fixture had a private generator.

### 🔬️ A near-miss of my own: PluginRuntime's tests are real

Chasing web-plugin-runtime's numbers, three separate shell greps on `PluginRuntime/🟦️component.tsx` returned **empty output** — no matches, no `0`, nothing — while `ls` resolved the same path fine. I was one step from recording that a packet had reported 26 tests that do not exist. Settled with python instead of the shell: **3 `import.meta.vitest` blocks, 6 `describe(`, 30 `it(`**. The tests are real and in-source; my shell was silently failing on the emoji path.

The lesson generalises past emoji: **an empty result from a query that cannot report its own failure is not evidence of absence.** Same shape as the two misses above. Where a negative result would change a judgement about someone else's work, confirm it with a second, independent tool.

### 🧰️ My own gate was broken — and it was the same failure shape, a fourth time

The file-existence gate I built to avoid chasing the peer's move ran **70 polls (~35 min)** and reported `PATHS_STABLE=0 … last_missing=.` — i.e. it never cleared. Read literally, that says the peer is still moving files. It is not what happened.

`.` cannot be a file, so my `[ -f "$p" ]` test could **never** succeed and the loop was structurally incapable of finishing, independent of the peer. Re-parsed with python: `💻️os/📦️packages/🦀️rust/📦️glue.rs` declares **47** `#[path]` targets, of which the 7 "missing" are all literally `#[path = "."]` — each immediately followed by `pub mod … {`, which is the ordinary Rust idiom for anchoring an inline module's **directory**, not a file reference. Every one of the **40 real file paths resolves**. The peer's consolidation has landed; my instrument was broken.

That is the fourth instance today of one shape: a **too-narrow or silently-failing query mistaken for a finding**. The grep that turned two distinct test failures into "one doubled". A `find -newermt` returning nothing while `ls` showed a one-minute-old file. A `grep wit_bindgen::generate!` that missed the scale fixture's private generator and cost the bench. And now a gate that could never pass. Three of the four would have produced a **confidently wrong conclusion about someone else's work** if I had not cross-checked with a second tool.

Stated as a rule, because "be careful with grep" is too weak to act on: **when a negative result would change a decision — especially about another session's or packet's work — reproduce it with a differently-implemented tool before believing it.** Shell globbing, `find` predicates and `grep` over emoji paths have now each silently under-reported in this session; python over explicit absolute paths has not.

`effects-async` verification is running against the now-coherent tree. It remains the wave's one genuinely unverified deliverable.

### ✅ effects-async COMPILES — the wave's one unverified deliverable is verified

```
cargo check -p semio-framework-plugin-host --lib   → CARGO_EXIT=0, 0 errors
```

The packet had reported **zero confirmed compiles** and could not get a build through: the harness detached its checks at the 120 s default, and three of my own attempts then died on an unrelated peer restructure. In that gap it did a line-by-line self-review and found **three real defects, one a hard compile error** (`scope` moved into an `async move` block while still needed afterwards for `spawn_scoped(&scope, …)`). That self-review is why this now compiles on the first clean attempt — it fixed the error that would otherwise have surfaced here.

Worth being precise about what this does and does not establish: the async effect executor, its `OperationContext` derivation, the routing to `HttpPool`/`StorageScheduler`/`TimerWheel`/`ComputePool`/`EventRouter`, the cancellation matrix and the per-instance `EffectBackbone` are now **type-coherent against the real service crate and the real kernel types**. They are not yet *proven to run* — that is the test suite, currently executing. This ticket's most expensive recurring lesson is exactly that gap, so the distinction stays explicit.

Also confirmed: its `🔀️PostTurnRelay` verdict of **KEPT** was right — it is a job-execution primitive `IoRouter`/`ArtifactInferenceRouter` still depend on, not a live per-turn dispatcher, and retiring it would have broken working code to satisfy a plan written before that was known.

### 🔧️ Scale fixture past the WIT barrier

The 0.57.1 bump is compiling (`wit-bindgen v0.57.1`, `wasmparser v0.247.0` building, no errors) — it has cleared the `async func` parse failure that blocked it. If it produces an artifact, the bench becomes runnable and **budget 5 gets a real number on a valid instrument for the first time**: K genuinely parallel shard threads via `kernel-loop`'s `ParallelRuntime`, instead of one physical loop behind K labels producing 30 samples inside a 0.1 ms band.

### ✅ Scale fixture rebuilt — bench unblocked, artifact is a REAL component

```
cargo build -p semio-framework-os-scale-fixture --target wasm32-wasip2 → FIXTURE_BUILD_EXIT=0, Finished in 10m 21s
magic bytes: 00 61 73 6d 0d 00 01 00  → component (not a core module)
```
The one-line `wit-bindgen 0.36.0 → 0.57.1` bump on the fixture's private `generate!` cleared the `async func` parse failure. Checking the magic bytes rather than trusting "the build succeeded" matters here: a core module and a component both come out of a successful `cargo build`, and only the second can be instantiated by the host — this ticket has already been bitten by an artifact that compiled into something the runtime could not load.

**Native scale bench launched** with `--shards 4` against `kernel-loop`'s `ParallelRuntime`. This is the first time every precondition has existed simultaneously: a live `Kernel` loop calling `tick`/`complete`, K real `ShardExecutor` OS threads rather than one physical loop behind K labels, DRR budgets travelling in `ShardFrame::Grant`, and a fixture component that builds against the WASI-0.3-bearing schema.

What I will and will not claim from it, decided **before** seeing the number so the criterion cannot bend to the result: budget 5's threshold is p95 ≤ 8 ms native. The previous archived run reported 295 ms with **30 samples inside a 0.1 ms band** — a constant, i.e. a broken instrument, not a latency measurement. If the new run's samples show real spread, the instrument is valid and **whatever it reports is the honest result, pass or fail**. A valid-instrument failure is a publishable design finding and will be recorded as such; it will not be "fixed" by tuning the harness.

### 🐛️ W0 fallout in the scale fixture — and I produced the false green that hid it

The bench failed at its **own** fixture build step, which passes `--features component-guest`:
```
error[E0560]: struct `RequestCapabilityEffect` has no field named `id` / `scope` / `reason` / `optional`
```
`W0-params` moved those four fields into `request-capability-params`, leaving `request-capability-effect { req, params }`. The fixture's feature-gated guest code still constructed the flat shape.

**Why three separate green signals missed it:**
1. `W0`'s acceptance covered `semio-framework-plugin` and `semio-framework-plugin-host`. The scale fixture has its **own** `generate!` and is a separate crate — outside every command in its brief. My brief, my omission.
2. `async-worlds` and the guest-generator bump likewise never built this crate.
3. **My own fixture build "succeeded" only because I omitted `--features component-guest`** — the exact feature gating the broken code. I then reported "fixture builds, bench unblocked, artifact is a real component" and even checked the magic bytes. All true, all irrelevant: I verified an artifact built from a code path that excludes the defect.

That is this ticket's signature failure — *a check that structurally cannot observe the defect* — and this time **I** authored it, one message after writing that magic-byte checking matters because a successful build proves less than it appears to. The correct instrument was never "does the crate build?" but "does the crate build **the way the bench builds it**?" Reproducing the consumer's exact command is what found it in one step.

Fixed (2-line change, the fixture is this ticket's own asset): import `RequestCapabilityParams` and nest the four fields. `cargo build -p semio-framework-os-scale-fixture --target wasm32-wasip2 --features component-guest` → **Finished in 2.96s**. Bench re-launched.

**Standing correction for the exit checklist:** any acceptance that names a crate must run the command **its consumers actually run**, feature flags included. A `cargo build` without the feature that gates the code under test is not evidence about that code.

### ✅✅ effects-async FULLY VERIFIED — the async effect layer runs, not just compiles

```
cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity
  → 113 passed; 0 failed; 1 ignored — TEST_EXIT=0
```
113 matched `kernel-loop`'s figure exactly, which is the kind of coincidence that deserves suspicion rather than relief, so I applied this ticket's own "prove it by name" rule to my own verification instead of accepting the total. The tests are there, and they are **properties, not mechanisms** — which is the standard the brief set precisely because this ticket has twice been fooled by suites that passed against a broken runtime:

| required property | test that proves it |
|---|---|
| revocation cancels only its own ops, actor survives (bench budget 8) | `revoked_capability_cancels_only_its_own_operations_and_actor_survives` |
| stale completions dropped after restart | `stale_generation_completion_is_dropped_current_generation_is_delivered` |
| suspend buffers rather than loses | `park_buffers_completions_and_resume_delivers_them_in_order` |
| completion floods hit mailbox bounds | `completion_burst_while_parked_is_bounded_not_unbounded` |
| quota denial is typed, never a panic | `storage_quota_denial_produces_a_typed_completion_not_a_panic` |
| deadlines enforced, loser cancelled | `an_effects_deadline_is_enforced_and_the_loser_is_cancelled` |
| backbone requires its capability | `backbone_send_is_rejected_without_the_capability` |
| store deltas coalesce per URI | `backbone_delta_fanout_coalesces_a_burst_for_the_same_uri` |
| jobs stay with the shard, are not re-dispatched | `spawn_job_and_cancel_job_are_reported_shard_owned_never_dispatched` |
| routers run off the async workers | `router_effect_runs_through_compute_pool_and_completes_ok` |

**This is the reference architecture's core claim, now executing.** Effects leave a turn as data; the executor derives an `OperationContext` (actor, generation, trace, lane, deadline, a child cancel token, capability) and spawns each into that actor's scope — no detached spawn is expressible, because `spawn_scoped` requires a `ScopeHandle`. Real resources stay host-owned (one timer wheel, pooled HTTP, a bounded storage scheduler, a semaphore-bounded compute pool), and completions re-enter as ordinary envelopes so a flood meets the same coalescing and backpressure as any other traffic. The per-instance `EffectBackbone` closes the guest↔store gap that had **no path at all** since the process-global channel was deleted.

Notably, the packet that produced this never got a single successful build of its own — the harness detached its checks and an unrelated peer restructure blocked three of mine. It responded by self-reviewing line by line, finding three real defects including the compile error, and reporting **zero confirmed compiles** rather than implying success. Both the compile and the full property suite then passed on the first clean attempt.

### ✅ `kit.catalog` resolution VERIFIED — both sides compile

```
cargo check -p semio-s-plugin-puzzle --lib  → PUZZLE EXIT=0
cargo check -p semio-s-plugin-block  --lib  → BLOCK EXIT=0 (Finished in 48.82s)
```

Consumer (puzzle, declaration removed, port references kept) and declarer (block, unchanged) both green. The last item I was carrying as *applied but unverified* is closed.

**A method note on how nearly I mis-recorded this.** My first attempt used `cargo check -q`, whose quiet mode suppresses the `Finished` line — so the grep returned nothing and the result *looked* like "no errors". That is absence of evidence, not evidence of success, and I would have written it down as a pass. Re-ran without `-q`, capturing explicit exit codes.

That is the same failure shape as this session's three biggest finds, arrived at from the opposite direction: **a green signal that was structurally incapable of being red.** The mock-backed tests that never loaded wasm, the census that could not distinguish two meanings of `exchange`, the descriptor gate reading a gitignored path — and now nearly my own verification. Worth recording precisely because I caught it in myself with two commands left to go.

## 🏁️ THE BENCH RAN ON REAL PARALLEL SHARDS — thesis proven, budget 5 measured honestly

`bench plugins --renderer native --count 50 --extensions 50 --shards 4` → exit 0, archived `🔣️bench-w5-parallel.json`.
**`1:pass 2:fail 3:fail 4:pass 5:fail 6:pass 7:fail 8:pass`**

Against the archived `🔣️bench-native-FINAL.json` (7/8) this reads as a regression, and **partly it is** — but the two runs are not comparable instruments, and the difference is the point of the whole wave.

### ✅ Budget 3 — the ticket's thesis, now measured on real threads

```
perShardCounts {"0":25, "1":25, "2":25, "3":25}   maxShardLoad 25   ceiling 26   activeActors 100/100   shards 4/4
```
Textbook balance across **four real OS threads**. The same measurement earlier in this ticket read `{"0": 100}` with a single physical loop wearing K labels. Every specified quantity passes; the row is red only on a `faultCount` of 2 whose messages were not captured.

### 📉 Budget 5 — a VALID instrument at last, and an honest failure

| | old run | this run |
|---|---|---|
| samples | 30 within a **0.1 ms** band | 30 spanning **146.4 – 242.3 ms** (spread **95.8 ms**) |
| p95 | 295 ms | **217.9 ms** |
| verdict | failure with a **known-invalid** instrument | **failure with a valid one** |

The old number was a constant, which is not a latency. This one has genuine variance, so it is measuring something real for the first time — and the real answer is **p95 217.9 ms against an 8 ms native target, ~27× over.**

I committed to the interpretation **before** seeing the number, and it stands: this is a publishable design result, not something to tune away. The likely mechanism is arithmetic rather than mysterious — 40 `cpu` actors over 4 shards is 10 per shard, and an interactive turn pinned to a shard queues behind that shard's CPU-bound turns; 10 busy-looping actors' declared milliseconds lands squarely in the 146–242 ms band observed. **K parallel shards alone do not protect interactive latency.** The design already anticipates the remedy — `request_exclusive` plus the 2 reserved exclusive shards now configured by `Kernel::new(Thread, K, 2, 64)` — but nothing yet moves CPU-bound actors onto them or keeps interactive actors off saturated shards. That is the next real piece of work, and it is now backed by a number instead of an assertion.

### 🐛️ A real bug the parallel runtime exposed — dispatched as `shard-routing`

```
budget 2: 6 × "ShardLoop::pump: actor <id> is not registered on this shard"   (also 2717 ms vs a 1500 ms ceiling)
budget 6: same message on actor 0
budget 7: checkpointHash == resumedCheckpointHash, identical=True — but resumed=False
```
With one shard every actor was trivially "on this shard", so this class of defect **could not exist** before today; K shards make routing load-bearing. Budget 7's state serialization is provably fine (hashes match) — the resume simply never happens, which is almost certainly the same routing fault.

The fix packet is explicitly told **not** to make `pump` tolerate unregistered actors: a grant arriving at the wrong shard is a routing bug, and swallowing it would convert a loud fault into a silently lost turn. It must keep least-loaded pinning (that is what produced the perfect distribution) and add a **property** test — every actor's grant arrives at the shard it is pinned to, across a suspend→resume round trip — because this ticket has twice shipped bugs that survived round-trip-style tests.

### Also measured

- **Budget 4 passes at 4 shards**: 2550/2550 actors live, **598 MB RSS** against a 1.5 GiB ceiling (390 MB at 1 shard — the increase is 4 real shard threads, still under 40 % of budget).
- Budget 1: 2.85 ms, 2550 records, **0 instantiations**. Budget 6: hang killed on its `instance-open` turn, 3 siblings restored, 2 ms pause. Budget 8: capability requested, revoked at runtime, actor survived both the revoke turn and a follow-up, status `Idle`.

## 🏁️ shard-routing — 7 of 8 budgets pass on REAL parallel shards

Coordinator-verified, not taken from the report:
```
cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity → 115 passed; 0 failed; 1 ignored — exit 0
bench --shards 4 --count 50 --extensions 50 → 1✅ 2✅ 3✅ 4✅ 5❌ 6✅ 7✅ 8✅
  budget 2: faultCount 0        (was 6)
  budget 3: perShard {0:25,1:25,2:25,3:25}   (balance preserved, not regressed by the fix)
  budget 7: resumed=True, identical=True     (was resumed=False)
  budget 5: p95 241.2 ms, spread 102.1 ms    (still failing, deliberately untouched)
```

### The root cause was not what any of us assumed — including me

I briefed this as a routing bug ("a grant is reaching a shard where the actor was never registered"). It was **not**: `ShardTable::pin` placement was correct and consistent across `Kernel::activate`, the `Scheduler`, and `ParallelRuntime` the whole time. The real defect was a **cross-thread ordering race introduced by this same wave**: `ShardExecutor::register` sent its request over an **independent `mpsc` channel and returned immediately**, while the executor thread drains that channel only once per loop iteration before parking on the *separate* `ThreadTransport`. So `activate` → `submit` → `tick_and_dispatch` could have the transport send wake the parked executor and reach `pump_primed` **before the registration was ever drained**.

The fault text said "is not registered on this shard" and it was literally true — just not for the reason the words suggest. Had the packet accepted my framing, it would have gone hunting through pinning logic that was already correct. Fixed properly by making `register` block on a real ack (`RegisterRequest{actor, instance, ack}`), establishing a genuine happens-before against any frame sent afterwards, with least-loaded placement untouched. **One file changed.**

Budget 7 was the same race in `Env::activate` — confirmed rather than assumed, and its checkpoint hashes had always matched, so the state machinery was never at fault; only the resume never happened.

### The test validation is the part worth copying

It did not merely add two property tests — it **temporarily reverted the fix and watched them fail with the exact bench fault text, then restored it.** That is mutation testing against the real defect, and it answers the question this ticket has been burned by three times ("does this test actually observe the bug?") with evidence instead of assertion. Both confirmed by name in my own run:
- `every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards` (K=4, 200 actors)
- `suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered` (60 actors)

### What 7/8 means now versus what it meant before

The archived `🔣️bench-native-FINAL.json` also read 7/8 — but that scoreboard was built on a weaker instrument in two places: budget 3 "passed" against a single physical loop wearing K labels, and budget 5 failed with 30 samples inside a 0.1 ms band. **Today's 7/8 is the same score on a genuinely different machine**: four real OS threads with measured 25/25/25/25 placement, and a budget-5 number with 102 ms of real spread behind it.

**Budget 5 remains the one honest failure: p95 241 ms against an 8 ms target.** It moved 218 → 241 ms across two runs, which is ordinary variance at this magnitude and emphatically not a regression to chase. It was not tuned, not skipped, and not reframed. The design gap it measures is specific and actionable: nothing yet keeps interactive actors off shards saturated by CPU-bound ones, even though `Kernel::new(Thread, K, 2, 64)` now reserves two exclusive shards and `request_exclusive` exists to use them. That is the next packet, and for the first time it starts from a number rather than an assertion.
