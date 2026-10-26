# Teaching Video Taxonomy Correction

The teaching-video sources now follow the authoritative emoji taxonomy. The existing transfer ticket was reopened for this correction; no commits, branch switches, worktrees, or AGENTS edits were made.

## Canonical Product Layout

```text
🎓️teaching/🏛️architecture/⚡️energy/
  🧲️physics/🎬️clip/
  🌬️ventilation/🎬️clip/
  🔥️heating/🎬️clip/
  ❄️cooling/🎬️clip/
  📊️demand/🎬️clip/
  🎬️clip/🔣️.json
  🎬️clip/👋️intro/🐍️.py
  🎬️clip/👋️intro/🏰️institution/📕️.pdf
  📖️concepts/📝️.md
  🏗️authoring/
    🎬️clip/⚙️runtime/🐍️.py
    🎬️clip/🖌️render/🐍️.py
    🎬️clip/🧬️schema/🔣️.schema.json
    🎬️clip/🧪️tests/
    🔤️fonts/🐍️.py
    🔤️fonts/🧪️tests/
    🎨️visuals/🐍️.py
    🔊️speech/🐍️.py
    📦️packages/🐍️python/{pyproject.toml,uv.lock}
    📜️script.ts
```

Each lesson owns its full wrapper at `🎬️clip/🐍️.py`. Chapter directories use fitting emoji plus English semantic names, with kind-only `🐍️.py` scene leaves. Composition, speech, timing and trace have separate semantic owners. The manifest preserves all five lessons, 26 chapters, 112 authored beats and five intros in explicit source order. Shared tools sit at the energy lowest common production consumer owner. The exact source path contract is [taxonomy-paths.cases.json](taxonomy-paths.cases.json).

All 141 superseded product coordinates are absent. There are 114 canonical product source/configuration/documentation/test files: 27 redundant Manim preview discovery configurations were removed because the existing renderer already supplies explicit quality/output configuration. No `.cfg` naming exception or old-name compatibility loader remains. The taxonomy registers 42 parent-aware semantic kinds and 10 exact member rosters, with additions in two existing energy/topic rosters. Other registry fields were preserved. `README.md`, `📜️script.ts`, `pyproject.toml` and `uv.lock` retain the exact registered external-contract names at their proper scopes.

Retained ticket inputs and audit filenames remain unchanged under the explicit `areas[".🧬semio/🦑️repo/🎫️tickets"] = "exempt"` authority. This exemption does not apply to product sources; `🎓️teaching` is a clean area. See [taxonomy authority](taxonomy-audit.md) and [full layout](taxonomy-layout.md).

## Validation

Independent parent-aware directory/leaf/fixed-contract/path checks, case-collision checks and manifest JSON Schema/AST closure passed. Live incoming/outgoing reference scanning found no superseded coordinates in governed source or documentation. All 611 authored callable ASTs in 26 chapters and the institutional intro remain unchanged; all 11 timing/trace JSON files and the PDF remain byte-identical to Niloufar source. See [layout checks](taxonomy-test.md) and [source audit](taxonomy-source-audit.md).

The documented canonical entry ran 32 tests in 30.661 seconds: 31 passed, with one Windows symlink-privilege capability check skipped. All nine font tests, including the independent FontTools glyph oracle, ran successfully. Real Manim testing rendered Unicode-path raw units through chapter/movie assembly and decoded the expected frame/color order. Optional generator/composition checks passed for all 26 speech generators, 25 section builders and five wrappers without invoking paid TTS. See [pipeline validation](taxonomy-pipeline.md).

The first lower-resolution smoke attempt exposed a Pango measurement-canvas defect: long wrapped-text trials lost trailing glyphs at width 320 even though the fonts contained them. The font interface now uses at least the validated 854×480 text-construction canvas while restoring the actual camera/output configuration, and isolates narrow-canvas SVG caches with fixed-length hash keys. Default/high profile cache keys and supported geometry remain unchanged. Exact failing strings, vector equality, stale-cache isolation, and restoration on errors are tested. The final font helper SHA-256 is `4c55b9d6a7372ce8de70a4de29d627f2093dc6fa27deedcc953f07350e7b2796`. See [independent font review](taxonomy-font-audit.md).

