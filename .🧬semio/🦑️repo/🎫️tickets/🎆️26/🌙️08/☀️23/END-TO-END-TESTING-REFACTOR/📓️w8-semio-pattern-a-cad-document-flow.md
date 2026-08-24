# Wave 8 — semio Pattern-A subsets `✳️cad`, `✳️document`, `✳️flow`

Scope: the three 🧿️semio Pattern-A subsets that already owned a mutation vocabulary but had no
`pub const KINDS`, no `🧪️oracle` manifest and no test case.

## What was found before anything was written

All three subsets **already have a handcrafted `🧬️schema/🧬️mutations` of their own** — a single
named-variant enum with handcrafted `diff()`/`inverse()` arms, a hand-rolled `OpText`/`OpBinary`
codec and one `📄set-snapshot` fixture leaf. None of them delegates to `✳️any`. Step 1 of the brief
("if it has no mutations, handcraft one") therefore did not apply; the gap was the declaration
(`KINDS`), the oracle manifest and the case.

The three vocabularies are genuinely distinct, and the distinction is structural rather than
cosmetic:

| subset | kinds | what makes it its own vocabulary |
|---|---|---|
| `✳️cad` | 16 | the only one addressing TWO nested name-keyed collections — four `*-block-entity` kinds reach an entity by handle INSIDE a named block definition |
| `✳️document` | 18 | the only one with a path-addressed recursive tree — `DocBlockPath` descends through `Quote` / list-item / table-cell containers before an index picks a slot |
| `✳️flow` | 13 | the only one with a two-level key — nodes by id, and a node's own `params` by `(id, key)` |

## What was written

Per subset:

1. `pub const KINDS: &[&str]` beside the enum, replacing the private `OP_KEYWORDS` ordinal table so
   the binary op frame's `tag`, the text grammar's keyword and the catalog all read one const.
2. A plain `#[test] kinds_match_the_enum_and_the_catalog` — asserts the length, walks
   `demo_mutation_cases()` asserting `KINDS[variant_ordinal(m)] == print_*_mutation(m)`'s keyword,
   asserts every ordinal was reached, and asserts every spelling appears in the committed manifest.
3. `🧪️oracle/🔣️component.json` — a recorded `noOracleDecision` with
   `substitutes: ["specification-vectors", "metamorphic-laws"]`, plus the `semio-v1-<subset>` catalog.
4. `🧪️tests/mutate-semio-<subset>/` — `component.feature` + `🦀️component.rs` + `🧫️fixtures/🦠️<kind>.json`.
5. Reachability wrappers in production so an owner-root test can actually drive the subset from
   outside the crate: `parse_semio_X_dsl`, `print_semio_X_dsl`, `decode_semio_X_pack`,
   `encode_semio_X_pack`, `encode_semio_X_snapshot_json`, `decode_semio_X_snapshot_json`,
   `decode_semio_X_mutation_json`, `inverse_semio_X_mutation`. Same shape `✳️kit` established;
   every signature names only types reachable from `semio_s_plugin_stdio::…`.

## Why no oracle — and what was rejected on the merits

`.dsl.semio` / `.pack.semio` are defined by this repository alone, so nothing reads them. What makes
the decision worth recording rather than assuming is that each subset ships export serializers that
make a third-party oracle *look* available, and each fails for a specific reason:

- **`✳️cad` → `dxf` 0.6** (already linked). The oracle role may never link the subject crate, so the
  only route to a DXF is through this repository's own exporter — the reference would be an artefact
  of the implementation it judges. Independently, DXF's `BLOCKS` section has no per-entity handle
  addressing that survives a write-read cycle, stranding 4 of 16 kinds.
- **`✳️document` → `comrak` 0.54** (already linked, reads AND writes CommonMark). Same
  exporter-dependency problem, plus CommonMark has no named style table, no id-keyed image store and
  no run-level formatting that survives a parse-render cycle — 10 of 18 kinds would have nothing to
  compare against.
