# Fenced Capacity Lock Release Repair R92

A real native cellar-contact check stopped before Python because synchronous removal of the shared host-capacity lock raised Windows EBUSY. The protocol now routes only this known busy removal through the existing declared read retry policy (6attempts,10/40/160/640/2560ms pauses). Every attempt rechecks the exact owner token, including stale-lock breaking; a peer that took over is preserved. Unrelated errors still fail, and exhausted busy errors remain bounded and explicit. Admission limits are unchanged.

Schema-first portable cases cover transientbusy, ownership replacement during retry, persistentbusy and unrelated permission errors. Baseline captured1schemaPASS/4retryFAIL. Independent Ajv validates the portable schema; RxJS supplies an independent retry/fencing state trace for each case. New regression and original ownership-fence tests run through repo and put all temporary artifacts under this ticket generated directory. GREEN result pending.

## Verification

GREEN8tests/26assertions/2.40s: all4 portable retry cases match independent RxJS traces, Ajv schema agrees, and all3 pre-existing stalled-owner/growth/progress fence tests pass. Generated JUnit stored under ticket output. No capacity policy or admission bypass changed.
