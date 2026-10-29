# Preservation Capsule Source Opacity

Bun workspace and Cargo membership discovery already skip dot-prefixed directory names, but several graph/compiler input walkers exclude only literal generated names. Renaming node_modules into .p8hex/p removes the node_modules component from its payload paths. Existing literal exclusions therefore do not make these capsules opaque. No heavy graph or payload count run by audit.

## Proven Gaps Relative to Repo Library

| Source/function | Predicate needing capsule admission refusal |
|---|---|
|⚡️caching/🔍️discovery/📂️source/🟦️.ts walk27|CACHE_POLICY.generatedDirectories literal-name check|
|🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs walkCargoToml35/40, walkCargoTomlControlledV150/57|Only .🧬semio and generated literal names excluded|
|same inferenceCandidates67/76|Path component literal-name check permits supplied .p/p config|
|same readBuildSourceInputMatchesV1148/150|Recurses all directories except literal skip list|
|🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs147,200|Authored config/native owner filters test literal generated names|
|🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs componentCrateRoots99|Cargo config list component filters permit .p|
|🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs36,103,151|Catalog manifest owner filters use generated literal names|
|🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs precisionManifestSkipV181|Precision Cargo walk receives only literal names|
|🏗️build/📥️inputs/🟨️.mjs289,302,311,329|Owner tree expansion, emitted negative input globs, native input construction and path filtering need pattern parity|
|🏗️build/🔧️tools/🏗️bazel/📜️script.ts bazelFilesystemSourceFilesV1215/217 and BAZEL_SOURCE_EXCLUDED221|Filesystem fallback literal skips; Git witness also needs reserved Source capsule classification|
|🏗️build/🩺️doctor/🔎️probes/🕸️graph/🟦️.ts newestImplementationMillisecondsV111/19|May count capsule source copies in implementation freshness heuristic|

The temporary generated opacity-predicates.json contains all matching source predicates for review; cache/internal collectors in that list are not automatically Source defects.

## Already Opaque and Native Search Boundary

Bun physical walkPlan97 skips any dot-prefixed child. Cargo workspace memberRules134 treats name.startsWith dot as hidden. Repository-scan198/212/224 likewise skips hidden directories. Go workspace ListDirEntries140 and WalkDir161 skip hidden; codebase GetFolderChildren132 excludes dot directories, technologies61/293 does too. These protect the corresponding discovery routes. However workspace IsRepoExcludedPath214–244 has no .p-component rule; pre-supplied file universes can bypass walker-hidden behavior, so reserved capsule filtering belongs at Source admission too. Actual search tree inflation from capsules was not measured and is not claimed.

## Canonical Ownership Recommendation

Capsule basename grammar belongs to existing low-level RelativePreservationProtocol (prefix.p,digest8hex). Expose a pure schema-derived reserved capsule name/path classifier under filesystem artifact classification, then use it consistently in Source discovery and compiler input/capture admission. Do not import a mutable retention receipt to decide opacity: an incomplete capsule after failed publication must remain opaque too. Only exact reserved grammar should be excluded; legitimate .prettier/.proto/.py assets must not become blanket ignored Source.

Mirrored Go/native Source filters and emitted Nx/Bazel globs need the same neutral corpus including valid capsule, incomplete capsule, nested paths, near-miss legitimate names and explicit supplied paths. Preserve low-level capsule inspection for its dedicated cleanup owner. No Source edits by audit.

## Independent Review of Implemented Opacity

The shared helper splits both slash conventions and matches only exact lowercase hexadecimal components derived from descriptor and retirement schemas. Source enumeration, owner/config filters, explicit selected-source matching, precision Cargo/source walks, Bazel string and NUL-byte witnesses, graph traversal, selection and doctor freshness now call it. Imported helper identifiers inspected have actual uses. Precision project assembly delegates to the updated cargoManifestDirectoriesV1 implementation, so its unchanged literal skip object is not itself a remaining gap. Neutral tests retain third-party SQLite, Ajv, TypeScript extraction and picomatch comparisons, including near misses and exact supplied paths. This audit did not rerun the test suite.

Remaining explicit admission gap: library/🏗️build/📥️inputs/📋️data/🟨️.mjs captureRepositorySourcePathV1 (10), captureSourceInputInventoryV1 (12), and captureCargoLockInventoryV1 (13) still admit reserved capsule components. compactSourceInputs can receive authored exact workspace inputs outside the current project/owner negative-glob roots. Close this at the closed inventory DATA policy, preserving dynamic byte-captured implementation loading; adding an unresolved relative runtime import to that implementation would break its current closed import design. Test exact reserved inventory paths independently of directory traversal.

Native generator review found retirement.$defs.Policy.const in workspace/🙈️source-opacity/🟦️.ts, while the actual JSON exports definitions.Policy.const. Reported directly to native owner for correction. Its new source filename is 81 relative UTF-16 units and 98 at this root; no path issue there. Root helper uses the correct definitions property. Full current Source path-budget and native corpus execution were not rerun by this audit.

### Closed Inventory Follow-up

Reviewed the subsequent fix: read policy contains opaquePathBasenames prefixes .p/.d and digestHex8. createSourceInventoryDataV1 copies and freezes the nested object and prefixes array; later mutation of caller policy cannot alter admission. Canonical relative path admission rejects exact reserved components before serialization; inventory and Cargo-lock paths both delegate there. The byte-loaded implementation retains only Node system imports, so the prior relative-import concern is avoided. Existing finite root/path/extent and unique roster checks remain present. Reported six-test execution was performed by the implementation owner, not this read-only review.

Final syntax verification parsed 17 current non-test Source modules using opacity/inventory/build-script closure helpers: 0 syntax errors. Temporary detailed file list is under generated/source-opacity-final-syntax.json.
