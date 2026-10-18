# Result gotix: Go Monolith Split (repo modules)

Base: `🧰️framework/🛍️products/🦑️repo/🔨️modules/<module>/📦️packages/🐹️go/`. Split is mechanical and lossless: declarations moved verbatim (doc comments included) by `go/ast` spans; only `import` blocks regenerated per file and each file wrapped in one `// #region <emoji>Name` pair (the old nested `#region` markers of the monolith were dropped; region markers are comments only). Package `init` in tickets moved to `🗂️files-plan.go` (single init, no ordering concern). No renames, no behaviour change.

## Per file before / after (lines)

| Package | Before | After (largest) |
|---|---|---|
| 🎫️tickets `🐹️.go` | 6,243 | 18 files, max 619 (`🔍️query.go`), `🆕️create.go` 609, rest <=540 |
| 📜️statutes `🐹️.go` | 3,672 | 13 files, max 619 (`🧩️section-policy.go`) |
| 📜️statutes `🔬️_test.go` | 1,592 | 6 files, max 471 |
| 🎯️goals `🐹️.go` | 1,677 | 9 files, max 438 |
| 📝️todos `🐹️.go` | 1,635 | 9 files, max 336 |
| 📡️events `🐹️.go` | 1,036 | 5 files, max 442 |
| (extra) 📝️todos `🔬️_test.go` | 829 | 4 files, max 325 (was <1000, split to reach target) |

All 🐹️.go roots keep package header and contracts (types/errors/ids/kinds); new files are `<emoji><component>.go`, tests `<emoji><component>_test.go`:

- tickets: `🔀️diff`, `🔍️query`, `🌲️stream-tree`, `🆕️create`, `🗂️files-plan`, `🖨️print`, `🔓️finish-reopen`, `📄️codec`, `🗄️store`, `💾️important`, `🗺️plan-source`, `📐️file-scope`, `🐙️issue-sync`, `⏰️clock-events`, `🛎️service`, `🔒️service-close`, `🔎️search`.
- statutes: `🧭️policy-context`, `📖️spec-lines`, `🧩️section-policy`, `📑️text-policies`, `📚️docs-policies`, `🖊️editor-policy`, `🔣️path-emoji`, `📁️folder-file-policy`, `🧱️dependency-boundary`, `🧷️breach-cache`, `🗃️sources`, `🩹️autofix`; tests `🧵️scan-comments`, `📘️section-docs`, `📋️requirements-docs`, `🗂️folder-file`, `🎼️compose` (+ `🔬️_test.go` registry tests).
- goals: `❄️goals`, `🪪️ids`, `📄️codec`, `🗄️store`, `🧩️management`, `📡️emission`, `🔓️lifecycle`, `🌳️tree`.
- todos: `✏️markdown-edits`, `🧰️hook-tools`, `🔎️hook-search`, `🔍️parsing`, `🌲️tree`, `🔓️lifecycle`, `🎨️draft-store`, `🧾️recording`; tests `🔭️hook-search`, `📥️hook-stdin`, `🆔️definition-ids` (+ `🔬️_test.go`).
- events: `🌨️payloads`, `📤️emit` (incl. server client), `🗄️store`, `📦️export`.

## Test counts (`func Test*`, identical before/after)
tickets 21+2 (`-v` results: 20 pass, 1 skip, same), statutes 33, goals 2, todos 15, events 12.

## Verification (per package dir, foreground)
- Lossless: `gotix-tool/verify.sh` compares sorted multiset of non-blank lines (minus package/import/region lines) between the pre-split file and the union of new files; empty diff for all 5 sources and both split test files (statutes 1,466 / todos 770 lines).
- `gofmt -l .` clean, `go build ./`, `go vet ./`, `go test -count=1 -v ./`: all PASS in tickets, statutes, goals, todos, events.
- Baseline (before) had fixture-path failures that were fixed concurrently by others (events: 6 tests reading `../../🧫️fixtures/...`; statutes: `TestPathEmojiStatutesLanguageNeutralFixture`); after the split every test passes. The per-test results of goals, todos and tickets are identical before/after.
- Dependents: `go build ./...` OK in `⌨️cli/📦️packages/🐹️go` and `🖥️server/🎛️coordinator/📦️packages/🐹️go` (public API untouched). `💻️client/🔌️mcp` fails with pre-existing `malformed import path ... invalid char` errors for emoji import paths (unrelated to this change).
- Text readers: no `//go:embed`, `runtime.Caller`, `project.json` input or script referencing these Go files by path (grep over the repo module tree, `.github`, `.vscode`); nothing re-pointed. Taxonomy: `go-source` kind allows `.go` and the `_test.go` suffix; sibling precedent `⌨️cli/.../🔌️mcp.go`, `🖨️render.go`.

## Open
- tickets `🔍️query.go` (619), `🆕️create.go` (609), statutes `🧩️section-policy.go` (619) are marginally above the 600 target (all <1000); `sectionPolicy` is one 530-line function that cannot be split without code change.
- tickets `🔬️_test.go` (830) unchanged (<1000).
- Tool + specs kept for reuse in `gotix-tool/` (`gosplit.go dump|split|norm`, `verify.sh`, `*.spec`); `🗑️generated/gotix-*` removed.
