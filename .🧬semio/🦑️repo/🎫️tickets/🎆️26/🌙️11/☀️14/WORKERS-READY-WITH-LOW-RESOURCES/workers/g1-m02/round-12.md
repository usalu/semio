# g1-m02 Round 12

round: 12

Pause round: no bootstrap run. No try was running when round 12 was read (2026-11-15T01:29:16Z); the last try (round 11 try 11) had ended at 01:26:55Z with `native_rc=1`. Not ready; `.cache/f/native-bootstrap.done` does not exist. Everything below was read at 2026-11-15T01:29:45Z.

## 1. Graph Lines of the Last Try

Service log `$TMPDIR/semio-repo/501/host-ac6bfe8f337c.log`, the part after the failure of the try before (01:22:53Z), verbatim:

```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 23 s
graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
graph: refreshing 1 projects (generation 75)
graph: generation 75 published, 1 projects refreshed in 45 s (0 kept their edges)
graph: fact index of generation 75: 758657 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed
```

No line with `stale projects`, `taken from the fleet` or `too heavy` in the last try. The try then failed with `request-failed 21c18fad Error: ENOENT … lstat '…/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'` (stack in `round-11.md`).

Earlier in the same log, for the sequence of generations in round 11 (verbatim, cut where the line was longer than my copy):

```
graph: refreshing 659 projects (generation 62)
graph: 659 stale projects, 303 taken from the fleet, 10 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (abo…
graph: 610 stale projects, 256 taken from the fleet, 10 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (abo…
graph: generation 68 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
graph: refreshing 5 projects (generation 69)
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
graph: refreshing 5 projects (generation 75)
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 7 s
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 9 s
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 24 s
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 23 s
graph: refreshing 1 projects (generation 75)
```

So on this machine the stale count fell from 659 to 5 and then 1 during round 11 (from try 5 on); since then the graph read no longer fails on memory here but on the missing closure file.

## 2. Service Logs and Replica

```
-rw-r--r--  1 ueli  staff   2099239 Nov 15 02:29 host-ac6bfe8f337c.log
-rw-r--r--  1 ueli  staff    389379 Nov 15 00:18 host-9888631afea4.log
-rw-r--r--  1 ueli  staff    253209 Nov 14 22:19 host-6f286e53e68a.log
```

Times are local (UTC+1). Replica code digest (`.cache/f/health.json`): `eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc`.

## 3. Machine State

- `kern.memorystatus_vm_pressure_level`: 1
- Swap (`vm.swapusage`): total = 2048.00M used = 1119.88M free = 928.12M
- Free disk: 100 GiB of 228 GiB
- Five largest processes (resident KiB, pid, parent pid, age):

| RSS KiB | pid | ppid | age | process |
| --- | --- | --- | --- | --- |
| 2430384 | 42199 | 35208 | 00:19 | `git` (the `git diff … --numstat` child of the Claude desktop app's helper, started about twice a minute) |
| 1124384 | 77771 | 77770 | 02:20:28 | `bun`: the replica daemon (child of `fleet/entry/boot.mjs run`) |
| 726096 | 35202 | 35172 | 2 d 00:56 | Claude Helper (Renderer) |
| 464704 | 42392 | 35208 | 00:03 | `git` (same kind as the first) |
| 384896 | 35208 | 35172 | 2 d 00:56 | Claude Helper |

- `repo service host`: pid 80600, parent 1, age 02:13:04, resident 56656 KiB; still running, not stopped.
- `repo show projects` child: none running.
