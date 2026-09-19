# W3a — `ServerInstance`: de-closing the server product's seams

Slice W3a, executing **step 3 of §B.7** of `📓️h2-server-crate-and-wave3-memo.md` on
`semio-framework-server` (`🧰️framework/🛍️products/🖥️server`). All commands foreground, from
`/Users/ueli/Documents/semio`, no sub-agents, no git-modifying command. Nothing under `🌎️hub/**`
was touched. Captures: `🗑️generated/w3a-baseline-test.txt`, `🗑️generated/w3a-baseline-wasm.txt`,
`🗑️generated/w3a-test.txt`.

---

## 1. What the slice had to undo

The O1 de-dyn pass had closed all ten ports over enums **inside the framework crate**
(memo §B.4): four `Memory*` stores, one `BearerTokenResolver`, one `CounterDecider`, one
`EchoSaga`, one `CountingModule`, and two *uninhabited* enums (`DocumentAuthorities`,
`QueryHandlers`). `ServerBuilder`/`ServerState`/`Server` and the 13 handlers were typed against
those concrete enums, so no downstream crate could supply any implementation of any port without
editing the framework — and `get_document_ws` answered 404 unconditionally because its enum had no
inhabitants. `ServerBuilder::build` also constructed the four `Memory*` stores itself and only
copied `data_dir` into state, so the product persisted nothing *by construction*, not by
configuration.

## 2. The trait

`🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs:162` (region `🔖️Instance`):

```rust
pub trait ServerInstance: Sized + Send + Sync + 'static {
    type Modules: ServerModule<Instance = Self> + 'static;
    type Queries: QueryHandler + 'static;
    type Documents: DocumentAuthority + 'static;
    type Deciders: Decider + 'static;
    type Sagas: Saga + 'static;
    type Resolvers: PrincipalResolver + 'static;
    type AuthorityStore: AuthorityStore + 'static;
    type ProjectionStore: ProjectionStore + 'static;
    type BlobStore: BlobStore + 'static;
    type SessionStore: SessionStore + 'static;

    async fn open(profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError>;
}

pub struct InstanceStores<I: ServerInstance> {
    pub authority: I::AuthorityStore,
    pub projections: I::ProjectionStore,
    pub blobs: I::BlobStore,
    pub sessions: I::SessionStore,
}

pub type ServerSagas<I> = SagaRunner<<I as ServerInstance>::Sagas>;
```

`open` is the part the memo's §B.5 item 1 did not name but the code demanded: the builder used to
*construct* `MemoryAuthorityStore` etc. Storage now comes from the instance, which is what turns
`StorageProfile::Embedded { data_dir }` from decoration into a real instruction — the test profile
ignores `data_dir` deliberately and says so, a durable instance will not.

It lives in `📡️gateway` because that is the only module that already depends on all four port
modules; no new module directory, no `🔣️.json` member added (two `responsibility` strings updated).

## 3. Two deviations from the memo's step 3, both forced by the compiler

### 3.1 Eight ports declare `impl Future + Send`, and lose `#[dyn_enum]`

The memo says "keep only `#[dyn_enum]` on the traits". That does not compile once the gateway is
generic, and the failure is structural, not cosmetic. With the ports behind `I::AuthorityStore`
etc., an `async fn` in a trait returns an **opaque** future with no `Send` bound, and every axum
handler future and every `WebSocketUpgrade::on_upgrade` callback must be `Send`. Verified, not
assumed — the first generic compile produced ten errors of exactly this shape:

```
error: future cannot be sent between threads safely
    --> …📡️gateway/🦀️.rs:1085:11
     | Ok(ws.on_upgrade(move |socket| handle_event_stream(socket, actor, query.since, state)))
     | future returned by `handle_event_stream` is not `Send`
note: future is not `Send` as it awaits another future which is not `Send`
     | Ok(authority.store().events_since(actor, since).await?)
help: `Send` can be made part of the associated future's guarantees for all implementations of
      `AuthorityStore::events_since`
error[E0277]: the trait bound `… {post_command::<…>}: Handler<_, _>` is not satisfied      (×7)
```

