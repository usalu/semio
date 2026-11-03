# Round 3 fleet brief (coordinator-owned)

Ticket `26/11/01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS`, reopened 2026-11-03 for round 3. Every worker reads this file before it starts.

## Paths

- **T** is the ticket folder, `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS`.
- **A** is `C:\git\semio\🎓️teaching\🏛️architecture`. The course series in A are:
  - `⚡️energy` (cooling, ventilation, demand, heating, physics, intro, concepts)
  - `🎯️optimization`
  - `🏗️bim`
  - `📊️excel`
  - `🦗️grasshopper`
  - `🏢️nakagin`
  - `🏙️typology`
  - `🔀️morph`
  - `📐️parametric`
  - `🧲️fit`
  - `📚️course`
- **S** is the canonical handcrafted staffage library, `A\🧍️staffage`. It contains:
  - `🗂️catalog` (assets)
  - `🔨️modules` (for example `📤️export/🎭️cast`, `🌡️thermal`)
  - `🧬️schema`
  - `🌐️site` (the library browser)
  - `🧪️tests` and `🧫️fixtures`
- **V** is the Python bridge, `A\⚡️energy\🏗️authoring\🎨️visuals`. It provides `staffage`, `staffage_motion`, `fitted_staffage` and `pendant_lamp`. Its cast manifest is `V\🧍️staffage\🔣️.json`, and its guard test is `V\🧪️tests\🐍️staffage_only`.
- **The course manifest** is `A\⚡️energy\🎬️clip\🔣️.json`.

## Required reading

Read only what your area needs. All of these are in T.

- `summary.md`
- `contract.md`
- `r2-open-items.md` (the OI-xx ids)
- `bridge-ts-report.md` and `bridge-py-report.md`
- `r2-coordinator-rulings.md`

## Rules (AGENTS.md applies in full)

**Git**
- Do not run any modifying git command: commit, stash, checkout, reset, restore, worktree.
- Other agents and humans edit the same tree. Reread a file just before you edit it, and keep each edit narrow.
- Never revert work that is not yours.

**Agents**
- Do not spawn agents. Do not start other Claude sessions.

**Files**
- Write JSON and TeX only with the Write and Edit tools, never through Bash heredocs or `node -e`, because the Windows shell mangles backslashes.
- `python -` reading from stdin hangs. Write a script file and run that file.
- Your scripts go in `T\🧰️r3-<id>\`. Tool output goes in `T\🗑️generated\r3-<id>\`. Your report goes in `T\r3-<id>-report.md`.
  - The report lists what you changed (file paths), the commands you ran with their results, and what is still open.
- Paths must stay under 256 characters. Do not copy source trees into T.
- Do not use recursive `fs.cpSync` on paths that contain emoji, because node crashes.

**Code style**
- Write concise code.
- Put no comments inside definitions.
- Every docstring starts with a unique emoji.
- Temporary logs carry the prefix `[DEBUG] `.

**Running tests and renders**
- Run builds and tests through the `repo` binary or repo MCP tools where they work.
- Python tests: run `uv run … python -m unittest` with `SEMIO_TEACHING_OUTPUT_ROOT` set. See `T\m9_tests.sh`.
- Staffage vitest: use the alias configs `T\q2-vitest-alias.config.ts` and `T\q2-vitest-react-alias.config.ts`, because `repo dependencies ensure` refuses.
- Render one scene at a time. Using `--jobs 3` together with large casts once filled 42 GB.

**Shared contracts and scope**
- **Contracts:** W05 (sonnet-contracts) owns the scene/cast provenance contract `T\r3-contract.md`.
  - To ask for a change, write `T\r3-requests\<your-id>.md`. Do not edit the contract yourself.
  - Until the contract is settled, design against the current schema and record your assumptions.
- **No legacy:** add no compatibility layers, fallbacks or deprecations, and do not wrap old drawings. A missing asset combination is a validation failure.
- **Scope of the canonical rule:** it covers every depicted staffage entity: people, animals, vegetation, furniture, objects, vehicles, machines and building components.
  - Teaching annotations may stay scene-specific: arrows, axes, equations, captions, and the diagram icons named in `r2-coordinator-rulings.md`.
- **Read-only workers** (haiku inventory, provenance, media and lesson audits) cannot write files. Return the full markdown report as your final message, and the coordinator saves it to `T\r3-<id>-report.md`. Keep the report dense, using tables and file:line references.

## Completion

The last line of your final message is `DONE r3-<id>`, followed by evidence:
- the tests you ran and their pass counts
- the renders you inspected
- the files you changed
