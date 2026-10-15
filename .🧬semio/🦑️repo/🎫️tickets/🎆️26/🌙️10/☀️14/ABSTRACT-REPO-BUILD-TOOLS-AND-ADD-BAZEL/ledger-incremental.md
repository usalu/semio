# Incremental Optional Graph Ledger

The actual producer-prefix profile is retained in [all-cpu-prefix-results.md](all-cpu-prefix-results.md). Its optional-publication stacks repeatedly entered reconciliation and whole inference inventory. This change removes that inventory only from clean operations under the new custody protocol; opening a ledger and repairing a dirty ledger still measure actual optional files.

## Protocol and safety

Each open, publication and retirement acquires the small gate database exclusively before accounting access. Publication commits `dirty=1` to the accounting database while retaining the gate transaction, performs the synchronous physical callback, then commits its exact retained-byte delta and `dirty=0`. A crash releases the process gate while leaving committed dirty intent. Another already-open handle repairs from the filesystem before it admits a subsequent write. Retirement commits intent first and measures after its callback. No gate transaction spans a producer body.

Both connections explicitly verify DELETE journals, FULL synchronization, 4 KiB pages, disabled cache spilling and finite page caps. Database leaves and journal/WAL/shared-memory sidecars are checked for original physical ancestry, regular type, link count and bounds before SQLite can open/recover them, and again during operations. Original control is checked after asynchronous opening and after commits. Exact gate/accounting SQLITE_BUSY admission returns false only for optional publication; other errors and original cancellation propagate. Cleanup attempts both connections and retains the primary failure.

The accounting database remains bounded to 32 KiB and the gate to 16 KiB. With cache spilling disabled and no application savepoints or attached databases, the journal allowance uses one maximum 64 KiB sector header plus one `(page bytes + 8)` record per capped database page. This yields 98,368 and 81,952 bytes, respectively. Both complete database and journal maxima total a conservative 229,472-byte metadata reserve, deducted alongside optional payload peaks from available space. Initialization requires only unavoidable metadata capacity; the optional minimum-free-space floor applies to publication admission, not mandatory Source work. No `journal_size_limit` assumption is used as an active-journal cap.

SQLite documents journal page uniqueness and the eight-byte page-record overhead in its [rollback journal format](https://www.sqlite.org/fileformat.html#the_rollback_journal); its [pager implementation](https://sqlite.org/src/doc/trunk/src/pager.c) clamps sector size to 64 KiB and defines the no-spill flag. Runtime tests additionally observe active journal sizes against the declared bound. This is a finite protocol bound, not a claim that filesystem power-loss testing ran.

Whole-image maintenance can delete optional leaves before a ledger opens. Unconditional initialization inventory remains the boundary that reconciles those changes and previously existing clean rows. Record opener cleanup now accounts only exact pack/index temporary leaves through the existing publication callback with zero peak and the actual negative byte delta. Denied cleanup leaves the file intact; unknown `other.tmp` is neither deleted nor charged. Mandatory producer publications and receipts remain outside optional accounting.

## Actual TDD and runtime evidence

- `ledger-incremental-red.log`: terminal 1; 5 pass, 2 fail, 114 assertions. A second SQLite connection could not see the old uncommitted dirty intent during the FS callback; orphan deletion invoked no accounting callback.
- `ledger-orphan-spelling-red.log`: terminal 1; the additional unknown temporary filename caused a second negative-delta publication.
- `ledger-floor-red.log`: terminal 1; the proposed open-time optional floor rejected initialization, and the unknown filename case remained red. The open-time floor requirement was corrected.
- `ledger-incremental-final2.log`: terminal 0; **7 tests, 185 assertions, 14.16 seconds**, under the retained authentic 120-second original SDK/namespace/test-artifact gate. Each actual-image law retains its original 60-second cap.

The final gate exercised genuine private fixed images and original controls. It proved independently visible committed intent, zero inference-directory scans for successive clean publications, cross-handle counters, dirty repair after physical failure, exact child exit 17 and expected physical results for actual API publication and retirement crashes, and recovery through a surviving open handle. Independent Node SQLite holders tested both accounting and gate contention without callback entry. Original narrower cancellation propagated. Six existing sidecar symlinks were refused before touching their targets. Actual whole-image exclusive maintenance followed by reopen reconciled the retained clean row.

The actual Node 24.15.0 ledger API branch also ran against the same genuine image using the locally supported `--experimental-transform-types` flag: clean publication, independent dirty-intent read, original physical failure, and subsequent repair all passed. The initial strip-only attempt refused a real TypeScript parameter property; no shim, copied Source, substitute adapter or inferred Node parity was used. Bun 1.3.14 exercised its own system SQLite adapter. The existing language-neutral publication/namespace corpus, Ajv validation and independent SQLite arithmetic remain in the same seven-test gate.

These tests are focused storage evidence. They do not establish a completed ALL graph or exhaustive Bazel/Stdio qualification. Root owns the subsequent broader graph gate; no new producer or CPU profile was run by this change.

## Exact source hashes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📒️ledger/🟨️.mjs`: `ef157658c605f54a316c1cb6d0cb09e4985ada9dd908f97f86a8aa53be8cf347`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/📒️ledger.json`: `e159280130358639e99c6f29d5983c285a679ff7586ecfe2871a56480b8ae824`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger.ts`: `1668a59804c17d6028e03edd68ca2132b6c7a3e834ac3d1c7adb6cc94abe29a1`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger.json`: `e6b79c3e6646c2cf393c99892688971b6b761ec7a45d47d6a1f26c42432ded40`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗃️records/🟦️.ts`: `fcd3af70ca6f745503b233292cf15c6b39d19c22101fe79e8bdfe46c0cf1c6c6`

The retained `qualify/📜️script.ts --ledger-gate` selects the existing permanent ledger test through its authentic SDK/namespace setup. Full hash evidence is generated output in `🗑️generated/ledger-incremental-source-hashes.json`.

## Permanent task registration

The existing root `workspace:test-workspace-build` target calls the existing build `📜️script.ts test`. That script explicitly includes `🏗️build/🧪️tests/🕸️graph/🟦️.ts`; the graph entry now statically imports `../../📥️inputs/💾️storage/🧪️tests/📒️ledger.ts`. The seven ledger laws therefore remain reachable through an existing permanent task after ticket cleanup. Root subsequently ran the combined existing graph entry with this import: terminal 0, 95 tests, 2,156 assertions, 47.12 seconds (`🗑️generated/final-graph-gate-namespace.log`). This confirms the seven imported ledger laws execute in the combined entry. No command or launch entry was added.
