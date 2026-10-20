# Review: x-release and x-ingress (aud-review1)

Read-only adversarial review. No source file was edited, no `bun repo` ran, no test suite was executed (the build bootstrap cannot boot yet). Generated copies live in `🗑️generated/aud-review1/`: `patched-script.ts` (adapter after `mode_patch.py`), `x-release.diff`, `patched-core.ts`, `patched-glue.ts`, `patched-boot.ts` (x-ingress after `apply.py`, rendered in memory). Path abbreviations: `T` = this ticket folder, `BZ` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`, `BUILD` = `…/📚️library/🏗️build`, `GEN` = `…/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation`, `COURSE` = `🎓️teaching/🏛️architecture/📚️course`. Line numbers refer to the live files at review time (about 16:29).

## Verdict summary

| Item | Verdict |
|---|---|
| X-REL-1 `apply.py --check` read-only, anchors match live adapter | OK |
| X-REL-2 design: global axis, mode in id/label/fingerprint, inheritance, EN+DE refusal | ISSUES (R1, R3) |
| X-REL-3 Nx parity and cache keys of existing tasks | OK (for inferred graphs) |
| X-REL-4 fixed-mode pins (e.g. `dev=prototype`) | ISSUES (R2, latent) |
| X-REL-5 live course release gate and its tests | OK (notes N1, N2) |
| X-REL-6 live README mode paragraph | ISSUES (R4) |
| X-ING-1 write-ceiling claim (`maximumWriteBytes`) | OK |
| X-ING-2 live one-token edit, completeness | OK (process note P1, latent cap C1) |
| X-ING-3 staged miss path: stale tree never accepted as ready | OK on the main path; ISSUES X-I1, X-I2 |
| X-ING-4 `apply.py check` read-only and anchors | OK |
| X-ING-5 protocol hygiene (raw unlink, best-effort placement, non-atomic apply) | ISSUES (low) |

## 1. x-release

### X-REL-1 apply.py and anchors: OK
- `staged/apply.py --check` (run as `python3 -I`): prints `adapter: pending; tests to install: 3 of 3`, exit 0. Read-only by construction: `main()` returns right after the print when `--check` is set, before any write (`replace_atomically` and `os.makedirs` come later). `patched()` only raises on a missing or ambiguous anchor, and it did not raise, so all four anchors (`import`, `identity`, `plan`, `environment`) occur exactly once in the live `BZ/📜️script.ts` (taskIdentity at line 81, planner use at line 116, issueBazelTaskEnvironmentV1 at line 474).
- `REPO` resolves to `/Users/ueli/Documents/semio` (checked: nine `..` from `staged/`).
- Writes are atomic per file (temp file plus `os.replace`), and the test suite is copied into `BZ/🧪️tests/📥️mode/`.

### X-REL-2 design: ISSUES
The axis design is right: one invocation axis, the caller's `buildMode` as authority (`📥️caller/🟦️.ts:28`, `:60`, `:68`), the mismatch check in `issueBazelTaskEnvironmentV1`, and dependencies inherit through `inherited` (`📜️script.ts:112`, `:119`). Three gaps:

- R1 (medium): env-only mode does not reach the planner for undeclared targets, so the task id and label lack the mode. `selectBuildTasksV1` (`BUILD/🧭️selection/🟦️.ts:100` for `run`, `:121` for `run-many`) only honours `SEMIO_BUILD_MODE` when the target declares that configuration. Its return (`:126`) has no `configuration` field, so `request.configuration` is always undefined in real runs (the planner's `configuration` at `📜️script.ts:578`, `:584`, `:757` is therefore `undefined` too). For an undeclared target under `SEMIO_BUILD_MODE=release`, the task id is the bare `site:publish` for both modes, the label is `t_<hash(id)>`, and the namespace (`hash(selected,args,configuration)`) is the same, so both modes share one Bazel workspace. The result's claim "task id and label contain the mode" is false for that path, and fixture row `undeclared-default` (`tests/📥️mode/🔣️.json`) enshrines the bare id. Impact is limited to undeclared targets: `targetBuildModes` (`BUILD/🎚️mode/🟨️.mjs`) declares both modes on every inferred target, so no current target takes this path.
  - Fix: derive the mode from the authority in selection (`process.env.SEMIO_BUILD_MODE`, validated with `readBuildMode`) regardless of declaration, or pass `configuration: caller.buildMode` into the request. Add a fixture row for undeclared target plus env-only release, and assert the id contains `:release`.
- R2 (medium, latent): fixed modes are enforced only at invocation. `fixedTargetBuildMode` (`🎚️mode/🟨️.mjs:5-9`) is consulted only in `resolveBuildModeV1` (`BUILD/🧭️invocation/🟦️.ts:119-125`). The adapter never reads `metadata.semio.buildMode.fixed` (grep finds no reader in `BZ` or the graph module). A release root that depends on a prototype-pinned target inherits `release` (`📜️script.ts:116-119`), and because the dependency declares `release`, it silently runs as `dev:release`. The new mismatch refusal does not fire, because the dependency's configuration equals the caller's. No current manifest pins a mode in `dependsOn` (JSON grep over the repo: none), so this is latent, but it contradicts the brief's "legitimately pin a fixed mode" requirement.
  - Fix: in `include` (or `taskIdentity`), refuse with EN+DE text when a dependency's fixed mode differs from the inherited mode, or pin it to its fixed mode with an explicit rule. Decide and test; do not leave it implicit.
- R3 (low-medium): the mismatch refusal runs at fingerprint and run time (`issueBazelTaskEnvironmentV1` is called from `taskFingerprint`, `📜️script.ts:484-500`, and from runs at `:685`), not at plan time. The planner has no caller authority, so a bad plan is discovered only after fingerprinting other tasks has started.
  - Fix: pass the caller's `buildMode` into `planBazelTasksV1` (a request field `authority`), and check in `include`, so refusals happen before any action is fingerprinted.

### X-REL-3 Nx parity and cache keys: OK (inferred graphs)
- For declared configurations, `bazelTargetV1` returns `resolveBuildTargetV1` unchanged, `taskIdentity` admits the same names (`Object.hasOwn` versus the old truthy lookup gives identical results for declared names and fixes `:constructor`-style names), and labels, ids and fingerprint are unchanged. Existing inferred tasks keep their keys.
- The only new runtime behaviour is the mismatch refusal. I grepped the repo's JSON manifests for `dependsOn`-style mode pins (`:prototype`, `:release`, `:dev`, `:staging`) and found none in a dependency position, so it does not fire on today's graphs.
- Nx is untouched; the shared invocation change is the message text only.

### X-REL-4 fixed-mode pins: ISSUES
See R2 above. Note also that `dev` depends on `assets:build` and `ui-styling-tokens:generate` without suffixes (course manifest), so the course graph itself is unaffected.

### X-REL-5 live course gate: OK
- `COURSE/🏗️builder/🎚️mode/🟦️.ts:18-24`: closed schema check first, refusal only for `intent=release` with a non-release original mode, `dev` accepts both. Messages are EN then DE, two lines, emoji docstrings unique in the file, no comments inside definitions.
- The gate call sites (site, docker-image, deploy, e2e, readiness, stack) already existed at HEAD. The live change is bilingual messages and the schema-backed request, not new gating.
- Tests (`COURSE/🧪️tests/🎚️mode/🟦️.ts`) cover all four fixture rows, the oracle, EN/DE structure and both build tools with real issued callers. They use `process.cwd()` as the workspace root, so they must run from the repo root (`check.sh` does).
- N1 (note): not proven live. The gate reads `readNativeCallerAuthorityV1(...).original.buildMode` inside a Bazel action, and no real `publish:release` run has happened (coordinator step 5).
- N2 (note): `resolveBuildModeV1` still refuses any configuration other than prototype or release (see R5), so the adapter's custom-configuration support is unreachable from the CLI.

### X-REL-6 README: ISSUES
- R4 (medium, doc accuracy): `🧰️framework/🛍️products/🦑️repo/README.md` line 164 (EN) and line 515 (DE) state as fact that a task of another mode "is refused" and that both tools accept prototype and release for every target. Both hold only after the staged patch is applied; the Nx path was not examined (result, open points). Either apply the patch first or make the paragraph conditional. The result's own note says the same, but the live README does not.

### Further notes on x-release
- R5 (low, pre-existing, claim mismatch): the result says declared custom configurations "work as before". At the CLI they do not: `BUILD/🧭️invocation/🟦️.ts:115-116` pushes the `:suffix` and `--configuration=…` values, and `:122` runs `readBuildMode` on each, refusing `staging`. Only the adapter path accepts custom names, so fixture row `declared-custom-staging` is adapter-only. Either admit declared configurations at the CLI or state the limitation.
- R6 (low, EN-only text): `🏗️build/🎚️mode/🟦️.ts:15` ("Build mode must be prototype or release") and `BUILD/🧭️invocation/🟦️.ts:111` ("Explicit compiler configuration value required") are English-only, against the EN+DE rule. Pre-existing, but the new flow now reaches them for `--configuration=staging`.
- Docstring emoji: the new `🎯️` in `bazelTargetV1` repeats an emoji already used in the adapter (pre-existing pattern; low).

## 2. x-ingress

### X-ING-1 write-ceiling claim: OK
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json:463` sets `RelativeCreateProtocol.maximumWriteBytes = 65536`. It is enforced in `…/📁️descriptor/🟦️.ts:107` (`captureArtifactRelativeWriteBytesV1`: `length>p.maximumWriteBytes` throws "Bounded closed relative write bytes required"), so the claim is correct.
- The same protocol has `maximumFileBytes = 67108864` (`:464`), enforced per write in `artifactRelativeWriteCapabilityV1` (`:109`), and `"failure": "retain-created-leaf-no-rollback-delete"`. The retained leaf is exactly the 0-byte `*.frontier.ndjson` that the result describes.

