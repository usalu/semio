# E22 findings: what the Nx tool namespace is keyed on and where a single-target run spends its admission

## 1. Identity of the tooling generation (exact)
`nxToolingInstallIdentityV1(environment, files)` (`C/🚀️bootstrap/🛠️tools/📦️sdk/🪪️identity/🟦️.ts`) hashes `nx-tooling-install-v4`, platform/arch/libc and every named input in order. The inputs are built in `captureNxToolingInputsV1` (`🛠️tools/🟦️.ts` l.147) and `provisionNxToolsOwnedV1`:
- `package.json`, `bun.lock` and the patch files of the Nx tooling recipe (Nx version, plugins, `semio.toolPatches`): stable, change only on a deliberate Nx edit;
- `.semio-sdk-material.json` (pristine snapshot digest + sealed source index digest): stable;
- **`.semio-authorizer-source.json`**: the manifest (path + sha256 of every file) of the transitive source closure of `🔏️authorizers/📦️runtime/🟦️.ts`: 1,823 files on 2026-11-22 (build 528, caching 378, native-build storage 113, schema 48, packages 39 ...);
- **`.semio-authorizer-compiler.json`**: the same for the compiler program `🔏️authorizers/📜️script.ts`: 1,985 files, plus `physicalSourceRoot`.
So the generation `nx/<digest>` (the "Nx tool-derived namespace": hardlink farm of the pristine SDK + owned patched `nx` package + `semio-authorizers.mjs`) is keyed by the whole library closure, not by Nx. Any edit of any of ~2,000 library files changes the digest. Measured churn: 21 generations of 199 MB (hardlinked) between 01:06 and 03:28 (one per about 8 min); both of my dry runs (03:50 and 03:56) rebuilt the generation because a peer landed in between.
Why the closure is that big: the authorizer entry imports `...typescript/🏗️build/🕸️graph` (1,530 files reachable), the graph startup/service (987), `🛠️tools/📥️use` (925), the Nx storage enforcement (581) ... (import census from the manifest, scratch script `closure.py`). The bundle `semio-authorizers.mjs` (8.5 MB) therefore really does contain the code of the build library, so the digest is honest: the bundle changes whenever the library changes.
Narrowing to Nx version/plugins/config is NOT possible without moving the authorizer bundle out of the generation: the bundle's own location is its identity (`withNxLoadedToolingUseV1`: `toolkitRoot = dirname(import.meta.url)`, `identity = basename(toolkitRoot)` must equal the receipt digest; the use-witness, retention, the `nx-tooling:<digest>` lease and `verifyNxSdkDerivativeNamespaceReadV1` all hang on that directory). Recommended design (not done, wide): two content addresses, `nx/<depsDigest>` (deps, patches, SDK material, binaries: stable for weeks) and `authorizers/<sourceDigest>` (bundle + manifests + receipt, rebuilt per landing in about 7 s: bun compile of the sealed snapshot), with the toolkit entry resolving `node_modules` through the deps generation. Owner: tools/retention (E10/E13 area), touches `🛠️tools/🟦️.ts`, `📥️use`, `🔏️authorizers/📤️receipt`, `📥️loading`, `🧹️retention/📥️physical`, the generation schema and ~15 fixtures.

## 2. Measured anatomy of one single-target dry run (`repo run repo:test-cache-reset --dry-run --exclude-dependencies`, load 25-35, 04:00)
Run 1 (generation rebuilt), total 416 s, seconds from start (script `T/🧰️scripts/E22/phases.py`):
| t | span | what |
|---|---|---|
| 0-39 | 39 | graph slice read (ledger wait 4, inventory 18, taken 4) |
| 39-69 | 31 | capacity gate: the run asks 11,176 MB ("at least 11176 MB because of its plan phase") for a dry run of one target, admitted with 5,596 MB after 31 s |
| 69-96 | 27 | source admission, implementation source read (10), dependencies generation verify (106k entries, 2.6 s x2), lease sweep |
| 96-106 | 10 | nx storage planning |
| 106-116 | 9.5 | `nx:source-index`: sha256 of all 11.6k pristine SDK files |
| 117-123 | 6 | derivative recheck |
| 123-172 | 44 | `nx:sdk-derivative`: hardlink clone of 11.7k entries |
| 172-183 | 10 | recheck |
| 183-189 | 7 | authorizer compile |
| 189-209, 220-235, 272-284, 295-315 | 19, 15, 12, 20 | `nx:source-verification` (sha256 of all 11.6k pristine files) x4 (installed(staging), installed(destination), activate, task child) |
| 209-220, 235-246, 284-293, 315-323 | 9, 11, 9, 8 | derived-namespace verification x4 |
| 246-272 | 26 | recheck |
| 323-416 | 93 | the Nx run itself (daemon graph, 3 MB graph published to the fleet) |
So Nx admission 96-323 = 227 s. A warm machine (generation present) still pays 3 readers x (source-verification + namespace) = about 80 s.
Micro-benchmark on the real pristine base at load 35 (`scratchpad/E22/bench.mjs`): sha256 of all files with the production descriptor pattern (open, stat, read, stat, close) 15.3 s; ancestors re-lstat per entry 3.2 s; lstat of each entry 1.6 s; root guard 0.4 s. Namespace walk raw fs cost: opendir of 1,478 dirs 0.5 s, lstat 0.7 s. So the hash is about 75 percent of every source-verification pass; the rest of each pass is checkpoint overhead.

## 3. For other lanes
- Capacity gate (host-capacity): 31 s wait for 11,176 MB for a one-target dry run; the ask comes from the "plan phase" class estimate and is out of proportion for `--exclude-dependencies --dry-run`.
- The graph service key file `semio-repo/501/key` vanished at 03:38 (directory mtime), after which every `repo` command failed with "The graph service could not answer (key)" for 10 minutes (the running service never recreates the key; the client retries only while the file is missing and only a short time). I ran `repo service host stop` then `start` (a service of this host started by a repo command, drained, nothing running) to recreate it. Cause of the deletion not found (nothing in `🏡️resident` removes the key); candidates are a test or sweep that wiped the runtime directory. The service should recreate a missing key on a client refusal or on a periodic check.
