# 📓️ terra — dedyn-fw-server report

Packet: `dedyn-fw-server`. Owned path: `🧰️framework/🛍️products/🖥️server/**`, scoped to `Decider`,
`PrincipalResolver`, `QueryHandler`, `DocumentAuthority`, `Saga`, `ServerModule`, `AuthorityStore`,
`ProjectionStore`, `BlobStore`, `SessionStore`.

## 1. Counts

**Starting count** (verified by python3 regex over the four owned files, re-verified with an
independently-implemented BSD `grep -nE` query — both agree): **26** first-party `dyn <Trait>`
occurrences in code, zero in comments:

| trait | file | count |
|---|---|---:|
| Decider | `🎭️authority` (3), `📡️gateway` (2) | 5 |
| PrincipalResolver | `🛡️policy` (3), `📡️gateway` (1) | 4 |
| QueryHandler | `📡️gateway` | 4 |
| DocumentAuthority | `📡️gateway` | 3 |
| Saga | `🎭️authority` | 2 |
| ServerModule | `📡️gateway` | 2 |
| AuthorityStore | `📡️gateway` (both inside the `DynAuthorityStore` wrapper) | 2 |
| ProjectionStore | `📡️gateway` | 2 |
| BlobStore | `📡️gateway` | 1 |
| SessionStore | `📡️gateway` | 1 |
| **total** | | **26** — matches the brief's own count exactly |

**Ending count**: **0** first-party `dyn <Trait>` anywhere in the four owned files. The only
remaining `dyn` in code is `Box<dyn Fn(&CommandEnvelope) -> PolicyDecision + Send + Sync>`
(`PolicyHook`, `🎭️authority/🦀️component.rs:262`) — `dyn Fn`, std, R1-permitted, untouched.

Verified with two differently-implemented queries, both re-run after the final edit:
```
python3 regex over the 4 absolute paths, comment lines excluded  -> 1 hit (dyn Fn)
grep -nE 'dyn[[:space:]]+(Decider|PrincipalResolver|QueryHandler|DocumentAuthority|Saga|
          ServerModule|AuthorityStore|ProjectionStore|BlobStore|SessionStore)\b' over the 4 files
          -> 0 hits in code (1 doc-comment mention of the historical Box<dyn AuthorityStore>
             pattern, in prose, describing what DynAuthorityStore used to be)
```

## 2. Mechanism per family, and why (R11's decision procedure)

Repo-wide search (not just the owned crate) for `impl <Trait> for` found, for every one of these
ten traits, **at most one real implementor, and it lived inside `#[cfg(test)] mod tests`** —
`CounterDecider`, `EchoSaga`, `StubResolver`, `CountingModule`; zero implementors anywhere for
`QueryHandler`/`DocumentAuthority`. This crate is a domain-neutral framework product (server's own
module doc: "an instance registers modules against this product's registries") whose real
implementors are meant to live in downstream products, none of which exist in this repo yet
(greenfield — confirmed by `terra-dyn-enum-macro-report.md` finding 6, which independently found
`Decider` "currently sync… not a real E0038 case today" at an earlier point in this session).

Given that, and given every one of these types is **embedded directly in `ServerState`/
`ServerBuilder`**, which axum's `State<S>` extractor requires to stay one concrete, `Clone`able type
across ~17 handler functions — genericizing `ServerState` over 4-5 type parameters to chase a
theoretical downstream implementor that does not exist would have (a) touched far more than the
~10-public-type R11 stop-threshold once every handler fn's signature is counted, and (b) bought
nothing today, since nothing in this repo would ever instantiate anything but the default. So:

- **Decider, Saga, PrincipalResolver, ServerModule** — closed set, `dyn_enum_close!`, **with their
  sole test-only implementor promoted to production scope** (`CounterDecider`, `EchoSaga`,
  `PrincipalResolver`'s stub renamed `BearerTokenResolver`, `ServerModule`'s `CountingModule`) so the
  generated enum (`Deciders`, `Sagas`, `PrincipalResolvers`, `ServerModules`) has a real variant and
  every existing test exercises genuine enum dispatch rather than a mock. Framework doc comments on
  each promoted type say explicitly that a real product adds its own implementor as a further enum
  variant alongside it — this is the same shape `store-dedyn`'s `NoMembers`/`BackboneChannelPorts`
  used for an as-yet-unimplemented family, except here a variant already exists.
- **QueryHandler, DocumentAuthority** — closed set with **zero** implementors anywhere, even in
  tests (`server.state().documents.is_none()` and an always-empty `queries` map are the actual,
  asserted behaviour today) — `dyn_enum_close! { pub enum X: Trait {} }`, uninhabited, exactly the
  macro's documented "default-composes-nothing" case. `Option<Arc<DocumentAuthorities>>` therefore
  provably stays `None` and `Arc<DashMap<String, Arc<QueryHandlers>>>` provably stays empty — matches
  every existing test's expectation byte-for-byte. The day a real query handler or replication engine
  lands, it is added here as the first variant, never as a `Box<dyn ..>`.
