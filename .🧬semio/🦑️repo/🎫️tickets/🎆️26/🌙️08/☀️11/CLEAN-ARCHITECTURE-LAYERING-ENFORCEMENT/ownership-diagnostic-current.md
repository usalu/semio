# Bounded Bun Ownership Refusal Diagnostics

The assigned three-file batch is frozen. This changes internal refusal presentation only; it adds no exported API, Source authority or admission quota. The actual Actor15 old/new membership rows were not retained and remain unknown. No writer, publication-material change or concurrent-owner cause is inferred.

## Guard and presentation

The existing observation key `[operation,path]`, JSON equality condition, original observation map, filtered byte-sorted physical list, Source/state observations, currentFence replay, control, cancellation, policy checks and publication behavior are unchanged. Only an already-triggered ownership refusal gains a `; change=` JSON suffix. It contains operation and path. For list mismatches it reports added/removed/typeChanged entries, at most eight rows total across all three categories in the existing Buffer byte-name order, plus complete changed-row and omitted-row counts. A fixed-state mismatch reports its existing fixed-size previous/current state arrays. No entire list, manifest contents, Source map or provenance is serialized. The eight-row limit only limits error presentation; all observations still participate in the original guard.

The helper is private to the existing CJS implementation. Existing portable fixture data specified the expected presentation before implementation; no new public/domain schema or caller shape is introduced. The production implementation uses system primitives only. The installed fast-json-patch library is used solely by tests to independently derive add/remove/type replacement facts from name-keyed before/after fixture maps.

## Executed evidence

- Initial actual test-first result: **2 fail, 1 pass**. Both changed-membership cases retained the original refusal but failed because the diagnostic suffix was absent; unchanged membership passed. Retained log: `🗑️generated/ownership-diagnostic/red-current.log`.
- New cases after implementation: **3 pass, 0 fail, 26 expectations**. They cover mixed add/remove/directory-to-symlink changes, ten additions with exactly eight rows and two omissions, and unchanged membership completing without refusal. Expected fields are exact and contain no provenance attribution.
- Final focused owning suite: **9 pass, 0 fail, 44 expectations**, including existing topology refusal, cancellation, final-continuation concurrent-edit preservation and unrelated-document work. The native-install tests were filtered out. Log: `🗑️generated/ownership-diagnostic/focused.log`.
- Ordinary TypeScript **strict: 0 diagnostics, 292 loaded sources** on the actual owning test entry. Output: `strict-semantic.json` in the same generated directory.
- The additional `noUncheckedIndexedAccess` probe reports one existing diagnostic, unchanged by this batch: code2532, actual owning test line76, `Object is possibly 'undefined'.` It is the earlier controlled-publication test’s `report.scopes[0].patterns` expression. There are no diagnostics in the new diagnostic cases. It was not repaired or suppressed. Exact output is `semantic.json`.
- All three owned hashes match before/after both final focused and strict probes (`inputs.json`, `strict-inputs.json`). Input recorder `od1/📜️script.ts` retains the exact test selection/compiler options. An earlier relative test-filter invocation was interrupted by its owner before discovery/assertions; `red.log` retains its header and supplies no test acceptance.

No registered frontend, configured factory, private endpoint, Store/native execution or dependency initializer ran. No cache reset, retry, observation pruning, budget/network change or authority renewal was added. All other held production Source is untouched by this batch. Root controls the fresh Actor16 dispatch after review.

## Current frozen Source

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs` — `525e20d7fd96fc4385fe8bcfdb554a69c013865e373abd01fc3a8d2ce832e0ae` (22519 bytes; 103 absolute path characters).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧪️tests/🟦️.ts` — `affac9f7931b6d3787dcc600ecce146ef88d90fe2c2c28ddba64d5eb54f3d3bb` (34865 bytes; 110 absolute path characters).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧪️tests/🧫️fixtures/🔣️.json` — `bdd5819155dbca3da6607678bc446f386e21cbc47cd2d9f2b974f972d2ca6439` (19415 bytes; 123 absolute path characters).

The fixture, test and private presentation helper are the entire production batch. Catalog, schema, GUI, endpoint, argument binding and inference files were not edited here.
