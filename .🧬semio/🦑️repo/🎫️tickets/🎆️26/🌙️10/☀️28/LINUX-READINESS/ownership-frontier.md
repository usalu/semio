# Dependency Lock Ownership Frontier Explosion

Read-only diagnosis after coordinator canceled the root scoped lock run at more than 2.6 million reported frontier operations. No canceled run was restarted by this audit.

## Proven Code Paths

The physical owner is library/🗂️workspaces/🟦️bun/🟨️.cjs. Its opaque set at12 excludes node_modules, generated, target, dist, build and other output trees. walkPlan97 excludes all dot-prefixed children, including .cache and .🧬semio ticket trees. Thus scanning tickets or dependency output is not supported by the source evidence.

The recursive source walk is broad: candidatesPlan107 derives the fixed prefix, then112 invokes walkPlan recursively for its complete subtree, filtering matching member patterns only afterwards at117. Even fixed-depth patterns walk deeper source trees than their selector can match. Inventories are cached within one owner, so shared prefixes are not independently rescanned in that computation.

The dependency lock caller at library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:669 unconditionally requests withPublishedBunWorkspaceOwnershipV1 with kind all, even for requested scope dot. Only after computing ownership and dependency input plans does it derive requested lock scopes. Therefore scope selection does not constrain ownership observation work.

Its fence675 calls custody.recheck. Physical replayCurrentMembershipV1 at322–335 performs lstat/state comparison of every observed path and awaits control.advance for every path. It then repeats a full synchronous verification at341–350 and rereads manifests. Every custody fence repeats this whole retained frontier. Publication validation and currentFence also duplicate membership replay (cjs447 and403, final468–469). The million-count progress can therefore be repeated validation of the same finite source frontier rather than millions of distinct filesystem paths.

## Minimal Regression Scenario

Create one root installation scope, a second independent top-level installation scope, and many nested non-package source directories in the second scope. Run the lock ownership path selecting scope dot with a recording control. Count unique observed paths, total frontier advances and complete recheck invocations separately. Assert unrelated scope traversal is absent when the contract permits exact scope admission; otherwise assert retained all-scope proof is revalidated in bounded batches and does not generate one expensive caller continuation per unchanged source path on every fence.

Add a fixed-depth member selector such as packages/* with a deeply nested source-only subtree under packages/a. Membership results must agree with an independent glob/library oracle while unnecessary descendant traversal is bounded. Add output and dot-directory trees containing fake package manifests and assert no reads occur inside them. Preserve mutation detection by editing an admitted source manifest, adding/removing a matching package, and changing a directory listing during the final continuation.

## Recommended Repair Boundary

Do not fix this by dropping freshness fences, ignoring concurrent edits, or adding arbitrary output exclusions already covered. Establish selection semantics at the caller, avoid repeated redundant full frontier passes, and retain progress/cancellation through bounded batches. Instrument full replay count and distinct paths inside ticket generated logs, not production permanent debug output. Root was sent exact owner and call-site references.
