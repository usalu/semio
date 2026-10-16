# Graph Carried Phase Source Audit

2026-10-16. Finite read-only Source review of the carried-phase cut. No new actionable dropped-phase, fresh Original, signature/caller, static callback self-wait, or accepted-operation lifetime finding identified within this bounded review. This is not semantic or runtime qualification.

## Evidence

- Neutral selection schema lines 30, 57–61 require Original plus actual phase at observe/acquire; issuance is checked before admission and after callback. Both calls enroll with `withIssuedResourceOperationV1`; retained recheck closes over the same phase and Original and enrolls its actual callback. No new phase is minted here.
- Runtime verification storage returns the actual `withRepositoryCompilerStoreV1` callback control as `scope.phaseControl` (storage lines 27–29); verification lines 367, 388–405, 418, 424–427 carry it into actor observations/acquisition and owned commands. Physical observation clocks are additionally bounded by `phaseControl.remainingMilliseconds()`; they do not replace the actual phase clock.
- Hub actors lines 30–31, 51, 69–82 require/carry the actual phase through read-only current observation, compiler producer recomputation and acquisition. Acquisition narrows with `withRequestedIssuedResourceControlV1(phaseControl,0,...)` and passes `scriptInvocationEnvironment(invocation,...)`; no new Original is created. Recheck retains context including the same actual phase.
- Browser verifier actual signature is `(observation, invocation, phase, control={})` at browser-bundle Script line 396. It authenticates the required phase before validator/fingerprint/scratch activity and still invokes `buildBrowserCodegenModule(scratch,check,invocation,phase)` at line 404. Required recomputation remains. Actual Hub caller supplies four arguments; runtime producer fixture supplies three, with pure-control default. Repository `.ts` call roster contains these two callers.
- Actual Hub Rust Script line 178 issues `hub:document-sweep` from `this.invocation`, calls the required CLI ports, and CLI lines 438–456 narrows the incoming phase with cancellation and supplies that actual child to sweep/publication proof. Sweep lines 330–336 authenticates before its first mkdir and forwards Original/phase to selected actor observation. Repeated file-observation clocks at line 335 are fresh per observation, but selected actor `checkCancellation` authenticates the retained actual phase, so these clocks do not mint replacement compiler authority. Storage funding and broad sweep lifetime remain outside this cut's qualification.
- Source caller inspection includes migrated foundation-source missing-ledger/cancellation fixture, neutral port identity/copied-port refusal fixture, runtime producer fixture, and the sole actual `runHubDocumentSweep` caller. Neutral fixture checks callbacks receive exact invocation/control identities and explicit acquisition order. Tests were inspected, not executed by this audit.
- Issued operation implementation registers through canonical `track(original,run)`; requested child settlement belongs to its actual parent. The selected callback path does not call settlement of its own phase. Root's separate canonical final post-drain audit remains independent.

## Limits

No actual graph, sweep, compiler, browser, SDK, image, download or runtime operation ran. The successor BOTH2/33 neutral results are prior ticket evidence, not rerun here. Broad actual-root strict remains blocked by the reported 361 syntax diagnostics in coedited normalization/schema; this audit does not claim a semantic pass, and absence of matched new-owner diagnostics would not imply one. Existing storage/publication funding and whole-graph synchronous physical reads are not qualified by this phase-threading review. Hashes below identify the inspected snapshot, which can be superseded by concurrent edits.

## SHA-256 Snapshot

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🎭️selection/🧬️schema/🟦️.ts`: `84b334674dba0048f4154c6bd1b5a7fab1b8687a30076b2ae1c5038d9bd5c306`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts`: `a5b8569b09ba2b3affd25db161348fee7549340f9689cafa752f0e677d35425b`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🟦️.ts`: `5a058566c6b932c07582befdf49dca8e65d9e79aea8c1900b4fd84d5e0b68f53`.
- `🌎️hub/🏗️bootstrap/🎭️actors/🟦️.ts`: `e31e6468a71b209514119a7b08c8d4a300b981e83e9d12fc9db7903f808ce5e4`.
- `🌎️hub/🧪️tests/🗂️document-sweep/🟦️.ts`: `c8b723bac60752864655ba0380bf32366b780f14bb24f3f60b1ae47b800bf5ed`.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: `d54a520171c733feeb085e895dd7f440a573019d486161fbb3c84aae145875a3`.
- `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts`: `d497dc11ca7d623a5f82dcbb93e617bada98592dc0049d81d32d1fc08fd7cb1f`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🎭️selection/🧪️tests/🟦️.ts`: `0898c20b4ec605c32b49d739963df96dbd5d61f68a7254120cf0514550156fa7`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts`: `fd335b5ffd47143d4ea5858ced3895594f633d0830b334e8dccb3969bed7eb58`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts`: `1696226b1b7227913eac5216fff7399b02346b7e08ca0ff18e973c60a7ec373d`.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`: `f2854f3a5820c190a793ba0eae625ab4acadb468d403de7ac60c2da9548ae5b3`.
