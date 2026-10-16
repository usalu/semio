# Selected Task Bounded Metadata Reader

2026-10-16. Actual normal-file metadata reader/neutral checkpoint. No Native image, cache write/retirement, SDK activation, Nx build/publication or compiler ran.

The selected-task scope now reads both nx.json and exact managed Graph source-closure records through an asynchronous bounded descriptor. Admission requires finite extent before allocation, ordinary one-link non-symlink file, descriptor/path dev/inode/extent/link/times agreement, real stable ancestry, exact EOF and growth refusal. Reads use64KiB chunks. Caller AbortSignal is checked around awaits, after descriptor close and before final scope return. Errors and descriptor-close failure are retained together rather than losing thrown undefined. Windows uses available no-follow flags plus path/descriptor observations; arbitrary same-user atomicity and platform runtime are not claimed.

A private captured schema/primitive policy bounds each closure16MiB, distinct aggregate64MiB,2048 distinct records,65536files per closure, Nx configuration1MiB. Repeated digests share immutable admitted file DATA. Full Source path/unique/closed-object contract is validated before scope acceptance. These are metadata consumer bounds, not cache filesystem quotas, reachability, retirement or new control authority. The existing optional-signal public API and complete paid Graph/SDK callback lifetime remain separate; this change issues no control and makes no full lifetime claim.

A genuine actual-reader RED showed a17MiB whitespace-padded valid JSON record was accepted. Successors also refuse actual hardlinked records and Source paths beyond the canonical256-codepoint contract; managed/missing/old-poison/managed-poison tests remain. Actual public JSON policy mutation cannot change the private reader. An actual pre-aborted AbortSignal preserves its exact primary before IO or planner effects. Independent Ajv2020 and SQLite reproduce contract/extent/link decisions and selected file rows. Every exact ordinary fixture is under ticket generated and removed in finally; no Source is copied there.

Current tests are BOTH4/36 and owned strict BOTH exit0, all handles terminal. The final language-neutral corpus carries hostile extent/link/path vectors and reader-limit/cancellation expectations; independent SQLite reproduces the declared decisions. Earlier import-path and test static-literal diagnostics are retained honestly; they are not semantic REDs. The final static repair annotates the actual assertion input unknown without changing runtime expectations. No hostile mid-read mutation/late-cancel/descriptor-close-failure runtime schedule or real managed image was exercised.

## Evidence

| File | Result | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| [selected-scope-bounds-red1-prototype.log](🗑️generated/selected-scope-bounds-red1-prototype.log) | genuine over-extent RED:1pass/1fail/27asserts | 3622 | `5d42ccb04c78865d2efcaa5c74678dbe84b459d42db2f0bd52d77456b75c54ff` |
| [selected-scope-bounds-green1-prototype.log](🗑️generated/selected-scope-bounds-green1-prototype.log) | implementation relative-import defect, not semantic RED | 2597 | `74ffa18441699ee9aa8b6fbddd2ef45b1bfc0e7beca8b24614f5b756f3efa9f5` |
| [selected-scope-bounds-green4-prototype.log](🗑️generated/selected-scope-bounds-green4-prototype.log) | exit0;4tests/36asserts | 1167 | `9ff82ffb4ee7032299e6424ec39305c0fb56bc517615e5ac0e373fd9254ff643` |
| [selected-scope-bounds-green4-release.log](🗑️generated/selected-scope-bounds-green4-release.log) | exit0;4tests/36asserts | 1169 | `13000299515b814fe188a18721eaf407396d53fa869914deb33e9fc35534ce97` |
| [selected-scope-bounds-types3-prototype.log](🗑️generated/selected-scope-bounds-types3-prototype.log) | strict failed only test JSON literal widening | 2653 | `77d85b1f8df7d0ef66c36adc701766f6dcb578fa19112ed05d042238c862d0c7` |
| [selected-scope-bounds-types5-prototype.log](🗑️generated/selected-scope-bounds-types5-prototype.log) | strict exit0 | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [selected-scope-bounds-types5-release.log](🗑️generated/selected-scope-bounds-types5-release.log) | strict exit0 | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

## Source Roster

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🟦️.ts` — `d9a8042f22c4f802470277b40122535acd28b223934e56bcda4bc086f4c5030f`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧬️schema/🔣️.json` — `85faaa95bf80afe3f4651d6008e7885a393b751cd8da09dc869c4b032f9e4bc5`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts` — `172cd4bae0253c1ae1e801a2f91f17ad3457ea518204f2dbb08cb9d74dfd40e9`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🔣️.json` — `a139a8e664500d1c7dd138f8c26a538c0106cbe83176da1b314aeba687ba0d37`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` — `5db19faacfb53d1ac62407e1493a53eb7ccc28d1eaa5e5a2e4e4c576c5882948`

## SDK Task Successor

The later Source successor and its independent corrective rereview are recorded in [Selected SDK Native Reader Task Successor](selected-sdk-native-task-current.md). Its combined reader/SDK/ledger corpus is BOTH7tests/60assertions plus BOTH owned strict exit0. Earlier source hashes and raw-port lifetime concerns above identify their pinned epoch; installed SDK/Native physical qualification and coupled mandatory closure retention remain separate.
