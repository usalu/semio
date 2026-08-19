# 📓️ terra — packet `dedyn-fw-os-hostasync`

## Scope

Owned writable paths: `🧰️framework/🛍️products/💻️os/**`, scoped ONLY to the `HostAsyncRuntime` family
(all other families in `💻️os` belong to siblings; `🔌️plugin/🖥️host/**`, `🛢️db/**` etc. are completed
packets, read-only for shape reference).

## Counts

Verified with two independently-implemented searches (python3 `re.findall` over explicit absolute
paths, and `grep -rn`/`awk` as a second implementation), comments excluded, both agreeing:

| file | `dyn HostAsyncRuntime` before | after |
|---|---:|---:|
| `📇️directory/🔌️client/🦀️component.rs` | 3 | **0** |
| `🛎️services/🦀️component.rs` | 20 | **0** |
| **total (my owned files)** | **23** | **0** |

The packet brief's "~33" estimate included `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` (10 uses),
which is explicitly listed as NOT MINE (`🔌️plugin/🖥️host/**` is a sibling's completed-packet path) —
confirmed still present there, untouched, for the owning packet. `📺️renderer/…/Shell/🧊️component.rs`
has one *comment* mentioning `Arc<dyn HostAsyncRuntime>` (explaining why `directory_runtime` is kept
concrete, not that type) — not a code use, left as-is since it remains accurate.

## Mechanism: GENERICS (R11 case "open set"), not an enum — per the packet brief

`HostAsyncRuntime`'s two impls (`TokioHostRuntime` in `🛎️services`, `ManualRuntime` test mock in
`semio-framework-async::testkit`, plus `InlineRuntime` in `🛢️db`) live ABOVE or BESIDE the trait's own
crate (`⏳️async`), so a closed-set enum in `⏳️async` could not name them without dragging tokio
downward. Every holder of `Arc<dyn HostAsyncRuntime>` became a generic `Arc<R: HostAsyncRuntime>` (or
a borrowed `&R` for per-call parameters — R11(a), trivially generic). This matches the shape already
adopted by the completed `🛢️db` packet (`DbBackend<R: HostAsyncRuntime>`, `FsStorage<R>`,
`SqliteStorage<R>`), read for reference, not re-edited.

### `📇️directory/🔌️client/🦀️component.rs`
- `NativeDirectoryTransport` struct + its `impl`/`impl DirectoryTransport` block → generic over
  `R: HostAsyncRuntime` (`+ 'static` on the trait impl, since `DirectoryTransport`'s async-trait
  desugaring boxes the future). Zero other callers of `NativeDirectoryTransport::new` /
  `with_new_http_pool` exist anywhere in the repo (verified: repo-wide grep for
  `BlockingHttpTransport::new`-style callers came back empty for this type too), so there was no
  fan-out beyond the one file.

### `🛎️services/🦀️component.rs`
- **Per-call borrowed parameters (R11a, trivially generic):** `TimerWheel::spawn_driver`,
  `ComputePool::run_blocking`, `HttpPool::spawn_refill_driver`, `HttpPool::fetch`, `HttpPool::request`
  — each gained a `<R: HostAsyncRuntime (+ 'static where the fn boxes a future)>` type parameter.
- **Struct fields holding `Arc<dyn HostAsyncRuntime>` (genuinely needed generics, not just a
  borrow):** `BlockingHttpTransport`, `StorageState`/`StorageScheduler`/`StorageTicket`
  (plus the free fn `storage_try_dispatch`, which takes `&Arc<StorageState<R>>`). Necessity confirmed
  by evidence, not assumption: this crate's own test suite plugs BOTH `TokioHostRuntime` (storage/HTTP
  streaming tests) and `ManualRuntime` (timer-driver/refill-driver tests, which need synchronous,
  clock-injectable execution) into the *same* generic code paths — so this is a real open set inside
  one crate, not an "exactly-one-impl" case.
- **Test-only structs (`LocalSocketBody`/`LocalSocketTransport`, inside `#[cfg(test)] mod tests`):**
  used the R11 "exactly one impl in this local context" branch instead — both are constructed only
  with `TokioHostRuntime` in this crate's tests, so the field became the concrete
  `Arc<TokioHostRuntime>` rather than an unnecessary extra generic parameter on a private test helper.
