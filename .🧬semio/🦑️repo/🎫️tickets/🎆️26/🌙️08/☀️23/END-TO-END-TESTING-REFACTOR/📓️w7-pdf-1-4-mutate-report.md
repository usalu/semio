# Wave 7 — 📄️pdf standard 🔖️1.4 subset ✳️any — mutation oracle + exhaustive round-trip case

Executor report for the fleet brief's PDF 1.4 subset assignment. Files touched are listed at the
bottom; nothing outside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/…/🔖️1.4/…` and the new
`🧪️tests/mutate-pdf-1-4` case directory was edited.

## 1. Is the 1.4 subset a deliberately reduced profile, or an unfinished stub?

**Unfinished stub, explicitly documented as such — not a deliberately reduced permanent profile.**
Evidence, all read directly from the code, not inferred:

- `🧬️schema/📸️snapshot/🦀️component.rs`'s own doc comment on `demo_pdf_snapshot`: `decode_pdf`
  "hardcodes those two literals unconditionally (never parses them back out of the encoded bytes,
  the documented **"1.4 stays a frozen stub"** scope boundary)".
- `🔺️diff/🦀️component.rs`'s file header: "1.4 stays the intentionally-frozen pre-real-codec stub
  (W0 recon: **"keep it minimally alive under its own path — do NOT give it the full 1.7
  object-graph model, that would contradict its own documented scope boundary"**)".
- `📸️snapshot/🟦️component.ts` and `📸️snapshot/💾️binary/📡️component.protocol.semio` repeat the same
  "intentionally-frozen pre-real-codec stub" / "no object-graph parsing exists at all" language.
- The originating design report (`…/2026-08-10/ARTIFACT-SYSTEM-OVERHAUL-…/f4-pdf-report.md`) labels
  1.7 the "main target" and describes 1.4's own scope as staying frozen at that boundary while 1.7
  got the real object-graph model, sparse diff and mutation vocabulary.
- The shared example fixture `📚️examples/🎓️bachelor-thesis` (physically filed under 1.4's directory
  tree only because that's the artifact-level examples convention) explicitly decodes through
  **1.7's** engine in its own `decoded_summary_json()`, never through 1.4's — even the example
  module itself doesn't trust 1.4's codec with the real fixture.

So this is a live TODO frozen mid-flight, not a considered "1.4 only needs this much" design
decision. Concretely, `decode_pdf` (`🚪️io/🦀️component.rs`) hardcodes `width=612.0`/`height=792.0`
for **every** input regardless of the real page size, and extracts `text` via a raw byte search for
the first `stream`…`endstream` pair anywhere in the file, then the first `(…)` run inside it — no
object graph, no xref, no page tree, one page always.

**Confirmed empirically against the real fixture** (`asset://…/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf`,
6,346,331 bytes, header `%PDF-1.5`): the file's real page 1 `MediaBox` is `[0 0 595.276 841.89]`
(A4) — `decode_pdf` would still report `612×792` (US Letter) regardless. `decode_pdf` does **not**
error on this file (its only validation is `starts_with(b"%PDF")`), but the snapshot it produces
(`width=612, height=792, text="SemIO"`) reflects almost none of the real 65-page document — 64
pages, the real geometry, and 99.9% of the extracted text are silently discarded, not rejected.
"Can read it" and "represents it" are two different claims here, and only the first is true.

## 2. What this wave built, and why the case is honest anyway

The vocabulary itself (`NoMutation`, `SetSnapshot`) is real and small, not fake: `SetSnapshot` is a
genuine full-replace of the thin `{width, height, text}` model, and `NoMutation` is a genuine
identity. Both are exercised end to end against the real 6.3MB document, never a synthetic one —
`decode_pdf` not erroring on it is used deliberately as the case's own input, per the brief's "if
the codec can read it" branch.

The one thing that can't be done honestly is treating `width`/`height` as something `decode_pdf`
"reads" from a real document — it doesn't, for any input, ever. Rather than fabricate that or (the
other extreme) leave a `mutate-no-mutation` scenario that's guaranteed to fail because an
independent reader's real `595×842` will never equal the stub's `612×792`, the oracle module
(`🧪️oracle/🦀️component.rs`) draws the SAME scope boundary the subset's own pre-existing unit test
already draws (`codec_retention_law_text_round_trips_through_encode_decode`, which only asserts
`text`, explicitly not `width`/`height`, "documented pre-real-codec scope boundary"): its
independent writer (`build_single_page_pdf`, built object-by-object through `lopdf`, never
delegating to this repo's own `encode_pdf`) always emits the same `612×792` the stub hardcodes, so
`width`/`height` in the projection check "does this geometry constant round-trip consistently
through apply/inverse", never "does `decode_pdf` read real page geometry" (it doesn't, and fixing
that is out of this wave's scope — the whole codec, not just the mutation surface, would need
rebuilding). `text` — the one field `decode_pdf` actually threads through — is read independently
via `lopdf`'s own content-stream decoder (real page 1, real `Tj`/`TJ` operators), never via this
repo's byte-search decoder, and independently confirmed to equal `"SemIO"` for the real fixture
(both by direct inspection and by the standalone check in §4).

