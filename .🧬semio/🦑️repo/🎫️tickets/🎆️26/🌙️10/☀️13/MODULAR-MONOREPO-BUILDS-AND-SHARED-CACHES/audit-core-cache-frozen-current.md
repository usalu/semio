# Frozen Core Cache Source Custody Audit

2026-10-16. Read-only Source audit of the frozen extraction; one concrete blocker found. No test, Native image, compiler, effect cohort, retirement, fake provider, Original/control/budget creation, git mutation or Source edit was executed. Only this audit Markdown is authored. Existing ticket ownership remains with Root.

## Blocker: Actual NUL Descriptor Output Is Not Parsed

Core `📦️artifacts/💾️cache/💾️volume/🟦️.ts` maintenance invokes `lsof -nP -F0pcfatn`. Its actual shared `📂️descriptors/🟦️.ts` parser iterates `observation.stdout.split("\n")`, matches PID only with `/^p[0-9]+$/`, and recognizes a name only when the whole newline-delimited row starts with `n`.

The installed authoritative `/usr/share/man/man8/lsof.8` lines 3186–3212 and 3309–3336 specify that `0` selects NUL field termination, with newline only after each process/file set. Thus a normal process set is `p72\0cproducer\0\n`, and a normal file set is `f3\0aw\0tREG\0n/root/i/0/output\0\n`. Neither row matches the parser's PID/name branches. `captureOwnedCommandResultV1` in `🎛️owned-execution/📥️capture/🟦️.ts` preserves these NUL bytes through strict UTF-8 decoding and string append; there is no intermediate conversion to newline fields.

With the actual cache guard `{pid:process.pid,paths:[]}`, ignored foreign file rows leave `seen` empty and the final required-path test vacuous. A successful lsof exit with empty stderr therefore permits pressure removal/final cleanup despite foreign open descriptors. This invalidates the report's complete rooted descriptor-exclusion claim. Existing descriptor tests author newline-separated `p`, `f`, `n` fields, so they do not cover the actual `-F0` wire format. This is a Source relationship finding, not an executed reproduction and not a newly introduced regression claim. Correct the actual parser/caller protocol and add language-neutral NUL process/file-set cases including foreign descriptors, malformed/truncated records and escaped paths before physical pressure qualification.

## Coherent Ownership Review

The canonical platform opener snapshots options before await, freezes the same actual backend image and port, and enrolls only that actual result in its private image map. The retained helper authenticates captured image/root/envelope/method references and exact private control refinement, then synchronously invokes the captured real port operation. Copied objects have no key. Closed readiness and accepted descendant drainage delegate to the actual Native borrower owner; a private map entry alone is not a live-image proof.

The budget factory alone maps its actual frozen partition wrapper to the real canonical image and retained lease dependency. The cache bridge uses that private real image. Its promise is added to owner/frame sets before invoking the requested child; nested frames carry the selected actual control. Close refuses its own live frame, closes top births, drains pending work, then attempts image close before lease close. Ordered outcomes and explicit failure bits preserve thrown undefined; admission cleanup attempts both resources and preserves the admission error first.

Both compiler base and final toolchain-bearing frozen objects are mapped in their actual factories to the same private retained owner. The cache entry checks exact store/storage/control/invocation/root and live owner, requires compiler workspace/writer custody, captures policy/key/content before deferred writer work and enters the closed partition operation. It reconstructs no image from root DATA. Copied/foreign/after-lifetime handles refuse by membership and owner/control checks; frozen factory objects prevent public-field mutation rather than relying on a structural alias. Actual successful physical interleavings remain unqualified.

The allocator's separate private policy map captures exact issuance control and immutable policy in `heldReservationV1`. The new read-only accessor first applies the existing checker, then requires exact control identity. Core compares complete parent policy key/value facts against that issued policy, requires initial floor equal actual maximum workspace, checks parent/allocation root and envelope, enters the real Native parent operation and awaits supplied allocation.recheck before lifecycle effects. The signal-only checker was not silently promoted into an exact-control proof.

Core receiver promises are enrolled before invoking work; own-frame close refuses, replacement overlaps refuse synchronously, finite accepted count is bounded, and outcomes retain admission order including undefined. Core keeps the borrowed parent outside its owned child-close set. Child receivers are tracked immediately after opening, including admission failure paths, and all owned closes are attempted before leaving the parent Native operation. Callback and accepted failure paths retain dirty state and cannot reach the success-only clean commit. Body failure remains first when closure also fails.

The actual canonical body phase reaches Core workspace facts, opaque Rust loan and Rust operation checks. No sampled timeout or root-control replacement appears in these reviewed cache owners. Rust retains repository/prototype/environment/plan/context/content/cwd/writer custody. The public cache token/facts expose no Native image/port/maintenance/close callback.

