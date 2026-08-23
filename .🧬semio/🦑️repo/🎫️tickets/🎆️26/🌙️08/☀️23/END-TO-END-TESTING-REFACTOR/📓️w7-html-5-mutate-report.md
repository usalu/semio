# Wave 7 — 🌐️html standard 🔖️5 subset ✳️any — mutation oracle + exhaustive round-trip case

Executor report for the fleet brief's HTML 5 subset assignment. Nothing outside
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/…/🔖️5/…`, the new `🧪️tests/mutate-html-5` case directory,
and the new `🧫️fixtures/` fixture was touched.

## 1. Reference implementation

`html5ever` 0.39 + `markup5ever_rcdom` 0.39 (both already linked behind the oracle crate's
`oracles` feature, per the brief). One oracle registry entry (`html5ever-html-5-mutate`), matching
the precedent set by `mutate-xlsx-ecma-376` (calamine+rust_xlsxwriter) and `mutate-json-rfc8259`:
`html5ever` only tokenizes/builds a tree via a caller-supplied `TreeSink`, so it cannot serve as an
oracle without `markup5ever_rcdom`'s `RcDom`/`SerializableHandle`; both crates are exercised by
every green scenario even though only one is the registry entry's package name.

## 2. Real input, and the one honest derivation

Primary real input: the committed presentation page at
`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🌐️zukunft-bau-entwerfen-mit-bestand.html`
— a real TYPO3-produced HTML5 document, 2,337 lines / 149 KB, genuine `<head>`/`<body>` structure,
real navigation/footer/external-resource references. Copied (`cp`) into this artifact's own
`🧫️fixtures/`, referenced `shared://`.

**One derivation, documented plainly rather than hidden**: the real page carries **zero** inline
`<script>`/`<style>` content anywhere (confirmed by grep before writing any code) — every script is
`src=`-external, every stylesheet is `<link>`-external. `SetRawText` needs an *existing* RAWTEXT
node to retarget, so a pristine copy of the real page has no genuine target for it. Rather than
invent one, the fixture inlines the page's own two ALREADY-referenced real committed files —
`zukunft-bau-entwerfen-mit-bestand_files/🎨️overwrite.css` (was a `<link rel="stylesheet">`) and
`.../🟨️default_frontend.js` (was `<script src="...">`) — verbatim, in place of the tag that
referenced them. Every other byte is the real page unchanged. The derivation script is not kept
(a one-shot Python string replacement, not a permanent migration); the two files it drew from
remain in the repo at the paths named above for anyone to re-verify against.

## 3. Real, documented normalizations (not silently absorbed)

Recorded in the oracle module's header, the catalog's `comparisonProfiles` entry, and the feature
file — three places, not one, per the ticket brief's "confirm exactly what you treat as specified
normalization vs. real information loss":

1. **Boolean attributes collapse to empty-string value.** The WHATWG tokenizer gives every
   attribute a value, defaulting to `""` when no `=` appears — `rcdom`'s `Attribute.value` is a
   plain string with no "valueless" state. This subset's own `HtmlAttr{value: Option<String>}` DOES
   distinguish `<p disabled>` from `<p disabled="">`; the oracle's DOM cannot. Every `set-attribute`
   example in this case therefore uses a concrete non-empty value, never the valueless branch — that
   branch is exercised instead by this subset's own pre-existing Rust-level unit test
   (`set_attribute_tristate_apply_and_inverse_round_trip`), not silently dropped from coverage.
2. **`html5ever::serialize`'s `write_doctype` drops the public/system id**, emitting only
   `<!DOCTYPE name>`. The real fixture only ever carries the bare `<!doctype html>` this costs
   nothing on; recorded anyway since it is a genuine, permanent limit of this reference writer.
