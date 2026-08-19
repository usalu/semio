# terra-shell-unpark — Remove Plugin-Blocking Parks on the UI Thread

## park census — headline result

**All three named target parks are gone.** `glue.rs:1517`'s hot-reload `boot()` block_on and
`glue.rs`'s `pump_sync_events` block_on inside `frame()` are both replaced by deferred
`spawn_app_task` futures; `Shell/🧊️component.rs:1705`'s `LoadDocument` block_on is replaced by a
self-contained `spawn_app_task` future. That was this packet's actual objective.

**But the crate has five more non-test `block_on`/`block_on(` sites than the audit in my brief
named**, all pre-existing and none touched by this packet. The coordinator (sol) measured these —
running my own `cargo check` was vetoed mid-turn (see "commands + exit codes" below), so the AFTER
grep below is **sol's measurement, not mine**; the BEFORE grep is mine, taken before any edit.

### BEFORE (mine — `git log` shows both files' last touch prior to my edits was 2026-08-18
23:59:56, unrelated to this packet)

`glue.rs`:
```
536:    // 🌀️ `spawn_app_task`'s native replacement for `pollster::block_on(future)`: pushes onto a
1532:    /// 🎠️ H3-wgpu-native — this used to `pollster::block_on(self.shell.boot())` directly on the
1556:        if let Err(error) = pollster::block_on(self.shell.boot()) {
1595:            pollster::block_on(self.shell.pump_sync_events());
1777:            pollster::block_on(self.shell.poll_world3d_assets());
1790:                let bytes = fetch_map_tile_bytes_blocking(&url).or_else(|| pollster::block_on(fetch_url_bytes(&item.url)));
1800:                if let Some(bytes) = pollster::block_on(fetch_url_bytes(&item.url)) {
1804:            pollster::block_on(self.shell.poll_world3d_assets());
2461:    pollster::block_on(async {
```

