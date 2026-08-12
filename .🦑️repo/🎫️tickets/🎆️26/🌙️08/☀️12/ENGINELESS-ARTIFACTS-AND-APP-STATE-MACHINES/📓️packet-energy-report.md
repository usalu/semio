# Packet report — `✏️s/🔌️plugins/🔋️energy` artifact-tree `⚙️engine` elimination

Target: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` — **50 domain
subdirectories + 1 root `🦀️component.rs` + 1 root `🟦️component.ts` stub, 11,872 LOC total (11,683 `.rs`
across 51 files + a 5-line dead TS stub). Directory **deleted**.

## ⚠️ Load-bearing deviation from the naive region map: **energy has no app**

`📦️glue.rs`'s own header (pre-existing, unedited by this line) states it outright: *"energy is a
headless library plugin — no document app, no DSL/pack/spr wire codec of its own, no command
surface."* Confirmed structurally before touching anything:

```
grep -rln "AppIo" ⚙️engine        → 0
grep -rln "App\b" ⚙️engine        → 1 (the root component.rs only, in a doc-comment mentioning ArtifactApp)
find 🎛️apps -type f               → only 🦀️component.rs, containing a single doc-comment, zero apps
```

Rules 4, 6 and 7 (`AppIo`/app-type surfaces, `register()` wiring into an app's `component.rs`,
stateful hosts placed app-side) therefore have **no destination to send anything to**. This is not
a corner case avoided — it is the plugin's entire design, documented before this ticket existed.

## What the 50 domain modules actually are

`⚙️engine/🦀️component.rs`'s own `EnergyModelEngine::run_simulation` calls `crate::sim::Engine::run(&model,
config) -> Result<Results, Error>` — a **pure function** (`Model` decoded from `EnergyModelSnapshot`
via `model_from_snapshot`, plus a `SimulationConfig`, → `Results`). `sim::Engine::run` in turn calls
into all other 49 domain modules (`kernel`, `meters`, `output`, `precompute`, `sizing`, `metrics`,
`economics`, `props`, `units`, …) as ordinary library functions — this is the entire compute core of
an EnergyPlus-class transient BEM simulator, not app/UI behaviour. It fits rule 2 verbatim: *"pure
fn: snapshot → value/projection/manifest."*

**Deviation, loudly flagged**: rule 2's other examples (conformance laws, field sweeps, retention
laws) are *total* functions of the formal `ArtifactInferrer`/`Inference<Snapshot>` machinery already
present in `🧬️schema/💡️inferences/🦀️component.rs` (the `EnergyModelInference`/`entries` family). That
family's own doc is explicit that it is deliberately **shallow/opaque** — `model_json` is *"not
guaranteed to decode into the full typed `Model` for every persisted snapshot"* (the default
snapshot's `model_json` is `"{}"`, which fails `Model`'s required fields), so it only derives byte
counts / key counts, never a typed `Model` field. The 50 domain modules are the opposite: a
**fallible, on-demand** computation (`Result<Model, String>`, `Result<Results, Error>`) that requires
a real, fully-populated `Model` and is not safe to call for every snapshot. I did **not** wire them
into `EnergyModelInference`/`ArtifactInferrer` — I placed them as physical siblings under
`🧬️schema/💡️inferences/<domain>/`, matching that family's own convention ("each named inference gets
its own child dir") without touching the formal total-law machinery.

## Region → destination map (as executed)

| Region (old) | Symbol(s) | Destination | Rationale |
|---|---|---|---|
| 50× `⚙️engine/<domain>/🦀️component.rs` | every `pub`/private item in each domain (air_exchange … zone_hvac) | `🧬️schema/💡️inferences/<domain>/🦀️component.rs` (verbatim `mv`, zero content edits) | Rule 2 — derived compute from a decoded snapshot; no app exists to receive it (see deviation above). Each domain is declared `pub mod <domain>;` **flat at the crate root** in `📦️glue.rs` (pre-existing design, preserved) — the `#[path]` attribute decouples Rust's logical module tree from the filesystem, so relocating the physical file while keeping the identical `pub mod` declaration changes **zero** internal cross-references (`crate::sim::Engine`, `crate::model::Model`, `crate::props::…`, etc. all resolve identically before and after). This is why zero of the 50 files needed content edits. |
| `⚙️engine/🦀️component.rs` `🔖️DocumentHelpers` | `empty_energy_model_snapshot`, `model_from_snapshot`, `snapshot_from_model` | `🧬️schema/🦀️component.rs` (new `🔖️DocumentHelpers` region) | Rule 3 — pure codec helpers over the document type itself (encode/decode JSON body), not derived compute. |
| `⚙️engine/🦀️component.rs` `🔖️ArtifactEngine` | `struct EnergyModelEngine` + `impl` (`new`, `run_simulation`, `artifact`, `snapshot`) | **Deleted outright** | Rule 1. Repo-wide grep for `EnergyModelEngine` outside this one file → **0** hits before deletion. The struct's only "consumer" was its own `#[cfg(test)]` module in the same file — self-testing dead code, not real usage. **Not the same as `sim::Engine`** (below) — distinguished explicitly because both are named "Engine" but only one is the ticket's fossil pattern. |
| `⚙️engine/🦀️component.rs` `🚪️DerivedIoRegistry` | `io_registry` module (whole: `entries()`, `rebuild_native_snapshot`, 4× `compose_export_*`, dialects) | `🚪️io/🦀️component.rs` (new `🚪️DerivedIoRegistry` region) | Rule 5, verbatim move. All internal references were already fully `crate::artifacts::model::…`-qualified (not same-file-relative), so no rewrite needed beyond the move itself. |
| `⚙️engine/🟦️component.ts` | `export function register()` (throws — dead WASM stub, never mounted by `📦️index.ts`) | **Deleted outright** | Confirmed zero references anywhere (`📦️index.ts`'s barrel does not import it; nothing else does). Not a rule-1 "*Engine struct" case, just genuinely dead scaffolding. |
| Tests (3, in `⚙️engine/🦀️component.rs`) | `empty_snapshot_matches_schema` | `🧬️schema/🦀️component.rs` new `🧪️Tests` region | Tests the relocated `empty_energy_model_snapshot` helper. |
| | `engine_owns_artifact_not_snapshot_alias` | **Deleted** with `EnergyModelEngine` | Tests only the dead struct; nothing to preserve once the struct is gone. |
| | `example_fixture_parses` | `🧬️schema/🦀️component.rs` new `🧪️Tests` region | Zero `EnergyModelEngine` dependency (tests `dsl::parse_dsl` directly) — schema/document-level sanity check, relocated rather than deleted. |
| Tests (237, across the 50 domain dirs) | all `#[test]` fns in every domain | moved with their file, unchanged | Verbatim relocation — see assertion arithmetic below. |

## The `io_registry` shadowing hazard — found and neutralized

`🗿️artifacts/🔋️model/🦀️component.rs` (the artifact root, one level up from `✳️any`) already carries the
exact shadow the ticket warns about: its own `pub mod io_registry { fn entries() -> &'static
[&'static ComposerEntry] { ENTRIES.get_or_init(|| v1::entries().iter().collect()) } }`, where `v1` was
`use crate::artifacts::model::standards::v1::engine::io_registry as v1;` — a **qualified**
`engine::io_registry` import, not a bare one, so it was never at risk of silently rebinding to itself.
But it did need updating in lockstep with the move (it pointed at a path I deleted):

- `🗿️artifacts/🔋️model/🦀️component.rs:65` — `.composers(crate::artifacts::model::standards::v1::engine::io_registry::entries())` → `…standards::v1::subsets::any::io::io_registry::entries()`
- `🗿️artifacts/🔋️model/🦀️component.rs:138` — `use crate::artifacts::model::standards::v1::engine::io_registry as v1;` → `use …standards::v1::subsets::any::io::io_registry as v1;`

Both call sites were already **fully qualified** in the source (not bare `io_registry::entries()`),
so requalifying them was a mechanical find-replace with no ambiguity risk. I additionally grepped the
whole plugin for any *bare* `io_registry::` call that could have resolved to the wrong module after
the move — **0 found** (the only two occurrences are the two fully-qualified ones above).

## Unqualified paths found and how they were qualified

Only **one** real unqualified-path hazard existed in the whole plugin (the 50 domain files needed no
qualification at all, per the `#[path]`-decoupling argument above):

- `🧬️schema/🔺️diff/📝️text/🦀️component.rs:104,111` — two test bodies called
  `crate::artifacts::model::engine::empty_energy_model_snapshot()`, resolving through the
  `pub mod engine { pub use super::standards::v1::engine::*; }` shim in `📦️glue.rs` (also removed this
  packet, since `standards::v1::engine` no longer exists to `pub use *` from). Requalified to
  `crate::artifacts::model::schema::empty_energy_model_snapshot()`, which resolves through the
  pre-existing `pub mod schema { pub use super::standards::v1::subsets::any::schema::*; }` shim —
  confirmed that shim already re-exports the newly-added `empty_energy_model_snapshot`.

Found by grepping the whole plugin for `artifacts::model::engine::` and `model::engine::` before and
after every edit, not by pattern-substituting.

## Call sites updated

6 files, 8 non-mechanical edits, outside the 50 pure-relocation files:

- `📦️packages/🦀️rust/📦️glue.rs` — 50× `#[path]` repoint (`⚙️engine/<domain>/` → `🧬️schema/💡️inferences/<domain>/`, mechanical `perl -pi` substitution, verified with a post-edit grep diff); removed `pub mod engine;` inner mount (pointed at the deleted root file); removed the `pub mod engine { pub use super::standards::v1::engine::*; }` shim; updated 2 header doc-comment blocks and the `//#region ⚙️Engine` → `//#region 💡️Inferences` marker.
- `🗿️artifacts/🔋️model/🦀️component.rs` — 2 requalified call sites (`.composers(...)`, `use … as v1`) + doc-comment update.
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — new `🔖️DocumentHelpers` region (3 fns) + new `🧪️Tests` region (2 tests); added `ENERGY_MODEL_DOCUMENT_SCHEMA` to the existing `use` import (needed by `snapshot_from_model`, which the original engine file used and this move preserves byte-for-byte).
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — new `🚪️DerivedIoRegistry` region (verbatim `io_registry` module); updated stale top-of-file doc comment (`⚙️engine::register` → `this file's own io_registry::register`).
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — 2 test-body call sites requalified (see above).
- `✏️s/🔌️plugins/🔋️energy/🦀️component.rs` — **not edited**: its `crate::artifacts::model::engine::register()` mention is historical prose describing an already-removed call from an earlier packet (`W1d`, visible via `git diff HEAD`), not a live reference — left as-is.

## Assertion-count arithmetic (rule 8)

```
Before (git show HEAD, whole ⚙️engine tree, 51 files):
  #[test] count:        240
  assert!/assert_eq!/assert_ne! count: 365

Breakdown:
  root 🦀️component.rs:        3 tests / 6 asserts
  50 domain dirs combined:   237 tests / 359 asserts

After:
  50 domain dirs (moved verbatim, byte-identical):  237 tests / 359 asserts  ✓ unchanged
  🧬️schema/🦀️component.rs new tests:                   2 tests / 3 asserts
                                                     -----------------------
  Total:                                            239 tests / 362 asserts
```

**Delta: −1 test / −3 asserts**, exactly `engine_owns_artifact_not_snapshot_alias` (3 asserts),
deleted alongside the dead `EnergyModelEngine` struct it exclusively tested. Every other assertion
survives — verified by direct `grep -c "#\[test\]"` / `grep -oE "assert(_eq|_ne)?!"` on both the
pre-move tree (via the 50 dirs, whose content never changed) and the two edited destination files,
not by arithmetic alone.

## Structural verification

```
grep -rn "<artifact>::engine\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🔋️energy   → 0
find ✏️s/🔌️plugins/🔋️energy -path "*🗿️artifacts*" -name "⚙️engine" -type d                        → 0 (directory gone)
grep -rn "⚙️engine" ✏️s/🔌️plugins/🔋️energy                                                          → 9 hits, all doc-comment prose (7 in files I edited, narrating the relocation; 2 pre-existing historical mentions), zero live paths
grep -rln "EnergyModelEngine" . --include="*.rs"                                                     → 1 hit: my own new docstring prose in schema/component.rs naming the deleted struct, zero code references
grep -n "sim::Engine\|engine::" (repo-wide, energy scope)                                            → 1 unrelated hit: 🧬️schema/💡️inferences/site/🦀️component.rs referencing semio_s_plugin_stdio's OWN engine (a different plugin, out of scope, untouched)
git status --short (energy plugin)                                                                    → 50× clean R(enames), 2× D(eletes, root .rs + .ts), 7× M(odifications) — matches this report exactly
```

## Compiler verification

```
RUSTC_WRAPPER="" cargo check -p semio-s-plugin-energy --all-targets
```

Ran to completion (exit code 0 from the shell wrapper — `cargo check` itself terminates non-zero
internally but the command was confirmed to actually run: `Blocking waiting for file lock on build
directory` at the top of the log, 652 warnings + 8 errors emitted, ending in the `Finished`-equivalent
terminal line below — not a silently-skipped or cached no-op). **semio-s-plugin-energy's own source
was never reached**: `grep -c "🔋️energy"` over the full ~408 KB log → **0 matches, anywhere**. Cargo
resolved the dependency graph, started building the (mandatory, everyone-depends-on-it)
`semio-s-plugin-stdio` crate first, and aborted there before compiling energy at all:

```
error[E0432]: unresolved imports `crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_material_base_color`, `…diff_set_material_pbr`, `…diff_set_primitive_geometry`, `…diff_set_snapshot`, `…diff_set_texture_bytes`
error[E0432]: unresolved import `crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_snapshot`
error[E0061]: this function takes 2 arguments but 1 argument was supplied   --> …stdio/…/🧿️semio/…/✳️mesh/🧬️schema/🔺️diff/🦀️component.rs:436:8
error[E0061]: this function takes 3 arguments but 2 arguments were supplied --> …:449:8
error[E0061]: this function takes 4 arguments but 3 arguments were supplied --> …:455:8
error[E0061]: this function takes 4 arguments but 3 arguments were supplied --> …:488:8
error[E0061]: this function takes 2 arguments but 1 argument was supplied   --> …:514:8
error[E0061]: this function takes 2 arguments but 1 argument was supplied   --> …:566:8
error: could not compile `semio-s-plugin-stdio` (lib) due to 8 previous errors; 601 warnings emitted
```

**Per-error attribution — all 8, individually:**

**(c) upstream, all 8** — every single error is inside
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/…/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/…`
(the `mutations`/`diff` component files of stdio's own `s.semio`/mesh artifact) — `crate ✏️s/🔌️plugins/🗄️stdio`,
outside `✏️s/🔌️plugins/🔋️energy` entirely, a plugin this packet is explicitly forbidden from touching and
which the ticket documents as "currently RED... signature keeps changing." Verified not mine:
`git log -1` on the exact `🔺️diff/🦀️component.rs` file implicated by the 6 `E0061`s shows its last commit
is `a445617cae` (2026-08-12 15:50, before this session started), and `git status --short` on the whole
`stdio` plugin shows active staged `M`/`A` churn from a **different, concurrent** session (new `.spicy`/
`.protocol.semio`/`.abnf` files being added under `☁️las`'s own `⚙️engine`→schema migration) — none of it
touching the `🧿️semio`/mesh files that actually errored. This is exactly the "stdio is currently RED,
signature keeps changing" condition the ticket warns every packet about, not fallout from this one.

**Conclusion**: structurally complete and verified (grep + rename-tracked `git status` + assertion
arithmetic, all above). The compiler check **ran** (not skipped, not `--no-deps`, not a cached no-op —
file-lock wait and 652 real warnings prove it executed) but could not reach `semio-s-plugin-energy`'s
own compilation unit because a mandatory dependency (`semio-s-plugin-stdio`) fails first, on 8 errors
in files this packet never touched, 100% outside its scope (`✏️s/🔌️plugins/🔋️energy` only), and
pre-dating this session. **I cannot claim "0 errors in my own code" from compiler evidence alone** —
only from the structural/mechanical argument above (verbatim `mv` for 50/51 files, fully-qualified
paths for the one file that did change content, byte-identical re-export surface). This is reported
honestly rather than papered over.

## Files touched

Renamed (50, verbatim `mv`, zero content change):
- every `⚙️engine/<domain>/🦀️component.rs` → `🧬️schema/💡️inferences/<domain>/🦀️component.rs` for domain ∈ {air_exchange, air_system, airflow_network, calendar, coils, comfort, controls, curves, daylight, dispatch, economics, electrical, envelope, error, evaporative, fans, faults, fenestration, gains, geometry, heat_recovery, humidity_eq, hvac_topo, iaq, ideal_hvac, kernel, material, meters, metrics, model, num, output, plant, precompute, props, refrigeration, results, room_air, schedule, shw, sim, site, sizing, solar, solar_thermal, terminal, units, water, zone_air, zone_hvac}

Deleted:
- `⚙️engine/🦀️component.rs` (189 LOC — split into schema/io as above)
- `⚙️engine/🟦️component.ts` (5-line dead stub)
- `⚙️engine/` directory itself (now empty)

Modified:
- `📦️packages/🦀️rust/📦️glue.rs`
- `🗿️artifacts/🔋️model/🦀️component.rs`
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`

## Deviations from the region map (summary — see inline rationale above for each)

1. **No app exists** — rules 4/6/7 have no target; everything that would have gone app-side instead went to `🧬️schema/💡️inferences/` under rule 2, since the whole engine tree is the artifact's own derived-compute core, not UI/host behaviour.
2. **Two distinct "Engine"-named symbols, only one deleted** — `EnergyModelEngine` (dead, rule-1 fossil) vs `sim::Engine` (live, heavily tested, kept and rehomed under rule 2). Naming alone is not the test; construction sites are.
3. **Not wired into the formal `ArtifactInferrer`/`Inference<Snapshot>` machinery** — placed as physical siblings under `💡️inferences/` without touching `EnergyModelInference`, because that family's own law is a *total* function and the 50 domain modules are fallible/on-demand, a different contract.
4. **Zero of the 50 domain files needed a single content edit** — the pre-existing flat crate-root `pub mod <domain>;` + `#[path]` design in `📦️glue.rs` decouples logical module nesting from physical file location, so a pure filesystem `mv` plus repointing the 50 `#[path]` attributes was sufficient and left every internal `crate::…` reference (including the ones inside `sim::Engine::run` calling into all 49 sibling domains) untouched.

## Anything not verified

- **Compiler correctness of my own edits** — `cargo check -p semio-s-plugin-energy --all-targets` never reached energy's own source because dependency `semio-s-plugin-stdio` fails first (8 pre-existing, out-of-scope errors, see above). I could not independently confirm zero type/borrow errors in the ~10 lines of genuinely new/edited Rust (the `DocumentHelpers`/`io_registry` moves, the 2 requalified call sites, the 2 test-body fixes) via the compiler this run. Structural/mechanical verification (grep, rename-tracked git status, byte-identical content on the 50 pure-relocation files) is complete, but is not a substitute for a real compile and is reported as such.
- `RUSTC_WRAPPER="" cargo test -p semio-s-plugin-energy` was not attempted — `cargo check` already couldn't get past the `stdio` dependency, so a `test` run would fail identically before reaching energy's own tests.
