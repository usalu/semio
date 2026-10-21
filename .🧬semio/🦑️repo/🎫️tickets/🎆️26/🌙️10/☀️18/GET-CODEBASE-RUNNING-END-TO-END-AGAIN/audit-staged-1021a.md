# 🔎️ Staged Wave 1021a Audit (read-only, auditor aud-staged-1021a)

📅️ Snapshot: 2026-10-21, about 09:21 local. Four sets were still being written during the audit (tool-iface, runtime-seam, pm-seam, polyglot-units). Their findings describe the snapshot and must be re-checked before round 6 applies.

📜️ Rules checked (AGENTS.md): emoji-first unique docstrings; no comments inside definitions; concise code; no new runtime dependency on external libraries and none exported; EN and DE texts with no default language; schema-first; no legacy, compat, deprecation or migration leftovers; no `[DEBUG]`; paths at most 256 chars; tool-independent interfaces.

🧪️ Method: repo-wide greps and scripted checks over all 13 staged sets (docstring leads, in-body comments, imports, shims, hard-coded paths, path lengths, raw spawns, English fallbacks). Read samples per set (about 15 files, selected by size and by new/changed status). No builds, no `bun repo`, no git-modifying commands, nothing written except this file.

## 🚦️ Verdict

NOT READY for round 6. 15 blocker items below. Most are cheap to fix. Three are structural: the tool compat layer and tolerated-spawn ledgers, and the stub re-export bridges.

Severity: **B** blocker (must fix before round 6), **H** high (AGENTS MUST violated), **M** medium, **L** low.

## 🗂️ Per-set findings

### repo-launcher (37 files)

Sampled: action.yml, install/🐚️.sh, install/🔵️.ps1, packages/go/📜️script.ts, 📦️main/🐹️.go, 📥️runtime-provision/🐹️.go, 🛎️command, 🧭️workspace-root, 🗂️runtime-registry, tests/🗂️registry-schema/🟦️.ts, tests/🧰️harness/🐹️.go, two .feature files, registry JSON, README.md.

Pass: 103 of 103 Go declaration docs start with an emoji. No in-body comments. go.mod has no `require` (stdlib only). Registry messages are complete in EN and DE (28 of 28). Registry schema is present. Ajv is used only in tests.

| Location | Rule | Sev | Note |
|---|---|---|---|
| 📥️runtime-provision/🐹️.go:144 | EN+DE, no default language | M | `"https or loopback http required"` is a hard-coded English reason in a Failure value |
| 📥️runtime-provision/🐹️.go:382 | EN+DE | M | `"larger than "` English fragment inside a Failure value |
| 🛎️command/🐹️.go:42, 51 | EN+DE | M | `h.Stderr(err.Error())` sends raw, untranslated error text to the user |
| 🧭️workspace-root/🐹️.go:57, 63 and 🗂️runtime-registry/🐹️.go:125, 136 | EN+DE | M | `Reason: err.Error()` passes raw error text into user-facing error values |
| 📥️runtime-provision/🐹️.go:42 | EN+DE | L | `Failure.Error()` prints a map dump (`fmt.Sprint(f.ID, f.Values)`) |
| README.md (new) | docs | L | New markdown file, EN only; confirm it is wanted |
| README.md:23, 🏗️entry/🐹️.go:95, 🛎️command/🐹️.go:141 | AGENTS forbids agent use of `repo dashboard` | L | `dashboard` is a routed verb; confirm no agent flow uses it |
| staged ticket copy `files/...🐹️.go` | path length | M | Longest ticket-side copy: 225 chars, 297 UTF-8 bytes. Under 256 by characters, over by bytes |

### tool-iface (166 manifest entries, changing)

Sampled bodies: after/0069, 0074, 0075, 0113, 0114, 0116, 0034, 0042, 0003, 0004, 0008, 0022, 0047, 0089, 0081, 0086, plus the manifest. Kinds: 120 migrated, 28 new, 9 edit, 2 generated (snapshot at the start was 136 entries).

Pass: 1,176 of 1,177 JS doc comments start with an emoji (the one exception is a keycap, see below). Third-party imports (manifold-3d, three, jsonschema, Ajv) are in migrated plugin code and are existing devDependencies, not new ones. Tests exist, with fixtures.

