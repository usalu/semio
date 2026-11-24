# Debug Sweep Audit: [DEBUG] Markers in Framework and FEM Trees

Read-only audit, 2026-11-23. No source files edited, no processes started or killed. This report is the only file written.

## Scope and Method

- Pattern: `\[DEBUG\]|recv_dbg|dbg!\(` (Grep, then `grep -rnI` for the raw listing).
- Trees: `C:\semio\🧰️framework` (`*.rs`, `*.ts`, `*.mjs`), `C:\semio\✏️s\🔌️plugins\🏗️fem`, `C:\semio\✏️s\🔨️modules\🏗️fem`. `node_modules`, `.cache`, `target`, `dist` excluded.
- Modified time from `ls -la --time-style=+%m-%d_%H:%M` (year 2026).
- Line text cut to 160 characters. Some lines are very long, so the match may sit past the cut; those rows are listed under "Unclear".

## Totals

| Tree | Files with hits | Hits |
|---|---|---|
| `🧰️framework` | 54 | 85 |
| `✏️s\🔌️plugins\🏗️fem` | 1 | 5 |
| `✏️s\🔨️modules\🏗️fem` | 0 | 0 |
| **Total** | **55** | **90** |

- `recv_dbg` and `dbg!(` have zero hits in all three trees.
- Class: **T** = temporary debug logging (eprintln, println, log, console, stdout/stderr write, or `debug_log`, including once-only statics around them). **L** = legitimate (test assertion or fixture text, or the census test that enforces the convention). **U** = unclear or externally dependent. Counts: T 67, L 11, U 12.

## Build-Break Check

No hit references an undefined identifier. Verified definitions:

- `debug_log`: defined in renderer `scene-render-entry` and `Shell/.../session-and-topology`, imported from `semio_renderer_wgpu_shell_async` by many shell files.
- `blocked_once`: defined in `canonical-edit` (two cfg variants).
- `debug_state` (called by FEM `native.rs` lines 421, 453): `NativeReceiveSlot::debug_state` at `value/decode/owned/value/🦀️.rs:45`. `NodeReceiver` is a `NativeReceiveSlot` alias.
- `LAST_MAINTENANCE_STAGE`: defined in `reactor-app/runtime-helpers`, re-exported through `pub use app_runtime_helpers::*`.
- `trace_failure_retirement`, `trace_retirement`, `trace_retirement_refusal`, `tracing`: defined in the same files as their uses.
- `dbg`, `dbgT0`, `dbgMarks` in `owner-discovery.mjs`: declared line 343, used line 355.
- `z` in `plugin/instance/🦀️.rs` lines 78 and 87: local `use std::mem::size_of as z`.
- `__dbg` in `agree.ts` line 116: read via `globalThis as any`, so an undefined global is harmless.

## Results: Per-Tree Tables

### Tree 1: `C:\semio\🧰️framework` (85 hits)

Paths relative to `C:\semio`. Modified time is `mm-dd_hh:mm`.

