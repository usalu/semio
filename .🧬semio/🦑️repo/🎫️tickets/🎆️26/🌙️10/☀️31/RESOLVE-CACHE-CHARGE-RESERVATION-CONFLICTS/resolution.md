# Cache Charge Reservation Conflict

The unmerged paths were a stash apply (`Updated upstream` against `Stashed changes`), not a second branch merge.

## Sides

Upstream admits the shared partition through `acquireNativeStorageOwnerV1` before the receipt lease is held, requires a committed allocation, and re-quotes the receipt under that lease. Its fixture and tests cover creation, recovery, refusal, and a displaced receipt, with Ajv, TypeScript, and SQLite as the independent checks.

The stash held the receipt lease first, then observed or recovered storage, and flushed the receipt directory with `syncNativeDirectoryMetadataV1`. Its fixture was only the call name and does not satisfy `📋️schema.json`.

## Resolution

Owner admission stays outside the receipt lease. After the receipt file is renamed into place, directory durability goes through `syncNativeDirectoryMetadataV1` instead of opening the image directory with `openSync`. The behavioral test records that flush while the receipt lease is held and requires it to sit between the receipt write and the lease release. The case fixture stays the upstream document.

The owner-admission test extracts both `acquireNativeStorageOwnerV1` and `acquireNativeStorageOwnerBodyV1`, because the public function delegates to the body. Both tests pass.
