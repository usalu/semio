# IFC2x3 Own Details Peer Review

Read-only native/TS source audit of base/🚪️io/🪟️details and base editor contextual hook, with one actual-source targeted TS reproduction. No production edits or native compile/execution.

## Proven SET Replacement Defect

IFCUNITASSIGNMENT.Units is SET. Source instance1 with Units=[#2,#3], both targets IFCSIUNIT, accepts reference replacement at document/instances/0/entities/0/arguments/0/values/0/reference with text3. Actual TS ifc2x3ReferenceMutation emits list[#3,#3]. Existing uniqueness scan runs only appendReference. Native normalized_reference_edit has same append-only duplicate check, so its corresponding replacement is also unguarded by source inspection. Reject duplicates in enclosing SET/UNIQUE collection excluding the edited slot, including nested collections; replacing a slot with its own current reference remains legal. Sent directly to High. Own generated modules for repro were cleaned.

## Native Type and Authority Seams

Provider imports IFC2x3 owned model snapshot/catalog and IFC2x3 mutations. Part21 values remain exact native decimal/list/typed/ref shapes. No IFC4 snapshot conversion appears. Provider catalog aliases match its own model nominal types. Native Set ref and type switch produce existing UpsertInstance with preserved instance.id,indexNone; contextual editor hook uses original caller tick/fuel/cancel, then checked declared row budget and publication-byte checks with final guards. One UpsertInstance stays within default2declaredrows for point-invertible leaf. No obvious new Rust lifetime/import/type mismatch found in displayed provider/hook source.

Target validation examines every constituent of a found instance, correctly supporting complex target types. IDs parse exact positive u64 decimal, no float conversion. Type switching preserves compatible exact values and instance identity, uses direct constituent attributes for complex instances and inherited serialization attributes for simple ones; abstract standalone types reject. Cross-constituent derived normalization uses declaredBy/name keys. Native and TS broad behavior is aligned in these seams.

## Bounded Control Gate

Native prewalk ticks each original instance node and string/decimal chunks before instance cloning/subtree materialization. This is a one-shot bounded refusal path, not a retained resumable copy cursor or owned-close proof. Template reference search takes at most256instances but inner constituent scan and recursive template construction do not receive original callback; an instance can contain many constituents. The generated type planner uses global ID scanning with ticks, while scalar subtree edit still materializes exact whole instance through to_value/edited_subtree/from_value. Keep these limits explicit; do not claim arbitrary artifact-sized smooth cancellation/resumption from node prewalk alone. High was notified.

## Independent Evidence Scope

Current TS tests enumerate653catalog entities and use own source catalog as expected attribute oracle, so this proves internal slot consistency, not independent normative declaration validation. Separate catalog IfcOpenShell metadata audit supplies independent declaration-name/attribute/predefined facts. AJV independently executes selected enum/coordinate-count domains from neutral fixtures. Base64-js independently checks shared compressed identity encoding from known128bit vectors. Decimal preservation checks compare original exact own representation; no external decimal library execution is claimed here. Callback-stop tests exercise actual TS planner but do not execute native retained factory/Store publication/UI. Native syntax checks remain distinct from typechecking.

No other accepted-invalid mutation was reproduced during this focused audit.
