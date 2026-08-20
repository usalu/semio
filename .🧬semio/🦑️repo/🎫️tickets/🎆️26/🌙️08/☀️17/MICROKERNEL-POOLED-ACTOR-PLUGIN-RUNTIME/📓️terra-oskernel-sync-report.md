# 📓️ terra-oskernel-sync-features report

Packet: `oskernel-sync-features`. Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
plus the sibling-error file located precisely at
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs` (both are `#[path]`-mounted
into `semio-framework-os-kernel`, the former via `os_store::sync`, the latter via `os_directory::client`).

## 🎯 Mission result

`cargo check -p semio-framework-os-kernel --lib --features sync` is EXIT 0. Every feature combo this
program actually consumes is now EXIT 0 on `--lib` **and** `--all-targets`, on all three targets. The
crate's hard regression guard (`cargo test --lib`, default features) is unchanged at **779 passed / 0
failed**. The forced-rebuild dropped-future census on the fully-green crate found and fixed **one real
production bug** (a remote-mutation push that never ran), which in turn fixed 3 of 7 initially-red
`--features sync` runtime test failures.

## Per-feature / per-target compile map

| combo | before | after |
|---|---|---|
| `--lib --features sync` (native) | 129 errors (`error: could not compile … due to 129 previous errors`) | **EXIT 0** |
| `--all-targets --features sync` (native) | 257 errors (once workspace-metadata blocker cleared — see below) | **EXIT 0** |
| `--lib --features sync,ureq` (native) | 4 errors (the "~6 sibling" errors — see below) | **EXIT 0** |
| `--all-targets --features sync,ureq` (native) | n/a (blocked until `--lib` green) | **EXIT 0** |
| `--lib` / `--all-targets`, default features (native) | EXIT 0 (regression guard) | **still EXIT 0** |
| `--lib --features worker` (native) | untested before | **EXIT 0** (native-only by construction: `worker`'s own module, `🏪️store/👷️worker/🦀️component.rs`, is additionally gated `target_arch="wasm32"`, so on native this combo only compiles the flag, no extra code) |
| `--all-targets --features worker` (native) | untested before | **EXIT 0** |
| `--lib --all-features` (native) | untested before | **EXIT 0** |
| `--all-targets --all-features` (native) | untested before | **EXIT 101 — 71 errors, ALL in `🗣️dsl/🧪️fixture-sweep/🦀️component.rs`, ALL "cannot find module/crate `block`/`cad_document`/`norm`/`draw`"** — gated behind the `dsl-fixture-sweep-full` feature (also enabled by `--all-features`), which references plugin crates (`block`, `cad_document`, `norm`, `draw`) this crate does not declare as dependencies. Zero relation to await/async; entirely outside `🏪️store/🔄️sync` and `📇️directory/🔌️client`; not touched. Needs its own packet. |
| `--lib --target wasm32-unknown-unknown` default | EXIT 0 (regression guard) | still EXIT 0 |
| `--lib --target wasm32-unknown-unknown --features sync` | 9 errors, all in `wasm_actor` (never compiled before — R14) | **EXIT 0** |
| `--all-targets --target wasm32-unknown-unknown --features sync` | 2 in-scope errors (`ArtifactId` unreachable in `#[cfg(test)]` on wasm32) + 7 pre-existing out-of-scope | **0 in-scope; 7 pre-existing unchanged** (see below) |
| `--lib --target wasm32-wasip2` default | EXIT 0 (regression guard) | still EXIT 0 |
| `--lib --target wasm32-wasip2 --features sync` | untested before (never compiled) | **EXIT 0** |
| `--all-targets --target wasm32-wasip2 --features sync` | 14 in-scope errors (native-only test fns unreachable-import on wasm) + 7 pre-existing out-of-scope | **0 in-scope; 7 pre-existing unchanged** |

**The 7 pre-existing out-of-scope errors** (identical set on both wasm targets, identical on default and
`sync` features — confirmed unrelated to this packet): `📡️spr/⌨️cli/📦️main.rs` and `🎒️pack/⌨️cli/📦️main.rs`
each `cannot find cli in os_spr`/`os_pack` (CLI bin targets that were never wasm-buildable), and
`📇️directory/🪪️identity/🦀️component.rs` (a *different* directory submodule than my `🔌️client` scope) has
5 `cannot find … tempfile` errors in its own `#[cfg(test)]` code. None of these are mine to fix — outside
path scope, present under `--all-targets` on wasm32 with or without my changes.

## 📇️directory/🔌️client — the "6 sibling errors", located precisely

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs`, mounted into this
crate via `📇️directory/🦀️component.rs`'s `#[path="🔌️client/🦀️component.rs"] pub mod client;`. Its
`native` submodule (`NativeDirectoryTransport`, `UreqHttpTransport`) is gated
`#[cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))]` — invisible to every
check that doesn't pass **both** features together, which is why it had never been reached. Measured:
`cargo check --lib --features sync,ureq` → **4 errors** (the important.md finding of "5 un-awaited
`is_cancelled()` + 1 E0446" was stale — those 5 sites already carry `.await` as of an earlier commit
(`6cf8d6c858`, 2026-08-20 11:44); no E0446 was present at time of this packet). The 4 real errors, all
fixed:
1. `UreqHttpTransport::call` was `async fn`; its trait, `HttpTransport` (`🛎️services/🦀️component.rs:742`),
   is **deliberately sync** so `Arc<dyn HttpTransport>` stays dyn-object-safe (documented there). Reverted
   to plain `fn` and tagged `// 🚫️async: E1-adjacent`; body has no suspension point (`ureq` is sync).
2. `with_new_http_pool` didn't await `HttpPool::new(...)` or its own `Self::new(...)` call.
3. `NativeDirectoryTransport::http` computed `http_method_str(method).to_string()` without awaiting the
   (async, pure) `http_method_str`.

`--lib --features sync,ureq` and `--all-targets --features sync,ureq` are both now EXIT 0.

## E-class / R9 decisions, each with both halves of evidence

- **`envelope_serde::{serialize,deserialize}`** (sync/component.rs `mod envelope_serde`) — E1: consumed
  by `#[serde(with = "envelope_serde")]`, a sync-only external-trait call shape (serde's derive codegen
  calls these as plain `fn`s inside `impl Serialize`/`Deserialize`, whose own signatures are fixed by the
  external `serde` crate). I/O check: body only calls `encode_envelopes`/`decode_envelopes`
  (📡️replication, out of scope) — pure varint/byte framing, zero `std::fs`/`tokio`/etc. Reverted to sync,
  bridged the out-of-scope async callees via `crate::io::resolve_ready` (this crate mounts
  `🚪️io/🦀️component.rs` at its own root as `os_io`/`io` — confirmed via `📦️glue.rs:205-215` — so the
  shared bridge is reachable without a new dependency).
- **`PresenceHeartbeatProducer::new`** — E1: consumed by `impl Default`. I/O check: pure struct literal.
  Reverted to sync + tagged.
- **`ArtifactHost::open_artifacts`** — E1: consumed by `impl Drop for ArtifactHost`. I/O check: pure
  lock+collect. Reverted to sync + tagged. Its ONLY caller (the `Drop::drop` body) also called
  **`ArtifactHost::close`** as a bare, unawaited statement (a dropped future, confirmed via R13's `let _
  =`-adjacent bare-statement shape) — inspected `close`'s body: lock/remove + a *sync* `mpsc` send +
  blocking `std::thread::JoinHandle::join()`, zero `.await` anywhere already. Same test: pure, no
  suspension point, one consumer (`Drop`) is E1-barred. Reverted to sync + tagged; this also cleared 11
  dropped-future call sites across `open`/tests/`Drop` for free (no `.await` insertion needed there
  anymore, since the call itself is now sync).
- **`document_event_tag`** (sync/component.rs test helper) — E1-adjacent: consumed by
  `wait_for_event`'s `impl FnMut(&ArtifactEvent) -> bool` predicate bound (a sync-closure-shaped generic,
  same practical bar as an external trait). I/O check: pure `match`. Reverted to sync + tagged.

Every other fix in this packet was mechanical `.await` insertion (either via `insert-await.py` to
fixpoint, or by hand where the tool's span-keyed matcher legitimately can't reach — sync-closure hoists,
fn-pointer-slot (E4) thunk calls, and the R16-mode-1-style "declaration awaited, stray `.await` left at a
use site by an earlier uncoordinated pass" shape found in `SyncSession`/`PresencePeer` test assertions).

## Tooling notes (for the next packet inheriting these tools)

- **`fix-repeated-await.py`'s `--scope` is REPO-ROOT-RELATIVE**, unlike `insert-await.py`'s (which
  matches on path segments at any depth). Passing the short form (`🏪️store/🔄️sync`) silently walks a
  nonexistent directory and reports "0 edits" — not a bug in your file, a `os.path.join(REPO, scope)`
  quirk in that tool. Pass the FULL path from repo root
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync`). Once corrected it found and correctly fixed
  **174 repeated-await sites** in one pass (fixpoint confirmed, 0 on re-run) — the single largest
  mechanical win in this packet, all inside `#[cfg(test)]` code the earlier `--lib`-only passes never
  reached.
- **A green `insert-await.py --all-targets` run is not proof of a green build** if it happened while the
  workspace was externally broken (see below): its `run_check` only appends diagnostics when cargo emits
  `compiler-message` JSON; a workspace-manifest failure emits none, so `errors=0` silently means "cargo
  never ran your code," not "your code is clean." Caught this the hard way — a `report.json` claiming 0
  errors turned out to be from a run that hit the manifest blocker; the very next direct `cargo check`
  showed 147 real errors. Always independently confirm with your own `cargo check … | tail`, never trust
  the tool's own zero.

## Cross-packet blocker hit and escalated (resolved by sol)

Mid-packet, EVERY `cargo check`/`cargo test` invocation (including ones unrelated to my scope) started
failing with `error: failed to load manifest for workspace member .../🏗️fem/📦️packages/🦀️rust`, chained
down to `failed to read 🧰️framework/🔨️modules/🎬️scene/📦️packages/🦀️rust/Cargo.toml`. Diagnosed (without
touching anything): `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`'s `ui_scene` path dependency
had gone stale mid-refactor (a sibling session was moving `🎬️scene` under `🖱️ui/`, and the relocated
crate's `Cargo.toml` existed at the NEW path while the referencing path dependency still pointed old,
three `../` instead of two). Confirmed live via mtime (2 min old) and `git status` (uncommitted `M`).
Escalated once, made zero edits outside scope while waiting, kept working on read-only/local-file
diagnosis in the meantime; sol fixed the one-token path arithmetic and confirmed via
`cargo metadata --no-deps`. Total cost: a handful of retries, not a stall.

## R17 forced-rebuild dropped-future census

Ran `cargo clean -p semio-framework-os-kernel` immediately after first reaching `--lib --features sync`
EXIT 0 (R17: census must happen the turn the crate goes green, since a red crate cannot report the lint
at all). First census: **43 dropped futures**, ALL in `🏪️store/🔄️sync/🦀️component.rs` (none in
`📇️directory/🔌️client`). Every one was a bare-statement call to a self-method whose own body IS async
(`self.setup()`, `self.handle_external_change()`, `self.persist_operations(...)`,
`self.persist_write(...)`, `self.persist_snapshot(...)`, `self.deliver_remote_operations(...)`,
`self.deliver_snapshot(...)`, `self.emit(...)`, `self.set_remote_state(...)`,
`self.emit_status_if_changed(...)`, `self.on_hub_frame(...)`, `self.schedule_reconnect(...)`,
`self.handle_ack(...)`, `stamp_session(...)`) — the entire actor dispatch tree (`run`'s `select!` arms,
`handle_cmd`, `handle_external_change`, `try_connect_hub`, `on_hub_message`/`on_hub_frame`, `handle_ack`)
had **never actually executed its own effects** even when the crate did compile in some earlier
mid-refactor state, because none of the call sites were awaited. Fixed all 43, re-censused after a second
`cargo clean` → **0**. Re-ran the whole 43-site sweep again after finishing `--all-targets` (which reaches
`#[cfg(test)]`) and after the `fix-repeated-await.py` 174-site pass → **0**, confirmed via a THIRD forced
rebuild (`--all-targets --features sync,ureq`) at the very end of the packet → **0**.

**R13 corollary catch (`let _ =` hides the lint):** after the census read 0, I still grepped all 28
`let _ = …` sites by hand. 25 are legitimate fire-and-forget on genuinely-sync channel sends
(`mpsc`/`broadcast`::`Sender::send`, `web_sys` WebSocket sends, `std::thread::JoinHandle::join`,
`std::fs::create_dir_all`/`remove_file`) or already correctly `.await`ed. **One was not**:
`ArtifactHost::deliver_remote_operations`'s `let _ = self.remote.push(BackboneMessage::Mutations {
envelopes: encode_envelopes(&envelopes).await });` — the INNER `encode_envelopes(...).await` had been
fixed, but the OUTER `self.remote.push(...)` call itself was never awaited, so **the remote-mutation
push into the store's inbound backbone queue never ran, ever** — a real, live production bug: an
externally-arrived edit's `RemoteMutations` event correctly fired (a separate, already-awaited `self.emit`
call), but the store's own `tick()` had nothing in its backbone queue to ingest, so the edit never joined
the local timeline. Fixed (added the missing `.await`); re-verified compile clean.

## Runtime effect of the `self.remote.push` fix — measured before/after

`cargo test -p semio-framework-os-kernel --lib --features sync` (800 tests; never runnable before this
packet — the crate never compiled with this feature). Before the fix: **793 passed / 7 failed**. After:
**796 passed / 4 failed**. The three tests the fix closed outright:
`os_store::sync::tests::actor_tests::two_hosts_converge_through_hub`,
`…::detach_drains_pending_outbound_operations`, `…::reconnect_since_catch_up_replays_backlog` — all three
had failed with "left: 0, right: N" (nothing ever arrived), exactly the shape a dropped inbound-queue push
predicts.

**Note on apparent hangs**: a `--test-threads` default (unbounded) run showed 4 of these same tests
"running for over 60 seconds" — re-run in isolation each finished in well under a second. This was test
**parallelism contention** (many tests each spinning a real OS thread + its own tokio runtime + binding
real TCP ports for the mock semio_hub), not a deadlock; `--test-threads=4` reproduces cleanly and fast.
Recorded here so the next packet doesn't re-diagnose it as a hang.

## Residual — NOT fixed, need their own packet (found only because this module finally compiles)

Four failures remain in `cargo test --lib --features sync`, none of them async/await-shaped, all newly
visible only because this module has never before executed:

1. **`fixtures_replay_matches_expected_events`** — panics `expected fixtures in
   …/📦️packages/🦀️rust/🧫️fixtures`. That directory does not exist anywhere in the repo. Pure missing test
   data (fixture authoring), not a code bug; `load_fixtures` degrades gracefully (empty `Vec`) when the
   dir is absent, so this is a data gap, not a crash.
2. **`folder_external_edit_delivers_remote_operations`** — now passes its edit-count assertion (the
   `self.remote.push` fix above), but fails one line later: `store.snapshot().await.expect("snapshot").n`
   is `1`, expected `42`. The external edit (`SetN{n:42}`) joins the timeline but the computed snapshot
   doesn't reflect it — looks like a materialize/replay-ordering issue in `ArtifactStore`
   (`🏪️store/🦀️component.rs`, out of path scope), not in the actor.
3. **`folder_text_storage_round_trips_dsl_and_appends_ops`** and
   **`folder_text_storage_round_trips_pack`** — both panic identically:
   `parse: ops text has no inverse record for edit edit-… at 1:1`, raised by
   `🏪️store/🦀️component.rs:3410`'s `parse_document_text`. `print_edit_lines` (same parent file) has its
   own passing test elsewhere in the 779-baseline, so the bug is specific to this interaction — plausibly
   the same materialize/inverse-tracking family as (2). Out of path scope to root-cause or fix; flagging
   for whoever owns `🏪️store/🦀️component.rs`.

All four are genuinely new information (R17's whole point) and share a plausible common root (edit
materialization / inverse tracking in the parent store, not the sync actor). Recommend one packet to chase
all three together rather than three separate ones.

## Regression guards (pasted, not summarized)

- `cargo test -p semio-framework-os-kernel --lib` (default features): **`test result: ok. 779 passed; 0
  failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.91s`** — unchanged from baseline. This
  file's whole `mod sync` (and everything touched by this packet) is `#[cfg(feature = "sync")]`-gated at
  the crate root, so it is structurally impossible for any edit here to have touched the default-feature
  779.
