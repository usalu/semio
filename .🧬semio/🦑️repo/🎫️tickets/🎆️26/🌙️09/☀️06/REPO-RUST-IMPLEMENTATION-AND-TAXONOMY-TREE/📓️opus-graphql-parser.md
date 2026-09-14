# 📓️ Opus executor report — `🔗️graphql` parser (Go move + Rust twin + language-agnostic tests)

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/`
Agent: Opus 5, wave 1, ticket `26/09/06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE`.

## 1. What exists now

```
🔗️graphql/
├── 🧬️schema/🔣️.json                     JSON Schema 2020-12: AST projection + the three fixture corpora + the diagnostic row
├── 🔮️oracle/🔣️.json                     contribution manifest: oracle `graphql-js`, decision `graphql-owned-subset-boundary`, oracleHostPackages
├── 📦️packages/🐹️go/{go.mod, 🐹️.go, 🧪️_test.go, 📋️project.json, 📜️script.ts}
├── 📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}
└── 🧪️tests/{document-parsing, syntax-errors, variable-coercion, unsupported-syntax}/
    └── {🥒️.feature, 🧫️fixtures/🔣️*.json, 🐹️.go, 🦀️.rs, 🟦️.ts (oracle, 3 of 4 cases)}
```

### Go

`internal/graphql/🔗️graphql.go` moved **whole** to `📦️packages/🐹️go/🐹️.go`, module
`github.com/usalu/semio/repo/graphql`, `go 1.25`, no dependencies. The whole file moved, not only the
grammar: `component.go` calls `graphql.NewObject`, `graphql.Do` and the type constructors, so leaving
`📜️Schema`/`⚡️Execution` behind would have broken the build the same day. Regions are now
`🧲️Header, 📜️Schema, 🔤️Lexer, 🌳️Ast, 🧩️Parser, ✅️Validation, ⚡️Execution`.

The AST was **exported without changing behaviour**: `document/selection/value` → `Document/Selection/Value`
with exported fields, `value.resolve` → `Value.Resolve`, plus `Parse`, `Selection.Key`,
`CoerceArguments` and the three `Projection()` methods. The executor now calls `CoerceArguments` and
`Selection.Key` instead of restating them inline, so the coercion rule has exactly one definition.
`Validate` and `OperationType` are unchanged.

Wiring: `go.work` gained the module; `💻️client/⌨️cli/go.mod` gained the `require` + `replace`; the
import line in `🧩️component.go` and `🔬️component_test.go` was repointed (import line only, nothing
else in those files was touched); `💻️client/⌨️cli/internal/graphql/` is deleted.

Deviation: the plan names the Go test file `🧪️.go`. `go test` only compiles `*_test.go`, so the file
is `🧪️_test.go` — with `🧪️.go` the package built and reported `[no test files]`.

### Rust

`semio-framework-repo-graphql`, `[lib] path = "🦀️.rs"`, `[lints] workspace = true`,
`[package.metadata.semio] role = "library"`, dependencies `serde`/`serde_json` workspace-only, added
to the root `Cargo.toml` members. Regions `🔤️Lexer, 🌳️Ast, 🧩️Parser, ✅️Validation, 🧪️Tests`.

It is a **byte-level twin**, not a re-interpretation. The behaviours that had to be reproduced
deliberately, each of which a "clean" Rust rewrite would have got wrong:

* The lexer is **byte-oriented**. Go does `rune(source[offset])`, so `unicode.IsSpace` also accepts
  `0x85` and `0xA0`, `unicode.IsLetter` accepts the Latin-1 letter bytes (`0xAA 0xB5 0xBA 0xC0-0xD6
  0xD8-0xF6 0xF8-0xFF`), and any UTF-8 lead byte outside those is `unexpected character`. Mirrored in
  `is_space` / `is_letter` / `is_digit`.
* String literals are decoded with Go's `strconv.Unquote`, **not** GraphQL's escape set — `\xNN`,
  `\NNN` octal and `\U########` are accepted, `\/` is not. Reimplemented in `unquote`.
* `Lexer::next` returns `(Token, Option<ParseError>)` rather than a `Result`, because Go assigns both
  in one statement: a failed `Unquote` leaves a **stale string token** in `current` while `err` is
  set, and that is observable — `{ a(b: "\q") }` reports `expected argument name at 7`, not an escape
  error.
* A lexer failure inside a selection set is **masked** by `unterminated selection set`, because
  `advance` zeroes `current` and the EOF branch is checked first.
* `%q` rendering of the offending byte and of token text is reproduced (`quote_byte`, `quote_string`),
  so the diagnostics are identical strings.

Every one of the 13 diagnostics in the Go unit test is asserted verbatim in the Rust unit test.

**Public API the executor agent codes against** (all of it stable, nothing else is needed):

```rust
pub struct Document { pub operation: String, pub selections: Vec<Selection> }
pub struct Selection { pub alias: Option<String>, pub name: String,
                       pub arguments: BTreeMap<String, Value>, pub fields: Vec<Selection> }
pub struct Value { pub literal: Option<Literal>, pub variable: Option<String>,
                   pub list: Option<Vec<Value>>, pub object: Option<BTreeMap<String, Value>> }
pub enum Literal { Bool(bool), Int(i64), Float(f64), Str(String) }        // serde untagged
pub enum ParseErrorKind { … 13 variants … }
pub struct ParseError { pub kind: ParseErrorKind, pub message: String, pub offset: Option<usize> }

impl Document { pub fn parse(&str) -> Result<Document, ParseError>; pub fn projection(&self) -> Json }
impl Selection { pub fn key(&self) -> &str; pub fn projection(&self) -> Json }
impl Value  { pub fn resolve(&self, variables: &Map<String, Json>) -> Json; pub fn projection(&self) -> Json }
pub fn parse(&str) -> Result<Document, ParseError>;
pub fn validate(&str) -> Result<(), ParseError>;
pub fn operation_type(&str) -> Result<String, ParseError>;
pub fn coerce_arguments(&BTreeMap<String, Value>, defaults: &Map<String, Json>,
                        variables: &Map<String, Json>) -> Map<String, Json>;
pub use serde_json;   // explicit re-export — clients never declare the dependency themselves
```

Field-for-field mirror of the Go struct, on purpose. `Value::default()` **is** the Go zero value, so
`None` on `literal` means the same thing it means in Go: the `null` literal. `list: None` is a nil
slice (an empty list literal produces it), `object: Some(empty)` is a non-nil empty map. Whoever
ports `Do`/`executeSelections` should port the shape, not tidy it — the executor's `if value.variable
!= ""` / `if value.list != nil` precedence is what `Value::resolve` already encodes.

## 2. Tests

Owner-level cases, Protocol v2, following `🔨️modules/🧪️test/README.md`:

| Case | Fixtures | Oracle | Profile | Scenarios |
| --- | --- | --- | --- | --- |
| `document-parsing` | 50 request strings | `graphql-js` `parse` | `ordered-json-v1` | corpus-projects-identically (fundamental), operation-kind-is-recovered (quick) |
| `syntax-errors` | 23 malformed inputs | `graphql-js` `parse` | `diagnostic-v1` | malformed-inputs-are-rejected (fundamental, `@mode-error`) |
| `variable-coercion` | 18 cases | `graphql-js` `valueFromASTUntyped` | `ordered-json-v1` | arguments-resolve-against-variables (fundamental), defaults-fill-only-absent-arguments (quick) |
| `unsupported-syntax` | 12 boundary inputs | none — `graphql-owned-subset-boundary` | `ordered-json-v1` | subset-boundary-is-identical (fundamental) |

The corpus covers aliases (single, doubled, nested), arguments of every literal kind, negative
numbers, exponents, enums, nulls, variables with and without defaults, list/nested-list/list-of-object
literals, empty and nested input objects, one/two/argument-less directives, leading/inline/trailing
comments, comments inside an argument list, `\"`/`\\`/`\n`/`\t`/`\uXXXX` escapes, literal `é 中 🧬`
and `Straße`, whitespace-heavy layout, and one 12-line document that combines all of it.

