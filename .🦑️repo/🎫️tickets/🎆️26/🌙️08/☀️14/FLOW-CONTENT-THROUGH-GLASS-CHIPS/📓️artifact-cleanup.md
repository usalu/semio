# Compiled Artifact Cleanup

## Finding

The ticket-local Cargo target at `🎯️target-wgpu` contained 14,885 compiled files and occupied 8.9 GB. Its `.future-incompat-report.json`, `.rustc_info.json`, and `CACHEDIR.TAG` had also entered the shared Git index. The ticket also contained a 5.2 MB `🎭️storybook-ui` output with copied assets, generated icons, `node_modules`, and `.DS_Store`. A follow-up repository-wide untracked-file audit found a third leak: `🧑‍💻️dev/📺️renderer-modules` contained 4,511 generated files and occupied 1.7 GB, including duplicated assets, `.stage`, plugin bundles, JavaScript, and WASM. The WGPU Trunk build and Vite mount both deliberately targeted that checked-out source directory.

## Resolution

- Moved the exact `FLOW-CONTENT-THROUGH-GLASS-CHIPS/🎯️target-wgpu` Cargo output and `🎭️storybook-ui` Storybook output to the recoverable macOS Trash.
- Added root `🎯️target/`, `🎯️target-*/`, and `🎭️storybook-*/` directory rules so ticket-local Cargo and Storybook build roots cannot be re-added while legitimate `🎯️targets/` source taxonomies remain trackable.
- Moved `🧑‍💻️dev/📺️renderer-modules` to the recoverable macOS Trash and added the explicit `**/📺️renderer-modules/` fallback ignore rule.
- Relocated the WGPU Trunk output and Vite `/renderer-modules` filesystem mount to `.🦑️repo/⚡️cache/📺️renderer-modules`, the repository's existing ignored cache root. Future WGPU dev builds no longer populate source directories.
- Preserved all human-readable ticket reports, validation logs, JSON results, and runtime screenshots.
- Audited the task's changed files for compiled extensions and build-output directories after removal.

## Shared Index Note

The three Cargo metadata files had been staged by another concurrent workflow before this cleanup. Repository policy forbids modifying Git commands in the shared worktree, so this task does not rewrite the shared index. Their working-tree deletion is intentional; the next authorized whole-tree staging operation must record those deletions rather than commit the earlier staged snapshots.

## Validation

- The ticket now contains only the ticket manifest, Markdown reports, text/JSON validation evidence, and the runtime screenshot.
- `git check-ignore` resolves `🎯️target/`, `🎯️target-*/`, and `🎭️storybook-*/` probes to the new root rules.
- `git check-ignore` resolves a legacy `🧑‍💻️dev/📺️renderer-modules/` probe to the new fallback rule, while the active renderer output resolves beneath the existing `.🦑️repo/⚡️cache` rule.
- A probe beneath the legitimate `🎯️targets/⚛️react/` source taxonomy remains trackable.
- The task-scoped diff passes `git diff --check`; the only repository-wide whitespace diagnostic is an unrelated concurrent prompt edit outside this ticket.
