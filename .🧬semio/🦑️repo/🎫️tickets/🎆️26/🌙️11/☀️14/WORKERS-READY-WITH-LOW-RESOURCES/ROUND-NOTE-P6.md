# Round note P6 (fourth burst: no worker waits for a click while any route works)

**Service start order on a Mac with the checkout below Documents/Desktop/Downloads:** (a) the launchd job under the launcher when the launcher holds the consent; (b) when the launcher has no consent yet **but the calling process can read the checkout itself**, the service is started as a direct child under the caller's access (the route of rounds 2 to 5), exit status 0, and the command prints ONE line instead of exit 4: "The graph service runs under the access of the calling app because repo has no consent for Documents yet; click Allow on the dialog "repo would like to access files in your Documents folder" to make it independent / Der Graphdienst läuft mit dem Zugriff der aufrufenden App, weil repo noch keine Erlaubnis für Documents hat; ..." ; (c) only when neither can read: exit 4 with the consent text. The probe job for the launcher keeps waiting in the background; when the owner clicks, the next service start is (a) by itself. A service of route (b) is never replaced or killed by a later command; a later command from a caller without access gets (c). **So the bootstrap of a worker no longer ends with `native_rc=4` because of a missing click; record the one-line notice as "runs under the app's access", not as a failure.** The click is still the way to make the service independent of the app that started it.
**Replica agent heal:** every real `repo` command (the front door, macOS only, only where `~/Library/LaunchAgents/tech.semio.fleet.replica.plist` exists, at most once a minute) looks at the agent; a loaded agent that is not running gets `launchctl kickstart gui/<uid>/tech.semio.fleet.replica` and one line "The replica agent was loaded but not running (launchd had deferred it); started it now / ...". A machine whose agent is penalised and whose worktree is **stale since before this landing** cannot fetch this code (its sync is the thing that is down): it needs the one manual kickstart once; afterwards the boot stage (bootVersion 6) keeps it up and this check is the second line.
**Check:** the stderr of the first `repo` command of the round (route b line, or the heal line), `launchctl list | grep tech.semio.repo` (service job only when route a), exit statuses.

---

# Round note P6 (third burst: the consent now survives bootstrap runs)

**Cause found in round 7:** every bootstrap run replaced the launcher file (and swapped `~/.local/bin/repo` between the launcher link and the native command), and macOS ties the consent of an ad-hoc signed program to the file, so each run reset it, even though the build is byte-identical. **Now:** the launcher installer keeps a stamp of its sources beside the launcher and does nothing while it fits ("The repo launcher is current, nothing was replaced"); a rebuild to the same bytes replaces nothing; the bootstrap no longer re-links `~/.local/bin/repo` over the native command or over a correct link.
**What the owner clicks:** once per Mac, for the launcher, "Allow" on "repo would like to access files in your Documents folder". The first bootstrap after this landing installs the new launcher (it is signed with the identifier `tech.semio.repo` and lives in `launcher/bin/versions/<hash>/repo`, so it is a real change): the click is needed once more, then it holds across every later bootstrap run and every service start. It is needed again only when the launcher's own sources change (one commit in the last seven days), and that is announced: the client then ends with exit 4 and the text "The launcher repo changed: macOS asks once more ... A service that is running keeps working until it stops"; the new file is installed beside the old one and a running service keeps the old file.
**Worker checks after the bootstrap:** `ls -l U/.local/share/semio/launcher/bin/` shows `repo -> versions/<16 hex>/repo` and `repo.stamp`; the same `stat -f "%i %m" U/.local/share/semio/launcher/bin/versions/*/repo` before and after a second bootstrap run (inode and mtime unchanged); `ls -l U/.local/bin/repo` is a regular file (the native command) or a link, and its inode does not change between runs when the native build did not change; the bootstrap log says "The repo launcher is current".

---

# Round note P6 (second burst, replaces the first version below where they differ)

