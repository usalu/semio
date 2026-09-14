# 📓️ `📐️model` — Rust twin, schema and language-agnostic tests

Agent: Opus 5 executor `model` (wave 2). Module:
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/` (new).
Source of truth read: the frozen Go snapshot
`$TICKET/🗑️generated/go-snapshot/client/🧩️component.go`, regions `💡️GraphQL Types` (9881–12346),
`🎺️Models` (5315–5555), `⭐️Codebase Types` (16444–16747), plus the pure shapes those close over
inside `⚙️Types`.

## 1. What exists now

```
📐️model/
├── 🧬️schema/🔣️.json                       141 $defs (128 object types, 12 enums, 1 table shape)
├── 🧬️schema/🔣️allowed-values.json         77 LLM · 4 effort · 12 client slugs
├── 🔮️oracle/🔣️.json                       1 oracle (ajv-model-schema), 2 no-oracle decisions
├── 📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📋️project.json,📜️script.ts}
├── 📦️packages/🐹️go/{go.mod,🐹️.go,📋️project.json,📜️script.ts}
└── 🧪️tests/{slug-normalisation, definition-kind-derivation, json-encoding-conformance}/
```

### 1.1 Schema (`🧬️schema/🔣️.json`)

JSON Schema draft 2020-12. One `$def` per domain type carrying the **exact** JSON member names the
Go struct tags declare; a member absent from `required` is the `omitempty` half of that tag. Every
slug and kind vocabulary is an enum `$def`: `DefinitionKind`, `TicketStatus`, `GoalStatus`,
`BreachPriority`, `TechnologyKind` (emoji-valued), `BundleKind`, `FolderKind`, `FileKind`,
`SemanticChangeType`, `TreeNodeKind` (17), `CbTreeNodeKind`, plus `AllowedValues` for the vocabulary
table itself. Objects are deliberately **not** closed (`additionalProperties` is left open) because
neither decoder rejects an unknown member.

It is **generated** from the Rust crate by `$TICKET/🗑️generated/model/build-schema.ts`, so the
schema and the two implementations cannot drift; re-run that script after any shape change. The
allowed-value table is extracted verbatim from the Go `AllowedLLMs`/`AllowedEfforts`/`AllowedClients`
vars by `$TICKET/🗑️generated/model/extract-allowed.ts` — the strings were never retyped.

### 1.2 Rust crate `semio-framework-repo-model`

`📦️packages/🦀️rust/{Cargo.toml → lib path 🦀️.rs}`, `[lints] workspace = true`,
`[package.metadata.semio] role = "library"`, dependencies `serde` + `serde_json` only. Registered in
the root `Cargo.toml` members list. Regions per aggregate: `🪶️Encoding Helpers`, `❌️Errors`,
`📋️Allowed Values`, `🔤️Slugs`, `🏷️Enumerations`, `🔢️Metrics`, `🌐️Repo Aggregate`,
`🧑️Contributor Aggregate`, `🔀️Checkpoint Aggregate`, `💬️Interaction Aggregate`,
`🎫️Ticket Aggregate`, `🎯️Goal Aggregate`, `📝️Draft Aggregate`, `✅️Todo Aggregate`,
`👮️Statute Aggregate`, `🖥️Codebase Aggregate`, `🌳️Tree Aggregate`, `📥️Input Types`,
`🔁️Wire Round Trip`.

Go→serde mapping actually applied (all verified byte-for-byte, §3):

| Go | Rust |
| --- | --- |
| `X \`json:"x"\`` | `#[serde(rename = "x")]` |
| `string,omitempty` | `skip_serializing_if = "String::is_empty"` |
| `int,omitempty` / `bool,omitempty` | `skip_serializing_if = "is_zero"` / `"is_false"` |
| `*T,omitempty` | `Option<T>` + `skip_serializing_if = "Option::is_none"` |
| `[]T,omitempty` / `map,omitempty` | `Vec<T>` / `BTreeMap` + `skip_serializing_if = "…is_empty"` |
| `[]T` / `map[string]T` **without** omitempty | `Option<Vec<T>>` / `Option<BTreeMap<..>>` — a Go `nil` encodes as `null`, an empty one as `[]`/`{}`, and only `Option` reproduces both |
| any `map` | `BTreeMap`, so key order matches Go's sorted map encoding |
| `json:"-"` | `#[serde(skip)]` |
| `time.Time` | `String` (RFC 3339 on the wire either way; no date crate enters the graph) |
| no tag at all (`🎺️Models` region) | `#[serde(rename = "<GoFieldName>")]` — `TreeNode`, `TicketNode`, `GoalNode`, `SemanticChange`, `DiffLines`, `Contributor*` tree nodes |

