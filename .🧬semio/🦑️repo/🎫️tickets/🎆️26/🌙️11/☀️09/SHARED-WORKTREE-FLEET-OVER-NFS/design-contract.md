# Shared Worktree Fleet over NFS: Design Contract

Binding contract for every agent implementing this ticket. Code lives in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/` (directories already exist: never type emoji
names by hand, copy them from `ls`, every emoji carries U+FE0F).

## Goal

One logical worktree, edited live by 50+ machines (macOS, Windows, Linux) in about 5 groups of 10. No branches,
no merges, no merge queue. Shared files are exactly the files not ignored by the root `.gitignore`. Ignored
files never leave the machine. Every group has one NFS server and one cache server.

## Decision: local replica + journal on NFS (not a direct NFS mount of the worktree)

Mounting the worktree itself over NFS is rejected:

- Ignored paths (`.cache`, `node_modules`, `.nx`, `**/dist`, `**/🤖️generated`, ...) are scattered through the
  tree and would land on the share. There is no cross-OS union file system to keep them local.
- Nearly every path contains emoji with U+FE0F / ZWJ. The Windows NFS client cannot be trusted with them.
- NFS delivers no FSEvents / inotify / RDCW for remote edits, and SQLite, flock and atomic exchange (which the
  caches need) are unsafe on it. The repo's own mount detector refuses NFS for exactly this reason.
- Builds on a network file system are slow.

So every machine keeps the worktree on its local NVMe at a canonical per-OS path, and a replica daemon keeps
the shared files identical on all machines through a **store** on the group's NFS export. The store uses only
ASCII names and single-writer directories, so it needs no NFS locking and works with every NFS client.

Semantics for agents: identical to many agents on one machine. A save appears on all other machines within
about a second. Two writers of the same file: one version wins on every machine, deterministically, and the losing
local bytes are kept locally under `.cache/f/lost/`. There is nothing to merge and nothing blocks. "Later" means
later by publish time (see Conflicts): two writes within the publish latency (about half a second, up to poll plus
debounce) on different machines are ordered by when each machine published, not by when each key was pressed.

## Store layout (on the group NFS export, mounted at `storeRoot`)

```
<storeRoot>/v1/
  origins/<originId>/segments/<12-digit segment number>.jsonl   append-only event lines
  origins/<originId>/head.json                                  {"version":1,"segment":n,"bytes":n,"events":n,"hlc":"..."}
  blobs/<first 2 hex>/<sha256 hex>                              file content, immutable
  tmp/<originId>-<random>                                       staging for blobs and head.json
  snapshots/<originId>/<hlc>.json                               full manifest for fast join
```

- `originId`: `^[a-z0-9][a-z0-9-]{0,39}$`, unique per machine. In its home group's store **only that machine**
  writes below `origins/<originId>/`. In every other group's store only that group's relay writes it.
- Write order is always: blob (tmp + rename, skip if it exists), then append event line(s) + fsync, then
  `head.json` (tmp + rename). Readers trust only the first `head.bytes` bytes of segment `head.segment` and
  all of the earlier segments. A reader therefore never sees an event whose blob is missing.
- A segment is sealed (next number started) once it exceeds 4 MiB or 4096 events.

## Event (one JSON line, schema `📒️journal/🧬️schema/🔣️.json`)

```
{"version":1,"hlc":"<13-digit unix ms>-<6-digit counter>-<originId>","operation":"put"|"delete"|"link",
 "path":"<relative, forward slashes, NFC>","sha256":"<hex>","bytes":n,"executable":bool,"target":"<link only>",
 "base":"<sha256 the writer replaced>"|null}
