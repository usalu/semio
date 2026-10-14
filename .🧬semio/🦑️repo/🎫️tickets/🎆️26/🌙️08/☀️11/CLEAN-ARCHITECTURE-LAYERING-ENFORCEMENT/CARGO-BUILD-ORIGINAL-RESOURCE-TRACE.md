# Cargo Build Original Resource Trace

Read-only static trace; no factory, issuer, import, Store, test or runtime execution. Build script SHA-256 `72d89666c666f002b498123d7a978cd231d6f3d341b3aa89fff776f9f078eed4`; build invocation SHA-256 `dfda026fae3fe0e39fb1c9bb8297cb84e55f8b1e19867b58026600ce871cc21c`.

The suspected replacement at Repo library `🏗️build/📜️script.ts:80` does **not** discard original native resources on the valid explicit-authoring launch path inspected. Fresh configured defaults are computed there, but its ancestor branch restores `original.native`.

Actual call chain:

1. `package.json:112` maps `bun repo` directly to `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo`. The large root script is not this package-command path.
2. Build script line188 calls `runBuildCliV1`; lines180–184 use an incoming admitted envelope if present, otherwise call `createBuildBootstrapCapabilitiesV1` with the actual environment before creating the initial envelope.
3. Factory lines59–63 reads explicit `SEMIO_SCRIPT_CAPABILITIES` through `readBuildAuthoringCapabilitiesV1`. That reader (caller file lines28–34) validates exact initial selection/launch, canonicalizes workspaceRoot to an absolute root, resolves artifactDirectory, and records the selected tool. Thus the new authoring rows' `workspaceRoot:"."` does not reach ancestor validation as an uncanonicalized relative root.
4. Router line185 dispatches RepoScript line68; it calls `runOwnedBuildInvocationV1`, which reaches `runOwnedBuildInLeasesV1` line80 through its existing genuine Store scope at line73.
5. Line80 computes fresh configured capabilities with raw authoring disabled, then evaluates `readBuildGraphAncestorSelectionV1(root,parent.capabilities)` before choosing which native resources to carry.
6. Ancestor reader lines152–159 returns undefined only when there is no caller or both native and selection are absent. A valid admitted explicit build caller has native resources and selection, absolute workspaceRoot, tool, and matching declared launches; it validates and returns the parsed original graph selection. Malformed present authority throws rather than selecting the defaults branch.
7. Therefore line80 selects `{...issued,native:original.native,...}` for the inspected direct explicit-authoring route. Original maximumLines65536 and child600000 remain; configured default maximumLines1000000 and child3600000 are computed but not selected. The original policy/control stay on the parent spread.

This conclusion applies to valid explicit-authoring launch6 and a Native11 caller with the same valid admitted build-caller shape. It does not establish that an arbitrary malformed Native11 envelope runs, and it does not claim runtime acceptance. Callers without native and selection enter the separate defaults branch intentionally; that is not the inspected explicit-authoring route. No confirmed original native-resource expansion was found here, and no edit was requested.
