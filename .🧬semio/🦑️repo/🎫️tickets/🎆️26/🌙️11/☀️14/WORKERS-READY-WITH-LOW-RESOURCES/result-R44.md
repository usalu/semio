# R44: provisioning scripts read JSON by walking it, not by text patterns (PREPARE ONLY, nothing landed)

Prepared 2026-11-15 16:10. No live file was edited. Everything to land is in `T/🧰️scripts/R44/` (`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `land.py`, `base/` (the live versions the overlay started from) and `overlay/` (the new versions). Scratch rehearsal root: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R44/root`.

## 1. Audit: every text-pattern read of JSON on the topology, provision and kit path

Module `🤝️shared-worktree/🗺️topology/🥾️provision` (`B` = base line numbers of `🐚️.sh` before the change).

| # | Where | What it reads | Pattern | What breaks it |
|---|---|---|---|---|
| 1 | `🐚️.sh` B79-82 `json_object`/`json_string` on `🔣️.json` | `worktree.<platform>` | `sed` on the file with newlines removed; `.*"key"` is greedy, so the LAST `"key"` anywhere in the file wins; `{[^}]*}` ends at the first `}` | any later key of the same name (`macos`, `worktree` inside a new section) hijacks the answer; a nested object or a `}` inside a string cuts the object; an escaped quote cuts a value; this is the generic cause of all rows below |
| 2 | B88 `GROUP_NFS` | `groups[g].nfs.host/export` | one regex `{ "id":"G", "nfs":{...}` | key order: anything between `id` and `nfs` (the incident: `ports`, `access`), `nfs` before `id`; a nested object inside `nfs`; a decoy `{"id":"g1","nfs":...}` inside any new key |
| 3 | B97 `GROUP_SERVER` | `groups[g].server.platform/protocol` | one regex `id, nfs, server` adjacent and in that order | the incident (`the topology has no server for group g1`); `nfs` with a nested object; `server` with a nested object before `platform`/`protocol` |
| 4 | B98-99 | `server.platform`, `server.protocol` | `json_string` on the captured `server` text | keys after a nested object are lost; greedy last match |
| 5 | B107 `machine.json` | `originId`, `groupId` | `json_string` on the flattened text | a nested object with the same key names (a `previous` identity) wins over the top-level one, because the last match wins; whitespace is fine |
| 6 | B136-138 old worktree `machine.json` | same | same | same |
| 7 | B337-340 `manifest.json` | `archive`, `sha256`, `files` | three line-based `sed` | a pretty-printed file with a second `"sha256"`/`"files"` line prints two values and the comparison fails; `"files": 1.0` or a string; any nested `sha256` |
| 8 | B372 `attest.json` | `code` (the digest the group server attests) | `grep -o '"code":"<64 hex>"'` | any space after the colon (pretty print) finds nothing and the script silently falls back to "newest sealed snapshot", i.e. possibly the wrong code; a nested `code` key is taken |
| 9 | B380 `sealed.json` | `createdMs` | `grep -o '"createdMs":[0-9]*'` | pretty print: no match, snapshot silently ignored, "the store holds no sealed code" |
| 10 | B387-391 `overlay_stage` on `sealed.json` | `version`, `digest`, every `files[i]` (`path`, `bytes`, `sha256`, `executable`) | `grep '"version":1'`, `grep "\"digest\":\"..\""`, `grep -oE '\{"path":"[^"]*","bytes":N,"sha256":"..","executable":(true\|false)\}'` | key order of an entry, whitespace, any new field in an entry (a `mode`), a quote in a path, a nested `"version":1`. Fails closed ("not intact", the checkout keeps old code, exit 5 later) but then EVERY fresh machine refuses the fleet code after a producer change |
| 11 | `🔵️.ps1` lines 59, 85, 108, 193, 200, 220, 347 | topology, `machine.json`, `attest.json`, `sealed.json`, `manifest.json` | `Get-Content -Raw \| ConvertFrom-Json` (a real parser: key order, whitespace, nesting, unknown keys and strings are safe) | only: lines 59, 85, 108, 193, 347 read without `-Encoding`; Windows PowerShell 5.1 then decodes UTF-8 without BOM as ANSI and the emoji paths of the topology come out as mojibake (facts are ASCII, so no failure today); 5.1 also rejects duplicate keys that differ only by case |

Same class, outside this package (listed, not changed):
- `🔩️native/🥾️bootstrap/🐚️.sh` L165/169 `pin_row`/`pin_field` and `🚀️launcher/📥️install/🐚️.sh` L32/36: `grep -F '{"tool":"go","version":"'` plus `"platform":"x","arch":"y",` and `sed '"key":"value"'` on the pins file. They need one object per line, compact, in key order `tool, version, platform, arch`. A reformat of the pins file makes them fail loudly ("No pinned Go ..."), they never answer wrongly. The same awk walker can serve them (follow-up).
- `🐚️.sh` B213/B353 (`sed` on launchd plists) and `🔩️native/🥾️bootstrap/🐚️.sh` L912 (`global.json`): XML and a line-based `"version"`; not JSON, written by this repo; mentioned for completeness. B213 reads a plist that `launchctl`/`plutil` may reformat across lines (then the installed worktree is not detected and no relocation happens).
- `🔁️replica/🥾️entry/🐚️.sh` and `🔵️.ps1`: no JSON text patterns.

## 2. Choice: a JSON walker in awk (not a per-group `facts.env`)

Chosen: `json_flat` in `🐚️.sh`, region `# #region 🔎️Json`. It is a recursive-descent walk in POSIX awk (`LC_ALL=C`, bytes, linear) that prints one `path<TAB>value` line per scalar (`groups[0].nfs.host`), refuses malformed JSON, trailing text and duplicate keys (exit 1, reason on stderr), decodes `\" \\ \/` and ` ..~`, keeps other escapes as written, skips a BOM. `json_fact FLAT PATH`, `json_group FLAT ID` (the one group with that id, empty when none or ambiguous) and `json_seal_files SUMMARY` (entries of the seal, with a `listed/readable` count) read the flat text. Callers use paths, so key order, whitespace, nesting, unknown keys and `{ } " ,` inside strings cannot change an answer.

Why not the flat `facts.env` per group published by `📜️script.ts facts`:
- It cannot cover the other five inputs (rows 5-10): `machine.json`, `manifest.json`, `attest.json`, `sealed.json` are written at runtime in the store or the worktree, not by the kit publisher.
- It adds a second tracked artifact that can drift from the JSON (needs a generator, a verifier, a schema, a refusal when they disagree) and it must be sealed into the kit; the walker has no second source of truth.
- The first provisioning steps run before the launcher exists, so the script cannot call `📜️script.ts` to generate or verify anything at that point; it can only read files.
- A fresh macOS has awk, sed, tr, grep, cut, mktemp, shasum and tar; it has no `jq`, and `python3` is the developer-tools shim that opens an install dialog. The script already avoids both before the launcher exists (python3 appears only inside the launcher and native-bootstrap fallbacks). awk adds no dependency.
- The safety net stays: after the launcher exists the script compares worktree, NFS source and server platform/protocol with `fleet facts` (B668-671) and refuses any difference.

Changes in the overlay: `🐚️.sh` (rows 1-10, header text), `🔵️.ps1` (`-Encoding UTF8` on the five reads), provision tests (harness takes the Json region next to the Overlay region; one assertion on the old regex text replaced), topology tests (two stubs `json_object/json_string` become `json_fact`), Python oracle (`facts` command, stdlib `json`), new `🥾️provision/🧪️tests/🟦️extraction.ts`.

## 3. Law tests (new file `🥾️provision/🧪️tests/🟦️extraction.ts`, 8 tests)

- Real topology and fixture topology, 60 permutations each (120 total, seeded): shuffled key order at every object and group order, unknown keys at every level (including decoys that carry the same names: a whole look-alike group, `worktree`, `host`, `originId`, `sha256`, braces and quotes in strings), compact/pretty/random whitespace (tabs, CRLF), `\/` and `\u00XX` escapes, BOM, trailing blank lines. For every group and platform the script's own extraction lines (sliced from `🐚️.sh`) must equal the facts of `readTopologyV1`/`groupV1` (TypeScript), the Python `json` module (`🐍️.py facts`) and, where installed, `jq` (present here, ran).
- The incident orders by name (`id, ports, access, nfs, server`; server before nfs; reversed; an extra object before `id`). The base `sed` returns an empty `nfs` for both incident orders (measured).
- Refusals: duplicate key, unterminated string, incomplete, text after the document, unbalanced, empty, key without quotes, stray comma; unknown or duplicated group id gives empty, never another group.
- `machine.json`, `manifest.json`, `attest.json`, `sealed.json` (3-42 file entries, emoji and spaces in paths) permuted and decorated with same-name decoys, 60 rounds: identity, archive name, hash, count, digest, `createdMs` and the full entry list equal the generated values; entries with a missing field, short or upper-case hash, negative size or non-boolean bit are counted listed-but-unreadable ("2 1").
- PowerShell: static assertions (no `Get-Content -Raw` without UTF-8 before `ConvertFrom-Json`) and, only when `pwsh` exists, the real extraction lines of `🔵️.ps1` run for 12 permuted topologies and compared with TypeScript. `pwsh` is not installed on this Mac: that part did not run.
- Last test: no remaining JSON text pattern (`json_string`, `json_object`, `sed "key"`, `grep "key"`, entry regex) in the non-comment code of `🐚️.sh`.

Measured in the rehearsal mirror (macOS 27, BWK awk 20200816, load 28-34): extraction suite 8 pass, 0 fail, 70 s (1720 expects at 120 permutations earlier); existing provision suite 22 pass, 1 skip, 0 fail (its overlay tests drive the real shell overlay: seal, `--plan`, tamper, bits, exit code 5); existing topology suite 70 pass, 1 fail that exists only because the mirror stubs `ajv` (see below). `json_flat` + seal entries on a real 16 KB `sealed.json` (67 files): 20 ms.

## 4. Landing (`T/🧰️scripts/R44/land.py`)

- `python3 land.py --dry-run` (read-only, no lock): against the live tree at 16:08 all five existing files are `fast-forward` (live sha equals base), the new test file is `new`, every file passes its syntax check (`bash -n`, `ast`, `bun build --no-bundle`).
- `python3 land.py --land [--label NAME] [--only SUBSTR]... [--skip-tests] [--keep-on-fail]`: takes `1113-landing.lock` (stale after 30 min only with `--take-stale`), re-reads, writes each file by rename (a changed live file is merged with `git merge-file`, base = `base/`; a conflict aborts), keeps the originals and the landed copies in `T/🗑️generated/R44-landing-<stamp>`, runs EVERY `.ts` test file under every `🧪️tests` directory of the module with `SEMIO_*` removed (protocol rule 3), appends the record (before/after sha256, tests, restarts, rollback pointer) to `1113-landings.md`, unlocks; any failure restores the originals.
- `python3 land.py --rollback [STAMP]`: restores; a file that changed after the landing is merged backwards, never overwritten.
- Rehearsed in the scratch root: fast-forward land with tests (22+8 green), land onto a live file that another agent had edited (three-way merge kept both), rollback in both cases (shas equal the originals, the new file removed, lock released), and an induced test failure (`--bun /usr/bin/false`) that restored everything automatically.

Who restarts what when it lands: nobody. Measured: `codeDigestV1` of the module is `3af3ac45…` before and after (the provisioning scripts are sealed companions and the tests are outside the import closure), so no replica daemon, relay, cache, scheduler or agent restarts on any machine and the 1113 rollout order does not apply. The files reach every machine through the replica like any file. The kit changes: the group server's kit maintenance re-seals the two changed companions under the same digest and republishes `v1/bootstrap/kit` (SHA256SUMS and `published`); `bootstrap-archive` picks them up on its next scheduled refresh. Machines provisioned after that use the new scripts; running machines are untouched.

## 5. Not verified / the coordinator must know

- Only BWK awk (macOS) ran. gawk, mawk and busybox awk (Linux workers, containers) did not; the program uses only POSIX features (`substr index length sprintf("%c") tolower ENVIRON`, `> "/dev/stderr"`). Run the extraction suite once on a Linux worker before relying on it there.
- `pwsh` is not installed here: the PowerShell extraction law and the mock layer `🧪️tests/🔵️.ps1` did not run; the `-Encoding UTF8` edit is syntactically trivial but unexecuted.
- No provisioning run, no real group, no store: the overlay path ran only through the existing test harness against synthetic stores; the real `fetch_archive` mount path did not run.
- `/Users/ueli/Documents/semio/node_modules` is a dangling link into the old `/Users/Shared/semio/.cache/b/...` (the native cache image is not mounted), so no suite that imports `ajv` can run from the live tree right now, which includes the provision and topology suites that `--land` runs by default. I rehearsed with a stub `ajv` in the mirror; the kit-receipt schema assertions and the Ajv half of `the fixture topology is accepted ...` therefore did not run for real. Restore `node_modules` before `--land`, or land with `--skip-tests` and run the suites by hand.
- `\u` escapes above U+007E in a fact stay as written (no UTF-8 encoder in awk); the later `fleet facts` comparison would refuse such a topology loudly. Control-character escapes (`\n`, `\t`) also stay as written.
- Rows outside the package (pins file, plists, `global.json`) are not changed.
