# 📓️ Audit (wave 5) — final verification

Run 2026-09-06, ~11:01–11:32 UTC, on this Windows 11 host. `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`,
`SEMIO_TEST_BUDGET_MS=600000`. A concurrent Opus fixer was closing known leftovers during this run
(definitions aggregate, export digest, loc performance, rename-casings, vscode codegen) — timestamps are
given below where the audit observed something mid-flight or already-fixed.

Evidence lives under `🗑️generated/audit/` (`parity-summary.txt`, `go-test-summary.txt`, `go/` per-module
build/vet/test logs, `contract-full.log`, `clippy-all-repo-crates.log`, `analyze-*.json`, `mcp-*.jsonl`).

## Verdict per plan §1 outcome

**1. Domain-driven, implementation-neutral tree — MET.**
All 29 `🔨️modules/*` entries match plan §2. No `internal/`, `cmd/`, OS-suffixed files, tracked binaries
(`git ls-files | grep -E '\.(exe|vsix|pdb)$'` → empty), or legacy `💻️client/`, `🎮️commands/`, `./repo/`
anywhere. No file over 10,000 lines (largest repo godfiles: Go `🔗️graphql` 7,349, `📐️model` 7,004,
`⌨️cli` 6,739 lines; two pre-existing `📚️library` TypeScript files are >10k but are `[KEEP]`, outside
this ticket's scope). Every test case directory under `🔨️modules/*/🧪️tests/` carries a leading emoji
(coordinator's 2026-09-06 06:10 decision — confirmed swept).

**2. Every Go domain has a Rust twin, same tests, third-party oracle where one exists — MET, with 4 known gaps.**
24/24 planned Rust crates exist and build; 26/26 Go modules (23 domain + cli/bin + coordinator/main +
test-host) build, vet, gofmt-clean, and `go test` green. Harness: 1,085/1,085 executed scenario-runs
passed across all 25 owners (0 failed, 0 errored) — see table below. Gaps: `🚚️move/🔤️rename-casings`
and `🔌️mcp`'s `🔗️event-log-chain` / `📞️tool-call-roundtrip` still lack a Go adapter (rust+typescript
only); `📜️statutes/🗜️breach-cache-envelope` likewise Go-less. `🎛️dashboard` is Rust-only by design
(plan §2 note). One scenario is not-exercised at all: `🧩️providers/🌿️git-version-control` — harness
itself reports "no implementation served the requested phase(s) oracle, subject" for both
fundamental and quick, at 11:20 UTC.

**3. `semio` binary carries the whole CLI+MCP surface, all entry points default to Rust — MOSTLY MET.**
`.mcp.json`, `.vscode/mcp.json`, `.cursor/mcp.json`, `.windsurf/mcp.json`, `.codex/config.toml`,
`.kiro/settings/mcp.json` all route through `bun ./📜️script.ts dev mcp stdio <client>`, which resolves
Rust by default and Go under `SEMIO_REPO_IMPLEMENTATION=go` (`resolveRepoImplementation` in the shared
library). `tools/list` is **identical** between Rust and Go (9 tools) once the client follows the MCP
handshake properly. Gap: the CLI verb table has **no standalone `tree` verb** (confirmed identical in
both `semio --help` and the Go `semio-repo --help`, 43-line list, `tree` absent) — only `goal tree` and
`statute tree` subcommands exist. The dashboard's own "what is left" note (opus-dashboard.md §4)
already flagged this as unverified ("`tree monorepo|goal|statute|territory`... must be confirmed
against the Go CLI's real verb table") — this audit confirms that argv shape does not exist in either
implementation, so a Go-leaf dashboard dispatch for a "tree" wizard node would fail. `command-tree
--dump-tree` does contain `ticket(s)`, `goal(s)`, `analyze`, `tree`, `statute(s)` keys (7.9MB JSON,
verified via `grep -o "\"key\": \"..."`), so the dashboard's own in-process tree/goal/statute
sub-branches are fine — only the free-standing CLI verb is missing.

**4. Terminal dashboard shows repo-domain commands in-process — MET** (command-tree-projection case
passes 1/1 fundamental + 2/2 quick, Rust-only by design). Windows daemon gap confirmed still open (see
defect list, D2).

## 1. Tree

| Module | schema | tests (n) | go | rust | ts | oracle |
| --- | --- | --- | --- | --- | --- | --- |
| ⌨️cli | no | 8 | yes | yes | yes | yes |
| 🌳️tree | yes | 5 | yes | yes | — | yes |
| 🎛️dashboard | yes | 1 | — | yes | — | yes |
| 🎫️tickets | yes | 5 | yes | yes | — | yes |
| 🎯️goals | yes | 4 | yes | yes | — | yes |
| 🏃️test-runner | yes | 5 | yes | yes | — | yes |
| 🏠️workspace | yes | 5 | yes | yes | — | yes |
| 📊️metrics | yes | 4 | yes | yes | — | yes |
| 📐️model | yes | 3 | yes | yes | — | yes |
| 📚️library | no | 68 | — | — | yes | no (KEEP) |
| 📜️statutes | yes | 5 | yes | yes | — | yes |
| 📝️todos | yes | 4 | yes | yes | — | yes |
| 📡️events | yes | 4 | yes | yes | — | yes |
| 🔌️mcp | yes | 4 | yes | yes | — | yes |
| 🔎️search | yes | 1 | yes | yes | — | yes |
| 🔗️graphql | yes | 8 | yes | yes | — | yes |
| 🔩️native | no | 0 | — | — | — | no (KEEP) |
| 🖥️server/🎛️coordinator | yes | 4 | yes | yes | yes(app) | yes |
| 🗂️codebase | yes | 4 | yes | yes | — | yes |
| 🗣️languages | yes | 5 | yes | yes | — | yes |
| 🚚️move | yes | 5 | yes | yes | — | yes |
| 🧑️contributors | yes | 3 | yes | yes | — | yes |
| 🧩️providers | yes | 4 | yes | yes | — | yes |
| 🧩️vscode | no | 0 | — | — | yes | no (KEEP) |
| 🧪️test | yes | 2 | yes | yes | yes | no (KEEP) |
| 🧾️yaml | yes | 2 | yes | yes | — | yes |
| 🪝️hooks | yes | 5 | yes | yes | — | yes |
| 🪪️identity | yes | 2 | yes | yes | — | yes |
| 🪶️sqlite | yes | 0 | — | — | — | no (KEEP) |

No `internal/`, `cmd/`, `_unix.go`/`_windows.go` module-root files, `💻️client`, `🎮️commands`, `./repo/`,
or tracked `.exe`/`.vsix`/`.pdb` anywhere. Only 3/29 modules (`🧩️vscode`, `🧪️test`, `🪶️sqlite`) carry a
`README.md`; the rest (including `📐️model`, which explicitly flagged this in its own report) do not —
minor, not an outcome-criterion blocker.

Durability note: plan §2 describes coordinator durability as living under a `🛡️durability/`
subdirectory; the actual Rust crate implements it inline in the single `🦀️.rs` godfile (no such
subdirectory exists). Functionally equivalent, just a naming/structure deviation from the plan prose.

## 2. Build

**Rust.** `cargo build --release -p semio-framework-repo-cli` → exit 0 (also pulls in the full
workspace graph; only warnings are in unrelated `semio-framework-ui-*` crates).
`cargo build` with `-p` for all 24 `semio-framework-repo-*` crates → exit 0, no warnings attributable
to any repo crate. `cargo clippy --no-deps` for all 24 → **0 warnings, 0 errors** in every repo crate;
the only "generated N warnings" lines in the 677-line clippy log belong to
`semio-framework-{ui-contract,ui-scene,replication,async,value-derive}` (unrelated crates pulled in by
the workspace clippy invocation).

**Go**, all 26 `go.work` members (23 domain dirs + `⌨️cli/🚀️bin` + `🖥️server/🎛️coordinator/📦️main` +
`🧪️test`): `go build ./...` — 26/26 exit 0. `go vet ./...` — 26/26 exit 0. `gofmt -l .` — 26/26 empty
(no unformatted files). `go test ./... -count=1` — **26/26 exit 0** (`ok` or `[no test files]` for the
three bin/main packages and `metrics`/`test`, which have no `_test.go`). All `go.mod` pin `go 1.25`,
matching `go.work`; zero external Go dependencies — every `require` line is an in-repo
`github.com/usalu/semio/repo/*` module. Slowest: `🌳️tree` 32s, `🪝️hooks` 14s, `🧩️providers` 13s,
`⌨️cli` 11s — all real work, not hangs.

Note: this contradicts several stale "what is left" items in earlier opus reports (e.g.
`🎫️tickets/🐹️.go:825` nil-`RepoContext` panic, `🏃️test-runner`'s `scope-identifier-normalisation`
adapter importing the dismantled `client` package) — both are confirmed **fixed** as of this run (grep
for the offending import found nothing; the full Go test suite is green).

Rust dependency hygiene: every repo crate's `[dependencies]` is `serde`/`serde_json` (mostly
`{ workspace = true }`, plan-compliant) plus sibling `semio-framework-repo-*` path crates only — no
external runtime crates leak in. Minor convention deviation: `📐️model` and `⌨️cli` pin `serde`/
`serde_json` with explicit versions instead of `{ workspace = true }` (plan §3 says workspace-pinned).
The `serde-json-equation-carrier-reader` oracle-registration conflict `📐️model`'s own report flagged
(193 repo-wide `testing/dependency` breaches) is **no longer present** — `grep 📐️model` against the
fresh `contract` run returns nothing.

## 3. Harness

`bun 🧪️test/📜️script.ts discover`: 430 cases repo-wide, **101 owned by `🦑️repo`** (`discover-repo.txt`).

`parity fundamental` + `parity quick` for all 25 owners (`⌨️cli 🌳️tree 🎛️dashboard 🎫️tickets 🎯️goals
🏃️test-runner 🏠️workspace 📊️metrics 📐️model 📜️statutes 📝️todos 📡️events 🔌️mcp 🔎️search 🔗️graphql
🖥️server/🎛️coordinator 🗂️codebase 🗣️languages 🚚️move 🧑️contributors 🧩️providers 🧪️test 🧾️yaml 🪝️hooks
🪪️identity`):

| Metric | Total |
| --- | --- |
| Owners run | 25/25 |
| Scenario-invocations executed | 1,085 |
| Passed | 1,085 |
| Failed | 0 |
| Errored | 0 |
| Not-exercised | 1 (`🧩️providers/🌿️git-version-control`, both levels) |

Full per-owner breakdown in `🗑️generated/audit/parity-summary.txt`.

Scenarios where fewer than both Go+Rust ran (from `discover-repo.txt`, cross-checked against the
`⚡️cache/tests/results/` jsonl naming in the parity log):

| Case | Implementations | Note |
| --- | --- | --- |
| `🎛️dashboard/🌳️command-tree-projection` | rust | by design (plan §2: "Rust only (TUI)") |
| `🔌️mcp/🔗️event-log-chain` | rust, typescript | no Go adapter yet |
| `🔌️mcp/📞️tool-call-roundtrip` | rust, typescript | no Go adapter yet |
| `🚚️move/🔤️rename-casings` | rust | no Go adapter yet — this is the leftover the concurrent fixer is closing |
| `📜️statutes/🗜️breach-cache-envelope` | rust, typescript | no Go adapter yet |

`contract` (full repo, completed in ~7 min): 85 breaches attributable to `🦑️repo` paths —
**76 testing/dependency** (nearly all `📚️library` `🧪️tests/*/🟦️.ts` files registered as
"Production source" importing the `node-crypto-repo-events`/`typescript-compiler` test oracles — a
pre-existing library-wide pattern, not new to this ticket), **6 testing/oracle** (oracle manifests
missing a `capability` declaration matching their feature file: `intl-segmenter` for
`🪪️identity/😀️entity-emoji-codec`, `semio-search-reference-ts` for `🔎️search/🔍️ranked-search`,
`node-zlib-crypto` for `📜️statutes/🗜️breach-cache-envelope`, `micromatch`/`gitignore-js` for
`🏠️workspace`'s two glob/ignore cases, `yaml-js` for `🧾️yaml/🔁️codec-roundtrip`), **3 testing/contract**
(the `🎛️dashboard/🌳️command-tree-projection` `.feature` file has 3 `Then`-clause continuation lines
that a strict Gherkin parser flags as "Unrecognized line" — the step text wraps across lines without
`And`/`But`, a real (if cosmetic) Gherkin-authoring defect; see D4 below).

## 4. Entry points

`dev mcp stdio client` (Rust default) and `SEMIO_REPO_MCP_CLIENT`/`SEMIO_REPO_IMPLEMENTATION=go`
(Go) both answer `initialize`+`notifications/initialized`+`tools/list` correctly, and **the tool
lists are identical**: 9 tools (`file_integrate`, `goal_close`, `goal_open`, `goal_reopen`,
`section_extract`, `section_move`, `ticket_close`, `ticket_open`, `ticket_reopen`). Both implementations
correctly ignore unknown `initialize` params (the `baseline-and-mcp-initialize-defect.md` issue is
fixed in both). **Found a new Go-only race condition — see D1.**

`.mcp.json`/`.vscode/mcp.json`/`.cursor/mcp.json`/`.windsurf/mcp.json`/`.codex/config.toml`/
`.kiro/settings/mcp.json`: all six route `repo` through `bun ./📜️script.ts dev mcp stdio <profile>`
(`client`/`copilot`/`cursor`/`client`/`codex`/`kiro` respectively) and `semio` through
`dev mcp stdio os`.

`target/release/semio --help`: 36 verbs (mcp, graphql, test, ticket, todo, goal, contributor, folder,
file, section, move, integrate, extract, rename, sync, search, list, query, export, hook, mermaid, loc,
technology, bundle, analyze, entity-emojis, configure, micro-commit, benchmark, update, auth, autofix,
statute, checkpoint, interaction, draft, definition) — byte-identical to the Go `semio-repo --help`
verb list, built fresh from `⌨️cli/📦️packages/🐹️go/🚀️bin`. Neither has a standalone `tree` verb (see §"3.
Entry points" above).

`semio ticket list --json` and `semio goal tree` return real data instantly. `semio analyze <scope>
--json` (positional scope, not `--scope`) works: identity scope → `{"analyze":{"breachs":[],...}}`
correctly empty; a full `🧰️framework/🛍️products/🦑️repo` scope also returned empty in <1s, which is
suspicious for a scope that size — not independently cross-checked against a differently-scoped
positive result in this run (worth re-verifying scope resolution semantics).

`command-tree --dump-tree`: 7.9MB JSON: contains `"key": "tickets"`, `"goal"`/`"goals"`,
`"analyze"`, `"tree"`, `"statute"`/`"statutes"` — all five required branches present.

`.vscode/launch.json`: 811 lines mention repo/semio/dashboard/mcp; spot-checked names for
`🛠️dev🧰️repo…`/`🧪️test🧰️repo…` naming convention — consistent.

`bun x nx show projects | grep -i repo`: **fails** — `nx show projects` cannot build the project graph
at all repo-wide: "projects defined in multiple locations" for
`test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-third-party-puzzle-2d-1`, defined at
both `.../🌐️third-party-puzzle-2d-1` and `.../🕸️third-party-puzzle-2d-1` under
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/...`. This is **unrelated to `🦑️repo`** (it's in the puzzle
plugin's test fixtures) but persists and blocks this specific verification step as noted in the task
brief.

## 5. Statutes

`analyze` on `🪪️identity` scope alone: 0 breaches. Full `analyze` on `🧰️framework/🛍️products/🦑️repo`
scope returned in <1s with an apparently-empty result — flagged above as needing re-verification of
scope-path semantics rather than trusted at face value. The full-repo `contract` run (§3 above) is the
more reliable statute signal obtained this session: 85 `🦑️repo`-attributable breaches, mostly
testing/dependency in `📚️library`.

## 6. Ticket hygiene

No stray `*.txt`/`*.jsonl`/`*.log`/`*.raw` outside `🗑️generated/` anywhere under this ticket folder.
`🗑️generated/` held (before this audit) `audit/` (this session), `cli-verbs/`, `final-fixes/`
(pre-existing from other agents) — all clean. This audit's own `🗑️generated/audit/` was pruned of its
largest disposable artifacts (7.9MB `command-tree.json`, the scratch `semio-repo-go.exe`, `nx show
projects` dumps) after extracting the facts above; the remaining ~2MB (`parity-summary.txt`,
`go-test-summary.txt`, `go/*.log`, `contract-full.log`, `clippy-all-repo-crates.log`, `analyze-*.json`,
`mcp-*.jsonl`) is kept as evidence for this report's claims, per "you MUST NOT delete all audits,
reports."

## Defect list (priority order)

**D1 — Go MCP server drops in-flight requests on stdin EOF (race condition, HIGH).**
`🔌️mcp/📦️packages/🐹️go/🐹️.go`, `Server.Serve` (~line 640-688): on scanner EOF it calls
`finish(ErrPeerDropped)`, which runs `session.drop(cause)` **before** `workers.Wait()` drains the
`jobs` channel. If the whole request stream arrives before the OS pipe reports EOF (any client that
writes-then-closes stdin promptly — a `printf ... | binary` pattern, or a short-lived script/CI
runner), every already-queued-but-not-yet-dispatched request is rejected with `{"code":-32004,
"message":"session closed"}` even though it was fully received. Reproduced directly against
`.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp.exe` (both via `bun ./📜️script.ts dev mcp stdio client`
with `SEMIO_REPO_IMPLEMENTATION=go`, and by invoking the binary directly): instant-EOF → both
`initialize` and `tools/list` error; keeping stdin open ~2s longer → both succeed with the same
payload the Rust binary returns instantly either way. The Rust implementation (`target/release/semio
mcp`) handles instant-EOF correctly in every trial. Fix: drain/complete already-queued jobs before
calling `session.drop` in the EOF path, or don't treat scanner EOF as an immediate hard-drop.

**D2 — Dashboard daemon has no Windows server-side listener (confirmed still open, MEDIUM).**
`🎛️dashboard/🌀️daemon/🦀️.rs`: `#[cfg(windows)] pub fn pipe_name`/`connect` exist (client side only);
the accept-loop/`ClientReader` and `serve()` are `#[cfg(unix)]`-only, and line ~544 returns
`"dashboard daemon listen is unix-only in this build"` unconditionally on Windows. On this Windows 11
host, `semio daemon start|attach` cannot work at all. Matches opus-dashboard.md's own "what is left"
#2, unresolved as of this audit.

**D3 — `tree` is not a CLI verb despite being a full domain crate (MEDIUM).**
`🌳️tree` has 5/5 passing test cases and is consumed in-process by the dashboard, but neither the Rust
nor the Go top-level CLI exposes a standalone `tree` verb with `monorepo|goal|statute|territory`
subcommands (only `goal tree` and `statute tree` exist as subcommands of other verbs). The dashboard's
own report already flagged this argv assumption as unconfirmed; this audit confirms the gap. Any
dashboard code path that shells out `semio-repo tree monorepo` (Go leaf dispatch) will fail with
"unknown command tree" today.

**D4 — `.NET` subject host for `host-protocol-parity` cannot build (MEDIUM).**
`🧪️test/📦️packages/🔷️dotnet/🧪️Semio.Repo.Test.csproj` has `<Compile Include="🔷️host.cs" />`, but the
actual source file is named `🔷️.cs` (godfile convention). `dotnet build` fails with `CS2001: Source
file '...🔷️host.cs' could not be found`, so the harness's `dotnet` subject for
`host-protocol-parity` never emits results in either `fundamental` or `quick` runs — one-line fix
(`Include="🔷️.cs"`).

**D5 — `🧩️providers/🌿️git-version-control` scenario not exercised (LOW-MEDIUM).**
Both `parity fundamental` and `parity quick` report "not-exercised ... no implementation served the
requested phase(s) oracle, subject" for this case, at both levels. Not investigated further in this
pass (git-cli oracle availability / harness registration worth checking).

**D6 — Dashboard feature file has 3 Gherkin syntax violations (LOW).**
`🎛️dashboard/🧪️tests/🌳️command-tree-projection/🥒️.feature` lines 23/31/33: `Then`/`When` clause text
wraps onto continuation lines without `And`/`But`, which `contract`'s Gherkin check flags as
"Unrecognized line." The harness tolerates it today (case still passes), but a strict third-party
Gherkin parser would not.

**D7 — 5 test cases still missing a Go adapter (LOW, actively being fixed).**
`🚚️move/🔤️rename-casings`, `🔌️mcp/🔗️event-log-chain`, `🔌️mcp/📞️tool-call-roundtrip`,
`📜️statutes/🗜️breach-cache-envelope` (plus the by-design `🎛️dashboard` case). Per the coordinator's
own log, `rename-casings` is one of the leftovers the concurrent Opus fixer is closing during this
audit window (11:01-11:32 UTC) — re-run `discover`/`parity` after that lands.

**D8 — 6 oracle manifests missing a `capability` declaration (LOW).**
`intl-segmenter` (identity), `semio-search-reference-ts` (search), `node-zlib-crypto` (statutes),
`micromatch` + `gitignore-js` (workspace, two cases), `yaml-js` (yaml) — all flagged by `contract` as
"Oracle X does not declare capability Y." Cosmetic/manifest-only fix, does not affect actual test
execution (all six cases pass).

**D9 — Minor convention deviations (LOW).**
`📐️model` and `⌨️cli` pin `serde`/`serde_json` by explicit version instead of `{ workspace = true }`
(plan §3). Coordinator durability lives inline in `🖥️server/🎛️coordinator`'s single godfile rather than
a `🛡️durability/` subdirectory (plan §2 prose implies a subdirectory; functionally equivalent). 26/29
modules have no `README.md` (not a stated outcome criterion, but several per-module reports flagged
its absence).

**Not a repo defect:** `bun x nx show projects` project-graph failure (duplicate project name collision
between two `✏️s/🔌️plugins/🧩️puzzle/...` fixture directories differing only by emoji) — pre-existing,
outside `🦑️repo`, blocks only the nx-listing verification step in this audit.