| Location | Rule | Sev | Note |
|---|---|---|---|
| after/0069 → 🔁️compat/🟦️.ts (new) | no compat layers | **B** | Drop-in replacement for `node:child_process` (`spawnSync` and related), named and documented as compatibility |
| after/0075:8 | no compat layers | **B** | Tool barrel re-exports `./🔁️compat/🟦️.ts` |
| after/0074:6, 12, 30 | no legacy support | **B** | Guard option `tolerated` ("spawns tolerated until they migrate") |
| after/0116 (generated, direct-tool-spawn ledger) | no legacy support | **B** | 592-row ledger of grandfathered raw spawns; after/0113:98-101 compares against it; after/0114:40 tests the tolerance |
| after/0034 (LEGACY_ENGINE_BIN, LEGACY_RECIPE, generateLegacy) | no legacy support | H | Legacy engine and "pattern-shell" recipe kept alive |
| after/0042 (ShimFileReader, ShimProgressEvent, globalThis `??=`) | no compat layers | H | Global browser-API polyfills in generator code |
| after/0003:53, after/0004:56 | no compat layers | M | Comment states "carrying them is one shim block" |
| after/0008:135, after/0022:19-20, after/0047:104 (Python inside definitions) | no comments inside definitions | M | Comments inside function bodies |
| after/0089:85 | emoji docstring | L | Docstring leads with keycap `#️⃣`; technically emoji, confirm fitting |
| manifest, 120 "migrated" entries | no migration leftovers | M | Manifest has no removal records for old paths; confirm the old copies are deleted, otherwise compat duplicates remain |
| after/0002, 0003, 0034, 0041 (manifold-3d, three) | no new runtime deps | L | Existing devDependencies in migrated generator scripts; confirm they never run as product runtime |

### runtime-seam (25 files, changing)

Sampled: selection, bun, node, portable/yaml (new), contract, law, law ledger JSON, sqlite, serve, spawn, toml, tests/🟦️.ts, plus greps.

Pass: all 109 JSDoc comments start with an emoji. `Bun` names are confined to the Bun adapter. yaml, toml, glob, jsonc, and semver are in-house. Third-party oracles (ajv, picomatch, @iarna/toml, jsonc-parser, semver) appear only in tests via `createRequire`.

| Location | Rule | Sev | Note |
|---|---|---|---|
| ⚖️law/📒️.json (76 rows) and ⚖️law/🟦️.ts, RuntimeLawLedgerRowV1 docstring | no legacy support | M | Grandfather ledger: findings "tolerated for a stated reason until its owner removes it". Same pattern as the tool ledger; drop it or justify |
| 🪶️portable/🌍️serve/🟦️.ts | EN+DE | L | `Request body too large` English only (dev-facing) |

### storage-seam (21 files)

Sampled: selection, contract, builtin, w/71362aa01863.ts, w/15fa4432e5e8.ts, w/b67a32a67260.py, map.json (15 destinations, all under 🧰️framework), plus greps.

Pass: 34 of 34 TS docstrings start with an emoji. Contract re-exports are explicit. Python oracle runs with `-I` and stdlib only. Schema is present.

| Location | Rule | Sev | Note |
|---|---|---|---|
| files/w/b67a32a67260.py:1 | emoji docstring | **H** | Module docstring "Independent oracle of…" has no emoji |
| files/🧰️framework/… mirror copies | path length | M | Longest ticket copy: 235 chars, 307 UTF-8 bytes, 249 UTF-16 units. Flatten the ticket copy layout |

### pm-seam (24 files, changing)

Sampled: dba25ea5de03.ts (test), c1318bbfafca.ts, 35bb3516dc67.ts, 27ff45c076c2.ts, 8e32c976d414.ts, 46504f1b8f77.py, 5bc8b73f1d83.ts, 4cf8678faf72.ts, 67478970175c.ts, db1fb054d413.ts, plus greps.

Pass: JSONC and TOML parsers are in-house. smol-toml, jsonc-parser, and Ajv are test-only oracles. Python oracle uses stdlib only. Descriptors are schema-first.

| Location | Rule | Sev | Note |
|---|---|---|---|
| new/w/c1318bbfafca.ts:61 | emoji docstring | L | Keycap `#️⃣` lead, confirm fitting |
| new/w/27ff45c076c2.ts | docstring hygiene | L | Two stacked docstrings: the `🔬️ Asks the tool itself` text sits above `spawnProbeV1` and the `🖥️ Describes` text is orphaned |
| new/w/dba25ea5de03.ts:270-273 | tool-independent interface | L | Raw `spawnSync("go"/"npm")` in a test, acceptable if the test is the only caller |

### backend-contract (27 files)

Sampled: backends.ts, bazel_backend.ts, semio_remote.ts, semio.ts, stub.ts, backends.registry.json, conformance_oracle.py, nx_driver.ts, semio_store.ts, semio_backend.ts, backends_test.ts, conformance_test.ts, semio_scheduler.ts, semio_test.ts, taskmodel.ts, plus greps.

