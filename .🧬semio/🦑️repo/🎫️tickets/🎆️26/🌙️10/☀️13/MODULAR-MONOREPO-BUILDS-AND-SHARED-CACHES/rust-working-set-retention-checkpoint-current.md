# Complete Working-Set Admission And Selective Descriptor Retention

Source and ordinary filesystem/SQLite checkpoint. The broad build goal and ticket remain active. Physical Rust-cache quota, Native compiler/Nx behavior and platform acceptance are not complete.

## Integrated Behavior

The actual selected Rust controller previously published each selected request immediately. On the first pressure result it cleared the inactive descriptor pool and repeated Cargo preparation. Later selections were unknown to retirement, and unrelated reusable descriptors were discarded.

The selected producer now prepares immutable project, descriptor and Source facts without descriptor writes. Unsupported/fallback selections contribute no descriptor working set to the backend. The controller collects every supported selection, combines actual owned graph/set DATA, deduplicates immutable artifacts and rejects conflicting same-unit identities. Preparation is called once. The complete requested set is accounted before any descriptor body is written.

Expected capacity/corruption results leave the real reader as DATA. Under the same genuine Store/Original, namespace maintenance obtains actual EX. Capacity retirement selects only the minimum oldest unprotected prefix needed to satisfy both byte and node deficits; all wanted digests remain protected. Corrupt metadata uses the existing guarded recovery. The same prepared set is published after maintenance; Cargo preparation is not repeated. Final descriptor and Source facts are observed before backend consumption. Existing real EX-to-SH handoff and accepted child drainage remain.

The selected publication wrapper reauthenticates actual Store, Original, mode, sealed code and image coordinates before examining owned set DATA or writing files. Each compiler output directory must equal the genuine store/mode/unit slot. Immutable sets, catalog snapshots and logical removal reports are DATA; none is a Native provider, volume, allocation or cleanup grant.

## Layers And Bounds

| Owner | Responsibility |
| --- | --- |
| Descriptor cache/set | Finite immutable complete requested set, digest deduplication, same-unit ambiguity refusal, actual progress/cancellation |
| Descriptor cache/retention | Pure oldest-unprotected minimum-prefix plan against privately captured ordered catalog DATA |
| File catalog | Genuine namespace EX verification for retirement, catalog EX, bounded row cursor, SQLite transactions and exact counters |
| Descriptor files | Exact warm file matching, real tracked staging FDs, sequential immutable rename, primary-preserving stage cleanup, inactive node removal |
| Selected producer | Genuine Native authentication, Cargo/materialization preparation, Nx unit/project facts and file-owner delegation |
| Selected backend | Whole-selection combination, one preparation, complete-set publication, maintenance, final Source observation and real backend reader |

The existing logical descriptor-body frontier is1073741824 bytes and65536 entries per mode. Individual nodes remain bounded at16777216 bytes. Individual graph construction remains bounded at268435456 bytes. The redundant old metadata-model `maximumBatchBytes` check/property was removed because it incorrectly treated a complete set as one graph. The whole set uses the existing1GiB frontier; Original time/output, compiler and physical-image budgets were not increased. A scalar schema/SQLite law checks the distinction; it is not an executed1GiB allocation or performance claim.

The catalog retains its16MiB page ceiling, bounded statements, DELETE journal, EXTRA synchronous policy and explicit journal extent. Warm admissions do not rewrite catalog rows. Actual file publication reuses unchanged descriptor bodies without rewriting their inode or mtime. The complete immutable descriptor/Nx unit interface remains; mutable accounting is separate.

These are logical metadata/body bounds. They are not a physical Rust directory cap or a maximum repository size. Source, Git, tickets, unmanaged caches, filesystem allocation and compiler state require their own qualified accounting. The intended managed-cache envelope must not be presented as a total-repository guarantee.

## Retirement And Failure Ordering

Retirement requires actual retained EX and catalog EX. It reads rows in bounded128-entry cursor batches and emits real progress/cancellation opportunities. It removes only declared canonical inactive node bodies/stages under exact bounded Source observations and closes descriptors before directory removal. It then deletes the corresponding exact catalog rows in bounded transactions. Same-connection SQLite `changes()` checks the direct deletion, excluding accounting-trigger side effects.

Files are removed before metadata deletion. An interruption may leave missing files conservatively over-accounted; it cannot commit lower accounting while those selected victim files remain. An intrinsically oversized requested set causes no eviction. Unknown entries, linked paths and unknown closure remain fail-closed. Logical removal incidence does not establish physical free space or Native cleanup custody.

