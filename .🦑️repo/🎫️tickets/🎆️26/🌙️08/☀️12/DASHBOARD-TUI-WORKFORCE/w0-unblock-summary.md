# W0 Unblock Summary

## Stale paths
- Fixed all `⚡️implementations` → `📦️packages` in `.vscode/� comboslaunch.seed.jsonc` and regenerated `.vscode/launch.json`.
- Fixed `.vscode/settings.json` eslint workingDirectory for vscode extension package.

## Nx project graph
- Root cause: nx native file walker emits lossy U+FFFD paths for some emoji segments; built-in `package-json` plugin + emoji `📋️project.json` plugin then register the same name at two roots.
- Mitigations:
  1. Emoji project plugin skips U+FFFD config paths and NFC-normalizes roots; dedupes by name across batches.
  2. `@semio-tech/infinite-world-r3f` package.json sets `nx.name` to `@semio-tech/infinite-world-r3f-pkg` so the lossy package-json registration cannot collide with the real project.
  3. `@nx/js` `analyzePackageJson` / `analyzeSourceFiles` / `analyzeLockfile` set to `false` so dependency edges do not crash on FFFD file-map entries.
- `bunx nx show projects` now lists ~179 projects including `@semio-tech/repo-cli-rs` and `@semio-tech/infinite-world-r3f`.
