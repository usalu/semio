# Package R23: the Windows machine (g1-m05) is served by the macOS graph server (2026-11-15, 06:50 to 07:45 CET)

T = `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, N = this folder, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, GS = `LIB/🪪️graph-state`, W = `GS/🙋️wanted`.

**LANDED** 07:42 under `T/landing.lock` (22 files, 20 edited and 2 new, entry in `T/landings.md`, originals in `T/🗑️generated/R23-landing-074229/`, `N/LAND-OK` written and removed again, lock released). Parse proof 22 of 22 (Bun transpiler for `.ts`, JSON parser, Python `ast`); `tsc --strict --noEmit` on the six changed non-test TypeScript files of the live tree is clean (not the repository's own tsconfig). All 22 test files of the landing list ran live: green apart from six reds that are red the same way without this package (below). No graph, build or cargo command was run, nothing was killed, no token was printed. The `repo` MCP server did not connect, so no ticket was opened.

## 1. Which facts of a project record differ between Windows and POSIX, and what became of each

Evidence: the 374 Windows records (`win32-x64`, checkout `C:\semio`) and the 2 870 other project records of the production cache (read-only, decoded outside the worktree, scripts in `T/🧰️scripts/R23/lab/`), 371 projects recorded on both sides. Before this package 252 of 256 comparable pairs differed (P14).

| Fact | Windows | POSIX | Class | What the portable record does |
|---|---|---|---|---|
| Root in the result text | `C:/semio/…` (commands are written with forward slashes: 6 172 of 6 172 tokens are `⟦semio-root-slash⟧/`) | `/Users/Shared/semio/…` | (a) | one root token followed by `/`; the taker spells the root with forward slashes in texts on every host (as its own inference writes them) |
| Root in row paths | `C:\semio\a\b` (JSON `C:\\semio\\a\\b`), mixed separators, `c:\SEMIO`, `\\?\C:\semio`, UNC shares | `/…/a/b` | (a) | `⟦semio-root⟧/a/b`; a Windows root is compared without case, the extended prefix is ignored, the rest keeps its case; a Windows taker writes the root as given and the rest with backslashes. Independent oracle: Python `PureWindowsPath` |
| Root in JSON-in-JSON paths (`resolve`, `closure`, `registry` rows: `["C:\\semio\\x.mjs","typescript"]`) | raw backslash root, which the old text rewrite could not see (6 286 `closure` and 375 `resolve` rows carried `C:\\semio`) | root token | (a) | first element converted as a path, the rest (closure queries hold whole commands with the root, 203 projects) as text |
| Drive root and parents above the checkout | `lstat C:\` | `lstat /`, `/Users`, … | (a) | dropped (location rows, now for any drive and without case); probes of absent entries above the root (`C:\node_modules`) stay and hold wherever the entry is absent |
| Identity of the source analysis providers | `build:rust-references:v1:<hash>` where the hash covers the module URL (`file:///C:/semio/…`) | different hash per checkout | (a) | measured: the hash differs between checkouts (`c4e967…`, `eb546a…`, `895b92…` on three POSIX checkouts, `e126d8…` on Windows); it becomes a token and the taker puts the id it has registered for that role (its `revision` is still checked, so a different parser never holds) |
| Closure keys (`closureRef`, recipe table, `closure` rows, bundle names) | digest of the recipe text, which names `C:/semio` | digest with the other root | (a) | digest of the neutral recipe in the record; the taker computes the key of the recipe in its own spelling. Checked on the real data: the digest function reproduces 5 025 of 5 025 Windows recipe keys. Recipes themselves equal in 371 of 371 pairs |
| Dependency generation id | `.cache\b\d\m\j\c6d139b…` | other id per checkout and platform | (a) | token (P14), now also for Windows rows because the separators are neutral before the id is looked for |
| Line endings in hashed content | none found: file sha256 equal for the same path in 370 of 371 pairs (the replica copies bytes) | | not a representation | **not normalised**: a CRLF checkout would change the sha, and the tool does read the bytes, so such a project is correctly inferred again |
| Executable suffixes `.exe`, `.cmd` | not in any of the 374 records (rows, texts) | | n/a | nothing to do; they appear only when tools run |
| Symlinks | `lstat` type rows equal in all pairs (the replica creates real links) | | n/a | none; a type that differs fails the replay and the project is inferred again |
| Directory listing order and case | listings are hashed after a sort by code unit, so the order is the same; Windows is case preserving and case insensitive | | verified by the taker | 22 of 371 projects differ in a `readdirNames` hash (entries only one machine has); the taker's replay refuses them |
| Host-bound tool output | `--host-platform=win32` | `darwin` / `linux` | **(b)** | the only declaration in the closure of inference is `metadata.semio.hostPlatformOutputsV1` (closed enum darwin, linux, win32), one project: `@semio-tech/os-plugin-describe-rs`. Keyed by the exact platform; a machine of another class names it and leaves it |
| Environment values | `inputs[].env` differs only in `os-hub` (tree) | | n/a | none |
| Machine-local tree state (generated files that exist on one machine, other `inferrer:*` stamps) | | | verified by the taker | 3 `stamp`, 1 `exists`, 2 whole-tree differences: refused by the replay |

