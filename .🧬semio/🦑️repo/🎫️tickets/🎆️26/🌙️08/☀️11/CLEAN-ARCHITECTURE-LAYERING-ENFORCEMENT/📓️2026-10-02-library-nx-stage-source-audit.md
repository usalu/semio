# Library Nx Stage Source Audit

Read-only source inspection; no Nx/compiler/jobs. Root reports a ten-minute createNodes timeout and High's candidate optimization near20 seconds; neither observation identifies the remaining library stage cause without stage measurements.

Owner base `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Actual repeated work boundaries

`🟨️.mjs:1278` emojiProjectJsonNodes already takes supplied candidates for project metadata, but :1326 unions supplied Cargo candidates with a full `walkCargoToml(workspaceRoot)` every callback. The walk :82–94 recursively enumerates every non-generated directory and excludes symlinks, compose/temp-compose and .🧬semio. This is concrete duplicate tree enumeration relative to Nx candidate collection, not a measured timeout cause. Unlike simply replacing the test plugin scan, eliminating this fallback requires proving Nx finds every intended resident Cargo manifest, preserves exclusions and fresh additions/deletions, and portable emoji path fidelity. Existing comments explicitly motivate the fallback by native glob path drift.

Per-callback project metadata is reread by collectPlaygroundCatalog :1056 and staging selection :1285, then again the project map :1301–1305; playground session/preparation functions loop configFiles :1104/:1138. These are known repeated reads/loops; there is no evidence their cost dominates. projectWithDefaults :1322 shares callback facts/scripts, and native preparation :1343–1345 shares preparationCache/dependencyRootsCache. nativeDependencyRoots :602–632 has shared manifest/cache plus root/test closure keys, although each distinct root traverses its own dependency graph. Preserve dev-dependency inclusion only at selected root.

Bootstrap :39–55 imports runtime closure, source-input, native-host, deployment/identity and browser authority. implementationRevision :1539–1555 rehashes policy/taxonomy/helpers at callback dispatch and reloads on changes. This is callback authority, not a fresh loader per project in the visible loop. Do not blame bootstrap on a per-project hypothesis. The test plugin independently imports library cacheInternals through a source revision; resident revision behavior must remain intact.

Import dependency stage is separate from createNodes: collectImportEdges :1431–1444 uses content-hash disk cache and bounded workers; projectFilesToProcess :1453 uses changed-file map when supplied, else full indexed map. A ten-minute createNodes observation is not evidence that import-edge parsing caused it.

## Actual grant hazard while changing enumeration

Project metadata :1301–1312 and Cargo candidates :1328–1331 use existsSync/readFile/realpath and can follow supplied linked leaf/ancestor paths; full walk refuses links but unioning Nx candidates already bypasses that refusal. An authoritative candidate-only cut needs no-follow raw parent/leaf membership, not only realpath deduplication. Distinguish authored source candidate admission from project-name duplicate detection. Root/root ancestor policy must be explicit for portable native platforms.

## Existing canonical laws

- `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:1167–1170` calls actual plugin callback with exact leaf/root lists and checks duplicate project refusal. This is concrete project metadata callback coverage, not a full manifest candidate/no-follow census.
- Its imported owner `⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts` is the resident implementation freshness law; preserve policy/taxonomy/helper revision behavior.
- Imported `⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts` owns native dependency closure; `🔗️import-edges/🟦️.ts` owns import edge equality and cache behavior. Keep these concerns independent from candidate enumeration.
- `🧪️tests/⚡️production-cache-input-boundary/🟦️.ts:95–106` validates closed native vocabulary and installed Nx pattern behavior against an independent glob oracle, including unknown refusal. It does not assert Cargo candidate discovery completeness.
- `🧪️tests/🧱️root-inference-law-source/🟦️.ts:198–218` covers virtual and actual linked ancestor no-follow source access. Reuse that domain authority rather than invent realpath-only safety.

Small portable candidate corpus needs explicit supplied subset/empty list, noncanonical/lossy emoji path, linked manifest leaf/owner ancestor, reserved generated candidate, added/removed actual manifest, Cargo with and without explicit project metadata, duplicate project identity and valid Windows separators. Retain full project/name/target/dependency census after change. A wall-clock receipt must identify callback stages (walk, nativeCommandInputs, project defaults, preparation, authority reload) before claiming which work fixes the timeout.
