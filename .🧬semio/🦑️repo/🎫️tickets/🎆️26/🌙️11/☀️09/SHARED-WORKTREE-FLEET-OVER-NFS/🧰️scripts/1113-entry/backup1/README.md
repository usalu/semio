# Shared worktree fleet: operator runbook

One logical worktree is edited live by 50 or more machines (macOS, Windows, Linux) in about five groups of ten.
Every machine keeps a local replica on its own NVMe. A replica daemon copies every file that the root `.gitignore`
does not ignore to all other machines within about a second, through a journal (the **store**) on the NFS export
of the machine's group. Ignored files stay local. There are no branches, merges, pull requests or merge queue.

The design contract is `SHARED-WORKTREE-FLEET-OVER-NFS/design-contract.md` in the ticket folder of 2026-11-09.
This file is the operator view. The agent view is the section "Shared worktree fleet" of the
[repo README](../../README.md#shared-worktree-fleet).

**State: staged.** The modules were written on a host without Bun, Node, Python or git. The Linux side (group server,
worker, kernel NFS 4.2, systemd) ran on two real Ubuntu 24.04 arm64 machines, then two group servers and a macOS 27 client ran against each other; the figures marked "measured" below
come from `SHARED-WORKTREE-FLEET-OVER-NFS/linux-nfs-report.md` and `mixed-os-report.md` in the ticket folder, which also list what stayed unverified.
Nothing here has run on a real fleet of 50 machines. Values marked "recommendation" are sizing judgements, not measurements.
Settings are marked **measured** when a run on real hardware shows their effect (Linux with kernel NFS 4.2 and macOS 27 so far) and **by reasoning** when they follow from how the client works but no machine confirmed them (Windows store clients); the table "Client caches" says which is which.
Run day 0 first with one group of two machines before you roll it out to all five.

## Parts

| Directory | Role |
|---|---|
| `📒️journal` | Store format: events, hybrid logical clock, segments, blobs, flat heads, snapshots; single-pass blob copy; hand-written event validators. |
| `🗜️compaction` | Store-wide compaction: where the newest snapshot is and what it covers, the retirement plan for journal segments, the reclaim of unreferenced blobs, the compaction lock and record, the size report of `status`. |
| `🚧️boundary` | What is shared: root `.gitignore` matcher, path policy, scanner, hashing. |
| `🌿️git` | The only place that starts `git`: plumbing interface, the stage as a register in the store, per-machine `.git/config` normalisation, git executable lookup. |
| `🎣️tiers` | How changes are found without operating-system file events: the hint drop directory and the Claude Code hook payload reader, the hot set, the table of directory modification times, the passive latency percentiles. |
| `🩺️attest` | What a worker states about itself (`origins/<origin>/attest.json`), the manifest and code digests, the fleet verdict behind `fleet`. |
| `🔁️replica` | The commands `seed`, `join`, `run`, `status`, `snapshot`, `relay`, `compact`, `compaction`, `verify`, `fleet`, `retire`, `touch`, `bootstrap-archive`, `bootstrap-settle`. |
| `🔁️replica/🥾️bootstrap` | The tar writer, the archive refresh of the relay and the settle step of the bootstrap flow. |
| `🗺️topology` | The topology file and the commands `env`, `mount-plan`, `server-plan`, `worker-service`, `facts`, `validate` (`validate --reachable` also probes the hosts) and `preflight` (checks a provisioned machine). |
| `🗺️topology/🥾️provision` | `🐚️.sh` (macOS, Linux) and `🔵️.ps1` (Windows): provision one worker or one group server from an empty disk. |

## Existing repository, adopt and relocate

The worktree of a machine is the account's existing repository. `join --adopt [--dry-run] [--force] [--sample N]` joins a
non-empty repository whose files differ from the fleet, loss-free and resumable (`.cache/f/adopt.json` names the last finished step):

- identical files are adopted, files missing here come from the store, files only here are published as new work;
- files that differ follow the conflict rule: the newer of the local modification time and the wall time in the fleet's hlc wins and the loser's bytes are kept under `.cache/f/lost/`; a file the fleet deleted is resurrected when it is newer here, otherwise deleted with its bytes kept;
- ignored files (the local and the fleet's `.gitignore`) and nested repositories are never touched by the adoption;
- git: the objects are united first; a repository that shares no commit with the fleet is refused unless `--force`; branch refs follow the linearisation of concurrent commits (every commit kept, one linear history); the old index is copied to `.cache/f/lost/adopt-<stamp>/index`, the fleet's stage replaces it (rebuilt on the new HEAD when HEAD moved); `.git/config` stays local;
- only an explicit list is published (files the store fold lacks, provably newer files, refs, unseen objects), never the whole tree; a worktree with missing state rebuilds from the store fold;
- `--dry-run` prints counts, sample paths, every decision with its winner and the git relation, and changes nothing; `lost/adopt-<stamp>.json` and `.md` report every difference of a real run.

`relocate --to <path> [--worktree <old>] --phase prepare|move|finish|all [--dry-run] [--force] [--wait S]` moves an already
joined machine: `prepare` stops publishing in the old tree, `move` (the old services must be stopped) carries `machine.json`, the cursors and the clock to the new
tree (the origin id never changes and is never reused) and adopts the store there, `finish` verifies the new tree (and the fleet with `--wait`) and only then writes
`.cache/f/relocated.json` and turns synchronisation off in the old tree, which is never deleted. Markers in both trees make every step re-runnable.
The provision script does this by itself when the installed replica service points to another worktree (`--plan` prints the dry run first, `--relocate-from` names the old tree).

### macOS privacy

A launchd job cannot read `~/Documents` without a consent, and a LaunchDaemon has no session to ask in. Measured on macOS 27: the process that launchd starts stays the
responsible binary after it execs the runtime, so one consent for the launcher (`repo`) covers the runtime of every version; a shell parent does not become responsible, and a runtime started directly is
its own responsible path. Therefore every macOS service that runs `repo` (replica, compact, relay, cache) is a per-user LaunchAgent in the `gui/<uid>` domain; the store mounts stay LaunchDaemons.
The first start asks once ("repo would like to access files in your Documents folder"); the consent is lost only when the launcher binary is rebuilt. A machine needs a logged-in session (auto-login on the group server).
While the folder cannot be read the daemon waits, writes `origins/<origin>/access.json`, and `status`, `fleet` and `preflight` report `worktree-access-denied` with the switch to flip.

## Commands

Every command runs as `repo ./📜️script.ts <command> [options]` in the directory that owns it: the replica
commands in `🔁️replica`, the topology commands in `🗺️topology` (both below this module). The current directory only
has to hold `📜️script.ts`: the replica script finds the repository root, and so the default worktree, from its own
location. The scripts have no `--help`; a wrong command or option is refused with a message that lists what is accepted. The
replica commands print their messages in English and German, `--lang en|de` picks one. Options are `--name value` or `--name=value`.

| Replica command | Options | What it does |
|---|---|---|
| `seed` | `--group --origin` `[--worktree --store]` | First machine only: publish the whole shared tree and write `.cache/f/machine.json`. |
| `join` | `--group --origin` `[--worktree --store]` | Materialise the tree from the newest snapshot plus the journal. A non-empty tree must hold only shared files that match the store by hash, otherwise it is refused; matching files are adopted. |
| `run` | `[--worktree --store --poll --scan --debounce]` | The replica daemon. Needs `machine.json`. `--scan` is the floor of the interval of the file sweep in seconds (jittered, 90 by default, see "Finding changes"). NDJSON progress on stdout. Writes its attestation and restarts itself when its code changed (see "Attestation, fleet and self-restart"). |
| `status` | `[--worktree --store --json --deep]` | Daemon, store reachability, queues (`--deep` scans the tree for the outgoing count and for the number of refused paths), overwritten files kept in `.cache/f/lost`, lag per origin and, when the store is reachable, the fleet table of `fleet`. Needs `machine.json`. |
| `fleet` | `[--worktree --store --wait <seconds> --json --diff]` | Whether every worker with sync on holds the same state: one row per origin (the column `view` is `modified/deleted/untracked` of its working tree), exit 0 on agreement, exit 1 naming origin and field otherwise. `--wait` polls once a second with progress lines until agreement or timeout; cancel with a signal (exit 130). `--json` writes one report. `--diff` (not with `--json` or `--wait`) reads the `view.txt` of every origin and prints, per origin whose working tree differs from this machine, the lines only there and the lines only here (40 per side, then `+N more`); exit 1 when any differs or an attested view is unavailable. Needs `machine.json`. |
| `retire` | `<origin> [--reason <text>] [--force] [--worktree --store]`, or `<origin> --undo` | Takes a machine that left the fleet for good out of the `fleet` verdict: writes `origins/<origin>/retired.json` (see "Retiring an origin"). Refused for the own origin, an origin that is not in the store, and an origin that attested within the last 60 seconds unless `--force`. `--undo` re-admits it. Needs `machine.json`. |
| `touch` | `<path>…` or `--stdin` or `--hook` `[--worktree]` | Tells the running daemon of this machine which paths changed, so it publishes them within about a second instead of waiting for a sweep (see "Finding changes"). Paths are absolute or relative to the worktree, outside paths are ignored; `--stdin` reads paths separated by newlines or NUL; `--hook` reads a Claude Code `PostToolUse` payload and always exits 0. Exit 1 only when no path was left to hand over. |
| `snapshot` | `[--worktree --store]` | Write a full manifest to the store. |
| `relay` | `[--group --store --worktree --peers --poll --bootstrap-minutes --concurrency --once]` | Group server: copy the other groups' origins into the own store, `--concurrency` origins and blobs at a time (default 8, 1 to 64); unless `--once`, also rebuild the bootstrap archive. |
| `compact` | `[--worktree --store --retention-days --collect-blobs]` | Snapshot, then drop this origin's old sealed segments and snapshots older than the retention. With `--collect-blobs` (group servers only) also the blobs that the folded manifest does not reference and that are older than the retention, streamed directory by directory (memory follows the live tree, not the garbage; progress per directory, cancellable). Every compaction also removes staging files in `<store>/v1/tmp` older than 6 hours. |
| `compaction` | `--plan` or `--apply` `[--worktree --store --no-blobs]` | Store-wide compaction (see Compaction): `--plan` measures against the newest snapshot and writes nothing; `--apply` takes the compaction lock, writes a snapshot, removes the journal segments and the blobs that the rules allow, prunes old snapshots and writes `compaction.json`. Works whether or not `replica.compaction.automatic` is on. |
| `verify` | `[--worktree --store --json]` | Rehash the tree and compare it with state and store; exit 1 on drift. Also lists the paths that the path policy refuses (`refused`, `refusedTotal`; not drift). |
| `bootstrap-archive` | `[--worktree --store]` | Write the bootstrap archive now (needs `machine.json`). |
| `bootstrap-settle` | `[--worktree --store]` | Prepare an extracted tree for `join` (needs no identity, refuses a joined tree). |

`--worktree` defaults to the repository root, `--store` to the store of `machine.json`, else to the `storeMount` of
the topology for this OS. `relay --peers <group>=<directory>[,<group>=<directory>...]` replaces the peer mounts of the topology
(a rehearsal on one host points them at local store directories; without the option the topology decides). `--group` and `--origin` default to `machine.json` where the command allows it.

| Topology command | Options | Prints |
|---|---|---|
| `env` | `--group [--shell sh\|powershell]` | `SEMIO_REMOTE_CACHE_ENDPOINT`, `SEMIO_REMOTE_CACHE_ACCESS=read-write`, `SEMIO_REMOTE_CACHE_INSTANCE`. |
| `mount-plan` | `--group [--platform --part]` | Mount of the group store for the OS, by the protocol of the group server (NFS, or SMB for a Windows server without Server for NFS). On Linux also the sysctl.d file with the inotify limits (parts `sysctl-path`, `sysctl`); on Windows the `cache` part that switches off the attribute and name caches of the SMB or NFS client. |
| `server-plan` | `--group [--part]` | Setup of the group server on the platform that the topology names for it (Linux, macOS or Windows). Every platform has `prepare`, `provision`, `start`, `relay-peers`, `cache-environment-target`, `cache-environment`, `cache-target`, `cache-unit`, `replica-unit`, `relay-target`, `relay-unit`; Linux adds `exports-target`, `exports`, `sysctl-target`, `sysctl`, `fstab`; macOS `exports-target`, `exports`, `nfs-conf-target`, `nfs-conf`, `peer-mount-target`, `peer-mount-unit`; Windows `feature`, `share`, `firewall`. |
| `worker-service` | `--group [--platform --part --role worker\|server]` | Service definitions; parts `target`, `definition` (replica daemon), `compact-target`, `compact-definition` and, on Linux, `timer-target`, `timer-definition`. `--role server` renders the group server's compaction with `--collect-blobs` (on the server's own platform). |
| `facts` | `--group [--platform --shell]` | Paths, protocol, mount values and server facts for the provisioning scripts. |
| `preflight` | `[--group] [--worktree --store --timeout <ms> --lang en\|de]` | Run on any machine after provisioning: checks without changing anything, see "Preflight" under Day 0. Progress per finished check on stderr, the report on stdout in English and German (`--lang` picks one), exit 1 when a check failed or Ctrl+C cancelled it. |
| `validate` | `[--file] [--reachable [--timeout <ms>]]` | Validate the topology offline. With `--reachable` it then resolves the store host and the cache host of every group and opens a TCP connection to the store port (2049, or 445 for an SMB group) and to the cache port (6090), each limited by `--timeout` (default 3000 ms, 100 to 60000). Progress lines (one per finished probe) go to stderr, the report per group to stdout, the exit code is 1 when a probe fails. Ctrl+C cancels the probes and reports `reachability cancelled`. It does not check mountd/rpcbind (111, 1048, 892) of NFSv3 servers. |

## Attestation, fleet and self-restart

Every replica daemon writes `origins/<origin>/attest.json` into the store of its group (own origin directory only,
through a temporary file, fsync and rename like `head.json`). It writes when a value changed (at most once a second)
and otherwise every 5 seconds, also while `sync` is off, and never while the store is unreachable. The relay copies the
file and `view.txt` with `head.json`, so `fleet` sees the origins of all groups from any store. Compaction never touches them, and names
in `origins/` that are not origin ids (`._name` AppleDouble files, `.DS_Store`) are ignored.

| Field | Meaning |
|---|---|
| `originId`, `groupId`, `platform`, `wallMs`, `hlc` | Who wrote it, on which OS (`linux`, `macos` or `windows`, like the topology), and when (wall clock of the writer). |
| `sync` | `false` while `.cache/f/sync.json` turns synchronization off. |
| `code` | sha256 over the sorted relative path, length and bytes of every `🟦️.ts` and `📜️script.ts` below `🤝️shared-worktree/`, `🧪️tests` excluded. |
| `cursors` | The replication cursor of this worker per origin (segment and bytes). |
| `files`, `manifest` | Number of shared files and the order-independent digest of the shared-file state: the XOR of `sha256(path \0 operation \0 value)` with the content hash of a put or the target of a link as the value. A delete contributes nothing, so a tombstone equals an unknown path; the executable bit and the index realm (`.cache/f/index-base`, `.cache/f/index/**`) are not part of it. The replica keeps it incrementally with every change of its state and restores it when it loads the state. |
| `head`, `indexTree` | The checked-out commit (`ref`, `commit`) and the tree of the shared index from `gitStateV1`, or `null` without a root `.git`. Git is asked at most once per heartbeat, in the background. |
| `view` | What a git GUI shows: `{staged, modified, deleted, untracked, digest}` from `git --no-optional-locks status --porcelain=v1 -z --untracked-files=all --no-renames` (never takes `index.lock`), or `null` without git. `digest` is the sha256 over the sorted (UTF-8 bytes, NFC paths) `XY path` lines of everything that is not purely staged (worktree column not blank, or `??`), joined by `\n`; the counts are the entries with a staged change, a worktree modification, a worktree deletion and untracked ones. The full lines are in `origins/<origin>/view.txt` (written like `attest.json`, own origin directory, only when the digest changed; its sha256 is the digest). The daemon asks git once at start and afterwards only when something relevant changed (a publish, an apply, a hint, a sweep finding, a stage or `HEAD` change), once the change has been quiet for 5 seconds or has waited 120 seconds, and at most every 120 seconds; an idle replica never asks again. Never concurrently with itself, cancellable, in the background (`git status` over 270 000 files costs 14 to 18 s on a loaded host, almost all of it the walk for untracked files). |
| `latency` | The passive latency of this daemon over its newest 100 samples: `publishMs` (from the modification time of an edited file to the moment its event was written) and `applyMs` (from the wall clock inside a remote event to its application here) as `p50` and `p95` (nearest rank, whole milliseconds, `null` without samples), `samples:{publish,apply}` and `tiers`, the last pass of each background tier (`hot`, `sweep`, `file`, `walk`, `view`) as `{passes, lastMs, dutyPercent}`. Edits and events older than ten minutes are not samples, so a checkout or a catch-up does not count. It refreshes with the 5-second heartbeat, not with every sample. `status` prints it, `fleet` shows `publish p50/p95 \| apply p50/p95`. `applyMs` compares two machines' clocks, so it is only as good as NTP. |
| `guards` | What this machine measures about the inputs of its own view, or `null` (exempt) until the first measurement: `version`, `pins`, `volume`, `infoExclude`, `caseIgnore`, `caseMismatch`, `trackedIgnored`, `trackedRefused`, `links`, `overBudget`, `modeMismatch` (see "The same view on every operating system"). |
| `queue.incoming` | Store winners the tree does not hold yet. |
| `pending`, `failures`, `fallback` | Only when present: a refused or reverted code (`pending`), the counted recurring failures, and a daemon that runs fallback code (`fallback`). |

The attestation of code that predates the guards (older still: the latency report) is read as present with `guards: null` and the warning `guards-unknown`; `fleet` compares it on code, manifest, head, stage tree and view like any other and never calls it `missing`.

`fleet` reads all attestations and heads. An origin is **fresh** when its attestation is at most 60 seconds old and
**settled** when its cursors equal every head of the store and its incoming queue is empty. Among the origins with sync
on that are fresh and settled, `manifest`, `head.commit`, `indexTree` and `view.digest` must agree (an origin whose `view` is `null` has no git and is left out of the last); the value most of them hold is the
expected one (on a tie the smaller). The verdict per row is `ok`, `off` (sync off, left out), `missing` (an origin has a
head but no attestation), `retired` (left out, see below), `stale`, `lagging` (not settled) or `diverged` with the differing fields. The command exits 0
when no origin is `missing`, `stale`, `lagging` or `diverged` and at least one is `ok`, otherwise 1; a busy fleet is
rarely settled, so use `--wait` after the edits stopped. The report shape is `Report` in `🩺️attest/🧬️schema/🔣️.json`.

**Retiring an origin.** Origin ids are never reused and a journal is history, so a machine that left the fleet for good
cannot be deleted; it is retired instead. `retire <origin> [--reason <text>]` writes `origins/<origin>/retired.json`
(`{version:1, originId, by, hlc, wallMs, reason, retired}`; `by` is the origin of the machine that ran the command,
`retired` is `true`) through a temporary file, fsync and rename like `attest.json`. This is the one deliberate exception
to "own origin directory only": the retired machine is silent and cannot write it itself. `retire <origin> --undo`
writes a newer record with `retired: false`, so a retirement and its undo both spread through the relay (the record
with the newer `hlc` wins). The command refuses the own origin, an origin that is not in the store, and an origin whose
attestation is at most 60 seconds old unless `--force`; `--undo` is never refused for that reason.

- `fleet` lists a retired origin with the verdict `retired`. It is not compared, does not have to be fresh or settled,
  and the other origins do not have to hold its head: cursors are judged against the heads of the other origins only.
  The replicas keep following its journal as before, because its events stay history. A store with only retired origins
  is not `ok`.
- `status` marks the origin as retired in the lag list and, on the retired machine itself, says who retired it.
- The relay copies `retired.json` of every origin of the peer store, also for origins homed elsewhere, and a compaction
  keeps it.
- A daemon whose own origin is retired reports one `retired` event, publishes and applies nothing, keeps attesting with
  `sync` false and keeps checking the record; a retired machine that comes back therefore cannot write silently. After
  `retire --undo` it resumes with a full scan.

**Self-restart.** The daemon records its code digest at start and compares it with the sources on disk every 5 seconds.
When the digest differs and has stayed the same for 5 seconds, the daemon persists its state, reports `restarting` and
`run` returns with exit code 0. The service manager must start the process again after exit 0: systemd `Restart=always`,
launchd `KeepAlive`, and on Windows the logon trigger plus a repeating time trigger every minute (the task ignores a new
instance while one runs). The Windows task keeps the interactive logon of the worker account because the Client for NFS
and SMB drives exist only inside that session; a task without the session (`S4U`, `SYSTEM`) would not see the store.
Only a daemon that already contains this code restarts itself: an older daemon is stopped once by hand
(`kill <pid in .cache/f/daemon.lock>`) and the service manager starts the new code.

## Local state and warm restart

The replica keeps its local state in `.cache/f/` as binary files (`🔁️replica/💾️state`, layout in `🧬️schema/🔣️.json`, independent reader `🧪️tests/🐍️.py`):

- `state.ckp` is a checkpoint of the typed tables (path book, local files, queued store winners, case-fold index, directory table, named memos): a 96-byte header with magic, format version, generation, counts and the sha256 of the payload, a directory of parts and the raw columns. It is written to a temporary file in 4 MiB chunks (the event loop runs between chunks, nothing is copied), fsynced and renamed, so a kill leaves the previous checkpoint. It is read once, straight into the final arrays; 272 000 entries load in about 0.1 s instead of 2.5 to 5 s of JSON parsing, and the file is 75 MiB instead of 106 MiB.
- `state.wal` is the delta log: a 16-byte header naming the checkpoint generation, then frames of records (each frame carries its length, the complement of the length and the first 8 bytes of its sha256). A persist appends one fsynced frame batch per round. A checkpoint is written only when the log has outgrown `max(8 MiB, an eighth of the checkpoint)` or more than 50 000 entries changed since the last persist; a clean stop appends the pending delta and never rewrites the checkpoint. A frame cut short or failing its checksum at the end of the log is a torn write and is cut off at the next start; a log of an older generation is stale and ignored.
- Anything else the daemon wants to survive a restart is a named memo of the state: the working-tree view of the last `git status` (so that the first attestation of a restarted daemon carries it with `stale: true` and does not wait for git; the fresh view replaces it, and a stale view is exempt from the `view.digest` agreement), the hot set (re-checked within a second of a restart) and the directory table (the directory sweep is cheap from the first round on).

Cursors and the clock live in the same files as memos (`cursors`, `clock`), written in the same frames after the state rows they belong to, so a cursor is never ahead of its state; `cursor.json` and `clock.json` are written only by daemons of the earlier format. Which state is used when old and new files meet (a daemon of the earlier format ran in between, as after a code revert):

- both formats exist: the checkpoint (complete) is loaded, the old-format rows are folded on top only where their hlc is newer, then the store fold is taken over (`repair` semantics: every register the store holds that the state does not, or holds older, is queued, so the follow adopts it when the disk content matches and nothing is published for it), then a new checkpoint is written;
- only an old-format state exists (possibly a partial rebuild): it is loaded and reconciled against the store fold the same way, then written as a checkpoint;
- the old files are never modified or removed by this daemon during a window of seven days, so a machine that goes old, new, old, new again finds consistent files at every step; the state used is always the one whose files are newer, but never at the price of dropping the complete checkpoint.

A checkpoint that is truncated, flipped or missing while the machine is joined (`machine.json` exists), or a log frame that fails its checksum before the end, is detected; the replica reports `state-rebuilt`, folds the store (newest snapshot plus journal), takes the cursors from the fold and queues every winner. Hard invariants of the publish path: a joined replica whose state is empty rebuilds before it publishes anything, and refuses to publish (`state-missing`) while the store cannot be folded; a walk that finds more than 1 000 paths unknown to the state takes the store fold over first and, when the fold holds more than 1 000 of them, reports `state-missing` (reason `republish-storm`) and publishes none of those. A daemon never publishes its tree as new.

**Start.** After the state is loaded (about 0.1 s) the daemon follows the store, attests and serves hints and the hot set within a second or two. The first full sweep of the tree is no longer a blocking publish: it runs in the background like every later one and its result is published by the loop. The code digest and the git state are read in parallel with the first follow, the `git status` view in the background (stale view first), and the git configuration is normalised after the first round.

## Finding changes

Publishing starts with noticing a change, and nothing in the daemon depends on operating-system file events: on a loaded macOS
host `fseventsd` drops most of them and delivers the rest seconds late, on Linux the inotify limits are a provisioning concern,
and a network share raises none. The recursive native watch (`fs.watch`) stays as an accelerator; four tiers find every change
without it, and they all hand their paths to one publish step per round, so a path that two tiers find at once is published
once (the publish compares size, modification time and hash with the recorded state).

| Tier | What it covers | Interval | Cost |
|---|---|---|---|
| Hint | A path someone named: `touch <path>…`, `touch --stdin`, `touch --hook`, or any process that drops a file into `.cache/f/hints/` | polled every 50 ms; published in the next loop round, not debounced | one `readdir` of a tiny local directory per poll (about 60 µs, 0.12 % of a core) |
| Hot set | The paths this daemon published, applied, was hinted about or saw from the watcher in the last 10 minutes (newest 20 000), the directories that contain them, the directories that recent sweeps found changed, **the root and every directory up to three levels deep (654 on the semio tree, so a create in the worktree root is found within a pass)** and the stat signature of `.git/index` | one pass about every 300 ms | one `lstat` per hot path and directory, in slices of 6 ms; the next pass starts after the larger of 300 ms and eleven times the CPU time of the last one (at most 8 % of a core) |
| Directory sweep | A create, a delete, an atomic-rename save, a directory rename or a removed tree anywhere in the shared tree: every known shared directory is `lstat`ed, only a directory whose modification time moved is listed again (not its subdirectories that did not move) | a pass starts at the earliest 2 s after the previous one ended and after nine times the CPU time it used (at most 10 % of a core) | on the 155 000 directories of the semio tree 0.4 s of CPU per pass with warm kernel caches (a pass every 4 s), up to twice that with cold caches on a loaded host (a pass every 8 s); at most 10 % of a core, in slices of 10 ms |
| File sweep | An in-place edit of a cold file that no event, hint or directory change announces: every path of the state is `lstat`ed (sizes, times, link targets compared with the record; new files are the directory sweep's job) | starts after the larger of `--scan` seconds (90 by default, jittered to 75 to 125 %) and nineteen times the CPU time of the last pass (at most 5 % of a core) | on the semio tree 4.6 to 6.5 s of CPU per pass (11 to 16 s wall at load 15), so a pass about every 2 minutes |
| Tree walk | Everything the others cannot see (refused paths, a changed ignore matcher, a lost state): the whole tree is walked and compared | at start only when no directory table was restored from the saved state, then every 30 minutes (`walkSeconds`, jittered) or twenty times the CPU time of the last walk, whichever is longer (at most 5 %); an outage, a `.gitignore` change or a repair asks for one and gets it when that gate opens | 42 s of CPU alone on the semio tree (272 000 files, 160 000 directories, 14 to 17 s wall in 16 parallel workers) and 240 s of process CPU in the first live walk under load 20, which is why it is rare |

The tiers work in slices of a few milliseconds, never run concurrently with themselves, and never make follow and apply wait
for them. Every background tier starts its next pass after the larger of its interval and the time that keeps its duty cycle: `wait = max(interval, cost × (100 / duty − 1))` with the CPU time of its own last pass as `cost` (hot 8 %, directory sweep 10 %, file sweep 5 %, tree walk 5 %, `git status` view 5 % of its wall time), so no tier can exceed its share however slow the host is. The last cost and duty cycle of each tier are attested (`latency.tiers`) and printed by `status`. The table of directory modification times lives in memory and is rebuilt by the first walk after start.

**Hints.** A hint is a file in `.cache/f/hints/` (a sealed, local directory, created with mode 1777 so that every local account
can drop one): paths separated by NUL or newline, each absolute or relative to the worktree. A writer creates it under a name
that starts with a dot and renames it inside the directory, which is atomic; the daemon consumes the finished files, never
reads one above 1 MiB, removes unfinished ones after a minute, and takes at most 5000 paths per poll. A path outside the
worktree, below `.cache/f`, or empty is dropped; an absolute path is also matched through the real path of the worktree
(symbolic links, case on macOS). A hinted directory is listed recursively. Hints that arrive while sync is off or the store is
unreachable wait in memory; more than 20 000 of them turn into a full sweep.

**The agent hook.** `.claude/settings.json` registers a `PostToolUse` hook for `Write|Edit|MultiEdit|NotebookEdit` that runs a
four-process shell one-liner in the background (`async`, bash, timeout 10 s; `hookCommandV1` in `🎣️tiers`): it copies the hook
payload from standard input into `.cache/f/hints/` under a dot name and renames it to `h<pid>-<time>.hook`, and does nothing
without that directory. There is no launcher and no JSON parser in the hook, because a `repo` start took 7.9 s under load. The
daemon parses `*.hook` files (up to 32 MiB, a `Write` carries its content): `tool_input.file_path` and `tool_input.notebook_path`,
everything else ignored, malformed payloads dropped. The hook always exits 0, so it can never block or fail an agent. On
Windows Git Bash runs the same line; without Git Bash set `shell: powershell` with `hookPowerShellCommandV1` in a per-machine
`.claude/settings.local.json`. `repo ./📜️script.ts touch --hook` stays available for clients that cannot run the one-liner. An agent edit is therefore
published within about a second on every machine whose daemon runs. No other client configuration of the repository has a post-edit hook (`.codex/config.toml` configures MCP servers only), so none is wired; a script or editor that wants
the same speed calls `repo ./📜️script.ts touch <path>` (or drops a hint file) after saving.

**Passive latency.** The daemon samples `publishMs` for every event that a hint, the hot set, the directory sweep or the watcher
found (now minus the modification time of the file; events found by the start-up walk, the tree walk or the file sweep are not
samples, their wait is the interval itself) and `applyMs` for every event published after this daemon started (now minus the wall
clock inside the event; the backlog that a restart applies is not a sample), and attests the percentiles of the newest 100
samples with the timings of the tiers, see the attestation fields below. `applyMs` also contains the other machine's relay,
store and poll delay (the follow round is 250 ms) and every wait for an object or an `index.lock`.

**Cost of the attestation.** The git state (`HEAD`, stage tree) is read from a stamp of `.git/HEAD`, the ref it names,
`packed-refs` and `.git/index`, and git is only started when the stamp moved (and at the latest every 5 minutes). The
working-tree view runs `git status` only after a relevant change and at most 5 % of the time (a view that took 20 s is followed by a pause of at least 380 s), see the field `view`. Per round the daemon reads the sync
switch, the store root, the retirement record and the repair request at most about once a second, and the list of origins about
every two seconds; each round still reads the head of every other origin. `core.untrackedCache=true` and `index.version=4` are
set for plain `git status` on this machine; the view uses `--untracked-files=all`, for which git builds no untracked cache.

## The same view on every operating system

The fleet shares files, not file systems. A path set must be valid on the most restrictive member (APFS and NTFS are
case-insensitive, APFS also normalisation-insensitive, NTFS has reserved names and `MAX_PATH`), and what a machine attests as `view` must
not depend on that machine. The rules, each with its test:

- **Pinned git** (`🌿️git`, `GIT_PINS_V1`, `GIT_VIEW_PINS_V1`, `gitPinsHashV1`). Every git process of the replica runs with `GIT_CONFIG_NOSYSTEM=1`,
  `GIT_ATTR_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL` and `XDG_CONFIG_HOME` pointing at an empty file and an empty directory below `.git/semio-pin/`
  (the account's `~/.gitconfig`, `$XDG_CONFIG_HOME/git/ignore` and the system configuration never count; `GIT_CONFIG_PARAMETERS`,
  `GIT_CONFIG_COUNT` and `GIT_CONFIG_KEY_*` are removed) and with `-c` pins: no excludes and no attributes file, `core.quotepath=false`,
  `core.precomposeunicode=true`, `core.autocrlf=false`, `core.safecrlf=false`, `core.eol=lf`, `core.longpaths=true`, `core.protectNTFS=true`,
  `core.protectHFS=true`, `core.fsmonitor=false`, `gc.auto=0` (the same list on every operating system, so the pins hash is one value).
  This needs git 2.32 or newer (`GIT_CONFIG_GLOBAL`); `preflight` fails below it. The attested view adds `core.ignorecase=false` (the
  `.gitignore` match is case-sensitive everywhere, like the replica's own matcher and like Linux), `core.filemode=false` (the executable
  bit is not part of the view; `guards.modeMismatch` counts it instead) and `core.symlinks=true`. Each spawn has a timeout (10 s for `--version`, 10 min otherwise).
  `.git/config` is still normalised for the human tools of the machine (`core.safecrlf`, `core.eol`, `core.protectNTFS`, `core.protectHFS`,
  and on Windows `core.longpaths=true`).
- **Case and normalisation in the view.** git on APFS or NTFS cannot see that the index spells a name differently than the disk
  (`Foo.txt` against `foo.txt`, a renamed directory, NFC against NFD): `lstat` finds the file, so no flag makes it print what Linux
  prints (` D Foo.txt` and `?? foo.txt`). On such a volume (probed once: a probe file looked up under another case and another
  normalisation) the view lists every tracked directory once and adds ` D <index path>` for each tracked path whose exact spelling is absent
  and `?? <path>` for an NFD-only spelling, so the lines equal Linux's for the same disk state (tested on APFS and ext4 with one expectation).
  A case-only rename of a tracked path stays a staging action: `git add` records the new spelling.
- **`.git/info/exclude`** is per machine, never replicated, and no flag removes it from git's view. `guards.infoExclude` counts its rules
  and the fleet verdict fails on a difference; `preflight` fails when the count is not 0. Use `.gitignore`, which is shared.
- **Names** (`🚧️boundary`, `foldNameV1`). The case fold is NFC plus a table of every code point (`🔤️fold/🔣️.json`, generated by `🔤️fold/📜️script.ts`
  from the case mapping of ICU: the fixpoint of upper case then lower case per character), the union of APFS (full case folding,
  normalisation-insensitive), NTFS (upper-case table) and Linux with a casefold directory: `ß`, `ẞ` and `SS` meet at `ss`, the final and the medial sigma
  meet, `ı` and `ſ` meet `i` and `s`, ligatures expand, `İ` folds to `i` plus a combining dot. Two paths that fold to one name are
  `case-collision` on every machine, so a path set that is valid here is valid on all member systems. The emoji variation selector `U+FE0F` is a code point
  of its own on APFS and NTFS (verified on APFS: `a.txt` and `a\uFE0F.txt` are two files) and stays significant. The replica, the scanner and
  `ReplicaFoldsV1` use this one function. Verified: the pairs of `🧪️tests/🧫️fixtures` are stored as one file by APFS (this Mac) exactly where the fold says "one name", ext4 stores none of them as one, and
  Python's `str.casefold` over every assigned code point agrees with the table (`🐍️.py`). `ı`/`i` is merged although APFS keeps them apart
  (NTFS' table is the source; unconfirmed on Windows).
- **Path length.** The policy counts UTF-16 units of the worktree-relative path (`PATH_LIMIT_UNITS_V1` = 256, the unit of Windows paths), not code points.
  The real limit differs per operating system and includes the worktree prefix: `pathBudgetV1` gives 259 UTF-16 units on Windows
  without `LongPathsEnabled` (`MAX_PATH` less the terminating NUL; with the prefix `C:\semio\` a relative path of more than 250 units does not fit), 32,766 with it,
  1,023 UTF-8 bytes on macOS and 4,095 on Linux. `preflight` (`path length`) and the attestation count the tracked paths over that budget
  (`guards.overBudget`; a failure on Windows without long paths, otherwise a warning). The provisioning script sets and reads back `LongPathsEnabled`.
- **Executable bit: one source of truth per kind of volume.** On POSIX the disk bit is the truth: a `chmod` publishes an event. A replica that
  cannot represent the bit (`modeless`, Windows; an option so that tests can simulate it) never reads it from disk, keeps the bit of the
  manifest entry when a file changes, takes the bit from the staged index entry (`100755`) for a file it creates, defaults to `false` for an unstaged new file (a POSIX machine
  `chmod`s it and publishes the bit), and never writes it. Such a replica publishes its stage before its files, so a new file that is staged
  with `--chmod=+x` is published executable. The view ignores the bit; `guards.modeMismatch` counts tracked files whose bit differs from the index mode
  (a warning in `preflight`, compared among the machines that can read it).
- **Guards** (`gitGuardsV1`, attestation field `guards`, `preflight` checks `guards.*`): git version and pins hash; the volume (case, normalisation, links, long paths);
  `.git/info/exclude` rules; rules of the root `.gitignore` whose verdict depends on case for paths that exist (the `Backup/` class: a
  case-insensitive git hides `backup/x`, the replica shares it); tracked paths spelled differently on this disk; tracked paths a rule ignores;
  tracked paths or links the path policy refuses (illegal or reserved name, too long, unsafe link target, case collision); tracked links on a volume that cannot create them;
  paths over the budget; executable-bit mismatches. They are measured in the background (every 5 minutes, and 10 s after the view changed at the earliest);
  the attestation says `guards: null` (exempt) until the first measurement arrives. The fleet verdict is `diverged` on a difference in
  `guards.pins`, `guards.infoExclude`, `guards.caseIgnore`, `guards.caseMismatch`, `guards.trackedIgnored`, `guards.trackedRefused` and `guards.modeMismatch`
  (the last among the machines that can read it), and only lists `warnings` (`fleet` prints them, `--json` carries them) for capability differences: another git version, another volume behaviour, paths over the budget, links without link support.
- **Windows services.** The scheduled task XML of the fleet is validated in the tests against the published Task Scheduler schema
  (`xmllint` with `🧪️tests/🧫️fixtures/🪟️task-schema.xsd`, and an own copy of the rules): trigger children are a sequence (`Enabled`, `StartBoundary`, `EndBoundary`,
  `Repetition`, `ExecutionTimeLimit`, then the trigger's own), the other groups are unordered. The replica task starts at logon and again every minute (restart after a clean exit),
  runs when the machine was off at the start time (`StartWhenAvailable`) and so also after sleep. The store is mapped by its own task `semio-fleet-store-mount`
  (`worker-service --part mount-definition`: `cmd.exe /d /c if not exist S:\ mount.exe ...`) at logon and every minute while the drive is missing, in the same logon session
  that runs the replica; a drive mapped by the elevated provisioning shell is not visible there. The replica retries an unreachable store, so the order of the two tasks does not matter.
  `preflight` reads the task state through `Get-ScheduledTask` (an enum, not localised text), time synchronisation through `sc.exe query` (the state number) and the `W32Time` registry type,
  and the store mount through `mount.exe` (Client for NFS) and `net use` (SMB). The user `PATH` is extended as `REG_EXPAND_SZ`, the task principal is `WindowsIdentity.Name`, and the script refuses a 32-bit PowerShell.

## Two writers of one file

Nothing is merged and nothing blocks. When a remote edit reaches a machine that has an **unpublished local edit** of the
same path (offline, or inside the poll interval), the daemon compares the two by wall time: the local file's modification
time against the wall-clock time inside the remote event, which is the moment the other machine **published** it.

- Local edit later: it is kept, reported as `kept`, and published at once with a new clock value, so it wins everywhere.
- Local edit earlier or equal: the remote version is applied and the local bytes are copied to
  `.cache/f/lost/<hlc>/<path>` (reported as `overwritten`). Copy back what you want to keep.
- A local delete has no modification time and loses to any remote edit: the file comes back, nothing is published.
  A remote delete is compared with a dirty local file like any other event.
- Both machines edited offline: the first one to publish wins, the other's edit goes to `lost/` unless its modification
  time is later than that publish time.

The honest limit: the clock inside an event is the time of publication, not the time the key was pressed. Writes to the
same file on two machines within the publish latency (a second or less with a hint or a hot path, a few seconds otherwise; `status` shows the measured value) are ordered by who published first, and an
edit made offline is ordered by when it was published. Every machine converges to the same version either way. The
comparison reads one machine's file clock against another's event clock, so keep NTP running.

## Structural conflicts

The register model is per path, so two live entries can be valid one by one and impossible together: `b.txt` and `B.txt` on a
case-insensitive volume, or a file `dir` next to `dir/c.txt`. One rule, a pure function of the folded manifest, decides every such
pair identically on every machine, so no machine waits for another and the queue drains.

- Two live entries (`put` or `link`) **clash** when their folded paths (`foldNameV1` of the boundary, which is also what the case-collision refusal uses) are equal, or when the folded path of one is a directory prefix of the other (a file or link with entries below it). Sibling directories that differ only in case do not clash.
- The live entries are taken **newest hlc first**; an entry is kept unless it clashes with an entry that was kept already, otherwise it is **retired**. A file newer than every live descendant retires the descendants; a file older than some descendant is retired itself (the older descendants then survive, they only clash with kept entries). Equal times are ordered by the origin in the hlc.
- Nothing is published for this. Retirement is a local reading of the manifest: every machine that folds the same manifest retires the same paths, the manifest keeps both entries, snapshots and compaction need no change. Publishing an explicit delete from one machine would race with a legitimate re-creation of the retired path and would need an election; the derived rule needs neither. An entry that becomes valid again (its winner is deleted or replaced by something that no longer clashes) is applied again, also after a restart: a retired entry stays in the persisted queue as a parked row (not pending, not counted in `queue`), its register is a delete carrying the hlc of the retired entry.
- A retired entry that was materialised here leaves the tree first (before the winner is written, so a case-insensitive volume never sees both) and its bytes go to `.cache/f/lost/<hlc of the winner>/<path>` (`overwritten`); `superseded` names the loser, the winner and the reason (`case-collision`, `directory-replaces-file`, `file-replaces-directory`). The blob of a retired entry stays in the store as long as the manifest references it.
- Apply never writes through a symbolic link: every ancestor directory of a path is inspected with `lstat` before anything is written. A link the fleet knows is an ordinary non-directory entry of the rule above. A link that stays local (an unsafe target, an ignored path) blocks the entries below it with a visible `blocked` event (`symlink-in-the-way`; `parent-is-a-file` for a file); nothing is written, and a symlink cycle produces no error event. The publisher applies the same inspection: a path reached through a symlinked directory is neither published nor read as a file of the tree.
- The apply pool removes before it creates (deletes and their `prune` of emptied directories finish before the first `put` or `link` of the batch), and the link branch retries `mkdir`/`symlink` on `ENOENT`/`EEXIST` as the file branch does, so a delete that empties a directory and a link that refills it never meet.
- A `put` that republishes content the path already had earlier is a **stale republish** when its `base` is not the content it replaces (a replica that lost its state republishes its old tree with fresh clocks and `base` null). The decision is a pure function of the events of the path (`staleRepublishV1` in `🔁️replica/⚖️clash`, fixture vectors replayed by TypeScript and by the Python oracle): the event is stale when it leaves content, differs from its predecessor (the event with the greatest smaller hlc), was not written on top of that predecessor (`base` differs) and leaves content that an earlier event left already. A deliberate revert by a machine that held the newest content has `base` equal to the predecessor and is an edit. A machine whose register still equals the predecessor's content does not take a stale event, reports `stale-republish` and publishes the newer content again as an ordinary change, so the fleet converges to the newer content even though the stale event has the greater hlc; the manifest itself is never changed by the rule. The events come from the batch being read, from the manifest entry the path held, and from a cache of the last 6 events of the last 20 000 changed paths that is kept with the local state (memo `stale.history`, the newest 1000 paths, written at most every 30 s and at close), so a restarted machine decides the same. A machine that joins later with only a snapshot has no older content to compare and applies the stale content, then takes the re-publication of the others.

## Network layout

- Five physical networks, one per group, form one logical network. Each group has one **group server** that runs
  four services: the share that holds the store, the cache server (`repo remote-cache serve`, port 6090 in
  the topology), the relay and a replica daemon of its own. The group server is a fleet machine: its origin is
  `<group>-server` (for example `g1-server`) and its worktree is the canonical path of its own operating system.
  The server may be a Linux, a macOS or a Windows machine; the topology names which (`server.platform`).
- A **worker** talks only to its own group's share (2049/tcp for NFS, 445/tcp for SMB; NFSv3 also needs rpcbind 111
  and mountd) and its own group's cache server. It needs no route to the other groups.
- The group servers must reach each other. Each relay mounts the other four exports read-only (the `peers` entries
  of the topology, for example `/mnt/semio-peers/g2`) and copies what is missing into its own store. An origin is
  only ever pulled from the store of its home group, so there are no loops, and below `origins/<originId>/`
  there is one writer per store: the home machine in its home store, that group's relay in every other store.
- Consequence: every group store holds the whole fleet's data, not only its own group's. The size of one store
  is the size of the fleet (see Capacity).

### Server platforms and the protocol each worker mounts

| Group server (`server.platform`, `server.protocol`) | Linux workers | macOS workers | Windows workers |
|---|---|---|---|
| Linux, `nfs`: systemd, `/etc/exports.d/semio-fleet.exports`, `nfs-server` | NFS 4.2 (`nfs4`) | NFS 4 (`mount_nfs vers=4`) | NFS 3 (Client for NFS, `mount.exe`) |
| macOS, `nfs`: launchd, `/etc/exports`, `nfsd` (serves NFS 3) | NFS 3 (`nolock`) | NFS 3 (`nolocks`) | NFS 3 |
| Windows Server, `nfs`: Server for NFS, `New-NfsShare` (NFS 3) | NFS 3 | NFS 3 | NFS 3 |
| Windows 10/11 or a server without Server for NFS, `smb`: `New-SmbShare` | not supported | not supported | SMB (`net use S:`) |

The group server picks no protocol silently: `server.protocol` in the topology decides, the scripts only check it
(the PowerShell script refuses `nfs` when `Install-WindowsFeature` does not exist, and says to set `smb`). A Linux or macOS
worker of an `smb` group is refused with "Windows workers required" by `facts` and `mount-plan`; a group with
`smb` can therefore only have Windows workers, and no macOS group server may exist anywhere in a fleet that has
an `smb` group (a macOS relay cannot read an SMB store; `readTopologyV1` refuses the topology). The relays of the other
group servers read each store read-only: a Linux server mounts NFS peers by `nfs4` (Linux peer) or `nfs` v3 (macOS or
Windows peer) and SMB peers by `cifs` with `/etc/semio/smb-peer.credentials`; a macOS server mounts NFS peers at
`/Volumes/semio-peers/<id>` with a LaunchDaemon; a Windows server reads the peers by UNC path (`\\host\share`) without a mount; its `feature` part therefore also installs Client for NFS (`NFS-Client` on Windows Server, the optional features on Windows 10/11) when any peer group is served over NFS.

How the three servers differ:

- **Linux.** `/etc/exports.d/semio-fleet.exports` (read-write for `server.network`, read-only for the peer servers),
  an fstab block (bind mount of the export at the store mount, peer mounts), systemd units for cache and relay.
- **macOS.** The export must lie below `/Users` or `/Volumes` (the root volume is read-only; the topology refuses other
  paths). `/etc/exports` gets two lines (workers read-write by `-network/-mask`, peers `-ro`), `/etc/nfs.conf` pins the mount
  daemon to port 892. The server has no mount of its own store: its replica, relay and compaction use the exported folder itself
  (`--store`, recorded in `machine.json`). Cache and relay are LaunchAgents of the account (see "macOS privacy"); the cache reads
  `/etc/semio/remote-cache.env` (mode 0600, owner = the server account) through `sh`.
- **Windows.** `nfs.export` is the single-segment share name (`/semio-store`) and `server.directory` the local folder
  (`C:\semio-store`). Server for NFS (`FS-NFS-Service`, Windows Server only) shares it with `New-NfsShare` (anonymous
  access mapped to uid 1500, read-only entries for the peer servers); the `smb` fallback creates the local accounts
  `semio-fleet` (workers, Change) and `semio-fleet-peer` (relays, Read) with `New-SmbShare`. Windows workers of an SMB group
  store the account once with `cmdkey /add:<host> /user:<server name>\semio-fleet /pass:<password>` (the `credential` part of
  `mount-plan`). The firewall allows the store ports from `server.network` and from the peer servers only. Cache and relay
  are Scheduled Tasks (`semio-fleet-cache`, `semio-fleet-relay`) next to the replica task; the cache task reads
  `C:\ProgramData\semio\remote-cache.env`, which only the server account can read. All three tasks run at logon of the
  server account (give the machine an automatic logon, or register them with a password).

### The one file the operator edits

`🗺️topology/🔣️.json` is the only file the operator edits to make the fleet real. The hosts in it are placeholders
(`nfs.gN.semio.internal`, `cache.gN.semio.internal`, `10.0.N.0/24`). Per group set `nfs.export`,
`server.platform`, `server.protocol` and, on Windows, `server.directory`; per fleet the canonical `worktree` and `storeMount` per OS and the `replica`
timings. The machine's own host, worker network and cache endpoint are not written into this file. Set `SEMIO_FLEET_GROUP`,
`SEMIO_FLEET_NFS_HOST` and `SEMIO_FLEET_NETWORK` together (and `SEMIO_FLEET_CACHE_ENDPOINT` when the cache URL is not
`http://<that host>:<the port already in the file>`). The commands and the provisioning script apply them for that run and leave the file as it is. Everything else is derived from it and never edited by hand: the mount lines, exports, fstab blocks, units,
LaunchDaemons, Scheduled Task XML, firewall rules, the cache environment of every machine, the scripts' checks (they
refuse a topology file that differs from the one in the worktree) and the Python oracle in `🧪️tests`. After the
edit run `validate` (offline, checks shape and the server rules), then `validate --reachable` from a machine of each
group network: it reports per group whether the hosts resolve and the store and cache ports answer. Re-provision the
machines whose group changed; the scripts are safe to repeat.

## Provisioning and the bootstrap archive

A brand-new machine has an empty disk, so it has no worktree, no `repo` launcher and no Bun, yet `join` lives in the
worktree. The group server closes the gap with an archive of its own worktree.

**The archive.** `<storeRoot>/v1/bootstrap/worktree.tar` is a POSIX pax tar written by our own code (only `node:`
built-ins) with one regular-file entry per shared file of the folded store manifest, read from the group server's local
worktree. Pax extended headers carry the long UTF-8 emoji paths and, from 8 GiB, the file size. Links are not in the
tar: `join` creates them. A file whose hash differs from the manifest (not yet replicated to the server, being
written, vanished) is skipped. `manifest.json` next to it is `{"version":1,"hlc":…,"files":n,"bytes":n,"sha256":…}`:
`files` and `bytes` count the entries and their content, `sha256` is the hash of the whole tar. The tar is staged in
`<storeRoot>/v1/tmp` and renamed, then the manifest the same way, so a reader that sees a mismatch retries. The writer
streams in 1 MiB chunks (bounded memory), reports progress as NDJSON and stops cleanly on cancellation. All names in
the store stay ASCII.

**Refresh.** The relay on the group server rebuilds the archive when it starts and then every
`replica.bootstrapMinutes` of the topology (default 60; `relay --bootstrap-minutes <n>` overrides it). The archive
needs the server's own replica daemon (`run`) to keep the server's worktree current; the server plan installs both.
`bootstrap-archive` builds it once by hand. A failed build is reported as an `error` event and retried after at
most two minutes (so a store that is still empty gets its archive soon after the first files arrive); staging files of a
crashed build older than six hours are removed by the next build. A relay started with `--group` but without `machine.json`
does not build it.

**Provision one machine.** `🗺️topology/🥾️provision/🐚️.sh <group> <origin> [--user <account>] [--archive <directory>]`
(macOS, Linux, as root or through sudo) and `🔵️.ps1 <GroupId> <OriginId> [-ArchivePath <directory>] [-User <account>]`
(Windows, elevated) take the machine from an empty disk to a running replica. Copy the `🗺️topology` directory of
the repository to the machine first: the scripts read the worktree path and the NFS export from the topology file next to
them, before any tool exists, and refuse to continue when the topology command later derives other values from the
worktree. Steps, in this order:

1. Create the canonical worktree directory. A directory that already holds a checkout skips
   the archive; when it holds the account's own repository (not joined yet) it is **adopted** (see "Existing repository"
   below), never refused; one with files but without the fleet module is refused with the instruction to update that
   repository first. **Windows: the script first sets
   `HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\LongPathsEnabled = 1` and reads it back** (the tree has deep emoji paths;
   the path policy allows 256 UTF-16 units below the root, and `MAX_PATH` is 260 units including the drive and the terminating NUL), and keeps the worktree root short:
   the schema accepts only a drive and one directory of at most 12 characters (`C:\semio`), the script refuses a longer one.
   The setting reaches processes that start afterwards; a shell that was open before needs a restart. Git for Windows
   is needed only for the shared stage and commit state (the replica starts `git` itself), and any other tool that opens these paths must be long-path aware.
2. Windows mounts the store first (Client for NFS); macOS and Linux mount the export read-only at a temporary
   directory. Copy `worktree.tar` and `manifest.json` to local disk (the `--archive` directory instead, if given),
   verify the sha256 of the tar against `manifest.json` (five attempts, 20 s apart, because the archive may be rebuilt
   while it is copied) and extract it into the empty worktree with the OS tar: `tar -xf` as the worker account on
   macOS and Linux (with a UTF-8 locale), `tar.exe` (bsdtar, reads pax UTF-8 names) on Windows 10 1803 or later. macOS and
   Linux then compare the number of extracted files with `manifest.json`, Windows the number of entries in the tar.
3. Build the standalone `repo` launcher for the account (`🚀️launcher/📥️install/🐚️.sh` or `🔵️.ps1`, from the checkout, into the account's
   home; it fetches its pinned Go itself and needs no compiler and no package manager, only `curl` or Python 3 and `tar`) and
   link it as `~/.local/bin/repo` (Windows: the launcher directory goes on the user `PATH`). On macOS and Linux the script then
   links `/usr/local/bin/repo` to it, which the service definitions use. **Joining the fleet never depends on the native build**:
   a fix for a broken build arrives through the shared worktree itself, so everything up to the service start runs through this
   launcher and the machine is syncing before the native bootstrap is even tried.
4. Mount the store for good (fstab line on Linux, LaunchDaemon on macOS, logon task on Windows), write the cache
   environment, install the replica daemon and the daily `compact` (see Day 2).
5. `bootstrap-settle`, then `join --group <group> --origin <origin>`. The archive can be up to `bootstrapMinutes`
   old, so some files differ from the store by then. `join` refuses a tree with such files, so `bootstrap-settle` first
   renames names that the OS tar extracted as NFD (macOS `tar` does this) back to NFC, then moves every shared file that differs from the store, or that the store deleted, to
   `.cache/f/lost/bootstrap-<ms>/<path>` (on macOS and Linux a file whose executable bit differs from the store counts as
   different; it never deletes, it refuses a store without files and a tree that already has `machine.json`); where the store holds a version of a moved file, settle puts that version back at once (otherwise the subsystem's own
   scripts, if they changed since the archive, would be gone before `join` starts); `join` then
   adopts the matching files and materialises the rest. The extracted files are recorded in the replica state as seen
   now, so the first scan of the daemon trusts those whose modification time is more than 2 s older and rehashes the others
   once (the racy-timestamp rule); the daemon follows the store before its first publish, so nothing of the archive is
   published as a local edit.
6. Start the replica daemon and the compact timer (on a group server also the cache and the relay).
7. **Last: the native bootstrap** (`🔩️native/🥾️bootstrap/🐚️.sh setup` or `🔵️.ps1 setup`, as the worker account): it installs the C/C++
   toolchain, Node.js, Rust, .NET, uv, builds and installs the native `repo` and runs `workspace:setup`. It needs network access and,
   on Linux, package-manager rights for the account (sudo) or the packages installed as root beforehand
   (`apt-get install -y build-essential pkg-config curl git unzip`; the script names the exact line of the machine's package manager
   (`apt-get`, `dnf`, `zypper`, `pacman`), `xcode-select --install` on macOS, and `winget` plus the
   `winget install ... Microsoft.VisualStudio.2022.BuildTools` command on Windows). Its output goes to
   `.cache/f/native-bootstrap.log` and the terminal. A completion marker `.cache/f/native-bootstrap.done` in the ignored local state
   is written only after it succeeded and the native `repo` exists; a run without the marker (or without the launcher) repeats the
   bootstrap, so one that failed half way is redone by the next run. The Windows script requires `-User` to be the account of the
   elevated shell, because the bootstrap installs per account.
   On Linux the native bootstrap runs through `sudo` without a login shell, so the profile files of step 5 do not reach it: the script
   hands it the `SEMIO_REMOTE_CACHE_*` variables of `env` and the write token of the account (`SEMIO_REMOTE_CACHE_TOKEN` from the
   environment of the script or the `export` line of `~/.profile` or `~/.zprofile`) through a `0600` file in `.cache/f` that it removes
   afterwards, never as an argument. That is how a fresh machine takes the project graph of its group from the cache server instead of
   inferring all projects cold (a cold graph needs a 7 to 8 GB heap and hours); the server's own native bootstrap is the warm-up that
   uploads it. Linux machines with less than 16 GiB of RAM plus swap also get a swap file (`/var/lib/semio-swap`, step 4) so that a
   cold build that does happen is slow instead of killed for lack of memory.

   **When the native bootstrap fails** (or its build tools are missing and cannot be installed), nothing above is undone and the
   script does not fail the fleet setup: it prints `NATIVE BUILD PENDING` with the failing command, the reason, the log and its
   last lines, leaves the marker absent and exits with code 3. `preflight` shows the same state as the warning `native build`.
   The machine is a member of its group and syncs, so the fix usually arrives by itself through the shared worktree: run the
   script again (it repeats only what is missing, then the native bootstrap) after the cause is fixed or the fix has arrived.

   | Exit code | Meaning |
   |---|---|
   | 0 | fleet ready and native build complete |
   | 1 | error: the failing step is named, nothing after it ran (Windows: the script threw) |
   | 2 | usage error (shell script) |
   | 3 | **fleet ready, native build pending**: joined, services running, native bootstrap failed or not possible, marker absent |

**Linux inotify limits.** The replica daemon watches the tree recursively, one inotify watch per directory, and the tree has
about 122,000 directories (121,884 measured in the rehearsal); the kernel default of 8,192 watches would drop events
silently. The script writes `/etc/sysctl.d/90-semio-fleet.conf` (`fs.inotify.max_user_watches=1048576`,
`fs.inotify.max_user_instances=1024`, the `sysctl` part of `mount-plan --platform linux`), applies it with `sysctl -p`
and reads every key back; it stops with the key and the value when a limit stayed too low (inside a container the host
must raise them). Watches cost kernel memory (about 1 KiB each, so 122k directories need roughly 130 MiB per user
that runs a daemon); the high limit leaves room for editors and build tools next to the daemon. Measured on Linux: a daemon on the
full tree holds 154,895 watches (the tree has 154,899 directories including empty ones and the ones that `repo` creates) of the
1,048,576 allowed; each of the 3 daemons on one account counts separately, so the limit is per user, not per daemon.

**Re-running the script.** It can be repeated at any time (new topology, new service definitions, a repaired mount).
Before it changes anything it reads `.cache/f/machine.json` of the worktree, if present: when the origin or the group in it
differs from the arguments, it stops with a message that names both and says to pass the right ones or to remove the worktree state; when
both are equal, `join` (or `seed`) is skipped, but the mounts, limits, environment, services and tasks are installed again
and restarted when a definition changed. A machine that was provisioned with another group must be reset by hand, never
silently rejoined.

Every step probes first, so a failed run can simply be repeated; a worktree from an interrupted extraction must be
emptied by hand (the script says so).

**Language.** The launcher refuses a start that chooses no language, and a fresh Ubuntu has `LANG=C.UTF-8`, which names none. The
script therefore runs every `repo` call with `SEMIO_LANGUAGE` (default `en`, override it in the environment of the script), and the
generated Linux cache unit carries `Environment=SEMIO_LANGUAGE=en`. The replica units do not need it: the replica prints
both languages.

**Changed mount plan.** The Linux worker replaces its own `/etc/fstab` line (same source and mount point, other options) and
stops with the foreign line when another source is mounted there. A mount that is already up keeps its old options: the
script warns; stop the replica services, `umount` and `mount` the store to apply the new ones (`remount` cannot change
`lookupcache`). The macOS script warns the same way when its mount LaunchDaemon changed while the store is mounted; the Windows
script restarts the Client for NFS service (`nfsadmin client stop`, `nfsadmin client start`) after it changed a cache value, before it mounts.

**Client caches (what is measured and what is reasoning).** The replica `stat`s `v1/heads` every 250 ms (and the `head.json` of origins without a flat head, and of all origins every 10 s); heads are replaced by a
rename or by a new name, so a client that answers a lookup from a cached name or from cached directory attributes shows the old head. Every store
client is therefore set to revalidate; the settings differ because the clients do:

| Client | Setting (mount plan and provisioning script) | Status |
|---|---|---|
| Linux NFS 4.2 and 3 (also the peer mounts of a Linux group server) | `actimeo=1,lookupcache=none` | **Measured** (`linux-nfs-report.md`): without `lookupcache=none` followers see 46 of 100 new heads, with `lookupcache=positive` after 505 ms median, with `none` after 10 ms; end to end 305 ms against 696 ms. |
| macOS NFS 4 and 3 (workers and the peer mounts of a macOS group server) | `acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache` | **Measured on macOS 27 (NFS 4.0 client of a Linux server)** (`mixed-os-report.md`; the NFS 3 variant for a macOS server was not run). `stat` of a replaced head, 100 replacements: with these options all 100 seen after 2 ms median (4 ms maximum); with the `mount_nfs` defaults only 28 of 100 seen at all, 672 ms median, 1.3 s late; with `actimeo=1` 446 ms median and 917 ms at p95; with `noac` as with the plan. `mount_nfs` has no `lookupcache`: `acdirmin=acdirmax=0` is what revalidates the name. Cost of the poll (four heads four times a second, `nfsstat -c`): 65 NFS calls per second with the plan, 79 with `noac`, 11.5 with `actimeo=1`, 0 with the defaults (which is why they are stale). `nonegnamecache` made no difference to the head and stays for names that appear later. The macOS client speaks NFS 4.0 for `vers=4` and NFS 4.1 for `vers=4.1` (same results, not needed), reads and writes in 32 KiB (`rsize` is not raised: nothing measured needs it) and has no `nconnect`. |
| Windows SMB (workers of an `smb` group, and a Windows server that reads an `smb` peer) | `Set-SmbClientConfiguration -DirectoryCacheLifetime 0 -FileInfoCacheLifetime 0 -FileNotFoundCacheLifetime 0` (the `cache` part of `mount-plan`) | **By reasoning.** The cmdlet and its parameters are the documented ones; the defaults (10 s, 10 s and 5 s) would show a replaced head up to 10 s late. Not run on a Windows machine. |
| Windows Client for NFS (workers of an NFS group, and a Windows server that reads an NFS peer) | two DWORD values `RemoteWriteCache` and `FileAttributeCache` = 0 under `HKLM\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Users\Default\Cache`, then `nfsadmin client stop` and `nfsadmin client start` (the `cache` part of `mount-plan`) | **Documented by Microsoft for older Windows, not yet confirmed on Windows 11/Server 2022.** Source: [Steps to disable Client for NFS cache](https://learn.microsoft.com/en-us/archive/blogs/technet/sfu/steps-to-disable-client-for-nfs-cache) (key, value names, DWORD 0, restart of the service; the same two values are in [All (well, almost) about Client for NFS](https://learn.microsoft.com/en-us/archive/blogs/msdn/sfu/all-well-almost-about-client-for-nfs-configuration-and-performance)). `mount.exe` has no cache option. The script creates the key if missing, sets the values, reads them back and restarts the service before it mounts. The folder content cache (30 s) is a different setting (Microsoft KB 894071), not set here. Whether the client honours the values on a current Windows is the first thing to check on day 0 (below). |
| Linux `cifs` (a Linux group server reading an SMB peer) | `actimeo=1` | Unchanged and not measured: a head of an SMB group can reach a Linux relay up to one second late, which only delays the cross-group copy. |

**NFSv4 delegations (measured).** A macOS client that replaces a file which another client holds a read delegation on waits about 4 s
(the Linux server answers `NFS4ERR_DELAY` while it recalls the delegation, and the macOS client retries late): every second save from
a macOS machine reached the group 4 to 8 s late (median 4,270 ms, p95 8,252 ms, 49 of 200 saves overtaken), because the relay of another group, or
any other worker, polls the `head.json` of every origin and so holds a delegation on it. Every Linux group server therefore sets
`fs.leases-enable=0` (`sysctl` part of `server-plan`, file `/etc/sysctl.d/91-semio-fleet-server.conf`, applied and read back by the script before
the NFS server starts): no delegations are granted, and the same saves arrive after 347 ms median (p95 477 ms, none overtaken). The macOS mount option `nocallback` does not help,
because the delegations belong to the other clients. Linux clients retry after 100 ms and never showed the stall.

Acceptance on a macOS or Windows client, before the fleet relies on it: save a file on a machine of another OS in the same group and watch
it appear (the sync latency probes of the ticket folder, `nfs-xwrite.py` and `nfs-xread.py`, do exactly this); it must appear within
about a second, not after 3 s to 60 s. If it does not, the settings of that row are not enough and the preflight cannot see it.

Flags of the scripts: `--user` (default: the sudo caller) is the account that owns the worktree and runs the daemons.
`--archive <directory>` replaces the store of the group as the source of the archive (a peer group mount, see day 0).
`--first-machine` (Windows `-FirstMachine`) is for the very first machine of the fleet: a checkout is at the canonical
path, the store is empty, and `seed` runs instead of `join`; it is refused when the store already has origins.
`--server` (Windows `-Server`) marks the group server, the first one (`--first-machine --server`) or a later one, on the platform
that the topology names for it (the script refuses another one). The script does the whole server setup of the
`server-plan` of that platform: creates the folders, writes the exports (Linux `/etc/exports.d`, macOS `/etc/exports` and `/etc/nfs.conf`;
Windows `New-NfsShare` or `New-SmbShare` plus firewall rules), mounts or reads the peers, installs the cache and relay
services and starts them after the join. It never writes the cache tokens: `/etc/semio/remote-cache.env` (Windows
`C:\ProgramData\semio\remote-cache.env`) must exist with real values for both lines of `server-plan --part cache-environment`, or the
script stops. On Linux the store is the export bind-mounted at the store mount (no NFS client entry); on macOS and Windows the
store root is the exported folder (`--store`). The daily compaction is installed with `--collect-blobs`; a later server (no
`--first-machine`) needs `--archive` (Windows `-ArchivePath`) and, before settle and join, runs `relay --once` with the peers of
the `relay-peers` part to copy the other groups into its still empty store.

## Day 0

1. **Topology.** Edit `🗺️topology/🔣️.json` (the one file, see "The one file the operator edits"): group ids (`g1` to `g5`), the NFS export,
   server platform and protocol, peer mount points per group, the canonical worktree
   path and store mount per OS, and the replica timings. Leave the placeholder host, network and cache name in the file, and set `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST` and `SEMIO_FLEET_NETWORK` on the machine. Run `validate`, then `validate --reachable`. The topology is a shared source file, so after the first machine has seeded, edits to it propagate
   like any other file; change it on one machine and let it replicate.
2. **Canonical path.** Cache hits across machines need identical checkout paths, so every replica lives at the
   worktree path that the topology fixes for its OS: a template below the home folder of the account that edits
   (macOS `~/Documents/semio`, Linux `~/semio`) or the fixed short root `C:\semio` on Windows. The provisioning resolves
   the tilde against the real home of that account (`dscl` or `getent`, never the `HOME` of root under `sudo`); homes with
   spaces or non-ASCII characters work. Do not pick another path, not even a symlink to it. There is exactly one
   repository per machine: the existing one, turned collaborative.
3. **Cache tokens.** Create two tokens of the characters `A-Za-z0-9._~+/=-`, for example `openssl rand -hex 32`:
   a read token and a write token. Keep them in your password manager.
4. **First machine: the server of the first group (say `g1`).** This is the only machine that starts from a checkout.
   1. `server-plan --group g1 --part prepare` prints the commands that create the `semio` account (uid 1500, home
      `/var/lib/semio`, it owns the worktree, the store and the cache and runs all server services), the store and
      cache directories, `/etc/semio` and the mount points (Linux). On macOS and Windows it prints the folders and
      the Windows features or local accounts instead; apply it. The script writes the exports and the fstab block itself
      (step 4 below); a peer whose server does not exist yet is skipped with a warning and mounted at boot (`nofail`) or by the next run.
   2. Write `/etc/semio/remote-cache.env` (owner root, group `semio`, mode 0640) with the two lines of
      `server-plan --part cache-environment` and the real tokens. The cache server refuses a non-loopback
      address without a token, and the unit binds `0.0.0.0`.
   3. Copy a complete checkout (a root `.git` may be included; only its allow-listed part replicates) to the worktree path and `chown -R semio:semio` it. Put the
      `🗺️topology` directory somewhere the script can read it.
   4. Run `bash 🥾️provision/🐚️.sh g1 g1-server --user semio --first-machine --server`. It builds the launcher, mounts the store, seeds it (about 270k
      files, 5 GB, as long as it takes to hash and copy 5 GB to NFS), installs the replica and compact units and starts them, and runs the native bootstrap last
      (give `semio` sudo rights for the packages until it has finished, or install the toolchain first; exit code 3 means that the server is ready and only the native build is pending).
   5. The script already installed the cache and relay services (Linux units, macOS LaunchAgents, Windows tasks) and started them after the join;
      `--part start` prints the equivalent commands. The relay builds the first bootstrap archive. Run `snapshot` once so that
      later machines join from a snapshot instead of replaying the journal. Check `http://<cache host>:6090/healthz`.
   6. Never run `seed` or `--first-machine` on a second machine. It would publish a second, competing copy of every file.
5. **The other four group servers (`g2` to `g5`).** On each: apply `prepare` (it mounts all peer exports, including `g1`), write
   the token file as above, then
   `bash 🥾️provision/🐚️.sh <group> <group>-server --user semio --archive /mnt/semio-peers/g1/v1/bootstrap --server`
   (the `provision` part of `server-plan` prints this line with the first peer's mount), then `--part start`. The new
   server has no archive of its own yet, so it takes the archive of `g1` from the peer mount, runs one relay round
   to fill its own store, settles and joins. Its relay then builds the archive of its own group. Wait until `status`
   shows the other origins before you join workers in that group.
6. **Workers.** On every other machine, as root (macOS, Linux) or elevated (Windows): copy the `🗺️topology` directory,
   run the provisioning script with the group and a fleet-unique origin id (`^[a-z0-9][a-z0-9-]{0,39}$`, for example
   `g2-m07`) and `--user <account>` (Windows `-User`). Then give the account the write token (see Cache tokens
   and the worker environment) and run `verify`; it must exit 0, then run `preflight` (below).
   **Which machine of a group is the server.** Only a machine with a 2 TB disk can be (the fleet's machines have 128 GB to 2 TB of
   NVMe, and the store of a group needs 0.5 to 1.5 TB, see Capacity): every group needs at least one 2 TB machine, and that one
   is the group server. Take the one that is always on, on a wired network, with 8 cores and 32 GB of RAM; a group without a 2 TB
   machine gets a dedicated NVMe disk for its server, or a shorter `replica.retentionDays` in the topology (7 days halves the blobs). The 128 GB machines are
   workers only. The preflight on the server fails below 50 GiB and warns below 300 GiB free in the store volume.

   **Preflight.** After provisioning, on every machine (worker or group server), run in the topology folder
   `repo ./📜️script.ts preflight` (`--group <id>` when `.cache/f/machine.json` is missing, `--timeout <ms>` per check, default 5000, `--lang en|de`).
   It reads and measures, and changes nothing but one probe file in `<store>/v1/tmp` that it creates, reads back and deletes at once.
   Checks, each with its own verdict (`ok`, `WARN`, `FAIL`, `skip`):

   | Check | Passes when |
   |---|---|
   | identity | `machine.json` exists and names this group; the origin `<group>-server` is the group server |
   | store mount | the store is mounted from the source of the topology (Linux and macOS: `mountinfo`/`mount`; Windows: `mount.exe`, then `net use`); on a Linux group server it is a local bind mount, on macOS and Windows servers the folder exists |
   | mount options | Linux: `hard`, `lookupcache=none` and the one-second attribute caches are in effect; macOS: `acdirmin=0`, `acdirmax=0`, `nonegnamecache` (a warning when `nfsstat -m` does not show them); skipped on Windows (see client caches) |
   | store writable | a probe file in `v1/tmp` is written, read back and removed |
   | cache endpoint | `<endpoint>/healthz` answers 200 with `status: ok` |
   | clock skew | the clock differs from the group server by at most 2 s (warning above, failure from 30 s). It uses the `Date` header of the server's cache endpoint (one-second resolution, round trip subtracted), so it is a measurement to about 0.5 s; 30 s is half of the 60 s after which a receiver holds an event back, so that two machines that are each below it still differ by less than 60 s. The group server itself is the reference and skips this check |
   | time synchronisation | the OS reports a synchronised clock (`timedatectl` NTPSynchronized, `systemsetup -getusingnetworktime`, `sc.exe query w32time` running and a `W32Time` type other than `NoSync`); a warning otherwise (the provisioning scripts enable it: `timedatectl set-ntp true`, `systemsetup -setusingnetworktime on`, the `w32time` service; chrony is the recommendation on Linux) |
   | inotify limits (Linux) | `max_user_watches >= 1048576` and `max_user_instances >= 1024`, the values of the sysctl file |
   | long paths (Windows) | `LongPathsEnabled` is 1 |
   | client caches (Windows) | the SMB lifetimes or the NFS registry values are 0 for the protocols this machine reads |
   | git version and pins | git 2.32 or newer (it honours `GIT_CONFIG_GLOBAL`); the pins hash is attested, the fleet verdict compares it |
   | private exclude rules | `.git/info/exclude` has no rules (failure otherwise: no flag removes them from git's view) |
   | case and normalisation | no tracked path is spelled differently on this disk (warning) and no rule of the root `.gitignore` hides an existing path only under a case-insensitive match (warning); the volume's behaviour is shown |
   | tracked paths | no tracked path or link is refused by the path policy (failure), none is ignored by a rule (warning), no executable bit differs from the index mode (warning) |
   | path length | every tracked path fits this operating system below this worktree (`pathBudgetV1`): a failure on Windows without long paths, a warning elsewhere |
   | symbolic links | links can be created on the worktree volume (a warning; a failure when the repository tracks links) |
   | free disk | the worktree volume has at least 10 GiB (warning below 30 GiB: tree 4.5 GiB, bootstrap copy 5 GiB, caches); on a server the store volume at least 50 GiB (warning below 300 GiB) |
   | native build | the completion marker `.cache/f/native-bootstrap.done` of the provisioning exists; **pending** (a warning, never a failure) while the native bootstrap has not completed, with the log `.cache/f/native-bootstrap.log` and the way to retry it (run the provisioning script again) |
   | services | the replica daemon runs (systemd instance, LaunchAgent `state = running` in the `gui/<uid>` domain, scheduled task state `Running` through `Get-ScheduledTask`), the `access` check names `worktree-access-denied` with the exact consent to give, the daily compaction is scheduled (a warning otherwise), and on a server the cache and the relay run |
   | replica status | `status` reaches the store, the daemon runs, no origin needs a new `join`; a warning when an origin is more than 1,000 events behind |

   The exit code is 1 when a check failed or Ctrl+C cancelled the run. Progress lines (`[n/total] mark name: detail`) go to stderr as each check finishes.
   Its probes (mount tables, registry, `schtasks`, `timedatectl`) are tested only with injected outputs and, on macOS, with the real
   system calls; the parsing of the real `mount` and `nfsstat -m` output of macOS 27 is verified (`mount options: ok` for the plan, `FAIL ... acregmin=1 (found acregmin=5), acregmax=1 (found acregmax=60), ...` for the defaults);
   `mount.exe`, `net use`, `reg query`, `sc.exe` and `Get-ScheduledTask` output are unverified on a real Windows host; their parsers read untranslated tokens only (drive letters, `REG_*`, enum names, state numbers).

7. **Archivist (optional).** Designate one machine for history. It is an ordinary fleet machine that commits its
   replica on a timer and pushes `main`. A commit on any machine replicates (see "Git state"), so one archivist
   keeps the history linear: the skills `commit`, `micro-commit` and `merging` in `.agents/skills/` apply only there.

## Git state

The root `.git` is shared by an allow-list: `HEAD`, `packed-refs`, `shallow`, everything below `refs`, and everything below `objects`
except names starting `tmp_`, `objects/info/alternates` and every name ending `.lock`. Everything else in `.git` (index, config,
logs, hooks, info, `FETCH_HEAD`, `ORIG_HEAD`, in-progress operation files, IDE files) stays on the machine that wrote it. A path that
becomes private is forgotten locally and never published as a delete; incoming events for a private path are dropped.

**The stage.** `.git/index` holds per-machine stat data, so it never travels. The stage is a register per path plus one base register in
a virtual realm under the sealed `.cache/f`, carried by ordinary `link` and `delete` events: `.cache/f/index-base` (`tree <oid>`) and
`.cache/f/index/<worktree path>` (`<mode> <oid>`, or `delete` when the path is absent from the stage). Effective stage = the base tree
overlaid by every entry whose hlc is greater than the base's; per register the greater hlc wins. The realm is never written to disk.

- Publish: when the stat signature of `.git/index` moved (checked with every hot pass, about every 300 ms, and by the watcher) and on every full sweep, `git write-tree`; an unchanged tree publishes nothing (a `git status`
  that only refreshes stat data is a no-op), otherwise `git diff-tree -r -z` against the last synchronised tree gives one entry per path.
  The last synchronised tree is the fold of the registers this machine has published or applied (`held`), computed as `verify`
  computes its fold and kept in memory afterwards, so it survives a lost `.cache/f/git-index.json`, a crash between a publish and
  the bookkeeping, and a restart. However many paths differ (30 000 were measured), they become entries, appended in batches of
  5000; a base is never published for a large change, so a bulk `git add` cannot void what another machine staged earlier. The only
  bases are the first one, published automatically by the group server (`<group>-server`) when the manifest has none (workers wait
  for it and `status` says `waiting-for-base`), and the one an explicit `rebase` publishes. `rebase` lists in its `published` event
  (`reason` `rebase`, `count`) and in `.cache/f/lost/rebase-*.tsv` (`path`, origin of the register, `<mode> <oid>` or `absent`) every
  register of any machine whose value differs from the stage it publishes, because the base voids exactly those.
- Apply: the local stage is published first, so it is never attributed to the remote; entries go into one `git update-index -z --replace
  --index-info` batch, a new base replaces the stage (the delta from the last synchronised tree while the stage still equals it, else
  `git read-tree`). When the stage has no proof of where it came from (nothing of the registers was applied yet, a lost state, a
  crash during a base replace), the shared fold wins: the old `.git/index` is copied to `.cache/f/lost/index-*`, an `overwritten`
  event (`reason` `local-stage-replaced`, `count` = replaced entries, `lost`) names the copy, and
  `GIT_INDEX_FILE=<copy> git ls-files -s` lists what was there. The paths an apply is writing are recorded in `.cache/f/git-index.json`
  before the write and cleared after the registers are settled, so a crash in between never republishes them as local edits.
  A missing `.git/index` is restored from the registers (`repaired`, `reason` `index-missing`) and no delete is published for it.
- Waiting and degraded states are never silent. The record `.cache/f/git-index.json` (atomic, fsync) holds `state` (`shared`,
  `waiting`, `degraded`) and `reason`; each change emits one `blocked` event and `status` prints it as `Git state: ...`; `fleet` shows
  the disagreement and the attested `queue.incoming` stays above zero. Reasons: `waiting-for-base`, `objects-missing` (an object
  has not arrived; the stage is untouched meanwhile), `unmerged` (a merge or rebase left unmerged entries: nothing is published, the stage is
  never reset, the registers stay queued until the conflict is resolved), `write-tree-failed`, `git-failed`, `index-busy` (`index.lock`
  held: retries back off from 1 s to 30 s), `index-lock-stale` (held for more than ten minutes: close the program or remove the file
  when no git runs) and `backup-failed` (the old stage could not be copied, so it is not replaced). The temporary indexes of the
  register fold run with `core.splitIndex=false` and `index.sparse=false`, so `index.version=4`, `core.untrackedCache`, split and
  sparse indexes of the real stage need no special case.
- A commit changes no register: it replicates as objects plus the ref file. Concurrent commits are linearised, see below.
- `verify` folds base and entries in a temporary index and compares the tree with `git write-tree`; a difference is reported as drift on `.git/index`.
- Without `git` or without a root `.git` the realm is skipped and files keep syncing. `status` prints `Git state: ...` with the reason
  (`no-git`, `no-repository`, `waiting-for-base`, `shared`); `preflight` warns when no git executable is found
  (`SEMIO_GIT`, `PATH`, Homebrew, Command Line Tools, distribution paths, Git for Windows below `Program Files`).
- A ref whose objects have not arrived stays queued (`blocked`, `objects-missing`, back-off up to 30 s) and is reported again as
  `objects-missing-timeout` with the waited seconds after five minutes; the queue count shows it in the attestation. New branches,
  tags and the `HEAD` file of a detached worktree replicate as plain last writer wins; concurrent commits on a detached `HEAD` are
  linearised like branch commits.

**Concurrent commits.** A branch ref (`.git/refs/heads/**`, and `.git/HEAD` while detached) is last writer wins per file, so two commits
from one parent would leave one of them unreferenced. The machine that authored the losing commit repairs it when the winning ref value
reaches it, with plumbing only (no worktree, no stage, no checkout):

- Trigger: applying a remote `put` of a branch ref while the local ref value was written on this machine (published by this origin, or
  changed since the last published value) and is neither equal to nor an ancestor of the incoming commit. An ancestor local value is a
  plain fast-forward; an incoming value that is an ancestor of the local one keeps the local ref, which is published again. A local value
  that was itself applied from another origin, every ref outside `refs/heads/` (remotes, tags) and `packed-refs` stay last writer wins.
- Rewrite: every local commit not in the incoming history, oldest first, is re-created (a commit whose author line and message are already in the incoming history is dropped, whatever its tree: it is the earlier rewrite of the same commit, and re-creating it would put it into the history twice) with `git commit-tree` on top of the incoming
  commit (one parent, merges flattened), keeping author, author date, committer and message. Its tree is the per-path three-way merge of
  the original parent tree (base), the new tip (ours) and the original tree (theirs), with `diff-tree`, a temporary index and
  `update-index --index-info` only: a path only the loser changed is taken from the loser, a path only the tip changed stays, a path
  both changed identically stays, and a path both changed differently (also a file against a directory) takes the side whose committer
  time is later, a tie keeping the tip. The winner's unrelated changes are never reverted and there is no fallback to a whole tree; the
  `linearised` event carries the number of such conflicting paths in `count`, the unreferenced original commit keeps the dropped
  version. `git update-ref` then moves the ref from the old
  local value to the result, and the next scan publishes it as an ordinary ref change with a greater hlc than the winner.
- Duplicates: a commit that adds nothing to the new tip (equal tree) and repeats the tip's message, or repeats an author line and message
  of a commit in the incoming history, is dropped. When all are dropped the ref simply takes the incoming commit.
- Only the origin of the losing value rewrites, so exactly one machine publishes the result; all others apply it as a fast-forward. If
  another commit wins meanwhile (three machines, a third commit during the rewrite), the same rule runs again for the then-losing
  value: every round has one hlc winner that is never rewritten, the losers re-create themselves on it, so with n concurrent commits there are
  at most n-1 rounds and the chain holds each message once (a replaced rewrite stays unreferenced). A missing object, a moved ref
  or a held lock keeps the event queued and retries it with the usual back-off. Each rewrite emits a `linearised` event naming winner,
  loser and result (`reason` `duplicate` when nothing was re-created).

**Configuration.** At daemon start, at join and when a base is first applied, `.git/config` is normalised through git: `core.autocrlf=false`,
`core.safecrlf=false`, `core.eol=lf`, `core.protectNTFS=true`, `core.protectHFS=true`, `core.longpaths=true` on Windows, `core.filemode` (false on Windows), `core.symlinks` and `core.ignorecase` from probes of the volume, `core.precomposeunicode=true` on macOS,
`core.fsmonitor=false`, `gc.auto=0`, `maintenance.auto=false`, `core.logallrefupdates=true`; remotes, branches and user keys stay, a
`commit.template` outside the worktree is removed. A joiner builds its stage from the base register and gets its own config; the bootstrap
archive carries only allow-listed `.git` paths.

## Cache tokens and the worker environment

Provisioning writes the machine environment of `env` (`SEMIO_REMOTE_CACHE_ENDPOINT` = the cache of the machine's own
group, `SEMIO_REMOTE_CACHE_ACCESS=read-write`, `SEMIO_REMOTE_CACHE_INSTANCE`): to `/etc/profile.d/semio-fleet.sh` on
Linux, `launchctl setenv` plus `/etc/zshenv` and `/etc/profile` on macOS, the Machine environment on Windows. The
build tooling resolves its remote cache contract from exactly these variables
(`📚️library/🏗️build/🌐️remote-cache`): the endpoint switches the cache on, `read-write` lets it upload local results, and the bearer
token is read from the variable named by `SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT`, default `SEMIO_REMOTE_CACHE_TOKEN`.

The scripts never write a token, because those profile files are readable by every local user. The operator gives
each worker account the **write token** in `SEMIO_REMOTE_CACHE_TOKEN` (a read token would make every upload fail with 403):

- Linux: a line `export SEMIO_REMOTE_CACHE_TOKEN=<write-token>` in the account's `~/.profile`, file mode 0600.
- macOS: the same line in `~/.zprofile` of the account, mode 0600.
- Windows: `[Environment]::SetEnvironmentVariable('SEMIO_REMOTE_CACHE_TOKEN','<write-token>','User')` as that account.

Only the build tools need the token; the replica daemons do not. Whether every way an agent starts a build
(ssh, IDE terminal, service) inherits the variable has not been checked: run `repo remote-cache status` as the
agent's own user, it prints whether the token variable is set (never its value). For a consumer that must not
upload (a CI job), give it the read token and `SEMIO_REMOTE_CACHE_ACCESS=read-only`. To rotate, change
`/etc/semio/remote-cache.env`, restart `semio-fleet-cache.service` and replace the variable in the accounts.

## Day 2

**Look at the state.** `status` on any machine shows its replica daemon, the store, the queues and the lag per
origin; `--deep` also scans the tree to count the unpublished local changes. The output of `run` (the service log) reports `refused` paths,
`overwritten` files (a local edit that lost, kept in `lost/`) and `kept` paths (a local edit that was newer than the remote one and was published), and the output of `relay` shows whether the relay is current. A refused path is a developer
problem (a name that is not NFC, a Windows-illegal name, a case collision, an absolute symlink, a path longer than 256
UTF-16 units); the file stays local until it is renamed. The 88 absolute symlinks inside ticket `mirror/` and `basetree`
trees are refused this way on purpose. Every refused path is visible in four places: a `refused` event with its reason
(`too-long`, `not-nfc`, ...) when a scan or a file event meets it, once per daemon run; `status --deep` counts them
(`refused`); `verify` lists them (`refused`, `refusedTotal`, at most 1,000 paths; they do not make `verify` fail); and
the daemon's follow of a remote event for such a path reports `refused` and drops it. A path inside the policy that the
operating system still cannot open (`ENAMETOOLONG`, e.g. a short limit and a deep worktree root) is listed by the scan
as refused `too-long` and, when a remote event meets it, reported as `skipped` (`too-long`) and dropped; it is never
silently ignored and never fails the walk.

**Windows and files held open.** A file that another process holds open (an editor, an indexer, antivirus) makes the
replica's rename, delete or write fail with `EBUSY`, `EPERM` or `EACCES`. Nothing is lost then: the incoming event is
already in the persisted queue when the cursor moves past it, the path is reported once as `blocked` (reason `EBUSY`,
`EPERM`, `EACCES` or `sharing-violation`), retried in place a few times, then at growing intervals (0.5 s to 30 s) on
every follow round, and applied after the release, also after a restart. A local file that cannot be read for publishing is
returned for retry (`blocked`) and published later; a journal write that stays blocked after all retries (`head.json`
held by the virus scanner of the store host) publishes nothing, keeps the events for the next round and never crashes
the daemon. **Symlinks without privilege:** where the account may not create symbolic links (Windows without Developer Mode
or the privilege), the entry is reported as `skipped` with the reason `symlink-unsupported`, removed from the queue
(not retried), nothing is published for it, and every other file of the round is applied; `verify` lists it as
`missing-local` until the privilege exists and the machine `join`s again or the link is created by hand.

**Verify.** `verify` rehashes the local tree and compares it with the local state and the store. It exits non-zero
on drift. Run it after an outage, after a disk problem and, as a recommendation, once a day from a timer. The
provisioning scripts do not install a timer for it.

**Compact.** Every origin compacts its own journal: the provisioning scripts install a daily compact job next to the
replica daemon (systemd timer `semio-fleet-compact@<account>.timer` with a random delay of up to 3 h, LaunchDaemon
`tech.semio.fleet.compact` at 03:30, Scheduled Task `semio-fleet-compact` at 03:30). `compact` writes a snapshot, then
deletes this origin's sealed segments below the snapshot and its older snapshots, all only when older than the retention
(14 days). It never touches another origin's directory, takes no daemon lock and can run next to `run`. Blobs are
collected only with `--collect-blobs`, and only the group server does that: `provision.sh --server` installs its job as
`compact --collect-blobs` (`worker-service --role server`; the collection streams, 2,048 doomed blobs per directory at most
wait in memory, and the references published since the fold are added before every deletion batch), the jobs of workers never delete a blob, because a worker's
view of the other origins may lag and a blob that looks unreferenced to it can still be needed. The server's own
replica sees the whole group store, including the origins that the relay copied in. A machine that has been offline for
longer than the retention resumes from the newest snapshot that is ahead of its cursor (see Compaction); only when no snapshot is ahead
it reports `rejoin-required` and has to `join` again.

**Staging files after `kill -9`.** Writers stage into `<store>/v1/tmp` (names `<origin>-<pid>-<hex>`; the relay uses
`relay-<hex>`) and `.cache/f/tmp`. Opening a replica (`run`, `seed`, `join`) removes the whole local `tmp` and the
own origin's leftovers in the store whose process is dead (a live `compact` of the same origin is spared); `compact` on any
machine removes every file in the store `tmp` that is older than 6 hours, so also the relay's and other origins'
leftovers (NFS `.nfs*` placeholders are skipped).

**Add a machine.** Provision it as in day 0 step 6. Choose an unused origin id. Never reuse the origin id of
a machine that is gone for a replica with an empty journal: other groups still hold the old journal of that
origin, and a restarted journal with the same id would conflict with it.

**Remove a machine.** Stop its service. Its origin directory stays in all stores until compaction removes
the segments that no snapshot needs.

**Replace a group server.**
1. Stop the daemons of the group's workers or let them run: they keep working locally and queue their publishes
   while the store is unreachable.
2. Provision the new server with `server-plan`, with the same host name if possible (otherwise change the
   topology and replicate it).
3. Copy the old export to it with its permissions (`rsync -a`) while the old server is still readable. This is the
   supported path: the store of a group contains the only copy of that group's own origins.
4. Start the export, the cache server, the replica daemon and the relay. The cache is only a cache; an empty one just
   misses until it is warm again.
5. When the workers see the store again they publish what they queued. Run `verify` on a few of them.
6. If the old export is lost and there is no backup, the contract defines no reverse path: relays only pull an
   origin from its home store. The fallback, untested, is to stop all daemons of the group, give every machine of
   the group a new origin id, pick the machine whose tree is right, `seed` it into the new store, and `join` the
   others. That republishes the whole tree with newer clocks, so it wins over older state. Do it in a quiet period.
   Back the export up (snapshot-capable storage plus a nightly copy) so that you do not need this.

**Recover a machine whose replica drifted.** Symptoms: `verify` exits non-zero, builds see files that other
machines do not have, or a disk was restored from a backup.
1. Copy the files you still need out of `.cache/f/lost/` (and any unpublished local edit) to a safe place.
2. Stop the `run` service.
3. Run `verify` to see what differs. If the difference is only unpublished local edits, start `run` again and
   they publish.
4. Otherwise `join` accepts only a tree whose shared files match the store, so move the differing files away (or empty
   the worktree and provision the machine again), run `join --group <groupId> --origin <originId>` with the same origin
   id, then `verify`, then start `run`.

**Clock skew.** A receiver does not apply an event whose clock is more than 60 s ahead of its wall clock: it reports
`clock-skew` once per outage, keeps its read position in front of that event, goes on with the other origins and applies
the event when its own wall clock has caught up. Keep every machine on NTP: the provisioning scripts check it and try to
switch it on (Linux `timedatectl set-ntp true`, with a warning that recommends chrony when the clock stays unsynchronised; macOS
`systemsetup -setusingnetworktime on`; Windows the `w32time` service set to automatic and started), and `preflight` measures the
skew against the group server (warning above 2 s, failure from 30 s). If one machine ran ahead, fix its clock; its held events
then arrive by themselves.

**Store unreachable.** Workers keep editing and building locally and queue their publishes. They need no action when
the server returns. Check `status` afterwards.

## Compaction

The journals grow with every publish, a republish storm multiplies them, and a machine that is behind, new or rebuilding reads all of it.
Compaction makes that cost follow the live paths. Everything automatic is **off** (`replica.compaction.automatic` is `false` in the
topology); the explicit command works regardless.

**Snapshot.** A snapshot is the fold of the whole store (all realms, stage registers included) plus the cursors it covers, one JSON file
`snapshots/<writer>/<hlc>.json` written through tmp, fsync and rename by any one machine (`compaction --apply`, the daily `compact`, the
group server when automatic). Two machines writing at once is safe: each file is a complete fold at its own cursors, readers take the newest by hlc
and then follow the journals from the cursors in the file, and since a fold is last-writer-wins per path by hlc, snapshot plus tail equals the
full fold (property-tested against an independent fold and `🗜️compaction/🧪️tests/🐍️.py`). A joining, rebuilding (`state-rebuilt`) or `verify`ing
machine loads the newest snapshot; a replica that is behind jumps to it (below).

**Catch-up.** With `automatic` on, a replica that is at least `catchUpEvents` (20,000) events behind (estimated from the heads) and finds a
snapshot that saves at least that many reads the snapshot instead (event `progress`, stage `jump`), applies its entries by the usual
rules (local edits that are newer are kept, older ones go to `lost/`) and follows the journals from the snapshot's cursors. Independent of the setting, a replica
whose cursor points into a segment that was removed resumes from the newest snapshot that is ahead of it (stage `rejoin`); only if there is none it reports
`rejoin-required`.

**Segment retirement.** A journal segment of any origin may be removed when all of these hold: it lies wholly below the snapshot's cursor for that origin and below the
head segment; every origin that attested within `staleHours` (12) with synchronisation on has attested a cursor past it (so a live machine is never forced to jump); it is
at least `safetyHours` (24) old; and it belongs to the prefix of that origin's segments that satisfy this (no holes). Origins that are silent for longer than `staleHours`,
have `sync` off, are retired (`retire`) or never attested do not hold segments back: they resume from the snapshot when they return. Only `wallMs`, `sync` and `cursors` of
`attest.json` are read, so a change of the rest of the attestation cannot make a machine invisible; an attestation that cannot be read holds back with no cursors until
its file is `staleHours` old. The plan is reported by `compaction --plan` (and `status`) and the Python oracle computes the same segments. Removal goes oldest first, one `unlink` at a time.

**Blob reclaim.** A blob is removed when no entry of the newest snapshot and no event after the snapshot's cursors names it and it is at least `safetyHours` old (mtime).
To be safe against a publish that is in flight, the blob is first renamed into `v1/trash`, the events appended meanwhile are read, and it is put back if it is named now or if
its mtime changed since the scan; a publisher that finds its blob already stored touches it (`utimes`) before it appends the event, and stores the blob again if the touch finds
it gone (`touchBlobV1`; the relay does the same for blobs it already holds). A blob written before its event is young and stays. A file that goes back to the content of a
reclaimed blob simply publishes that blob again. Blobs that only an event before the snapshot names (a superseded version) are reclaimable: a replica that still reads that old
event sees one `blob-missing` for a version that is superseded in a later slice. `.cache/f/lost` is local to a machine and is not a reference; nothing in it needs the store blobs.
A reclaim whose snapshot was overtaken by another compaction (a segment it needs vanished) puts everything back and stops (`aborted`).

**Crash safety.** One compaction at a time per store (`v1/compaction.lock`, `mkdir`, refreshed every minute, taken over after two hours; `locked` is reported). A kill at any
filesystem mutation leaves a store that folds to the same manifest; set-aside blobs of a dead run are put back by the next one (`recoverTrashV1`); the next run finishes the work
(`🗜️compaction/🧪️tests/💥️killpoints.ts` kills a full compaction before and after every mutation).

**Visibility.** `status` shows the journal (bytes, segments, origins), the newest snapshot and its age, what the retirement rule would remove right now, and what the last applied
compaction removed and left; `status --deep` also lists the blob directory and measures the blobs a reclaim would remove now (about 30 seconds for 240,000 blobs; `compaction --plan` reports the same). All texts exist in English and German.

**Litter.** macOS clients leave AppleDouble sidecars (`._<name>`) and `.DS_Store` files in the store (about 230,000 of them next to the blobs on the live store, one per blob); no name of the store begins with a dot, so `compaction --apply` removes every such entry older than `safetyHours` from the blob, head, snapshot, origin and segment directories and reports it as a `litter` progress count.

**Enable it.** In the topology file set `replica.compaction.automatic` to `true` (thresholds: `snapshotEvents` 100,000, `safetyHours` 24, `staleHours` 12, `catchUpEvents` 20,000).
The group server (`<group>-server`) then checks every ten minutes whether the newest snapshot is missing, `snapshotEvents` events behind or, with anything behind, `safetyHours`
old, and runs the compaction in its daemon; every replica enables the jump. First try `repo ./📜️script.ts compaction --plan` on the group server, then `compaction --apply` once by hand.
The daily `compact` job stays as it is (snapshot and the retention-based cleanup of the own origin); it is no longer needed for retirement.

## Capacity

Stated hardware: machines with 4 to 16 cores, 8 to 128 GB RAM and 128 GB to 2 TB of NVMe.

- **Replica memory.** Measured on the real tree (269,331 shared files): an idle `run` daemon holds 240 to 290 MB resident
  and `seed`, `join` and `verify` peak below 520 MB (before: 0.8 to 1.3 GB idle and 1.8 to 2.8 GB peak). The in-memory state is
  one compact row per path, not a map of objects; plan about 1 MB of resident memory per 1,000 shared paths on top of the
  runtime's 30 MB. Measured on Linux (Bun 1.4.2, 288k paths): idle `run` 207 to 229 MiB, `join` into an empty directory 342 to
  410 MiB, `verify` 296 to 374 MiB. The inotify watches add kernel memory that the process does not show: three daemons on
  three full trees (468k watches) left about 1.0 GB of slab on a 3 GB machine, mostly the directory inodes that the watches keep
  in memory, and a Python tool of 1.2 GB was killed by the OOM killer next to them. **Worker minimum: 8 GB RAM works** (measured:
  about 0.25 GB for the idle daemon, about 0.5 GB at the peak of `join` and `verify`; the rest is for builds and the inotify slab of about
  130 MiB per tree); more for every extra replica tree.
- **Replica.** The shared tree is about 270k files and 5 GB. A replica fits the smallest 128 GB disk. The disk
  is pressured by ignored data, not by the replica: the build partitions in `.cache/b`, Cargo and Bun outputs and
  `node_modules`. Run `repo preflight` on the 128 GB machines and size their cache budgets accordingly. **Worker minimum: 128 GB
  NVMe** (measured: the tree is 4.5 GiB; the fleet preflight fails below 10 GiB free and warns below 30 GiB). A bootstrap
  needs another 5 GB of free space for the local copy of the archive while it is extracted. Such a machine is a worker, never a group server.
- **Store.** The store is the blobs plus the journal.
  - Blobs are whole files, stored once per content hash, never deltas. The first seed adds about 5 GB, minus
    duplicate contents. Every later save of a changed file adds one new blob of that file's size.
  - The journal is one JSON line per change, roughly 200 to 400 bytes. The seed alone is about 270k lines, under
    100 MB.
  - The bootstrap archive is one more copy of the shared tree (about 5 GB) per group store.
  - Blobs and sealed segments older than 14 days (`replica.retentionDays`) are dropped by `compact` when no snapshot needs
    them. Steady state is the live tree plus 14 days of churn. **Per group server: 0.5 TB (agents work 8 h a day) to 1.5 TB (round the
    clock) of blobs, plus 10 to 31 GB of journal, the tree (5 GB), the two newest snapshots per origin (up to 9 GB) and the bootstrap
    archive (5 GB). Recommendation: a disk of at least 2 TB.** Basis: measured per operation (11.3 KB of blobs and 258 bytes of
    journal per operation in the 50-machine rehearsal) times the rate of 100 operations per second for the whole fleet, so the totals
    are extrapolations (`scale-report.md`), not measurements of a fleet. A 2 TB disk is enough for 8 h a day with room to spare and
    tight for round-the-clock work (1.55 TB plus the 0.2 TB cache directory below is 88 % of 2 TB): shorten `replica.retentionDays`
    (7 days roughly halves the blobs) or give the server a larger disk. Watch the real growth for two weeks and adjust. Because
    every store holds all groups' data, the figure does not shrink with the group size. The fleet's own machines have 128 GB to 2 TB
    of NVMe, so the machine of a group that has the 2 TB disk is the group server (Day 0, step 6).
  - Use local SSD or NVMe on the group server, not another network file system. Back it up.
  - **Compaction cadence.** Every origin compacts daily (workers and servers: systemd timer with up to 3 h random delay on
    Linux, 03:30 as LaunchDaemon on macOS and as Scheduled Task on Windows; installed by the provisioning scripts), and the group
    server's daily job is `compact --collect-blobs`. Daily is a limit, not a taste: the blob collector keeps every doomed file in memory
    while it runs, which the rehearsal extrapolates to 0.6 to 1.7 GB for one day of garbage and 4 GB for a week. A worker compaction
    takes 2 to 7 s on the real tree (peak 460 MB), a collection a few minutes to 30 minutes on a day of garbage.
- **Poll load.** An idle follow round costs two `stat`s (`v1/origins` and `v1/heads`) whatever the number of origins. Every origin that
  publishes leaves, after `head.json`, an empty file `v1/heads/<origin>_<segment>_<bytes>_<events>_<hlc>` whose name is its head; it is
  created before the previous name of that origin is removed, so a reader sees both or the new one and takes the later. Each create and
  remove changes the mtime of that one directory. A follower lists the directory with one `readdir` only when its signature (inode, mtime,
  size, link count) changed, once more after 30 ms and after 1.1 s without a further change (a coarse file system timestamp can hide a second
  change inside one tick), and every 10 s (a client whose attribute cache is stale), and reads the heads from the names without opening a file.
  The flat head is a hint, never the truth about the journal bytes: every 10 s the follower polls the `head.json` of all origins as before
  and takes the later head, and an origin without a flat head (code that does not write it, a failed write, a store that has no `v1/heads`)
  is polled through its `head.json` every round, so a fleet that is half upgraded works and a lost hint costs at most ten seconds. Code
  that does not know the directory ignores it. The relay reads the flat heads of the peer store the same way and writes them for the
  origins it carries. Measured on this Mac with a synthetic store on local APFS (`🧰️scripts/1113-bench-store.ts`, not NFS): store operations
  per idle round 5.2 to 2.3 at 5 origins and 52 to 4.1 at 50 (2 between the 10 s audits), so 208 to 17 per second at 50 origins. What an
  NFS client does is not measured: the table "Client caches" measured how fast a replaced `head.json` shows, and the directory mtime
  has to be re-measured on a worker (Linux `actimeo=1`, macOS `acdirmin=0`) before the 10 s audit is lowered or raised. The figures
  of the table (65 NFS calls per second for four heads on macOS, 32 RPC/s for two heads on Linux) describe the old per-head poll, which
  now only runs for origins without a flat head and every 10 s.
- **Catch-up.** A replica that is far behind reads at most one segment and 1,024 events per origin and slice, applies them, and goes on
  with the next slice only while less than half a second has passed, so the loop publishes its own changes and attests at least that
  often whatever the backlog (100,000 events behind: one round of 30 s before, rounds of at most about one second now). Journal lines are validated by a
  hand-written twin of the schema (`isEventV1`, `isManifestEntryV1`, `isHeadV1`, `isPathV1`) whose verdict is tested against the generic
  validator, Ajv and the Python oracle on fuzzed values; reading 272,000 events takes 0.18 s instead of 1.1 s.
- **Latency (measured).** A save appears on another machine of the same group after about 0.3 to 0.5 s (Linux NFS 4.2 with the mount
  options above: median 305 to 388 ms, p95 424 to 535 ms; macOS rehearsal with 50 daemons on local stores: median 343 ms, p95 696 ms; **macOS 27 against a Linux server over real NFS 4.0, 200 saves each way
  (`mixed-os-report.md`): Linux to macOS median 324 ms (p95 436 ms), macOS to Linux median 347 ms (p95 477 ms), none overtaken**) and on a machine of another group
  after about 0.8 to 1 s (**two Linux group servers over real NFS, 200 saves: group 1 to a worker of group 2 median 839 ms, p95 1,318 ms; group 2 to group 1 median 1,000 ms, p95 1,475 ms**;
  the relay polls once a second, so a copy waits up to one second and a half). The peer mount must have the plan options: with the NFS defaults the relay saw a replaced head
  up to 3 s late, the median was 889 ms, p95 2,944 ms and 91 of 200 saves were overtaken before they were copied).
  Files of 14 to 48 MB take 0.7 s in the group and 1.1 s across groups at the median. A host at load or a store that was away shows up in the tail first
  (p95 of 5 s while a daemon catches up, up to 30 s after a 30 s store outage).
- **Writes.** A save writes one blob, appends to one segment, replaces `head.json` and swaps the flat head name. The file is read once:
  it is hashed while it is copied into `v1/tmp`, so the blob is named by the bytes that were written (a file of the same size as its
  recorded version is hashed first, because an unchanged file then needs no copy at all; a blob that already exists is dropped after the copy,
  or not copied when the hash is known). Durability order, which nothing weakens: the blob is fsynced before its rename (a blob name never
  exists without durable bytes; on NFS the close of the file commits as well), the events are fsynced (one write and one fsync per
  segment and batch of up to 256 events), `head.json` is fsynced before its rename, the flat head is a hint and is not synced. A file
  that changed while it was read is not published (`unstable`, retried). Applying copies from the store into `.cache/f/tmp` while hashing
  (one read, no read back, no fsync: the tree is a cache and a rescan restores the state) and compares with the name before the rename,
  so a blob whose content does not match its name is never applied (`blob-corrupt`, the path stays queued); the relay copies and verifies
  each blob the same way and fsyncs it before it appends the events that name it. From 64 MiB a copy first tries a copy-on-write clone
  (where source and target share a volume, as on the group server) and otherwise streams in 1 MiB chunks through two buffers, so memory does
  not follow the file; progress events (`publish-blob`, `apply-blob`, `relay-blob`, `done` of `total` bytes) and cancellation work
  between chunks, and a cancelled copy leaves nothing in `v1/tmp`. A file that was applied is recorded as seen two seconds after its own
  timestamp (when that has sub-second precision), so the next full sweep does not hash it again. Measured on this Mac, local APFS: 1,000 small
  files 380 ms to 308 ms and 5.0 to 3.3 store operations per file (1 fsync per file as before); one 1 GiB file published in 5.3 s before and
  0.7 s now (a clone and one read), 1.25 s by stream; applied in 1.0 s before and 0.6 s now. Large bursts (a
  generated tree committed by a tool, a dependency update) are many small files; expect the burst to take
  seconds to minutes, not milliseconds.
- **Relay traffic.** Every blob crosses the links between groups once per other group. Peak traffic is the
  size of the changes times four. Keep the cross-group links well above the sustained change rate.
- **Cache server.** It stores build results of its group only; the default store is 2.5 GiB of a 3 GiB
  budget, which is too small for ten machines. The generated unit sets `--max-bytes` to 200 GiB, so put the store
  on its own NVMe volume of at least 250 GB (`/var/lib/semio-remote-cache`) and adjust the number in the topology
  module if your volume differs. Watch `repo remote-cache status` for the hit rate and evictions and grow the
  volume if recent entries are evicted before they are reused. Entries are limited to 512 MiB by default.
- **Server memory and cores.** The NFS server needs memory for the page cache of the active tree (a few GB
  beyond the store's hot set) and a few cores. The relay and the cache server are light on CPU. Recommendation: 8
  cores, 32 GB RAM for a group server, 10 GbE if the group network has it.

## What breaks the model

- A worker at a non-canonical path builds, but never gets cache hits from other machines.
- A path ignored too late: a new generated or machine-local file that the root `.gitignore` does not ignore is
  replicated to everyone. Fix the rule first; then delete the file, which also replicates.
- Two machines amending or rebasing the same branch at once, or force-moving a branch ref: such a ref change is an ordinary last-writer-wins write, only plain commits are linearised.
- Reusing an origin id with a new journal, or running `seed` twice.
- A clock more than 60 s ahead of the others.
- Expecting the keystroke order across machines: two writes to one file within the publish latency are ordered by
  publish time (see "Two writers of one file").
- A Windows NFS mount of the store under a name with non-ASCII characters. The store only uses ASCII names; keep
  `storeMount` ASCII too.

## Not verified

The Windows provisioning script ran end to end under PowerShell 7 against a mock layer (`🗺️topology/🧪️tests`, skipped without `pwsh`): the parser accepts it and every generated snippet, the scheduled task XML validates against the published Task Scheduler schema (set `SEMIO_TASK_SCHEMA` to the schema file and the test runs `xmllint` against it), and the recorded cmdlet calls follow the promised order for the NFS and SMB variants, first, joining and repeated runs. No Windows machine ran it, so the real cmdlets, executables and services stay unverified (the operator checklist is in `windows-ci-report.md` of the ticket folder). Nothing else here was run. The server setups for macOS and Windows, the SMB fallback, the inotify step and the repeated run were checked
only statically on a macOS host (see `provisioning-review.md` in the ticket folder for the list that could not be executed):
`nfsd` and its `exports` syntax with two lines for one folder, the NFS version that `nfsd` serves, `New-NfsShare` and
`Grant-NfsSharePermission` precedence between the all-machines entry and the per-peer read-only entries, `cmdkey` with a
local account name, scheduled tasks at logon on a server, the long-path setting for every tool that opens the tree.

Verified on Linux (Ubuntu 24.04 arm64 VMs on a macOS host, kernel NFS 4.2 server and client; figures and commands in
`linux-nfs-report.md` of the ticket folder): the provisioning script as server and as worker, the exports, the fstab block and
line, the sysctl file, the systemd units (`systemd-analyze verify` clean) and the timer, the cache server and the relay under
systemd, `/usr/local/bin/repo` as a symlink to the per-account launcher under systemd, the bootstrap archive through the NFS
mount (sha256, `tar -xf`, file count), `bootstrap-settle` followed by `join` on a real extracted tree, seed, join, run,
verify, an NFS outage of 30 s and `kill -9` of a daemon in the middle of a publish.

Preflight and the macOS and Windows client caches: `preflight` was run against injected probe outputs (Linux, macOS, Windows, worker and
group server, every verdict) and once on a macOS machine with the real system calls; the real outputs of `mount` and `nfsstat -m` of macOS 27 and of `/proc/self/mountinfo` are parsed correctly (see below), those of `net use`, `reg query` and
`schtasks` were not parsed yet. The Windows SMB and NFS client settings are by reasoning (see "Client caches"); the Windows NFS
registry value names in particular were not verified on a Windows machine.

Verified between systems (`mixed-os-report.md` of the ticket folder): a macOS 27 machine as an NFS 4.0 client of a Linux group server (an unprivileged mount, the
plan's options, latency both ways, bursts, NFC and NFD names, case-only renames, executable bits, symlinks, a 30 s server stop, idle cost, the real
`mount` and `nfsstat -m` output in `preflight`), two Linux group servers with real peer mounts (relay both ways over NFS, a 30 s stop of one server, `kill -9` of the
relay in the middle of a copy, the `--archive` peer flow), and the native bootstrap on fresh Linux machines up to its last step: `cargo` passes once the C toolchain
that the provisioning script lists is installed (without `cc` the pinned `cargo build` exits 101 with ``linker `cc` not found``), but `repo run workspace:setup`
fails in the resident graph service on every machine tried (first `Cannot find module 'typescript'`, then `Original captured runtime entry provider required`),
so the completion marker is only written by hand, see the report.

Still unverified: a macOS group server and NFS 3 for a macOS or Windows server, a macOS worker whose mount is made by the LaunchDaemon (the tests mounted by hand, unprivileged, with
`noresvport` against an export with `insecure`), the other distributions' package managers, the figures under the production load, the tar writer against bsdtar on
Windows with the real tree, the native bootstrap as a service account on macOS, the scheduled tasks and their XML on Windows, the Windows NFS and SMB client settings,
and `compact` that removes segments or blobs while a daemon runs (on Linux it ran next to the daemons and found nothing to remove).

## Resilience (self-restart, store stalls, recurring failures)

- **Self-restart is proven first.** When the code digest on disk (module sources plus every file they import: shared libraries, JSON schemas, text catalogues) differs from the running digest for 5 s, the daemon starts a fresh process (`<runtime> 🔁️replica/📜️script.ts self-check`). It loads every module, compiles every schema definition, checks the text catalogues in all languages and dry-opens the current local state read-only. Only on exit 0 with the same digest does it restart, after a per-origin delay of 0 to 60 s (sha256 of origin and digest), at least 30 s after its own start and while the incoming queue is empty (given up after 5 minutes). A failing check keeps the old code running: event `restart-refused`, `pending:{code,ok:false,reason}` in the attestation, `status` line; it is repeated only when the digest changes again.
- **Reverts are not adopted.** `.cache/f/code/history.json` records the digests this machine ran and the digests it refused. Code on disk that was refused, or that ran before and was superseded, is not checked and not restarted into: event `code-reverted`, `pending` in the attestation, the daemon keeps its code. To roll back on purpose, delete that file.
- **Last known good.** After every passed check, and at every start, the checked closure is sealed into `.cache/f/code/<digest>/` (mirror of the repository paths, `sealed.json` written last, the newest two kept). The service entry `📜️script.ts` imports nothing but Node built-ins: it loads the worktree code, self-checks it, and runs the newest sealed snapshot that loads instead when the worktree code fails or is a revert (`self-check` itself never falls back). The fix then arrives through the replica and the next restart picks it up. No re-provisioning is needed: the entry file is replicated like any other.
- **A stalled store does not freeze the daemon.** A filesystem call blocked by a hard NFS mount cannot be cancelled. The daemon probes the store with one outstanding probe; when it has not answered for `storeDeadlineMilliseconds` (10 s) the round is abandoned and aborted, `store-unreachable` (reason `stalled`) is reported, no further store call is issued, and the loop, hints, state persistence and the health file keep running. When the store answers again it resumes with a full walk, no restart. Calls already in flight stay blocked until the kernel returns. `status` shows `health` (store state, refused code, recurring failures).
- **Recurring failures stay visible.** Each failure key is counted; the first occurrence is announced, a repeat at most once a minute with `count`. Counts and last time per kind are in `status`, `.cache/f/health.json` and the attestation (`failures`). Failing rounds back off from 250 ms to 30 s.
- **One machine per origin.** `machine.json` carries a random key; the first publish records it in `origins/<origin>/home.json` by exclusive create. A machine with another key is refused (`OriginTakenV1`) at open, join and publish. The daemon lock carries the boot time and a 5 s heartbeat; steals are serialised by `daemon.lock.steal`.
- **Silent drift heals itself.** Every two minutes (`driftMilliseconds`) the daemon reads the attestations of the fleet; when its own origin is `diverged` in the manifest against the settled origins it reports `drift` (fields, check number) and runs the repair: it takes over the store registers it lacks or holds older, drops manifest rows the complete store fold does not have, queues every manifest entry whose applied row is missing or carries another hlc, and forgets applied rows without a manifest entry (the next walk publishes the file if it still exists). Repeated findings back off from two minutes to one hour, and after three repairs that did not end the difference (a machine that ignores a path the others share differs by design) the daemon stops repairing until the difference has gone, so a by-design divergence never costs a store fold every hour. Differences in code, git state or view are never repaired automatically.
- **Clocks.** Stamps more than 60 s ahead of the wall clock are refused (`ClockAheadV1`, event `clock-refused` at open) instead of freezing the origin; the fleet verdict votes `code` and treats an attestation more than 30 s in the reader's future as stale.
- **Journal.** A segment shorter than its head is sealed at its last complete line and the append continues in the next segment; snapshots are refused while a fold needs a rejoin; an event of an unsupported version stalls its origin visibly (`unsupported`) instead of being skipped; existing blobs are rehashed once per process before they are trusted.
- **Bounds.** `.cache/f/lost/` is pruned by age and to 2000 entries, the service log and the native bootstrap log are rotated in place at 64 MiB (4 MiB kept in `.1`), caches of notes are capped.
- **Tests.** `🛡️resilience/🧪️tests` (unit, Python oracle with hashlib and jsonschema), `🔁️replica/🧪️tests/🛡️resilience.ts` (daemon), `🔁️replica/🧪️tests/💥️killpoints.ts` (aborts at every filesystem mutation of publish, follow, close and journal append, reopens, checks convergence).

## Boot that cannot be bricked (supervised start, fallback code, admission, watchdog)

The 2026-11-13 08:41 incident (new files plus two stale ones, `tables.dirs.adopt is not a function`, a service manager restarting a crashing daemon forever) is the case this section closes. Nothing in the worktree can keep a machine from running working code, and no machine switches to code that nobody has proven.

- **The service entry is a supervisor.** The unit, plist and task still run `📜️script.ts run`; that file imports nothing but Node built-ins. For `run` it starts the daemon as a child process (`<runtime> 📜️script.ts --boot-child run …`) of the worktree code. Any exit other than 0 before the child reported `started` (a syntax error, an import that does not fit, a throw while opening the state, a crash, a hang), or within `earlyMs` (60 s) of its start, is an early failure and the next code is tried: (a) the newest intact sealed snapshot in `.cache/f/code/` that `history.json` does not list as `refused`; (b) a snapshot in `<store>/v1/bootstrap/code/<digest>/` (see below), preferring one that a fresh attestation reports, fetched file by file against its `sealed.json`, hash-verified and sealed locally; (c) the worktree code that was set aside as a revert, as a last resort. A snapshot that fails early is added to `refused`. A daemon that was ready and fails later is not replaced: its exit code is final and the service manager restarts the whole sequence. Exit code 75 (another daemon holds the lock, the origin belongs to another machine) is final as well and never starts a fallback, and a daemon that waits for the consent to read the worktree (macOS) is not killed for being slow to start; a daemon that is neither ready nor waiting after `readyMs` (15 min) is killed and counts as an early failure. Failing everything, the stage waits 5 s and exits 1 (`No replica code starts`).
- **A machine on a fallback keeps working and says so.** The child gets `SEMIO_FLEET_CODE_WATCH` (the worktree module), so the daemon keeps syncing and watches the worktree code; it reports `fallback:{guard:"code-fallback",source,reason}` in its attestation (fleet warning `code-fallback`), `status` prints the source and phase of the code, and `.cache/f/boot/beat.json` carries `source`. When the code on disk passes the admission and the self-check the daemon restarts (exit 0) and the next start runs the worktree code again. When the worktree code is byte-identical to the fallback code (the fix was made on disk without a new digest) the daemon still returns, but not before 5 minutes of uptime (`returnMilliseconds`), so a worktree that can start only in theory does not cause a restart loop.
- **Store snapshots.** A healthy daemon (running for `publishCodeMilliseconds`, 10 min, with no failure younger than that and the store reachable) copies its sealed snapshot to `<store>/v1/bootstrap/code/<digest>/{sealed.json,files/…}` through a private `.tmp-` directory and a rename, so a snapshot is complete or absent; `sealed.json` is the schema `SealedCode` of `🛡️resilience` (per-file `sha256` and `bytes`, sorted bytewise), the same document as a local seal. A damaged published snapshot is replaced. The newest four stay, plus every digest that a live origin attests; unfinished directories older than an hour go. A hand-made example without a manifest does not load and is replaced by the first real publish.
- **The stage outlives the worktree.** `join` and `seed` copy `📜️script.ts` to `.cache/f/boot/stage.ts` with `stage.json` (its sha256, written last); a healthy daemon refreshes that copy from the code it runs, so a machine that joined before the stage existed gets it from the next healthy daemon. The worktree entry runs the copy when its hash matches the signature and the copy imports, otherwise itself. A service definition cannot be repaired from inside the worktree: a worktree `📜️script.ts` with a syntax error is the one failure the supervisor cannot catch on a machine whose definition still points at it; a definition that points at `.cache/f/boot/stage.ts` does not have that limit.
- **A new code is admitted before it is checked.** A daemon switches only to a digest that another origin attests (any origin's attestation carries it), that a healthy run left in the store, or whose changed files (compared with the sealed copy of the running code) were all published by this origin. Anything else waits, shown as `pending` with `waiting: …`, asked again every heartbeat; the digest changes with every file that arrives, so a partial change set is never switched to and the decision is made again for each state. The self-check now also dry-opens the real state on a private mirror with `openReplicaV1` (not only the load of the checkpoint) and boots the checked daemon once in a scratch worktree and store (seed, open, close, then `runV1` until its first attestation), so a mixed module fails before any restart; its limit is 180 s.
- **A blocked event loop is visible and bounded.** The child starts a watchdog thread that writes `.cache/f/boot/beat.json` every second with the lag of the event loop; `status` prints `Daemon: busy, the event loop has not answered for N s` instead of `not running` (the lock is not taken over while the beat of its pid is fresh), a lag of 60 s is recorded as the failure `loop-blocked` in `health.json` and in the attestation, the supervisor kills a child blocked for 15 min, forwards `SIGTERM` and kills a child that does not stop within 20 s, and the child ends itself when its supervisor dies. Whole-queue loops (clash settling, event parsing) yield every 8 ms (`yieldingV1`).
- **Limits** are environment variables of the entry: `SEMIO_FLEET_BOOT_EARLY_MS`, `_READY_MS`, `_BLOCK_MS`, `_KILL_MS`, `_GRACE_MS`, `_BEAT_MS`, `_STORE_MS`.
- **Tests.** `🔁️replica/🧪️tests/🛟️boot.ts` (stage, store fetch, watchdog, admission, status, publication, the incident with a real module copy), `🛡️resilience/🧪️tests` (store snapshots with the Python oracle, beat, yield).