| # | File | Line | Modified | Class | Line (160 chars) |
|---|---|---|---|---|---|
| 1 | 🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs | 590 | 11-23_11:28 | T | `        if effect.as_deref_mut().is_some_and(|effect|effect.actual().is_some()){eprintln!("[DEBUG] page allocation occupied caller={} actual={:?}",std::panic::L` |
| 2 | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs | 45 | 11-23_10:00 | U | ` pub fn debug_state(&self)->String{format!("[DEBUG] slot cu={} req={:?} dem={} eph={:?} tag={} nested={:?} closing={} out={}",self.checkpoint_unresolved,self.ac` |
| 3 | 🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs | 167 | 11-23_16:52 | T | `    {static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);if N.fetch_add(1,std::sync::atomic::Ordering::Relaxed)<4{eprintln!("[DEBUG] ` |
| 4 | 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧭️drive/🦀️.rs | 41 | 11-23_16:47 | T | ` let fixture=super::fixture_grant();for index in 0..65536{if close.terminal_is_empty(){return}let(result,born,freed)=semio_framework_trace_memory_testing::obser` |
| 5 | 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧭️drive/🦀️.rs | 57 | 11-23_16:47 | T | `   let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture receiver omitted its issued gra` |
| 6 | 🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs | 68 | 11-23_15:47 | T | `  if !admitted{static SEEN:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);if !SEEN.swap(true,std::sync::atomic::Ordering::Relaxed){let ` |
| 7 | 🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs | 115 | 11-23_16:38 | T | `  if !table.has_reclaimable(){return Ok(false)}{static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);let n=N.fetch_add(1,std::sync::at` |
| 8 | 🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs | 319 | 11-23_16:10 | T | `                if t>200_000_000&&!DONE.swap(true,Ordering::Relaxed){let mut v:Vec<_>=m.iter().collect();v.sort_by_key(|(_,e)|std::cmp::Reverse(e.1));for(size,(` |
| 9 | 🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/retained-charge.rs | 144 | 11-23_04:43 | T | `    eprintln!("[DEBUG] {count} borrowed Source item and fuel debits in {duration:?} (combined: {fast})");` |
| 10 | 🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs | 270 | 11-23_13:58 | T | ` pub fn record_retained(&mut self,progress:P)->Result<(),ValueError>{if self.origin.is_none(){return Ok(())}let mut state=self.snapshot();let issue=state.advanc` |
| 11 | 🧰️framework/🔨️modules/🧵️job/🛬️receiving/🎟️work/🦀️.rs | 14 | 11-23_15:43 | U | ` fn admits<'reply,'turn,'source>(&mut self,c:&mut OriginalJobWorkRecipient<'turn>,quote:semio_framework_value_native_owned::native_decoding::OriginalWorkQuote<'` |
| 12 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🦀️.rs | 46 | 11-21_10:45 | T | `    if std::env::var_os("RUST_BACKTRACE").is_some() { eprintln!("[DEBUG] poll close: {status:?}; primary: {:?}; retirement: {:?}", poll.failure(), poll.cleanup_` |
| 13 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🦀️.rs | 103 | 11-21_10:45 | T | `    if std::env::var_os("RUST_BACKTRACE").is_some() { eprintln!("[DEBUG] poll fault: {status:?}; primary: {:?}; retirement: {:?}", poll.failure(), poll.cleanup_` |
| 14 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🖼️display/🧪️tests/🦀️.rs | 27 | 11-22_03:05 | L | `  turns+=1;assert!(turns<100_000,"[DEBUG] display never completed under {items} item(s) per turn");` |
| 15 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🖼️display/🧪️tests/🦀️.rs | 41 | 11-22_03:05 | T | `  println!("[DEBUG] display turns: 1 item={single}, 64 items={mounted}");` |
| 16 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs | 301 | 11-23_15:30 | U | `                let Some(step)=receiving.with_effect(grant.retained_grant(),work,|_,effect|owner.advance_digest(effect)).and_then(Option::transpose).map_err(ref` |
| 17 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs | 1082 | 11-23_15:30 | T | `    { static SEEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false); if !SEEN.swap(true, std::sync::atomic::Ordering::Relaxed) { eprint` |
| 18 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs | 22 | 11-23_12:05 | T | `pub fn blocked_once(reason: std::fmt::Arguments<'_>) { static SEEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false); if !SEEN.swap(tru` |
| 19 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/👥️presence-store-and-peers-publication/🦀️.rs | 473 | 11-23_13:52 | T | `    { static SEEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false); if !SEEN.swap(true, std::sync::atomic::Ordering::Relaxed) { eprint` |
| 20 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs | 62 | 11-23_16:11 | T | `  if self.next_digest_work_demand()?==0{static SEEN:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);if !SEEN.swap(true,std::sync::atomic` |
| 21 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source/🦀️.rs | 221 | 11-23_15:47 | T | `  if p==RetainedCloneProgress::default()&&!self.terminal_is_empty(){static SEEN:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);if !SEEN` |
| 22 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/🌫️transient-and-interaction-stores/🦀️.rs | 525 | 11-23_13:52 | T | `    { static SEEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false); if !SEEN.swap(true, std::sync::atomic::Ordering::Relaxed) { eprint` |
| 23 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs | 221 | 11-23_02:50 | T | `                    let refuse = || debug_log(&format!("[DEBUG] File \"{name}\" exceeds the {limit}-byte import limit"));` |
| 24 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs | 18 | 11-23_02:50 | U | `/// 🪲 Formats `[DEBUG] shell node-graph fit` with the `{x,y,zoom}` object the port probe parses.` |
| 25 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs | 20 | 11-23_02:50 | U | `    format!("[DEBUG] shell node-graph fit {{\"x\":{x},\"y\":{y},\"zoom\":{zoom}}}")` |
| 26 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🔑️key-and-move/🦀️.rs | 88 | 11-22_03:12 | T | `                            debug_log("[DEBUG] world3d terrain capture cancel reached the intent queue limit");` |
| 27 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs | 218 | 11-23_16:48 | T | `            eprintln!("[DEBUG] retired scene close error {error:?}");` |
| 28 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs | 107 | 11-23_16:12 | T | `  Err(error)=>{eprintln!("[DEBUG] poll_wit authority refusal epoch={} kind={:?} error={:?} spent={:?}",input.epoch,error.kind,error,spent);Err(failure(input,spe` |
| 29 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/🦀️.rs | 70 | 11-21_12:24 | U | `  semio_plugin_abi::wasip2::semio::framework::pure::log("debug", "[DEBUG] unresolved original input receipt");` |
| 30 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs | 86 | 11-23_01:52 | T | `            eprintln!("[DEBUG] FEM cleanup semantic failure instance={} stage={} progress={:?} archive-load: {archive}", cell.id, semio_plugin_reactor_app::app_` |
| 31 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs | 53 | 11-23_05:34 | T | ` let difference=|whole:usize,earlier:usize,later:usize|later.checked_sub(earlier).and_then(|posted|whole.checked_sub(posted)).ok_or_else(||{eprintln!("[DEBUG] u` |
| 32 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs | 69 | 11-23_05:34 | T | `            Err(error) => { eprintln!("[DEBUG] unposted mismatch whole={:?} before={:?} after={:?} primary={:?}\n{}", failure.retained_progress(), before, after` |
| 33 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs | 315 | 11-22_00:46 | T | `                eprintln!("[DEBUG] natural-file worker operation={} phase={:?} calls={calls} bytes={}/{} pack_work={} pending_checkpoint={} checkpoint_delivered` |
| 34 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🎫️operation-admission.rs | 50 | 11-21_21:27 | T | `fn trace_failure_retirement(stage:&str){if std::env::var_os("SEMIO_TRACE_RELAY_RETIREMENT").is_some(){eprintln!("[DEBUG] relay retirement {stage}");}}` |
| 35 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs | 75 | 11-21_21:27 | T | `     eprintln!("[DEBUG] relay retirement contents {attempts}: next={}, result={}, aliases={:?}, remaining={:?}",` |
| 36 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs | 81 | 11-21_21:27 | T | `  if tracing{eprintln!("[DEBUG] relay retirement entering root close");}` |
| 37 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs | 110 | 11-21_21:27 | T | `fn trace_retirement_refusal(stage:&str,error:&GuestRelayAdmissionFailure){if std::env::var_os("SEMIO_TRACE_RELAY_RETIREMENT").is_some(){eprintln!("[DEBUG] relay` |
| 38 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🪙️escrow.rs | 199 | 11-21_21:53 | T | `fn trace_retirement(stage:&str){if std::env::var_os("SEMIO_TRACE_RELAY_RETIREMENT").is_some(){eprintln!("[DEBUG] relay retirement {stage}");}}` |
| 39 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 104 | 11-21_21:30 | T | `     eprintln!("[DEBUG] poll refusal in {:?}: actor={}, transport={:?}, input={:?}, scalar={:?}, decoding={}",poll.phase,poll.actor_failure.is_some(),poll.trans` |
| 40 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 162 | 11-21_21:30 | T | `     eprintln!("[DEBUG] poll VM body: {body:?}, input failure: {:?}, scalar failure: {:?}, settlement: {settlement:?}",poll.input_failure(),poll.scalar_progress` |
| 41 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 172 | 11-21_21:30 | T | `     eprintln!("[DEBUG] poll non-VM refusal in {:?}: actor={}, input={:?}, scalar={:?}, fixed={}",poll.phase,poll.actor_failure.is_some(),poll.input_failure(),p` |
| 42 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 175 | 11-21_21:30 | U | `       if let Some((_,detail))=diagnostics.rsplit_once("[DEBUG] unresolved original input receipt"){` |
| 43 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 176 | 11-21_21:30 | T | `        for line in detail.lines().skip(1).take(2){eprintln!("[DEBUG] poll guest Input: {line}");}` |
| 44 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 189 | 11-21_21:30 | T | `   eprintln!("[DEBUG] poll native progress: step {step}, frontier {frontier:?}, guest loan epoch {:?}, VM fuel spent {:?}, status {status:?}",budget.map(|loan|l` |
| 45 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 209 | 11-21_21:30 | T | `   eprintln!("[DEBUG] poll driver progress: step {step}, stage {}, receipt {:?}, frozen {:?}, retirement {cleanup}, VM fuel spent {:?}",self.stage,self.capture,` |
| 46 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 252 | 11-21_21:30 | T | `      eprintln!("[DEBUG] Input SRE1 settled: receipt {sample}, epoch {}, result {:?}, guest work {}",call.input.epoch,call.kind,call.work_units_spent);` |
| 47 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs | 366 | 11-21_21:30 | T | `    eprintln!("[DEBUG] poll close frontier: {}.{} in {attempts} steps, diagnostic empty {}, guest held {}, remaining {:?}, status {status:?}",self.close_phase,s` |
| 48 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs | 399 | 11-21_13:53 | T | `        if self.phase==Phase::Boot&&std::env::var_os("RUST_BACKTRACE").is_some(){eprintln!("[DEBUG] Boot SRE1 settled: epoch {}, result {:?}, guest work {}",cal` |
| 49 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs | 411 | 11-21_13:53 | T | `        if self.phase==Phase::Boot&&std::env::var_os("RUST_BACKTRACE").is_some(){eprintln!("[DEBUG] Boot SAB1 settled: epoch {}, phase {}, sequence {}, result {` |
| 50 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs | 443 | 11-21_13:53 | T | `                        eprintln!("[DEBUG] poll unframed scalar: {:?}, issued {:?}, actual clock {:?}, host deadline {}, work {}, retained {:?}",self.phase,self` |
| 51 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/📃️policy/📥️operation/🦀️.rs | 125 | 11-21_16:35 | T | `  if std::env::var_os("RUST_BACKTRACE").is_some(){eprintln!("[DEBUG] poll physical currency: treasury {remaining:?}, semantic available after cleanup {:?}, clea` |
| 52 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs | 38 | 11-21_20:14 | T | `    eprintln!("[DEBUG] Core Input witness length={size} fnv1a={fingerprint:016x} scalar_budget={budget:?}");` |
| 53 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs | 206 | 11-21_20:14 | T | `            "[DEBUG] Core physical event={event} operation={operation} generation={generation} epoch={epoch:?} stage={name} function={function} cleanup={cleanup` |
| 54 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs | 272 | 11-21_20:14 | T | `        "[DEBUG] Core pending frame return sample={step} operation={operation} generation={generation} epoch={epoch:?} root={root} leaf={leaf} pc={pc} depth={de` |
| 55 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs | 142 | 11-21_19:24 | T | `      eprintln!("[DEBUG] Core scalar: step {step}, execution epoch {}, root function {:?}, frame depth {}, leaf functions and byte offsets {leaf:?}, VM fuel {us` |
| 56 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs | 84 | 11-21_21:53 | T | `     else if std::env::var_os("RUST_BACKTRACE").is_some(){eprintln!("[DEBUG] poll Source clock lookup did not match: requested {binding}, issued {issued:?}");}` |
| 57 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs | 378 | 11-23_16:47 | T | `        eprintln!("[DEBUG] constructor receipt phase={phase:?} progress={progress:?}");` |
| 58 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs | 478 | 11-23_16:47 | T | `    eprintln!("[DEBUG] app close start spent={:?} child={:?} reserved={:?}", cx.retained_progress(), cx.receiving_child_progress(), cx.reserved_return_progress(` |
| 59 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs | 535 | 11-23_16:47 | T | `    eprintln!("[DEBUG] app close end spent={:?} child={:?}", cx.retained_progress(), cx.receiving_child_progress());` |
| 60 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🔧️replacement-fixtures-and-recursive-documents/🦀️.rs | 71 | 11-23_01:12 | T | `    eprintln!("[DEBUG] maint demand {demand:?} cxgrant {:?} fuel {} yield {}",cx.retained_grant(),cx.fuel_remaining(),cx.retirement_should_yield());` |
| 61 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 79 | 11-23_16:58 | T | `        eprintln!("[DEBUG] store parts log={} owners={} wire={} cursor={} live={} replay={} envelope={} tickets6={}", z::<store::StoreLog<A::Snapshot,A::Mutatio` |
| 62 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 80 | 11-23_16:58 | T | `        eprintln!("[DEBUG] disposer_val={:?}", A::build_document_store_disposer().map(|d| std::mem::size_of_val(&*d)));` |
| 63 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 81 | 11-23_16:58 | T | `        format!("[DEBUG] stores slot_doc={} slot_cfg={} slot_draft={} presence={} transient={} wincfg={} wintr={} artifact_store={} owners={} interaction={}",` |
| 64 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 88 | 11-23_16:58 | T | `        format!("[DEBUG] sizes Self={} A={} snapshot={} mutation={} config={} ctl_close={} constructor_box={} stores={} roster={} ingress={} typing={} emission=` |
| 65 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs | 387 | 11-23_14:40 | T | `                        eprintln!("[DEBUG] worker continuation cause: {cause}");` |
| 66 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs | 434 | 11-23_16:52 | T | `        if demand.release_bytes==27136{static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);if N.fetch_add(1,std::sync::atomic::Orderi` |
| 67 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts | 338 | 11-23_03:56 | U | ` if(selection.kind==="products-graph-fixture"){if(!row.producer||!row.fixture)throw Error("Current exact fixture and known producer selector required");stable(r` |
| 68 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts | 41 | 11-19_20:02 | U | ` expect(text).toContain('row.args.includes("--offline")&&scope.selections.has(');expect(text).toContain("offline:false");expect(source).toContain("options.reque` |
| 69 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔓️sealed-read/🟦️.ts | 47 | 11-22_02:37 | L | `  const written:string[]=[],granted=await withBuildCapacityV1(args,{},async value=>value,{write:line=>written.push(line),disk:{partition:()=>{throw new Error("[` |
| 70 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🌐️host/💥️throwing-host-main.mjs | 4 | 11-16_20:20 | U | `const code=await runHostForegroundV1({implementation,identify:async({root})=>createHash("sha256").update(root).digest("hex"),policy:{residentCheckMilliseconds:5` |
| 71 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🌐️host/🖥️host-main.mjs | 7 | 11-10_03:14 | U | `const code=await runHostForegroundV1({implementation,identify:async({root})=>createHash("sha256").update(readFileSync(file(root))).digest("hex"),policy:policyJs` |
| 72 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs | 355 | 11-23_05:23 | T | ` await stage("catalog-targets");dbg("end");if(performance.now()-dbgT0>500)process.stderr.write("[DEBUG] [owner-declaration] "+owner.name+" "+dbgMarks.join(" ")+` |
| 73 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧮️conformance/🧪️tests/🟦️.ts | 224 | 11-14_12:48 | L | `   writeFileSync(join(target,file),file.endsWith(".json")?JSON.stringify(JSON.parse(original),null,4)+"\n":"// [DEBUG] body-only edit\n"+original+"\n// trailing` |
| 74 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🔖️journal-memory/🟦️.ts | 34 | 11-23_17:02 | T | ` console.log("[DEBUG]",JSON.stringify({small,large}));` |
| 75 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️host/🧪️tests/🌊️wave-prime/🧫️fixture.ts | 30 | 11-21_19:16 | U | `process.stdout.write("[DEBUG] "+JSON.stringify({outcomes,trace})+"\n");` |
| 76 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📍️relocate/🧪️tests/🟦️.ts | 137 | 11-14_13:06 | L | `   await expect(run(fleet,{fault:reached=>{if(reached===step&&!crashed){crashed=true;throw Error(`[DEBUG] injected crash after ${reached}`);}}})).rejects.toThro` |
| 77 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🧭️agree.ts | 95 | 11-14_00:43 | T | `   for(const other of report.rows)if(other.originId!==who.name&&other.verdict==="diverged"){named.push(`wrong:${other.originId}`);console.log("[DEBUG]",JSON.str` |
| 78 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🧭️agree.ts | 116 | 11-14_00:43 | T | `   for(const other of report.rows)if(other.originId!==who.name&&other.verdict==="diverged"){named.push(`wrong:${other.originId}`);if(!(globalThis as any).__dbg)` |
| 79 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧲️adopt/🧪️tests/🟦️.ts | 225 | 11-15_17:29 | L | `   await expect(run(fleet.adopter,{fault:reached=>{if(reached===step&&!crashed){crashed=true;throw Error(`[DEBUG] injected crash after ${reached}`);}}})).reject` |
| 80 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts | 152 | 10-25_13:12 | L | `/** 🐞️ One line carrying the `[DEBUG]` tag, which AGENTS.md reserves for temporary logs removed before a change lands. */` |
| 81 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts | 155 | 10-25_13:12 | L | `/** 📖️ Every line of one source that carries the `[DEBUG]` tag. */` |
| 82 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts | 157 | 10-25_13:12 | L | `  return text.split("\n").flatMap((line, index) => (line.includes("[DEBUG]") ? [{ path, line: index + 1, text: line.trim().slice(0, 120) }] : []));` |
| 83 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts | 163 | 10-25_13:12 | L | `/** ▶️ The `[DEBUG]` census over every tracked text source outside the ticket tree, Markdown prose and` |
| 84 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts | 168 | 10-25_13:12 | L | `  const listed = runToolSyncV1("git", ["grep", "-l", "-z", "-I", "-F", "[DEBUG]", ...scope], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });` |
| 85 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts | 182 | 10-25_13:12 | L | `  const oracle = new Map(runToolSyncV1("git", ["grep", "-c", "-I", "-F", "[DEBUG]", ...scope], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.s` |

