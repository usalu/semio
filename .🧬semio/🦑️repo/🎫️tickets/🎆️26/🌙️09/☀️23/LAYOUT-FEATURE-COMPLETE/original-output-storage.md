# Original Output Storage

Layout export now owns raw and encoded bytes in separately admitted PagedList byte backing. ChunkRope performs no eager maximum-output queue allocation. A per-turn OutputWork forwards only the caller's original remaining grant and aggregates accepted child effects. Byte copy, backing reservation, depth and physical release are independent. Native byte/page and pending transfer owners use ManuallyDrop guards so abandoning a live owner cannot silently perform unfunded disposal.

Encoder startup is a distinct BeginEncode stage. Ordinary output admission changes work progress but does not commit an emitted replay milestone. A pure output_work_demand quote includes byte backing, bounded emission copy, PDF offset preparation, and PNG scanline capacity. These quotes gate actual original grants; there is no grant inferred or enlarged from a quote. InteractiveJob currently has only close-demand scheduling hooks, so the pure execution quote has no external scheduler consumer yet.

Base64 reads the retained raw bytes in bounded aligned blocks and uses fixed stack arrays. It no longer allocates temporary input/output vectors or drops consumed raw chunks. Original-funded contiguous transfer vectors are copied in512-byte steps and remain with the job through queue admission. ArtifactOutputChunks.push_original couples queue and optional media credit; refusal/busy/cancel retains the exact pending Vec. seal_original completes the same original queue. The parent reserves its own scalar metadata effects with with_retained_loan and posts them only after child acceptance. See [ORIGINAL-OUTPUT-PRODUCER.md](ORIGINAL-OUTPUT-PRODUCER.md).

PNG reuses a whole admitted scanline allocation, funds its initialization and pixel writes, and uses an inline IHDR array. Output close returns physical byte/metadata allocations and pending transfer capacity under the same original caller. Native old fixtures that assumed eager VecDeque capacity were remounted onto the real writer and stable byte-page addresses; close grants now cover indivisible complete backing and paired owner birth. The physical-buffer fixture enters its target stage by driving the entire close ladder, so it cannot bypass newly owned plan lists. Dedicated plan list/offset ownership is recorded in [export-plan-storage.md](export-plan-storage.md).

Actual executed portable evidence:

- 4 shared binary cases (empty, complete byte alphabet, byte-page crossing, metadata fanout crossing), pinned independent Node Buffer base64 and SHA-256 output.
- 389 original-funded turns,982 denied undergrant probes,16 cancellation cuts; all effects fit the declared total stock and every logical byte/page owner reaches terminal-empty.
- Independent test-only Buffer, crypto, Lodash and AJV libraries stay behind owned oracle interfaces.
- The combined Layout public facade, Drawing composition and export test closure checks709TypeScript sources with0diagnostics. Composed Drawing rerun:10cases/20runs,50cancels; Page Drawing6runs/24cancels;38940positive-axis denials.
- SVG precision remains8cases/6refusals plus2 W3C paint probes through the actual paint writer, described in [svg-paint-precision.md](svg-paint-precision.md).

Native source/type inspection is not compilation. Scoped output/module syntax checks have no error or missing nodes, and two read-only audits found the stale BeginEncode fixture now assigned/repaired by the plan owner. Native compilation, actual OriginalJobFixtureCaller laws, allocator witnesses and GUI execution remain unrun: fresh repo --help still reports CommandNotFound. No direct Cargo/Bun bypass was used.

SVG/PDF formatting now uses bounded stack views with original copy receipts; see [bounded-export-text.md](bounded-export-text.md).

Remaining work: package-entry and raster extraction owners still need complete original accounting; JSON/interchange validation/encoder owners contain separate cold paths; work and committed replay milestones are now separate; see [checkpoint-milestones.md](checkpoint-milestones.md) for the native execution and production restore-consumer limits. Keeping raw bytes through base64 trades bounded extra memory for explicit source custody. The new byte owner is not a claim that the whole export pipeline or Layout feature set is complete.