### X-ING-2 live one-token edit: OK, with notes
- `GEN/💾️storage/🟦️.ts:144` now reads `Math.min(65536,…)`. Every other chunked writer in the bootstrap and dependency storage uses 65536: `install.json` declares `chunkBytes` 65536 in all four places it is defined, and `BUILD/🚀️bootstrap/🟦️.ts:82` uses 65536. So no other writer exceeds the ceiling, and the edit is complete.
- Live evidence: `.cache/b/d/m/generation/` now holds sealed `*.receipt.json` files (16:25 and 16:29), 20 MB frontier files, and no 0-byte frontier files. The fix works in a real run.
- Observation: that directory holds three receipts for one scope (`cdb4ee2aea69cc6a`, mtimes 16:25, 16:29, 16:29). Superseded-receipt retirement (`retireJavascriptDependencyGenerationFilesV1`, grace 0 on the live path) is not keeping up. Not analysed further; check before relying on the store size.
- C1 (latent, low): `GEN/🧬️schema/🔣️.json:16` sets `maximumEvidenceBytes` to 134217728 (128 MiB), but the relative-create `maximumFileBytes` is 67108864 (64 MiB). Evidence between 64 and 128 MiB fails mid-write and never seals, the same symptom as before. Reconcile the two constants (lower evidence to 64 MiB, or raise the file cap in the descriptor protocol).
- P1 (process): the edit sits in the storage layer that the brief's Peer Freeze lists ("the storage/native-image layers"). It is recorded under "Backend edits", but the coordinator should acknowledge it explicitly.
- The staged span replacement in `glue` (`apply.py` edit "glue write files") covers the line-144 writer, so applying the staged set does not conflict with the live token.

