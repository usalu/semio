# Wave 0 Baseline

## Governance evidence

The checked-in repo MCP stdio server was started from its Go module. `resources/read repo://goals` returned the open `AI-optimized Repo` goal, whose repository identity is `🎯aioptimizedrepo`. No exact broader ticket existed. `ticket_open` created this ticket and GitHub issue 2557. The MCP LLM enum rejected `gpt-5.6-sol` because its current allowlist ends at earlier GPT identifiers, so ticket metadata records its newest accepted GPT value while the collaboration workers use the requested GPT-5.6 model families.

## Catalog and topology

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json` has 36 catalog artifacts.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` has an extra generic `🧬️schema` root, yielding 37 filesystem roots. It is not a catalog artifact.
- Existing records describe 32 standards as graduated or partly graduated, but graduation is not equivalent to the program's requested exhaustive public-revision/profile/codec closure.
- MP4, AVI, MP3, WAV, EPW, TSV, and HTML remain partial family scaffolds.
- The TypeScript package facade omits Semio and those seven partial families.
- Current policy contains hand-maintained allowlists/counts and explicitly lists the false `stdio/schema` shell.

## Framework gaps

- `ArtifactDeclaration` is centered on a single schema/document-codec declaration even though several artifacts own multiple standards.
- The native inference registry already rejects conflicting inference-service keys, but its executable seam is cold-only and the wire path rejects non-empty policy.
- The generic IO composer, subset-validator, format, and document-codec registries use mutable `HashMap` storage and have overwrite paths.
- Event-sourced artifact storage exists, but inference projection is externally triggered rather than a uniform revision/generation-aware projection path.
- Current GLTF work demonstrates schema/runtime depth but retains an aggregate geometric analysis implementation and has extensive concurrent taxonomy edits; Wave 0 treats every GLTF path as externally owned.

## STEP gaps

- Current external support is AP214-labeled Part 21 with CC1–CC6 heuristic validators, not an EXPRESS-compiled schema implementation.
- The physical parser drops comments/trivia and unknown header records, collapses useful spans, loses exact decimals through `f64`, and lacks strict end/trailing-input handling.
- `FILE_SCHEMA` does not robustly route an actual schema/profile; `.stp` is not registered equivalently to `.step`.
- Part 28/XML and other public physical representations are absent.
- The current geometry bridge is faceted and duplicates a stronger analytic STEP/BREP implementation inside the Semio BREP subset.
- The process3d generic STEP bridge converts incompatible pack envelopes.

## Dirty-tree boundary

At program open, the shared tree contains extensive in-progress GLTF additions, relocations, and deletions plus a modified stdio Rust glue file. These changes predate the umbrella ticket and are preserved. Wave 0 leases prohibit touching GLTF and assign each shared framework/root file to one exclusive writer.

## Baseline verification honesty

No fresh whole-workspace green claim is made. A prior direct full stdio library run was killed with exit 137, and earlier work recorded Cargo-lock contention. Barrier gates are therefore serialized and results are attributed only to commands actually run on the current combined tree.
