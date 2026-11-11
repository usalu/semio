# Owner file changes for the shared-worktree model

Status 2026-11-10: sections 1 and 2 (both `AGENTS.md` files) are applied on the owner's request. Section 3 (the owner prompt file) is applied too: all eight locations matched exactly.

Agents may not edit these files (`AGENTS.md` files and the owner prompt file). Each location below gives the exact current text and ready-to-paste replacement text. Line numbers are those of 2026-11-09; search for the quoted text if they drifted. Model: one shared worktree replicated live between 50+ machines by the `shared-worktree` replica daemon, no branches, no pull requests, no merges, `.git` never shared, no git hooks, only the optional archivist machine commits linear history to `main`, pushes, and tags releases.

## 1. Root `AGENTS.md` lines 51-53

Current text:

```md
- You MUST work simultaneously with others on the same files.
  - You MUST NOT use any modifying git command such as `git commit`, `git stash`, `git stash pop`, `git checkout`, … because others will lose their work.
    - You MUST NOT use git worktrees.
```

Replacement (lines 51-53 become the block below; the two bullets after it, `SHOULD NOT stop ...` and `SHOULD ignore ...`, stay unchanged):

```md
- You MUST work simultaneously with others on the same files.
  - The repo is one shared worktree that a replica daemon copies live to 50+ machines (about one second). Files change under you: re-read a file before you edit it. Concurrent writes to the same file resolve last-writer-wins.
  - You MUST NOT use any git command that modifies the repository or the working tree, such as `git commit`, `git stash`, `git stash pop`, `git checkout`, `git switch`, `git reset`, `git clean`, `git restore`, `git rebase`, `git merge`, … because every working-tree change is replicated to all machines within about a second, so others will lose their work. There are no branches, pull requests or merges, `.git` is never shared and no git hooks are installed.
    - Exception: the single archivist machine, when you are explicitly told that you run on it, commits the replica to `main` (linear history), tags releases and pushes, only through the `commit` and `micro-commit` skills. It still never branches, merges, rebases, stashes or checks out.
    - You MUST NOT use git worktrees: the shared worktree is the only one, and a second worktree would be replicated or fall out of sync.
```

## 2. `🧰️framework/🛍️products/🦑️repo/AGENTS.md`

### 2a. Lines 1024-1042: Version section (checkin/checkout mermaid with contributor branches and squash-merge)

Current text (lines 1024-1042):

````md
### 🔀 Version

```mermaid
sequenceDiagram
Contributor->>+repo: checkin
repo->>+git: fast foward `contributor/latest` to  `main`
git->>-repo: ✅
repo->>-Contributor: ✅
Contributor->>+repo: checkpoint
repo->>+git: commit to `contributor/latest`
git->>-repo: ✅
repo->>-Contributor: ✅
Contributor->>+repo: checkout
repo->>+git: create branch `contributor/backup`
git->>-repo: ✅
repo->>+git: squashmerge `contributor/latest` to  `main`
git->>-repo: ✅
repo->>+git: `contributor/backup` to `contributor/YY/MM/DD`
git->>-repo: ✅
````

Replacement (the same `### 🔀 Version` heading stays; the diagram describes the archivist, the only actor that touches git):

````md
### 🔀 Version

All contributors edit one shared worktree that the replica daemon replicates; there are no contributor branches, no squash-merges and no pull requests. Only the archivist machine touches git: it commits the whole replica on a timer to the linear `main`, pushes at most once an hour, and tags releases on `main`.

```mermaid
sequenceDiagram
Contributor->>+repo: checkin
repo->>+replica: settle (the machine's replica is current with the journal)
replica->>-repo: ✅ journal position
repo->>-Contributor: ✅ version = journal position
Note over Contributor,replica: edits are replicated live, last writer wins
Archivist->>+repo: checkpoint (timer)
repo->>+git: commit the replica to `main` (linear, never branches)
git->>-repo: ✅ commit sha
repo->>-Archivist: ✅
Archivist->>+repo: version (release)
repo->>+git: tag `rYY.MM-V` on `main`, push
git->>-repo: ✅
repo->>-Archivist: ✅
```
````

