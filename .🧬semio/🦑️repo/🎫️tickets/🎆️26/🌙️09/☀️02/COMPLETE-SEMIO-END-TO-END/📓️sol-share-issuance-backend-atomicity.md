# Share Issuance Backend Atomicity

## Contract

Document-share issuance succeeds only when the exact persisted `(space_id, document_id)` descriptor and its Space exist at the insertion linearization point. The same backend transaction and writer ownership that inserts the secret digest also evaluates that predicate and appends the success audit. A missing or concurrently deleted owner rolls the whole operation back: no grant, no success audit, and no returned capability.

The predicate is intentionally independent of membership role and Space kind. An archived Space and its demoted Spectator remain eligible for read sharing when the administrator policy has already admitted the operation. This backend boundary does not invent an Author-only or writable-Space rule.

SQLite and PostgreSQL bind `hub_share_grant(space_id, document_id)` to the persisted descriptor with an `ON DELETE CASCADE` composite foreign key and issue through one `INSERT ... SELECT` joined to both persisted owners. PostgreSQL additionally retains the selected owner rows with a locking clause until commit. Neo4j matches both exact owner nodes, creates the grant between their ownership relationships, consumes the returned count before auditing, and authenticates only through the still-live owner path. Both the ordinary issuance method and the retained administrator issuance method obey the same predicate; the latter also commits the private administrator effect receipt in that transaction.

## Acceptance

The language-neutral corpus must cover live, archived-Spectator, missing-Space, missing-descriptor, wrong-document, and deletion-race outcomes. Bun's SQLite implementation is the independent query oracle. Production source hostiles remove each backend predicate/lock/owner relation and must fail the source gate.

The native SQLite law must prove an exact persisted descriptor issues and authenticates, archived-Spectator sharing remains valid, every absent/crossed scope rejects without grant or success audit, and a second backend writer cannot leave a successful orphan across descriptor/Space deletion. PostgreSQL and Neo4j receive source/query qualification only until live services are available; no live backend claim is permitted from their source gates.

## Current Status

The schema-first corpus is implemented at `🌎️hub/📇️directory/🧪️tests/🔐️share-issuance-atomicity`. SQLite now begins an immediate writer transaction, evaluates the exact persisted Space/descriptor join in the insert, checks the affected row before the audit, and owns grants through the descriptor foreign key. PostgreSQL uses the same insert predicate, composite owner foreign key, and `FOR KEY SHARE OF s, d`. Neo4j creates the grant only between exact Space and descriptor ownership relationships, consumes the result count before auditing, and requires both owner relationships during authentication.

The registered source run `os-hub:share-issuance-atomicity-check` is GREEN `8d7771`: AJV 2020 validated the neutral schema/corpus, Bun SQLite independently executed all seven cases, and all seven production-source hostiles were rejected. The source oracle separately inspects the ordinary and retained-administrator writer bodies, so removing the predicate from either writer is rejected. The final run completed with `checks=14 mode=source`. The retained-admin follow-up source gate `1ec7e9` additionally verifies PostgreSQL's effect-receipt insert conflicts on the unique operation owner, giving a substituted intent digest the same established-identity comparison as SQLite and Neo4j. PostgreSQL and Neo4j are source/query-qualified only; neither backend was run live.

The project targets and launch entries are registered for source and native modes. Registry generation and the immediate `check-generated` pass are GREEN; generated launch lines are 6172–6202 in the current file. The SQLite native law is implemented but remains unexecuted while the coordinated Hub GIS native gate owns the single heavy build lane. No native/backend runtime verdict is claimed yet.
