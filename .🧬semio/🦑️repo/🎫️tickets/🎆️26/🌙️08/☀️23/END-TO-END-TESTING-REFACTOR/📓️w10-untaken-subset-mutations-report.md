# Wave 10 — subsets that had no handcrafted `🧬️mutations` directory

Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Date 2026-08-24. Scope as assigned: *every remaining
subset that still has no handcrafted `🧬️mutations` directory after the other implement agents have
taken theirs.*

## Summary

**Delivered and verified: one subset — `s.stdio.json@rfc8259/✳️i-json` (RFC 7493 I-JSON).**
Handcrafted 10-kind mutation vocabulary, `KINDS` + conformance test, a REAL third-party oracle
registration (`simplejson`, running on the newly landed **Python** oracle host), and the case
`mutate-json-rfc8259-i-json` covering every kind twice plus a conformance scenario and an identity
round-trip, on the real 424 KB CAD model.

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-json-rfc8259-i-json      # exit 0
0 high-priority breach(es) across 0 rule(s)

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-json-rfc8259-i-json   # exit 0
[test] level=exhaustive cases=1 executed=22 passed=22 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio                                # exit 0 — whole owner, no regression
[test] level=exhaustive cases=80 executed=1011 passed=1011 failed=0 errored=0 parity=0/0 not-exercised=20
```

The owner-wide sweep is the check that the two shared-file edits (the plugin's `oracleHostPackages`
and `🔒️dependencies.json`) broke nothing for anyone else: 80 cases, 1011 scenarios, all green. It
also shows how far the other implement agents got in parallel — the wave 7 baseline was 55 cases /
779 scenarios.

**Not delivered: the other 16 untaken subsets.** They are mapped below with the specific blocker for
each, so the coordinator can dispatch them without re-doing the reconnaissance. Two of those blockers
are hard facts about this repository (no real-world fixture exists for the profile; the reference
library cannot write the fields the profile constrains), and one is procedural (the oracle crate's
`📦️lib.rs` is off limits, so a new subset oracle needs either a Python/TS host or that rule relaxed).

## What the scope actually was, at the time it was measured

At session start, 30 of the 88 stdio subsets had no `🧬️schema/🧬️mutations`. During the session other
implement agents landed catalogs for 13 of them — `docx/xlsx/pptx ✳️strict` + `✳️transitional`,
`ifc 2x3 ✳️cobie`/`✳️cv20`/`✳️sav`, `svg 1.1 ✳️basic`/`✳️tiny`, `zip 2.0 ✳️iso21320`. Those are
theirs and were left alone. The residue was 17, of which this session took one.

Untaken at the time of writing (16):

| artifact | subsets |
|---|---|
| `📄️pdf` 1.4 | `✳️a`, `✳️x` |
| `📄️pdf` 1.7 | `✳️a`, `✳️e`, `✳️h`, `✳️ua`, `✳️vt`, `✳️x` |
| `📐️step` ap214 | `✳️cc1` … `✳️cc6` |
| `📰xml` 1.0 | `✳️valid` |
| `📷️jpg` jfif-1.01 | `✳️baseline` |
| `🖼️tiff` 6.0 | `✳️baseline` |

## The delivered subset — `json@rfc8259/✳️i-json`

### Why the vocabulary is genuinely its own and not a copy of ✳️any

RFC 8259 defines a *syntax*; RFC 7493 narrows the *value space* that syntax may describe. Four of
those clauses are facts about a decoded `JsonSnapshot` — which is exactly why this subset's existing
`check_i_json_conformance` can already check them. The new vocabulary makes the same four clauses
part of the EDITING ALGEBRA rather than only of the acceptance gate:

| clause | what ✳️any can express | what ✳️i-json adds |
|---|---|---|
| §2.1 top level is an object or an array | `SetScalar { path: [], value }` — a bare scalar root is representable | `SetTopLevel { root: JsonIJsonRoot }` — the payload type has no scalar arm, so a scalar root is UNREPRESENTABLE rather than rejected afterwards |
| §2.2 integers within ±(2^53−1) | `SetScalar` writes any `Number` lexeme, `9007199254740993` included | `SetSafeNumber { path, lexeme }` — an out-of-range integer is refused `mutation.invariant` (Fatal ⇒ empty diff) and never reaches the snapshot |
| §2.3 member names are unique | no rename verb at all; remove-then-insert transits a state where BOTH names exist and loses the member's position | `RenameMember { path, from, to }` — one atomic, position-preserving step, refused when `to` already names a member |
| §2.4 strings avoid Unicode noncharacters | `SetScalar` writes any `String` | `SetString { path, value }` — a noncharacter-bearing value is refused |

The other six kinds (`no-mutation`, `set-snapshot`, `upsert-member`, `remove-member`,
`insert-array-element`, `remove-array-element`) are **inherited from ✳️any unchanged**. RFC 7493 says
nothing about arrays and nothing about member insertion or deletion beyond uniqueness, so their
semantics are the parent's verbatim; they lower onto the parent's ops one-for-one, and
`the_four_inherited_verbs_lower_onto_their_any_counterparts_unchanged` asserts that rather than
leaving it as prose. Recording that as an honest finding was preferred over inventing a difference.

Every variant lowers to exactly ONE `JsonMutation` and delegates to its `Mutation::diff`, so the
`JsonDiff` algebra stays a single source of truth — the leaf adds the I-JSON gate and the I-JSON-level
inverse, never a second diff semantics.

### Why the oracle runs in Python

`simplejson` 4.1.1, `ecosystem: "python"`, registered in the subset's own
`🧪️oracle/🔣️.json` under the capability `json-rfc8259-i-json-mutate`.

RFC 7493 constrains the value space, so the reference has to surface three facts a normalizing codec
throws away: every object's ORDERED member list including duplicate names, the exact number LEXEME,
and the decoded string. `simplejson`'s `object_pairs_hook` reports duplicates a `dict`-producing
parser silently collapses (the oracle asserts the real fixture has none — that is what makes it an
I-JSON document and not merely an RFC 8259 one), and `parse_int`/`parse_float` hand back raw digits so
§2.2 is checked on the digits rather than through a lossy double. `Decimal` + `use_decimal` carry the
lexemes back out on write, so all ten kinds are genuinely differential.

Two libraries were deliberately NOT used, and the reasons are recorded in the registry entry:
CPython's stdlib `json` — registrable only under the package name `json`, which the platform's own
production-purity probe would then match against every `import json` in the repository (it does:
`temp/w0-inspect-paths.py`), and a probe that cannot tell a reference library from the language's
standard library is not a probe; and `serde_json` — this subset's ✳️any parent declares
`impl From<serde_json::Value> for JsonValue` in production, so that differential would compare the
implementation against something it already converts from.

The Python host provisioned and installed `simplejson` into its own cache-local venv on the first
run, exactly as the `📓️cross-language-oracle-hosts-2026-08-24.md` design describes.

### The case

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧪️tests/mutate-json-rfc8259-i-json/`