### X-ING-3 staged miss path: stale tree never accepted as ready: OK on the main path
Why a stale tree cannot be accepted:
- Shared readiness requires `confirmGenerationWalkV1` to return a sha equal to the owner's sha (`patched-boot.ts:81`, the throw "Current parser Source-owner custody changed"). Replay returns `match` only when `proof.sha256===walk.sha256` (`glue.insert.ts`, replay tail). Verify's sha is `sha256(receipts, required observed, execution scopes)` and must equal `scopeReadSha256`, so `match` means the live tree reproduces the walk's digest.
- Any doubt in replay (deferred, mismatch, seal) falls back to one complete fresh walk. If the tree changed, that walk's sha differs from the owner's and the command throws, which fails closed rather than accepting.
- Receipts are persisted only after capture is re-verified against the live tree (`writeJavascriptDependencyGenerationReceiptV1`, `proof.sha256!==walk.sha256` yields "stale", live `GEN/💾️storage/🟦️.ts:129-140`). Receipt JSON is written last, and evidence names are content hashes (`evidenceOf`, `sha256+suffix`), so a partial write cannot be mistaken for a sealed receipt.
- The writer lease is non-blocking (`timeoutMs:0`), so there is no deadlock with shared custody. The regex `^Resource lease timed out` matches the library's message (`🔒️leases/🟦️.ts`).

Issues:
- X-I1 (medium): replay has no failure containment. `replayJavascriptDependencyGenerationWalkV1` (`glue.insert.ts:18-25`) calls `captureJavascriptDependencyGenerationReceiptV1` → `captureGenerationReceiptV1` without try/catch. That function throws on "Closed dependency generation receipt required" (schema), "Generation evidence capacity exceeded" (`evidenceOf`: 524288 rows or 128 MiB) and "Bounded unique implementation closure required". Nothing in `confirmGenerationWalkV1` (`core.append.ts`) or the bootstrap at `patched-boot.ts:64`/`:81` catches it, so one such error fails `ensureBuildEnvironmentV1` and therefore every `repo` command on that workspace. The live code swallows the same error on the publish path.
  - Fix: wrap the replay body in try/catch; rethrow only when `control.signal.aborted`; otherwise return `{status:"deferred", reason:"capture-failed"}` so the walk fallback decides.
