# 📓️ Opus executor — mcp hygiene: the `repository_server` E0308, adapter `gofmt`, the duplicate SDL, the VS Code codegen residue

Owner: `🔌️mcp` (Rust crate + its Rust test adapters), every `🧪️tests/**/🐹️.go` adapter outside `⌨️cli`,
`📜️statutes`' policy catalogue (two scope strings), `🧩️vscode`'s committed codegen output. A concurrent
`cli-verbs` executor owns the cli crate and `⌨️cli/🧪️tests`; nothing there was touched. Environment for
every command below: `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`.

## 1. `semio-framework-repo-mcp` E0308 at `repository_server`

### 1.1 What was actually broken

`cargo build -p semio-framework-repo-mcp` was **green** on its own — the E0308 recorded in
`📓️opus-schema-cleanup.md` §5 lives in the *harness-generated Rust subject hosts*, which compile the
crate together with `🔌️mcp/🧪️tests/**/🦀️.rs`. The consolidation wave's `needless_pass_by_value` clippy
fix had narrowed the signature to

```rust
pub fn repository_server(repository: &Arc<dyn Repository>, profile: Profile, limits: Limits)
```

and fixed up the two in-crate call sites with `&(Arc::new(x) as Arc<dyn Repository>)`, but the two
adapters (`📋️capability-listing/🦀️.rs`, `🤝️jsonrpc-handshake/🦀️.rs`, three call sites) still passed
`Arc<RecordingRepository>` by value. `Arc<T> → Arc<dyn Trait>` is an unsizing coercion and applies in
argument position; `Arc<T> → &Arc<dyn Trait>` is not, hence E0308 in the generated hosts only.

### 1.2 The fix

Reverting to `Arc<dyn Repository>` by value compiles but re-raises the clippy lint the consolidation
wave was answering — confirmed empirically, not assumed:

```
warning: this argument is passed by value, but not consumed in the function body
    --> …\🔌️mcp\📦️packages\🦀️rust\🦀️.rs:1914:38
     = note: requested on the command line with `-W clippy::needless-pass-by-value`
```

The body genuinely needs *owned* clones (one `Arc::clone` per registered tool, resource and prompt),
so the clean signature is the consuming, generic one already used by this module's two other entry
points — `serve_stdio(repository: impl Repository + 'static, …)` and `run_with(…, repository: impl
Repository + 'static)`:

```rust
pub fn repository_server(repository: impl Repository + 'static, profile: Profile, limits: Limits) -> Result<Server, Error> {
    let repository: Arc<dyn Repository> = Arc::new(repository);
```

The argument is consumed by `Arc::new`, so the lint does not fire; the three interior `Arc::clone(&repository)`
calls are unchanged; both interior call sites lose their `&(… as Arc<dyn Repository>)` casts; and the
three adapter call sites become `mcp::repository_server(mcp::RecordingRepository::new(), …)`. All
three entry points of `🔌️mcp` now take a repository the same way. Nothing outside the crate passed an
`Arc<dyn Repository>` — grepped across every `.rs` under `🧰️framework`: the only external callers are
`📦️mcp-main.rs` (`run_with`) and `⌨️cli`'s `mcp_verb` (`serve_stdio`), neither touched.

Files: `🔌️mcp/📦️packages/🦀️rust/🦀️.rs`, `🔌️mcp/🧪️tests/📋️capability-listing/🦀️.rs`,
`🔌️mcp/🧪️tests/🤝️jsonrpc-handshake/🦀️.rs`.

### 1.3 Verification

```
$ cargo build -p semio-framework-repo-mcp
    Finished `dev` profile [unoptimized] target(s) in 1.53s

$ cargo test -p semio-framework-repo-mcp
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (doc-tests)

$ cargo clippy -p semio-framework-repo-mcp --all-targets
    Finished `dev` profile [unoptimized] target(s) in 1.11s          (no warnings)

$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity quick --owner 🔌️mcp
[test] level=quick cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
```

"All Rust hosts execute" is not inferred from the summary line — every row of the run's
`📤️results.jsonl` was counted:

| case | go subject | rust subject | typescript oracle |
| --- | --- | --- | --- |
| `📋️capability-listing` | 2 passed | **2 passed** | 2 passed |
| `📞️tool-call-roundtrip` | — (no go adapter) | **2 passed** | 2 passed |
| `🔗️event-log-chain` | — (no go adapter) | **2 passed** | 2 passed |
| `🤝️jsonrpc-handshake` | 3 passed | **3 passed** | 3 passed |

Zero `failed`, zero `errored`, no `not-exercised` line. Before the fix the two rust hosts that build
against the adapters could not compile at all.

### 1.4 `cargo build --release -p semio-framework-repo-cli`

```
$ cargo build --release -p semio-framework-repo-cli
   Compiling semio-framework-repo-cli v0.1.0 (…\⌨️cli\📦️packages\🦀️rust)
    Finished `release` profile [optimized] target(s) in 25.08s
```

The first attempt died on `error: failed to remove file C:\git\semio\target\release\semio.exe —
Access is denied. (os error 5)`: a running `semio` process (dashboard/MCP server) held the binary.
That is a host lock, not a compile error — the immediate retry above linked cleanly. No cli verb
region was edited.

## 2. `gofmt` over the harness Go adapters

23 offenders, not 24: the 24th was under `⌨️cli/🧪️tests`, which today holds no `🐹️.go` at all (the
`cli-verbs` executor is still adding them), so the count is one lower than the snapshot in
`📓️opus-schema-cleanup.md` §5.4. Every offence was the same one — import grouping, `semio.tech/repo/test`
sorted before `github.com/usalu/semio/repo/*`:

```
$ gofmt -d 🧾️yaml/🧪️tests/🔁️codec-roundtrip/🐹️.go
-	host "semio.tech/repo/test"
 	yaml "github.com/usalu/semio/repo/yaml"
+	host "semio.tech/repo/test"
```

Formatted with `gofmt -w` over `find . -path "*/🧪️tests/*" -name "🐹️.go" -not -path "./⌨️cli/*"`
(23 files across `🌳️tree` ×2, `🏃️test-runner`, `🏠️workspace` ×5, `📊️metrics` ×2, `📐️model`, `📡️events` ×2,
`🔎️search`, `🧩️providers` ×4, `🧪️test`, `🧾️yaml` ×2, `🪪️identity` ×2). Re-checked over **all 96**
adapters including `⌨️cli`:

```
$ gofmt -l $(find . -path "*/🧪️tests/*" -name "🐹️.go")
(no output)
```

Nothing but import order changed, proven by re-running the harness for two owners whose adapters were
reformatted — the numbers match the `📓️opus-schema-cleanup.md` §4 baseline (`identity 5/5 parity 4/4`)
exactly:

```
$ … parity fundamental --owner 🪪️identity
[test] level=fundamental cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4

$ … parity fundamental --owner 🧾️yaml
[test] level=fundamental cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4
```

## 3. The duplicate SDL is gone

