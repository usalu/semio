# 🖥️ S End to End — summary

## What was blocking `s`

`s` is the **host** playground variant backed by `semio-s-plugin-space`, so its dev build compiles the
whole plugin fleet. Two things stopped it, one behind the other:

1. **`semio-framework-os-kernel` did not compile for `wasm32-wasip2`.** A stale `.await` survived a
   peer's de-async of `ArtifactStore::detach_backbone`. One error — but it aborted the fleet check
   before anything downstream was reached, so the tree *looked* one error from green when it was not.
2. **`semio-s-plugin-stdio` did not compile — 329 errors.** Every other s plugin crate links it,
   including `space`/`s` itself, so it was the single gate for the entire fleet. `protocol::Mutation`
   had gained two required items (`DESCRIPTORS`, `descriptor`), and 42 hand-rolled stdio artifacts
   still predated them.

## What was done

The sanctioned way to obtain those two trait items is `#[derive(dsl::Mutations)]` over per-variant
**mutation leaves**. Exactly one artifact in the repo was already migrated — `🖼️tiff 6.0 ✳️baseline` —
so the recipe was derived from it and its derive (`📓️plan-mutation-leaf-migration.md`), then applied
across all 42 artifacts by a fleet of agents (`📓️fleet-report-wave-1.md`), plus:

- the corrupted `📊️csv` mutations file repaired (`📓️fix-csv-mutations.md`)
- `semio ✳️drawing`'s lost shared helpers restored
- the `NoMutation` retirement carried through **every** language surface the repo's
  `mutation/language-parity` policy covers: 47 TypeScript mirrors, 45 JSON Schemas, 33 GraphQL, 20
  Protobuf, 52 grammar files (`.g4`/`.ebnf`/`.abnf`/`.ksy`), 41 `.grammar.semio`/`.protocol.semio`
- two pre-existing parity gaps closed on the way: the `✳️any` envelope was missing 5 members on every
  non-Rust surface, and `✳️drawing`'s surfaces mirrored a vocabulary with **zero** name overlap
  against its own Rust enum

## The result

```
cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --lib
    Finished `dev` profile [unoptimized] target(s) in 5m 47s
```

**0 errors.** The gate is open.

## Three defects the fleet could not have found

Each lived outside the artifact folders the agents owned, and each was found by a lock-free audit
rather than by another compile round-trip:

1. **`📦️glue.rs` shadowed every migrated leaf module.** 21 stub `pub mod set_snapshot { … }` blocks
   left over from an older sweep beat the aggregates' own `pub use component::*`, and for 7 of them
   compiled the leaf a second time in a scope with none of its imports. Removed exactly those 21; the
   other 98 leaf stubs were kept, because `✳️mesh`/`✳️brep` genuinely depend on them.
2. **`semanticKind` must be verb-entity.** `mutation_leaf_descriptor_kebab` returns its `hyphen` flag,
   so a bare noun is rejected — `binary`'s `Splice` and the `✳️any` envelope's 18 wrappers all failed.
   Renamed (`ReplaceByteRange`, `ApplyBrep`, …) with the **wire tag pinned** via `#[serde(rename)]`, so
   no fixture, catalog or `.feature` scenario changed.
3. **`to_kebab` splits inside compound words.** `SetTextBoxBlocks` → `set-text-box-blocks`,
   `InsertTexCoord` → `insert-tex-coord`. Rather than discover these one compile at a time, a script
   replicating the derive's own `to_kebab` audited **87 aggregates** and **913 leaf descriptors**:
   0 mismatches remain.

## What is NOT done

- **`s` has not been observed running.** `dev s served` reaches the framework engine wasm build and
  dies on the shared cargo target lock — peer sessions held 130–200 concurrent cargo processes
  throughout. This is environmental, not a code defect; the repo's own error text names the cause.
- **`.storybook/s-end-to-end.spec.ts` is written but never executed.** It is the gate that would prove
  `s` boots ready and is interactive.
- **The other 25 s plugin crates are unverified.** They were unreachable behind stdio before; with the
  gate open they can finally be measured, but no run completed.

## Incidental finding worth keeping

`.claude/launch.json`'s `s-react` entry does not serve React: `frameworkOsPlaygroundDevEnv` defaults
`SEMIO_RENDERER` to **`wgpu`**, so a bare `dev s` builds 59 crates and hands off to `trunk serve`,
never reaching Vite. A `served` segment was added to `📜️script.ts` (`SEMIO_RENDERER=react` +
`SKIP_PLUGIN_BUILD=1`, the pair `collabStartUserDevServer` already uses) and registered as
`s-react-served`.

## Two knobs that make `s` bootable on a contended machine

