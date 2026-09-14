# 📓️ Opus executor report — `🔨️modules/🗣️languages`

Module root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages/`.
Source of truth for the port: `$TICKET/🗑️generated/go-snapshot/client/🧩️component.go` regions
`🎽️Languages` (13335–16749) and `📝️Sections` (17779–18420), plus `📚️library/🐹️.go` region `📹️Parsing`
(365–803). `💻️client/⌨️cli/🧩️component.go` and its tests were **not** touched.

## 1. What is on disk

```
🗣️languages/
├── 🧬️schema/🔣️.json               JSON Schema draft 2020-12: language table, pattern language, section/definition/scope/header records
├── 🧬️schema/🔣️languages.json      the table itself — 13 languages, 12 registered, shared vocabulary, scope patterns
├── 🔮️oracle/🔣️.json               oracle registry v2: 1 oracle, 3 no-oracle decisions
├── 🧫️fixtures/                    13 owner fixtures, one source file per language
├── 📦️packages/🦀️rust/             semio-framework-repo-languages (lib 🦀️.rs, Cargo.toml, 📋️project.json, 📜️script.ts)
├── 📦️packages/🐹️go/               github.com/usalu/semio/repo/languages (🐹️.go, 🧪️_test.go, go.mod, 📋️project.json, 📜️script.ts)
└── 🧪️tests/{section-parsing, definition-parsing, scope-ids, header-roundtrip, malformed-regions}/
```

Registered: root `Cargo.toml` members, root `go.work`, `.vscode/🧩️launch.seed.jsonc` (3 entries),
`.vscode/launch.json` regenerated.

## 2. The declarative pattern language (no host regexes)

`🧬️schema/🔣️.json#/$defs/pattern` defines an **ordered token list matched anchored at the start of one
line with ordered backtracking**, which reproduces Go RE2's leftmost-first submatch semantics exactly
(Go's default `regexp` is leftmost-first, not POSIX leftmost-longest, so ordered alternation plus
greedy-with-backtracking repetition is behaviourally identical). Tokens:

`ws` `ws1` `wsChar` `end` `wordEnd` `lineStart` `ident` `identScoped` `identUpper` `typeIdentUpper`
`word` `restTrim` `restTrimNonSpace` `lit` `anyOf` `charIn` `until` `opt` `alt` `rep` `cap` `repChar`.

Every one of the 92 `regexp.MustCompile` sites in the two Go regions that belongs to this module is
expressed as such a token list in `🔣️languages.json`. The matcher is ~200 lines in `🦀️.rs`
(`token_ends` + `run_tokens` with a linked-list continuation); the crate has **no regex dependency**.

Two subtleties worth recording because they are easy to get wrong:

* `(.+?)\s*$` (`restTrim`) is lazy-minimal, so on an all-blank remainder Go captures a single space,
  not the empty string. `rest_trim_capture` reproduces that exactly.
* `\s*` and `\s+` must backtrack (they are greedy but not possessive); `Token::Ws`/`Ws1` therefore
  yield every reachable end offset, longest first.

The table also carries what used to be hard-coded Go: `definitionKeywords` (modifier set, keyword
set, multi-word keywords, fallback), `definitionKindMap`, `refinements` (arrow/function/class
initialiser promotions), `scopeRegionMarker` and `scopeDefinitionPatterns`.

## 3. Public Rust API — what `🗂️codebase`, `📜️statutes` and `🚚️move` will use

Crate `semio-framework-repo-languages`. It re-exports `Section`, `Definition` and `DefinitionKind`
from `semio-framework-repo-model` (path dependency), so a consumer never has to name the model crate
to hold a parse result.

### Table

