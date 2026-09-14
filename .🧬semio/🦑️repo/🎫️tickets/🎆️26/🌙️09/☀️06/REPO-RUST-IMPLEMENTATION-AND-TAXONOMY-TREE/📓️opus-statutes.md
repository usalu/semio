# 📓️ `🔨️modules/📜️statutes` — Rust crate, schema-first catalog, language-agnostic tests

Executor: Opus 5. Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes`.
Source of truth for the port: `$TICKET/🗑️generated/go-snapshot/client/🧩️component.go`, region `🧊️Policies`
(18420–21613), the `👮️Statute` type block (15770–16400), `buildBreachID` (36406) and the
`📌️Tree Cache` gzip envelope (7594–7710).

## 1. What is on disk

```
🔨️modules/📜️statutes/
  🧬️schema/🔣️.json            JSON Schema 2020-12: catalog, statute meta, territory, policy,
                               breach, ignore directives, breach cache envelope
  🧬️schema/🔣️statutes.json    THE catalog: 73 statutes + 9 policies with their territory trees
  🧫️fixtures/📁️some/📁️folder  the golden analyzer trees, MOVED here from 🦑️repo/🖼️assets/🧫️fixtures
  🔮️oracle/🔣️.json            one oracle (node zlib + crypto), one recorded no-oracle decision
  🧪️tests/📚️statute-catalog/    🥒️ 🦀️ 🐹️
  🧪️tests/🔍️analyze-breaches/   🥒️ 🦀️ + 🧫️fixtures/🔣️breaches.json (69 reviewed golden breaches)
  🧪️tests/🩹️autofix-roundtrip/  🥒️ 🦀️
  🧪️tests/🙈️ignore-directives/  🥒️ 🦀️ 🐹️ + 🧫️fixtures/🔣️vectors.json
  🧪️tests/🗜️breach-cache-envelope/ 🥒️ 🦀️ 🟦️(oracle) + 🧫️fixtures/🔣️vectors.json
  📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}
  📦️packages/🐹️go/…            NOT mine — created by the concurrent `go-split` agent
```

Also changed:

- root `Cargo.toml`: `📜️statutes/📦️packages/🦀️rust` added as a workspace member.
- `.vscode/🧩️launch.seed.jsonc`: `🧪️test🧰️repo📜️statutes{🦀️rust,🐹️go,🥒️parity}` added after the
  `🏃️test-runner` block, following the existing grouping and naming.
- `📚️library/🔣️taxonomy.json`: the `repo-analyzer-path-fixture` reservation now points at
  `📜️statutes/🧫️fixtures/📁️some` instead of the removed `🦑️repo/🖼️assets/🧫️fixtures/📁️some`.
- `🦑️repo/🖼️assets/🧫️fixtures/` is gone; `🖼️assets/LICENSE.md` is untouched.
- `$TICKET/🏗️extract-statute-catalog.ts`: the one-shot extractor that produced
  `🧬️schema/🔣️statutes.json` from the Go snapshot. Kept as ticket input, re-runnable.
- `$TICKET/🏗️statutes-golden/`: the standalone crate that regenerates the analyze golden from the
  committed fixture tree. Kept as ticket input, re-runnable.

## 2. The catalog is now one table

`🧬️schema/🔣️statutes.json` carries all 73 statutes — id, owning policy, priority, reason,
solution, autofixable — and all 9 policies (`code`, `dev-docs`, `sketchpad`, `repo`, `system`,
`folder`, `file`, `compose`, `dependency-boundary`) with their full nested territory trees. The
Rust crate `include_str!`s it and parses it into the `📐️model` types, so it holds no second copy of
the vocabulary.

Extraction was verified to be lossless: all 73 constants resolve to real metadata (0 fall back to
the `Unknown breach` entry), 71 of 73 are claimed by a territory, and the two that are not
(`compose/description/missing-emoji`, `compose/description/emoji-not-unique`) are unclaimed in the
Go source too — the `📚️statute-catalog` case projects that set so the gap stays visible rather than
being silently papered over.

**Open item for `go-split` / a later wave**: the Go package still carries the catalog as a Go
literal (`var policies` and `model.StatuteInfoTable`) rather than reading `🔣️statutes.json`. The
plan's §3 schema-first rule wants one source of truth. Until Go loads the JSON, the
`📚️statute-catalog` case is what keeps the two copies honest — its Go adapter projects from Go's own
tables and its Rust adapter from the JSON, so any drift fails parity. I did not edit the Go package,
per the split of ownership in my brief.

## 3. What the Rust crate implements

Complete and faithful to the Go snapshot:

- **Catalog**: `catalog`, `statutes`, `policies`, `find_policy`, `statute_info` (with the
  `Unknown breach` fallback), `is_autofixable`, `territory_kinds`, `policy_kinds`.
- **Ignore directives**: `parse_ignore_directives` (the `// compose-ignore-` prefix, comma-separated
  patterns, blanks dropped), `is_ignored` (strict `>` on the directive line, `<= line + 100`, plain
  string prefix on the statute id), `extract_file_from_scope`, `filter_ignored`.