### Tree 2: `C:\semio\✏️s\🔌️plugins\🏗️fem` (5 hits, one file)

Paths relative to `C:\semio`. The `artifacts/🧊️3d` copy of this `native.rs` has zero hits, so the two subset copies have diverged.

| # | File | Line | Modified | Class | Line (160 chars) |
|---|---|---|---|---|---|
| 86 | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs | 254 | 11-23_09:00 | T | `        if !(self.nodes_receiver.as_ref().is_some_and(|rx| rx.advance_pending()) || self.materials_receiver.as_ref().is_some_and(|rx| rx.advance_pending())) { i` |
| 87 | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs | 255 | 11-23_09:00 | T | `        macro_rules! effect { ($units:expr; $e:ident => $run:expr) => { match { let r = work.measured(control, |c| c.with_allocated_effect(d, $units, |_, $e| $r` |
| 88 | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs | 270 | 11-23_09:00 | T | `                    if turn.result.is_err() { eprintln!("[DEBUG] rx.advance refused phase={} adv_after={} ctl={}", self.phase, rx.advance_pending(), rx.requires` |
| 89 | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs | 453 | 11-23_09:00 | T | `        let d = match self.close_demands(body) { Ok(d) => d, Err(error) => { if let Some(r) = self.nodes_receiver.as_ref() { eprintln!("{}", r.debug_state()); }` |
| 90 | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs | 477 | 11-23_09:00 | T | `            if let RetainedWorkPoll::Complete(turn) = &poll { if turn.result.is_err() { eprintln!("[DEBUG] nodes close refused adv={adv} ctl={ctl} birth={birth}` |

