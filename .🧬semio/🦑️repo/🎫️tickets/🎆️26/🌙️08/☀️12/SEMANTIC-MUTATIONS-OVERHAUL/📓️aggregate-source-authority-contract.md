# Aggregate Source Authority Contract

## Required Authority

An aggregate declaration is owned by the mutation collection itself, not by one child leaf. Its compiler-resolved source must be the taxonomy's canonical Rust primary directly inside the taxonomy's mutation collection directory. The resulting mutation root is that immediate parent. A leaf primary, an ancestor artifact file, a historical primary name, or an arbitrary file inside the collection cannot establish aggregate authority.

FND-AGGREGATE-AUTHORITY-14 adds the private resolver in both existing derive mirrors. It reuses the already verified raw compiler-path normalization, pre-I/O exclusion, no-follow traversal, exact workspace markers, manifest taxonomy locator, and taxonomy vocabulary. Share workspace/taxonomy/filename parsing between leaf and aggregate authority rather than maintaining two conflicting implementations. The common token calculation accepts the safely resolved workspace root and taxonomy path and preserves the frozen domain-separated SHA-256 input exactly.

The aggregate authority facts are physical workspace root, normalized actual source path, normalized mutation root, taxonomy path, and the taxonomy-derived Rust/descriptor filenames. The following aggregate expansion will emit the lower `MutationLeafSourceScope` from these facts; this packet does not activate the mandatory aggregate behavior/registry cutover or invent a second public trait.

The resolver does not inspect sibling implementation contents, infer a variant roster, or accept user-provided source/owner overrides. Existing `Mutations` behavior is left unchanged until the complete mandatory consumer transaction. Metadata-only dependencies for the eventual aggregate are the exact two workspace markers and taxonomy, not hidden leaf implementation includes.

## Required Verification

Schema-first neutral cases must cover direct canonical aggregate acceptance; valid compiler-relative and parent-mounted paths; rejection of leaf-primary, historical-name, outside-root and unsafe/excluded locations; exact nested workspace-marker behavior; safe canonical filename changes supplied through taxonomy; and symlink rejection before normalization. Compare valid aggregate and leaf authority within one workspace: root, taxonomy, canonical names, and workspace token must agree. Identical relative layouts in different physical workspaces must have different tokens. Never create or access an actual compose directory to test rejection; use an unmaterialized forbidden path.

Retain all filesystem fixtures and logs inside the existing ticket. The real derive suite must rerun, including previously accepted private source/descriptor/public-derive tests. Independent coordinator compiler replay remains required before accepting this shared source refactor.
