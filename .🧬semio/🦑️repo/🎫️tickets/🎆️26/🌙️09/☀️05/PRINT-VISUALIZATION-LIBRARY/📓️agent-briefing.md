# Agent Briefing — Print Visualization Library Fleet

You are one execution agent of a parallel fleet rebuilding the print visualization library. Read, in this order: `C:\git\semio\CLAUDE.md` (repository rules, binding), `📓️architecture.md` (the contract, binding), then the exploration notes you need (`📓️explore-latex-viz-api.md`, `📓️explore-gallery-taxonomy.md`, `📓️explore-test-build-infra.md`, `📓️explore-semio-style.md`).

## Environment
- Repo `C:\git\semio`, Windows 11, use the Bash tool (Git Bash). Paths contain emoji: always quote, prefer absolute paths, avoid chained `cd`. Never run git commands that modify state (no commit, stash, checkout, worktree). Other agents edit other files concurrently; never revert or "clean up" files you do not own.
- Product root: `C:\git\semio\🧰️framework\🛍️products\📓️print\`. LaTeX packages: `🖋️latex/`. Taxonomy: `🖼️assets/📊️viz-taxonomy.md`. Schema: `🧬️schema/🔣️.json`.
- Ticket folder (this folder): all temporary files, logs, probe compilations under `🗑️generated/<your-agent>/`. Your report: `📓️status-<AGENT>.md` (create at start, update as you go: done / in progress / blocked / decisions / files touched / requests to other agents). Read other agents' status files when you need their vocabulary; never edit them.
- Local TeX compile for quick checks (until the repo toolchain is bootstrapped): MiKTeX is installed.
  ```
  cd "<ticket>/🗑️generated/<agent>/<probe>" && export TEXINPUTS="C:/git/semio/🧰️framework/🛍️products/📓️print/🖋️latex;" && xelatex -interaction=nonstopmode -halt-on-error <file>.tex
  ```
  A plain `\documentclass{article}` with `\usepackage{semio-viz-<package>}` is enough for kernel probes. Full documents need `\documentclass[type=report,theme=light,language=de]{semio}` and the fonts under `🖼️assets/🔤️font` (Kelly Slab may be missing until `bun ./📜️script.ts fonts` works). MiKTeX may auto-install packages on first use.
- The repo TypeScript toolchain (`bun ./📜️script.ts …` in `📦️packages/🟦️typescript`) is being bootstrapped by the BOOTSTRAP agent; read `📓️bootstrap.md` for its state. Do not start a second bootstrap.

## Coding rules (from CLAUDE.md, applied to TeX)
- expl3 with `\ExplSyntaxOn`, `l3keys` for every option, `\NewDocumentCommand` for public macros. Public names `\SemioViz…`, internals `\semio_viz_<module>_…`, variables `\l_semio_viz_<module>_…`.
- `%region 🔖️Name` / `%endregion 🔖️Name` blocks; a one-line emoji-prefixed `%` note above a definition; no comments inside definitions.
- Everything configurable through keys with documented defaults; no hard-coded counts, radii or colours inside renderers — read design tokens through `semio-viz-theme` (colours `semio-…` from `semio-tokens.sty`, strokes `\semio@stroke@…`).
- All user-visible text in both `en` and `de`, selected by the document language (`\l_semio_language_tl` in `semio-core.sty`); never a default language in code.
- No legacy support, no compatibility aliases, no deprecations. Handcraft names. Delete what you replace (only within files you own).
- Add your modules/commands to the sibling `🔣️.json` collection manifests (`🔨️modules/🔣️.json`, `🎮️commands/🔣️.json`) when you create TS modules or commands.
- Register any new nx target in `📦️packages/🟦️typescript/📋️project.json` and any new dev command in `.vscode/🧩️launch.seed.jsonc` following the existing `🖨️print` naming (never edit `.vscode/launch.json` directly).
- Test-driven: write the Gherkin feature and fixture before the implementation; make the probe produce numbers; compare against the third-party oracle. Never claim a test passes without running it; log with `[DEBUG] ` prefix when temporary.

## Test harness contract (see architecture §5)
- Case folder: `🧰️framework/🛍️products/📓️print/🧪️tests/<kebab-case>/` with `🥒️.feature`, `🟦️.ts` adapter, `🧫️fixtures/` (probe `.tex` documents and JSON vectors).
- Probe protocol: `\usepackage{semio-viz-probe}`; `\SemioVizProbeBegin{case}{scenario}`, `\SemioVizProbeValues{key}{clist}`, `\SemioVizProbePoints{key}{{x,y}{x,y}…}`, `\SemioVizProbeOn` (families/marks emit `geometry/<primitive>` records through `\semio_viz_probe_geometry:nn {kind} {numbers}`). Output: `\jobname.probe.jsonl`, one JSON object per line `{"case","scenario","key","values":[…]}`.
- The TS side (`🔨️modules/🧪️viz-probe/🟦️.ts`, TESTS-HARNESS agent) compiles a probe with tectonic and returns the records; adapters call it as subject and a d3 package as oracle. Until it exists, verify your probes by compiling locally and inspecting the `.probe.jsonl`; write the feature + fixtures + adapter anyway against the contract in `📓️status-TESTS-HARNESS.md`.

## Coordination
- Family registry: `\SemioVizFamily{name}{code}` (`semio-viz-family.sty`); inside `code`, `#1` is the option list. Options are `l3keys` in `semio / viz / family / <name>`; document every key (type, default, meaning) in a `%region 🔖️Keys` block at the top of the family.
- Catalogue: add entries for your sections to `🖼️assets/🔣️viz-catalog.json` (array `kinds`, shape in architecture §4 and `🧬️schema/🔣️.json`). If the file does not exist yet, create it with `{ "schemaVersion": 1, "kinds": [] }` and append; keep the array sorted by `id` (numeric section, then slug). Edit only entries whose section you own; re-read the file immediately before each edit and write the whole file back atomically to avoid clobbering concurrent edits.
- When you need a kernel capability another agent owns, write it in your own package under `%region 🔖️Pending-<kernel-package>` and record the request in your status file; the owner moves it later.
- Finish with: files touched list in your status file, all tests you ran with their real output tails, open issues. Delete your logs under `🗑️generated/` you no longer need.
