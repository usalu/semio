# Rust Enforcing Producer Integration Design

Status: pre-production Source design. The independent retirement correction audit is CLEAN (253629fb0433ea2b8ad691da3e4665a68c1adbcefbf15bf07b202a66d75f98a5, 222 identities). The existing queue/context checkpoint is accepted only within its narrow scope; full producer concurrency remains RED. This document does not qualify Native execution, cold/warm builds, compiler throughput, Windows recovery or sustainable physical eviction. No production/schema/test changes belong to this design epoch.

## Actual Current Consumer Chain

U/script holds the actual per-output unit writer and dependency readers, then authentic RepositoryCompilerStore and withCompilerWorkspaceWriteV1 around compile+publish. U/state requires that genuine writer and rejects a second active store.root. L/compiler authenticates its private store owner and wraps the entire Core Volume callback in the same workspace writer. Capacity takes image-root EX on the existing compiler resource. Core cache already takes the actual per-context queued SQLite writer, but admits and commits ordinary scratch directories under whole state-image EX and full descriptor census; its catalogue commit requires the entire global sequence to be unchanged. Its inner ordinary workspace allocation consumes all 4031 MiB of the one state filesystem. Removing only an outer guard therefore creates unsafe shared filesystem producers and stale catalogue commits.

U/context v3/direct-rlib-v3 includes closed execution root/policy; prototype uses compiler-context-v1. Hash-bound scratch is currently <stateRoot>/<full-context>/m/w, output m/b/p, but incremental state remains numeric ordinary slots i/<slot> in the common image. Actual rustc temporary variables already target its issued workspace. Source, environment, CPU reservation, consumer/receipt and publication chains must retain their defining owners.

## Exact Envelope And Layout

The outer 4 GiB state charge, 64 MiB parent metadata, 4032 MiB backing cap, 4031 MiB filesystem, 1 GiB state free floor, 4 GiB Root compiler headroom, eight catalogue slots, 32768 catalogue bytes and all entry/path/operation caps remain unchanged. Counts/environment are never capacity authority.

Proposed explicit sub-envelope: 1 GiB producer filesystem, filesystem+1 MiB backing, 65536 metadata bytes within that exact child charge. This permits two charged producer images plus the unchanged 1 GiB floor and a reused fixed catalogue metadata reservation within the existing outer envelope. It is a bounded policy choice requiring Source arithmetic laws; it claims neither actual filesystem fit nor performance. Finite summed issued Native charges, in-flight creation peaks and actual free-space floor gate admission. The previous whole-filesystem ordinary workspace allocation is replaced in the new closed contract, not layered over the new allocations.

Each context owns a genuine Native child at <stateRoot>/<context>, mounted at its canonical m. Cwd is m/w, incremental m/i, output m/b/p/units/<private attempt>. The complete context hash is stable; the attempt is privately captured once before waits and bound with actual allocation, original generated request, immutable request DATA, receipt hash and backing identity. Catalogue carries the bounded intent/binding; child-local metadata lives inside its removable filesystem. No external per-success binding file survives normal successful retirement. Native sidecar/namespace custody remains in its existing genuine defining owner.

The contract changes to context v4/direct-rlib-v4 and compiler-context-v2 by hand across actual descriptor/materializer/fixtures. Release temporary-v1 stays. Intrinsic compiler profile semantics stay unchanged; derived context/receipt hashes change intentionally. Historical ordinary directories are refused, never adopted or migrated.

## Parent Protection Frontier Found In Fresh Census

The desktop peer's protected-parent API is a read-only observation under actual nonwaiting parent EX. It verifies receipt/backing/root CAS and bounded stable descendant DATA; it is not a producer mutation port and cannot be acquired while our own parent SH is retained.

Fresh Source differs from the earlier checkpoint: Fixed Linux observe now calls withOwnedLinuxChildrenV1, which takes child SH leases then checks nested image receipt/inode/loop association and reobserves the roster. This is unknown peer ownership. It does not yet prove the same original committed parent allocation/creator sidecar chain, and its child lease waits may occur during a parent journal transaction. A child EX owner performing parent retirement can then wait on that same journal while parent recheck waits for the child SH. The exact source below is recorded; do not assume the old no-descendants implementation or claim this peer change supplies safe producer custody.

Before replacing guards, the Native defining owners need a genuine parent mapping-mutation boundary. Birth/retirement must hold the same actual parent mutation EX only through the mapping/receipt transition, after their actual child kernel EX. Ordinary parent journal transactions/rechecks hold its SH before parent journal EX, retaining it through the transaction. Reentrant operations reuse the exact private held proof/control, never a caller boolean. Parent mapping census authenticates descendants from the exact complete parent child journal request + private creator sidecar/receipt/inode/kernel mapping, recursively within finite schema depth/roster bounds. It must not acquire child kernel leases while holding parent journal EX. No caller allowlist, path registration or public DATA supplies authority.

The mapping SH must not wrap full withOperation or the full Volume callback: either would self-deadlock the child birth EX. The actual storage transaction seam, birth preparation handoff, retained cleanup and both Mac/Fixed backend observers must consume it. Cleanup captures its original resource loan with the accepted enrollment before waits; the same genuine Original remains. Unknown mapping transitions remain unresolved, not healed.

