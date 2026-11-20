# Churn retry that never converges ("attempt 63 of ∞")

## Where it lives
- Message: `retry` text of `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json`, rendered by `plan()` in `…/🔁️churn/🟦️.ts`. "∞" came from `converge:true` on the `build` layer: bounded only by `convergence.wallClockMilliseconds` (2 h, about 240 attempts at the last 30 s pause).
- Converging callers: `native-build/🟦️.ts` (the plan/produce loop of `withCargoArtifactPlanLedgeredV1`) and `🖨️describe/🏭️fresh-component/🟦️.ts`.

## What the retry compares against
The retry loop itself holds no baseline: it only re-runs `body`. Each retry's "new snapshot" is compared in the body against facts that are not refreshed by the loop. The raising sites with reason `bytes` / `input` that outlive an attempt are: the shared file custody (`CurrentPhysicalFilesV1` claims, `📁️current/🟦️.ts` `commit`/`recheck`, "Current physical bytes changed"), the completed-Source digest check against the receipt (`native-build/🟦️.ts`, "Completed Cargo Source differs from admitted artifact plan") and baselines owned by an enclosing scope (the repository Cargo configuration / workspace-selection view wraps the converge loop). Without a live process (the looping child was gone) I could not pin which one held the stale Cargo.lock / `context/🦀️.rs` expectation; all of them share the property that the owner is outside the converge loop, so re-running the body inside it cannot refresh it.

## Fix (generic, in the churn policy)
1. A converging layer now detects non-convergence: when consecutive new snapshots report the same `reason`+`path` and the path's identity (dev, ino, size, mtime, ctime) has not moved between the reports, `convergence.stableFailures` (2) times, it stops at once. It raises the escaped `ConcurrentEditErrorV1` (rank of the build layer, new bilingual schema text `stuck`), which the restart handler (`exitIfStaleImplementationV1`) already turns into exit 75 after the declared pause; the driver restarts the job with fresh inputs and a new process (new baseline). A file that is still being edited (identity moves) keeps converging as before; a path that cannot be stat-ed never counts as stuck.
2. "∞" is gone: `convergence.maximumAttempts` (120) joins the wall clock as a finite schema bound; retry events carry that maximum and the exhausted message names it.
3. Schema: `convergence.{maximumAttempts,stableFailures}`, `texts.{en,de}.stuck`. Option `identityOf` for tests.

## Tests
`…/🔁️churn/🧪️tests/🟦️.ts`: new test "a converging layer ends once every new snapshot reports the same edit of a path that no longer moves…" (once-changed-then-stable file against a pinned expectation yields after 3 runs, a genuine converge, a moving file and the attempt bound); the existing converge test and the text-parity test were updated. `bun test ./🧪️tests/🟦️.ts` in the churn folder: 16 pass. Races, layers, busy and job-wait tests pass. `📸️snapshot` (3) and `🧾️refusals` (2) tests fail identically on HEAD (pre-existing, verified on a scratch copy at HEAD, removed).

## Not done
- No build was started; the real native-transpile loop was not re-run. Root owner of the stale baseline is still worth fixing at its site (a baseline must be re-established by the layer that retries it).
