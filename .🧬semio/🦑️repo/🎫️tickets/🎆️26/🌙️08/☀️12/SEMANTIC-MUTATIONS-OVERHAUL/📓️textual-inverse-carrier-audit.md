# Independent Textual Mutation Inverse and Codec Audit

## Scope and Verdict

Read-only source audit of exactly the converted JSON RFC 8259 Any (5), XML 1.0 Any (6), SVG 1.1 Any (9), and TXT UTF-8 Any (5) direct mutation owners. All **25/25** publicly expose `Apply(TypedPayload) | Restore(AggregateDiff)`; all 25 return the supplied aggregate diff without limiting it to the named semantic operation. All 25 production inverse functions return that same owner's `Restore` carrier.

These roots are **not semantically closed direct-leaf implementations**, despite their earlier structural 0/17 results. No source was modified, no Cargo/rustc/Nx runtime was started, and `compose/**` was not inspected. This audit does not assert coverage of other mutation families. PDF 1.4 remains frozen; the coordinator's approved base naming is `resize-page` / `replace-page-text`, preserving the actual 1.4 `PageDoc` domain.

Exact roots:

- JSON: [✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations)
- XML: [✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations)
- SVG: [✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations)
- TXT: [✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations)

## Verified Counts

| Family | Concrete owners | Aggregate carrier type | Public Restore phases | Payload-only leaf codec files | Root whole-enum codec bypasses | Leaf tests |
| --- | ---: | --- | ---: | ---: | ---: | ---: |
| JSON | 5 | `JsonDiff` | 5 | 10 | 2 | 5 identity-only |
| XML | 6 | `XmlDiff` | 6 | 12 | 2 | 6 identity-only |
| SVG | 9 | `SvgDiff` | 9 | 18 | 2 | 9 identity-only |
| TXT | 5 | `TxtDiff` | 5 | 10 | 2 | 5 identity-only |
| Total | 25 | 4 aggregate types | 25 | 50 | 8 | 25 identity-only |

All 25 descriptors claim `invertibility: explicit-mutation`, `diffParticipation: detect`, `composition: atomic`, and all seven required surfaces. There are zero `Undo` tokens in the 25 direct owner files; the hidden operation name is uniformly `Restore`. Four additional root tests assert aggregate roster length only. Each direct text/binary facet takes `&<Kind>Payload`, not `&<Kind>Mutation`, so its function signature cannot encode a returned `Restore`.

## Findings

### 1. Critical: Per-leaf Arbitrary Aggregate-Diff Escape Hatches

Every owner has the equivalent of:

```rust
pub enum SetMemberMutation { Apply(SetMemberPayload), Restore(JsonDiff) }
Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone())
vec![JsonMutation::SetMember(Self::Restore(inverse))]
```

The semantic label, target and descriptor do not constrain Restore. These are public serde-deserializable constructors, not inaccessible undo bookkeeping. A `SetMember::Restore` can replace the entire JSON value; XML/SVG `SetText::Restore` can modify declaration, doctype, prolog or root; TXT `SetLineEnding::Restore` can edit arbitrary lines and trailing-newline state. Diff apply validation only validates structural applicability, not the named operation's footprint.

Concrete evidence:

- [set-member](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-member/🦀️component.rs:23) exposes `JsonDiff`; lines 37/45 bypass behavior and construct inverse.
- [JsonDiff](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:122) owns root `value`; `JsonValueDiff::Replace` at line 96 accepts any `JsonValue`.
- [XmlDiff](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:34) and [SvgDiff](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:31) own whole-document fields and root replacement.
- [TxtDiff](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:232) owns line-ending, trailing-newline and arbitrary line triples. Its apply path (245–257) does not call `non_canonical_shape`; any Restore can bypass the leaf's canonical-text invariant.

Do not rename this carrier to Undo, wrap it in another struct, add a generic restore-diff leaf, or serialize it as an opaque payload. Remove the aggregate-diff phase and return an explicit sequence of concrete semantic mutations.

### 2. Critical: Root Wire Codecs Bypass All 50 Leaf Codecs

