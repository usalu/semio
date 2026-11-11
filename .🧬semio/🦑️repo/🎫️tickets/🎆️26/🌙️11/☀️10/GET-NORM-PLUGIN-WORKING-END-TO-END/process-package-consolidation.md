# Process Package Consolidation

- Canonical: `🧰️framework/🔨️modules/🏃️process` (`@semio-tech/process`). Stale copy `🧰️framework/📦️packages/🟦️typescript/🏃️process` had no file absent from canonical, so nothing needed porting.
- Migrated ~470 files: `@semio-tech/framework/process/*` -> `@semio-tech/process/*` (all 13 used subpaths already exported canonically; no export added) and relative paths `📦️packages/🟦️typescript/🏃️process/` -> `🔨️modules/🏃️process/📦️packages/🟦️typescript/`. Repo-library `📚️library/.../🏃️process` is a different module and untouched.
- Removed all `./process/*` (and `./routing/invocation/process*`) exports from the framework package.json and deleted the stale directory.
- Verification: all relative targets exist; bun bundle resolves 5 sample entry points (os test config, renderer react script, procedural script, mcp test config, NodeGraph script); sample tests run.
- tsc on the framework package is blocked by unrelated syntax errors in `🖱️ui/📖️stories/🎭️navbar-example-select/🧪️.story.tsx`.
