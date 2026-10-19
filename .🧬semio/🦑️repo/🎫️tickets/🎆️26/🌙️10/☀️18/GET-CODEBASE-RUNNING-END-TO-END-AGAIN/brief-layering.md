# Brief: Layering Executor
Read `brief-executor.md`, `audit-layering.md` (your WP), `result-lay0.md` (policy fixes, depcruise command), `result-lay6.md` (method and exports-spec format).
- Fix direction violations by dependency inversion (ports in the lower layer, implementations injected by the higher layer, schema-first where data crosses) or by moving code to the layer that owns it; fix cycles by leaf extraction.
- Deep cross-package relative imports: switch to the package's public entry where an `exports` entry already exists; otherwise append the needed entry to `C/<prefix>-tools/exports-needed.json` (lay6 format) and leave the import for the batch. NEVER edit package.json/bun.lock (freeze); never add compat barrels/re-exports.
- Nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`. No cargo.
- Verify with tsc on touched packages and the affected bun/vitest suites; re-run depcruise scoped to your WP and report rule totals before/after.
- Other layering executors run concurrently on other WPs; stay inside your source area.
Report `C/result-<prefix>.md`.