Staging uses exclusive-create tracked FDs and awaits descriptor closure before rename. A body Error or undefined remains primary if awaited stage removal also throws. A sole cleanup failure still fails. Tests of post-removal acknowledgement do not qualify unknown physical unlink or close outcomes.

## Test-First Evidence And Current Qualification

Neutral schema-authored vectors cover duplicate/distinct complete sets, protected FIFO eviction, byte/node pressure, missing-file accounting, intrinsic overflow, concurrent requests, interrupted removal/rollback and staging cleanup. Independent SQLite queries validate sums, deduplication, admission and the minimum eligible prefix; TypeScript AST checks qualify actual integration structure. Exact warm body hashes are compared with Bun CryptoHasher.

Four actual selective-retirement cases preserve the catalog inode: node pressure retires1/keeps2 unrelated old nodes; byte pressure retires2/keeps1; a missing victim retires1row with0 removed logical file bytes; an intrinsic6-node request against a reduced5-node limit removes0 and preserves all3old nodes.

Four isolated failed child Originals exercise before-unlink, after-unlink, after-unlink cancellation and a DELETE failure followed by real rollback. Wanted3 files remain in every case. Catalog6 rows remain conservatively accounted; old files remaining are3/2/2/1 respectively. Parent Originals remain healthy; failed Originals are not retried or repaired.

Two actual concurrent child requests each contain6 wanted nodes. Equal sets both admit and invoke their bodies once; distinct sets whose union exceeds the reduced8-node frontier yield one admission and one pressure with0 body calls, retaining6 catalog rows. This is ordinary shared descriptor IO, not Native compiler concurrency or end-to-end Nx cache restoration.

The actual file owner is exercised directly without a fake Native Store. Cold publication writes3 nodes; warm publication writes0/reuses3 and preserves all3 inode/mtime pairs. Four isolated staging cases preserve Error/undefined primaries, fail on sole cleanup errors, leave0 stage files and retain conservative3-row accounting.

Actual intended REDs were retained: missing set/retirement/selected-preparation/file-publication implementations, and the redundant graph-vs-complete-set frontier. Fixture errors were identified separately: a wrong relative backend URL and live mocked-fs unlink recursion. Later laws after same-Original failure are dependent, not independent successes. Final corrected runs are recorded below; no failed-original healing, clock reset, grant substitution or cap expansion was used.

Prototype strict and all8Sourcefiles returned0 in terminal32072 after actual file-layer integration. The six-command qualification collector91646 then returned0 for every command; both enclosing modes selected219tests/2992assertions across15files, with separate adapters. Its1741-path static Source window retained25owned identities and had3other Code/Schema drifts in actual filesystem descriptor source/schema and dependency install schema. These are earlier results, before the incremental collection follow-up below.

A further growth audit found that retaining every per-request graph until the final combine allowed overlapping selections to retain many copies of the same immutable data. Added a live incremental collection scope shared by array combination, the actual producer and the actual backend. It deduplicates each graph as preparation completes, checks the existing unique-set frontier immediately and freezes one set only after accepted additions drain. Returned set DATA may outlive preparation; the append callback and issued preparation controls may not. Descriptor metadata is keyed by digest; unsupported metadata is keyed and capped by authored producer policy. This bounds retained descriptor copies by unique requested nodes rather than the number of overlapping selections; it is not an OS heap/RSS measurement.

First collection Source command39453 passed, while strict failed only on an untyped test SQLite row. The row shape was supplied; accepted-child-drain plus5isolated copied/cancelled/expired/ambiguous/undefined guards then passed in88039. The subsequent six-command18343 matrix returned0 throughout. A final footprint regression below was added and corrected before the final affected verification.


## Fixed Publication Lease Cardinality

A real four-generation regression exposed a second persistent cache: redundant `rust-descriptor:<digest-path>` leases created18 databases, even though descriptor publication already held namespaceSH/catalogEX. The intended RED33684 failed on18 versus3 before implementation changes. The actual file writer now relies on that existing genuine catalog writer; it creates no content-addressed leaf lease. The same actual cold/warm/four-generation law then returned3 lease databases and no `rust-descriptor:` identity in an independent SQLite census. Warm descriptor writes remain0.

This3-count is for a fresh publication-only ordinary root. Namespace maintenance can add its fixed writer-admission key; other subsystems and preexisting lease files are outside that test. No live lock was physically purged. Residual shared-metadata retirement, stale-stage accounting/census and the unqualified Native fixture path are explicitly recorded in `rust-cache-residual-footprint-follow-up-current.md`. A bounded logical body catalog and bounded newly created publication keys do not establish the user's required maximum physical Rust cache/repository footprint.