## Funding, Census, and Freeze Limits

Production literal policy is unchanged: 4,294,967,296-byte child charge = 4,227,858,432 backing + 67,108,864 parent metadata; 4,226,809,856 filesystem + 1 MiB = backing; eight slots; 1,073,741,824-byte pressure floor; 4,294,967,296-byte initial parent floor. First creation requires charge + floor = 8,589,934,592 available bytes. Candidate recovery requires exact durable binding/intent/allocation, rejects ambiguity and unbound owner-name candidates, and retains retiring/unknown charges. Logical journal funding remains distinct from actual physical creation/free-space checks.

Inventory is bounded, no-follow, same-device, complete hardlink-group, EOF/chunk checked and fingerprinted; slot pressure uses allocated bytes, deterministic clean-prefix planning and exact retiring dev/ino tombstones. Fresh recensus and nonblocking Native EX predicates are present. The descriptor blocker above prevents accepting that these collectively establish safe physical pressure.

I read the actual macOS persistent constructor (`createPersistentImage`) and shared Linux/Windows fixed receiver constructor, plus canonical forwarders/census. Public image/port members are object-literal data/methods; lifecycle mutation is in private receiver/closure state. The preserved census has 427 files and four public-write-pattern matches, all ordinary SDK test FileHandle.close instrumentation. This bounded pattern census and Source inspection support freezing the same object; they are not exhaustive dynamic or cross-platform runtime proof. Constructor hashes below were independently rechecked.

All five report-listed old generic implementation paths are absent; bounded old-path import search over framework and ticket build-mode/build-maintenance found no references to state/model, state/inventory, state/volume or old pressure/descriptors paths. Core implementation imports no repository owner. No blanket repository-wide semantic duplication proof is claimed.

## Design-Only Successor and Evidence Boundary

The 64 MiB charge / 48 MiB backing / 47 MiB filesystem / 16 MiB metadata-floor / two-slot / 4 MiB recovery consumer is distinct policy DATA only. Proposed 128 MiB parent + 48 MiB child backing requires a separate finite quote (176 MiB known backing, suggested 192 MiB scratch), a genuine 16 MiB consumer allocation and at least 112 MiB actual parent free for child creation. It does not fit or extend earlier 128 MiB total fixture authority.

Whole-cache teardown remains unimplemented DESIGN ONLY. Proposed bound -> retiring-content -> retiring-image -> retired sequencing, exact receipt/backing authentication, real descendants drainage/EX/FD/census, inner retirement and canonical backing retirement before parent-charge acknowledgement need their own implementation and review. Dirty-slot pressure is not whole-cache authority. No physical operation is authorized by this report.

Reported BOTH 30 tests / 371 assertions are Storage's retained evidence, not tests executed by this auditor. The two final strict logs remain reported failures at moving peer roots; no successor pass is inferred. No unrelated CPU/Nx lane was awaited. This audit blocks physical pressure qualification on the descriptor protocol defect; otherwise it asserts only the Source relationships and exact hashes listed here.

## Exact Frozen Roster Hash Recheck

Report SHA256: `49a419a45b7ad36fb78e00118cc06059230bf3d34d1b4aed6ea2549e845f3ddf`. Every one of its 26 existing Source/script roster entries matched at this read. The complete checked manifest follows; tests/schema entries are hash checked and selected laws inspected, not a claim of exhaustive independent review of every assertion.

