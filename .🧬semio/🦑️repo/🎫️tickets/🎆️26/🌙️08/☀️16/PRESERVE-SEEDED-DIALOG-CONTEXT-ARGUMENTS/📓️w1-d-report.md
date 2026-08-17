# Lane 1-D report — Rust directory client + native identity mint/restore (contract §C0/§C2/§C3/§C6)

## Changed files

**New (my lease):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs` — `DirectoryTransport`/
  `DirectoryWsConnection` (the injection seam, `?Send` — see design notes), `DirectoryClient<T>`
  (`spaces`, `space`, `events`, `me`, `mint_session`, `command`, `stream`), `DirectoryStream`
  (auto-reconnect resuming from the last observed `seq`/`headSeq`), native `NativeDirectoryTransport`
  (`ureq` + `tokio-tungstenite`, gated behind this crate's EXISTING `ureq`/`sync` features — no new
  dependency), and a `browser` module (`web_sys` fetch/WebSocket) gated `target_arch = "wasm32"`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️component.rs` — `Identity` (mirrors
  contract §C3 field-for-field), `actor_id`, `IdentityEnv::from_process_env` (`S_HUB_URL`/`S_USER`/
  `S_DATA_DIR`), a native JSON file cache under `S_DATA_DIR/os/🪪️identity.json` (wasm32: documented
  no-op seam), and `mint_or_restore` (restore-then-confirm via `/auth/sessions/me`, 401 falls through
  to mint, any other transport failure degrades to the cached identity `Offline`, no cache + hub down
  → `IdentityError::Unavailable`, never panics).

**Modified (additive mount, my lease):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️component.rs` — two new `#[path]` mounts
  (`client`, `identity`) beside the existing `schema` mount. No existing line touched.

**Modified (peer-leased, surgical — see worker-brief's explicit carve-out):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` — `PersistenceBinding::Hub`
  gained `surface: Option<String>`; `hub_ws_url` gained a `surface: Option<&str>` parameter and
  appends `?surface=` when set; both call sites (native `try_connect_hub`, wasm `WasmActor::connect`)
  and both binding-extraction loops (native `ArtifactActor::new`, wasm `spawn_actor`) pass it through.
  Existing test assertions updated for the new arg (+1 new assertion for the `surface` case). No other
  region touched.

**Modified (collateral, unavoidable — see "Collateral fixes" below):**
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` — `hub_binding` gained a `surface: Option<String>`
  parameter (zero existing callers, confirmed by repo-wide grep — non-breaking).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` —
  `parse_persistence_binding`'s one `PersistenceBinding::Hub{...}` literal gained `surface: None`.
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — `web-sys` feature list gained
  `WebSocket`, `MessageEvent`, `BinaryType`, `Request`, `RequestInit`, `Response`, `Headers` (see
  "Collateral fixes").

## Design notes

- **Transport seam**: `DirectoryTransport`/`DirectoryWsConnection` mirror `🎒️pack/🌐️http`'s
  `RangeTransport` pattern (no concrete HTTP/WS client type in any public signature). Declared
  `#[async_trait::async_trait(?Send)]` with NO `Send`/`Sync` supertrait — discovered while wiring the
  browser transport that `web_sys`/`wasm_bindgen` handles are never `Send`, and `🏪️store/🔄️sync`'s own
  native actor sidesteps the identical problem by running a CURRENT-THREAD tokio runtime rather than a
  multi-threaded one. Dropping the bound uniformly (rather than forking the trait definition per
  target) keeps one trait for both native and wasm32, at the cost of callers needing a
  single-threaded/current-thread executor if they ever want to hold a `DirectoryClient` across
  threads — matches the existing precedent's own constraint, not a new one.
- **`DirectoryClient` methods** — exactly the seven asked for. `command`/`mint_session` build the
  request body as `serde_json::Value`/typed structs, matching contract §C2 wire shapes literally,
  including the one documented exception: `POST /auth/sessions`'s response is snake_case
  (`SessionMintResponse{token,user_id}`) because that route predates this wave and its hub handler
  (`🌎️hub/📦️bin.rs::CreateAuthSessionResponse`) has no `rename_all`; everything else is camelCase.
  **Cross-checked against lane 1-C's independently-built TS `DirectoryClient`** (1-C's own
  `📓️w1-c-report.md`: "1-D's `SessionView`/`SessionMintResponse` wire shapes … and
  `HUB_RECONNECT_MIN_MS`/`MAX_MS` (500/30_000) match what I independently built … cross-checked
  field-for-field, no drift") — both sides genuinely agree on the wire, arrived at independently.
- **`DirectoryStream`** tracks the highest `seq`/`headSeq` it has actually observed (not the caller's
  original `since`) and reconnects from there; it owns no timer — `recv()` returns
  `Reconnecting{after_ms}` and expects the caller to sleep before calling again, same division of
  labor `🏪️store/🔄️sync`'s actor already uses (it owns its own `tokio::time::interval`). Backoff
  constants (500 ms / 30 000 ms, doubling) match `🟦️backbone-worker.ts`'s existing
  `HUB_RECONNECT_MIN_MS`/`MAX_MS` exactly (contract requires no drift between the two document-WS
  reconnect policies).
- **Identity is intentionally NOT the CQRS `os.config.identity` facet.** That facet
  (`💻️os/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️component.ts`) is lane 1-C's, self-contained per
  their own report, and is the persisted-local-only op log the **browser/React** shell folds. The
  wgpu **native** shell reads `S_*` env vars directly instead (confirmed in `📓️scout-client.md` §4/§7)
  — it has no config-op-log reader wired to it, so it needs a plain restore-or-mint-then-cache helper,
  which is what `🪪️identity` is. The two `Identity` shapes are structurally identical (field-for-field,
  cross-checked, see above) but are two separate Rust/TS declarations by design — `💻️os/🎚️config/**`
  is peer-leased to 1-C and I never touched it.
- **`hub_binding`'s new `surface` parameter has zero wiring today** — no lane has called it yet (C6's
  actual "bind `[hub{...,surface}, folder{...}]` on `openDocument`" wiring is explicitly 2-C/2-D's
  task, not mine). Flagging so 2-C/2-D know the parameter exists and is ready to use.

