# Coordination Rules for All Workers

Ticket folder (abbreviated `$T` below): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️11/DECLARATIVE-TEACHING-MORPHS-AND-FAST-PREVIEWS`
Repository root: `C:\git\semio` (Windows 11 host, Git Bash and PowerShell available; not a Steam Deck).
Kit: `teaching-morph-preview-launch-kit/acceptance.md` (binding verification contract) and `fleet.md`.
Contract: `$T/CONTRACT.md` (coordinator-owned; read before implementing; propose changes through your report, do not edit it).

## Hard rules (from AGENTS.md)

- Never run git commands that modify the repo or working tree (commit, stash, checkout, reset, restore, clean, ...). Read-only git (status, diff, log, show) is fine.
- The tree is shared live with other agents and humans. Re-read a file right before editing it, patch narrowly with Edit, never overwrite unrelated work, never revert others' changes.
- Never edit `AGENTS.md`. Never spawn agents, Claude sessions, or `claude` CLI processes.
- Write JSON and multi-line files with the Write/Edit tools, never via Bash heredocs or `node -e`/`python -c` one-liners (Windows shell strips backslashes and mangles emoji).
- Python modules live at emoji paths and are imported literally: `importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")`.
- Concise code, no comments inside function bodies, every docstring starts with a unique fitting emoji; add reference links in docstrings when useful.
- Schema-first: data contracts are JSON Schema files (`🔣️.schema.json` / `🧬️schema`), code validates against them.
- No backwards compatibility, no legacy shims, no deprecations, no migration scripts left behind: migrate all consumers by hand.
- English first, German second for any user-visible text. Temporary logs start with `[DEBUG] `.
- Every feature gets a language-agnostic test (JSON cases) and an independent third-party oracle where feasible (e.g. NumPy/SciPy/Shapely already in the venv).
- Put scratch scripts and research notes in `$T` (markdown for research), raw command output in `$T/🗑️generated/`.

## Running Python and tests

- Preferred: `RUN_NAME=<you> bash $T/run-tests.sh '<module>' [...]` (sets absolute env paths; a relative `UV_PROJECT_ENVIRONMENT` silently builds a second venv, see FUNDAMENTAL-FIXES.md FD-001).
- An `en` course render of another session is running (see HOST.md): do not start any `en` render outside the queue.
- Teaching venv: `bun 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test <extra unittest modules>` runs the full listed suite (slow). For focused runs use
  `uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest <module>`
  from the repo root with env `PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1 UV_PROJECT_ENVIRONMENT=C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python SEMIO_TEACHING_CACHE_DIR=C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/fonts SEMIO_TEACHING_OUTPUT_ROOT=$T/🗑️generated/<you>`.
- Do not run `python -` reading stdin (hangs on this host): write a script file under `$T` and run it.
- Two `render` runs for the same language at once clobber each other's partial-movie slots. Renders, browsers and benchmarks are heavy jobs: request them from the validation owner's queue in `$T/QUEUE.md` (append a line `REQUEST <worker> <command> <why>`), and only run them when the queue marks them `RUN <worker>`. Unit tests with Manim at test quality are not heavy jobs.
- Other sessions may be running long renders (several python.exe). Never kill processes you did not start.

## Reporting

Finish with a short report: files created/changed/removed, tests run with outcomes, open issues, and any defect found outside your scope (with reproduction). Write longer findings to a markdown file in `$T` and cite its path.