- Bare `Arc<dyn HostAsyncRuntime>` local-variable type annotations and `as Arc<dyn HostAsyncRuntime>`
  upcasts in tests (10 sites) were simply deleted — type inference already resolves them to the
  concrete `Arc<TokioHostRuntime>` / `Arc<ManualRuntime>` once the callees are generic.

## Consumer fan-out (one hop, required to keep the tree compiling)

`NativeDirectoryTransport` going generic broke 3 bare references in
`📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` (`directory_client: Option<DirectoryClient<NativeDirectoryTransport>>`,
`directory_transport: NativeDirectoryTransport`, and one tuple-type-annotated `let`). All three
already only ever construct/hold it against the ONE concrete `TokioHostRuntime` the shell mints once
(see that file's own doc at `directory_runtime: Arc<TokioHostRuntime>` — deliberately concrete, not
the trait object, for its own `?Send`/`block_on` reasons unrelated to this packet), so all three became
`NativeDirectoryTransport<TokioHostRuntime>`. This is not a different family — it is exactly the
"the type parameter threads through its holders" consequence R11 describes, one hop, no new
public-type fan-out beyond the 3 sites, well under R11's ~10-type stop-and-report threshold.

## Macro friction

None — `dyn_enum_close!` was never invoked for this family; the packet brief already ruled generics,
and reading `📓️terra-dyn-enum-macro-report.md` confirmed the macro cannot apply here anyway (open,
cross-layer implementor set).

## `#![allow(async_fn_in_trait)]`

Not added: neither `🛎️services` nor the `semio-framework-os-kernel` crate (which owns
`📇️directory/🔌️client`) declares a first-party trait with an `async fn` method as part of this
packet's edits — `HostAsyncRuntime` itself is declared (and presumably already allow-listed) in
`⏳️async`, out of scope here. `DirectoryTransport`/`AsyncHttpTransport` etc. use
`#[async_trait::async_trait]` or plain `fn -> HostFuture<_>`, neither of which trips this lint, and
neither is part of the `HostAsyncRuntime` family this packet owns.

## Acceptance — UNRUN, blocking crate named

`CARGO_TARGET_DIR=…/scratchpad/target-dedyn-os-hostasync cargo check -p semio-framework-os-services --lib`
→ **exit 101**, but **zero errors reference `🛎️services` or `📇️directory`** — all 266 errors are in
the dependency `semio-framework-actor` (`fn` → `async fn` conversion mid-flight there, e.g.
`self.actor_metrics_samples()` returning an unawaited future into a struct-literal field, `Display`/`Debug`
impls receiving `impl Future` instead of the value). Full output saved in the ticket folder:
`terra-dedyn-os-hostasync-services-check.txt`.

`CARGO_TARGET_DIR=…/scratchpad/target-dedyn-os-hostasync cargo check -p semio-framework-os-kernel --lib --features ureq,sync`
→ **exit 101**, same story: 278 errors, **zero in `📇️directory`**, all in `semio-framework-actor`
again (`semio-framework-os-kernel` pulls it in transitively). Full output saved:
`terra-dedyn-os-hostasync-kernel-check.txt`.

Both commands run in the foreground, single turn, with explicit `CARGO_TARGET_DIR` under the session
scratchpad (not the ticket folder — rule 24) and no `--workspace`. This matches the ticket's documented
"COMPILE REALITY": the guest SDK / actor gate is not yet green, so a real build cannot reach my source.
Verified with grep that my two touched files contribute **zero** lines to either error log.

## For siblings

- `🛢️db`'s existing `R: HostAsyncRuntime` shape (bounds with no extra `+ Send`, since the trait itself
  is `Send + Sync`) is the right template — I followed it exactly; no new idiom introduced.
- `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` still has 10 `dyn HostAsyncRuntime` uses (lines 86, 549,
  975, 1012, 1024, 1038, 1089, 1211, 1225, 1237 as of this report) — confirmed out of my scope, left
  untouched for its owning packet.
- Nothing in `HostAsyncRuntime`'s own shape needed changing; it was already R1-legal
  (`spawn_scoped`'s `HostFuture<()>` argument is the one sanctioned erased-Future channel, R3).
- No `lease-request` needed — no shared/registrar file was touched.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
  (3 consumer-side type-parameter fixups only, per "Consumer fan-out" above)
- This report and two scratch logs in the ticket folder:
  `terra-dedyn-os-hostasync-services-check.txt`, `terra-dedyn-os-hostasync-kernel-check.txt`.