### 1.3 Public Rust API other crates will use

```rust
// vocabularies — the table is loaded from 🧬️schema/🔣️allowed-values.json with include_str!
pub fn allowed_llms() -> &'static [String];
pub fn allowed_efforts() -> &'static [String];
pub fn allowed_clients() -> &'static [String];
pub fn normalize_llm_slug(llm: &str) -> String;
pub fn normalize_effort_slug(effort: &str) -> String;
pub fn normalize_client_slug(client: &str) -> String;
pub fn resolve_allowed_llm(llm: &str)       -> Result<String, ModelError>;
pub fn resolve_allowed_effort(effort: &str) -> Result<String, ModelError>; // blank input ⇒ Ok("")
pub fn resolve_allowed_client(client: &str) -> Result<String, ModelError>;

// derivations
pub fn derive_definition_kind(raw_kind: &str) -> DefinitionKind;   // total
pub fn derive_technology_kind(name: &str)     -> TechnologyKind;   // total

// errors — a class, not a rendered string
pub enum ModelError { NotAllowed { field: &'static str, slug: String, allowed: Vec<String> } }
impl ModelError { pub fn class(&self) -> &'static str; }   // "not-allowed"
// Display reproduces the Go message verbatim:
//   "llm '<slug>' is not allowed. Please use one of: <joined>"

// wire contract — owned entry point, so no serialisation crate appears in any signature
pub fn wire_type_names() -> &'static [&'static str];                       // 60 types
pub fn round_trip_json(type_name: &str, text: &str) -> Result<String, String>;

// behaviour on the tree filter
impl TreeFilter {
    pub fn has_only_kinds(&self) -> bool;
    pub fn is_kind_visible(&self, kind: TreeNodeKind) -> bool;   // Category is always visible
    pub fn matches_sub_kind(&self, kind: TreeNodeKind, sub_kind: &str) -> bool;
    pub fn matches_date(&self, year: i64, month: i64, day: i64) -> bool;
    pub fn matches_status(&self, status: &str) -> bool;
    pub fn matches_contributor(&self, contributor: &str) -> bool;
}
impl Territory { pub fn all_kinds(&self) -> Vec<Statute>; }

// vocabularies as data
pub const ENTITY_KINDS / ARTIFACT_KINDS / DIFFABLE_KINDS / RELATED_TO_FILE_KINDS: &[&str];

// every enum: as_str() + Display; Statute is a transparent String newtype
```

Plus the ~110 data types themselves, all
`#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]`.

**Two deliberate deviations from a literal port**, both documented in the source:

* `slugify` is a **private** helper, not public API. It is the repo's camel-case-aware slug rule that
  `🪪️identity` owns for id building; `📐️model` sits at the bottom of the dependency DAG and may not
  import a peer, and the three vocabularies are defined in terms of it. Go's *simple* case mapping is
  reproduced exactly (`go_to_upper`/`go_to_lower` keep `ß` as `ß` where Rust's full mapping would
  produce `SS`) — covered by the `Grüße` and `ß` vectors.
* `Statute` is a transparent `String` newtype, not the ~200-member Go enum, and `Territory` is
  carried only as the shape `Policy` needs. The closed statute vocabulary and its metadata table
  belong to `📜️statutes`; duplicating it here would have created two sources of truth for that
  agent's module.

### 1.4 Go package `github.com/usalu/semio/repo/model`

`📦️packages/🐹️go/{go.mod, 🐹️.go, 📋️project.json, 📜️script.ts}`, added to the root `go.work`.
It contains **only** the allowed-values loader plus a `📐️Pending` region comment, exactly as
briefed — the type definitions are NOT copied, because they still live in the godfile and a second
declaration would break the build the moment the split lands.

