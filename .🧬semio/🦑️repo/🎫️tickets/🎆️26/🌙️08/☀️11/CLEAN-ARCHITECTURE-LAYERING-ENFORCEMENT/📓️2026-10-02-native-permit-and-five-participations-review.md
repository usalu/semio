# Native Permit and Five Participation Review

Read-only actual source inspection; no jobs. Full replay1079 is parent-reported live; no pass claim.

Owner test `🧪️tests/🧱️rust-source-direction/🟦️.ts:16–36` nativeCommand has one module-wide four compiler pool. Waiter FIFO hands off an occupied permit without increment/decrement; no-waiter completion decrements. Spawn failure releases through finally. Runtime binaries bypass compiler count; all rustc call sites now go through helper while two direct runtime Bun.spawn sites remain. Child args/CWD/explicit env are preserved by helper spread; no production/binding API type changes introduced by test-local helper. Source review retains all17 law bodies and original expectation call sites; this is not a historical byte-diff proof or execution receipt.

One exceptional limit: Promise.all stdout/stderr/exited rejects as soon as any promise rejects, then finally releases permit even if child.exited has not settled. A failed pipe-read could admit another compiler before the old one exits and cleanup could race it. Use settled stream collection while always awaiting child.exited before permit release (then propagate first owned error). Ordinary successful child completion is correct; no observed pipe failure is asserted.

## Five explicit observation projections

Existing fixture/schema graphAuthority fields are files/unavailable/nativeInputs/contexts/targets/invalidManifests; they currently have no participation projection. Add required closed `participations` array. Minimal row fields sourcePath,crateRoot,manifestPath,modulePath,sourceScope,mount,state,reason(optional iff denied). Preserve exact mount/declaration offset provenance in complete API; projected fixture may use closed mount kind and parent from where enough for this five-row contract.

- readable-empty-root: root.rs/root.rs/Cargo.toml/[]/[]/root/admitted, no reason.
- unavailable-root-bytes: same authored root origin/denied reason unavailable-source; retain observation despite contexts [].
- readable-empty-leaf: admitted root plus leaf.rs/root.rs/Cargo.toml/[leaf]/[]/module from root.rs at declarationOffset0/admitted.
- unavailable-leaf-bytes: admitted root plus same leaf candidate denied unavailable-source; never infer root unavailable or erase root context.
- invalid-manifest-revokes-targets: root and leaf origins both denied invalid-manifest. Also authored foreign key origin must be retained as denied nonportable-target with raw path `C:/outside.rs`; there is no repository physical sourcePath for that target. Therefore sourcePath cannot be the sole required observation identity: a closed target union `{kind:source,path}` versus `{kind:unresolved,rawPath}` (or separate declaration observation) is necessary. Do not fabricate C:/outside.rs as a canonical repo locator.

State finalization follows complete per-crate admission, after manifest invalidation/prefix conflict discovered; earlier root/leaf temporary admissions become denied. Reason choice is stable documented precedence, not whichever branch happened last. Root/leaf observations remain physical authored facts even when admitted context/target empty.

Schema-owned union should also cover malformed manifest with unknown root (manifest observation, no invented crateRoot) before attempting admitted/denied/unmounted consumer distinction. Existing five rows all have readable valid manifest TOML and root candidate, so they do not prove this missing-origin shape. Add its closed route case in the separate consumer10 matrix, together with true orphan. Strict inventory adapter must receive participation and invalid-manifest observations rather than equating no context with orphan.
