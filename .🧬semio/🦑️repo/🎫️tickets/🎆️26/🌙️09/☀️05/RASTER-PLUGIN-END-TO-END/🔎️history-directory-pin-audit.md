# History Directory Pin Audit

The cursor retains a raw directory-chunk pointer and increments ledger read_count on admission. Its one-chunk advance requires one fuel and the fixed copy/item/depth grant; a denied valid turn leaves state unchanged. Ledger terminal page release requires read_count zero, and mutable iteration/get_mut also require loans to return. Cursor close accepts a changed caller revision to permit pin return and decrements the matched ledger count.

Two concrete gaps were reported to runtime and root:

1. Structural insertion and removal paths including reserve_one, try_push, try_insert_before, removal_demands and remove_key do not check read_count. A pending cursor's target can be removed and its slot reused while the caller supplies the same revision argument; the cursor can then lend the replacement generation/value. The revision parameter is not an internal mutation witness. A real pin must either prevent relevant structural changes or compare an internally owned mutation identity before lending.
2. Ledger read_issuance is publicly writable, while matches accepts any authority when the cursor captured authority zero. Pin empty ledger A and nonempty B, assign A's issuance to B, and close A's cursor on B: B's count is decremented and A's remains pinned. Issuance must be unforgeable and the identity check exact even for an empty source.

Drop frees the ledger's ManuallyDrop pages during unwinding despite its active-read assertion exemption, so raw pointer lifetime also depends on preserving the genuine ledger through unwinding and preventing forged identity. No Send/Sync soundness claim is made while those custody gaps remain open. No native build was run.

Mutation surfaces include initial/open page, reserve/cancel/insert, bind/stage/adopt/abort group, push/insert/deque admission, removal/extract/pop, mutable access and terminal page retirement; replacements.rs additionally stages and settles replacement decisions. A shared group visibility or replacement decision can change independently of a ledger mutable borrow. Cursor admission must reject outstanding groups, reservations and replacements or pin their actual decision authority as well; gating only later mutable methods does not freeze an already-issued shared decision.
