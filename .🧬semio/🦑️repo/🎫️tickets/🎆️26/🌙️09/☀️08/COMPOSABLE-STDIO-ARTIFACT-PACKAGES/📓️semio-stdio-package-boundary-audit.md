# Semio Native Package-Boundary Audit

## Scope

Read-only audit of [stdio-representative-native-after-disk.txt](🗑️generated/stdio-representative-native-after-disk.txt).

The Semio default library run completed with **2,306 passed, 196 failed, and 1 ignored** in 395.56 seconds (receipt lines 3386–3388). It compiled and executed Semio. No missing crate, unresolved import, source-path, or dependency diagnostic appears in the receipt. Binary and TXT executor gates remain unrun after this failed representative gate.

The receipt prints 191 named FAILED notifications: 89 Kit, 55 Object, 39 BREP, 2 Document, 2 Value, 1 Drawing, 1 Presentation, and 2 root Semio tests. The 196 test-harness total is authoritative; do not infer that the named grouping completely partitions the failures.

## Ticket-Owned Regressions

### Composite snapshot decoders contradict their published schemas

The dominant family is **89 Kit** and **55 Object** failed notifications. The receipt contains 100 instances of missing field properties and 44 of missing field brep. First Kit evidence: add-design at receipt lines 1521–1523. First Object evidence: create-brep reports its fixture source line 23.

The assets are schema-valid:

- [Object schema](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🔣️.json) lines 7–10 requires only schema and transform. Its brep, mesh, and properties slots are optional.
- [Kit schema](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🔣️.json) lines 7–9 requires only schema. Types, designs, child slots, and link slots are optional.
- Object create-brep before has only schema and transform; after adds only brep. Kit add-design before and after contain objects, models, and representations but intentionally omit properties.

The manual decoders disagree:

- [Object snapshot](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🦀️.rs) lines 67–79 requires brep, mesh, and properties.
- [Kit snapshot](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs) lines 128–142 requires types, designs, objects, models, properties, and representations.

**Repair:** make FromValue honor the schemas and snapshot Default: absent Object child slots decode as None; absent Kit vectors decode empty and properties decodes None. Preserve errors for present malformed values. This matches the value derive's documented behavior: a missing Option decodes as None, while non-Option collections need an explicit default. Add schema-minimal Object and Kit codec laws. Do not make valid fixtures verbose or alter the schemas to claim optional slots are mandatory.

### Drawing binary decoder reconstructs an invalid keyword

Receipt lines 950–952 show the Drawing binary round-trip failing at rotateNode:

    decode_op failed: malformed op text at offset 2: drawing mutation: unknown keyword "rotateNode"

[Drawing binary codec](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/💾️binary/🦀️.rs) lines 19–37 stores camel-case OP_KEYWORDS; lines 89–93 rebuild text and call parse_op. The parser's canonical [KINDS](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🦀️.rs) lines 67–85 is kebab-case, including rotate-node.

**Repair:** derive the binary decode keyword from canonical kebab-case KINDS, or one shared canonical list. Leave the ordinal binary tag unchanged and reconstruct only valid parser text.

### Presentation test confuses semantic catalog identity with op-text vocabulary

Receipt lines 2388–2392 fail because the oracle has set-text-box-blocks while KINDS uses set-textbox-blocks.

This is a stale test invariant:

- [The leaf](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/✍️set-text-box-blocks/🦀️.rs) lines 5–12 expressly distinguishes semantic set-text-box-blocks from established op-text set-textbox-blocks.
- The descriptor and oracle use semantic spelling; parser, KINDS, text schema, and grammar use the text spelling.
- [The unit test](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs) lines 8–29 incorrectly requires exact equality.

**Repair:** split the assertion into a print_op/parse_op/KINDS text law and an oracle-to-semantic-descriptor law. Preserve the documented two vocabularies; do not rename the text protocol or add an alias.

## Separate Findings

| Group | Notices | First diagnostic | Assignment |
| --- | ---: | --- | --- |
| BREP | 39 | inverse tolerance drift at BREP unit line 36; boolean/body validation errors | BREP semantics and fixtures |
| Document | 2 | invalid-remove-key: img1 absent or duplicated at unit line 189 | Document logic or fixture |
| Value | 2 | absorb_map_associativity and absorb_nodes_associativity | Value diff algebra |
| Root member | 1 | ArtifactStore Drop lacks terminal-empty witness, receipt 3154–3156 | retained-ownership protocol; the same test and diagnostic were recorded before this ticket in END-TO-END-TESTING-REFACTOR/📓️w19-three-crosscutting-defects.md lines 94–100 |
| Root retirement | 1 | shared value retired before terminal-empty, receipt 3158–3160 | test cleanup; [root unit test](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🔬️unit/🦀️.rs) lines 37–42 leaves a known nonterminal retirement to drop |

The last two are not package-path or extraction failures. The root retirement test needs its owning lane to drive the state to Complete after proving a zero grant is resumable.

## Retry Order

1. Repair and directly exercise schema-minimal Object and Kit snapshot decoding.
2. Repair Drawing binary keyword reconstruction and run its one binary round-trip law.
3. Split the Presentation semantic-catalog and text-vocabulary test law.
4. Rerun the Semio default representative suite.
5. Run Binary and TXT executor gates only once Semio accepts. Route BREP, Document, Value, and retained ownership to their owning lanes.

No implementation or native command was run for this audit.