- **Breach**: `build_breach_id` (`repo/breach/<scope>[#line[:col]]`), `create_breach`.
- **Specification text**: `is_spec_text` (the whole-word MUST/SHOULD/SHALL/MAY/REQUIRED/
  RECOMMENDED/OPTIONAL check, hand-rolled — `MUSTNOT` correctly does not match) and
  `has_implementation_syntax` (backtick pair, then the leftmost qualified or bare call, with the
  `MUST(`/`SHOULD(`/`SHALL(`/`MAY(` exemption).
- **Analysis** — `header_policy`, `section_policy`, `requirements_policy`, run in that order by
  `analyze`. These are the three policy functions the Go `codePolicy` runs first, and between them
  they emit 16 statutes: `code/file/{missing-header-region,missing-contributors,missing-license,
  wrong-license,missing-summary}`, `code/section/{missing-start-name,missing-end-name,name-mismatch,
  empty,wrong-format/newline-after-region,missing-summary,orphan-definition}`,
  `code/definition/{wrong-format/not-native-docstring,missing-summary,missing-requirements}` and
  `code/requirements/implementation-syntax` — including the JSDoc, `///` and Python-docstring
  native-docstring scanners, the orphan-definition ranges and the comment-block synthesis.
- **Autofix**: `autofix` repairs `code/section/wrong-format/newline-after-region`, the one statute
  the golden `🧪️file-fixable` → `🧪️file-fixable-expected` pair exercises, and is idempotent.
- **Digest**: hand-rolled SHA-256 (`sha256`, `sha256_hex`). Nothing lower in the DAG exports one —
  `🔌️mcp` has one but sits at L5 — so the crate owns its own, judged by the oracle.
- **Compression**: hand-rolled CRC-32, a gzip writer over stored DEFLATE blocks (`gzip_encode`) and
  a **full** INFLATE reader (`gzip_decode`) that handles stored, fixed-Huffman and dynamic-Huffman
  blocks and verifies the CRC and length trailer.
- **Breach cache**: `BreachCacheEnvelope` (the `entityId`/`script`/`breachs` shape the JS lint runner
  writes), `encode_breach_cache_json`, `parse_breach_cache`, `breach_cache_digest`,
  `encode_breach_cache`, `decode_breach_cache`, `breachs_from_cache`.

### 3.1 Deliberate deviations, stated plainly

1. **The brief's "breach cache = gzip + sha256 envelope" is not what the Go snapshot does.** In the
   snapshot the breach cache (`🧷️BreachCache`, 28491–28554) is a **plain JSON** envelope, and the
   gzip-`BestSpeed`+`sha256` envelope is the **tree cache** (`📌️Tree Cache`, 7594–7710) — sha256 keys
   the cache directory, gzip wraps the tree JSON. I implemented both halves in this module: the
   plain-JSON reader faithfully (`breachs_from_cache`), and the gzip+sha256 codec as the shared
   primitive, with the compressed breach-cache document defined as gzip over the envelope's canonical
   JSON encoding, keyed by the SHA-256 of that same encoding. `🌳️tree` can reuse it.
2. **The gzip writer emits stored blocks, not compressed ones.** Byte-for-byte equality with Go's
   `gzip.BestSpeed` is not a property of gzip and is not something an oracle would confirm; what
   matters is that a member this repository writes inflates anywhere and a member written anywhere
   inflates here. The oracle case asserts exactly that, in both directions, on the subject's own
   bytes. A real LZ77+Huffman encoder can be dropped in behind `gzip_encode` later without touching
   any caller or any test.
3. **`analyze` takes `(path, content)` pairs, not a scope and a bundle list.** The Go
   `PolicyContext` reads through the process-global `rootDir`; the walking, the bundle model and
   `ScopeToFiles` belong to `🗂️codebase`, which sits at the same DAG level and was still empty when I
   started. `SourceSet` keeps the crate a pure function of its input and keeps the golden tree case
   hermetic. Wiring it to `🗂️codebase` when that crate lands is a one-function change.
4. **`is_test_or_benchmark_file` inlines the `FileKindLab` predicate.** `📐️model` has `FileKind` but
   no `derive_file_kind`; that lives in `🗂️codebase`. The Lab suffix table is reproduced from
   `DeriveFileKind` (10720–10737) and must be folded back when `🗂️codebase` exports it.

### 3.2 Not yet ported

