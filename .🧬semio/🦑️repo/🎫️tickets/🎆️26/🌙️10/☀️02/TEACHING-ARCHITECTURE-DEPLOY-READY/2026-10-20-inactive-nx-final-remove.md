# Inactive Nx Final Capsule Removal

Root27274 actualexit1 beforecollection. Final151615-byte log at2026-10-20T18:52:01.404Z records resumed intent, complete kernel/reader preflight and retirement of image.json/fixed.dmg/m (2files,1609564353logicalbytes), then ENOENT on capsule path during filesystem remove’s actual post-unlink checkpoint. Caller heldCapsule was cleared after awaiting remove, so it required the deliberately deleted name during that callback.

The filesystem API owns an exact issued same-frame entry, validates current named/retained inode, executes unlink, checks its inode transition, adopts the surviving parent and proves the deleted name absent. Its guards remain unchanged. Caller should perform the explicit final check after exact empty-entry comparison, relinquish only its no-longer-live capsule-path observation, then await the exact expected-entry remove under the surviving parent/ancestor/writer custody. A failure stays a failure; no restore, invented terminal or extra cleanup.

Schema and two neutral vectors were written before producer code. Existing real filesystem actual-owner test expands14to16scenarios under the same title: positive records the real post-unlink checkpoint before completion, negative replaces the empty capsule immediately before remove and proves the foreign replacement plus retained original are preserved. Independent SQLite computes acceptance from the replacement flag; Ajv admits closed vectors. Existing root/nested and all prior cuts/cancellation tests remain.

Test-first roster:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧬️schema/🔣️.json`: 8454bytes, SHA256 `1c7eeb331c7f1ae9588cb5bee4499af3834445333c9d56a6536a77041c372b57`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🔣️.json`: 15507bytes, SHA256 `54f589f8e69268eb66514612539b8bac069a2ddc6fbe80e9abe1279a9b144c44`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🟦️.ts`: 26808bytes, SHA256 `2d2d2ce7a38a4211db8e59bdc48c2fb4b0c4f0162a5afff3f114eb472a60dc12`.

Final candidate (syntax0, unexecuted):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧬️schema/🔣️.json`: 8454bytes, SHA256 `1c7eeb331c7f1ae9588cb5bee4499af3834445333c9d56a6536a77041c372b57`, mtime `2026-10-20T19:03:26.725Z`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🔣️.json`: 15507bytes, SHA256 `54f589f8e69268eb66514612539b8bac069a2ddc6fbe80e9abe1279a9b144c44`, mtime `2026-10-20T19:03:26.726Z`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🟦️.ts`: 26808bytes, SHA256 `2d2d2ce7a38a4211db8e59bdc48c2fb4b0c4f0162a5afff3f114eb472a60dc12`, mtime `2026-10-20T19:03:26.727Z`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts`: 27637bytes, SHA256 `3430da766c59cd88375ba654e977b504806c8b042478794170c52f534bc9a8f9`, mtime `2026-10-20T19:03:26.730Z`.

Unchanged filesystem descriptor SHA256 `0e1ec1d4e7fddd9d69672fccc10cf7f4cf08861a8edc31765c9387e7090cd008`. Active38213 and its eight-phase seal remain unchanged; no retry dispatched.
