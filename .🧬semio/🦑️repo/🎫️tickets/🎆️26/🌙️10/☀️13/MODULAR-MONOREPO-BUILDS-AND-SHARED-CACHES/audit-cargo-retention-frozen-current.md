# Frozen Cargo Retention Source Audit

Read-only Source audit, 2026-10-16. No compiler, Native, cache, provider, Original issuance, test, or product run was performed. Only this report was written. Implementation findings below are not runtime reproduction claims.

## Findings

**P1 — Original directory and absent-input observations are not retained for final rechecks.** `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts:378–380` creates a separate `CurrentPhysicalOwnerV1` for each invocation, but the shared session stores only its `CurrentPhysicalFilesV1`. `settle` checks `owner.recheckStructure()` once. The write settles at line 759, then retains the `physical` port at line 761. Session close at 771–795 only calls `recheckCompletedFiles` and `physical.checkpoint`; completed snapshot callbacks at 434–435 also call only `recheckCompletedFiles`. The latter, at 429–430, checks shared regular-file claims and retained output copies, not directory/absent rows. `CurrentPhysicalOwnerV1` keeps directory and absent observations in private maps (`🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts:50,79–80`), outside the shared file map. Therefore creating a previously absent `.cargo/config` or changing an observed resource directory after `lease.settle()` and before session close/final consumer recheck is not covered by these checks. The receipt roster preserves these rows as DATA, but Native's completed view never re-observes them. Preserve and recheck invocation structural owners, or independently recheck every current directory/absent receipt row during close and every completed-view recheck. This is a concrete Source omission; no physical mutation was attempted.

**P2 — Native local disposal does not await all concurrent unit-graph requests.** `…/🧩️component-pass/🟦️.ts:509–521` starts all graph/input promises outside `pending`; line 677 uses fail-fast `Promise.all`. On one graph rejection, lines 693–695 enter `disposeDriver` (506), which terminates known children and drains `pending` only, then discards the provenance session and closes resources. Sibling graph promises can still be inside input admission, awaiting child close, or writing graph logs. The actual repository registry scope additionally drains admitted nested operations (`…/📥️dependencies/🟢️ready/💾️storage/🟦️.ts:28–35`), so this is not evidence that actual OS registry SH exits before those admitted operations settle. It remains a local ordering gap, especially for supported input-free Core plans without that repository scope; join every started graph promise before local disposal. No descendant-death or timing behavior was tested.

## Checked Source Boundaries

The inspected implementation requires the domain transaction when `withCargoInputs` exists (`component-pass:373`), completes job-only prerequisites before graph creation (`720–724`), and wraps planning, pass execution, consumer and continuation drain in `withPassInputs`. Actual OS wiring supplies that port (`driver-stages:68`). Registry nested pass admission borrows a private matching live scope before EX (`ready/storage:95`); ordinary fetch uses EX before final SH (`97–101`). Session writes are tracked and settled before close (`native-build:573,776`), full file snapshots include current files (`retention/snapshot:25–27`), eligible artifact transfers check actual shared claims and retained copies (`native-build:421–422`), and cross-pass current claims delete/prevent transfers. These checks do not repair the structural observation omission above.

Emitter token ownership spans actual compiler storage and product reader (`emitter:40–41`); each tool task acquires the read-only current configuration/registry reader (`44–45`). Its current-row loop plus physical recheck covers directory/absent rows in that reconstructed reader. Fresh component build is outside the tool task and final consume enters it (`fresh-component:345`); the actual model control is passed through the fresh lease consumer (`254–268,350`). DATA roster decoding is not treated here as authority or evidence of fresh runtime success.

## Exact Source Snapshot

All 27 primary roster hashes matched at audit start. The table below is the final recheck, identifying the exact scope; no moved rows were silently treated as frozen. Additional structural-owner source hash follows it. No existing test result was rerun or relabeled.

