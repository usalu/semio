# ✅️ Verified outcomes — what is PROVEN by a passing run

Every claim below was produced by a command that actually completed. Claims that were reported by
an agent but never executed are in the "not proven" section at the bottom, deliberately separated.
Three agent self-assessments on this ticket turned out wrong in ways that mattered — twice
pessimistic, once optimistic — which is why everything load-bearing was re-run centrally.

## Headline

| metric | baseline (aad3d81959) | now |
|---|---|---|
| third-party entries in **s production** manifests | **119** | **20** |
| distinct third-party crates leaking into s production | **23** | **7** |
| JS third-party in s `dependencies` | 11 pkgs | 6 |
| gate `oracle-conflicts` | 18 | 6 |

Of the remaining 20: **15 are real** (`serde_json` 8, `serde` 7, across 9 manifests). The other 5
are not violations — 3 are the `proc-macro = true` trio (compiler plugins, build-time only) and 2
are `🧩️puzzle`'s live browser bridge, now correctly excluded from the component target.

## Proven by passing runs

- **`semio-s-plugin-draw-fsm` builds clean for `wasm32-wasip2`** — `cargo check` exit 0, zero
  errors, 30m52s. First end-to-end proof that a real plugin compiles for the shipped target on the
  first-party `ToValue`/`FromValue` foundation.
- **`semio-framework-replication`** — clean check, and 225/226 tests pass (the 1 failure is an
  unrelated concurrent taxonomy fixture).
- **`semio-framework-os-kernel` compiled green** — observed via `cargo check -p semio-framework-3d`
  exiting 0 with os-kernel building as a dependency at 33 warnings / 0 errors.
- **First-party BLAKE3 — byte-exact** vs `blake3 1.8.7`: 28 official vector lengths (0…1,000,000)
  one-shot, plus 300 randomly-chunked incremental cases. Closes the persisted-digest risk.
- **parry3d replacement — at parity**: compiles; 600/600 random rigid transforms vs
  `parry3d::query::intersection_test` (150 intersecting, so not trivially always-false); 20/20
  degenerate cases including exact face/edge/corner contact; 1728/1728 `contains_point` vs
  parry3d's `PointQuery`.
- **First-party DEFLATE — fixed and re-verified**: 513/513 lengths, all three parity directions.
- **png/image codec** 12/12 with differential oracle · **glTF/mesh-engine** 26/26 incl. 3
  differential oracle tests · **text+path** 11/11 · **animate raster/typeset** 9/9 ·
  **kurbo arclen** 200 curves at 1e-6 · **puzzle** 15/15 · **animate** 148/148 ·
  **cad** 316/322 (6 pre-existing flakes).

## Real defects found BY verification (not by review)

1. **`inflate()` had never worked** — 0/513. Shipped on the DEFAULT feature path of
   `semio-framework-pack` and `os-kernel`. Root cause: the one-shot driver treated `NeedInput` as
   fatal. Fixed. See `📓️deflate-inflate-defect.md`.
2. **Browser glue was linked into the shipped WASI component.** `target_arch = "wasm32"` is TRUE
   for `wasm32-wasip2`, so `[target.'cfg(target_arch = "wasm32")'.dependencies]` is not
   browser-only. Proven with
   `cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i wasm-bindgen`, which showed
   `wasm-bindgen → js-sys → semio-framework-async → semio-framework-actor → semio-framework`.
   Puzzle's own manifest fixed here; 13 framework manifests being narrowed to
   `cfg(all(target_arch = "wasm32", not(target_env = "p2")))`.
3. **Puzzle's 2d bridge was missing the `not(target_env = "p2")` gate** its 3d/5d siblings had —
   same bug class, found independently.
4. **A `semio-framework-hash` path dep used six `../` where siblings use nine**, breaking
   `cargo metadata` repo-wide and making every agent's build fail for an unrelated reason.
5. **`read_pixels` never padded `bytes_per_row`** to wgpu's 256-byte alignment — latent, masked
   because every existing caller used dimensions that were multiples of 64.

## Root causes fixed (why the plugin manifests could finally drop serde)

