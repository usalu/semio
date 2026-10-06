# Printed Package Whole-Row Source Projection Audit

Current source owner SHA 
B7EA3EBC85E8BFE73D3AE357250C511B534845E028095798AF45220223E038CB
.

Whole-row reader explicitly selects ApiScopeSource-bound owner tables first, admits only complete type/EN/de rows, rejects conflicting same-name explicit contracts, then fills missing family-local/inherited keys without overriding the explicit owner. Each projected family carries its actual scope. Inheritance traversal follows actual declaration forwards/helper installs and family renderer reached paths, excluding unbound printed references.

Grouped-row concern investigated and cleared: owner merge skips overlapping rows, but current parser1078 emits names:[name] singleton records for every grouped first-cell key. Therefore the suggested multi-name missing-key loss is not a present source omission. Catalogue plans explicit missing-name projection to preserve this invariant; no actual RED is claimed here.

Current region projection still begins with source comment-derived entry.keys and filters by actual accepted namespace keys (region function379–401, projection434–449). It does not yet append un-commented declared keys in the inspected revision. Extending actual declaration keys is source-owned only when the same physical scope/forward graph and complete printed contract rows are used; catalogue read-only18542 owns the bounded declaration-gap inventory. This audit does not infer undocumented controls from same spelling or global tables.