### Tree 3: `C:\semio\✏️s\🔨️modules\🏗️fem`

No hits.

## Safe to Remove

Temporary logging. Remove each helper together with its call sites so no dead functions or statics are left behind.

- All other T rows (see table): one `eprintln!`, `println!`, `debug_log`, or formatted diagnostic with no reader of its result. This covers the runtime, poll, host, trace, instance, maintenance, and FEM rows.
- Relay helpers (rows 34-38): `trace_failure_retirement` in `🎫️operation-admission.rs:50`, `trace_retirement_refusal` (`📦️operation.rs:110`), `trace_retirement` (`🪙️escrow.rs:199`), and the `tracing` binding (`📦️operation.rs:69`, used at line 81). Remove the helpers with their calls at `operation-admission.rs` lines 43-48. All are gated by `SEMIO_TRACE_RELAY_RETIREMENT` or `RUST_BACKTRACE`, but still temporary.
- Once-only statics (`SEEN`, `DONE`, `N`) that guard a log: rows 6, 7, 8, 17, 18, 19, 20, 21, 22, 66. Remove the static with its log.
- Test prints: rows 4, 5, 9, 15, 57-60.
- TS/MJS: `journal-memory` (row 74, `console.log`), `agree.ts` (rows 77, 78; the `globalThis.__dbg` guard goes too), and `owner-discovery.mjs` row 72 (remove the `dbg` helper, `dbgT0`, `dbgMarks`, and the `process.stderr.write` together).
- FEM `native.rs` (rows 86-90, 2d copy only): remove the `[DEBUG]` lines and the unprefixed `debug_state()` eprintlns at lines 421 and 453 together. The 3d copy has no hits.

