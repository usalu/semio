# Stranded storage image (2026-11-22)

## Incident

The `dependencies` image `.cache/b/d/fixed.dmg` was attached at the private scratch mount point `semio-reclaim-*` of a reclaim cycle instead of `.cache/b/d/m`. Every `node_modules` link into `.cache/b/d/m/...` dangled. `repo dependencies ensure` could not start (bootstrap cycle), the loader reported the resolution failure as "edited by another process" (exit 75, 6 restarts) and `repo doctor` said "any repo command attaches it".

## Root causes found

1. **Cleanup ran on the cancellable runner.** Both callers of `reclaimNativeImageV1` (`reclaimImageIfQuiescentV1` and `reclaimRepositoryRustImageWithWaitV1` in `.../📥️opening/♻️reset/🟦️.ts`) passed a single `run` that carries the lease `control` (and, in the wait variant, `check()` before every command). The cleanup (`hdiutil info` and `hdiutil detach`) used the same runner, so a cancelled or lease-lost cycle failed its own cleanup and left the image at the scratch mount. No `cleanupRun` was supplied although the Darwin port supports one.
2. **No recovery on the next opening.** `inspectPersistentReceiver` only accepted an image mounted at its canonical root; an image attached elsewhere ended in "Exact bounded persistent Native mounted association required" and nothing detached it.
3. **Late attach race.** An attach that fails or ends by its deadline can still finish in its child process after the cleanup looked for the device.
4. **Churn classification.** `isHalfEditedModuleLoadV1` treats every `ResolveMessage` as a half-edited module, including "Cannot find package" behind a dangling `node_modules`.
5. **Bootstrap cycle.** The static import closure of `repo dependencies` (`dependencies` command, `📥️entry`, graph inference, `📍️coordinates`, plugin rust model contract, plugin model contract, `🌱️actor-preparation`) imported the bare specifier `@semio-tech/schema-validator-ts`, which only resolves through `node_modules`, the very link the command has to repair. Everything else in the closure of the front door and of the dependency command is relative-only.
6. **Doctor.** The storage probe knew only mounted at canonical / not mounted; it never asked the kernel where an image is attached.

## Changes

1. Reclaim and opening
   - `🍎️image/🏚️stranded/🟦️.ts` (new): `nativeImageStrandedMountV1` (pure: `hdiutil info` image path to entities to mount points, device = whole disk entry) and `recoverNativeImageStrandedV1` (detach without force, bilingual refusal `nativeBuild.image-stranded` naming image, mount points, device and canonical mount point when busy). Facade `📦️packages/🟦️typescript/.../🏚️stranded/🟦️.ts` and package export added. Text added to `🔨️modules/🏃️process/📖️texts/🔣️.json`.
   - `🧹️cleanup/🎛️command/🟦️.ts` (`inspectPersistentReceiver`): an image of this exact path attached at a foreign mount point is detached and the opening continues as "inactive" (so it is attached at its canonical root); not applied for terminal retirement reads. It runs under the shared custody that excludes any reclaim cycle, which holds the exclusive lease.
   - `🍎️image/♻️reclaim/🟦️.ts`: the cleanup detach is forced when the plain detach fails (the scratch mount is private to the cycle); after a failed attach the macOS port polls up to the declared settle polls for a late device before it reports none (injectable `sleep`).
   - `♻️reset/🟦️.ts`: both callers now pass a `cleanupRun` without `control` and without `check()`, so cancellation and lease loss cannot stop the cleanup.
2. Doctor
   - New facts: partition state `misplaced` with `attachedAt` and `device`, storage `loaderBlocked` (node_modules is a dangling link). Schema, types, probe (`hdiutil info -plist` on macOS through the `command` port, failures read as unknown), analysis and the Python oracle updated.
   - New rules `storage.image-misplaced` (error, fix: `hdiutil detach {device} && hdiutil attach -nobrowse -owners on -mountpoint {directory}/m {directory}/fixed.dmg`) and `storage.image-detached-blocking` (warn, replaces the "any repo command attaches it" claim while node_modules dangles, with the manual attach command). `storage.image-detached` (info) is unchanged for a healthy loader. Texts en and de.
3. Loader churn
   - `🧷️format-stamps/🟨️.mjs`: `danglingDependencyStoreOfV1` (message names a bare package; the first `node_modules` or package link above the importer is a link whose target is missing; injectable fs) and `DanglingDependencyStoreErrorV1`. `staleImplementationOfV1` no longer classifies such a failure as `load`; `exitIfStaleImplementationV1` ends with its own exit code (`danglingStore.exitCode` = 1, not 75) and a localized message naming package, dangling link and store path; `runWithStaleReexecV1` raises the same error without sleeping or relaunching. Schema protocol entry and en/de text `danglingstore` added; typings updated.
4. Bootstrap cycle
   - `🌱️actor-preparation/🟦️.ts` imports the schema validator by relative path (same file the package link resolves to).
   - Guard test: the static import closure of the front door and of the dependency command (relative and literal lazy imports followed) contains no package that needs `node_modules`.

## Tests

| File | What |
|---|---|
| `🍎️image/🏚️stranded/🧪️tests/🟦️.ts` (+ fixture json + `🐍️.py`) | seven plist cases, equal to the independent `plistlib` oracle; recovery detaches only the stranded whole disk; busy refusal bilingual with all placeholders |
| `🍎️image/♻️reclaim/🧪️tests/🟦️.ts` | two new tests: forced cleanup detach and bounded late-device polling; uncancelled cleanup runner after cancellation of the work runner |
| `♻️reset/🧪️tests/⌛️wait/🟦️.ts` | the Rust wait route passes a `cleanupRun`, and every cleanup command runs without the cycle control |
| `🧷️format-stamps/🧪️tests/🏚️dangling-store/🟦️.ts` (+ fixture json + `🐍️.py`) | package extraction equals the oracle, real junction/symlink trees (node_modules link, package link, healthy tree), no `load` classification, exit code and en/de message, no restart, both-language placeholders |
| `⌨️command/📦️dependencies/🧪️tests/🔄️bootstrap-closure/🟦️.ts` | closure guard plus a self-check that a bare static import is reported with its chain and a lazy one is not |
| `🩺️doctor/🧪️tests/🔎️probes/🟦️.ts` and fixture json | misplaced probe and finding with exact fix, canonical mount stays info, dangling node_modules gives the warn rule without the false claim, linux and failing hdiutil; two hand-stated fixture cases (every catalogued rule must be used) |

No real `hdiutil` is used by any new test; the macOS-only integration test already in the reclaim file is unchanged.

## Not covered

- The wiring inside `inspectPersistentReceiver` is a one-line call into the tested module; the surrounding opening flow needs real kernel images and the repo test harness and was not run here.
- The `♻️reset/🧪️tests/🟦️.ts` and `⚡️recovery` suites fail under plain `bun test` with "Build test receiver admission closed" (they need the `repo test` harness) and were not touched.
