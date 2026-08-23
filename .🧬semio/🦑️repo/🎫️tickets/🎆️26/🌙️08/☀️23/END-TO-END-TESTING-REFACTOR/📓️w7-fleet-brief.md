# Wave 7 fleet brief — exhaustive real-world mutation round-trips

Every executor agent follows this document exactly. The framework half is already landed; do not
re-implement it, do not edit it.

## Where things live

A mutation is not a property of a format. It belongs to **one subset of one standard of one
artifact** — PDF 1.4 declares 2 mutation kinds and PDF 1.7 declares 15, and they are different
vocabularies, not versions of one list. Everything you write follows that shape:

| What | Where |
|---|---|
| Mutation vocabulary (already exists) | `<artifact>/🏅️standards/🔖️<std>/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` |
| **Mutation oracle** (yours to write) | `<artifact>/🏅️standards/🔖️<std>/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` |
| **Catalog + oracle registration** (yours) | `<artifact>/🏅️standards/🔖️<std>/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` |
| **Test case** (yours) | `<artifact>/🧪️tests/mutate-<fmt>-<std>/component.feature` + `🦀️component.rs` |
| Shared reference-library helpers | `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/{📄️document,🖼️raster,🎒️archive,🔊️audio,📊️tabular,🧊️mesh}/🦀️component.rs` |

Two subsets that need the same helper **share it through the shared family module** — never by
copying it into both. Do not create any new top-level mutation directory; the taxonomy above is the
only place mutations exist.

Your subset oracle module is already created and wired into the crate at
`artifacts::<fmt>::standards::<vstd>::subsets::any`, holding a dispatcher that currently rejects
every kind. Reach it from an adapter as, for example,
`semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_7::subsets::any::oracle_apply_mutation`.
Nobody else writes your subset's file.

## 1. The catalog and the oracle registration

`<artifact>/🏅️standards/🔖️<std>/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`:

```json
{
  "oracles": [
    {
      "id": "lopdf-pdf-1-7-mutate",
      "ecosystem": "rust",
      "package": "lopdf",
      "version": "0.44",
      "capabilities": ["pdf-1-7-mutate"],
      "comparisonProfiles": ["semantic-pdf-v1"],
      "license": "MIT",
      "testOnly": true
    }
  ],
  "mutationCatalogs": [
    {
      "id": "pdf-1-7-any",
      "capability": "pdf-1-7-mutate",
      "kinds": ["no-mutation", "set-snapshot", "insert-page", "remove-page", "..."]
    }
  ]
}
```

`ecosystem` must be `"rust"` — that is the value every working precedent uses and what the runner
maps to a cargo host. `"cargo"` is rejected at run time with "needs a cargo adapter to run in".

**You must register your own oracle entry.** The existing shared entries declare capabilities like
`pdf-edit` and `jpg-raster`, not `…-mutate`; reusing one fails the contract with
`oracle-capability-mismatch`. Give yours a distinct id, point it at the same package, and declare
exactly the capability your feature tags. The registry merges every discovered manifest repo-wide,
so a new id never collides and nothing central needs editing.

`kinds` is the **kebab-case spelling of every variant of your subset's `XMutation` enum**
(`InsertPage` → `insert-page`), including `NoMutation` and `SetSnapshot`. Omit nothing: the contract
fails if a declared kind has no scenario, and also if a scenario exercises a kind the catalog does
not declare. Do **not** use `deferredKinds` — this wave defers nothing.

Beside the enum add `pub const KINDS: &[&str]` and a `#[test]` asserting it matches both the enum's
variants and the manifest's `kinds`. That is what keeps the declaration honest — the framework never
parses Rust.

## 2. The feature file

`<artifact>/🧪️tests/mutate-<fmt>-<std>/component.feature`. The case sits at the artifact root (the
existing convention, and what lets `asset://` reach examples committed under any standard), while
the vocabulary it claims is the subset's.

```gherkin
@capability-pdf-1-7-mutate
@oracle-lopdf-pdf-1-7-mutate
@comparison-semantic-pdf-v1
@mutations-pdf-1-7-any
Feature: Apply every typed PDF 1.7 mutation to a real-world document
  The input is a real 65-page bachelor thesis produced by LaTeX, not a synthetic fixture, and it is
  read where the domain already keeps it. Every scenario copies it into the case work directory
  before touching it; the committed document is never written to.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id          | params       |
      | remove-page | {"index": 7} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    ... the same Examples table ...

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
```

Parser rules, all enforced:
- Expanded scenario id is `<baseId>-<row.id>`, so `@id-mutate` + row id `remove-page` yields
  `mutate-remove-page`. The gate looks for exactly `mutate-<kind>` and `inverse-<kind>`.
- Row ids are kebab-case and must equal the catalog's `kinds` spellings.
- A cell may not contain a bare `|` — escape it `\|`.
- Exactly one `@level-` and one `@mode-` tag per scenario.
- You are the first case in the repo to use `Scenario Outline`; the parser supports it fully.

**Fixture schemes**, all digest-pinned at plan time: `asset://<path-under-the-artifact-root>` reads a
real artifact where it already lives — use it for large real-world files and never copy a
multi-megabyte document into a fixtures directory. `shared://<name>` reads
`<artifact>/🧫️fixtures/<name>`; `local://<name>` reads `<case>/🧫️fixtures/<name>`.

## 3. The oracle dispatcher

