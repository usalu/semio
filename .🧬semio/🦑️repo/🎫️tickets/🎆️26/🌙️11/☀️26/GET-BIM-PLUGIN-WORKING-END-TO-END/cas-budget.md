# CAS limit "1 GiB" against 27.76 GiB used

## Findings

- The 1 GiB is `minimumCapBytes` of the CAS policy, reached through the hard free-space reserve term of `casEffectiveCapBytesV1` (`🌡️partition/🟦️.ts`): `min(proportional, max(minimumCapBytes, store + free - reserve))`.
- This host holds a live `cache-host` role record (`~/.cache/semio/cas/v1/r/cache-host.json`, renewed by the fleet cache server). Its reserve is `max(8 % of 228 GiB, 72 GiB) = 72 GiB`. With store 27.76 GiB + free 37 GiB = 64.8 GiB the term is negative, so the cap clamps to 1 GiB. Frozen vector row (oracle): cache-host, 228 GiB, free 37, store 27.76 -> 1 GiB.
- Not caused by `hostSharePercent` (60 %): that cap was never applied to the CAS. The CAS ceiling was the raw declared 256 GiB, unrelated to the machine budget.
- Report vs derivation, both were wrong:
  - Report: `gc --unreferenced` takes every unreferenced candidate and never consults a cap, yet its progress line printed "used of {cap}" with the pressure cap. `status` showed only the pressure cap, not the cap that held entries are judged against (`heldCapBytes`, the proportional cap, about 51 GiB here). So "27.76 of 1 GiB, 0 held evicted" was consistent internally but unexplained.
  - Derivation: the CAS ceiling ignored the machine budget.
  - Enforcement: `repo cache gc` collected the host store with the raw `CAS_POLICY_V1` (256 GiB, no role, reserve or partition derivation), while `repo cache cas gc` used the derived policy. Two different limits for one store.
- The reserve semantics (a store never grows into the role reserve; held entries are only judged against the proportional cap) are deliberate and covered by existing tests, so they are unchanged. Non-held, unreferenced objects are the only thing the 1 GiB term can free; held, pinned, leased, linked and young entries stay protected.

## Fix

- Ceiling derived from the machine budget: new `casBudgetCapBytesV1` = `min(capBytes, machineHostCapV1(...))` (60 % of the volume, aligned: 136.75 GiB on 228 GiB), used as the ceiling of `casProportionalCapBytesV1` and therefore of every cap and `heldCapBytes`.
- One shared derivation: new `casJudgementV1(root, partition)` (census, usage hint, derived policy) used by `repo cache cas` and `repo cache gc` (`collectHostStoreV1`), so both enforce and report the same limit.
- Report: unreferenced gc progress no longer names a cap (new messages `progressUnreferenced`, `previewProgressUnreferenced`); `status` text and JSON show cap in force, budget ceiling (`budgetCapBytes`) and the cap for held entries (`heldCapBytes`); `statusCap` wording names the machine budget ceiling and the held cap (EN+DE).

## Changed files (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`)

- `🗄️cas/🌡️partition/🟦️.ts` (casBudgetCapBytesV1, proportional ceiling)
- `🗄️cas/⌨️command/🟦️.ts` (casJudgementV1, status fields, unreferenced progress key)
- `🗄️cas/🧬️schema/🔣️.json` (messages)
- `🧹️cache-gc/⌨️command/🟦️.ts` (host store uses casJudgementV1)
- `🗄️cas/🧪️tests/🟦️.ts` (new 228 GiB test), `🗄️cas/🧪️tests/🐍️capacity.py` (oracle applies hostSharePercent), `🗄️cas/🧪️tests/📋️capacity/🔣️.json` (hostSharePercent, 36 rows, 6 expected values changed)

## Verification (no gc run on the live store)

`SEMIO_LANGUAGE=en repo test-file`: cas tests 73 pass / 0 fail, cache-gc tests 22 pass, machine ledger tests 12 pass. Python oracle agrees with TypeScript on all 1192 vectors.

## Open

- Effective cap stays 1 GiB on this host while the cache-host role is live and free < 72 GiB; a regular `repo cache gc` (now using the derived policy) may free non-held unreferenced objects down to it. Pick `SEMIO_CAS_ROLE=builder` or let the cache-host record expire to size it as a builder (cap 20 GiB). The 72 GiB reserve floor on a 228 GiB volume is a role design question, not changed here.
- The CAS is not a store of the machine plan (`loose` counts only declared cache roots); not changed.
