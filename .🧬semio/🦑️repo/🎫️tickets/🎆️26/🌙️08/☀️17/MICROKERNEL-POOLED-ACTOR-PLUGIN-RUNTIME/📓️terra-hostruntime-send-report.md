# terra-hostruntime-send — R15 implementation report

Executor packet `hostruntime-send`. Implements R15 (ratified ruling in `📌️important.md`):
`HostAsyncRuntime`'s method DECLARATIONS become RPITIT with an explicit `Send` bound; every `impl`
keeps its literal `async fn` body.

## Files changed

- `🧰️framework/🔨️modules/⏳️async/🦀️component.rs`
  - `pub trait HostAsyncRuntime` (was ~line 378): all 6 method declarations
    (`open_scope`, `spawn_scoped`, `run_blocking`, `sleep_until`, `cancel_scope`, `now_ms`)
    converted from plain `async fn` to `fn … -> impl Future<Output = T> + Send + '_` (named `'a` where
    two borrowed parameters need to share a lifetime: `open_scope`, `run_blocking`, `cancel_scope`).
    Each carries a `// 🚫️async: R15` doc note pointing back to the ruling.
  - `ManualRuntime::cancel_scope` (testkit impl) — **genuine finding, fixed, not papered over.**
    Holding a `std::sync::MutexGuard` (`scopes`, `tasks`) across an `.await` compiled fine under the
    old implicit (unprovable) Send bound but is a hard error once the trait declares `Send`
    explicitly (`MutexGuard` is `!Send` on every platform, unconditionally). Rewrote the method to
    snapshot `scopes` into an owned `Vec<(u64, ManualScopeRecord)>` (added `#[derive(Clone)]` to
    `ManualScopeRecord` — its only field beyond two `u32`s is `CancelToken`, already `Clone`,
    `Arc`-backed) before any `.await`, await against the snapshot with no lock held, then re-acquire
    `tasks`/`scopes` only for the plain-synchronous mutation at the end. Behavior preserved exactly
    (same test, `manual_runtime_cancel_scope_reports_finished_and_cancelled`, passes unchanged).
    This is the one impl-side change R15 anticipated as possible ("if one of them fails the Send
    bound, that is a genuine finding") — it was a code-shape defect (lock held across await), not a
    structural non-Send type, so "all three impls are already structurally Send" in the ruling still
    holds; nothing was restructured to be Send, a latent bug in how a lock was scoped was fixed.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`
  - **Deleted `BoxedHostAsyncRuntime`** (trait + both impls: `TokioHostRuntime`,
    `#[cfg(test)] ManualRuntime`) — the double-future workaround R15 named as the thing to remove.
    Confirmed zero remaining references anywhere in first-party `.rs` code (two hits left are my own
    explanatory comments naming it historically, not code).
  - `TimerWheel::spawn_driver` / `HttpPool::spawn_refill_driver`: bound relaxed from
    `R: BoxedHostAsyncRuntime + 'static` to `R: HostAsyncRuntime + 'static`. Both now call
    `runtime_for_loop.sleep_until(..)` / `.now_ms(..)` directly — provably `Send` for a generic `R`
    now that the trait declares it — instead of going through `sleep_until_boxed`/`now_ms_boxed`.
    `spawn_driver`'s `tokio::select!` still needs its two match arms unified to one type, so `sleep_fut`
    is now a locally-scoped `Pin<Box<dyn Future<Output = ()> + Send + '_>>` (borrowed, not
    `HostFuture`'s `'static`) rather than a boxed-at-`Arc`-ownership detour.
  - Added `use std::future::Future;` / `use std::pin::Pin;` at the top (needed for the above); fixed
    two now-unnecessary-qualification warnings this exposed (`block_on`, `resolve_ready` had spelled
    out `std::future::Future` inline before `Future` was in scope — trivial cleanup, not R15 itself).

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`
  - `impl<I: EnvelopeInjector, R: HostAsyncRuntime + 'static> AsyncEffectExecutor<I, R>` → added
    `I: EnvelopeInjector + 'static`. **Second genuine finding.** Once R15 unblocked the 6 target
    E0277 Send errors at this file's `dispatch_http`/`dispatch_storage`/`dispatch_set_timer`/
    `dispatch_router_effect` (`Box::pin(async move {...})` into `HostFuture<()>`), rustc's *next*
    check — `'static` outlives, not Send — surfaced 4 E0310 errors: those same boxed blocks capture
    `sink: Arc<EnvelopeCompletionSink<I>>`, which needs `I: 'static` to be nameable inside a
    `Pin<Box<dyn Future + Send + 'static>>`. This was always true; the Send error just got reported
    first and apparently suppressed the outlives check on the same expression (rustc error-recovery
    behavior, not something R15 introduced). Fixed exactly as rustc's own `help:` suggested. No other
    `AsyncEffectExecutor<I, R>` impl block exists in this file, so this is the only site.
  - No other lines in this file needed editing — `runtime.sleep_until(..)`/`.run_blocking(..)` calls
    inside the `Box::pin(async move {...})` blocks (lines ~841/861/903/977, the original 6-error
    sites) needed no code change at all once the trait declaration carried its own `Send` bound; the
    fix is entirely upstream in the trait.

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/**` (`InlineRuntime`) — **read, not edited.** Its
`impl HostAsyncRuntime for InlineRuntime` already has no lock held across any await and needed zero
changes; confirmed by compiling `semio-framework-os-kernel-db` and checking for zero
`InlineRuntime`/`HostAsyncRuntime`-attributed errors in its (large, pre-existing, unrelated) error
list — see Non-fix below.

## Acceptance — all run in the foreground, this session, real exit codes pasted

1. `cargo check -p semio-framework-plugin-host --lib` → **EXIT 0** (38 warnings, all pre-existing
   classes: `async_fn_in_trait` R7-sanctioned + the documented R13 dropped-future census — see below)
2. `cargo check -p semio-framework-async --lib` → **EXIT 0**.
   `cargo check -p semio-framework-os-services --lib` → **EXIT 0** (2 trivial unnecessary-qualification
   warnings fixed along the way).
3. `cargo check -p semio-framework-os-kernel --lib` → **EXIT 0** (57 warnings, all `async_fn_in_trait`).
   `cargo test -p semio-framework-os-kernel --lib` → **779 passed / 0 failed / 0 ignored**.
4. `cargo check -p semio-framework-plugin --lib` → **EXIT 0**.
   `cargo check -p semio-framework-plugin --lib --all-features` → **EXIT 0**.
5. Triple-target guard, `semio-framework-os-kernel --lib`:
   `--target wasm32-unknown-unknown` → **EXIT 0**. `--target wasm32-wasip2` → **EXIT 0**.
6. `cargo check -p semio-s-plugin-note --lib` → **EXIT 101, 23792 errors** (report only, per brief —
   see Non-fix #1 below; this is NOT the "38, all guest SDK" baseline any more).
7. `grep -rn "BoxedHostAsyncRuntime" **/*.rs` → only 2 hits, both my own explanatory prose comments in
   `🛎️services/🦀️component.rs` naming the deleted type historically. **Zero code references.**

Extra confidence runs beyond the brief's list (not required, done anyway since I touched these files
directly):
- `cargo test -p semio-framework-async --lib` → **17 passed / 0 failed** (was 16 in the last recorded
  baseline; +1 is pre-existing drift from another session, not from this packet — every existing test
  name still present and green, including `manual_runtime_cancel_scope_reports_finished_and_cancelled`
  which exercises the exact method I rewrote).
- `cargo test -p semio-framework-os-services --lib` → **30 passed / 0 failed** (was 26; same
  pre-existing-drift note, `spawn_driver`/`spawn_refill_driver`'s own tests are in this set and green).

## Non-fixes — found, diagnosed, explicitly NOT touched (out of R15's scope)

1. **`semio-s-plugin-note --lib` no longer reaches the fleet-readiness question at all.** It aborts
   immediately on its dependency `semio-s-plugin-stdio`, **23792 errors** (breakdown: 6994×E0271,
   6161×E0277, 5799×E0308, 1937×E0599, 718×E0728, plus smaller codes). Confirmed by `git status` this
   is live, uncommitted work under `✏️s/🔌️plugins/🗄️stdio/**` — explicitly listed in this packet's own
   brief as "NOT yours... packet `stdio-await`". Not caused by anything in this packet (no file I
   touched is anywhere near `🗄️stdio`); not fixed. The "38 errors, all guest SDK" baseline this
   acceptance step expected to re-measure is stale — `stdio-await`'s in-flight edit is the actual
   blocker now and needs to land before that number means anything again.
2. **`semio-framework-os-kernel-db --lib` is RED, 280 errors** — checked for my own confidence since
   `InlineRuntime` lives there and is in this packet's path_scope, even though it wasn't on the
   brief's numbered acceptance list. Confirmed **zero** of the 280 errors mention `InlineRuntime` or
   `HostAsyncRuntime` (grepped explicitly; no E0053 signature-mismatch errors at all — that's the
   class an actual trait-shape regression would produce). The errors are missing `.await` /
   missing-import / recursive-async-needs-`Box::pin` shapes, matching this ticket's own prior
   recorded finding that `semio-framework-os-kernel-db` was left RED (84 errors then) when packet
   `db-trait-flip` was stopped mid-flight (R25). It has since grown to 280 from further unrelated
   concurrent churn. Not this packet's to fix — `InlineRuntime` itself compiles clean against the new
   trait; the crate around it doesn't.
3. **`semio-framework-plugin-host --all-targets`** (checked for extra confidence, not required):
   **919 errors**, entirely `#[cfg(test)]`. Spot-checked the E0053 subset specifically (the class that
   would indicate an actual trait-signature regression from this packet) — all of them are sync
   traits (`RouterEffectHandler::handle`, a `StorageBackend`-shaped `read`/`write`/`delete`) whose
   `#[cfg(test)]` mock impls were written `async fn` against a plain `fn -> Result<...>` trait method.
   None involve `HostAsyncRuntime`. This is the same documented residue class as this ticket's
   `sdk-final`/`dispatch-group-split` findings (a separate, already-flagged, not-yet-owned
   `#[cfg(test)]` packet) — not new, not from R15, not fixed here (out of this packet's numbered
   acceptance, which only demands `--lib`).

## Scope discipline

Touched only: `⏳️async/🦀️component.rs` (trait + `ManualRuntime`), `🛎️services/🦀️component.rs`
(`TokioHostRuntime`'s `BoxedHostAsyncRuntime` deletion + the two driver loops), `🔌️plugin/🖥️host/
⚡️effects/🦀️component.rs` (the `'static` bound fix at the exact 6-error site named in the brief). Did
not touch `🛢️db/**` (read-only), did not touch any second trait with `+ Send` (R3 stands everywhere
else), did not touch `stdio`, `dsl`, `inference`, or `🏪️store`.
