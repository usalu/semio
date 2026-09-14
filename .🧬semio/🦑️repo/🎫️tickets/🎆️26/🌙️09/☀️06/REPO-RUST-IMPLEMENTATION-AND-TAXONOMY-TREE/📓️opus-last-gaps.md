# 📓️ Last gaps — D3, the statutes Go adapter, the pipelined-burst race, and the convention residue

Executor: Opus 5 (`last-gaps`).
Host: Windows 11, `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`,
bun 1.4.2, go 1.25, 16 cores. Every command below was run; every number is real output.

The `repo` MCP server refused `initialize` for this session (`-32602 invalid initialize params`), so
no ticket lifecycle call was made; the work landed in the existing ticket folder.

## 1. Result

| # | Item | Outcome |
| --- | --- | --- |
| 1 | D3 — standalone `tree` verb in both CLIs | done; `tree monorepo --json --scope <bundle>` byte-identical, every Go-leaf argv verified |
| 2 | `📜️statutes/🗜️breach-cache-envelope` Go adapter | done; the case is 6/6 (go + rust + oracle × 2 scenarios) |
| 3 | Pipelined-burst `-32002` race | reproduced deterministically in Go, fixed identically in both, new scenario, `parity quick --owner 🔌️mcp` 33/33 |
| 4 | Gherkin continuation lines | fixed; zero non-Gherkin lines in every `🦑️repo` feature |
| 5 | Four `README.md` + `serde` workspace pins | done |
| 6 | Stale build outputs and the Go bin path | done; every Go binary now lands in `⚡️cache/🗃️bin`, and the Go MCP build was building a library archive |
| 7 | Residue of `📓️opus-final-fixes.md` §11 | two closed (checkpoint author, ticket path), three carried forward with reasons in §8 |

## 2. D3 — the `tree` verb

### 2.1 What it is

```
tree
  monorepo [query]   Show the monorepo tree      --scope <path> + every `list`/`search` filter flag
  goal               Show the goal and ticket tree
  statute            Show the statute tree
  territory          Show the territory tree
```

registered between `query` and `export` in **both** roots, and rendered through the persistent
`--json` / `--md` / `--text` flags exactly the way `search` renders one tree. `monorepo`, `statute`
and `territory` render a `TreeNode`; `goal` renders `render_goal_tree_nodes` / `RenderGoalTreeNodes`.
Every subcommand goes through the `🌳️tree` module's own ports — `build_monorepo_tree`,
`build_goal_tree`, `build_statute_tree`, `build_territory_tree` — never through a GraphQL document,
so the two implementations project one document from one repository.

`--scope <path>` is a place in the codebase: every folder and file at or below it, every bundle rooted
there, and the technologies that still own one. The aggregates that have no place — goals, tickets,
drafts, policies, contributors, checkpoints — are not part of a scoped projection. An empty scope is
the whole repository.

### 2.2 What had to be built to make the two agree

- **`🌳️tree` gained the production catalog.** `DeclaredStatuteCatalog`, `declared_statutes()` and
  `declared_territories()` / `DeclaredStatuteCatalog`, `DeclaredStatutes()`, `DeclaredTerritories()`
  now live in the tree module in both languages (it already depends on `📜️statutes` in both). The
  dashboard's private copy is gone and it uses these. `DeclaredStatutes()` sorts by identifier, because
  Go's catalog is a map and Rust's is a slice — without the sort the two orders are unrelated.
- **The Go CLI gained `FsTreeSource`**, the twin of the Rust `tree_source::FsTreeSource`: every record
  the monorepo tree projects, read through `model.RepoContext`, with `LoadFsTreeSource(ctx,
  includeSections, scope)` mirroring `FsTreeSource::load_scoped`. `NormalizeTreeScope` /
  `normalize_scope` and `PathWithinScope` / `path_within` are the shared scope grammar.

### 2.3 The Go root is the projected root

