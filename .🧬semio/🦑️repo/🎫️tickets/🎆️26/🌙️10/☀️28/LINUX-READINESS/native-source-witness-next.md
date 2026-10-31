# Native Source Witness: Next Owner Checkpoint

Read-only Source plan, 2026-10-29. No script changes, Source copies, native/container/Cargo/compiler/tests/process/lease operations were performed. The installed candidate remains stale until the owner completes the broad checkpoint and actual rebuild/install acceptance.

## Finding

The historical ticket file called a build manifest, `🗑️generated/container-native-build-source.json`, is **not an authoritative canonical Go compiler contributor manifest**. It has exactly the same selected-extension format as capture-container-source.cjs: {version:1, coverage:string, rows:[{path,bytes,mode,sha256}]}. Its coverage explicitly says all present nonignored Go plus changed/untracked implementation/configuration files. The historical1257rows/800Go count does not establish every runner/schema/asset or exact linked ELF input.

Current capture helper now calls Git through registered repo and adds rs/cs/native/shader/etc extensions. That corrects the earlier changed-Rust omission, but still omits unchanged non-Go inputs and newly discovered nonmatching assets. Broadening extensions cannot turn it into exact compiler provenance or an all-code cold-setup witness.

## Existing Authoritative Pieces

Paths relative to `🧰️framework/`:

1. `🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts:244` runCanonicalGoBuild is the actual Go owner. At249 it selects pinned environment and canonicalGoPlan; at252 writes the exact {Replace:plan.replacements} overlay; at254 invokes framework buildGoArtifactsV1. The overlay is removed at256, so the historical ticket census does not automatically preserve this actual compiler projection.
2. `🔨️modules/🏃️process/📦️artifacts/🐹️go-build/🟦️.ts` buildGoArtifactsV1 owns mode flags, admitted compiler capacity, staged output and publication. Policy schema is adjacent 🧬️schema/🔣️.json; policy {version,mode,cwd,captureDirectory,leaseDirectory,budgetMs} is not a Source contributor receipt. Current inspected implementation does not persist an exact Go dependency/embed/overlay-source manifest or bind one to the output digest.
3. Repo library `⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts:151` has an existing **authoritative admitted JS runner capture**, NxAuthorizerSourceManifestV1: {version:1,mode:"release",entry,resolutions:[{specifier,path}],sources:[{path,sha256}]}; digest binds JSON.stringify(manifest)+"\\n". Adjacent snapshot/🟦️.ts carries base64 Source bytes **internally**; do not copy that payload into tickets. Metadata-only manifest/hash projection is appropriate evidence.
4. Repo library `🏗️build/📥️source/🔏️closure/🟦️.ts` captures BuildSourceClosureV1 from actual admitted parent Source map. Its internal files carry path/sha256/base64/imports. requireAdmittedBuildSourceClosureV1 checks exact admitted Original. This is authoritative runner/helper execution closure, not the Go package closure. Do not call the capture standalone without its genuine Original to manufacture provenance.
5. Native install Receipt at /home/vscode/.local/share/semio/repo/installed.json records version/platform/digest/executable/directory/source/mode. It has no Source-manifest digest. Binary/Receipt physical validation and pre/post digest rechecks establish installed artifact identity, not complete build inputs.

No single persisted combined exact Native-Go installer contributor manifest was found in these actual owner paths. Reuse these existing admitted captures for their real scopes; do not relabel the extension census as such a facility.

## Smallest Honest Owner Witness

At the next coordinated Source freeze, preserve metadata-only evidence for **three explicitly separate scopes**:

- Actual admitted installer/runner manifest from the canonical operation, entry/resolution/source hashes and manifest digest, without base64 Source payload.
- Actual Go projection and selected compiler inputs from the same canonical build: retain the exact overlay Replace identities while live, selected module/workspace/go.sum inputs, platform/toolchain/build tags/mode, generated Go files and any real go:embed resources. The current owner does not persist this full set; extending its schema-owned provenance is required before claiming an exact linked-input witness. Do not run an independent unauthenticated go list/build as a substitute.
- Broad workspace selected-input census for host/private-overlay synchronization, accurately labelled partial. Full setup has a much broader multi-language closure and its own actual canonical graph admission; no Native Go receipt can certify all-code cold setup.

The immediate practical checkpoint can still rebuild/install canonically under a coordinated broad freeze, retain actual command/graph identity and bound binary digest/Receipt to the broad saved census. Describe it as current frozen selection acceptance, not exact compiler contributor certification. New or changed contributors invalidate the freeze even if ELF bytes happen to remain equal.

## Stable Host/Container Comparison

For both views compare the **same immutable saved metadata document**, never regenerate and overwrite it between before/after. Preserve relative path/size/mode/SHA256; root-specific absolute paths differ by host/container and must not be compared as equal.

For each selected file use no-follow regular-file admission, open descriptor identity, bounded chunk hashing, and before/after dev/ino/size/mtimeNs/ctimeNs/mode agreement. Rewalk selected member/manifest/import/embed sets to detect additions/removals, not only changed existing rows. Perform host-before + container-before, canonical build/install, then container-after + host-after with the same saved membership. A quiet-window promise plus per-file checks does not make a tree-wide atomic snapshot; explicitly invalidate/restart on any membership drift or contributor change.

Current capture checks lstat identity before/after each read; verifier currently only checks shape/hash and lacks descriptor/post-read stability and membership recensus. Its ELF header check is container-specific and not a host-neutral recheck. Treat zero drift from that helper as exactly its limited listed-row evidence.

Retain ticket input scripts/reports, place metadata/log outputs only under generated, never copy Source files, and preserve protected caches. Exact Go provenance improvement and full cold devcontainer lifecycle remain distinct tasks; neither is achieved by this audit.