All four root text codecs serialize `self` directly with `serde_json::to_vec`, hex-encode it behind one `<format>-mutation payload=` prefix, and deserialize the entire enum. All four binary codecs directly serialize/deserialize the enum as JSON bytes. None calls its direct leaf `encode_payload`/`decode_payload`. The opcode/tag arrays are inert documentation/rosters, not executable dispatch.

Representative [JSON text codec](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs:9) and [JSON binary codec](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:6); identical structure exists at the other three exact roots.

Therefore a root Rust roundtrip can appear successful while serializing the forbidden Restore carrier through a path absent from every direct payload schema. Root binary tags 1–N are never emitted as binary tags. The root grammar/protocol merely names the semantic identities in comments and describes opaque payload framing.

Correction: root codecs may own framing and an executable typed registry, but each registry entry must call the concrete leaf codec and wrap/unwrap its own direct mutation. Root `OpText`/`OpBinary` must not use whole-enum serde as an alternate route.

### 3. High: The Declared Forward Shapes Already Disagree

For all 25 operations:

- Rust aggregate serde uses camelCase mutation identities (e.g. `setMember`) and leaf `{phase:"apply", value: ...}`.
- TypeScript uses kebab-case identities (e.g. `set-member`) with that same Apply wrapper, and has no Restore alternative.
- Root JSON Schema uses kebab-case identities with the **bare direct payload**, no phase/value wrapper.
- Protobuf oneofs carry only the direct payload messages; there is no Restore identity or typed restore data.
- GraphQL root accepts an opaque `MutationPayload` scalar instead of composing its typed leaf payload inputs.

Read-only Ajv 8.20.0 counterexample check executed successfully: **25/25 bare-schema controls accepted; 0/25 declared TypeScript Apply shapes accepted; 0/25 inferred Rust-serde Apply shapes accepted; 0/25 inferred Rust-serde Restore shapes accepted**. This is 100 schema checks, not a Rust runtime proof. Rust spellings/shapes were inferred directly from the checked serde attributes. Exact inputs/results and commands are in the companion evidence files.

Additional payload closure gaps worth fixing in the same schema-first repair: XML/SVG `set-attribute.value` allows `None/null` in Rust/TS but JSON Schema permits only string if present; SVG `set-transform.transform` allows `None/null` in Rust/TS but JSON Schema permits only array if present. Optional proto3 plain `string value` cannot distinguish absent attribute from empty attribute without explicit presence. Raw `bytes` payload fields need a specified typed representation, not an undeclared opaque encoding.

### 4. High: XML/SVG Restore Serialization Is Lossy for Clears

`XmlDiff` and `SvgDiff` declare `Option<Option<XmlDeclaration>>` and `Option<Option<XmlDoctype>>` with ordinary serde derive and only default/skip-if-none attributes. The newly substituted root JSON-serde codec does not use their existing handcrafted tri-state diff codecs. A production inverse of setting previously absent declaration/doctype carries `Some(None)`; ordinary JSON serialization spells this `null`, while deserializing `Option<Option<T>>` yields outer `None` (unchanged). Thus encode/decode can erase the clear intent.

This is a source-level determinism finding, **not executed Rust runtime evidence in this audit**. Add the absent → set → production inverse → text/binary roundtrip → apply test before claiming runtime closure. Removing Restore and expressing the clear as the concrete setter's nullable payload avoids this aggregate tri-state transport defect.

### 5. High: Current Feature Inverse Tests Bypass Production Inverses

The four subject adapters do not call the production `Mutation::inverse` implementation:

| Family | Exact adapter location | Actual inverse route | Current feature mutation/inverse rows |
| --- | --- | --- | ---: |
| JSON | [subject::inverse](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧪️tests/mutate-json-rfc8259/🦀️component.rs:259) | Separate `inverse_spec` → `mutation_from_spec` | 5 + 5 |
| XML | [inverse_of](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧪️tests/mutate-xml-1-0/🦀️component.rs:309) | Handwritten concrete match → one mutation | 6 + 6 |
| SVG | [inverse_of](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1/🦀️component.rs:240) | Handwritten concrete match → one mutation | 9 + 9 |
| TXT | [subject::inverse](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🧪️tests/mutate-txt-utf-8/🦀️component.rs:218) | `oracle_inverse_spec` → `mutation_from_spec` | 5 + 5 |