The oracle's projection mapping is a **declared rename** of the graphql-js AST, not a second parser:
it drops variable definitions and directives (the subject grammar drops them), nulls an absent alias,
sorts arguments and object fields by name, and maps `EnumValue` to a string literal.

The fourth case exists because a conforming parser cannot judge the boundary of a deliberate subset —
it accepts the fragments we refuse and refuses the `{ }` we accept — so that case rests on the
recorded decision and on Go↔Rust pairwise equivalence including the verbatim message.

## 3. Commands run and their output

```
$ cd 🔗️graphql/📦️packages/🐹️go && GOWORK=off go test ./...
ok  	github.com/usalu/semio/repo/graphql	0.403s

$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-graphql
running 5 tests
test tests::reports_the_operation_type ... ok
test tests::projects_the_canonical_shape ... ok
test tests::reproduces_the_reference_diagnostics ... ok
test tests::round_trips_the_ast_through_json ... ok
test tests::applies_defaults_only_to_absent_arguments ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ bun 🧪️test/📜️script.ts discover | grep graphql
test-…-document-parsing    …/🧪️tests/document-parsing    [rust,typescript,go]
test-…-syntax-errors       …/🧪️tests/syntax-errors       [rust,typescript,go]
test-…-unsupported-syntax  …/🧪️tests/unsupported-syntax  [rust,go]
test-…-variable-coercion   …/🧪️tests/variable-coercion   [rust,typescript,go]

$ bun 🧪️test/📜️script.ts oracle fundamental --owner 🔗️graphql
[test] not-exercised …/unsupported-syntax (recorded no-oracle decision graphql-owned-subset-boundary — its evidence is discharged by the subject phase)
[test] level=fundamental cases=4 executed=3 passed=3 failed=0 errored=0 parity=0/0 not-exercised=1

$ RUSTC_WRAPPER="" bun 🧪️test/📜️script.ts parity long --owner 🔗️graphql
[test] level=long cases=4 executed=17 passed=17 failed=0 errored=0 parity=16/16

$ cd 💻️client/⌨️cli && GOWORK=…/go.work go build ./...   → exit 0
$ cd 💻️client/⌨️cli && GOWORK=…/go.work go vet ./...     → exit 0   (compiles 🔬️component_test.go too)

$ bun nx run @semio-tech/repo-graphql-go:test
ok  	github.com/usalu/semio/repo/graphql	(cached)
NX   Successfully ran target test for project @semio-tech/repo-graphql-go

$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
   → .vscode/launch.json:7898 @semio-tech/repo-graphql-rs:test, :7905 @semio-tech/repo-graphql-go:test
```

`bun ./📜️script.ts test` inside `📦️packages/🦀️rust` fails before reaching cargo with
`Exact owner catalog mode drift: …/📚️library/📦️packages/🟦️typescript/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json`
— a repo-wide file-mode check unrelated to this module (it fires from
`resolveCargoPackageName` for any crate). `cargo test -p …` and the nx Go target are green, so the
crate itself is proven; the nx Rust target is blocked on that pre-existing breakage.

## 4. Two framework changes this needed (both in `🔨️modules/🧪️test/📜️script.ts`)

1. **`goSutModule` + `materializeGoHost`.** The generated Go host's `go.mod` required only the test
   host, so a Go adapter physically could not import the module its case is about — the Rust host had
   linked its subject crate since day one, Go had no twin. `materializeGoHost` now walks up from the
   owner for `📦️packages/🐹️go/go.mod` and emits the `require` + `replace`. Replace paths are
   normalised to forward slashes.
2. **`ownerShipsImplementation` stops at the nearest package root.** It walked all the way up, so
   `🧰️framework/📦️packages/🟦️typescript` made *every* owner in the repository "ship TypeScript". The
   oracle-only `🟦️.ts` adapters were therefore dispatched as subjects and every scenario errored with
   `adapter has no subject registration`, dragging a fully-passing case to zero (observed: 5 errored
   results before the fix, 0 after). The nearest directory that has a `📦️packages` at all now answers
   for every language.

