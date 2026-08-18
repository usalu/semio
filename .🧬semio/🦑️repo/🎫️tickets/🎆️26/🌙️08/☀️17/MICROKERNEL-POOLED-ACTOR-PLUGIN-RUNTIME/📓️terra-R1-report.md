# 📓️ terra R1-native-manifest report

## 1. Committed-artifact-vs-live-describe decision

**Decision: prefer the committed `🛂️descriptor.semio` (zero instantiations); live `describe()` is a
designed-but-not-yet-wired fallback.**

Evidence:

- `📓️design-abi.md` §3 (line 91): the freshness gate `descriptor_is_fresh()` byte-compares a
  natively-assembled `describe()` against `<crate>/🤖️generated/🛂️descriptor.semio` — the descriptor
  is treated as the artifact of record, describe() as its build-time producer, not a runtime source.
- `📓️design-abi.md` §3 (line 93): the emitter (`semio-framework-plugin-describe`) runs
  `describe <component.wasm> --out <dir>` **once, at build time**, writing `🛂️descriptor.semio` +
  `🔣️descriptor.json` next to the plugin crate.
- `📓️design-runtime.md` §3 (line 107): `ActivationRegistry` is seeded from "manifest-only records...
  + **build-time descriptors**" — the runtime path is explicitly build-time-descriptor-driven, not
  live-describe-driven.
- `📜️component.wit`'s own `describe` interface doc (quoted verbatim inside
  `🔌️plugin/🖥️host/🦀️component.rs`'s `IoRouter::register_plugin` doc comment): "build-time only,
  never called at runtime."
- The ticket packet brief itself states the premise directly: "2550 installed records consume zero
  runtime resources; static `PackageDescriptor` emitted at build, zero instantiations at load" — and
  explicitly names live `describe()` as the *fallback*, not the primary path, for a plugin with no
  committed descriptor yet.

So `load_runtime_recursive` tries `descriptor_path_for_plugin.get(plugin_id)` — a
`🛂️descriptor.semio` path derived from the plugin registry's own `cratePath` (`🔣️plugins.json`,
`📦️bin.rs`'s new `resolve_descriptor_paths`) — first. Only a plugin with no entry there, or whose
file does not exist on disk, fails to load. **The live-`describe()` fallback is NOT wired in this
packet** — see §3 "Known gaps" below for why, and the lease-request in §5.

## 2. What was wired

`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`:

- `WasmtimeNodeHost` gained a `descriptor_path_for_plugin: HashMap<String, PathBuf>` field
  (`new()`'s signature grew a matching parameter) and an `app_router()` accessor (mirrors the
  existing `plugin_graph()`/`mutation_router()`/`inference_router()`/`instance_directory()`
  accessors — needed by the new native-load test).
- New `read_committed_descriptor(plugin_id)`: reads the `.semio` file, decodes it with
  `store::pack_rt::decode_wire_value` + `dsl::from_dsl_value::<PackageDescriptor>` — the exact
  decode `🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`'s `describe_component` uses on a live
  `describe()` return, applied here to the committed bytes on disk instead.
- `load_runtime_recursive` now does the full walk the old "NOT YET WIRED" comment specified:
  compile → decode descriptor (zero instantiation) → recurse over `manifest.dependencies` → mint a
  `RuntimeActorId` (`semio_framework_actor::ActorId::new(0, 0, ordinal, 0)`, `next_actor_ordinal`
  counter, no longer dead code) → `runtime.instantiate` → wrap in `PluginInstanceHandle` → register
  with `io_router` (composer_entries → `artifact_dialect_entries`), `mutation_router`
  (`register_roster`, mapped from `contributions.mutation_services` + `.artifact_contributions[].
  mutations`), `inference_router` (`register_plugin` with a JSON roster built from
  `contributions.inference_services` + `.artifact_contributions[].inferences`), `plugin_graph`,
  `app_router.register_manifest` → the `owned_surface_gaps` hard-gate → `self.manifests.insert`.
- `next_actor_ordinal`'s `#[allow(dead_code)]` removed (now genuinely read).

`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`:

- New `resolve_descriptor_paths(repo_root)`: reads the registry's generated `🔣️plugins.json`
  (`plugin-registry:generate`'s own output — the SAME generator `resolve_plugin_paths` above it
  reads `🦀️artifacts.rs` from), maps each row's `cratePath` two levels up (past
  `📦️packages/🦀️rust`) to the plugin crate root, appends `🛂️descriptor.semio`. A plugin id absent
  from the JSON, or whose descriptor file does not exist, is simply not in the resulting map —
  `WasmtimeNodeHost` treats that as a normal per-plugin load failure, not a startup hard error.
- `main`'s `WasmtimeNodeHost::new(...)` call updated to pass the new map.

`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml`:

- Added `semio-framework-actor = { workspace = true }` for `ActorId` — `semio_framework::kernel`
  deliberately does NOT re-export `RuntimeActorId` (its own doc comment in
  `🎠️kernel/🦀️component.rs`: "this packet must not depend on it").
- 🐛️ **Found and fixed while wiring `instantiate`'s budget**: `GuestRuntime::instantiate`'s `Budget`
  parameter is `semio_framework::kernel::Budget` (`reactor::poll`'s shape — fuel/deadline_ms/
  max_effects/max_patch_bytes/max_frames), NOT `semio_framework_actor::Budget` — a DIFFERENT struct
  with the SAME name (`🎠️kernel/🦀️component.rs` vs `🎭️actor/🦀️component.rs`, both `pub struct
  Budget`). First `cargo check` failed with `E0433` trying `semio_framework_actor::lane_defaults`/
  `Lane` (design-runtime.md's planned kernel re-export of those never actually landed). Fixed by
  building the budget literal directly, mirroring the closest real precedent for a one-off
  compile+instantiate — `🌉️mcp/🏠️workspace/🦀️component.rs`'s `activate_plugin_instance`.

Added a `#[test] note_plugin_manifest_loads_from_its_committed_descriptor` at the end of
`🏃️run/🦀️component.rs`'s existing `mod tests` — see §4.

`🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`: **checked, no matching gap** —
this file is a pure type-definition module (`ExtensionManifest` schema etc.), not a runtime loader;
it has no `load_runtime_recursive`-shaped function and no "NOT YET WIRED" comment. Nothing to wire
here for this packet.

## 3. Known gaps (honest, not papered over)

- **Live `describe()` fallback is not wired.** Implementing it properly means either (a) adding a
  `GuestRuntime::describe`-shaped seam to `🔌️plugin/🖥️host/🦀️component.rs`, or (b) hand-rolling a
  second wasmtime+WASI linker setup directly inside `🏃️run/🦀️component.rs`, which would put raw
  `wasmtime` calls outside the `GuestRuntime` interface CLAUDE.md requires external libraries to
  stay behind — (a) is the right shape. P1/T1 finished in that file partway through this packet
  (their commit `ee16e76c4e` landed while I was waiting on a build), so the lease that blocked this
  earlier is no longer live — but implementing it was explicitly descoped as new work this late in
  the packet (coordinator instruction), so it is left as a **lease-request/follow-up**, not
  attempted. §5.
- **`ContributionSet.io_entries` cannot be mapped to `io_schema::IoEntryDescriptor`.** The manifest
  crate's `IoEntryDescriptor` (owner/counterpart/direction — `🛂️manifest/🦀️component.rs`) and the io
  crate's `IoEntryDescriptor` (from/into/fidelity/sniffs — `🚪️io/🧬️schema/🦀️component.rs`) are two
  DIFFERENT types with the same name; the descriptor schema carries no `fidelity`/`sniffs` data, so
  there is no honest way to populate `IoRouter::register_plugin`'s `io_entries` roster from a
  descriptor today. `load_runtime_recursive` passes `&[]` for it, documented in-line. `note`'s own
  descriptor has zero `io_entries` regardless, so this is invisible on the one fully-wired smoke
  path. A future packet (E1/A2) must either widen `ContributionSet.io_entries` with those two
  fields at emission time, or resolve them from `IoFidelityDeclaration` some other way.
- **`run_transaction`/`undo_transaction_group`'s `exec`/`plan` closures are UNCHANGED** — still
  return `TransactionError::rejected("transaction.not-wired", ...)`. That gap is one layer up
  (post-turn effect dispatch over `execute_turn`, H1-H4/T1's job) and was never in this packet's
  scope; the struct's own doc comment already said so before this packet started.

## 4. Native smoke — how far it gets

**Real, verified: `🗒️note` loads natively end to end** through the newly-wired path — compiled,
descriptor decoded from disk (zero instantiation for the manifest read), instantiated as a
`GuestInstance`, registered with every router, `owned_surface_gaps` clean. Proven by
`note_plugin_manifest_loads_from_its_committed_descriptor` (§2), run with `--nocapture` to confirm
it exercised the real path rather than its soft-skip branch (no `[DEBUG] ... SKIPPED` line; 2.68s
wall time — real wasmtime compile+instantiate work, not a 0.00s no-op). The test needed a compiled
`note` wasm that did not exist anywhere in the tree at verification time (the D0 packet's earlier
build, which a first pass of this test depended on, was gone — its own ticket-scoped target dir was
cleaned between when I first wrote the test and when I actually ran it), so I built one myself,
foreground, single build:
`CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo build -p semio-s-plugin-note --target wasm32-wasip2`
(exit 0, §"Commands"). The test also tries the canonical `target/wasm32-wasip2/{debug,wasm-release}`
locations first, so it will pick up a real dev-shell build transparently once one exists there.

**Honest count for the other 32 plugin ids**: as of this check, `find ✏️s/🔌️plugins -maxdepth 2
-iname 🛂️descriptor.semio` returns **10** committed descriptors (🕸️dag, 💡️reasoning, 🎬️sequence,
✒️writer, 🎞️animate, 🌿️vcs, 🪵️sourcing, 🗒️note, 📋️forms, ➗️mathematical) — up from the 4 the
coordinator named moments earlier, confirming sibling packet D0 is actively emitting more while
this packet ran. This packet did **not** build wasm for, or attempt to load, the other 9 — that
would have meant 9 more multi-minute `wasm32-wasip2` builds, explicitly out of scope as new work at
this point in the packet. The blocker for the remaining ~23 plugin ids is squarely **D0's own
emission plumbing**, not this packet's wiring — `read_committed_descriptor` returns a clear,
correctly-worded error naming exactly that for any plugin without a descriptor file yet (§2's
`read_committed_descriptor` doc). Once D0 lands a descriptor for a given plugin AND its wasm is
built, this packet's `load_runtime_recursive` should load it with no further changes — the one
path-dependent risk being the `io_entries`/mutation/inference mapping gaps in §3 for a plugin whose
descriptor actually populates those (unlike `note`, whose descriptor happens to leave them empty).

## 5. Lease-requests

- `🔌️plugin/🖥️host/🦀️component.rs`: add a `GuestRuntime::describe` seam so
  `WasmtimeNodeHost::read_committed_descriptor` can fall through to a live `describe()` call for a
  plugin with no committed descriptor yet. The file is no longer occupied by P1/T1 (they finished
  mid-packet), but this was explicitly descoped as new work this late in the packet — a real
  follow-up, not a blocked one.

## peer-coexistence

- Start-of-packet liveness (`git log --date=iso --oneline -5` + `pgrep -fl cargo`): `🏃️run/**` last
  touched by `abd29c08d0` (unrelated, ticket 535-series). `🔌️plugin/🖥️host/🦀️component.rs` at
  `abd29c08d0` too. No cargo processes running yet.
- Mid-packet: `pgrep -fl cargo` showed FIVE concurrent cargo invocations sharing this workspace —
  `cargo check -p semio-framework-plugin-host` (P1/T1), `cargo check -p semio-framework-os-run`
  (mine), `cargo check --workspace ...` (an IDE background check), `cargo test -p
  semio-framework-actor` (K1), `cargo build -p semio-s-plugin-forms --target wasm32-wasip2` (D0-ish)
  — exactly the "five sibling packets" the packet brief warned about. My own `cargo check` sat at
  0% CPU for ~9 minutes waiting on a shared lock before it started making progress; not a hang.
  `🔌️plugin/🖥️host/🦀️component.rs`'s content visibly drifted between two of my own reads minutes
  apart (function bodies at the same `grep`-reported line numbers came back different) — P1/T1
  editing live, exactly as the ticket brief said to expect. I only READ that file; nothing in my own
  diff touches it.
- End-of-packet: `🔌️plugin/🖥️host/🦀️component.rs` is now at commit `ee16e76c4e` (P1/T1 landed while
  I waited on my own build) — every signature I coded against (`IoRouter::register_plugin`,
  `ArtifactMutationRouter::register_roster`, `ArtifactInferenceRouter::register_plugin`,
  `PluginGraph::register`, `AppRouter::register_manifest`/`owned_surface_gaps`,
  `PluginInstanceHandle::new`, `GuestRuntime::instantiate`) still compiled clean against that final
  state — confirmed by the passing `cargo check`/`cargo test` runs below, run AFTER their commit
  landed.
- No edits made anywhere under `🔌️plugin/🖥️host/**` — read-only throughout, per `path_scope`.

## Commands + exit codes

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo check -p semio-framework-os-run --all-targets
   Finished `dev` profile [unoptimized] target(s) in 19.71s
EXIT_CODE=0

$ CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo test -p semio-framework-os-run --lib
running 16 tests
test run_lib::tests::note_plugin_manifest_loads_from_its_committed_descriptor ... ok
(...15 other tests, all ok)
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.57s
EXIT_CODE=0

$ CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo build -p semio-s-plugin-note --target wasm32-wasip2
   Finished `dev` profile [unoptimized] target(s) in 5m 52s
EXIT_CODE=0

$ CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo test -p semio-framework-os-run --lib note_plugin_manifest_loads -- --nocapture
running 1 test
test run_lib::tests::note_plugin_manifest_loads_from_its_committed_descriptor ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 2.68s
EXIT_CODE=0
```

Intermediate failing run (before the `Budget` type fix in §2), for the record — first `cargo check`
attempt failed real compile errors (`E0433: cannot find lane_defaults/Lane in kernel`), exit
non-zero (build failed, exact code not captured since it was piped through `tail`); fixed, then all
four commands above are the clean re-runs, every one exit 0.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml`