Measured with the new code on the real records (`T/🧰️scripts/R23/lab/measure.ts`, both records written back into their producers' spelling, then both neutralised): **342 of 371 projects (92.2 %) have one identical neutral form** (rows, result text and recipe keys, volatile stamps ignored). The 29 others: 22 directory listings, 3 `stamp`, 1 `exists`, 1 listing+`exists`+`analysis`, 2 large trees (`os-hub`, `os-hub-ts`). No record was refused as unportable (0 of 374 Windows, 0 of 371 POSIX).

## 2. What was built (schema first, English and German texts, tests first)

- `GS/🌍️platform-class`: policy with the classes `posix-shared` and `windows-shared`, the neutral tag `all`, `boundOwners`; `recordPlatformV1` = neutral tag except for host-bound key texts; **`splitRequestV1`**: a machine of the requester's class serves the request whole, a machine of another class serves every project but the bound owners.
- `GS/📦️project-records/🧳️portable` (schema, module, fixture, oracle): `hostOfRootV1`, `neutralPathV1`/`localPathV1`, `neutralRecordV1`/`localRecordV1`, closure bundles, `closureKeyV1`, `UnportableV1`. A record that names the root in a spelling that cannot be mapped is not published (counted `refused`), a taken one with a foreign root token is not used.
- `GS/📦️project-records/🟦️.ts`: the port publishes the neutral record and bundles, takes with its own providers (the registered source analyses, loaded only when the record has analysis rows), generation and closure keys; new counters `refused` and `reinferred` (a taken record that did not hold here and was computed again). Injectable `providers` and `generation` for tests.
- `GS/🙋️wanted`: `claimWantedV1` claims every request it can serve at least in part (no more "left to others" for a whole class), the taken request holds the served projects and names `left`; the answer carries `left`; the requester takes the left names out of the wait and infers them itself (`leftHere` line). Server lines and heartbeat below.
- Tests: `platform-class` 6, `portable` 12, `portable/🌍️real` 4, `project-records` 6 + `two-platforms` 10, `wanted` two-platforms 6, server 15, read 16, wanted 9, cost 20+.

## 3. How each behaviour is proven

| Behaviour | Test |
|---|---|
| Path table for Windows semantics (17 rows: drive and root case, mixed separators, extended prefix, sibling prefix `semio2`, other drive, UNC, POSIX case) | `portable` "the path table" **and** the Python oracle (`PureWindowsPath`) re-derives the same neutral and local paths |
| Six record cases (Windows with a bundle taken by macOS, Linux, another Windows checkout and a machine without generation; macOS to Windows; Linux; UNC share; two refusals) | `portable` "every record case" + oracle with its own SHA-256 of the sorted recipe |
| Real data: the neutral form of a real Windows record equals that of the real macOS record of the project, and the neutral Windows record spelled for the Mac is the record the Mac produced (and the reverse) | `portable/🌍️real` (3 tests on 2 projects with root-bearing recipe keys) |
| Windows record taken by macOS and the reverse through a real local cache server | `two-platforms` tests 4 and 5 |
| **30 neutral projects served across classes with identical digests, the bound ones refused** | `two-platforms` "thirty neutral projects": 27 taken, 3 bound refused, one digest |
| Windows rows spelled for a macOS tree hold in the **real replay of the observations** (`observationRowsHoldV1`) and do not without the rewrite | `two-platforms` last test |
| A macOS server claims and serves a Windows request except the bound owner, said once and counted; a bound-only request is left unclaimed; a Windows-class server takes it whole | `wanted/🌍️two-platforms` 2 tests |
| The Windows worker keeps the bound project and says so, 300 stale: the server served the rest | `wanted/🌍️two-platforms` last test |
| Refusal is typed and counted | `refused` counter test; `foreign` text; split table in the platform-class fixture and oracle |

Mutation checks (each turns the named tests red; red counts are `portable`, `real`): backslashes kept after the root 4/1, owner not tokenised 3/3, closure keys not moved 3/2, root compared with case 4/0, Windows slash text not mapped 4/2, raw backslash text accepted 2/0, closure query text not mapped 3/1, unknown kind left alone 1/1, extended prefix kept 3/0.

## 4. Heartbeat and request lines (English; the German texts are in the schema)

- `graph-server: alive, <n> machines seen, <h> requests held, <cross> projects served across platform classes, <left> projects left to a machine of their class, read <none|request|refresh>`
- request claimed: `graph-server: serving <n> stale projects for <requester>`, then for another class `graph-server: <requester> (<platform>, class <class>) is served across platform classes by a machine of the class <own>: <served> projects served, <left> bound to the host and left to a machine of its class: <names>`
- nothing servable: `graph-server: all <n> stale projects of <requester> (<platform>) are bound to the host and left to a machine of the class <class>; this machine serves the class <own>`
- requester: `graph: <n> projects are bound to the host (<names>); the servers of other platform classes leave them, this machine infers them itself`; progress `graph: <s> projects taken from the fleet with <c> closure entries, <r> of them did not hold here and were inferred again, <f> not shared, <k> inferred here`.

## 5. Not verified

- **Anything on a real Windows machine.** Windows is proven by 374 real stored Windows records written back into their producers' spelling, by simulated `C:\semio\…` facts and by an independent Python oracle; no Windows process ran the new code, no Windows server served its own class.
- **The live taking.** `registeredProvidersV1` (a dynamic import of `source-analyses` and `prepareSourceAnalysesV1()`) was never run in a real inference process; if it throws, the analysis rows keep a token that cannot hold and the project is inferred locally. The real `repo` build command of the landing rules was not run (no graph command allowed on this machine): the coordinator runs one.
- The reuse rate on a real Windows taker (92.2 % is the share of equal neutral forms on stored records, not a taker's measurement). Closure bundles are proven on synthetic bundles only (the stored ones are 2 to 41 MB). Cost of the neutral form on the largest records (os-hub) not measured on a loaded machine.
- Case-insensitive listings that differ in case only, 8.3 short names, UNC and long-path roots beyond the fixtures, CRLF checkouts (by design not normalised).
- The group server restarting on the new code; the taxonomy and name-registry gates for the new test folder `🌍️real`.
- Known reds that are independent of this package: `🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking` (3) and `🍏️macos-reading` (3) fail the same way on the tree before the landing (`Expected "unserved"`, received undefined; some other change of the fleet-read result); `land.py` treats exactly these as the baseline.

## 6. For the others

- Existing project records (keyed by the P14 class tags `posix-shared` / `win32-x64`) and the wanted/served slots written by P14 are unreachable now (no migration); each machine offers its local records again in the neutral form (the `.fleet` marker holds the old state digest), the next refresh of the group server re-publishes.
- `servesPlatformV1` remains (same class); the serving rule is `splitRequestV1`. A second host-bound declaration must be added to `boundOwners` in `GS/🌍️platform-class/🧬️schema/🔣️.json` (the record is keyed correctly anyway, by its key text; the list only names it in the request): the test "the bound owners of the policy are the projects of the repository that declare a host platform output" fails for the one known declaration only.
- The Windows worker's agent: `ROUND-NOTE-R23.md` (this folder). Rollback: `python3 -I T/🧰️scripts/R23/land.py --rollback T/🗑️generated/R23-landing-074229`.
- Scripts: `T/🧰️scripts/R23/` (`land.py`, `payload.json`, `new/`, `lab/` with the extraction of the stored records, the measurement, the fixture generators and the mutation runner).
