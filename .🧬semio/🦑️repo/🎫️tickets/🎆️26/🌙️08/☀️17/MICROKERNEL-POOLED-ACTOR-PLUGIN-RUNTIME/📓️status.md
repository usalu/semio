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

### 🔓️ `bun install` FIXED — repo has been uninstallable on a clean box since 2026-08-06

Verified myself rather than taking the peer's report:
```
bun install --dry-run
error: Couldn't find patch file: 'patches/@electron-forge%2Fcore-utils@7.11.2.patch'
```

**The evidence chain, which made the fix safe rather than a judgement call:**
- Root `package.json:249` declared `patchedDependencies` for `@electron-forge/core-utils@7.11.2`, added **2026-08-06** (`git log -S`).
- `patches/` has **never been tracked in git** (`git ls-files | grep -c '^patches/'` → 0), and is **not gitignored** — it was simply never committed.
- It does not exist on this box either. **So the patch is not applied anywhere, for anyone, right now.**
- `@electron-forge/core-utils` is not a declared dependency at all; it arrives transitively via `compose/client/ui/desktop`.

That last chain is what made removal safe rather than a trade-off: **you cannot regress behaviour by deleting the declaration of a patch that is not in effect.** Removing it changes nothing about installed packages; it only stops bun from refusing to resolve. Keeping it guaranteed failure for every clean checkout.

Removed the whole `patchedDependencies` block (asserted first that the electron-forge entry was its ONLY key, rather than deleting blind). `bun install --dry-run` now completes in 2.66s; `git diff --stat` shows **3 deletions, no other churn**.

**Seventh instance of this ticket's signature pattern, and the first outside Rust**: *the artifact was created, its registration was committed, the artifact never was.* Previous six were mount tables, workspace members, struct initializers, generated-file paths. This one is a package manager's patch table — same shape, different toolchain.

**Consequence for exit item 3**: my earlier "parity is unblocked" claim was true only on this box (where `node_modules` predates the breakage). It is now true on a clean checkout too. If the desktop app genuinely needs that patch, it must be **committed to `patches/`** — flagged to peers rather than silently reinstated.

## ✅ SLICE A COMPLETE — the WASI 0.3 ABI is landed and all four gates pass

```
gate 1  guest  cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest → exit 0
gate 2  host   cargo check -p semio-framework-plugin-host --all-targets                                → exit 0
gate 3  RUN    cargo test  -p semio-framework-plugin-host --lib schema_parity → 4 passed; 0 failed — exit 0
gate 4  emit   cargo check -p semio-framework-plugin-describe --all-targets   → Finished in 25.65s, exit 0
```

### Gate 3 — the schema was right, the test was wrong, and only *running* it could tell

On first execution the parity suite failed 3 of 4:
```
`host-async.blob-load` param `params` must reuse the SAME type as `blob-load.params`
  left: Id(168)   right: Id(34)
`emit`'s parameter must be THE SAME `effect` type `effects.effect` defines, not a copy
  left: Id(187)   right: Id(103)
```
Read literally, that says the async world **copied** the payload records instead of sharing them — precisely the drift the `*-params` refactor exists to prevent, and a serious finding if true.

It was not true. `interface host-async` genuinely does `use effects.{storage-read-params, …, blob-load-params, …, effect}`. But `wit-parser` materialises a `use`d type as a **fresh `TypeDef` whose kind is `TypeDefKind::Type(original)`** — an alias — so two genuinely-shared types compare unequal by raw `TypeId`. The test compared raw ids; it had to resolve alias chains to their root first.

Fixed by me: a `canonical_type(resolve, ty)` helper that walks `TypeDefKind::Type` to the defining type, applied at all three comparison sites. **4/4 pass.**

Two things worth keeping from this. First, it is the second time this same parity suite was **too strict** — the first was asserting `world actor` imports *only* `pure` when type-only `use` legitimately materialises six interfaces. A test that asserts something which cannot be true is worse than no test, and both defects were invisible until the suite actually executed: `--all-targets` **compiled** these tests clean while they were wrong. Second, it vindicates insisting on gate 3 separately from gate 2 rather than accepting "it compiles" as coverage — this ticket's most expensive recurring lesson, now paid forward instead of paid for.

