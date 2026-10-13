# 🧿️ Lane C: stdio semio split, waves S1–S3

Executes waves S1, S2 and S3 of `📓️2026-10-13-stdio-semio-split-plan.md`. Laws L1–L9 and the §10 additions in `📓️2026-10-13-scalable-build-architecture.md` apply, as do the drift findings in `📓️2026-10-13-wave0-feature-drift.md`.

- **Inputs** (in `lane-c-semio-split/`):
  - the wave scripts `w1_*`, `w2_*` and `w3_*`;
  - the helpers `semio_split.py`, `use_rewrite.py`, `mount_graph.py`, `ownership.py` and `consumer_map.py`;
  - the cargo wrapper `🦀️cargo.sh`, plus `✅️check-semio.sh`, `🌎️hub-check.sh`, `🔎️manifest-sweep*.{sh,py}`, `🧪️per-test.py`, `🧪️subject-sweep.sh`, `📏️measure-split.sh`, `🔎️taxonomy-kinds.ts` and `📋️census-plan.json`.
- **How cargo was run:** every cargo command ran in the foreground, gated through `🚦️cargo-slot.sh`, with private dirs under `.🧬semio/🦑️repo/⚡️cache/s/`.

## Result at a glance

| Measure | Before | After |
| --- | --- | --- |
| Semio family crates | 1 (feature-carrying monolith, 8 units in the census) | 47 feature-free rlibs: `-base`, 18 subsets, 26 import/export, the aggregator and `-catalog` |
| Per-package unit census, 44-component hub plan (`🌳️unit-census.py`) | 375 keys / 388 units (wave 0) | **422 keys / 422 units**, featureDrift 0, multiSignature 0, at both wasm-release and wasm-iterate |
| Semio family and `semio-hub-stdio*` keys | multi-signature | 58 keys, each with **1 signature** |
| `bun nx run os-hub:unit-census-check` | allow-list had the semio family | **2 pass / 0 fail / 54 expects**, `featureVarianceAllowed: []` |
| Largest semio unit peak RSS, wasm-release | 14–20 GiB (documented); 6.34 GB sampled live at wasm-dev | **1.60 GiB** for `semio-s-artifact-stdio-semio`, wall 37.0 s; slowest unit is `-document` at 39.5 s / 0.86 GiB |
| Largest semio unit peak RSS, dev (wasm32) | (same monolith) | **1.48 GiB** for `semio-s-artifact-stdio-semio`, wall 24.7 s |
| Semio lib tests | 2953 / 3026 pass | **2953 / 3026 pass**, 3026 tests listed across 47 binaries. The failing set matches baseline test for test, and was re-run after the warning fixes. |

The peaks come from a 2 s `ps` sampler over the private build dir. The wall times come from `cargo --timings` of a full `--workspace --lib --target wasm32-wasip2` build at each profile. At wasm-release the heaviest units in the same build were dwg (4.36 GiB), pdf (3.75 GiB) and os-kernel (2.65 GiB). No semio unit exceeds 1.6 GiB, so semio now sits well below the 2.5 GiB target and is no longer a heavy unit.

## Wave S1: decoupling inside the monolith

1. **Shared base for text/binary helpers and child-reference codecs.**
   - New `✉️base/🚪️io/📝️text/🔤️codec/🦀️.rs` holds the wire2 primitives, which were deduplicated from drawing, flow, video, mesh and document.
     - It also holds the child and reference text codecs (`enc_/dec_ref`, `enc_/dec_child`, `…_opt`) and `plain`, the Display-flavoured float/point/transform/rgba parsers taken from the cad, mesh and model diffs.
   - New `✉️base/🚪️io/💾️binary/🔤️codec/🦀️.rs` holds the binary primitives and the child/reference binary codecs.
   - The refusal vocabulary moved to `✉️base/🧬️schema/🧬️mutations/🎯️outcome/🦀️.rs`.
   - `📏️native` gained `native_child` and `native_reference`.
2. **Object/kit cycle broken.** The union decode and encode live in inline modules (`union_decoding`, `union_encoding`) in the union sqlite file. Kit's `snapshot_codec` re-export is no longer test-gated.
3. **Orphan-rule impls moved into the subsets.** Each subset declares:
   - `artifact_retire_struct!` / `artifact_retire_leaf!`;
   - `semio_s_artifact_stdio_semio_base::semio_member_owner!(Snapshot, Mutation[, open = …])`. This is a `#[macro_export]` macro in `-base` that implements `SnapshotRetirementFactory<S>` for `SemioSnapshotRetirementFactory<S>` and `MemberStoreOwner<M>` for `S`.
4. **Conversions get their own roots.**
   - `🪆️subsets/<s>/🚪️io/📥️import/🦀️.rs` and `📤️export/🦀️.rs` are the roots, for 13 subsets each (26 roots).
   - Each root exports `entries()` and `pub const CONVERSION: SemioConversion = SemioConversion { declare, register }`.
   - Video, audio and animation leaves were normalized to `…::artifacts::<fmt>::<v_ver>::any`.