## Collateral fixes (found while making the ONE surgical `🏪️store/🔄️sync` change)

Adding `surface` to `PersistenceBinding::Hub` is a Rust struct-literal change: every existing
construction site needs a value for the new field (serde's `#[serde(default)]` only helps
*deserialization*, not Rust struct literals). A repo-wide grep for `PersistenceBinding::Hub {`
turned up two struct-literal sites OUTSIDE `🏪️store/🔄️sync` itself that would otherwise fail to
compile:
1. `🖥️host/🦀️component.rs`'s `hub_binding` helper (zero callers today — extended its signature
   directly rather than defaulting `None` internally, so 2-C/2-D get the real hook).
2. The wgpu shell's `parse_persistence_binding` (`🔨️modules/📺️renderer/…/Shell/🧊️component.rs`) — one
   literal, `surface: None` (this fn's own doc says it's the deprecated manual-override path;
   contract §C6 wiring is out of scope for me).

Both are one-line-per-site, mechanical, and unavoidable — the alternative was leaving the whole crate
red for every peer session touching either file. Re-read each file immediately before editing;
`git log --date=iso` showed both quiet (`🏪️store/🔄️sync` last touched 2026-08-16 20:26, the wgpu
Shell file last touched 2026-08-16 02:50 — both well over 30 minutes idle at edit time).

**`web-sys` feature list** (`💻️os/📦️packages/🦀️rust/Cargo.toml`): while wiring the browser transport I
found the crate's declared `web-sys` features (`Window`, `Storage`, `console`) do NOT include
`WebSocket`/`MessageEvent`/`BinaryType`, even though `🏪️store/🔄️sync`'s own `wasm_actor` module
already uses exactly those types. Root cause: `wasm_actor` lives inside `#[cfg(feature = "sync")]
pub mod sync;` in `📦️glue.rs` — it has NEVER actually been compiled for `wasm32`, because the `sync`
feature also unconditionally requires `tokio/net` (→ `mio`), which does not support `wasm32` at all
(confirmed: `cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --features
sync` fails with 36 `mio` errors, unrelated to anything in this ticket). `wasm_actor`'s `web_sys`
usage is therefore latent, unverified dead code today, not a currently-working path I broke. I added
the missing features (`WebSocket`, `MessageEvent`, `BinaryType`, `Request`, `RequestInit`,
`Response`, `Headers`) so my OWN browser transport (gated only on `target_arch = "wasm32"`, not on
`sync`) actually compiles — this is a strict, additive feature-list extension on an existing
required (non-optional) wasm32 dependency; it cannot regress any other build. `git log --date=iso`
on the Cargo.toml showed it last touched 2026-08-14, quiet.

## sharedFileRequest — 📌️important.md is stale for this ticket's own store touch

