# Async Implementation Pin Repair Independent Review

Static read-only review; no Source edits, commands, helpers/functions/validators/tests executed. Final four fingerprints match owner: runtime8913B/730a2ec303cd43e66f919e9e96086d128fe3ef3c0a32b1e30f09411f4255f523; schema6707B/a01f271e9c76a46fce157bbab55619e0dfe0f1bf549945a8e6fca7a65f03e63c; corpus5995B/5c8608c02d4acf78dd21ccc039014bfaf7273599353832fb001a3aed1d66184b; test15990B/0ee7dfad9f84a7ac4c19847c4c46c2f83acd20e720378194c66df6822801aa52.

## Scope Repair

The ambient integer is replaced with builtin AsyncLocalStorage, shared between reader module copies through a new Symbol.for key. Each call creates its own active token, runs body in that async context and deactivates that token in finally on resolve or reject. implementationPinnedV1 reads only the current async store token. Unrelated concurrent branches no longer inherit another branch’s pin; legitimate descendants and module-copy calls share it. Nested return restores outer store; delayed descendants retaining the inner token see inactive after inner settlement even while outer remains active, as explicitly specified. Outer delayed descendants likewise deactivate after outer settlement. No unbounded global counter or retained token collection is introduced.

## Actual API and Independent Oracle

Two distinct query-loaded actual reader APIs are checked for distinct function identities while sharing context. Two closed neutral scope forests have bounded16nodes; recursive SQLite computes pin ancestry independently and verifies complete unique forest membership before actual concurrent roots/children are run. All roots wait until every scope entered, ensuring outsider reads occur while another lane holds a pin. Unpinned scopes test the actual current function AST with changed-capture facts and must refuse stale loaded reader; pinned scopes must follow the retained loaded-code policy. This guard test uses a capture port and does not claim a new physical-drift integration result.

Four closed lifetime cases cover every inner/outer resolve/reject combination, exact object failure identity, restoration after nested settlement and deactivation of gated descendants after each corresponding body completes. Gates are awaited after finally paths, so false observations do not merely reflect return to the caller’s outer context. Existing physical Source laws and registered names remain. The old pin test now uses actual API copies/guard AST rather than copying Source into a system temporary directory. Neutral corpus/schema define these scope/lifetime cases; SQLite is independent of runtime ALS implementation.

## Disposition

The concrete ambient concurrent-scope blocker is resolved by this narrow candidate. No remaining blocking static finding under Root-authorized loaded-implementation continuation semantics. Requested Source target descriptor/ancestry/captured-hash checks remain separately intact; pin semantics still intentionally suspend loaded-implementation self checks within the admitted async body and do not constitute current whole-Source freshness proof. All six selected reader titles (including registered physical laws and new isolation test) require actual future collection before any pass claim.
