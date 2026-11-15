# Result: content-based adopt and relocate (2026-11-13)

Module `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Landed under `1113-landing-protocol.md` (lock `1113-landing.lock`, each of the 11 files replaced only while byte-identical to the base it was built on, `🧰️scripts/1113-adopt2/land.py`, hashes in `1113-landings.md`). Plan for this Mac: `1113-adopt-plan-g1-server.md`.

## The rule (no modification time anywhere in the decision)

Per differing path the adopt knows: the fleet's history of the path (every sha256/link target any event of any origin left there, streamed from all journal segments, plus the blobs the path had in the commits of the fleet's history that the repository holds: fleet tip and every ref value the journals ever recorded for `.git/HEAD`, `refs/**`, `packed-refs`) and the repository's base (the path's content at the fork point of HEAD from the fleet's history, i.e. HEAD itself when the repository has no commits of its own; so locally committed work counts as local work).

| class | condition | effect |
|---|---|---|
| same | equals the fleet now | adopted |
| stale | content seen in the path's history, or unchanged since the base while the fleet holds something else, or fleet-deleted (history/base) | fleet wins, nothing published, bytes in `lost/adopt-<stamp>/stale/` |
| publish | no fleet entry (or a delete with the base never having the path), content unseen | published as new work |
| edit | unseen, fleet's state equals the base | published as a new edit |
| conflict | unseen, fleet moved since the base (or no base, or fleet deleted an edited file) | fleet's version stays; local version in `lost/adopt-<stamp>/conflict/<path>`; listed with both hashes in the report; counted and listed by the dry run |

Conflict form, chosen and justified: the local version goes to `lost/`, not to a sibling file beside the path, because a sibling would be published to all eight machines, pollute the shared tree and could trip taxonomy gates; `lost/` keeps the bytes on the one machine whose owner decides, and the report names every conflict. A local deletion is not local work (the fleet file comes back).

Git refs and state follow the same spirit: `.git/HEAD`, `packed-refs`, `shallow` and non-branch refs always go to the fleet (local copy in `lost/adopt-<stamp>/git/`), local-only refs and every local stash commit are lifted into loose refs first (`refs/adopt/<stamp>/...` for differing non-branch refs), loose branch heads keep the commit-ancestry linearisation. The queued reconcile of a relocated tree is detached from the objects drain so no regular file is applied by the old modification-time logic before the decisions are made; the decisions run after the common-history refusal.

## Changed files (all under M)

`🔁️replica/🧲️adopt/🟦️.ts` (classifier `classifyV1`/`reasonV1`, journal history, git evidence, fork base, persisted decisions `adopt.json.decided`, set-aside, ref lifting), its schema (`Counts`, `Stale`, `Conflict`, `Reason`, report keys), texts (EN+DE), tests (`🟦️.ts`, `⌨️command.ts`, fixture of 28 vectors, Python oracle `🐍️.py`), `🔁️replica/⌨️command/🟦️.ts` (plan output, thresholds 50 conflicts / 1000 new-work paths), `🔁️replica/📍️relocate/🧪️tests/🟦️.ts` (incident test through relocate), `README.md`, comment of `🗺️topology/🥾️provision/🔵️.ps1`. Relocate needed no code change: it calls `adoptV1`.

## Tests

New/changed: 28 classification vectors incl. the incidents (old version with a fresh mtime, resurrected delete, both changed, mtime moved over 0/1/year 2100 never changes the result) decided identically by the TypeScript rule and the independent Python oracle; diverged-repository adoption (every class, reasons `journal`, `git`, `both-changed`, `deleted-by-fleet`, refs/stash lifting, convergence of three replicas, fleet files untouched); dry run (counts, conflict list with reasons, evidence, nothing changes); crash and resume after each of 7 steps; unrelated repository refused, forced one decides by content (`base` reason); relocate into a stale repository with a future mtime; CLI in EN and DE; property test, 12 seeds: adopting an old snapshot of the fleet with random local edits and random mtimes never changes a fleet path the repository did not edit and never loses a local edit (fleet content or a copy under `lost/`). Mutation check: making `stale` publish as `edit` fails 11 of 12 seeds. Third-party validation: the report is validated against the JSON schema with `jsonschema` (Python), content hashes and blob ids are cross-checked against `git` (`ls-tree`, `log --raw`, `hash` of git objects).

Whole module run against the live tree before the lock was released: see the table below.

## Known limits

- Evidence beyond the journals is limited to commits the repository holds; a repository without any commit of the fleet's history relies on journals and its own base.
- Content filters (autocrlf, clean filters) make a file differ from its base blob: such a file becomes a conflict, never a loss.
- A fresh `join --adopt` no longer publishes unknown old local files as new work when the fleet had their content (the earlier "Known limit"); tombstones older than 90 days are not in the journals, such a file is `publish`.
- Segments the compaction may retire later shrink the journal evidence; the git evidence covers tracked files.