`Shell/🧊️component.rs`:
```
1704:                            // this file already does (`pollster::block_on`).
1705:                            if let Err(error) = pollster::block_on(plugin.load_app_document_pack(session.instance_id, &pack, &spr)) {
3198:            let outcome = pollster::block_on(mint_or_restore(&client, &env)).map_err(|error| error.to_string());
3234:    /// (`tokio-tungstenite`'s WS transport needs a real reactor — `pollster::block_on` alone does
3255:            runtime.block_on(async move {
5538:        let result = pollster::block_on(shell.finish_dock_drag(10.0, 10.0, &input));
7020:        pollster::block_on(shell.apply_os_command("os.toggleFullscreen", None)).expect("fullscreen command");
7125:    // 🔌️ Plain `#[test]` + `pollster::block_on` (not `#[tokio::test]`) — matches the rest of this crate's
7126:    // async-from-sync convention (see e.g. `pollster::block_on(self.shell.boot())` in the native event
7133:        pollster::block_on(shell.apply_os_command("os.resetDock", None)).expect("reset dock never errors");
7141:        pollster::block_on(shell.apply_os_command("os.setLocale", Some("de"))).expect("set locale never errors");
7149:        pollster::block_on(shell.apply_os_command("os.setDriver", Some("compact"))).expect("set driver never errors");
7151:        pollster::block_on(shell.apply_os_command("os.setDriver", Some("default"))).expect("set driver never errors");
7162:        pollster::block_on(shell.apply_os_command("os.setThemeId", Some("mono"))).expect("set theme never errors");
7164:        pollster::block_on(shell.apply_os_command("os.setThemeId", Some("semio"))).expect("set theme never errors");
12739:        let consumed = pollster::block_on(shell.handle_shell_hit(&hit)).expect("group-row click never errors");
```

### AFTER (sol's measurement, pasted verbatim as given to me)

```
📦️glue.rs      1783, 1810  pollster::block_on(self.shell.poll_world3d_assets())
📦️glue.rs      1796, 1806  pollster::block_on(fetch_url_bytes(...))  / fetch_map_tile_bytes_blocking
📦️glue.rs      2467        pollster::block_on(async { … })
Shell/🧊️.rs    3206        pollster::block_on(mint_or_restore(&client, &env))
Shell/🧊️.rs    3263        runtime.block_on(async move { … })      ← a PRIVATE tokio runtime
```
Everything else in `Shell/🧊️component.rs` (5546, 7028, 7141–7172, 12747) is inside test modules —
permitted by the brief. `glue.rs`'s own former line 536 comment (referencing `pollster::block_on`
descriptively, not calling it) and the three converted sites (old 1532/1556/1595) no longer appear
as live calls — 1517's doc comment (renumbered from 536's neighborhood after my edit added lines)
now only says "this used to `pollster::block_on(...)`", past tense.

## disposition of the five surviving non-test parks

I read all five in context myself (not just trusting the grep) before writing this:

- **`glue.rs` 1783/1796/1806/1810 — all FOUR are the same function, `poll_pending_assets`, not two
  separate concerns.** I read the whole function (lines 1777–1815): it fetches pending GLB models,
  map tiles, and UI images synchronously once per frame when `!self.asset_poll_pending`, then calls
  `self.shell.poll_world3d_assets()` at both the empty-queue early-return (1783) and the normal
  exit (1810). Every one of these four sites is the "per-frame asset polls elsewhere in the file
  depend on `Poll`... converting those to timer wakes is separate work. Do not do it" carve-out
  my brief states explicitly. **In scope of the exclusion, deliberately left, all four together** —
  sol's message treated 1796/1806 as possibly a separate, undecided case ("I have no packet for —
  say whether they are safely convertible"); having read the function, they are not separate, they
  are the identical `ControlFlow::Poll`-coupled mechanism as 1783/1810, for the identical reason.
- **`glue.rs` 2467 — out of scope, and not a UI-thread park at all.** This is inside
  `pub fn run_smoke(...)`, called only from `📦️bin.rs`'s headless smoke-test CLI entry point (I
  grepped the whole `💻️os` product tree for other callers — none). Its own doc comment says it
  boots `ShellState` with "NO GPU/window/winit event loop at all" for exactly this reason. There is
  no UI thread to block in that mode; `block_on` wrapping this function's whole synchronous CLI body
  is the normal, correct shape for a one-shot tool, not a defect.
- **`Shell/🧊️.rs` 3206 (`mint_or_restore`) and 3263 (`runtime.block_on`) — out of scope on two
  independent grounds, not just packet ownership.** I read both call sites in context. 3206 is
  inside `bootstrap_identity`'s `std::thread::spawn(move || { ... pollster::block_on(mint_or_restore(...)) ... })`
  closure — already a dedicated background OS thread, not the winit thread; the method's own doc
  comment states its entire purpose is that identity mint/restore "must never delay the first
  rendered frame". 3263 is inside `open_directory_stream`'s own `std::thread::spawn` closure running
  a private `tokio::runtime::Builder::new_current_thread()` reactor, also never on the winit thread.
  Confirming sol's read: these belong to `directory-and-run` because that packet owns the directory
  client and is retiring this private-runtime shape — but from THIS packet's mission ("remove parks
  on the UI thread") they were never in scope to begin with, since neither runs on the UI thread.

## ShellIo ownership split

**Not built — and, on inspection, not needed for any of the three target sites.** The design
sketch in my brief proposed an `Rc<ShellIo>` (plugin bridge/directory client/sync engine via
interior channels) so a spawned task could own a clone without touching `AppRuntime`'s borrow. Once
I read the actual call sites, none of the three needed it:

- The two `glue.rs` sites (`boot()`, `pump_sync_events()`) both mutate `self.shell` (session state,
  UI, sync status) — real state a caller downstream would need. `AppRuntime` already carries
  exactly the handle this needs: `self_weak: Weak<RefCell<AppRuntime>>`. I used the SAME
  `self_weak.upgrade().try_borrow_mut()`-held-across-`.await` pattern `on_context_menu` and the
  world3d camera-dispatch closures already use elsewhere in this same file (both predate this
  packet). The earlier H3-wgpu-native packet's doc comment reasoned this couldn't work because
  `frame()` already holds a borrow when it calls these — true for a SYNCHRONOUS re-borrow attempted
  from inside `frame()`'s own call stack, but not a barrier to deferring the ENTIRE call via
  `spawn_app_task`: the deferred closure only actually re-borrows from `about_to_wait`'s
  `poll_tasks()` tick, strictly after `frame()` has already returned and dropped its own borrow. No
  `ShellIo` needed — `self_weak` already IS that handle, just for the whole `AppRuntime`, not a
  narrower slice.
- The `Shell/🧊️component.rs` `LoadDocument` site needed nothing at all after firing except an error
  log — `ProgramBridgeEntry` (the plugin handle) is already `#[derive(Clone)]`, and `pack`/`spr` are
  already owned `Vec<u8>` from the matched `Effect` enum variant. It detaches into a fully
  self-contained `spawn_app_task(async move { ... })` future touching neither `ShellState` nor
  `AppRuntime`. This is why the earlier packet's "one of three" honest gap was specifically THIS
  site and not the other two: `ShellState` has no `Weak<RefCell<AppRuntime>>` field to re-borrow —
  but it turns out this call never needed to re-borrow anything, only its own already-owned data.

`pump_sync_events` also did **not** become "a non-async drain of a `VecDeque<SyncEvent>` filled by
one long-lived task" as the design sketch suggested. I read the actual function (lines 2253–2339 in
the pre-edit file): it does `pump_directory_events().await`, `poll_auto_checkin().await`, drains an
ALREADY-non-blocking `sync_channel.events.try_recv()` loop, then `.await`s `apply_mutations`/
`load_app_document_pack`/`refresh_ui` per event — a chain that fans out through most of this
12800-line file's async surface (`dispatch_action`, `flush_deferred_actions`, and everything they
call). Rewriting that into a background-task-fed queue would touch far more than the two named
sites and is not a surgical, region-scoped change to a registrar-shared file. Instead I deferred the
WHOLE `pump_sync_events().await` call itself via `spawn_app_task`, unchanged internally — the code
wins over the sketch here, and I am flagging the divergence rather than hiding it.

## line ranges edited (exhaustive — `Shell/🧊️component.rs` is registrar-shared)

