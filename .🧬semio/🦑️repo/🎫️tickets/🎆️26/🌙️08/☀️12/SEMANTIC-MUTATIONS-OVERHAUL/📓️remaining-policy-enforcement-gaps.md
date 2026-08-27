# Remaining Structural Policy Enforcement Gaps

Coordinator read-only follow-up against the current `DirectMutationPolicies` region in root `📜️script.ts`. These findings are open implementation work, not a final audit or a production mutation result. Real `compose/**` was not accessed.

1. `mutation/shared-helper-purity` currently only emits when the aggregate contains `NoMutation` or `SetSnapshot`. It does not actually inspect resolved shared-helper targets for concrete mutation identities, wire tags, default branches or validation. The new consumer graph must supply real helper edges before this policy can be exhaustive.
2. `mutation/test-presence` currently accepts existence of a `🧪️tests` directory or a detected test module. An empty directory or empty test module can satisfy it; required executable mutation/codec/algebraic laws are not established by that condition.
3. Root reachability uses substring presence of a module or variant name. It does not prove that a public path resolves to the correct direct payload file. The pending source-module resolver and derive contract are required to close this gap.
4. Non-Rust language/schema/catalog parity still uses substring presence for kind or variant names. Comments, duplicate/orphan union members and empty wrappers can pass portions of these checks. Exact parsed roster correspondence remains required for the final zero-violation gate.
5. Aggregate purity currently uses payload/match/method/include facts but does not directly distinguish structural correspondence tests from mutation-specific behavior tests. TXT's concrete tests were relocated on manual review; the source guard must eventually enforce that ownership systematically.

Registering all17 policy names at high severity does not mean all17 architectural obligations are completely enforced. Existing focused tests establish only their exercised contracts. These gaps must remain visible in the work ledger until independently tested and closed.
