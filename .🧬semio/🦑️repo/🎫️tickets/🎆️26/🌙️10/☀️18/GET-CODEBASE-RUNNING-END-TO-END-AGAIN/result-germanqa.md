# Lane germanqa (prefix gq), wave 1027b: German text review of the repo tool

Scope: every German text in the repo tool's catalogs (`🧰️framework/🛍️products/🦑️repo/**`, `🧰️framework/🔨️modules/🏃️process/**`): 202 JSON files with EN/DE pairs (7996 pairs, 3875 distinct German texts), 16 `EN: … DE: …` schema descriptions, 10 `English / Deutsch` schema descriptions and titles (found late; their German half was ASCII in 5 bazel schemas), the German sections of 7 markdown files (build guide first), the German string literals of the sources that pin them (Go/Rust/TS/Python/mjs). The English side was never edited (law below proves it).
Mirror: `gq-main` (cloned ~19:35, dropped at the end of the lane; one file synced from live because a peer landed a catalog change in between). No live edits. No sub-agents, no ticket tools, no git writes.

## FOR coordinator

- Landing group: 7 file-disjoint batches, 138 files, 928 changed lines, one group (the tests that pin catalog wording change with the catalogs; applying a catalog batch alone breaks pinned tests). Order does not matter, `apply_all.sh` uses `process libbuild libcache libother mcp modules docs`.
- REBASE first (hot files: `🏗️bazel/📜️script.ts`, `🕰️generations/🟨️.mjs`, `build-guide.md`): `sh C/germanqa-tools/rebuild_all.sh` re-reads the live files and re-applies the deterministic rules (stops on a missing anchor, runs the edit law).
- check: `sh C/germanqa-tools/apply_all.sh /Users/ueli/Documents/semio check` (all `base` on live at build time; `DRIFTED` = rebuild).
- apply: `sh C/germanqa-tools/apply_all.sh /Users/ueli/Documents/semio apply`
- laws after apply (must be GREEN): `python3 -I C/germanqa-tools/law_germanqa.py /Users/ueli/Documents/semio` and `python3 -I C/germanqa-tools/law_edits.py C/germanqa-tools/staged/*`
- per batch (mirror or live): `VERIFY_ROOT=<root> SEMIO_TEST_ARTIFACT_DIR=C/🗑️generated/germanqa/a sh C/germanqa-tools/staged/<batch>/verify.sh` (selftest, edit law, RED count before, apply, GREEN law, the test directories next to the changed files).
- revert: `sh C/germanqa-tools/apply_all.sh /Users/ueli/Documents/semio revert`
- all `stage.py` keep file modes (`python3 -I C/xplat5-tools/tests/test_stage_mode.py <stage.py>` prints OK for the seven).
- Effects of landing: catalogs under the build library change, so expect ONE graph re-inference (like `umlaut`). No code logic changed, only string literals. Cheap proof after landing (no build needed): `SEMIO_LANGUAGE=de bun repo capacity status`, `SEMIO_LANGUAGE=de bun repo leases status`, `SEMIO_LANGUAGE=de bun repo doctor`, `SEMIO_LANGUAGE=de bun repo cache status` must read German with the glossary below; product runs: `course:build`, `course:test` (the German catalogs are inputs), dashboard build (the Rust label/usage literals and the two Go constants are inputs; not compiled by me).
- NOT done by me (needs a build): the committed `💻️client/🔌️mcp/mcp` binary embeds the old `RepositoryInstructions` (Go const + Rust twin changed in batch `mcp`): `repo run repo-mcp:install` after landing. Go/Rust tests were not run (cargo budget); the edits there are inside string literals only.
- Not mine, same pattern: the product `README.md` German section (316 KB) keeps the old vocabulary (about 130 glossary findings: Belege, Claim, Deadlock, Dependency-Sync, Collector, Policy, Lease-Gender, Worktree …). Only its one ASCII fallback (`ausserhalb`) is fixed. Re-running `build.py` with `TRANSLIT_ONLY` emptied applies the full rules to it; it needs a read-through of the 316 KB first, so I left it as a decision.

## Law (red on live, green on the batches, independent of the fix rules)