```

- `put` carries `sha256`, `bytes`, `executable`. `delete` carries none of them. `link` carries `target`
  (relative, must stay inside the worktree; absolute or escaping links are never shared).
- Directories are implicit: created on demand, removed when empty after a delete.
- Total order: compare `(ms, counter, originId)`; the HLC string compares correctly as a plain string.
  Hybrid logical clock: `send = max(wall, last.ms)`, counter +1 when ms did not advance; on receive
  `last = max(last, received)`. A received hlc more than 60 s ahead of the wall clock is refused and reported.
- State per path = the event with the greatest hlc (a `delete` is a tombstone and also wins by hlc).
  This is plain event sourcing with one deterministic total order; no CRDT, no merge.

## Conflicts: unpublished local edit against a remote event

The only moment two versions of one path meet on a machine is the follow step above. A path is **dirty** when the
local file differs from the recorded `state.json` entry (size or hash; a missing file, a file the state does not
know, or a different link target count) and also from the incoming content. Then:

- `conflictWinnerV1(localMtimeMs, remoteHlc)`: the local edit wins if and only if its modification time
  (`lstat` mtime) is strictly greater than the wall-clock milliseconds inside the remote hlc. A local **delete** has
  no modification time and always loses to a remote put (the file comes back, nothing is published, nothing goes
  to `lost/`). The remote event may itself be a delete; the same rule applies to a dirty local file.
- Local wins: the file is left untouched, `state` takes the remote entry with `mtimeMs` 0 (so the next observation
  rehashes the file), the event `kept` is reported and the path is published immediately with `base` = the sha256 of
  the remote version and a **new hlc**. That hlc is greater than every hlc this machine has seen (the clock was
  merged with the remote one on receipt), so it wins on every machine.
- Remote wins: the local bytes are copied to `.cache/f/lost/<hlc>/<path>`, `overwritten` is reported and the remote
  event is applied. This is the behaviour of all earlier rounds.

Deterministic: the verdict is a pure function of two numbers (checked by fixture vectors in TypeScript and Python
`🐍️.py conflicts`), and whatever the verdict, all machines converge by the plain hlc order, because the kept edit is
just a new event. Loop-free: a verdict is only taken for a dirty path; publishing or applying makes the path clean
at once, and a further verdict needs a further local edit whose mtime is later than the wall time of the newest event
it meets; the machine that lost has no dirty path left to re-decide.

Honest limits. The wall time inside an hlc is the time of **publication**, not of the edit. Writes to one file on
different machines inside the publish latency are therefore ordered by publish time: whoever publishes first
gets the lower hlc and loses to the other, even if it was typed later (the rehearsal showed A, B, B winners for
writes 5 to 8 ms apart). An edit made offline and published after reconnecting is ordered by its publish time too; if
both machines were offline, the first one to publish wins and the other goes to `lost/`, unless the other's
modification time is later than that publish time (then it is published afterwards and wins). The comparison puts one
machine's file clock against another machine's hlc clock, so it is as good as NTP (the 60 s skew bound applies).

## Boundary (what is shared)

- Shared iff: not ignored by the root `.gitignore` (full gitignore semantics: anchoring, `**`, negation,
  directory-only rules, a rule on a parent directory hides everything below and cannot be re-included, character
  classes, escapes), and not below `.git/`, and passes the path policy.
- Only the root `.gitignore` is read (the repo has no nested ones). It is itself shared; the matcher reloads
  when its content hash changes and the daemon then rescans.
- Path policy (refuse + report, never share): not NFC; longer than 256 characters; a component longer than
  255 UTF-8 bytes; Windows-illegal characters `<>:"|?*` or control characters; trailing dot or space; reserved
  device names (CON, PRN, AUX, NUL, COM1-9, LPT1-9, any extension); case-fold collision with an existing shared
  sibling; absolute or escaping symlink.
- Machine-local state of this subsystem lives in `.cache/f/` (ignored by rule `.cache`):
  `machine.json` (originId, groupId, storeRoot), `state.json` (path -> sha256, hlc, bytes, mtimeMs, executable;
  one JSON object that is written and read as a stream, never as one string),
  `cursor.json` (origin -> segment, bytes), `clock.json`, `lost/<hlc>/<path>`, `tmp/`, `daemon.lock`.

## Memory budget

Budget: an idle daemon at most 300 MB resident, `seed`, `join` and `verify` at most 1 GB at their peak (target
machines have 8 to 16 GB and run compilers and agents next to the daemon). Measured on the real tree (269,331 shared
files, 122k directories, Bun 1.4.2, macOS, `/usr/bin/time -l` and `ps`): idle 238 MiB (234 to 287 MiB over six runs),
peaks 402 to 490 MiB; before 1,261 MiB and 1.7 to 2.7 GiB. How the replica stays inside it, as rules for the code:

- One id per path. A `ReplicaBookV1` holds the UTF-8 of every path once, in 1 MiB pages behind a hash table of ids; the
  tables of local state, of store winners and of the incoming queue are typed columns by id (hlc as milliseconds,
  counter and origin index, sha256 as 32 raw bytes, sizes and times as numbers). No path-keyed `Map` of objects,
  no lower-cased copy of the paths (case collisions go through a hash of the folded name that the book stores).
- Everything that can be as big as the tree is streamed, never held as a string or an array: `state.json` and
  snapshots are written entry by entry and read through `streamMembersV1`; the journal is folded one segment at a time;
  `scanSharedV1(…, visit)` hands entries over without keeping them (depth first, so the pending directories stay few);
  a walk keeps only the entries that differ from the state, as columns; `verify` compares each file as soon as it is
  hashed.
- The ignore matcher caches at most 20,000 names and directories and decides children without walking their
  ancestors (`child`).
- Garbage is collected on purpose every few thousand entries of a scan, a load or a publish, because the runtime
  gives freed pages back only slowly.

## Replica daemon (`🔁️replica`)

Loops, all cancellable, all reporting progress as NDJSON on stdout:

1. **Observe local**: one recursive native watch where available (no per-directory watcher), plus a full stat scan (mtime + size, rehash on
   change) every `scanSeconds` (default 900: a full scan of the 270k-file tree costs 16 CPU seconds, measured in the rehearsal) as the safety net. Debounce 150 ms. Unstable files (size or mtime
   changed during hashing) are retried, never published half-written.
2. **Publish**: for each shared path whose content differs from `state.json`, write blob, append event with
   `base` = previous state sha256, update head, update state.
3. **Follow**: every `pollMilliseconds` (default 250) read every other origin's `head.json`; read new bytes;
   for each event that wins against local state, first look at the path: if the local file differs from both
   `state` and the incoming content (an unpublished local edit; a local delete counts too), decide by
   Conflicts below. The remote event is then either applied (the loser is copied to
   `.cache/f/lost/<hlc>/<path>` and reported `overwritten`) or the local edit is kept and published at once
   (reported `kept`). Applying materialises via `.cache/f/tmp/<random>` + rename (retry with backoff on Windows
   sharing violations), sets the executable bit where the OS has one and updates state so the observer does not
   echo it back.
4. **Degrade**: if the store is unreachable keep working locally, queue publishes, resume on return. Never
   block or freeze local editing.
   Local obstacles degrade the same way and never lose an event: a file held open by another process (Windows
   `EBUSY`/`EPERM`/`EACCES` on rename, unlink, write or read) leaves the incoming event in the persisted queue (the
   cursor may pass it), is reported `blocked`, retried with growing back-off and applied after the release; a local
   file that cannot be read is retried by the next publish; a journal write blocked after its retries publishes
   nothing and keeps the paths for the next round. A symlink that the account may not create (no privilege) is
   reported `skipped` (`symlink-unsupported`) and dropped from the queue: no retry, nothing published for it, all other
   files applied; `verify` lists it as `missing-local`. A path that the policy refuses is reported `refused` (once per
   run), counted by `status --deep` and listed by `verify` (not drift); a path inside the policy that the operating
   system refuses as too long is `refused`/`too-long` in a scan and `skipped`/`too-long` when applied.

Commands (`🔁️replica/📜️script.ts`, run as `repo ./📜️script.ts <command>` like every other repo script):
`seed` (first machine: publish the whole shared tree), `join --group <id> --origin <id>` (materialise from the
newest snapshot + journal), `run` (the daemon), `status`, `snapshot`, `relay` (see below), `compact`
(write a snapshot, drop sealed segments and unreferenced blobs older than the retention), `verify` (rehash and
compare local tree, state and store; exit non-zero on drift).

## Relay between groups

Each group server runs `relay`. It mounts the other groups' exports read-only (`peers` in the topology) and,
for every origin whose **home group** is that peer, copies missing blobs, then missing segment bytes, then
`head.json` into its own store. An origin is only ever pulled from its home store, so there are no loops and
every `origins/<originId>/` directory keeps a single writer per store. Copies are idempotent.

## Topology (`🗺️topology/🔣️.json`, schema beside it, shared like any source file)

