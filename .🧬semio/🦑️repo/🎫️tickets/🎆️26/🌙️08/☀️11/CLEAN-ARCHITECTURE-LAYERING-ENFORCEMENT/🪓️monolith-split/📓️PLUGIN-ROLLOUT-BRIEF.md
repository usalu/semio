# 🔌 Plugin Crate Rollout Brief (fan-out by family)

Read first: `📓️EXECUTOR-RULES.md` (including the SPEED and no-stub-harness rules), `📓️WAVE4-BRIEF.md` (including the god-object rule), and `RESULT-PLUGIN-WAVE0-GENERATORS.md`. Section 16 of that file is the recipe; §16.6a gives the per-package command sequence.

1. Per package:
   - Precondition and re-measure.
   - `bun ./📜️script.ts new crates <pkg> --verify --oracle --sandbox=<dir>`. The first package of each agent goes through the CLI.
   - Apply.
   - `new crates <pkg> --check-applied --sandbox=<dir>`, which MUST exit 0 with 0 extra diagnostics for lib and test.
   - Repo checks: owners scan `owners-tools/scan.ts`, taxonomy validators, `lib-wiring-budget`, and both launch JSONs must parse.
2. When the generator cannot plan a package within budget, make the minimal correct source change, then regenerate. Typical causes are a `cfg(test)` hook in lib code, an over-budget schema family, a private module reached across crates, or root regions that merge into a cyclic unit. Fixes: hand-split the schema family along its taxonomy, decompose the god-objects, move the test hook into test code, or make the module item `pub` in its rightful owner crate. No compatibility layers.
3. Never leave a package failing `--check-applied`. If time runs out, finish the package you are on, or restore that one package from the generator's `--backup`.
4. Generator bugs: fix them in the library authoring module with a test, and report them in your RESULT file.
5. One heavy job at a time, run niced. Do not use recursive greps.