The loader follows the precedent the concurrent `🔌️mcp` agent set for the same problem: `go:embed`
cannot reach a parent directory, and the table is owned by the *module* rather than by the Go
package, so it resolves `🧬️schema/🔣️allowed-values.json` relative to the source file
(`runtime.Caller`) with a repo-root walk-up fallback and a `SEMIO_REPO_MODEL_ALLOWED_VALUES`
override. Surface: `Allowed()`, `AllowedLLMs()`, `AllowedEfforts()`, `AllowedClients()`.

The `📐️Pending` region names every symbol the wave-2 `go-split` agent must MOVE here, by name and by
snapshot line range, grouped by source region — including an explicit **STAYS BEHIND** list
(`Slugify`/`Flat`/`emojiText`/id builders → `🪪️identity`; `Statute` + `Territory` → `📜️statutes`;
the draft/interaction/diff functions → their own modules; `FilterInput.ToStreamOptions` → `🔎️search`)
and an instruction to DELETE the three `AllowedLLMs`/`AllowedEfforts`/`AllowedClients` var blocks
rather than move them, since this file's loader replaces them.

### 1.5 Language-agnostic tests

Owner-level `🔮️oracle/🔣️.json` (schemaVersion 2, at the taxonomy's `testContributionDirName`
location) declares one oracle and two no-oracle decisions.

| Case | Scenarios | Subjects | Oracle |
| --- | --- | --- | --- |
| `slug-normalisation` | 4 (normalise+idempotence, resolve, reject→error class, table identity) | rust, go | none — decision `repo-slug-vocabulary` |
| `definition-kind-derivation` | 2 (per-keyword kind, totality) | rust, go | none — decision `repo-definition-kind` |
| `json-encoding-conformance` | 1 (60 golden documents round-trip) | rust, go | `ajv-model-schema` (ajv 8.20.0, draft 2020-12) |

* Fixtures freeze **inputs only** for the two no-oracle cases — writing an answer key by hand would
  let a shared misreading pass. The LLM vectors deliberately include `XMLHttpRequest`, `aBcDeF`,
  `Grüße`, `ß`, `"  gemini 3 pro  "`, `SWE-1.5` and `anthropic/claude-opus-4-1` to pin camel-case
  splitting, Unicode case mapping and longest-containment resolution.
* `🧫️fixtures/🔣️goldens.json` holds 60 golden documents, one per type, whose `json` is the exact
  text a conforming encoder must reproduce: members in declaration order, map keys sorted, and no
  member present that an `omitempty` tag would drop. The oracle validates each against
  `🧬️schema/🔣️.json` with real ajv *before* re-serialising it, so a drifted schema fails instead of
  agreeing with itself; its `JSON.parse`/`stringify` is additionally a third, independently written
  serialiser of the same bytes.
* The Go adapters import the **current** implementation `github.com/usalu/semio/repo/client`
  (the godfile package), each with the marker
  `// 🚚️ repoint to github.com/usalu/semio/repo/model after split`, so parity holds today.
* The Rust adapters gate their subject halves behind the `sut` feature the generated host turns on
  for the subject role only, per the host's documented convention.

## 2. One framework change I had to make

`🔨️modules/🧪️test/📜️script.ts` → `materializeGoHost`. The generated Go host runs with `GOWORK=off`
and previously required only the host module and (after the concurrent foundation agent's
`goSutModule`) the owner's own module. A `replace` directive only takes effect from the **main**
module, so requiring `github.com/usalu/semio/repo/client` failed with
`unknown revision repo/events/v0.0.0` for all six of its in-repo dependencies. Added
`goWorkspaceModules(repoRoot)`: every member `go.work` lists gets a `require` + `replace`, and the
generated `go` directive now follows `go.work`'s (`1.23` → `1.25`, otherwise a `go 1.25` dependency
is rejected). It names no case, owner or module, and it is what lets an adapter test a domain whose
behaviour is still mid-migration in a neighbouring module.

## 3. Verification — real command output

**Both implementations round-trip all 60 goldens byte-for-byte** (standalone probes, before the
harness was wired):

