# JSON Original Recursive Receiving Map

Read-only implementation map; no builds/production changes. Domain base `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/`. Original JsonSnapshot fields schema/value; original JsonValue6variants Null,Bool,Number{lexeme},String{value},Array{items},Object{members}. JsonMember has key/value; members are insertion-ordered Vec, duplicate keys legal original structure, not a BTreeMap or serde_json::Value.

## Exact Four-Table Semantics

Current DDL root document(id,schema,root_value_id); value(id,kind,boolean_value,number_lexeme,string_value,number_value); object_member(id,object_value_id,ordinal,key,value_id); array_element(id,array_value_id,ordinal,value_id). Null/Array/Object inactive scalar columns must be null; Bool only0/1; Number exact lexeme authority and independently derived query numeric REAL or Null; String literal text. number_value is a derived numeric query view, not an independently persisted original IEEE f64 field. Do not round-trip original lexeme through binary64 or add unrelated bit/class authority.

Object member parent edges retain duplicates and order; empty Array/Object always have their own tagged value row even with no edge rows, distinct from Null. Document schema is arbitrary original literal. Exact signed/unsigned limits apply to surrogate IDs/ordinals, not lexeme precision. Original number meaning helper should remain the shared lexical/numeric oracle; reject contradictory derived numeric cell without replacing lexeme.

## Retained Projection

Projection frame owns actual SqliteDatabase/table/row/cell backing plus retained iterative work descriptors using stable source paths/indices, not unretirable borrowed-reference heap pointers. Native receive requires RetireOwned+'static intermediate; a Vec<&JsonValue> frame with borrowed source lifetime is not a legal erased recipient. Prefer owned integer source path stack into borrowed source, or an allocation-free external traversal iterator with all allocated state inside retained frame. Descriptors carry original parent kind/root/memberordinal/key-location/value index without cloning Data Strings. Birth frame before original paid index vector/table/schema allocation; reserve/growth uses Port and retained replacement vectors.

Iterative borrowed census must honor both original and SQL callbacks and monotonic item/row/grant bounds; current cold measure starts plain local vec![&value], so cannot be reused as a whole receiving body. Projection installs each Text/String key/lexeme empty destination before Port.text; Number lexeme is copied once into actual owned cell, derived numeric computed while borrowed. All children/edges created under original paid row/value headers. Both original string and number/key-copy cancellation preserve exact actual prefix/cause.

## Retained Reconstruction

Frame owns empty typed JsonSnapshot, value-row indices/IDs, member/element indices, parent/child/ordinal records, ownership/visited state and postorder Visit/Finish descriptors. Use paid sortable Vecs and binary-search IDs instead of unmetered BTreeMap/BTreeSet. Construct each actual JsonValue candidate in installed retained slots before String/Vec/child birth. Array children and Object JsonMembers move into paid preallocated destination in original order; duplicate key Strings must not coalesce. Frame must retain not-yet-attached subtrees on refusal and close them iteratively under original grant; static retirement depth should reflect genuine tree custody.

Validate actual declarations,width,positive rowid=ID,one root,exact active scalar cells,known tag/parent type,known child,one owner per value,root has no parent,contiguous unique ordinals per parent,no cycles/disconnected/unconsumed values or edges. Duplicate object keys are permitted; duplicate ordinal/identity or multiple child owners are not. Own tree index scans and sorting must checkpoint original control before each expensive bounded block; empty typed variants preserve marker identity.

Current cold reconstruct does explicit active-cell checks, multiple-owner/cycle and postorder connectivity, but holds local stacks/Vec/BTreeMap/BTreeSet/strings. Reuse those semantic predicates, not their unretained allocation lifetime. Shared workspace defaults route original allocation only; explicit four receiving P/R+directional hooks are the next genuine typed body task. Both validators preserve JSON base/GeoJSON/i-json actual dialect policy without narrowing duplicate/source lexeme legal scope.

## Neutral and Native Law Trigger

Handcraft actual input schema and independently authored rows for all6tags; duplicate keys with different values; nested empty arrays/objects/Null; lexical0/-0/1e+000/hugeinteger/highprecision, nonnumeric query overflow ->Null, UTF8/NUL/schema/key/text; root/parent/ordinal/type/cycle/multipleowner/disconnected malformed cases. Bun SQLite json functions are not an oracle for duplicate/order/lexeme preservation; query exact row/key/ordinal/lexeme and use a third-party event/token parser or explicit independent lexical predicate witness, not serde_json object collapse.

Native tests invoke explicit original decode projection and encode reconstruction, canceled real schema/key/lexeme/text/indexprefix, every currency/frame/publication refusal, exact copieditems/bytes/capacity/release and causepointer promotion, zero/canceledfundedclose then same observer rearm. Typed fullfile API compares all original semantic fields; raw JSON serializer parse may normalize lexical spelling and must not replace that proof. No passed runtime is asserted.

Source41798 terminal result was not recovered from saved ticket reports during this task; producer guard verification remains Root's execution evidence, not an audit-inferred pass.
