# 🧯️ Lane O — De-async the nine remaining `s` plugins

Ticket `26/09/05/S-END-TO-END` · lane `plugin-de-async` · Opus 5 · 2026-09-06

## 🎯️ Problem

The framework's mutation/dsl/editor traits went synchronous on 2026-09-01. Nine `s` plugins still
declared `async fn` for those trait methods, so the wasm32-wasip2 catalog rebuild died on hundreds of
`E0053`/`E0308` signature mismatches per crate.

## 🧭️ Decision rule

Same rule the `26/09/05/BLOCK-PLUGIN-END-TO-END` wave used
(`📓️w6-de-async.md`), re-verified against the framework **as it stands today** — the block note's
line numbers have since moved, so every trait was re-read:

| trait | methods | file:line (2026-09-06) | async? |
|---|---|---|---|
| `protocol::Mutation` | `descriptor`, `diff`, `inverse` | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:151-153` | **sync** |
| `protocol::MutationDiff` | `apply`, `absorb` | `…/🎮️mutation/🦀️.rs:99,114` | **sync** |
| `protocol::DiffAlgebra` | `inverse`, `between`, `is_empty` | `…/🎮️mutation/🦀️.rs:128-132` | **sync** |
| `protocol::OpText` | `print_op`, `parse_op` | `…/🎮️mutation/🦀️.rs:1253-1254` | **sync** |
| `protocol::OpBinary` | `encode_op`, `decode_op` | `…/🎮️mutation/🦀️.rs:1269-1270` | **sync** |
| `protocol::DiffCodec` | `print_diff`/`parse_diff`/`encode_diff`/`decode_diff` | `…/🎮️mutation/🦀️.rs:1289-1292` | **sync** |
| `store::ArtifactDsl` | `parse_dsl`, `print_dsl` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4900-4901` | **sync** |
| `store::ArtifactPack` | `encode_pack_with`, `decode_pack_with` | `…/🏪️store/🦀️.rs:9333-9334` | **sync** |
| `plugin::ArtifactEditor` | `ephemeral`, `initial_snapshot`, `handle`, `render`, `render_with_request_context`, `build_document_store_initialization_job` | `…/🔌️plugin/🦀️.rs:26756-26892` | **sync** |
| `plugin::ArtifactViewer` | same surface | `…/🔌️plugin/🦀️.rs:27067-27107` | **sync** |
| `plugin::ArtifactAnalysis` | `sniff`, `analyze` | `…/🔌️plugin/🦀️.rs:986-988` | **sync** |
| `plugin::ArtifactComposition` | `reads`, `compose` | `…/🔌️plugin/🦀️.rs:997-999` | **sync** |
| `plugin::ArtifactBuilder` | `empty`/`from_snapshot`/`from_text`/`from_binary`/`mutate`/`absorb`/`build` | `…/🔌️plugin/🦀️.rs:927-933` | **sync** |
| **`io::io_mechanism::Serializer`** | `serialize` | `🧰️framework/🔨️modules/🚪️io/🦀️.rs:2376` | **ASYNC** (`-> impl Future + Send`) |
| **`io::io_mechanism::Deserializer`** | `sniff`, `deserialize` | `🧰️framework/🔨️modules/🚪️io/🦀️.rs:2390,2393` | **ASYNC** |
| **`plugin::ArtifactInferrer`** | `infer_cached` | `…/🔌️plugin/🦀️.rs:1264` | **ASYNC** |
| **`plugin::ArtifactDecomposer`** | `decompose` | `…/🔌️plugin/🦀️.rs:963` | **ASYNC** |

`#[semio_framework_async_macros::async_test]` bodies stay `async fn`: the macro *rejects* a sync fn
(`🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️.rs:30`). That is the block wave's ruling and it is
load-bearing here — 4258 of the 9598 `async fn` in these nine plugins are such tests.

## 🔨️ Tool

`🔨️lane-o-de-async.py` (this ticket folder), two idempotent passes:

* `strip <plugin-root>` — `async fn NAME(` → `fn NAME(` everywhere except `#[async_test]` bodies and
  the four async-trait impl blocks above; a *sync* declaration found inside one of those blocks is
  restored to `async` (three `Deserializer`/`Serializer` impls in `sequence` and one
  `ArtifactInferrer::infer_cached` in `architect` were sync and therefore already broken).
* `unawait <cargo-json-log>` — deletes exactly the `.await` rustc reported as `E0728` (`await` in a
  now-sync fn) or `E0277 … is not a future`. Compiler-driven, so no call site is guessed at.