```
$ cargo run --manifest-path <scratch probe>          # links semio-framework-repo-model
checked=60 bad=0
$ GOWORK=off GOFLAGS=-mod=mod go run .               # links github.com/usalu/semio/repo/client
checked= 60 bad= 0
```

**Builds and lints:**

```
$ cargo build -p semio-framework-repo-model
    Finished `dev` profile [unoptimized] target(s) in 5.98s
$ cargo clippy -p semio-framework-repo-model
    Finished `dev` profile [unoptimized] target(s) in 6.74s        # zero warnings
$ go build ./…/📐️model/📦️packages/🐹️go/... && go vet ./…/📐️model/📦️packages/🐹️go/...
GO-BUILD-VET-OK
$ bun nx run @semio-tech/repo-model-rs:build
 NX   Successfully ran target build for project @semio-tech/repo-model-rs
$ bun nx run @semio-tech/repo-model-go:test
?   	github.com/usalu/semio/repo/model	[no test files]
 NX   Successfully ran target test for project @semio-tech/repo-model-go
$ cargo test -p semio-framework-repo-model
test result: ok. 0 passed; 0 failed; 0 ignored
```

**Protocol v2 harness** (`🔨️modules/🧪️test`):

```
$ bun ./📜️script.ts discover | grep model
test-…-model-b7298f-📦️json-encoding-conformance   …/🧪️tests/json-encoding-conformance   [rust,typescript,go]
test-…-model-b7298f-🔤️slug-normalisation          …/🧪️tests/slug-normalisation          [rust,go]
test-…-model-b7298f-🧩️definition-kind-derivation  …/🧪️tests/definition-kind-derivation  [rust,go]

$ bun ./📜️script.ts contract | grep 📐️model
  testing/dependency  …/📐️model/📦️packages/🦀️rust/🦀️.rs  Production source imports the registered oracle serde-json-equation-carrier-reader

$ bun ./📜️script.ts subject quick --owner …/📐️model
[test] level=quick cases=3 executed=14 passed=14 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle quick --owner …/📐️model
[test] not-exercised …/definition-kind-derivation (recorded no-oracle decision repo-definition-kind — its evidence is discharged by the subject phase)
[test] not-exercised …/slug-normalisation (recorded no-oracle decision repo-slug-vocabulary — its evidence is discharged by the subject phase)
[test] level=quick cases=3 executed=1 passed=1 failed=0 errored=0 parity=0/0 not-exercised=2

$ bun ./📜️script.ts parity quick --owner …/📐️model
[test] level=quick cases=3 executed=15 passed=15 failed=0 errored=0 parity=9/9

$ bun ./📜️script.ts parity exhaustive --owner …/📐️model
[test] level=exhaustive cases=3 executed=15 passed=15 failed=0 errored=0 parity=9/9
```

`contract` reports **no** taxonomy, oracle or contract breach for this owner. The single remaining
`testing/dependency` line is discussed in §5.

**launch entries** — `.vscode/🧩️launch.seed.jsonc` gained `🧪️test🧰️repo📐️model🦀️rust`,
`🧪️test🧰️repo📐️model🐹️go` and `🧪️test🧰️repo📐️model🥒️parity` in the existing test group, then:

```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …/🤖️generated
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
```

`git diff --numstat` on the seed is `217 0` — additions only.

## 4. Incident and recovery (disclosed in full)

While adding the launch entries, a Python heredoc lost a backslash level, the resulting
`UnicodeEncodeError` fired mid-write and **truncated `.vscode/🧩️launch.seed.jsonc` to 0 bytes**. I
restored it with `git show :<path> > <path>` (a read-only git command; no `checkout`, `stash` or
`commit` was run at any point). The restore was byte-identical to the index, and I verified the file
still carried the concurrent agents' entries at their original line numbers
(`🛠️dev🧰️repo🔌️mcp🦀️rust` at 812, `🛠️dev🧰️repo🔌️mcp🐹️go` at 823,
`🧪️test🧰️repo{🔌️mcp,📡️events,🔗️graphql}{🦀️rust,🐹️go}` at 6289–6324) — nothing was lost. All
later edits to that file went through the Edit tool. **Nobody else needs to act on this**, but it is
recorded so the audit wave can confirm the file's integrity independently.