The failed attempt was cancelled through its exact owned process tree; all descendants ended before the tested fix and fresh run. Its diagnostic evidence is retained in Markdown, while generated clips/logs are disposable. The corrected full end-to-end smoke run uses 320×180 at 5fps for every authored unit; this validates the final namespace, tool and cache integration. The original complete 854×480/15fps validation of 113,285 frames remains historical evidence of the unchanged authored construction bodies, with detailed hashes in [the original report](summary.md).

### Accepted Corrected Render

All 117 raw units, 31 logical clips and five final movies passed full PyAV decoding and independent FFmpeg `-xerror` decoding. FFprobe format/frame/timing checks, exact ordered frame totals, monotonic presentation timestamps, nested source/settings provenance and 351 ordered raw-boundary sample hashes passed. Source dependencies match the final frozen implementation. The independent assembly audit also passed.

| Lesson | Chapters | Beats | Frames | Seconds | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| physical_fundamentals | 1 | 8 | 5005 | 1001.000 | Pass |
| ventilation | 6 | 22 | 8158 | 1631.600 | Pass |
| heating | 6 | 35 | 8709 | 1741.800 | Pass |
| cooling | 6 | 21 | 5435 | 1087.000 | Pass |
| energy_balance | 7 | 26 | 10613 | 2122.600 | Pass |

Total: 37,920 frames and 7584.000 seconds at 320×180/5fps. All five 10/50/90% final contact sheets were inspected; their layouts, glyphs and diagrams are intact in the sampled frames. See [complete render/decode audit](taxonomy-render-audit.md), [assembly audit](taxonomy-assembly-audit.md), and [verifier review](taxonomy-verifier-audit.md).

### Accepted Generated Movie Identities

| Movie | Bytes | SHA-256 |
| --- | ---: | --- |
| Full_Physical_Fundamentals_320x180p5.mp4 | 4690554 | `80b5a1ab82a619267b89264539eab498391ffde68a5c671a5b630e6baa5337b2` |
| Full_Ventilation_320x180p5.mp4 | 7334312 | `5792801151d2fcd6ff9a4ee7c21e11f5c3134681099df5ac90cd8a34077ec86b` |
| Full_Heating_Demand_320x180p5.mp4 | 8011648 | `2e4bf1d4f40abb17955e805ae6d6bd15770f60adead48005b9ecebabe0a77907` |
| Full_Cooling_Demand_320x180p5.mp4 | 4552622 | `618f71d8f5ac1fcae37df0a46a3038c939a2378df13e70682dc112018bc910d6` |
| Full_EnergyBalance_320x180p5.mp4 | 10505004 | `1d65ef068311ef4413ab352a4867de74a79b3b1cd3d8789ff433e68ba744ed62` |

All generated movies, previews, logs, compiler/runtime caches and machine reports are removed at completion under the ticket rules; authored inputs, fixtures and Markdown evidence remain. The final generated-path audit checked 55,596 entries: longest 228 UTF-16 units, zero over 256.


## Commands

```sh
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --jobs 4 --quality l --output-root ./videos --no-play
```

Product staging is explicit and uses selective projection of shared README/native/taxonomy changes. Current independent index certification finds all 114 canonical source blobs current, all 141 old coordinates absent, and no unrelated index changes. Three neutral projection cases check preservation of concurrent registry content. Final ticket metadata/evidence staging and generated cleanup complete the accepted task; see [index audit](taxonomy-index-audit.md) and [finalizer review](taxonomy-finalizer-audit.md).

Final report and retained-input audit passed: [report audit](taxonomy-final-report-audit.md). Final closure, source-hash verification and closed-metadata staging are recorded in [finalization](taxonomy-finalization.md). Generated cleanup follows the last repository command.
