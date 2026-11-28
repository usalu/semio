# Unattributed SIGTERM of long `describe --all` runs

## Root cause

The host memory warden killed the runs. Its own alarm journal says so (`~/Library/Caches/semio/host-capacity/warden/alarms.jsonl`):

- u4: `63164.de245e07`, action `cancel`, reason `overdue`, kind `granted`, 4813 MB held against a grant of 3510 MB, at 1795817033813. The descendants log "killing process 63164", so 63164 is the outermost `repository:command`.
- u5: `32678.04381fcd`, same shape (4390 MB). An earlier run died the same way: `89494.8fc832e0`.

The chain:

1. Under memory pressure the warden SIGSTOPs the "granted" subject. That subject is the whole script tree of a foreground run, whose booking is a capacity grant (`grantedWardenSubjectsV1`).
2. After `suspendMaximumSeconds` it is "overdue", and the overdue rule cancels it with SIGTERM to the tree. The subject is marked `lastResort`, but that only orders it last. Under severe danger it is still cancelled, together with 40 to 85 minutes of progress.
3. The warden wrote a cancel record (`cancelled/<pid>.json`), but nothing in the victim read it.
4. The victim's SIGTERM handler aborted with the anonymous `Error("Original Script process terminated")`, which printed as `Internal error ... (reference 557d4d71)`.

Other senders of SIGTERM to foreign processes were reviewed and ruled out for this event:

- `terminateOwnedProcessTree` and the shared-child host record their kills (the "owned execution" lines).
- The lease yield, the orphan watch (SIGTERM to self) and the replica supervisor have no matching records.
- There is no `log show` evidence of anything else.

## Fix

(a) Attribution. `attributedTerminationV1` (`🧭️routing/📥️invocation/🏃️process/🟦️.ts`) runs when a script process receives SIGTERM or SIGHUP. It reads the warden's cancel record for its own pid. If one exists, it aborts with the typed `HostWardenCancelledErrorV1`. That error has code `host-warden-cancelled`, carries sender, kind, pid, reason key and megabytes, and has English and German text from the existing warden catalogue. The text is also printed to stderr. Without a record the old behaviour stays.

(b) Policy. The warden never cancels a subject that holds progress:

- New subject flag `keepsProgress` (schema `🧬️schema/🔣️.json`, type, and the Python oracle). The granted subject sets it.
- A `kills()` predicate replaces `canCancel` on every cancel path of the decision law: limit, overdue, danger victims and the wake rules. Cancels by cap and ceiling are unchanged.
- Such a run is only suspended and resumed. Once it is stuck beyond the hard limit it is resumed, never killed. Other kinds (slice workers, units) are still cancelled and requeued by their owners.

(c) Hang of u6 (executors with unreaped `<defunct>` children, nx waiting forever). The watchdog's tree-stall rule waited for the full 30 min idle-tree period, and a long enough declared wait (up to 4 h) suppresses it entirely. The cause of the wedge itself (an executor that never reaps) is not diagnosed. The watchdog is now typed and faster for this case:

- New policy `defunctIdleMilliseconds` (300000, env `SEMIO_SCRIPT_DEFUNCT_IDLE_MS`) and new state field `defunct` (schema).
- `defunctChildCountV1` counts unreaped (state Z) descendants.
- A tree that has an unreaped descendant and zero CPU for 5 min is now `tree-stalled`: the children are terminated and the process exits with the existing typed stall notice.
- A declared wait still wins (healthy). I could not confirm from the logs whether u6 sat in a declared wait. If it did, the 4 h wait ceiling is the next lever.

## Tests (language-agnostic, next to the code)

- Warden fixtures (`🧪️tests/🧫️fixtures/🔣️.json`): two new `progress:` decision rows, checked against the Python oracle. Both resume the overdue granted run and never cancel it. A TS test also asserts that without the flag the same run is cancelled.
- Test `attribution:` writes a cancel record and checks the typed error. It checks the English and German text, and that an unannounced pid stays anonymous.
- Watchdog fixture (`🧪️tests/🔣️.json`): four new decision rows (defunct at, just below and without the period, and inside a declared wait). They are checked by the schema, `scriptProcessStallDecisionV1` and an independent SQLite query.

Results: warden suite 63 pass; watchdog suite 15 pass (run after the fixtures were added); typecheck of the touched files passed.

## Changed files

All under `🧰️framework/🔨️modules/🏃️process/`:

- `🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts`
- `🏗️build/🌡️host-capacity/🛡️warden/🧬️schema/🔣️.json`
- `🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/🟦️.ts`
- `🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/🐍️.py`
- `🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/🧫️fixtures/🔣️.json`
- `🧭️routing/📥️invocation/🏃️process/🟦️.ts`
- `🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts`
- `🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧬️schema/🔣️.json`
- `🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts`
- `🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🔣️.json`

## Not done

- The resident warden process must restart to pick up the new decision law. It restarts itself when its sources change, so no action is needed.
- Only the warden is attributed. There is no general sender record for other kill sources, because none fired here.
