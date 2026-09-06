# GIS Durable Three-Store Assembly

## Outcome

The Store durable-group subtree now contains a retained fixed-three assembly owner for Map, Drawing, and Value Stores. Construction accepts exact Store owners, unbound mutations, role-specific one-item preparation factories, and the decision sink. It does not accept a group id, prepared seal, recovery pack, or caller-supplied outcome hash.

Admission uses `begin_apply_one` with each explicit typed factory. Each publication is advanced only to `Publishing`; the assembly then takes the three Store-verified prepared candidates, closes the publication shells, derives the unbound outcome hashes from those candidates, performs the sole private `None -> decision hash` bind, and mounts the existing retained decision host. A rejection or cancellation before journal start closes every previously admitted publication and terminally hands back the exact three Stores and sink. An uncertain journal start retains the same mounted host for controlled retry.

The GIS editor exports exact parent, drawing, and value preparation-factory builders. The Store assembly remains generic and does not import GIS.

## Language-Neutral Contract

The strict `semio.plugin.gis.map.durable-three-store-assembly/v1` fixture models canonical role order, no caller group input, prepare-before-journal ordering, late-role rejection, cancellation handback, and uncertain journal ownership. The Bun oracle validates the schema with AJV and the canonical fixture identity with independent Node and WebCrypto SHA-256 implementations.

Source receipt:

- `gis-durable-three-store-assembly-oracle: AJV=1 SHA256=node+webcrypto roles=3 cancellation=4 rejection=3; no WAL recovery or Hub publication claim`
- Registered `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run @semio-tech/gis-plugin:durable-three-store-assembly-check --skip-nx-cache`: exit 0.

## Exact Laws

1. Store assembly admits exact factories and binds one decision.
2. Drawing or Value late rejection closes earlier publications and hands back owners.
3. Cancellation before journal returns all terminal owners without publication.
4. An uncertain journal start retains the same mounted host and does not call `begin_commit` twice.
5. GIS builders expose the exact three role-specific factory ports.

The source and native Nx targets and launch seeds 411.088/411.089 are registered. The Home native build first exposed missing `OpBinary + OpText` bounds at all three `begin_apply_one` calls; the assembly bounds were corrected. No post-correction native law receipt exists yet.

## Nonclaims

- The current decision sink remains a test implementation; no ArtifactWal or database journal adapter is wired.
- The assembly is not yet mounted by the GIS Map actor or Hub approval path.
- Event-only WAL bytes are not treated as recovery proof or an `InferenceWalWitness`.
- This slice does not claim atomic publication, crash recovery, or Hub acceptance until the typed WAL decoder and same-transaction replay authority exist.