## Final Observed Results

The final shared-source four-command collector82006 returned0 for prototype types/Source and release types/Source. The preceding5859 failure is retained: actual shared fixed-storage Source referenced two missing/unimported Sync helpers. The peer then supplied genuine implementations and imports. An assertion-guarded attempted import edit wrote nothing because the current peer import already contained the helpers in a different order. No alias, stub, policy relaxation or Native execution was used to obtain a green result.

| Enclosing mode | Selected Source tests | Assertions | Files | Failures |
| --- | ---: | ---: | ---: | ---: |
| prototype | 223 | 3035 | 15 | 0 |
| release | 223 | 3035 | 15 | 0 |

Each mode comprises136/1234 across8affected Source files,67/615 across5artifact files, and20/1186 across2configuration files. Each test command also separately passed the mandatory10-test/216-assertion adapter. The artifact file with27filtered tests is included as1selected test/9assertions; the first collector regex that omitted its filtered line was corrected. Final type rosters contain43ordered roots and retain the prior41; four launch rows are unchanged.

The final file-writer/lease change and shared-source completion are covered by82006. Artifact/configuration results are retained from18343 at their earlier loaded revisions. No identical whole execution closure is claimed. Neither enclosing release invocation executes a release Native binary or proves native build performance.

| Command | Collector | Exit | Elapsed seconds |
| --- | --- | ---: | ---: |
| prototype types | 82006 | 0 | 18.953 |
| prototype source | 82006 | 0 | 68.626 |
| prototype artifact | 18343 | 0 | 22.149 |
| prototype configuration | 18343 | 0 | 45.324 |
| release types | 82006 | 0 | 11.462 |
| release source | 82006 | 0 | 52.367 |
| release artifact | 18343 | 0 | 22.018 |
| release configuration | 18343 | 0 | 18.094 |

Final static Source census:1741 identities,26owned stable,0missing and4other Code/Schema/test drifts. These are actual Source/Schema/test inputs and cannot be dismissed as harmless or non-executing. Command results remain tied to loaded revisions. Observed drift:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`: 15352 bytes, SHA256`0a196db037f3b809a760a65ccc3931c94f2147daaf4ec11cc9e04309e763dc13`.
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json`: 32662 bytes, SHA256`d680953087dd5d449f4edf94a1a1323753c9b1436c178b2f344f7d8867410d92`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts`: 116530 bytes, SHA256`675dd7d5cbc0a1d7591320ff446868a015f07c22be55d83086bacb04dac90429`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧬️schema/🔣️.json`: 29635 bytes, SHA256`9741532907b24048060e0a0ac32608d2f1b35a8a785b5829fae61e555b1a16af`.

Generated terminal receipts, bounded logs, exact rosters, static Source rows and final summary are under the ticket's `🗑️generated` directory. They remain while the ticket/goal is active and must be removed only at actual ticket completion. Failed attempts and fixture errors are retained in the work note, not silently reclassified as passes.

## Owned-Scope Source Snapshot

The role includes carried-forward catalog/admission laws as well as current implementation; it is not Git attribution or a claim that every row changed in this slice. No source was copied into the ticket. Source observations at the final window:

| Path | Bytes | SHA256 |
| --- | ---: | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | 119072 | `1d8d7c23e909b774dce8fdc33509e9c539e2dc9c74579ca96195cd075e6a9d6f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts` | 37898 | `b5443471ea0afc802d684509cac5f3e4c9e78f7fe7a06c27aa7c891ac3f74be7` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🟦️.ts` | 15352 | `2aea5d3f400690868439d86ef031c9846f305f4f6ea630b8f7106c03ec9c33bc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🧬️schema/📥️producer.json` | 849 | `67b69606a29e02e1d22d27f932f29eae6905866c5a2dc18cc43ae42bc7e9708a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/♻️retention/🟦️.ts` | 2878 | `bb4700db850578ce3681dff6b87819f7649edc0a04119dd9cbb75a37592753e8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/♻️retention/🧬️schema/🔣️.json` | 198 | `e5daf47d484022920df6e2dbde95483a9f0669be6ede62135a9832338d195fd8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🗄️catalog/🟦️.ts` | 23939 | `dae09e230c115355f0f005cd62f3a9431af73ac1510d54f9f05895be9c9a1f0e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🗄️catalog/🧪️tests/🔣️.json` | 1659 | `24b178ee704556c77427d71412bd54d65951f71d61aa340a81aa192921ec2a07` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🗄️catalog/🧪️tests/🟦️.ts` | 7853 | `cb8c416996ec41009df03a36be02fc7eab0720cebf376a3acaa9e5bf7905c6b8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🗄️catalog/🧪️tests/🧬️schema/🔣️.json` | 2361 | `f00a69ce649e9ba99bbe42646a1c3ac2696043f56f09e0cd4412f023df7458a4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🗄️catalog/🧬️schema/🔣️.json` | 2533 | `0b1ac7c7ad995a3867d2702bbfed8a609768d0791be448298aa0ab9fd8bdd5a9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🟦️.ts` | 16071 | `82718af79e00908931ec03fcd1c8e4b639d59fd6166236a75f187995f1570c23` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🟦️.ts` | 9289 | `dcdc3d887bd35048e13931b8bada56c22775bc91581477f9b817d7c6eab4cd87` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/📦️child/📜️script.ts` | 15541 | `98f604515d4867624002844c3dab1449162fb91fabb5a2d55eed2499e10b79e3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/🔣️.json` | 4560 | `454a3f516e6c7eabe563405de1666d98495875fea55685551515a32ca0fc4521` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/🟦️.ts` | 27960 | `6d9327be250ac628ea33bd5de1c5f2e2215df6b32d9970b28a4ab06ed0f5f468` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/🧬️schema/🔣️.json` | 6028 | `1f9da3102b6ddd48f50922c4e036e80e226c62e6c5d0439d93eec6da220242e5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧬️schema/🔣️.json` | 2556 | `9bd6096137f2178a86d446bf2f466a232526e54f2c2639ba2108f2ccfa36ce9a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🟦️.ts` | 6940 | `1a3d6adf0d606da760d4b1b6fd5868c50c737f54dd9629b2b46f9205ff11669a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🔣️.json` | 4601 | `eb47216b23ff14e5646a5723e3096294f648341933bb2e57bf5e60ebcdb9449b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts` | 33888 | `94c62962283beaaf82849250006fd46fee91f655fe38a5463d2d671d258e97f6` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🧬️schema/🔣️.json` | 7160 | `90e8c5cdb65acb585d4eefab334b1825b9925f54dfcfb9cf0d167489415d2161` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧬️schema/🔣️.json` | 664 | `7c8dfbb40a6b38b1d91117f95be0e1d63338599d31764acbade02c729a06846b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🔒️namespace/🟦️.ts` | 6874 | `ca9ea79a999105d97c41862a5bcd0f3282ae15d70f6fcfe68f866a6ab3e5e158` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts` | 83792 | `9dda293437d3d06b81a6625952ccfd17c3a70409726643968af34d77fe591e00` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts` | 18997 | `c258ad8faee77336fe0c2332a62e7c5f35a6783532a5a85b01a7467bbc789f7b` |

## Scope And Remaining Acceptance

The Source roster is static and not a complete execution closure. Each command must be tied to the actual loaded revisions; shared Code/Schema/test drift must be reported, including changes in other agents' areas. The strict roster retains the prior41 ordered roots and appends2, for43. Four existing launch rows remain unchanged; no new developer command was added.

Both modes here mean enclosing Source test/type commands. Ordinary filesystem vectors predominantly use prototype-mode fixtures. They do not execute release Native binaries, exercise the Node SQLite adapter/platforms, prove crash/power-loss behavior or qualify the actual physical cache image.

Concrete next Source work is recorded in `rust-selected-graph-growth-follow-up-current.md` and `rust-cache-residual-footprint-follow-up-current.md`; no full architectural-growth closure is claimed.

Still required: genuine hard physical quota/headroom/reservation and full-image pressure; unknown-close and failed-unlink custody; power-loss/platform qualification; actual Nx task hashing/recompilation/restoration and warm end-to-end builds; compiler concurrency/CPU/memory acceptance; whole-cache and repository footprint accounting; reader fairness; and giant-crate/package/API closure. No broad completion, total-repository cap or hold release follows this checkpoint.

The three execution workers remain terminal on “Fatal error: application network permission was revoked”. No replacement network/review route was used. Repo-specific ticket/goal MCP tools are unavailable in the current tool inventory; the existing ticket is retained and not closed through another interface. The broad goal stays active while authorized Source work remains possible. Physical/default SDK/publication/compiler/cache-image/OS-sampling holds remain in force.