```
{"version":1,
 "worktree":{"linux":"/srv/semio","macos":"/Users/Shared/semio","windows":"C:\\semio"},
 "storeMount":{"linux":"/mnt/semio-store","macos":"/Volumes/semio-store","windows":"S:\\"},
 "groups":[{"id":"g1","nfs":{"host":"nfs.g1.semio.internal","export":"/srv/semio-store"},
            "cache":{"endpoint":"http://cache.g1.semio.internal:6090","instance":"semio"},
            "peers":{"g2":"/mnt/semio-peers/g2", ...}}, ... g5],
 "replica":{"pollMilliseconds":250,"scanSeconds":900,"debounceMilliseconds":150,"retentionDays":14,"bootstrapMinutes":60}}
```

Host names are placeholders the operator replaces. `🗺️topology/🟦️.ts` validates it, resolves a machine's
group, and renders the environment for a machine: `SEMIO_REMOTE_CACHE_ENDPOINT` = its group's cache endpoint,
`SEMIO_REMOTE_CACHE_ACCESS=read-write`, `SEMIO_REMOTE_CACHE_INSTANCE`. The canonical worktree path is fixed per
OS because cache hits across machines require identical checkout paths.

## Git

`.git/` is never shared. Workers do not need git at all. History is optional and taken by one designated
archivist machine that commits the replica on a timer; nobody else runs mutating git commands (already the
rule in `AGENTS.md`).

## Module interfaces (so the parts can be built in parallel)

`📒️journal/🟦️.ts` (pure logic + store I/O, node built-ins only):
```ts
export type FleetEventV1=Readonly<{version:1;hlc:string;operation:"put"|"delete"|"link";path:string;sha256?:string;bytes?:number;executable?:boolean;target?:string;base:string|null}>;
export type FleetHeadV1=Readonly<{version:1;segment:number;bytes:number;events:number;hlc:string}>;
export type FleetCursorV1=Readonly<{segment:number;bytes:number}>;
export type FleetClockV1=Readonly<{ms:number;counter:number}>;
export function tickClockV1(last:FleetClockV1,wallMs:number):FleetClockV1;
export function mergeClockV1(last:FleetClockV1,receivedHlc:string,wallMs:number):FleetClockV1; // throws when >60 s ahead
export function formatHlcV1(clock:FleetClockV1,originId:string):string;
export function parseHlcV1(hlc:string):Readonly<{ms:number;counter:number;originId:string}>;
export function winsV1(candidateHlc:string,currentHlc:string|undefined):boolean;
export function requireEventV1(value:unknown):FleetEventV1;
export function requireOriginIdV1(value:string):string;
export function storePathsV1(storeRoot:string):Readonly<{root:string;origins:string;blobs:string;tmp:string;snapshots:string}>;
export function blobPathV1(storeRoot:string,sha256:string):string;
export async function writeBlobV1(storeRoot:string,originId:string,sha256:string,source:string):Promise<boolean>; // false = already there
export async function appendEventsV1(storeRoot:string,originId:string,events:readonly FleetEventV1[]):Promise<FleetHeadV1>;
export async function readHeadV1(storeRoot:string,originId:string):Promise<FleetHeadV1|undefined>;
export async function listOriginsV1(storeRoot:string):Promise<readonly string[]>;
export async function readEventsV1(storeRoot:string,originId:string,after:FleetCursorV1|undefined):Promise<Readonly<{events:readonly FleetEventV1[];cursor:FleetCursorV1}>>;
export type FleetManifestEntryV1=Readonly<{hlc:string;operation:"put"|"delete"|"link";sha256?:string;bytes?:number;executable?:boolean;target?:string}>;
export type FleetManifestTargetV1=Readonly<{get(path:string):FleetManifestEntryV1|undefined;set(path:string,entry:FleetManifestEntryV1):unknown}>; // a Map or a compact table
export function foldEventsV1(manifest:FleetManifestTargetV1,events:readonly FleetEventV1[]):readonly FleetEventV1[]; // returns the events that won
export async function writeSnapshotV1(storeRoot:string,originId:string,hlc:string,manifest:Readonly<{size:number;keys():Iterable<string>;get(path:string):FleetManifestEntryV1|undefined}>,cursors:Readonly<Record<string,FleetCursorV1>>):Promise<string>; // same JSON document, streamed in sorted order
export async function readNewestSnapshotV1<M extends FleetManifestTargetV1>(storeRoot:string,onlyOrigin?:string,create?:()=>M):Promise<Readonly<{hlc:string;manifest:M;cursors:Record<string,FleetCursorV1>}>|undefined>; // entries stream into create()
export async function readEventSpansV1(storeRoot:string,originId:string,after:FleetCursorV1|undefined,segments?:number):Promise<…>; // segments: read at most that many segments
export async function streamMembersV1(file:string,streamed:ReadonlySet<string>,visit:(outer:string,inner:string|undefined,text:string)=>void,opened?:(outer:string)=>void,chunk?:number):Promise<void>;
```