```rust
pub fn table() -> &'static LanguageTable;
pub fn language_by_name(name: &str) -> Option<&'static Language>;
pub fn language_for_path(path: &str) -> Option<&'static Language>;
pub fn language_for_extension_unregistered(ext: &str) -> Option<&'static Language>;  // reaches `json`

pub struct LanguageTable { schema_version, registry, definition_keywords, definition_kind_map,
                           refinements, scope_region_marker, scope_definition_patterns, languages }
pub struct Language { name, emoji, extensions, section_engine, section_start, section_end,
                      policy_section_start, policy_section_end, definition, definition_engine,
                      orphan_definitions, aux_patterns, comment_prefix, block_comment_start,
                      block_comment_end, section_start_format, section_end_format,
                      section_both_format, supports_headers, uses_indent_scoping,
                      supports_definitions_override, supports_comments_override, skip_directives,
                      string_features, section_naming }
impl Language {
    pub fn supports_sections(&self) -> bool;
    pub fn supports_definitions(&self) -> bool;
    pub fn supports_comments(&self) -> bool;
    pub fn matches_extension(&self, ext: &str) -> bool;
    pub fn skip_directives_all(&self) -> Vec<String>;   // built-ins + the language's own
}
pub enum SectionEngine { Markers, MarkdownHeadings, JsonKeys, None }
pub enum DefinitionEngine { Braces, Indent, RubyEnd }
pub enum OrphanDefinitions { None, GoPackageAndImports, RustModDeclarations, RubyModules }
pub enum SectionNaming { Verbatim, ModName }
pub struct DefinitionKeywords { modifiers, keywords, multi_word, fallback }
pub struct Refinements { arrow_function, function_expression, class_expression, promotes, promoted_to }
pub struct ScopeRegionMarker { strip_prefixes, strip_suffixes, start_keyword, end_keyword }
```

### Pattern matching — needed by `📜️statutes` for `PolicySectionStartMatch`/`EndMatch`

```rust
pub struct Pattern { pub ci: bool, pub tokens: Vec<Token> }
pub enum Token { … }                       // the 22 variants above
pub type Captures = Vec<(String, String)>;
pub fn pattern_match(p: &Pattern, line: &str) -> Option<(usize, Captures)>;   // anchored
pub fn pattern_find(p: &Pattern, line: &str) -> bool;                          // unanchored
```

### Sections — `🗂️codebase`, `📜️statutes`, `🚚️move`

```rust
pub fn parse_sections(content: &str, file_path: &str) -> Vec<Section>;
pub fn parse_code_sections(content: &str, language_name: &str) -> Vec<Section>;
pub fn parse_sections_with(lang: &Language, content: &str) -> Vec<Section>;
pub fn parse_markdown_sections(content: &str) -> Vec<Section>;
pub fn parse_json_sections(content: &str) -> Vec<Section>;
pub fn hydrate_sections_with_definitions(sections: &[Section], definitions: &[Definition]) -> Vec<Section>;
pub fn normalize_section_path(section_path: &str) -> Vec<String>;
```

### Definitions — `🗂️codebase`, `📜️statutes`

```rust
pub struct DefinitionRange { name, kind, start, end, excerpt }   // `kind` is the RAW keyword
pub fn parse_definitions(content: &str, file_path: &str) -> Vec<Definition>;
pub fn parse_definition_ranges(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange>;
pub fn extra_orphan_definitions(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange>;
pub fn derive_definition_kind(raw_kind: &str) -> DefinitionKind;
```

### Scope identifiers — `🗂️codebase`, `📜️statutes`, `🚚️move`

```rust
pub struct ScopeEntry { kind, id, file_path, section_path, definition, start_line, end_line }
pub struct ParsedSection { name, path, start_line, end_line }
pub fn build_scope_id(kind: &str, file_path: &str, section_path: &str, definition: &str) -> String;
pub fn build_scopes_for_file(path: &str, content: &str) -> Vec<ScopeEntry>;
pub fn parse_region_marker(line: &str) -> Option<(String, bool)>;   // (name, is_end)
pub fn parse_markdown_heading(line: &str) -> Option<(usize, String)>;
pub fn parse_sections_from_lines(lines: &[&str], ext: &str) -> Vec<ParsedSection>;
pub fn parse_definitions_from_lines(lines: &[&str], patterns: &[Pattern]) -> Vec<(String, i64)>;
```

Grammar (pinned by the `scope-id-grammar` scenario): `file:<path>`,
`section:<path>#<sectionPath>`, `def:<path>#<sectionPath>::<name>`, and `def:<path>#<name>` when the
definition sits outside every section.

### Headers and section markers — `🚚️move`, `📜️statutes`