Re-running both passes is a no-op (verified: second `strip` over all nine printed `0 declarations`).

## 📋️ Per-crate result

`cargo check` counts are cargo's own `due to N previous errors` for the `lib` unit.
Native = `--lib --tests`; wasm = `--target wasm32-wasip2 --lib`.

| # | plugin | crate | files changed | `async fn` de-asynced | `.await` removed | native before | native after | wasm after | async-shaped before → after |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `🎬️sequence` | `semio-s-plugin-sequence` | 63 | 292 (+3 restored) | 0 | 553 | **251** | **102** | 228 → 8 |
| 2 | `🕸️dag` | `semio-s-plugin-dag` | 98 | 304 | 0 | 654 | **197** | **197** | ~113 → 8 |
| 3 | `📋️forms` | `semio-s-plugin-forms` | 93 | 354 | 4 | 136 | **0 ✅** | **0 ✅** | 91 → 0 |
| 4 | `💡️reasoning` | `semio-s-plugin-reasoning-mindmap` | 84 | 292 | 22 | 426 | **23** | **23** | ~99 → 0 |
| 5 | `➗️mathematical` | `semio-s-plugin-mathematical` | 84 | 692 | 25 | 2299 | **36** | **36** | ~126 → 0 |
| 6 | `📏️layout` | `semio-s-plugin-layout` | 90 | 464 | 0 | 800 | **211** | **211** | ~171 → 8 |
| 7 | `🎥️shooting` | `semio-s-plugin-shooting` | 162 | 517 | 0 | 545 | **59** | **59** | ~198 → 0 |
| 8 | `🗒️note` | `semio-s-plugin-note` | 184 | 477 | 1 | 807 | **51** | **51** | ~188 → 0 |
| 9 | `🏛️architect` | `semio-s-plugin-architect` | 852 | 1987 (+1 restored) | 10 | 2604 | **48** | **48** | ~1204 → 0 |
| | **total** | | **1710** | **5383** | **62** | **8824** | **876** | **727** | — |

*"async-shaped before"* is the crate's `E0053` count in the baseline (every one of them is a
`diff`/`inverse`/`handle`/`render`/… signature mismatch); *"after"* is every diagnostic in the final
run whose text mentions `future`. `E0728` (`await` outside an async fn) is **0** on both targets.

No `async fn` outside the sanctioned set survives — audited mechanically over all nine roots:

```
✏️s/🔌️plugins/🎬️sequence     {'async_test': 203, 'io/inferrer': 8}
✏️s/🔌️plugins/🕸️dag          {'async_test': 185, 'io/inferrer': 12}
✏️s/🔌️plugins/📋️forms        {'async_test': 191, 'io/inferrer': 8}
✏️s/🔌️plugins/💡️reasoning    {'async_test': 172, 'io/inferrer': 12}
✏️s/🔌️plugins/➗️mathematical  {'async_test': 382, 'io/inferrer': 8}
✏️s/🔌️plugins/📏️layout       {'async_test': 328}
✏️s/🔌️plugins/🎥️shooting     {'async_test': 343}
✏️s/🔌️plugins/🗒️note         {'async_test': 389, 'io/inferrer': 12}
✏️s/🔌️plugins/🏛️architect    {'async_test': 2063, 'io/inferrer': 1}
```

(`layout`/`shooting` reach the io traits through `Box::pin(async move { … })` closures instead of an
`async fn`, so they legitimately have no `io/inferrer` row.)

## 🧾️ Commands + real output

All cargo runs used the private target dir and no wrapper:

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-o \
  cargo check -p <crate> --lib --tests --keep-going --message-format=short
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-o \
  cargo check --target wasm32-wasip2 -p <crate> --lib --keep-going --message-format=short