Pass: docstrings start with emoji (129 TS, 1 Python). EN+DE is required by the type `BackendTextV1`. Schemas present for backends, conformance, and semio. Python oracle is stdlib only.

| Location | Rule | Sev | Note |
|---|---|---|---|
| semio_scheduler.ts:23 | tool-independent interface | **B** | `spawn("git", …)` in runtime code bypasses the tool interface (see tool-iface `🌿️git`) |
| conformance_test.ts:26 | cross-platform | M | `"/tmp/markers.log"` is POSIX-only |

### hot-cache (104 files)

Sampled: dev/bench/legacy.mjs, bench.mjs, replay.mjs, events.mjs, service-edit.mjs, dev/resident/index.mjs, protocol/policy.mjs, dev/resident/service/index.mjs, dev/resident/tests/common.ts, dev/go/resident.go, dev/native/watch.rs, staged/apply.py, edits.json, plus greps.

Pass: all 21 Go declaration docs start with an emoji. Schemas present (service, resident). EN+DE policy messages are complete. apply.py maps dev/ into emoji taxonomy paths.

| Location | Rule | Sev | Note |
|---|---|---|---|
| dev/bench/workspace.mjs:7; dev/resident/tests/delegate.test.ts:9, service.test.ts:66, common.ts:27, common.ts:44, watch.test.ts:126; dev/resident/delegate/executor.mjs:45, 49; delegate/index.mjs:58, 66 | emoji docstring | **H** | 10 JSDoc comments without an emoji lead |
| 9 Python module docstrings | emoji docstring | H | Includes ticket tools mirror.py, mutation-check.py, make-real-overlay.py, run-staged-tests.py (ticket-only, see below) |
| dev/bench/legacy.mjs:15; bench.mjs:10, 12, 14; replay.mjs:7, 9; events.mjs:10; service-edit.mjs:18 (`/tmp/hb-`) | cross-platform, zero-touch | **H** | macOS-only paths (`/private/tmp/...`, `/tmp`). apply.py rewrites only serve.mjs (line 103), so these survive into the repo |
| dev/resident/service/index.mjs:321, 360 | no default language | **H** | `context.languages?.[0] ?? "en"` makes English the default |
| dev/bench/legacy.mjs | naming, legacy | L | Stateless benchmark path named "legacy"; rename |
| staged root: apply.py, mirror.py, mutation-check.py, verify.sh (`/private/tmp` default at line 7), edits.json | migration leftovers | M | Ticket-only apply tooling. Keep it out of the repo, delete it from the ticket after apply. apply.py also embeds `/Users/ueli/...` as anchor text |

### polyglot-units (20 files, changing)

Sampled: units/🟦️.ts, 🦀️rust, 🟦️typescript, 🛠️cpp (new), 📓️tex (new), 🔷️dotnet (new), 🕸️wit (new), 🏃️engine, 🐹️go, ⌨️command, tests/🟦️.ts, tests/🐍️oracle/🐍️.py, cargo-metadata.json, plus greps.

Pass: TS docstrings start with an emoji (except one, below). No in-body TS comments. In-house parsers (csproj, WIT, TeX, cargo) keep the runtime dependency-free. Ajv is test-only.

| Location | Rule | Sev | Note |
|---|---|---|---|
| 🐹️go/🟦️.ts:55, 71, 88, 127, 159, 251 | no legacy support | H | `legacyConstraints` and `// +build` evaluation (labelled legacy). Keep only if required for correct Go semantics, and name it accordingly |
| ⌨️command/🟦️.ts:302 | tool-independent interface | **B** | `spawn("git", …)` in runtime code bypasses the tool interface |
| tests/🟦️.ts:133 | emoji docstring | H | Docstring without emoji lead |
| tests/🐍️oracle/🐍️.py (module + 2 functions) | emoji docstring | H | Three docstrings without emoji lead; the walrus comment at line 400 sits inside a definition |
| tests/🧫️fixtures/cargo-metadata.json | cross-platform | M | Fixture hard-codes `/private/tmp/polyglot-units/...` |
| 🧩️units/🧪️tests/🔣️.json:181 `_compat_pickle` | compat | — | False positive: a Python name in a fixture |

### ide-free (56 files, plan and payloads, no code)

Sampled: plan.json (164 edits, 61 deletes, 438 moves), manifest edits to package.json, new/writes/agent-client-text-5.txt, providers Go payloads.

Pass: no IDE or extension dependency added. The package.json edits only remove vscode and Jupyter packages. Providers Go payloads import stdlib only. Agent client names match the AGENTS list.