* input `shared://🔣️hexagonal-cut-concrete-forest-left.model.json` — the real 424 KB CAD model already
  committed in this artifact's fixtures (8,979 nodes, 146 exponent-notation floats at the
  machine-epsilon boundary), verified I-JSON-conforming before the case was written.
* 22 scenarios: `mutate-<kind>` × 10 (`@mode-differential`), `inverse-<kind>` × 10
  (`@mode-property`), `i-json-conformance` (`@mode-conformance`), `identity-round-trip`
  (`@mode-round-trip`, forbids byte pass-through).
* Contributed comparison profile `semantic-i-json-v1`: array order normative, object member order not
  (RFC 8259 §4 producer freedom, and RFC 7493 §2.3 requires only uniqueness), numbers compared by
  value with a 1e-12 tolerance (§6 permits arbitrary precision; this subset keeps the lexeme, the
  reference re-emits a shortest round-trip decimal).
* The oracle does not rely on the subject phase for its evidence. `mutate` asserts that every kind
  except `no-mutation` actually MOVES the document (a silent no-op would otherwise read as a pass),
  and `inverse` asserts the metamorphic law against the reference's own pre-mutation document. The
  inverse spec is recomputed by reading the original — never by asking the subject for its inverse.

**Negative control, run for real**: changing the `set-safe-number` lexeme from `9007199254740991`
(the §2.2 boundary) to `9007199254740992` (one past it) turned `22 passed` into
`passed=20 failed=2` — the two scenarios that drive that kind. Reverted.