`C/germanqa-tools/law_germanqa.py <root> [--files-from manifest] [--limit N]` (stdlib only; `--selftest` runs 13 red fixtures and 4 green ones; macOS/Linux/Windows/devcontainer, works without `.git`):
- parity: EN and DE hold the same `{fields}` (oracle: `string.Formatter().parse`), the same `` `code spans` ``, DE exists and is not empty, no untranslated English sentence; fixtures with deliberately broken pairs are exempt by path.
- translit: no ASCII fallback of ä ö ü ß. Two detectors: the explicit map `translit_map.json` (103 words, derived from the macOS German spell checker: token rejected, umlaut form accepted) and a corpus detector (any token whose umlaut twin occurs elsewhere in the tree) plus a stem regex.
- glossary: 46 forbidden-pattern rules (one per concept or grammar fact; `closure` and `receipt` are EN-aligned: EN `closure` must not become `Hülle`, EN `receipt` must not become `Beleg`).
- grammar: Lease/Task/Crate/Verb gender, `Sie` imperatives, `Agents`, `Nutzerin`, capitalised verbs at the end of titles, `n %`, `min/h/d`, `{n} s`.
- `C/germanqa-tools/law_edits.py <staged>...` (independent of the builder): every staged JSON keeps its structure, every English leaf is byte-identical, only German leaves changed (German leaves = `de` of a pair, `DE:` halves of schema descriptions, expectations of fixtures carrying `language: "de"` / `SEMIO_LANGUAGE=de`); text/source files keep their line count.

Numbers: law on live (whole roots, README glossary excluded) = RED 1200 findings in 48 rule ids (translit 266 in 22 files, policy 204, closure 99, task 81, rechner 56, receipt 48, action 45, image 42, pin 24, titles 24, ledger 21, lease gender 20, user 20, worktree 20, …). On mirror `gq-main` with all seven batches applied: GREEN 0. `law_edits`: GREEN. Selftest: GREEN.

## Glossary (one German term per concept; dominant term wins, counts are distinct German texts before)

| Concept (EN) | German term | Replaced / forbidden | Before |
|---|---|---|---|
| closure (transitive) | Abschluss (Befehlsabschluss, Anbieterabschluss, Quellabschluss, Projektabschluss) | Closure, Hülle, Hüllen-Inventar | Abschluss 36, Hülle 24, Closure 22 |
| envelope / process shell | Hülle (Aufruf-Hülle, Skript-Prozesshülle, Implementierungshülle) | kept only for this concept | |
| receipt | Quittung (Generationsquittung, Quittungsformat) | Beleg | Quittung 15, Beleg 13 |
| evidence / proof; claim | Nachweis; Anspruch | Beleg, Claim | Anspruch 13, Claim 2 |
| cache ledger | Cache-Bilanz (Größenbilanz); Hauptbuch for the unit-cache shared ledger; Ledger only as a component name (`stdio-Ledger`) | Cache-Verzeichnis for a ledger, Größenverzeichnis | Bilanz 20, Ledger 5 |
| pin / pinned | Pin / gepinnt | angeheftet, Anheftung, fest eingebunden, fest gehalten | gepinnt 46, angeheftet 15 |
| machine, host (the computer) | Rechner (Rechnerkapazität, Rechnerspeicher, Rechnerklasse, Rechner-Budget) | Maschine(n-), Host-Kapazität, Host-Speicher, Host-Zähler | Rechner 134, Host 53, Maschine 35 |
| host (software) | Host (Plugin-Host, Pseudo-Terminal-Host, native Host, Host-Vertrag, Bind-Mount vom Host) | | |
| retire / retirement | stilllegen / Stilllegung | ausmustern | stillgelegt 17, ausgemustert 5 |
| reclaim | zurückgewinnen / Rückgewinnung | Reclaim | |
| collect (gc) / collector | einsammeln / Sammler | Collector, Einsammler, Müllsammler, Garbage Collection | Sammler 12, Collector 4, Einsammler 7 |
| snapshot | Schnappschuss | Snapshot | 25 vs 5 |
| lane | Lane | Bahn(en) | Lane 28 (incl. guide), Bahn 16 |
| slot | Slot | Sitzungsplatz, Kernplätze, Kapazitätsplatz, Rechenplätze, Platzdatei | Slot 47 |
| image | Image (Cache-Image, Disk-Image, Datenträger-Images) | Abbild, Abbilddatei, Scratch-Abbilder | Image 75, Abbild 34 |
| mount | einhängen / eingehängt | angehängt (kept only for the `hdiutil attach/detach` pair) | 20 vs 3 |
| owner; holder | Eigentümer; Halter | Besitzer; Inhaber | Eigentümer 57 |
| policy | Richtlinie (Richtlinien-Datei) | Policy | Richtlinie 34, Policy 32 |
| scope; namespace | Scope; Namensraum | Geltungsbereich; Namespace | 14 vs 3; 14 vs 1 |
| deadlock | Verklemmung (Verklemmungszyklus, Selbst-Verklemmung) | Deadlock | 11 vs 3 |
| task (build task) | Task (feminine: die Task) | Aufgabe, der Task | Task 118, Aufgabe 79 (all `Aufgabe` were EN `task`) |
| task (dashboard UI) | Aufgabe (dashboard module and `project.json` titles only) | kept: separate concept, one term per concept | |
| action | Aktion (Aktions-Cache, Aktionsergebnis) | Action(s) | Aktion 89, Action 36 |
| worker | Worker (Worker-Prozess, Datei-Worker) | Arbeitsprozess, Dateiarbeiter | Worker 81 |
| worktree | Arbeitsbaum | Worktree | 30 vs 11 |
| capacity governor | Kapazitätswächter | Kapazitätsregelung, Steuerung | 4 vs 2 |
| grant; admission | Zusage; Zulassung | Zuteilung, Freigabe (capacity), Aufnahme | Zusage 37 |
| builder | Builder | Bauer | |
| staged (git) | vorgemerkt | gestaged, gestagt, stagen | |
| user | Nutzer | Benutzer, Nutzerin | 17 vs 7 |
| agent | Agent / Agenten | Agents | |
| swap | Auslagerung | Swap | 3 vs 3 |
| disk | Datenträger | Platte, Plattenbudget, Plattenplatz | 16 vs 15 |
| cache miss | Fehltreffer | Fehlgriff | 4 vs 3 |
| compile units | Compile-Einheiten (units of a compile), Compiler-Einheiten, Kompilationseinheiten (EN compilation units) | Kompiliereinheiten, Compilereinheiten | |
| unit cache | Unit-Cache | Einheiten-Cache | |
| quota | Quote (Quotenanteil) | Quota | |
| id | ID | Id | 17 vs 4 |
| micro-commit | Micro-Commit | Mikro-Commit | |
| ready address | Bereitschaftsadresse | bereite Adresse | |
| file operation | Dateivorgang | Dateioperation | |

