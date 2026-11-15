# g1-m01 Replica Upgrade Record (2026-11-12)

Result: **stopped at step 1** — `verify` exited 1 (drift against the store). The running replica was not stopped; steps 3–6 were not run.

- origin: g1-m01
- account: ueli
- OS: macOS
- worktree: /Users/Shared/semio
- old pid (`.cache/f/daemon.lock`): 44206 (started Thu Nov 12 13:48:01 2026, `bun ./📜️script.ts run`, still running)
- new pid: none (no restart performed)

## Step 1

- `🌿️git` and `🩺️attest` exist beside `🔁️replica`: yes
- sha256 of `🔁️replica/🟦️.ts` before verify: `b995a53c39da1d81104ada63488f3de4bd9944502e1e1e9de026da9f23f1d5fb`
- sha256 of `🔁️replica/🟦️.ts` after verify: `a01f4e42df35f54137ec06cbe81137c84b36b7fc68c67db06510e77834b770f1`
- `repo ./📜️script.ts verify --lang en`: **exit 1**

Verify output (progress events omitted, refused-path list truncated by the command itself to the first 50):

```
Drift on 41 paths (272381 shared paths checked).
  state differs: .git/compose-metrics-cache.json
  state differs: .git/compose-micro-commit-active
  state differs: .git/compose-micro-commit-bullets
  state differs: .git/compose-micro-commit-digest
  state differs: .git/config
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/index.bin
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/metadata.json
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/postings.bin
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/index.bin
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/metadata.json
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/postings.bin
  state differs: .git/description
  state differs: .git/gkcommittemplate.txt
  state differs: .git/hooks/applypatch-msg.sample
  state differs: .git/hooks/commit-msg.sample
  state differs: .git/hooks/fsmonitor-watchman.sample
  state differs: .git/hooks/post-update.sample
  state differs: .git/hooks/pre-applypatch.sample
  state differs: .git/hooks/pre-commit.sample
  state differs: .git/hooks/pre-merge-commit.sample
  state differs: .git/hooks/pre-push.sample
  state differs: .git/hooks/pre-rebase.sample
  state differs: .git/hooks/pre-receive.sample
  state differs: .git/hooks/prepare-commit-msg.sample
  state differs: .git/hooks/push-to-checkout.sample
  state differs: .git/hooks/sendemail-validate.sample
  state differs: .git/hooks/update.sample
  state differs: .git/index
  content differs: .git/index
  state differs: .git/info/refs
  state differs: .git/logs/HEAD
  state differs: .git/logs/refs/heads/⛳wip
  state differs: .git/logs/refs/heads/🐙ueli/⛳wip
  state differs: .git/logs/refs/remotes/origin/HEAD
  state differs: .git/logs/refs/remotes/origin/🐙ueli/⛳wip
  state differs: .git/logs/refs/stash
  content differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🌿️git/🟦️.ts
  content differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/⌨️command/🟦️.ts
  content differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📖️texts/🔣️.json
  content differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🟦️.ts
  content differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧬️schema/🔣️.json
87 paths are refused by the path policy and not shared:
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/wbs-tools/sb-config/🎨️styling
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/basetree/🧰️framework/🔨️modules
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🔨️modules
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⏱️profile
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎯️host-platform
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎯️targets
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📊️reporting
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📐️conformance
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔌️backends
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔒️leases
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/host-capacity
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/⚙️operations
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌐️platform
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌐️remote-cache
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎯️affected
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🐹️go
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👁️evidence
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👷️workers
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📋️task-plan
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️conformance
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔁️probe
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔗️closure
  ... and 37 more
```

## Observations

- 36 of the 41 drift paths are under `.git/` (raw `.git/index`, `.git/logs`, `.git/config`, hook samples, tool caches). The contract's Rollout section says old daemons keep publishing these and new daemons drop them.
- 5 drift paths are `content differs` on the module's own sources (`🌿️git/🟦️.ts`, `🔁️replica/🟦️.ts`, `🔁️replica/⌨️command/🟦️.ts`, `🔁️replica/📖️texts/🔣️.json`, `🔁️replica/🧬️schema/🔣️.json`): the local worktree copy of the new replica code does not match the store.

## Steps 2–6

- Step 2: pid 44206 read and noted, replica is running.
- Steps 3–6 (stop, wait for new pid, `status`, `fleet --wait 120`): not run, per the stop condition in step 1. status exit code: n/a. fleet exit code: n/a.

## Note on the sha256

`🔁️replica/🟦️.ts` changed during the run: the sha256 taken before verify and the one taken a few minutes later differ. The module sources are still being edited and replicated, so the code on this machine was not yet stable when verify ran. Rerun the upgrade from step 1 once the sources are settled.

# Continuation after the coordinator answer (2026-11-12 16:07–16:13 PST)

Result: **replica restarted on the new code; `verify` shows no drift; `fleet --wait 120` exited 1** (g1-m03 has no attestation; Git state is still waiting for the base stage).

- old pid: 44206
- new pid after `kill -TERM 44206` (sent once, 16:08:40, exit 0): **59826**, in `daemon.lock` after 2 s; launchd restarted it
- pid now: **60477** (started 16:11:39). The daemon restarted itself: `📒️journal/🟦️.ts` changed after 16:07 and the attested code digest moved from `1001ce4e940a` to `c2d15c800191`. No second kill was sent.
- current `daemon.lock`: `{"version":1,"pid":60477,"startedMs":1794528699690,"originId":"g1-m01"}`

