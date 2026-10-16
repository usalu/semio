# Compact Recency Investigation

## Actual current observation

Actor Original 98939 refused retirement at the exact io-base64 unit `prototype/build/semio-framework-io-base64/2f5745745ec5f9bc`. Planned and current bytes both equaled 609608 and `lockHeld` was false, while maximum recency moved backwards by 71000 ms (1792064587000 to 1792064516000). The recensus guard correctly refused that unequal observation. No plan row was adopted and no guard changed.

A closed read-only Source prerequisite (`actor-current/📜️script.ts inspect prototype`, Original 16666) opened the actual Rust image with its real shared Native guard and finite 120000 ms Original, without adding a compiler charge or bypassing compiler admission. It enumerated only this exact unit, capped at 4096 entries, and captured 43 regular files with path, device/inode, bytes and atime/mtime/ctime nanoseconds. The image closed before completion. No SDK/tools store was opened and no Rust file contents or metadata were written.

Independent SQLite reduction over that captured output gives 609608 bytes and max(file atime, file mtime) = 1792064516000, matching the refusal's current observation. No current file carries the earlier planned maximum. The full per-file observations remain in `actor-recency-inspection1.log` until ticket output cleanup. Current data cannot identify the path/inode/field that supplied the historical planned maximum.

## Exact Source derivation

`⚡️caching/🧹️pruning/🟦️.ts:112–123` measures regular files only, using maximum `stat.atimeMs` / `stat.mtimeMs`; directory timestamps and symlinks are excluded. It only calls lstat/readdir and does not read file contents. `scanCargoBuildUnits:147–158` returns this value with `lockHeld:false`. Incremental sessions separately include their sibling lock timestamps. `🏪️budget/⚙️execution/🟦️.ts:49` compares the two aggregate byte/recency values before any deletion; equal aggregate bytes are not proof of equal per-file membership or identities.

Automatic recovery's `finally` removes its bounded diagnostic directory, so its per-operation plan output is not retained after failure. The current refusal log contains no historical maximum-source path, timestamp field or per-file inode snapshot. No timestamp-restoration call was found in the active pruning owner; the caching class passes utimes into its test context, which is not evidence that Actor execution invoked it. A concurrent writer, timestamp restoration or platform metadata behavior remains a hypothesis, not an attributed cause.

## Unchanged scanner oracle

A new test-only language-neutral corpus separates file-access maxima from modification maxima, sets unrelated directory timestamps later, and then moves file timestamps backwards without changing file bytes or inodes. An actual carried finite Original runs the real unchanged scanner over this private bounded fixture. Independent Python `os.walk`/`os.lstat` and SQLite `SUM(bytes), MAX(MAX(atime,mtime))` must agree with both the fixture and owned scanner. The final case proves that equal bytes/same inodes can coexist with a lower maximum timestamp; it does not attribute the real Actor event to that deliberate fixture transition.

Prototype is terminal GREEN: one test / seventeen assertions (`compact-recency-oracle-prototype.log`). No Source defect was reproduced and there is no production fix or Actor retry on the basis of this passing oracle. Release/expanded strict counterparts are pending collection at this checkpoint. Both editor catalogs register the new `issuer-bootstrap recency` modes; the real image inspection has its own exact entry.

## Remaining causal proof

Historical maximum-source provenance is missing. The next useful runtime diagnostic would retain bounded maximum-source path/field/device/inode/time for both planned and current observations, and distinguish any changed file membership. Such diagnostics must preserve exact recensus refusal and complete physical custody; they cannot turn a newly observed row into the original authoritative plan. A Source defect requires a genuine deterministic failing law before repair. Actor release and full composite remain held pending that evidence or the retirement owner's independently qualified capacity checkpoint.


## Collected oracle checkpoint

The unchanged scanner oracle is now BOTH GREEN, one test / seventeen assertions per mode (`compact-recency-final-recency-prototype.log`, `compact-recency-final-recency-release.log`). Initial expanded strict caught one new test-only `string | undefined` expected-inode lookup; the fixture now explicitly refuses a missing original inode before comparison. No production scanner/retirement implementation was changed. Expanded strict including current inspection/oracle code is recorded after collection below.

The fixture preserves the original device/inode identities and constant nine bytes while its maximum recency goes 1700000005000 → 1700000004000 → 1700000002000. Python/SQLite and the owned scanner agree at every step, and unrelated future directory timestamps are excluded. This validates derivation and confirms that aggregate byte equality does not establish stable timestamp evidence. It does not reproduce or identify the cause of the actual io-base64 transition, so it authorizes no production repair or Actor retry.


## Actual current scanner comparison

A second actual read-only image scope completed (`actor-recency-comparison1.log`). The production `measureUnit` path through `scanDirectoryUnits` measured the exact current io-base64 hash directory. Two independently executed Python/SQLite filesystem walks inside the same retained real image scope agreed: 609608 bytes, recency 1792064516000. This establishes current measurement agreement and stability across those observations; it still cannot recreate the historical planned maximum-source path or timestamp field.

Final controlled oracle BOTH remains one test / seventeen assertions per mode. Expanded strict after the local fixture narrowing currently stops only at the publication lane's in-flight `capacity/📥️physical/🟦️.ts:25` `NxSdkPublicationInputsV1` generic return mismatch (`compact-recency-final-types-prototype.log`); the owner received the exact diagnostic. Earlier Core/Actor expanded strict exit 0 remains recorded separately. No scanner, recensus, metadata or custody implementation was changed.

A next observational option, requiring a future authorized cohort, is a bounded same-Original observer in the closed Actor prerequisite. It would retain only the genuine Native image SH, take metadata-only snapshots of the exact unit, record changed maximum-source path/field/inode/times (with a bounded event count), and drain/close before Original retirement. It would hold no unit/file descriptor or Cargo-store lease, establish no deletion authority and never accept a changed plan row. That future cohort has not been launched; current Actor release remains held.


