# Ordinary Warm Cost Audit

Read-only audit on 2026-10-15. No builds, lock probes, or Source edits were performed. Evidence: retained `🗑️generated/rust-protected-ordinary-prototype2.log` and its `on-0jhVx0-{cold,warm,restore}.jsonl` child logs. The complete test timed out at 240000 ms with 119 assertions; later workspace invalidation remains unqualified.

| Measured span (ms) | Cold | Warm | Restore |
| --- | ---: | ---: | ---: |
| Phase wall | 136565 | 30343 | 36926 |
| Nx bootstrap + child | 52548 | 19686 | 24453 |
| Difference: preparation + graph pin + surrounding work | 84017 | 10657 | 12473 |
| Bootstrap admission | 2010 | 2740 | 2521 |
| Bootstrap provisioning | 5512 | 3471 | 6622 |
| Bootstrap activation | 701 | 213 | 260 |
| Nx createProjectGraphAsync | 11528 | 9119 | 9648 |
| Specified configuration glob | 10196 | 6813 | 7130 |
| Retrieve workspace files | 430 | 985 | 922 |
| Nx task run duration | 28900 | 715 | about 1400 |

These spans overlap and must not be summed indiscriminately. Aggregate preparationMs/graphMs did not survive this timeout. Their separate contributions cannot be reconstructed from phase differences. Buffered `cache:tools-store` messages provide no stage-duration evidence.

The strongest retained warm bottleneck is specified-plugin configuration glob discovery: a single `📋️project.json` result costs 6.8–7.1 s and dominates the 9.1–9.6 s graph creation span. `workspace context init` is only 5–7 ms; treating its constructor as the expensive traversal would misattribute cost. Workspace-file extraction is a secondary ~0.9–1.0 s span. Warm Native task hashing is hundreds of milliseconds (first group 203 ms, later group 113 ms), not the multi-second primary cost. Engine Source closure is 191 ms warm and 101 ms restore. Actual cached tasks complete in 715 ms warm; replayed cached compiler log text is not fresh compiler execution. Miss sets are genuine cold base/consumer/sibling, warm empty, restore empty.

Bootstrap catalog admission separately reports git inventory 2.29 s warm and 2.20 s restore despite zero structural manifests. Provisioning is another directly measured 3.47/6.62 s warm/restore. Nx execution owns those changes; this audit does not duplicate them. Selected Rust diagnostic spans for metadata, Source boundaries, publication and storage remain useful component measurements, but do not prove the unexplained preparation remainder belongs to any one component.

Current Source already contains a finite snapshot owner, schema, tests, runtime export and Nx WorkspaceContext patch under `🏗️build/📥️inputs/🎯️selected/📦️snapshot` and `⚡️caching/🩹️patches/nx@23.2.0.patch`. Searches show `withSelectedNativeSnapshotV1` is called only by its tests at audit time, so the ordinary selected frontend has not yet demonstrated the snapshot path. The patch changes the Native context physical root while retaining the original authority root and checks every admitted Source path appears in actual Native file inventory. Existing explicit-inventory probes are feasibility evidence, not measured production speedup.

A bounded admitted inventory/snapshot can target expensive Native discovery, provided the ordinary path passes genuine current graph inputs and the full required ignore/manifests/Cargo Source closure. Keep ignored explicitly admitted Source, changing crate consumer invalidation, include/env inputs, ABA and missing-path refusal, symlink/ancestor identity, actual reader image/generation authority, cancellation and finite capacity checks. Do not replace these with a remembered file list, unchecked cache receipt, or narrower guessed extension glob. Compare the next retained specified-glob/profile spans and complete invalidation assertions before claiming improvement. Raising the deadline alone does not address the measured discovery cost.