Both were needed together; neither is discoverable from the failure message alone.

1. **`CARGO_TARGET_DIR`** pointed somewhere private. The engine wasm step
   (`wasm-pack build … framework_surface`) uses the shared `target/`, and peer sessions held it
   continuously.
2. **`SEMIO_BUILD_BUDGET_MS`** raised. `buildBudgetMs()`
   (`🦑️repo/…/📚️library/📦️packages/🟦️typescript/📦️index.ts:1233`) defaults to a 20-minute wall clock
   and *kills* the build — so escaping the lock is not enough, because a **cold** private-target build
   of the surface crate tree cannot finish inside 20 minutes on a machine running 130–200 concurrent
   cargo processes. The symptom is a bare `spawnSync bun ETIMEDOUT`, which reads like a hang rather
   than a deliberate kill.

Neither default was changed: both are shared infrastructure, and the right values depend on how loaded
the machine is. They are recorded here because the failure mode is opaque and recurs.

## The tree moved under this ticket, twice

This repo runs many concurrent sessions, and two of them landed inside this ticket's blast radius
while it was in flight. Both are recorded because they change how the result above should be read.

**1. `protocol::Mutation` lost its serde supertraits.** Read at the start of this ticket:

```rust
pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
```

Read at the end of it:

```rust
pub trait Mutation<P>: Clone + crate::value::ToValue + crate::value::FromValue {
```

That is ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` removing serde as
a runtime dependency — which is what AGENTS.md asks for. 15 framework files changed in the hour it
took to notice. Its immediate consequence is that `semio-framework-os-kernel` currently fails with
**75 errors** for the wasm target (`SpaceHistoryMutation` has no `ToValue`/`FromValue` yet;
`assert_store_roundtrip` still demands `Serialize + DeserializeOwned` from
`SubsetRoundtripSpec::Mutation`). That is their migration mid-flight, not this ticket's to finish —
but it now sits between `s` and a boot, and it means **the clean stdio compile reported above was
measured against the previous trait definition**. It is being re-measured.

**2. The `base64` extraction.** The same ticket moved `base64_standard_encode/decode` out of
`📡️replication/⚙️codec` into a new `semio-framework-io-base64` crate. A fleet check taken mid-creation
caught `E0432: unresolved import semio_framework_io_base64`; the crate materialized minutes later.

The lesson worth carrying: in this repo a green compile is a **timestamp**, not a property. Anything
this ticket claims is true of the tree at the moment it was measured.

### Re-measurement against the moved tree

Re-running the stdio check after the supertrait change:

```
error: could not compile `semio-framework-os-kernel` (lib) due to 5 previous errors
  🏪️store/🧬️schema/🧬️mutations/🦀️.rs:30  error: #[value(...)] does not support container attribute `content`
  …/📌️commit-space-checkpoint/🦀️.rs:8    SpaceCheckpoint: protocol::ToValue is not satisfied
  …/🌿️create-space-alternative/🦀️.rs:8   SpaceAlternative: protocol::ToValue is not satisfied
  (+2 FromValue)
```

**All five are inside `semio-framework-os-kernel`'s own `🏪️store/🧬️schema/🧬️mutations/`** — the peer's
serde-elimination migration, mid-flight. `semio-s-plugin-stdio` was never reached: cargo stops at the
failed dependency. Nothing here implicates this ticket's work, and the count is falling on its own
(75 → 5 within about half an hour), so the peer is actively closing it.

Their files are deliberately left alone. A poll re-runs the stdio check the moment the kernel goes
green, so the result can be restated against a settled tree rather than guessed at.

## Semantic verification — the part a compiler cannot do

A green build proves the shapes line up. It does not prove `agg_diff`/`agg_inverse` still compute what
the old `impl Mutation` computed. Nine agents re-derived the pre-migration baseline
(`git show 67fb4216b2^:"<dir>/🦀️component.rs"` — the migration commit also renamed the file, so
`HEAD:` returns the already-migrated copy and proves nothing) and compared **every match arm's
right-hand side, character for character**, across all 42 artifacts.

**Result: no semantic regressions.** Every difference fell into the sanctioned classes — the pattern-head
rewrite, `vec![E::NoMutation]` → `Vec::new()`, ordinal renumbering after dropping tag 0, and the forced
re-wrap when an inverse constructs one of the enum's own variants.

Two arms deserve calling out because a careless sweep would have flattened them and no compiler would
have noticed:

- `✳️flow`'s `SetNodeParam` inverse falls back to `RemoveNodeParam`, not to a no-op. It was **not**
  swept into the `Vec::new()` pattern. Correct.
- `✳️image`'s `SetMetadataEntry` inverse falls back to `RemoveMetadataEntry`. Likewise preserved.

### One real defect found — and it was pre-existing

`🧊️obj`'s `📖️component.grammar.semio` pinned `insert-tex-coord` / `remove-tex-coord` / `set-tex-coord`
while the Rust `#[dsl(keyword)]`, `KINDS`, the `🧪️oracle/🔣️.json` catalog and the `.feature` scenario
ids all say `insert-texcoord`. That breaks DSL text round-trip for three ops, and nothing type-checks a
grammar file against a keyword string, so it compiled clean either way.

