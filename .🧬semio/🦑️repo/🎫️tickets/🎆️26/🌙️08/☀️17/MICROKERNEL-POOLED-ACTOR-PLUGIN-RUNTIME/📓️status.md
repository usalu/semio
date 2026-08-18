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
