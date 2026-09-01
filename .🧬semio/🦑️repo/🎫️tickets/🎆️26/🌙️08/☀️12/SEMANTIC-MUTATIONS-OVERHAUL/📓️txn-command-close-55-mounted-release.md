# Transaction Command-Close Mounted Source Release

The transaction fixture mounts six new native tests under `🧪️tests/🧪️command-close/🦀️.rs`. Its existing `TxnFixtureJob::close_step` behavior is unchanged for the first actual native RED run. No Plugin main, Store, derive, lifecycle, or completion implementation was edited in this mount.

## Exact Source Boundary

- Transaction parent: `44199bbd5b06fffb8bd71ef91285a7235c6e84e4e8522c8a9f687a2353bf57c7`.
- Mounted Rust: `15438e324d90b46ea3b9964c93a7c980bdd2825b04d7f15a989857802b6f470f`.
- Mounted vectors: `e3f6b8a4c7236e52f17bbd0e637599a09730af718cad6c53d354cffebd118398`.
- Mounted schema: `ad076a4cf63443b31143082f68f19baadf3ba4ca5d224dc487e9b801529d9848`.
- Controller: `49eb7b3e6eb28dee18ca55003ba00e298a55adb7d9c508511384418e08820a1d`.

## Executed Source Oracle

The scoped Bun/Nx `mounted` command exited zero. Receipt: [run-GEKiWP](🧪️txn-command-close-native-55/🧫️run-GEKiWP/🔣️.json). The controller checked exact mounted bytes against the reviewed ticket packet, six unique schema-selected expected outputs, twelve negative duplicate-ID/premature-completion checks, and equal before/after hashes for all four ticket inputs. Root read the complete receipt and independently hashed the mounted files and parent.

The Rust tests have not run. The requested native selector is exactly `txn_command_close_` (six tests), using runtime's sole compiler and current retained target/default profile. Actual results must distinguish compile failures from semantic RED.

## Limits

This packet tests the retained command Box's value-layout byte grant and command/completion separation. It excludes allocator overhead and final completion allocation/last-owner close. Test output is drained through the actual completion consumer before assertions; this cleanup does not establish bounded completion retirement. No all-Plugin or mutation-migration completion is claimed.

## Actual Native RED and Command-Only Repair

Runtime's sole executor ran all six tests: two passed, four failed, 524 unselected, 0.219 seconds, no abort. Root read both the complete `INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️txn-command-close-six-r2-native-red-2026-08-28.md` report and raw `🧪️member-txn-command-close-six-r2-2026-08-28.md` output. Native command value size is exactly one byte: zero and command-minus-one both grant zero, not two distinct positive short grants. Both exact one-byte tests reported zero released bytes; both zero-byte cases released the command instead of blocking.

The subsequent parent hash is `88d8ac707cec6330e09ff76af2d94599c46f245f9be38093244b90e74e3757e3`. Only `TxnFixtureJob::close_step` changed: borrow the command, measure its actual value layout, refuse insufficient bytes before taking ownership, then drop the exact Box and report one item plus that byte count. Return before touching completion. The later completion branch, tests, schema, vectors, quotas, and payload layout remain unchanged. The same six-test native GREEN rerun is requested; it has not been reported at this source-release checkpoint.

## Actual Native GREEN

Runtime's R3 rerun completed all six unchanged tests: six passed, 524 unselected, 0.159 seconds, Nx exit zero, no abort. Root read the complete `INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️txn-command-close-six-r3-native-green-2026-08-28.md` report and its complete captured output. The two exact command grants now report one item and one byte; all four refusal laws retain ownership. The selected capture identifies the same repaired parent and unchanged test/vector/schema hashes. Source holds are released.

This is command-value release and completion-separation proof only. Final completion allocation/last-owner retirement, allocator overhead, timing, and the full Plugin suite remain unproven. Nx's flaky-task advisory followed the successful footer; no replacement repeat was used. The independent post-repair mounted source oracle also passed at `🧪️txn-command-close-native-55/🧫️run-BahSZ1`.