### Slice A final state
One WIT package, two worlds: `world actor` (poll/compat, untouched, still what all 33 plugins build against) and `world actor-async` (WASI 0.3 — `async func`, `stream<u8>` HTTP bodies, `run(events: stream<event>)`), sharing every record by construction. `host-async` reuses the `*-params` records rather than redeclaring them, and a **running** parity test now enforces that mechanically. Toolchain: wasmtime **47.0.3**, wit-bindgen **0.57.1** (SDK **and** the scale fixture's private generator, which had to move in lockstep).

## 🚀️ Final wave dispatched — five packets, no stopping

| packet | mission |
|---|---|
| `interactive-isolation` | **budget 5**: stop interactive turns queueing behind CPU-bound ones. Given the measurement (p95 241 ms, valid instrument) and the unused primitives (2 reserved exclusive shards, `request_exclusive`, lane deadlines, count-blind `pin`), told to **design the mechanism itself** and forbidden from tuning the bench or threshold |
| `web-effect-backbone` | the TS counterpart to the verified Rust `EffectBackbone`, conforming to it field-for-field with a cross-language parity test |
| `directory-and-run` | the last synchronous holdouts: `AppChannelHost::exchange`'s blocking duplex, and the directory client's blocking-`ureq`-on-a-thread + `block_on` + **private tokio runtime** (tokio must exist only in the services crate) |
| `shell-unpark` | the last UI-thread parks — scoped write access granted to registrar-only `Shell/🧊️component.rs`, with the `Rc<ShellIo>` split that resolves the `RefCell` borrow conflict which blocked an earlier attempt |
| `web-shellhost` | scoped access to registrar-only `ShellHost/🟦️component.tsx`: non-blocking effect batches, bounded install concurrency, event-driven identity wait replacing a fixed 2 s sleep, abortable fetches |

Both registrar-file grants carry the same hard conditions: surgical region-scoped edits, re-read from disk before each edit, exhaustive line-range reporting, and no reformatting of adjacent code — the two files are shared with live presence/hover/dock tickets.

### ✅ shell-unpark — 3 of 3 named parks removed, and it corrected my analysis twice

**All three targets are gone from the live tree** (census run by me, not taken from the report): the hot-reload `boot()` park and the `pump_sync_events()` park in `📦️glue.rs` both became `spawn_app_task` via `self_weak`, and `Shell/🧊️component.rs`'s `LoadDocument` park became a self-contained `spawn_app_task`. `📦️glue.rs:1517` now carries only a comment recording what used to block there.

**Correction 1 — my census was wrong about what a "park" is.** I handed it five surviving non-test `block_on` sites and asked it to classify them. It read each call site and found my framing was too coarse:
- `1783`/`1796`/`1806`/`1810` are **all inside one function**, `poll_pending_assets` — a single `ControlFlow::Poll`-coupled exclusion the brief already named, not four open questions.
- `2467` (`run_smoke`) is a **headless CLI entry point with no winit loop at all** — never a UI-thread park.
- `3206`/`3263` are inside **`std::thread::spawn` closures already on dedicated background threads** — also never UI-thread parks, and independently `directory-and-run`'s to retire.

The distinction matters and I had blurred it: the objective is *the UI thread never waits on a plugin*, so a `block_on` on a background thread or in a headless binary is not the same defect. Grepping for `block_on(` finds parks; only reading the call site tells you which thread is blocked. **The census I published overstated the remaining work.**

**Correction 2 — it declined my design sketch, correctly.** The brief specified an `Rc<ShellIo>` + `VecDeque<SyncEvent>` split to resolve the `RefCell` borrow conflict that defeated an earlier attempt. On reading the code it found the machinery unnecessary: `AppRuntime::self_weak` already covers both `glue.rs` sites, and the `Shell` site needs no shared state after firing (`ProgramBridgeEntry` is `Clone`; `pack`/`spr` are already owned). Building the sketch anyway would have meant sprawling edits across a **registrar-shared file** to satisfy a plan the code had already made redundant. The brief said "adapt if the code disagrees — the code wins", and it did exactly that instead of following instructions off a cliff.

Edits are correspondingly tiny for a registrar-shared file: `Shell/🧊️component.rs` **1697–1718 only**; `📦️glue.rs` **1517–1556** and **1590–1601**. `ControlFlow::Poll` confirmed untouched (still `Poll` at 2270, no `Wait` anywhere). It reused the `self_weak`/`try_borrow_mut()` pattern already established at the `on_context_menu`/camera-dispatch call sites rather than inventing a second convention.

**Honest position on verification**: it marked all three acceptance commands **UNRUN** and stopped its detached build and monitor when told, rather than letting them run unobserved and reporting numbers it had not seen. Its stated confidence comes from a manual signature check against the real type definitions and the pre-existing call sites using the same pattern — explicitly *not* a build result. It also declined to confirm the pre-existing `Dock` break it never reproduced, recording "not contradicted, not confirmed by me". Coordinator-run wgpu verification is in flight.

### ✅ web-effect-backbone — the guest↔store path exists on BOTH sides now

Coordinator-verified: `💻️os/📦️packages/🟦️typescript` → **206 passed / 1 failed (207)**, real exit 1 from the single remaining pre-existing failure.

New `💻️os/🟦️effect-backbone.ts`: a **per-instance** `EffectBackbone` (never module-global — a process-global is exactly what was deleted and cannot survive pooled multi-instance actors), capability-gated sends on `messaging.backbone:<uri>`, latest-wins delta coalescing, and lossless reject-and-report send queues built on the already-landed `createBoundedMailbox` rather than a second queue implementation. It routes through `🟦️backbone-worker.ts`'s existing `open`/`send`/`publishPreview`/`preview` protocol instead of opening a second path to the hub, and **edited neither that file nor `🟦️component.ts`**. Its wire mirrors are machine-parity-tested against the Rust source, so drift fails loudly — the same protection `web-shardframe` added, and the direct answer to `Lane`'s casing having diverged silently earlier in this session.

It also added its file to `includeSource`/`coverage.include` unprompted, which is what stops a new in-source suite running zero tests while reporting green.

**It corrected my briefed baseline.** I stated 184 passed / **2** failed; the live tree was 184 / **1** — a concurrent session had already fixed `decodes the Rust-generated binary wire fixtures byte-identically`. I confirmed that independently: only `matches the Rust plan_workflow … decoded via wasm` remains, and my `bun 📜️script.ts wasm` build of the missing `pkg/` artifact is queued behind the build lock to clear it. That is the second time this wave a packet has caught a stale number in my brief, and both times measuring beat quoting.

**Honest gap it declared:** the worker bridge can only feed the coalesced/delta path, not a lossless inbound "send", because the reused `publishPreview` wire carries no such distinction. Stated rather than papered over — and it is a wire-shape limitation, not a bug in the implementation.

**Both halves of the backbone now exist**: the Rust host side (`⚡️effects/`, verified by name — `backbone_send_is_rejected_without_the_capability`, `backbone_delta_fanout_coalesces_a_burst_for_the_same_uri`) and this TypeScript counterpart conforming to it field-for-field. The gap that had **no path at all** since `set_host_backbone_channel` was deleted is closed on both platforms.

### 🧷️ Fifth idling incident — and the rule that follows is structural, not disciplinary

`interactive-isolation` ended its turn waiting on detached builds, as four packets did before it. I stopped treating this as executor error some time ago; this instance settles it. At the moment it detached, the box was at **35 concurrent cargo processes** (peer sessions running a large sweep alongside three of my packets), where **even a 600 s foreground timeout cannot finish a wgpu build**. "Run it in the foreground" had simply ceased to be an available option, and the harness's ~120 s auto-background did the rest.

Recorded as rule 23: **the coordinator owns every acceptance build.** Executors write code and reasoning, run only cheap checks, and mark acceptance UNRUN. This costs nothing, because I have re-run every packet's acceptance myself all wave anyway — an executor's own number has never once been accepted as evidence on this ticket. What it buys is that a packet's *reasoning* — the part only it has — stops being lost to a build queue it cannot win.

Its redirect asks for exactly that: the mechanism and why it works, confirmation the kernel keys off **observed behaviour rather than fixture profile names** (`hang`/`cpu`/`idle` exist only in the bench; keying on them would make the whole mechanism a lie outside it), the budget-3 non-regression argument, purity, exact line ranges, and an explicit warning if the file is mid-refactor — since I am about to build it. It was told **not to predict a p95**: a predicted number in a report is worse than no number.

Measured before redirecting: `🎭️actor/🦀️component.rs` modified ~6 minutes prior, and **nothing else** under the actor/shard/wgpu trees — so the scheduler-side change exists and is narrow. Verification is queued behind a drain-gate that waits for builders to fall below 8 before spending a build.

## 🔬️ BUDGET 5 HAS NEVER MEASURED WHAT IT CLAIMS — the fourth instrument defect, and the decisive one

`interactive-isolation` delivered a sound mechanism and then flagged, unprompted, that **its gate structurally cannot fire for budget 5**. Chasing that admission down produced the real answer.

**What it built** (one file, `🎭️actor/🦀️component.rs`, pure): `ActorMetrics::is_saturating` (≥2 turns and `wall_us_p95` ≥70 % of the actor's own `Budget::wall_ms` — **behaviour-derived, no fixture profile strings anywhere in the crate**), `ShardTable::pin_avoiding` sharing a refactored `least_loaded` with `pin` (empty avoid-set ⇒ byte-identical arithmetic, so budget 3 cannot regress), and `Kernel::activate` routing **only `Lane::Interactive` actors** through it. Coordinator-relevant: it verified budget 3's non-regression by *reading* `scale_bench::Env::activate` and finding it hardcodes `Lane::Background`, so the new branch is never taken there. **actor 70/70**, wasm32-unknown-unknown clean, purity grepped — the two commands it could complete are real; the rest it marked UNRUN rather than estimating.

**The finding that matters.** Reading budget 5's loop (`📦️glue.rs` ≈986-1017):
```rust
let start = Instant::now();
for actor in &cpu_actors { env.send(*actor, &Event::Wake); }  // wake all 40 cpu actors
env.send(interactive_actor, &Event::AppCommandEvent{…});       // then the interactive command
env.pump();                                                    // drives EVERY actor to completion
let outcomes = env.drain();
samples_ms.push(start.elapsed());                              // ← the whole round
```
`pump()`'s own docstring says it *"drives every actor with a non-empty mailbox **to completion**"*. **So every budget-5 sample ever recorded is the wall time of the entire round — 40 CPU actors plus the interactive one — not interactive command→patch latency.**

That resolves the mystery of a number that would not move: 295 → 241 → 217.9 ms across a single-shard servant, K real shard threads, DRR grants and a live kernel loop. It could not move, because **40 actors busy-looping their declared milliseconds cannot finish in 8 ms**. The budget was unreachable by construction, and no scheduler work of any quality could have reached it.

Compounding it: `Env::send_payload` hardcodes `lane: Lane::Background` for **every** bench envelope, so the "interactive" probe never travelled the interactive lane — which is also exactly why `interactive-isolation`'s `Lane::Interactive` gate could not fire, precisely as it reported.

**Four instrument defects in one budget**, each hiding the next: one physical `ShardLoop` behind K labels (30 samples in a 0.1 ms band); a `faults == 0` criterion absent from the spec; the probe on the wrong lane; and the measured interval spanning the entire round.

### The line I am holding
Correcting an instrument to measure the quantity the budget **names** is required. Making a number look better is forbidden. `bench-instrument` is dispatched to time from the interactive command to that actor's own outcome — with the 40 CPU actors still saturating, just **outside** the measured interval — and to put the probe on `Lane::Interactive`. It may not touch the 8 ms threshold, the fixture, the actor count or the round count.

It must also state plainly that **the old and new numbers are not comparable** (round wall-time vs command→patch latency) and must **predict the expected sample behaviour before I measure** — a tight cluster after the fix would itself indicate a *remaining* instrument problem, and I want that prediction on record so it functions as a check rather than a rationalisation. If a correct measurement still exceeds 8 ms, that ships as a failure.

### ✅ web-shellhost accepted — and the last "pre-existing failure" has a real root cause

**web-shellhost**: all 5 findings confirmed live and fixed in `ShellHost/🟦️component.tsx` (registrar-shared, scoped grant): `invokeExtension` extracted to module scope and dispatched through the **already-tested** `serializePerActor` without awaiting in the loop, so one slow extension no longer stalls the batch; boot install fan-out bounded and aborting on unmount; the fixed 2 s identity `setTimeout` replaced by `waitForEvent` raced against `AbortSignal.any([unmount, timeout(2000)])`; three `/extensions/install` fetches on a component-lifetime controller; presence beat parallelised per document with latest-wins. **325/336**, and the 11 remaining failures are an **exact subset** of the 15-name pre-existing baseline recorded in this ticket's own `terra-H1-vitest-final.txt` — named-set comparison, not counts. Two lease-requests filed (export `poolConcurrency` rather than keep its documented duplicate; wire real test coverage).

### 🔍️ The `plan_workflow` failure: not "pre-existing", a broken build

I tried to build the missing `pkg/` artifact that a peer session had diagnosed as the cause. It **fails**:
```
error: The "wasm_js" backend requires the `wasm_js` feature for `getrandom`
error: could not compile `getrandom` (lib)
```
The repo **does** configure that backend — in `.cargo/config.toml`'s wasm32 rustflags. But this ticket already documented the hazard, in V1a's entry: **`RUSTFLAGS` REPLACES rather than merges with `.cargo/config.toml` rustflags**, which is why deny-on-warnings must use clippy's trailing `-- -D warnings`. The wasm-pack path sets `RUSTFLAGS` and so silently drops the `getrandom_backend` cfg (along with `-Z threads=8` and mold).

So the artifact was never "not built yet" — **the build has been broken**, and a test failure has been carried for over a day as "pre-existing wasm-artifact" when it had a specific, fixable cause one layer down. Two independent sessions classified it without diagnosing it; the peer got as far as "`pkg/` was never built", and this is the reason why.

Routed out-of-band with the full diagnosis, `📌️important.md` rule 6 (do not edit `.cargo/config.toml` or add per-crate `RUSTFLAGS` — the fix belongs in the build helper's env handling, preserving existing flags), and an exclusion list of every path this ticket's live packets hold. Not fixed here: it is build infrastructure, unrelated to the async rewrite, and the machine is at 31 concurrent cargo processes.

**Generalisable point, since this is the second time today the same shape appeared:** *"pre-existing failure"* is a classification, not a diagnosis. Both times — the `plan_workflow` wasm test, and the four-in-isolation SDK failures — the honest move was to keep the label only until someone asked *why*, and both times the answer was specific and actionable rather than ambient.

## 📌️ W5 CONSOLIDATION — async-first rewrite, coordinator-verified state

Every number below was re-run by me; no packet's own figure is quoted as evidence.

### Landed and verified

| area | evidence |
|---|---|
| **Async interface crate** `semio-framework-async` (PURE) | `OperationContext`, tri-state `CancelToken` + `child()`, `Scope`/`ScopeDrainReport`, `ChannelPolicy`, `ThreadPlan`/`ThreadBudget`, `HostAsyncRuntime`. **16/0**, wasm32-unknown-unknown clean, purity grep clean. Member + alias applied by me |
| **Services crate** `semio-framework-os-services` (tokio confined) | `TokioHostRuntime` (takes `ThreadPlan`, never reads core count), `ScopeTable` with honest `leaked` accounting, one `TimerWheel` (pure `WheelCore` + driver), semaphore-bounded `ComputePool`, `HttpPool`, `StorageScheduler`, `EventRouter`, `CompletionSink`. **26/0**, clippy `-D warnings` clean, **no tokio type in any public signature** |
| **WASI 0.3 ABI** | one package, two worlds (`actor` poll + `actor-async`), `host-async` reusing the `*-params` records, `stream<u8>` HTTP bodies, `run(events: stream<event>)`. **All 4 gates**: guest parse ✅, host parse ✅, **parity tests EXECUTE 4/4** ✅, describe crate ✅ |
| **Toolchain** | wasmtime **22.0.1 → 47.0.3** (86/0/1 preserved, descriptor byte-identical across versions on fixed input), wit-bindgen **0.36.0 → 0.57.1** (SDK **and** the scale fixture's private generator) |
| **Async effect execution** | `AsyncEffectExecutor` + per-instance `EffectBackbone`. **115/0/1**, with properties verified **by name**: revocation cancels only its own ops, stale-generation completions dropped, `Park` buffers and resumes in order, completion bursts bounded, quota denial typed not panicking, deltas coalesced per URI |
| **Live parallel kernel** | `ParallelRuntime`: real `submit`→`tick`→`Grant`→**`Kernel::complete`** over **K real `ShardExecutor` OS threads**, K from `thread_plan`, traps synthesised so the failure ladder finally sees them |
| **Shard wire** | `ShardFrame{Register,Unregister,Grant,Envelope}`, DRR budgets travelling in `Grant`, `TURN_BUDGET`/`JOB_STEP_BUDGET`/`budget_for` **deleted** |
| **serde sibling sweep** | 6 internally-tagged newtype variants → struct variants; regenerated TS mirror has **zero** `} & ` impossible intersections (which is what unblocked the web adopting the wire) |
| **Registration race** | `ShardExecutor::register` now acks — a genuine happens-before. Found by **reverting the fix and watching the new property tests fail with the exact bench fault text** |
| **Bench (native, K=4)** | **7 of 8** budgets: registry 2.85 ms / 0 instantiations; cold boot 0 faults; **perShard {25,25,25,25}**; 2550 actors @ 598 MB; hang killed, siblings restored; suspend→resume identical hash **and genuinely resumed**; capability revoked, actor survived |
| **TypeScript** | glue **87**, actor **29→40**, kernel **29** (was in NO gate at all), os **206/1**, dev **17**, PluginRuntime **26**, ShellHost **325/336** (11 = exact subset of the 15-name baseline) |
| **Web async** | SSE-primary backbone with bounded lossless outbox; sustained-health backoff reset (both transports); directory calls all signal+timeout so a hung server degrades to offline instead of hanging boot; lane-priority turn scheduler; self-ticking watchdog (had **zero** production callers); `failShard` clearing routing; memory-aware LRU; `ShardFrame` on the worker wire; per-instance TS `EffectBackbone` |

### Honest remaining

1. **Budget 5 — the instrument, not the runtime.** Now known: every sample ever recorded timed the **whole round** (40 CPU actors + interactive) via `pump()`-to-quiescence, and the probe travelled `Lane::Background`. Correction in flight; **old and new numbers will not be comparable**, and if a correct measurement still exceeds 8 ms that ships as a failure.
2. **`directory-and-run`** in flight — `AppChannelHost::exchange`'s sync duplex, and the directory client's blocking `ureq` + `block_on` + **private tokio runtime** (the one remaining tokio outside the services crate).
3. **`shell-unpark` verification** queued — its 3 named parks are gone (census by me); the 5 survivors it classified are asset-polls coupled to `ControlFlow::Poll`, a headless CLI entry, and two background-thread sites owned by `directory-and-run`.
4. **`pkg/semio_framework_os.js` build is BROKEN** (`RUSTFLAGS` clobbering `.cargo/config.toml`'s `getrandom_backend`), routed out-of-band — root cause of the last os-package failure.
5. **Not started**: 23 remaining descriptors (peer-gated), Z1 zero-warnings ×3 targets, parity 58/58, web benches, launch-seed entries.
6. **~253 tests never run** repo-wide (cad-js broken by a stale package rename; two infinite-canvas projects green against zero files) — routed out-of-band.

### The wave's methodological result

Five packets ended a turn idling on detached builds. The cause is structural — the Bash tool auto-backgrounds at ~120 s, a subagent's detached job cannot report across its turn boundary, and above ~20 concurrent cargo processes even the 600 s maximum cannot finish a wgpu build. **Rule 23 moves every acceptance build to the coordinator**, which costs nothing because no executor figure has ever been accepted as evidence here anyway.

Four instrument defects were found in a single budget, each hiding the next. Three separate "pre-existing failure" labels dissolved into specific causes once someone asked why. And four times a too-narrow or silently-failing query produced a confident wrong picture — three of which would have misjudged **another session's** work. Rules 19–23 exist so the next session inherits the checks rather than the anecdotes.

### ✅ shell-unpark verified — the UI thread no longer waits on a plugin

Coordinator-run: `cargo check -p semio-framework-os-renderer-wgpu --lib` → **WGPU_EXIT=0, 0 errors**.

That closes the packet the executor could not verify itself (it marked all three commands UNRUN rather than reporting numbers it had not seen). Combined with my own park census, the position is: **all three named UI-thread parks are gone** — hot-reload `boot()` and `pump_sync_events()` in `📦️glue.rs` both moved to `spawn_app_task` via `self_weak`, and `Shell/🧊️component.rs`'s `LoadDocument` park became a self-contained `spawn_app_task` needing no `AppRuntime` re-borrow at all.

The surviving `block_on` sites are **not** UI-thread parks, which its call-site reading established and my grep-based census had wrongly implied: four are one function (`poll_pending_assets`, coupled to `ControlFlow::Poll`, explicitly out of scope), one is a headless CLI entry with no winit loop, and two sit inside `std::thread::spawn` closures on dedicated background threads and belong to `directory-and-run`.

Total edits in the registrar-shared file: **22 lines** (`Shell/🧊️component.rs` 1697–1718), because it declined the brief's `Rc<ShellIo>` sketch on finding the existing `self_weak` pattern already sufficient. `ControlFlow::Poll` confirmed untouched.

### ✅ interactive-isolation — scheduler primitives verified (70/70)

Coordinator-run: `cargo test -p semio-framework-actor` → **70 passed / 0 failed, exit 0** (69 baseline + its 1 new placement test). Its own wasm32-unknown-unknown check and purity grep were already real and clean.

What landed, in **one file** (`🎭️actor/🦀️component.rs`), all pure:
- `ActorMetrics::is_saturating(&self, budget)` — true once an actor has ≥2 turns **and** its own `wall_us_p95` reaches ≥70 % of its own `Budget::wall_ms`. **Behaviour-derived**: no `hang`/`cpu`/`idle` profile strings anywhere in the crate, so the mechanism means the same thing outside the bench as inside it. That constraint was explicit in the brief and it honoured it.
- `ShardTable::pin_avoiding(actor, avoid)` sharing a refactored `least_loaded` with `pin` — with an empty avoid-set the arithmetic is **byte-identical** to before, which is the structural reason budget 3's 25/25/25/25 cannot regress.
- `Kernel::activate` routes **only `Lane::Interactive`** actors through it, fed by `saturated_shards()` reading already-tracked metrics — **no new state, no clock**.

It proved budget-3 non-regression by *reading* `scale_bench::Env::activate` and finding it hardcodes `Lane::Background`, so the new branch is never taken there — and then reported the uncomfortable corollary itself: **its gate therefore cannot fire for budget 5 either.** That admission is what led to discovering the four-deep instrument defect. A packet that had quietly claimed success would have left the real problem buried.

Honest gaps it declared: placement-time only (no live migration of an already-pinned actor), and plugin-host/wgpu compiling against its diff genuinely unverified — both now running from this session.

### 🔴️ Cross-crate red found by verification — attributed correctly, not to the packet under test

Verifying `interactive-isolation`'s downstream impact, both consumers failed:
```
cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity → HOST_EXIT=101
cargo check -p semio-framework-os-renderer-wgpu --lib                    → WGPU_EXIT=101, 3 errors
```
**The failures are not its.** Both are the same two errors, in `semio-framework-os-kernel`:
```
error[E0061]: `client.me()` — argument #1 of type `&OperationContext` is missing
error[E0061]: `client.mint_session(&env.user_email)` — argument #1 of type `&OperationContext` is missing
  --> 📇️directory/🪪️identity/🦀️component.rs:149, :167
```
Attribution by timestamp rather than inference: `📇️directory/🔌️client/🦀️component.rs` modified **2.2 min** ago and `🏃️run/🦀️component.rs` **9.8 min** ago (both `directory-and-run`, live), while `📇️directory/🪪️identity/🦀️component.rs` has not been touched in **36 hours**. So the in-flight packet added `&OperationContext` to the client — which is exactly what it was asked to do — and the call sites in a **sibling module outside its owned scope** have not caught up.

This is the fifth time this ticket has met the signature *"the artifact changed, its registration did not"* — but the first time we generated it ourselves rather than absorbing it from a peer, which is worth recording plainly rather than filing under someone else's lesson.

**Scope extended** to `📇️directory/🪪️identity/**` so it can repair its own fallout instead of filing a lease and waiting, with the standing conditions (surgical edits, re-read before each edit, exhaustive line ranges) and an instruction to stop and report if the fallout reaches further rather than widening again unilaterally.

One substantive constraint attached: **thread a real `OperationContext`, do not pass a default to make it compile.** An empty context at the identity/session call sites would compile cleanly and silently sever cancellation, deadline and trace propagation precisely on the path most worth cancelling at shutdown — a green build concealing the exact regression the packet exists to prevent. It was also authorised to run `cargo check -p semio-framework-os-kernel --lib` itself despite the coordinator-owns-builds rule, since the tree is red *now* and that gate is cheap.

**`interactive-isolation` is therefore not yet cleared downstream** — its actor crate is verified (70/70) but plugin-host/wgpu could not be evaluated against its diff while an unrelated red sat upstream of it. Re-verification is queued behind the fix.

### 🔧️ bench-instrument delivered — budget 5 will finally measure command→patch latency

Its diagnosis matched mine independently and added the mechanism: `pump()`'s `wait_for_outcomes(decision.run.len(), …)` **blocks until all 41 granted outcomes arrive**, and `start.elapsed()` was taken after it returned. So every sample was the round's wall time.

The fix, entirely inside `//#region 🔖️ScaleBench`:
- **`Env::pump_tracking(target)`** — same tick-drives-to-completion loop, but waits **one outcome at a time** and stamps `Instant::now()` the moment the *target's* own `ShardOutcome` arrives. The other 40 CPU actors keep running and keep contending on the real `ShardExecutor` threads; they simply no longer gate the clock. That preserves the load the budget specifies while removing it from the measured interval — the distinction that makes this a correction rather than a weakening.
- **`Env::send_payload_lane(actor, payload, lane)`**, with `send_payload` delegating at `Lane::Background`, so **every other budget's call site is byte-for-byte unchanged**. Budget 5's round loop is the only `Lane::Interactive` caller.
- Round loop rewritten: the 40 `Wake`s are submitted **before** `start`; `start` is taken immediately before the interactive send; `pump_tracking` replaces `pump()`.
- It also corrected a **stale doc comment on `pump()` claiming "this is the instrument budget 5 measures"** — the sentence that would have misled the next reader exactly as it misled everyone until now.

**Two things it did that I want on record as the standard:**
1. **A pre-registered prediction, before I measured**: expect genuine spread (shard assignment is fixed across rounds, but the interactive actor's queue position among its shard-mates is subject to real OS scheduling jitter); expect a p95 far below the old 150–240 ms band; but **not necessarily under 8 ms** — and *"if samples cluster tightly again, that would itself indicate a remaining instrument problem, not a pass."* A prediction filed before the number exists is a check; filed after, it is a rationalisation.
2. **An honest limitation**: fixing the *envelope's* lane does **not** make `interactive-isolation`'s placement gate reachable, because that gate reads the actor's **activation** lane, and `Env::activate` hardcodes `Lane::Background` for every budget — out of scope here. So the two packets do not yet compose, and it said so rather than implying a combined win.

**Upstream red cleared first**: `cargo check -p semio-framework-os-kernel --lib` → **exit 0, 0 errors**, so `directory-and-run` repaired the `&OperationContext` call sites in the scope I extended to it. Bench now running against the corrected instrument.

### ⏳️ Bench blocked again — second fallout wave from the same signature change

The corrected budget-5 instrument is **in the tree but not yet measured**: `bench … --shards 4` → `BENCH_EXIT=1`, because `semio-framework-os-renderer-wgpu` fails to compile with **7 × E0061, all `&OperationContext` missing, all in `🧱️elements/Shell/🧊️component.rs`** (`:3148, 3173, 3205, 3206, 3221, 3264, 3268`).

Same root as the first wave — `directory-and-run` threading `OperationContext` through the directory client — and its `🪪️identity` fix genuinely worked (`os-kernel --lib` → **exit 0**, verified). The wgpu path simply has its own call sites, in a different file.

Two of them are recognisable from this ticket's own park census earlier today: `:3206` is `pollster::block_on(mint_or_restore(&client, &env))` and `:3264` is the `runtime.block_on(…)` wrapping a **private tokio runtime**. Those were always that packet's to convert — `shell-unpark` had already classified them as *"not UI-thread parks, and `directory-and-run`'s to retire"*. The only surprise is which file they live in, and the earlier classification is what let me attribute this in one step instead of suspecting the instrument change.

**Scope extended a second time**, now to registrar-only `Shell/🧊️component.rs`, with the conditions `shell-unpark` operated under successfully (surgical region-scoped edits, re-read before each edit, exhaustive line ranges, `--lib` only because another session's `Dock` test-module break lives in that file's `--all-targets` path). It was also told that if fallout reaches a **third** file it must stop and report the full list rather than widening again unilaterally — scope should grow by decision, not by drift.

Three constraints attached, in priority order: **retire the private tokio runtime at `:3264` rather than merely adding an argument** (tokio outside `semio-framework-os-services` is the containment this whole architecture rests on, and this is the last instance in the tree); **thread a real `OperationContext`, never a default** (an empty one at `mint_or_restore` would compile, pass, and silently sever cancellation on shutdown-sensitive identity paths — a green build hiding the exact regression the packet exists to prevent); and it may run the wgpu `--lib` gate itself, with an explicit 600 s timeout and instructions to report **unrun** rather than detach if it exceeds that.

**Status of budget 5, stated precisely:** the instrument correction is written and reviewed but **unmeasured**. No number exists yet. The prediction filed before measurement — real spread, p95 far below the old 150–240 ms band, but not necessarily under 8 ms — remains untested and stays on record as a check.

### ⚠️ Disk emergency during D4 — 78 GB → 11 GB, halted correctly

D4 stopped after 2 of 5 plugins because free space hit **11 GB**, honouring the 60 GB floor its brief set rather than pushing on and failing mid-build. That is the behaviour the floor exists for, and the first packet to actually exercise it.

Reclaimed **62 GB** from my own completed packets (`🎯️target-v1b` 27 G, `-d3` 19 G, `-d2` 6.2 G), each verified as a cargo cache before deletion. **Peer dirs left untouched** — `🎯️target-kl` is the async session's live `kernel-loop`, and `witb`/`u1`/`dr`/`sr`/`w0`/`s1` are not mine to judge. Free space 11 GB → **62 GB**.

D4's own footprint was 6.9 GB against ~119 GB of accumulated target dirs in this ticket folder — **the pressure was cumulative, not any one packet's fault.** Per-packet target dirs remain correct for lock contention and remain expensive for disk; the reconciliation is that finished packets' caches must be reclaimed promptly, not at session end.

### 📋️ D4 result: 2/5 verified, 0 ratcheted — and it declined to ratchet on principle

`🌀️procedural` and `🌍️gis`: `describe` exit 0, both descriptors landed at the owner root, **both confirmed real** (`pluginId` = `procedural`/`gis`, not `assembly-failed`). Neither ratcheted — procedural's test build fails on **36 pre-existing unrelated compile errors** (`ArtifactStore::new()` `Result`-unwrap fallout, already known), and gis never got its run before the disk floor. `energy`/`imperative`/`layout` untouched.

It also rode out a transient kernel-red blip from a peer's `OperationContext` refactor, confirmed it unrelated, and continued — correctly distinguishing a peer's moving target from its own breakage.

### 🔍️ Git index discrepancy it caught and could not fix

The **index** has `"procedural"` staged into `DESCRIPTOR_MIGRATED_PLUGINS` (from an earlier session); the **working tree** does not. Confirmed by `git diff --cached`:
```
-  … "animate", "draw"];
+  … "animate", "draw", "procedural"];
```
D4 reverted its own working-tree edit to net zero and reported the staged entry rather than touching it — correct under the no-git-modifying-commands rule, which forbids unstaging as much as committing.

**Assessment**: low risk *today* — procedural now has a real committed descriptor, and its `descriptor_is_fresh` cannot run at all while those 36 compile errors stand. But when someone repairs them, that test executes for the first time against an entry nobody verified, and a failure will read as a fresh regression rather than a pre-existing unknown. **Flagged for whoever commits: the working tree (11 entries, no procedural) is the intended state.**

Descriptors now **23 committed, 11 ratcheted, zero placeholders** — the emitter guard is holding across two further emissions.

### ✅ Launch entries registered — closing a CLAUDE.md obligation I had deferred and forgotten

A peer warned that `.vscode/launch.json` **silently lost four of their entries** — rewritten by another session with no conflict and no error, and nothing surfaces it because `bun nx run` keeps working. Checked my own: **`bench` and `verify rust-warnings` were registered nowhere.** Not lost — I deferred them earlier ("the seed is regenerated, so adding entries for not-yet-existing verbs would bake a broken launch.json") and never came back once the verbs existed.

That deferral was defensible at the time and became a **standing violation of repo law** the moment the commands worked: *"All devs are using `launch.json` and never use the cli. You MUST register all executable commands there."* A verb no dev can reach through their launcher is, for this repo's purposes, not shipped.

Added to `.vscode/🧩️launch.seed.jsonc`, following the existing group/order conventions:

| entry | group | order |
|---|---|---|
| `⚖️gate🧵️plugin-runtime🖥️native` | 4_gate | 411.0 |
| `⚖️gate🦀️zero-warnings🖥️native` | 4_gate | 411.4 |
| `⚖️gate🦀️zero-warnings🌐️wasip2` | 4_gate | 411.41 |
| `⚖️gate🦀️zero-warnings🎭️actor-wasm` | 4_gate | 411.42 |

Those orders match `📓️design-workforce.md` §5's own plan (`⚖️gate🦀️zero-warnings🌐️wasm` at 411.4, `⚖️gate🧵️plugin-runtime…` at 411), split three ways because the warnings gate turned out to need one entry per target rather than one for all.

Verified rather than assumed at each step: seed diff **44 insertions, 0 deletions** (purely additive — nobody else's entries touched, which matters given the peer's loss); `plugin-registry:generate` exit 0; `launch.json` went **0 → 4** matching entries by name.

**Exit-checklist item 7's `launch.json` clause is now met.** The peer's own four lost entries are theirs to restore — I checked and the four `scale-fixture` entries from this ticket are intact, so nothing of ours went missing in the same incident.

Two file-handling notes from them worth keeping: the seed is **JSONC**, so a plain `json.load` dies on the leading comment (strip `^\s*//.*$` first), and it is one of the few hand-maintained files with **no generator guard** — no equivalent of the `#[path]` test that would catch silent loss.

### ✅ Tree green again — and tokio containment verified INDEPENDENTLY, not taken from a report

```
cargo check -p semio-framework-os-renderer-wgpu --lib → DR_WGPU_EXIT=0, 0 errors
cargo check -p semio-framework-os-kernel        --lib → exit 0, 0 errors
```
`directory-and-run` repaired both fallout waves (`🪪️identity`, then the 7 sites in `Shell/🧊️component.rs`) inside the scope I extended to it.

**The design question I cared about — answered by measurement, not by asking.** A green build cannot tell you whether a private tokio runtime was *retired* or merely *given an argument*. Searching for live constructions outside `🛎️services`:

| location | verdict |
|---|---|
| `Shell/🧊️component.rs:46, :1058` | **comments only** — `:46` records the `Builder::new_current_thread()` that `open_directory_stream` *used* (past tense). **The private runtime is genuinely gone.** |
| `🛢️db/🐘️postgres`, `🛢️db/🌐️neo4j` | live, and legitimately so — database drivers, pre-existing, never in this wave's scope |
| `🔄️sync`, `🌉️mcp/🚚️transport` | live, pre-existing, separate subsystems |

So **tokio containment holds where this architecture claims it**: the plugin-runtime path (kernel → shard → host services) constructs tokio only inside `semio-framework-os-services`. The surviving runtimes are database and gateway subsystems that were never part of the claim. That distinction is worth stating precisely rather than declaring a blanket "tokio is confined", which would be false.

**Method note, since it nearly bit me a fifth time:** my first containment grep used `Runtime::new\(\)` and matched **`MockGuestRuntime::new()`** — eight false positives that, read carelessly, would have suggested tokio runtimes scattered through the shard layer. Re-run with an anchored pattern (`tokio::runtime::(Runtime|Builder)`, `Builder::new_(multi_thread|current_thread)`) it resolves to 12 real hits, of which the two in `Shell` are comments. Rule 21 exists for exactly this; I applied it to myself.

Bench now running against a green tree with the corrected budget-5 instrument.

### ✅ directory-and-run — the last sync holdouts converted, and the honest gap is the valuable part

`Shell/🧊️component.rs` compiles clean, all 7 sites done; both my check and its own agree (`wgpu --lib` exit 0, warnings only in the unrelated `Dock` module).

**On the tokio runtime, its answer matches my independent grep and then improves on it.** `open_directory_stream` no longer constructs anything: it captures `self.directory_runtime.clone()` — an `Arc<TokioHostRuntime>`, the services crate's own type, minted exactly once in `ShellState::new` — and calls its inherent `.block_on(...)` rather than `spawn_scoped`. The reason is a real constraint, not a shortcut: `DirectoryStream`/`DirectoryWsConnection` are deliberately `?Send` because the browser transport closes over non-`Send` `wasm_bindgen::JsValue`, so `spawn_scoped`'s `Send` bound is **unsatisfiable** there, while `block_on` drives the future locally without it. So the containment claim holds in the form that matters — tokio is *owned* by the services crate and merely *used* here — and the deviation from "everything goes through `spawn_scoped`" is explained rather than glossed.

**No default or empty `OperationContext` anywhere.** All 7 sites route through one `directory_ctx()` helper built from `self.directory_cancel.child()` — a genuine child of a shared root, which is precisely what I warned against faking. It would have been trivially easy to pass a fresh empty context, compile green, and be technically able to say "contexts are threaded".

**And then it named the gap that makes the difference:** nothing ever calls `.cancel()` on `directory_cancel`. **Cancellation is architecturally live but inert** — the plumbing is correct end to end and no shutdown hook is wired to pull it. A second, smaller gap: `deadline_ms: None` at every site, so no deadline path is exercised. Both are recorded as named follow-ups rather than discovered later by someone trusting the word "propagates".

That distinction — *wired* versus *effective* — is the same one this ticket has paid for repeatedly (a heartbeat watchdog with zero callers, a metrics publisher with no caller, `descriptor_is_fresh` reading the wrong path). This is the first time a packet volunteered it about its own work **before** anyone measured.

Budget: ~524k tokens, 342 tool calls — by far the wave's largest, driven by two unforeseen fallout waves in registrar-shared files rather than by its own scope.

## 🎯️ BUDGET 5 MEASURED HONESTLY AT LAST — p95 140.9 ms, and it FAILS

`bench plugins --renderer native --count 50 --extensions 50 --shards 4` → exit 0, archived `🔣️bench-instrument.json`.
**`1:pass 2:pass 3:pass 4:pass 5:fail 6:pass 7:pass 8:pass`**

```
budget 5   p95 = 140.911 ms   threshold 8.0 ms   →  FAIL
           min 82.333   max 142.356   spread 60.023 ms
           30 rounds, 0 round faults
```

### The prediction, filed before measurement, scored honestly

| predicted | actual |
|---|---|
| real spread, not a constant | ✅ **60.0 ms** spread (the broken instrument gave 0.1 ms) |
| p95 "far below" the old 150–242 ms band | ⚠️ **partly** — 140.9 vs 217.9/241.2 is a ~35 % reduction, not the order of magnitude implied |
| **not necessarily under 8 ms** | ✅ correct — it is **17.6× over** |

Scoring my own agent's prediction against the result rather than quietly dropping it: two of three held, and the miss (how far it would fall) is itself informative — it says the remaining latency is *not* mostly measurement overhead.

### What this number means, precisely

It is the **first honest measurement of budget 5 in this ticket's history**. Every earlier figure (295, 241, 217.9) timed the whole 41-actor round to quiescence; this one times the interactive command to that actor's own outcome, with the 40 CPU actors still saturating real `ShardExecutor` threads but outside the clock. **The numbers are not comparable and the earlier ones should not be cited as a baseline.**

And it is a **real design result, not an artefact**: an interactive turn still waits ~82–142 ms because its actor is pinned to a shard shared with ~10 CPU-bound actors and cannot start until the turns ahead of it finish. K parallel shards distribute *work*; they do not protect *latency*.

### The one remaining instrument variable — named, not silently fixed

`interactive-isolation`'s placement gate keys off an actor's **activation** lane, but budget 5 selects its probe from budget 4's live 2550-actor fleet, all activated `Lane::Background`. So the isolation mechanism **still cannot fire**, exactly as that packet reported about its own work.

I added `Env::activate_on_lane(…, lane)` with `activate()` delegating at `Lane::Background` — **byte-identical behaviour for every budget**, verified (`wgpu --lib` exit 0, 0 errors). It is a seam, deliberately unused. Wiring budget 5's probe through it means changing how budget 4's shared fleet is activated, which is real surgery on another budget's setup and belongs to a scoped packet, not to a coordinator editing at the end of a long wave.

**So the open question is stated exactly**: *does interactive-lane activation plus saturation-aware placement bring command→patch under 8 ms?* Unknown — the mechanism is built and verified (70/70) but has never been exercised. That is a far better position than this wave started from, where the question could not even be asked because the instrument measured the wrong quantity.

### Everything else on the bench passes

1 registry (2550 records, 0 instantiations) · 2 cold boot, 0 faults · 3 **perShard {25,25,25,25}** · 4 2550 actors live · 6 hang killed, siblings restored · 7 suspend→resume identical hash, genuinely resumed · 8 capability revoked, actor survived.

### ✅ D4 finished — 5/5 real descriptors, and it found two genuine data bugs

**`🔋️energy`**: `describe` failed with *"no declared codec capability owns the runtime claims"*. Root cause was a real mismatch — the declared `extension` claim said **`"model"`** while the runtime derives **`"energy"`** from `EnergyModelSnapshot::EXTENSION`. Fixed, re-ran, EXIT 0.
**`📜️imperative`**: same class, different capability — *"no declared composer capability"*. The native composer row (`s.imperative@1/*`) was **missing entirely** from `definition()`. Added, re-ran, EXIT 0.

Both are exactly the defect the capability-claim rule exists to catch, and both were invisible until `describe` actually ran. Neither is a channel migration; both are one-line data corrections inside the plugin's own declarations.

### 🔒️ Ratchet advanced to 13 — coordinator-run, on measured passes only

```
cargo test -p semio-s-plugin-energy --lib descriptor_is_fresh → EXIT 0, 1 passed
cargo test -p semio-s-plugin-layout --lib descriptor_is_fresh → EXIT 0, 1 passed
```
`DESCRIPTOR_MIGRATED_PLUGINS` = note, sequence, vcs, forms, sourcing, dag, mathematical, writer, reasoning-mindmap, animate, draw, **energy**, **layout**.

**Three plugins have real descriptors but cannot be ratcheted, all for the same reason and none of it descriptor-related** — their test crates do not build:
| plugin | pre-existing test-compile failure |
|---|---|
| `🌀️procedural` | 36 errors, `ArtifactStore::new()` `Result`-unwrap fallout |
| `🌍️gis` | `no field `definition` on type `AppDefinition`` |
| `📜️imperative` | `cannot find type `App` in this scope` |

**`imperative`'s is a bug I have already fixed twice today.** Earlier this session I added missing `EditorApp`/`App` imports to `➗️mathematical` and `🖍️draw`'s `#[cfg(test)]` modules — their test code had simply never compiled. This is the third instance of the identical defect in a third plugin, which makes it a **class, not three coincidences**: plugin test modules using `App`/`EditorApp` without importing them, invisible because nobody had run those suites.

Descriptors now **26 committed, 13 ratcheted, zero placeholders** across the whole fleet.

## 2026-08-19 W6 — the waiting model itself

User verdict on W5: *"I still see no async e.g. all the io, etc. Make sure the architecture is async-first as described."* **Correct, and the distinction is exact**: W5 delivered async *infrastructure* (interface crate, services, effect executor, WASI-0.3 schema, parallel kernel, TS sweep) while *execution* stayed synchronous. `GuestRuntime` is a sync trait; `ShardLoop::pump` blocks an OS thread per shard; `HttpPool` wraps blocking ureq; storage wraps `std::fs`; four private tokio runtimes sit beside the sanctioned one; **not one plugin has ever executed on `world actor-async`**. W6 changes the waiting model.

### Three read-only censuses (measured; full table in `📓️luna-sync-surface-audit.md`)

| finding | measurement |
|---|---|
| `block_on` on production paths | **50+**: postgres **25**, neo4j **22** (both bridging sync traits → async drivers), mcp 1, services 1 |
| `GuestRuntime` (~:490-510) | entirely sync; wasmtime `Config` has **`async_support`/`concurrency_support` NOT enabled** |
| `ShardExecutor` | **5 ms `recv_deadline` poll** per pump iteration, one OS thread per shard |
| private runtimes | postgres :101, neo4j :360, sync-engine :1465, mcp :239 — **plus a FIFTH minted by the wgpu Shell itself** (`Shell:1323-1332`) |
| `HttpPool` | budgets charged by **estimate** (`url.len()+body.len()`); refill driver **doc-admittedly inert** |
| `🎒️pack/⏳️async` (peer's) | async-first — but contains a **`std::thread::sleep(200µs)` inside `Future::poll`** |
| `db_engine` | spawns **one bridge OS thread per `submit()` call** (:912) |
| TypeScript | essentially clean after W5 — 9 `Bun.sleep` await-polls in the dev script, 1 legitimate SSE fallback, watchdog/metrics intervals by design |

### The design, in one line
Guests run `world actor-async` under wasmtime-47 component-model-async with **epoch-`Yield` preemption**; one root task per actor owns its Store; shards become async executors multiplexing many actor futures; a **`GrantedEventProducer` stream-boundary** synthesises today's `TurnResult` so **DRR accounting is untouched**; host services do real async I/O; every private runtime consolidates to one per process. A suspended plugin costs a Store + a parked future — **state, not a thread**.

Three Plan agents produced the slices; every wasmtime claim was checked against the vendored 47.0.3 source (`UpdateDeadline::Yield` store.rs:388-401, `run_concurrent` Send bound concurrent.rs:984, `StreamProducer` futures_and_streams.rs:583-599). **Eight assumptions that could only be verified by RUNNING code are spikes S1–S8 with pre-named fallbacks** — the `wasmtime 34.0.2 exposed the whole async API over 35 todo!() bodies` lesson, applied before committing four packets to it.

### Two deliberate NEGATIVE decisions, argued from evidence
- **Storage stays bounded-blocking.** `tokio::fs` is internally `spawn_blocking` onto an *unbounded anonymous* pool; our `StorageScheduler` is bounded, lane-prioritised and quota-accounted. Swapping would be a downgrade dressed as async. Compiled-artifact cache explicitly deferred (cold path, once per package, already off-runtime).
- **No router becomes `async fn`.** Read rather than assumed: `IoRouter` forwards into other plugins' wasm (CPU-bound), inference/mutation routers are in-memory planning, and the coordinator/AppRouter's I/O leaves are *files* — correctly bounded-blocking. The genuinely-async wins are the network paths. This is the don't-boil-the-ocean line, drawn on what the code does.

### Wave A dispatched — 5 packets, parallel editing (rule 23: coordinator owns every build)
`probe-spikes` (the ONLY packet allowed to build/run — its deliverable IS an executed probe answering S1–S8) · `trait-asyncify` (one boxed-future `GuestRuntime`; poll impls return `ready(…)`; must be observably a runtime no-op) · `http-streaming` (`AsyncHttpTransport`/`HttpBody` seam, budgets charged on REAL bytes, refill task that actually runs, deadline racing) · `db-trait-flip` (**atomic, workspace-wide**: six sub-traits → `DbFuture`, both drivers drop private runtimes, ~55 `block_on` bodies unwrapped, 14 consumer crates propagate `.await`, ONE sanctioned bin-entry park in the cli) · `dev-polls` (TS, free).

Session-start: 206 GiB free, **zero source churn in 30 min**, 1 peer builder.

### ✅ dev-polls accepted — the TS sweep is finished, and the rule is now written down in code

Coordinator-verified: `bun ./📜️script.ts test` → **27 passed, exit 0** (baseline 17 + 10 new, all fake-clock, no real sleeps).

New `//#region 🔖️PollHelpers` carrying **THE RULE** as a docstring — *a deadline-bounded poll of an external resource emitting no observable event is acceptable; a poll of a resource we spawned and whose handle we hold is not* — plus `awaitTcpReady` / `awaitHttpOk` / `awaitChildExit`, each with test-injection points (`probe`/`fetchImpl`/`sleep`/`now`) and returning a `PollOutcome` rather than throwing.

**It settled the API question with evidence rather than assumption**: before choosing `awaitChildExit`'s mechanism it confirmed `SpawnDaemonHandle.child` is a Node `child_process.ChildProcess` — not a Bun `Subprocess` — from the file's existing `.pipe()` usage, then used the real `'exit'` event. Getting that backwards would have compiled and silently never fired.

**And it declined to over-apply its own helper**: 5 of the 7 nominated sites funnel through the helpers with deadlines/intervals preserved exactly; the plugin-build lease flag and the parity mkdir-lock are fs/pid-shaped, not TCP/HTTP-shaped, so it left them as legitimate polls with inline comments citing the rule. Forcing them through a TCP helper would have been worse code that merely scored better on a census.

My own classification of every surviving `Bun.sleep` (7 total): **3 are docstrings**, **2 are the helpers' injectable defaults** (`opts.sleep ?? (ms => Bun.sleep(ms))`), **2 are the documented fs/lock exceptions**. **Zero exit-code poll loops remain** — the pattern the packet existed to remove is gone, verified by pattern-classification rather than by counting occurrences.

Honest gaps it declared: no standalone `tsc --noEmit` (esbuild transform only); did not run collab-e2e/parity end to end (would spawn real processes); and it noticed unrelated Playwright `page.waitForTimeout` DOM waits elsewhere in the file, correctly judged out of this census's scope and left untouched rather than opportunistically widened.

### ✅ trait-asyncify — the execution contract is async-shaped, and provably still a no-op

`GuestRuntime` (`🖥️host/🦀️component.rs:485-523`) now returns `semio_framework_async::HostFuture<…>` from `execute_turn`/`start_job`/`step_job`/`cancel_job`/`checkpoint`/`restore`; `compile`/`instantiate`/`drop_instance` stay sync by design (instantiate will BUILD the async task spec, not run it). One trait, no compatibility layer, dyn-compatible — `async fn` in trait was ruled out because every consumer holds `Arc<dyn GuestRuntime>`.

Both existing impls keep their **exact prior bodies** inside an immediately-invoked closure and return `Box::pin(std::future::ready(result))` — computed eagerly so no `&mut` borrow is captured. New `poll_ready<T>(HostFuture<T>) -> T` (`:525-555`) polls once with `Waker::noop()` and panics loudly if not ready, with a comment explaining why that is sound for a backend that is always-ready by construction.

Coordinator-verified:
```
cargo check -p semio-framework-plugin-host --all-targets → exit 0, 0 errors
cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity → 115 passed / 0 failed / 1 ignored
cargo test  -p semio-framework-plugin-host --lib schema_parity           → 4 passed / 0 failed
```
**Exactly the baseline** — which IS the acceptance criterion for this packet: a boxed-future indirection that changed no behaviour.

**Two leases applied by me** (both mechanical `poll_ready(...)` wraps it correctly refused to make outside its scope): `🔀️PostTurnRelay`'s `start_job`/`step_job` in the same file, and two `execute_turn` sites in `🌉️mcp/🏠️workspace/🦀️component.rs` — a different product entirely. It had made `poll_ready` `pub` specifically so the mcp lease could reuse it rather than duplicate a waker dance; that is the right instinct and saved a second implementation.

**A third impl it missed, found by my build**: `RecordingRuntime` inside `🧵️shard/🦀️component.rs`'s own test module (`:1057-1085`) — 7 errors, all the same shape. It WAS inside its owned paths, so this is a genuine miss rather than a scope question; converted by me to ready futures. Worth naming because the packet's own single permitted `cargo check -p … --lib` could not see it: `--lib` skips `#[cfg(test)]` code, and the impl only compiles under `--all-targets`. **The narrow check that was permitted to conserve build capacity is precisely the check that could not observe this defect** — the same shape as W5's `--features component-guest` omission (rule 22). Rule 22 is hereby extended: the consumer-exact command includes `--all-targets` wherever a trait has test-only impls.

## 2026-08-19 W6/W7 — the PLUGIN side goes async-first

User critique, and it is correct: *"I still see no async — e.g. all the io inside plugins for all artifacts are still synchronous."* W5 built async **infrastructure**; the guest side never adopted it. Three read-only explorations measured the gap:

- The SDK's **24 `async fn` host methods** (`🌐host/🦀️component.rs:56-330`) have **zero call sites** in the fleet. `RequestRegistry`/`LocalExecutor` are unused scaffolding.
- **`Emit.tasks` never existed** (`Emit` :8557 — no tasks field), so no command can await anything. design-abi §4 was specified and never implemented.
- **Artifact IO is sync fn pointers** — `ComposerEntry{compose: fn(..)}` (`🚪️io:748`, dispatch :1007) across **143 io modules**.
- **Jobs are a closed hard-coded match** (`💼️jobs:62-66`), io-run/io-sniff only, single-step, `JobBudget` accepted and **ignored** — so **plugins cannot author jobs at all**. Hence a 10,930-LOC WFC solver dormant behind a "blocked" comment, and FEM/SfM/tessellation/exports running inside turns.
- **`Effect::HttpChunk` discards non-final chunks** (`⚛️reactor:143-147`).
- The **async execution backend was never built** — no `WasmtimeAsyncRuntime`, no `component-guest-async`, no async packaging.

### 📊️ Pre-wave adoption census (the number this wave exists to flip)

`census-async-adoption.py` (python over absolute paths — shell grep under-reports on emoji paths, rule 21), 33 plugins / **10,078 `.rs` files**:

| metric | today |
|---|---|
| `host::*` async call sites | **4** (draw 2, space 2) |
| `.await` | **6** |
| job registrations | **0** |
| `AsyncTask`/`Emit.tasks` | **0** |
| `block_on` | **134** (flow 59, cad 45, process 13, stdio 15, animate 2) |
| `fn pending_effects` | 3 definitions |
| `DownloadMediaExport` | 41 |
| `async fn` | 186 — but **184 are stdio's already-async BrepKernel trait**, which guests then `block_on` |

That last row is the whole story in one line: the codebase already has an async engine and the plugins block on it.

### Scope (user-confirmed)
1. **Infra slice dropped** — the pending runtime/db refactor owns 🛢️db/🔄️sync/🌉️mcp/HTTP-transport/shell+kernel consolidation/`🎒️pack`. A peer session is live in `🛎️services` right now (its wgpu + services checks were running at wave start). We consume services as-is and never edit those paths.
2. **Async execution backend IN scope** — plugins cannot run async without it.
3. **IO doctrine: every artifact io surface goes `async fn` uniformly** (user override — "regardless the effort"), with **jobs layered on top** for preemptibility of heavy codecs/solvers/exports. `handle`/`render` stay pure sync reducers; no detached spawn; bounded outstanding tasks.

### W6-A dispatched (4 packets, file-disjoint)
`probe-spikes` (owns the one build slot — its product IS build results) · `task-emit-core` (`🦀️component.rs` + `⚛️reactor/` + `📸️checkpoint/`) · `jobs-runtime` (`💼️jobs/` + `🏗️builder/`, leases the two hooks it needs in the sibling's files) · `trait-asyncify` (`🖥️host/` + `🧵️shard/`). Rule 23 applied from the start: executors write code + reasoning and mark acceptance **UNRUN**; the coordinator owns every build. Peers held 2 cargo slots at dispatch.

### ⚙️ Operational finding — ticket-folder target dirs now fail with EPERM

The wave's first acceptance build died on:
```
error: couldn't read `<ticket>/🎯️target-trait-asyncify/debug/build/serde_core-*/out/private.rs`:
       Operation not permitted (os error 1)
```
Not a code error. Reproduced in a **fresh** ticket target dir and again in a **warm** one from a prior pass; the file is plainly readable from the shell (`head` prints it, mode `rw-r--r--`, `com.apple.provenance` xattr) but rustc gets EPERM. The identical command with `CARGO_TARGET_DIR` under `/private/tmp/…/scratchpad/target-ta` **finished clean in 17.44s**.

Recorded as rule 24: **build target dirs live in the session scratchpad from now on.** Two benefits beyond unblocking — the ticket folder had accumulated ~20 target dirs (one at **5.1 GB**), and scratch build output never belonged in a ticket directory that `ticket_close` walks.

Worth noting how this presented: it looked exactly like a broken dependency ("could not compile `serde_core`"), and the honest first read — *is our code wrong?* — was answered by testing the same crate in a different target dir rather than by editing anything.

## ✅ S1b — INTER-STORE FAIRNESS IS GO. The async architecture's core premise holds.

The probe first returned **S1 NO-GO**: two CPU-bound guest tasks never interleave, task B getting zero polls until A completes. Read literally that kills the headline claim — *a suspended plugin costs state, not a thread* — for all CPU-bound work.

**It was measuring a shape the design never uses.** S1 tested `Accessor::spawn`-ed tasks **inside ONE `Store`** — wasmtime's own intra-store scheduler. The design's central rule is the opposite: **one root task owns one Store, never concurrent reentrant calls into one instance**; fairness is supposed to come from a level up, where *separate* Stores' `run_concurrent` futures are multiplexed by **our** executor. So I sent it back with the exact shape rather than accepting the blocker — keeping S1 in the record rather than overwriting it, since the two answers mean different things.

**S1b, run exactly as specified — two separate `Store`s, each its own `run_concurrent`, multiplexed by a host-level `join!` on the current thread, no `Accessor::spawn`, no extra OS threads:**

| | run 1 | run 2 |
|---|---|---|
| context switches | **149** | **139** |
| progress entries | 19,532 | 19,532 |
| exit | 0 | 0 |

Guest B is polled **well before** guest A finishes — the exact inverse of S1's signature. Sub-answers:
1. **Granularity ~1.25 ms** between switches (deltas 2.06 → 3.32 → 4.57 → 5.83 → 7.09 ms), tracking the 1 ms epoch ticker. The design's ~1 ms slice target is achievable, measured not assumed.
2. **Epoch `Yield(1)` propagates outward to OUR executor** — `fut_b` can only be polled if `fut_a`'s `run_concurrent` returned `Poll::Pending` to the combinator, and it observably does. This is the mechanism-level confirmation that S1's failure was specific to `Accessor::spawn`, not to epoch-Yield.
3. **Fuel-only is independently sufficient**: a separate `Engine` with `epoch_interruption` never enabled and `fuel_async_yield_interval(500_000)` gave **3,041 switches, byte-identical across both runs** — deterministic, because fuel consumption is not wall-clock dependent.

That third result is a gift the brief did not ask for: **fuel yields are deterministic, epoch yields are wall-clock.** So the scheduler can use epoch for real fairness in production and fuel intervals for **reproducible** fairness tests — which is exactly what this ticket has repeatedly lacked (budget 5's 30-samples-in-a-0.1 ms-band was a wall-clock artefact nobody could reproduce deterministically).

**Binding consequences, now in the spike register:**
- ✅ one `Store` per actor + host-level `join!`/`select!`/`FuturesUnordered` — the confirmed pattern.
- ❌ never `Accessor::spawn` across actors — S1's measured dead end.
- ⚠️ **S3 footgun promoted to a schema action item**: once a store enables `wasm_component_model_async(true)`, a **plain sync `func` export fails at RUNTIME** ("store configuration requires that *_async functions are used instead") with **no compile-time signal**. The `checkpoint` export must therefore be declared `async func` in the WIT. That would have been discovered the hard way, in the suspend/restore path, at the worst possible moment.

Other verdicts: **S2 GO** (dropping a pending host-import future signals cancellation to the guest — the drop-guard fired), **S3 GO with the caveat above**, **S4 GO** (`run_concurrent` futures are `Send`; no `LocalSet` fallback needed), **S5 GO** (custom `StreamProducer` parks on an empty queue, stores its waker, resumes when the host pushes — the turn-boundary-without-turns mechanism), **S6 GO** (a hand-rolled `Rc<RefCell>` local executor drives a wit-bindgen import future correctly; the SDK reactor needs no special-casing).

**Honest gap it volunteered:** S1b used 2 actors, equal 40 M-iteration workloads, one epoch interval and one fuel interval. The *mechanism* is proven; scheduling **quality** under 3+ actors or unequal workloads is unmeasured and is flagged for whoever builds the real scheduler. Correct scope discipline — proving the mechanism is the spike's job, tuning the policy is not.

### ✅ trait-asyncify accepted — `GuestRuntime` is awaitable, behaviour unchanged

Coordinator-run (scratchpad target per rule 24): `cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity` → **115 passed / 0 failed / 1 ignored, exit 0** — exactly the named baseline.

Verified by direct source reading before building, not taken from the report: `pub trait GuestRuntime` (`🖥️host/🦀️component.rs:506-524`) now returns `semio_framework_async::HostFuture<…>` from `execute_turn`/`start_job`/`step_job`/`cancel_job`/`checkpoint`/`restore`, while `compile`/`instantiate`/`drop_instance` stay plain `Result` — the async backend does its own async instantiation inside the actor task, so those never needed to move. 21 `HostFuture` usages; 10 `poll_ready` call sites in `🧵️shard`.

The consumption path is `poll_ready<T>(HostFuture<T>) -> T` (:544-555): polls once with `Waker::noop()` and **panics loudly** on `Pending`. That is the right shape — the poll-world impls do their work eagerly inside an immediately-invoked closure and return `Box::pin(ready(result))`, so always-ready is true *by construction*, and a future impl that is genuinely pending belongs to the async shard executor rather than to `pump`. A silent fallback here would have hidden exactly that mistake.

No `Send`-bound fight: the returned future owns only a plain `Result`, never the `Store`. Both earlier lease-requests (the `PostTurnRelay` `run_job_to_completion` sites and two `🌉️mcp/🏠️workspace` `execute_turn` sites) had already been applied by their region owners, so the crate compiles as a whole.

Bench re-run in flight to close the second half of the gate — the claim under test is that boxed-future indirection perturbs neither correctness (proven above) nor the measured pipeline.

### 🔧️ trait-asyncify's blast radius reached the wgpu bench — found by building, not by grepping

The executor's whole-repo grep concluded "no unwrapped live `GuestRuntime` call site anywhere". The **bench build disagreed**:
```
error[E0599]: no method named `is_ok` found for struct
              `Pin<Box<dyn Future<Output = Result<TurnResult, TurnFault>> + Send>>`
+ 7 × error[E0308] mismatched types
--> 🎯️targets/🧊️wgpu/📦️glue.rs:1315,1320,1326,1332  (budget-8's capability-revocation block)
```
Four raw `runtime.execute_turn(...)` calls in the scale bench, un-wrapped. `cargo test -p semio-framework-plugin-host` was **115/0 green** the whole time, because the defect lives in a *consumer* crate — precisely why rule 23 puts acceptance on the coordinator and why a packet's own grep is never accepted as proof of completeness (the standing lesson from A3's 132-file rename that missed a live import).

Fixed by me (unowned region, mechanical): all four wrapped in `semio_framework_plugin_host::poll_ready(...)`, then a census over the whole file confirming **0 remaining raw** `execute_turn`/`start_job`/`step_job`/`cancel_job`/`checkpoint`/`restore` sites. `cargo check -p semio-framework-os-renderer-wgpu --lib` → **exit 0, 0 errors**. Bench re-running.

### ✅ jobs-runtime delivered — plugins can author jobs for the first time

`⚛️reactor/💼️jobs/🦀️component.rs` **170 → 704 lines**: the closed hard-coded `match` is gone, replaced by a `kind → JobFn` registry with `register_job_kind`, a `JobCtx` (`tick`/`progress`/`checkpoint`/`budget`, plus `host()` **feature-gated to the async world**), slicing across `step_job` calls on a **dedicated** `LocalExecutor` — deliberately not the reactor's, so job slices never starve UI-turn tasks — a stall guard emitting `job.stalled` after 3 consecutive no-progress slices, and checkpoint/restore. `semio.io-run`/`semio.io-sniff` are preserved **byte-identically** as ordinary registry entries, so behaviour is unchanged where it already worked. 10 tests. `PluginBuilder::job(kind, run)` threaded through the same typestate plumbing `commands` already uses.

It filed **two precise lease-requests** rather than reaching into a sibling's files (the `JobsGuest` impl in `🔌️plugin/🦀️component.rs`, and a `jobs:` field in the checkpoint pack) and grep-verified those are the only real call sites — correctly noting that a lookalike `JobOutcome`/`step_job` in the scale-fixture crate is an independent hand-rolled module. **Its crate does not compile until those leases land**, which it stated plainly; they are sequenced behind `task-emit-core`, which still owns both files.

Honest gaps it declared: cancelling a job frees its own bookkeeping slot but not the underlying `LocalExecutor` slot (that executor has no by-id removal and is outside its ownership — a real follow-up), and the stall guard uses budget-equality as a proxy because there is no fuel metering yet.

## 🤝️ Handover absorbed — the previous coordinator of this ticket lost filesystem access mid-wave

The session that ran W5/W6 until ~09:40 today (author of the async `GuestRuntime`, `poll_ready`, and the 115/0/1 baseline this session is building on) reached me cross-session. It holds **nothing** — both its agents stopped, no edits in flight, no leases — and handed over sole coordination. Its handover (`…/41af9a75-…/scratchpad/W6-HANDOVER.md`) is folded in below because `📓️status.md` was unreachable from that session and the wave is otherwise undocumented from 09:40 onward.

### Landed and coordinator-verified there (before access was lost)
| packet | evidence |
|---|---|
| `dev-polls` | **27 passed, exit 0** (baseline 17 + 10). Zero exit-code poll loops remain; all 7 surviving `Bun.sleep` classified (3 docstrings, 2 injectable helper defaults, 2 documented fs/lock exceptions). New `🔖️PollHelpers` region carries the rule as a docstring |
| `trait-asyncify` | plugin-host `--all-targets` exit 0; `--lib --skip schema_parity` **115/0/1**; `schema_parity` **4/4** |
| `http-streaming` | `--all-targets` Finished; **30 passed / 0 failed** (baseline 26). Real-byte budget charging, one body impl feeding both worlds, a refill driver that actually runs (with a test), storage deadline racing, boot `block_on` removed |

### 🔴️ `semio-framework-os-kernel-db` is RED — 84 errors, and it is NOT a peer regression
An **atomic** `db-trait-flip` was interrupted mid-refactor when scope shifted: 9 db files + `🌎️hub/…/📦️bin.rs` are half-converted to async `DbFuture` traits. Signature errors: `E0425 cannot find function inline_fs_runtime`; many `E0277 ? on non-Try`; many `E0277 [u8] size not known` (borrowed slice params crossing into `Box::pin(async move …)`; the specified fix shape is `DbFuture<'a,T>` with `&'a self` + `&'a [u8]`).

**Deliberately NOT absorbed by this wave.** `🛢️db/**` is explicitly out of scope — the pending runtime/db refactor owns it — and finishing-vs-reverting a half-applied atomic refactor is an owner decision, not something to quietly adopt mid-wave. **Flagged upward to the user.** Recorded here so nobody misattributes it to a peer or to this wave's plugin work.

### Two rules this ticket earned (renumbered — 24 is already the target-dir rule)
25. **An atomic packet may be redirected BEFORE it starts, or allowed to FINISH — never interrupted.** A scope change does not make a half-applied atomic refactor safe. Cost of learning this: the 84 errors above.
26. **Neither `--lib` nor `--all-targets` is a sufficient gate alone — run both.** Hit from opposite directions the same day: `--lib` hid a `cfg(test)` trait impl (7 errors), and `--all-targets` hid a missing *production* `tokio` `macros` feature by unifying it from dev-dependencies. This wave immediately confirmed it: my `--lib` wgpu check was green while `--all-targets` surfaced a real remaining error.

### wgpu site count reconciled — 4 fixed here, 9 measured there, 0 outstanding
The handover listed 9 × E0308 in `🎯️targets/🧊️wgpu/📦️glue.rs` plus one at `Shell/🧊️component.rs:2471`. Measured on the **current** tree: my 4 `poll_ready` wraps cover every raw site in `📦️glue.rs` (whole-file census: 0 remaining raw `execute_turn`/`start_job`/`step_job`/`cancel_job`/`checkpoint`/`restore`), and `Shell/🧊️component.rs` has **zero** `GuestRuntime` call sites at all — line 2471 is presence-heartbeat code today. The 9-vs-4 gap is tree drift between the two measurements, not a missed fix, and **no Shell lease is needed**. `Dock/🧊️component.rs:1256,1259,1634` remains a third session's pre-existing break, unrelated to either of us.

Following rule 26, `cargo check -p semio-framework-os-renderer-wgpu --all-targets` now reports exactly **one** error, and it is not wgpu's: `⚛️reactor/🦀️component.rs:282` calls a private `plugin_runtime::instance_actor` — `task-emit-core`'s own in-flight work (that file was modified seconds before the check). Transient, its own file, left alone.

## ✅️ S1c — the S1/S1b contradiction is RESOLVED: **(A)**, CPU-bound actors CAN be multiplexed

Two sessions reported opposite results on the single question the whole async shard executor rests on. Settled by a third experiment I specified to kill my own hypothesis, not to confirm it.

**My hypothesis was that S1b was an artifact.** S1b's guest `burn` loop periodically called the `progress` **host import**, and every host-import call is an `.await` — a natural yield point. If that were the source of the 149 context switches, the interleaving would be *import-driven*, not `UpdateDeadline::Yield`-driven, and the peer's S1 NO-GO would stand.

**S1c: `burn-pure` — the identical CPU loop with ZERO host-import calls anywhere**, in S1b's exact shape (two separate `Store`s, host-level `futures::join!`, no `Accessor::spawn`), across epoch×{symmetric,asymmetric} and fuel×{symmetric,asymmetric}.

| lever | shape | result |
|---|---|---|
| epoch | symmetric 40M/40M | `t_a=229ms, t_b=230ms` — ratio **1.00**, not the 2.00 that sequential execution requires |
| epoch | asymmetric 300M/5M | tiny call returned at **30ms** while the huge call ran on for another **847ms** |
| fuel (separate `Engine`, `epoch_interruption` never enabled) | symmetric | ratio **1.00** |
| fuel | asymmetric | tiny **6ms** vs huge **197ms** |

All four reproduced on a second full run. **Verdict (A): epoch- and fuel-Yield genuinely preempt pure CPU-bound guest code across separate Stores, with no import confound.** S1b's GO stands on its own merits; my challenge was right to make and wrong in its conclusion.

**Both prior reports were also correct, about different shapes.** The peer's S1 NO-GO measured `Accessor::spawn` *inside a single Store* — that really is a NO-GO, and it is now permanently recorded as such. It simply is not the shape the design uses. Nothing narrows: **the async shard executor may multiplex CPU-bound actors**, and the architecture does not retreat to I/O-bound-only.

**A false start is preserved deliberately.** The first S1c attempt showed a "300M-iteration" call returning in 6µs with 0 epoch hits — LLVM had strength-reduced the side-effect-free loop to closed-form arithmetic and deleted it. Fixed with `std::hint::black_box` per iteration. The run log is kept as `terra-s1c-*-BROKEN-optimized-away.txt` so it can never be mistaken for a real measurement, and the pitfall is written into the probe report as a standing warning: **any future CPU-bound guest probe must defeat the optimizer or it measures nothing.**

S1, S1b and S1c are all kept as separate permanent entries in `📓️terra-probe-spikes-report.md`. This closes the last open spike; the EC slice is unblocked on its own terms.

## ✅️ `db-trait-flip` FINISHED TO GREEN — the RED flagged upward has been closed out by the owner

The user, holding the decision this ticket correctly refused to make mid-wave, chose **finish, not
revert**. Done. Full detail: `📓️db-trait-flip-completion-report.md`; plan: `📓️db-trait-flip-completion-plan.md`.

| gate | before | after |
|---|---|---|
| `semio-framework-os-kernel-db --lib` | 83 errors | **exit 0** |
| `… --all-targets` | 361 errors | **exit 0** |
| `… --lib` test suite | could not build | **424 passed / 0 failed / 0 ignored** |
| `semio-hub --all-targets` | 3 errors | **exit 0** |
| `semio-hub` tests | could not build | **11 + 20 passed / 0 failed** |

**The packet was missing a decision, not code.** The trait family was already `DbFuture`; *no* caller had
been converted (`grep -c "async fn"` was 0 in every db component). It stopped precisely between its two
halves, so the sync/async boundary had never been chosen. Chosen now: **pure-logic layers go `async fn`**
(`db_snapshot`/`wal`/`index`/`compact`/`sync`/`cluster`/`projection`/`query` — they own no threads, and
`async fn` keeps them `wasm32`-clean without boxing); **thread-owning layers keep their sync signatures
and bridge once** with `db_actor::block_on` (`db_artifact` on the `ArtifactAuthority` thread, `db_engine`
on its per-submit bridge threads, `db_cli`, and every `#[cfg(test)]` module); **`🌎️hub` is genuinely async
and just `.await`s.** The handover's hard constraint held verbatim: **no db-actor thread converted, no
`db_engine` bridge thread deleted.** Blocking moved outward one level — out of each backend body into the
thread that already owned the call, which is exactly where it used to live.

**`E0425 inline_fs_runtime` resolved by deduplication rather than by writing it.** `db_cli` already had a
private `CliRuntime` doing that job. The one implementation now lives beside the `FsStorage` that needs it
as `db_storage::InlineRuntime` + `FsStorage::open_inline(owner, root)`; `CliRuntime` is deleted. It also
absorbed two stale 1-arg `FsStorage::open(root)` call sites (`db_cli` at HEAD, `db_testkit`) that the
interrupted packet had left un-compilable.

**🚨️ One real defect surfaced only by finally being able to RUN the suite.**
`db_preview::tests::preview_crate_never_references_wal_shaped_symbols` — that crate's single most
important law, "previews are never durable" — **failed**. W6 had added prose to the crate's `Cargo.toml`
explaining the sync/async boundary, and that prose names `db_storage`; the guard did a raw
`manifest.contains(…)`, so a *comment* tripped a *dependency* law. Invisible because the crate had not
compiled since. Fixed at the guard (comment lines stripped first) so the law tests what it means to test.
**This is rule 26 one level up: a green `--lib` is not a passing suite either. Compile both, then RUN.**

### `wasm32-unknown-unknown` is red for this crate, and it is NOT a regression from this work
66 errors, verified pre-existing rather than assumed: `db_artifact` calls `recv_blocking`/`ask_blocking`
(both `not(wasm32)`-gated in `db_actor`) — `git diff` shows zero working-tree changes from me in that
file and `git log -S` dates those calls to **2026-08-10**; `db_engine`/`db_cli` name the correctly-gated
`FsStorage`, and `git show HEAD:…` confirms that reference predates my edit. The thread- and fs-owning
trio has never been `wasm32`-clean; the module doc's `wasm32` claim is scoped to `db_storage` itself and
still holds. Every new `block_on` landed **inside those same already-native-only modules** — the
pure-logic layers went `async fn` specifically so they stay clean. Making the trio `wasm32`-clean belongs
to the pending runtime/db refactor. Per rule 21 this is stated as measured, not as "pre-existing" by assertion.

### 🔁️ Rule 27 — a compiler-driven refactor still needs a region guard on name-keyed edits
~450 mechanical edits were driven off `--message-format=json` spans to a fixpoint (four scripts kept in
this folder), which is the right tool at this size. Two traps, both caught by the compiler and neither
findable by grep: (a) a non-greedy "wrap the tail expression" regex swallows `assert_eq!(`'s opening
paren — the paren structure stays valid, so the repair is a *swap* of the two prefixes, not a re-parse;
(b) **a pass keyed on a variable NAME (`result`) is not scoped to tests and will hit production code with
the same name** — it did, one site in `db_compact::Compactor::run`, caught by `Result<…> is not a future`.
Span-keyed edits are safe; name-keyed edits need an explicit line-range guard.

### Re-verified, not assumed: `wgpu-poll-ready` was already closed by the peer
The handover listed it "not started". On the current tree `semio-framework-os-renderer-wgpu --lib` is
**exit 0** and `poll_ready` wraps are present at `📦️glue.rs:1315,1320,1326,1332` — matching this ticket's
own "4 fixed here, 9 measured there, 0 outstanding" reconciliation. Nothing to do. `--all-targets` still
reports 26 errors, all in `#[cfg(test)]` UI-schema code in `Dock`/`Shell`/`Interpreter`
(`LocalizedLabel::data`, `UiPresence`, `PresencePeer.presence_pack`) — a third session's live work
(`Shell` was committed at 09:57 today), with **zero** working-tree changes from me in any of the three.
Left alone.

## ✅️ W6-A CLOSED — all four packets accepted, with three real defects found only by the acceptance build

Every packet was delivered UNRUN per rule 4, so all of the following was found by me, not by the executors. This is rule 23 doing exactly what it exists to do.

### `async-imports` — accepted after one fix
`⏳️imports.rs` (807 lines, all 24 `host-async` imports) + a 6-line mount. It had **never been compiled**; the first build failed with a single clean error:

`additional_derives: [Clone]` applies **blanket to every generated type**, and the async world carries `stream<u8>` → `StreamReader<u8>`, a one-shot resource handle deliberately not `Clone`. The poll world has no streams, which is why the sibling `mod actor_bindings` gets away with it. Removed the derive (nothing needed it) and dropped one unused import. Now **`--lib` and `--all-targets` both exit 0**, plugin-host tests **113/0/1 + schema_parity 4/4**. The caveat is written into the module docstring so nobody re-adds it.

### plugin-host baseline moves 115 → **113**, and it is not a loss
Two tests genuinely disappeared: `an_effects_deadline_is_enforced_and_the_loser_is_cancelled` and `race_deadline_returns_the_primary_result_when_it_finishes_first`. I did not accept the count and did not accept "probably fine" — I diffed the named sets against `HEAD`.

**`git diff` reported the file as unchanged, which was a lie of tooling, not of the tree**: the auto-commit stages everything (`git add -A`), so worktree == index and a bare `git diff` is empty by construction. `git diff HEAD` showed −96/+5. **Standing correction: in this repo, always diff against `HEAD`, never bare `git diff`.**

The deletion is correct: `race_deadline` was a call-site helper compensating for `StorageScheduler` not racing deadlines internally. `StorageTicket::await_result` now races its own deadline and returns `StorageError::DeadlineExceeded`, so the helper became dead code and went out with its tests. Coverage moved down a layer and is real — verified by name in `🛎️services`: `storage_scheduler_races_a_queued_job_against_its_deadline_and_frees_its_reservation_when_lost` and `run_blocking_deadline_actually_fires_and_the_late_result_is_not_awaited`.

⚠️ **One genuine behavioural narrowing, recorded not hidden**: the new in-scheduler race only fires for **queued** jobs. `StorageScheduler`'s own doc says an already-dispatched job is not preempted. The deleted call-site race covered *any* storage op, dispatched or not. So a slow already-running storage op no longer times out at the effect layer. Documented as an honest limitation at the source; **belongs to the pending runtime/db refactor**, not absorbed here.

### `task-emit-core` + `jobs-runtime` — accepted after three fixes
`--lib` was green; **`--all-targets` was red**, rule 26 again, immediately after it was written down. Two broken test lines: an unescaped `{value:42}` read as a format placeholder, and a missing `use crate::store::FaultFrom`.

Then the suite ran **6 failed, then 7 failed on a re-run** — a moving count, which is never "flakiness to retry past" but a signal of shared mutable state. Against the named 5-failure baseline, two were new, and isolation separated them cleanly:

**🔴️ Real defect — a leaked quota slot on every cancellation.** `instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot` failed deterministically. **`RequestFuture` had no `Drop` impl anywhere in the repo.** Dropping a task's future — the *only* cancellation mechanism, used by both `cancel_instance_tasks` and the key-dedupe replacement path — left its `Pending` slot and instance tag in the registry forever. The registry sweep hid it at instance close, but nothing hid it for key-dedupe: a plugin re-keying a task (the `latest-wins` idiom this very wave introduces, e.g. a search-as-you-type task) would leak one `outstanding_requests` unit per keystroke and eventually be refused its own quota **with nothing actually pending**. Fixed at the root with `impl Drop for RequestFuture` releasing slot + tag, making drop-is-cancellation complete and matching the host side's `CancelOnDrop`. The test's expectation was right; the implementation was missing.

**🟡️ Test-isolation defect.** `host_media_conflicts_reject_the_whole_candidate_before_execution` passed alone, failed in-suite: two tests share the process-global `MESH_DWG_EXECUTIONS` counter and **both reset it to 0**, so concurrently each observes the other's increments. Fixed with a shared `MESH_DWG_GUARD` mutex both tests take.

**Result: exactly the 5 known-by-name pre-existing failures, stable across 3 consecutive runs — 263 passed.** The long-documented 5-vs-6 count wobble is **gone**, because the mutex removed the actual race rather than papering over it.

### 🕳️ A gate that was never checking anything
`jobs-runtime` gated `JobCtx::host()` behind `#[cfg(feature = "component-guest-async")]` — a feature **never declared in `Cargo.toml`**. An undeclared feature makes every such block permanently unreachable, so that code had never been type-checked by any build. Declared it (with the reason in a comment). This is the wasm-gated-code trap in a new costume: **a `cfg` gate you cannot enable is indistinguishable from deleted code, and the compiler only warns.**

### Consumer re-check
`semio-framework-os-renderer-wgpu`: **`--lib` exit 0**, and the `instance_actor` privacy error is gone now that `task-emit-core` landed. `--all-targets` still fails, **27 errors, none of them ours** — `LocalizedLabel` not imported in element test modules (Dock 11, Shell 13, Interpreter 2). The type is publicly defined at `🖱️ui/…/🦀️label.rs:90`; those test modules simply lack the `use`. Files last touched 08-17/08-19 00:58 — **stale, not live**, so this is a pre-existing break that has grown beyond the Dock-only note in the baseline. Renderer element test modules are outside this wave; recorded, not absorbed.

## ▶️ W6-B dispatched — 3 packets, disjoint ownership

| packet | owns | note |
|---|---|---|
| `io-async-signatures` | `🔌️plugin/🦀️component.rs`, `🚪️io/**`, all fleet io modules | **ATOMIC** — briefed under rule 25 in the strongest terms: do not stop halfway, do not ask |
| `sdk-async` | `🌐host/**`, `⚛️reactor/🦀️component.rs`, `📮️requests/**` | dual `HostBackend{Poll,Direct}`, `BodyReader`, and the discarded-chunk bug |
| `cold-kinds` | `⚛️reactor/💼️jobs/**`, `🖥️host/🦀️component.rs` | `semio.infer`/`mutation-plan`/`migrate` + host routing |

**The plan's scope figure for the io sweep was wrong and I corrected it before dispatch.** The plan said "143 io modules". Measured: **223 files carrying 226 artifact-io trait impls** (`ArtifactEditor` 147, `ArtifactDeserializer` 39, `ArtifactSerializer` 39, `ArtifactComposer` 1), plus **163 `ComposerEntry{…}` constructions** across 31 files and reference sites at `composer_entry_of` 227/91 files, `serializer_entry_of` 126/16, `deserializer_entry_of` 64/15. `🗄️stdio` alone holds 164 of the impls. Compose is a **fn pointer**, not a trait method, so it needs a named future-returning type alias rather than 163 hand-spelled `Pin<Box<dyn Future…>>`.

An earlier, cruder census of mine said 894 files / 946 functions — that counted every `fn serialize(`, which sweeps up serde's own `Serializer` impls. **Recorded as a caution: `fn serialize(`/`fn deserialize(` are serde-shaped names and are useless as an artifact-io census; count trait impls instead.**

**`semio.compose` was deliberately withheld from `cold-kinds`.** Its body needs the `ComposeStepper`/`ComposeState` types that `io-async-signatures` is defining right now; letting two packets define them would be exactly the interleave rule 25 exists to prevent. `compose-await` (W6-C) picks it up once the types are real.

### 📊️ Adoption census after W6-A — deliberately still zero, and that is the correct reading

`census-async-adoption.py` re-run against the fleet (`✏️s/🔌️plugins/**`, 10,078 `.rs` files) is **unchanged from the pre-wave baseline**: host_calls **4**, async_fn **186** (184 of them the stdio BrepKernel), await **6**, `block_on` **134**, `pending_effects` **3**, `register_job_kind` **0**, `AsyncTask` **0**, DownloadMediaExport **41**.

This is not a disappointing result, it is the expected one, and it is worth stating plainly so nobody reads W6-A as progress against the user's actual complaint. W6-A built **mechanisms inside the framework** — `AsyncTask`/`Emit.tasks`, the open jobs registry, the async `GuestRuntime`, the 24 `host-async` imports. The census deliberately measures **the plugin fleet**, which is where the user looked when they said "I still see no async". Nothing there has moved yet, and nothing was supposed to.

The number that must move is `async_fn`, and W6-B's io sweep is what moves it — 226 impls across 223 files. `block_on` → 0 and `pending_effects` → 0 belong to W6-C; `register_job_kind`/`AsyncTask` climb through W7. **This census is the wave's headline metric and gets re-run at every gate.**

## 🔍️ S7 opened — the async world may not be able to run jobs at all

Checking the S3 caveat myself (rather than delegating it) turned up something larger than the caveat.

Measured in `🔌️plugin/🧬️schema/📜️component.wit`:

| interface | funcs | exported by |
|---|---|---|
| `runner` | `run: **async func**(events: stream<event>)` | `actor-async` only |
| `host-async` | all 24 imports **`async func`** | `actor-async` only |
| **`jobs`** | `start-job`, `step-job`, `cancel-job` — **all plain sync `func`** | **both worlds** |
| **`checkpoint`** | `checkpoint`, `restore` — **both plain sync `func`** | **both worlds** |

The jobs runtime gates `JobCtx::host()` behind `component-guest-async` — a job body that **awaits a host import**. S2 already ruled that out for the poll world (`run_job_to_completion` never pumps `poll`, so a host-await deadlocks). If a sync-lifted `step-job` also cannot await an async import, then **`JobCtx::host()` is unimplementable in both worlds** and we have shipped a gate that can never be switched on — a sibling of the undeclared-feature trap found earlier today, one layer down.

**I am not deciding this from the spec.** It reads both ways: CM-async may forbid it outright, or wasmtime may permit it by blocking that instance while other stores keep running — which, for our one-store-per-actor shape, could be entirely acceptable since jobs are budget-stepped anyway. The failure mode that actually matters is narrower and testable: **`runner.run` is driving the event stream in the same instance while `step-job` blocks on an import — is that a deadlock?**

S7 dispatched to the probe agent (it owns the only working async harness) to answer by experiment, with a copy of the WIT — **not the live file**, since four packets are mid-flight and the schema is shared. Verdict (A) leaves the schema untouched; verdict (B) requires splitting into `jobs-async`/`checkpoint-async`, hoisting `job-budget`/`job-step` into a shared types interface for both to `use` (rule 20: `use`d types are aliases), and **consciously re-specifying** plugin-host's `both_worlds_share_the_same_export_surface_and_actor_is_untouched` parity test — that test encodes an invariant the split would deliberately break, so it must be changed with intent, never "fixed" to go green.

I also **withdrew the schema-fix instruction from `async-runtime`** mid-flight: it was told to fix `checkpoint` if it wasn't async, which would have had it editing the shared WIT against a live probe. It is now building against the schema exactly as it stands and recording any async-dependency as an explicit blocked seam instead of quietly working around it.

## ⚠️ Seam recorded: `poll_ready` becomes a live panic the moment an async runtime exists

`poll_ready` polls a `HostFuture` **once** with a no-op waker and **panics** on `Pending`. That is correct and harmless today, because the only real `GuestRuntime` impl is the poll-world `WasmtimeRuntime`, whose futures are always eagerly ready. It stops being harmless the moment `async-runtime` lands a genuinely-async impl behind the **same `Arc<dyn GuestRuntime>` trait object** — nothing in the type system stops an async runtime from reaching a `poll_ready` call site, and the failure mode is a panic deep inside a turn rather than an error at wiring time.

Production (non-test) call sites, audited:

| site | owner | risk |
|---|---|---|
| `🧵️shard/🦀️component.rs` ×8 | `ShardLoop` | **By design** — `ShardLoop` is explicitly the poll backend; `async-shard` (W6-D) is its async counterpart |
| `🖥️host/🦀️component.rs:1444,1446` | `run_job_to_completion` | **Live risk** — takes whatever runtime it is handed, and `cold-kinds` is routing compose through it right now |
| `🌉️mcp/🏠️workspace/🦀️component.rs:327,399` | MCP workspace gateway | **Live risk, and OUT OF SCOPE** — `🌉️mcp` belongs to the pending runtime/db refactor |

The wgpu `📦️glue.rs` sites are bench/test and carry no production risk.

**Required end state:** a doc comment is not a guard. Backend selection must make this impossible by construction — an async runtime must be rejected at wiring time with a typed error, not discovered by a panic mid-turn. Concrete shape: a provided method on `GuestRuntime` (e.g. `supports_synchronous_polling()`, defaulting true, overridden false by the async impl) asserted at `ShardLoop`/`run_job_to_completion` **construction**.

**Not doing it now, deliberately.** The trait lives in `🖥️host/🦀️component.rs`, which `cold-kinds` owns this minute; editing a file another packet is mid-flight in is precisely what rule 25 exists to prevent. Folded into the W6-D backend-selection brief instead, and `🌉️mcp` will be handed to the pending refactor as a named seam rather than silently left as a latent panic.

## 🔴️ S7 verdict: **(B)** — and the finding is categorical, not conditional

The probe built a deterministic test (no wall-clock timing, no background threads): a host import requiring exactly 5 real polls before resolving, against three guest exports — a trivial no-import sync `func`, a sync `func` that spin-polls the import, and an `async func` control.

**A plain sync `func` export is UNCALLABLE on any `Store` configured with `wasm_component_model_async(true)`.** Not "deadlocks", not "blocks the instance" — every call fails immediately with `"store configuration requires that *_async functions are used instead"`. It fails identically whether called reentrantly inside `run_concurrent` via `Accessor::with` **or** with a classic `&mut Store` call on a completely idle store that never opened a concurrent session, and it fails **even when the export touches no import at all**. The `async func` control twin worked normally, proving the harness sound. Reproduced twice, exit 0.

This is stronger than the question I asked. I framed it as "trap, block, or work"; the real answer is that the export cannot be reached in the first place. **Good news for diagnosability** — it is a loud, catchable `Err`, never a hang — and unambiguous for the schema: `world actor-async`'s sync `jobs` and `checkpoint` exports are dead code that would fail on first call.

The probe's WIT diff (`terra-s7-component-wit-diff.md`, **not applied to the live tree**) hoists `job-budget`/`job-step` into `interface types` so both interfaces `use` the identical type (rule 20), adds `jobs-async` + `checkpoint-async` with `async func`, and repoints `world actor-async`. **`world actor` is byte-identical — zero lines changed.**

### 🕳️ The diff is incomplete, and I found the hole by checking the consumer
`world actor-async` also carries **`export describe;`**, and `interface describe` is likewise a plain sync `func`. By S7's own categorical rule it is equally uncallable there.

Checking the actual consumer rather than reasoning about it: `📇️describe/…/📦️glue.rs:122` builds its **own** `wasmtime::Config` with only `consume_fuel(true)` — **no async** — links `pure` + wasi-p2, and instantiates `actor_bindings::Actor`, i.e. the **poll** world on a sync store. So today the descriptor pipeline cannot describe an async-world component at all: its `host-async` imports are unlinkable there, and async imports cannot be lowered on a non-async store.

Two coherent resolutions:
- **(A)** Every plugin ships dual artifacts, the descriptor is always generated from the **poll** artifact, and the async world's `describe` is simply never called. Cheap, consistent with the planned `hashes.async_wasm_sha256`, but it leaves an exported function that would fail if anyone ever called it — and *"it is never called"* is exactly the assumption that produced today's other two traps (the undeclared `component-guest-async` feature, and a `cfg` gate nothing could enable).
- **(B)** Add `describe-async` so an async component is describable on its own terms.

**Taking (B).** The cost is one WIT interface plus a macro export; the alternative preserves a latent trap of a kind that has already bitten this ticket twice today. `abi-descriptor` (W6-D) will be briefed to run describe on an async-configured store with stub `host-async` imports for async artifacts.

### ⏸️ Deliberately NOT applying the schema change yet
It touches the shared schema, the guest SDK's `plugin_exports!` macro (in `🔌️plugin/🦀️component.rs`), the host bindgen, and plugin-host's parity test. **`io-async-signatures` owns `🔌️plugin/🦀️component.rs` right now and is mid-atomic-sweep.** Applying this on top would be precisely the interrupted-atomic-packet failure rule 25 exists to prevent — the one that cost 84 errors in the db crate. It lands as its own atomic packet once the io sweep reports.

### 🏷️ Numbering collision, recorded so it never confuses anyone
"S7" was **already** assigned to the earlier sqlx/neo4rs `Send` check from the W6 sweep. The probe renumbered this spike **S9** in the report prose, but its code and run logs literally print "S7" because that is what was executed. Both names refer to this one experiment. The verdict table and section header in `📓️terra-probe-spikes-report.md` flag it.

**Also relayed immediately to the live `async-runtime` packet** (it is wiring epoch/fuel budgets this minute): `set_epoch_deadline` takes a **delta, not an absolute epoch** — it computes `current_epoch + delta`, so the natural "no deadline" sentinel `u64::MAX` **wraps and traps the whole process**. The probe hit exactly that on its first run. That packet was also told the sync-export shape is dead rather than merely awkward, so it builds against async `jobs`/`checkpoint` directly instead of recording a seam.

## 🚩️ F-wave announced — the fleet turn (all 33 plugins, every surface) begins as a SECOND coordinator

A second coordinator (Opus 5, separate session) is starting the **fleet program**: turning every surface of
all 33 plugins — `🚪️io`, `🧬️mutations`/`🦠️mutation`, `🪛️utilities`, `🎮️commands`, `💡️inferences`,
editors/viewers — onto the async/actor model, to an exit bar of **G3 + the 8 bench budgets on all three
renderers**. Plan of record: `/Users/ueli/.claude/plans/you-must-turn-all-nested-papert.md`.

This is the `M0…M8` / W3 work finished properly, plus the jobs half that W3 correctly declared out of reach.
Packet slugs (ruling 5) audited against this log and the ticket folder before reserving — **zero collisions**:
`fleet-stdio` `fleet-small` `fleet-cad` `fleet-flow` `fleet-imperative` `fleet-heavy-a` `fleet-heavy-b`
`fleet-long-tail` `fleet-demonstrator` `fleet-census-zero` · `claim-helper` `dialect-arbitration` ·
luna audits `claims-audit` `dialect-audit` `census-baseline` `brep-await-spec`.

**Path contract, so the two coordinators cannot collide:** fleet packets own `✏️s/🔌️plugins/<p>/**` only.
Root `Cargo.toml`, `📇️registry`, `launch.json`, `🎠️kernel`, `🛂️manifest` and every `🔌️plugin/**` framework
file stay registrar-only, by lease-request. W6-B/W6-C landing notices in this log are treated as gate triggers.

### ⚠️ Measured before dispatching: `io-async-signatures` is sweeping WIDER than its declared scope

Its registry row reads `🔌️plugin/🦀️component.rs`, `🚪️io/**`, "all fleet io modules". Measured just now by
mtime under `✏️s/🔌️plugins` (python over absolute paths, rule 21 — shell globbing under-reports here):

| touched < 60 min | area |
|---|---|
| 104 | `🚪️io/**` — its declared scope |
| **39** | **`✏️editor/🦀️component.rs`** — NOT in its declared scope |
| 1 | `🗄️stdio/🗿️artifacts/🖊️dwg/🦀️component.rs` |

The 39 editor files are almost certainly legitimate cascade (`ArtifactEditor` is one of the traits being
asyncified), so this is a **scope-statement** gap, not misbehaviour — but it matters to anyone planning around
that row, because `✏️editor/🦀️component.rs` is exactly where the three `pending_effects` sites live
(`🌊️flow:324`, `🌀️procedural2d:257`, procedural3d). **The fleet wave is therefore holding its entire
plugin-editing tranche until the io sweep reports**, rather than discovering the overlap by breaking an
in-flight atomic packet. Rule 25, applied to somebody else's packet.

Suggested for the next registry row: state cascade paths in the scope, or say "scope = declared paths + their
compile cascade" explicitly. A reader cannot infer 39 editor files from `🚪️io/**`.

### ✅️ Correction: the `describe` per-crate wiring is ALREADY DONE, fleet-wide

`📓️design-abi.md` §3 lists "each plugin crate: `describe`" as outstanding, and an exploration pass this
session reported it unwired. Both are stale. Measured: **33 of 33** plugin crates carry a `describe` target in
`📦️packages/🦀️rust/📋️project.json` and a `DescribeScript` calling `describePluginComponent(...)` in their
`📜️script.ts`. Verified by reading `🖍️draw`'s pair and by a python scan of all 33.

So descriptor emission tooling is **not** a blocker for any plugin. The 23 unemitted descriptors are blocked
only by the five already-classified data/mechanism causes (claim-rule ×7, weak-linkage ×2, dialect collision
×2, kit.catalog ×1, crate-type ×1). The planned `describe-scripts` packet is dropped as a no-op before anyone
spent a wave on it.

## 📦️ `sdk-async` delivered — acceptance genuinely blocked, and the block is verified not its own

Delivered inside its owned paths: `HostBackend{Poll(RequestRegistry), Direct}` with all 24 `Host` methods as two-arm matches (Poll arms byte-for-byte unchanged), a second `wit_bindgen::generate!` for `world actor-async` gated on `component-guest-async` + real wasm32-wasip2, a new `BodyReader` (`🌐host/📖️body/🦀️component.rs`) over `StreamReader<u8>`/accumulated bytes wired into `http_fetch`/`blob_read`, and **the discarded-chunk fix**: registry slots gained a `partial: Vec<u8>` plus `append_chunk(id, bytes, done, cap)` capped by `QuotaSchema.message_bytes` (default 64 MiB), faulting rather than truncating over cap. 7 new tests including exact byte-for-byte multi-chunk reassembly.

It reported acceptance as blocked rather than claiming green — correct behaviour. **I verified the block instead of taking it on trust:** `cargo check -p semio-framework-plugin --lib` gives **9 errors, all 9 in `🔌️plugin/🦀️component.rs`** — the file `io-async-signatures` owns and is mid-atomic-sweep in. Zero errors in `🌐host/**`, `⚛️reactor/🦀️component.rs`, `📮️requests/**`, or the new `📖️body/`. Its claim stands; acceptance waits on the sweep.

### ⚠️ One cross-packet API change to reconcile at acceptance
`sdk-async` found that `⚛️reactor/💼️jobs` calls `crate::reactor::host()` with **zero args** while the only `host` function took `instance: u32`. It fixed this **in its own file** by splitting into `host_for_instance(instance)` + a new zero-arg `host()`. That is within its ownership, but `💼️jobs/**` belongs to the live `cold-kinds` packet, which may be resolving the same mismatch from the other side. **Flagged for reconciliation at acceptance — two independent fixes to one seam is a merge hazard, not a bonus.**

## ✅️ `cold-kinds` accepted (host half) + `async-runtime` delivered + one wiring gap closed

### `cold-kinds` — host half accepted by my own build
Guest side: `semio.infer`, `semio.mutation-plan`, `semio.migrate` as real submodules dispatching to the **existing** native registry (`wire_artifact_infer`, `wire_artifact_mutation_plan`, `store::migrate_document`), sliceable via a shared `run_two_phase` (2 real ticks, monotonic progress, checkpoint/restore round trip), 10 tests each exercising a real registered service rather than a mock. Host side: `PluginInstanceHandle::{mutation_plan, migrate, compose}`, `IoRouter::compose`'s hard-coded error replaced with real `run_job_to_completion` dispatch, and `ArtifactMutationRouter` gaining `runtimes` + `plan()`.

Coordinator-run: `--lib` **0 errors**, `--all-targets` **0 errors**, tests **118 passed / 0 failed / 1 ignored** (baseline 113, net **+5**) and `schema_parity` **4/4**. After my wiring change below, the full `--lib` run is **122 passed / 0 failed / 1 ignored**.

Its **10 guest-side tests remain UNRUN** — that crate is held down by the live io sweep, and the packet said so rather than claiming green. Queued for re-run.

### 🔌️ The wiring gap that would have made `plan()` dead on arrival
`ArtifactMutationRouter::plan()` had **no live production caller**: `🏃️run/🦀️component.rs:1521` called `register_roster(...)` directly, which never populates `runtimes`. So `plan()` would have compiled, tested green, and dispatched to an empty map in production — the same shape as today's other three "compiles but can never run" findings.

The packet offered two ways to fix it and asked. Both were wrong, and checking the call site is what showed why:
- **(a) route it through `register_plugin`** — impossible: that method decodes raw wire bytes, and `🏃️run` builds `HostMutationRosterEntry` values straight off the descriptor. It would mean encoding typed rows to JSON purely to decode them again. (The inference twin at :1532 *does* pass wire bytes, but only because its roster is already `serde_json::Value`.)
- **(b) add a separate `register_runtime(plugin_id, handle)` call beside the existing one** — a second call a caller can forget, which is precisely the failure being fixed.

Nor could I simply add the handle to `register_roster`: its docstring marks it the deliberately **pure, wasm-free half**, and **15 routing tests** exercise the contract §4 gating rules without ever constructing a `PluginInstanceHandle`. Forcing a handle through it would destroy that split.

**Applied instead:** `register_roster_with_runtime(plugin_id, deps, handle, roster)` — a wrapper over the pure core that registers the runtime in the same call, with `register_plugin` (wire path) now delegating to it, and the `🏃️run` call site switched to it. One call, impossible to forget, pure core intact. `semio-framework-os-run` shows **0 errors in `🏃️run`** (its other 31 are the live io sweep's stdio composers mid-flight — callers still `?`-ing what is now a future, exactly the expected intermediate state of an atomic signature change).

### `async-runtime` delivered — `⏳️runtime.rs`, 544 lines, in-tree compilation honestly UNRUN
One `Store` per actor owned **inside** a spawned task, `store.run_concurrent` with a `select!` over the guest `run()` call, grant exhaustion, refill and command channels. Turn results synthesized host-side (fuel delta + drained sink) and emitted as raw `TurnResult`, mirroring `ShardOutcome::Turn`; it deliberately never calls `Kernel::complete`, matching the `ShardLoop`/`ParallelRuntime` precedent.

**Its most consequential finding, proven in a 6-test scratch harness against real wasmtime 47.0.3: dropping the `run_concurrent` future alone does NOT cancel an in-flight host import — the owning `Store` must be dropped too.** That is why the `Store` is constructed inside the spawned task body, so `JoinHandle::abort()` genuinely cancels. Anyone who assumed future-drop was sufficient cancellation (the reference architecture's own phrasing invites that assumption) would have shipped a runtime that leaks live imports on every cancelled actor.

Both mid-flight corrections landed: first "leave `jobs`/`checkpoint` sync, record a seam", then after S7 settled it, "the sync shape is dead — wire the async exports through the same accessor path". The generated binding names for `jobs-async`/`checkpoint-async` are **predictions**, since that schema change has not landed.

**Lease granted:** `⏳️imports.rs`'s `mod host_async_bindings` → `pub(crate)`. Applied. **Mount line held back** (`#[path = "⏳️runtime.rs"] pub mod runtime;`) until the schema packet lands, since the file cannot compile against a schema that does not exist yet.

## 🌊️ SCOPE ESCALATION (user, 2026-08-19) — universal async, and WIT 0.3 async throughout

Two direct instructions:
1. *"Every single function must have async keyword and be implemented with async, doesn't matter if it breaks the code."*
2. *"use wit 0.3 that accepts async"*

This supersedes the wave's io-only async doctrine. Not a widening of `io-async-signatures` — a different, larger job that subsumes it.

### Measured scope (and a correction to my own first number)
My first census reported 567,016 functions. **That was wrong** — the modifier group in my regex over-matched, and it swept in `.🧬semio` probe fixtures. Corrected, over 10,559 first-party `.rs` files:

| category | count | share |
|---|---:|---:|
| **CONVERTIBLE → `async fn`** | **76,211** | 95.1% |
| already `async` | 2,441 | 3.0% |
| **external-trait impls — language-fixed** | **1,442** | 1.8% |
| `const fn` — language-fixed | 45 | 0.1% |
| `fn main` | 22 | 0.0% |
| `extern "C"` — language-fixed | 8 | 0.0% |
| TOTAL | 80,169 | |

**~1.9% cannot carry `async`, and that is the compiler's rule, not a judgement call**: `Drop::drop`, `Display`/`Debug::fmt`, `Iterator::next`, `Future::poll`, serde's `serialize`/`deserialize`, `From`/`TryFrom`, `const fn`, `extern "C"`. Their signatures are fixed by traits declared outside this repo; `async` on them does not compile under any setting. Everything else — all 76,211 — goes async.

### WIT: 12 remaining sync funcs → `async func`
The schema is already 25 `async func` vs **12 plain `func`**: `pure`'s 3 (`log`/`now-ms`/`trace-span`), `reactor`'s, `jobs`'s 3, `checkpoint`'s 2, `describe`'s 1. All 12 become `async func` under WASI 0.3.

**This dissolves the `jobs-async`/`checkpoint-async` split S7 called for** — with every func async there is nothing to split, and the shared-interface problem disappears. Simpler than the diff the probe produced, and it lands S7's finding rather than working around it. ⚠️ **Consequence to face squarely:** `world actor` exists *because* it is the sync/poll compatibility backend for web/jco, so making its funcs async removes the very thing that distinguished it. The dual-world split largely collapses into one async world. Recorded here as a deliberate consequence of the instruction, not an oversight.

### 🛠️ Codemod built and validated: `asyncify-universal.py`
One shared, proven tool rather than each agent inventing its own. Signature-only — **call sites are fixed compiler-driven afterwards, never guessed**, which is the discipline that keeps a 76k-function rewrite reviewable.

It determines "external trait" properly instead of by a hardcoded name list: it first collects every trait **declared in first-party code** (236 found), then treats any `impl X for Y` whose `X` is absent from that set as external and leaves its methods alone. Validated on `⏳️async/🦀️component.rs`: **54 converted, 3 skipped — and the 3 are exactly `Debug::fmt` ×2 and `Future::poll`.** Diffed against the original: only `fn` → `async fn`, indentation/visibility/signature otherwise byte-identical. Idempotent, so it is safe to re-run.

### Sequencing
`io-async-signatures` is still mid-atomic-sweep and is **being allowed to finish** (rule 25). Its work is a strict subset *and* complementary: the codemod rewrites `fn` **definitions**, while that packet is converting `ComposerEntry`'s **fn-pointer types** to future-returning aliases — something no signature codemod can do. Interrupting it to start the bigger job is exactly the failure that cost 84 errors in the db crate.

### 🌊️ Owner directive: TOTAL async conversion of the fleet — applied, and the wall it hits

Directive (2026-08-19, verbatim): *"Every single function must have async keyword and be implemented
with async, doesn't matter if it breaks the code."* Then: *"Now get everything running again end to end."*

**Applied.** `asyncify-fleet.py` (this folder) converted **56,680 `fn` → `async fn` across 9,269 files**
in `✏️s/🔌️plugins`. Census confirms `async_fn` **301 → 56,979**. Two language facts were respected to keep
files parseable (an unparseable file cannot be repaired by any later pass): 19 `const fn` and 2 `extern "abi" fn`
had those qualifiers dropped, since neither can coexist with `async`. Both counted, neither silent.
`🧰️framework/**` was deliberately NOT converted — it was green before this and still is.

**Where it stands now:** framework `semio-framework-plugin --lib` **exit 0**. The fleet does not compile:
one crate alone (`semio-s-plugin-energy`, the smallest) reports ~40k errors, dominated by **E0053 across 84
distinct SDK trait methods** — `handle`, `render`, `initial_snapshot`, `encode_op`/`decode_op`, `diff`,
`inverse`, `absorb`, `mutate`, `sniff`, `compose`, `from_text`/`from_binary`, `parse_dsl`/`print_dsl`, …
Every one is the same shape: the impl is now `async`, the trait declaring it is not.

### ⛔️ The contradiction, stated precisely — it is a language rule, not an effort estimate

`async fn` in a trait is **not `dyn`-compatible**, and this SDK's whole registration model is trait objects
(`.editor::<>()`, `.viewer::<>()`, codec/composer registries, `Arc<dyn …>` consumption). So for those 84
methods, *"every function carries the `async` keyword"* and *"the fleet compiles"* are mutually exclusive.

The only shape that satisfies both intents is the one `db_storage` adopted this same day (`DbFuture<'a,T>`):
the trait method returns `Pin<Box<dyn Future<Output = …> + Send + 'a>>` and the impl body is
`Box::pin(async move { … })`. That is an async **body** with **no `async` keyword on the fn** — it honours
"implemented with async" but not "has the async keyword". There is no third option; this is the same reason
`db_storage`'s module doc says "deliberately NOT `async fn` in trait: AFIT is not dyn-compatible".

Cost of the boxed-future route, measured not guessed: 84 trait methods in the SDK, ~56k impl bodies to rewrap
in the fleet, and the cascade outward through `🖥️host`, `🎠️kernel`, both renderers and `🌎️hub` — i.e. the
whole repo, and it lands on top of a peer's in-flight `io-async-signatures` sweep doing exactly this for the
io subset. Routed to the owner as a scope decision rather than started unilaterally.


# 🌅️ 2026-08-19 — SINGLE-COORDINATOR TAKEOVER, and the program that replaces both wave plans

**Owner decision, this session.** One coordinator from here on. The W5/W6 coordinator and the F-wave
(papert) fleet coordinator are both stood down; their undispatched packets are superseded, their
landed work is absorbed. Plan of record: `📋️master-u.md` (verbatim copy of
`/Users/ueli/.claude/plans/get-s-working-again-quiet-raccoon.md`). Designs of record:
`📓️design-dedyn.md` (compile repair / zero first-party dyn) and §"Design B" of `📋️master-u.md`
(one async world, runtime end to end).

## 📐️ The four owner rulings this program is built on

1. **Drop dyn dispatch.** NOT the boxed-future route. Every first-party dyn-dispatched seam becomes
   enum / static / generated dispatch so plain AFIT (`async fn` in trait) works everywhere, and every
   first-party fn keeps the LITERAL `async` keyword. This settles the contradiction routed upward at
   the end of the previous entry — the answer was neither of the two options that entry named.
2. **Single coordinator** (above).
3. **Legacy compose excluded**: the root `compose/` tree is out of scope entirely. The framework's own
   `semio.compose` cold-job path stays in scope.
4. **External sync deps**: literal reimplementation where no async version exists; async-native
   replacement where one does — always behind a first-party interface.

## 📏️ Ground truth re-measured before planning — the log was stale in three ways

- The **fleet asyncify is COMMITTED** (`09c3cf6df6`, 9,291 files). The **framework asyncify is STAGED
  and uncommitted** — 388 files incl. the WIT. The previous entry's claim that "`🧰️framework/**` was
  deliberately NOT converted — it was green before this and still is" is **false as of now**. It is
  kept, not reverted: under ruling 1 its direction is correct.
- **The wall moved from E0053 to E0038.** The fleet's ~56.7k `async fn` bodies now MATCH the AFIT
  traits; what breaks is that ~88 async methods sit behind trait objects. Counted:
  `PluginApp` 49 methods / 26 dyn uses · `SpaceMember` 25/16 · `GuestRuntime` 9/15 ·
  `HostAsyncRuntime` 3/10 · `Backbone`+`BackbonePort` 5/3 · the db storage family.
  **The fleet contains ZERO first-party `dyn`** — de-dyn is framework-only surgery.
- **The WIT already flipped**: 37 `async func`, 0 plain `func` (staged). Both worlds still exist even
  though the sync/async distinction that justified two of them is gone.

## 🩹️ Mechanical damage inventory (what "get it compiling" actually means)

| damage | count | repair |
|---|---:|---|
| `#[test] async fn` (cannot compile) | **16,427** in 2,897 files | `#[async_test]` proc-macro + rewrite script (`macros-blockon`, dispatched) |
| external-trait impls wrongly asyncified in the fleet (Default 548, serde 600, From 53, fmt 31) | **~1,232** | S1 `deasyncify-external-impls.py` (reverse of the 236-trait census) |
| `const fn` / `extern "abi" fn` qualifiers dropped by the blind fleet codemod | 19 + 2 | S2 `restore-qualifiers.py`, byte-equality-guarded |
| boxed-future prior art DOUBLE-FUTURED (`async fn … -> DbFuture/HostFuture/ComposeFuture/PluginAppMediaFuture`) | ~300 | S4 unwrap; the aliases contradict their own module docs today |
| `ComposerEntry`/`IoEntry` fn-pointer rows whose targets became `async fn` (uncoercible) | 163 | S6 `compose_thunk!` macro-generated E4 thunks |
| missing `.await` after signature flips | tens of thousands | S5 span-keyed fixpoint loop off `--message-format=json` (db-trait-flip precedent) |

## 🧭️ Two exception classes ADDED to the async-literal rule (language-fixed, not judgement calls)

- **E4 — fn-pointer slots.** An `async fn` item's pointer type is unnameable, so any fn whose VALUE is
  stored in a fn-pointer-typed slot (`AsyncComposeFn`, `IoEntry.run/sniff`, `SurfaceDeclaration.factory`,
  `OnceLock<fn()>` installers, `RawWakerVTable`) CANNOT be async. Same class as `extern "C"`.
  E4 fns are macro-generated (invisible in source) or tagged `// 🚫️async: E4`.
- **E5 — executor bridges.** `block_on`, `LocalExecutor` internals, `resolve_ready`. ≤1 per crate, tagged.

Both are ratified in `📌️important.md` alongside R1 (dyn scope), R2 (exception classes) and R3 (the
Send boundary: guest futures are ?Send; host Send-ness is obtained STRUCTURALLY by enums at every
former dyn seam, never by adding `+ Send` bounds).

## ▶️ U1 dispatched — 5 packets, all read-mostly or new-directory, zero contention

| packet | owns | question it answers |
|---|---|---|
| `jco-spike` | `💻️os/🧫️fixtures/🔌️jcoprobe/**` (new) | **The single biggest external risk**: can jco 1.27 transpile and drive a P3 async-export component in a Worker, without JSPI? Verdicts GO-callback / GO-jspi / NO-GO. |
| `async-harness-spike` | `💻️os/🧫️fixtures/🔌️asyncprobe/**` | Re-prove `⏳️runtime.rs`'s claims on the REAL turn shape: cancel-needs-Store-drop, cross-Store preemption, jobs-during-suspended-poll, epoch delta semantics, tokio-Handle injection. Produces the correction list for the rewrite. |
| `brep-probe` | `💻️os/🧫️fixtures/🔌️brepprobe/**` | The never-run probe from `📓️luna-brep-await-spec.md`. Does a guest `.await` of the in-process async BrepKernel get driven inside a turn and inside a job step? Gates the 134-site sweep. |
| `macros-blockon` | `⏳️async/✨️macros/**` (new crate), `⏳️async/🦀️component.rs` block_on region | Unblocks 16,427 test fns. Also lands the E5 `block_on`. **Critical path.** |
| `luna dyn-census` | read-only → `📓️luna-dyn-census.md` | Proves the six-family list COMPLETE over all 236 first-party traits, or finds what it misses. A missed family blows a packet's scope mid-flight. |

## 🗺️ Wave DAG (slugs reserved; audited against this log — no collisions)

```
U1  jco-spike ∥ async-harness-spike ∥ brep-probe ∥ macros-blockon ∥ luna dyn-census
U2  vocab-repair → { io-thunks ∥ store-dedyn ∥ db-dedyn } → sdk-dedyn (ATOMIC)
    → world-collapse (sol, ATOMIC) → host-dedyn → os-ripple(∥) → framework-tests
    → fleet-codemods (offline) → asyncfleet-stdio → asyncfleet-a..f (≤6 ∥) → GATE C
U3  async-plugin-runtime ∥ describe-async → fleet-wasm-descriptors → GATE R
U4  web-bridges → { wgpu-native-async ∥ winit-unblock ∥ wgpu-web-shard ∥ run-through-kernel
    ∥ extension-activation } → exchange-removal → GATE W
U5  http-hyper ∥ pack-waker · adopt-stdio → adopt-a..f → GATE F
U6  parity-rebaseline ∥ bench-web-rows → exit checklist
```

**GATE C** workspace compiles + tests run + dyn census 0 · **GATE R** async runtime executes a real
`🗒️note` turn, 33/33 descriptors · **GATE W** dev boot round-trips a turn, native smoke through the
kernel, extension install→activate · **GATE F** census `block_on` 0 (minus the sanctioned allow-list),
`pending_effects` 0, job kinds registered · **EXIT** 58/58 parity on a RE-BASELINED harness (it
currently compares new-react against old-wgpu — an invalid cross-architecture diff), 8 bench budgets ×
3 renderers, census zero, the full end-to-end `s` scenario web AND native.

## 📌️ Absorbed in-flight state from the two stood-down coordinators

- **`io-async-signatures` never reported**, but its symbols landed and are re-exported
  (`ComposeFuture`/`AsyncComposeFn`/`resolve_ready`/`io_compose_via` at `🚪️io/🦀️component.rs:751,756,766,1077`,
  re-exported at `🧰️framework/📦️packages/🦀️rust/📦️glue.rs:97,103`). **Declared ABSORBED; its path scope is
  released.** Rule 25's hold on the fleet-editing tranche is lifted — the universal-async codemod
  overtook it. Its `ComposerEntry` fn-pointer work is what S6 now completes.
- `sdk-async` delivered; its acceptance block (9 errors, all in `🔌️plugin/🦀️component.rs`) is resolved by
  the same absorption. Its `host_for_instance`/`host()` split still needs reconciling with `cold-kinds`
  at `sdk-dedyn` time — carried forward as a named item.
- `async-runtime` delivered `⏳️runtime.rs` **unmounted, never compiled, against predicted binding names
  that were never created**. `async-harness-spike` produces its correction list; `async-plugin-runtime`
  rewrites and mounts it.
- The papert plan's `fleet-*` tranche was never dispatched (it was holding on the io sweep). Superseded
  by `asyncfleet-*` (compile) + `adopt-*` (census targets).
- `describe-scripts` stays dropped (proven no-op: 33/33 crates already carry the target).
  `dialect-arbitration` verdict (d) stands: nothing was broken.


## 🔴️ SCOPE CORRECTION — the de-dyn surface is **93 trait families / 957 uses**, not 6 families / ~88 methods

`luna dyn-census` came back claiming the design's six-family list was "severely incomplete". A claim
that large reshapes the program, so I did **not** take it on trust — I measured it myself with an
independent script (`sol-dyn-families.json` in this folder holds the full machine-readable table).
**The census was right, and my own numbers are the ones now binding:**

| | |
|---|---:|
| first-party `.rs` files scanned (`🧰️framework`, `✏️s`, `🌎️hub`; ticket archives excluded) | 10,522 |
| distinct first-party traits declared | 234 |
| **first-party traits used as `dyn`** | **97** |
| **total `dyn <first-party trait>` occurrences** | **985** |
| of those, traits whose methods are now `async` ⇒ MUST be de-dyn'd | **93 traits / 957 uses** |
| still fully-sync traits (`Operator`, `HttpBody`, `RouterEffectHandler`) | 3 / 26 uses — they still need asyncifying, so effectively 96 |
| std/lang `dyn` residue (LEGAL under R1) | 133 — `Fn` 56, `FnMut` 23, `Future` 18, `Any` 12, `FnOnce` 12, `Error` 8, `Iterator` 4 |

**Two prior claims are now retired as false.** (a) "The fleet contains ZERO first-party dyn" — it does
not: `Sobject` **131 uses** (semio-s-plugin-animate, the single most-used trait object in the repo),
`BrepKernel` 34 (stdio), `Animation` 22, `Constraint` 20, `Element` 20, `MachineCatalog` 11 are all
fleet traits. (b) The per-family counts the design carried were low across the board — `SpaceMember`
101 (not 16), `HostAsyncRuntime` 47 (not 10) — while `PluginApp` was *over*-stated at 19 (not 26).

### 🟢️ The good news, and it is what makes this tractable
**81 of the 93 have every impl inside a SINGLE crate**, so the erased enum can be generated in place.
Only **12** span crates and need generics or a closing enum in an aggregating crate.

### ⛔️ Why hand-writing was never going to work
Method counts: `BrepKernel` **92**, `PluginApp` **51**, `Sobject` **37**, `SpaceMember` 25. Impl counts:
`Animation` **43 impls**, `LogitsProcessor` **34**. Hand-written match-delegation across 93 traits is
several thousand lines of mechanical code plus permanent drift risk — every trait method added later
silently breaks one enum somewhere.

### ▶️ Decision: ONE mechanism, applied 93 times — packet `dyn-enum-macro` dispatched
A new proc-macro crate `semio-framework-dispatch-macros` at `🧰️framework/🔨️modules/🔀️dispatch/`
(sibling-module pattern, precedents `🧬️schema/✨️derive` and draw's `🔄️fsm/✨️macros`; it is deliberately
NOT put in `⏳️async/✨️macros`, which `macros-blockon` owns — rule 17):

- `#[dyn_enum]` on a trait re-emits it unchanged and captures its signatures in a `#[macro_export]`ed
  hidden `macro_rules!` — this is what makes it work **across crate boundaries**, which matters because
  traits are routinely declared in one crate and closed in another.
- `dyn_enum! { pub enum SpaceMembers: SpaceMember { Text(..), Sketch(..) } }` generates the enum, the
  `From` impls (`From` is external ⇒ E1 ⇒ stays sync) and the match-delegating `impl SpaceMember`, with
  `.await` present exactly on the async methods.
- Required to handle at our scale: mixed async/sync methods, all receiver forms, default bodies,
  generics/where-clauses/lifetimes, and to FAIL LOUDLY (clear compile error) on receiver-less
  associated functions, associated types/consts and supertraits rather than emit broken code.
- **Must add no `Send`/`Sync` bounds** (R3): guest futures are deliberately `?Send`.
- Must generate the uninhabited case (`enum NoMembers {}` with `match *self {}` bodies) — that is the
  default type parameter for every plugin that composes nothing.
- Acceptance includes a **≥40-method** trait test, precisely because `BrepKernel` is 92.

`store-dedyn` is mid-flight and hand-writing a local 25-arm `space_members!` decl-macro. Per rule 25 it
is **not** being interrupted — 25 arms is affordable, and its work is a useful independent check on the
macro's generated shape. Everything after it uses `dyn_enum!`.

## 🧰️ Shared tool landed: `insert-await.py` (coordinator-owned, in this folder)

Every packet needs the same thing — tens of thousands of missing `.await`s — so it is built ONCE here
rather than reinvented per agent (the `asyncify-universal.py` precedent).

It applies **rustc's own suggestions** parsed from `cargo check --message-format=json`, never a
source-text heuristic. **The safety property that distinguishes it from `cargo fix`:** rustc frequently
offers MORE THAN ONE candidate `.await` position for a single error — e.g. in the `compare_exchange`
line of `⏳️async` it offered to await either argument. Applying both is wrong; picking one is a coin
flip. **The tool therefore applies an edit only when a diagnostic yields exactly ONE distinct
candidate**, and writes everything ambiguous to a review list untouched. Also: byte-offset keyed (not
line/column — this repo is full of multi-byte emoji), edits applied per file in descending offset order,
a guard set so no span is ever edited twice (no `.await.await`), overlapping edits deferred to the next
pass, `--scope` to confine edits to a packet's owned paths, and a fixpoint loop with `--max-passes`.

Validated in `--dry-run` against `semio-framework`: **2 errors, 2 unambiguous edits, 0 ambiguous**
(`sol-awaittool-dryrun-framework.txt`).

## 📊️ Coordinator-measured compile baselines (these supersede the stale ones above)

| target | result | note |
|---|---|---|
| `semio-framework-async --lib` | **exit 101, 18 errors** (`sol-baseline-async.txt`) | ALL in one cluster, `⏳️async/🦀️component.rs:127–341` (the `CancelToken` atomics region): E0605 casts, E0308 arg mismatches, E0369 `==` on a future, E0599 on opaque futures — i.e. pure missing-`.await`. **One is different and important**: E0277 at :175, `impl Debug for CancelToken` calls the now-async `state()` inside `write!`. `Debug::fmt` is an **E1** exception — it cannot be async and cannot `.await`. Folded into `macros-blockon` (same file — rule 17) with instructions to make its resolution the **repo-wide recipe** for "external-trait impl needs a value that is now behind `async fn`", because this pattern will recur hundreds of times. |
| `semio-framework --lib` | **2 errors** | Both in `🖱️ui/🎨️styling/📦️packages/🦀️rust/📦️glue.rs`, both plain missing-`.await`. The io crate is in far better shape than assumed — `io-thunks` is a much smaller packet than planned. |

`vocab-repair` is **merged into `macros-blockon`** (one file, rule 17). Its `Arc<dyn HostAsyncRuntime>`
→ generics ripple stays OUT of that packet and belongs to `db-dedyn`/`os-ripple`.


## 🌐️ `jco-spike` VERDICT: **GO-jspi** — the web path works, and it costs us a browser-support constraint

The single biggest external risk in the program is answered, by a real running experiment across four
environments rather than from documentation. Full evidence: `📓️terra-jco-spike-report.md`.

**jco 1.27.0 does transpile and drive a wasip2 `wit_bindgen::generate!({ async: true })` component in
which every WIT function is `async func`.** All four success criteria PASS with measured evidence,
not assertions:

| | result |
|---|---|
| **S1** async export callable, returns a promise of the right value | PASS |
| **S2** guest awaits a real 50 ms host import **without blocking the event loop** | PASS — a concurrent `setInterval` fired **13–17 times** while the guest was suspended |
| **S3** `spawn`-ed detached guest task survives past the export's return | PASS — export returned in ~1 ms, its detached task finished ~80 ms later |
| **S4** `stream<u8>` read chunk-by-chunk from JS | PASS |
| **S5** works with JSPI unavailable | **FAIL — and this is the finding** |

### ⛔️ The plan's central assumption about jco was WRONG, and it was wrong in the expensive direction

The brief (and the design behind it) assumed the P3 **callback ABI** is event-loop-driven and therefore
would not need JSPI, with `--async-mode jspi` being merely a legacy path. **Measured: jco's generated JS
uses `WebAssembly.Suspending`/`WebAssembly.promising` UNCONDITIONALLY.** `--async-mode jspi` produces a
byte-identical file to the default — there is no flag that yields JSPI-free output.

**I verified this myself rather than taking the packet's word for it**, since it determines the product's
browser matrix: both transpiles contain **21 `WebAssembly.Suspending` + 7 `WebAssembly.promising`**
(341,919 vs 341,958 bytes — identical but for a path string). The fallback the brief proposed ("verify
the generated JS contains no JSPI references") is therefore not a fallback at all; it can never pass.

Failure mode without JSPI is **hard and early**: plain Node 24 throws
`TypeError: WebAssembly.Suspending is not a constructor` at module top level, before any call — not a
graceful per-call degradation. So there is no partial-capability story either.

### 📱️ Consequence: the web renderer requires a JSPI-capable browser
Chrome/Edge ship JSPI on by default; bun and Node-with-`--experimental-wasm-jspi` work. Firefox has it
behind a flag; Safari is implementing. **Firefox could NOT be tested** — the Browser pane is
Chromium-only — and the report is explicit about that gap rather than papering over it.

This does not block the program: the plan anticipated GO-jspi and it is Chrome-first by default. But it
is a genuine product constraint, and **fallback F2 (a hand-rolled callback-ABI driver in the bridge
generator) is now the only route to non-JSPI browsers** — it is NOT free, and it should only be built if
the owner requires Firefox/Safari before those ship JSPI. **Flagged to the owner as a scope decision.**

Concrete required changes to `🌐plugin-web-materialize.ts` (`transpilePluginComponent`,
`pluginComponentBridgeSource`) are itemised in the report; `web-bridges` consumes them.


## 🧊️ `brep-probe` VERDICT: **GO-with-constraints** — and it found something that makes the 134-site sweep SAFE

Full evidence: `📓️terra-brep-probe-report.md`. Q1/Q2/Q3 all PASS on a real native probe crate: the
`LocalExecutor` genuinely drives a multi-`Pending` guest await across pumps (Q1); a `JobCtx::tick()`-sliced
job body completes across ≥3 `step_job` calls (Q2); a never-ready guest-internal future does **not** hang
the host — the stall guard reclaims it and every call returns in microseconds (Q3).

### 🎁 The finding that changes the risk profile of the whole fleet sweep
**The real BrepKernel contains ZERO `.await`.** I verified this independently rather than accepting it:
the `✳️brep/🧬️schema` tree is **82 files carrying 1,600 `async fn` and exactly 0 `.await`**. Every kernel
future therefore resolves on its first poll, which means `block_on(kernel.op())` → `kernel.op().await` is
**behaviourally identical today**. The 134-site conversion is a mechanical signature change, not a
semantic risk — which is the opposite of what the plan assumed when it gated the sweep behind this probe.

### ☠️ `pollster::block_on` in the guest is a live landmine, not a style problem
Measured: on `wasm32-wasip2` a genuinely-`Pending` future with no synchronous self-wake **abort-traps the
entire wasm instance** — `condvar wait not supported`, exit 134, in 0.01 s. Not a hang; an instant crash.
It compiles, and it only appears to "work" today because that path is dead code. So removing guest
`block_on` is a **correctness requirement**, not cleanup. This retroactively justifies R4's rule that no
wasm host path is ever a sanctioned `block_on` site.

### 🕳️ A production file that does not compile, found by accident
`⚛️reactor/🧵️executor/🦀️component.rs` — the guest executor itself — is broken by the same mechanical
conversion (2 missing `.await`, plus raw-waker vtable functions turned into `async fn`, which
`core::task::RawWakerVTable` can never accept). A patched copy + exact diff is in the probe fixture.
Assigned to `sdk-dedyn`.

## 🔁 That last defect is SYSTEMIC — I swept for it repo-wide

`async fn` cannot be stored in a `RawWakerVTable` (**E4**: an `async fn` item's pointer type is
unnameable). Four PRODUCTION files carry exactly this break:

| file | async helpers wrongly created | owner |
|---|---|---|
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` | `noop`, `clone_raw` | `io-thunks` (already briefed) |
| `🔌️plugin/⚛️reactor/📮️requests/🦀️component.rs` | `futures_test_waker`, `noop`, `clone` | `sdk-dedyn` |
| `🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs` | `wake`, `waker_for`, `raw_waker`, `waker_clone`, `waker_wake`, `waker_wake_by_ref`, `waker_drop` | `sdk-dedyn` |
| `🔌️plugin/🌐host/📖️body/🦀️component.rs` | `noop`, `clone` | `sdk-dedyn` |

All four take the same fix: revert those helpers to sync (**E4**, tagged `// 🚫️async: E4 fn-pointer slot`)
or replace the hand-rolled vtable with `std::task::Waker::noop()`. **This is now a named checklist item on
`sdk-dedyn`, not a discovery to be made mid-packet.**

## 📉 CENSUS CORRECTION: `block_on` is **766** repo-wide, not 134

The long-quoted "block_on 134 → 0" target counted **the fleet only**. Measured across all first-party code:

| area | `block_on(` sites |
|---|---:|
| `🧰️framework/🛍️products/💻️os` | **619** |
| `✏️s/🔌️plugins/🌊️flow` | 59 |
| `✏️s/🔌️plugins/📐️cad` | 45 |
| `✏️s/🔌️plugins/🗄️stdio` | 15 |
| `✏️s/🔌️plugins/🏭️process` | 13 |
| `🧰️framework/🔨️modules/🎒️pack` | 12 |
| `✏️s/🔌️plugins/🎞️animate` | 2 (and these two are **not** BrepKernel at all — they are wgpu `request_adapter`/`request_device`, a different risk class) |
| `🧰️framework/🔨️modules/◻2d` | 1 |
| **total** | **766** |

The 619 in `💻️os` include the db `postgres`/`neo4j` dedicated-thread bridges, which **R4 explicitly
sanctions** — so the exit target is not "766 → 0" but "766 → the R4 allow-list, enumerated by name".
`census-zero` must therefore report *classified* counts, never a bare total; a bare total here would be
either a false alarm or a false all-clear. The fleet's own 134 remain a true 134 → 0.


## ⚙️ `async-harness-spike` VERDICT: **GO on all six questions** — the native async runtime design is validated

Full evidence + code shapes: `📓️terra-async-harness-report.md`. Built a reduced turn-shaped world
(`semio:turnharness@0.1.0`, records matching the real `reactor`/`jobs`/`checkpoint` field-for-field) plus
a wasmtime 47.0.3 host harness under `🧫️fixtures/🔌️asyncprobe/{👽️guest-turn,🖥️host-turn}`.

| Q | question | verdict |
|---|---|---|
| Q1 | turn shape: async `poll` on a Store owned by a spawned task, guest awaits a host import mid-turn | **GO** |
| Q2 | cancellation requires dropping the **Store**, not just the call future | **GO** — re-confirmed on the poll shape |
| Q3 | epoch **and** fuel preempt CPU-bound guests across separate Stores | **GO** |
| Q4 | `step-job` against an instance whose `poll` is suspended | **GO — and it needs no dedicated instance**, the existing `accessor.spawn` pattern works |
| Q5 | `set_epoch_deadline` delta semantics | **GO** — reproduced the overflow trap decisively, plus both epoch and fuel cutoffs with exact trap text |
| Q6 | `tokio::runtime::Handle` injection + `abort()` tears down the Store | **GO** |

**Q4 is the valuable one**: jobs do NOT need their own instance, which removes a whole branch from the
`async-plugin-runtime` design. The packet also found and fixed two bugs in its own harness rather than
hiding them (fuel armed before `instantiate_async` gets eaten by instantiation; an unconditional-Yield
epoch callback masks the delta-overflow signal) — both are traps the real runtime would have hit.

⚠️ **It correctly flagged that the target world I described does not exist verbatim yet**: today `world
actor` imports only `pure`, while `world actor-async` imports `host-async` but exports stream-based
`runner::run` rather than `poll`. That is exactly what the `world-collapse` packet is for; the harness
proves the *destination* is sound. Its "corrections required to `⏳️runtime.rs`" list is now the spec for
`async-plugin-runtime`: drop `GrantWindow`/`StreamProducer` for a command channel, interface names carry
**no `-async` suffix**, tokio-handle injection is mechanical, and `DeadlineCell`/`install_epoch_budget`
are confirmed correct as written.

## 🧱️ THE REAL COMPILE SPINE — measured, and it is much narrower than feared

`cargo check -p semio-framework-plugin --lib` does not even reach the SDK: it dies upstream. I ran it
with `--keep-going` to enumerate the whole closure (`sol-spine-keepgoing.txt`). **In the guest SDK's
entire dependency closure, exactly FOUR crates fail:**

| crate | module | errors |
|---|---|---:|
| `semio-framework-schema-derive` | `🧬️schema/✨️derive` | **1** |
| `semio-framework-os-kernel-dsl-derive` | `🗣️dsl/✨️derive` | **8** |
| `semio-framework-mesh-engine` | `🔺️mesh-engine` | 29 |
| `semio-framework-replication` | `📡️replication` | **~481** |

(plus `⏳️async`'s 6, already owned by `macros-blockon`.)

**Two of the four are proc-macro crates, and that is the whole bottleneck.** A `#[proc_macro]` /
`#[proc_macro_derive]` / `#[proc_macro_attribute]` entry point must be exactly
`fn(TokenStream) -> TokenStream` — **E3** — and the codemod made them `async`. Nine errors in two tiny
crates are blocking every crate in the repo that uses a derive. Dispatched as packet **`spine-upstream`**,
with the standing ruling that **proc-macro crates stay entirely sync** (a proc-macro runs inside rustc at
compile time, where async is meaningless) — that reasoning generalises to every `✨️derive`/`✨️macros`
crate in the repo.

Closure-wide error profile: **E0277 219 · E0308 171 · E0053 34 · E0599 32 · E0369 8 · E0605 6 · E0600 3 ·
E0271 2 · E0608 1** — overwhelmingly missing-`.await`, which is exactly what the shared tool automates.
The 34 E0053 are the judgement cases (trait and impl disagree); the standing rule given to the packet is:
**never resolve an E0053 by making a first-party trait sync to match a stale impl.**

## 🧹 Second shared tool landed and VALIDATED: `deasyncify-external-impls.py` (repair codemod S1)

The blind fleet codemod wrote `async fn` into impls of traits this repo does not own; those can never
compile (**E1**). The reverse tool reuses `asyncify-universal.py`'s own local-trait census — so "external"
means exactly what it meant when the damage was done — and adds three safeguards the original lacked:
a `FORCE_EXTERNAL` set that overrides the census (so a first-party trait coincidentally named `Default`
cannot shield uncompilable code), **real brace-depth tracking** instead of the original's
`line.startswith('}')` pop heuristic (a false negative is harmless when *adding* a keyword but corrupts
meaning when *removing* one), and string/comment stripping before brace counting.

Scanned over `✏️s`: **890 methods in 626 files** — `Default` 571, serde `Deserializer` 59 / `Serializer`
59 / `Serialize` 7 / `Deserialize` 7 / `Visitor` 13, `From` 53, `Display` 43, `Sub` 10, `Add` 9,
`PartialEq` 7, plus a long tail of operator traits.

**I checked its five most suspicious trait names before trusting it** — `GltfSemanticMutation`, `World`,
`Parse`, `ApplicationHandler`, `Visitor` all *look* like they could be first-party. Measured: **none of
them is declared anywhere in first-party code** (0 hits each), so they are genuinely external
(winit/syn/gltf/serde) and reverting them is correct. The tool is cleared for use; the fleet-wide `--apply`
belongs to `fleet-codemods`, not to any crate packet.


## ✅️ `macros-blockon` ACCEPTED — the first crate in the program is GREEN

Coordinator-run acceptance (not the packet's own figures — rule 23):

```
cargo check -p semio-framework-async --lib          LIB_EXIT=0
cargo check -p semio-framework-async --all-targets  ALLT_EXIT=0
cargo test  -p semio-framework-async                TEST_EXIT=0   17 passed / 0 failed / 0 ignored
cargo check -p semio-framework-async-macros --all-targets  MACRO_EXIT=0
```
(First capture attempt lost the exit codes to a `${PIPESTATUS}`/brace-group interaction and was re-run
clean rather than reported from the "Finished" lines — rule 7 demands the real code, including from me.)

Landed: the proc-macro crate `semio-framework-async-macros` (`⏳️async/✨️macros/`) with `#[async_test]`,
which **keeps the literal `async fn` in source** and expands to a `#[test] fn` driving the body through an
inline, dependency-free thread-park executor — so 65+ crates gain a dev-dependency and nothing else, no
tokio, no futures-lite. Plus `block_on` in `⏳️async/🦀️component.rs` as a tagged **E5** bridge (native
thread-park, wasm32 spin fallback), the R1 de-double-futuring of `HostAsyncRuntime`, and the 18 baseline
errors + 1 more that only `--all-targets` revealed (rule 26 earning its place again).

**Nice confirmation of the E3 ruling from the field**: the packet found empirically that a
`#[proc_macro_attribute]` entry cannot be `async`, and that **the two precedent macro crates in the repo
do not currently compile for exactly that reason** — which is independent corroboration of the
`spine-upstream` diagnosis that the two `✨️derive` crates are broken proc-macro entry points.

### 📉 Correction to my own figure
`async-test-attr.py --scan` (the real tool, over the real tree) reports **13,294 sites in 2,718 files**,
not the 16,427/2,897 I recorded earlier from a coarser grep. **The tool's number is the binding one**; my
earlier estimate over-counted. Same class of error this ticket has logged repeatedly — a grep is not a
census.

### ⚠️ Anomaly recorded, not swallowed
Mid-session the packet found `⏳️async/🦀️component.rs` reverted to its pristine pre-session state, with git
evidence, and judged it tooling/sync trouble rather than a peer edit. It redid the work and flagged it
instead of quietly absorbing it — correct behaviour. **I verified the end state on disk myself**: `block_on`
present, E5 tag present, **zero** `async fn … -> HostFuture` double-futures remaining, 19 `async_test` uses,
and all five macro-crate files present. The work is real and survived.

### 🔀 Follow-up handed on (not dropped)
Three crates still implement `HostAsyncRuntime` with the pre-R1 signatures: `🛢️db/🗄️storage` (assigned to
`db-dedyn`, briefed), `🛎️services` and `🌎️hub/…/📦️bin.rs` (assigned to `os-ripple`, recorded here so it
cannot be forgotten).

## 📏️ NEW RULING **R7** — `async_fn_in_trait` is ALLOWED crate-wide; never "fix" it with `+ Send`

All 6 warnings on the first green crate were the same lint:
`use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified`.
Under universal async it fires on **every public trait with an async method** — ~93 families, so
potentially hundreds of warnings against an exit bar that demands zero. One central ruling now, rather
than 93 packets each improvising:

- ✅ `#![allow(async_fn_in_trait)]` at crate root with a comment citing R3/R7.
- ⛔ **Never** silence it with rustc's own suggested `-> impl Future<Output = T> + Send`. The compiler
  prints that suggestion in the warning text and it is the WRONG fix here: it re-imposes `Send` on guest
  traits whose futures cannot be `Send` (single-threaded wasm, `LocalExecutor`, thread_local state) and
  contradicts R3 in the letter. **This is a case where following the compiler's advice breaks the
  architecture** — the lint's concern is answered structurally by concrete enums at every former dyn seam,
  which is exactly what O1 is building.
- ⛔ Never resolve it by making a trait method sync.
- Every other warning class still counts toward the zero-warning exit bar.

Broadcast immediately to the four in-flight packets most likely to hit it (`dyn-enum-macro` above all —
a macro that emitted `+ Send` would inject the defect 93 times), rather than left in a report for them to
find. Rule W4-8: a cross-packet finding must be lifted the moment it is read.

## ▶️ `db-dedyn` dispatched
`semio-framework-async` going green unblocked it. Owns `🛢️db/**`: un-double-future the 7 storage traits
(~233 `DbFuture` lines), `DbBackend<R>` + per-facet ref enums replacing `-> &dyn WalStorage`,
`Arc<dyn HostAsyncRuntime>` → generic `Arc<R>`, the R1 fix to its own `InlineRuntime`, and the R4
classification of its `block_on` sites (clause 2 sanctions the postgres/neo4j dedicated-thread bridges).
Briefed to USE `dyn_enum!` if `dyn-enum-macro` has landed by the time it gets there, and to hand-write in
the same shape if not — never to block on it.


## 🎒️ `pack-waker` delivered — accepted on substance, acceptance BUILD deferred (blocked upstream, verified not its own)

Both headline correctness defects are fixed in `semio-framework-pack`:
- `CancelWatch::poll` no longer busy-waits with a 200 µs sleep inside `Future::poll`; it uses a
  `CancellationToken` over `Arc<Mutex<{cancelled, wakers}>>` with an atomic check-and-register.
- `http`'s `Sleep::poll` no longer sleeps inside `poll`; it spawns one timer thread on first poll that
  calls `Waker::wake` at the deadline.
- A poll-counting regression test for each asserts **≤4 polls** where the old busy-poll shape logged
  ~75–100. That is the right kind of test: it fails if anyone reintroduces the defect, rather than merely
  asserting the future eventually completes.

**Its compile acceptance could not run**, and the packet said so plainly instead of claiming green: the
crate's dependency `semio-framework-replication` was failing with **209 errors, then 350 errors twenty
minutes later** — a moving count, which here is not flakiness but the live `spine-upstream` packet
refactoring that very crate. Correct diagnosis, correct restraint. Re-verification is queued behind
`spine-upstream`.

### 🕵️ Its provenance inference was WRONG, and checking cost one command
The packet observed its file change between two reads and concluded that "another session's repair
codemod (`deasyncify-external-impls.py`) plus a broader companion pass had already reverted most of the
mechanical async damage." That script is **mine**, and it was dispatched to exactly two packets under
tight `--scan`-then-`--apply` scoping. If it had genuinely been run repo-wide, 890 fleet sites would have
been rewritten without review — a serious scope violation.

**Measured instead of assumed: the fleet is INTACT.** Re-running the scan over `✏️s` still reports
**892 pending sites in 627 files** (vs 890/626 earlier — the drift is ordinary churn, and `local traits
known` moved 233 → 239 because sibling packets are adding traits). Nothing was swept. This is the second
packet this session to report a file mutating under it; the pattern is recorded, but it is **not** a rogue
codemod. Standing reminder that just earned its keep: *a file changing under you is evidence of churn,
never evidence of what changed it* — settle attribution by measuring, not by inferring.

### ✅️ Its cross-packet finding was RIGHT, and is now ruling **R8**
`#[async_trait]` desugars to exactly the `Pin<Box<dyn Future>>` trait-method return shape that **R1 bans
and O1 rejects**. The packet flagged it rather than acting outside scope — correct. I measured the whole
surface so it can be closed rather than discovered piecemeal: **12 attribute sites in 6 files**, 5
`Cargo.toml` declarations — `🎒️pack/🌐️http` 5, `🎒️pack/⏳️async` 3, `🌎️hub/📇️directory` 4. Small, bounded,
and now assigned by name in `📌️important.md`.

### ⚖️ Ruling clarification it correctly asked for → **R4 clause 5**
All 14 of its `block_on` sites live in `#[cfg(test)]`, and it noted R4 never literally names `#[test] fn`.
Fair question, ruled: **a test harness is a `main`-equivalent thread root, so `block_on` in `#[cfg(test)]`
is sanctioned and is not counted against the census target.** Preferred form remains `#[async_test]`.
Consequence for `census-zero`: it must report **production** and **test** `block_on` as separate numbers —
a blended total would be simultaneously a false alarm and a false all-clear.

It also found the crate had **zero** `#[test] async fn` breakage, contrary to the brief's expectation —
noted, because it means the 13,294 sites are unevenly distributed and per-crate expectations should be
measured, not assumed.


# 🚨️ INCIDENT — the staged framework asyncify was REVERTED out of the working tree (index intact, fully recoverable)

Three packets in a row reported "my file changed under me" (`macros-blockon`: reverted to pristine;
`pack-waker`: damage already undone between two reads; `store-dedyn`: **overwritten twice**, its
`async fn` count collapsing 739 → 2). Two of the three misattributed it — one guessed a peer session, one
guessed a rogue codemod. **Neither guess was right, and guessing is what this ticket keeps paying for.**
I measured it instead.

## What is actually true

| file | HEAD | git INDEX | WORKING TREE |
|---|---:|---:|---:|
| `🔌️plugin/🦀️component.rs` (guest SDK) | 19 | **1,489** | **19** |
| `🔌️plugin/🖥️host/🦀️component.rs` | 1 | **238** | **1** |
| `🏪️store/🦀️component.rs` | 0 | **739** | 208 (store-dedyn rebuilt part on the reverted base) |
| `🎭️actor/🦀️component.rs` | — | — | 224 ✅ untouched |
| `🚪️io/🦀️component.rs` | — | — | 192 ✅ untouched (live packet) |

(counts = `async fn`.) **The working tree was restored from HEAD; the index still holds the conversion.**
388 files remain staged. Framework-wide the tree is now only **34.2% async** (6,785 async vs 13,044 plain).

**Mechanism, from mtimes**: `🔌️plugin/🦀️component.rs` and `🖥️host/🦀️component.rs` were both rewritten at
**14:41:32 — the same second**. That is a bulk operation, not editing. It happened ~30 min into this
session. `.git/index` was touched later (15:03:47), consistent with the auto-commit bot continuing to
stage. Cause is NOT established and I am not going to invent one; what is established is the shape:
*something restored working-tree files from HEAD while leaving the index alone.* Flagged to the owner —
it is their environment and the cadence may be recognisable to them.

## Why I did NOT "just restore it from the index"

`git restore --source=:0 <paths>` would recover the asyncify **and destroy `store-dedyn`'s entire de-dyn
refactor**, because the index predates it. It is also a git-modifying command, which rule 1 forbids
outright precisely because several sessions share this tree. Recovering one packet's work by silently
deleting another's is not a recovery.

**The correct route is the idempotent codemod**, which composes with the current tree instead of
replacing it: it re-adds `async` while leaving `store-dedyn`'s enums, `MemberFactory` and `space_members!`
in place. Verified intact after the revert: **zero live first-party `dyn` in `🏪️store`** (11 residual hits
are all comments), `MemberFactory` ×14, `space_members!` ×6, `Backbones` enum, `NoMembers` ×8.

## 🔒️ The codemod is now SAFE TO RE-RUN — tag awareness added (this was the real gap)

`asyncify-universal.py` skipped external-trait impls, `const fn`, `extern` and `main`. It knew **nothing**
about the E3/E4/E5 exception classes this program has since established, so re-running it over a repaired
tree would have silently re-broken **every** raw-waker vtable helper, fn-pointer thunk and `block_on` that
packets have just fixed — converting a recovery into a regression, at scale.

Patched: it now skips any fn carrying a `// 🚫️async: E<n>` tag in the attribute/comment block directly
above its signature, and any `#[proc_macro*]` entry point, and reports them as `tagged_exempt`.
**This makes the tag convention load-bearing rather than decorative** — a tag is now the only thing
standing between a hand-made repair and the next codemod run.

Verified on two corpora:
- `⏳️async` (repaired + accepted): `tagged_exempt: 2`, `external_trait: 7`, `already: 58` — the E5
  `block_on` is correctly protected.
- The reverted SDK file: `converted: 1,470`, `external_trait: 41`, `const: 3`, `extern: 6` — which
  reconciles with the index's 1,489, confirming a re-run reproduces the lost conversion.

## ⛔️ Deliberately NOT blanket re-applying now
The `⏳️async` scan shows **11** would-be conversions in a crate that is already green and accepted —
i.e. a blanket `--apply` would regress accepted work. Re-asyncify is therefore folded into the packet that
**owns** each file, as its first step, rather than run centrally across files five packets are live in:
`sdk-dedyn` re-asyncifies `🔌️plugin/**` before its de-dyn work; `host-dedyn` does `🖥️host/**`;
`store-dedyn`'s remainder is picked up at re-acceptance.

## 📋️ Standing consequences
1. **Verify the end state on disk, never from your own earlier read** — three packets were fooled today.
2. **`git diff HEAD` is not the whole story here.** Index and working tree have diverged; compare
   `git show :<path>` (index) against the file when a count looks wrong.
3. A file changing under you is evidence of churn, never evidence of *what* changed it. Settle
   attribution by measuring index/HEAD/worktree, not by naming a suspect.

## `dyn-enum-macro` delivered — `#[dyn_enum]` / `dyn_enum_close!`, full findings in `📓️terra-dyn-enum-macro-report.md`

`semio-framework-dispatch-macros` (`🧰️framework/🔨️modules/🔀️dispatch/**`) is written, 28/28 tests green,
zero clippy/rustfmt warnings — proven via a scratchpad standalone build (root `Cargo.toml` lease pending,
see the report's `lease-request`). Five findings worth knowing before the next 90 applications:

1. **`use crate::__semio_dispatch_X;` (an absolute path) to a macro-expanded `#[macro_export]` macro,
   from the SAME crate, hits `error: macro-expanded 'macro_export' macros … cannot be referred to by
   absolute paths` (rust-lang/rust#52234) — downgraded to `warn` here by `future_incompatible = "warn"`,
   but this ticket's gates run `-D warnings`.** Fix: `dyn_enum_close!` emits a BARE (unqualified)
   invocation, never a `use` — works with zero warnings whenever the trait is declared before its closing
   enum in the same module (true everywhere in this program). Cross-module/cross-crate call sites must
   write their own `use …__semio_dispatch_<Trait>;` — the macro cannot inject that without retriggering
   the lint.
2. An attribute macro and a function-like macro **cannot share one name** in one crate (`E0428`) — the
   closing macro is `dyn_enum_close!`, not `dyn_enum!`.
3. **`sol-dyn-families.json`'s census looks stale** for at least `AuditSink` and `Decider` — both are
   fully SYNC in the live tree today (no `async fn` anywhere), contradicting the census's
   `async_methods: 1`/`3`. Recommend re-measuring before the next wave trusts it. This is also why
   neither became the required worked-application family — a sync `dyn Trait` isn't actually broken by
   E0038 yet.
4. `Migration` (`🔄️machine/🦀️component.rs:1303`) IS genuinely async but its only 2 uses are
   `&[&dyn Migration]` (an open, caller-supplied list per `Machine::restore`/`step`) — a materially
   harder shape than a `Box`/`Arc<dyn T>` field; `dyn_enum` doesn't apply to it mechanically.
5. **The sibling proc-macro crates this task was told to copy the shape of are currently BROKEN** by the
   blind asyncify tooling: `semio-framework-schema-derive` and `draw-fsm-macros` both have
   `#[proc_macro_derive]`/`#[proc_macro]`/`#[proc_macro_attribute]` entry functions marked `async fn`,
   which is a hard rustc error (`expected fn(TokenStream) -> TokenStream, found fn(..) -> impl Future`).
   Proc-macro entry points are E3 — plain `fn`, always. Not fixed (out of path scope); whoever owns those
   two files next has a one-line-per-entry mechanical fix.

No family was converted live this turn (`GuestRuntime`'s file was contended per this session's own
`cold-kinds`/`macros-blockon` activity; `AuditSink`/`Decider` aren't actually broken yet per finding 3;
`Migration`'s call-site shape needs real design). The exact `AuditSink` before/after diff is ready in the
report, proven end-to-end against the real macro via a standalone scratch crate, pending the two
Cargo.toml leases (root workspace member + `semio-framework-os-mcp`'s own dependency line) named there.


## ✅️ `dyn-enum-macro` ACCEPTED — the force multiplier for 50 remaining families is live

Coordinator-run acceptance after I applied its lease to the root `Cargo.toml`:
```
cargo check -p semio-framework-dispatch-macros --all-targets   EXIT=0
cargo test  -p semio-framework-dispatch-macros                 EXIT=0   28 passed / 0 failed (5 targets)
cargo check -p semio-framework-async-macros    --all-targets   EXIT=0   (unaffected by the change)
cargo metadata --no-deps                                       EXIT=0   (workspace still parses)
```
Registrar action: added **both** new proc-macro crates as workspace members
(`🔀️dispatch/📦️packages/🦀️rust` and `⏳️async/✨️macros/📦️packages/🦀️rust`) — the latter had been working
only as a transitive path dev-dependency, which is fragile and would have surprised the first packet to
run `cargo check -p` on it.

Coverage that matters: a **45-method** trait test (so `BrepKernel`'s 92 is not a leap of faith), the
zero-variant `match *self {}` case for all four receiver kinds, and mixed async/sync + `&self`/`&mut self`
+ default bodies + generics/where-clauses verified at runtime, not just compiled.

Three real constraints it discovered, now part of the recipe: the closing macro must be invoked **bare**
(macro-expanded `#[macro_export]` macros cannot be referenced by absolute path from the same crate,
rustc#52234); it had to be named **`dyn_enum_close!`** because a function-like and an attribute macro
cannot share a name (E0428); and cross-module use needs an explicit `use crate::__semio_dispatch_<Trait>;`.

It also **independently confirmed the `spine-upstream` diagnosis**: the two sibling `✨️` macro crates it
was told to copy are themselves broken by the blind codemod marking proc-macro entries `async fn`.

## 📊️ Post-revert de-dyn census — the scope moved, and the old table is now WRONG

The `dyn-enum-macro` packet flagged that `sol-dyn-families.json` was stale for two families. It was right,
and the cause is the revert: that census was taken at ~14:30, the bulk revert landed at 14:41, and a trait
whose methods reverted to sync **no longer needs de-dyn at all**. Re-measured into
`sol-dyn-families-postrevert.json`:

| | traits | dyn uses |
|---|---:|---:|
| used as `dyn` (first-party) | 92 | 783 |
| **still need de-dyn (async methods today)** | **50** | **571** |
| currently fully sync — de-dyn deferred until re-asyncified | 41 | 210 |

`SpaceMember` fell **101 → 17**, which is `store-dedyn`'s work showing up in the census — an independent
confirmation that its refactor survived the revert. `PluginApp` now reads 51 methods / **2 async**, which
is the reverted SDK exactly as expected.

**Binding consequence: de-dyn scope must be re-measured AFTER each file is re-asyncified, never before.**
Scoping a packet off the pre-revert table would send it hunting for trait objects that are not currently
broken, and miss ones that will be. `sol-dyn-families-postrevert.json` supersedes the original.

## 🐛 `io-thunks` found a REAL BUG IN MY OWN TOOL — and caught it itself

`insert-await.py --apply --scope '🧰️framework'` reached into **314 files**, far outside the packet's
grant. The packet noticed, unwound it correctly (restoring 203 out-of-scope files via `git show` + Write,
**never** `git checkout` — exactly right under rule 1), and reported it plainly instead of quietly keeping
the wider diff.

**The defect was mine.** `--scope` was a bare substring test, so `🧰️framework` matched every file in the
framework tree — including `🛍️products/**`, which belongs to other packets. **I had told four packets that
passing `--scope` would confine their edits.** A scope argument that silently means "almost the whole repo"
is worse than none, because it is trusted.

Fixed, with the incident written into the function's own docstring so the reason cannot be lost:
- `--scope` now matches on **path segments**, not substrings (`in_scope()`), verified against six cases
  including the exact failure (`🧰️framework/🔨️modules` must NOT match a `🛍️products/**` file) — **all 6 pass**.
- New **`--max-files` blast-radius guard** (default 60): a pass that would edit more files than the cap
  aborts and prints what it would have touched, because an over-broad scope looks exactly like this in
  practice. The 314-file pass would have been stopped dead.

Standing lesson: **a safety rail nobody has tested is not a safety rail.** I shipped `--scope` on
reasoning and it was wrong on first contact; it now has test cases.

## 📦️ `io-thunks` delivered
`compose_thunk!` / `io_run_thunk!` / `io_sniff_thunk!` (E4 fn-pointer-slot thunks), `resolve_ready`'s
broken `RawWakerVTable` internals replaced with `std::task::Waker::noop()` and demoted to a tagged **E5**
plain fn, plus ~40 cascading `.await`/sync-closure repairs. It fixed `SubsetValidatorEntry.validate` —
**the same defect class, not named in its brief, found by reading rather than by waiting to be told**.
10 tagged sites verified by grep.

It **declined to run `compose-thunk-rewrite.py --apply`** despite the literal instruction, having verified
concretely that it would double-wrap already-correct thunks. Correct call: an instruction that would
corrupt the tree is not followed, it is reported. The script is written, bug-fixed and idempotency-checked
on a fixture, ready for `fleet-codemods` to point at `✏️s/**`.

Acceptance blocked upstream by `📡️replication` (77 errors and shrinking as `spine-upstream` works it),
with **zero errors attributed to its own file at every measurement** — stated plainly rather than glossed.
One `lease-request` for a guest-SDK call site (`🔌️plugin/🦀️component.rs:3739`), folded into `sdk-dedyn`.


## ✅️ `spine-upstream` ACCEPTED — and the upstream blocker is now fully CLEARED

The packet took 3 of its 4 crates to green and, more usefully, **isolated the fourth precisely instead of
grinding on it**: `semio-framework-replication` had **zero errors of its own**; it was blocked entirely by
two files its crate root `#[path]`-mounts but does not own — `⚠️diagnostic/**` (11 errors) and
`🌱️value/**` (55, a hand-rolled `Serializer`/`Deserializer`). It filed a lease rather than reaching
across the boundary. That is exactly the behaviour rule 3 exists to produce.

- `semio-framework-schema-derive`, `semio-framework-os-kernel-dsl-derive` — **green**, whole crate kept
  sync, tagged `// 🚫️async: E3 proc-macro entry`. Its written reasoning ("a proc-macro runs inside rustc
  at compile time, where async is meaningless") is now the standing recipe for every `✨️derive`/`✨️macros`
  crate in the repo.
- `semio-framework-mesh-engine` — **green**, `cargo test` **20/20 passed** (runtime, not just compile).

### 🕳️ A real pre-existing bug it surfaced by accident
`⚙️codec/🦀️component.rs` and `🚰️source/🦀️component.rs` had **lost their `#[cfg(test)] mod tests {}`
wrapper entirely** — braces still balanced, so nothing ever caught it, and their test code was compiling
into the plain `--lib` build. Restored. Found only because the `#[async_test]` rewrite walked those files;
a whole class of "tests silently in the production build" that no gate was checking.

## 🔧️ I took the lease myself and cleared it — replication is GREEN

`⚠️diagnostic` and `🌱️value` are unowned, and this was the critical path, so I did it as registrar rather
than spending a dispatch round.

1. `deasyncify-external-impls.py --scan` on `🌱️value` → **34 sites** (`Serializer` 28, `Deserializer` 6);
   `⚠️diagnostic` → **0**. Applied to `🌱️value`. **66 → 36 errors.**
2. The residue was the interesting half: **5 × E0728 `await is only allowed inside async functions`** —
   removing `async` from a serde impl leaves its `.await`s orphaned. The failing calls were
   `self.get(key).await`, `key.as_str().await`, `self.take().await`, and `TextError::expected(..).await`
   — i.e. **pure in-memory accessors on a data enum, made async for nothing, consumed by impls that can
   never be async.**
3. Verified both modules contain **zero I/O markers** (`std::fs`/`tokio`/`reqwest`/`ureq`/`File::`/
   `TcpStream`/`spawn`/`sleep`/`SystemTime`) before deciding, then de-asyncified them wholesale and
   removed the orphaned `.await`s in the same edit: `🌱️value` 11+8 fns, `⚠️diagnostic` 39+2 fns, 25
   `.await`s.

**Result: `semio-framework-replication` `--lib` EXIT=0 and `--all-targets` EXIT=0, zero errors.**

This is now ruling **R9** in `📌️important.md` — *E1 is transitive*: a pure computation whose consumers are
language-barred from being async stays sync, tagged, with **both halves of the test shown in the report**
(no suspension point exists AND a consumer cannot be async). Explicitly guarded against misuse: R9 is a
fallback, never a shortcut for avoiding await-insertion work — if the consumer *can* be async, make the
consumer async.

## 🎯 The critical-path number, at last — and the next blocker

`cargo check -p semio-framework-plugin --lib` now gets **past** replication and stops at
**`semio-framework-pack`: exit 101, 68 errors** (E0277 35 · E0308 15 · E0369 9 · E0728 3).

I ran the shared await fixpoint over it first (`--scope '🎒️pack'`, now segment-matched): **3 edits
applied, 0 ambiguous, fixpoint reached.** So the mechanical part is done and the remaining 65 need
judgement — which is precisely when a packet, not a tool, is the right instrument. Two root causes, both
repo-wide patterns:

1. **Pure computation made async, called from sync code** — a CRC helper in `🎒️pack/📐️format` is now
   `async` while `fn encode_segment(..)` calls `crc.await`. **93 diagnostics in one file.** Textbook R9,
   and `pack-finish` is briefed to work it as a *pattern* and write the recipe, since this is the second
   worked instance today.
2. **`#[async_trait]`** in `🎒️pack/🔌️io` — `error: method should be async or return a future, but it is
   synchronous` on `len`/`write_all`/`position`. That is ruling **R8** landing in practice; the 8 `🎒️pack`
   sites are `pack-finish`'s, the 4 `🌎️hub/📇️directory` sites stay with `os-ripple`.

Dispatched **`pack-finish`** with both, plus an instruction to explain a measurement oddity rather than
ignore it: pack's build cites `📡️replication` files that were green minutes earlier in isolation —
most likely **feature unification** (pack enabling a different feature set), which this ticket has been
bitten by before ("acceptance must run the command the CONSUMER runs, feature flags included").


## 📦️ `db-dedyn` DELIVERED — held UNACCEPTED until I compile it myself

The whole conversion is done: `DbFuture` alias deleted and every storage signature converted to direct
`async fn -> Result<T, DbError>` via a purpose-built brace-matching script (zero `Box::pin` left in real
code); `DbBackend<R>` plus six facet-ref enums (`WalRef`/`SnapshotRef`/`PayloadRef`/`CatalogRef`/
`IndexRef`/`LeaseRef`) replacing `-> &dyn WalStorage`; the `DbStorage` trait deleted; the ripple pushed
through every consumer (db_index's 13 structs, db_snapshot, db_wal, db_cluster, db_sync, db_compact,
db_projection, db_query, `ArtifactEngine<R>`, `Database`, db_cli, `FaultStorage<R>`); `FsStorage`/
`SqliteStorage` generic over `R: HostAsyncRuntime`; `InlineRuntime` rewritten against the R1-corrected
trait — **read from the source rather than from my brief's paraphrase**, which is the right instinct.

**It could not run acceptance**, and said so instead of claiming green: every attempt was blocked one
crate short by sibling work outside its scope — first `🌱️value` (which I cleared mid-session), then
`🎒️pack` (still red, `pack-finish` is on it). Correct restraint.

### ⚠️ Why this stays UNACCEPTED rather than "accepted pending build"
**A whole-crate refactor verified only by source review is exactly the shape that left this very crate
with 84 errors once before** (ruling R6 exists because of it). `db-trait-flip` also "looked right".
I will run `--lib`, `--all-targets` and the **424-passed** test baseline myself the moment `🎒️pack` is
green, and only then accept. Queued, not forgotten.

### 🔁 It hand-wrote the enums, and that is a useful accident
`dyn_enum_close!` did not exist yet when it got there (the macro crate was still an empty skeleton at that
moment; it went green ~20 minutes later). So `db-dedyn`'s six facet-ref enums are an **independent
implementation of the same shape** the macro generates — a free cross-check on the macro's design. Worth
diffing the two shapes at acceptance; if they disagree, one of them is wrong.

### ✅️ Two judgement calls it made correctly
- Left `ErasedProjection` and `Emit` as sync `dyn` traits rather than converting them. Correct under my own
  ruling: those are sync **today** (post-revert), and de-dyn scope must be measured **after** re-asyncify,
  never before. Converting them now would have been work against a stale census.
- Left test-module `block_on` untouched and flagged it as a judgement call. Already ruled — **R4 clause 5**:
  a `#[test] fn` is a `main`-equivalent thread root, so `block_on` there is sanctioned and is not counted
  against the census target.

## ▶️ `sdk-dedyn` DISPATCHED — the program's bottleneck, started before its dependency is green

Deliberately started while `🎒️pack` is still red, because this is the longest pole (20,700 lines, 51-method
`PluginApp`, 63 fleet crates downstream) and its first two steps need no compiler:

1. **Re-asyncify its own file first** — it is one of the reverted files (**19 `async fn` in the tree vs
   1,489 in the index**). Via the now tag-aware codemod, never via git: `git restore` would recover the
   conversion and destroy sibling packets' completed work in the same stroke.
2. **Fix the three E4 waker files it owns** (`⚛️reactor/🧵️executor`, `⚛️reactor/📮️requests`,
   `🌐host/📖️body`) — handed over as a **checklist from my earlier repo-wide sweep**, not left to be
   rediscovered mid-packet. The executor's exact patch already exists in the brep probe fixture.
3. Then `PluginApp` de-dyn with the shared macro, plus the three leases filed against its file by
   `io-thunks` and `store-dedyn`.

Briefed with a STOP condition on the one genuinely open design question (if the `A: PluginApp` type
parameter threads through far more public types than the design anticipates, that is my call, not the
executor's) and with explicit permission to report acceptance UNRUN while `🎒️pack` is red — **and an
explicit prohibition on editing `🎒️pack` to unblock itself**, which is how packets historically start
trampling each other.


## ✅️ `wgpu-web-shard` ACCEPTED — with the first genuine RUNTIME evidence of the new path

This packet did the thing this ticket has repeatedly failed to do: it **proved the code runs**, not that
it compiles. Driving the live `s-react` dev server through the Browser pane, it imported the new bridge,
called `loadPluginModule("note", ...)` and captured the network log:

- **4 real `GET /plugin-modules/_shard/🟨️shard-worker.js`** — the pooled shard worker actually spawning.
- **ZERO requests to `🟨️plugin-worker.js`** — the one-worker-per-plugin path is genuinely dead, not merely
  unreferenced in source.
- The call then failed with the **expected stale-bridge error** (`does not provide an export named 'plugin'`)
  **deep inside the real activation pipeline** — not an import error, not a `PluginWorkerClient` error.
  A failure in the right place, for the right reason, is strong evidence; a green compile would have been none.

It also **lifted shared logic instead of copying it** — `🧵️shard-runtime.ts` and `🖼️wire-turn.ts` now live in
`🎭️actor/📦️packages/🟦️typescript/` and serve both renderers. A third divergent copy of worker management is
precisely the defect this ticket exists to delete, so this was the deciding requirement, and it was met.
It also fixed a pre-existing `TS2502` self-reference that only became visible once the dead imports resolved.

Honest gaps it declared rather than buried: render granularity, `windowEngagements`/`windowMeasures` not
carried over, simplified turn serialization. The `nx` pipeline build is unrun because it also builds the
Rust wgpu crate, which a sibling packet has mid-rewrite (`MM` on `🎠️runtime.rs`/`📦️bin.rs`/`📦️glue.rs`) —
reported as unrun rather than faked.

Verification: scoped `tsc --noEmit` clean on every touched file; wgpu vitest **4/4 by name** (up from 1);
`🎭️actor` **40/40** unchanged.

## 🧹 BANNED-SYMBOL SWEEP — I ran it myself; source is clean, and the residue is GENERATED

| symbol | live in source | tombstone comments |
|---|---:|---:|
| **`PluginWorkerClient`** | **0** | 8 |
| `pluginWorkerUrl`, `loadPluginModuleViaWorker`, `PLUGIN_WORKER_UNRESPONSIVE_MS`, `loadPluginModuleUncached` | 0 | — |
| `WasmPluginRuntime`, `ExtensionRuntime`, `ProgramSupervisorState`, `PLUGIN_FUEL_BUDGET`, `INSTANCE_GUARD`, `install_io_fallback_dispatcher`, `set_host_backbone_channel` | 0 | 52 total |

**Both `PluginWorkerClient` copies are gone.** The "Replace, never wrap" list is now clear in first-party
source for 11 of 13 entries.

### ⚠️ The two apparent survivors are NOT source — and this becomes a census rule
- `pluginWorkerUrl` ×1 in `🌎️hub/…/📤️dist/assets/🌐️index-*.js` — a **built dist bundle**.
- `runSerialized` ×390 in `🧑️‍💻️dev/🔌️extension-modules/**/*.js` — the **stale generated jco bridges**,
  the same artifacts that made the wgpu probe fail above. They are build output of
  `🌐plugin-web-materialize.ts` and are regenerated by `web-bridges`, not hand-edited.

**Binding for `census-zero`: the banned-symbol census MUST classify source vs generated/dist output.**
A flat repo-wide grep reports 391 "violations" that no human should ever fix by hand, and would keep the
exit gate red forever for the wrong reason — the same false-positive class as the `TransactionCoordinator::exchange`
trap already recorded on this ticket. Generated output is cleared by regenerating it, and the real check on
it is that the regenerator no longer *emits* the symbol.
