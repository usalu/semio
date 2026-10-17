# Bounded Root-Family Directory Path — Implementation Checkpoint

The existing neutral descriptor module now exposes `ArtifactRelativePathEntryV1 {name, identity: ArtifactRelativeEntryV1}` and `ArtifactRelativeDirectoryV1.withDirectoryPath(path, body)` on both existing platform branches. It accepts1..64 steps, at most16KiB aggregate UTF-8 names, a dense closed array of closed data step records, and only exact issued immutable directory entries from this receiving root family. Empty routes use the root's direct methods instead and are refused by this nonempty-path operation.

## Current Authority and Lifetime

Each private entry proof now records the exact actual parent facts when that entry was observed. The first entry must belong to the receiving frame; each later proof-parent record must match the previous issued directory record. Windows checks the full private128-bit ID, volume, security and stream facts, in addition to the seven public facts. Serialized, copied and previous-root entries never supply this proof. Already closed intermediate observation frames provide immutable comparison DATA only; the actual live root receiver and a fresh parent-relative open remain required. No Native image, reservation, store or Core authority is issued or upgraded.

Proxy arrays and step objects are rejected with the existing system `node:util.types.isProxy` before any property or reflection operation; the actual Proxy counterlaws observe zero traps. Copied or proxied identity objects fail the private proof lookup.

A private handoff reserves existing shared custody before opening, registers the exact returned object before another checkpoint, verifies both actual parent and child, and closes the previous intermediate before advancing. A bounded event-loop yield occurs every8 verified handoffs; the next backend checkpoint (including before the final POSIX body) enforces the same genuine cancellation and remaining budget. Root+current+next is the fixed handle bound; the32-owner policy is unchanged. Accepted final callbacks and descendants drain before their real final owner closes. A verified known handle closes exactly once on cancellation or verification failure. A throw during opaque acquisition or an ambiguous close keeps its exact uncertainty and never yields success. Arbitrary undefined/frozen primaries remain first alongside distinct close failures. No generic privileged callback or administrative import was added.

Existing direct file/directory/entry/remove/sync routes remain semantically unchanged. The POSIX directory callback body was extracted to reuse it with the final actual descriptor, and both entry registries received private parent records. The explicit facade adds only the new first-party type. The generic filesystem root constructor remains ordinary, and the Windows mapped Native reparse-root and actual NTFS qualification gaps remain separate.

## Executed Tests

Final `stdio-phase/📜️script.ts --relative-strict`, session41149, `🗑️generated/relative-path-thirtysix-strict-final.log`, terminal0:

- strict3explicit roots, zero diagnostics;
- POSIX1test266assertions0fail, including actual Generated depth33 traversal/read/removal/sync, matching all35 directory/file observations with independent Python `os.open(...,dir_fd=...)`, `fstat` and SHA256;
- Windows4tests193063assertions0fail: original60 ABI/custody cases plus36 new exact-production-body path cases, including depth64 and private-ID/parent/lineage/copy changes, wrong names, closed array extras/getters/holes, unawaited descendants, arbitrary failure plus ambiguous close, cancellation, finite expiry and opaque acquisition expiry;
- actual existing recursive POSIX depth33 fails at the unchanged32 shared-owner cap and fully closes; the new path method succeeds with at most4 total shared owners (library+root+current+next);
- the exact Windows base recursive method likewise refuses at32, while depth64 handoff uses at most4 actual model handles, with libraries separately tracked by the same shared registry;
- Ajv validates every portable case, SQLite independently checks allowed outcomes/effects/closure/handle bounds, and the base ABI packet/ctypes/Node oracles are retained.

The Windows port is explicitly a Source NT syscall model. It observes buffers, access masks, sharing, private identities, real issued controls and closure order; it does not establish Windows runtime, NTFS deletion/flush durability, or mapped Native-root support. Actual filesystem operations occurred only inside owned Generated POSIX fixtures. Every generated absolute fixture path was admitted through the256-character first-party guard. No protected/Native operation was performed.