```

`target-s-e2e-o/debug` was seeded by `rsync -a target-s-e2e-m/debug/ target-s-e2e-o/debug/` (lane M's
check-mode tree) so the framework rmeta was warm; the wasm32-wasip2 tree was built cold by this lane.

Baseline (before any edit):

```
error: could not compile `semio-s-plugin-sequence` (lib) due to 553 previous errors; 12 warnings emitted
error: could not compile `semio-s-plugin-reasoning-mindmap` (lib) due to 426 previous errors; 44 warnings emitted
error: could not compile `semio-s-plugin-forms` (lib) due to 136 previous errors; 20 warnings emitted
error: could not compile `semio-s-plugin-shooting` (lib) due to 545 previous errors; 19 warnings emitted
error: could not compile `semio-s-plugin-mathematical` (lib) due to 2299 previous errors; 85 warnings emitted
error: could not compile `semio-s-plugin-note` (lib) due to 807 previous errors; 76 warnings emitted
error: could not compile `semio-s-plugin-layout` (lib) due to 800 previous errors; 77 warnings emitted
error: could not compile `semio-s-plugin-dag` (lib) due to 654 previous errors; 18 warnings emitted
error: could not compile `semio-s-plugin-architect` (lib) due to 2604 previous errors; 16 warnings emitted
```

Final native (`--lib --tests`, all nine in one `--keep-going` run):

```
error: could not compile `semio-framework-os-flow` (lib) due to 7 previous errors; 29 warnings emitted
error: could not compile `semio-s-plugin-reasoning-mindmap` (lib) due to 23 previous errors; 44 warnings emitted
error: could not compile `semio-s-plugin-mathematical` (lib) due to 36 previous errors; 85 warnings emitted
error: could not compile `semio-s-plugin-shooting` (lib) due to 59 previous errors; 19 warnings emitted
error: could not compile `semio-s-plugin-note` (lib) due to 51 previous errors; 76 warnings emitted
error: could not compile `semio-s-plugin-layout` (lib) due to 211 previous errors; 78 warnings emitted
error: could not compile `semio-s-plugin-dag` (lib) due to 197 previous errors; 18 warnings emitted
error: could not compile `semio-s-plugin-architect` (lib) due to 48 previous errors; 16 warnings emitted
error: could not compile `semio-s-plugin-sequence` (lib) due to 251 previous errors; 12 warnings emitted
```

Final wasm32-wasip2 (`--lib`):

```
error: could not compile `semio-s-plugin-reasoning-mindmap` (lib) due to 23 previous errors; 44 warnings emitted
error: could not compile `semio-s-plugin-mathematical` (lib) due to 36 previous errors; 85 warnings emitted
error: could not compile `semio-framework-os-flow` (lib) due to 18 previous errors; 29 warnings emitted
error: could not compile `semio-s-plugin-shooting` (lib) due to 59 previous errors; 19 warnings emitted
error: could not compile `semio-s-plugin-note` (lib) due to 51 previous errors; 76 warnings emitted
error: could not compile `semio-s-plugin-sequence` (lib) due to 102 previous errors; 12 warnings emitted
error: could not compile `semio-s-plugin-layout` (lib) due to 211 previous errors; 78 warnings emitted
error: could not compile `semio-s-plugin-dag` (lib) due to 197 previous errors; 18 warnings emitted
error: could not compile `semio-s-plugin-architect` (lib) due to 48 previous errors; 16 warnings emitted
```

`semio-s-plugin-forms` appears in neither list: it is **green on both targets**, lib *and* tests.

## 🧪️ Tests

`forms` is the only one of the nine whose `#[cfg(test)]` suite compiles today, and it is the one the
strip touched most invasively (354 declarations, 4 `.await` removed). Its **test code type-checks** —
the 15:00 `cargo check -p semio-s-plugin-forms --lib --tests` run is green, `lib` and `lib test`
both. `cargo test -p semio-s-plugin-forms --lib` was launched twice and **could not be executed**:

* 16:12 — died on `error: failed to move dependency graph … incremental/semio_s_plugin_stdio-…/
  dep-graph.part.bin … No such file or directory`. The private target dir's `debug/incremental/` was
  deleted underneath the run (the volume is at **96 % / 35 GiB free**; a sweeper or a peer reclaimed
  it). Retried with `CARGO_INCREMENTAL=0`.
* 16:47 and again at 17:50 — `semio-framework-plugin` itself no longer compiles:

  ```
  🧰️framework/…/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:199:77: error[E0425]:
    cannot find function `outcome_to_result` in module `crate::host`: not found in `crate::host`
  🧰️framework/…/⚛️reactor/🔄️turn/🦀️.rs:240:113: error[E0425]: … same …
  error: could not compile `semio-framework-plugin` (lib) due to 2 previous errors; 131 warnings emitted
  ```

  The function is live at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs:46` but
  under a module path the reactor no longer resolves — the peer host migration the coordinator
  flagged, landed some time between 15:10 and 16:47. **Every `s` plugin, mine included, is
  unbuildable while this stands**; it does not affect the numbers in the table above, which were all
  measured between 13:40 and 15:10 while `semio-framework-plugin` was green.

For the other eight, `cargo test --lib` could not start even before that: the `lib` unit is still RED
for the non-async reasons in the blocker list. Their `#[async_test]` bodies were left byte-identical,
so nothing in this wave can have changed their outcome.

