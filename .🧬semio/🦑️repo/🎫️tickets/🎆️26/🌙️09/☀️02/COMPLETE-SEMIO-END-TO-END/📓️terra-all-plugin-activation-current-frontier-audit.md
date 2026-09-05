# All-Plugin Activation Current Frontier Audit

Current-source, read-only audit on 2026-09-04 after the native-provider and public-directory changes. No Cargo, Nx, or process build was started: active implementation lanes own those targets. This is source evidence, not an execution claim.

## Verdict — RED

The new native provider closes one honest D0 slice, but it cannot activate the installed plugin/artifact catalog. The first material **all-catalog** stop is not a taxonomy or compilation diagnostic:

`os-hub` statically links exactly the `stdio` provider and nothing else. A trusted profile selecting an executable artifact from any other plugin deterministically fails before server publication because the loader cannot find that artifact's exact native binding. The failure is fail-closed and avoids partial codec registration, which is correct; it also means a real OS/hub boot cannot load every installed plugin/artifact today.

There is an earlier operational precondition in every checked-in launch: no registered dev/secure launch provisions a trusted bundle/profile. The only `stdio-native-openable-v1` profile occurrence is a Rust test fixture. Thus the ordinary boot has no artifact authority unless a caller supplies undeclared inherited environment, while the isolated secure smoke explicitly removes both variables. This is a real launch gap, not evidence that the all-catalog provider works. It must be repaired without treating an environment injection as catalog activation.

## Current activation chain

1. `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5315-5321` constructs bindings before database/routes, then reads only the paired `OS_HUB_TRUSTED_CATALOG_BUNDLE` and `OS_HUB_TRUSTED_CATALOG_PROFILE` inputs.
2. `linked_native_codec_bindings()` at `📦️bin.rs:394-396` delegates exclusively to `NativeOpenableCatalogProviderV1::linked()`.
3. The provider imports `semio_s_plugin_stdio` at `🌎️hub/🗿️artifact-authority/🗂️📇️native-openable-provider/🦀️.rs:5`; `linked()` calls only its registry at `:18-22`; `from_receipts()` accepts only `plugin_id == "stdio"`, `package_id == "semio:stdio"`, and exactly 26 receipts at `:29-59`. `🌎️hub/📦️packages/🦀️rust/Cargo.toml:31-45` corroborates that `semio-s-plugin-stdio` is the sole plugin dependency of the hub binary.
4. `TrustedCatalogLoader::load()` maps each selected `(plugin, package, artifact kind, schema)` to a linked binding at `🌎️hub/🗿️artifact-authority/🗂️🛡️trusted-catalog/🦀️.rs:376-401`; an absent binding returns `selected artifact kind has no explicit native codec binding` at `:380-385`. It then rejects an extra linked binding outside the selected closure at `:402-407` and `:464-468`, before the assembly preflight/registration transaction at `:474-477`.
5. Only a successful verified catalog reaches `HubState.openable_catalog`; readiness derives `artifactAuthority` and `openPlan` from that same option at `📦️bin.rs:5347-5359`. There is no partial catalog publication.

The workspace genuinely contains other installed plugin packages—e.g. the GIS crate owns `gisterrain` artifacts and codec/schema leaves (`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/🦀️.rs:23-133`)—but the current hub has neither a GIS factory receipt nor any non-stdio provider import. A profile declaring an executable GIS artifact therefore reaches the missing-binding rejection above; it cannot be made executable by a generated marketplace row or a descriptor alone.

## First deployment blocker versus first all-catalog blocker

