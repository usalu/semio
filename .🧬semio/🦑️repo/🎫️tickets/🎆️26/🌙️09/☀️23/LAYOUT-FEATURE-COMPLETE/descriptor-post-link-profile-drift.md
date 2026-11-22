# Post-Link Descriptor Profile Drift

The `wiLotj4` guest built and staged the Layout component. Its post-link `describe` stage was first to refuse with “Cargo profile policy Source differs from running canonical policy.” The retry log records one staged component and zero running compiler units immediately after that failure. They were followed by cancellations from the same closed Original.

The failing reader is `withCompilerStoreDescriptorEmitterV1`. The component driver calls it both in preflight and again after linking. It requested live Cargo consumption with `producer: "descriptor-emitter"` and `operation: "build"` to open a completed tool publication. This invoked Cargo configuration admission and compared the now-current profile constraints to the module's process-loaded digest. No emitter compile was requested by that reader. The actual emitter build and publication entry still need normal Cargo admission.

The staged fix uses the existing product artifact reader and a live compiler store. The compiler store authenticates the Original, the selected build mode and the Cargo home recorded in the sealed compiler index. The reader holds exact artifact custody through use. Success still requires the original receipt's successful, uncancelled emitter Cargo run, manifest, exact executable digest, complete retained inventory, and physical-byte rechecks on both sides of use. That reader only opens the publication's retained files. The original registry tree and current Cargo profile are outside that read.

The component driver keeps its own admitted Cargo source/configuration rechecks and uses actual component provenance to bind the published model. This patch removes stale profile admission for an already-sealed emitter. A source change in the active component build can still trigger the original strict rejection. The `wiLotj4` run stopped before this later condition could establish whether its own pass also needed readmission. Allowing unrelated constraints edits through an active pass would require an authored closure-aware profile provenance rule. It would have to prove unchanged effective selectors for the actual target and host Cargo unit graph while retaining the original receipt. Whole-file comparisons remain in the compiler ingress.

## Verification

- The new regression against the checked-out source failed on the same stale profile refusal as the production run. The staged source passed the case while holding artifact custody around the writer. SQLite independently checked the lock order.
- The staged descriptor suite, existing sealed-byte suite, and existing Cargo profile recheck suite passed 10 tests and 121 assertions. The unchanged Cargo tests continued to reject changes to consumed physical Source and to the process-loaded profile policy.
- The staged two-file `🗑️generated/descriptor-sealed-profile-drift.patch` passed the read-only exact patch and whitespace check.
- Production sources, running Cargo plans and the listener were untouched. The currently running guest has not exercised this candidate.

## Applied Verification

The two-file patch is applied. The applied-source request, sealed-executable and Cargo-profile recheck suites passed ten tests and 121 assertions. The current guest owner was launched with a fresh matching profile before this source change and remains its only canonical owner.