## 3. Does 1.4 need a richer vocabulary to be worth having? (for the 1.7 peer / coordinator)

**Yes, if it's meant to be more than a frozen placeholder.** As shipped, the two-kind
vocabulary is real but the SNAPSHOT it mutates is not: no object graph, no independently-readable
page geometry, one page always. `NoMutation`/`SetSnapshot` are honestly testable (this wave proves
that), but nothing PDF-1.4-specific can be added to the vocabulary (e.g. anything object- or
page-tree-shaped, matching 1.7's `InsertPage`/`RemovePage`/etc.) without first giving 1.4 a real
decode/encode pair — which is explicitly out of THIS wave's scope and, per the ticket's hard rules,
not something I invented variants for. That is a separate, larger piece of work than wave 7; I did
not touch 1.7's vocabulary or file, per the brief.

## 4. Standalone verification (the oracle crate is currently blocked by an unrelated sibling file)

**Contract** (`bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-pdf-1-4`): the only breaches
reported are (a) `testing/contract mutation-catalog-unclaimed` for OTHER artifacts' catalogs
(zip/gif/pdf-1-7/csv/jpg/png/wav/tiff/deflate/obj/stl) and (b) pre-existing `testing/dependency
oracle-in-production` findings in `animate`/os-host/`tiled-map` — confirmed pre-existing and
unrelated by re-running the identical command against the already-accepted `edit-existing-pdf` case,
which reproduces the exact same list (that check evaluates the WHOLE registry against only the
`--case`-filtered feature set, so it always lists every catalog the filtered-out cases would have
claimed). Nothing on the list names `pdf-1-4`, and a full `--owner 🗄️stdio` run (no `--case` filter)
confirms `pdf-1-4-any` is claimed and produces zero breaches of any kind for my catalog specifically
(`grep -i "pdf-1-4"` on that output: no hits). My own case passes every check
`validateCaseContract` performs (capability, comparison profile, oracle id, oracle
capability/profile match, mutation-catalog completeness, adapter presence, scenario syntax).

**Oracle exhaustive** (`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-pdf-1-4`):
blocked. `semio-s-plugin-stdio-test-oracle` is ONE crate mounting every stdio artifact's
`🧪️oracle/🦀️component.rs` (`📦️lib.rs`, read-only per the hard rules), so it must compile as a whole
before ANY case's oracle role can run. Across four attempts spaced through this session, the
compiler errors moved (a csv `E0502` borrow error present on the first two attempts was gone by the
third — another wave-7 session fixing its own file live) but a `📷️png` file still fails on every
attempt:
```
error[E0106]: missing lifetime specifier
  --> …/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs:41:51
   |
41 |     fn as_str(params: &Json, key: &str) -> Option<&str> {
   |                       -----       ----            ^ expected named lifetime parameter
```
This is not my file and not my artifact; the fleet brief's "Rust SUBJECT phase cannot compile right
now" warning covers a different, unrelated cycle (`📡️spr/🧵️channel`) — this is the ORACLE crate,
blocked by a different sibling's in-progress png work. I did not edit `📷️png`'s file, per "stay
inside your artifact".

**Given that block, I independently verified the PDF-1.4-specific logic** (`independent_first_text`,
`project_pdf_1_4`, `build_single_page_pdf`, `oracle_apply_mutation`) in an isolated scratch crate
outside the repo tree (`lopdf = "=0.44.0"`, the exact registered version), copying the functions
verbatim with only the `Json` type swapped for a local structural twin of
`semio_repo_test_host::Json`. All three tests pass, including one that runs
`independent_first_text`/`project_pdf_1_4` against the REAL, committed `bachelor-thesis.pdf` bytes
(not a synthetic stand-in) and confirms the independently-read `text` is exactly `"SemIO"`, matching
what `decode_pdf`'s own naive extraction finds on the same file (confirmed separately by a direct
Python simulation of `decode_pdf`'s byte-search algorithm before writing any Rust). Scratch crate
left in this ticket folder at `w7-pdf-1-4-mutate/lopdf-check/` (`cargo test` from that directory
reproduces the three green tests).

I could not run the real `oracle exhaustive` command green because of the blocker above — reporting
this plainly rather than claiming a pass. Re-running
`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-pdf-1-4` from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` once the png fix lands (one line, not mine to make)
should be the only remaining step; nothing else in this case is expected to need changes for it to
go green, per the standalone verification above.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS` + `oracle_mutation_kinds_law_matches_enum_variants` /
  `oracle_mutation_kinds_law_matches_manifest_catalog` tests.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  — new: oracle registration (`lopdf-pdf-1-4-mutate`) + mutation catalog (`pdf-1-4-any`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — filled in `oracle_apply_mutation` + independent reader/writer helpers.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/component.feature` — new.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🦀️component.rs` — new adapter.

No shared family module (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs`) function was
added or modified — this subset's projection/writer needs are narrow enough (and different enough
from 1.7's real object-graph projection) that they belong in this subset's own oracle module, not
the shared one.