`comment_policy` and six of the nine policies are **not** in the Rust crate:

| Not ported | Why |
| --- | --- |
| `commentPolicy` | needs `LanguagePlugin::ScanComments` (≈370 lines of char-level block-comment state) plus `SpecLines`/`SectionDocLines`/`DefinitionDocLines`, which are per-language and belong with `🗣️languages` |
| `emojiPolicy`, `docsPolicy` | need the file walk and `DeriveFileKind` from `🗂️codebase` |
| `devDocsPolicy`, `sketchpadPolicy`, `repoPolicy`, `systemPolicy` | walk the repository and read devcontainer/workspace JSON; need `🏠️workspace` + `🗂️codebase` |
| `folderPolicy`, `filePolicy` (`pathEmojiTaxonomy`, `Godfile`) | need the whole-repo path inventory from `🗂️codebase` |
| `composePolicy`, `dependencyBoundaryPolicy` | need manifest parsing across five ecosystems |

This is why `🔍️analyze-breaches` and `🩹️autofix-roundtrip` ship **without** a Go adapter: Go exports
only `CheckPolicies*`, which runs *every* scope-matching policy, so a Go run cannot be narrowed to
the three policies Rust implements and parity would fail for a reason that is not a real
disagreement. Adding the Go adapters is the last step of finishing the port, not a separate task.

## 4. The cases

| Case | Scenarios | Adapters | Oracle |
| --- | --- | --- | --- |
| `📚️statute-catalog` | `the-catalog-is-one-table`, `the-catalog-is-consistent` | rust, go | none (`repo-statutes-owned-law`) |
| `🔍️analyze-breaches` | `the-golden-tree-breaches`, `the-clean-sources-are-clean` | rust | none |
| `🩹️autofix-roundtrip` | `the-fixable-source-becomes-the-expected-source` | rust | none |
| `🙈️ignore-directives` | `a-directive-suppresses-its-prefixes`, `a-directive-only-reaches-forward` | rust, go | none |
| `🗜️breach-cache-envelope` | `the-digest-is-sha-256`, `a-member-inflates-anywhere` | rust (subject), typescript (oracle) | node `zlib` + `crypto` |

`🗜️breach-cache-envelope` carries `@oracle-input-subject-raw`: the Rust subject writes its gzip
members as the raw artifact and the TypeScript oracle inflates **those bytes** with `zlib.gunzipSync`
rather than compressing the payloads itself, so the case proves cross-implementation readability and
not merely that two implementations agree with themselves.

`🔍️analyze-breaches` compares against `local://🔣️breaches.json`, 69 breaches reviewed by hand
against the Go source. The clean half asserts that `🧪️file-fixed/*` and `🧪️file-fixable-expected`
raise nothing, which is the property those fixtures were written to have.

## 5. Verification — real output

### 5.1 `cargo build`

```
$ RUSTC_WRAPPER="" cargo build -p semio-framework-repo-statutes
   Compiling semio-framework-repo-model v0.1.0 (…📐️model📦️packages🦀️rust)
   Compiling semio-framework-repo-languages v0.1.0 (…🗣️languages📦️packages🦀️rust)
   Compiling semio-framework-repo-statutes v0.1.0 (…📜️statutes📦️packages🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 8.33s
```

### 5.2 `discover`

```
$ bun "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" discover | grep statute
test-…-statutes-94acfb-analyze-breaches      …/📜️statutes/🧪️tests/🔍️analyze-breaches       [rust]
test-…-statutes-94acfb-autofix-roundtrip     …/📜️statutes/🧪️tests/🩹️autofix-roundtrip      [rust]
test-…-statutes-94acfb-breach-cache-envelope …/📜️statutes/🧪️tests/🗜️breach-cache-envelope  [rust,typescript]
test-…-statutes-94acfb-ignore-directives     …/📜️statutes/🧪️tests/🙈️ignore-directives      [rust]
test-…-statutes-94acfb-statute-catalog       …/📜️statutes/🧪️tests/📚️statute-catalog        [rust]
```

(The two Go adapters were added after this run; `discover` lists `[rust,go]` for
`statute-catalog` and `ignore-directives` once re-run.)

### 5.3 `contract`

`contract` reports no breach owned by `📜️statutes`. Every line it prints is pre-existing and
repo-wide (`✏️s/🔌️plugins/🗒️note/…` mutation vectors, and five `testing/discovery` baselines for
`temp`, `🧰️framework`, `.storybook`, `✏️s`, `♻️mit-bestand`).

### 5.4 `subject` (rust)

```
$ RUSTC_WRAPPER="" bun "./…/🧪️test/📜️script.ts" subject fundamental \
    --owner "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes" --implementation rust
[test] level=fundamental cases=5 executed=9 passed=9 failed=0 errored=0 parity=0/0
```