The existing Source helper now passes Bun's per-test timeout as the actual remaining issued Original budget. The outer Original remains60s, and the owned-child deadline and exact signal remain authoritative. No Original, Native, filesystem, read or descriptor cap increased. Root and Windows suites still run in separate owned Bun processes and receive Original once each.

## TDD and Failure Attribution

Schema `RelativePathProtocol`, `RelativePathCase`, and `RelativePathCases` preceded implementation. The first missing-API check was authored first, but its gate `relative-path-api-red.log` stopped at the unrelated existing POSIX5s clang-test timer before reaching Windows; it is a harness failure, not semantic RED evidence. The actual recursive capacity counterexamples execute the existing production method with the new schema vectors and demonstrate the concrete32-versus33/64 defect.

`relative-path-yield-red.log` is semantic RED: an already queued genuine cancellation at depth64 was not observed before the old nonyielding body returned (expected refusal/received success). The fixed8-handoff yield exposes the cancellation and closes all known handles. `relative-path-proxy-red.log` is semantic RED: a Proxy trap ran and its exact arbitrary marker escaped instead of refusing the closed input without invocation. Both are GREEN in final36.

`relative-path-closed-array-red.log` is semantic RED: an extra array property was accepted when the schema/closed protocol requires refusal (expectedfalse/receivedtrue). The corrected snapshot reads own data descriptors, refuses holes/getters/extras before entry/IO admission, and does not invoke supplied getters.

`relative-path-owner-first.log` was an omitted test callback annotation caught by strict, not a production failure. `relative-path-owner-second.log` and `relative-path-physical-first.log` originally demanded known closure after the Source expiry injection itself threw inside opaque `NtCreateFile` after acquiring a model handle but before returning/writing it. The production correctly retained that unknown acquisition plus its3 DLL scopes. This was a faulty test expectation, now split deterministically: a genuine1ms descendant phase created after acquisition either expires and returns the actual handle before the next checkpoint (all known handles close), or throws within the opaque port (uncertainty must remain). No guard was weakened, no fake control/Original was introduced, and the prior natural GREEN was not treated as resolution until this cause was established.

`relative-path-thirtysix-final.log` stopped at an unrelated concurrently edited Native fixed module readonly-container TS2339; Root corrected and qualified that container separately before final41149. No filesystem/test diagnostic occurred in that failed transitive gate.

Fresh domain commands must still recapture path entries within the new root-family callback and compare their command DATA through the authentic Core owner. Parent or directory changes caused by earlier owned removals require fresh current entries for later path walks. Holding a final directory for a bounded set of operations avoids stale path proofs. The filesystem operation itself never performs journal/charge changes.

## Point-in-Time Seven-File Manifest

| File | Bytes | SHA256 |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` | 59455 | `39023189613f9eef7656a568d321c421780da82c4ecb44bdb70609aa98e38bac` |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json` | 25311 | `c1bc81c5465c0785d900219f86f61544cecffd340b0a4a9ec6d6715b2f5de348` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | 105980 | `9de6e0f8f32454da2ef9a3b8e778b7d0f53c4c513df9e3bf553f819f828d0be3` |
| `🧰️framework/📦️packages/🟦️typescript/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` | 675 | `3b1c619e8c2ad485a0bad901677734020ef5704176961ef6eda0a8e21e912e30` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/stdio-phase/📜️script.ts` | 8587 | `9e5da1594574a79c05596f078b984d26134794a1e7c470aeb6a1f93b6d617ab9` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/windows-relative/🔣️.json` | 620 | `fa0e9fe02fc0488a236a59f3ab7aa255d78d7ce67b559cbe0e59a89da6da52d1` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/windows-relative/🧬️schema.json` | 1043 | `369d62ad95643b75029442ef7e81fb5657b4c1211e55af32ef01ac9c96a19fc9` |
