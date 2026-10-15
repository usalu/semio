# 🪓 Monolith Split and Dependency Direction: Session Summary (2026-10-15)

A Claude Code fleet worked this sub-track of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`. Opus 5.5 coordinated, Sonnet 5.5 High agents executed, and Haiku 5.5 High agents mapped seams and audited. The repo MCP `ticket_open`/`ticket_reopen` failed (`structuredContent` null) and created nothing, so the work lives in this subfolder. Chronology: `📓️FLEET.md`. Rules: `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`.

## Monoliths split (parent lines before → after, plus children)

| File | Before | After | Report |
| --- | ---: | --- | --- |
| wgpu Shell `🦀️.rs` | 35,581 | 966 + 20 | RESULT-WGPU-SHELL-SPLIT.md |
| wgpu renderer `🦀️.rs` | 19,807 | 1,006 + 16 | RESULT-WGPU-RENDERER-SPLIT.md |
| infinite world / normal / dag | 17,408 / 13,836 / 5,430 | 2,581 / 2,669 / 1,649 + 13 / 6 / 5 | RESULT-INFINITE-SPLIT.md |
| ShellHost `🟦️.tsx` | 13,707 | 591 + 9 modules + 21 ordered hooks | RESULT-SHELLHOST-SPLIT.md |
| Scenes `🦀️.rs` | 12,343 | 397 + 15 | RESULT-WGPU-SCENES-SPLIT.md |
| hub bootstrap | 11,963 | 579 + 22 | RESULT-HUB-BOOTSTRAP-SPLIT.md |
| plugin instance / host | 10,927 / 7,295 | 1,256 / 838 + 6 / 3 | RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md |
| puzzle 5d / 2d editors | 9,141 / 4,059 | 1,703 / 1,357 + 8 / 4 | RESULT-PUZZLE-EDITORS-SPLIT.md |
| World3dHost / ShellHelpers | 8,551 / 6,708 | 2,845 / 390 + 9 / 10 | RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md |
| manifest | 8,138 | 589 + 6 | RESULT-MANIFEST-SPLIT.md |
| bim model mount manifest | 7,817 | 127 (re-homed mounts) | RESULT-BIM-MODEL-REHOME.md |
| architect registers | 7,727 | 107 + kernel + 5 families | RESULT-ARCHITECT-REGISTERS-SPLIT.md |
| EngineCanvas / Interpreter | 7,651 / 6,860 | 205 / 157 + 9 / 10 | RESULT-ENGINECANVAS-INTERPRETER.md |
| CAD renderer | 6,859 | 424 + 10 | RESULT-CAD-RENDERER.md |
| os root `🟦️.ts` / r3f | 5,775 / 4,601 | 504 / 495 + 5 / 12 | RESULT-OS-TS-R3F.md |
| mcp workspace / os host | 5,831 / 4,667 | 12 children / 8 files | RESULT-OS-HOST-MCP-WORKSPACE.md |
| presentation react (two copies) | 5,560 + 5,550 | one copy: 364 + 14 | RESULT-PRESENTATION-DEDUPE.md |
| repo GraphQL / CLI | 5,038 / 4,298 | 61 / 35 + 6 / 8 | RESULT-REPO-CLI-GRAPHQL.md |
| PluginRuntime | 4,582 | 369 + 18 | RESULT-PLUGIN-RUNTIME-TSX.md |
| hub directory / inference runtime | 4.4k / 4.0k | 156 / 94 active lines + 8 / 11 | RESULT-HUB-DIRECTORY-INFERENCE-SPLIT.md |
| tiled-map | 4,187 | 277 + 12 | RESULT-TILED-MAP.md |
| NodeGraph | 4,005 | 627 + 8 + 7 hooks | RESULT-NODEGRAPH-TSX.md |

## Dependency direction

- Framework production code no longer names `s.` plugin ids. The framework owns `framework.io.binary/text/sqlite-snapshot`, and the space plugin contributes `os.space-index` (RESULT-FRAMEWORK-S-IDS.md).
- Test-only framework→`✏️s` edges: 94 → 0 cruiser edges. R3 test edges: 2 → 0 (RESULT-TEST-EDGE-RELOCATION.md).
- Plugin→plugin edges: 32 → 4. `QuestionDictionary` moved into the framework value crate, and an s-level `🏘️plugin-composition` test project was added (RESULT-PLUGIN-CROSS-EDGES.md).
- `🖱️ui` cycles: 146 → 44 (RESULT-UI-CYCLES.md). The os-root barrel's cycles went 12 → 0, PluginRuntime's to 0 and NodeGraph's 3 → 0. Element cycles: 469 → 445, with 7 hosts now cycle-free (RESULT-ELEMENT-LEAF-REWIRE.md). `element_contracts` cut EngineCanvas loose from Interpreter and Scenes.
- `image` was removed from tiled-map's runtime dependencies in favour of `semio_framework_pixels::decode_png`, and kept as a test oracle.
- CAD: 3 rules-of-hooks bugs fixed test-first.

## Crates carved under the compiler source-boundary gate (CRATE-SOURCE-GATE-CENSUS.md)

- compiler: 1.58 MB → 113 KB. Fonts go through `FontSet`/`CompilerFonts`, and the native, browser-worker and Flow hosts install them. `bun repo cargo check -p semio-framework-compiler` passed, the first real compile of the session.
- print: 1.85 MB → 139 KB (schema documents load at runtime). server: 5 crates plus a facade. quiz and repo-graphql: each gained a `-schema` crate. pets: 7 producers plus a facade. GraphQL: new `-fs` adapter crate (11 dependencies off the core).

## Verification and limits

- The compiler-ingress source-boundary gate (4000 lines / 32 files / 262144 bytes per crate, closure-wide) refuses nearly every framework and os crate. Later the launcher itself failed ("Live Original process issuance required", "Finite original maintenance authority required"). So apart from `compiler`, the Rust work was verified statically:
  - the patched full-resolution rustc differential (`--emit=metadata -o /dev/null`) against sandboxes holding the originals;
  - definition scans for field and method privacy, proven sensitive with seeded regressions;
  - losslessness checks, rustfmt parsing, path resolution and region markers.
  The hardening passes found and fixed 12 plugin-instance, 8 hub, 51 infinite-struct and 170 renderer-kernel visibility regressions.
- TS/TSX work was verified with scoped tsc and vitest (failure sets identical before and after, apart from documented load flakes), TS export-identity checks and dependency-cruiser.
- Region markers dropped by the splitters were restored everywhere (RESULT-REGION-RESTORE.md, AUDIT-REGIONS-NAMES-SIZES.md).
- Coordinator fixes outside fleet scope:
  - 10 invalid `<X as ArtifactPack + …>::` qualified paths from a peer sweep.
  - The navbar-footer-parity test parse error.
  - The World3dHost shading oracle path.
  - The os build-script sqlite id.
  - Root, layout and forms lock refreshes.
- No modifying git command, no worktree, no AGENTS.md edit. Scratch compiles around the gate were stopped and their artifacts deleted.

## Follow-ups (not done)

1. Run real `cargo check` (native, wasm32-unknown-unknown and wasm32-wasip2, `--tests`) on every split crate once the gate and launcher admit them. Field, method and trait-scope privacy (E0616/E0624) are not proven statically. The fix scripts are in the tool folders.
2. Keep carving tier 1+ crates (canvas, ui-render, pack, server, repo-cli, then os-kernel and others). The quiz test closure is still 6,695 lines.
3. Relocate the wrong-direction items each split grouped:
   - manifest `🧳️product-vocabulary`;
   - tiled-map `🌍️gis-policy`;
   - hub inference `🌍️gis-map-binding`;
   - puzzle2d naming in board normal;
   - plugin SDK `tool-run`/`time-travel`/`ui_wgpu` and `os_vcs` in `GuestRuntime`;
   - EngineCanvas plugin imports;
   - 5d→3d and 2d→3d.
   Also queued: option 1 from the plugin cross-edge work, moving the Semio standard from stdio into the framework.
4. Cut the remaining Interpreter↔Scenes function cycle and Scenes' private `WorkerCell`. Remove the two remaining PluginRuntime→Shell type edges' siblings and apply the 20 World3dHost barrel narrowings.
5. Fix the dependency-cruiser rule `no-cross-technology-✏️s-to-🧰️framework` false positive on sanctioned `@semio-tech/framework` imports, then do SDK barrels (audit packages 7 and 8).
6. Peer-owned:
   - The root `Cargo.lock` has about 200 registry bumps that are missing from the offline cache (`metadata --offline --locked` fails).
   - `scopedPresenceOracle` needs `ScopedPresenceCloseV1` restored.
   - Regenerate the stale distribution `🧾️manifest.json`.
   - The CAD vitest config is missing its `cad-js`/`brepjs` links.
   - Puzzle 2d has an existing E0603 in snapshot privacy at HEAD.
