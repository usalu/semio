# Result E15 (go-client)

## Finding: the premise is stale, no code change was needed (nothing edited)
`sandbox.OriginPath` and `providers.McpClientKind` exist (providers `🏛️core/🐹️.go`, sandbox `🐹️.go`). The Go modules are not compilable with plain `go build ./...` by design: package directories are emoji, so Go rejects the import paths (`malformed import path ... invalid char`). The repo compiles them through a canonical overlay (`canonicalGoPlan` / `writeCanonicalGoWorkspaceOverlay` in `📚️library/🧪️testing/🐹️go-plan/🟦️.ts`; `-overlay=` maps virtual ASCII `zz_semio_*.go` files to the emoji sources). E12 most likely saw plain-go errors or a peer's transient half edit. `git log -S` shows nothing newer than the 11-18 commit, so no removing refactor exists.

## Verified in place (overlay built by the repo's own planner, GOPROXY=off, scratch GOCACHE/TMPDIR, 11-22 01:3x, load 33-39)
- `💻️client/🔌️mcp`: `go build` links (17.9 MB binary in scratch, not installed); `go vet` clean; `go test` (all) ok, also `-count=5` ok (22 s). One first run failed `TestTaskAugmentedCallRunsOnAndDeliversTheResult` ("a finished task is announced: []") under load 38; 8 later runs passed: a timing flake under load, not analysed further.
- E12 sweep `TestEveryFlagIsTypedByWhatItsValueNameMeans`: PASS in place.
- `💻️client/⌨️cli` (9 packages): `go build` links, `go vet` clean, tests: internal/command and internal/eventstore ok; root package all ok except one test (below). Two failure groups seen first (`TestExhaustiveCliE2E_TicketLifecycle...`, `TestExhaustiveTicketTitleValidation/Emoji_*`: "ticket path exceeds 256 characters") were caused by my long scratch TMPDIR; with a short TMPDIR they pass.
- Remaining failure: `TestCanonicalGoTestDispatcherCancellation` (`🧪️tests/🚦️test-dispatch`). It launches the real repo test dispatch (a bun script needing native owner capabilities); under bare `go test` the bun child dies in `readNativeOwnerCapabilities`, so the ready marker never appears. It needs the repo runner (`repo run <project>:test`), not bare go. Unproven either way.
- Test pollution: tests wrote only into the scratch TMPDIR; `/private/tmp/e15t` removed. Real checkout `git status` changes cannot be attributed (peers edit continuously), no test in these packages targets the checkout as far as seen.

## Not possible / not done
- `repo show project repo-mcp` fails: "this machine holds no sealed generation of the workspace"; starting the graph is a whole-repo inference, not done (brief rule 7). So `repo test`/`repo run` on `repo-mcp` and the cli project were not used; the overlay route above is equivalent to what `runCanonicalGoTests/Build` do minus the lease/budget wrapper.
- Not installed, running binary and MCP server untouched.

## Commands for the coordinator
- Build (official): `repo run repo-mcp:build` (script `📜️script.ts build`, output `.🧬semio/🦑️repo/⚡️cache/🗃️bin/<mode>/repo`), needs a sealed graph (`repo graph --graph-policy fresh` first).
- Install (owns `~/.local/bin/repo`): `repo run repo-mcp:install` (depends on build, side effecting). Then restart the MCP server so E12's schema fix (`ids|all`, integer flags) is served. Check with `repo run repo-mcp:test-install` beforehand if desired.
- Direct equivalent used here: `go build -overlay=<overlay.json> -o <out> .` in `💻️client/🔌️mcp` with the overlay from `writeCanonicalGoWorkspaceOverlay`; scripts in scratch `.../scratchpad/E15/` (plan.ts, mcp.json, cli.json).
