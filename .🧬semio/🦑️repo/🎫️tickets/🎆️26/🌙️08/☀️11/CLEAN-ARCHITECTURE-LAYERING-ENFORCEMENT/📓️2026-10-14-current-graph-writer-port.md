# Current Graph Writer Port Inspection

Read-only defining API inspection on 2026-10-14; no new runtime capture or module/lease/quota/record-store mutation.

The retained media-native6 process terminated with `SyntaxError: Export named 'repositoryBuildGraphWriterDirectoryV1' not found` after discovery and before source/strict/Cargo tests. This is an actual historical failure, not acceptance.

Fresh current source inspection now finds that exact function explicitly defined/exported at `🏗️build/📥️inputs/💾️storage/🟨️.mjs:29`. Its current file SHA-256 is `4a5008b62c87da60677aaa78d8201c42cf11d0289fd29bf029e06bec8d1e3254`. Both import callers (`🏗️build/📥️inputs/🟨️.mjs:1` and `🏗️build/🕸️graph/📥️inference/🟨️.mjs:1`) name that exact defining export. No textual missing export remains at this inspected frontier. Source inspection alone cannot qualify fresh runtime linking.

The storage module owns a singleton `AsyncLocalStorage` holding live original fixed-volume writer custody. An alias or separately revision-imported singleton would not be a valid repair. The current defining method refuses absent original custody, admits its closed namespace, creates the directory on the original device, and rejects symlink/type/device changes. Its concurrent owner already authored the defining export before this inspection; no duplicate export or compatibility branch was added.

A language-neutral expected defining-port list and an existing graph test were authored. The test compares the module's explicit export declarations using the independent TypeScript syntax tree, checks current runtime namespace functions, and asserts writer-directory refusal without original custody. It remains unrun because registered source tests have not collected. This witness does not substitute for a fresh registered Actor/DSL/catalog source/native capture.

Initial GUI normalization is complete at the authored source frontier described in `📓️2026-10-14-initial-gui-authority-repair.md`. No new capture was started after these source changes. Current runtime acceptance remains unqualified.