`📌️important.md`'s "What we never touch" section currently lists `🏪️store/**` as untouched by this
ticket — but my worker-brief explicitly authorized the ONE surgical `🏪️store/🔄️sync` change above
(with re-read-before-edit / git-log-recency guardrails), which I made. Per the worker brief I must
not edit `📌️important.md` myself. **Coordinator: please add a line under "What we are adding"**
noting the `🏪️store/🔄️sync` `PersistenceBinding::Hub.surface` / `hub_ws_url` touch, so
`MUTATION-OUTCOMES` (which owns `🏪️store/**`) sees it in the shared notice rather than only in this
report.

## Commands run + results

**Methodology note**: my very first `cargo check -p semio-framework-os-kernel` (default features)
was launched BEFORE the `🏪️store/🔄️sync` surgical edit + collateral fixes were made (it ran
concurrently while I was still editing), so its "GREEN" result could have raced a stale file state.
I re-ran it (and every other check) AFTER all edits settled — those final re-runs are what's
reported below; the possibly-stale first run's raw log is kept as `🧪️1-d-cargo-check-default.txt`
for the record but is NOT what I'm citing as evidence.

### `cargo check -p semio-framework-os-kernel` (default features — the required gate)
**GREEN**, re-run after all edits settled. 9 pre-existing warnings (`js` cfg ×2, `unnecessary
qualification` ×5, `unused_variables` ×1, `unused_mut` ×1 — same set lane 0-A's baseline recorded;
`grep` confirms zero warnings/errors from `📇️directory/🔌️client` or `📇️directory/🪪️identity`), 0
errors. `Finished dev profile … in 3m 45s`. Full log: `🧪️1-d-cargo-check-default-final.txt`.

### `cargo check -p semio-framework-os` (the shell crate, per Verify's "make sure the store change compiles for the shell")
**GREEN**. 8 pre-existing warnings (unused extern crate ×4, unused import ×2, unused `#![feature]`
attribute ×2 — all pre-existing, unrelated to this lane), 0 errors. `Finished dev profile … in 7m
01s`. Full log: `🧪️1-d-cargo-check-os-shell.txt`.

