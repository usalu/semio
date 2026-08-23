# TSV IANA/any mutation-oracle case — summary

Wave 7 subset: `📑️tsv` standard `🔖️iana` subset `✳️any` (7 `TsvMutation` variants: `NoMutation`,
`SetSnapshot`, `SetTrailingNewline`, `SetLineEnding`, `InsertRow`, `RemoveRow`, `SetCell`).

## Fixture: derived once from the real committed CSV fixture, provenance chain recorded

No genuine TSV file existed in the repo beyond the tiny 6-row demo asset. Rather than invent
synthetic data, this case reuses the csv rfc4180 wave's real committed research table one hop
further:

1. Original source: a genuine systematic survey of 50 European building-component reuse
   marketplaces (`♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex`).
2. Hop 1 (already committed by the csv wave): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv`
   — 51 records (1 header + 50 data rows) × 12 columns, RFC 4180, CRLF.
3. Hop 2 (this wave): standalone Rust binary `tsv-iana-any-fixture-gen` (this ticket folder) reads
   that same CSV with the `csv` crate (`has_headers(false)`, `flexible(true)` — same convention the
   csv oracle module itself uses) and writes it back out through the SAME crate, reconfigured for
   IANA TSV: tab delimiter, `QuoteStyle::Never` on write / `quoting(false)` on read (TSV has no
   quoting mechanism at all). Before writing anything, the program scanned all 51×12 real cells for
   literal tab or newline bytes — IANA TSV cannot represent either, having no escaping mechanism —
   and found **none**. Many cells are genuinely comma-laden (`"Beschreibung, Bilder, Preis, Menge,
   Materialstandort"`), but a comma is not TSV's delimiter, so those pass through unmodified with no
   policy decision required.

Committed to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🧫️fixtures/reuse-marketplaces.tsv` (11,484
bytes), LF-terminated, trailing newline present. Referenced as `shared://reuse-marketplaces.tsv`.

## Comparison profile: a subset-owned variant, not the shared one

RFC 4180 CSV's own mutation case (`mutate-csv-rfc4180`) reuses the shared `semantic-tabular-v1`
profile, which deliberately ignores `trailingNewline`/`lineTerminator` as CSV writer freedom. IANA
TSV draws no header/data distinction for `set-trailing-newline`/`set-line-ending` to stand in for
the way CSV's `has_header` convention does — those two kinds are this subset's own real
serialization concerns, genuinely rewriting bytes. Reusing the ignoring profile would make both
kinds no-ops. Following the `zip-2-0-any` precedent (`semantic-archive-mutate-v1`, which keeps
`comment` live for the same reason), this case declares its own `semantic-tabular-mutate-v1`
profile in its own catalog (`🧪️oracle/🔣️component.json`), differing from the base profile only in
that `trailingNewline`/`lineTerminator` are NOT ignored.

## Oracle: the already-linked `csv` crate, reconfigured

No new dependency. The registered `csv` crate (already linked for RFC 4180) is driven a second time
under a distinct oracle id (`tsv-iana-mutate`, capability `tsv-iana-mutate`) and configuration: tab
delimiter, no quoting on either side. `read_grid`/`write_grid`/`project_tsv_grid` are exposed
publicly from the subset's own `🧪️oracle/🦀️component.rs` for the test case adapter to build
inverse-mutation specs from real pre-mutation state, mirroring the csv case's `read_grid`/
`write_grid` pattern exactly.

## The identity-round-trip scenario needed the artifact's real document codec, not the bare grid codec

The subject's own bare `decode_tsv`/`encode_tsv` pair (`../🧬️schema/📸️snapshot/🦀️component.rs`) is
a byte-exact split/rejoin BY DESIGN — TSV has no quoting mechanism to invent and every retention
field (line ending, trailing newline) is captured and replayed verbatim — so a raw decode/encode
round trip through those two functions alone can never diverge from a well-formed input. Unlike RFC
4180 (whose encoder always writes LF regardless of source, guaranteeing divergence against the
CRLF-committed csv fixture), TSV genuinely has no writer freedom left once quoting is off the table,
so that approach would make the pass-through tripwire untestable rather than satisfied.

The subject's `identity_round_trip` instead goes through this artifact's REAL document codec —
`store::ArtifactDsl::parse_dsl`/`print_dsl`, the same pair `register_document_codec` wires into
production (`../🚪️io/🦀️component.rs`). `print_dsl` always prepends the `semio iana.tsv.dsl v1`
envelope line (confirmed by reading `wrap_text`/`split_text_preamble` directly), which the real
committed fixture does not carry — a genuine writer choice this artifact's persisted form makes, not
a fabricated difference. The projection compared against the oracle is computed from the body alone
(`encode_tsv(&snapshot)`, envelope stripped back out) so the independent reader compares the same
grid shape on both sides; the raw enveloped bytes are what the pass-through check itself runs
against.

## Verification (run from 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test)

`bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-tsv-iana`:
exit code 1, 2 high-priority breaches, **neither names tsv** — both belong to `ply-1-0-any` and
`dxf-r12-any`, subsets other peers are still writing:
```
testing/contract  ply-1-0-any  Mutation catalog ply-1-0-any (10 kinds) is claimed by no feature
testing/contract  dxf-r12-any  Mutation catalog dxf-r12-any (19 kinds) is claimed by no feature
```

`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-tsv-iana`:
```
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0
```
15/15 = 7 kinds × (mutate + inverse) + 1 identity-round-trip.

The Rust **subject** phase was not compiled this wave (repo-wide os-kernel refactor mid-flight, per
the fleet brief §5 — expected, not this subset's bug). The subject half is written and `sut`-gated
so it compiles into the subject role the moment that lands; only the oracle phase is verified here.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS` + `kinds_match_enum_and_catalog` test (no other change to the
  pre-existing vocabulary).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — filled in (was a rejecting stub): `oracle_apply_mutation`, `read_grid`/`write_grid`/`TsvGrid`,
  `project_tsv_grid`, unit tests.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  — new: oracle registration + mutation catalog + `semantic-tabular-mutate-v1` profile.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🧫️fixtures/reuse-marketplaces.tsv` — new, real derived
  fixture.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🧪️tests/mutate-tsv-iana/component.feature` — new.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🧪️tests/mutate-tsv-iana/🦀️component.rs` — new.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/tsv-iana-any-fixture-gen/`
  — scratch derivation program (this ticket folder).

No shared/framework file was edited. No new runtime dependency was added — the `csv` crate was
already linked behind the oracle crate's `oracles` feature.
