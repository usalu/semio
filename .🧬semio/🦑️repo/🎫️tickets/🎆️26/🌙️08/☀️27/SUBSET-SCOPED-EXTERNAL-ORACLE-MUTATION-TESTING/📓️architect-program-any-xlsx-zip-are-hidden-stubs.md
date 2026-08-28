# 📓️ `architect/program@1/any` (266 mutations) — its xlsx and zip exporters are stubs, a FOURTH shape the gate cannot see

**Verdict: NO oracle registered. Per the pilot playbook Step 0 ("say so plainly and STOP rather than
registering an oracle against bytes no reader can interpret"), this is that outcome.**

This subset was handed to me as "the biggest un-oracled block (266 mutations) … measured to have REAL
(non-stub) `xlsx` and `zip` exporters." `📓️reachability.md:43` and `📓️gap-report.json#15` both record
the same premise. Reading the two `serialize_bytes` bodies byte-for-byte — not the directory tree, not
`stubSerializerBreaches`'s verdict — shows the premise is wrong. Both exporters compile, both run, both
return `Ok`, and **neither one ever puts a mutation's effect into the bytes**, regardless of what the
mutation did.

## The bug, quoted

`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️component.rs`:

```rust
pub async fn serialize(snapshot: &ProgramSnapshot) -> Result<XlsxSnapshot, store::TextError> {
    let _ = STDIO_XLSX_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(...)?;
    serde_json::from_value(value).map_err(...)?
}
```

The same pattern, verbatim, in the sibling zip serializer
(`.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs`):

```rust
pub async fn serialize(snapshot: &ProgramSnapshot) -> Result<ZipSnapshot, store::TextError> {
    let value = serde_json::to_value(snapshot).map_err(...)?;
    serde_json::from_value(value).map_err(...)?
}
```

`ProgramSnapshot` (`.../🧬️schema/📸️snapshot/🦀️component.rs`) has fields `schema, meta, project,
stakeholders, users, activities, … governance` — 66 named `Vec<Record>` registers plus `meta`/`project`/
`governance`. `XlsxSnapshot` has exactly THREE fields: `schema: String` (no default), `opc: OpcPackage`
(`#[serde(default)]`), `workbook: XlsxWorkbook` (`#[serde(default)]`). `ZipSnapshot` has `schema: String`
(no default), `entries: Vec<ZipEntry>` (`#[serde(default)]`), `comment: String` (`#[serde(default)]`).
`OpcPackage`'s own four fields are ALSO all `#[serde(default)]`.

`serde_json::to_value(program)` produces a JSON object keyed `schema`, `meta`, `project`,
`stakeholders`, … — none of which are `opc`/`workbook` (xlsx) or `entries`/`comment` (zip).
`serde_json::from_value` does not reject unknown keys (no `deny_unknown_fields` anywhere in this chain)
and every field the target actually needs beyond `schema` carries `#[serde(default)]`. The
deserialization therefore **succeeds** — it does not error, so `stubSerializerBreaches`' regex gate
(which only matches `print_dsl(...)` and `encode_pack(snapshot)…decode_pack(&bytes)` transmutation) sees
neither shape and reports nothing. What it actually returns is `XlsxSnapshot { schema: "architect.program",
opc: OpcPackage::default(), workbook: XlsxWorkbook::default() }` — an empty, valid, zero-sheet workbook —
and the zip equivalent, an empty, valid, zero-entry archive. Every stakeholder, element, adjacency,
requirement, decision, risk … is silently dropped. No mutation to any of the 266 kinds can ever be seen
in the resulting bytes, because the bytes never carried the snapshot's content in the first place.

## Verified two independent ways (not assumed)

1. **Read the actual struct definitions** confirming the field-shape mismatch and the exact
   `#[serde(default)]` annotations quoted above (`XlsxSnapshot`/`XlsxWorkbound`/`OpcPackage` at
   `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/…/📸️snapshot/🦀️component.rs`; `ZipSnapshot` at
   `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/…/📸️snapshot/🦀️component.rs`).
2. **A standalone serde_json repro** (kept at
   `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🔬️architect-xlsx-zip-stub-repro/`)
   with structs shaped exactly like the two real types — `ProgramLike { schema, meta, stakeholders }`
   serialized, then deserialized into `XlsxLike { schema, opc: #[default], workbook: #[default] }`.
   Real, quoted output:
   ```
   program JSON = {"meta":"Sample Clinic","schema":"architect.program","stakeholders":["Facilities Director","Reception"]}
   xlsx-like result = XlsxLike { schema: "architect.program", opc: OpcLike { parts: [] }, workbook: WorkbookLike { sheets: [] } }  (sheets.len()=0, parts.len()=0)
   ```
   Deserialization succeeds silently; the content is gone.

I also tried to confirm this directly against the real crate with `cargo test -p semio-s-plugin-architect`
(an isolated `CARGO_TARGET_DIR` under this ticket's scratch, a temporary `#[cfg(test)]` added and then
removed from the xlsx serializer file — `git diff` on that file is now empty, nothing left committed to
the tree). It could not finish: `semio-s-plugin-stdio` currently fails with 4620 compile errors
(`SvgMutation`/`JsonMutation`/`XmlMutation`/`PdfMutation`: `Mutation<T>` not satisfied), and
`📌️status.md`'s own "Blocked" section already attributes this to a peer's in-flight refactor
(`d394744295`, a new `Mutations` derive check) landing after this ticket's baseline — not to anything in
this investigation, and not chased further per repo rules. The repo-wide `git status` also shows ~1856
pending renames under `stdio` (`component.feature` → `🥒️.feature`, `🦀️component.rs` → `🦀️.rs`) — the
same concurrent churn. The standalone repro above needed none of that broken crate graph and is the
basis for the verdict.

## This is confirmed by the repository's own gate, negatively

Ran `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts contract` for real (full output at
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`). It flags `program-1-any`'s **csv** serializer as
`stub-serializer` ("emits the artifact's internal DSL text, not csv" — this one was already fixed
honestly, see its own doc comment history) and flags `capability-without-manifest` /
`binary-protocol-drift` for the same 266-kind catalog, but **raises nothing at all for the xlsx or zip
serializers** — because the JSON-value type-confusion shape they use matches neither of
`stubSerializerBreaches`' two known patterns. This is a THIRD OMISSION the same audit family missed:
`📓️reachability.md` already documents that the shipped gate under-reported by 62% across three known
false-negative classes (`print_dsl` text, `encode_pack`/`decode_pack` envelope transmute, text-only
serializers). JSON-`Value`-shape transmute through mismatched `#[serde(default)]` fields is a fourth,
and this pair (xlsx + zip, both owned by `architect/program`) is — as far as this investigation went —
the only occurrence of it found. Repo-wide, `zip` already has 5 known-stub serializers and `xlsx` 1
(`📓️reachability.md`); this pair are two MORE, uncounted anywhere yet.

## What this means for the 266 mutations

None of them are reachable through xlsx or zip today. csv is a known, honest stub (wraps the whole DSL
text as one quoted field — no external csv reader can check anything through it either). json is
explicitly excluded by policy (own-schema-in-JSON, not a third-party-checkable format). txt is `print_dsl`
by construction. **Every declared export dialect for this artifact is unreachable as a carrier oracle
right now** — the "664 mutations behind a real carrier" count in `📓️reachability.md:41-43` should not
include this subset's 266; they belong in the "blocked on an exporter being written first" bucket
(currently counted as 1047) alongside the 130 already-known stub serializers.

I did not edit `📓️reachability.md` or `📓️gap-report.json` myself — both are live shared ticket records
another session may be actively extending, and the correction is one line each; recorded here instead so
whoever next reconciles those totals has the exact citation.

## What would actually unblock this

Someone has to write real field-mapping code: `ProgramSnapshot`'s ~66 named registers into
`XlsxWorkbook.sheets` (one sheet per register, one row per record, one column per field — the same
shape `stdio.xlsx`'s OWN mutation catalog already round-trips for its ~229-entry committed fixture,
so the target type is capable) and into `ZipSnapshot.entries` (e.g. one member per register, csv- or
json-encoded). That is a real, scoped implementation task — writing two production serializers for a
66-register schema — not an oracle-registration task, and not something to rush inside this ticket
against CLAUDE.md's "no pragmatic shortcuts, clean long-term solution" rule. `calamine` 0.36 (reader) and
`zip` 6 (reader) are already `🔒️dependencies.json`-approved `test-oracle`s for exactly
`xlsx-ecma-376-mutate` / `zip-2-0-mutate`, so once the exporters are real, the oracle registration this
ticket wanted can proceed with no new dependency approval needed — mirroring
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/…/✳️any/🧪️oracle/🦀️component.rs`'s existing
`calamine` + `rust_xlsxwriter` pairing (already built and tested for `stdio.xlsx`'s OWN mutation
vocabulary, just not reachable from `architect/program`'s broken export path).

## Files touched by this investigation

- Read only (no lasting change): every file quoted above, plus
  `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs`,
  `.../🚪️io/🦀️component.rs` (the composer registry proving these are the actual production export paths,
  not dead code).
- Temporarily edited then fully reverted (`git diff` empty):
  `.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️component.rs`.
- New, kept: this file, and
  `🔬️architect-xlsx-zip-stub-repro/{Cargo.toml,src/main.rs}` (the standalone serde repro above).
