# 📓️ Opus executor report — `🌳️tree`

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree`
Rust crate: `semio-framework-repo-tree` · Go package: `github.com/usalu/semio/repo/tree`
This session resumed the job after a rate limit killed the previous agent mid-way (its last note:
"Now the Go adapters").

## 1. What the module holds now

```
🌳️tree/
  🧬️schema/🔣️.json
  🔮️oracle/🔣️.json                      3 recorded no-oracle decisions, 0 oracles
  🧫️fixtures/                            🌳️tree-source.json, 📤️tree-build-expectations.json,
                                          🎯️goal-statute-territory.json,
                                          📤️goal-statute-territory-expectations.json,
                                          🔎️filter-vectors.json, 🔀️sort-vectors.json,
                                          🧜️mermaid-vectors.json
  🧪️tests/🌳️monorepo-tree-build/          🥒️.feature 🦀️.rs 🐹️.go
  🧪️tests/🔎️tree-filtering/               🥒️.feature 🦀️.rs 🐹️.go
  🧪️tests/🔀️child-sorting/                🥒️.feature 🦀️.rs 🐹️.go
  🧪️tests/🧜️mermaid-rendering/            🥒️.feature 🦀️.rs 🐹️.go
  🧪️tests/🎯️goal-statute-territory-trees/ 🥒️.feature 🦀️.rs 🐹️.go   ← 🐹️.go written this session
  📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs (2 249 lines), 📋️project.json, 📜️script.ts}
  📦️packages/🐹️go/{go.mod, 🐹️.go, 📋️project.json, 📜️script.ts, 🔬️_test.go, 🔭️exhaustive_test.go}
```

Rust regions: `🧲️Header 🏷️Vocabulary 🔌️Ports 💿️Records 🌿️NodeHelpers 🩻️MonorepoTree 🎯️GoalTree
📜️StatuteTree 🔀️Sorting 🧹️Filtering 🔎️Search 🎨️Rendering 🧜️Mermaid 📌️Cache 🔣️Encoding 🧪️Tests`.
The crate never touches the filesystem: `TreeSource`, `ArtifactIdentifier`, `EntityRenderer` and
`StatuteCatalog` are traits; the fixtures satisfy them through `MemoryTreeSource` and
`MemoryStatuteCatalog`.

## 2. Changes made this session

### 2.1 Cache envelope — statutes finding 7a applied

`📓️opus-statutes.md` finding 7a (lines 97–101) records that the gzip+sha256 envelope implemented in
`📜️statutes` **is** the tree cache and that `🌳️tree` should reuse it. The crate had a second
hand-rolled SHA-256 (61 lines) and no gzip half at all. Now:

- `Cargo.toml` gained `semio-framework-repo-statutes` (path dep). DAG-legal: statutes is L2, tree is
  L3, and the Go `go.mod` already declares the same edge.
- The duplicate `sha256_hex` body was deleted; the crate re-exports the statutes codec:
  `pub use semio_framework_repo_statutes::{gzip_decode, gzip_encode, sha256_hex};` — so
  `tree_content_digest` keeps its public signature and one hand-rolled SHA-256 and one hand-rolled
  gzip now serve both caches.
- The missing payload half was added, mirroring Go's `saveTreeCache`/`loadTreeCache`
  (`📦️packages/🐹️go/🐹️.go:2490-2552`, `tree.json.gz`):
  `pub fn encode_tree_cache(&TreeNode) -> Vec<u8>` and
  `pub fn decode_tree_cache(&[u8]) -> Result<TreeNode, String>`.
- New unit test `cache_envelope_round_trips_through_the_shared_gzip_codec` asserts the gzip magic
  `1f 8b 08`, a lossless round trip and an error on non-gzip input.

Net: −61 lines of duplicated SHA-256, +12 lines of envelope, one codec owner.

### 2.2 Clippy

The crate carried 8 workspace-lint warnings. All fixed at the source, none silenced with `#[allow]`:

| Site | Lint | Fix |
| --- | --- | --- |
| `statute_label` | `map_unwrap_or` | `map_or_else` |
| `statute_leaf_node`, statute-tree leaf | `map_unwrap_or` (×2) | `map_or(BreachPriority::Low, …)` |
| `kind_set`, `sub_kind_map` (in `to_filter`) | `unnecessary_wraps` | return the map, `Some(…)` at the call site |
| `data_of` | `unnecessary_wraps` | returns `BTreeMap`, 20 call sites wrapped in `Some(…)`; `goal_node_data`/`ticket_node_data` return it directly |
| `sort_goal_nodes` | `ptr_arg` | `&mut [GoalNode]` |
| fuzzy search window | `int_plus_one` | `start < word.len()` |

