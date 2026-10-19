# Result docs: developer documentation matches the current build CLI

Executor `docs`, 2026-10-18. Sources: `brief-executor.md`, `coordination.md`, `HANDOVER.md`, the two memory notes, `bun repo --help` (ran, prints the usage line), `.cargo/config.toml`, the budget `🔣️policy.json`, the native-build `ImageProtocol`, `result-gocanon.md`, `result-seed.md`, `result-sweep.md`. No `AGENTS.md`, `package.json` or `bun.lock` edited. No build, no modifying git command.

## Files changed
- `README.md`: new section "Build" under Development (TOC entry added): `bun repo run`, `--tool=bazel`, `tools setup`, `dependencies sync|lock`, `cargo --store=shared` (raw cargo refused), cache partitions `.cache/b/<d,t,g,o,p,l,n>`, launch seed + `workspace:generate-launch`, Go canonical layout, zero-touch rule. The CI/CD paragraph now points to it instead of duplicating the Bazel/tool text.
- `🧰️framework/🛍️products/🦑️repo/README.md`: new sections "Build and run", "Cache partitions" (partition table, APFS images, Linux/Windows backend note), "Recovery" (six steps from the incident notes), "Launch catalog", "Go layout".
- `.agents/skills/build/SKILL.md` (new, short): commands, rules, startup-refusal triage. No existing skill described building or running.
- `.agents/skills/clean/SKILL.md`: never-removes list now covers `.cache/b/*` images and open-ticket `🗑️generated`.
- `.devcontainer/README.md`: tool provisioning is `bun repo tools setup all`, not a "locked tooling bootstrap".
- Stale Nx wording replaced by tool-neutral wording in: `🧰️framework/🛍️products/❓️quiz/README.md`, `🐾️pets/README.md`, `📓️print/README.md`, `🎓️teaching/README.md`, `🎓️teaching/🛂️proctor/README.md`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md`.

## Verification
- `bun repo --help` prints the documented usage (run/run-many/affected/exec/graph/show/watch/reset/tools setup/dependencies/cargo). `bun repo cargo` without arguments fails, matching the documented gate.
- Partition names, capacities and the rust partition path read from `📦️budget/🔣️policy.json`; APFS from `ImageProtocol`; target names `workspace:generate-launch(-check)` read from the root `📋️project.json`; Go layout from `result-gocanon.md`.
- Code fences balanced in both READMEs. Heading anchors for emoji headings are a best guess (`#-build-`, `#-build-and-run`, `#-recovery`); the repo's own TOC anchors are inconsistent for emoji, so check once on GitHub.

## Open
- Not verified live: the recovery steps were taken from the incident notes, not replayed. The Linux/Windows backend (`💾️storage/🧱️fixed`) is described from code reading only.
- `AGENTS.md` line 7 still says `package.json` must call `nx`; forbidden to edit, reported before in `result-nxneutral.md`.
- Left unchanged: dashboard README ("actual Nx native command runner") and per-plugin `AGENTS.md` files that say "via nx" (forbidden). Docs under `.cursor/plans` and tickets are history.
- The `README.md` "four golden paths" still document `bun run dashboard`; not re-verified here.