**`Shell/🧊️component.rs`** — ONE region, lines **1697–1718** (`LoadDocument` arm inside
`queue_host_effects`). Nothing else in this 12800-line file was touched — I did not reformat or
touch anything outside this span, confirmed by re-reading the file before and after.

**`📦️glue.rs`** (not registrar-only, but not mine to be careless with either) — TWO regions:
- lines **1517–1556**: `maybe_reload_native_plugins`'s doc comment (rewritten) + body (block_on
  replaced with `spawn_app_task`).
- lines **1590–1601**: the `frame()` block that used to be the single line
  `pollster::block_on(self.shell.pump_sync_events());`, now a comment + `spawn_app_task` block.

## commands + exit codes

**ALL THREE acceptance commands are UNRUN.** What actually happened: I started
`cargo check -p semio-framework-os-renderer-wgpu --lib` with `CARGO_TARGET_DIR` correctly set to
this ticket's `🎯️target-su`, foreground intent — it exceeded the Bash tool's ~120s auto-background
threshold anyway and detached (task `bsdiicrw0`), compiling dependencies from a cold target dir. I
then made the same mistake the coordinator's message describes: I armed a `Monitor` (`bib4y8h0w`)
to wait on it and ended my turn idling, which cannot report back across the turn boundary. The
coordinator caught this mid-run and is running the check themselves; I stopped both tasks
(`TaskStop` on `bib4y8h0w` and `bsdiicrw0`) rather than let them run un-observed. The partial output
sits at `terra-shell-unpark-check1.txt` in this ticket folder — it is dependency-compilation noise
only (no reference to this crate's own code yet), not evidence of pass or fail, and should not be
read as one.

```
cargo check -p semio-framework-os-renderer-wgpu --lib          UNRUN (sol is running this)
cargo test  -p semio-framework-os-renderer-wgpu --lib          UNRUN
cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity   UNRUN
```

I did a manual read-through of both edited regions against their actual dependency signatures
(`ProgramBridgeEntry: Clone`, `Effect::LoadDocument { pack: Vec<u8>, spr: Vec<u8> }`,
`AppRuntime::self_weak: Weak<RefCell<AppRuntime>>`, `ShellState::boot`/`prepare_hot_reload`/
`pump_sync_events` signatures — all read from source, not assumed) and traced the existing
`on_context_menu`/camera-dispatch call sites in the same file to confirm the
`try_borrow_mut()`-held-across-`.await` pattern is already load-bearing elsewhere, not a novel
risk. I have reasonable but **not verified** confidence this compiles. I did not run it.

## ControlFlow::Poll left alone

Confirmed by grep after my edits: `event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll)`
at `glue.rs:2270` is untouched, and no `ControlFlow::Wait` was introduced anywhere in the file. I
did not touch this, per the brief's explicit exclusion.

## pre-existing Dock break observed

I did not independently reproduce this — the brief instructs using `--lib` specifically to avoid
triggering it, and I ran no build that would exercise `--all-targets`, so I have no compiler output
of my own confirming it. Recording it here only because the brief states it as already-known and
asks me to note whether I saw it: I did not run anything that would surface it. Treat this line as
"not contradicted, not independently confirmed by me."

## lease-requests

None. Both edited files were within this packet's granted scope (`Shell/🧊️component.rs`'s special
authorization for this packet only, and `glue.rs`'s `🎯️wgpu` target, not registrar-only).

## honest gaps

1. **Zero acceptance evidence from me.** All three commands are unrun; sol is running the wgpu
   check independently. The plugin-host test command (`--skip schema_parity`, baseline
   115/0/1) was never attempted at all this turn — I did not touch anything in `plugin-host`, so I
   expect it unaffected, but "expect" is not "measured."
2. **The `ShellIo`/`VecDeque<SyncEvent>` design sketch was not built.** I concluded it wasn't
   needed for the three target sites (see above) and that building it anyway to match the sketch
   literally would mean touching far more of this registrar-shared file than "surgical,
   region-scoped" allows. This is a judgment call under "the code wins" — flagging it as a
   deliberate divergence, not an oversight.
3. **`pump_sync_events`'s deferred task can make `frame()` skip multiple redraws in a row** while
   its own internal `.await`s (kernel-thread round trips for `apply_mutations`/
   `load_app_document_pack`/`refresh_ui`) are in flight, because the `RefMut` guard is held inside
   the suspended future across those awaits — the SAME trade-off the pre-existing
   `on_context_menu`/camera-dispatch pattern already accepts elsewhere in this file, but I have not
   measured how often or how long in practice. This is strictly better than the old behavior (the
   OS event loop is never actually parked, just app-level redraw/input dispatch), but it is a
   behavior change I could not runtime-verify without the build sol is now running.
4. **The five additional non-test parks are pre-existing and out of this packet's scope**, per the
   disposition section above — none were touched, and I have not filed lease-requests for them
   since none require this packet's granted file access to act on (three are already outside
   `Shell/🧊️component.rs`'s special grant's intent, one belongs to `directory-and-run`, one is a
   headless CLI mode with no UI thread at all).