## Clicks: exactly ONE per Mac, for the launcher, at a moment you announce

The identity everywhere is the launcher `U/.local/share/semio/launcher/bin/repo`. A Mac that is already in the fleet needs **one** click on "Allow" of the dialog "repo would like to access files in your Documents folder" (for the launcher, even if the replica was consented earlier under `U/.local/bin/repo`). Nothing else asks: the replica keeps its old identity until the click, the shim switches to the launcher by itself afterwards (rollout of `bootVersion 6`, no provision run, no human step), the service uses the same launcher.
When it appears: within about a minute after the Mac's replica daemon runs the new code (the daemon starts the probe job `tech.semio.repo.probe.<hash>` at start and every hour), or earlier if a `repo` command starts a service first. **Announce it to the owner before the round**: the dialog needs a person at the logged-in session; it stays on screen until answered. After a launcher rebuild with other bytes it appears once more.
What each side does while the click is pending: the replica runs on as before; `repo preflight` fails the check `access` with "these macOS roles cannot read <worktree>: launcher" and the click text; a `repo` command that needs the service ends within seconds with **exit 4** and the consent text (EN/DE, naming the launcher path and System Settings > Privacy & Security > Files & Folders > Documents Folder), not after 60 s. After the click the probe writes the marker (`U/Library/Application Support/semio/access/<hash>/launcher.json`), the next `repo` command starts the service as a launchd job (`launchctl kickstart`, a pid within 6 s); if launchd does not run it, the old direct spawn follows and one line says so.

## What launchd shows now

`launchctl list | grep tech.semio.repo`: `tech.semio.repo.probe.<hash>` (only while a click is pending or until it was cleaned) and `tech.semio.repo.service.<hash>` per implementation. `launchctl print gui/<uid>/<label>`: `program` = the launcher, `runs >= 1` or a pid after the start, **no** TOKEN/EMAIL/SESSION/UUID variable in `environment` (allow-list: HOME, PATH, locale, TMPDIR, XDG_*, the runtime memory bound and SEMIO_* without secret-like names), property lists in `U/Library/Application Support/semio/service-jobs/` (stable, owner-only). `repo service host stop` unloads all of them.

## If a Mac's replica agent is penalised (`pended nondemand spawn = inefficient`)

Run once as the logged-in user: `launchctl kickstart -k gui/$(id -u)/tech.semio.fleet.replica`. After `bootVersion 6` the boot program restarts a killed or crashed daemon itself and no longer ends (only exit 0 and 75 end it), so launchd sees no exit and the penalty does not come back; watch `launchctl print gui/<uid>/tech.semio.fleet.replica | grep -E "runs|pended"`: `runs` must stop growing.

---

# Round note P6: what a macOS worker checks after the file access consent landing

Applies to every Mac worker whose checkout lies below `~/Documents`, `~/Desktop` or `~/Downloads` (g1-m02, g1-m03, g1-m06, g1-m07, g1-m08). Read-only checks; `<uid>` = `id -u`, `U` = the worker's home.

## New meaning of exit status 4

A `repo` command, the bootstrap (`native_rc`) or `repo service host start` that ends with **4** has not failed: macOS has not given the launcher the consent to read the checkout. Do not retry in a loop. Record `consent`, copy the text the command printed, and ask the owner to click (see "Consent text"). Exit 75 stays "fleet did not serve", exit 1 stays an unexpected failure. `request-failed e07192d7` / `access-denied` in a service log must no longer appear as `internal`: it is now `consent` (key `access-denied`) and the service ends itself afterwards.

## 1. How the service is started now (run after the first `repo run` of the round)

