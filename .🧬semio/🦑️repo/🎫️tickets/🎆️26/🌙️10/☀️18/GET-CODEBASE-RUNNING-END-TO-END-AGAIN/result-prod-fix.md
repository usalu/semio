# Result: prod-fix (2026-10-19)

## Changes
- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts` (L58): removed `SEMIO_RENDERER: "react"` from the `buildViteArtifact` environment. Decision from code: play uses its own Vite config (`🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`), which never reads `SEMIO_RENDERER`; `git grep SEMIO_RENDERER -- 🏢️semio-tech` has no other hit. The variable stays where a real renderer switch exists (framework os-dev Vite config, `process.env.SEMIO_RENDERER ?? "react"`, and the demonstrator, which builds with that config). The env-hash allowlist in `⚡️caching/🔣️policy.json` is untouched (shared by other products).
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json`: 4 scripts `bun nx run @teaching/architecture-course:X` -> `bun --cwd "../../../../.." repo run @teaching/architecture-course:X` (build, test, test:e2e, typecheck; all four targets exist in its `📋️project.json`).
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/package.json`: 2 scripts -> `bun --cwd "../../../../../../.." repo run @semio-tech/fem-3d-rs:verify-fem3d-numerical-child-{native,source}` (both targets exist). Script fields only, no dependency edits.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🦀️.rs`: USAGE text `workspace:dev -- storybook` -> `workspace:dev-storybook` (real root target; dashboard README already uses it).
- Deleted untracked/ignored `🎓️teaching/🏛️architecture/❓️quiz/` (41 MB: `dist/`, `node_modules/` incl. one symlink into `.cache`, `.vite*`). `git ls-files` for it returned nothing; the symlink target in `.cache` is intact.
- `workspace:start` / `workspace:dev-mcp-engine`: no change needed. `git grep` over all tracked files outside tickets and `.cursor` (docs, devcontainer, CI, dashboard Rust, oracle) finds zero references. Both only existed as `.vscode` launch rows; `bd9e4e02bd0^:.vscode/launch.json` already has "▶️start" -> `bun repo run os-hub:local-hub-owner` (target exists in `🌎️hub/📦️packages/🦀️rust/📋️project.json`) and no `dev-mcp-engine` row. Nothing dangling remains, so no target defined and nothing removed.

## Verification
- `bun build --target=bun <play site 📜️script.ts> --outdir <ticket generated>`: OK (1967 modules bundled), output removed.
- `bun -e JSON.parse` on both edited package.json: OK. Resolved `--cwd` relative paths from each dir: both resolve to `/Users/ueli/Documents/semio`.
- `git grep "bun nx run" -- '*package.json'` (excluding tickets): 0 hits. Remaining hits are test fixtures (`🧭️authority/🔣️.json`, `🔧️build-surfaces/🔣️.json`, `🧫️frozen-seal-ledger/🔣️.json`, an ooxml README fixture): illustrative data, left alone.
- Dashboard Rust change: string literal only; `cargo` not run (rule). The USAGE text is not referenced by any other file (`git grep "routes of the root script"`).
- `bun test`: not run (no test covers these edits).

## Open decisions
- Conflict: AGENTS.md says package.json MUST call `nx`; the repo's neutral layer (other ~700 scripts, Bazel parity) uses `bun --cwd <root> repo run X`. I used the neutral form. User must decide whether to amend AGENTS.md (agents may not edit it).
- `.vscode/` is gone (user decision pending): "▶️start" row is not registered anywhere now.
- Owner registry row for `@teaching/architecture-quiz` (`📋️build-owners.json`, WP-6) is still stale; not in my scope.