## Wave S2: cut into feature-free crates

- **47 new `📦️packages/🦀️rust/Cargo.toml` files.**
  - All carry role `library`. None has `[features]`, except `-base`'s `testing`, which kit's dev-dependency uses for cfg-test serde.
  - All are rlib-only, all package names are 48 characters or fewer, and all use workspace dependencies.
  - The import/export crates carry the `# 🧩️ Capability Dependencies` region.
  - `-catalog` carries `# 🔗️ Conversion Dependencies`.
- **Aggregator `semio-s-artifact-stdio-semio`.** It holds:
  - the envelope union plus `SemioMembers`;
  - `declaration(definition, conversions: &[SemioConversion])` and `register(conversions)`;
  - the base `SemioAny` editor and viewer.

  The `contribution`, `assembly`, `native_codecs` and `apply_diff` re-exports are removed. The subset sqlite snapshot tests are mounted under `#[cfg(test)] subset_sqlite_snapshot_tests`, so `--lib sqlite_snapshot_` still selects all 271.
- **`-catalog` (`🧿️semio/🧩️composition/🦀️.rs`).**
  - It holds `//#region 🔗️Conversions pub const ALL` (generated), plus `declaration`, `assembly`, `native_codecs` and `contribution`.
  - It is the only crate that links every conversion.
- **Taxonomy.** `🔤️codec` is admitted by a new semantic directory kind `representation-codec` (🔤️, `^codec$`, parents `text` and `binary`) in `🔣️taxonomy.json`. `🎯️outcome` resolves to the existing `comparison-outcome`. Nothing is placed under `🧬️schema`. `🔎️taxonomy-kinds.ts` resolves both. The full `verify taxonomy report` was cut at 580 s during `plan/references` (18 k of 130 k files), the same fleet-load limit recorded in 86 (i).
- **Explicit registration.** Consumers name their conversions:
  - raster: `register(&[drawing_import, drawing_export, image_import, image_export])`;
  - remodel: image, mesh and drawing;
  - layout and shooting: drawing;
  - the gismap test: drawing.

  The only host that declares the full family is `-catalog::declaration`.

## Wave S3: consumers and generators

- **Consumers.**
  - 258 source files were rewritten by `consumer_map.py`.
  - 41+ package manifests and 41 workspace roots were updated.
  - 85 lockfiles were refreshed offline. The base bridge and the hub lockfile picked up the new members on their first check.
- **Contract `semio.stdio.composition-contribution/v1`.**
  - `catalogFeatures` is replaced by `catalogPackage: string | null` + `symbolPackages: string[]`, with schema, fixtures (+4 vectors) and an Ajv oracle.
  - All 36 artifact contributions were updated.
  - Contract checks pass: 25.
- **Semio conversion generator (`🧩️composition/🔗️conversions/🟦️.ts`).**
  - It is schema-first (`semio.stdio.semio.conversions/v1`) and writes the catalog `ALL` region, the per-crate capability regions and the definition dependencies.
  - Checks pass: 43 (`conversion-definition-check`).
  - It is idempotent: a second `composition-prepare` changed 0 files.
- **Hub stdio composition generator (`🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts`).**
  - No more `appAssembly` or Composition Features region. The `full-artifact-catalog` and `home-io` features no longer exist.
  - The full catalog is the new feature-free crate **`semio-hub-stdio-catalog`** (`🔌️plugin/📇️catalog/📦️packages/🦀️rust`). gis, vcs and the native hub (`native-artifact-execution = [… "dep:semio-hub-stdio-catalog" …]`) link the catalog, not the stdio guest component.
  - Hosts declare an artifact through its catalog package (`semio_s_artifact_stdio_semio_catalog::declaration(…)`). The stdio-semio family component therefore serves every conversion that the definition's runtime requirements name. Hosting with `&[]` left 35 composer requirements unmet, which `shipped_fleet::every_package_hosts_the_runtime_of_every_kind_it_opens_in_its_own_process` caught. That law now passes.
  - New generated region `# 🧩️ Composition Test Dependencies` in `semio-hub-stdio`'s `[dev-dependencies]`. It lists every hosted package minus the direct ones, and is what `editor_catalog`, `shipped_fleet` and `native_openable_provider` name.
  - The mutation bridge moved out of `semio-hub-stdio` into its own tool package `semio-hub-stdio-mutation-bridge` (`🏭️bridge/📦️packages/🦀️rust`). Its generated `# 🧩️ Composition Dependencies` lists exactly the packages that the mutation descriptors live in (54).
  - `CATALOG_MANIFEST` and `BRIDGE_MANIFEST` are exported, and the production-preparation test copies them (2/2 pass).
  - The generator is idempotent: a second run changed 0 files.