## Unclear or Owned by Others

- **Row 2** `value/decode/owned/value/🦀️.rs:45` (`debug_state`): diagnostic formatter used by FEM `native.rs` 421/453. Remove only with those calls, and check other `debug_state` implementations.
- **Row 11** `🛬️receiving/🎟️work/🦀️.rs:14` and **row 16** `bundle/🦀️.rs:301`: match is past the 160-char cut. Read the full line.
- **Rows 24-25** `dispatch-action/🪟️layouts/🦀️.rs:18,20`: a doc comment and a formatter emitting `[DEBUG] shell node-graph fit {x,y,zoom}`, which the doc comment says a port probe parses. External contract; confirm with the probe owner.
- **Rows 29 and 42**: `call/📤️receipt/🦀️.rs:70` writes `"[DEBUG] unresolved original input receipt"`, and `🎛️session/🦀️.rs:175` splits diagnostics on that exact string. Removing line 70 silently breaks the session parsing. Change both together.
- **Rows 67-68** (`nx/📦️graph/🟦️.ts:338`, `consumption/🟦️.ts:41`): match is past the cut. Read before deciding.
- **Rows 70-71** `resident/…/throwing-host-main.mjs:4` and `host-main.mjs:7`: match is past the cut (likely an error message).
- **Row 75** `wave-prime/🧫️fixture.ts:30`: JSON trace on stdout with a `[DEBUG]` prefix. Check whether a test parses stdout before removing.

## Legitimate Uses (keep)

- **Row 14** `display/…/🦀️.rs:27`: test assertion message.
- **Row 69** `host-capacity/…/sealed-read/🟦️.ts:47`: deliberately throwing test disk stub.
- **Row 73** `stamps/…/conformance/🟦️.ts:224`: writes a `// [DEBUG] body-only edit` marker into a fixture file.
- **Rows 76, 79** `relocate/🟦️.ts:137`, `adopt/🟦️.ts:225`: crash-injection messages that tests match on (`injected crash`). Keep.
- **Rows 80-85** `docstring-and-debug-census/🟦️.ts`: the census test enforcing the `[DEBUG]` convention. Keep.

## Notes

- Grep reported `[Omitted long matching line]` for several hits; the table uses `grep -n` with the 160-char cut.
- Most hits were modified 11-21 to 11-23. The FEM hits are all in `native.rs` (2d copy, modified 11-23 09:00).