`🚧️boundary/🟦️.ts` (pure logic + scanner):
```ts
export type FleetIgnoreV1=Readonly<{digest:string;ignores(path:string,isDirectory:boolean):boolean;child?(path:string,isDirectory:boolean):boolean}>;
export function compileIgnoreV1(gitignoreText:string):FleetIgnoreV1;
export type FleetPathRefusalV1="not-nfc"|"too-long"|"component-too-long"|"illegal-character"|"trailing-dot-or-space"|"reserved-name"|"case-collision"|"unsafe-link";
export function refusePathV1(path:string):FleetPathRefusalV1|undefined; // collision is decided by the scanner
export function linkRefusalV1(path:string,target:string):FleetPathRefusalV1|undefined;
export type FleetScanEntryV1=Readonly<{path:string;kind:"file"|"link";bytes:number;mtimeMs:number;executable:boolean;target?:string}>;
export type FleetScanV1=Readonly<{entries:readonly FleetScanEntryV1[];refused:readonly Readonly<{path:string;reason:FleetPathRefusalV1}>[]}>;
export async function scanSharedV1(worktree:string,ignore:FleetIgnoreV1,signal?:AbortSignal,visit?:(entry:FleetScanEntryV1)=>void):Promise<FleetScanV1>; // never descends into ignored directories or .git; with visit the entries are handed over one by one and `entries` stays empty
export function sharedV1(path:string,isDirectory:boolean,ignore:FleetIgnoreV1):boolean; // ignore + .git + refusePathV1
export async function hashFileV1(file:string):Promise<Readonly<{sha256:string;bytes:number;stable:boolean}>>;
```

`🗺️topology/🟦️.ts`:
```ts
export type FleetPlatformV1="linux"|"macos"|"windows";
export type FleetTopologyV1=...; // mirrors the schema
export function readTopologyV1(value?:unknown):FleetTopologyV1; // default = the committed 🔣️.json, validated
export function groupV1(topology:FleetTopologyV1,groupId:string):FleetTopologyV1["groups"][number];
export function platformV1(nodePlatform?:string):FleetPlatformV1;
export function machineEnvironmentV1(topology:FleetTopologyV1,groupId:string):Readonly<Record<string,string>>;
```

`🔁️replica/🟦️.ts` imports the three above and exports the loops as testable functions that take a `worktree`
and a `storeRoot` directory (tests use two temp worktrees and one temp store on the local disk).

## Rules for implementers

- This machine has no bun, node, cargo, python or git. Nothing can be executed here. Write for Bun >= 1.2 using
  only `node:` built-ins and existing repo modules; reread your own code carefully; never claim a test passed.
- Follow the neighbouring modules' style (`📚️library/🏗️build/🌐️remote-cache/`, `📚️library/🏗️build/🚢️fleet-status/`):
  schema-first JSON schema in `🧬️schema/🔣️.json`, `V1` suffixes, emoji docstring on every export, no comments
  inside definitions, tests in `🧪️tests/🟦️.ts` with `bun:test` driven by `🧪️tests/🧫️fixtures/🔣️.json`, plus an
  independent Python oracle `🧪️tests/🐍️.py` that reads the same fixture (language-agnostic test).
- No external runtime dependencies. No migrations, no compatibility layers. Paths below 256 characters.
- Stay inside your own directory. Do not edit `AGENTS.md`. Do not run git. Other agents edit this repo at the
  same time; ignore unrelated changes.
