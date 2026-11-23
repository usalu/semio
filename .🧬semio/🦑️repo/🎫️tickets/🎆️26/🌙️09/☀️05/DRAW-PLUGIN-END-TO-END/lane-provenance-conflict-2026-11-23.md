# Lane Provenance Name Conflict (2026-11-23)

## Symptom
Native component build finished (1 component staged, 0 failures), then failed with
`lane p409ae0 holds 1 provenance file(s) whose names already exist in the shared ledger with other bytes (cargo-unit-provenance-release-a0189217...json)` and `Lane receipt custody remains unresolved`.

## Root cause
The name of a pass receipt is `nativeBuildPassObservationPathV1`: `sha256([pass.manifest, args, generation.build])[0..32]`.
The content is not a function of those inputs: it records `builtAtMs` (the run start), the observed units, artifact digests and build-script outputs.
For a warm generation `generation.build` is the build directory of a pooled lane, and pooled lanes are reused (same crate set gives the same lane, `p409ae0`).
`CargoProvenanceSession.write` stages the receipt twice: in the run's report directory and in `<generation.build>/semio-cargo-provenance/<same name>` (the lane ledger).
Retention publishes the lane ledger by content into the shared ledger and never replaces a name.
So run N publishes the name, run N+1 of the same pass in the same reused lane produces the same name with other bytes (`builtAtMs` alone differs) and retention reports a conflict, refuses to retire the lane, and fails an otherwise successful build after 25 minutes.
It is not a race between concurrent lanes (a lane is held by one process) and not a nondeterministic serialisation; two genuinely different observations shared a name.

## Fix
`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`: the name now also covers `generation.report`, the directory private to one run (`runs/<random>/r`), so the name is the receipt of one run of one pass in one lane.
Conflicts stay loud: nothing is ignored or merged, because the observations differ semantically (time, units), so canonicalising bytes would be wrong.
Names are unique per run, so the ledger grows one file per pass per run, as it already does for relay receipts (`randomUUID`); the shared ledger is trimmed by the existing cache collection.

## Verification
- New `🧪️tests/🏷️observation-names/🟦️.ts` beside retention: reproduces the conflict with the old name through the real `retainLaneOutputsV1` (second run in the reused lane conflicts) and proves with the new name both runs publish byte for byte (independent `hashlib` SHA-256), plus the name covers manifest, args, lane and run.
- Updated the pinned formula in the vite artifacts test (`🌐️vite/📥️artifacts/🧪️tests/🟦️.ts`).
- Ran: `repo tools exec --light bun -- test ./<file>`: 2 pass, 0 fail. No component build was run.

## Note
The lane `p409ae0` still holds the unpublished old-name file from the failed run; the next retention of that lane will keep reporting it until the stale file in the lane ledger is removed (it equals no shared bytes). Delete that one lane file by hand once, or the next run's release will fail the same way.