```
launchctl list | grep tech.semio.repo.service          # one job per implementation tag, pid or "-" when exited
launchctl print gui/<uid>/tech.semio.repo.service.<label> | grep -E "program|arguments|working directory|state|last exit"
```
Expected: `program = U/.local/share/semio/launcher/bin/repo` (the launcher, not a bun path, not `U/.local/bin/repo`), arguments `script.ts repo service host run`, working directory = the checkout, no `bun` as program. `Could not find service` means the launchd route was skipped or failed and the old direct spawn ran: report that line, the service log (`host-*.log` first lines) and `echo $SEMIO_SERVICE_LAUNCHD`.
Also: `ps -o pid,ppid,command -p <service pid>` (the wrapper's parent is 1) and `ls -l /usr/local/bin/repo` (macOS: link to `U/.local/share/semio/launcher/bin/repo` after the provision script ran again; before that it still names `U/.local/bin/repo`, which the native command replaced).

## 2. Identity of the replica daemon

```
grep bootVersion "U/Library/Application Support/semio/fleet/entry/entry.json"     # 5 after the rollout
ps -axo pid,ppid,command | grep -E "boot.sh|boot.mjs|supervisor" | grep -v grep   # chain: sh boot.sh -> launcher repo ... -> bun
```
Expected for a checkout below Documents: the child of `boot.sh` is `U/.local/share/semio/launcher/bin/repo` (it then replaces itself with bun, so the process listing shows bun; check `SEMIO_WORKSPACE_ROOT` in the shim log `entry/shim.txt` and that no line says the launcher was skipped). A chain `boot.sh -> bun` directly (as on g1-m08 before) means the old shim still runs.

## 3. What macOS recorded (the decisive line)

```
python3 .../FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/P6/tcc-attribution.py --last 30m
```
For every row of the service (accessing `bun ... host`), the column `subject` must be `U/.local/share/semio/launcher/bin/repo` and `answer` `2/2`. A row with another subject (the agent app, `U/.local/bin/repo`, a bun path) or `0/...` is the reason of a denial: paste the row. For a round that failed before, also paste the rows around the failure time.
Also `codesign -dvvv U/.local/share/semio/launcher/bin/repo U/.local/bin/repo 2>&1 | grep -E "Identifier|CDHash"`: if the launcher CDHash differs from the previous round, the consent is lost once (the launcher sources changed); say so.

## 4. Receipts and preflight

```
ls "U/Library/Application Support/semio/access/"*/ && cat "U/Library/Application Support/semio/access/"*/service.json
repo preflight        # check "access"
```
`service.json` / `client.json` with `state: ok` and `identity` = the launcher path, or no file for an unprotected checkout. `denied` / `waiting` and `repo preflight` printing `worktree-access-denied: these macOS roles cannot read <worktree>: <roles>` names the role that lacks the consent.

## Consent text (what the owner and the log show)

English: "macOS does not let `repo` read your Documents folder, so <checkout> cannot be read. Open System Settings, Privacy & Security, Files & Folders, switch on Documents Folder for `repo` (<launcher path>) (or give it Full Disk Access) or click "Allow" on the dialog "repo would like to access files in your Documents folder", then run the command again. The consent is asked once per machine; a checkout outside Documents, Desktop and Downloads needs none."
German: "macOS erlaubt `repo` nicht, deinen Ordner Documents zu lesen, daher kann <checkout> nicht gelesen werden. Öffne Systemeinstellungen, Datenschutz & Sicherheit, Dateien & Ordner, schalte den Ordner Documents für `repo` (<launcher path>) ein (oder gib ihm Voller Festplattenzugriff) oder klicke bei der Abfrage „repo möchte auf Dateien in deinem Ordner Documents zugreifen“ auf „Erlauben“ und führe den Befehl dann erneut aus. ..."
Waiting variant: "A macOS dialog waits for your answer: "repo would like to access files in your Documents folder" ...". The dialog appears in the session of the logged-in user; the owner must be present at the Mac.
Entry to switch on = the launcher path printed in the text (also the `subject` column of section 3).

## Report back per machine

consent text seen (yes/no, exit code), section 1 `program` line, section 2 chain, section 3 rows of the service, section 4 `repo preflight` access line.