Kept English on purpose (dominant or technical): Lease, Cache, Worker, Slot, Lane, Image, Pin, Scheduler, Commit, Checkout, Fixture, Hold-and-Wait, Handshake, Allowlist, Birth (protocol phase of a Bazel action: `Birth-Eingabe`, `Birth-Handshake`), Facet. Translated for consistency: Loader -> Lader.

## House style and typography

- Address: `du`-imperative for direct instructions (68 texts: `führe`, `starte`, `installiere`), impersonal infinitive for terse hints (80: `erneut ausführen`), `Sie` never; the single formal outlier (`Hängen Sie es ab … führen Sie den Befehl erneut aus`) and `Ihre Version` are now du.
- Units: `{n} s`, `ms`, `min`, `h`, `d` with a space (was `Min.`/`Std.`/`T.` next to `min`); percent as `{n} %` (was 11 with, 11 without); `ID` not `Id`.
- Titles of dashboard tasks are sentence case with a lowercase verb (`Cache zurücksetzen`, was `Cache Zurücksetzen`: 24 titles).
- Unchanged (mirrors EN, consistent): straight quotes, spaced em dash, `…`.
- Grammar fixed: `ein Lease` (neuter) -> `eine Lease` (26 feminine vs 12 neuter before), `der Task` -> `die Task`, `Das Crate` -> `Die Crate`, `Der Micro-Commit-Verb` -> `Das …`, `des Agents` -> `des Agenten`, `als losgelöste … Kindprozess` -> `losgelösten`, `wurde nicht bereit` -> `ist nicht bereit geworden`, `Ambiente Auflösungsbedingungen` -> `Umgebungsweite …`, `der speicherzulassende rustc-Scheduler` -> `der rustc-Scheduler mit Speicherzulassung`, the relative clause `Lease, der statfs … liest`, `die Arbeit kann nicht mehr nutzen` -> `… mehr davon nutzen`, `Inkrementell-Sitzung` -> `Inkrementsitzung`, `Nutzerin` -> `Nutzers`.

## What changed (by batch; lines = changed lines)

| Batch | Files | Lines | Content |
|---|---|---|---|
| process | 23 | 105 | process module catalogs (budget, host-capacity, leases, agent-sessions, toolchains, sandbox law messages, progress/routing) and two tests |
| libbuild | 62 | 585 | build library catalogs (operations help and its rendered help fixtures, graph, bazel incl. the `English / Deutsch` schema descriptions, doctor, preflight, fleet, bench/slo, parity, remote cache/execution, resident), 10 TS tests/sources, one `.mjs` copy |
| libcache | 21 | 106 | cache/ledger/storage/bootstrap/unit-cache catalogs, 3 tests |
| libother | 7 | 25 | commit/verify/commands/laws catalogs |
| mcp | 4 | 9 | MCP surface JSON, Go const + Rust twin `RepositoryInstructions` |
| modules | 14 | 30 | dashboard/client/workspace/launcher JSON, Rust labels/usage literals, Go/Rust bilingual error strings |
| docs | 7 | 68 | build guide (German half), sealed README (whole German section was ASCII), door/doctor/launcher/dashboard READMEs, product README (one token) |