Fill in `oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String>` in your
subset's `🧪️oracle/🦀️component.rs`, keyed on `spec.str("kind")`, performing each mutation with the
registered reference library and returning the re-serialized bytes. An unknown kind must return
`Err`, never a silent no-op — a quietly skipped mutation reports as a pass.

**Do not edit `🧪️oracle/📦️packages/🦀️rust/Cargo.toml` or `📦️lib.rs`.** If you need a library that is
not linked, stop and report it. Already available: `pdf-writer, lopdf, png, gif, zip, flate2, hound,
csv, image, tobj, stl_io`.

## 4. The adapter

**Registration is by FULL expanded scenario id.** `Adapter::oracle(id, handler)` keys on the exact
id the plan carries (`mutate-remove-page`), not on the outline's base id, and a missing registration
is a hard error, never a skip. So register in a loop over your kinds — the same shape
`🗿️artifacts/🧊️obj/🧪️tests/create-and-round-trip-obj/🦀️component.rs` already uses:

```rust
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        { built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse); }
    }
    built.oracle("identity-round-trip", round_trip_oracle)
}
```

The handler reads `ctx.doc_json()`, takes `spec.str("kind")`, and dispatches into
`oracle_apply_mutation`. Other useful context methods: `ctx.copy_fixture(uri, Some("input.pdf"))`
returns a mutable copy's path, `ctx.fixture_bytes(uri)` reads without copying.

`🦀️component.rs` in your case directory, structured like the working example at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/edit-existing-pdf/🦀️component.rs`: oracle handlers
at top level, subject handlers inside `#[cfg(feature = "sut")] mod subject`, both projected through
the same independent reader before comparison.

### The no-byte-pass-through rule — the point of this wave

The subject must **fully parse the artifact into the typed snapshot and re-serialize from it**.
Copying, splicing or patching source bytes is cheating and will be rejected in review.

```rust
let snapshot = decode_<fmt>(&input)?;          // complete semantic parse
let text = encode_<fmt>_text(&snapshot)?;      // the subset's own text codec, if it has one
let snapshot = decode_<fmt>_text(&text)?;      // the only channel from input to output
let output = encode_<fmt>(&snapshot)?;         // re-serialized from the model alone
if output == input { return Err("byte pass-through: output is bit-identical to the input".into()); }
```

The tripwire is real: our encoder cannot reproduce another writer's object layout, so identical bytes
mean the input was smuggled rather than parsed. What must PASS is the semantic projection of output
versus input under the case's `@comparison-` profile, metadata excepted.

**Lossy formats**: comparison tolerance is per-number and absolute, with no aggregate mode, so never
put raw lossy sample arrays in a projection. Follow the existing precedent in
`🧪️oracle/🖼️raster/🦀️component.rs`, where the JPEG projection reports geometry plus an 8-bucket luma
histogram, and the GIF projection reports palette-resolved counts, rather than pretending exact
samples survive.

## 5. Verify before you report

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```
bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-<fmt>-<std>
bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-<fmt>-<std>
```

Contract must report zero breaches and the oracle phase must actually execute your scenarios green.
Report the real output. Do not claim a phase passes without running it.

**The Rust SUBJECT phase cannot compile right now** — a parallel session is mid-refactor in the
os-kernel (`📡️spr/🧵️channel` has a `semio_framework::` cycle). That is expected and is not your bug.
Write the subject half anyway, keep it `sut`-gated, and verify the oracle phase only.

## 6. When the reference library can only READ

A differential oracle must PRODUCE a result to be compared against. If your assigned crate can parse
the format but cannot re-serialize it, you must not type your scenarios `@mode-differential` — there
would be no second producer, and the contract will reject it. Instead:

- Use the third-party library as the **independent reader** that projects both the real input and our
  re-serialized output, and type the scenarios `@mode-round-trip` and `@mode-property`.
- Say plainly in the Feature description that the reference reads but does not write, and what that
  costs in evidence.
- Register the crate as your oracle anyway — it IS the independent parser — but do not claim
  differential coverage you do not have.

If no credible crate exists at all, do not register a weak one. A first-release or long-unmaintained
crate put in the position of judging our implementation is worse than none. Record a
`noOracleDecision` naming its substitutes (specification vectors from the standard, plus the inverse
law as a metamorphic property) and type the scenarios accordingly. Report the decision in your
summary so the coordinator can confirm it.

**Never make a scenario pass by comparing our output against our own output.** That compares an
implementation with itself and is the precise failure this platform exists to prevent. A documented
failure is a better result than a green that hides one.

## Hard rules

- Stay inside your artifact. Do not edit another artifact's files, the framework, the taxonomy, the
  shared stdio oracle manifest, the shared family modules' existing functions, `Cargo.toml`,
  `📦️lib.rs`, `.gitignore`, `project.json` or `launch.json`. Adding a new helper to a shared family
  module is allowed only if two subsets genuinely share it; say so in your report.
- **Do not close or reopen the ticket** — the coordinator owns it.
- Do not run `git commit`, `git stash`, `git checkout`, or any other modifying git command.
- Other sessions edit this tree concurrently. If a file you do not own changes underneath you,
  ignore it and continue.
- Scratch files go in this ticket folder, never in the source tree.
- Write code that reads like the code around it: regions, docstrings starting with an emoji, no
  comments inside definitions, concise. No placeholder or throwaway implementations.
