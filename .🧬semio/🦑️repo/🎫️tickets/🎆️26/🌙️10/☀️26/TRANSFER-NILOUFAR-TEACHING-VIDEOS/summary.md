> Historical transfer validation. The current emoji taxonomy layout and follow-up validation are recorded in [taxonomy-summary.md](taxonomy-summary.md).

# Teaching Video Transfer and End-to-End Validation

Completed transfer into `C:/git/semio` on Ueli WIP (`🐙ueli/⛳wip`) from the existing Niloufar WIP worktree. The source tree contains authored video sources, not pre-rendered curriculum movies. All 131 canonical authored files are present under `🎓️teaching`; 141 teaching source/tool/fixture paths plus three shared setup/documentation files are staged. The 26 scene scripts, institutional intro/PDF and shared visual helper (29 immutable assets), and all 38 configuration/timing/trace files remain byte-identical to Niloufar’s source. No commits, branch changes, stash, or new worktrees were made.

## Accepted Complete Render

All five corrected silent lessons rendered from the staged implementation at 854×480, 15fps. The source-defined plan contains 112 authored beats and five intros, rendered as 117 independent complete units and assembled into 26 chapters / 31 logical clips. Independent AST comparison confirms the original authored segment order. This follows the original per-beat builder approach; Manim partial caching and prefix skipping are disabled.

| Lesson | Chapters | Beats | Frames | Duration (s) | Full decode |
| --- | ---: | ---: | ---: | ---: | --- |
| physical_fundamentals | 1 | 8 | 15070 | 1004.647786 | Pass |
| ventilation | 6 | 22 | 24333 | 1622.187305 | Pass |
| heating | 6 | 35 | 25918 | 1727.856250 | Pass |
| cooling | 6 | 21 | 16142 | 1076.123568 | Pass |
| energy_balance | 7 | 26 | 31822 | 2121.459831 | Pass |

Total: 113,285 frames, 7552.274740 seconds. The verifier fully decoded every raw unit, every logical clip and every final movie with PyAV, checked frame counts and monotonic timestamps, and compared 351 ordered first/middle/last raw-unit frame hashes at chapter-local and movie-global offsets. FFprobe independently counted final frames and checked format/timing; FFmpeg `-xerror` fully decoded all five movies successfully. Exact expected file sets and all recorded current dependency hashes also passed. `verify-videos.py` finished with exit 0. See [assembly audit](final-assembly-audit.md) and [verifier audit](verification-audit.md).

Sampled visual QA covered every final lesson at 10/50/90%, four nearby build-in frames, and targeted native-resolution scenes. Missing Georgia temperature/arrow glyphs found in the rejected first pass were corrected with symbol-only system-font fallback. Corrected ϑ, ← and → are visibly clean in actual final footage; all 44 declared symbols passed the native Pango/Cairo and independent FontTools checks. No visual blocker remained in inspected samples. The frozen helper SHA-256 is `00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2`. See [final visual audit](corrected-visual-audit.md). First-pass metrics and images in earlier reports are historical rejected evidence.

## Reproduction and Tests

From the workspace root, use the native/devcontainer repository tool entry:

```sh
repo 🎓️teaching/📜️script.ts test
repo 🎓️teaching/📜️script.ts render --jobs 4 --quality l --output-root ./videos --no-play
```

The documented test entry provisioned a fresh pinned Python 3.12 environment and ran 27 tests in 65.495 seconds: 26 passed; one native symlink-capability test skipped on this Windows host. The independent seven-test glyph suite also passed in that minimal environment with no skips or external environment import paths. Language-neutral fixtures cover rendering, cache invalidation, owned-process cancellation, font fallback, and native prerequisites; real miniature Manim segment→chapter→movie and independent JSON Schema checks passed. Local pipeline checks covered all 26 audio generators and 25 section builders; live paid TTS was not invoked. Canonical complete movies are silent and require no TTS account. Optional narration tools use explicit environment credentials through the local tool interface.

Hardcoded personal macOS project imports and `.venv/bin/manim` selectors were removed. The renderer uses the active Python, pinned Manim/NumPy/imageio-ffmpeg tools, local interfaces, private caches, source provenance, global concurrency, progress and process-group cancellation. Native/devcontainer setup supplies Cairo/Pango and math fonts. Windows rendering and tests ran here; macOS/Linux/devcontainer configuration passed fixtures and shell syntax checks, but those hosts were not executed. See [pipeline validation](pipeline.md), [minimal glyph test](minimal-font-test.md), [native fonts](native-fonts.md), and the [teaching README](../../../../../../../🎓️teaching/README.md).

The separate Bauen mit Bestand presentation MP4 already exists byte-identically in the destination (SHA-256 `54b0672cca68a474d44c6096abb6579160b4d33b0f637f588e2e0752373e05c7`) and also passed a full FFmpeg decode; it was not duplicated. See [source integrity](source-integrity.md).

## Accepted Movie Identities

These identify verified generated outputs; repository instructions require generated movies, logs, dependency caches and machine reports to be deleted after validation. Authored audit inputs, fixtures and Markdown evidence remain in this ticket.

| Movie | Bytes | SHA-256 |
| --- | ---: | --- |
| Full_Physical_Fundamentals_480p15.mp4 | 19159937 | `be12077d2cc62182c2ebaf25f5cbc4cee0f2611bc45b91a8e0adc1ddd36940f1` |
| Full_Ventilation_480p15.mp4 | 31414709 | `cdf019e0996187cfedad728890dc70ca0dd2852942fd8db45c87fec3d2a1deb5` |
| Full_Heating_Demand_480p15.mp4 | 34284491 | `4dd44a5d030b2443e777f73a5da89c3818f75097b7a4d8554841f6582c55523d` |
| Full_Cooling_Demand_480p15.mp4 | 20008732 | `97f4eb1eacb4fe099c77a72bf3ccb5173ce46fdc187bbf8eadad37c80155e09b` |
| Full_EnergyBalance_480p15.mp4 | 43159255 | `3a727ab0858a997be1e5714831025c3bf6b90c685bcd23998205940d2d65cf2d` |

## Staging and Cleanup

Explicit source staging and selective shared-file staging preserve concurrent work. Shared README/native content was independently reconstructed against the original index; all 144 product paths match the intended staged result, with no unrelated index changes at certification. No AGENTS files, generated cache/media, secrets or tool binaries are staged. Longest audited generated path was 229 UTF-16 units; none exceeded 256. See [final transfer/index audit](final-transfer-audit.md), [staging isolation](staging-coherence.md), and [input retention](retention-audit.md).

The repository ticket is closed through repo MCP with all 257 changed product/audit/input paths recorded; repository goals were left untouched. All render and verification processes have ended. Final staging includes the retained ticket evidence. Generated outputs are deleted as the final housekeeping step after the last repository command; source files, fixtures and reports are retained.
