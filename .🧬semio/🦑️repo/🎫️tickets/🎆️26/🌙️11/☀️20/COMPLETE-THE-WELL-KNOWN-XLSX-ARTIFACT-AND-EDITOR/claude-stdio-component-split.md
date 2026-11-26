# Stdio component split: findings and decision record

Status: analysis only. No source file was changed. The requested carve (move CAD/BIM/imaging families into extension components) already exists; what still bloats the stdio core is something else, and fixing it is an authority redesign, not a copy of the office pattern.

## 1. The premise is already implemented

`🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/` is tracked in git (commits since 2026-11-21, see `git log -- 🧩️extensions`) and holds NINE extension plugins, not one:

| plugin id | folder | apps carried |
|---|---|---|
| stdio-office | 💼️office | docx, pptx, xlsx, idml |
| stdio-cad | 🛠️cad | step (cc1-6), dxf, dwg |
| stdio-bim | 🏠️bim | ifc, bcf |
| stdio-mesh | 🔺️mesh | gltf, obj, stl, ply, las |
| stdio-image | 🖼️image | svg, png, jpg, gif, bmp, tiff |
| stdio-media | 🎵️media | mp3, mp4, avi, wav, epw |
| stdio-pdf | 📘️pdf | pdf (with 1.4 and 1.7 shards) |
| stdio-binary | 🔢️binary | binary, zip, deflate |
| stdio-semio | 🧿️semio | semio family |

Each has its aggregate lib (`📦️packages/🦀️rust`) and its cdylib component (`🧩️component/📦️packages/🦀️rust`), both listed in `🌎️hub/Cargo.toml` workspace members (lines 161 to 184), in `📋️build-owners.json`, in `🏗️bootstrap/…/bootstrap-selection/🟦️.ts` (`TRUSTED_BOOTSTRAP_ALL_PACKAGES` and the per-plugin component rows), in the composition owner list `🧩️composition/🔣️.json` (10 owners) and in the per-artifact `🧩️composition/🔣️.json` app rows (`"owner": "stdio-cad"` and so on). The HEAD stdio plugin (`🔌️plugin/🦀️.rs`) already carries only the seven text families (html, md, csv, tsv, txt, json, xml).

So a second or third extension component for ifc, dwg, gltf and imaging would duplicate crates that exist. Nothing to add there.

## 2. Why the stdio core is still 934k functions

The extension split moved only APPS (editors and viewers). The OWNERSHIP of every kind stays in the stdio core by design: `PluginBuilder::host_artifact` says "hosting is not owning ... its definition is never registered here ... no descriptor or hub catalog row moves". The core plugin builds `semio_hub_stdio_catalog::selected_registry()` from the generated `selected_contributions()` (38 contributions: binary, txt, xml, deflate, zip, json, csv, md, gltf, obj, stl, ply, las, step, ifc_catalog, dwg, dxf, svg, png, jpg, gif, bmp, tiff, pdf, docx, pptx, xlsx, bcf, semio_catalog, mp4, avi, mp3, wav, epw, tsv, html, idml, idf) and applies it with `AssemblyPlan::apply` (runtime declarations, native codec kinds, receipts, the committed `stdio.artifact-catalog` topic). Every extension then hosts the SAME runtime again through `host_artifact`. The heavy runtime is therefore linked twice: once in the core (as owner) and once in the extension (as host).

Measured function counts (core module function section of the stripped components, `claude-core-function-count.py`):

| component | functions | wasm bytes | built |
|---|---|---|---|
| semio_hub_stdio_component | 934,477 | 402,046,338 | 2026-11-25 23:36 (HEAD at that time) |
| semio_hub_stdio_office_component | 445,753 | 210,167,265 | 2026-11-25 23:48 |

The current tree no longer links: `retry-6.log` (2026-11-26 08:50) fails in `wasm-component-ld` with `functions count exceeds limit of 1000000`, so the module is never written and the new count cannot be read from a wasm. Growth since yesterday comes from the uncommitted work in the same area (per-format app libraries `🗃️apps/*`, the new `StdioApps` variants, owner-list edits in `🧩️composition`), which another agent is actively editing (many files modified within the last hour).

