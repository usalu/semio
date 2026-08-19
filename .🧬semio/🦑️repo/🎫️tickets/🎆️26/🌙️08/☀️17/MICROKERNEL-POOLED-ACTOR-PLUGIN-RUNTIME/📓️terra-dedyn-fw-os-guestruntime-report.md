# 📓️ terra-dedyn-fw-os-guestruntime report

Packet: `dedyn-fw-os-guestruntime`. Scope: zero first-party `dyn GuestRuntime` / `dyn ErasedProjection`
in `🧰️framework/🛍️products/💻️os/**`, restricted to those two families.

## 1. Starting → ending counts

Measured with python3 over absolute paths (`dyn\s+(?:\w+::)*(GuestRuntime|ErasedProjection)\b`,
comments excluded from the "real" count but shown separately), then re-verified with a second,
differently-implemented query (`grep -rn --include=*.rs 'dyn.*<Trait>'`).

| family | starting (real code) | ending (real code) | doc-comment mentions left (not counted) |
|---|---:|---:|---:|
| `GuestRuntime` | 16 | **0** | 2 (both explain the removal — one is mine, one pre-existing in `🔌️plugin/🖥️host` which is not my file) |
| `ErasedProjection` | 22 | **0** | 3 (same pattern) |

Both totals match the packet brief's estimates exactly (16 / 22) once the search regex is widened to
catch qualified paths (`Arc<dyn semio_framework_plugin_host::GuestRuntime>` in `🏃️run` and
`Arc<dyn Fn() -> Vec<Box<dyn db_projection::ErasedProjection>>>` in `🛢️db/📄️artifact` were both
missed by a bare `dyn\s+GuestRuntime` pattern on the first pass — flagging this as a trap for any
sibling still counting: **qualify your regex for `dyn\s+(?:\w+::)*Trait\b` or you will under-report**,
exactly rule 21's "negative from a too-narrow query" pattern.

## 2. `GuestRuntime` (16 uses) — mechanism: use the already-built closed-set enum

A completed packet (`host-dedyn`) already built `GuestRuntimes` (hand-written match-delegation, not
`#[dyn_enum]`/`dyn_enum_close!` — two of its three variants are `#[cfg(test)]`-gated, see its own doc
at `🔌️plugin/🖥️host/🦀️component.rs:1047`) and verified zero `dyn GuestRuntime` inside that crate
itself. My 16 were exactly the downstream consumers it left: the renderer's wgpu glue/runtime and
`🌉️mcp/🏠️workspace`, plus one more the brief didn't name — `🏃️run/🦀️component.rs`'s
`WasmtimeNodeHost.runtime` field, found only because I widened the search regex to catch its
fully-qualified `Arc<dyn semio_framework_plugin_host::GuestRuntime>` form.

Files touched (all `Arc<dyn GuestRuntime>`/`&dyn GuestRuntime` → `Arc<GuestRuntimes>`/`&GuestRuntimes`,
constructors wrapped in `GuestRuntimes::Wasmtime(..)`):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` — 1 field (`WasmtimeNodeHost.runtime`)
  + 1 constructor. Added `use semio_framework_plugin_host::GuestRuntime;` (new — the file previously
  called `.compile`/`.instantiate` on the field with NO trait import at all, which only works for a
  `dyn Trait` receiver; a concrete `GuestRuntimes` enum needs the trait in scope for method-call
  syntax to resolve).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs` — 3 sites
  (`activate_plugin_instance`'s `&dyn GuestRuntime` parameter, `PluginArtifactChannel.runtime` field
  + its constructor, and one bare `WasmtimeRuntime` local in `attempt_plugin_activation` that used to
  coerce into `&dyn GuestRuntime` at the call site — a concrete enum needs an explicit
  `GuestRuntimes::Wasmtime(..)` wrap since unsizing coercion no longer applies).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
  — 10 sites: `KernelThreadState.guest_runtime` + constructor, 6 `scale_bench` budget-fn parameters,
  and the bench's own `main`-path `WasmtimeRuntime::new(..)` construction.
- `…/🧊️wgpu/🎠️runtime.rs` — 2 sites: `ParallelRuntime.guest_runtime` field + `ParallelRuntime::new`'s
  parameter. `ShardExecutor::spawn` (in the NOT-mine, already-completed `🖥️host/🧵️shard/🏃️executor.rs`)
  already takes `Arc<GuestRuntimes>` directly, so no bridging was needed there — confirms the enum's
  shape was correctly anticipated for its downstream consumers.

**R11 case: exactly-one-impl-in-scope, closed-set enum already generated upstream.** No design
decision was mine to make here — just propagate the enum type through its remaining holders.

## 3. `ErasedProjection` (22 uses) — mechanism: **generics, not "delete the trait object"**

**The brief's premise ("7 methods, 1 impl ⇒ delete the trait object, use the concrete type") does not
survive contact with the code and I did not follow it as written — recording why, since this is
exactly the kind of finding R11 asks to be lifted the moment it's read.**

`ErasedProjection` has exactly one *impl block* (`impl<P: ProjectionClass> ErasedProjection for
ErasedWrapper<P>`), but it is a **blanket impl over a generic P** — a type-erasure wrapper, not a
single concrete type. Its whole purpose is letting a `ProjectionEngine` hold a `Vec` that mixes
*differently-typed* `ProjectionClass` implementors (`db_projection`'s own tests build `vec![
erase(CounterProjection{..}), erase(SumWithDependencyProjection{..}) ]` — two different concrete
`ErasedWrapper<P>` types in one `Vec`). "Delete the trait object, use the concrete type" is not
available for a genuinely heterogeneous collection — there is no single concrete type to substitute.

I verified the actual implementor set repo-wide (python3 over absolute paths, not shell globbing) before
choosing a mechanism, per the "validate your assumptions" rule:

- `CounterProjection` / `SumWithDependencyProjection` — both `#[cfg(test)]`-private to
  `📽️projection/🦀️component.rs`'s own test module.
- `CountingProjection` — `🧪️testkit/🦀️component.rs`, used alone (never mixed with another type).
- **Zero production implementors anywhere.** `db_artifact::ArtifactEngineConfig.projections: Arc<dyn
  Fn() -> Vec<Box<dyn db_projection::ErasedProjection>> + Send + Sync>` is a caller-supplied factory
  that COULD register real projections, but every one of the ~50 `ArtifactEngineConfig::default()` /
  `{ ..Default::default() }` call sites repo-wide (confirmed by grep, none excluded) leaves it at the
  empty-`Vec`-returning default. The mechanism this field exists for has never actually been exercised
  outside its own crate's tests.

All three implementors turned out to live in the **same crate** — `semio-framework-os-kernel-db`
(`db_projection`/`db_artifact`/`db_testkit` are `#[path]`-glued MODULES of one crate, not separate
crates, confirmed by reading `📦️glue.rs`) — so this is R11's **closed-set** case, not the open/generics
case its own examples describe. But the fix a closed set calls for is still a generic parameter, not a
single concrete swap-in, because the *engine* (`ProjectionEngine`) must stay agnostic to which closed
set a given call site chose:

- `ProjectionEngine<'a, S: IndexStorage, E: ErasedProjection>` replaces
  `ProjectionEngine<'a, S: IndexStorage>` — `projections: Vec<Box<dyn ErasedProjection>>` becomes
  `projections: Vec<E>`. Same for the free fns `ProjectionGraph::build`/`should_run` (both gain
  `<E: ErasedProjection>`), and the methods `projection_by_id`/`decode_checkpoint`/`load_checkpoint`
  (parameter type `&dyn ErasedProjection` → `&E`).
- `erase<P: ProjectionClass + 'static>(class: P) -> ErasedWrapper<P>` — drops the `Box`; `ErasedWrapper`
  made `pub` (required — a `pub fn` cannot return a private type, E0446).
- **`db_projection`'s own tests** (the ONLY call site, repo-wide, that mixes types): a small,
  hand-written, `#[cfg(test)]`-local closed enum `AnyTestProjection { Counter(ErasedWrapper<
  CounterProjection>), SumWithDependency(ErasedWrapper<SumWithDependencyProjection>) }` +
  `impl ErasedProjection for AnyTestProjection` (match-delegation) + two `From` impls, so `.into()`
  works at each `erase(..)` call site exactly like the `GuestRuntimes` precedent's recipe. **Not**
  `#[dyn_enum]`/`dyn_enum_close!`: this crate has zero existing dependency on
  `semio-framework-dispatch-macros`, and wiring a brand-new proc-macro dependency into
  `🛢️db/📦️packages/🦀️rust/Cargo.toml` for one 2-variant, 7-method, all-sync trait is more risk (an
  as-yet-unlanded-in-any-live-crate macro, per its own report: "no family was actually converted in
  the live tree") than a 60-line hand delegation. Same call `host-dedyn` made for `GuestRuntimes`.
- **`db_testkit`** needed **zero edits** — it uses exactly one `ProjectionClass` (`CountingProjection`),
  so `erase(CountingProjection)`'s inferred return type (`ErasedWrapper<CountingProjection>`) flows
  straight into `ProjectionEngine::new`'s now-generic `E` parameter with no annotation anywhere.
- **`db_artifact::ArtifactEngineConfig.projections`** (the field the brief's "22" count implicitly
  included, via its qualified `Box<dyn db_projection::ErasedProjection>` form): swapped for a new
  zero-variant `pub enum NoProjections {}` in `db_projection`, `impl ErasedProjection for
  NoProjections` with every method body `match *self {}` (uninhabited ⇒ exhaustive over zero arms —
  same shape the shared macro's own `tests/uninhabited.rs` exercises for the generated case, just
  hand-written here for the same reason as `AnyTestProjection`). This is the honest reflection of
  today's reality (nothing ever constructs one, so the factory can never return anything) rather than
  a generic parameter threaded through `ArtifactEngineConfig`/`ArtifactEngine<R>` — the latter would
  ripple into every one of the ~50 call sites across `📄️artifact`/`🧪️testkit`/`⚙️engine` (all files
  outside my `ErasedProjection`-family grant), for a capability that has ZERO live consumers to prove
  it against. Per R11's own stop condition ("if it threads through more than ~10 public types, stop
  and report") I did not do that wider threading — `NoProjections` is a one-line, narrowly-scoped
  alternative that removes the `dyn` with no behavior change and no blast radius, and the doc comment
  on the field names exactly what a future real consumer needs to do (swap the type parameter... which
  does not exist yet — swap `NoProjections` for its own closed enum, which DOES require adding the `E`
  generic at that point; today there is nothing to generalize for).

**Files touched**: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📽️projection/🦀️component.rs` (trait +
engine + tests — the bulk of the 22), `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
(1 field's type + its doc comment only — nothing else in that file touched: the other `Arc<dyn
AuthzHook>`/`Arc<dyn VersionGraph>`/`Arc<dyn Emit>` fields in the same struct are different families,
not mine).

## 4. Macro friction

None encountered directly — I chose NOT to invoke `semio-framework-dispatch-macros` for either
hand-written enum (`AnyTestProjection`, `NoProjections`), for the reasons in §3. `GuestRuntimes` (the
one enum I consumed rather than built) was ALSO hand-written by its own packet, for the parallel reason
(cfg-gated variants). Net: two-for-two hand-written enums in this packet's actual path, both because a
`#[cfg(test)]` boundary made the macro's proven acceptance surface (uncertain for cfg-gated variants
per its own report, finding under §3 there) not worth the first-ever live wiring of a brand-new
dependency into a production crate manifest.

## 5. Verification

**Structural** (both families, two differently-implemented queries, comments excluded):
```
$ python3 <regex: dyn\s+(?:\w+::)*(GuestRuntime|ErasedProjection)\b over 🧰️framework/🛍️products/💻️os>
→ 5 total hits, all 5 in doc comments (2 GuestRuntime, 3 ErasedProjection)
$ grep -rn --include=*.rs 'dyn.*GuestRuntime' / 'dyn.*ErasedProjection' 🧰️framework/🛍️products/💻️os
→ same result, comments only, exit 0
```
Zero real-code `dyn GuestRuntime` / `dyn ErasedProjection` remain in owned paths.

**Compile** — the ticket's documented "COMPILE REALITY" applied in full:

- `CARGO_TARGET_DIR=<scratchpad>/target-dedyn-os-guestruntime cargo check -p semio-framework-os-kernel-db --lib`
  → **exit 101, 281 pre-existing errors**, all in files I did not touch (`📝️wal` byte-reader
  async-conversion fallout, `🧪️testkit::StorageWrapper::capabilities` recursive-async-fn, `⚙️engine`'s
  `HashProjection`/`MutationSchema` fallout — an entirely different "projection" naming, unrelated to
  `db_projection::ErasedProjection`). **Zero of the 281 errors mention `📽️projection/🦀️component.rs`,
  `ErasedProjection`, `NoProjections`, `AnyTestProjection`, or `ProjectionEngine` anywhere in 8000
  lines of rustc output** (checked by substring search over the full log, not just the tail) — the
  file my edits actually live in compiles clean; the crate as a whole is red from unrelated concurrent
  churn (per rule 21/W4-item-8, confirmed pre-existing: `git log` on the WAL file shows no edit from
  me and the errors are dated to other packets' async-conversion work).
- `cargo check -p semio-framework-os-renderer-wgpu --lib` → **blocked before reaching my files**:
  `semio-framework-actor` (a transitive dependency, not touched by this packet) fails with 266
  pre-existing errors (`SceneStore`/`FailureState`'s `Default::default()` returning a future,
  `Debug` not implemented for `impl Future<Output = u32>`) — confirmed via `git log --date=iso` on
  that crate's file: last commit 2026-08-19 15:51, no uncommitted delta, so this is a landed-but-broken
  state from other concurrent async-conversion work, not something this packet caused or can fix
  (out of `path_scope`). `wgpu-check.txt` has zero mentions of `📦️glue.rs`/`🎠️runtime.rs`/
  `GuestRuntimes` — my crate never started its own compile pass.
- `cargo check -p semio-framework-os-mcp --lib` / `-p semio-framework-os-run --lib` — **acceptance
  UNRUN**: both were still compiling their (heavy, wasmtime/cranelift-bearing) dependency trees ~10
  minutes in, under measured system-wide contention that climbed from 41 to 60 concurrent
  `cargo`/`rustc` processes from sibling sessions — past rule 23's own "~20 concurrent processes" line
  past which even a 600s foreground timeout will not finish a wgpu-class build. Stopped waiting per
  that rule rather than burning further budget on a build rule 23 already predicts will not land.
  Given `semio-framework-actor` is upstream of both and is confirmed broken independent of this
  packet, they would very likely hit the same wall before reaching `🌉️mcp`/`🏃️run` source anyway.

**Reported per the brief's instruction: acceptance UNRUN for `os-mcp`/`os-run`/`os-renderer-wgpu`,
blocking crate named (`semio-framework-actor`, not mine), structural zero-dyn proof stands in its
place.** `os-kernel-db` DID run to completion (`--lib`) and is clean for my files specifically,
despite the crate's unrelated pre-existing red state. `--all-targets` for `os-kernel-db` (needed to
actually exercise the `#[cfg(test)]` `AnyTestProjection` code — `--lib` alone never compiles
`#[cfg(test)]` modules) was still queued behind the `os-mcp`/`os-run` builds on the same
`CARGO_TARGET_DIR` (self-inflicted lock contention — should have used one target dir per concurrent
check, per the ticket's own rule) when this packet's turn ended, under measured 45-60 concurrent
`cargo`/`rustc` processes system-wide.

To close that specific gap without waiting out the full dependency tree, I extracted the exact shape
of the new code — the trait, `NoProjections`, `ErasedWrapper<P>`, the generic `ProjectionEngine<E>`,
and `AnyTestProjection` with its two `From` impls, byte-for-byte the same pattern, dependency types
stubbed to unit structs — into a standalone file and compiled+ran it directly with `rustc` (no cargo,
no dependency tree, real compiler):
```
$ rustc --edition 2021 --crate-type lib --test -o <scratchpad>/erased_projection_shape_check_test \
    <scratchpad>/erased_projection_shape_check.rs
(zero output — clean compile, zero warnings)
$ <scratchpad>/erased_projection_shape_check_test
running 3 tests
test tests::heterogeneous_vec_via_any_test_projection ... ok
test tests::single_type_vec_infers_erased_wrapper ... ok
test tests::no_projections_vec_is_always_empty ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Exit code `0`, real rustc, real execution — proves the single-type-inference case, the heterogeneous
`.into()` case, and the uninhabited `NoProjections` case all type-check and behave correctly. This is
not a substitute for the real crate's `--all-targets` (still recommended for whichever session next
holds a clear `CARGO_TARGET_DIR`), but it is genuine evidence for exactly the part of this packet's
work that had no precedent to lean on (unlike `GuestRuntimes`, which a prior packet already proved
live).

## 6. Anything a sibling must know

- **The qualified-path under-count trap** (§1) — any packet still grepping bare `dyn\s+TraitName` is
  under-reporting; use `dyn\s+(?:\w+::)*TraitName\b`.
- **`semio-framework-actor` is currently red** (266 errors, `SceneStore`/`FailureState` `Default`
  returning futures, `Debug` not implemented for `impl Future<..>`) as of 2026-08-19 ~19:30, landed at
  commit `d16fc1017c` (2026-08-19 15:51), not a live in-progress edit at time of my check. This blocks
  `cargo check` for anything downstream (confirmed: `semio-framework-os-renderer-wgpu` never reaches
  its own source over it). Whichever packet owns `🎭️actor` should see this independently, but it is
  worth having in two places per rule 8 ("cross-packet findings must be lifted the moment they are
  read").
- **`ErasedProjection`'s "1 impl" is a blanket generic impl, not a single concrete type** — any packet
  that inherited a similar "N methods, 1 impl ⇒ delete the trait object" instruction should check
  whether that one impl block is itself `impl<P: SomeBound> Trait for Wrapper<P>` before assuming the
  trait-object removal is a plain type substitution; it may actually be R11's generics case wearing a
  "1 impl" disguise.
- `db_artifact::ArtifactEngineConfig.projections`'s new `NoProjections` type parameter is NOT generic
  yet (see §3) — the day a real caller wants to register a projection through `ArtifactEngine`, that
  caller's packet needs to add an `E: ErasedProjection` generic parameter to `ArtifactEngineConfig`/
  `ArtifactEngine<R>` (with `NoProjections` as its default, so the ~50 existing call sites keep
  compiling unchanged) — flagging so nobody rediscovers this design question from scratch.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎠️runtime.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📽️projection/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs` (1 field's type + doc comment)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dedyn-fw-os-guestruntime-report.md` (this file)

Not touched: `🔌️plugin/🖥️host/**` (read-only, per brief), `🛢️db/📄️artifact`'s other 3 dyn fields
(different families), `🧪️testkit`/`⚙️engine` (needed no edits at all — see §3).