| Location | Rule | Sev | Note |
|---|---|---|---|
| plan.json, deletes (61, 59 in the "extension" group, plus duplicate plan files) | destructive ops | **B** | Permanent deletes of user files (matched by sha256). Needs explicit user approval before apply |
| new/writes/agent-client-text-5.txt:18-21 (providers Rust crate) | no new runtime deps | M | `serde` and `serde_json` as runtime crate deps. Justify, or put behind an interface |
| 🧩️providers/📦️packages/🎆️editors/🐹️.go (new) | no IDE | L | Confirm "editors" is not IDE integration |

### langs-conc (21 files)

Sampled: toolchains.ts, native.ts, native-port.ts, native-script.ts, bazel-native.ts, toolchains-stub-process.ts, toolchains-stub-framework.ts, Directory.Build.props, global.json, pins-test.ts, plus layout.py.

Pass: 55 of 55 TS docstrings start with an emoji. toolchains.ts:9 is bilingual. Third-party TOML, Ajv, and typescript appear only in tests.

| Location | Rule | Sev | Note |
|---|---|---|---|
| toolchains-stub-process.ts and toolchains-stub-framework.ts, mapped by layout.py:38-39 | no compat layers | **B** | One-line `export *` bridges placed at second package locations of the toolchains module. Compatibility bridges; use one canonical location |
| bazel-native.ts:7 | EN+DE | L | `Closed Bazel native toolchain classes required` English only, while toolchains.ts:9 is bilingual |

### graph-mem (18 files)

Sampled: new/memory/module/memory.mjs, memory.d.mts, tests/test.ts, wiring/test.ts, shared/test.ts, claims/test.ts, both oracle.py files, schemas, ops.json, bench.json.

Pass: 21 of 21 JS docstrings start with an emoji. Python oracles are stdlib only. ajv and typescript are test-only.

| Location | Rule | Sev | Note |
|---|---|---|---|
| new/memory/module/memory.mjs:29 | no default language | **H** | `?? "en"` default |
| new/memory/tests/oracle.py (module + FunctionDef), new/claims/oracle.py (module) | emoji docstring | H | Three docstrings without emoji lead |
| new/memory/module/memory.mjs:1-6, 5 comment | schema-first | M | Policy constants and bilingual messages hand-coded in JS, kept equal to the schema by a law. Make the schema the single source |

### rcache-server (11 files)

Sampled: server.ts, command.ts, store.ts, texts.json, test-action-result.ts, oracle.py, plus greps.

Pass: 71 of 71 JSDoc comments start with an emoji. texts.json has 32 keys, all in EN and DE. protobufjs is test-only. No compat code.

| Location | Rule | Sev | Note |
|---|---|---|---|
| server.ts (RemoteCacheServerStatusV1, GET /status document), store.ts, command.ts, collector.ts | schema-first | M | Public JSON and config types are code-first; add schema files |

### cli-latency (11 files)

Sampled: profile-module/🟨️.mjs, tests/🟦️.ts, files/door-light-test.ts, files/revision-own-stamp-test.ts, apply.py, schema.

Pass: EN+DE legend (REPO_PROFILE_LEGEND_V1). Schema present. Ajv and yargs-parser are test-only.

| Location | Rule | Sev | Note |
|---|---|---|---|
| apply.py:2 (ticket tooling) | emoji docstring | L | Module docstring without emoji lead; fix if copied |
| tests/🟦️.ts:77 | cross-platform | L | Test data uses `/usr/bin/vm_stat`, `/usr/bin/hdiutil`, `C:\\tools\\Python3.11.exe`; fine if gated per platform |

## 🌐️ Repo-wide checks

| Check | Result |
|---|---|
| `[DEBUG]` leftovers | None in all 13 sets |
| AGENTS.md edits | None in the 13 sets (a reference exists in another set, out of scope) |
| `repo dashboard` executed by staged code | No; `dashboard` only appears as a routed verb (repo-launcher) |
| Modifying git commands in staged sets | None found |
| Docstring emoji lead | Go 124 of 124 (repo-launcher, hot-cache); TS and JS: failures listed per set; Python: failures listed per set |
| Comments inside TS/Go definitions | None found. Python comments inside definitions: tool-iface after/0008, 0022, 0047 and polyglot oracle line 400 |
| New runtime external libraries | Rust providers (serde, serde_json, ide-free). Others: test-only oracles. Go: stdlib only |
| Exported external types | None found in the sampled exports |
| Path length, target paths | Max 120 chars, 165 UTF-8 bytes. Within limit |
| Path length, ticket staged copies | Max 235 chars (storage-seam), 225 chars (repo-launcher). In UTF-8 bytes: 307 and 297, over 256 |
| Schema-first | Present in repo-launcher, runtime-seam, storage-seam, pm-seam, backend-contract, hot-cache, polyglot-units, langs-conc, cli-latency. Missing or partial in graph-mem and rcache-server |
| Hard-coded absolute paths | macOS paths in hot-cache dev/bench, polyglot fixture, langs-conc test data (Windows paths in tests are fine) |