`🔨️modules/🔌️mcp/🔗️graphql/` held nothing but the stale copy and its own bundle metadata —
`🔗️.graphql` (728 lines, against the canonical document's 607), `README.md` (whose only Docs entry was
that file), `AGENTS.md` (the bundle spec stub) and `📋️project.json` (an nx bundle registration named
`repo/graphql`, colliding with the real graphql module's Go module name). The whole folder was
deleted. The only SDL in the repo is now `🔗️graphql/🧬️schema/🔣️schema.graphql`, the one both
implementations render and `graphql-js` reads in `📜️sdl-dump`.

References, grepped repo-wide over `*.json,ts,js,yml,yaml,jsonc,md,go,rs,toml` excluding
`.🧬semio/…/🎫️tickets`:

- **Nothing imports, embeds or reads the file.** No `include_str!`, no `go:embed`, no codegen config,
  no nx target input.
- The only live references were two identical scope lists in the `repo` policy of the statute
  catalogue: `📜️statutes/🧬️schema/🔣️statutes.json` (policy scopes + its `Parity` group scopes) and the
  hand-written twin table in `📜️statutes/📦️packages/🐹️go/🐹️.go`. Both entries changed
  `graphql/repo/🔗️.graphql` → `graphql/repo/🔣️schema.graphql`, keeping the sibling entries' pseudo-path
  shape (`go/repo/main.go`, `js/vscode/package.json`). These scopes are declarative labels: `repoPolicy`
  only ever `ReadText`s `go/repo/main.go`, never the graphql scope — verified by reading the whole
  function.

```
$ cargo test -p semio-framework-repo-statutes            → ok (Rust reads the catalogue through
                                                            include_str!("../../🧬️schema/🔣️statutes.json"))
$ go test ./…/📜️statutes/📦️packages/🐹️go                  → ok  github.com/usalu/semio/repo/statutes  1.024s
$ … parity fundamental --owner 📜️statutes
[test] level=fundamental cases=5 executed=18 passed=18 failed=0 errored=0 parity=9/9
$ gofmt -l 📜️statutes/📦️packages/🐹️go/🐹️.go              → (clean)
```

## 4. `🧩️vscode`: the `fix`/`FixResult` codegen residue

### 4.1 There is no generator to re-run

The extension package declares **no** graphql-codegen: `package.json` has no `scripts`, no
`@graphql-codegen/*` dependency (devDependencies are only vsce, node/vscode types, the vscode test
cli/electron, typescript and vite), there is no `codegen.yml`/`.ts` anywhere in the module, and
`📜️script.ts` (`dev|test|build|lint|build-vsix`) never mentions graphql. The codegen output is
committed by hand into the one big `🟦️.ts`, so it was hand-fixed, as the fallback allows.

### 4.2 What was removed from `🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`

Seven blocks, all of them the removed mutation's footprint:

| Region | Removed |
| --- | --- |
| `🧬️CodegenGraphql` | `export type FixResult = { fixed, remaining, breachs }` |
| `🧬️CodegenGraphql` | `fix: FixResult;` on `export type Mutation` |
| `🧬️CodegenGraphql` | `export type MutationFixArgs = { scope? }` |
| `🧬️CodegenGraphql` | `export type FixMutationVariables` and `export type FixMutation` |
| `🧬️CodegenGraphql` | `export const FixDocument = { "kind": "Artifact", … }` (the `DocumentNode`) |
| `🧬️CodegenGql` | the `graphql(source: "\n  mutation Fix($scope: String) …")` overload |
| `⌛️Queries` | `export const FixDocument = graphql(\`mutation Fix($scope: String) { fix(…) … }\`)` |

`grep -n "FixResult\|FixDocument\|FixMutation\|MutationFixArgs\|mutation Fix"` over `🟦️.ts` and
`🧪️extension.test.ts` now returns nothing. No call site had to change: nothing in the extension body
ever referenced `FixDocument` — the two `export const FixDocument` declarations (one per region) were
in fact a duplicate-export defect that went with them.

### 4.3 The build

**Exact command that succeeds** (run from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️vscode/📦️packages/🟦️typescript`):

```
$ bun ./📜️script.ts build
…
out/extension.js                             …
out/test/extension.test.js                   5,422.44 kB │ gzip: 1,216.33 kB
✓ built in 16.89s                                                            (exit 0)
```

This is verbatim the command `📋️project.json`'s `build` target runs. **The nx wrapper
`bun x nx run @semio-tech/repo-vscode:build` cannot run at all right now**, and not because of this
project — the whole project graph is refused by a duplicate project name from another fleet's
in-flight rename:

```
NX   Failed to process project graph.
The following projects are defined in multiple locations:
- test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-third-party-puzzle-2d-1:
  - ✏️s/🔌️plugins/🧩️puzzle/…/🧪️tests/🌐️third-party-puzzle-2d-1
  - ✏️s/🔌️plugins/🧩️puzzle/…/🧪️tests/🕸️third-party-puzzle-2d-1
```

Reproduced twice, minutes apart. Nothing under `🦑️repo` contributes to it; every nx target in the
repo is blocked until that duplicate is resolved by its owner.

### 4.4 One forced edit outside `🦑️repo`

The extension build failed *before* any of my edits, and for an unrelated reason:

```
RollupError: Module format "cjs" does not support top-level await.
file: C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts
 code: "INVALID_TLA_FORMAT"
```

The extension imports `ephemeralBox`/`ephemeralMap` from the `@semio-tech/framework` barrel, which
pulls 271 modules in, among them that one. Its in-source vitest block carried a misindented
`const { default: fixture } = await import("../🧪️fixtures/🏷️fields/🔣️.json");` at the top level of
`if (import.meta.vitest) { … }` — genuine top-level await, fatal for every CommonJS consumer of the
barrel. `fixture` is used only inside the single `it(…)`, never by the `prepared` helper, so the
import moved into that test and its callback became `async`. Behaviour under vitest is unchanged; the
module no longer has top-level await. File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts`
(last committed by another fleet in `fe7c8a8f8b`, clean in the working tree when edited).

## 5. Left open

1. **The nx project graph is broken repo-wide** by the duplicate `third-party-puzzle-2d-1` test case
   under `✏️s/🔌️plugins/🧩️puzzle/…` (§4.3). Its owner must drop one of the two directories.
2. **`🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`'s codegen output is corrupt beyond the `fix` field.**
   There is no `const documents = { … }` object in the file, yet `graphql()` returns
   `(documents as any)[source] ?? {}` and several `graphql(source: "…")` overloads carry visibly
   truncated/spliced query strings (e.g. `"… into a d\n    analyze(scope: $scope) …"`,
   `"query Codebase { repo { id nam definitions lines breachs }"`). Every `⌛️Queries` document is
   therefore `{}` at runtime. Restoring this needs the generator to be reintroduced as a zero-touch
   target against `🔗️graphql/🧬️schema/🔣️schema.graphql` — a `wiring`-wave job, larger than this ticket
   item.
3. **The `repo` statute policy is itself legacy.** Its remaining scopes name `go/repo/main.go` (gone
   with the old tree) and its `canonicalCommands` list still contains `"fix"`, so the parity check it
   performs can only ever report "Could not read go/repo/main.go". The scope rename in §3 keeps the
   catalogue honest about the SDL but does not rehabilitate the policy.
4. **`📚️library/…/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json`** still names
   `💻️client/🔌️mcp/🔗️graphql/README.md` as a source/destination path — a pre-existing reference to the
   pre-move tree, unrelated to the file deleted here.
5. **Committed binaries remain** (`🔌️mcp/📦️packages/🐹️go/mcp.exe`, `⌨️cli/📦️packages/🐹️go/semio-repo.exe`,
   `⌨️cli/📦️packages/🐹️go/🚀️bin/bin.exe`); plan §2 wants them gone. One of them held
   `target/release/semio.exe`'s lock in §1.4.
6. **`📜️statutes` keeps two copies of the policy catalogue** — the schema JSON that Rust reads and a
   hand-written Go table that duplicates it. They were edited in lockstep here; schema-first (plan §3)
   wants the Go side to embed the JSON instead.

## 6. Files touched

- `🔌️mcp/📦️packages/🦀️rust/🦀️.rs` — `repository_server` takes `impl Repository + 'static`
- `🔌️mcp/🧪️tests/📋️capability-listing/🦀️.rs`, `🔌️mcp/🧪️tests/🤝️jsonrpc-handshake/🦀️.rs` — three call sites
- 23 × `<module>/🧪️tests/<case>/🐹️.go` — `gofmt -w` (import grouping only)
- `🔌️mcp/🔗️graphql/{🔗️.graphql, README.md, AGENTS.md, 📋️project.json}` — **deleted** (folder removed)
- `📜️statutes/🧬️schema/🔣️statutes.json`, `📜️statutes/📦️packages/🐹️go/🐹️.go` — two scope strings each
- `🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` — seven `fix`/`FixResult` blocks removed
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts` — top-level await moved into its test