`NewRootWithConfig` registered thirty-one verbs and `ProjectionRoot` added six more — `autofix`,
`statute`, `checkpoint`, `interaction`, `draft`, `definition` — plus every `list` subcommand and
`goal tree`. So the tree the harness measured and the tree `semio-repo` actually served were
different, and `semio-repo statute list` (a dashboard leaf) did not exist. All of it is registered in
`NewRootWithConfig` now and `ProjectionRoot` is `NewRoot` with its writers redirected; the dead
`projectionReplaceChild` is gone. The two roots list the same thirty-eight verbs in the same order:

```
$ semio --help | tail -38          # 38 verbs
$ semio-repo --help | tail -38     # byte-identical list
```

### 2.4 `ticket show` and `ticket files`

The dashboard offers `tickets/<id>/{show,files,…}`; neither CLI had a verb for either, so the Go leaf
argv `ticket read <id>` / `ticket files <id>` could not have run. Both are GraphQL verbs over the
existing root field `ticket(year, month, day, slug)` — no SDL change — with the same
operand-or-`--slug` target resolution `close`/`reopen` use.

### 2.5 Every Go-leaf argv, verified

`SEMIO_REPO_IMPLEMENTATION=go semio command-tree --dump-tree` over a repository carrying one ticket,
then each emitted argv run against the real binary:

```
OK   ticket close 26/09/06/DEMO-TICKET --summary Closed from the semio dashboard --no-management
OK   ticket files 26/09/06/DEMO-TICKET
OK   ticket reopen 26/09/06/DEMO-TICKET --client claude-code --no-management
OK   ticket show 26/09/06/DEMO-TICKET
OK   goal list          OK   goal tree
OK   analyze all|go|markdown|python|rust|typescript
OK   tree goal|monorepo|statute|territory
OK   statute list
```

Three of them were wrong before: `ticket read` (no such verb), `ticket close … --bulk` (no such flag)
and `analyze --scope <scope>` (the operand is positional). `goal list`, `goal tree` and `statute list`
existed only in the projected tree, not in the binary.

**One gap remains and is a domain gap, not an argv gap.** The in-process close leaf closes in bulk —
no summary, no file list — and `ticket close` has no spelling for that: `TicketCloseInput.all` means
"every open ticket" in the Go resolver and "bulk-close this one" in the Rust one, which is itself a
divergence. The Go leaf therefore passes an explicit summary, and the CLI still asks for a file list.
Closing this properly is one field — `bulk: Boolean` on `TicketCloseInput` — plus the two resolvers
agreeing on what `all` means; it touches the SDL, both models, both resolvers, the `📜️sdl-dump`
golden and the `🧩️vscode` codegen, and is left as its own ticket.

### 2.6 The diff

A repository materialised from `🗂️codebase/🧫️fixtures/📡️repo-tree.json` (two technologies, four
bundles), both binaries, whole files compared byte for byte:

```
$ semio-repo --repo R tree monorepo --json --scope compose/📚️core   > mono-go.json
$ semio      --repo R tree monorepo --json --scope compose/📚️core   > mono-rs.json
$ cmp mono-go.json mono-rs.json     → MONOREPO-SCOPED-IDENTICAL (5059 bytes)

scope compose/🖥️app   → IDENTICAL (2586 bytes)
scope repo            → IDENTICAL (2422 bytes)
tree goal  --json / --md / --text   → IDENTICAL
tree statute   --json (whole monorepo) → STATUTE-IDENTICAL
tree territory --json (whole monorepo) → TERRITORY-IDENTICAL
```

The **unscoped** tree over the same repository differed in exactly one member before the fix in §7.1
and is identical in every other node.

### 2.7 Goldens and parity

`📖️usage-goldens.json` gained `tree` between `query` and `export` in `rootVerbs` (one line). The
`🧭️command-parsing` vectors needed no change — no vector names a verb that moved.

