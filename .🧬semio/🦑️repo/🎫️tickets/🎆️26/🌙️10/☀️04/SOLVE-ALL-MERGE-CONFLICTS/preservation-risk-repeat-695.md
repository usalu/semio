# Preservation Risk Audit, Repeat 695

Read-only Git inspection; no builds, tests, compiler, resolver, runtime, source/index/Git mutations. AGENTS.md read. Immutable inputs: base b84bf526c7ba26f99c4054b47da1c1a4d5064e3d; HEAD 8f91299152ceab6c19df64c251f5ad23252ce18b; stash 6e8eff657fb70675db7ad30b890f61b7a3e201d7.

## Scope

HEAD commits since base: 7880cc36f78 (693), 4dab415ed90 (694), 8f91299152c (695). Raw no-rename HEAD delta: 16,830 additions, 2,536 deletions, 18,508 modifications. Stash: 3,862 additions, 307 deletions, 4,913 modifications. Git exhaustive rename detection is skipped at ordinary rename limit, so rename output alone cannot establish ownership.

## Base to HEAD Ownership Map

| Base owner | HEAD receiving owner / decision |
| --- | --- |
| value/retirement/shared/sealed Rust | value/retirement/arc-custody/sealed Rust, schema, fixture and tests; preserve custody owner. |
| value/retained-clone/shared Rust | retained-clone dynamic/wire/domain leaves plus derive companion; compare symbol ownership before transplant. |
| ui/tui root Rust | ui/tui/engine input-dispatch, frame-layout-and-overlays, construction-and-focus, paint-and-gutters, mouse-and-hover; never restore root body wholesale. |
| plugin/tests/completion root Rust | completion/receiving TS/schema and carved plugin runtime contract tests; preserve receiving boundary. |
| plugin/tests/composition root Rust | composition/emission Rust/TS and composition/receiving tests/schema; adapt stash changes to the new owners. |
| plugin/tests/document-archive-load-legs Rust | plugin-runtime-plugin-builder-contract-part-1-x/document-archive-and-replacement-publication and part-4/archive-load-reload-and-revert; review exact assertions before moving. |
| repo cli, workspace, graphql Go package-local unit tests | respective module/tests/unit/go receiving owners; do not resurrect package-local tests. |
| repo/library root TS | library/packages/typescript leaf ownership plus domain laws/build/process owners; deleted root is intentional dissolution. |

## Collision Risks

HEAD deletes 20 paths changed by stash, including three FEM/Puzzle stories, sealed/shared custody, retained-clone/shared, TUI root, lifted store/document Rust, UiPreferences TS, three plugin Rust test roots, DB sync unit root, repo Go test roots and library root/bootstrap input. Treat these as relocation/semantic transplant, not absent-file restoration.

Stash deletes HEAD-modified quiz/course/proctor and framework quiz React owners and architecture-quiz workflow; deletion requires a deliberate Course receiving-owner decision. The old Puzzle 5d editor snapshot Rust is another reverse delete/modify collision.

Root nx.json/package.json/script.ts all changed on HEAD; root script has 31 added/28 removed lines and package 137 added/4 removed. Preserve current dispatch and workspace ownership while adding backend-neutral Nx/Bazel behavior. New .bazelrc/.bazelversion are incoming; no corresponding HEAD root Bazel files appeared in the inspected root diff.

## Path Length Audit

Absolute UTF-16 path length >256: 0 already present in immutable HEAD; 0 stash paths absent from HEAD. These are raw tree counts including ticket/archive content, not a claim that all should be merged.

New stash-only overlong examples:
