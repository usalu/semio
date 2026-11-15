# Shared Git State

Binding description of how git state replicates in the fleet. The details of the register protocol are in `1113-git-state-contract.md`; the operator view is the section "Git state" of the module `README.md`.

## What replicates

The allow-list inside the root `.git`: `HEAD`, `packed-refs`, `shallow`, everything below `refs`, and everything below `objects` except names starting `tmp_`, `objects/info/alternates` and every name ending `.lock`. These are ordinary shared files, so a commit is the objects plus the ref file. Everything else in `.git` stays on the machine: `index`, `config`, `logs`, `hooks`, `info`, `description`, `FETCH_HEAD`, `ORIG_HEAD`, the files of an in-progress operation and every IDE file. A nested `.git` is never shared.

A path that becomes private (a boundary change or a `.gitignore` change) is forgotten in the local state and no delete is published for it. An incoming event for a private path is dropped, so an older daemon that still publishes `.git/index`, `.git/logs` or `.git/config` changes nothing on a current one.

## The stage

The stage is not a file on the wire. It is a register per path plus one base register, carried by `link` and `delete` events in the virtual realm below the sealed `.cache/f`:

- base: path `.cache/f/index-base`, `link` target `tree <oid>`;
- entry: path `.cache/f/index/<worktree path>`, `link` target `<mode> <oid>` (stage 0), or `delete` when the path is absent from the stage;
- effective stage = the base tree overlaid by every entry whose hlc is greater than the base's; per register the greater hlc wins.

An older daemon refuses these paths as ignored (they lie below the sealed directory) and drops them, so a mixed fleet is safe during a rollout. The realm is never written to disk.

Publish: on a change of `.git/index` and on every scan, `git write-tree`; equal to the last synchronised tree: nothing; otherwise `git diff-tree -r -z` gives one entry per changed path, and more than 5000 changed paths publish a new base instead. The group server `<group>-server` publishes the first base when the manifest has none; workers wait. Apply: the local stage is published first, then entries go into one `git update-index -z --replace --index-info` batch and a base replaces the stage (delta from the last synchronised tree, else `git read-tree`). A missing object or a held `index.lock` leaves the registers queued and retried. Join: the base, then the newer entries. Verify: the fold of base and entries in a temporary index must equal `git write-tree`.

A machine without `git`, or a worktree without a root `.git`, skips the realm; `status` names the reason and `preflight` warns about a missing git executable.

## Configuration

`.git/config` is normalised locally through git at daemon start, at join and when the first base is applied: `core.autocrlf=false`, `core.filemode`, `core.symlinks`, `core.ignorecase`, `core.precomposeunicode`, `core.fsmonitor=false`, `gc.auto=0`, `maintenance.auto=false`, `core.logallrefupdates=true`; remotes, branches and user keys stay; a `commit.template` outside the worktree is removed.

## Switch

Synchronization is on unless this machine has `.cache/f/sync.json` with `{"version":1,"enabled":false}`. That file is gitignored, so the switch is per machine.

```
repo ./📜️script.ts sync --off --lang en
repo ./📜️script.ts sync --on --lang en
```