Checked against `git`: the drift is **committed at HEAD with no working-tree change**, so it predates
this ticket. Fixed the grammar to the spelling the other four sources agree on, and corrected the doc
comment that caused it — it asserted `keyword == to_kebab(variant)` and cited a misspelled
`InsertTexcoord`, when the real variant is `InsertTexCoord` and these three leaves carry an explicit
`#[dsl(keyword)]` override. The leaf DESCRIPTOR's `semanticKind` legitimately stays the derived
`insert-tex-coord`; the two vocabularies are independent, which the comment now says.

## The wasip2 cfg split — a peer's refactor, and where this ticket helped vs. stopped

`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION` is doing something subtle and correct: `target_arch =
"wasm32"` is **TRUE for the WASI component target too**, so every browser-only dep and every
`js_sys`/`wasm_bindgen` code path had been leaking into `wasm32-wasip2` — the very target the plugin
fleet builds for. They are narrowing those to
`cfg(all(target_arch = "wasm32", not(target_env = "p2")))`, with the mirror image
`cfg(any(not(target_arch = "wasm32"), target_env = "p2"))` on the arm wasip2 should take.

Their migration was ahead of its call sites in three places, all of which blocked every fleet check
this ticket tried to run. Two were mechanical and are fixed here, in their own idiom:

1. **`📡️replication`** — the `wasm-bindgen` dep block had been deleted outright while
   `⚠️diagnostic`'s `fault_to_js`/`result_fault_to_js` still used it and `🧩️puzzle`'s `🌉️wasm` bridge
   still called them on ~20 lines. Restored the dep **under the narrowed cfg**, and gave the two
   functions the same gate — which is exactly the gate the puzzle bridge already carries
   (`#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`), so dep, definition and consumer
   now agree.
   *(A first pass restored it under the old broad `cfg(target_arch = "wasm32")`, which would have
   pulled wasm-bindgen straight back into wasip2 — the opposite of the peer's intent. Corrected once
   their reasoning was read properly.)*

2. **`📇️directory/🪪️identity`'s `now_ms`** — still a two-arm native/wasm32 split, so wasip2 took the
   `js_sys::Date` arm. Copied the peer's own already-solved version from `🏪️store/🔄️sync` verbatim,
   including its reasoning: `SystemTime` on native **and** wasip2 (WASI's clock backs it fine),
   `js_sys::Date` only in the real browser build.

3. **`🏪️store/🔄️sync`'s `spawn_actor` — left alone, deliberately.** Under wasip2 neither arm exists:
   `mod native_actor` is `cfg(not(target_arch = "wasm32"))` and `mod wasm_actor` is already narrowed
   to `not(p2)`, while the call site at `:1078` still switches on the broad `target_arch = "wasm32"`.
   Fixing it means deciding **which actor runtime wasip2 gets**. The `now_ms` precedent says wasip2
   follows the native arm, and widening `native_actor`'s gate would very likely compile — but
   `native_actor` is built on `semio_framework_async::WorkerPool`, and "it compiles" is not evidence
   that a thread-pool actor runtime behaves correctly on a single-threaded component target. This is
   the plugin-host path for `s`; a silently wrong answer here is worse than a red build. Left for the
   owner of that refactor.

   **That caution turned out to be load-bearing.** `mod native_actor` opens with
   `use tokio::time::Instant; use tokio_tungstenite::tungstenite::Message;` and
   `type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>`
   — real TCP sockets and a WebSocket client. Widening its gate to `p2` would **not** have worked;
   it would have traded one compile error for a pile of them, or worse, compiled into something that
   cannot open a socket at runtime. `wasm32-wasip2` needs a genuine third arm (or an explicit
   unsupported path), which is a design decision, not a cfg edit.

   The same in-flight narrowing is visible one layer down in `⏳️async/🦀️.rs`: its browser-JS module
   is already `cfg(all(target_arch = "wasm32", not(target_env = "p2")))` (lines 48-98) while
   `:508`/`:529` still switch on the broad `target_arch = "wasm32"`, and it carries two separate
   `WorkerPool` structs. That is the layer the wasip2 actor answer has to come from.
