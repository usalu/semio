> Final refinement: the all-member manifest read described in the early proposal is superseded. Bun locates a candidate; only captured root membership and the selected nofollow member manifest authorize it. Unrelated member manifests are neither read nor counted. See [implementation](captured-workspace-exports-implementation.md).

# Captured Workspace Exports Proposal

The existing authorizer scanner refuses every non-relative first-party import. The immutable Bun compiler has the same relative-only restriction. Current Source uses `@semio-tech/schema-validator-ts`; it is a private concrete root-workspace member with explicit Source exports. Externalizing it would permit live resolution and is excluded.

## Selected Boundary

Extend the existing authorizer manifest with a mandatory closed package resolution projection. Keep source bytes and every admitted package manifest in the existing captured file census, digest, per-file4MiB, aggregate16MiB,256-file and snapshot bounds. There is no runtime dependency or new Native/SDK/store entry.

A nearby pure first-party resolver consumes captured root package.json and concrete member package.json bytes, validates exact published membership, unique names/private:true and explicit export targets, then resolves only supported Source exports. The current root publishes 99 concrete paths. The canonical workspace owner supplies discovery/publication, but no existing function resolves immutable captured package exports; it must not be called during compilation because it observes live filesystem state. Root workspaces are the explicit input authority; globs and ambient node_modules/symlink lookup are refused. Referenced entries stay within the package and workspace, use explicit extensions, and are captured as ordinary Source.

Root metadata and the package manifests needed to authenticate a complete name index enter the same bounded capture. A capacity refusal is retained rather than enlarging bounds or hiding manifests. Actual count/bytes must be measured after implementation. Duplicate package names, false private flags, missing exports, traversal, wildcard targets, unsupported conditional ambiguity and uncaptured targets refuse.

The compiler reconstructs the exact map from captured manifest bytes and compares it with the manifest projection before Bun.build. Its onResolve consults that map and the captured file map only. Altering/removing live root/package manifests or node_modules after capture cannot affect compilation. The immutable snapshot must still refuse forged mapping, metadata bytes, membership or digest.

## Planned Footprint

- Existing authorizers/🟦️.ts and authorizers/🧬️schema/🔣️.json: capture metadata, exact map and its digest.
- Existing authorizers/📦️snapshot/🟦️.ts: reconstruct and validate map from captured bytes, resolve only into its virtual namespace.
- Nearby authorizers/📦️workspace pure resolver/schema/portable corpus/tests; imported by the permanent existing authorizer test entry.
- Ticket Source-only harness/report and inventory. No runtime/tool inputs are copied into the ticket.

## TDD And Qualification

First run the existing actual capture against a generated private workspace with a private named package, retaining the present dependency refusal as RED. Portable schema/corpus and independent SQLite cover membership/name/export decisions; actual Bun virtual compilation plus independent esbuild/Node oracle compare exports after live package edits/removal. All fixtures live under this ticket Generated and use genuine carried Original/phase with owned child/progress/cancellation. Tests that need a canonical Tools/store owner are not executed. Focused strict and Low review precede production hold; Root owns fresh SDK role capture and any eventual publication.

This is a proposal and Source task scope, not current SDK/public interface acceptance. No package alias is mass reverted and no external allowlist or capacity is expanded.
