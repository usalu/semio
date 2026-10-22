# Authenticated Provider Boundary Design

Status: source investigation and design constraints; no implementation or runtime acceptance. This report follows actual Source0 session 36712, whole 1 before any test body or backend admission.

## Observed Boundaries

The immutable implementation loader in `library/🏗️build/📥️source/🔏️closure/🟦️.ts` admits a createRequire base only when its physical module is in the captured image map. Nonbuiltins require an exact declaring-module import-map entry. The normal import hook similarly requires captured images and declared resolutions. Its transform already assigns physical file URLs to import.meta.url; URI normalization is not an evidenced defect.

The parser bootstrap witness in `library/🏗️build/🚀️bootstrap/🟦️.ts` used root package.json DATA as its createRequire base. Its operation is resolve-only for typescript and @iarna/toml package manifests and entrypoints, followed by actual store-path, generation binding, exact lock version, containment, and complete bounded package-tree checks. High owns the closed resolve-only repair; those receipt checks must remain.

The later Graph native-manifests, script-inputs, and owner-discovery modules dynamically require Iarna or TypeScript from their captured declaring files. Their external execution is separate from the witnessed resolve-only bootstrap failure. The current immutable Source map intentionally excludes external runtime modules. No later runtime refusal or wrong managed Bun selection is claimed from this static review.

The Nx SDK facade in `library/⚡️caching/🚀️bootstrap/🛠️tools/🔌️sdk/🟦️.ts` has a closed policy catalogue but currently takes only a base string. The genuine ownership seam exists in tools/📥️use/🟦️.ts: withNxToolingUseV1 checks actual identity, toolkit realpath and receipt, owns the shared tooling lease and repository tooling store, and enters withNxSdkDerivativeReaderV1 through the complete callback/cleanup lifetime. The derivative reader validates material, snapshot, source index, authorizer receipt and complete namespace. A copied receipt DTO alone does not reproduce this authority.

## Required Coherent Design

Keep repository Source imports immutable and closed. External parser or SDK execution must use an explicit first-party provider port issued within the existing actual generation or derivative-reader lifetime, carrying the same genuine Original and control. Its package/module catalogue and verified physical provider roots must be closed. The port must be revoked when its callback settles, and must preserve cancellation, first failure and cleanup. It cannot permit physical reloading of first-party repository Source or allow an arbitrary package.json/base path to grant execution. Prefer an existing authenticated child/SDK seam over a new generic bridge. Root, High and Low are checking the exact interface before broader changes.

## Handled Replay Refusal

The actual Source0 log contains six handled current-dependency-generation-receipt-failure records, one per scope. Each reports an uncaptured routing-module import with eight parent traversals before the framework process module path. These records precede the final parser-witness failure and do not replace its terminal cause. Root independently read the exact log region; the same routing target occurs in the observational 857 source inventory, but that inventory is not the actual admitted import map. The missing declaring-module resolution must be traced from actual capture-map evidence; no broad import fallback or actor/loaded-byte attribution is justified. Repeated ownership walks are observed, but this report does not claim a measured causal speedup.

## Runtime Identity Qualification

The failing captured bootstrap reports Bun 1.3.14; current package/tool declarations pin Bun 1.4.2. The failure precedes managed target/backend admission, so these facts do not prove incorrect managed runtime selection. Preserve the bootstrap-versus-managed distinction and validate the eventual real selected provider.


Root bounded public repo-capacity rg checks for redirect in the closure root TS and closure subtree returned no matches (including explicit hidden/no-ignore retry); all commands used128MiB/oneCPU and terminated1/no matches. This did not inspect the actual redirected-parent owner identified by peer and establishes no startup cause. Peer exact compiler→redirect→function serialization trace and bounded real evidence remain authoritative; no Root source edit or extra product probe resulted.