- **`✳️flow` → `json` 0.12** (already linked, reachable through the subset's JSON export). Technically
  possible and **declined deliberately**: json-rust contributes a generic DOM and zero knowledge of
  nodes, edges, ports or params, so all 13 mutation semantics would still be hand-written on top of
  it. That is the weak oracle the brief forbids.

Cross-language hosts have landed (Rust, Python, TypeScript, Go, .NET). A Python/JavaScript reference
was checked and fails one level earlier: no package in any registry reads a semio envelope at all.

## The substitutes are real, not a formality

The committed `(before, mutation, after)` vectors were derived by an **independent Python
implementation** of both the committed DSL grammar and each vocabulary's specification
(`🐍️semio-pattern-a-vectors.py`), starting from this standard's own committed real artifacts. None of
this repository's Rust was executed to produce them.

Three verifications were actually run, and all three pass:

1. **`🐍️semio-pattern-a-decoder-check.py`** — re-encodes each decoded real artifact back to
   `.dsl.semio` with an independently written Python encoder and compares byte for byte with the
   committed file:
   ```
   flow   re-encode is byte-identical to the committed artifact (249 B)
   cad    re-encode is byte-identical to the committed artifact (479 B)
   doc    re-encode is byte-identical to the committed artifact (610 B)
   ```
2. **Vector self-consistency** — 47 vectors, one shared before-snapshot per case, `no-mutation`
   changes nothing, every other kind changes something, `set-snapshot`'s after equals its payload,
   every after-snapshot survives a DSL round trip. 0 problems.
3. **`🦀️semio-pattern-a-serde-shape-probe.rs`** — a standalone crate carrying the REAL type
   definitions extracted verbatim from the production sources, which deserializes all 47 mutation
   payloads and all 94 snapshot payloads and re-serializes them to byte-equal JSON:
   ```
   mutate-semio-flow: 13 kinds checked, 0 problem(s)
   mutate-semio-cad: 16 kinds checked, 0 problem(s)
   mutate-semio-document: 18 kinds checked, 0 problem(s)
   TOTAL PROBLEMS: 0
   ```
   This matters because serde renames enum VARIANTS with `rename_all` but leaves struct-variant
   FIELDS snake_case — verified empirically rather than assumed.

## The subject handler asserts; it does not merely return Ok

A recorded no-oracle case runs no oracle role, so nothing compares its results. The existing semio
Pattern-B adapters return `Ok(projection)` from `mutate`/`inverse` without checking anything, which
means their subject phase would report a pass having verified nothing. These three adapters put the
assertion inside the handler and return a diffing `Err` when it fails:

- `mutate-<kind>` — applies to the committed before-snapshot and requires the result to EQUAL the
  committed after-snapshot.
- `inverse-<kind>` — applies the kind and then its own computed inverse and requires the committed
  before-snapshot back exactly.
- `identity-round-trip` — parses the REAL committed text artifact, requires it to equal the
  before-snapshot the vectors start from (this is what keeps the Python decoder honest), reprints and
  reparses it, decodes the committed BINARY twin and requires the same snapshot, and re-encodes and
  re-decodes a pack. Byte-identical text re-emission is the EXPECTED result here — the committed text
  is this codec's own output — so the wave's usual pass-through tripwire does not apply and the
  text/pack cross-check carries that evidence instead.

## Defect found and fixed

**`enc_block`'s `Quote` arm emitted a stray `.await` literal.**
`✳️document/🧬️schema/🔺️diff/🦀️component.rs:1293` read

```rust
DocBlock::Quote { blocks } => format!("Q[{}.await]", enc_list(blocks, enc_block)),
```

while `dec_block`'s `"Q"` arm (line 1324) expects `Q[<blocks>]`. Any document carrying a blockquote
therefore could not round-trip through its own DSL text codec, and this standard's committed real
memo carries exactly one. The committed artifact predates the defect, which is why it still holds the
correct form — and why the independent Python encoder, written from the committed grammar, reproduces
it byte for byte without the `.await`. That is the corroboration. Fixed to `format!("Q[{}]", …)`.

The literal is a leftover from an automated async sweep; a repo-wide grep for `format!("…\.await\]`
found this as the only occurrence.

## Finding NOT fixed — remove/insert is not position-preserving for name-keyed collections

`RemoveEntity`'s inverse is `AddEntity`, and `apply_named`
(`✳️cad/🧬️schema/🔺️diff/🦀️component.rs:76`) pushes additions at the END of the vector. Removing a
NON-FINAL entity and then applying its own computed inverse therefore restores the value but not the
position, and the inverse law fails. The same holds for `RemoveLayer`/`RemoveBlock`/
`RemoveBlockEntity` in cad and `RemoveNode`/`RemoveEdge`/`RemoveNodeParam` in flow — every
name-keyed collection. `✳️document`'s `RemoveBlock` does NOT have the problem: `blocks` is
index-keyed and `inverse_indexed` reinserts at the base index.

The repository has never exercised it: the in-crate `inverse_law` tests all remove the LAST element.
These vectors do the same (`remove-entity h8`, `remove-node n2`, `remove-node-param unit`) so the
cases can go green when the subject phase unblocks, and the cad generator carries a comment naming
the one-line change (`h8` → `h2`) that exposes it. Deciding whether the inverse should carry a
position, or whether appending is the intended semantics, is a schema decision for the owner — it is
not something to settle inside a test.

## Verified output

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-flow
0 high-priority breach(es) across 0 rule(s)
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-cad
0 high-priority breach(es) across 0 rule(s)
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-document
0 high-priority breach(es) across 0 rule(s)
$ bun ./📜️script.ts contract --owner 🗄️stdio
0 high-priority breach(es) across 0 rule(s)

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-flow
[test] not-exercised …/mutate-semio-flow (recorded no-oracle decision semio-flow-mutation-semantics
       — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
   … identical for mutate-semio-cad and mutate-semio-document
```

**Zero executed scenarios is the designed outcome for a recorded no-oracle case, not a pass.** The
runner skips the oracle role whenever `@no-oracle-…` is set (`📜️script.ts:504`), so these three cases
contribute no executed evidence today; their evidence is written and waiting on the subject phase.

The oracle HALF of each adapter was compiled standalone against `semio-repo-test-host` (no `sut`
feature, exactly as the generated oracle host builds it) — all three compile clean.

## Honest limits

- **The Rust subject phase does not compile**, so not one of the 47 assertions above has actually
  run. The failure is in `semio-framework-job`, upstream of everything:
  `error[E0308]` / `error[E0499]` / `error[E0599] no method named 'generation' found for struct
  'WorkerJobSession<J>'` — the `RetainedJobPayload` refactor named in the brief. `cargo check -p
  semio-s-plugin-stdio` and `-p semio-framework-os-kernel` fail the same way. My adapter is never
  reached.
- The `#[test] kinds_match_the_enum_and_the_catalog` in each production file is blocked by the same
  build failure. The catalogs were audited against `KINDS` from this ticket folder instead: all three
  match exactly and are complete against their enum's variant count (13 / 16 / 18).
- The real artifacts are the repository's own committed example documents (249 B / 479 B / 610 B),
  not multi-megabyte foreign files. For a semio-native format there is no such thing as a foreign
  real-world file; these are as real as the format gets, and they are the same artifacts the
  subsets' own example modules ship.

## Files

Production, per `<subset>` ∈ {`✳️cad`, `✳️document`, `✳️flow`} under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`:
- `<subset>/🧬️schema/🧬️mutations/🦀️component.rs` — `KINDS`, `inverse_semio_*_mutation`,
  `decode_semio_*_mutation_json`, conformance test
- `<subset>/🧬️schema/📸️snapshot/🦀️component.rs` — `🌉️ExternalCodecBridge` region
- `<subset>/🧪️oracle/🔣️component.json` — new
- `✳️document/🧬️schema/🔺️diff/🦀️component.rs` — the `Q[{}.await]` fix

Cases under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/`:
- `mutate-semio-{cad,document,flow}/component.feature`
- `mutate-semio-{cad,document,flow}/🦀️component.rs`
- `mutate-semio-{cad,document,flow}/🧫️fixtures/🦠️<kind>.json` (16 / 18 / 13)

Ticket-folder tooling:
- `🐍️semio-pattern-a-vectors.py`, `🐍️semio-pattern-a-decoder-check.py`,
  `🦀️semio-pattern-a-serde-shape-probe.rs`
