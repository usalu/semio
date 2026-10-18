# Shared Schema String Validation Checkpoint

The complete build/cache goal and ticket remain active. This changes the actual shared validator used by plan, receipt and build infrastructure; Source checks do not establish Native quota, Nx/compiler warm behavior or platform acceptance.

## Production Change

The prepared validator previously computed [...value].length for every string, including contracts without minLength/maxLength. That constructed an iterator and character array even for ordinary type-only or digest-pattern checks.

The actual compiler now prepares the two numeric character bounds once with the existing immutable schema snapshot. It counts characters only when a bound exists. A private constant-space UTF16 scan counts valid surrogate pairs as one and retains isolated units as one, matching the former JavaScript iteration behavior. Pattern checks, type decisions, all other keywords, fresh value observation and complete diagnostic order/text remain.

This adds no public production API, runtime dependency, value cache or new cache directory. No Root clock, schema/Plan/data cap, cache quota, compiler flag, source/receipt/task identity, reader, CPU or Native storage authority changes. Existing Rust string validation already counts scalar Unicode without constructing a character array; no Rust code or Native result is changed or claimed here.

## Schema-First Proofs And Actual Work Counter

The neutral corpus uses explicit UTF16 units and declares18vectors, including empty/NUL/BMP, combining characters, surrogate pairs, isolated/broken units, maximum scalar, ZWJ family, CRLF and a4096repeat/12288character case. Its closed schema precedes runtime changes. Independent Ajv checks exact/minimum/maximum decisions and three exact ordered diagnostics, including a nested path.

The proofs retain all62existing neutral cases and5comparisons, plus4096generated unit pairs across5prepared bounds, per-value mutation and all old prepared-snapshot/diagnostic/regex tests selected below. UTF16-specific vectors qualify the JavaScript value model; no unexecuted Rust equivalence is implied.

Source laws live in the validator domain and expose only a first-party immutable law interface (name and synchronous primitive assertion count) through the existing public testing surface. Node system assertions and existing Ajv stay inside testing. Bun wrappers belong to callers. The actual Rust binding tests invoke that same public surface. This avoids a Bun test-runner dependency in the testing API; no Native provider/grant is manufactured.

The red prototype root returns1/5.924s. Each internal/public caller records12288actual string-iterator births across4096values and3prepared contracts; other semantic proofs already pass. After the production change, every iterator counter is0. Type-only and pattern-only character reads are0; the bounded64ASCII case uses258048character reads without an array. Those are exact Source work counters, not proof of zero total allocation, causal wall-speed ratios or Native build performance.

Each law set executes33132Node assertions (portable359, generated20483, work12290). Focused tests run both internal/public paths, so66264Node assertions supplement Bun counts. Full selected Source runs the public path once. The work console prints12288before its two final counter assertions; the returned count includes those two. All4096/old corpus vectors remain.

## Validation

Prototype focused tests5.384s/strict3.749s and release4.899s/strict3.986s pass. Focused Bun totals10tests/317assertions across2files, plus a separate mandatory adapter10/215. The existing compiler/bundle/Node-qualification case is filtered under the explicit holds; it is not claimed to pass or removed.

| Planned Gate | Prototype | Release |
| --- | --- | --- |
| Focused schema Source tests | pass / 5.384 s | pass / 4.899 s |
| Focused strict types | pass / 3.749 s | pass / 3.986 s |
| Broad selected Source types | pass / 12.402 s | pass / 12.295 s |
| Complete selected Source cohort | pass / 50.136 s | pass / 50.377 s |
| Complete selected artifact Source cohort | pass / 28.340 s | pass / 18.156 s |

All ten planned roots are terminal and pass. Full selected Source passes172Bun tests/7763assertions across12files in each mode, plus33132Node proof assertions. It includes the unchanged complete4097Plan/4096edge binding frontier and all reader/current-reuse/lifetime laws. Artifact Source passes73/714 across6files in each mode, retaining27filtered cases. All six test commands also pass their separate adapter10/215. Focus retains its1filtered compiler/bundle qualification case. Every final log is valid UTF8, and exactly one terminal header appears per isolated file (2focus,12Source,6artifact). Actual statuses, byte hashes and counts are in generated/schema-string-final-verification.json. These are complete existing selected cohorts, not all monorepo tests.

Prior owned-binding release Source115.216s/late body-deadline failure and raw invalid-UTF8 log remain frozen failures. New changed-runtime passes do not heal that Original or establish the cause/settlement of old SIGKILL runs. No unchanged retry, larger clock/reserve/cap/frontier, fewer old cases or internal split is used. Native/default SDK/publication/compiler/cache-image/OS-inventory holds remain.

## Source Identity And Remaining Goal

The prospective-to-terminal fixed census contains1854Source/input paths, with all173owned identities and all10authored files stable. All1854observed identities remain unchanged; the same3historical missing paths remain visible. Maximum observed path is191UTF16units. Four new launch rows exactly match their saved definitions. Prototype focus retains its earlier loaded revision; release/broader gates follow the prospective census. Terminal details are in generated/schema-string-final-source-drift.json. This fixed roster is not a complete loaded execution/input closure, SDK/OS inventory or independent review; Source stability does not prove Native behavior.

Exact output-relevant cache-key/dispatch/consumption and Nx restore, per-crate/transitive/sibling invalidation, shared concurrent warm builds and resource use, actual prototype/release runtime/size, hard physical allocated-byte pressure/accounting/metadata/temp/headroom/history/retirement, full65536/65537inventory, crate growth, zero-touch platforms and predictable total repository footprint remain open. Existing sealed program/receipt/task correctness dependencies stay until their replacement is qualified.

The three workers are terminal with application network permission revoked and are not restarted or routed around. Ticket/goal MCP tools remain absent; the same ticket continues. All actual requirements remain; this Source checkpoint does not mark the goal/ticket complete or lift holds.

## Delivery Revision Qualification

After the unchanged terminal census, a later1854-pathdelivery census observes two shared changes:

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧬️schema/🔣️.json

The Native borrower runtime and cleanup-operation contract are substantive possible execution/input changes, not assumed harmless. All10authored files and four launch rows still match. Additional affected strict checks pass prototype12.393s/release12.618s, so all12terminal roots pass at their loaded revisions. Details are in generated/schema-string-delivery-source-drift.json.

The post-strict1854-pathcensus records six further changed identities:

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🟦️.ts

Borrower/fixed storage, routing runtime/shim/contract and build selection are substantive possible execution/input drift; none is treated as harmless. All10authored files remain stable. Post-strict hashes are in generated/schema-string-post-strict-source-drift.json. Each gate retains its actual loaded revision; no unchanged full current Native/execution closure or independent review is claimed. Earlier failed Originals are not reset or retried unchanged.