### 2.3 Fifth Go adapter

`🧪️tests/🎯️goal-statute-territory-trees/🐹️.go` did not exist (the other four did). Written as the
twin of the Rust subject, same six scenario ids, same stub renderer stated by the feature, same
`require`/`stringList`/`lines` helpers as the sibling adapters, `Adapter() *host.Adapter` registering
`builds-the-goal-tree`, `renders-the-goal-tree`, `builds-the-statute-tree`,
`builds-the-territory-tree`, `groups-statutes-by-entity-kind`, `counts-open-subgoals-and-tickets`.

### 2.4 launch.json

`.vscode/🧩️launch.seed.jsonc` gained three entries in the existing test group, placed between
`🧪️test🧰️repo📜️statutes🥒️parity` and `🧪️test🧰️repo🪝️hooks🦀️rust`:

- `🧪️test🧰️repo🌳️tree🦀️rust` → `bun nx run @semio-tech/repo-tree-rs:test`
- `🧪️test🧰️repo🌳️tree🐹️go` → `bun nx run framework-products-repo-modules-tree-go:test`
- `🧪️test🧰️repo🌳️tree🥒️parity` → `… 🧪️test/📜️script.ts parity fundamental --owner …/🌳️tree`

Regenerated:

```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …/🤖️generated
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
```

## 3. Verification (real output)

### 3.1 `cargo test`

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-tree
   Compiling semio-framework-repo-tree v0.1.0 (…\🌳️tree\📦️packages\🦀️rust)
    Finished `test` profile [unoptimized] target(s) in 1.57s
     Running unittests 🦀️.rs (target\debug\deps\semio_framework_repo_tree-272ce81c925855e5.exe)

running 4 tests
test tests::cache_envelope_round_trips_through_the_shared_gzip_codec ... ok
test tests::digest_is_stable_and_content_addressed ... ok
test tests::mermaid_treemap_indents_by_four_spaces_per_level ... ok
test tests::sorts_folders_before_files_then_by_label ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 3.2 `cargo clippy`

```
$ RUSTC_WRAPPER="" cargo clippy -p semio-framework-repo-tree --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.53s
```

Zero warnings attributed to `semio-framework-repo-tree` (lib and lib test). Two warnings remain in
`semio-framework-repo-identity` (`map_unwrap_or`, `🦀️.rs:168`, `:173`) and one in
`semio-framework-repo-statutes` (`needless_lifetimes`, `🦀️.rs:438`) — other executors' crates, left
untouched.

### 3.3 Harness — oracle

```
$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts oracle fundamental --owner …/🌳️tree
[test] not-exercised …/🔀️child-sorting (recorded no-oracle decision repo-tree-filtering-and-sorting — its evidence is discharged by the subject phase)
[test] not-exercised …/🎯️goal-statute-territory-trees (recorded no-oracle decision repo-tree-projection — …)
[test] not-exercised …/🧜️mermaid-rendering (recorded no-oracle decision repo-tree-mermaid — …)
[test] not-exercised …/🌳️monorepo-tree-build (recorded no-oracle decision repo-tree-projection — …)
[test] not-exercised …/🔎️tree-filtering (recorded no-oracle decision repo-tree-filtering-and-sorting — …)
[test] level=fundamental cases=5 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=5
```

The registry accepts all three no-oracle decisions of `🔮️oracle/🔣️.json`.

### 3.4 Harness — Rust subject

```
$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts subject fundamental --owner …/🌳️tree --implementation rust
[test] level=fundamental cases=5 executed=12 passed=12 failed=0 errored=0 parity=0/0

$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts subject quick --owner …/🌳️tree --implementation rust
[test] level=quick cases=5 executed=19 passed=19 failed=0 errored=0 parity=0/0
```

Green: 12/12 at `fundamental`, 19/19 at `quick` (the extra 7 are the `@mode-property` metamorphic
scenarios — sorting and filtering idempotence, escaping idempotence).

### 3.5 Harness — parity: **RED, blocked on the Go package** (run last, as instructed)

