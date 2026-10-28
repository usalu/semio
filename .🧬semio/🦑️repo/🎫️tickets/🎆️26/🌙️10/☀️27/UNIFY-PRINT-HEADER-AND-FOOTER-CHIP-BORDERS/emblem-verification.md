# Semio Emblem Inside the Project Chip

The semio emblem and project label now share one chip border. The Fingerübung header no longer has a separately framed emblem. Both contents retain their vertical centering, and the existing shared header border closes the combined chip.

The generic project-chip renderer also handles other titles. An empty project label leaves one emblem-only chip without a label gap. Footer composition is unchanged.

## Verification

- Added language-neutral English, German and empty-label cases.
- The regression check failed before implementation because the project chip did not contain the emblem.
- The focused suite now passes: one chip surrounds both contents, the standalone emblem chip is absent, and the emblem-only fallback remains.
- Combined-chip width arithmetic agrees with the independent mathjs implementation for each neutral vector.
- Previous shared-border and LUH asset checks still pass.
- Whitespace diff checks pass.
- Full quick pipeline cannot start because @semio-tech/framework-transient remains unresolved in the UI theme module. Rendered-PDF verification remains unavailable.

Reproduce the focused checks through the retained verify.ts using repo tools exec bun. No generated ticket outputs remain.