## 5. Deviations from the brief, and why

* **Oracle manifest path.** The brief says `🔗️graphql/🔣️oracle.json`. `discoverTestContributions`
  looks for `<owner>/🔮️oracle/🔣️.json` (`testContributionDirName` + `testContributionFileKindId`),
  which is where all ~200 existing manifests live; `🛍️products/📓️print/🔣️oracle.json` is not
  discovered by anything. The manifest is therefore at `🔗️graphql/🔮️oracle/🔣️.json`.
* **No `🔗️graphql/📦️packages/🟦️typescript`.** Creating it would declare a TypeScript *implementation*
  of this owner (see change 2 above) and demand subject registrations that do not exist. `graphql`
  `16.14.0` — the version already resolved in the checkout — is declared as a root devDependency and
  named in `oracleHostPackages`; the TypeScript host resolves it from the repository's own
  `node_modules`, which is what the README says it does. No `bun install` was needed.
* **Case directories are plain kebab-case** (`document-parsing`, not `🧩️document-parsing`).
  `testCaseSlugPattern` is `^[a-z0-9]+(?:-[a-z0-9]+)*$` and `validateAllContracts` raises
  `testing/taxonomy case-slug` otherwise. The existing `🖥️host-protocol-parity` case is in breach of
  this today.
* **A fourth case** (`unsupported-syntax`) was added: the brief asked for fragments, inline fragments
  and block strings in the parsing corpus, but the grammar rejects all three, so they cannot be
  compared against a conforming oracle. They are covered honestly as boundary evidence instead.

## 6. Known divergences and fidelity gaps — for the executor agent

1. **`null` and `[]` are the same AST node.** Go's `[]` leaves `list` nil, which is the zero value,
   which is also the `null` literal. Both project as `{"kind":"null"}`. Rust reproduces it. If the
   executor ever needs to tell an empty list from a null argument, the AST must change in both
   implementations together.
2. **A string token and a bare enum name are the same node.** `"OPEN"` and `OPEN` both become
   `Literal::Str("OPEN")`. The executor's `Enum` projection already resolves this at the schema level.
3. **An unbound variable inside an input object.** This executor keeps the member as an explicit
   `null`; graphql-js's `valueFromASTUntyped` **omits** it. Real semantic divergence, found by the
   parity run. The corpus binds that variable and records the note; if the executor is ever expected
   to match GraphQL here, `Value::resolve` on the object branch is the single place to change.
4. **The grammar is wider than GraphQL in three places** — `{ }`, `@` without a directive name, and a
   variable definition with no type are all accepted, because the corresponding loops exit or skip
   rather than diagnose. All three are pinned in `unsupported-syntax`.
5. **`diagnostic-v1` drops `offset` and `detail`,** so the harness does not compare messages between
   the subjects and the oracle. Go↔Rust message equality is instead pinned by the two unit-test suites
   (13 identical assertions) and, for the subset boundary, by `unsupported-syntax`, which compares the
   verbatim message under `ordered-json-v1`.

## 7. Left for later waves

* The executor (`Do`, `executeSelections`, `project`, `resolveDefault`), the schema builder and the
  resolvers still live in Go in `💻️client/⌨️cli/🧩️component.go` and are not ported to Rust. The Rust
  AST above is complete and stable for that port; the Go executor already routes through
  `CoerceArguments`/`Selection.Key`, so those two rules do not have to be rediscovered.
* `📜️Schema` and `⚡️Execution` have no Rust counterpart yet; the Rust crate is grammar-only.
* The nx Rust test target is blocked by the repo-wide `Exact owner catalog mode drift` check.
* `testing/dependency` reports `Production source imports the registered oracle
  serde-json-equation-carrier-reader` for `🦀️.rs` — 142 identical breaches repo-wide (every crate
  that uses `serde_json`, including all sibling repo crates); a framework-level false positive, not
  addressed here.
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` was in an unresolved
  `git stash pop` conflict state (four conflict hunks, the two sides not semantically equal) for part
  of this session, which blocked every harness command. It was resolved by another session; it was
  not touched here.