```
$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts parity fundamental --owner …/🌳️tree
[test] …/🌳️monorepo-tree-build: go subject host exited 1 without emitting results
[test] # semio.test/host
.\adapter.go:54:14: undefined: tree.ParseTreeSource
.\adapter.go:131:18: undefined: tree.TreeOutline
.\adapter.go:164:47: too many arguments in call to tree.RenderMonorepoTree
	have (*tree.TreeNode, stub)
	want (*tree.TreeNode)
.\adapter.go:165:59: too many arguments in call to tree.RenderMonorepoTreeMarkdown
	have (*tree.TreeNode, stub)
	want (*tree.TreeNode)
.\adapter.go:185:37: too many arguments in call to tree.PropagateParentIDs
	have (*tree.TreeNode, string, stub)
	want (*tree.TreeNode, string)
.\adapter.go:206:22: undefined: tree.TreeContentDigest
[test] …/🌳️monorepo-tree-build: no result stream at …\📤️results.jsonl
[test] …/🌳️monorepo-tree-build: no-oracle decision repo-tree-projection claims the
       independent-implementations substitute but only one implementation ran
[test] …/🔎️tree-filtering: go subject host exited 1 without emitting results
.\adapter.go:41:20: undefined: tree.ParseTreeNodeSpec
.\adapter.go:54:20: undefined: tree.ParseTreeFilterSpec
…
```

Parity cannot pass while the Go subject does not compile. The cause is **not** in the adapters:
`📦️packages/🐹️go/🐹️.go` is the go-split output of the original `component.go`, which renders and
propagates through package-level globals instead of ports and exposes no fixture/outline helpers. It
is owned by the concurrent `go-split` executor and was **not** edited here.

**Go API the five adapters need** (Go names are the exported PascalCase twins of the Rust items —
the naming the four pre-existing adapters already assumed):

| Missing / wrong in Go | Rust twin |
| --- | --- |
| `ParseTreeSource([]byte) (*MemoryTreeSource, error)` | `MemoryTreeSource::from_json` |
| `ParseStatuteCatalog([]byte) (*MemoryStatuteCatalog, error)` | `MemoryStatuteCatalog::from_json` |
| `ParseTreeNodeSpec` / `TreeNodeSpec.ToTreeNode` | `TreeNodeSpec` |
| `ParseTreeFilterSpec` / `TreeFilterSpec` | `TreeFilterSpec::to_filter` |
| `TreeOutline(*TreeNode) []string` | `tree_outline` |
| `TreeContentDigest(*TreeNode) string` | `tree_content_digest` |
| `SortTreeChildren` (currently unexported `sortTreeChildren`, `🐹️.go:1389`) | `sort_tree_children` |
| `BuildGoalTree`, `GoalNode`, `TicketNode`, `CountOpenSubgoals`, `CountOpenTickets` | same |
| `RenderGoalTreeNodes(roots, format, EntityRenderer)`, `TreeRenderFormatText/Markdown` | `render_goal_tree_nodes`, `TreeRenderFormat` |
| `DecodeStatutes`, `DecodeTerritories`, `BuildStatuteTree`, `BuildTerritoryTree`, `BuildPolicyEntityKindTree` | same |
| `MermaidTreemap`, `MermaidNode`, `ParseMermaidTreemap`, `RenderMermaidTreemap`, `MermaidTreemapFromTree`, `MermaidEscapeLabel` | same |
| `RenderMonorepoTree(*TreeNode, EntityRenderer)` — Go takes 1 arg | `render_monorepo_tree` |
| `RenderMonorepoTreeMarkdown(*TreeNode, EntityRenderer)` — Go takes 1 arg | `render_monorepo_tree_markdown` |
| `PropagateParentIDs(*TreeNode, string, ArtifactIdentifier)` — Go takes 2 args | `propagate_parent_ids` |

The three signature mismatches are the substantive ones: the Go package must take the renderer and
the artifact identifier as **ports**, exactly as the Rust crate does, or the language-agnostic tests
cannot state one contract for both languages.

Also relevant to the harness itself: `materializeGoHost` writes a `go.mod` that only `replace`s the
test host and sets `GOWORK: "off"` (see `📓️harness-verification.md` §2). Once the Go package exports
the API above, the generated host will additionally need a `replace` for
`github.com/usalu/semio/repo/tree` (and its transitive siblings) — otherwise the import will not
resolve even with a correct package. Flagged for the harness owner; not changed here.

## 4. Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🦀️rust/Cargo.toml` — statutes dep
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🦀️rust/🦀️.rs` — codec reuse, gzip
  envelope, clippy fixes, new unit test
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/🧪️tests/🎯️goal-statute-territory-trees/🐹️.go` — new
- `.vscode/🧩️launch.seed.jsonc` — three entries
- `.vscode/launch.json` — regenerated

## 5. Left open

1. **Parity is red** until `go-split` exports the API in §3.5 from
   `🌳️tree/📦️packages/🐹️go/🐹️.go`, including the three port-taking signatures. Re-run
   `parity fundamental --owner …/🌳️tree` then.
2. `materializeGoHost` needs `replace` directives for domain modules (harness owner).
3. Test-case directory names already carry leading emojis, so the audit-wave rename of §5 of the plan
   does not apply to this module.