```rust
pub struct Header { file_id, file_uri, summary, contributors, license, requirements }
pub fn format_section_start(lang: &Language, name: &str) -> String;
pub fn format_section_end(lang: &Language, name: &str) -> String;
pub fn format_section_both(lang: &Language, name: &str) -> String;
pub fn format_header(lang: &Language, header: &Header) -> String;
pub fn parse_header(lang: &Language, content: &str) -> Option<Header>;   // NEW — Go has no reader
pub fn section_name_to_mod_name(name: &str) -> String;                    // Rust `mod` wrapper
```

`parse_header` is new (the Go original only formats). A formatted header carries no marker between
the summary block and the requirements block, so the first optional block is read as the summary and
the second as the requirements; the property that holds for every input is **idempotence** —
`format(parse(format(x))) == format(x)` — and it is asserted per language in the crate's own tests.

### Identity (temporary)

```rust
pub fn is_emoji_rune(r: char) -> bool;
pub fn extract_entity_emoji(s: &str) -> (String, String);
```

These sit in a `🔖️IdentityPending` region and belong to `🔨️modules/🪪️identity` per the plan's DAG.
When that crate exposes them, delete the region and re-export.

## 4. Go package

`github.com/usalu/semio/repo/languages` currently contains the **table loader only**:
`LoadTable`, `TablePath`, `LanguageByName`, `LanguageForPath`, the decoded `Table`/`Language`/
`Pattern`/`Token`/`DefinitionKeywords`/`Refinements`/`ScopeRegionMarker` types, and the
`SEMIO_REPO_LANGUAGES_TABLE` override. The table is read at `init` from a module-relative path
(`runtime.Caller` → `../../🧬️schema/🔣️languages.json`, falling back to a walk up for the repo root),
so the Go side reads the **same bytes** the Rust side embeds with `include_str!`.

The `🗣️Pending` region in `🐹️.go` lists, symbol by symbol with snapshot line ranges, exactly what the
AST-split agent must move here (58 entries across the two `component.go` regions and the library's
`📹️Parsing` region), what must **not** come here (`ScanComments` and its two overrides → `📜️statutes`;
`ExtractImports`/`FormatImports`/`ExtractPackage` and the `json*` editing helpers → `🚚️move`), and two
recorded Go defects:

* **🐞️JSONSectionAliasing** — `ParseJSONSectionsDetailed` keeps `*Section` pointers into `Children`
  slices it keeps appending to, so every key but the last in an object has its end position written
  through a stale backing array and is returned with `endLine`/`endIndex` still `-1`.
* **🐞️JSONSectionPath** — it stores each key's slash-joined path on the `JsonSectionLocation` record
  and never on the `Section` it returns, so every JSON section comes back with an empty `Path`.

The Rust twin implements the intended semantics for both. The `json-object-key-tree` scenario
therefore projects names, nesting and start positions only; the feature file says so and points at
the Pending region. Fix the Go side during the split and add `path` and the end positions back.

## 5. Language-agnostic tests

Owner `🗣️languages`, 5 cases, 12 scenarios, all `@level-fundamental`.

| Case | Oracle | Scenarios |
| --- | --- | --- |
| `section-parsing` | `@no-oracle-repo-region-markers` | marker regions across 11 languages (differential), markdown heading ranges (differential), JSON object key tree (differential) |
| `definition-parsing` | `@oracle-typescript-compiler` | TypeScript top-level declarations (differential), callable const is a function (differential) |
| `scope-ids` | `@no-oracle-repo-scope-grammar` | scope id grammar (conformance), scopes of a source file (differential) |
| `header-roundtrip` | `@no-oracle-repo-file-header` | header per language (differential), header region is parseable (round-trip) |
| `malformed-regions` | `@no-oracle-repo-region-markers` | unclosed region runs to the end (error), stray endregion ignored (error), unclaimed extension has no sections (error) |

**Oracle.** The only credible third-party reference for this owner is the **TypeScript compiler API**
(`typescript@5.9.3`, `ts.createSourceFile`), which answers two real language questions the subjects
must agree with: which top-level declarations exist and where each starts, and whether a variable
binding's initialiser is callable (arrow function, function expression or class expression) rather
than a value. It is registered in `🔮️oracle/🔣️.json` as `typescript-compiler`, `kind:
third-party-library`, `testOnly: true`, `productionReachable: false`.

