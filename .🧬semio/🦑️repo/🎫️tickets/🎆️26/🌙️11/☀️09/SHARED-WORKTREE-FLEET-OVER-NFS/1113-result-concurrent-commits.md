# Result: Concurrent commits (2026-11-13)

Module root `MOD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`.

## Final rule

Branch refs (`.git/refs/heads/**`, and `.git/HEAD` while it holds a bare oid) are linearised by the machine that authored the losing value; everything else is last writer wins.

- Trigger (in `materialise`, thin hook `linearise`): a remote `put` of a branch ref is applied while the local ref file holds a commit that is not equal to the incoming one and was written on this machine (known entry missing, published by this origin, or the file differs from the known sha). A local value that was applied from another origin stays plain last writer wins, so only the authoring origin ever rewrites.
- Decision (`linearizeRefV1`): missing object or git failure -> `retry` (event stays queued, `blocked` note engages the cooling back-off); local is an ancestor -> fast-forward; incoming is an ancestor -> keep local ref (`keepNewer`, republished); otherwise rewrite.
- Rewrite: for every local commit not in the incoming history (`rev-list --reverse --topo-order ^incoming local`) `git commit-tree` on top of the running tip, one parent, author, author date, committer and message kept. Tree = three-way merge (`read-tree -i -m --aggressive` in a temporary index: original parent tree, new tip tree, original tree), the original tree if that does not resolve. A commit whose merged tree equals the tip tree and which repeats the tip message, or repeats an author line plus message found in the incoming history, is dropped. `update-ref` compare-and-swaps from the old local value to the result; the next scan publishes it with a greater hlc.
- Duplicate (all dropped): ref takes the incoming commit and the plain apply continues.
- Convergence: each hlc round has one winner that is never rewritten; losers rewrite onto it and publish with greater hlcs; a new round only exists while new concurrent values exist. n concurrent commits need at most n-1 rounds; rewritten intermediates stay unreferenced. A third commit during a rewrite makes the CAS fail (retry) or starts a new round with the same rule. Bystanders apply plain.
- Event: `linearised` (en/de texts `event.linearised`, `event.linearised.duplicate`, printed by `run`) with `path`, `hlc`, `winner`, `loser`, `result`, `reason: duplicate` when nothing was re-created.

Design change against the brief: the tree is the three-way merge, not the losing tree verbatim. Concurrent commits can have trees that lack each other's staged paths; the verbatim tree would drop the winner's change from HEAD and leave the shared stage permanently different from HEAD.

## Files changed

- `MOD/🌿️git/🟦️.ts`: `Run.env`, `branchRefV1`, `RefLinearV1`, `linearizeRefV1` (+ private helpers).
- `MOD/🔁️replica/🟦️.ts`: event union (`linearised`, `winner`, `loser`, `result`), `linearise` hook before the delete/put apply in `materialise`.
- `MOD/🔁️replica/🧬️schema/🔣️.json` (event name, fields), `MOD/🔁️replica/📖️texts/🔣️.json` (two messages), `MOD/🔁️replica/⌨️command/🟦️.ts` (`run` prints the event).
- Tests: `MOD/🔁️replica/🧪️tests/🌿️git.ts` (+4 tests, `cli` takes env), new oracle `MOD/🔁️replica/🧪️tests/🌿️git.py` (git CLI only: linear, no merge, one root, each message once, dangling commits only superseded copies, clean status).
- Docs: `MOD/README.md` (git section "Concurrent commits", "What breaks the model"), `1113-git-state-contract.md` (sentence rewritten), `1113-rollout.md` (one line).

## Tests

Run from the member `🧪️tests` dir with the given bun, `--timeout 30000`, the `SEMIO_*` variables unset.

| Suite | Pass |
|---|---|
| `📒️journal` | 127 |
| `🚧️boundary` | 39 |
| `🌿️git` | 9 |
| `🩺️attest` | 35 |
| `🔁️replica/🧪️tests/🟦️.ts` | 61 |
| `🔁️replica/🧪️tests/🏁️race.ts` | 14 |
| `🔁️replica/🧪️tests/🌿️git.ts` | 10 (6 before + 4 new), three consecutive runs green, then a fourth in the full pass |
| `🔁️replica/🥾️bootstrap` | 12 |
| `🗺️topology/🧪️tests/🟦️.ts` | 56 |

All 0 fail. New tests: two concurrent commits (same HEAD on three replicas, linear, both messages, both changes in HEAD, original commits still present, clean stage, exactly one `linearised` event, none on the idle machine, Python oracle), identical commits (one survives, one `duplicate` event), three concurrent commits (4 commits, all messages and files, at least two events), non-concurrent commit (fast-forward, no event).

## Not verified

- Not run on Linux or Windows; no live fleet run; old daemons (g1-m01..m03) still drop the loser until restarted.
- Real parallel git processes racing the CAS were not exercised, only sequential interleavings through `converge`.
- Conflicting paths (both sides changed one path differently) fall back to the loser's tree; not covered by a test.
- Detached `.git/HEAD` forks use the same code path with `update-ref --no-deref HEAD`; not covered by a test.
- Amend, rebase or force-moved refs are plain last writer wins.
- A loser machine that never runs the daemon again leaves its commit unreferenced.