## 🚧️ Blockers — none of them async, none of them this lane's

The 876 native / 727 wasm errors that remain fall into five buckets, all owned elsewhere:

1. **Framework free functions still `async fn`** — 24 errors (`dag` 8, `layout` 8, `sequence` 8) and
   the *only* async-shaped residue left anywhere. Each is a non-suspending framework helper called
   from plugin code that the framework's own trait made synchronous:
   * `ActionDefinition::bounded_catalog` — 12 errors (`expected ActionDefinition, found future`).
     Repo-wide it has **194 call sites, of which only 22 `.await` it**: the 172 sync callers are the
     convention, this fn is the outlier.
   * `semio_framework_plugin::engagement_token_matches`
     (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:38638`) — 8 errors in `layout`
     (`expected bool, found future`); `💠️lowpoly` already ships its own **sync** copy of the same fn.
   * `selection_domains_from_surface` (`…/🔌️plugin/🦀️.rs:12533`) and
     `node_graph_delete_selection_spec` — 4 errors in `sequence`.

   These were errors *before* this wave too (the plugin fns were `async` but never awaited them), so
   nothing regressed; they simply cannot be fixed from inside a plugin. Fixing the three framework
   fns clears the last async residue in all nine crates.
2. **Mutation-descriptor wave** — `protocol::Mutation` gained `const DESCRIPTORS` + `fn descriptor`
   (`📡️replication/🎮️mutation/🦀️.rs:148-151`) that the plugin mutation leaves do not implement;
   surfaces as `E0046 missing DESCRIPTORS, descriptor` and a cascade of
   `X: MutationLeaf is not satisfied` (`dag` alone: 30). Also the `E0432 unresolved import
   crate::artifacts::layout::mutations::…::mutation` family (`layout` 62, `mathematical` 16,
   `shooting` 13) — the `dsl::Mutations` derive's expected module shape.
3. **UiNode / ComponentTree + `Label` migration** — `E0053 method render has an incompatible type`
   (11 across the nine), `expected Label, found semio_framework_plugin::Label`,
   `Label: From<LabelText>` (~60 total).
4. **Interactive-job wave** — `E0433 cannot find module or crate semio_framework_job` (35 on wasm:
   `sequence` 19, `note` 10, others), plus `E0425 cannot find type ArtifactOwnedToolJobContext`.
   `semio-framework-job` exists but is not a dependency of these plugin manifests. Untouched, per
   the explicit instruction not to move classification.
5. **The framework regressed under us mid-run, twice** — `semio-framework-os-flow` was clean
   (warnings only) in the 13:40 baseline and had **7 errors native / 18 errors wasm** by 15:00; then
   `semio-framework-plugin` went RED (`crate::host::outcome_to_result`, see Tests) between 15:10 and
   16:47, which blocks *every* `s` plugin from compiling at all. Both framework-owned, both a peer's
   in-flight edit, neither reachable from this lane. Re-run the two commands in this note once the
   host migration settles — the plugin sources need no further change for it.

Two defects were repaired because they are hard blockers inside the crates this lane owns, both
handcrafted to the peer convention (`🗄️stdio` gates the same declaration this way):

* `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/🦀️.rs:409,505` and
  `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/🦀️.rs:813` declared a `📚️examples/…/🧪️tests/🦀️.rs`
  module **without `#[cfg(test)]`**, pulling `#[async_test]` (a dev-dependency macro) into the `lib`
  build — 22 `E0433` that had nothing to do with the tests themselves. Added the missing gate.
* Three `impl Serializer`/`impl Deserializer` in `sequence` and one `impl ArtifactInferrer` in
  `architect` declared their method **sync** against an `-> impl Future` trait requirement; restored
  to `async` (see the tool's `strip` restore rule).

## 📦️ Files

* Codemod: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/🔨️lane-o-de-async.py`
* Touched: 1710 `🦀️.rs` files under the nine `✏️s/🔌️plugins/*` roots, plus the two lib roots above.
* Nothing outside `✏️s/🔌️plugins/{🎬️sequence,🕸️dag,📋️forms,💡️reasoning,➗️mathematical,📏️layout,🎥️shooting,🗒️note,🏛️architect}` was modified — no framework file, no peer plugin, no manifest.