## Proposed Consumer And Lock Order

1. Authenticate original repository store/workspace lifetime. An actual SH on the SAME existing compiler writer resource admits isolated prototype work and excludes general Cargo/release EX writers. Keep writer checker EX-only; add a separate private producer reader scope requiring genuine lifetime/proof/Native loan. CPU scheduling is untouched.
2. Acquire exact context queue EX and retain it through all accepted descendants, result/receipt consumption, publication, final state commit and closure. Distinct contexts use distinct queue resources.
3. Brief outer creation queue opens or creates the same genuine state image and binds its existing Root allocation; release creation queue inside successful actual open callback before user producer body. Retain actual parent SH through child lifetime.
4. Brief catalogue EX snapshots current entries/charges/peaks and writes this attempt's bounded creating intent. Release before child kernel/parent mapping waits. Actual funded factory obtains original child EX, original parent journal/enrollment/cleanup loan and generated original request; mapping mutation window ends only after genuine receipt publication and retained-reader handoff.
5. Brief catalogue EX binds that exact genuine child/charge/request/receipt/attempt, commits the actual parent reservation, then exposes only an opaque Core Volume/Rust loan to compiler consumers. Child workspace reservation is issued inside that child filesystem. Parent floor/outer envelope cannot be borrowed by another reservation.
6. Seal admission, join all accepted descendants including consumed-byte/receipt copies, retain exact first Error or undefined, retire scratch through its existing compiler owner. Actual child maintenance closes its own reader, obtains child kernel EX/full descriptor exclusion, validates original cwd/output identity, removes only privately owned scratch and measures/syncs retained incremental state. Close child writer/open lifetime before catalogue commit.
7. Brief catalogue EX rereads latest catalogue and CASes this exact context/content/attempt/allocation/binding revision. Other entries and global sequence may advance. Persist clean state/digest; the current whole-catalogue equality and global-sequence commit precondition are replaced together. Never loosen context/attempt identity.

Logical ordering is context EX -> short catalogue EX, with catalogue released before child kernel EX -> parent mapping mutation EX -> parent journal EX. Ordinary parent journal work is parent mapping SH -> journal EX. Ancestor mapping/journal observations must never wait for descendant kernel leases. No parent whole-image EX is requested while our own retained SH exists.

## Sustainable Pressure Ordering

Pressure selection briefly holds catalogue EX and selects a finite candidate. Release it before trying the exact victim context queue, nonwaiting on the SAME defining resource. Busy candidates are skipped; pressure never queues behind another producer while holding its own active child or catalogue EX. A new actual nonwaiting producer acquisition must share the existing defining queue implementation and be consumed here, not left as an unused helper.

Under victim context EX, reread/CAS the exact entry and persist a retiring tombstone; release catalogue EX before invoking canonical retireNativePlatformFundedChildStorageV1 with the genuine recovered reservation/parent. That defining API performs actual child EX, complete reader-FD exclusion, private creator claim recovery, both physical operation owners, receipt/backing/mapping removal, claim/namespace durability, child kernel/original child reader closure, then same parent journal retirement. Only its successful return permits tombstone deletion and reusable catalogue slot. Normal successful operation leaves no per-child sidecar/metadata residual.

A thrown outcome preserves unresolved catalogue custody. It is not proof replay remains charged: complete retired bytes may already replay retired after late barriers fail, and final retained-parent close occurs after discharge. The prior CLEAN correction's exact first-cause/order distinction remains. No path absence/DATA/event alone authorizes metadata deletion or allocation restoration. Fault recovery may exhaust the finite envelope; no same-operation healing or silent rollback is added.

Dirty complete images are never reused/adopted. Whether safe discard is allowed requires actual recovered private child retirement under victim context EX/full FD exclusion; the current dirty-retain neutral model must be replaced only with explicit proof-gated consumed owner logic. Incomplete/unknown birth cannot be discarded by inference.

Whole-image EX remains solely for genuine quiescent creation/outer retirement after all retained parent/child views close. Read-only protected-parent observation is not repurposed as a write port. Active children cannot be selected/removed by counts or path checks.

## Test And Integration Gates

Before production edits: closed schema-first envelope/attempt/per-entry revision/lock-order/pressure/failure corpus, independent SQLite oracle, actual Source RED requiring withBoundCompilerStateProducerV1 and authentic birth/retirement consumers. Ordinary real NodeFS/SQLite cross-process laws cover distinct contexts overlap, same-context queue, busy-victim skip, no parent-journal/child-kernel cycle, cancellation, exact Error/undefined, late descendants and first failure. They must not fake positive Native providers.

Then consume defining Native parent transaction/mapping boundaries in both Mac/Fixed owners, then Core enforcing child lifetime and pressure, then actual Rust/store/script guard replacement. No global guard is removed while any required enforcing link is unused. Preserve all current strict35 roots/flags/test roots, expand for new actual modules, run BOTH modes with all Bun births cacheoff0/1. Capture terminal handles and exact Source/log SHA+size before/after; retain RED and shared drift. Root independently reviews before physical qualification. No new executable script/launch command is planned.