## 3. Attribution (pre-link-GC, from the intermediates of the last build)

`claude-rlib-function-weights.py` sums the defined functions in every wasm object of every rlib under `🗑️generated/claude-pane/native/build/wasm32-wasip2/wasm-prototype` (1220 rlibs, 2.95M functions before link GC). `claude-contribution-attribution.py` intersects `cargo tree` closures of each catalog contribution to find which crates only that contribution pulls in (exclusive weight). Pre-GC totals overstate the final module (stdio: closure 2.92M pre-GC against 934k final, ratio 0.32; office: 0.64M against 446k, ratio 0.69), but the ranking is what matters:

| contributor to the stdio core | exclusive functions (pre-GC) | share of 2.92M |
|---|---|---|
| semio catalog runtime (26 conversions to every family) | 736,646 | 25 % |
| ifc catalog runtime (all schema subsets) | 262,076 | 9 % |
| text-family apps (html, md, csv, tsv, txt, json x2, xml x2) | 243,418 | 8 % |
| gltf | 26,549 | 1 % |
| xlsx / docx / pptx / idml | 21,751 / 17,478 / 14,264 / 11,356 | 2 % |
| epw | 21,273 | 1 % |
| shared by all contributions (framework, plugin, pack, contract) | 439,673 | 15 % |
| remaining families (pdf, tiff, step, png, bcf, zip, svg, obj, ply ...) share their dependencies and are 70k to 255k each | counted above in shared closures | |

Reading: the carve candidates named in the task (dwg, dxf, step, gltf, obj, stl, ply, las, bcf) are small as exclusive weight. The two big single levers are the semio catalog and the ifc catalog runtimes, and the text apps are a smaller third. Moving the CAD/BIM apps again changes nothing, because their apps are already out.

## 4. Why no code was changed

Removing the semio and ifc runtime (and any other extension-hosted runtime) from the core means the core stops being the owner with a runtime declaration. That touches, all at once:

1. `ArtifactAssembly::Definition` versus `Runtime` in `✏️s/🔌️plugins/🗄️stdio/🦀️.rs` (`AssemblyPlan::new` enforces a factory, runtime and receipt bijection) and in `📇️registry/🧬️contract/🏗️assembly`.
2. The committed catalog topic and its exact-equality validators (`validate_native_artifact_catalog_contributions`, `validate_native_codec_artifact_kinds` in `🔌️plugin/📇️catalog/🦀️.rs`), which compare the stdio descriptor against the FULL native catalog and accept exactly one `stdio.artifact-catalog` topic.
3. `📜️native-codec-factories.json` (every receipt is `plugin_id: stdio`, one provider `stdio/native-codecs/v1`) and the trusted-catalog authority in `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog` (provider-selection, owned-guest and open-target laws).
4. The committed binary descriptors (`🛂️.descriptor.semio` and `🤖️generated/📜️catalog/🗜️.bin`), which can only be regenerated from a successful wasm build of every plugin, each of which is a multi-hour guarded build.
5. `describe` semantics: composer rows exist only for kinds the plugin owns, codec rows only for kinds its apps open, so moving ownership moves hub catalog rows.

That is a decision about where descriptor authority lives, not a mechanical copy of the office pattern, and it cannot be verified in this environment (23 GiB free disk against the resource guard's 30 GiB minimum; the single guard slot is held by another agent's job, pid 71361 at the time of writing). Changing the shared composition generator or owner list in the middle of the other agent's concurrent edit of exactly those files would also risk losing their work (last writer wins). A half-moved ownership would break the trusted-catalog laws, so I did not start it.

## 5. Recommended path (ordered by payoff over risk)

