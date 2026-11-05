# Borrowed Ordered Lookup Feasibility

Read-only inspection confirms feasibility without a second AVL or copied query. LookupState::advance uses its owned SharedOwner<String> only in compare_bytes. The rest of the algorithm uses retained base/current roots, offset, left_byte, ordering and complete.

Factor the same body into an internal method accepting an optional borrowed query. The original method supplies its existing owned key; the borrowed method supplies the actual held &str. Borrow the key and mutate disjoint state fields directly, without an extra SharedOwner clone.

Keep actual LookupCursor result/root/cancellation custody. Add scalar query pointer+length identity and a borrowed constructor retaining the same two map roots under the real metadata admission quote. Store no Rust reference or unsafe pointer. Every borrowed demand/advance receives the actual query again and checks pointer+length. The real display owner must retain the original immutable OperatorInfo/catalogue lease; scalar identity does not replace it.

Proposed APIs: OrderedMap::admit_lookup_borrowed_original(&self,query:&str,grant); LookupCursor::next_borrowed_original_demands(&self,query:&str); LookupCursor::advance_borrowed_original(&mut self,query:&str,grant). The latter reuses the actual compare_bytes engine with one selected byte per turn. Existing result and close APIs retain actual roots. Owned-only APIs must refuse borrowed mode instead of unwrapping an absent key.

The constructor can retain the current fixed metadata copy quote: two root handles plus two scalar query words. Borrowed query has no owned key to retire. Native tests should compare original owned and borrowed results under one-byte grants, identity refusal, denied construction/bytes/depth, and canceled root closure with exact heap receipts. Add schema-first SQL ordering cases.

No defining source edits for this inspection; the generation-accessor RED closure remains stable.
