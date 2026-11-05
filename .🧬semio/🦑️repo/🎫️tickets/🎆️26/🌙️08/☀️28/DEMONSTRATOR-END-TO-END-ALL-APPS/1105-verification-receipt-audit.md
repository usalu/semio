# Verification Receipt Audit

Read-only metadata observation, 2026-11-05T01:06:24.383Z. Only this report is written. No worker/process inspection, source edits, builds, tests, cache writes, cancellation, or installed-store recursion.

## Actual Location

Storage owner joins context.root with protocol.directory and receipt/evidence name (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts:79-98). Read context is store.root and requires the dependencies partition (dependencies/💾️storage/📥️read-context/🟦️.ts:42-44). Partition getter delegates to data getter (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:38), which joins workspace, policy directory and native mountDirectory (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts:7-9). Policy dependencies directory is .cache/b/d; native mountDirectory is m (🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json:390); live named receipt/evidence directory observed is .cache/b/d/m/generation. Schema generation directory and suffixes: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧬️schema/🔣️.json:11-17.

## Exact Receipt Counts

Input groups=6 and input sources=5564 for every receipt; manifest texts=323 for every receipt. Bindings below counts groups, not pairs, matching verifier advances.

| Receipt | Bytes | Bindings | Ready | Frontier rows | Execution rows | Before frontier | Frontier end | Full total |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1a41d0c66f416526.a516ae12a30e300b583c9c431c3aef4759faf13eaf1906d6574306e941a86627.receipt.json | 1151831 | 2 | 2 | 94920 | 2979 | 5891 | 100811 | 103790 |
| 75ec76fbfd76deb6.65bd5f50cb23cac64e53dd844bbc866ff1e0c63a8f6e0a00af64a31047468a41.receipt.json | 1181976 | 3 | 3 | 94920 | 6009 | 5893 | 100813 | 106822 |
| a6555dd723934201.1410c75a89602d343a1c29aba80b60ccecef9a95aa13762c778e6274d10709c5.receipt.json | 1180776 | 2 | 2 | 94920 | 4181 | 5891 | 100811 | 104992 |
| b49788f118289ae6.8274042c66b9f0da3efd4162dafede82ca0657783bb100bab12d8997e4480e72.receipt.json | 1181639 | 3 | 3 | 94920 | 6000 | 5893 | 100813 | 106813 |
| cdb4ee2aea69cc6a.faa67441a8202beee13e51eaf1ccfe6dd4c2a23bec46ce3cdda737510ca0e47d.receipt.json | 1150376 | 1 | 1 | 94922 | 1753 | 5889 | 100811 | 102564 |
| eebc46265979a619.df11bbd57ac432bd695aca60bc1a26609d7fe43fe126b07c181292152e9cbded.receipt.json | 1182522 | 3 | 3 | 94920 | 6042 | 5893 | 100813 | 106855 |

## Evidence Metadata And Bounded Stream Counts

Every evidence file was streamed with a reused 64KiB buffer to count newline bytes, without JSON parsing or printing row contents. No single read exceeds 64KiB. Receipt reads are each below 2MiB. Counts are framing checks only; seals and schemas were not validated.

| Evidence | Declared bytes | Stat bytes | Declared rows | Newlines | Final LF | Stable stat |
|---|---:|---:|---:|---:|---|---|
| fb33db0cab67d29c40b0f76169e43fd76bb2fb7881d1bd7b7bc6289fcb929c34.frontier.ndjson | 29438301 | 29438301 | 94920 | 94920 | true | true |
| 367a61d298a20f2456d031639d3741799a47bb1a26b570cf8d2c7ea649f7c499.execution.ndjson | 449771 | 449771 | 2979 | 2979 | true | true |
| bcbcfbbeb06e49a545c5417cd305c7da6c8e8a65b96be4f77ac2060ec5e025d0.execution.ndjson | 920553 | 920553 | 6009 | 6009 | true | true |
| 59fc6767e78162adec99cc880250963a54bfa45d6ed5a4e62c04dc46e0497688.execution.ndjson | 663829 | 663829 | 4181 | 4181 | true | true |
| 67cdff671365acb8e1346036eab15b65fa6ae626af4206cb6d10d95025401c56.execution.ndjson | 919294 | 919294 | 6000 | 6000 | true | true |
| c01c1265d105918409808323035901c96ed6e9ea321e98276bd8a568c796a3f3.frontier.ndjson | 29438854 | 29438854 | 94922 | 94922 | true | true |
| 4bb080ac59d2012086b20c6389cade7da5f14981bdc25e32e4957b4481d1b21d.execution.ndjson | 278980 | 278980 | 1753 | 1753 | true | true |
| 4d57cdc543681754cf17eba23a6f4b74678eb7dc0053bf0e2cd289da398e325d.execution.ndjson | 925504 | 925504 | 6042 | 6042 | true | true |

## Mapping 98304

98304 = 24 × 4096. Verification advances bindings, ready, individual input sources, manifest texts, then frontier rows, then execution rows (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts:184-239). Pre-frontier is 5891 or 5893, except the one-binding receipt at 5889. Thus checkpoint 98304 lies at frontier row 92413, 92411, or 92415 respectively. All receipts still have 2507 or 2509 frontier rows remaining immediately after that checkpoint. Frontier ends at 100811 or 100813, before the next checkpoint 102400.

For full verification, execution rows 1589 or 1587 would reach checkpoint 102400. Full totals above all exceed 102400; the one-binding receipt total is 102564. A recheck skips execution entirely and can finish below 102400 with no further checkpoint. Synchronous work in the last frontier interval or before execution advances can therefore retain 98304; counts do not locate a live instruction or establish an infinite loop.

Capture uses a separate zero counter and advances only membership paths and execution observations (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts:132-154). Capture totals are frontier+execution. For the one-binding receipt that is 96675, so its persisted counts cannot explain capture checkpoint 98304; other persisted receipts could place capture checkpoint 98304 in execution (3384 observations after a 94920 frontier). Persisted receipts do not prove which live scope, receipt, recheck, or capture is active.

## Confidence And Next Check

High confidence in named-file sizes, metadata counts, bounded newline counts, and arithmetic for these observed files. Medium confidence in applicability to foreign workers: worker receipt identity, scope, operation and captured version were deliberately not inspected. No uncounted referenced large evidence blob remains; large frontiers were streamed only, never parsed. Other non-receipt metadata leaves were statted but not decoded.

Smallest concrete next check: correlate a worker-owned existing safe diagnostic with its selected receipt key and recheck/capture flag, without eval bodies or environments. Then use its exact frontier-end bound above to choose whether owner-side observation of the final 2507/2509 frontier rows or the execution decode boundary is relevant. If no safe diagnostic exists, add bounded tier/index progress only in the defining owner as a separately authorized code task; this observation does not authorize reset, invalidation or restart.
