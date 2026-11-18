# 📓️ Release components through describe (fem, demonstrator, energy)

## Status at hand-off

| Component | Release wasm | Describe | Note |
|---|---|---|---|
| fem | 137 224 732 B after the LazySlots fix (was 204 704 697) | ABI discovery and channel handshake pass; `describe()` now stops on the caller clock (fix 2b below, needs a describe-tool rebuild to validate) | built natively in 57 min at `-j1` |
| demonstrator | 519 MB (sibling log) | not rebuilt here | over the 256 MiB shipped cap |
| energy | 612 MB, code section 525 MB (sibling log) | not rebuilt here | over the 256 MiB shipped cap |

## Root cause 1: "no core module implementing the owned Semio actor ABI"

`OwnedSemioArtifact::from_component` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🔌️owned-semio-boundary/🦀️.rs`) selects the core module whose exports match `OwnedSemioExport::core_type()` exactly. In the committed tree `core_type()` listed `Checkpoint`, `Describe`, `Restore`, `PackSchemaHash`, `Genesis`, ... all as 19 x i64 parameters, but the guest macro (`📣️macros/⚛️wp4-reactor/🦀️.rs`) exports `() -> i64` (checkpoint, describe) and `(i32, i32) -> i64` (restore, codec, sqlite). No module can match, so every real component fails. A concurrent edit in the working tree (18:07) already corrected those arms; I verified every arm against a scripted extraction of the 36 `semio_owned_*` guest signatures (`wasm-exports.py` in this folder) and against the real fem core module (`describe` `() -> i64`, `poll` 19 x i64, `restore` `(i32, i32) -> i64`).

Result in the fem run: `[describe] owned phase=compile` passed, i.e. the ABI module was found.

## Root cause 2: "owned call returned an invalid pointer/length pair" (found by the fem run)

`semio_owned_channel_version_v1` is the one export whose guest and host disagreed:

- guest macro (since 3904c31bd05) returned a scalar `u32`; `core_type()` said `() -> i32`;
- host `admit_owned_channel` (`🏃️runtime/📣️host-replies-wasi-and-channel-faults`), `OwnedStage::Call` (`⏯️operation-begin-resume-and-cancel`), the startup state machine (`🏗️startup`, `Phase::ChannelParsing`) and the synthetic fixture (`🧫️fixtures/🌱️channel`) all expect a packed `(pointer, length) -> i64` carrying the JSON integer.

Fix (host side stays untouched, the guest returns the pair like every other export):

- `📣️macros/⚛️wp4-reactor/🦀️.rs`: `semio_owned_channel_version_v1() -> u64 { return_json(&channel_version()) }`;
- `owned-semio-boundary/🦀️.rs`: `ChannelVersion` joins `Checkpoint | Describe` as `() -> i64`.

The WIT-level `channel_version() -> u32` (component model export) is unchanged.

Evidence: `🗑️generated/components/fem-4.log` (fem rebuilt with the new guest macro, 153 units compiled, 737 reused, 39 min): the channel handshake and ABI discovery pass; the run then stops at root cause 2b.

## Root cause 2b: "native io: original interpreter deadline requires its caller clock"

`OwnedRuntime::describe_observed` (`🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs`) resumes with `OwnedDeadline::Original`, which reads the caller clock from `state.original_job_clock`; only `job_call` and the startup path ever bind it, so the descriptor tool (which passes an absolute `default_now_us() + DESCRIBE_DEADLINE_MS` deadline, `describe_budget()`) always fails with this refusal. Fix: `describe_observed` binds `(budget.retained.work.clock_binding, self.original_clock)` right after `instantiate_actor`. Not yet validated: the describe tool (`semio-plugin-describe`, built by `describe-build`) has to be rebuilt first; the stale binary (19:58) makes the component stage stop with `Original mutable compiler input changed` (`fem-5.log`), and the rebuild (`describe-build-2/3.log`, native `repo ./📜️script.ts build --host-platform=darwin`) starved on `[repo cargo] all 2 live lanes are in use (maximum 32); waiting for a free lane` while other agents' cargo runs held the lanes.

Next run (nothing else needed): `describe-build` (cwd `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust`), then the fem component command from the task; the unit cache reuses all 890 units, so the run is about 5 minutes plus the describe call.

## Root cause 3 (size): release wasm is far above what the profile should give

The release wasm profile is already `opt-level=s`, `lto=thin`, `codegen-units=16`, `strip=symbols`, `debug=0` (schema `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json`, `release-wasm`), unchanged since 2026-10-16, so the growth is in the code, not the profile. The shipped cap is 256 MiB (`ComponentProfilePolicies.release`), the 512 MiB number is the prototype/iterate cap and `DESCRIBE_ARTIFACT_MAX_BYTES`.

## Size root cause found and fixed (validated on fem)

Owning statics: four `thread_local!`/`component_persistent_local!` tables `RefCell<[Option<T>; 1024]>` (`REQUEST_SLOTS`) in `⚛️reactor`: `RETAINED_EXTENSION_INVOCATIONS` (`🔄️turn`), `RESPONSES` (`🔄️turn/📥️receiving`), `STAGED_EVENTS` (`📥️receiving/forms.rs`), `JOBS` (`💼️jobs`). The `None` niche of those `T` is not all-zero, so each 1024 x ~7-30 KB table was emitted as an active data segment (about 68 MB of zero bytes per component; found by matching the three table periods 23 360 / 29 960 / ~6 800 B x ~1000 slots to the `REQUEST_SLOTS = 1024` tables, not by a linker map).

Fix at the source: new `semio_framework_async::LazySlots<T, N>` (`🧰️framework/🔨️modules/⏳️async/💤️lazy-slots/🦀️.rs`, with a unit test): an empty `OnceCell<Box<[Option<T>]>>` (all-zero, so `.bss`, zero image bytes) that builds the heap table on first access with `Vec::with_capacity` + `resize_with`. The four statics now use it; call sites are unchanged (slice methods through `Deref`/`DerefMut`).

Measured (fem-6, full release rebuild, 892 units, 72 min): wasm 204 704 697 -> 137 224 732 B (-67.5 MB, 131 MiB); data section 76.5 MB -> 9.05 MB; code section unchanged (127 MB). Every component carries these tables, so each shrinks by about 67 MB.

Projected, not built: demonstrator 519 -> about 450 MB, energy 612 -> about 545 MB. Both stay above 256 MiB because their code sections (hundreds of MB) dominate. The remaining levers need a profile change (a full release rebuild of everything and a new policy digest), so I did not apply them while the coordinator's play release driver is running:

1. `-Zlocation-detail=none` + `-Zfmt-debug=none` for `wasm32-wasip2` release only. `.cargo/config.toml` rustflags are shared by dev, so this needs cargo's per-profile `rustflags` (`-Zprofile-rustflags`, availability on the pinned nightly unverified) emitted by `renderCargoProfileConfigurationV1`, driven by `release-wasm` in the profile schema.
2. `release-wasm.optLevel` `s` -> `z` (schema `🧬️schema/🔣️.json`), typically 5-10 % of code.
3. Only after 1-2: fat LTO / `codegenUnits: 1` for composition leaves (memory: leaf already peaks 1.6 GiB under thin LTO).
4. Structural: demonstrator and energy link the whole stdio format fleet and every artifact editor; a code section of that size needs the composition split (separate components) rather than flags.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📣️macros/⚛️wp4-reactor/🦀️.rs` (channel version returns the packed JSON pair)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🔌️owned-semio-boundary/🦀️.rs` (`ChannelVersion` signature; the other arms were the concurrent 18:07 fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs` (`describe_observed` binds the original clock)
- `🧰️framework/🔨️modules/⏳️async/💤️lazy-slots/🦀️.rs` (new), `🧰️framework/🔨️modules/⏳️async/🦀️.rs` (export), and the four tables in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/` (`🔄️turn/🦀️.rs`, `🔄️turn/📥️receiving/🦀️.rs`, `🔄️turn/📥️receiving/forms.rs`, `💼️jobs/🦀️.rs`)
- ticket helpers: `native-generate-closure.sh`, `wasm-exports.py`

## Process finding: native component builds need generated sources first

The native route (no graph) fails with `RepoError process.source.missing-input 🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🦀️.rs` until the closure's generators ran. `native-generate-closure.sh` runs the six that fem needs (framework-schema, graph, ui-contract, ui-styling, ui, dsl-derive, framework root `generate`); the wrapper needs the working directory `"."` (an empty string fails `Original native caller capabilities required`), `SEMIO_PLUGIN_CATALOG` set to the generated catalog, and about 55 minutes at `-j1` for a cold fem closure.
