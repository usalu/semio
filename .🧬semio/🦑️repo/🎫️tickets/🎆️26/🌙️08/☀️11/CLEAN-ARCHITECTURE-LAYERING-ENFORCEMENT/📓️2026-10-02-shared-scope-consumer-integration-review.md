# Shared Scope Consumer Integration Review

Read-only actual source inspection; no jobs/typecheck/compiler. Root provider replay remains pending; High reports binding adapter 59 laws/349 assertions GREEN, not independently rerun here. Paths relative to Repo library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Actual remaining issues

**Sibling poisoning in finite normalization:** `🧹️normalization/🟦️.ts:4570` returns the entire result when *any* module fact is unresolved, before :4571 checks its lexical scope and :4573 checks the required context module prefix. An unknown dormant sibling module, even inside an unrelated inline subtree, therefore denies a valid private source chain. Move unresolved refusal after exact sourceScope and required module-prefix selection, or rely on the owned graph denied-prefix proof for that required chain. Root-scope unknown metadata still properly denies all descendants through the shared selector. Small positive: known mounted component plus dormant unknown rewritten sibling; valid component must retain finite manifest targets while sibling key is refused. Existing earlier macro-trust scan can independently refuse some examples, so a focused extracted/helper fixture must distinguish the new erroneous check from broader trust policy.

**Direct fallback still restores graph denial:** `🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts:179–192` improves source lexical proof and exact authored declaration cardinality, but still directly returns physical path after graph resolution fails. It never checks the source's admitted graph contexts, the target's readable scope or unresolvedTargets of the selected key. Example: valid root source with a single known path module plus a separate absolute mount revoking manifest; bare `sealed::Mutation` returns the leaf through fallback after graph contexts are removed. Unreadable leaf callback/captured source can similarly leave filename membership sufficient. This is independent of the now-fixed count check for known+unresolved duplicate declarations. Require admitted context/key target authority before this path confers Rust module ownership; source-only scope proof cannot reconstruct revoked manifest or unavailable target bytes.

The graph resolver's :163 denied-prefix test now closes the earlier shortening problem, but an empty result does not distinguish denied from genuinely unmodeled; the caller fallback above still discards that distinction.

## Integration inspected

Authoring mutation-tree :187–188 requires root scope; :194–200 counts all canonical named alternatives before accepting and checks unresolved. Structural reachability :328 root/:344 leaf/:352 child selectors are correctly placed; child lookup :347–348 counts unknown alternatives before filtering, preserving ambiguity. Unrelated unresolved child does not globally poison structural lookup.

Finite extraction harness `🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts:163–178` now declares required unresolvedTargets and full graphFacts modules/uses/includes/scopes plus scope proof; injected runtime dependency :130 includes selector. Writable harness import :10 and injection :38 also include selector. No missing required graphFacts field was identified in these inspected declarations; this is source review, not typecheck confirmation.

Binding adapter `🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts:13` retains graphFacts on RustBindingFacts and :211/:228 selects current/source-root scopes. No independent fallback issue identified in that adapter during this review.