Three `noOracleDecisions` are registered with rationales, covering the rest:
`repo-region-markers`, `repo-scope-grammar`, `repo-file-header`. The `#region <emoji><Name>`
convention, the `file:`/`section:`/`def:` addressing grammar and the header document shape are this
repository's own inventions; substitutes are `independent-implementations`,
`specification-vectors` and `metamorphic-laws` as appropriate.

Not registered as oracles, and why: `remark-parse` and `jsonc-parser` are both present in
`node_modules` and were evaluated. Neither can compute this owner's projection — the Markdown
scenario compares byte ranges produced by a specific closing rule (`endIndex = lineStart - 1`,
front-matter lines added to every line number) and the JSON scenario compares key start offsets under
this repository's own path convention. An adapter built on either library would have to restate those
rules, which makes it a third implementation of the specification, not an independent reference. That
is exactly the failure mode the `noOracleDecision` mechanism exists to record, so they are recorded
there instead of being dressed up as oracles.

Adapters: `🦀️.rs` uses the crate; `🐹️.go` imports the **current** Go homes with the
`// 🚚️ repoint to github.com/usalu/semio/repo/languages after split` marker — four cases import
`github.com/usalu/semio/repo/client`, and `scope-ids` imports
`github.com/usalu/semio/repo/events`, because the `📡️events` executor has already moved the
library's `📹️Parsing` region (`BuildScopeID`, `BuildScopesForFile`) there. `🟦️.ts` in
`definition-parsing` is the oracle.

## 6. Verification — real command output

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-languages
running 22 tests
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

$ RUSTC_WRAPPER="" cargo clippy -p semio-framework-repo-languages --all-targets
(0 warnings)

$ cd 🗣️languages/📦️packages/🐹️go && go test ./... && go vet ./... && gofmt -l .
ok  	github.com/usalu/semio/repo/languages	0.706s
(vet clean, gofmt clean)

$ bun ".../🧪️test/📜️script.ts" discover | grep languages
test-framework-products-repo-modules-languages-d65b1a-section-parsing   …/🗣️languages/🧪️tests/section-parsing   [rust,go]
(and the four other cases)

$ bun ".../🧪️test/📜️script.ts" contract --case <each of the five>
(no breach whose scope is under 🗣️languages)

$ bun ".../🧪️test/📜️script.ts" subject fundamental --case <case> --implementation rust
section-parsing      executed=3 passed=3 failed=0 errored=0
definition-parsing   executed=2 passed=2 failed=0 errored=0
scope-ids            executed=2 passed=2 failed=0 errored=0
header-roundtrip     executed=2 passed=2 failed=0 errored=0
malformed-regions    executed=3 passed=3 failed=0 errored=0

$ bun ".../🧪️test/📜️script.ts" subject fundamental --case <case> --implementation go
(identical counts, all passed)

$ bun ".../🧪️test/📜️script.ts" oracle fundamental --case definition-parsing
executed=2 passed=2 failed=0 errored=0

$ bun ".../🧪️test/📜️script.ts" parity fundamental --case <case>
[test] level=fundamental cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3   section-parsing
[test] level=fundamental cases=1 executed=6 passed=6 failed=0 errored=0 parity=6/6   definition-parsing
[test] level=fundamental cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2   scope-ids
[test] level=fundamental cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2   header-roundtrip
[test] level=fundamental cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3   malformed-regions

$ bun nx run @semio-tech/repo-languages-go:test
NX   Successfully ran target test for project @semio-tech/repo-languages-go

