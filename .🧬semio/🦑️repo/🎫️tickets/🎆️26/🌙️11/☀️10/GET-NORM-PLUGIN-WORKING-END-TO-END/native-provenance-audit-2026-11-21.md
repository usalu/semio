# Native provenance audit

Read-only observation near 00:12 CET on 21 November. The native build graphs identify `w6JXJHa` as Energy, `wqw4JWI` as Base, and `wB34Xuf` as the newest grouped Eurocode, structural and materials build. None had a final report or linked `w` outputs at observation. No new staged component hash or successful publication can yet be compared; the previous canonical receipts remain the last established publications.

## Admitted snapshots and actual compiler reads

The admitted `w6JXJHa` source snapshot for the member opener has SHA-256 `e2f0376f109f231423fdda1728f46f81fc21908f50bdaab3adc295403ab706e0`. Its hydration constructor uses the stored replacement operation. The newer opener, SHA-256 `92b91f3b57dd0a8ab8ebc8c5fdeed832018af2b91e7de831904bd8cca64f11de`, uses `cx.operation()` and `cx.generation()` and is present in the admitted `wqw4JWI` and `wB34Xuf` snapshots.

Admission is not the same as rustc's actual source read. Both completed member-opener compiler artifacts, in Energy lane `p263819` and Base lane `p5d96da`, have rustc dep-info recording BLAKE3 `5eaa4a494d7ed3e0302a2e89def18279e6c0d85c3aad261315c802b255dad89b`, length 50,728; the Energy investigator independently matched that checksum to the current caller-identity source. Thus the actual Energy compiler read is newer than its admitted snapshot. The final provenance recheck remains necessary to determine whether that run can be published authoritatively. This does not establish linked output or replay behavior.

The admitted cleanup-pumps snapshots differ: Energy `6d052d84621fbc867a89c2e1eaf97b3d53344e1a6e47193b0cbd6c53931923da` uses the mounted close grant and forwards the publication child receipt; Base `55c67ba859404f6cf722c149d1416250b0f9bb88c4276ae8c6f08dfa9d01ddeb` uses `cx.retained_grant()` but still forwards child completion; grouped Euro `db144144ec1399349352a2150fea167d2a16ed5f27d4ac7b1062f4aecbe2bc26` includes both the original caller grant and both parent terminal-witness Complete-to-Pending checks. Energy and Base emitted rustc dep-info independently report cleanup-pumps BLAKE3 `a76ba01671c63bf09f3291a19a94c7a14500434aec77b3f5af05a8baed2df3e1`, length 41,160. The grouped snapshot is also 41,160 bytes; the older admitted snapshots are 40,705 and 40,706 bytes. Equality of length alone is not a hash comparison, so exact identity of the compiler-read cleanup code remains to be verified.

## Reclaim observation

`repo cache gc --area=cargo-store --dry-run` identified superseded component run `wx8bJEq` at 473 MiB and protected three active component runs. It also proposed 774 MiB of incremental sessions, reported six compiler-held sessions protected, and proposed zero of 3,305 build units because compilers held them. The image had 16.3 GiB free while the host was reported 11.81 GiB below its free-space floor. The supported command operates on the whole Cargo area, not just the one candidate; no separately verified one-run reclaim command was found. Recheck the dry run and live holders before any collection, and do not manually remove active runs or lanes.

A separate Node BLAKE3 run hashed the live cleanup-pump source. Its 41,160 bytes produced `a76ba01671c63bf09f3291a19a94c7a14500434aec77b3f5af05a8baed2df3e1`, exactly the checksum in both Rust compiler dep-info records. It contains the caller-grant validation and both guarded parent terminal transitions. The final wrapper receipts will identify the published bytes.

## Coherent Energy successor

The fresh native generation `wi1LQBf` entered its own compiler lane `p4e9cbe` after the `wdwI4gt` retries were exhausted. The older invocation was cancelled after the successor confirmed source admission and actual compilation. Its terminal compiler error was E0599 in visual chrome because the source called the new tree accounting methods through an older Rust dependency. The successor compiled both the current rich tree kit and the current visual chrome source. It also compiled the four receiving Work edits. Compiler dep-info for both host and WASI matches the independent BLAKE3 and byte count for the owned job context. The corresponding document-media and arc-custody compiler records match the live initializer and arc-custody sources that blocked previous publications.

| Compiler-read source | Bytes | BLAKE3 |
| --- | ---: | --- |
| Owned job context | 16,684 | `10afda45943b11a3f2b2d77d1888d8ff8f95bba3c74af865dc74b661ad6501b3` |
| Store document initializer | 31,128 | `1913889bab605b56eeadee495df96b8ee2881f8f750d31e74481cf94c8f6825d` |
| Arc custody contract | 25,243 | `85c8873a23c93817e7216502b288592a97f46769d66090961da920545a1e284e` |
| Rich tree kit | 58,041 | `0a9a35b87affd4c817db873cbc12b31480823dec676f60fbe0d0a1918411fc65` |
| Norm visual chrome | 10,604 | `a4dd26d5bbbc49553a33a31562512ba1781d72932d0ff804535116fb40f5f946` |

The native publisher still controls the final artifact admission. The source observations identify the actual Rust reads independently of that terminal decision.
