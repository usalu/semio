# Current Bounded Cache Writer Coverage Refresh

Read-only authored Source review on 2026-10-15. No generated cache trees were scanned; no tests, builds, network, images, stores or Originals were acquired. The 2026-10-14 writer audit is stale and its first three priorities must not be repeated as current findings. This refresh changes only this report. `L` below means `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## Current Managed Roots

`L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` declares 64 GiB total, a 1 MiB control allowance, and finite partitions: Rust 16 GiB, tools 8 GiB, graph 4 GiB, dependencies 8 GiB, products 24 GiB, diagnostics 2 GiB, leases 64 MiB. Fixed backing paths are Rust `.🧬semio/🦑️repo/⚡️cache/rust/fixed` and `.cache/b/{t,g,d,p,o,l}`; mount roots come from nativeImageRootV1, not arbitrary environment paths. Bootstrap control is `.cache/b/c`.

Opening `leases` uses the bounded control directory to break recursive lease-image provisioning. Ordinary repository leases resolve through `repositoryLeaseDirectoryV1` to the leases image `data`; other partition openings retain the lease image through close. The opening owner retains actual image/store checks and bounded settled queue-claim retirement. This supersedes the old unbounded `.cache/framework/leases` finding for these managed paths. This Source refresh does not requalify physical platform adapters or expand their support.

## Confirmed Fixed Since Prior Audit

| Prior issue | Current actual Source |
| --- | --- |
| Direct workspace Bun install | `L/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts` now enters withJavascriptDependencyStorageV1 for each admitted scope. `💾️storage/🟦️.ts` projects exact Source manifests/lock/patches into dependencies `j/<generation>`, retains membership/Source facts, stages compact workspace paths and publishes authenticated node_modules bindings. |
| Bun cache/install writes | `dependencies/🟦️.ts`, runBun uses actual store.environment plus forced bounded bun-cache/global/bin, ignores lifecycle scripts, selects hardlink backend and explicit cache-dir. Actual child and recheck remain inside dependency volume/binding EX. Offline admission only reuses an existing immutable generation. |
| Bootstrap dependency receipt outside partition | `L/🏗️build/🚀️bootstrap/📜️script.ts` now authenticates real parser package/binding receipts and writes bootstrap receipt inside the same dependency store. It verifies locked versions plus bounded parser bytes. |
| Public pinBuildGraph bypass | Current graph owner uses retained immutable graph artifact/store routes; the old public pinBuildGraph writer is absent from inspected owner. |
| Inference env/fallback directory | `L/🏗️build/🕸️graph/📥️inference/🟨️.mjs:1823` uses repositoryBuildGraphWriterDirectoryV1. Actual `🏗️build/📥️inputs/💾️storage/🟨️.mjs` provides issued writer/dataset/fact scope; arbitrary SEMIO_BUILD_INFERENCE_DIRECTORY no longer selects the active writer. |
| Nx admission snapshots/candidates | Bootstrap `publishNxCollectionBoundary` now receives directory only inside withNxCollectionAdmissionStoreV1; snapshot bytes are capped, candidate writes use exclusive temporary+rename/cleanup. These are no longer old loose nx-admission cache writes. |
| Graph/show arbitrary --file | `L/🏗️build/📜️script.ts` now routes output through withRepositoryBuildReportV1. The report owner admits a closed name, finite JSON bytes, diagnostics store and actual reader/writer custody. |

## Nx Watcher And Daemon Coverage

`L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts` issues NX_CACHE_DIRECTORY, native cache, workspace data, TMPDIR and socket namespace from actual finite images and graph namespace custody. Watcher identities enter a bounded same-parent namespace, not an arbitrary directory. `📦️graph/🟦️.ts` authenticates original live AsyncLocal issuance and exact tooling/workspace/graph/implementation/watcher binding; copied environment text alone does not mint native or diagnostic writer authority. Watcher roots require physical directory identity and maximum membership. Diagnostics receive their own funded namespace. Daemon service resource and socket namespace remain distinct from artifact and graph cache roots; disabled marker bytes are closed and bounded before write. No fresh Source evidence here supports repeating the former generic unbounded Nx daemon/watch claim.

This is writer-boundary Source review, not proof that every supported Nx runtime path executed. Existing unsupported Rust cache adapters remaining cache:false are known scope, not a new finding.

## Remaining Concrete P1: TypeScript Artifact Build/Publication Still Writes Loose Roots

`L/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts:47–60,67–68` is an active normal artifact package command owner. Build stages into caller/environment-selected ticket generated path or `repoCacheDirectory(...,"typescript-staging")`; Check uses loose repoCacheDirectory `typescript-check-*`. It then publishes both package `📤️dist/<mode>` and package `dist` directly. `repoCacheDirectory` is still a plain join into `.🧬semio/🦑️repo/⚡️cache`, not a finite reservation.

`L/⚡️caching/📦️bundle/🟦️.ts:25–29,34,86–94` additionally stages Bun workers under arbitrary SEMIO_TICKET_DIR/generated or loose `cache/bun-workers` and copies successful bytes/maps to arbitrary options.output after a path-length check. The esbuild branch supplies arbitrary output directly as --outfile. Source/output hash validation authenticates content, not physical capacity. The actual production caller above reaches both build/check and repeated retained package output growth; this is not merely a hypothetical exported helper.

`L/⚡️caching/📦️artifacts/🟦️.ts:7–9` only adds a publication lease directory before calling stageArtifacts. That gives publication coordination but does not issue a products partition reservation or require the output path to belong to one. Thus the old bundle-output concern remains confirmed by an actual caller. Managed Nx wrapping may cover some selected jobs, but these package commands themselves lack finite output custody and can execute directly through existing Script routes.

Minimal coherent repair: issue diagnostics/temporary custody for worker/check/declaration scratch, products custody for immutable per-mode outputs, and bind package-facing active paths under that owner. Change the shared artifact TypeScript command and bundle boundary together; require actual reservation/owned destination instead of raw output/environment strings. Keep source lockfiles/package inputs distinct. Do not add another path allowlist and call it a physical bound.

## Remaining Diagnostic Writer Escape

`L/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts`, ContractCheckScript, still accepts process.env.SEMIO_TEST_ARTIFACT_DIR or defaults to loose `cache/tests/bootstrap-dependencies`, mkdirs it directly, and launches the test child with that inherited destination. This is the old diagnostic gap still present, unlike the repaired installer. Route the existing contract-check command through actual diagnostics custody and exact child destination; arbitrary env must not issue storage. Priority is below the artifact product gap for normal repeated build volume, but it is a concrete active generated writer.

## Preserved Baselines And Startup Policy

Dependency binding retains preexisting unqualified node_modules directories/links under `.cache/held-javascript`, with bounded-store retained.json recording physical original identity. These are preserved input baselines, not newly installed generated bytes, and must not be deleted/relabelled as disposable cache to satisfy accounting. Repeated baseline retention can add host namespace entries; close the lifecycle explicitly if the fixed-size objective includes old acquired inputs. Current install bytes themselves now remain finite; the old direct-install P1 is resolved.

`🧰️framework/🔨️modules/🏃️process/🏗️build/🎛️environment` now owns the closed startup environment law requiring BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 and NODE_DISABLE_COMPILE_CACHE=1. Tests parse both editor catalogs with jsonc-parser and independently query every selected launch row using SQLite. Parent reports BOTH 2/18285 and strict exit 0; this audit did not run them. This establishes authored editor startup coverage at the tested selection, not arbitrary manually spawned external processes. Do not revive implicit runtime-cache startup as an unfixed editor P1 on the prior report alone.

## Result

Current actionable remaining Source work is the shared TypeScript artifact/bundle temporary and output publication boundary, followed by the explicit dependency contract-check diagnostics destination. Current dependencies, lease bootstrap/ordinary leases, graph/inference/admission/report and inspected Nx watcher/daemon boundaries have materially changed and should be treated through their new real custody owners. This is a bounded writer closure refresh, not an exhaustive assertion about every unrelated product exporter in the monorepo or a runtime/cache-size result.
