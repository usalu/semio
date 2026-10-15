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
