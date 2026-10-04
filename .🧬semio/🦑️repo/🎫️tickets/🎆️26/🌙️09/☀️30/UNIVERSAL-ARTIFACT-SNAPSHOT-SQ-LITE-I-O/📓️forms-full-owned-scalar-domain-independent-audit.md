# Forms Full Owned Scalar Domain Independent Audit

Read-only source audit, 2026-10-03. No production/test changes or runtime probes. Parent reports Forms 12-law green; this audit does not independently authenticate a runtime result.

## Conclusion

The finite widget/vector restrictions are established Forms definition invariants, not newly invented SQL-only guards. They are enforced across validated persistence and mutation publication. Therefore nonfinite widget/vector positions are outside the presently validated native Forms snapshot domain. However the public durable snapshot constructor does not validate its supplied steps, so not every construction boundary authenticates this narrower domain. Claims covering every constructed retained owner state need that boundary gap resolved or an explicit validated-domain qualification. Do not infer unrestricted IEEE acceptance from the Rust type or SQL companion syntax alone.

## Actual Authorities and Line Origins

Paths below are relative to the repository. Let `F` denote `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any`.

- Forms root `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:22` explicitly aliases PlaybookBlock and PlaybookVectorField. The actual shared owner at `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:49,67,70,73,110,117` derives DslRecord/ToValue/FromValue and owns min/max/step and vector value as optional binary64.
- Native scalar binding `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs:143-160` declares Shape::Float and passes the actual f64 through FieldValue::Float; no finiteness restriction is encoded in that scalar shape. Native controlled text encoding at `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encoding/🦀️.rs:296` prints NaNs using raw-bit syntax. This scalar transport authority is broader than the Forms domain validator.
- `F/🧬️schema/📝️definition/🦀️.rs:18` requires finite min/max/step, ordered bounds, and positive step. Line22 requires finite optional vector values. Snapshot validation invokes that definition validation at `F/🧬️schema/📸️snapshot/🦀️.rs:75`; FromValue invokes snapshot validation at line65.
- The mirrored TypeScript definition validator at `F/🧬️schema/📝️definition/🟦️.ts:42-43,52` enforces the same limits. This conclusion does not rest on ordinary JSON numeric constraints.
- Public DSL parsing at `F/🚪️io/📸️snapshot/📝️text/🦀️.rs:36` validates the reconstructed snapshot. Public Pack encoding and decoding at `F/🚪️io/📸️snapshot/💾️binary/🦀️.rs:19,31` validate. Controlled flat-native reconstruction at `F/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:35-37` enforces the same constraints; SQLite reconstruction at `F/🧬️schema/📸️snapshot/🪶️sqlite/📥️reconstruction/🦀️.rs:52` repeats the question boundary.
- SQL at `F/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql:5` permits finite, infinity, and NaN companion classes for widget numbers. This is representational capability, not evidence the typed validated Forms domain accepts them. The scalar triplets still preserve negative zero and every allowed finite binary64 bit pattern.
- Mutation diff validation at `F/🧬️schema/🔺️diff/🦀️.rs:76` validates a supplied definition; apply-to-artifact and apply-to-snapshot call diff validation at lines183 and213. Change-block-field diff refusal at `F/🧬️schema/🧬️mutations/🎛️change-block-field/🔺️diff/🦀️.rs:32-43` rejects nonfinite numeric bounds and nonpositive steps. Its Fields branch checks only key uniqueness, but final FormsDiff application rejects nonfinite vector values through definition validation.

## Construction Boundary Gap

The explicitly public helper `forms_snapshot_with_state` at the Forms root lines361-364 creates a durable FormsSnapshot from arbitrary supplied steps, clones them directly into FormsDefinition, and returns without validation. This is concrete constructor behavior, not merely public field typeability. The shared block FromValue derive is likewise a structural decoder; the enclosing snapshot validator supplies Forms business constraints. The constructor can therefore return an owner that public Pack persistence subsequently rejects. Existing native codecs intentionally require a valid Forms owner, including print_dsl's valid-state expectation.

For universal full retained-owner coverage, either enforce the established definition invariants at this constructor boundary and treat invalid supplied steps as construction failures, or deliberately revise the entire Forms definition domain and all paired boundaries to accept full IEEE widgets. Removing SQL admission checks alone would create asymmetric codecs. No silent fixture narrowing is appropriate.

## Existing Scalar Test Coverage

The allocator callback `F/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:9-23` converts the entire actual and expected snapshot to values and recursively checks all tags, ordered keys, list lengths, scalar literals, and Float.to_bits at line16. It does not isolate only answer/default DslValue positions; widget and vector scalar fields participate in that whole-owner traversal. The fixture at `F/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json:18-20,37,41` uses minimum negative zero, maximum ten, increment one, vector negative zero, and vector 2.5. NaN/raw-bit fixtures reside in intrinsic DslValue variants. This verifies selected finite widget/vector cases and intrinsic nonfinite values, not an exhaustive scalar-position matrix.

Add explicit boundary laws for constructor rejection or documented invalid-owner handling; paired native/SQL refusal for nonfinite widget/vector values; and finite widget/vector bit coverage including subnormals, maximum finite values, absent optionals, and signed zero where business constraints allow. Positive-step semantics exclude zero and negative values only in the increment slot. Existing passing allocator laws must not be described as full IEEE acceptance at every widget/vector position.
