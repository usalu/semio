# XML Owned Snapshot I/O

## Executed evidence

| Registered gate | Result | Evidence |
| --- | --- | --- |
| Native original eleven-law baseline | 7 pass / 4 genuine fail | `f0ca3a1d-c562-4a2e-8b16-df091f8a03da`, missing output, declaration policy and interior copies |
| Native guarded-copy baseline | 10 pass / 2 genuine fail | missing whole native input/output; sixteen copy cases and deep retirement passed |
| Native literal whole-owner implementation | 13 / 13 pass | `bf8a8a90-b4ec-4e59-a192-5eab4957babc`, 0.463s assertions / 1m28 uncached Nx |
| Strengthened native intermediate-state policy baseline | 13 pass / 2 genuine fail | `776e20c3-a376-4c48-bf1a-8e4412561f9c`, wire restrictions and missing named-valid root rejection |
| Native intermediate-state and valid implementation | 15 / 15 pass | `e8b6ab4b-d7af-4b30-aaee-ae1c451afd0c`, 0.230s assertions / 48.8s uncached Nx |
| Full XML owner quick regression | 85 / 85 selected pass, one existing ignored | `8c5b06da-08dc-4a76-a372-810c0b24a6ee`, 0.544s assertions / 1m50 uncached Nx, `xml-full-owner-current-quick.log` |
| Source intermediate-state original baseline | 6 pass / 1 genuine fail | actual-schema Ajv admission followed by miscellaneous-node refusal |
| Source intermediate-state first repair | 6 pass / 1 genuine fail | second reconstruction doctype-placement restriction |
| Source corrected projection and reconstruction | 7 / 7 pass, 49 assertions | 1.394s Bun / 25.2s uncached Nx, `xml-owned-intermediate-source-policy-current.log` |

Logs are retained under `🗑️generated/xml-*`. The sixteenth actual-declaration wildcard/valid export-import law was authored after the fifteen-test compilation; the subsequent full owner quick run includes the refreshed test binary. It preserves the intermediate state through both native-encoding metadata choices, rejects named-valid export, and changes the SQLite subset metadata to prove named-valid import rejects the invalid document. Metadata column index three is the literal subset column in the declared `semio_snapshot` SQL.

## Actual implementation

The thirteen handwritten XML entity and relationship tables preserve every owned node variant, ordered duplicate attributes, child relations, root/prolog/epilog membership, declaration optionality and quotes, typed DTD external identifiers and entity declarations. SQL wildcard boundaries preserve schema-admitted intermediate state, including non-element roots, non-miscellaneous prolog/epilog and independent nonnegative doctype placement. Ordinary XML file parsing and writing keep their distinct wire rules.

Every native relational string projection and reconstruction uses bounded UTF-8 chunk copying under cumulative caller admission. Explicit forest, node-list and document guards retire partial trees iteratively. Tests cover sixteen long-field phase cases, depth 8192 successful exact native re-emission and late-schema/external-identifier cancellation on a 256 KiB stack.

The actual literal snapshot Binary/Text reader and writer preserve the existing owned node/attribute/DTD/declaration vocabulary. The reader uses exact borrowed envelopes, byte-stage scanning, pre-allocation ownership admission and guarded iterative frames. The writer measures borrowed fields with row and file bounds, admits one exact final buffer, then emits literal fields in bounded chunks with known progress. Neither calls an ordinary encoder or printer. The snapshot lifecycle hook uses its actual iterative document retirement.

Named `valid` runs its common owner checker: required element root, miscellaneous boundaries, placement consistency, declaration version/encoding-name grammar, root/doctype name agreement and retained standalone/external warnings. This remains limited DTD validation; full DTD content models are not represented and that limitation remains an explicit diagnostic. It is not full XML-standard verification.

## Independent authority

Neutral fixtures are admitted by the actual snapshot JSON schema through Ajv and queried through Bun SQLite. Malformed-file tests disable SQLite CHECK enforcement before editing invalid enums/negative positions, so importer rejection is tested independently of edit rejection. Wildcard intermediate state is proved separately from meaningful named-subset rejection.

## Remaining work

Full native usize placement width exceeds the current relational i64 and TypeScript safe-integer domains. Source asynchronous mixed UTF-16 yield and intrinsic byte-copy controls remain separate unfinished work. Office needs an explicitly owned multi-document XML graph, without inferred column remapping or whole XML/ZIP carriers. The full owner quick regression executed after refreshing the test binary; all 85 selected laws passed.

## Owned files

- XML snapshot root module, `🪶️sqlite/🦀️.rs`, `🪶️sqlite/🟦️.ts`, `🛫️native/🦀️.rs` and `🛬️native/🦀️.rs`.
- XML snapshot `🧪️tests/🪶️sqlite/🦀️.rs` and `🟦️.ts`, ownership fixture/schema and literal intermediate fixture.
- XML owner mutation-support ordinary logical snapshot decoding: removal of wire-only boundaries, preserving actual XML file helpers.
- XML named-valid checker module: common ordinary/controlled semantic diagnostics and bounded traversal.
- Shared SQLite artifact module: explicit pre-admitted projection helper and genuine reconstruction chunk copying, with neutral reconstruction fixture/schema/tests.
