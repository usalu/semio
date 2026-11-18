# Graph admission memo: reuse across unrelated graph edits (2026-11-18)

## Root cause (confirmed in code)
- `🕸️graph/📥️load/🟦️.ts` `inventory` accepted the persisted memo `selected/<scope>.admission.json` only if `buildGraphProducerAuthorityV1` matched `loadedGraphModuleAuthorityV1.implementation`, the hash of about 55 graph modules (entry modules, roster, observation, inputs, storage, ...). Any peer edit of one of them missed the memo; the per-owner facts memo (`ownerFactsMemoV1`) was keyed by the same hash and lives in the process only, so a miss re-derived all 763 owners cold.
- The memo stored only the closed admission plus one flat observation list: no per-owner facts, so no per-row reuse was possible even when the format was unchanged.

## Design
- New admission format hash `graphAdmissionFormatHashV1` / `loadedGraphAdmissionFormatV1` (`🕸️graph/🔐️authority/🟦️.ts`): sha256 over only the modules that define the admission format and discovery semantics: `📥️inference/🔎️owner-discovery/🟨️.mjs` (+ `🕸️wasm-outputs/🟦️.ts`), `🚪️admission/🟦️.ts`, `🧭️roster/🟨️.mjs`, `🧭️file-owner/🟨️.mjs`, schemas `🚪️admission.json`, `📋️owners.json`, `🚪️selection.json`. `graphAdmissionFormatUrlsV1` lists them. The capture function is injectable for tests.
- Memo v2 (`📥️load/🧠️admission-memo/🟦️.ts`): `{version:2, format, scope, admission, observations, owners:[{suffix, facts, rows:number[]}]}`. Owner rows are stored once as indexes into `observations` (`packAdmissionMemoV1`); `ownersOfAdmissionMemoV1` resolves them. `readAdmissionMemoV1(path,scope,accept)` only parses and checks scope and format; the caller asks whether the observations hold.
- Load (`📥️load/🟦️.ts`): the memo is a full hit when its format matches and all observations hold. Whatever the outcome, the owner facts of a format-matching memo are seeded into `ownerFactsMemoV1` (skipping suffixes the process already knows). On a miss, `observeOwnerFactsMemoizedV1` revalidates each owner's own rows and re-derives only owners whose rows changed. The per-owner key and `selectedAdmissionFactsV1` (fleet exchange seed) are now keyed `scope\0format\0owner...` instead of by the implementation hash. After a derivation the memo is rewritten with the current per-owner entries.
- Progress text: schema `🧠️memory/📋️slice-report/🧬️schema/🔣️.json` got `counter` ("elapsed seconds of expected seconds" / "verstrichene Sekunden von erwarteten Sekunden"); `describeSliceReadStepV1` appends it to the phase in both languages, so the line reads "...(elapsed seconds of expected seconds) 2010 of 600". The sidecar step regex still matches.

## Files changed
- `🕸️graph/🔐️authority/🟦️.ts` (format urls, hash, loaded value)
- `🕸️graph/📥️load/🧠️admission-memo/🟦️.ts` (memo v2, pack/unpack)
- `🕸️graph/📥️load/🟦️.ts` (imports, keys, read/seed/hit/write)
- `🧠️memory/📋️slice-report/🟨️.mjs`, `🧠️memory/📋️slice-report/🧬️schema/🔣️.json`
- Tests: `🕸️graph/📥️load/🧠️admission-memo/🧪️tests/🟦️.ts` (rewritten), `🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🔗️wiring/🟦️.ts`, `🧠️memory/📋️slice-report/🧪️tests/🪜️read-steps/🟦️.ts`, bindings of `🧪️tests/🕸️graph/{🩹️repair-rounds,🧊️stale-facts,🌊️seal-under-churn}/🟦️.ts`
(all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`)

## Tests added (admission-memo test file)
- unrelated graph module hash change (every `graphSelfUrlsV1` module and observation mjs) keeps the format hash, and a memo written under it still hits;
- a change of any format module changes the hash, and the old memo misses;
- stale memo lends owner facts: re-derives exactly the one owner whose row changed (real `observeOwnerFactsMemoizedV1` extracted from the load source);
- pack/unpack, corrupt/foreign/other-format/v1 memo misses, size limit, source-order guard (no `loadedGraphModuleAuthorityV1` in the inventory block).

## Results (`bun test <file>` from the module folder, no repo process env needed)
- admission-memo 8 pass; repair-rounds 11 pass; stale-facts 2 pass; selected-exchange wiring 3 pass; read-steps 6 pass; step-overdue 8 pass (9 skipped as before).
- `🌊️seal-under-churn`: 3 fail, pre-existing on Windows: the test's in-memory store keys are `/owned/...` while `join` yields `\owned\...` (receipt lookup returns undefined). Unrelated to this change; bindings were updated for the new names anyway.
- No `repo run` executed. The existing 118 MB v1 memo on disk is a one-time miss (version 2); the first run after this change derives once, later runs reuse per owner.

## Caveats
- The memo no longer covers the generic graph implementation or `publication`; correctness of reuse rests on per-row observation revalidation plus the format modules. Add any module whose semantics change admission to `graphAdmissionFormatUrlsV1`.
- Memo size grows by the owner facts; writes above `maximumSnapshotBytes` are skipped as before (rows are deduplicated by index to limit growth).