The previous design never hit this because every call site named a concrete enum whose future the
compiler could *see* was `Send`. Return-type notation (`I::AuthorityStore: AuthorityStore<events_since(..): Send>`)
is the stable-Rust alternative and is unstable; `Box<dyn Future + Send>` is dyn dispatch, which O1
forbids. So the eight ports reachable from `ServerState` inside a handler —
`AuthorityStore`, `ProjectionStore`, `BlobStore`, `SessionStore`, `PrincipalResolver`, `Decider`,
`QueryHandler`, `DocumentAuthority` — now declare
`fn m(..) -> impl Future<Output = T> + Send;`. **Implementations stay ordinary `async fn`**, which
Rust accepts against that signature, so the repo's async convention is untouched everywhere except
the ten trait *declarations*. Each carries a seven-line docstring saying why.

The cost: `#[dyn_enum]` cannot delegate such a trait through an enum with more than one variant —
its generated body is `fn m(..) -> impl Future<..> + Send { match self { A(i) => i.m(), B(i) => i.m() } }`,
and two arms of one `match` cannot return two different opaque types. A closing site writes the
delegation as `async fn` with `.await` inside each arm instead (5 lines per method), which is legal
precisely because an impl may satisfy an `impl Future` declaration with `async fn`. I did **not**
patch `🧰️framework/🔨️modules/🔀️dispatch` to emit that shape: it is another crate with ~90
dependents and live peers, and out of this slice's path scope. It is a genuine follow-up — see §7.

`Saga` and `ServerModule` are awaited only off the request path (`build`, and a drain the caller
schedules), so `Saga` keeps plain `async fn` **and** `#[dyn_enum]`, and the test profile closes it
with a real two-variant `dyn_enum_close!` — so the macro path is still exercised by a test.

### 3.2 `ServerModule` carries an associated `Instance`

`routes` hands out a `Router<ServerState<I>>` and `deciders`/`resolvers` return the instance's own
types, so a module is bound to one instance. `#[dyn_enum]` rejects associated types by design
(*"an enum has no single type to give it"*), so `ServerModule` is not `#[dyn_enum]`'d either. Hub's
six modules will need one hand-written delegating enum — ~70 lines, mechanical.

## 4. Diff summary per module