### `cargo check -p semio-framework-os-kernel --features ureq,sync` (extra — proves `NativeDirectoryTransport` itself typechecks; not in the required Verify list, since `ureq`/`sync` are off by default)
First attempt: 2 real errors — `tokio-tungstenite` 0.26.2's `Message::Text` wraps `Utf8Bytes`, not
`String` (a pinned-version API detail; fixed with `.into()` on send / `.to_string()` on recv, exactly
the compiler's own suggested fix). Second attempt: **GREEN** — 0 errors, pre-existing warnings only
(including 9 `unnecessary qualification` warnings already present in `🏪️store/🔄️sync` before this
ticket, per lane 0-A/0-B's baseline — none from my new files). `Finished dev profile … in 6m 05s`.
Full log: `🧪️1-d-cargo-check-native-features-fix2.txt`.

### `cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown` (extra — proves the browser transport typechecks; wasm32 is not in the required Verify list either)
First attempt: 3 real errors — missing `web-sys` features (see "Collateral fixes"). Second attempt
(full crate, all targets): the **lib** compiled clean (`warning: semio-framework-os-kernel (lib)
generated 9 warnings` — no errors), but the crate's `[[bin]] name = "spr"` / `name = "pack"` targets
then failed: `cannot find cli in os_spr` / `cannot find cli in os_pack"`. Root cause, confirmed by
reading the error: both bins' `main.rs` call `…::os_spr::cli::main_impl`/`…::os_pack::cli::main_impl`,
but `📦️glue.rs` mounts BOTH `cli` modules behind `#[cfg(not(target_arch = "wasm32"))]` — i.e. these
two native-only CLI binaries were never buildable for `wasm32` in the first place, unrelated to
anything in this ticket (I did not touch `📦️glue.rs`, the `cli` modules, or the `[[bin]]` sections).
Isolated with `--lib` (skips the two native-only bins) to confirm: **GREEN**, 0 errors — see
`🧪️1-d-cargo-check-wasm32-lib-only.txt`. Full un-isolated log (bin failures included, for the
record): `🧪️1-d-cargo-check-wasm32-fix2.txt`.

### `cargo test -p semio-framework-os-kernel --lib os_directory::client` / `os_directory::identity`
Ran after all four checks above confirmed clean. Real counts (`🧪️1-d-cargo-test.txt`,
`🧪️1-d-cargo-test-client-fix.txt`):

- **Brief's literal filter `directory_client`**: `running 0 tests … 0 passed; 0 failed … 971 filtered
  out`. As predicted before running it — my modules are `os_directory::client`/`os_directory::
  identity` (siblings of lane 0-A's already-landed `os_directory::schema`, same naming convention),
  so the bare substring `directory_client` (no `::`) never occurs in any fully-qualified test path
  here. Not a bug; just the wrong filter for this module layout.
- **Brief's literal filter `identity`**: `running 16 tests … 16 passed; 0 failed … 955 filtered out`
  — matches (superset of) my 6 identity tests plus 10 unrelated pre-existing tests elsewhere in the
  crate whose names happen to contain "identity"; all 16 pass.
- **Real path `os_directory::client`**: first run — **1 real failure**,
  `stream_reconnects_and_resumes_from_last_seq`: `assertion left == right failed: … left: 2 right: 3`.
  This was a bug in my OWN test, not the implementation: `DirectoryStream::recv()` reports
  `Reconnecting` the moment a connection closes but does NOT eagerly re-dial in the same call (by
  design — the stream owns no timer, the caller is expected to sleep `after_ms` then call `recv()`
  again, exactly like `🏪️store/🔄️sync`'s own actor with its `tokio::time::interval`); my test only
  drove 3 `recv()` calls and asserted 3 dials, but the 3rd dial only happens on a 4th call. Fixed by
  adding the 4th `recv()` call and asserting the correct (now-advanced) backoff value on it. Re-run:
  `running 5 tests … 5 passed; 0 failed … 966 filtered out`.
- **Real path `os_directory::identity`**: `running 6 tests … 6 passed; 0 failed … 965 filtered out` —
  clean on the first run, no fixes needed.

**Net: 11/11 of my own directory-client/identity tests pass** (5 + 6), after fixing one genuine
test-authoring bug I found and fixed myself (not shipped broken).

### `cargo test -p semio-framework-os-kernel --lib os_directory` (holistic — whole module tree, my regions plus lane 0-A's)
**16/16 pass**: 0-A's `schema` (3) + `fold` (2), my `client` (5) + `identity` (6). Zero regressions
against 0-A's pre-existing tests. Full log: `🧪️1-d-cargo-test-os-directory-full.txt`.

### `cargo test -p semio-framework-os-kernel --lib --features sync os_store::sync` (extra — the `🏪️store/🔄️sync` file I surgically edited; NOT in the required Verify list, attempted for my own diligence since I touched peer-leased test assertions there)
**Did not complete in this session** — launched, but `ps` shows it sitting at ~0 s CPU time (blocked
on the shared `target/` build lock; this machine had 16 concurrent `rustc`/`cargo` processes from
other live sessions at the time, matching this repo's known "concurrent cargo workspace churn"
pattern). I am NOT claiming this passes — I did not observe it finish. What IS verified, and is
sufficient evidence the edit is sound: (1) `cargo check -p semio-framework-os-kernel --features
ureq,sync` compiled the `🏪️store/🔄️sync` file (including its edited `hub_ws_url` signature and the
`PersistenceBinding::Hub` literal I touched in its test module) with zero errors — `cargo check`
does not run `#[cfg(test)]` assertions, but it DOES type-check them, so a signature mismatch in my
edited test literals would already have failed there; (2) I hand-traced the one assertion I added
(`hub_ws_url("remote://host:6070", "studio-1", "doc-1", Some("s.space.home@1/*#editor"))` against the
function body) and it matches byte-for-byte. If this background run finishes before the ticket
closes, its log is at `🧪️1-d-cargo-test-sync-module.txt` — re-check it before relying on this claim
being complete.

## What is NOT done

- The browser (`wasm32`) `DirectoryTransport`/identity cache are compiled-and-typechecked only, never
  runtime-tested (no wasm32 test harness in this lease) — consistent with them being a documented
  seam, not the production browser path (that's 1-C's TS `DirectoryClient`, already live).
- No caller wiring: nothing in the wgpu shell or React shell constructs a `DirectoryClient` or calls
  `mint_or_restore` yet — that's lanes 2-C/2-D's task per `ownership-and-handoffs.md` §C
  ("1-C/1-D → 2-C/2-D: `DirectoryClient` + identity fold + `PersistenceBinding.hub.surface`").
- `hub_binding`'s new `surface` param is plumbed but unused by any caller (see design notes).
- The full `🏪️store/🔄️sync` test suite (under `--features sync`) never finished running in this
  session (shared-machine lock contention) — see the honest status above under "Commands run". If a
  later session re-runs it and finds a real failure in `hub_ws_url_derives_ws_endpoint_from_remote_uri`
  or any `PersistenceBinding::Hub` literal, that is now my bug to fix, not a false-green claim.