## Authorized bounded max-source observer

The parent authorized one fresh prototype cohort with an observer under its existing 600000 ms Original and unchanged 240000 ms Cargo child limits. A genuine missing-helper RED was collected before implementing `pruning/⏱️recency/🟦️.ts` (`actor-observer-red.log`). The new language-neutral corpus covers access maxima, equal maxima and backwards maxima. Independent Python/SQLite produces identical file count, exact byte sum, nanosecond maximum and ordered source path/field/device/inode/times. The carried-Original recency gate is BOTH GREEN, one test / twenty-nine assertions each (`actor-observer-final-prototype.log`, `actor-observer-final-release.log`). Duplicate paths, malformed integer metadata, more than 4096 rows, empty input and the eight-source tie limit are also checked.

The actual observer is wired only into `actor-current/📜️script.ts test`. It opens the authentic Rust image before any selected Cargo invocation, registers its complete lifetime with the same Original and awaits its initial sample. It observes exactly `b/p/prototype/build/semio-framework-io-base64/2f5745745ec5f9bc`, retaining image SH but no unit lease, file descriptor, Cargo store lease or deletion authority. Every walk is bounded to 4096 entries, performs metadata-only lstat/opendir operations, closes directory handles before sleeping and hashes ordered facts. Missing paths or disappearance during traversal are explicitly labeled observational states. No snapshot is treated as an atomic authoritative plan.

Sampling is limited to 1200 turns with 500 ms between walks. Changed facts emit at most 64 records and 262144 encoded record bytes, with at most eight deterministic maximum-source ties. Each record includes monotonic elapsed time, exact directory, status, full-fact fingerprint and maximum-source provenance. The observer is stopped, awakened, awaited and the actual image closed before the Original can settle, including after Cargo failure. Both primary and observation errors are retained if both fail. The production scanner, recensus comparison, retirement policy and all existing witness evidence remain unchanged.

The first expanded strict caught one test-only expected JSON string-literal narrowing, now corrected. It also exposed the orchestration owner's pending `.mjs` declaration seam; that owner was notified. The one authorized prototype cohort has not started at this writing. Release remains held until prototype actual terminal; no unchanged unexplained repeat is authorized.


## Fresh prototype started

Expanded strict is now collected exit 0 (`actor-observer-types-final.log`) after the precise peer orchestration declaration landed and the new test assertion's expected-value type was widened without changing runtime checks. The one authorized prototype Original is 82098, log `actor-observed-prototype.log`; it retains the same authored 600000/240000 ms limits. Release has not started.

The observer's first sample recorded the original 43-file, 609608-byte io-base64 unit, maximum 1792064516000000000 ns, including exact fingerprint and output inodes. At 5261 ms it observed the exact directory absent while the normal automatic compact retirement retired the previous 94-unit selection under existing custody and recensus. The prior changed-row refusal did not recur in that selection. Subsequent actual compilation recreated the unit by 42623 ms with different fingerprint/output inodes and a new maximum. These are fresh-run observations; they do not reconstruct or attribute the historical 71000 ms reversal. No old plan row was adopted and the production guard remains unchanged. The six-owner cohort is still live at this writing.


## Collected observed prototype terminal

The single authorized prototype Original 82098 is collected terminal exit 1 after 77514 ms. Actor Pack completed 0/0 and RetainedTurn completed 10/10. RetainedContext's actual preparation stopped before its compiler at an existing recensus refusal on a different exact unit: `prototype/build/serde/a08cbd69c84178a8`. Expected/current bytes both equal 1405739 and both lock flags are false; planned maximum recency 1792066731000 became 1792066730000 (1000 ms backwards). The unchanged physical-custody recensus guard refused before that selection's deletion. No row was adopted, no guard was weakened and no manual retirement occurred.

Before this new refusal the normal enforcing recovery completed the old 94-unit selection (436322533 logical bytes) and a seven-unit selection (41637566 bytes), reaching 4297543680 physical free bytes. Later finite recoveries also ran through the actual existing owner. This qualified successful recovery at those defining observations, not permission to ignore the subsequent changed row or a permanent free-space guarantee.

The observer drained 145 samples, emitted 18 bounded change events totaling 18564 record bytes, and closed its actual shared image before the Original settled. It captured the io-base64 unit's original 43-file tree, removal, fresh inodes after compilation, subsequent access maxima and final removal during genuine recovery. Sample 128 at 68638 ms identifies the io-base64 maximum 1792066731000000000 ns from `fingerprint/dep-lib-semio_framework_io_base64` inode 514250 and the retained rlib inode 514227, both atime fields. That is evidence about this exact io-base64 tree only. The newly refused serde unit was deliberately outside the authorized observer target; its historical maximum-source identity remains unavailable. No causal defect in the scanner is established by this run.

Release remains held. No repeat of the cohort was launched. The next step belongs to the retirement/provenance owner: qualify the actual source-supported promotion race or collect exact planned/current per-file provenance for the serde row under its real custody. The bounded observer may be reused only by a separately authorized future cohort. Its metadata never supplies or repairs a retirement plan. Current complete Actor composite is still pending RetainedContext, Vocabulary, JobPublication and Actor plus the release cohort.


## Same-scan production diagnostic handback

The generic actual planning/recensus scans now retain bounded maximum-source provenance rather than relying on a hardcoded separate observer. Full design, finite capture/output limits, same-stat numeric equivalence and genuine RED→BOTH 1/75 + expanded strict exit 0 are retained in `compact-recency-provenance-current.md`. All diagnostic Source is frozen for Root review. The existing recensus guard remains unchanged. No Actor repeat or release was launched after 82098.
