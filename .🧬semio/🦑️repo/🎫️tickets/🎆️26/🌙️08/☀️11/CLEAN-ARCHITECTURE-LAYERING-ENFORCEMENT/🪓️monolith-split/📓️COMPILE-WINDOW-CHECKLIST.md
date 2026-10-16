# ✅ Compile Window Checklist

Run this once the wave-4 carves finish and registry edits are frozen.

1. Refresh the locks: root, `🧰️framework`, `🌎️hub`, the layout and forms plugin workspaces, and `🎓️teaching` (`bun repo cargo … -- update --workspace --offline [--manifest-path]`). Then download the registry crates the lock needs, using the route documented in RESULT-INFRA-UNBLOCK.md.
2. Run `verify dependencies write-baseline` for all new manifests (pets, server, quiz, graphql, dashboard, scene, runtime, render, surface, os host/neural/dag, and those still to come).
3. Run check and test bottom-up in census tier order, fixing errors per crate. Each RESULT-W4-*.md lists its first-run law set. Known existing breaks:
   - `use ::graph::manifest` in the dag vcs root has no dependency;
   - the puzzle 2d snapshot functions are imported while private (E0603 ×9);
   - `PUZZLE2D_NODE_KIND_OPTIONS_MAX` is not imported in the 2d unit test;
   - quiz's test closure is 6,695 lines.
4. Regenerate the layout plugin descriptors: `bun repo run workspace:components -- iterate layout`, then `bun repo run @semio-tech/plugin-registry:generate`. Regenerate the distribution `🧾️manifest.json`.
5. React renderer activation path: deliver declared package assets (RESULT-W4-UI-RENDER.md, addendum 3).
6. Taxonomy registration sweep for all new folders (`directory-kind-unresolved` findings).
7. End-to-end boots: os dev / play with the wgpu and React renderers, and the headless layout open. Look for the `[TRACE]` font installs and that icons render.
8. Regenerate the descriptors and rebuild the components for the 7 canvas-asset plugins (dag, flow, forms, reasoning, sequence, space, writer) and for layout, so hosts deliver their declared package assets (RESULT-W4-CANVAS.md addendum). Then run the end-to-end check that icons and labels render in a plugin guest.
9. Add production staging for layout's 15 text fonts, and package-asset staging in the workspace release orchestrator.
10. Remove the remaining double mounts: ui-host and ui-backend-webgpu still `#[path]`-mount the framework `abi` file (RESULT-W4-FRAMEWORK-FACADE.md).
11. Run the first-compile law sets listed in each RESULT-W4-*.md, and the trace testkit (RESULT-W4-TRACE-TESTKIT.md).

- [ ] `check -p semio-framework-ui-contract-conformance --tests`: the taxonomy sweep renamed 9 conformance folders and their `#[path]` lines (RESULT-TAXONOMY-SWEEP.md).
- [ ] Re-run `verify taxonomy` inventory after all carves land (new folders keep appearing; slug-pattern rule in RESULT-TAXONOMY-SLUG-PATTERN.md).