- **Laws retargeted to the catalog crate.**
  - hub stdio `home-io-surface`: fixture v2, catalog crate feature-free with 36 artifact owners, no feature that selects artifacts (source mode passes);
  - hub `native-artifact-provider-frontier`: `pluginDependencies` changes to `["stdio-catalog","gis","vcs"]`, and the law is derived from the fixture (`execution-target-relay-check` passes; frontier checks=12);
  - hub `native-openable-catalog-provider-check` groups, retargeted to `semio-hub-stdio-catalog`;
  - the headless metadata capture/imports law: `semio_hub_stdio_catalog` with `features: []`. This also fixes its stale `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs` source path.
- **Test framework: subject host packages.** A split family's subject spans several crates. A subset crate builds on `-base` and dispatches through the kernel, yet the oracle role must not link the subject. The fix is schema-first and has a test:
  - **Schema:** new `SubjectHostPackage` and `subjectHostPackages` in the test protocol schema.
  - **Registry:** `subjectHostPackagesFor` in the registry, which merges per package and refuses a package declared from two paths.
  - **Host:** the Rust host materializer links them as optional dependencies under `sut` only.
  - **Test:** a new test-platform case passes 2/2.
  - **Semio declarations:** semio declares `-base` and os-kernel at artifact level, the aggregator for ✉️base, json for value, csv and value for table, and document for presentation. Trinity's oracle host package moved from the aggregator to the graph crate.
- **Artifact package runner.** `runArtifactRustPackageMain` gained `packages`, and `runArtifactRustPackagesTests` is added. The semio script passes its 47 workspace members, so the nx `check` and `test` targets cover the whole family (`bun nx run @semio-tech/stdio-semio-rs:check` checked all 47). The per-crate verify filters (`-base stdio_document_contract`, `-object`, `-kit`) replace the old `subsets::object::` and `subsets::kit::` filters.
- **Warnings.** `cargo fix` was applied to the semio workspace natively. The workspace was then re-checked at native and wasm32 and re-tested. Semio-owned warnings fell from 621 to 135. All 135 are pre-existing dead code or glob notices; the monolith baseline had 147.

## Verification (all run)

- **Semio workspace.**
  - `--workspace --lib --tests --all-features` native: rc 0.
  - `--workspace --lib --all-features --target wasm32-wasip2`: rc 0.
- **Hub workspace** (`--exclude semio-hub --exclude semio-hub-auth-client`): native rc 0 (211 units), wasm32-wasip2 rc 0 (206 units).
- **Hub stdio tests.**
  - `editor_catalog`: **177/177**.
  - `semio-hub-stdio-catalog` lib: 4/4.
  - `shipped_fleet`: 12/13 plus 2 ignored child probes. The one failure is `every_registered_snapshot_contract_resolves_in_each_package_process`: packages `stdio` and `stdio-pdf` register `s.stdio.pdf.1.7`, whose schema has an unresolved `Binary64Transport` cross-document reference. That is the pdf schema, lane E3.
  - `native_openable_provider`: 7/9. Neither failure is semio:
    - `reverse-object-fields` wire canonicalization (os-kernel `pack_rt`);
    - the tsv neutral fixture, where `compile_dsl` expects a semio preamble.
- **Plugin manifest sweep.** All **268** manifests were recorded: 223 ok, 25 bin-only, 20 failing.
  - The 25 bin-only manifests have no lib target. That is pre-existing.
  - The 20 failures are E0603 `app_contract` private, from `semio_framework_plugin::extension_exports!`. That is lane E2's plugin crate, re-confirmed at the end on flow/bim.
  - Spot re-checks after the warning fixes (raster, remodeling, cad, rewriting) are ok.
- **Repository-test subject phase** (exhaustive, rust) for all 19 semio owners. Every subject host now builds.
  - 13 owners are fully green: graph, value, flow, animation, video, model, table, cad, document, audio, text, image and brep.
  - The remaining failures are fixture or data issues that do not depend on the crates:
    - an undeclared `shared://` fixture (object 1, drawing 2, kit 2);
    - `slideMaster-druck` missing (presentation 2);
    - mesh `identity-round-trip`, documented red in its feature file.
  - ✉️base parity is 74/76. On `mutate-apply-graph` the oracle itself reports `matchesReference: false`.
- **Generators:** both are idempotent.

## Open items for other lanes

- **E2:** `semio_framework_plugin::extension_exports!` names the private `app_contract`. This blocks 20 extension manifests, among them 9 flow extensions, 4 process, 4 cad and 3 sourcing.
- **E3:** the pdf 1.7 snapshot schema has an unresolved `Binary64Transport` `$ref`. It fails `shipped_fleet`'s contract law for `stdio` and `stdio-pdf`.
- **A1:** the stdio-semio family component now links `-catalog` and the 26 conversion crates, because it must serve its definition's composer requirements. Check this component's link against the `wasm-component-ld` function ceiling when it is next built. The wasm32 check is green, but the component was not linked here.
- **Not run:**
  - the cargo half of `native-openable-catalog-provider-check`, because its nx runner refused a `--oracle-only` pass-through with "Original native artifact or network authority changed";
  - the `home-io-surface-native-check` cargo half. The hub wasm check covers both of its crates.