## Proposed Production Touch Set

Exact existing owners are listed by the roster below. Expected additional schema/test paths are the adjacent Core cache producer and Native storage schema/test corpora, U context/state/units schemas and fixtures, capacity schema/laws and existing ticket wrapper selectors only if new roots are required. No O/B/graph/W production edits. Compiler lane confirms no active wrapper/capacity/U mutations; mesh coordination is pending. Unknown peer protected-parent work stays attributed separately; no restoration of consolidated deleted schemas. This is a proposed touch set, not a claim all paths have changed.

## Census Source Identities

These are design observation identities, not a test freeze or transitive Source closure.

| Path | SHA-256 | Bytes |
|---|---|---:|
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts` | `498b32ba544f57da8d9c8504948fa02ac8210eb420d2ff4b3992e6aa659ed38b` | 66021 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json` | `60bfde9f1c4a56927baa6e945070765bd520d59cdc50a138c044c0e93ad3f2db` | 23515 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🟦️.ts` | `6f0aef41db71dc1b59f2a4482cdb12136fca5da6a0ab48248feba29a39842da7` | 7372 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧾️claim/🟦️.ts` | `c3ea11bf2e37fff61c82fedee9cd5e9294274f70a280c61d3c3cfc786fde5a7b` | 22088 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` | `e61887c142ac5a761477f08db9da219a385e7b75f77e06bc013d75dc97dfcadb` | 108601 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` | `bc4a86c2ba90f41c2868ce878bec74b4fd9142bd09757cb72c64e00d55863749` | 136569 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts` | `e6999b3ca79a8bb0f8d40fad91c0ba4974ceb35073c6704de27a572ce8b88caa` | 22429 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts` | `cfac6fd2b87e8c2069c6693b537d0dca9b1771585a13b289dcbe08a48da31b67` | 11056 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📋️state/🟦️.ts` | `76a19d582462cbd5275eb766ed6f909cc1b147008eeaabc779e402061a7ebe47` | 15362 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts` | `aca5285e822f91b99d01101e1905acb35eeac9cea7c945a57f995fd5b824328e` | 25952 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📋️model/🟦️.ts` | `b207d1ce4637de204007e79145dfa70800811fc8f1d7ee35291fe268e3a4f71c` | 9564 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/⚙️policy/🟦️.ts` | `e4c4302f2f3aae03933f893a2087190a67192522c1415ddcc06d8bb55d822e06` | 3759 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧬️schema/🔣️.json` | `000756a89bc2fb272eb21f856068c3c6ec0c9b846482171a58b141ae71fdac50` | 24356 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📍️layout/🟦️.ts` | `3194007395883e038ec49770913f693c68cde880ae1959a0dcba86da28f4c535` | 2257 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🏗️producer/🟦️.ts` | `24b93aa95b8a2b2e332765726d32162a59265bd0db2b4de36d0c11dadcb3a234` | 3620 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🏗️producer/🧬️schema/🔣️.json` | `5b2cbfbf9df41c17b27b16d26578896058addff3f9728d70fea78128bae21e4c` | 1325 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` | `34a574528ab03fcddfd5e6fe08873381ef3cf6bfcc93fdb711b30ed2641da500` | 181439 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📜️script.ts` | `feb09094b9623605860cd03f0012e5c881d092fd73921abdf2881f16418ebc67` | 11779 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts` | `e5703a483ed3824d07970301b0fefb6296bdc2c9e36f9a396925c78f9cdcefcf` | 11706 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧬️schema/🔣️.json` | `6f6d453eb0468b804cd6dd8593bdf722f1f78e4013df1b265f89cb7c3308032f` | 12040 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🟦️.ts` | `999ebb75bb6cea58e8fe78b55cc8cc5e3d0d166c16398b52484d9c47f8091448` | 2578 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧬️schema/🔣️.json` | `f8dd75fd72bcf0cc67d1cfda9758c8b038868d35bd78a87a79cfede09f9c8900` | 17384 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🟦️.ts` | `945f297a8f669cc31981c7935acb7ed83ec9cd2cf9ce30b042ab1a20172a2947` | 6825 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📍️layout/🟦️.ts` | `eee4a258ad34ec146088af390efb03f4f0e654af1099169ec1071695686184f9` | 1912 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/⚙️policy/🟦️.ts` | `94332ca35a21aca497306bd5b5f44d5d92e946cb172f3378a0d5c51eaf73697c` | 1497 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🔣️policy.json` | `e4225e951f0017352d472a190cc3a10c165d743b2c14fc226587a01e595f027c` | 828 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts` | `acc0c101e5fcae6cc7e2a09dd96fd7714d4c74e923da02041675cd079ae715f1` | 11216 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` | `f1b7f111a123576ff68849fe7c95330869313752e8405116e130699b4f860a7e` | 13109 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts` | `7741359351364a92a1437eff25b85dadcf1b5222fe639784dc0318ef0b491ac0` | 8519 |