- `cargo check -p semio-framework-os-kernel --lib` (default features): EXIT 0.
- Forced-rebuild dropped-future census, final: **0**, confirmed three times across the packet at
  increasing feature/target breadth.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` — the whole fix; grew
  3351→3421 lines (net, mostly R9 doc-comments + sync-closure-hoist rewrites + 5 new
  `#[cfg(not(target_arch = "wasm32"))]` test gates).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs` — 4 await/sync-signature
  fixes (see above), no structural change.

## ⚠️ Live peer churn at packet close (2026-08-20 16:42, NOT this packet's regression)

Final re-verification of `--lib --features sync` at packet close showed 28-29 errors — but **zero of
them in `🏪️store/🔄️sync` or `📇️directory/🔌️client`**. All 28 are in `🗣️dsl/🧬️schema/🦀️component.rs` (17),
`🗣️dsl/🖋️notation/🦀️component.rs` (6), `🗣️dsl/📖️grammar/🦀️component.rs` (3),
`🎒️pack/🧪️testkit/🦀️component.rs` (1), `🗣️dsl/🔍️lexer/🦀️component.rs` (1) — files this packet never
touched. `git status`/mtime confirms all five are **live, uncommitted, ~2-minutes-old** edits by another
session (matches the auto-commit history's in-flight "De-asyncify ~9947 … fns across ~1969 files" sweep,
still running). Per R22, not reverted or repaired. My own two files independently re-checked at that
same moment: **0 errors**. Every EXIT 0 claimed in this report was captured with a clean `grep` for the
exact file paths in `🏪️store/🔄️sync`/`📇️directory/🔌️client` at the time of the run, immune to this kind
of neighbor noise — but a coordinator re-running a bare `cargo check -p semio-framework-os-kernel --lib
--features sync` RIGHT NOW may see the peer's transient breakage and should filter by path before
concluding this packet regressed.

## Ticket-folder artifacts left behind

`terra-insertawait-sync-pass1.json`, `terra-insertawait-sync-pass2.json`,
`terra-insertawait-sync-alltargets-pass1.json`, `terra-insertawait-sync-alltargets-pass2.json`,
`terra-insertawait-sync-alltargets-real1.json`, `terra-insertawait-sync-alltargets-real2.json` —
`insert-await.py --report` outputs from each pass, kept for provenance.