```
$ … parity fundamental --owner ⌨️cli
[test] level=fundamental cases=8 executed=34 passed=34 failed=0 errored=0 parity=17/17
$ … parity quick --owner ⌨️cli
[test] level=quick cases=8 executed=50 passed=50 failed=0 errored=0 parity=25/25
```

(the root-verb scenario is `@level-quick`, so it is the `quick` run that judges the new verb's place.)

## 3. `📜️statutes/🗜️breach-cache-envelope` — the Go adapter

The Go statutes package had no envelope at all: `breachCacheEnvelope` was a private struct read by
`LoadBreachsFromCache`, with no digest, no gzip and no canonical encoding. It has the twin of the Rust
surface now — `BreachCacheEnvelope`, `SHA256Hex`, `GzipEncode`, `GzipDecode`,
`EncodeBreachCacheJSON`, `ParseBreachCache`, `BreachCacheDigest`, `EncodeBreachCache`,
`DecodeBreachCache` — over `crypto/sha256` and `compress/gzip`, which are the framework's own system
libraries, and `LoadBreachsFromCache` reads through `ParseBreachCache`.

`EncodeBreachCacheJSON` states the canonical form the digest is taken over: declaration order, **no
HTML escaping** and no trailing newline. Go's `encoding/json` escapes `<`, `>` and `&` by default and
`json.Encoder` appends a newline; either would have moved the digest away from `serde_json`.

`🧪️tests/🗜️breach-cache-envelope/🐹️.go` is the new adapter, with the same length-prefixed frame the
Rust subject writes so the Node `zlib` oracle inflates both.

```
$ … parity fundamental --case 🗜️breach-cache-envelope
[test] level=fundamental cases=1 executed=6 passed=6 failed=0 errored=0 parity=6/6     (was 4)
$ … parity fundamental --owner 📜️statutes
[test] level=fundamental cases=5 executed=20 passed=20 failed=0 errored=0 parity=13/13
```

## 4. The pipelined-burst race

### 4.1 Reproduced

`printf` writing `initialize`, `notifications/initialized` and `tools/list` as one burst, ten trials
each:

```
$ cat burst.txt | .🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp.exe
{"jsonrpc":"2.0","id":2,"error":{"code":-32002,"message":"session not initialized"}}
{"jsonrpc":"2.0","id":1,"result":{…}}
                                          → 8 of 8 trials, deterministic

$ cat burst.txt | target/release/semio.exe mcp
{"jsonrpc":"2.0","id":1,"result":{…}}
{"jsonrpc":"2.0","id":2,"result":{"tools":[…]}}
                                          → 8 of 8 trials passed by timing luck
```

Go failed every time because its reader hands notifications through inline and queues requests, so
`tools/list` reached a second worker while `initialize` was still on the first. Rust has the identical
worker-pool construction and only wins the race because spawning eight threads is slower than reading
three lines — the defect report already said so, and a slower host would flip it.

### 4.2 Fixed, identically

A **handshake latch** on the session, in both implementations:

- `SessionState.handshake_pending` / `Session.handshakePending` counts the `initialize` requests the
  transport has delivered and no worker has answered.
- The transport arms it **before** the payload reaches the queue (`expect_handshake` /
  `ExpectHandshake`) — arming afterwards races the worker that already answered and the decrement
  would be lost for good — and settles it again on the queue-full refusal.
- `dispatch` settles the latch on every path an `initialize` leaves by (an RAII `HandshakeSettle` in
  Rust, `defer SettleHandshake()` in Go), and every other request waits on the latch before it routes.
- `drop_session` / `shutdown` clears and broadcasts, so a peer that goes away never leaves a waiter.

The `notifications/initialized` half was already ordered by `pending_initialized`: the notification is
remembered when it arrives in `Connected` and applied when `initialize` completes. The latch is what
makes the request that follows it in the same burst wait for that promotion.

```
$ cat burst.txt | .🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp.exe   → id 1 then id 2, both results, 10/10
$ cat burst.txt | target/release/semio.exe mcp                        → id 1 then id 2, both results, 10/10
```

### 4.3 The scenario

`@id-pipelined-burst-serves-requests-after-initialized` (`@level-fundamental`, `@mode-differential`)
in `🔌️mcp/🧪️tests/🤝️jsonrpc-handshake`, with a Go, a Rust and a TypeScript-SDK-oracle adapter:
"the client writes initialize, the initialized notification and a request as one burst" → "every
implementation serves that request instead of refusing it as an uninitialized session". The projection
states the ids answered, whether the following request carries a result, and how many replies refused
an uninitialized session.

```
$ … parity quick --owner 🔌️mcp
[test] level=quick cases=4 executed=33 passed=33 failed=0 errored=0 parity=33/33      (was 30 / 30)
$ go test ./... -count=1   # github.com/usalu/semio/repo/mcp
ok  	github.com/usalu/semio/repo/mcp	0.856s
```

`TestG2TransportRejectsBusyBranchMutation` is a source-mutation test over the busy branch; the settle
call had to go **after** `server.handlerQueued.Add(-1)` so the `credit` mutation still matches the
text it mutates. It does, and the test is green.

## 5. Gherkin continuation lines

`🗂️codebase/🧪️tests/🚶️workspace-walk/🥒️.feature` carried two prose paragraphs inside `Scenario:`
blocks (lines 34–37 and 46–48). The harness's Gherkin reader accepts free text only while the block is
the `Feature:` itself, so those seven lines were `Unrecognized line` breaches. Both paragraphs moved
into the feature description, where they say the same thing about the scenarios below them.

A scan of every `.feature` under `🧰️framework/🛍️products/🦑️repo` for a line that is neither a tag,
a keyword, a step, a table row, a doc string nor a feature description now reports **none**.

## 6. Conventions

- `README.md` written for `📊️metrics`, `📐️model`, `🗂️codebase` and `🚚️move`, in the same shape as
  the other twenty-two: what the module owns, its packages with the Go module path and Rust crate
  name, and its cases with the oracle situation. No `AGENTS.md` was touched.
- `📐️model` and `⌨️cli` `Cargo.toml`: `serde` and `serde_json` are `{ workspace = true }`. A sweep of
  every `Cargo.toml` under the product finds no pinned version left.

## 7. Build output

- `defaultCliBin` for `go` answers `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo[.exe]`, beside the MCP
  binary that already lived there; `REPO_BIN_CACHE_DIR` is the one literal, shared by the TypeScript
  resolver, its shell twin `compose_resolve_repo_cli`, and the dashboard's `go_binary_path`.
- `.devcontainer/post-create.sh`, `.devcontainer/post-attach.sh`, `🔩️native/🥾️bootstrap/🐚️.sh` and
  `🔵️.ps1` build and resolve the same path.
- **The Go MCP bootstrap was building a library.** All four scripts passed
  `./…/🔌️mcp/📦️packages/🐹️go` — the package directory — instead of its `🚀️bin` entry point, and
  `go build -o` on a non-main package writes an **archive**: the produced `semio-repo-mcp` began
  `!<arch>` and could not be executed. Only `buildRepoMcpBin` in the TypeScript library had it right.
- `.gitignore` no longer names the two in-tree Go CLI binaries; `*.exe` and `⚡️cache/` already cover
  the new location. `TestBuildBinaryArtifactsGitIgnored` states the cache paths now.
- Deleted (all untracked, `git ls-files` confirms): `⌨️cli/📦️packages/🐹️go/semio-repo.exe`,
  `⌨️cli/📦️packages/🐹️go/🚀️bin/bin.exe`, `🔌️mcp/📦️packages/🐹️go/mcp.exe`,
  `🔌️mcp/📦️packages/🐹️go/🚀️bin/bin.exe`,
  `🖥️server/🎛️coordinator/📦️packages/🐹️go/📦️main/bin.exe`. No `*.exe` or `*.pdb` remains under
  `🦑️repo` outside the `🧪️test` dotnet host's `bin/`/`obj/`.
- `launch.json` regenerated from the seed; the seed needed no edit, because every repo entry goes
  through `📜️script.ts` and never names a binary path.

## 8. `📓️opus-final-fixes.md` §11, item by item

1. **`🌳️tree`'s two builders.** The new `tree monorepo` verb builds through the port-based twin in
   **both** implementations, so the surface this ticket adds is identical. `list`, `search`, `query`
   and `mermaid` in the Go binary still call `BuildMonorepoTreeCached` → `BuildMonorepoTreeFromRepo`,
   the godfile builder. Retargeting it is not a call-site change: `🌳️tree`'s own
   `🔭️exhaustive_test.go` (twelve `//go:build exhaustive` tests) drives `BuildMonorepoTreeCached`
   over the real repository and cannot construct the new `FsTreeSource`, which lives in `⌨️cli`
   because that is where the Rust twin lives. Closing it means moving those twelve tests to `⌨️cli`
   or giving `🌳️tree` a `model.RepoContext`-shaped source of its own — a `🌳️tree` module change with
   its own fixtures. **Carried forward.**
2. **`Section.path` is never filled.** Unchanged, identical in both implementations. Owner
   `🗣️languages`. **Carried forward.**
3. **The `loc` `Data` residue** (270 lines of 2.15 M). Unchanged. Owner `📊️metrics`. **Carried forward.**
4. **The nx duplicate project.** Another fleet's `✏️s/🔌️plugins/🧩️puzzle/…` pair; still reproduces,
   still not ours to rename. **Carried forward.**
5. **`list`/`search` are slow.** Unchanged; `tree monorepo --scope` gives the wizard and the diff a
   bounded projection, but the walk underneath is still repository-wide in both implementations.
   **Carried forward.**

Two divergences the new verb exposed were **closed** here:

- **`checkpoint.authorId` was empty in Go.** `contributorspkg.LoadCheckpoints` was a godfile-era
  duplicate of the ported `ListCheckpoints`/`ParseCheckpointLog` that read the same
  `%H|%aN|%ad|%s` log and then dropped field two. It delegates to the ported reader now, and the
  unscoped `tree monorepo --json` became identical.
- **`Ticket.path` carried Windows separators in Go.** `ticketSource` now normalises it, and
  `ticket show --json` is byte-identical between the two binaries.

One divergence the new verb exposes is **recorded, not closed**:

- **`tree monorepo --md` / `--text` disagree on a bundle's rendered artifact id** — Go prints
  `🎼compose📚📚️core`, Rust `👤compose📚📚️core`. The JSON is identical, because the node id comes
  from the source; the renderers recompute it from node data, and Go's `model.GetArtifactID` resolves
  the technology emoji through the `LookupTechnologies` hook `🗂️codebase` installs at init while
  Rust's `artifact_id` has no such hook and falls back to the name-derived kind. It is a `📐️model`
  divergence that `list --md` and `search --md` already had; the fix is a settable technology lookup
  in the Rust model, which needs a repository root and a cache to stay affordable.

## 9. Verification — every gate, real output

```
$ cargo build --release -p semio-framework-repo-cli --bins
    Finished `release` profile [optimized] target(s)

$ cargo test --release -p <every semio-framework-repo-* crate, 24 of them>
    every target: test result: ok. 0 failed          (0 failing assertions across the whole set)

$ cargo clippy --release --no-deps -p <the same 24> --all-targets
    no `generated N warnings` line for any repo crate            → 0 warnings

$ for every go.work member (27): go build ./... && go vet ./... && gofmt -l . && go test ./... -count=1
    27/27 ok
```

Three Go tests had to follow the changes and did:
`TestBuildBinaryArtifactsGitIgnored` (the cache paths), `TestG2TransportRejectsBusyBranchMutation`
(the settle call moved after the credit line), `TestMcpBootstrapAssetsStayRepoRelative` (the cache
path and the `🚀️bin` entry points).

### `parity fundamental --owner <owner>`

```
⌨️cli          [test] level=fundamental cases=8 executed=34 passed=34 failed=0 errored=0 parity=17/17
🌳️tree        [test] level=fundamental cases=5 executed=24 passed=24 failed=0 errored=0 parity=12/12
🎛️dashboard   [test] level=fundamental cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0
🎫️tickets     [test] level=fundamental cases=5 executed=57 passed=57 failed=0 errored=0 parity=42/42
🎯️goals       [test] level=fundamental cases=4 executed=9 passed=9 failed=0 errored=0 parity=6/6
🏃️test-runner [test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
🏠️workspace   [test] level=fundamental cases=5 executed=17 passed=17 failed=0 errored=0 parity=13/13
📊️metrics     [test] level=fundamental cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
📐️model       [test] level=fundamental cases=3 executed=13 passed=13 failed=0 errored=0 parity=8/8
📚️library     [test] level=fundamental cases=0 executed=0 passed=0 failed=0 errored=0 parity=0/0
📜️statutes    [test] level=fundamental cases=5 executed=20 passed=20 failed=0 errored=0 parity=13/13
📝️todos       [test] level=fundamental cases=4 executed=11 passed=11 failed=0 errored=0 parity=7/7
📡️events      [test] level=fundamental cases=4 executed=27 passed=27 failed=0 errored=0 parity=27/27
🔌️mcp         [test] level=fundamental cases=4 executed=24 passed=24 failed=0 errored=0 parity=24/24
🔎️search      [test] level=fundamental cases=1 executed=3 passed=3 failed=0 errored=0 parity=3/3
🔗️graphql     [test] level=fundamental cases=8 executed=21 passed=21 failed=0 errored=0 parity=18/18
🗂️codebase    [test] level=fundamental cases=4 executed=20 passed=20 failed=0 errored=0 parity=10/10
🗣️languages   [test] level=fundamental cases=5 executed=26 passed=26 failed=0 errored=0 parity=16/16
🚚️move        [test] level=fundamental cases=5 executed=32 passed=32 failed=0 errored=0 parity=16/16
🧑️contributors [test] level=fundamental cases=3 executed=7 passed=7 failed=0 errored=0 parity=5/5
🧩️providers   [test] level=fundamental cases=4 executed=29 passed=29 failed=0 errored=0 parity=22/22 not-exercised=1
🧪️test        [test] level=fundamental cases=1 executed=10 passed=10 failed=0 errored=0 parity=20/20
🧾️yaml        [test] level=fundamental cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4
🪝️hooks       [test] level=fundamental cases=5 executed=54 passed=54 failed=0 errored=0 parity=45/45
🪪️identity    [test] level=fundamental cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4
```

Every owner under `🔨️modules`: 25 owners, 0 failed, 0 errored. `📚️library` owns no case
and `🏛️dashboard` is Rust-only by design, so both report 0 parity pairs; `🧩️providers` reports
`not-exercised=1` for `🌿️git-version-control`, whose four scenarios are all `@level-long`.

### The MCP handshake through the dev entry point

```
$ printf '<initialize>
<notifications/initialized>
<tools/list>
' | bun ./📜️script.ts dev mcp stdio client
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-11-25","capabilities":{"prompts":{},"resou…
{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"file_integrate","description":"Use when two file…

$ SEMIO_REPO_IMPLEMENTATION=go  … the same burst through the same entry point
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-11-25","capabilities":{"prompts":{},"resou…
{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"file_integrate","description":"Use when two file…
```

Both entry points answer both ids in order, with a result on each. Before the fix the Go one answered
`{"id":2,"error":{"code":-32002,"message":"session not initialized"}}` first.

### `contract`

```
$ bun 🧪️test/📜️script.ts contract
Unrecognized line / Data table / Doc string / Step outside a scenario   → 0
```

The 76 breaches still attributed to the product are the pre-existing `📚️library`
`testing/dependency` oracle-in-production pattern (73) plus three in `🧪️tests/🧪️transcript`, exactly as the
audit characterised them; the run's exit code 1 comes from those and from the unrelated
`testing/discovery` baselines of `temp`, `.storybook`, `✏️s` and `♻️mit-bestand`.

## 10. Files

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🗜️breach-cache-envelope/🐹️.go`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-last-gaps.md` (this file)

Updated — Rust:

- `⌨️cli/📦️packages/🦀️rust/🦀️.rs` — `tree_spec::tree`, `ticket show|files`, `repo_cli::tree_verb`,
  `tree_monorepo`, `tree_goal`, `render_forest`, `render_node`, `FsTreeSource::load_scoped`,
  `normalize_scope`, `path_within`, `tree_filter_public`, `wants_sections`
- `⌨️cli/📦️packages/🦀️rust/Cargo.toml` — `serde`/`serde_json` workspace
- `📐️model/📦️packages/🦀️rust/Cargo.toml` — `serde`/`serde_json` workspace
- `🌳️tree/📦️packages/🦀️rust/🦀️.rs` — `DeclaredStatuteCatalog`, `declared_statutes`, `declared_territories`
- `🔌️mcp/📦️packages/🦀️rust/🦀️.rs` — `handshake_pending`, `expect_handshake`, `settle_handshake`,
  `await_handshake`, `HandshakeSettle`, the armed transport
- `🎛️dashboard/🌳️command-tree/🦀️.rs` — `REPO_BIN_CACHE_DIR`, `go_binary_path`, the corrected
  `go_argv`, `DASHBOARD_CLOSE_SUMMARY`, the shared catalog, the argv test

Updated — Go:

- `⌨️cli/📦️packages/🐹️go/🐹️.go` — `treeCommand`, `FsTreeSource`, `LoadFsTreeSource`,
  `NormalizeTreeScope`, `PathWithinScope`, `singleTicketCommand`, the merged `NewRootWithConfig`,
  the reduced `ProjectionRoot`
- `⌨️cli/📦️packages/🐹️go/🔬️_test.go` — the bootstrap asset assertions
- `🌳️tree/📦️packages/🐹️go/🐹️.go` — `DeclaredStatuteCatalog`, `DeclaredStatutes`, `DeclaredTerritories`
- `🔌️mcp/📦️packages/🐹️go/🐹️.go` — the handshake latch and the armed transport
- `📜️statutes/📦️packages/🐹️go/🐹️.go` — the breach-cache envelope
- `🧑️contributors/📦️packages/🐹️go/🐹️.go` — `LoadCheckpoints` delegates to the ported reader
- `🔗️graphql/📦️packages/🐹️go/🐹️.go` — `ticketSource` normalises `path`
- `🏠️workspace/📦️packages/🐹️go/🔬️_test.go` — the gitignored build-artifact paths

Updated — features, adapters, fixtures and wiring:

- `🔌️mcp/🧪️tests/🤝️jsonrpc-handshake/{🥒️.feature, 🦀️.rs, 🐹️.go, 🟦️.ts}`
- `🗂️codebase/🧪️tests/🚶️workspace-walk/🥒️.feature`
- `⌨️cli/🧪️tests/📖️usage-text/🧫️fixtures/📖️usage-goldens.json`
- `📚️library/📦️packages/🟦️typescript/🟦️.ts` — `REPO_BIN_CACHE_DIR`, `defaultCliBin`, `defaultMcpBin`,
  `compose_resolve_repo_cli`
- `.gitignore`, `.devcontainer/post-create.sh`, `.devcontainer/post-attach.sh`,
  `🔩️native/🥾️bootstrap/🐚️.sh`, `🔩️native/🥾️bootstrap/🔵️.ps1`, `.vscode/launch.json` (regenerated)

No git-modifying command was run.
