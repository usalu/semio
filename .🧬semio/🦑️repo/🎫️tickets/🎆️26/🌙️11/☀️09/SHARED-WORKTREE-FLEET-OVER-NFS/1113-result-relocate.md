# Result: existing repository becomes the fleet worktree (2026-11-13) — LANDED

Status: landed under `1113-landing-protocol.md` (three-way merge, entry in `1113-landings.md`). Every test file of the module passes against the live tree: git 36, tiers 57, journal 127 + store 18, state 8, relocate 11, bootstrap 12, chaos 9, replica git 19, race 14, replica store 8, replica 85, adopt 11 + command 4, access 5, topology 60, boundary 39, attest 61; PowerShell suite 20 pass in the arm64 Linux container (no pwsh on this Mac). No daemon was touched, no real machine relocated, the old pre-landing state of the live files is in `1113-landings.md` as hashes.

Changes forced by the warm-restart landing: cursors and clock now live inside `state.ckp`, so `relocate` reads them by opening the old replica and writes `cursor.json`/`clock.json` (the old-format takeover) into the new tree; the new tree's open reconciles the store fold into its queue, so adopt refuses only when local file rows exist. The working copy is no longer needed and is deleted.

Follow-ups written in the ticket folder: `1113-fleet-setup-existing-repo.md`, `1113-relocate-prompts.md` (four prompts). Repo-wide grep for `/Users/Shared/semio` and `/srv/semio` outside the module: nothing in `.agents/`, `.github/`, `.devcontainer/`, the hub; only an unrelated blob path in `🖥️server/🎛️coordinator/.env.example` (`UPLOAD_BLOB_ROOT=/srv/semio/blob`) and a fake workspace string in a remote-exchange test; inside the module the README line was rewritten and the fixtures/tests now use templates.

## Commands to relocate THIS Mac (g1-server, account ueli) — run by the coordinator, dry run first