Those 50 feature rows validate forward behavior plus the adapter's/oracle's chosen inverse, not the returned production inverse. No `print_op`, `parse_op`, `encode_op`, or `decode_op` call exists in these four adapter sources. Every one of the 25 direct owner tests asserts only `SEMANTICS.kind`; none exercises diff, inverse, or codecs.

The XML/SVG adapter fallback arm substitutes `SetDeclaration(base.declaration)` for unmatched phases; SVG also substitutes it for several missing targets. JSON's independent inverse helper still contains `no-mutation`/`set-snapshot` branches, although those are no longer concrete root identities. These test shims must not become the production repair.

Existing differential fixtures compare semantic projections that intentionally ignore JSON member order and XML/SVG attribute order; they cannot prove exact restoration of the ordered snapshot vectors. TXT's feature explicitly records no independent third-party line-structure producer; do not relabel that debt as independent semantic validation.

## Typed Inverse Remediation Plan

Return `Vec<RootMutation>` containing only direct typed operations. Compute the plan from the pre-mutation snapshot, validate the forward operation first, and return no inverse for a no-op/refusal. Data below must be captured before applying. Keep all semantic choices in their leaf owners.

### JSON: Five Current Owners

| Forward owner | Preimage needed | Exact concrete inverse |
| --- | --- | --- |
| set-member | Object path, member presence, existing value; occurrence/order when duplicate keys exist | Existing member: set-member(old value); new member: remove-member at the affected occurrence |
| remove-member | Object path, original ordered index, key and full original JsonValue | **insert-member(path,index,key,value)**; direct ordered insertion capability is missing |
| insert-array-element | Resolved array length and actual insertion index `min(requested,len)` | remove-array-element(path,actual index) |
| remove-array-element | Path, original index, full element JsonValue | insert-array-element(path,index,old value) |
| set-scalar | Path and full previous JsonValue | Current implementation accepts any JsonValue, not just scalars. Model **replace-value(path,value)** explicitly (rename the current operation if that is its intended domain), or constrain scalar-to-scalar and add the real subtree replacement operation separately. Inverse uses the previous value with the same explicit operation. |

Do not use whole-document diff as an inverse shortcut. Ordered insert-member is the clean missing primitive; a longer delete/reappend-suffix plan using current members could restore order for unique keys, but must not be hidden in an arbitrary diff or silently claimed equivalent under an order-insensitive oracle.

JSON Any currently preserves duplicate keys on parse ([parse_object](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:219)), while set/remove-member and object diff use first matching key. The generic inverse's `object_diff_between` ([object_diff_between](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:426)) cannot reconstruct a removed duplicate occurrence. Include an ordered duplicate-key fixture and make occurrence/index targeting explicit rather than assuming unique keys. This is an additional actual domain requirement discovered in the bounded inverse trace.

### XML: Six Current Owners

| Forward owner | Preimage needed | Exact concrete inverse |
| --- | --- | --- |
| set-declaration | Previous Option<XmlDeclaration> | set-declaration(previous option) |
| set-doctype | Previous Option<XmlDoctype> | set-doctype(previous option) |
| insert-element | Parent path and actual accepted/clamped insertion index | remove-element(parent,actual index) |
| remove-element | Parent path, original index, complete XmlNode subtree | insert-element(parent,index,old node) |
| set-attribute | Path, name, prior presence, old string, **original attribute index** | Update: set-attribute(old string); new: set-attribute(None); removed: **insert-attribute(path,index,name,value)** |
| set-text | Path and old text of the existing Text node | set-text(path,old text); reject wrong node kinds before producing inverse |

A direct ordered `insert-attribute` operation is the missing clean primitive (or an explicitly modeled equivalent position-capable attribute operation). Existing nullable set-attribute appends absent attributes, so it cannot alone restore the original vector order after deletion. `insert-element` actually accepts any XmlNode, including Text/CDATA/Comment/PI; document/rename that domain as insert-node/remove-node, or narrow the operation deliberately. A root/prolog replacement operation is **not required** to invert the current six legal operations; remove that hidden capability rather than recreating Restore under another name.

### SVG: Nine Current Owners

The XML declaration, doctype, insertion/removal, attribute and text plans apply with `NodePath` and `parent` field names. Additional plans:

| Forward owner | Preimage needed | Exact concrete inverse |
| --- | --- | --- |
| set-element-name | Path and previous element name | set-element-name(path,old name) |
| set-view-box | Path, raw prior `viewBox` string, presence and attribute index | set-attribute of the **raw original string**, remove if previously absent, or ordered insert-attribute after deletion |
| set-transform | Path, raw prior `transform` string, presence and attribute index | Same typed attribute plan using original string, not parse/reprint |

Do not reconstruct old viewBox/transform by parsing typed numeric data then printing it: that normalizes lexical forms, can lose malformed-but-preserved attributes, and does not preserve attribute order. The current handwritten SVG test inverse does parse/reprint, so it is insufficient as the implementation template. Add one direct ordered insert-attribute primitive to SVG as well, with all descriptor/wire/schema surfaces.

### TXT: Five Current Owners

| Forward owner | Preimage needed | Exact concrete inverse |
| --- | --- | --- |
| set-trailing-newline | Prior boolean | set-trailing-newline(old boolean), preserving canonical-shape checks |
| set-line-ending | Prior LineEnding | set-line-ending(old ending) |
| insert-line | Original length and actual accepted/clamped insertion index | remove-line(actual index) |
| remove-line | Original index and exact old string | insert-line(index,old string) |
| set-line | Original index and exact old string | set-line(index,old string) |

No new TXT semantic leaf is needed for inversion of legal operations. Existing five typed payloads are sufficient. Use actual normalized indices: insert-line currently computes `min(index,len)` for its shape check but writes the raw requested index into the diff. Invalid raw additions can reach `TxtDiff::inverse`'s `apply(base).unwrap()` at line 320. Inverse planning must not panic on a refused/invalid forward operation. Add invalid-index, empty-document, final-empty-line and trailing-terminator cases.

## Required Regression Evidence Before Acceptance

1. Language-neutral literal cases per owner: base, typed mutation, expected exact snapshot, expected **concrete inverse plan**, no-op/refusal, text frame and binary bytes. Every inverse element must have a descriptor identity and no aggregate diff/snapshot carrier.
2. Subject tests call the real production inverse; independently compute expected results with the existing third-party oracle where available. Keep oracle-only inverse reasoning independent, but never substitute it for the subject's implementation.
3. For every successful mutation: apply forward; generate inverse from base; roundtrip every inverse through leaf text/binary and root text/binary; validate emitted payload against the direct JSON Schema; apply decoded inverse sequence; compare exact snapshot. Test every intermediate state for multi-step plans.
4. Explicit cases: absent declaration/doctype clears; removed attribute/member in the middle; duplicate JSON keys; nested subtree removal; append/clamped indices; SVG raw lexical viewBox/transform; TXT canonical shape/refusal.
5. Snapshot projection parity and exact state/order equality are separate assertions. Existing semantic projections cannot replace the latter.
6. A root wire registry test must instrument or otherwise prove leaf codec invocation; a static roster constant beside whole-enum serde is not delegation.
7. Re-run the coordinator's policy gate with the new AST rule, then uncached targeted Nx runtime only after the shared Cargo freeze is released. No runtime pass is claimed here.

## Cross-family Detector Recommendation

The coordinator is adding an AST-backed rule. This audit independently supports these requirements:

- Resolve root aggregate `#[mutations(diff = AggregateDiff, snapshot = Snapshot)]` types and wrapped leaf types. Inspect payload/enum fields transitively, including aliases, `Box`, `Option`, structs and collection wrappers, not only the spelling Restore.
- Flag **any** concrete mutation owner that carries the root aggregate Diff/Snapshot, or an opaque bytes/JSON payload decoded into those types, in a purported semantic operation. Names such as Restore/Undo/Replay/Patch/ApplyDiff are evidence but not the rule's sole trigger.
- Flag behavior that directly returns a supplied aggregate diff or applies a supplied snapshot without constructing an operation-local semantic change. Local scalar/value/subtree types are not automatically forbidden; validate their declared operation's scope.
- Do not flag ordinary leaf implementation imports/return types of AggregateDiff: leaves legitimately construct diffs. The forbidden capability is an unconstrained input/carrier, including generated/deserialized variants, not a diff calculation.
- Verify inverse constructors resolve to registered direct semantic variants and codecs cover every reachable variant. Reject a descriptor that says explicit-mutation while its inverse payload is opaque aggregate state.
- Flag whole-enum serde root codec paths that bypass required leaf codec registries. Resolve executable leaf calls, not comments or unused opcode/tag constants.