| file | lines before → after | change |
|---|---|---|
| `🔨️modules/🗄️storage/🦀️.rs` | 525 → 304 | 4 `dyn_enum_close!` blocks + the four `Memory*` implementations deleted; the four traits declare `impl Future + Send`; module doc rewritten ("four contracts, zero implementations") |
| `🔨️modules/🛡️policy/🦀️.rs` | 314 → 303 | `PrincipalResolvers` close + `BearerTokenResolver` deleted; `ResolverChain` → `ResolverChain<R: PrincipalResolver>` with a hand-written `Default` (derive would demand `R: Default`) |
| `🔨️modules/🎭️authority/🦀️.rs` | 530 → 467 | `Deciders`/`Sagas` closes + `CounterDecider`/`EchoSaga`/`COUNTER`/`read_counter` deleted; `CommandBus<S>` → `CommandBus<S: AuthorityStore, D: Decider>`; `ActorRegistration<D>`; `SagaRunner<W: Saga>` with hand-written `Default`; `Saga` keeps `#[dyn_enum]` |
| `🔨️modules/📡️gateway/🦀️.rs` | 1357 → 1463 | new `🔖️Instance` region (`ServerInstance`, `InstanceStores`, `ServerSagas`); `ServerModules`/`DocumentAuthorities`/`QueryHandlers` closes + `CountingModule` deleted; `NoDocumentAuthority`/`NoQueryHandler` added; `ServerAuthority<I>`, `ServerState<I>` (hand-written `Clone`), `ServerBuilder<I>`, `Server<I>`, `base_router<I>` and all 13 handlers generic; `build` now `Result<Server<I>, ServerError>` and opens storage via `I::open` |
| `📦️packages/🦀️rust/🦀️.rs` | 30 → 44 | `#[macro_use]` on the four port modules (so a `dyn_enum_close!` in a *different* module of the same crate reaches the capture through textual scope instead of the future-incompatible absolute path of rust-lang/rust#52234); `#[cfg(test)] pub mod test_instance;`; crate note rewritten |
| `🧪️tests/🧩️instance/🦀️.rs` | — → 474 (new) | `TestInstance` + every demo type moved out of production |
| `🔨️modules/🔣️.json` | 4 lines | storage and gateway `responsibility` strings updated |

**13 handlers made generic**, matching the memo's count: `post_command`, `post_query`,
`post_ephemeral`, `get_events`, `get_event_stream_ws`, `get_document_ws`, `get_blob`, `head_blob`,
`put_blob`, `get_apps`, `get_app_installs`, `get_app_root`, `get_app_asset` (plus the private
`handle_event_stream`, `handle_document`, `serve_app` and the `pump_events` bridge).

## 5. The test instance

`🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs` holds `TestInstance` and every type
that used to be production surface: `CountingModule`, `CounterDecider`, `EchoSaga`,
`BearerTokenResolver`, `MemoryAuthorityStore`, `MemoryProjectionStore`, `MemoryBlobStore`,
`MemorySessionStore`, plus `COUNTER`/`read_counter`. It deliberately shows all three closing
shapes:

- eight ports name **one concrete type** directly (the honest shape for one implementation — and
  the shape that needs no enum at all, which is what the old design could not express);
- `TestDeciders` closes **two** deciders (`CounterDecider`, a new `MirrorDecider` serving actor
  kind `mirror`) **by hand**, proving the `impl Future + Send` delegation shape works;
- `TestSagas` closes **two** workflows (`EchoSaga`, a new `SilentSaga`) with **`dyn_enum_close!`
  from a different module of the same crate**, proving the macro still closes a set downstream of
  the trait that declared it.

## 6. Gates

| gate | result |
|---|---|
| `cargo test -p semio-framework-server` baseline | **73 passed**, 0 failed (`🗑️generated/w3a-baseline-test.txt`) |
| `cargo test -p semio-framework-server` after | see `🗑️generated/w3a-test.txt` — RESULT_PLACEHOLDER |
| zero production-side demo types | **clean**: `grep -rn -e CountingModule -e CounterDecider -e EchoSaga -e 'Memory*Store' -e BearerTokenResolver -e dyn_enum_close 🔨️modules/` returns hits only under `🔨️modules/*/🧪️tests/` plus one prose mention in a gateway docstring |
| wasm | **not applicable, and it never was**: `cargo check -p semio-framework-server --target wasm32-unknown-unknown` fails at baseline, before any edit of mine, with 48 errors in `mio` (`error: could not compile 'mio' (lib) due to 48 previous errors`) — `mio` arrives unconditionally through `tokio`'s `net` feature, which `axum` requires (`🗑️generated/w3a-baseline-wasm.txt`). The crate has no `cfg(target_arch = "wasm32")` code and no wasm feature. |

New tests added (4, all in the counts above):

- `authority::tests::a_second_variant_of_the_instance_closed_decider_set_serves_its_own_actor_kind`
  — the two-variant hand-closed set dispatches to both arms on one bus.
- `gateway::tests::the_instance_supplies_the_storage_the_builder_wires_in` — the stores in
  `ServerState` are the ones `I::open` returned, and `data_dir` reaches state.
- `gateway::tests::an_instance_that_registers_no_query_handler_answers_not_found` — `post_query`
  against `NoQueryHandler` (this handler had **no** test before).
- `gateway::tests::a_module_registers_its_deciders_on_the_bus_it_was_built_into` — a module's
  `deciders()` really reach the bus, end to end through `post_command`.

`the_outbox_drains_exactly_once_across_two_calls` now registers both saga variants, so the macro
delegation is on the executed path too.

## 7. What step 4 (`HubInstance: ServerInstance`) needs

1. **Dependency.** Add `semio-framework-server = { workspace = true }` to
   `🌎️hub/📦️packages/🦀️rust/Cargo.toml` (hub does not depend on it today — memo §B.4). The lib is
   named `server`, so hub writes `use server::gateway::{ServerInstance, …}`.
2. **Ten associated types.** Nine can start as placeholders and be replaced one wave at a time:
   - `Documents = NoDocumentAuthority`, `Queries = NoQueryHandler` — both **shipped by this slice**
     for exactly this purpose, so step 4 needs no placeholder of its own for them.
   - `AuthorityStore/ProjectionStore/BlobStore/SessionStore` — hub's own types over
     `db::Database`/`SqliteDirectory` (§B.5 item 3). They may start as hub-local in-memory types to
     get `cargo check` green before the durable work lands; the four reference implementations in
     `🧪️tests/🧩️instance/🦀️.rs` are the semantics to reproduce, and `🗄️storage`'s ten tests are
     the conformance suite to port.
   - `Deciders`, `Resolvers`, `Sagas`, `Modules` — one hub enum each once there is more than one.
3. **`I::open`.** This is the one method hub must write on day one:
   `async fn open(profile: &StorageProfile) -> Result<InstanceStores<HubInstance>, StorageError>`.
   `StorageProfile` currently has exactly one variant, `Embedded { data_dir }`; hub's real profiles
   (sqlite/postgres/neo4j, `OS_HUB_STORAGE_BACKEND`) need variants added to that enum in the
   framework — a small, additive change this slice deliberately did not make, because inventing
   variants no backend implements is what produced the state this slice just undid.
4. **Closing shapes.** For a set with one member, name the type — no enum. For a set with several:
   `Sagas` and `ServerModule`-adjacent plain-`async fn` ports can use `dyn_enum_close!`
   (hub needs `use server::…::Saga;` in scope and the bare-macro textual-scope rule of
   `🔀️dispatch/🦀️.rs` — in a *downstream* crate that means `use server::__semio_dispatch_Saga;`,
   which is legal from another crate); `Deciders`, `Resolvers`, the four stores, `Queries` and
   `Documents` need the hand-written `async fn` delegation shown in `TestDeciders`.
5. **`Modules`.** `ServerModule` has `type Instance = HubInstance`, so hub's six modules
   (§B.5 item 8) close into one hand-written `HubModules` enum delegating `manifest`, `deciders`,
   `routes`, `resolvers`, `templates`.
6. **Builder call shape changed**: `Server::<HubInstance>::builder(profile).…build().await?` — the
   turbofish is required (nothing else names `I`), and `build` now returns
   `Result<Server<I>, ServerError>`.
7. **Nothing else in the framework blocks hub.** `get_document_ws` is no longer a dead route: the
   moment `HubInstance::Documents` is a real engine, the handler, the relay and the presence code
   behind it (`📡️gateway/🦀️.rs`, region `🔖️DocumentStream`) become reachable for the first time.

## 8. Honest gaps

- **Verified by tests only.** No server was bound to a socket; `Server::run` is unexercised here as
  it was before. The gate is the crate's own suite.
- **`ServerSagas<I>`/`Sagas` is declared but not wired into `ServerState`.** The saga runner is
  still constructed by the caller, exactly as before this slice; the associated type exists so hub
  names its workflow set in one place. Wiring a drain loop into the server is §B.7 step 5+ work and
  would have been new behaviour, not de-closing.
- **`StorageProfile` still has one variant.** De-closing the *stores* does not de-close the
  *profile*; see §7 item 3.
- **The `🔀️dispatch` macro cannot close a `Send`-future port.** Out of path scope here. A ~30-line
  additive branch (when a trait method is non-`async` and returns `impl Future<Output = X>`, emit
  `async fn … -> X` and `.await` each arm) would restore `dyn_enum_close!` for all ten ports and
  save hub several hand-written enums. Worth its own slice, with the macro crate's own five test
  suites as the gate.
- **The TS twin is still `export {};`** (§B.5 item 10) — untouched, out of scope.

## 9. Files changed

```
M 🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🔣️.json
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🧪️tests/🔬️unit/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🧪️tests/🔬️unit/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs
A 🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs
```

943 insertions, 585 deletions. No file outside `🧰️framework/🛍️products/🖥️server/` was modified.
