# Descriptor Cargo source drift

The retained Cargo compilation path previously discovered changed admitted Rust inputs when it read the completed compiler receipt. A proven byte mismatch can now stop the running Cargo subprocess and enter the existing build-level concurrent-edit retry.

- The baseline comes from the already-admitted workspace units' absolute source paths and SHA-256 digests. The child monitor does not replan Cargo or acquire a new compiler grant.
- Nonrecursive directory watches provide event hints. Only named admitted inputs are queued. One stable physical file observation runs at a time and must produce a SHA-256 mismatch before cancellation. Same-byte rewrites, unrelated paths, missing or unstable files, and watcher errors do not cancel the child.
- Each monitor considers up to 4,096 admitted entries and opens at most 128 directory watches. In parallel with event handling, it rotates through the deduplicated available paths, checking up to 32 file metadata records per batch with at least 500 ms between batches. This supplies hints for unwatched paths, missed events, and edits between admission and child start. Metadata at or after source admission, allowing one second for filesystem time precision, can enqueue a path. The same hash check applies to every hint.
- Its queue holds at most 128 distinct inputs. Each SHA-256 observation checks at most 8 MiB through 64 KiB chunks and has a two-second observation deadline, with at least 200 ms between reads.
- The live compiler path terminates its owned child tree, drains output and closes the watch on terminal exit, resource cancellation or failure. A proven mismatch propagates as the build-level concurrent-edit error without spending time on a failed invocation receipt. Completion still requires the existing artifact receipt digest comparison and full plan rechecks.

## Verification

Both Bun fixtures under this ticket passed. The first launches a synthetic owned child, checks a fixed SHA-256 against Bun's independent implementation, replays same-byte and unrelated writes, then atomically replaces the admitted file with different bytes of the same length. It confirms both the expected digest mismatch and terminal output drain. The second detects an edit made before registration in directory 129, beyond the native watch limit. The actual native build entry imports in Bun, and all sixteen existing churn tests passed. No Cargo job was started or stopped for verification. Paths beyond the admitted-input budget, files beyond the observation budget, file metadata outside the fallback window, and churn after the monitor closes still depend on the existing final checks.

## Retry ownership

`concurrentEditV1("input", path, detail, "build")` creates the error just below build rank. `captureCargoArtifacts` drains its child before that error leaves the capture scope, and the target lease releases in the caller's `finally`. The build retry allocates a new compiler graph, runs Source admission, and issues new plan facts and identity on each attempt. It does not reuse the failed attempt's selected digest. The source memo accepts a hit only if every probed path's identity and high-resolution modification/change times still match, and rehashes entries stamped within its race window. The enclosing issued Cargo configuration and storage remain held across retries. A change to one of those enclosing authorities can still require the outer process restart.

## Release coverage

The third describe-tool attempt in run `wCsXqmg` recorded 501 workspace unit admissions, 2,950 input entries and 345 package names. The original ordered per-unit path roster was unavailable. The read-only `cargo-source-drift-census.ts` joins that observed package order to workspace Rust inputs from the published Darwin compiler receipt and sorts paths within each first-seen package. This proxy counted 1,949 physical Rust candidates in 1,897 directories. It includes compiler observations which may differ from the admission roster and assumes order within each package, so the following ranks are estimates.

| Source | Directory rank |
| --- | ---: |
| Actor retained-turn context | 45 |
| Job context | 293 |
| Job failure work | 299 |
| Manifest view-model selection | 435 |
| Value retirement boxed factory | 821 |
| Store document history hydration fold | 1,342 |

The rolling metadata scan covers every candidate considered under the 4,096-entry limit. At the configured cadence, 2,950 distinct paths would take about 47 seconds per pass plus filesystem latency. The native watches still accelerate named events inside the first 128 selected directories. Both mechanisms require an exact observation before the child can be stopped. The census read the retained receipt and phase log without running Cargo.

## Live release retry timings

Generation `wqJ4HZO` recorded two exit-75 descriptor retries for changed Store Rust inputs. The first recorded 1,234.331 seconds in Cargo. The second recorded 23.760 seconds in Cargo and 52.999 seconds total. Neither log shows a completed provenance receipt. The third job began compiling on the fresh plan. These attempt logs report the changed paths and retry, while the synthetic test independently verifies early watcher cancellation.