- X-I2 (medium-low): the exclusive pass replays the shared walk with the wrong start time. `patched-boot.ts:64` passes `walkStartedAt`, which is the exclusive pass's own start. When `walk` is `cached.walk` (candidate set at live `:80`, startedAt from the shared pass), the walk began earlier. The settle rule (5 s before walk start) then protects a later window than the one the walk covered, so on a coarse-timestamp filesystem a same-second modification during the shared walk could be accepted. On APFS with nanosecond stamps the stamp comparison still catches most cases, so this is about the settle guarantee, not a hole on this machine.
  - Fix: use `cached?.startedAt ?? walkStartedAt` in `confirmWalk`, or store the start time with the walk object.
- X-I3 (low-medium, protocol): the raw `unlinkSync` of a partially created leaf (`glue.insert.ts:11`) bypasses the descriptor API and goes against the protocol's documented `retain-created-leaf-no-rollback-delete` and parent-quiescence rules. It is also needed: without it, the next attempt hits "Existing immutable generation file changed" forever (content-addressed name, size mismatch). The live retire path already uses raw `unlinkSync`, so this has a precedent. Preferred: a descriptor-relative remove for owned leaves, or an explicit protocol amendment that allows removing a leaf created by the same call. Record the decision.
- X-I4 (low): `publishGenerationBestEffortV1` (`patched-boot.ts:82`) runs capture, verify and write synchronously before the shared pass returns "ready", on the critical path of every miss (on a 375k-operation walk this is seconds to minutes). It cannot affect correctness, but it delays the command. Consider returning readiness first and publishing afterwards in the same custody, or accept and note it.
- X-I5 (low): C1 above is the trigger for X-I3's partial-file path, producing a permanent "failed" on large evidence.
- X-I6 (low, process): `apply.py` writes file by file, so a refusal can leave the set half applied. The order CORE, then GLUE, then BOOT is safe (each step is additive, and BOOT is applied last so it never calls an absent export), but a two-phase apply (compute all edits in memory, then write) makes refusals leave the tree untouched. Recommended.
- X-I7 (test gaps): `ingress.fixture.json` `confirm` rows and `ingress.oracle.py` restate the same rule table, so they are not an independent oracle. The real race (package.json edited between walk and replay, replay must not match, shared pass must throw) is covered only by the mirror laws. Add tests for: replay that throws (X-I1), the exclusive pass using the shared walk's start time (X-I2), and an integration law that the shared pass returns "ready" only with a confirmed sha. No test covers the Nx path through `ensureBuildEnvironmentV1`, which is shared, so the miss-path change also changes Nx behaviour without coverage.

### X-ING-4 apply.py check: OK
- `apply.py check` (run as `python3 -I`): `24 operations, 24 pending (dry run)`, exit 1 by design. Every anchor was found exactly once (no `SystemExit`). `run(root, dry=True)` guards all writes with `if not dry`. `DEFAULT_ROOT` resolves to the repo root (eight `..` from `x-ingress-tools/`, checked).
- Idempotency: each edit's `marker` is the new text, so a re-run reports `present`. Verified by reading `patch_text`.

## Recommended changes before apply

x-release (before `apply.py`):
1. Fix R1: derive the mode for undeclared targets in `BUILD/🧭️selection/🟦️.ts` (`:100`, `:121`) from the authority, and add the undeclared-plus-env-release fixture row asserting `:release` in the id.
2. Fix R3: carry the caller's `buildMode` in the planner request and refuse in `include`, so no action is fingerprinted before a refusal.
3. Decide R2 (fixed-mode dependencies) and test it; the current behaviour is silent.
4. Fix R4 or apply first: make the two README paragraphs conditional on the patch.
5. Address R5/R6 (declare the CLI limitation or admit custom configurations; EN+DE for the two English-only refusals).

x-ingress (before `apply.py apply`):
1. Fix X-I1: contain replay failures and fall back to the walk.
2. Fix X-I2: use the shared walk's start time in `confirmWalk`.
3. Reconcile C1 (evidence cap versus file cap).
4. Decide X-I3 (leaf removal) and record it in the protocol decision.
5. Make `apply.py` two-phase (X-I6).
6. Add the tests in X-I7.
7. Coordinator: acknowledge the live edit in the storage layer (P1). The live edit itself is correct and complete.

No `bun repo` was run and no test suite was executed in this review. Verification of the staged tests remains with the executors' own logs (`T/🗑️generated/x-release/`, `x-ingress` mirror logs).
