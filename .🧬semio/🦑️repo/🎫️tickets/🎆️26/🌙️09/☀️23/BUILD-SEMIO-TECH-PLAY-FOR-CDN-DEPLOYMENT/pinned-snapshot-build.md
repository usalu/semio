# Pinned Snapshot Build for the Play Release (Research, Read-Only)

## Verdict

No existing switch makes the live-checkout release driver read one admitted revision. The only existing mechanism that gives a build a frozen source tree is a disposable private workspace: `repo incremental prepare --root=<short-root>` (APFS `COPYFILE_FICLONE` copy of the tracked and non-ignored roster, no `.git`, own `.cache`). Point `native-play.sh` at that root with `SEMIO_WORKSPACE_ROOT` and the build can no longer see peer edits; `concurrentEditV1` never fires from peers because nothing under the private root changes unless we refresh it.

## 1. "sealed" is not a source snapshot

- `SEMIO_REPO_SEAL=sealed|live|promote` and `.cache/b/r/m/u/sealed/<gen>/` (README: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/README.md`) seal the repo tool's own implementation (front-door closure, `.🧬semio/🦑️repo/⚡️cache/implementation/<entry>/`). `fleet status` `sealed.entries` lists those implementation entries (`🚪️entry/🟦️.ts` plus verb kernels). It never captures workspace crates.
- `graph_policy=current` pins only the graph generation that nested tasks use. It does not pin Rust source bytes.
- Revision machinery exists (`📚️library/🏗️build/🧊️revision/📌️pin` `pinRevisionV1`/`withPinnedRevisionV1`, `🗂️fs` `createPinnedFsV1`, `🔓️view` `materializeRevisionV1`) but is wired only to the host planner and Bazel capture. Cargo/rustc/build scripts need a physical tree; the Play driver has no input for a revision (ticket `release-source-snapshot.md`, "Release integration"). grep found no `SEMIO_SOURCE_*` or `SEMIO_SEALED*` variable in `🏗️native-build/`.
- The revision reader (`📥️inputs/🔏️revision`) requires nlink === 1 for live sources: never hardlink (`ln`, `cp -l`). Clonefile is fine (verified in scratchpad: `cp -c` gives distinct inodes, nlink 1 on both; only `ln` raised the source to 2). The older memory note blaming `cp -c` is wrong on this point; hardlinks are the culprit.

## 2. Retained admitted sources (wasm-retained-source.md) cannot be the answer

`RetainedCargoSourcesV1` (`🏗️native-build/📥️source/📎️retained/🟦️.ts`, shared helpers `captureAdmittedCargoSourcesV1` / `deferCargoSourceDriftV1`, used by descriptor path `native-build/🟦️.ts` and `🕸️wasm-build`) is admit, defer, prove: the compile still reads the live tree and the result is accepted only if rustc's dep-info `# checksum:blake3=` equals the admitted bytes. An edit that lands before a unit consumes the file still refuses (`concurrentEditV1(..., "build")`, retry). Extending it to all units only converts some late-edit restarts into acceptances; it cannot make a unit read old bytes, and dep-info checksums are unverified for the native lane. The ticket note itself says elimination needs a private source view.

## 3. concurrentEditV1

Defined in `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts` (`concurrentEditV1(reason,path,detail,from)`, `retryOnConcurrentEditV1(profile,...)`, profiles `read|snapshot|build` with `maximumAttempts`/`backoffMs` in `🧬️schema`). Users: `🏗️native-build/🟦️.ts`, `🧾️receipt/📎️source-witness`, `🧩️component-pass` (+ `🔁️failure-churn`), `📥️source/🎟️admission/📖️read`, `🗃️stage-cache`. There is no "keep snapshot" mode: restart is the protocol, and the options (`converge`, `modifiedAgeMs`) only tune waiting. Raising attempts does not converge under constant peer churn. Only removing the cause (private root) works.

## 4. Recommendation: private clone root, launcher pointed at it

Existing pieces: `incrementalCloneV1`, `incrementalRefreshV1`, `incrementalDisposableV1` (refuses a root inside/containing the live checkout), `incrementalChildEnvironmentV1` (drops inherited `SEMIO_*`/`NX_*` except `SEMIO_LANGUAGE`, disables the Bun transpiler cache) in `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏃️runner/🟦️.ts`; command in `📏️incremental/⌨️command/🟦️.ts`. Prior analysis: `📓️2026-11-21-private-play-release-executor.md` (same ticket folder).

Steps (P = private root, e.g. `$HOME/.sinc/play`, default length policy keeps it at most 28 UTF-16 units):

1. Free space: live volume showed 63 GiB free (93 percent used), floor 15 GiB. The clone is near-zero until written, but the private `.cache` (tools, dependencies, release Cargo build dir) is cold and large.
2. `cd /Users/ueli/Documents/semio && repo incremental prepare --root=$HOME/.sinc/play --fresh --lang=en`. It clones the roster (169,052 paths, 26 ignored generated roots), then runs `preflight`, `tools setup all`, `dependencies sync --all`, `tools setup all` inside P with its own CLI. Wait for a quiet moment of foundational edits; each file is read once, so the result is per-file consistent but not one atomic revision. If the first build fails on a half-edited crate, run `incrementalRefreshV1` (prepare without `--fresh`) once, deliberately, then rebuild. The root only moves when we refresh.
3. Copy the ticket launcher inputs (byte copy) to the same relative ticket path inside P (the roster excludes tickets): `native-play.sh`, `native-launch.ts`, `native-build-launch.ts`, `native-unit-cache-policy.ts`, `native-generators.py`, `native-source-boundary-preflight.ts`, `📜️script.ts`, the publication audit. Use a fresh `🗑️generated/native`; do not copy `done/` markers.
4. One edit to `native-play.sh` (the only launch change): replace
   `ROOT="$(git -C "$T" rev-parse --show-toplevel)" || exit 1`
   with
   `ROOT="${SEMIO_WORKSPACE_ROOT:-$(git -C "$T" rev-parse --show-toplevel)}" || exit 1`
   (the clone has no `.git`). The script already exports `SEMIO_WORKSPACE_ROOT="$ROOT"`, `SEMIO_TICKET_DIR`, `SEMIO_BUILD_MODE=release`.
5. Launch from inside P with a scrubbed environment:
   `cd $P && env -u REPO_ROOT -u SEMIO_MACHINE_STATE_DIRECTORY -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR -u NX_WORKSPACE_ROOT SEMIO_WORKSPACE_ROOT=$P SEMIO_BUILD_SERVICE=off BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_REPO_SEAL=live SEMIO_BUILD_MODE=release bash "$P/<ticket-rel>/native-play.sh" site` (then `verify`). Also unset any inherited `SEMIO_CALLER_*`/build-service token. Launcher root order: `SEMIO_WORKSPACE_ROOT`, `REPO_ROOT`, cwd.
6. Verify isolation before building: `find $P/.cache -maxdepth 2 -type l -exec readlink {} +` shows no path into the live root; after the build grep dep-info under `$P/.cache` for the live root string (must be empty) and the live `.cache` cargo build dir must have no dep-info naming `$P` (poison check).

Hazards: (a) never hardlink; clonefile is OK (nlink 1); post-check `find $P -type f -links +1` empty. (b) Private root needs its own cargo build/target dir (it gets one via its own `.cache`; never set the live `SEMIO_MACHINE_STATE_DIRECTORY`/`CARGO_BUILD_BUILD_DIR` or the live build dir gets poisoned with clone paths, and the product partition/leases would collide with the supervisor still running in the canonical workspace). (c) Path budget: every file path at most 255 UTF-16 units incl. `$P` prefix and `.cache/b/...` outputs; longest tracked path is 210, so keep P at most 28 units (observed 210 + 28 = 238; check expanded ignored roots and symlink-resolved output paths). (d) Cold build: expect a full cold release compile of the closure in P (no warm units shared); the current live release attempt must reach a terminal state first or be accepted as concurrent load. (e) Published artefacts: the verified cohort lands at `$P/.cache/b/p/m/play/release/pages/{play,map,media,modules}`; export with a ticket exporter using `withPublishedPlayPagesV1` (`🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/🟦️.ts`) so the canonical checkout is not mutated. (f) Remove P (`incrementalDetachMountsV1` first: cache images mounted below it) when done.

## Not done / unverified

No clone, build or process was started. The no-`.git` ROOT fallback and the whole sequence are unexecuted; whether `capacity run`/lease state is machine-wide and collides with the live attempt is unverified. A real fix (producer-issued pinned revision materialised as the driver's workspace) is the design sketched in `release-source-snapshot.md`.
