# Course Workflow Preservation, Repeat 695

The entire current Quiz workflow delta is exactly two install-command replacements from bun install to bun repo dependencies sync --all (verified against immutable base). Incoming Course workflow already includes dependency-sync commands in all three jobs, with authored preflight/capability/storage admission and scoped sudo environment. Therefore exact incoming Course source preserves the current dependency-installation change and intentionally replaces old unowned cache/test steps. No old workflow is revived.

- `.github/workflows/architecture-course.yml`: exact incoming source; current dependency-sync behavior retained in all three jobs.
- `.github/workflows/architecture-quiz.yml`: retired after source provenance and destination-byte verification.

No workflow, build, tests or runtime was executed.