Four framework seams hard-typed to serde, each migrated to first-party `ToValue`/`FromValue` over
`DslValue`: `MutationDiff`, `Mutation`, `CompositeMutationKind`, and `TopicContribution`'s payload.
A fifth, `ArtifactApp::Snapshot`, is in progress. A `#[derive(ToValue, FromValue)]` macro now
supports adjacently-tagged enums (`tag` + `content`) and 2-tuples.

The subtle one, and the reason error counts *grew* before they shrank: `Diff` structs named by
`#[mutations(diff = …)]` frequently live in a **different, undecorated file** from their `Mutation`
enum (e.g. `WorkflowDiff` sits in `🔁️workflow/🦀️component.rs`, its enum in `🧬️schema/🧬️mutations/`).
A `dsl::Mutations` grep misses them entirely. Enumerate authoritatively instead:
```bash
grep -rn '#\[mutations(' --include='*.rs' 🧰️framework ✏️s | grep -o 'diff *= *[A-Za-z0-9_:]*' | sort -u
```
That yields **141** diff types — the complete set, by construction.

## NOT proven — stated plainly

- **No large plugin has been confirmed building for `wasm32-wasip2`.** `draw-fsm` passed; `puzzle`'s
  last observed attempt failed with 247 `E0277`s in `🔁️workflow`, but that observation is now stale
  (`WorkflowDiff` has since gained its derives). Needs a fresh run.
- The 13-manifest wasip2 glue fix is in flight; the `cargo tree -i` "package not in graph" evidence
  is not yet collected.
- 9 manifests still carry serde/serde_json, several deliberately deferred with measurements rather
  than rushed: `🗄️stdio` (~563 real call-site files), `🏗️fem` (1186 occurrences / 179 files),
  `🏭️process` parent (363 call sites), `🌀️procedural` (~1277 sites, was blocked on the `Snapshot`
  bound). Each is its own wave.

---

# 🔧️ Root cause of the framework-wide build failure — one derive bug, not 80

`#[derive(ToValue)]` emitted `match *self` for enums carrying fields
(`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs:363`). Dereferencing makes each pattern
bind its fields **by value**, so the generated `ToValue::to_value(field)` passed an owned
`f64` / `String` / `WorkflowParameter` where the trait's `&self` receiver needs `&_`. Result: 80
identical `E0308 mismatched types: expected &_`, every one pointing at a `WorkflowDiff` / `RunDiff`
field span in `🔁️workflow`.

Fix: `match self`. Default binding modes then bind each field as `&T`, which is what `to_value`
wants. The sibling `match *self` at line 293 is deliberately left alone — that arm only handles
all-unit-variant enums, which bind nothing.

This resolves the sequence that looked like three separate problems (247 → 80 → 0). It was one bug,
reported once per field, plus one stale observation.

## Phantom blockers — all three traced to the same cause

A `cargo check` blocked on the shared target-dir lock compiles the source **as of when it started**,
so its errors can describe a tree that no longer exists. On this ticket that produced:

1. **247 `E0277`s in `🔁️workflow`** — reported by two agents as a live blocker. The check had been
   queued since 1:18pm; `WorkflowDiff` had gained its derives in the meantime.
2. **`semio_framework_hash` unresolved in `◻2d`** — reported as a hard failure. No such reference
   exists in that module.
3. **"`os-kernel` is very likely still red"** — reported by the pilot that fixed it. A completed run
   showed 0 errors / 33 warnings / exit 0. Every observation it had was lock-blocked.

Countermeasures now in force: kill stacked duplicate checks with `TaskStop` before starting a fresh
one; prefer lock-free structural commands (`cargo tree -i`, `cargo metadata`) over compiles; and
check how long a command was queued before believing its failure.

## Confirmed clean by completed runs

`semio-framework-os-kernel` (0 errors, exit 0, 41m20s) · `semio-framework-pack` (exit 0, 3m42s) ·
`semio-framework-replication` (clean, 225/226 tests) · `semio-s-plugin-draw-fsm` for
`wasm32-wasip2` (exit 0, 30m52s).

## Known broken, NOT mine to fix

The repo's own dependency gate (`bun ./📜️script.ts verify dependencies literal-external`) currently
throws before running:

```
Invalid taxonomy schema:
- fixedFilenameContracts["cargo-integration-test"].pathPattern must end in one exact literal basename.
- packageSourceDispositions is missing source-format contract "cargo-integration-test".
```

An uncommitted edit to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` added a
`cargo-integration-test` contract whose `pathPattern` is `**/📦️packages/🦀️rust/tests/*.rs` — ending
in a glob where the schema demands a literal basename — and did not add the matching
`packageSourceDispositions` entry. The `tests/` directories it describes predate this ticket
(`🗺️surface`, `🔀️dispatch`, `🖱️ui/🧬️contract`), and there is an active peer ticket
`26/08/17/END-TO-END-TAXONOMY-NORMALIZATION`, so this is very likely a peer's in-flight work.

**Consequence for measurement:** the official gate number is unavailable until it is fixed. All
counts in this ticket therefore come from a targeted scanner over `✏️s/` manifests, which excludes
`🧪️oracle` / `🔬️probes` / `🏭️generator` / `🧫️fixtures` and resolves workspace-vs-path deps.

---

# 🎯️ Plugin `wasm32-wasip2` builds — the ticket's core evidence

Run after `semio-framework` went green (derive `match self` fix):

| plugin | errors | note |
|---|---|---|
| `semio-s-plugin-animate` | **0** | had `typst`, `typst-svg`, `typst-assets`, `usvg`, `vello`, `wgpu`, `kurbo`, `image` ALL removed |
| `semio-s-plugin-flow` | **0** | |
| `semio-s-plugin-draw-fsm` | **0** | confirmed separately, exit 0, 30m52s |
| `semio-s-plugin-energy` | 34 | own conversion still in progress |
| `semio-s-plugin-cad` | 751 | downstream of the store-bound chokepoint |
| `semio-s-plugin-stdio` | 751 | downstream of the store-bound chokepoint |
| `semio-s-plugin-puzzle` | 980 | downstream of the store-bound chokepoint |

`animate` at 0 is the single strongest result in this ticket: the plugin that carried the heaviest
third-party stack now compiles for the **shipped target** with zero third-party entries in its
manifest, consuming first-party framework interfaces instead.

The 751/980 figures were captured BEFORE the `🏪️store` bound fix below and are expected to fall.

## The five serde seams — why plugins could not drop serde until the framework moved

Each of these baked `serde::Serialize + serde::de::DeserializeOwned` into a bound that every plugin
implementing it inherited. All now bound on first-party `ToValue + FromValue`:

1. `MutationDiff` / `Mutation` — `📡️replication/🎮️mutation`
2. `CompositeMutationKind` — same crate
3. `TopicContribution::payload` / `contributes_topic` — moved to `DslValue`; ~40 call sites
4. `ArtifactApp::Snapshot` / `Config` / `Draft` / `Presence` / `Transient` — 53 restatements
5. `ArtifactStore` / `MemberStoreOwners` — 71 generic where-clause bounds in `🏪️store/🦀️component.rs`,
   the final chokepoint with all 997 workspace errors downstream of it

Seam 5 was rewritten by the coordinating session: only lines matching a where-clause bound shape
were touched, so `#[derive(Serialize, Deserialize)]` attributes and the hand-written
`impl Serialize for ArtifactCursor` remain untouched. 71 rewritten, 0 bound-shaped occurrences left.

## Derive-macro defects found and fixed

- `match *self` for enums with fields → bound each field by value, so `to_value(field)` got an owned
  value where `&_` was required. 80 identical `E0308`s from one bug. Now `match self`.
- `match self {}` was non-exhaustive for empty enums (`NoConfigMutation` and friends would never
  have compiled). Fixed with a plain-unit-enum mode, caught by a standalone 7/7 differential
  harness against `serde_json`.

Both were found by *running* the code, not by review — consistent with every other real defect on
this ticket.

---

# 🧵️ The through-line: six framework seams, not a plugin problem

Every "this plugin still carries serde" report traced back to a framework **type signature or trait
bound the plugin could not influence**. The plugin-side work was never the blocker. In order found:

| # | seam | where | why it forced serde on plugins |
|---|---|---|---|
| 1 | `MutationDiff` / `Mutation` | `📡️replication/🎮️mutation` | bound on `Serialize + DeserializeOwned` |
| 2 | `CompositeMutationKind` | same crate | same supertrait |
| 3 | `TopicContribution::payload` | `🔌️plugin` | field hard-typed `serde_json::Value`; ~40 call sites |
| 4 | `ArtifactApp::{Snapshot,Config,Draft,Presence,Transient}` | `🔌️plugin` | 53 bound restatements |
| 5 | `ArtifactStore` / `MemberStoreOwners` | `🏪️store` | 71 generic where-clause bounds; **997 workspace errors downstream** |
| 6 | `ArtifactEditor::command_from_action` | `🔌️plugin` | `args: Option<&serde_json::Value>`; 143 sites |

All now on first-party `ToValue`/`FromValue` over `DslValue` (seam 6 in flight). Consequence:
`semio-framework`, `semio-framework-os-kernel`, `semio-framework-plugin`,
`semio-framework-plugin-host`, `semio-framework-pack` and `semio-framework-replication` all compile
clean, and plugin manifests can finally drop serde honestly rather than cosmetically.

**No serde was deleted from the framework.** Every type keeps its serde derives/impls alongside the
new first-party ones. Removing serde from os-kernel itself (~150 references, hand-written
`impl Serialize` for `ArtifactEnvelope`/`ArtifactCursor`) is explicitly a LATER wave.

## Derive macro: grown to meet real shapes, each gap found by compiling

`#[derive(ToValue, FromValue)]` (`🌱️value/✨️derive`) now supports: `rename`, `rename_all`,
adjacently-tagged enums (`tag` + `content`), **externally-tagged enums** (serde's default), plain
unit enums, and 2-tuples. Two real defects were found and fixed by running it, not by review:

- `match *self` for enums with fields → bound each field by value, so `to_value(field)` received an
  owned value where `&_` was needed. **80 identical `E0308`s from one bug.**
- `match self {}` was non-exhaustive for empty enums (`NoConfigMutation` and friends could never
  have compiled).

Shapes it still cannot express, handled by hand-written impls throughout: tuple structs, generic
structs (`FixedTable<K,V>`), composed-child structs, and any type in a crate *below* os-kernel in
the DAG — the macro's generated code is rooted at a hard-literal `::semio_framework_os_kernel::`
path, so `replication`-resident types (`HybridLogicalTimestamp`, `Edit<Mutation>`, `MutationMeta`,
`MutationOrigin`, the id newtypes) must be hand-written regardless.

## Deferred with measurements, not hand-waves

| crate | measured | status |
|---|---|---|
| `🗄️stdio` | ~563 production call-site files | in flight; last seen 2217 errors mid-conversion |
| `🏗️fem` | 340 files, 179 with serde, 905 call sites, 168 derive sites | derive sites converted (additive); call sites are their own wave |
| `🏭️process` | 78 derive sites, 369 call sites | `serde` removed; `serde_json` blocked on seam 6 |
| `➗️mathematical` / `🔋️energy` | 106 files converted | serde-free in production paths; manifests pending `stdio` |
| `♾️infinite` `🌍️world` | ~14k lines, ~26 wgpu-engine symbols | GPU-tier split in flight |

---

# 📉️ THE REAL SCOREBOARD — link-level, and it is not yet met

Manifest-level progress is real but it is **not** the goal. The goal is that the shipped
`wasm32-wasip2` component links no third-party code. Measured:

```bash
cargo tree -p <plugin> --target wasm32-wasip2 --edges normal --prefix none \
  | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
```

| plugin | third-party crates LINKED into its wasip2 component |
|---|---|
| `semio-s-plugin-draw-fsm` | **31** |
| `semio-s-plugin-animate` | **267** |
| `semio-s-plugin-puzzle` | **274** |
| `semio-s-plugin-flow` | **282** |

Even the leanest still links `serde`, `serde_json`, `tokio`, `zip`, `flate2`, `miniz_oxide`,
`base64`, `zopfli`, `thiserror`, `indexmap`, `hashbrown`.

**Every one of these is transitive through a framework crate. No plugin manifest is at fault.**
`draw-fsm` at 31 vs `flow` at 282 is the proof the low number is reachable — they differ only in
which framework crates they pull.

## Drivers, traced

| driver | pulled in by |
|---|---|
| `typst` (+ biblatex, citationberg, comemo, csv, ciborium, chinese-number, codex…) | `os-infinite` |
| `tokio`, `zip` (+ flate2, miniz_oxide, zopfli, crc32fast, adler2) | `os-kernel` |
| `serde_json` | `os-kernel-dsl-derive` |
| `rustybuzz` | `framework-compiler` |
| `taffy` | `ui-render` |

## Why manifest-clean ≠ link-clean

Wrapping a library behind a framework interface satisfies `CLAUDE.md`'s *"use all external libraries
behind an interface"* — and it is what made the plugin manifests clean. But the crate still **links**
into the guest component. The interface boundary and the link boundary are different properties, and
only the second one satisfies "dependency free **at runtime**".

Sharpest illustration from this ticket: removing `typst`/`vello`/`wgpu`/`usvg`/`kurbo`/`image` from
`🎞️animate`'s manifest moved them into `semio-framework-raster` and `semio-framework-typeset` — which
then linked `wgpu` straight back into animate's component. Only the subsequent target-table split
actually removed it.

## The method that works, proven four times today

Host-side capabilities (GPU rasterization, typesetting, async runtime, archive I/O, text shaping)
are not things a WASI **guest** component performs. Gate them in the framework crate that owns them:

```toml
[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]
```

with matching `#[cfg]` on the code. **`target_arch = "wasm32"` is TRUE for `wasm32-wasip2`** — a bare
arch gate does not exclude the component target, and that single misunderstanding is the origin of
this entire bug class.

Where a capability IS genuinely guest-reachable, the working pattern is two implementations behind
one identical public API, the wasip2 one returning an honest error that was already the true runtime
outcome — never a stub, never a broken call chain.

---

# 📈️ Scoreboard after the host-crate splits

| plugin | third-party in wasip2 graph | was |
|---|---|---|
| `semio-s-plugin-draw-fsm` | **11** (6 actually linked) | 31 |
| `semio-s-plugin-flow` | **117** | 282 |

`tokio`, `zip`, `flate2` and their tails (`miniz_oxide`, `zopfli`, `crc32fast`, `adler2`) are gone
from the guest path. `base64` is not in the graph at all.

## Measurement correction — `cargo tree` overstates the linked set

`cargo tree --edges normal` lists proc-macro crates and their dependencies even though they are
compiled **for the host** and never linked into the `.wasm`. Verified per crate with `-i`:

| crate | reachable only via | linked? |
|---|---|---|
| `syn`, `quote` | `semio-framework-dispatch-macros` **(proc-macro)** | ❌ host only |
| `proc-macro2`, `unicode-ident` | via `quote`/`syn` | ❌ host only |
| `serde_derive` | proc-macro | ❌ host only |
| `serde`, `serde_core` | `framework-async` → `job`/`pack` → `os-kernel` | ✅ **linked** |
| `serde_json` | `replication`, and `os-kernel-dsl-derive` | ✅ **linked** |
| `itoa`, `memchr`, `zmij` | `serde_json` | ✅ **linked** |

**So `draw-fsm`'s genuine runtime-linked third-party surface is 6 crates — all one family, serde.**
Report the linked figure, not the raw `cargo tree` count.

## The remaining path to zero is narrow and known

Three framework crates put serde on the guest path:

| crate | pulls |
|---|---|
| `semio-framework-async` (→ `job`, `pack` → `os-kernel`) | `serde` |
| `semio-framework-replication` | `serde_json` |
| `semio-framework-os-kernel-dsl-derive` | `serde_json` |

This is the same seam work that has already succeeded six times, applied to three framework crates
instead of to plugins. `draw-fsm` at 6 linked crates is the template; `flow`'s remaining 117 is
dominated by `typst` (split in flight) plus `rustybuzz` (`framework-compiler`) and `taffy`
(`ui-render`).