## 5. What is left

1. **`serde_json` vs the dependency rule.** `contract` reports
   `Production source imports the registered oracle serde-json-equation-carrier-reader` for
   `📦️packages/🦀️rust/🦀️.rs`. Some owner has registered `serde_json` as a test oracle, which makes
   every production `use serde_json` a `testing/dependency` breach — there are **193** of them
   repo-wide today and mine joins that set. Plan §3 explicitly permits `serde`/`serde_json` as the
   only external runtime crates, so this is a standing conflict between two rules, not a defect of
   this module. It needs a repo-level decision (re-classify that registration, or exempt the
   workspace-pinned `serde_json`); I did not change another owner's manifest to make my own line
   disappear.
2. **The `go-split` agent must action `📐️Pending`** in `📦️packages/🐹️go/🐹️.go` and then flip the
   two `// 🚚️ repoint …` import lines in `🧪️tests/*/🐹️.go` from
   `github.com/usalu/semio/repo/client` to `github.com/usalu/semio/repo/model`, at which point
   `goSutModule` alone is enough to wire the Go host and `goWorkspaceModules` becomes a convenience.
   `Ticket`/`Goal`/`Interaction` also need their hand-written `(Un)MarshalJSON` split: the
   identity-id rewriting belongs to `🪪️identity`/`🎫️tickets`/`🎯️goals`, and the pre-schema field
   spellings must simply go (no legacy support). Those three types are therefore **absent from the
   golden set** — noted in the fixture's own `$comment`.
3. **Coverage of the golden set is 60 of the ~110 data types.** It was chosen to exercise every
   encoding *rule* (each `omitempty` flavour, `nil`-slice-vs-`[]`, sorted map keys, nested and
   recursive refs, every enum, the transparent newtype, `#[serde(flatten)]`, and the untagged
   Go-field-name types) rather than to enumerate types. Extending it is one entry per type in
   `$TICKET/🗑️generated/model/make-goldens.ts` plus one line in each adapter's type table.
4. **`Statute` and `Territory` are provisional here.** When `📜️statutes` lands its enum, `📐️model`
   should either depend on it (it is L2, so the dependency must go the other way) or the two modules
   must agree that the model keeps the opaque id and the statutes module owns the vocabulary. My
   reading of the DAG says the latter; the statutes agent should confirm.
5. **No `README.md`/`AGENTS.md`** was written for the new module or its two packages. Sibling
   packages have them; the wiring or audit wave should add them for consistency.
6. **`GoalStatus` is now a closed enum** (`open`/`closed`) where Go's `Goal.Status` is a bare
   `string`. Every goal in `.🧬semio/🦑️repo/🎯️goals` uses one of the two values, and greenfield
   rules forbid keeping the looser shape; the split should tighten the Go side to match.

## 6. Files created or changed

Created:
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧬️schema/🔣️.json`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧬️schema/🔣️allowed-values.json`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🔮️oracle/🔣️.json`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📋️project.json,📜️script.ts}`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🐹️go/{go.mod,🐹️.go,📋️project.json,📜️script.ts}`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧪️tests/slug-normalisation/{🥒️.feature,🦀️.rs,🐹️.go,🧫️fixtures/🔣️vectors.json}`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧪️tests/definition-kind-derivation/{🥒️.feature,🦀️.rs,🐹️.go,🧫️fixtures/🔣️vectors.json}`
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧪️tests/json-encoding-conformance/{🥒️.feature,🦀️.rs,🐹️.go,🟦️.ts,🧫️fixtures/🔣️goldens.json}`
* `$TICKET/🗑️generated/model/{extract-allowed.ts,build-schema.ts,make-goldens.ts}` — the authoring
  scripts, kept as inputs; re-run them after any shape change.

Changed:
* `Cargo.toml` — `semio-framework-repo-model` added to `workspace.members`.
* `go.work` — `📐️model/📦️packages/🐹️go` added to `use`.
* `🔨️modules/🧪️test/📜️script.ts` — `goWorkspaceModules` + `materializeGoHost` (§2).
* `.vscode/🧩️launch.seed.jsonc` — three test entries; regenerated `.vscode/launch.json`.