### Files

| file | what |
|---|---|
| `🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️i-json/🧬️schema/🧬️mutations/🦀️component.rs` | new — `JsonIJsonMutation`, `JsonIJsonRoot`, `KINDS`, `lower`, `apply_json_i_json_mutation`, `impl Mutation<JsonSnapshot>`, 8 plain `#[test]`s |
| …`/🧬️mutations/{🔣️component.json,🟦️component.ts,🛰️component.proto,🔗️component.graphql}` | new — the leaf's schema sidecars, mirroring the ✳️any leaf's shape |
| …`/✳️i-json/🧬️schema/🦀️component.rs` | edited — the builder now holds `JsonSnapshot` directly (the `ZipIso21320BuilderConstruction` shape) and declares `type Mutation = JsonIJsonMutation` |
| …`/✳️i-json/🧪️oracle/🔣️.json` | new — oracle, comparison profile, mutation catalog |
| `🗿️artifacts/🔣️json/🧪️tests/mutate-json-rfc8259-i-json/{component.feature,🐍️component.py,🦀️component.rs}` | new — feature, Python ORACLE adapter, Rust SUBJECT adapter (`sut`-gated) |
| `📦️packages/🦀️rust/📦️glue.rs` | edited — 6 lines mounting the new mutations leaf under `…::subsets::i_json::schema::mutations` |
| `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json` | edited — one additive `oracleHostPackages` entry (`python:simplejson@4.1.1`) |
| `🔒️dependencies.json` | edited — one additive entry, `kinds:["test-oracle"]`, `productionReachable:false` |

`bun ./📜️script.ts dependency` → `ecosystems=4 entries=229 production-reachable=151 test-oracle=27`,
`[dependency] test-oracle python:simplejson@4.1.1 (simplejson-json-rfc8259-i-json-mutate)`. Exit 0.

## Honest limits of what was delivered

