# Gitlink Source Admission Contract

## Closed Observation

The proposed canonical observation requires `repositoryBoundary: "gitlink" | null`. Existing source spelling, physical kind/mode, explicit-directory flag, index identities, origins, and generator matches remain in their current fields. There is no duplicate path/object identity and no caller-supplied boundary list in the public projector input.

A non-null tag requires exactly one deduplicated stage-zero `160000` index entry, tracked provenance, consistent safe observations, and either an actual `directory/040000/true` or actual `absent/null/false` tuple. Absence keeps the existing nonblocking `tracked-path-absent` diagnostic; it does not become a directory. A file, symlink, nonregular node, unsafe/unobserved target, contradictory index identity, or nonzero stage cannot earn the tag. The existing artificial file/160000 negative stays rejected.

The tag means index-owned terminal repository boundary, not authored directory or completed mutation coverage. Supplied descendants below an index boundary are contradictory source admission and must be rejected rather than silently filtered. Boundary discovery uses all supplied index facts before scope filtering.

## IO Ordering

The existing full stage-aware Git-index reader will supply conservative no-descent fence paths before any scoped walk. Any index160000 identity fences descent even when conflicts make final admission invalid. This is a projection of the same index rows, not a separate roots policy or path allowlist.

Lexical and root-ancestry checks run first. The current index reader's unused taxonomy argument can be removed so the reader runs before taxonomy-file observation. The private prepared value carries that captured index and its fences to the collector; the collector must not repeat enumeration as a different authority. Taxonomy/cancellation inputs at or below a fence and scope/ticket/output roots strictly below one must reject before nested probing. The walker may observe an exact fence root but must not enumerate its children. All Compose and opaque-path checks remain intact.

This ordering protects against the captured index facts. It is not an atomic guarantee against arbitrary concurrent index replacement; any observed drift must remain an unsuccessful capture, not a complete census claim.

## Consumers and Explicit Limits

Root mutation source-index and direct-folder discovery retain the admission record but do not count a repository boundary as an authored file, directory, mutation root, or direct leaf. Structural accounting must not accidentally reintroduce a tagged direct observation as a valid leaf.

For this packet, full normalization inventory will reject a retained boundary immediately after admission and before authored classification. It will not omit a mount and then normalize an ancestor. A complete normalization feature would also need carried boundary/digest authority and operation-overlap checks across recursive digests, embedded-root staging, generator trees, destinations, and pruning.

That inventory guard alone is not stale-plan/apply safety: existing apply/recovery can probe or recursively digest planned directories before fresh inventory. Those paths remain outside this packet's acceptance and require separately coordinated guards. No normalization/apply readiness is inferred.

## Planned Source Footprint

1. Canonical normalization source-admission schema: required observation tag and closed Gitlink tuple constraints; canonical neutral expected records receive explicit null or Gitlink tags.
2. N source-admission types, projector, existing index reader/prepare/collector/walker; full inventory's pre-classification refusal only. Parser/reference-coordinate code remains untouched.
3. Root source-files, structural-directory, mutation-root and direct-child accounting joins only. No toolJobDispositions changes.
4. Canonical and ticket neutral tests plus their existing harness dependency joins. No real nested repository traversal, shared index change, source move, cache cleanup, or global census rerun until the packet is coherent.

The source is not mounted by this document. Neutral projector and IO RED tests are being prepared independently before the production change.
