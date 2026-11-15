# Launcher Go tests: root causes and fixes

Command (one at a time, from `/Users/Shared/semio`, `SEMIO_LANGUAGE=en`): `repo run @semio-tech/repo-launcher-go:test --skip-remote-cache`.

Run 1: exit 1, four failures (not three). Run 2 (after fixes 1-3): exit 1, one failure. Run 3 (after fix 4): exit 0, `ok github.com/usalu/semio/repo/launcher` (single Go package, 1.4 s; the whole suite, no individual test left failing).

None of the four is caused by the dangling-link/`namespaceDangling` change.

| Test | Root cause | Fix |
| --- | --- | --- |
| `TestWorkspaceLinksPublishNeverReplacesAnotherTarget` | Product bug: Unix `publishWorkspaceDirectoryLink` was `os.Rename`, which silently replaces an existing destination (Windows `MoveFileW` refuses). | `🐚️unix-host/🐹️.go`: publish reads the stage link and creates the destination with `os.Symlink` (atomic, `EEXIST` if occupied); the caller already removes the stage and treats a matching racer as success. |
| `TestRegistryShapeChangesBumpTheContract` | Commit 711 changed the registry schema `$schema` dialect from 2020-12 to draft-07; digest drifted from the pin (made at 707). Not related to the `namespaceDangling` text (schema only constrains message ids by pattern). A dialect URI does not change which registries are valid, so no contract bump. | Re-pinned `registryShape.schemaDigest` (same contract 1) in `🧫️fixtures/🔣️.json`. Digest verified independently with Python for both the old and new schema. |
| `TestRouteVectors` | Catalogue (`📚️library/🏗️build/⚙️operations/🔣️catalog.json`, commit 735) gained verb `scheduler`; the shared CLI fixture `buildVerbs` was not updated. | Added `scheduler` to `⌨️cli/🧫️fixtures/🚪️entrypoint/🔣️.json`. |
| `TestWorkspaceLinksColdImportAndConcurrentIdempotence` | Test bug: Bun colors a printed number with ANSI codes when `FORCE_COLOR` is inherited from the runner; output `\e[33m42\e[0m` != `42`. | Test script now prints `String(value)`. |

Files changed (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/`):
- `🚀️launcher/📦️packages/🐹️go/🐚️unix-host/🐹️.go`
- `🚀️launcher/🧫️fixtures/🔣️.json`
- `🚀️launcher/🧪️tests/📦️workspace-links/🐹️.go`
- `⌨️cli/🧫️fixtures/🚪️entrypoint/🔣️.json`

## Install

`🚀️launcher/📥️install/🐚️.sh` (no argument, no sudo, home only): exit 0, installed to `~/.local/share/semio/launcher/bin/repo` (documented default location). `~/.local/bin/repo` is NOT produced by this script: it is the full native `repo` command (18.6 MB, unchanged) installed by `repo run repo-mcp:install` per the README; overwriting it with the 6.5 MB launcher would downgrade it, so it was left untouched. The script itself reports it as "another repository frontend".

Checks: `repo --help` exit 0; `repo fleet status` exit 0 (resolves `~/.local/bin/repo`); `~/.local/share/semio/launcher/bin/repo --version` prints launcher 1.0.0 (contract 1), runtime bun ready.

## Unverified
- The full native `~/.local/bin/repo` was not rebuilt (separate installer `repo run repo-mcp:install`, not run); it embeds the launcher, so it does not yet contain fix 1 (Unix link publish).
- Windows paths not exercised on this host.
- Exact count of individual Go tests not captured (runner prints only the package result).
- Log files: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9d0eceba-b1b8-4814-b9cd-caae5bbd19aa/scratchpad/launcher-*.log`.
