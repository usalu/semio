# Adaptive Base64 Review

The adaptive cursor now accepts an actual maximum work budget, selects quartet-aligned chunks, accounts nonfinal validation's second byte visit and reports actual work, reads, copies and capacity. The inference and node-graph callers have been migrated to its parameterized signature. Inspected tests condition their inertness assertions on a zero selected demand rather than assuming every grant below the largest quote is inert. This is source review, not native verification.

A constructor-detected invalid input length or range is retained in the cursor with next_work_demand returning zero. Inference receiving interprets zero demand as no available work and returns pending before calling the cursor; node-graph scene advance similarly breaks when stage 10 has zero demand. Both can therefore stall indefinitely on such malformed input. Controls was notified to distinguish a frozen initial refusal from an undersized live grant and publish it without fabricating work. The synchronous one-shot decoder calls step even with zero demand and observes its refusal.

The inference change returns codec failure inside admit_original_construction, which permits the helper to reconcile its actual physical progress and semantic failure together rather than returning a previously posted progress value afterward. No native test was run.

Follow-up: the cursor now exposes the already-owned refusal by reference. Inference checks it before selecting a work quantum, and node-graph reports it before treating zero demand as pending. The existing malformed node-graph corpus includes invalid-length inputs, and a new Base64 law asserts that this refusal is visible under a zero-work grant. These changes are source-authored and not yet native-verified.
