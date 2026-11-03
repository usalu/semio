# 🚀 Fleet Wave 3 Preamble (2026-11-03 15:00)

Read /home/deck/semio/AGENTS.md and TICKET/fleet-wave2-brief.md (ALL sections, incl. "Schema evolution tool", "No mechanical warning fixes", "Full rustc diagnostics", "No agent-started repo run").
- 20 agents run in parallel. Memory is the bottleneck: run every `repo ./📜️script.ts …` and every heavy Python sweep through `TICKET/🗑️generated/build-runner/heavy.sh`, in the foreground, one at a time, with `nice -n 19`. Keep each tool call under 2 minutes; never poll with sleep loops; never leave background jobs when you finish.
- Never run cargo/rustc/`repo run`. Builds are run by the runner; request them with "- ready for rebuild: <what>" in `TICKET/🗑️generated/build-<code>/latest.md`.
- Check `TICKET/🗑️generated/build-runner/freeze.txt` before every edit batch; never edit an artifact while it is named there.
- Schema-first: schema JSON → generator (or evolve.py) → layers. Never hand-edit generated twins. No defaults that invent values; never invent normative values (unverified registry rows with a source note).
- Recurring compile pitfalls to avoid: typed enums used as String (imports, `&text`, `.as_str()`, Display), `nested_patch!` order/duplicates, missing `row_key!`, `crate::` across `-more` splits, missing Cargo deps (plus Cargo.lock entries validated with tomllib), mutation kind must equal the derive's to_kebab(variant) (law test kind-kebab), visibility of types used in pub(super) items, new collections in every snapshot initializer.
- Repo law tests to keep green: kind-kebab, binary-tags, i18n-mutations, i18n-sinks, remedy-law, ndp-registry(-law), handle-paths, source-budget, path-budget, spec-trace, schema-sync, fixture checker.
- Report to TICKET/🔍️audits/wave3/<your-name>.md. No git writes. Final reply: 5 lines + path.
- Infra (W3-INFRA): the runner now runs `repo run` with `NX_DAEMON=false` (no Nx daemon, so no inotify watch-limit failures) and holds a graph-plan gate (needs available RAM >= 5700 MB and RAM + free swap >= 7000 MB, then waits for the running heavy command). While a build is planning (`freeze.txt` or `build-runner/graph-pending/*`), `heavy.sh` waits unconditionally (up to 45 min); expect "[heavy] waiting: a build is in graph-plan" for up to a few minutes and retry. Only commands that go through `heavy.sh` are paused; do not start other heavy work around it.
- Before creating any crate: grep all Cargo.toml for the package name (duplicate package names break every build's graph-plan repo-wide), and use emoji folder names with the U+FE0F variation selector like the rest of the repo.
- Python: ALWAYS /home/deck/semio/.venv/bin/python -I (numpy/scipy/sectionproperties live there; the system python lacks them).

## Rule reminder (17:44)
- NEVER run any modifying git command (git rm, add, commit, stash, checkout, restore, reset, mv …). One agent tried `git rm --cached` at 18:0x; it was blocked. Leave index/tree state alone; deletions are plain file deletions.