| Boundary | Current evidence | Classification |
| --- | --- | --- |
| Registered boot input | `🛠️dev🗄️os-hub` sets only port/data (`.vscode/launch.json:4521-4529`); the secure launches at `:4560-4619` do likewise. `startLocalHub()` explicitly deletes the pair for isolated secure smoke (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:615-620`). The sole `stdio-native-openable-v1` text is the temporary bin-test bundle (`📦️bin.rs:5619-5672`). | **RED: no shipped, verified launch input even for stdio.** A caller may inherit an environment pair in the ordinary dev runner, but that is neither registered nor reproducible evidence. |
| Linked executable authority | Provider/manifest are fixed to stdio as above. | **RED: the first actual all-catalog stop.** Non-stdio executable artifacts fail loader closure validation pre-publication. |
| Atomic loader/readiness | All bytes/hashes/descriptors and every selected binding are validated before batch registration (`trusted-catalog/🦀️.rs:321-478`); readiness remains false if no authority exists. | **Source-qualified positive.** Preserve this fail-closed boundary; do not add ambient discovery or partial publication. |
| Public directory | The public/member/author directory work changes discovery authorization; it does not create native codec bindings or a trusted launch bundle. | Out of the activation stop; it cannot supersede either RED. |

## Current tests and launch evidence

`native-openable-catalog-provider-check` is registered in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:2882-2911` and exposed by `.vscode/launch.json:4444-4452`. It independently runs its neutral oracle, exact-lists, and exact-runs six laws:

- the 26-receipt stdio factory bijection;
- complete, missing/extra/duplicate, and substituted stdio receipts;
- descriptor-owned surface publication; and
- a test-state readiness journey.

The script explicitly says at `📜️script.ts:2910` that it makes **no all-plugin or client-mount claim**. The readiness law itself creates `native_openable_stdio_bundle()` and manually installs its configured authority in `test_state` (`📦️bin.rs:5659-5692`); it is valuable fail-closed coverage, not an invocation of production `main` with a shipped multi-plugin bundle. No fresh terminal was run by this audit.

## Smallest honest dependency order

1. **Deployment-catalog input packet.** Produce one immutable, bounded trusted bundle/profile as a build receipt owned by the registry/build path, pass its exact paths through the registered hub launches, and preflight the exact pair before spawning the hub. Absent/missing/mismatched inputs must leave startup fail-closed; do not create a fallback scan of local packages or a generated-row trust bypass. This makes the existing stdio D0 slice reproducible only; it does not claim every plugin.
2. **Static provider-set packet (first all-catalog repair).** Replace the singleton call in `linked_native_codec_bindings()` with an explicit compile-time `NativeOpenableCatalogProviderSetV1`. Each selected package supplies a receipt/factory only through its linked Rust crate and verifies `(plugin_id, package_id, artifact kind, schema, pack-schema hash, factory identity)`. Merge all sets only after duplicate/zero/hash/schema/factory conflicts have been rejected. Retain the loader's selected-closure bijection and its single assembly transaction.
3. **Truthful installed-catalog classification.** A package/artifact without an actual native codec/open surface must be emitted as non-native/non-openable and excluded from the executable profile, not padded with a dummy binding. The release condition for “every installed artifact” is that each descriptor is either backed by a real native factory and open target or is removed/recategorized schema-first. This prevents a catalog row from falsely claiming execution.
4. **Full-profile process proof.** Build a multi-plugin bundle from real immutable component/descriptor bytes, start `os-hub` through the registered launch with that pair, observe ready only after the entire closure publishes, and open a representative document from each admitted provider in native, browser, and MCP. Negative cases must cover missing/extra provider receipt, a descriptor/component/hash/factory mismatch, a selected foreign artifact with no binding, an extra unselected binding, cancellation before assembly, and no partial readiness/catalog after any failure.

The first two packets are ordered: adding launch inputs without the provider set can demonstrate stdio only; adding a provider set without the immutable build receipt still leaves registered boot non-reproducible. Neither packet authorizes a false “all plugins” success until every selected package has an actual executable factory/surface chain.

## Acceptance boundary

Accepted only at source level: strict stdio receipt validation, closed trusted-loader assembly, and readiness coupling. Not accepted: a registered hub boot with a shipped catalog; any non-stdio native artifact; catalog-wide plugin activation; browser/native/MCP app mounting; or public-space discovery as an artifact-execution authority.
