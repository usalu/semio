# Final Canonical Layout Assembly Audit

PASS for corrected `🗑️generated/taxonomy-fixed` only. The cancelled earlier smoke root was not used as final evidence. All five final movie/sidecar markers were present, and the verifier owner and parent were notified immediately before this independent lightweight audit.

Exact on-disk MP4 sets match the current canonical manifest: 117 raw units (112 beats plus five intros), 31 logical clips (26 chapters plus five intros), and five movies. Every raw unit is nonempty beyond the reuse threshold. Its ordinal `{script,scene}` identity exactly matches the canonical descriptor, and settings are consistently `quality=-ql`, `resolution=320,180`, `fps=5`.

All recorded dependency hashes match current sources across 40 distinct dependency files. Each raw unit includes required canonical entry script, font helper and visual helper hashes. The independently verified frozen font hash is `4c55b9d6a7372ce8de70a4de29d627f2093dc6fa27deedcc953f07350e7b2796`. Every logical sidecar contains exactly its manifest-ordered raw states and matching settings; every final sidecar contains exactly its ordered intro/chapter states, matching video identity and settings. No missing, extra, duplicated or reordered descriptor was found.

| Video | Chapters | Beats | Logical Clips | Final Bytes |
| --- | ---: | ---: | ---: | ---: |
| physical_fundamentals | 1 | 8 | 2 | 4690554 |
| ventilation | 6 | 22 | 7 | 7334312 |
| heating | 6 | 35 | 7 | 8011648 |
| cooling | 6 | 21 | 7 | 4552622 |
| energy_balance | 7 | 26 | 8 | 10505004 |

The final restaged source/index recheck also passed before waiting: all 114 current source blobs match (five Git LF-normalized), all 141 old coordinates are absent, 257 owned product entries differ, exact registry/shared projections preserve baseline content, and zero outside-scope changes were found. See taxonomy-index-audit.md.

This audit certifies file sets and nested recorded source/order/settings integrity; actual full PyAV/FFmpeg decoding and ordered pixel samples belong to the independently notified verifier run. No product/index/render/controller mutation occurred. The index diagnostic exited 0 before the wait; the assembly checks used filesystem reads only. No audit-owned process remains.