### 2b. Lines 1098-1130: event YAML examples

Current text:

```yaml
  version:
    starting: # e.g. in git pre-commit on main branch.
      DATA:
        new-checkpoint-description: "{{checkpoint-description}}" # e.g. in git commit message

  checkpoint:
    starting: # e.g. in git pre-commit
      DATA:
        new-checkpoint-description: "{{checkpoint-description}}" # e.g. in git commit message
    ended: # e.g. in git post-commit
      DATA:
        old-checkpoint: "{{checkpoint-id}}" # e.g. in git commit sha
        new-checkpoint: "{{checkpoint-id}}" # e.g. in git commit sha
        new-checkpoint-description: "{{checkpoint-description}}" # e.g. in git commit message

  checkin:
    starting:
      DATA:
        checkin-version: "{{version-id}}" # target version id to use as starting checkpoint
    ended:
      DATA:
        version: "{{version-id}}" # new checkin checkpoint id

  checkout:
    starting:
      DATA:
        checkout-checkpoint-description: "{{checkout-description}}"
        checkout-checkpoints: ["{{checkpoint-id-between-checkin-and-checkout}}"] # e.g. in git commit sha of squash checkpoints between checkin and checkout
        archive: ["{{archive-checkpoint-id}}"] # e.g. in git branch name of the archive branch e.g. "kinan/2026/02/24"
    ended:
      DATA:
        id: "{{checkpoint-id}}" # e.g. in new git commit sha of squash checkpoints between checkin and checkout
      description: "{{checkout-description}}" # e.g. in git commit message
```

Replacement (hook wording replaced because no git hooks exist: the events are emitted by the replica and by the archivist's commit script; the `archive` field is removed because there are no archive branches):

```yaml
  version:
    starting: # e.g. in the archivist's release script, before it tags a commit on `main`.
      DATA:
        new-checkpoint-description: "{{checkpoint-description}}" # e.g. in the archivist commit message

  checkpoint:
    starting: # e.g. in the archivist's commit script, before it commits the replica
      DATA:
        new-checkpoint-description: "{{checkpoint-description}}" # e.g. in the archivist commit message
    ended: # e.g. in the archivist's commit script, after the commit (no git hook)
      DATA:
        old-checkpoint: "{{checkpoint-id}}" # e.g. the previous commit sha on `main`
        new-checkpoint: "{{checkpoint-id}}" # e.g. the new commit sha on `main`
        new-checkpoint-description: "{{checkpoint-description}}" # e.g. in the archivist commit message

  checkin:
    starting:
      DATA:
        checkin-version: "{{version-id}}" # target version id (journal position) to settle the replica on
    ended:
      DATA:
        version: "{{version-id}}" # journal position the replica is settled on

  checkout:
    starting:
      DATA:
        checkout-checkpoint-description: "{{checkout-description}}"
        checkout-checkpoints: ["{{checkpoint-id-between-checkin-and-checkout}}"] # e.g. the archivist commit shas on `main` between checkin and checkout
    ended:
      DATA:
        id: "{{checkpoint-id}}" # e.g. the last archivist commit sha on `main` at checkout
      description: "{{checkout-description}}" # e.g. in the archivist commit message
```

Note: replace from the `version:` key (line 1098) to the end of the `checkout:` block (line 1130); keep the `event:` header above it and `session:` (line 1132) as they are.

### 2c. Lines 1391-1396 and 1722-1727 (inside the ranges 1373-1402 and 1704-1733): class diagrams of `event.checkout.starting`

Current text (identical in both places; lines 1391-1396 and 1722-1727 respectively):

```md
class event.checkout.starting {
  checkoutCheckpointDescription
  checkoutCheckpoints
  archive
}
event.checkout <|-- event.checkout.starting
```

Replacement (remove `archive`, there are no archive branches; leave the rest of the diagram, `checkin`, `checkout.ended` and the inheritance edges, unchanged):

```md
class event.checkout.starting {
  checkoutCheckpointDescription
  checkoutCheckpoints
}
event.checkout <|-- event.checkout.starting
```

Verify the second location with `sed -n 1722,1727p`; it must read the same as the current text above before replacing. If your tooling keeps a schema from these diagrams, drop the `archive` property there too.

### 2d. Line 3465 (and the heading at line 17): checkpoint, hook and archive-branch items

Current text (line 3465):

```md
and release/checkpoint/checkin/checkout event families.
```

Replacement:

```md
and release/checkpoint/checkin/checkout event families. In the shared-worktree model a checkpoint is a commit of the archivist machine on `main` and a release is a tag on `main`; both are emitted by the archivist's scripts and the replica, never by git hooks, and there are no archive branches.
```

Optional: rename the heading at line 17 `### Events, Commands, Hooks` to `### Events, Commands, Agent hooks` so it is not read as git hooks (agent hooks of Claude Code, Cursor and others stay).

## 3. Owner prompt file `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md`

### 3a. Lines 10, 16, 22, 30, 38 (identical line, five prompt templates)

Current text:

```
Use the latest wip ueli branch: github.com/usalu/semio/tree/🐙ueli/⛳wip at this commit:
```

Replacement (replace each of the five lines):

```
Use the latest `main` of github.com/usalu/semio (pushed by the archivist machine, there are no other branches) at this commit:
```

### 3b. Lines 550-556 and 564-571: the contributor-branch workflow and the git scripts

Current text (lines 550-555):

```
We are building a general tool that abstracts source control managment tools like git.
It should provide task-focused and simplified
The workflow in git is:
Every person has a working branch `{{contributor-name}}/latest`.
Whenever a person starts working `{{contributor-name}}/latest` is fast-forwarded to the latest `main`.
Every time a person finished working, then a temporary branch `{{contributor}}/YY/MM/DD` is created at the current `{{contributor-name}}/latest`
```

Replacement:

```
We are building a general tool that abstracts source control managment tools like git.
It should provide task-focused and simplified
The workflow in the shared worktree is:
All persons and agents edit one shared worktree that a replica daemon copies live to every machine; there are no personal branches.
Only the archivist machine uses git: it commits the replica on a timer to the linear `main`, pushes it and tags releases `rYY.MM-V`.
```

Current text (lines 564-571):

```
## Later

repo:
Write scripts to handle all git interactions and make them available as vscode launch (create archive branch `{{contributor}}/<{{YY}}/{{MM}}/{{DD}}`, checkout to `main` squash merge `{{contributor}}/latest>` into `main`), etc

- Merge (prioritize me)
- Merge (prioritize main)
- Pull # fast forwards `{{contributor}}/latest>` to `main`
```

Replacement:

```
## Later

repo:
Write scripts for the archivist machine only (available as vscode launch): commit the replica to `main`, push `main` (at most once an hour), tag a release `rYY.MM-V`, import an external git history (see `.agents/skills/merging/SKILL.md`).
Fleet machines have no git scripts: their sync is the replica daemon (`join`, `run`).

- Import (archivist only, external history)
- Push (archivist only)
- Tag (archivist only)
```

### 3c. Lines 2976-2981: git hook for commit messages

Current text (lines 2976-2978):

```
Create a git hook that automatically creates a commit messages. If the commit contains new tickets `.repo/🎫️/{{YY}}/{{MM}}/{{DD}}/*/ticket.json` add them to the description. Only change the commit message for branches that match `{{dev-emoji}}{{dev-alias}}/🏗️dev`

scheme:
```

Replacement (the `scheme:` block that follows stays; optionally rename `CommitCountSinceLastMerge` to `CommitCountSinceLastBundle` in the template and the example, since a merge no longer exists):

```
Create a commit message generator (a script of the archivist machine, not a git hook: no git hooks are installed) that creates the commit message of the archivist's commit. If the commit contains new tickets `.repo/🎫️/{{YY}}/{{MM}}/{{DD}}/*/ticket.json` add them to the description. The archivist commits only on `main`.

```