$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
```

`definition-parsing` `parity=6/6` is the strongest result on the board: oracle×rust, oracle×go and
rust×go for both scenarios. The Rust implementation, the untouched Go implementation and the
TypeScript compiler all agree on which declarations exist, where they start, and which `const`
bindings are callable.

## 7. Deviations from the brief, each with its reason

1. **Case directories are plain kebab-case**, not `🧩️section-parsing` etc. `🔣️taxonomy.json`'s
   `testCaseSlugPattern` is `^[a-z0-9]+(?:-[a-z0-9]+)*$` and `contract` reports an emoji-prefixed case
   directory as a `testing/taxonomy` breach. Verified by running `contract` with the emoji names
   first; that was the only breach the run reported for this owner.
2. **The oracle manifest is `🗣️languages/🔮️oracle/🔣️.json`**, not `🗣️languages/🔣️oracle.json`. The
   taxonomy declares `testContributionDirName: "🔮️oracle"` + `testContributionFileKindId: "json"`, and
   the discoverer only reads that path. With the file at the brief's location the feature failed
   contract with `Unknown no-oracle decision @no-oracle-repo-region-markers`.
   `🛍️products/📓️print/🔣️oracle.json` is a schema-v1 leftover at the old location.
3. **No module-local `📦️packages/🟦️typescript`.** Creating one made `ownerShipsImplementation`
   (`🧪️test/📜️script.ts:377-392`) conclude the owner ships a TypeScript implementation, so the
   oracle-only `🟦️.ts` adapter was dispatched as a *subject* and every one of its scenarios errored
   with `adapter has no subject registration`. The documented mechanism for a TypeScript oracle is
   `oracleHostPackages` in the owner's manifest resolved from the repository's own `node_modules`
   (README §"Reaching a reference library"); `typescript@^5.9.3` is already a root devDependency, so
   the oracle runs with no new install. The root `package.json` workspace list is back to what it was.
4. **13 languages in the table, 12 in the registry.** Faithful to the Go original: `languageRegistry`
   omits `JSONLanguage` while `NewJSONLanguage` exists and is reached only through
   `ParseJSONSectionsDetailed`. `language_for_extension_unregistered` is the escape hatch, and the
   registry-order rule is pinned by a test.
5. **Python is in the table.** The brief listed 12 languages without Python; the Go registry has it and
   omits JSON. The table follows the Go registry.

## 8. What is left

* **For the go-split agent** — the `🗣️Pending` region in `📦️packages/🐹️go/🐹️.go` is the work order.
  Move the 58 listed symbols, rewrite each `regexp.MustCompile` as a table lookup through a Go port of
  the token matcher, fix 🐞️JSONSectionAliasing and 🐞️JSONSectionPath, then flip the five `🚚️ repoint`
  markers in the adapters from `repo/client` and `repo/events` to `repo/languages` and re-run parity.
  Only after that does the Go side load the table for anything but its own contract test.
* **For `🪪️identity`** — take `is_emoji_rune` / `extract_entity_emoji` out of the
  `🔖️IdentityPending` region; the emoji ranges are duplicated between this crate and whatever the
  foundation executor built.
* **`📐️model` duplicates `derive_definition_kind`** as a hand-written match while this crate derives it
  from `definitionKindMap`. `definition_kind_map_agrees_with_the_model_crate` asserts they agree today,
  but the long-term fix is for the model crate to read the table.
* **Not ported, by design** (they belong to other modules per the plan's DAG): `ScanComments` and the
  `CommentScanState`/`CommentTemplateState` machinery (→ `📜️statutes`, which also needs
  `PolicySectionStartMatch`/`EndMatch` — the patterns are in the table and `pattern_match` is public),
  `ExtractImports`/`FormatImports`/`ExtractPackage` and the `json*` editing helpers (→ `🚚️move`).
* **Pre-existing, not mine**: `bun nx run @semio-tech/repo-languages-rs:test` fails in
  `loadTaxonomy()` with `generatorContracts["wgpu-frame-worker"] tracked output …🤖️generated/🟨️.js is
  missing`. `@semio-tech/repo-model-rs:test` fails identically, so every Rust nx test target in the
  repo is currently blocked by that missing generated file. Direct `cargo test -p
  semio-framework-repo-languages` passes 22/22, and the Go nx target succeeds.
* **Coverage gaps worth a later case**: definition parsing outside TypeScript is covered by the crate's
  own tests (Go receivers, Python dedent, SQL multi-word keywords, GraphQL `extend type`, Ruby `end`
  scoping) but not yet by a language-agnostic case, because no scenario there can be held to a
  third-party oracle and the differential value is already carried by `section-parsing`. The
  `graphql` npm package could back a GraphQL-definition scenario if a later wave wants one.