Biggest classes: 266 ASCII fallbacks the `umlaut` batch did not cover (`🏖️sandbox` law messages, toolchain pinned, process-chain test, sealed README, build guide, schema `DE:` descriptions, 8 more files), 204 `Policy`, 99 `Closure/Hülle`, 81 `Aufgabe`, 56 Rechner/Host/Maschine, 48 `Beleg`, 42 `Abbild`, 24 titles, 20 Lease gender.

## Reviewed and left (reasons)

- `Birth-*` words (14) are the protocol name of a Bazel action phase; `Geburtsprozess` became `Birth-Prozess` to match.
- Registry words stay split by sub-concept: `Registry` (cargo registry, backend registry; 18), `Registratur` (runtime and tool registry; 10), `Register` (lease registry rows; 8). Unifying touches the launcher registry catalog that lane langpick stages right now.
- `Disk-Cache`, `Disk-Image` (Bazel/macOS names) stay next to `Datenträger-Images`.
- `der Lock` (lock file, masculine, 5 uses, consistent), `Fixture` (neuter, consistent).
- `abgelehnt` vs `verweigert` for refuse: both natural, split by object (request vs action).
- `Kindprozess … das Kind abgeschlossen`: awkward but correct; `Authorizer`, `Actor`, `Loader`-like English component names in schema titles.
- Inline `en`/`de` ternaries and `both("EN","DE")` calls in sources (bench/units/conformance commands, ci-report, goal-gate) were reviewed for wording only; moving them into catalogs is the language-law backlog (not a German-quality issue).
- Machine-checkable but not claimed: naturalness beyond the rules above (read through all 3108 distinct German catalog texts except the generated help fixtures, which are copies; the product README German section was not read).

## Verification on mirror gq-main

- RED on base: `law_germanqa.py` 1200 findings on live now (1103 at the first mirror run, before later rule additions), GREEN (0) on the mirror with all seven batches applied; `law_edits.py` GREEN; selftest GREEN.
- Tests (see "Test comparison" below): 84 test directories next to the changed files, run on the mirror in the base state and again with all batches applied, through `mirror.py run` (sealed, guard clean), failure sets compared.

## Files

- Tools: `C/germanqa-tools/` — `rules.py` (data: ASCII map, glossary rules, grammar fixes, one `transform`), `translit_map.json`, `build.py` (re-reads live, deterministic), `gqlib.py` (readers), `stage_all.py`, `rebuild_all.sh`, `apply_all.sh`, `law_germanqa.py`, `law_edits.py`, `runtests.py`, `cmp_tests.py`, `staged/<batch>/` (`manifest.json`, `files/`, `base/`, `stage.py`, `files.json`, `tests.txt`, `verify.sh`).
- Analysis helpers (read-only, kept): `extract.py`, `vocab.py`, `variants.py`, `glossary_mine.py`, `ctx.py`, `q.py`, `dump.py`, `dryrun.py`, `iterate.py`, `showdiff.py`, `translit_map.py`.
- Generated (delete at ticket end): `C/🗑️generated/germanqa/` (pairs, vocab, dumps, test results).

## Test comparison (mirror gq-main, bun test per directory, timeout 60 s/test, base state vs all batches applied)

- 84 directories, 1427 tests: base 1304 pass / 123 fail (28 directories already fail on live: envelope-only suites, stale fixtures, load), after 1302 pass / 125 fail.
- Failure sets identical in 83 of 84 directories. The only difference is `🏗️build/📏️incremental/🏎️bench/🧪️tests` (7 failures on base): under load an 8th/9th test fails, a different one on every run (`real ports find the resident service`, `prepares a fresh copy`, `stores every run in the history`, `a crashed command's chunk …` in another run); each passes when run alone on the applied mirror (`-t` filter) and with a 180 s timeout the failing set is the 7 base failures plus one of these load flakes. Not German related (ports and process timing).
- What the comparison found and fixed on the way (the pins are now part of the batches): fixtures with extra keys next to `en`/`de` (generation receipt reasons), fixtures that carry the German expectation next to `language: "de"`, `environment.SEMIO_LANGUAGE=de` or `LANG=de…` (failure tail, toolchain pinned, graph-phase, fleet golden corpus), test literals too short to look German (`"[bazel] Aufgabe "`, `GiB/Std.`, `Inkrementell-Sitzung`), `English / Deutsch` schema descriptions, Python oracle messages. Re-run: `python3 -I C/germanqa-tools/runtests.py <label> 0 100 [only.json]`, `python3 -I C/germanqa-tools/cmp_tests.py`.
- Not run: Go/Rust tests (cargo/go budget), the dashboard Rust label tests that pin `Bereitschaftsadresse` (edited together with the labels), `bun repo` product runs (coordinator).
