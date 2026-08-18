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

- Added `semio-framework-actor = { workspace = true }` (needed for `ActorId`/`Lane`/
  `lane_defaults` — `semio_framework::kernel` deliberately does NOT re-export `RuntimeActorId`, per
  its own doc comment in `🎠️kernel/🦀️component.rs`: "this packet must not depend on it").

`🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`: **checked, no matching gap** —
this file is a pure type-definition module (`ExtensionManifest` schema etc.), not a runtime loader;
it has no `load_runtime_recursive`-shaped function and no "NOT YET WIRED" comment. Nothing to wire
here for this packet.

## 3. Known gaps (honest, not papered over)

- **Live `describe()` fallback is not wired.** Implementing it properly means either (a) adding a
  `GuestRuntime::describe`-shaped seam to `🔌️plugin/🖥️host/🦀️component.rs` — **out of `path_scope`,
  and P1/T1 are live in that exact file right now** — or (b) hand-rolling a second wasmtime+WASI
  linker setup directly inside `🏃️run/🦀️component.rs`, which would put raw `wasmtime` calls outside
  the `GuestRuntime` interface CLAUDE.md requires external libraries to stay behind. Neither is safe
  to do inside this packet's `path_scope`. **Lease-request**: a follow-up packet should add
  `GuestRuntime::describe(&self, compiled: &CompiledHandle) -> Result<PackageDescriptor,
  PluginHostError>` (fuel-capped, `pure`+WASI-only linker, mirroring `describe_component`'s own
  setup) so `read_committed_descriptor`'s caller can fall through to it when no committed file
  exists.
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

(filled in after the build/test run below)

## 5. Lease-requests

- `🔌️plugin/🖥️host/🦀️component.rs`: add a `GuestRuntime::describe` seam so
  `WasmtimeNodeHost::read_committed_descriptor` can fall through to a live `describe()` call for a
  plugin with no committed descriptor yet. Not editable this packet — P1 and T1 are live there.

## peer-coexistence

(filled in below)

## Commands + exit codes

(filled in below)

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml`
