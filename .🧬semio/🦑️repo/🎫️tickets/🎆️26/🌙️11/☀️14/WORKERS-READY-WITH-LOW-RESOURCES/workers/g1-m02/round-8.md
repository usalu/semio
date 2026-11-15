# g1-m02 Round 8

round: 8

Result: `consent`. Two tries, both ended with `native_rc=4` before the graph step; no nx task ran (succeeded: none, failed: none, not run: all). The owner's click did not happen during 30 minutes after try 1 nor during 18 minutes after try 2; then round 9 was published and the runner ended. Not ready, marker absent.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk not measured in this round.

## Before the Run (2026-11-14T21:03:14Z)

- Replica agent: `launchctl print gui/501/tech.semio.fleet.replica` shows `state = running`, `runs = 20`; not down, so I started it 0 times in this round. `.cache/f/health.json` was 1 s old.
- Replica code digest (`.cache/f/health.json`): `590fdef8b75853d124d6f615cd2612d607146e27aaf2b94715a5d7b7606ad2f1`
- `bootVersion` (`~/Library/Application Support/semio/fleet/entry/entry.json`): 6
- `repo service host` processes: pid 2926 ppid 1 age 47:50 rss 92544 KiB;

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Largest process on the machine | Largest in the run's group |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-8.log` | 21:03:20 | 21:07:31 | 4 | 3193280 KiB | 431904 KiB |
| 2 | `~/native-bootstrap-round-8-try-2.log` | 21:38:43 | 21:42:11 | 4 | 3097024 KiB | 482560 KiB |

Pressure level 1 in all 16 samples; no hard stop. The consent ended each try 4 min 11 s and 3 min 28 s after its start (the time of the tool and dependency phases; the consent text came right after `Starting the repo graph service for this host.`).

The printed text, verbatim, the same in both tries:

```
[semio] workspace:setup…
Starting the repo graph service for this host.
A macOS dialog waits for your answer: "repo would like to access files in your Documents folder" (/Users/ueli/.local/share/semio/launcher/bin/repo). Click "Allow" in the session of the logged-in user (or switch on Documents Folder for `repo` under System Settings, Privacy & Security, Files & Folders), then run the command again. /Users/ueli/Documents/semio cannot be read before that.
end 2026-11-14T21:07:31Z
native_rc=4
```

The graph service log got no new line in either try.

## Consent Wait

`repo preflight --lang en` was run every 60 s: 29 times after try 1 (21:07:57Z to 21:37:43Z), 18 times after try 2 (21:42:18Z to 22:00:20Z). Its output never contained a line with the word `access`. Run by hand at 22:02Z it exits 0 and prints only two `[capacity]` lines and two `[script]` progress lines, no check list at all. So the wait could not see a click through preflight; the 30 minutes ran out and try 2 showed the consent was still missing.

## Launcher Identity

| When | `~/.local/share/semio/launcher/bin/repo` | Target inode | Target mtime (local) | Size | Identifier | CDHash |
| --- | --- | --- | --- | --- | --- | --- |
| Before try 1 | regular file, inode 3672778 | 3672778 | 2026-11-14T21:35:44 | 6561378 | `a.out` | `713fd0d142560c91e4fd5ed6c449b32909e667e7` |
| After try 1 | link, inode 3679487, to `versions/cf25f2b751f83336/repo` | 3679486 | 2026-11-14T22:03:22 | 6523360 | `tech.semio.repo` | `6de7547a8b8ff384bbfbfc45e962462063ff0357` |
| Before try 2 | same link | 3679486 | 2026-11-14T22:03:22 | 6523360 | `tech.semio.repo` | `6de7547a8b8ff384bbfbfc45e962462063ff0357` |
| After try 2 | same link | 3679486 | 2026-11-14T22:03:22 | 6523360 | `tech.semio.repo` | `6de7547a8b8ff384bbfbfc45e962462063ff0357` |

Plainly: try 1 changed the launcher (the announced one-time install of the signed one); try 2, the second bootstrap run, left inode, mtime and CDHash unchanged. `~/.local/bin/repo` stayed a regular file with inode 3672881, 18657778 bytes, mtime Nov 14 21:35 through both tries; it is not a link.

Launcher lines of the logs, verbatim. Try 1:
```
[semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…
[semio] Installed the repo launcher: /Users/ueli/.local/share/semio/launcher/bin/repo
[semio] The launcher changed: macOS asks once more for the file access consent of the launcher (a running service keeps its file until it stops).
[semio] Add to PATH: /Users/ueli/.local/share/semio/launcher/bin
[semio] Another repository frontend is on PATH: /Users/ueli/.local/bin/repo. Installed launcher: /Users/ueli/.local/share/semio/launcher/bin/versions/cf25f2b751f83336/repo
```
Try 2:
```
[semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…
[semio] The repo launcher is current, nothing was replaced: /Users/ueli/.local/share/semio/launcher/bin/repo
[semio] Add to PATH: /Users/ueli/.local/share/semio/launcher/bin
[semio] Another repository frontend is on PATH: /Users/ueli/.local/bin/repo. Installed launcher: /Users/ueli/.local/share/semio/launcher/bin/versions/cf25f2b751f83336/repo
```

## `[capacity]` and Graph Lines

Try 2, before the consent text: `[capacity] admitted run: 1 job(s), 2048 MB reserved (7338 MB available, 2048 MB floor)`. No graph line (`taken from the fleet`, `inferred here`, `waiting for the fleet`, `restored`, `closureRepaired`) in either try: the graph step was not reached.

## Files

`round-8.tail.txt` (last 200 lines of try 2), `round-8.samples.txt` (samples, launcher records, preflight notes), `run-round-8.py`.