glTF was **not inspected** in this bounded audit. The parent/pilot's analogous-family report warrants applying the same detector there, but these 25 findings are the only census claimed here.

## Exact Owner Evidence

Each row references its direct Rust owner. `E/D/I` are the enum declaration / unrestricted Restore diff arm / inverse Restore constructor line numbers.

| Family | Owner | Carrier | E/D/I |
| --- | --- | --- | --- |
| json | [✏️set-member](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-member/🦀️component.rs:23) | `JsonDiff` | 23/37/45 |
| json | [🗑️remove-member](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-member/🦀️component.rs:22) | `JsonDiff` | 22/30/38 |
| json | [📥️insert-array-element](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-array-element/🦀️component.rs:23) | `JsonDiff` | 23/31/39 |
| json | [✏️set-scalar](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-scalar/🦀️component.rs:22) | `JsonDiff` | 22/30/38 |
| json | [🗑️remove-array-element](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-array-element/🦀️component.rs:22) | `JsonDiff` | 22/30/38 |
| xml | [✏️set-doctype](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-doctype/🦀️component.rs:21) | `XmlDiff` | 21/29/37 |
| xml | [📥️insert-element](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-element/🦀️component.rs:23) | `XmlDiff` | 23/31/39 |
| xml | [✏️set-attribute](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-attribute/🦀️component.rs:23) | `XmlDiff` | 23/31/39 |
| xml | [✏️set-text](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-text/🦀️component.rs:22) | `XmlDiff` | 22/30/38 |
| xml | [🗑️remove-element](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-element/🦀️component.rs:22) | `XmlDiff` | 22/30/38 |
| xml | [✏️set-declaration](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-declaration/🦀️component.rs:21) | `XmlDiff` | 21/29/37 |
| svg | [✏️set-doctype](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-doctype/🦀️component.rs:22) | `SvgDiff` | 22/30/38 |
| svg | [✏️set-view-box](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-view-box/🦀️component.rs:23) | `SvgDiff` | 23/31/39 |
| svg | [📥️insert-element](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-element/🦀️component.rs:24) | `SvgDiff` | 24/32/40 |
| svg | [✏️set-attribute](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-attribute/🦀️component.rs:24) | `SvgDiff` | 24/32/40 |
| svg | [✏️set-text](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-text/🦀️component.rs:23) | `SvgDiff` | 23/31/39 |
| svg | [🗑️remove-element](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-element/🦀️component.rs:23) | `SvgDiff` | 23/31/39 |
| svg | [✏️set-element-name](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-element-name/🦀️component.rs:23) | `SvgDiff` | 23/31/39 |
| svg | [✏️set-transform](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-transform/🦀️component.rs:23) | `SvgDiff` | 23/31/39 |
| svg | [✏️set-declaration](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-declaration/🦀️component.rs:22) | `SvgDiff` | 22/30/38 |
| txt | [📥️insert-line](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-line/🦀️component.rs:22) | `TxtDiff` | 22/30/38 |
| txt | [✏️set-line](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line/🦀️component.rs:22) | `TxtDiff` | 22/30/38 |
| txt | [✏️set-trailing-newline](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-trailing-newline/🦀️component.rs:21) | `TxtDiff` | 21/29/37 |
| txt | [✏️set-line-ending](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line-ending/🦀️component.rs:21) | `TxtDiff` | 21/29/37 |
| txt | [🗑️remove-line](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/🦀️component.rs:21) | `TxtDiff` | 21/29/37 |

Companion files: [machine-readable census and Ajv results](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🔣️textual-inverse-carrier-audit.json) and [exact audit commands](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️textual-inverse-carrier-audit-commands.md). The source SHA-256 per direct owner records the audited checkpoint; concurrent changes after it require revalidation. Confidence is high for the source-level carrier/codec/test findings, with runtime deliberately unexecuted.

