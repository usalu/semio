# D1 — `🔀️dispatch`: closing `impl Future + Send` ports, and the eight server ports re-closed

Slice D1, executing the follow-up `📓️w3a-server-instance-seams.md` §8 named ("the `🔀️dispatch` macro
cannot close a `Send`-future port"). All commands foreground, from `/Users/ueli/Documents/semio`, no
sub-agents, no git-modifying command. Captures in `🗑️generated/d1-*.txt`.

**Status: DONE.** Both crates green, and every one of the eight ports is now closed over a
two-variant enum **from another crate** by a permanent test — the exact position hub is in.

---

## 1. The problem, from the compiler's side

W3a made the gateway generic over `ServerInstance`, which put the eight request-path ports behind an
associated type. An `async fn` in a trait returns an opaque future with no auto-trait guarantee, so
every axum handler and `on_upgrade` callback rejected it (`future cannot be sent between threads
safely`, `E0277 … : Handler<_, _> is not satisfied` ×7). The fix was to declare the ports
`fn m(..) -> impl Future<Output = T> + Send`, which cost them `#[dyn_enum]`: the macro emitted

```rust
fn m(&self, ..) -> impl Future<Output = T> + Send {
    match *self { Self::A(ref i) => i.m(..), Self::B(ref i) => i.m(..) }
}
```

and two arms cannot put two *different* opaque future types in one return position (`E0308`). W3a
therefore hand-wrote the one set it needed (`TestDeciders`, 21 lines for 3 methods) and left the
other seven ports unclosable, i.e. hub would have hand-written ~5 more.

## 2. Design

One additive branch, exactly as W3a's §8 predicted. When a trait method is **not** `async` and its
return position is `impl … Future<Output = T> … `, the delegate is emitted as `async fn .. -> T`
with `.await` in each arm instead of reproducing the opaque return. This is legal in both
directions: Rust accepts an `async fn` implementation against an `impl Future` declaration, and one
`async` body is one type however many arms it matches. The trait itself is still re-emitted
byte-unchanged — only the generated delegate differs.

Three deliberate boundaries:

- **`Future` is matched on the last path segment** (`Future`, `core::future::Future`,
  `::std::future::Future`) — a proc-macro has no type resolution.
- **`impl Future` with no `Output = ..` binding is a hard error** naming the method: an `async fn`
  delegate has no way to name its own return type. It was already unclosable for >1 variant; now it
  says so.
- **Any other `impl Trait` return is left alone** (`ReturnShape::Plain`), so a one-variant set over
  e.g. `-> impl Iterator` still closes verbatim, exactly as before.

A side benefit that matters downstream: the delegate of such a method never mentions `Future`, so a
closing site in another crate does not need `std::future::Future` in scope.

## 3. A second defect the work uncovered: unqualified delegation

Writing the eight-port fixture (§5) produced, from the macro's own output, five instances of

```
error[E0034]: multiple applicable items in scope … multiple `get` found
  candidate #1 is defined in an impl of the trait `BlobStore` for the type `Rung<N>`
  candidate #2 is defined in an impl of the trait `ProjectionStore` for the type `Rung<N>`
```

The generated arm called `inner.method(..)` — **method-call syntax**, which resolves by name against
everything the variant type offers. That is wrong twice for a delegate, and both ways bite hub
specifically:

1. one backend type implementing two ports that share a method name (`ProjectionStore::get` and
   `BlobStore::get`, or their two `put`s — the obvious shape for a single durable store, which is
   precisely what W3b is building) is an outright compile error at the closing site;
2. an **inherent** method of the same name silently wins over the trait's, so the closed set
   delegates to something it never promised — a silent wrong answer, not an error.

The arms now call through the trait: `Trait::method(inner, args)`. The trait is already required to
be in scope at the closing site (the generated `impl Trait for $enum_name` names it), so this adds
no requirement. Both failure modes are now regression-tested (§5).

## 4. Fixes (file:line)

| file | what |
|---|---|
| `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs:155–215` | new region `🔖️Return-position analysis`: `enum ReturnShape` (:158) and `fn classify_return` (:179) |
| `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs:365` | `build_delegate_method` now takes `trait_ident: &Ident` |
| `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs:373–378` | the signature rewrite: `ReturnShape::FutureOutput(ty) => (async, -> #ty)`, and `.await` driven by the rewritten asyncness rather than `sig.asyncness` |
| `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs:396` | the arm calls `#trait_ident::#method_name(#receiver_expr, ..)` instead of `#receiver_expr.#method_name(..)` |
| `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs:16–20, 347–364, 503–509` | crate docstring, `build_delegate_method` docstring (why the call is trait-qualified), and the `dyn_enum_close!` recipe's second half (type scope at the closing site) |
| `🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs:171, 235, 280, 322` | `#[dyn_enum]` restored on `AuthorityStore`, `ProjectionStore`, `BlobStore`, `SessionStore` (+ import :43) |
| `🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️.rs:212` | `#[dyn_enum]` on `PrincipalResolver` (+ import :27) |
| `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs:118` | `#[dyn_enum]` on `Decider` (import already present for `Saga`) |
| `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs:288, 334` | `#[dyn_enum]` on `DocumentAuthority`, `QueryHandler` (+ import :30) |
| `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs:169–172` | `ServerInstance`'s doc no longer tells a downstream crate to hand-write `Send`-future delegations |
| all eight port docstrings | the "Send futures, declared not inferred" paragraph now ends by saying the macro closes such a set too, and how |
| `🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs:137–144` | `TestDeciders`, 21 hand-written lines → one `dyn_enum_close!` block |

`ServerModule` is deliberately still not `#[dyn_enum]`'d: it carries an associated type, which the
macro rejects by design (an enum has no single type to give it). W3a's §3.2 stands unchanged.

## 5. Tests and counts

**`semio-framework-dispatch-macros` — 41 tests, 0 failed, 0 warnings** (was 28; capture
`🗑️generated/d1-dispatch-test-final.txt`).

Six new unit tests in `🧪️tests/🔬️unit/🦀️.rs` (28 in that target, was 22):
`build_delegate_method_rewrites_a_send_future_return_to_an_async_fn`,
`…_accepts_a_qualified_future_path`, `…_rejects_a_future_without_a_named_output`,
`…_leaves_a_non_future_impl_trait_return_alone`, `…_keeps_an_async_fn_method_async`,
`end_to_end_send_future_port_expansion_parses_as_valid_rust` (asserts the trait keeps all three
`impl Future`s and the body gains exactly three `async fn` delegates). The existing
`…_strips_mut_from_forwarded_params` now asserts the trait-qualified call shape.

New integration target `[[test]] send_future` → `🧪️tests/🔮️send-future/🦀️.rs` (254 lines, **7
tests**): `Send`-ness asserted both directly on the delegate's future and through a generic
`S: Store` bound; the future **moved onto a real `std::thread`** and polled there; `&mut self` and
default-bodied arms; a generic method; a zero-variant closing enum over the same port; and
`a_name_shared_by_two_ports_and_an_inherent_method_still_delegates_to_the_closed_one`, which pins
§3 (two ports both declaring `read` on the same two types, plus an inherent `TextStore::read`
returning a different type).

**`semio-framework-server` — 83 tests, 0 failed, 0 warnings** (78 lib + 5 new; the lib count was 73
at W3a and rose to 78 through W3b's conformance work, not mine — capture
`🗑️generated/d1-server-test-final.txt`).

New integration target `[[test]] closed_ports` → `🧪️tests/🔒️closed-ports/🦀️.rs` (336 lines, **5
tests**). This is the deliverable that makes §4 real rather than decorative: it is a separate crate
linking `server`, so it uses the **cross-crate** half of the recipe
(`use server::__semio_dispatch_AuthorityStore;` …, which a same-crate site may not use —
rust-lang/rust#52234), and it closes **all eight ports over two-variant enums**
(`ClosedAuthorityStores`, `ClosedProjectionStores`, `ClosedBlobStores`, `ClosedSessionStores`,
`ClosedResolvers`, `ClosedDeciders`, `ClosedDocuments`, `ClosedQueries`). `Rung<const N: u8>`
implements every port once and is instantiated at two const parameters, so each set has two
genuinely distinct variant types and each arm reports its own `N` — the tests assert *which* arm
answered, not merely that it compiled. It also covers the `&mut self` port, the generic method
(`QueryHandler::handle<P>`, answered through a closed `ProjectionStore`), and `Send`-ness behind a
generic `S: AuthorityStore` bound.

Regression gates for the codegen change, which touches ~90 existing applications:

| gate | result |
|---|---|
| `cargo test -p semio-framework-machine` | 31 passed, 0 failed |
| `cargo check -p semio-framework-math` (lib) | clean |
| `cargo check -p semio-s-plugin-dag` | clean — pulls `semio-framework-os-kernel`, `semio-framework`, `…-ui`, `…-os-plugin`, i.e. the repo's heaviest `dyn_enum_close!` users; only pre-existing peer warnings |
| `cargo check -p semio-framework-server` | clean |

## 6. Gates

| gate | result |
|---|---|
| `cargo test -p semio-framework-dispatch-macros` | **41 passed, 0 failed, 0 warnings** |
| `cargo check -p semio-framework-dispatch-macros` | clean (implied by the test build) |
| `cargo test -p semio-framework-server` | **83 passed, 0 failed, 0 warnings** |
| `cargo check -p semio-framework-server` | clean |
| `cargo check -p semio-framework-server --target wasm32-unknown-unknown` | **fails, and did before this slice**: `error: could not compile 'mio' (lib) due to 48 previous errors`. `mio` arrives unconditionally through `tokio`'s `net` feature, which `axum` requires; the crate declares no wasm feature and contains no `cfg(target_arch = "wasm32")` code. Identical to W3a's baseline (`🗑️generated/d1-server-wasm.txt`). |

No new nx target or `launch.json` row: both new `[[test]]` targets are picked up by the existing
`test` targets of `@semio-tech/dispatch-macros-rs` and `@semio-tech/framework-server-rs`, which run
plain `cargo test -p <crate>`.

## 7. Honest gaps

- **Verified by tests only.** No server was bound to a socket in this slice; the gate is the two
  crates' own suites. The `Send`-ness claim is nevertheless stronger than a bound check: one test
  polls a delegated future on a second OS thread.
- **`ServerModule` still cannot be enum-closed** (associated type). Hub's module set remains the one
  hand-written delegating enum W3a predicted, ~70 lines. Supporting associated types would mean
  deciding what type an enum gives them, which is not expressible.
- **`StorageProfile` variants and `ServerSagas` wiring** are untouched — W3a §8 items, not this
  slice's.
- **`cargo test -p semio-framework-math` is red, and was before this slice**: 7 errors in
  `🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs` (`assert_eq!` on an un-`await`ed `impl Future<Output = u32>`
  at :1300–1303, a spurious `.await` on the sync `Rng::from_seed` at :501). That file is unmodified
  in the working tree (last commit `025ec86a42`, 2026-09-08) and the crate's **lib** checks clean
  with this slice's macro, so this is pre-existing asyncify damage in another slice's path, reported
  and not touched.
- **The `Output` type must resolve at the closing site.** The delegate reproduces the trait's
  parameter and return types as literal tokens, so hub imports them exactly as it would to
  hand-write the impl. Pre-existing property of the macro, now documented on `expand_dyn_enum_call`;
  the eight-port test crate is the worked example of which imports that means.
- **Peers were live in `🖥️server` throughout** (W3b's `conformance` module, `StorageProfile::Ephemeral`,
  doc-link fixes in `🎭️authority`). Everything above was re-read immediately before editing; the
  diffstat for the crate therefore contains peer lines that are not mine (notably the storage and
  gateway unit-test files, which I did not touch).

## 8. Files changed

```
M 🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs
M 🧰️framework/🔨️modules/🔀️dispatch/🧪️tests/🔬️unit/🦀️.rs
A 🧰️framework/🔨️modules/🔀️dispatch/🧪️tests/🔮️send-future/🦀️.rs
M 🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/Cargo.toml
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs
A 🧰️framework/🛍️products/🖥️server/🧪️tests/🔒️closed-ports/🦀️.rs
M 🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml
```