1. Owner-split of runtime, starting with the two big catalogs. Let the `stdio-semio` and `stdio-bim` plugins OWN `semio` and `ifc` (definition, runtime, receipts) and give each plugin its own catalog topic `stdio.artifact-catalog.<plugin>`; the validators already match that topic prefix. The core registry then drops `semio_catalog` and `ifc_catalog` (about 34 % of pre-GC closure weight, an estimated 280k to 350k final functions), landing the core near 600k to 650k functions. Needs: per-owner `selected_contributions()` in the generator (`🧩️composition/🟦️.ts`, owners already listed), per-owner receipts and provider ids in the trusted catalog, regenerated descriptors.
2. If the owner decides that only descriptor authority must stay in stdio: give contributions a data-only form (`definition` plus format claims plus kind spec plus receipt data without `factory` and `declaration` pointers) for the core, while the extension keeps the full runtime. This removes the same weight with no authority move, but needs the guest `describe` to accept receipts without a codec pointer.
3. Only after 1 or 2: move the text apps into a `stdio-text` extension (about 8 % pre-GC, roughly 80k functions) or keep them in the new `🗃️apps` per-format libraries that the other agent is introducing.
4. Build-profile levers, untested and independent of the above: `codegen-units=16` makes every CGU re-instantiate inline functions, so `codegen-units=1` for the two giants lowers the function count at the price of single-CGU memory; the 6.3 GB thin-LTO link already shows fitting the count is possible but not under the warden. Measure before adopting.

## 6. How to measure and build

```
T/claude-resource-guard.sh T/claude-pane-cargo-logged.sh      # builds stdio, then stdio-office, guarded, -j 4
python3 -I T/claude-core-function-count.py T/🗑️generated/claude-pane/native/target/wasm32-wasip2/wasm-prototype/semio_hub_stdio_component.wasm
```

The guard waits for 30 GiB free disk and 30 % free memory; free space first (the `🗑️generated/claude-pane` build directory is 6.3 GB of intermediates that the attribution scripts need, so run `claude-rlib-function-weights.py` before deleting them). The wasm only appears once the function count is below the one-million limit.

## 7. Tests

None added or changed, because no source changed. The laws that will have to keep passing after step 1 or 2 are `🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet`, `🧩️component-assembly`, `✏️editor-catalog`, `📇️native-openable-provider`, `🧩️composition/🧪️tests/🪪️owner-pairs` and the trusted-catalog provider-selection and owned-guest laws.

## 8. Scripts left in this folder

- `claude-core-function-count.py`: function count of the core module of a component wasm.
- `claude-rlib-function-weights.py`: per-crate defined-function counts from rlibs (writes JSON, takes the build dir and the output path).
- `claude-contribution-attribution.py`: exclusive weight per stdio catalog contribution (needs `fc.json` from the previous script in its scratch folder argument and `CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER=` in the environment).

## 9. Follow-up: the coordinator's owner-move decision hits a hard invariant (no code changed)

Decision received: stdio-semio owns `semio`, stdio-bim owns `ifc`, each with a `stdio.artifact-catalog.<plugin>` topic. Blocker found before any edit:

- `PluginBuilder::try_build` runs `preflight_artifact_identity(plugin_id, kind)` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️identity/🦀️.rs:14`): `ArtifactKindId::plugin()` must equal the declaring plugin id, otherwise `plugin-assembly.artifact-owner` refuses the build. A kind `s.stdio.semio` can therefore only be owned by the plugin id `stdio`. `host_artifact` exists precisely because of this (owner = identity namespace, host = runtime).
- Making stdio-semio the owner means renaming the kind to `s.stdio-semio.semio`: `s.stdio.semio` appears in 4,289 files (energy 1,221, stdio 869, architect 689, remodel 302, norm 229, cad 112, block, note, gis, lowpoly, raster, forms, process, trinity, sourcing ...). `s.stdio.ifc` appears in 143 files. That is a repo-wide kind rename across other agents' plugins plus every stored document, format claim and descriptor, not a stdio carve.
- Native receipts are NOT affected either way: `semio` and `ifc` publish no native codec (neither is in `📜️native-codec-factories.json`).

Alternative that respects the invariant and gets the same saving (about 1.0M of 2.92M pre-GC functions, estimated 280k to 350k final):
1. Keep identity ownership in `stdio`, but make the core assemble `ArtifactAssembly::Definition` (data only, no runtime declaration) for `semio` and `ifc`; stdio-semio and stdio-bim keep hosting the full runtime via `host_artifact` as today. Per-artifact switch `"runtimeHost": "stdio-semio"` in the artifact `🧩️composition/🔣️.json`, read by the generator.
2. Per-host catalog topic `stdio.artifact-catalog.<host plugin>` for the hosted runtime rows (executable capabilities), core topic keeps the definition rows; validators take the owner/host id; vcs/gis keep the full bundle (unchanged roster).
3. Open semantic risk: `describe` lists composer and inference rows only for kinds a plugin owns, so composer/inference rows of `semio` and `ifc` would disappear from descriptors unless `describe` is taught to list hosted rows; this can only be verified by the guarded builds and descriptor regeneration.

Decision needed: (a) the kind rename (huge), or (b) the definition-only core plus runtime-host topics above. Nothing was edited meanwhile, so the other agent's per-format apps work is untouched.

## 10. Implemented: definition-only core plus runtime hosts (coordinator decision)

Design: identity ownership never moves (`s.stdio.semio` and `s.stdio.ifc` stay owned by plugin `stdio`); the core assembles them as `ArtifactAssembly::Definition`, and stdio-semio and stdio-bim host their complete runtime and publish `stdio.artifact-catalog.<plugin>`.

Files changed (schema first):
- Composition contract: `🧩️composition/📋️contract/🧬️schema/🔣️.json` (optional `runtimeHost`), `📋️contract/🟦️.ts` (type, admission, rule: host needs a catalog package, no native factory, owns every app), contract tests and fixture (`🧪️tests/🟦️.ts`, `🧫️fixtures/🔣️.json` case `runtime-host`).
- Artifact rows: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧩️composition/🔣️.json` (`runtimeHost: stdio-semio`), `🏗️ifc/🧩️composition/🔣️.json` (`runtimeHost: stdio-bim`).
- Generator `🧩️composition/🟦️.ts`: `selected_contributions()` emits a definition-only contribution for hosted artifacts, `selected_runtime_hosts()`, `selected_runtime_contributions()` (full runtime view), the catalog crate depends on the artifact package instead of its catalog package for hosted artifacts, host plugins call `runtime_host_artifact(<catalog>::assembly()?.into_runtime()?)` and `contributes_topic(hosted_artifact_catalog_contribution_for(..))`, and a host must be a declared deployable owner. Run directly with `claude-prepare-composition.ts` (the router entry needs a parent invocation envelope).
- Registry and contract: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs` (`with_runtime_hosts`, `runtime_host_of`, `hosted_by`, `runtime_host_ids`), `🧬️contract/🏗️assembly/🦀️.rs` (`ArtifactAssembly::into_runtime`).
- Projection and catalog: `🔌️plugin/📇️catalog/🏗️projection/🦀️.rs` (identity-parameterised commitment, per-topic publication, owner view projects hosted definitions with their authored executables so the owner topic equals the complete runtime topic, `hosted_artifact_catalog_contribution_for`), `📇️catalog/🦀️.rs` (`runtime_registry`, `runtime_artifact_assemblies`, `runtime_host_plugin_ids`, `expected_hosted_artifact_catalog_contribution`, `validate_hosted_artifact_catalog_contributions`).
- Trusted-catalog side: `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs` validates a runtime host's dependency and its hosted topic.
- describe at the root: `🧰️framework/…/🚪️io/📇️composer-entries/🦀️.rs` (`register_runtime_hosted_artifact_kind`, `runtime_hosted_artifact_kinds`), `…/🔌️plugin/🏗️builder/🦀️.rs` (`PluginBuilder::runtime_host_artifact`), `…/🔌️plugin/🛂️describe/🦀️.rs` (composer, io and inference rows of runtime-hosted kinds are listed by the host).
- Tests: contract and generator (`🧪️tests/🏠️runtime-host/🟦️.ts`, bun test, 2 pass), describe law `a_runtime_host_lists_the_io_rows_of_the_kind_it_alone_runs`, catalog laws (`runtime_hosted_artifacts_are_definitions_for_the_owner_and_runtimes_for_the_host`, `the_owner_catalog_commitment_equals_the_complete_runtime_catalog_commitment`, `each_runtime_host_publishes_its_own_exactly_validated_catalog_topic`), fleet law `every_runtime_host_descriptor_commits_exactly_its_hosted_runtime_catalog`, and the existing fleet laws now read `runtime_artifact_assemblies()` where they mean runtime declarations.