## 🧱️ Blockers before round 6 applies

1. tool-iface: remove the compat module `🔁️compat/🟦️.ts` (after/0069) and its barrel re-export (after/0075:8).
2. tool-iface: remove the `tolerated` spawn option (after/0074), the 592-row direct-tool-spawn ledger (after/0116) and its checker (after/0113, after/0114). Migrate the call sites instead. Same for runtime-seam's 76-row law ledger (⚖️law/📒️.json).
3. langs-conc: drop the stub bridges (`toolchains-stub-process.ts`, `toolchains-stub-framework.ts`, mapped by layout.py:38-39). Place one canonical toolchains module.
4. Raw `spawn("git")` in runtime code: backend-contract semio_scheduler.ts:23 and polyglot-units ⌨️command/🟦️.ts:302. Route through the tool-interface git adapter.
5. hot-cache dev/bench: replace macOS-only and `/tmp` paths (legacy.mjs:15, bench.mjs:10/12/14, replay.mjs:7/9, events.mjs:10, service-edit.mjs:18). apply.py does not fix them.
6. Legacy support: polyglot-units go/🟦️.ts `legacyConstraints` (lines 55 to 251), tool-iface after/0034 LEGACY_* recipe, after/0042 global polyfills. Remove, or justify each as required by correct semantics.
7. Emoji docstrings (cheap, MUST): hot-cache 10 JS and 9 Python; polyglot tests/🟦️.ts:133 and oracle.py (3); graph-mem oracle.py (2) and claims/oracle.py (1); storage-seam files/w/b67a32a67260.py:1.
8. Comments inside Python definitions: tool-iface after/0008:135, after/0022:19-20, after/0047:104. Move to the ticket doc.
9. Default language `"en"`: graph-mem memory.mjs:29 and hot-cache dev/resident/service/index.mjs:321, 360. Require an explicit language, no default.
10. i18n leaks in repo-launcher Go: command/🐹️.go:42, 51; workspace-root/🐹️.go:57, 63; runtime-registry/🐹️.go:125, 136; runtime-provision/🐹️.go:144, 382. Map to registry message IDs.
11. ide-free plan: 61 permanent deletes (duplicates matched by sha256). Get explicit user approval before apply.
12. Schema-first gaps: graph-mem memory.mjs policy and messages (make the schema the source); rcache-server public JSON and config types (add schemas).
13. Runtime dependencies: ide-free agent-client-text-5.txt:18-21 adds serde and serde_json to the providers crate. Justify, or wrap behind an interface.
14. Path length: ticket staged copies exceed 256 UTF-8 bytes (storage-seam 307, repo-launcher 297). Flatten the copy layout.
15. tool-iface: confirm the 120 migrated entries delete their old paths. The manifest has no removal records.

## 📝️ Non-blocking (medium and low)

- repo-launcher: new README.md (EN only); `dashboard` verb (confirm no agent use).
- tool-iface: keycap docstring (after/0089:85); ShimFileReader comment at after/0003:53 and 0004:56; Bun.spawn global monkeypatch in the guard (after/0074).
- runtime-seam: English-only error in serve.
- pm-seam: stacked docstrings in 27ff45c076c2.ts; keycap docstring at c1318bbfafca.ts:61.
- backend-contract: `/tmp/markers.log` in conformance_test.ts:26.
- hot-cache: ticket tools (apply.py, mirror.py, mutation-check.py, verify.sh) must be deleted from the ticket after apply; `legacy.mjs` naming.
- polyglot-units: cargo-metadata fixture with `/private/tmp` paths.
- langs-conc: English-only error at bazel-native.ts:7.
- cli-latency: apply.py docstring; platform-specific test data.

## 🔁️ Caveats

- Sample based: about 15 files per set, chosen by size and new or changed status. Repo-wide greps cover all files in each set.
- tool-iface, runtime-seam, pm-seam, and polyglot-units changed during the audit (tool-iface went from 136 to 166 manifest entries; runtime-seam from 23 to 25 files; polyglot-units from 18 to 20). Re-run this audit on the final snapshot.
- No build or test runs (per instructions). Test pass status is unverified.