### 5.5 `parity` (all roles, all implementations)

```
$ RUSTC_WRAPPER="" GOWORK=C:/git/semio/go.work bun "./…/🧪️test/📜️script.ts" parity fundamental \
    --owner "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes"
[test] level=fundamental cases=5 executed=11 passed=11 failed=0 errored=0 parity=2/2
[test] …/🔍️analyze-breaches: no-oracle decision repo-statutes-owned-law claims the independent-implementations substitute but only one implementation ran
[test] …/🩹️autofix-roundtrip: no-oracle decision repo-statutes-owned-law claims the independent-implementations substitute but only one implementation ran
[test] …/🙈️ignore-directives: go subject host exited 1 without emitting results
[test] # github.com/usalu/semio/repo/workspace
C:\…\🏠️workspace\📦️packages\🐹️go\🐹️.go:856:27: plan.binary undefined (type formatterPlan has no field or method binary, but does have field Binary)
C:\…\🏠️workspace\📦️packages\🐹️go\🐹️.go:857:46: plan.binary undefined …
C:\…\🏠️workspace\📦️packages\🐹️go\🐹️.go:861:43: plan.binary undefined …
[test] …/📚️statute-catalog: go subject host exited 1 without emitting results        (same three errors)
```

**`parity=2/2` is the oracle result**: both `🗜️breach-cache-envelope` scenarios agree with Node's
`crypto` and `zlib`. The hand-rolled SHA-256 reproduces the reference digests, and every gzip member
the Rust subject wrote was inflated by `zlib.gunzipSync` back to the exact payload.

**The Go subject failure is not mine.** `github.com/usalu/semio/repo/workspace` — owned by the
concurrent `go-split` agent, which I must not edit — does not compile:

```
$ GOWORK=C:/git/semio/go.work go build ./…/🏠️workspace/📦️packages/🐹️go/
# github.com/usalu/semio/repo/workspace
🐹️.go:856:27: plan.binary undefined (type formatterPlan has no field or method binary, but does have field Binary)
🐹️.go:857:46: plan.binary undefined …
🐹️.go:861:43: plan.binary undefined …
```

`repo/statutes` transitively requires it, so the two Go adapters I wrote cannot run yet. They are
committed and correct as far as I can tell by reading; **they have never executed**, and I am not
claiming otherwise. Re-run the parity command above once `🏠️workspace` compiles: it should go to
`executed=15` with real cross-implementation parity on `📚️statute-catalog` and `🙈️ignore-directives`.
That is also the first moment the extraction into `🔣️statutes.json` gets independently checked
against Go's own tables.

### 5.6 The golden is reproducible

`$TICKET/🏗️statutes-golden/` regenerates `🧪️tests/🔍️analyze-breaches/🧫️fixtures/🔣️breaches.json`
from the committed fixture tree:

```
$ cargo run --quiet --manifest-path "$TICKET/🏗️statutes-golden/Cargo.toml" -- \
    "…/📜️statutes/🧫️fixtures/📁️some/📁️folder"
MATCH 69 69
```

### 5.7 One real bug found in the Go source, reproduced deliberately

`requiresDefinitionRequirements` for TypeScript does
`strings.TrimLeft(noExport, "async abstract declare default ")`. `TrimLeft` takes a **cutset**, not
a prefix, and `{a,b,c,d,e,f,l,n,r,s,t,u,y,' '}` eats the leading `f`,`u`,`n`,`c`,`t` of `function `
and the `c`,`l`,`a`,`s`,`s`,` ` of `class `, so the subsequent `HasPrefix("function ")` /
`HasPrefix("class ")` can never be true. **No TypeScript definition ever raises
`code/definition/missing-requirements`.** My first pass "fixed" this and produced 71 breaches; I
reverted it to match Go exactly and the golden is 69. Faithfulness beats correctness while the two
implementations are being made twins — but this belongs on someone's list, and changing it will move
the golden by exactly the two `🧪️file/🟦️.tsx` entries (`TestComponent`, `TestClass`).

## 6. Left for whoever picks this up

1. Port `commentPolicy` and the six repository-walking policies once `🗂️codebase` exports its walk
   and `DeriveFileKind`; then add the Go adapters for `🔍️analyze-breaches` and
   `🩹️autofix-roundtrip` and widen the golden.
2. Make the Go package read `🧬️schema/🔣️statutes.json` instead of its Go literal.
3. Replace the stored-block gzip writer with an LZ77 + Huffman encoder behind the same signature.
4. Fold `is_test_or_benchmark_file`'s Lab predicate back onto `🗂️codebase::derive_file_kind`.
5. Wire `analyze` to `🗂️codebase`'s scope-to-files so the `⌨️cli` `analyze` verb can call it.