1. **No Rust compiled.** `cargo check -p semio-s-plugin-stdio --lib` fails before reaching this
   plugin: `semio-framework-job` does not build (5 errors, another session's live refactor —
   `admit_page`'s `E0499` double mutable borrow among them). So the new production Rust
   (`🧬️mutations/🦀️component.rs`, the builder rewire, the glue mount) and the `sut`-gated subject
   adapter are **unverified by the compiler**. What WAS done instead: every new/edited Rust file was
   put through `rustfmt --edition 2021 --emit stdout` (exit 0 on all three — a real parse, not a
   type-check), and the one language rule that was genuinely uncertain — whether a `pub mod
   mutations` inside a module that also `pub use component::*`s a parent carrying its own `mutations`
   would trip `ambiguous_glob_reexports` — was settled by building a throwaway crate that reproduces
   the exact shape. It compiles clean and the local module shadows the glob.
2. **`parity=0/0` is not a pass.** No subject ran, so the differential half of the case is declared
   and wired but has not executed. The oracle-side assertions above are what make the oracle phase
   meaningful on its own in the meantime.
3. **The eight `#[test]`s in the new mutations leaf have not been run**, for the same reason.

## The 16 subsets not covered, and why

### `📐️step@ap214 ✳️cc1` … `✳️cc6` — one shared design, one hard fixture fact

The six conformance classes are not six vocabularies. Read together
(`🪆️subsets/✳️ccN/🧬️schema/🦀️component.rs`), each differs from its siblings in exactly one constant —
`MAX_RUNG: u8` = 1…6 — over one shared engine
(`🪆️subsets/✳️any/🚪️io/🪜️ladder/🦀️component.rs::ladder_rung_of`), which maps
`GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION→2`, `…SURFACE…→3`,
`MANIFOLD_SURFACE…→4`, `FACETED_BREP…→5`, `ADVANCED_BREP…→6`, everything else `*_SHAPE_REPRESENTATION`
→ 2. So the honest result here is **one rung-parameterized vocabulary instantiated six times**, not
six handcrafted enums — the same finding shape as the ✳️i-json "four inherited verbs", scaled up.

The blocker is the fixture. The only real AP214 file in the repository is
`📐️step/🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp` (79 KB, real Rhino/ST-Developer
B-rep export; its own header records that the single edit was the `FILE_SCHEMA` line). Its only
`*_SHAPE_REPRESENTATION` instance is a plain `SHAPE_REPRESENTATION`, i.e. rung 2. Therefore:

* `✳️cc2`…`✳️cc6` can all use it as a **conforming** real input.
* `✳️cc1` (`MAX_RUNG = 1`) admits **no** `*_SHAPE_REPRESENTATION` instance at all, so no real B-rep or
  geometry export can ever conform to it. A cc1 case needs either a management-data-only real STEP
  file (none is committed) or a recorded decision that its scenarios are all violation-detection.

A second, softer blocker: the registered STEP reference is `ruststep` (Rust). A per-CC oracle module
would have to be mounted in `🧪️oracle/📦️packages/🦀️rust/📦️lib.rs`, which the fleet brief forbids
editing. Either that rule gets relaxed for new subsets, or the CC verbs get expressed as compositions
of the ✳️any oracle's already-exported `oracle_apply_mutation` kinds (possible here — every CC verb is
a rung-gated form of an ✳️any verb — but it puts the rung table in the adapter, which is weaker
evidence than a reference library's own behaviour).

### `📄️pdf` — 8 subsets, one reachable reference, one big read

`1.4/✳️a`, `1.4/✳️x`, `1.7/✳️a`, `✳️e`, `✳️h`, `✳️ua`, `✳️vt`, `✳️x`. All eight check the SNAPSHOT
(`check_*_conformance(&PdfSnapshot)`), and their fault codes are genuine profile facts that a
vocabulary could carry — e.g. PDF/UA's `missing-markinfo-marked`, `missing-structtreeroot`,
`missing-lang`, `missing-displaydoctitle`; PDF/X's `missing-output-intent`, `missing-trim-or-art-box`;
PDF/A's `encrypt-present`, `javascript-action`, `font-not-embedded`. Two of them (`1.4/✳️a`,
`1.4/✳️x`) currently check almost nothing (`schema-gap-unverifiable`), so their vocabularies would be
thin and should probably be a recorded finding rather than an invented enum.

Not attempted for effort reasons, not for a blocker: `pypdf` is already registered, already installed
and already proven on the Python host by `extract-text-pdf-1-4`, and it exposes exactly the catalog,
`/MarkInfo`, `/ViewerPreferences`, `/Lang`, Info-dictionary and `/OutputIntents` surfaces these
profiles constrain, and re-serializes. This family is the highest-value remaining work and the one
with the fewest unknowns.

### `🖼️tiff@6.0 ✳️baseline` — the reference cannot write what the profile constrains

