# Print and Mit Bestand Integration

Status: integration and validation complete.

The `temp/merge/print` and `temp/merge/mit-bestand` snapshots are integrated into the shared `🐙ueli/⛳wip` working tree. Existing application architecture, semantic paths, generated design tokens and font provisioning remain the destination authority. No commit, checkout, stash, worktree or other modifying Git operation was used.

## Integrated Work

- Print layout/navigation/table updates, graph and tree packages, the 80-section visualization gallery, public API gallery and 1,966-entry taxonomy.
- Interim, research and compact reports, their bibliography and assets. Common appendices have one source under the report family. Two missing scaling bibliography entries were copied from the supplied research bibliography, and every literal citation is now checked.
- Actor-network validation and rendering with 798 nodes, 444 edges and 14 programs. Rebuild/write guards account for edits during long rendering.
- Compiler-safe staging preserves canonical semantic names while compiling TeX-compatible paths. New commands are registered through Bun script routers, Nx and both VS Code launch configurations.
- Incoming historical research is preserved byte-for-byte in the existing Neo4j intake workflow: 687 files pending domain review. No graph promotion is claimed.
- The current demonstrator/presentation architecture was retained after comparison. Existing logo assets already match; the incoming `Überblick` display label is restored.

## Validation

The detailed checks and observed runtime output are in [Validation](📓️validation.md). Uncached print/report contract tests and the complete 12-PDF standard-template suite pass. All three reports build: interim 192 pages, research 205 pages, compact six pages. The repaired interim bibliography has no undefined references. Both visualization API themes pass PDF.js text checks and reproduce identical normalized hashes across separate builds. The 16-page distribution gallery builds in both themes. The scoped whitespace check passes.

## Review Records

- [Source decisions](📓️source-manifest.md)
- [Integration audit](📓️merge-audit.md)
- [Research intake provenance and hashes](📓️research-intake.md)
- [Presentation comparison](📓️presentation-comparison.md)
- [Actor rendering review](📓️actor-render-review.md)

## Scope and Limits

The incoming report draft prose and existing TeX layout warnings are preserved. PDF compilation is checked on native macOS ARM64; native Windows/Linux and devcontainer execution were not run in this session. Cross-platform staging uses native path APIs and grapheme-aware filename handling. Exhaustive compilation of all 162 visualization PDFs is available as an Nx/VS Code command; this merge validates full static coverage and representative distribution/API rendering.

The result is an uncommitted working-tree integration, consistent with the repository prohibition on modifying Git while others edit concurrently. Concurrent semantic renames and unrelated changes were retained.

## Generated Output Cleanup

Ticket-owned compiler outputs, PDF previews, logs, temporary inventories, baseline copies and isolated Nx workspace data are removed at completion. Source snapshots, imported source assets and all Markdown audits remain.