- **AuthorityStore, ProjectionStore, BlobStore, SessionStore** — closed set, one real (non-test)
  reference implementor each already (`Memory*Store`, declared right in `🗄️storage`) —
  `dyn_enum_close!` straight onto that implementor: `AuthorityStores`, `ProjectionStores`,
  `BlobStores`, `SessionStores`. `AuthorityStore` additionally required **deleting**
  `DynAuthorityStore` — a hand-rolled `Box<dyn AuthorityStore>`-in-a-newtype erasure wrapper whose own
  doc comment said the quiet part out loud ("so a `StorageProfile` can pick a backend at runtime
  without the bus becoming a dynamic dispatch point") — exactly the O1-rejected pattern, now replaced
  by `CommandBus<AuthorityStores>` (`CommandBus` was already generic over its store type; only the
  concrete type argument changed). Repo-wide grep confirmed nothing outside this file referenced
  `DynAuthorityStore`, so the deletion is safe.
- **None of the ten used generics/associated types (R11's "open set" branch).** Every implementor
  that exists, or that this crate's own tests need, is enumerable **in this same file**, which is
  exactly R11's closed-set trigger. `BlobStore` here is a same-named but textually distinct trait from
  `os`'s own `BlobStore` (`💻️os/🏪️store`) — confirmed via `trait BlobStore` declaration search — so
  there is no cross-crate open-set question for it either.

No exactly-one-impl-so-delete-the-trait case applied: every trait here is a genuine, documented
extension seam (`ServerModule`'s doc: "no rung is hard-coded into this crate"), so even where only
one implementor exists today, keeping the trait (closed over a real + extensible enum) is the
correct call, not a trait-object with an enum of one.

## 3. Universal-async (O1) — done for the owned families, not chased further

This whole crate was untouched by the async program before this packet (confirmed:
`terra-dyn-enum-macro-report.md` found `Decider` fully sync at an earlier point this session, and
every trait method here was plain `fn`). Per the brief's "asyncify first, then insert awaits":

- Every method of all ten owned traits, and their concrete impls, is now `async fn`.
- Direct glue that had to become `async` as a mechanical consequence (not optional): `CommandBus::
  register`/`submit`, `ActorRegistration::new`, `SagaRunner::register`/`drain_outbox`,
  `ResolverChain::push` stayed sync (no `.await` inside)/`resolve` became async,
  `ServerBuilder::build`, `ServerState::identify`, `storage::content_hash` (blocked on
  `protocol::crypto::RecordHasher::hash`, already `async fn` from an earlier, unrelated packet — this
  one function was a genuine pre-existing compile blocker in my owned `storage.rs`, fixed as directly
  in-scope fallout).
- Two **R10 residue-shape #1** hits (`.await` inside a sync closure), fixed by hand exactly as R10
  prescribes — hoisted to explicit `for` loops, no bulk tool involved:
  `ResolverChain::resolve` (was `.iter().find_map(..)`), `SagaRunner::drain_outbox` (was
  `.flat_map(..).flat_map(..)`).
- `#![allow(async_fn_in_trait)]` added at the crate root (`📦️glue.rs`) with the R3/R7 rationale
  comment, matching the pattern already used in `semio-framework-math`.
- **Not asyncified** (explicitly out of scope, no `dyn` involved, not a method of any owned trait):
  `AuthorityDirectory`, `PolicyEngine`, `Fanout`, `Presence`, `KickMap`, `AppRegistry`,
  `StaticAppHost`, `CommandBus::store`/`store_mut`. These are plain structs/methods this packet never
  touched the shape of; asyncifying the rest of this crate under O1 is real remaining work but a
  separate, much larger undertaking than "de-dyn ten named families," and chasing it here risked the
  kind of half-finished, untested sprawl R10/rule 25 warn against. Flagging honestly rather than
  claiming it.
- No `// 🚫️async:` tags were needed — nothing in my owned families hit an E1–E5 exception; every
  method that exists is genuinely `async fn`.

## 4. Macro friction

None beyond what `terra-dyn-enum-macro-report.md` already documented and this packet confirmed by
using it for real, ten times, in a live shared crate:
- Every `dyn_enum_close!` call is in the **same module** as its trait's `#[dyn_enum]`, trait declared
  first — bare invocation, zero `rustc#52234` warnings, exactly per finding 1.
- The zero-variant ("default-composes-nothing") shape was exercised for real for the first time in
  this crate (`QueryHandlers`, `DocumentAuthorities`) — worked exactly as the macro report's §5 item 7
  described (`match *self {}` bodies), first try.
- One easy-to-miss trap: a `///` doc comment placed directly above a `dyn_enum_close! { .. }`
  invocation triggers `unused_doc_comments` ("rustdoc does not generate documentation for macro
  invocations") — fixed by using a plain `//` block comment instead. Worth adding to the recipe.

## 5. Acceptance — real commands, pasted output, exit codes

All run foreground, `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/
e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-dedyn-server`. Full logs saved to this ticket
folder: `terra-dedyn-server-check-lib.txt`, `terra-dedyn-server-check-alltargets.txt`,
`terra-dedyn-server-test.txt`.

```
$ cargo check -p semio-framework-server --lib
    Finished `dev` profile [unoptimized] target(s) in 0.60s
```
Exit code: `0`. Zero warnings attributed to `semio-framework-server` itself (all warnings in the
transcript are pre-existing R7 lint fallout in the unrelated, not-owned `semio-framework-replication`
crate — 59 of them — plus 2 in `semio-framework-dispatch-macros` itself, neither mine to fix).

```
$ cargo check -p semio-framework-server --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.24s
```
Exit code: `0`. Same attribution: 0 warnings on `semio-framework-server`.

```
$ cargo test -p semio-framework-server
running 73 tests
... (73 lines, every one "... ok")
test result: ok. 73 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

   Doc-tests server
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Exit code: `0`. **73 passed / 0 failed** — every test in `storage`, `authority`, `policy`, `gateway`
(including `build_collects_every_module_contribution`, which now exercises the real
`ServerModules::Counting(CountingModule)` enum-dispatch path end to end, and
`the_outbox_drains_exactly_once_across_two_calls`, which exercises `Sagas::Echo(EchoSaga)`). This is
not a subset — it is the crate's full pre-existing test suite, unchanged in what it asserts, now
running async and green.

The "COMPILE REALITY" warning in the ticket about `semio-framework-plugin` not yet being green did
**not** block this packet — `semio-framework-server`'s dependency graph (`replication`, `serde`,
`serde_json`, `thiserror`, `axum`, `tokio`, `dashmap`, `futures`, plus the two macro crates this
packet added) never reaches the guest SDK, so a real, full, green build was obtained rather than a
structural-only report.

## 6. Files touched

- `🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️component.rs` — `AuthorityStore`,
  `ProjectionStore`, `BlobStore`, `SessionStore` asyncified + `#[dyn_enum]`'d, four
  `dyn_enum_close!`s, `content_hash` asyncified (fallout), tests converted to
  `#[semio_framework_async_macros::async_test]`.
- `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️component.rs` — `Decider`, `Saga`
  asyncified + `#[dyn_enum]`'d, `CounterDecider`/`EchoSaga` promoted out of `#[cfg(test)]` into
  production scope, two `dyn_enum_close!`s, `ActorRegistration`/`CommandBus::register`/`::submit`/
  `SagaRunner::register`/`::drain_outbox` updated (submit/drain_outbox now `async fn`, the latter's
  closure-based iteration rewritten as an explicit loop per R10), tests converted.
- `🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️component.rs` — `PrincipalResolver`
  asyncified + `#[dyn_enum]`'d, `StubResolver` promoted and renamed `BearerTokenResolver`, one
  `dyn_enum_close!`, `ResolverChain::push`/`::resolve` updated (`resolve` now `async fn`, loop
  rewritten per R10), tests converted.
- `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️component.rs` — `QueryHandler`,
  `DocumentAuthority`, `ServerModule` asyncified + `#[dyn_enum]`'d (three `dyn_enum_close!`s, two
  zero-variant), `CountingModule` promoted out of `#[cfg(test)]`, `DynAuthorityStore` **deleted**,
  `ServerAuthority`/`ServerState`/`ServerBuilder` fields switched from `Box<dyn ..>`/`Arc<dyn ..>` to
  the generated concrete enums, `build()` now `async fn`, every call site of a now-async trait method
  (`identify`, `post_query`'s `handler.handle`, `handle_document`'s `welcome`/`submit_frame`,
  `replay_events`'s `events_since`, blob handlers' `put`/`get`/`has`, `content_hash`) updated with
  `.await`, tests converted (`#[tokio::test]`, matching this file's own pre-existing convention rather
  than introducing a second async-test macro into an already-tokio-using file).
- `🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml` — added
  `semio-framework-dispatch-macros` (`[dependencies]`) and `semio-framework-async-macros`
  (`[dev-dependencies]`).
- `🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/📦️glue.rs` — added
  `#![allow(async_fn_in_trait)]` with the R3/R7 rationale comment.
- Ticket folder: this report, `terra-dedyn-server-check-lib.txt`,
  `terra-dedyn-server-check-alltargets.txt`, `terra-dedyn-server-test.txt`.

## 7. Anything a sibling must know

- **No `lease-request`** — nothing outside `🧰️framework/🛍️products/🖥️server/**` needed a change;
  repo-wide grep confirmed zero external references to `DynAuthorityStore` or to any of the four
  promoted test-only types before this packet started.
- `semio-framework-server`'s `Cargo.toml` had **no `[dev-dependencies]` section at all** before this
  packet (confirmed by reading it) — the 63-crate central `#[async_test]` dev-dependency rollout
  never reached this crate. If another packet also touches this crate's tests, the dependency is now
  there.
- `storage::content_hash` was silently broken before this packet touched it (blocked on
  `protocol::crypto::RecordHasher::hash` already being `async fn` from an unrelated, earlier packet) —
  fixed here as direct fallout in an owned file; flagging in case another packet independently
  "discovers" the same defect and assumes it is still open.
- Full crate-wide O1 (universal async) compliance for `semio-framework-server` is **not** complete —
  only the ten owned families and their direct glue are asyncified (see §3's explicit not-done list).
  Whoever's scope eventually covers `AuthorityDirectory`/`PolicyEngine`/`Fanout`/`Presence`/`KickMap`/
  `AppRegistry`/`StaticAppHost` should know none of those were touched here.
