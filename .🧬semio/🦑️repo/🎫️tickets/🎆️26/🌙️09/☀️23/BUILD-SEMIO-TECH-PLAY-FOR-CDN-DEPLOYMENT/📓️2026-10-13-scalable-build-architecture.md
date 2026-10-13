# 🏗️ Scalable Build Architecture (2026-10-13)

This is a read-only design with an execution plan. No source file was edited. Its goal is a build and crate architecture that stays clean and fast at 10× today's plugins, artifacts, subsets and apps, and that keeps all 10 cores of the build machine busy from start to finish.

It builds on these ticket notes and does not repeat them:

- `📓️2026-10-13-stdio-semio-split-plan.md` (the split plan; its waves 0–4 are integrated here as S0–S3 and M0)
- `📓️2026-10-12-build9-timeline-audit.md`
- `📓️2026-10-12-provenance-perf.md`
- `📓️2026-10-10-release-verification-path.md`
- `📓️2026-10-10-migration-corrections.md` (#56, #57, #68, #72, #75, #79)
- `📓️2026-10-10-coordinator-handover.md`

**Inputs I created.** Both are kept as input files in `build-architecture/`:

- `🕸️unit-graph-census.py`: one `cargo build --unit-graph` over a prebuild plan, under both feature-unification modes. It compiles nothing.
- `🧮️build-dir-unit-census.py`: reads the shared build-dir (fingerprint start time, rlib time, rlib size, dep-info lines).

I compiled nothing, so no private `CARGO_TARGET_DIR` was created and none had to be deleted. Wave 0 was edits in flight while I measured, so the unit-graph numbers describe the tree at 10-13 ~03:30.

Labels used throughout:

- **Measured**: comes from a log, a timings file, a unit graph or a manifest census.
- **Estimate**: derived from measured anchors, with the derivation shown. Wave M0 replaces every estimate with a measurement.

---

## 1. Decisions at a glance

| # | Decision | Why, in one line |
|---|---|---|
| D1 | **One unit per crate per profile per target**, enforced by the resolver (`feature-unification = "workspace"`) and by a gate. First-party crates have no consumer-dependent features. | Today 1 crate → up to 16 units. Workspace unification measured 381 → 372 units on today's plan. Behaviour stays correct only if features carry no behaviour. |
| D2 | **Size law per unit:** library ≤ 15 k own lines and ≤ 3 GiB peak (fail at 25 k / 6 GiB); component link ≤ 6 GiB (fail at 10 GiB). | 26 GiB usable ÷ 10 cores = 2.6 GiB per core. Offenders measured: semio 125 k lines / 13.3 GB, framework-plugin 87.6 k, os-kernel 83.6 k, stdio-pdf 51.9 k. |
| D3 | **Library XOR component:** a crate is either an `rlib` library or a thin `cdylib` component crate, never both. | 8 libraries (os-kernel, os-flow, actor, …) link a dead wasm cdylib in every pass. os-kernel's cdylib turns kernel → plugin into a full (non-pipelined) edge: 115 s on the critical path instead of 24 s. |
| D4 | **Artifact family pattern:** base, subsets, import/export conversions, aggregator, catalog. Explicit `CONVERSION` registration lists are generated from schema. This generalises the stdio-semio split to every family. | Linear growth: a new subset adds crates, never features or units. |
| D5 | **Hubs compose by depending on aggregator/catalog libraries.** A hub is a generated library plus a one-macro component crate. The generic boundary sits at the aggregator. | Kills the `plugin-entry` default feature and the embedding problem, and makes the per-component final unit as small as possible. |
| D6 | **Two Cargo workspaces** replace 273: `product` (everything that ships or is product-tested) and `tools` (generators, probes, oracles, bridges). One lockfile and one profile set each. | 8 drifting profile sets exist today. The 26 `✏️s` extension components rebuild their whole closure under other profiles (≈ 1.8 CPU-h per profile). Oracles must not unify features into product. |
| D7 | **Exactly one shipped component profile** (`wasm-release`). Describe, provenance, materialize, receipts and the trusted catalog all act on the shipped bytes. Committed descriptors become **semantic only**; byte hashes move to generated **component receipts**. | Today every component compiles twice. wasm-dev is slower than wasm-release (raster cold 918 s vs 317 s) and has no incremental benefit (`incremental = false`). Descriptors pin bytes that are never shipped. |
| D8 | **Dev loops:** native `check`/`test` plus a `wasm-iterate` profile for the one plugin being edited (incremental, no LTO, debug assertions). All other plugins come from the warm release outputs. Iterate bytes are never described for commit and never shipped. | Fast loop without polluting the release graph. |
| D9 | **One build driver task per release** owns cargo (one pass per target and profile), a jobserver shared with all post-link stages, memory admission through a rustc wrapper that calibrates itself from recorded peaks, streaming stages, bounded output and telemetry. The nx release graph shrinks from ≈ 375 to ≈ 10 tasks. | Build #9 ran 98 % serially. Build #10 died because the one-pass prebuild runs inside a consumer task with a 1 h deadline, at 2 jobs. |
| D10 | **Caching:** cargo freshness is the sole authority for Rust (nx `cache: false` on cargo tasks). Post-link stages are cached by output content digest. One shared warm build-dir. The fresh build stays cold but is linked to warm builds by a **determinism gate**. No sccache. | Removes the nx hasher blind-spot class. Keeps the cold proof without paying for it on every iteration. |
| D11 | **Gates:** unit census, feature census, size and memory budget, path budget, determinism, output budget. Each build records a CPU and memory timeline, a per-unit table and the critical path. | Every regression above has bitten this ticket at least once. |
| D12 | **Static composition stays for now, with a codified trigger** for moving a family into its own separately linked component. stdio-semio meets the trigger today; spike C2. | Relink fan-out (34 of 44 components link stdio-semio) is the remaining term that does not scale. |

**Expected result:**

- Cold play release: from ≥ 8 h 17 min (build #9, unfinished) to **≈ 18–23 min** after all waves, at **≥ 85 % CPU** during compile.
- Warm release after a leaf-plugin change: 2–3 min.
- Single-plugin iterate: 20–60 s.

Details are in §9.

---

## 2. Measured facts (new in this note, plus the anchors used)

### 2.1 Build #10 (new pipeline, `⚡️cache/fChPzBn`) failed structurally

- **Duration and exit.** Start 23:40:25, exit 1 at 02:01:08, after 140 min. Log: `fChPzBn/o.log`.
- **The wasm-dev pass.** It is hosted inside `@semio-tech/cad-plugin:component-dev` and ran at `jobs = 2`. After 3,296 s it had linked **1 of 44** components and compiled **208 units**.
- **The 1 h deadline.** The hosting task's `native:owner-command` child budget is 1 h (`📋️native-orchestration/🔣️policy.json` `child.maximumElapsedMilliseconds = 3600000`). It killed the pass: `stopped: timeout 3600000ms`.
- **The release pass.** It started under the next claimant, `gis-plugin`, and also reached 1 of 44 after 2,369 s.
- **Every waiting component** (demonstrator, gis, playbook, process, procedural, sourcing) timed out at the same 1 h.
- **Cause.** The pass is a side effect of whichever narrow task claims it first. It inherits that task's deadline and its memory cap of 2 jobs (`bytesPerJob` 10 GiB from the 14–20 GiB semio units). It is invisible to nx.

### 2.2 A single component, cold: the critical path is measured

Source: `⚡️cache/py/t/cargo-timings`, `semio-hub-process`, wasm-release, codegen-units 1, `jobs = 10`, 158 units.

- Wall 925 s. Sum of unit durations 2,521 s. Average 2.73 units in flight.
- **Exactly one unit ran for 483 s** (52 % of the wall).

Critical chain, read from the timings' unblock edges:

| Unit | Start s | Duration s | Edge to next |
|---|---:|---:|---|
| proc-macro2 → quote → syn → value-derive | 0.8 | 8.6 | |
| semio-framework-value | 9.4 | 9.2 | rmeta |
| semio-framework-replication | 13.4 | 37.0 (frontend 7.5) | **full** (kernel links a cdylib) |
| semio-framework-os-kernel | 50.4 | 115.4 (frontend 24.1, codegen 91.3) | **full** |
| semio-framework-plugin | 165.8 | 82.7 (frontend 20.0) | rmeta |
| semio-s-artifact-stdio-contract | 185.8 | 19.3 (frontend 2.5) | rmeta |
| semio-s-artifact-stdio-semio | 188.3 | **519.5** (frontend 77.8) | full (link) |
| semio-hub-process (component) | 707.8 | **217.0** (frontend 17.5, codegen and thin LTO 199.5) | |

The `full` edges into and out of os-kernel exist because os-kernel is `crate-type = ["rlib", "cdylib"]`. Cargo cannot pipeline on rmeta, because the unit also links a cdylib.

### 2.3 Library units that link a dead cdylib

Source: `🕸️unit-graph-census.py` over the 44-package plan.

- 52 cdylib units. **8 are library dependencies, not components:** `semio-framework-actor`, `-editor`, `-math`, `-os`, `-os-flow`, `-os-kernel`, `-surface` and `semio-s-spatial-kernel-semio-session`.
- All 44 hub components are `["cdylib", "rlib"]`. An embedding hub therefore waits for, and pays for, a cdylib link it never uses.
- Repository-wide, 85 manifests declare both crate types.

### 2.4 Feature unification, measured on today's plan

Wave 0 was mid-flight, so these numbers reflect partial wave 0 edits.

| Unification mode | Units | Distinct keys | First-party units | Duplicated keys |
|---|---:|---:|---:|---|
| `package` (what the prebuild uses) | 381 | 370 | 238 | semio ×8, hub-stdio ×2, serde/serde_core/serde_json ×2 |
| `workspace` | **372** | 370 | 230 | serde_core ×2, serde_json ×2 |

- Wave 0's own census (`🗑️generated/wave0/census-before.txt`) measured **708 units (375 keys, 43 crates drifting)** before wave 0, and 388 at its midpoint.
- Under workspace unification the semio monolith becomes 1 unit even before the split. It would then carry all 59 features, which would change registration behaviour and keep the 11.8–13.3 GB peak. The split is still required; D1 depends on it.

### 2.5 Workspaces, profiles and features (manifest census, ticket scratch excluded)

- **273 Cargo workspaces.** 226 of them have a single member, and 275 lockfiles exist.
- **8 distinct profile sets.**
  - 136 workspaces copy the `✏️s/Cargo.toml` set: wasm-release `codegen-units = 1`, wasm-dev at `opt-level` 0.
  - The hub uses cgu 16 and dev opt-level 1.
  - The root has a third variant.
- **Consequence for the 26 `✏️s` extension components.** Their release builds compile their closure again under their own profile and lockfile.
  - Build #9 release cargo wall for those 26 tasks was **2,614 s**. Examples: playbook-extension-procedural 634 s, process-extension-concrete 430 s, cad-extension-aec-building 418 s, sourcing-extension-beams 342 s, flow-extension-brep 329 s.
  - That is ≈ 1.8 CPU-h per profile (estimate, at ≈ 2.5 cores).
- **444 first-party packages, 161 of which declare features.**
  - Classes: test-only (`oracles` 47, `testing`, `testkit`, `cross-fem` 16, …), composition (`component-app-assembly` 23, `conversion-*` ≈ 60), entry selection (`plugin-entry` 11, `plugin-root` 10, `component-guest` 11, `extension-entry` 5), tooling (`typegen` 8, `mutation-inventory` 7) and backends (`sqlite`, `postgres`, `neo4j`, `ureq`).
  - `plugin-entry` is default-on, and it is switched **off** by an embedding crate. It is a consumer-dependent feature by construction (`🌎️hub/🧩️compositions/🀄️wfc/📦️packages/🦀️rust/Cargo.toml`).

### 2.6 Profiles: dev is not cheaper

Raster, cold, from `🗑️generated/cgu-*`. Note that `.cargo/config.toml` shares the build-dir, so these runs reused some host units.

| Profile / cgu | Wall s | User CPU s | Peak RSS | Component size |
|---|---:|---:|---:|---:|
| wasm-dev / 1 (today's dev) | 918 | 2,009 | 14.3 GB | 103.7 MB |
| wasm-dev / 16 | 502 | 2,556 | 6.0 GB | 111.2 MB |
| wasm-release / 1 | 612 | 1,715 | 10.6 GB | 49.7 MB (5.01 MB br) |
| wasm-release / 16 (today's release) | **317** | 1,887 | 8.2 GB | 55.2 MB (5.13 MB br) |

- wasm-dev runs `incremental = false` (it pins describe identity). So it brings neither a faster cold build nor incremental rebuilds.
- It also needs per-package opt-level overrides to meet the 8 ms interactive step, and it hits the 1 M-function link ceiling and the 256 MiB cap (#56, #57, #79).

### 2.7 Nx scale

The build #10 project graph has:

- 1,365 projects, 15,607 targets, a 186 MB `project-graph.json` and 129 MB of source maps.
- Construction takes 3.5–6 min under load (memory note).
- 1,128 projects × the levels `test-quick`, `-long`, `-exhaustive`, plus ≈ 640 × `oracle`, `contract`, `subject`, `parity`, make up ≈ 9,000 test targets.
- The release graph has 71 components × 5 component targets.

At 10× this graph would be ≈ 1.9 GB and would not be viable.

### 2.8 Unit sizes from the shared build-dir

Source: `🧮️build-dir-unit-census.py`, raster cgu experiments, wasm-release.

| Crate | Own lines | Release unit s | rlib MB | Peak (sampler) |
|---|---:|---:|---:|---:|
| stdio-semio | 125,541 | 306 (cgu16) – 520 (cgu1) | 386 | 13.3 GB |
| framework-plugin | 87,615 | 75–83 | 68 | 3.6 GB |
| os-kernel | 83,581 | 99–115 | 66 | 3.9 GB |
| stdio-pdf | 51,923 | 152 | 142 | 5.4 GB |
| framework-3d | 37,601 | 53–104 | 45 | 2.0 GB |
| stdio-gltf | 34,307 | 160 | 51 | 3.6 GB |
| stdio-dwg | 27,174 | 165–241 | 144 | 7.0 GB |
| hub-raster / hub-stdio (component units) | (own: generated) | – | – | 9.75 / 9.4 GB |
| architect-program, bim-model, energy-model | – | – | – | 8.0 / 5.6 / 4.0 GB |

---

## 3. Crate-graph laws

Every law has a gate (§8). A law with an allowlisted offender carries the wave id that removes it. The allowlist may only shrink.

### L1: One unit per crate per profile per target

- The product workspace resolves with `resolver.feature-unification = "workspace"`. Concretely: `.cargo/config.toml` gets `[unstable] feature-unification = true` and `[resolver] feature-unification = "workspace"`.
  - Every `cargo build -p X` therefore selects the same unit as the full pass.
  - The pass and narrow builds share every unit by construction, without the `package`-unification environment trick in `🔒️lease/🟦️.ts`.
- Profiles are declared once per workspace. No `--config profile.*` overrides and no `RUSTFLAGS` appear in pipelines. Target-scoped rustflags live only in `.cargo/config.toml`.
- **Gate G1** `verify units` fails if any `(package, target, profile, mode, platform)` key has more than one unit across the release plan and the native test plan.

### L2: First-party crates carry no consumer-dependent features

| Feature class today | Replacement |
|---|---|
| composition (`conversion-*`, `component-app-assembly`, `full-artifact-catalog`) | separate crates plus explicit registration lists (L6) |
| entry selection (`plugin-entry`, `plugin-root`, `component-guest`, `extension-entry`) | library crate plus thin component crate (L5, L7) |
| backends (`sqlite`, `postgres`, `neo4j`, `ureq`), `native-renderer`, `preview-window`, `mcp-service`, `native-bin` | adapter crates selected by the binary's dependencies, or `cfg(target_*)` |
| tooling (`typegen`, `mutation-inventory`) | tools-workspace crates |
| test-only (`testing`, `testkit`, `oracles`, `protocol-laws`, `cross-fem`, `test-serde`, …) | **allowed**, but only enabled through `[dev-dependencies]` or by the crate's own tests |

- **Gate G2** `verify features` reads the resolved feature set of every component and binary unit from the unit graph. It fails on:
  - any first-party feature outside the test-only allowlist;
  - any test-only feature that is reachable in a non-test unit.

### L3: Unit size budget

Targets, from the measured scaling of ≈ 5.1 s per k-line (release, cgu1) and ≈ 0.6 GB + 38 MB per k-line peak:

| Unit kind | Target | Fail | Basis |
|---|---|---|---|
| Library own lines (non-generated, non-test) | ≤ 15 k | > 25 k | 15 k ≈ 75 s cgu1 / ≈ 35 s cgu16, ≈ 1.2–1.5 GB |
| Library peak RSS p95 | ≤ 3 GiB | > 6 GiB | 26 GiB ÷ 10 cores = 2.6 GiB per core |
| Component final unit peak | ≤ 6 GiB | > 10 GiB | hub links measured 8.2–9.75 GB |
| Unit release wall (recorded, uncontended) | ≤ 60 s | > 180 s | critical-path share |
| Hub library own lines | ≤ 1 k (generated) | > 3 k | L7 |

- Existing offenders are allowlisted by wave: stdio-semio (S3), framework-plugin and os-kernel (E1, E2), stdio-pdf, stdio-dwg and stdio-gltf (E3), framework-3d (E4), architect-program (E4, if over budget after measurement).
- **Gates:** G3 `verify unit-budget` checks static lines, and G4 checks the memory and wall values recorded by telemetry (§6.3).

### L4: Package-name and path budget

- Package names are at most 48 characters.
- The worst-case uplifted path is computed from the unit graph and the build-dir layout before compiling, and must be ≤ 240 UTF-16 units: 16 units of headroom under the 256 budget from #68.
- Measured for the longest planned crate, `semio-s-artifact-stdio-semio-presentation-import`, in the fresh layout: **239**. That is why the name limit is 48.
- **Gate G5** `verify path-budget` runs pre-flight, plus a post-flight check over every recorded dep-info and output path.

### L5: Library XOR component

- A library is `crate-type = ["rlib"]` (the implicit default).
- A component crate is `["cdylib"]` and contains only the export macro, with no logic.
- Removing `cdylib` from the 8 libraries in §2.3 restores rmeta pipelining on the kernel edges. The estimated critical-path saving is ≈ 90 s per component chain: kernel's dependents start at 24 s instead of 115 s.
- It also removes 8 dead thin-LTO links per pass.
- **Gate:** G2 also fails any non-component unit that has `cdylib` in its `crate_types`.

### L6: Artifact family pattern (generalised split plan)

Every artifact family F (stdio-semio first, then every plugin artifact that exceeds L3) has this shape:

| Crate | Role | Depends on |
|---|---|---|
| `F-base` | foundation vocabulary, codec primitives, shared editor plumbing, `Conversion` type, member-owner macro | framework |
| `F-<subset>` | schema, io core, editor and viewer of one subset, all unconditional | base, real domain edges only |
| `F-<subset>-import`, `F-<subset>-export` | conversion leaves plus `pub const CONVERSION: Conversion`, depending on format crates as non-optional | subset |
| `F` (aggregator) | union envelope, `Members`, `declaration(def, &[Conversion])`, `register(&[Conversion])`, plus the **concrete instantiations** the hubs use (L8) | base + subsets |
| `F-catalog` | `contribution()` with the generated `ALL: &[Conversion]` | aggregator + all conversions |

- **Schema first.** The family's `🧩️composition/🔣️.json` lists subsets, conversions and their format crates. Its generator writes:
  - the manifest dependency regions;
  - the `-catalog` `ALL` list;
  - the `📜️artifact-definition.json` dependencies;
  - the product-workspace `members`.
- **Registration is a value, not a cfg.** A consumer's conversion set is exactly the `CONVERSION`s it names. The split plan's `catalogPackage` / `symbolPackages` in `CompositionContributionV1` become the generic contract for every family.
- **Growth.** A new subset adds one crate, plus two if it has conversions. It adds no feature and no unit variant. Its cost is linear and parallel.
- The stdio-semio instance is exactly `📓️2026-10-13-stdio-semio-split-plan.md` §4.

### L7: How hubs compose

- **`semio-hub-<x>`** is a generated rlib: the registration bundle, the composition contribution and the policy constant. Embedding compositions such as the demonstrator depend on these libraries.
- **`semio-hub-<x>-component`** is a cdylib holding one line, `plugin_exports!(semio_hub_<x>::bundle, POLICY)`. It is the only crate with an entry symbol, so duplicate-symbol links are impossible and `plugin-entry` disappears.
- Hubs depend on aggregator and catalog crates only, never on subset internals. The exception is the symbols the composition schema names (`symbolPackages`).
- Component crates are listed by the composition generator; nobody hand-edits them.

### L8: Generic boundary at the family aggregator

- Today every hub monomorphises the family generics (stores, apps, codecs, derives) inside its own unit. That is why hub units cost 150–217 s and 8–10 GB.
- The aggregator exports concrete or `dyn`-erased constructors for everything a hub registers, so hubs only call already-compiled code.
- rustc's generic sharing (`-Zshare-generics`, on by default at opt-level `s`; to be verified in M2) reuses upstream instantiations only if the upstream crate instantiated them. L8 makes sure it did.
- **Gate (report, then ratchet):** the hub library's own codegen time ≤ 30 s, from telemetry.

### L9: Relink fan-out trigger (when static composition must give way)

- Fan-out cost of family F = (number of components linking F) × (F's share of each component's final unit), taken from telemetry.
- If a family is linked by more than 8 components and costs more than 30 s per component, it becomes a candidate for its own separately linked component, composed over a WIT interface.
- stdio-semio is linked by 34 of 44 components today, so it meets the trigger.
- The decision itself changes the runtime ABI, so it gets a spike with runtime benchmarks (C2) and is not done blindly.

---

## 4. Workspaces and dependency resolution

| Workspace | Members | Lockfile | Profiles |
|---|---|---|---|
| **product** (root `Cargo.toml`) | every crate that ships or is tested as product: `🧰️framework/**`, `✏️s/**` artifacts, modules and plugin crates, `🌎️hub/**`, products, `🎓️teaching` | one root `Cargo.lock` | `dev`, `release`, `wasm-release`, `wasm-iterate`, declared once |
| **tools** (`🧰️framework/🛍️products/🦑️repo/🧰️tools/Cargo.toml`, path to be admitted by the taxonomy gate) | `🏭️generator`, `🔬️probes`, `🔮️oracles`, `🏭️bridge`, `👽️guest` test crates, plus the build scheduler binary (§6.2) | its own lockfile | `dev`, `release` |

- **Members** are written by the taxonomy generator as an explicit list. Nobody edits them by hand. This removes the "contended root members list" reason that produced 226 single-member workspaces.
  - Measured: cargo's recursive `members` glob works, but `exclude` takes no globs. The explicit generated list is therefore the clean choice.
- **Why two workspaces and not one.** Under workspace unification, an oracle's external features (for example a full `image`) would unify into product units. Keeping tools apart also keeps the dependency-truth gate's oracle-conflict check meaningful.
- **Why not more.** Every additional workspace has its own lockfile and profile set, so it cannot share units, and it recompiles the world (§2.5).
- **Scale check.** About 700 product members today and about 7,000 at 10×. Cargo resolution and `cargo metadata` stay in seconds. Rust-analyzer loads by member set. The risk is tracked in §13.
- **All nested `[workspace]`, `[profile.*]` and `Cargo.lock` files disappear.** That is 271 workspaces and ≈ 274 lockfiles. The repository gate `verify features` (G2) also fails on any nested `[workspace]`.

---

## 5. Profiles and artifacts

### 5.1 One shipped component profile

| Profile | Target | Used by | In release graph | Notes |
|---|---|---|---|---|
| `wasm-release` | wasm32-wasip2, wasm32-unknown-unknown | components, wasm-bindgen modules; **describe, provenance, materialize, receipts, trusted catalog** | yes | opt `s`, thin LTO (until M2 decides, §6.6), cgu 16, `strip = "symbols"`, `trim-paths`, plus `--remap-path-prefix` of repo and build dir (§7.4) |
| `release` | host | describe tool, hub server, CLI tools | yes (describe tool only) | cgu 16 (decided in M2 for tools; runtime-critical hub binary keeps cgu 1 if measured faster) |
| `dev` | host | native check, tests | no | `incremental = false` (fleet), debug assertions |
| `wasm-iterate` | wasm32-wasip2 | single-plugin dev loop only | **never** | inherits `wasm-release`; `opt-level = 1` with the existing hot-crate `opt-level = 2` table; `lto = false`; `codegen-units = 256`; `incremental = true`; `debug-assertions` and `overflow-checks` on; `strip = "none"` |

- `wasm-dev` is deleted: it is the release graph's second compile, and it fails on ceilings (#56, #57, #79).
- The 256 MiB `FRESH_COMPONENT_MAX_BYTES` and the 1 M-function ceiling then only constrain shipped bytes, at ≈ 50 MB.
- Iterate gets its own local-only cap (512 MiB) in its own schema constant.

### 5.2 Descriptors, receipts, registry and trusted catalog

| Object | Today | After |
|---|---|---|
| Committed descriptor (`🔣️.json` + `🛂️.descriptor.semio`, 72 pairs) | semantic declaration **plus `hashes.wasmSha256` of `dist/component-dev`** | semantic declaration only. Profile-independent and reviewed in diffs. `wasmSha256` is removed from the schema. |
| Component receipt (new, generated, never committed) | — | `ComponentReceiptV1 { pluginId, packageId, profile: "wasm-release", target, wasmSha256, wasmBytes, semanticDescriptorSha256, toolchain, sourceTreeDigest, provenanceSha256 }`, written by the describe stage to `dist/components/<id>/🧾️receipt.json` and published with the modules page. Schema-first, with a fixture and Ajv tests. |
| `describe` | runs `dist/component-dev`, rewrites the committed pair | runs the **shipped** bytes. It fails with a canonical diff if the emitted semantic descriptor ≠ the committed one, and writes the receipt. `describe --write` (dev command) updates the committed pair from release or iterate bytes. |
| `📇️registry/🛂️descriptor-verification` | hashes `dist/component-dev/<wasm>` against `wasmSha256` | semantic pair consistency (unchanged). Where a receipt exists, `receipt.semanticDescriptorSha256 = sha256(committed pack)` and `sha256(dist/components/<id>/component.wasm) = receipt.wasmSha256`. |
| Hub `trusted-catalog-preflight` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts` ≈ L7978–8009) | `component-dev` deliverables against committed hashes | receipts plus shipped bytes. `os-hub trusted-catalog publish` stores the semantic descriptor and the receipt per entry. |
| `📇️registry/🔁️rebuild` (`rebuild-all`) | dev staging convergence | `workspace:components` then `describe --write` |
| `📽️projection` `PLUGIN_COMPONENT_PROFILE_DIRS` | `dist/component-dev`, `dist/component-release` | `dist/components` (one directory) |

- **Profile-independence law.** The semantic descriptor of a component must be equal under `wasm-release` and `wasm-iterate`.
- A nightly gate describes a rotating sample of 8 components under both profiles and compares them. This proves that iterate-written descriptors are valid for release.

### 5.3 Dev loops (what replaces component-dev)

1. **Logic.** Native `cargo check` / `cargo test -p <crate>` in the `dev` profile, through the shared build-dir. This is where most iteration happens.
2. **One plugin in the browser.** `components iterate <plugin-id>` builds `wasm-iterate` for that one component. It describes the result locally (uncommitted, marked `provenance: local-iterate`) and transpiles it without wasm-opt.
   - The os-dev host accepts local-iterate descriptors only in dev mode.
   - All other plugins are served from `dist/components` of the last warm release build.
3. **Framework change across all plugins.** A warm `workspace:components` (release) build. With cgu 16 that is already faster cold than today's wasm-dev (317 s vs 918 s for raster).
   - An optional `components iterate --all` lane, with assertions on, runs nightly.

---

## 6. Orchestration

### 6.1 Build driver (one owner per release)

`workspace:release` is one nx target, `cache: false`. It runs one first-party driver process, which:

1. Creates a jobserver with N = logical cores tokens (10). Unix uses a fifo (`--jobserver-auth=fifo:<path>`); Windows uses a named semaphore.
2. Starts, concurrently and as jobserver clients:
   - (a) `cargo build --locked -p <all components> --lib --target wasm32-wasip2 --profile wasm-release --message-format=json-render-diagnostics`
   - (b) `cargo build -p <host tools> --profile release`
   - (c) the wasm32-unknown-unknown wasm-release modules (os-flow-core, surface, …)

   Cargo inherits the jobserver, and rustc's `-Z threads=8` frontend takes its extra threads from the same pool.
3. Sets `RUSTC_WRAPPER` to the scheduler binary (§6.2) for memory admission and telemetry.
4. **Streams stages per component** the moment cargo prints `compiler-artifact` for its cdylib:
   - stage (copy to `dist/components/<id>`)
   - → describe (verify and write the receipt)
   - → provenance (§7.3)
   - → `wasm-opt -Oz`
   - → jco transpile into module files

   Each stage holds one jobserver token. Stages are cached by input digest (§7).
5. Starts at t = 0, as token-holding stages, the work that does not depend on compiled bytes:
   - catalog render from committed semantic descriptors;
   - Vite site bundle;
   - map-tile prefetch (network-bound, 0 tokens);
   - typecheck.

   The app reads component hashes from a runtime `catalog.json` manifest bound at pages assembly, so the bundle does not depend on compiled bytes. Verify this in N1; if the bundle embeds hashes today, change it to a manifest.
6. Assembles the pages at the end and binds the receipts into the modules manifest.
7. Supports cancellation everywhere: one AbortSignal terminates the cargo trees, the wrappers and the stage workers.
   - There is no wall-clock deadline on the pass.
   - A stall watchdog fires only if no unit has started or finished for 15 min while units are pending. It records a `sample` of every cargo process before stopping, which detects the fine-grain-lock deadlock from memory note `feedback-fine-grain-locking-wasm-deadlock`.

This replaces:

- the claim, ready and marker protocol and the in-consumer pass of `ensureCargoPrebuildV1`;
- the per-profile exclusive build lease (`acquireCargoBuildLeaseV1`) in release builds. Only the driver writes to its generation's build-dir; the shared dev build-dir keeps fine-grain locking.

### 6.2 Memory-aware admission that calibrates itself

`semio-build-scheduler` is a tools-workspace Rust binary with its own jobserver client. Cross-platform peak RSS comes from hand-declared FFI: `wait4`/`getrusage` on Unix, `GetProcessMemoryInfo` on Windows. It uses no external crates.

**Wrapper mode.** For every rustc invocation:

1. Derive the key `(crate name, target, profile, crate-types)`.
2. Ask the driver's broker, over a local socket or named pipe, for admission with `predicted(key)`.
3. While waiting, hand the implicit jobserver token back, so light units keep the CPUs busy. Reacquire it before spawning rustc.
4. Spawn rustc and wait for it. Report `{key, peakRss, userCpu, sysCpu, start, end, waitMs}`.

**Prediction.**

- `predicted(key) = max(1.15 × p95(last 8 recorded peaks of key), 256 MiB)`.
- An unknown key uses `0.6 GiB + 40 MiB × kloc`, with kloc taken from the last dep-info or from a manifest-root walk.
- An unknown component final unit is assumed to need 6 GiB.

**Admission rule.**

- `Σ predicted(running) + predicted(u) ≤ physical − 6 GiB reserve − nonRustcRss`.
- `nonRustcRss` is the rolling 10 s maximum of the driver's other children (Vite, stage workers).
- If nothing is running, the unit is admitted anyway. Progress is guaranteed and a unit that needs more than the budget runs alone.

**Feedback.**

- The broker samples actual RSS every second.
- If the actual total exceeds the budget, or the swap-out rate exceeds 64 MiB/s, admissions freeze until usage falls below 90 % of the budget.

**Priority among waiters.**

1. Longest recorded remaining path first (from the last build's unit graph and durations).
2. Then the largest predicted peak first, so big units start while memory is free.
3. Then FIFO.

**Storage.** Telemetry is append-only events in `.🧬semio/🦑️repo/⚡️cache/build-telemetry/units.jsonl`. This is scheduling knowledge, not build input. It is compacted to 8 observations per key, and it is **never cleared by fresh builds**.

This replaces `bytesPerJob`, `heavyUnitBytes`, `cargoPrebuildJobsV1` and `cargoPrebuildPassesV1`. The split plan's wave 4 ("re-measure, then set constants") disappears, because the constants become data.

### 6.3 Output

- **Files.** Cargo JSON goes to `<generation>/logs/cargo-<target>-<profile>.jsonl`. Each stage writes `<generation>/logs/<id>/<stage>.log`. Both are capped at 128 MiB.
- **Terminal.** One progress row every 30 s, for example: `[release] 212/372 units · 18/44 components staged · cpu 87 % · mem 21.3/32 GiB · admitted-wait 3 · eta 11 min`. After that, a summary, the report path and, on failure, at most 200 lines per failed unit or stage and 2,000 lines in total.
- **No nx-in-nx.** `build-fresh` stops wrapping a nested `nx run` inside an nx task. That nesting is what made nx join ≈ 950 MB and crash with `RangeError: Invalid string length` (build #9). The release graph runs in one nx invocation, and the driver keeps its own output bounded.
- **Gate G7** caps the terminal output of every task at 1 MiB or 2,000 lines.

### 6.4 Nx task graph shape

| Release graph | Today | After |
|---|---|---|
| Tasks | ≈ 375 (71 × `component-dev`, `describe`, `component-release`, `materialize-*`, sessions, prepare, catalog, site) | ≈ 10 |
| Shape | plugin-serial chains | generators (cached, t = 0) → `workspace:release` (driver) → `workspace:release-verify` (gates, publication audit) |

- Per-component nx targets disappear from the release graph. They remain only as dev commands (`components iterate <id>`, `describe --write <id>`), which take arguments, not as 71 × 5 inferred targets.
- **Test targets (N2).** One `test` target per owner project, with `--level quick|long|exhaustive` and `--facet oracle|contract|subject|parity` arguments, instead of 9 inferred targets each. That is ≈ 9,000 targets fewer.
- Rust crates do not need one nx project each. Owner (plugin or product) projects select their crates through cargo.
- Target: fewer than 400 projects now, growing with owners rather than with crates. Graph construction drops from 3.5–6 min to under 30 s (estimate).

### 6.5 Keeping 10 cores busy end to end: phases and critical path

| Phase | Today | After all waves |
|---|---|---|
| Head (proc-macros, derive, value, replication, kernel, plugin) | ≈ 250 s at about 1 unit | ≈ 60–80 s. Kernel and plugin split (E1, E2) and pipelining (L5) give ≈ 4–6 units. Host tools, catalog, Vite and typecheck fill the gap. |
| Middle (≈ 330 library units, 44 component links) | serial plugin by plugin | CPU-bound. Admission keeps ≥ 85 % busy with ≤ 2–3 heavies in flight. |
| Tail (last links, stages, pages) | ≈ 2–5 min per plugin, serial | stages already streamed. The tail is the last component link plus ≈ 60 s of stages. |

Critical path of one component chain. "Today" is measured (§2.2); "after" is an estimate.

| Segment | Today (s) | After (s) | Mechanism |
|---|---:|---:|---|
| proc-macro chain | 9 | 9 | — |
| value → replication | 41 (full edges) | ≈ 15 (rmeta) | L5 |
| os-kernel | 115 (full) | ≈ 15–25 to rmeta | L5 + E1 |
| framework-plugin to rmeta | 20 | ≈ 10–20 | E2 |
| stdio-contract to rmeta | 2.5 | 2.5 | — |
| stdio-semio | 520 (full) | ≈ 60–90 (rmeta chain base → value → table → aggregator, plus table full at cgu16) | S1–S3 |
| hub component unit | 217 | ≈ 60–90 | cgu16 + L7/L8 (+ link mode M2) |
| stages (component-ld, wasm-opt, jco, describe, provenance) | ≈ 60–120 | ≈ 45–60 | streaming, cached |
| **Total** | **≈ 1,000** | **≈ 220–330** | |

The critical path ends at ≈ 4–6 min. Cold wall is therefore set by total CPU ÷ (10 × utilisation) (§9), not by the dependency chain. That slack of more than 3× is what makes high utilisation possible.

### 6.6 Link mode and generic sharing (decided by measurement M2, rule fixed now)

- **Variants.** Three components (raster, stdio, process), each built 2 × 2: `lto ∈ {thin, false}` × `share-generics ∈ {default, forced}`, after L7/L8.
- **Choose the cheapest variant** whose:
  - Brotli bytes are ≤ +3 % against thin LTO;
  - interactive-step p95 is ≤ +5 % on the lowpoly tessellation and dag-walk benchmarks named in the hub `Cargo.toml` opt-level notes;
  - descriptor equality holds.
- The decision goes into the product `Cargo.toml`, with the numbers in its docstring comment.

---

## 7. Caching for fast iteration

### 7.1 Authority

- **Rust:** cargo decides freshness: dep-info, `checksum-freshness`, unit hashes. Nx cargo tasks are `cache: false`.
  - A no-op release pass is a fresh check of about 420 units. Estimate: ≈ 5–10 s, plus stage-cache hits.
  - This removes the hasher blind-spot class (gitignored `🤖️generated` Rust, files read by build scripts, environment) recorded in memory note `project-nx-hasher-blind-to-gitignored-inputs`.
- **Post-link stages:** keyed by `sha256(input wasm) + toolVersionsDigest + stageSchemaVersion` and stored in `.🧬semio/🦑️repo/⚡️cache/stages/<stage>/<key>/`. They are exact by construction.
- **TypeScript and generators:** nx cache with declared inputs, plus runtime byte-digest inputs for generated or ignored files (the existing `generatorOutputCouplingInputs` pattern).

### 7.2 Build-dir sharing

- **One shared warm build-dir** (`⚡️cache/cargo/build`, fine-grain locking) for dev, iterate, tests and warm release.
  - With one product workspace and one profile set, every agent and every lane hits the same units.
  - Today, the 26 standalone `✏️s` extension workspaces get none of that sharing.
- **The fresh build does not share** with the warm build-dir. It is the cold reproducibility proof, and it keeps its own generation (`⚡️cache/fXXXXXX/{t,b,…}`).
- **Determinism gate G6 links the two.** The fresh build's receipts must equal, byte for byte, the warm build's receipts for the same source tree digest.
  - If G6 is green, a deploy may ship warm outputs, and the fresh build moves from "every release" to "nightly and before deploy".
  - Requirements: toolchain pinned (already), `trim-paths`, `embed-metadata = DoNotEmbed` (already), and `--remap-path-prefix=<repo>=/semio --remap-path-prefix=<build-dir>=/build` in the target-scoped rustflags. That last flag means the fresh path `fXXXXXX/b` and the shared `cargo/build` produce the same bytes.

### 7.3 Provenance at O(units), not O(components × closure)

- Today every component receipt re-hashes its whole closure: ≈ 3–4 GB per component (4.05 GB for process). At 10× that is ≈ 1.8 TB of hashing per build.
- **After:** the driver hashes each unit's inputs and outputs **once per build session**, writes one unit digest table (a Merkle leaf per unit), and rechecks every observed file **once** at session end. The physical-custody rule from the provenance note stays: every input is hashed fully twice, never by mtime.
- Each component receipt references its closure's unit leaves and stores the Merkle root.

### 7.4 Evaluated and rejected: sccache-like caching

- **Measured history:** a 16.8 % hit rate, and every session serialised through one server (memory note `project-shared-cargo-build-dir-fine-grain-locking`).
- **It cannot cache the dominant costs.** cdylib final units and thin-LTO links are not cacheable by sccache, and incremental units are not either.
- **Within one build-dir it duplicates** what cargo already does.
- **Across fresh generations** it would void the cold proof.
- **The replacement is G6.** Deterministic builds plus a component-level receipt cache keyed by `(sourceTreeDigest, toolchain, profile, unit-graph digest)` deliver cross-generation reuse with proof. That is local-first, with an optional shared store later, and it needs nothing in product code.

### 7.5 What a fresh build clears and keeps

| Clears (new generation) | Keeps |
|---|---|
| cargo target-dir and build-dir, stage cache, nx cache, nx workspace data, generated catalog, `dist/components`, `dist/pages` | toolchain, the cargo registry (`--locked --offline`), `node_modules` (lockfile integrity), `wasm-opt` and browsers, map-tile cache (content-checked, needs network), fonts, **build telemetry** (scheduling data), the shared warm build-dir |

- After a green run, the generation's target-dir and build-dir are deleted. The receipts, the timeline and the report stay.
- **GC.** `repo:cache-prune` gets the lock-aware unit-generation prune: keep the newest 2 units per package, and delete older ones only if idle for more than 12 h and an exclusive lock is free. This is the `🧹️s5-stale-unit-prune.py` method from memory note `project-ticket-generated-nx-workspace-caches-hog-disk`.

### 7.6 Incremental

- `wasm-iterate` is incremental. Its bytes never reach a descriptor or a receipt, which resolves the conflict between `incremental` and describe identity noted in the hub `Cargo.toml` (`[profile.wasm-dev]`, ticket 26/09/23 W1).
- `dev` stays non-incremental for fleets and CI, because incremental session directories regrew 10–23 GB per hour under fleets (memory note `feedback-incremental-cache-regrows-under-fleet`).
- Interactive humans use the iterate launch entries.

---

## 8. Gates and observability

| Gate | Command (root `📜️script.ts`) | Fails when | Fixture and oracle (AGENTS: language-agnostic plus third-party) |
|---|---|---|---|
| G1 units | `verify units` | any key has more than 1 unit (release plan + native test plan) | plan + expected counts JSON; oracle `cargo tree -e features` |
| G2 features | `verify features` | first-party feature outside the test allowlist; test feature in a non-test unit; non-component `cdylib`; nested `[workspace]` | unit-graph fixture; oracle `cargo metadata` |
| G3 unit size | `verify unit-budget static` | own lines over L3, or package name over 48 | dep-info fixture; oracle `tokei`-style count in a tools crate |
| G4 unit memory and wall | `verify unit-budget recorded` | recorded p95 over L3 | telemetry fixture; oracle `/usr/bin/time -l` on a sample |
| G5 path budget | `verify path-budget` | worst-case uplift path > 240 (pre-flight) or any recorded path > 256 | layout fixture; oracle `find` over a real build |
| G6 determinism | `verify build-determinism` (nightly, pre-deploy) | fresh receipts ≠ warm receipts | receipt fixture; oracle `cmp` |
| G7 output budget | inside the driver and the nx plugin | task terminal output > 1 MiB or 2,000 lines | — |
| G8 utilisation (report, warn) | build report | compile-phase CPU < 75 %, or an idle window over 30 s while units are memory-blocked | timeline fixture; oracle cargo `--timings` HTML |

**Per-build report.** It is written to `<generation>/report/` and also to `dist/release-report/` for the publication audit.

- `timeline.jsonl`: one row every 2 s with CPU % (cross-platform from `os.cpus()` deltas), free memory, swap (macOS/Linux), running rustc count, admitted-wait count and token pool use. This productises `📈️rustc-peak-sampler.sh`.
- `units.json`: per unit, start, end, peak, CPU and wait, from the wrapper.
- `report.md`:
  - wall, CPU-h and utilisation by phase;
  - the critical path recomputed from the unit graph and measured durations;
  - top 20 units by CPU and by peak;
  - idle windows with their cause (dependency-blocked, memory-blocked, stage-blocked);
  - the relink fan-out table for L9;
  - stage overlap.

---

## 9. Expected effect

### 9.1 CPU model (estimate, ±30 %, replaced by M0)

Unit-seconds approximate CPU-seconds at cgu 1. cgu 16 adds about 10 % (raster: 1,715 → 1,887 user-s).

| Component of one cold release pass | CPU-h | Derivation |
|---|---:|---|
| Hub plan after the split | 4.3 | Process closure 2,521 unit-s (measured) + 43 other component links × ≈ 150 s + ≈ 171 other units × ≈ 30 s ≈ 14,100 unit-s, + 10 % for cgu16 |
| stdio-semio duplicates before the split | +1.0 | 7 extra units × 520 s |
| 26 standalone `✏️s` extension workspaces | +1.8 | build #9: 2,614 s of cargo wall × ≈ 2.5 cores |
| Host tools and wasm-bindgen modules | +0.3 | describe tool 274 s, os-flow-core 263 s, surface 52 s |
| wasm-dev pass (today only) | ≈ 1.06 × release | raster cold user CPU: dev cgu1 2,009 s vs release cgu16 1,887 s |

### 9.2 Cold play release per wave (cumulative, quiet 10-core / 32 GB machine)

| After wave | CPU-h | Compile-phase CPU utilisation | Cold wall | Why |
|---|---:|---:|---:|---|
| Baseline: build #9 pipeline | ≈ 22 | ≈ 27 % (measured ≈ 2.7 rustc; 140–170 % of 1,000 % observed) | ≥ 8 h 17 min (unfinished) | serial lease, 708 units per profile, 2 profiles |
| Build #10 pipeline | — | 2 jobs | did not finish (1 h deadline) | §2.1 |
| S0 (wave 0, running) on the #9 pipeline | ≈ 15 | ≈ 27 % | ≈ 5.5 h | 708 → 388 units per profile |
| **A1** driver + jobserver + admission + streaming | 15 | 65 % | **≈ 2.3 h** | parallel, memory-limited by 8 semio units of 11–13 GB |
| **A2** single shipped profile | 7.4 | 65 % | **≈ 70 min** | one compile per component |
| **S1–S3** stdio-semio split | 6.4 | 75 % | **≈ 51 min** | 1 unit per semio crate, largest library 1.5–2.5 GB |
| **L1** library XOR component, thin component crates | 6.1 | 78 % | **≈ 47 min** | pipelining on kernel edges, 8 dead links gone |
| **WS1** product and tools workspaces, unification, one profile set | 4.4 | 80 % | **≈ 33 min** | standalone extension recompiles 1.8 → ≈ 0.1 CPU-h |
| **E1–E4** kernel, plugin, pdf/dwg/gltf, 3d splits | 4.2 | 88 % | **≈ 29 min** | head phase parallel, no 5–7 GB library units |
| **C1** thin hubs (L8) + link mode (M2) | 2.9–3.4 | 88 % | **≈ 20–23 min** | component links −40…−60 % (1.8 → 0.8–1.1 CPU-h) |
| **N1** coarse nx, t = 0 overlap, stage cache, Merkle provenance | 2.9–3.4 | ≥ 88 % | **≈ 18–21 min end to end** | graph construction 3.5–6 min → < 0.5 min, serial tail removed |

### 9.3 Iteration after all waves (estimate)

| Change | Rebuild | Wall |
|---|---|---|
| Nothing (no-op warm release) | fresh check + stage hits | < 30 s |
| Leaf plugin artifact crate | 1 crate + 1–3 component links + stages | 2–3 min |
| One stdio-semio subset | subset + aggregator + 34 relinks | ≈ 6 min (this fan-out is what L9 / C2 targets) |
| os-kernel | all component links | ≈ 15 min |
| Single plugin in the browser (`wasm-iterate`) | incremental, no LTO | 20–60 s |

### 9.4 10× projection

- Assumptions: about 4,000 first-party units, about 440 components, and the same per-unit and per-link cost.
- Result: ≈ 30 CPU-h cold, so ≈ 3.5 h on this machine at 88 %. The critical path stays at ≈ 8 min.
- Consequences:
  1. Cold builds must become rare, made safe by G6 (determinism) and served from the receipt-keyed component cache (§7.4). The nightly fresh build can move to a larger build host without changing the design.
  2. Warm and iterate builds scale with the change's fan-out, not with repository size.
  3. Relink fan-out is the only super-linear term. L9 / C2 caps it by turning the widest families into separately linked components.

---

## 10. Execution waves

Rules for every wave:

- **Compile-atomic.** Each wave ends green on its gate before dependants start.
- **Foreground cargo** through `🚦️cargo-slot.sh`.
- **Private directories.** Test and uplift runs use a private `CARGO_TARGET_DIR` under `⚡️cache/<wave>`, and scratch clones get a private build-dir.
- **No git-modifying commands.** Verify by diffstat.
- **Generators, not hand edits**, wherever a generator exists.
- **Every new command** is registered in `.vscode/🧩️launch.seed.jsonc` → `launch.json` in the matching group: `⚖️gate…`, `🔎️diagnostic…`, `📦️build…`.

### S0: upstream feature normalization (split-plan wave 0; running, other executor)

- **Scope:** U1–U5 and the census command, as in `📓️2026-10-13-stdio-semio-split-plan.md` §6.
- **Integration:** its census becomes **G1** unchanged, first scoped to the hub plan. WS1 widens the plan to the product workspace.
- **Note:** after WS1, U2/U3 (external feature normalization) are guaranteed by workspace unification. U1/U5 (first-party features) remain required.

### M0: baseline measurement (directly after A1; no source edits)

- **Run:** one cold `workspace:release` with telemetry on today's tree.
- **Output:** `📓️<date>-build-baseline.md` with measured CPU-h, utilisation, critical path, unit peaks and the L9 fan-out table.
- **Effect:** replaces every estimate in §9, and seeds `units.jsonl` so A1's admission predicts well from its second build onward.

### A1: build driver, scheduler, telemetry (TypeScript and tools-Rust)

- **Lane:** A. Runs in parallel with S1/S2; the files are disjoint.

Files:

- **New:** `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/` facets `🎰️jobserver`, `🎟️admission`, `📈️telemetry` and `🧩️component-pass`. Each has `🟦️.ts`, `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json` and `🧪️tests/🟦️.ts`. The slugs are to be admitted by the taxonomy gate.
- **New:** the scheduler binary crate (`semio-build-scheduler`) under `🎟️admission/📦️packages/🦀️rust`, in the tools workspace once WS1 lands. Until then it is its own standalone workspace, like today's tooling crates.
- **Changed:**
  - `🔒️lease/{🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures, 🧪️tests}`: delete the prebuild plan, claim, ready, passes and jobs logic.
  - `native-build/🟦️.ts` (`buildCargoArtifacts`: no prebuild, no profile lease in driver mode).
  - `⚡️caching/🦀️cargo/🟦️.ts`: `componentPrebuildPlanV1` becomes the driver's plan source.
  - `⚡️caching/📦️artifacts/📋️native-orchestration/{🟦️.ts, 🔣️policy.json}`.
  - `📚️library/🟨️.mjs`: `componentTargets` → `workspace:release`.
  - `🏢️semio-tech/🎡️play/🔨️modules/📦️site/{📜️script.ts, 🆕️fresh-build/🟦️.ts}`: no nested nx; driver invocation.
  - The root `project.json` workspace targets.
  - `.vscode/🧩️launch.seed.jsonc` and `launch.json`.

Verification:

- Driver, admission and jobserver laws pass.
  - The jobserver protocol is cross-checked against GNU make 4.4's `--jobserver-style=fifo` as the oracle.
  - Admission vectors are language-agnostic.
  - Telemetry critical path is cross-checked against cargo `--timings` HTML.
- `bun nx run @semio-tech/framework-process:test-native-artifacts`.
- The fresh-build contract test (`🧪️tests/🧪️playfreshbuild`).
- One real `workspace:release` on the current tree reaches all 44 hub components and the 26 `✏️s` components, with no deadline kill and a report written. That run is M0.

**Gain:** ≈ 5.5 h → ≈ 2.3 h. The build can finish again.

### A2: single shipped profile, semantic descriptors, receipts (TypeScript, Rust host script, 72 descriptor pairs)

- **Lane:** B. Runs in parallel with A1 (disjoint files) and with S1–S2.

Files (the ≈ 57 files that reference `component-dev` / `materialize-dev` / `wasm-dev`). The main ones:

- `🔌️plugin/🖨️describe/{🏭️fresh-component, 🏗️component-build}/🟦️.ts`
- `📇️registry/{🛂️descriptor-verification (+ schema + fixtures), 🔁️rebuild, 📽️projection, 🧪️tests/*}`
- `🔌️plugin/🏗️build/{📋️plan, 🔍️freshness}`
- `🔌️plugin/📦️packages/🟦️typescript/📜️script.ts` (materialize)
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (trusted-catalog preflight and publish)
- `🌎️hub/🧪️tests/🔬️bin-unit/🪶️count-lease`, the draw guest test, `🖥️host/🧪️testing/🧩️component`
- `📚️library/{🟦️.ts, 🟨️.mjs}`
- the root and hub `Cargo.toml` profiles (delete `wasm-dev`, add `wasm-iterate`)
- os-dev activation (`🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/♻️activation`)
- every `🛂️.descriptor.semio` + `🔣️.json` pair (72), regenerated by `describe --write` from release bytes
- the new `ComponentReceiptV1` schema, fixture and tests

Verification:

- The descriptor-verification suite passes.
- The hub trusted-catalog preflight test passes.
- `describe` equality on all 72 components from release bytes.
- The profile-independence sample (8 components, release vs iterate).
- The registry `check` passes.
- The play unit gate passes.

**Gain:** CPU halves, ≈ 2.3 h → ≈ 70 min.

### S1 / S2 / S3: stdio-semio split (split-plan waves 1–3, unchanged in content)

- **Lane:** C. Sequential.
- **S1:** decouple inside the monolith. **S2:** the crate cut into 47 feature-free crates. **S3:** consumers and generators.
- **Files, generators and gates:** exactly as in split-plan §6, with these additions:
  - **S2:** every new crate is rlib-only (L5), and its package name is ≤ 48 characters (L4). Today's longest planned name, `…-presentation-import`, is 47 characters.
  - **S3:** run G1 and G2 over the hub plan. The semio family must show 1 unit per crate.
- **Gain:** ≈ 70 min → ≈ 51 min. The 13 GB peak becomes ≤ 2.5 GiB.

### L1: library XOR component, thin component crates (Rust manifests, hub composition generator)

- **Lane:** C, after S3. Both rewrite hub manifests and the stdio composition generator.

Files:

- The 8 library manifests from §2.3: drop `cdylib`.
- 44 hub manifests: split into `semio-hub-<x>` (rlib) and `semio-hub-<x>-component` (cdylib, `📦️packages/🦀️rust` under a `🧩️component` facet, to be admitted by the taxonomy gate). Delete `plugin-entry`, `plugin-root` and `component-guest`.
- The composition generators under `🌎️hub/🧩️compositions/*/🧩️composition/🟦️.ts`. Their outputs are regenerated.
- The nx plugin component detection: `metadata.component` moves to the component crate.
- The remaining 77 dual crate-type manifests across `✏️s` and the framework.

Verification:

- G2 shows no non-component cdylib.
- `cargo check` for wasm32-wasip2 and native.
- The hub stdio `editor-catalog` and `shipped-fleet` tests pass.
- A2's describe equality holds for all components.

**Gain:** ≈ 51 → ≈ 47 min, and the critical path drops by ≈ 90 s.

### WS1: product and tools workspaces (manifests, lockfiles, taxonomy generator, cargo config)

- **Lane:** C, after L1 and after G2 shows zero non-test first-party features.

Prerequisite in the same wave: convert the remaining first-party feature classes per L2.

- backends and `native-*` → adapter crates;
- `typegen` and `mutation-inventory` → tools crates;
- `component-receipt-acceptance` → test-only.

Files:

- Root `Cargo.toml`: generated members, profiles once, `[workspace.dependencies]` for all crates.
- The new tools workspace manifest.
- Delete the nested `[workspace]` and `[profile.*]` sections of 271 manifests and ≈ 274 lockfiles.
- `.cargo/config.toml`: `[resolver] feature-unification = "workspace"` and remap-path-prefix.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` and the normalization generator that writes `members`.
- `.gitignore` line for `**/📦️packages/**/Cargo.lock`.
- Every script passing `--manifest-path <artifact>/Cargo.toml`: they become `-p <crate>` on the product workspace.
- `verify dependencies` baselines.

Verification:

- `cargo metadata` for both workspaces.
- `cargo check --workspace` for native, wasm32-wasip2 and wasm32-unknown-unknown.
- The `--lib` test sweeps (plugin, os-kernel, value, replication) at their current pass counts.
- G1 shows 1 unit per key on the product release plan and the test plan.
- `verify dependencies literal-external` shows no new oracle conflicts.
- G5 passes.

**Gain:** ≈ 47 → ≈ 33 min. The 26 extension components share all units.

### E1–E4: unit-size splits (one executor per crate, in parallel, on disjoint files)

- **Lane:** D. After WS1, which makes adding crates a generator step.
- **Two steps per crate,** using the family method from the split plan:
  1. Decouple inside the crate first. This step can start before WS1 because it only touches files inside the crate.
  2. Then do the crate cut.

| Wave | Crate | Measured | Split axis (by owning facet) |
|---|---|---|---|
| E1 | `semio-framework-os-kernel` | 83.6 k lines, 99–115 s, 3.9 GB, critical path | kernel core (store, ledger) / canonical edit / history and time travel / retirement and lease / laws (test-only) |
| E2 | `semio-framework-plugin` | 87.6 k lines, 75–125 s, 3.6 GB, critical path | reactor and turn / jobs and session / publication and mounted owner / window and kits / declaration and describe |
| E3 | `semio-s-artifact-stdio-{pdf, dwg, gltf}` | 51.9 k / 27.2 k / 34.3 k lines; 5.4 / 7.0 / 3.6 GB | format family pattern (L6): model / read / write / editor and viewer |
| E4 | `semio-framework-3d`, `semio-s-artifact-architect-program` (8.0 GB), `bim-model` (5.6 GB), any M0 offender | per M0 | per L6 |

- **Verification per crate:** G3/G4 green for that crate, `cargo check` across the workspace, and its `--lib` tests at the previous pass count.
- **Gain:** ≈ 33 → ≈ 29 min (head phase parallel, no library over 3 GiB).

### C1: thin hubs and link mode

- **Lane:** D. After L1; it can overlap E.
- **M2 first:** the measurement in §6.6.
- **Then:**
  - Move concrete instantiations into family aggregators (L8).
  - Hub libraries call concrete or `dyn` constructors only.
  - Apply the chosen `lto` and `share-generics` setting in the product `Cargo.toml`.
- **Verification:**
  - Each hub library's own codegen is ≤ 30 s (telemetry).
  - Brotli and interactive benchmarks are within the §6.6 rule.
  - Descriptor equality holds.
  - `test-release` (149 cases) is green.
- **Gain:** ≈ 29 → ≈ 20–23 min.

### N1 / N2: nx graph shape, overlap, stage cache, Merkle provenance

- **Lane:** A, after A1 and A2. N2 can run any time on disjoint nx-plugin regions.
- **Files:**
  - `📚️library/🟨️.mjs` (inferred targets, test levels as arguments);
  - the receipt and provenance modules (`🏗️native-build/🧾️receipt`, `📁️filesystem/🧾️observation/📁️current`);
  - the stage cache facet;
  - play catalog and runtime manifest binding (`🔨️modules/🧩️runtime`, `📦️site/📄pages`);
  - launch entries.
- **Verification:**
  - The nx contract fixtures (`⚡️caching/🧫️fixtures/nx-contract`) pass.
  - The project count stays under the target.
  - Provenance receipts are semantically identical to A1's (sorted, decoded), and the hash-once property is proven by a byte counter.
  - Publication audit and `test-release-routing` pass.
- **Gain:** ≈ 20–23 → ≈ 18–21 min end to end. Graph construction falls from minutes to seconds on every nx invocation.

### C2: family-component spike (decision wave, after C1 and M0's fan-out table)

- **Scope:** prototype stdio-semio as a separately linked component behind a WIT interface. Measure relink savings against runtime cost: cross-component call latency and the 8 ms step.
- **Decision rule:** adopt if the fan-out savings exceed 50 % of warm-release CPU for the semio-subset change in §9.3, and interactive p95 stays within +5 %.

### Parallel lanes

```
time →
Lane A  A1 ─(M0)─────────────── N1 ── N2
Lane B  A2 ────────
Lane C  S0* ─ S1 ─ S2 ─ S3 ─ L1 ─ WS1
Lane D                         E1 ║ E2 ║ E3 ║ E4 (in-crate decoupling may start early) ── C1(M2) ── C2
                                                   (* S0 running now)
```

- A1 and A2 touch disjoint files: the native-build, lease and orchestration modules versus the describe, registry and descriptor modules.
  - A1's change to `🟨️.mjs` (`componentTargets`) and A2's change (the profile list) sit in the same function.
  - One executor owns `componentTargets` for both waves, or A2 rebases on A1's landed version. The function is region-guarded.
- S1–S3 touch the Rust semio crate and its consumers. They conflict with L1 and WS1 manifests, hence the sequential lane C.
- E1–E4 run on disjoint crates and are fully parallel.

---

## 11. Rejected alternatives

| Alternative | Why rejected |
|---|---|
| Keep `package` unification and normalise features by hand | Drift returns with every new consumer. The resolver mode makes the law structural. Measured: workspace unification removes it without hand edits. |
| One workspace including tools and oracles | Oracle features would unify into product units and reach shipped bytes. |
| Keep wasm-dev in the release graph, at cgu 16 | It is still a second full compile (2,556 user-s for raster), still exposed to the function ceiling and the size cap, and it describes bytes that are never shipped. |
| Describe on dev, ship release (status quo) | What is described is not what is shipped. That breaks the trusted-catalog premise. |
| Commit `wasmSha256` in descriptors | Every toolchain bump churns all descriptors, 72 today and 700 at 10×. Hashes belong in generated receipts. |
| Fixed `bytesPerJob` / `heavyUnitBytes` constants | They are wrong as soon as the crate graph changes; build #10 ran at 2 jobs. The measured self-calibrating admission replaces them. |
| nx-cached cargo tasks | Gitignored and generated inputs are invisible to the hasher. Cargo's own freshness is exact and cheap. |
| sccache | See §7.4. |
| Fat LTO, or cgu 1 for release | 2–4× link time for low single-digit size gains. cgu 16 is measured at 317 s vs 612 s for +2.4 % brotli bytes. |
| Private build-dirs per agent | Disk blow-ups (186 GB, 104 GB) and no sharing. The shared build-dir plus private uplift dirs for tests is the established rule. |
| Dynamic family components now | They change the runtime ABI. They are adopted only through C2's measured rule. |

---

## 12. Integration notes for the coordinator

- Wave 0 is running. Do not touch its files: 2d, ui, the derive crates, stdio txt/xml/json/csv/md/tsv/html, the hub stdio generator and the census command.
- A1 and A2 can start now in lanes A and B. S1 can start as soon as S0's gate is green.
- **Build #11 should wait for A1.** Build #10's failure is structural. An interim deadline bump would only reproduce the 2-job crawl (208 units in 55 min) and hide the defect.
- All executors must avoid `isolation: "worktree"`, git-modifying commands and `🗑️generated` sweeps (memory rules).

---

## 13. Risks and open measurements

- **The CPU model is an estimate** (±30 %). M0 replaces it. If hub links dominate more than modelled, C1 rises in priority above E.
- **Determinism (G6)** may expose path or time leaks from build scripts (`OUT_DIR` embeddings). Each leak is fixed at its source, never allowlisted.
- **Unstable cargo features** in use: `feature-unification`, `fine-grain-locking`, `build-dir-new-layout`, `checksum-freshness` and `--timings=json`. The toolchain is pinned (`nightly-2026-07-20`). Every toolchain bump runs G1 and G6.
- **Jobserver on Windows** (named semaphore) and fifo support in the pinned cargo must be proven in A1's tests. The fallback is anonymous pipes on Unix.
- **One product workspace at 10×** (about 7,000 members). Monitor `cargo metadata` time and rust-analyzer load in the build report. The threshold for action is `cargo metadata` over 15 s.
- **Taxonomy admission** is needed for the new facet slugs: `🎰️jobserver`, `🎟️admission`, `📈️telemetry`, `🧩️component-pass`, the hub `🧩️component` facet and the tools workspace root. Run the taxonomy gate before each wave that adds directories.
- **Iterate component size** at cgu 256 may exceed 256 MiB for stdio and norm. It gets its own local cap (512 MiB). Measure it in A2.