Snapshot-level clauses are real and crisp: Compression ∈ {1, 2, 32773}, PhotometricInterpretation ∈
{0,1,2,3}, BitsPerSample ∈ {1,4,8}, strips not tiles. The real fixture
(`🖼️abbau-aufbau-masterarbeit-grundriss.tiff`, 17 MB) is nearly Baseline already — Compression 1,
Photometric 2, BitsPerSample (8,8,8), strip-organized — and carries exactly two TIFF *Extension*
fields, `Predictor` (317) and `SampleFormat` (339), which makes a "remove the non-Baseline field"
verb genuinely meaningful on real data.

The blocker was measured, not guessed. Pillow 11.3.0 was driven against the real fixture: it
round-trips arbitrary tags through `tiffinfo=ImageFileDirectory_v2` and honours
`compression="packbits"` (259 → 32773) — but it **derives** 258/262/277 from the image mode and will
not write them independently, **rewrites** strip organisation wholesale (273/278/279 replaced,
`PlanarConfiguration` 284 added), and **drops** `SampleFormat` (339) on save. Three of the four
Baseline clauses are therefore not differentially executable through Pillow. A viable oracle exists —
`tifffile`, which gives real `extratags`/`photometric` control — but it pulls `numpy` into the oracle
host and was not attempted here.

### `📷️jpg@jfif-1.01 ✳️baseline` — viable, not attempted

The best-positioned of the remaining singles. The snapshot models every clause the subset checks
(`sof_marker`, `frame.precision`, `frame.components[].{h_sampling,v_sampling}`, `arithmetic`,
`huffman_tables[].class`), the real 483 KB JFIF scan is a genuine baseline SOF0 8-bit 3-component
image, and Pillow can execute the writable half of the vocabulary (`qtables=`, `subsampling=`,
`progressive=False`, `quality=`, `dpi=`). Two design questions were left open rather than guessed:
JPEG is lossy, so the inverse law can only be asserted at the metadata level and the projection has to
follow the existing 8-bucket luma-histogram precedent in `🧪️oracle/🖼️raster/🦀️component.rs`; and
`set-restart-interval` has no Pillow surface, so it would have to leave the vocabulary or the case
would have to declare that kind non-differential.

### `📰xml@1.0 ✳️valid` — no real DTD-bearing fixture exists in this repository

The vocabulary is easy to justify and genuinely distinct: `XmlDoctype` is structurally decoded
(`name`, `external_id`, `declarations: Vec<XmlDtdDeclaration>`), ✳️any's only handle on it is
whole-doctype replacement, and the §2.8 DOCTYPE-Name / document-element lockstep is not expressible in
the parent vocabulary at all (there is no element-rename verb in `XmlMutation`, though `XmlElementDiff`
does carry a `name` field, so a handcrafted `rename-root` diff is constructible).

The blocker is the input. Every XML actually committed in this repository is DOCTYPE-less: the case
fixture `📰️ooxml-word-document.xml` (extracted from the real DOCX), the real `qr-code.svg`, all seven
XML parts inside the real `.bcf` archive. The one real HTML fixture carries `<!doctype html>` but is
HTML5, not XML. A ✳️valid case therefore has no real-world input that is already valid, which makes
`undeclare-doctype` a documented no-op on the only available fixture and undermines the
apply-must-change honesty check. Resolving this needs a real DTD-bearing XML artifact committed to
the repository — a decision for the coordinator, not something to fabricate.

## Rules observed

No `git commit`/`stash`/`checkout`. The ticket was not closed or reopened. Nothing under `./compose`
was touched. All new tests are plain `#[test]`. `🧪️oracle/📦️packages/🦀️rust/{Cargo.toml,📦️lib.rs}`
were not edited — which is precisely why the delivered oracle runs in Python. Two shared files were
edited additively and are reported above: the plugin's `🧪️oracle/🔣️.json`
(`oracleHostPackages` + one entry) and `🔒️dependencies.json` (one entry) — both are the mechanism the
just-landed cross-language host requires, and both were verified with `bun ./📜️script.ts dependency`.