3. **Adjacent `Text` siblings serialize with no boundary marker and re-coalesce on reparse** (a
   real, HTML5-inherent property, not a bug in `html5ever`: two `Text` nodes written back to back
   are byte-indistinguishable from one). This bit `remove-node`'s naive "reparse the mutated bytes,
   then apply the inverse" implementation directly — the fixture's own indentation formatting means
   almost every element in `<body>` sits between two whitespace-only `Text` siblings, and removing
   it makes them adjacent. **Fixed, not routed around**: `oracle_apply_mutation_inverse` now applies
   the forward mutation and its inverse on the SAME in-memory tree, never round-tripping through
   bytes in between (serializing only once, at the very end) — which is what `HtmlMutation::inverse`'s
   own law already assumes at the model level. Documented in the function's own doc comment for
   whoever touches this file next.

## 4. `InsertNode`/`RemoveNode` — the page-operation analogue the brief asked for

`insert-node` adds a real, marked `<div>` as `<body>`'s first child. `remove-node` deletes the real
(empty) `<div class="sidebars">` at `<body>`'s 10th position — a genuinely real, safely-removable
element on the real page, not an invented one; its inverse restores it (attributes and position)
exactly, verified both by the exhaustive runner and by a dedicated Rust unit test
(`real_fixture_every_declared_kind_mutates_and_inverts_cleanly`) that runs all ten declared kinds,
with the EXACT params the feature file uses, against the real fixture and asserts
`apply(inverse(m, base), apply(m, base)) == base` at the projection level for every one.

## 5. Verification (run for real, not claimed)

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

**Contract**: `bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-html-5`
```
2 high-priority breach(es) across 1 rule(s):
      2  testing/contract

  testing/contract  epw-energyplus-any  Mutation catalog epw-energyplus-any (13 kinds) is claimed by no feature
  testing/contract  docx-ecma-376-any  Mutation catalog docx-ecma-376-any (13 kinds) is claimed by no feature
```
Neither breach names `html` — both are other subsets' catalogs, still mid-write by other wave-7
sessions at the time this ran (per the fleet brief's own warning about this).

**Oracle exhaustive**: `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-html-5`
```
[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0
```
21/21 green: 10 `mutate-<kind>` + 10 `inverse-<kind>` + 1 `identity-round-trip`.

Before that, the whole oracle module (including the exhaustive real-fixture sweep test above) was
also verified standalone: `semio-s-plugin-stdio-test-oracle` is a dependency-free-workspace crate,
so `cargo test --features oracles` runs directly from
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — 13/13 green there too. The case adapter's
ORACLE-only half (everything outside `#[cfg(feature = "sut")] mod subject`) was additionally
type-checked in isolation against the real `semio-repo-test-host` and this subset's real oracle
module (`cargo check`, zero errors, only the expected "unexpected cfg value: sut" warning since no
`sut` feature was declared in that scratch harness) — the SUBJECT half cannot be verified this wave
(the fleet brief's known `📡️spr/🧵️channel` cycle), so it is written and `sut`-gated per the brief,
using this repository's own `parse_html_document`/`write_html_document`/`apply_html_mutation` as its
only channel from input to output, but its own compile/run is unverified and reported as such.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS` + `kinds_const_matches_enum_variants_in_declaration_order` test (no
  other change; the enum itself was pre-existing, already-wired scaffolding).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — filled in the dispatcher stub: independent `HNode`/`HDoc` tree, `html5ever`/`rcdom`-backed
  parse/serialize, `oracle_apply_mutation`/`oracle_apply_mutation_inverse`/`project_html_5`, 13 unit
  tests (incl. the real-fixture exhaustive sweep).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  — new: oracle registration (`html5ever-html-5-mutate`) + `semantic-html-v1` comparison profile +
  mutation catalog (`html-5-any`, 10 kinds).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🧫️fixtures/🌐️zukunft-bau-entwerfen-mit-bestand.html` —
  new: the real, derived fixture (§2).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🧪️tests/mutate-html-5/component.feature` — new.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🧪️tests/mutate-html-5/🦀️component.rs` — new adapter.

No shared family module (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs` etc.) function
was added or modified — HTML has no genuine sharing partner among the other stdio artifacts this
wave.