Precondition: `~/Documents/semio` must contain the landed module (update that repository with the owner's normal git workflow; the script says so if the module is missing), the account is logged in at the screen with automatic login on, and the other machines are told. Use the addresses of `address-environment.md`.

```
export SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=<g1 host> SEMIO_FLEET_NETWORK=<g1 worker CIDR>
unset SEMIO_FLEET_CACHE_ENDPOINT
rm -rf /tmp/semio-topology && cp -R "$HOME/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology" /tmp/semio-topology
# 1. dry run: mounts nothing new, prints `relocate --dry-run` (counts, decisions, git relation); record it, stop if more than 25 conflict/deletion decisions or relation `unrelated`
bash /tmp/semio-topology/🥾️provision/🐚️.sh g1 g1-server --user ueli --server --plan
# 2. real run (replaces the system daemons by LaunchAgents, relocates, restarts, verifies); Allow the one prompt "repo would like to access files in your Documents folder"; exit 4 = consent pending
bash /tmp/semio-topology/🥾️provision/🐚️.sh g1 g1-server --user ueli --server
# 3. verify from ~/Documents/semio/<replica directory>
repo ./📜️script.ts status --lang en; repo ./📜️script.ts verify --lang en; repo ./📜️script.ts fleet --wait 180 --lang en
```
`/Users/Shared/semio` stays, retired (`.cache/f/relocated.json`, sync off); delete it only when sure.

## Design decisions

1. Path model. `topology/🔣️.json` `worktree` is a template: `linux ~/semio`, `macos ~/Documents/semio`, `windows C:\semio`. Schema `HomeRoot` (`^~(/seg){1,4}`), `worktreePathV1(topology, platform, home)` resolves it (spaces, non-ASCII kept; relative home or `/` refused); service definitions keep the placeholder `__WORKER_HOME__`; `facts --home`, `preflight --home`. The provision script reads the template with sed, resolves it against the account home from `dscl`/`getent` (not `$HOME` of root) and fills the placeholders with sed/XML-escaped values.
2. macOS privacy, experiment run here (evidence in `1113-relocate-tcc/tcc-evidence.txt`, probe scripts `probe.sh`, `probe.js`; the probe folder, agents and files are deleted): a LaunchAgent in `gui/501` started through `tccd` log capture.
   - Job program = bun directly: the accessing and responsible process is bun, the grant is keyed by the content-addressed bun path (lost on a runtime update). It created one stray consent entry for that bun path (the owner clicked Allow within 4 s); it can be removed in System Settings.
   - Job program = `/usr/local/bin/repo` (the launcher `execs` bun): `responsible` stays the launcher (`~/.local/share/semio/launcher/bin/repo`), bun is only `accessing`. The launcher already held a Documents consent (no prompt, authValue 2, authReason 2) and the whole chain, including the bun under `.cache/b/...` inside `~/Documents/semio`, read the folder. So one consent for the launcher survives runtime updates; it is lost only when the launcher binary is rebuilt (ad-hoc signature).
   - A shell parent does not become responsible (child bun was its own responsible). A copy of `/bin/sh` cannot run (AMFI launch constraint). Not tested: a system LaunchDaemon (needs root) and Full Disk Access (needs the settings UI).
   - Chosen: option (a). All macOS services that run `repo` (replica, compact, relay, cache; they all start the launcher in the worktree) are LaunchAgents in the user's gui domain (`LimitLoadToSessionType Aqua`, logs in `~/Library/Logs`); store mounts stay LaunchDaemons. Provisioning removes the old system daemons of the same labels, loads agents with `launchctl bootstrap gui/<uid>`, warns when no session exists, waits 90 s for the daemon and exits 4 with the exact prompt to accept. The server Mac needs auto-login for cache and relay.
   - `worktree-access-denied`: the launcher cannot read the folder before any of our code runs, so the signal comes from (i) `runV1`, which waits in `awaitWorktreeAccessV1` while the worktree listing is refused (EPERM/EACCES) or does not return, writing `origins/<origin>/access.json` and a `blocked` event, (ii) `status` (field `access`, text with the hint), `status --json`, `fleet` (exit 1) and (iii) the macOS agent check and a new preflight check `access`. There is no attestation field: a daemon that cannot start cannot attest.
3. Adopt-join (`🔁️replica/🧲️adopt`, `join --adopt [--dry-run] [--force] [--sample N]`). Classification `classifyV1` (same / materialise / publish / conflict-local / conflict-remote / resurrect / delete-local / nested) with `conflictWinnerV1`; ignore rules are the union of local and fleet rules; nested repositories are excluded from every adopt action; git objects are united first, no common commit is refused unless `--force`; branch refs go through the existing `linearizeRefV1` path of `materialise` (never drops a commit); the old index is copied to `lost/adopt-<stamp>/index`, the fleet stage replaces it, and when HEAD moved the stage is rebuilt on the new HEAD and republished as a new base (`restage`). Markers `.cache/f/adopt.json` per step make it resumable. The adopt publishes only an explicit list (local-only files, newer-winning local files, git refs and unseen objects), never a full scan, which is the incident requirement: state is rebuilt from the store fold, nothing is published as new.
4. Relocate (`🔁️replica/📍️relocate`, `relocate --to <path> [--worktree <old>] --phase prepare|move|finish|all [--dry-run] [--force] [--wait S]`): identity (`machine.json`, cursors, clock; the stronger clock wins; origin id unchanged), adopt in the new tree, `finish` verifies (and with `--wait` requires `fleet` ok) and then writes `relocated.json` + sync off in the old tree; the old tree is never deleted. Markers in both trees, re-runnable.
5. Provisioning accepts a worktree that holds the account's own repository (adopt), `--plan`, `--force`, `--relocate-from` (also detected from the installed replica service); the bootstrap archive is only for machines without a checkout.

## Files (all under the module; counts of the diff in the working copy)

Changed: `🗺️topology/🔣️.json`, `🧬️schema`, `🟦️.ts`, `⌨️command/🟦️.ts`, `📖️texts/🔣️.json`, `🥾️provision/🐚️.sh`, `🥾️provision/🔵️.ps1`, `🧪️tests/🟦️.ts`, `🐍️.py`, `🧫️fixtures/🔣️.json`; `🔁️replica/🟦️.ts` (seven `export` keywords on `applyQueue`, `enqueueId`, `persist`, `foldInto`, `guarded`, `filesKey`, `settle`; import and one line in `runV1`; `access` in `StatusReportV1` and `statusV1`), `🔁️replica/⌨️command/🟦️.ts`, `🔁️replica/🧬️schema/🔣️.json` (`AccessRecord`, optional `Status.access`).
New: `🔁️replica/🧲️adopt/**` (code, schema, texts, tests incl. `⌨️command.ts`, Python oracle, fixture), `🔁️replica/📍️relocate/**`, `🔐️access/**`.

## Tests (macOS, working copy on the 05:10 live code)

topology `🟦️.ts` 60 pass; adopt `🟦️.ts` 11 pass (classification vectors + Python oracle, diverged repository with three replicas, dry run changes nothing, crash and resume after each of seven steps, no-common-history refusal and `--force`, already joined), adopt `⌨️command.ts` 4 pass; relocate 11 pass (origin id, cursors, clock kept; dry run; running-daemon refusal; resume after each of eight steps; refusals); access 5 pass; replica `🟦️.ts` 77 pass. Not run again after the last edits: journal, boundary, git, attest, replica `🏁️race.ts`, `🌿️git.ts`, `🥾️bootstrap`, `💾️state`, `🎣️tiers` suites.

## Not verified

- The real `launchctl bootstrap` of the new agents, the LaunchDaemon and Full Disk Access paths, Linux/Windows daemons, the PowerShell script on real Windows.
- Nested repositories: adopt leaves them alone, but the steady-state boundary still shares a nested repository's working files (not its `.git`).
- The dry run reports `unknown` when the fleet tip object is not yet in the local repository; the real run decides after the object union.
- The oracles' `jsonschema` check is skipped when the module is not importable under `python -I`.
- One stray consent entry for a bun runtime path (from my probe, clicked by the owner) can be removed in System Settings.