## Hashes before the stop (sampled every 10 s, 16:07:26–16:08:37, unchanged)

- `🔁️replica/🟦️.ts`: `a01f4e42df35f54137ec06cbe81137c84b36b7fc68c67db06510e77834b770f1`
- `🌿️git/🟦️.ts`: `196abadcd2a573372ce542bacf180a7fce91e9c1700b8f864b90546abf847108`

Both are unchanged at 16:12:21.

## status, 16:08:47 (pid 59826) — status exit=0

```
Replica g1-m01 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 59826)
272371 files tracked; queued: 0 incoming, ? outgoing; 0 overwritten files kept in .cache/f/lost
Git state: waiting for the base stage of the group server; stage and commit state is not shared on this machine yet.
g1-m01: 0 events (0 B) behind, last event 30s ago
g1-m02: 1 events (376 B) behind, last event 6s ago
g1-m03: 0 events (0 B) behind, last event 30495s ago
g1-server: 0 events (0 B) behind, last event 15s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 - - - - - missing (no attestation)
  g1-m02 darwin 6s 0/0 on 1001ce4e940a ok
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 1s 0/0 on 1001ce4e940a ok
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

## verify, 16:08:54 (13 s after the restart) — verify exit=1

35 `.git/` paths plus `g1-m02-upgrade.md` (being written by g1-m02 at that moment); no module source drift.

```
Drift on 36 paths (272336 shared paths checked).
  state differs: .git/compose-metrics-cache.json
  state differs: .git/compose-micro-commit-active
  state differs: .git/compose-micro-commit-bullets
  state differs: .git/compose-micro-commit-digest
  state differs: .git/config
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/index.bin
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/metadata.json
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/postings.bin
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/index.bin
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/metadata.json
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/postings.bin
  state differs: .git/description
  state differs: .git/gkcommittemplate.txt
  state differs: .git/hooks/applypatch-msg.sample
  state differs: .git/hooks/commit-msg.sample
  state differs: .git/hooks/fsmonitor-watchman.sample
  state differs: .git/hooks/post-update.sample
  state differs: .git/hooks/pre-applypatch.sample
  state differs: .git/hooks/pre-commit.sample
  state differs: .git/hooks/pre-merge-commit.sample
  state differs: .git/hooks/pre-push.sample
  state differs: .git/hooks/pre-rebase.sample
  state differs: .git/hooks/pre-receive.sample
  state differs: .git/hooks/prepare-commit-msg.sample
  state differs: .git/hooks/push-to-checkout.sample
  state differs: .git/hooks/sendemail-validate.sample
  state differs: .git/hooks/update.sample
  state differs: .git/index
  state differs: .git/info/refs
  state differs: .git/logs/HEAD
  state differs: .git/logs/refs/heads/⛳wip
  state differs: .git/logs/refs/heads/🐙ueli/⛳wip
  state differs: .git/logs/refs/remotes/origin/HEAD
  state differs: .git/logs/refs/remotes/origin/🐙ueli/⛳wip
  state differs: .git/logs/refs/stash
  state differs: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m02-upgrade.md
87 paths are refused by the path policy and not shared:
  ... and 37 more
```

## fleet --wait 120, 16:09:27–16:11:28 — fleet exit=1

Progress lines "Waiting for agreement: N origins pending, S s left." omitted; final table:

```
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 1s 0/0 on 1001ce4e940a ok
  g1-m02 darwin 5s 1/0 on 1001ce4e940a lagging (not settled)
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 1s 0/0 on 1001ce4e940a ok
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

## status, 16:11:35 — status exit=0

```
Replica g1-m01 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 59826)
272336 files tracked; queued: 0 incoming, ? outgoing; 0 overwritten files kept in .cache/f/lost
Git state: waiting for the base stage of the group server; stage and commit state is not shared on this machine yet.
g1-m01: 0 events (0 B) behind, last event 198s ago
g1-m02: 0 events (0 B) behind, last event 175s ago
g1-m03: 0 events (0 B) behind, last event 30663s ago
g1-server: 0 events (0 B) behind, last event 4s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 2s 0/0 on c2d15c800191 ok
  g1-m02 darwin 2s 0/0 on c2d15c800191 ok
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 3s 0/0 on c2d15c800191 ok
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

## verify, 16:11:36 — verify exit=0

```
No drift: 272336 shared paths agree with the state and the store.
87 paths are refused by the path policy and not shared:
  ... and 37 more
```

## status, 16:12 (pid 60477) — status exit=0

```
Replica g1-m01 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 60477)
272336 files tracked; queued: 0 incoming, ? outgoing; 0 overwritten files kept in .cache/f/lost
Git state: waiting for the base stage of the group server; stage and commit state is not shared on this machine yet.
g1-m01: 0 events (0 B) behind, last event 260s ago
g1-m02: 0 events (0 B) behind, last event 237s ago
g1-m03: 0 events (0 B) behind, last event 30725s ago
g1-server: 0 events (0 B) behind, last event 66s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 3s 0/0 on c2d15c800191 ok
  g1-m02 darwin 3s 0/0 on c2d15c800191 ok
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 2s 0/0 on c2d15c800191 ok
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

## Open points

- `Git state` still prints `waiting for the base stage of the group server`, not `shared`.
- `fleet` cannot reach agreement while g1-m03 has no attestation (its last event is about 8.5 h old). g1-m01, g1-m02 and g1-server are `ok` on the same code digest.