| Path | Actual SHA256 | Frozen Match |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/⚙️policy/🟦️.ts` | `a9d019290af89fae77ab9197ccfa47792e53320606ae719867cd7dc8a12aed0f` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts` | `858aad39aff33ff87559b7fc3c8bf75e34cf6a08fccf72b2a615817247be904d` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts` | `3fb031bd6043e5d61e07ff03ecadf13f9bb41519694d8397d51f43c3d4757660` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📋️model/📨️receiver/🟦️.ts` | `381445e40e0cdfc8593aa6861fc1e74fda87a40759920dfb77cb0d4bea2b3bf0` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📋️model/🟦️.ts` | `b207d1ce4637de204007e79145dfa70800811fc8f1d7ee35291fe268e3a4f71c` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📍️layout/🟦️.ts` | `8ab38c5b6278d12e73f32e218d80e5a1d44e310875361b697fe779325aaf292b` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📏️inventory/🟦️.ts` | `0b3508cf2da2777d6a3cd547c1d42de5007735669edb80497caac1ced97cc3f5` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🟦️.ts` | `d62d4812b9ef5a1590fcf883252afcb17e10f35d026eeaf459632370e1f88e83` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🔣️.json` | `f1ccf2f4eafd808827f4cb5728962498312ad0d7a8dbb5959a3182dc8e9d8641` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | `f4af5cda030497229d63ed52a7b7fa207276b829a1cadbcefdb8493e59c1363e` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧬️schema/🔣️.json` | `35a61ad6e719034792ccaa34f408701df85025b06d527e73042ccdde799525b2` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts` | `2826304c1b8ba5c92b3323d4cae955bb893f5b1609b723123b33cfd2dab9da45` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts` | `6b02ba012f0e6ff9367ae52c8a945faae77bcaf724ecbcc4d28be64e04cbbd68` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts` | `3f371803307c563169f7e9227140e19dea5df86764d6fd5540e0bd33ee2ba2d1` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` | `9286dd571434492905bd29e313c896f00ca57080c7e28f8786c9eb4558e6dec2` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🟦️.ts` | `bd762245b3fe0ceef54bc30bd220bfbf52ce67a7733de0294f7e91c578c11840` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/⚙️policy/🟦️.ts` | `94332ca35a21aca497306bd5b5f44d5d92e946cb172f3378a0d5c51eaf73697c` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📍️layout/🟦️.ts` | `fa5a86b0ee8853977c0f95096818cb94d4929ab28a1c192c9a6a24218493f3ac` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🔣️policy.json` | `e4225e951f0017352d472a190cc3a10c165d743b2c14fc226587a01e595f027c` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧬️schema/🔣️.json` | `075f6c9fd47d6671abc64a3e3ba36247c56725826220d8983dd371e63cd2ba8d` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🟦️.ts` | `673c37fec32f18aa9a93e12bea95d68e07c26c48a52adc2f97a7786ab9bca17b` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🔣️.json` | `a86cd0c9b36e85b5d669a951326227e1d96f433323adcfb6fb0bbb36c9eba599` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts` | `86a056c785135f92bcb45a0b8dddb8c37d109af079958bc27cfd904c1a129e98` | yes |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/🧪️tests/🟦️.ts` | `7f4df8ed88074ca373c74677ddbd868eca5363284a18d34643614da3bd3cf436` | yes |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-maintenance/📜️script.ts` | `954a90ebd6d3d78648e3da6d0468e520a8ab6c67cb7644b4a3c919d13ed40d65` | yes |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts` | `538ca1c93fd18dfdee586d778b3167990dff147fd09729950ed45f2901ddb50e` | yes |

## Additional Constructor Hash Recheck

| Path | Actual SHA256 | Census Match |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` | `3854269ba9896f7018b449b6f43ea58769965db475db74ac8f214ce2ab6cb9c2` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` | `b6e6e918bbc59399587bdaed43595ed757f1f1c9dd3694263b0eed52293cada2` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts` | `7bf592a85980bd55d4a4a5c4dcfb7309e856cf35f72ebea88b3076de1e0c78fb` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🐧️image/🟦️.ts` | `73eeac87fb10da753b10a654fb5c62622900706996ad1ebe49858f6f34d3fa7b` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️image/🟦️.ts` | `2fdbd5642a5c5fb9e0cbb74469e3436b505b93473d57c19d372ad72eeebd8309` | yes |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts` | `2826304c1b8ba5c92b3323d4cae955bb893f5b1609b723123b33cfd2dab9da45` | yes |

## Exact Counterexample and Shared Deletion Consumers

Neutral observation: status 0, signal null, reason `exit`, stderr empty; directory `/root`; guard `{pid:71,paths:[]}`; stdout literal `p72\0cproducer\0\nf3\0aw\0tREG\0n/root/i/0/output\0\n` (each `\0` denotes one NUL byte). The foreign PID 72 owns a rooted file. Source branch evaluation ignores both sets and admits. This counterexample was reasoned directly from Source and the installed man page; it was not executed.

Exact references: Core descriptor owner lines 6–11 (especially line 9); Core volume line 57 (actual `-F0pcfatn`, empty allowed guard paths, maintenance precedes mutation); owned-execution capture lines 39–42 (UTF-8 stream decode/append).

The same descriptor parser has three actual deletion-owner uses:

- Core `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts:57`: NUL format `-F0pcfatn`; affected blocker.
- Repository `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts:50`: newline format `-Fpn`, checking root and relatedDirectories with exact real guard paths. The NUL mismatch is not asserted for this caller.
- Ticket `build-maintenance/📜️script.ts:66–68`: newline format `-Fpn`, checking buildDirectory and actual profile-lock paths. The NUL mismatch is not asserted for this caller.

A shared parser correction must preserve both real newline callers' rooted path/PID/guard-coverage behavior as well as admit only the actual NUL protocol requested by Core.