| File | Bytes | SHA-256 | Checkpoint |
|---|---:|---|---|
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` | 85110 | `bc62f7dc4ac622e580dfaec400e84cfb0fb28016d99bea0b512fe2cf767353cd` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` | 79680 | `8926d59d7ab15214a55477abfdbbc7419847937938ac8b44d84cd2a3eb85ce39` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📋️prerequisites/🟦️.ts` | 1290 | `0029c4f688affe2a30a22c46711d2c9ac64c298a3e16a6fe7275d1cea075e058` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📋️prerequisites/🧬️schema/🔣️.json` | 647 | `068dcf8cc8775ebf35e5e67d6b3660e5ec713a06172fa37d771bee380632f466` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🤲️continuations/🟦️.ts` | 3375 | `8c7b4071c73118c6f6fab56dd12344d4dc66835596148f8fd5520ddc975b3ede` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🤲️continuations/🧬️schema/🔣️.json` | 83 | `bd1d6f854447ec3db13d379e4dde25f3ebc6854d3ef552a9932c048691e48e0b` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📁️copy/🟦️.ts` | 6081 | `8f059423eb63d2faf51f37fd05c22e397caee3de282638a32387ad0ec9e4d70c` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📋️inventory/🟦️.ts` | 7635 | `4b14ea311033721f2a81435d3213b5e072cc1857c2f96802d27331a9aa108846` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📥️reading/🟦️.ts` | 3562 | `997458956e37e78597ee1a96c2f315081640928dfc42152af5629286cfeb6906` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📥️snapshot/🟦️.ts` | 11606 | `4c2fbf72f4a5722151fb14cfae00df9944b5526a2311024d4c225aa296efd3aa` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🟦️.ts` | 193 | `1c6fb8d4257dfbab775c5a17b894a120b94ded57fadc3f0ff6729d7630065b2b` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🔣️.json` | 2112 | `15c59ba982fcd6d4ffa3e3178ef5af7565c27a8a56420323a7acde3061f289e3` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts` | 28389 | `ef37b9111d953b92296e80d11a4d645e97d14ed6196710c6abb2df570e73ac49` | MATCH |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧬️schema/🔣️.json` | 1814 | `bc23f9ae1da2848e8c97e5119ad1aa89634d808a5e242c5410f7d250ee3df411` | MATCH |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` | 18543 | `67cea6f70ed342d14380d23ec70f54f7706e0e4531ba3be1b49226cb2d02693e` | MATCH |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts` | 20425 | `08d7d056b2549cbc5e2facd048e5dfeb8ef1590751c204bbec49560b938a1fca` | MATCH |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/📋️selection/🟦️.ts` | 843 | `45a6e60a5427c91b27b300c81045a473742375b57a7b9f488510e0557c570351` | MATCH |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/📋️selection/🧬️schema/🔣️.json` | 427 | `20177e87218f946a147e7c4de5c28ace551fce9fac00c011049e2eaf69d21a57` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` | 17547 | `d842d94e517bdefc9da4d9b516b5374ca8e5a0c3b3aa109b36bc0b0d123fb265` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts` | 34694 | `0cea112c92997c7d7adf170164e09e7a37c3900df1b87dfd9e0f00a23513f246` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts` | 17511 | `69e334a5e24e0cf742c8d10f09bff55f9582ab8562dc25e5afaf0447a946dfab` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts` | 12484 | `4e250ae288f34447b926afcf408a99ce6ff5cc844a32a2f0c7633bd5c9a436f0` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/📋️contract/🟦️.ts` | 2918 | `a9dc40b026a997e98ead2dc64fadba9586f9cfe6395896358cd34aad6abb67d2` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🧬️schema/🔣️.json` | 448 | `84d6a40698d893a02efd096ce37867808a3744397e5eb4882b54e511cee7f271` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts` | 46351 | `c13b759d7cb98bbbe4b1858d484f34753fb3e66ab9d6d979651ce5821c7c40a8` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧬️schema/🔣️.json` | 6227 | `21416888a0284d686b2d8076d738241373bd1d5b531bfc4afcb527c349d5ca9b` | MATCH |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧪️tests/🟦️.ts` | 17608 | `6ef49e09d48f539dc1088cd08fc5c99e368c50ac9ad28b9c05970bd01f1d3bcc` | MATCH |

Additional inspected owner: `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts` — 14404 bytes, SHA-256 `b3fa92065908f3730654ae321cea611caeff3e95275da82a3856294d22141470`.
