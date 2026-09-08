# Test Layout Verification Plan

## Structural Acceptance

The policy must fail for legacy test suffix/prefix filenames, unadorned or incomplete test directories, implementation bodies embedded in production files or command scripts, and authored tests owned by delivery/package scopes. Canonical Rust external module wiring and minimal TypeScript private-dependency wiring are allowed; actual test bodies must be in canonical case files. Fixture strings that illustrate invalid code are data, not executable test declarations.

## Migration Preservation

Check each moved/extracted body against its pre-move contents or test declaration count. Rebase module imports, mock targets, relative fixture reads, Cargo declarations, source `#[path]` targets, script runner arguments and test discovery globs. Do not attribute unrelated concurrent edits to this migration.

## Runtime Checks

- Policy cases through the repo test domain with language-neutral inputs and independent installed oracle validation.
- Renderer named and element tests through `@semio-tech/framework-renderer-react:test` / applicable level, with no stale file-name selection.
- Remaining TS suites through their owning Nx targets, including `os-hub-ts:test` and native JS node tests where applicable.
- Rust syntax/module path checks over all extracted files; owning representative crates through existing Nx test targets. Any pre-existing compile errors must be named from observed output and distinguished from move-related errors.
- Go original package tests through canonical overlay registration, preserving private access. An isolated proof already passed; repository runtime execution is still required.
- Python parse/discovery and safe local test execution. Do not run ad-hoc research scripts that would mutate external services merely to prove paths.
- Final independent Terra scans for residual legacy file paths, inline declarations, delivery-scoped cases, broken imports/module paths, and stale runner references.

## Records and Cleanup

Each worker retains a Markdown report and an exact list of changed files. Generated command outputs live only in the ticket generated folder and are removed after findings are captured. Keep research reports and authored fixture inputs; do not retain migration scripts. Close the existing repo ticket and mark the app goal complete only when all required work is done.
