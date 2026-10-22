# 🔁️ Staged Wave 1021a Re-audit, Round 2 (read-only, aud-staged-1021a)

📅️ Snapshot: 2026-10-21, about 11:52 local. Nothing written except this file. No builds, no `bun repo`, no git-modifying commands, no sub-agents.

## 🗺️ Snapshot locations used (newest per set)

| Set | Audited location | Files | Newest mtime |
|---|---|---|---|
| repo-launcher | r6-repo-launcher-tools (manifest.json + blobs, sha256 verified 37/37) | 37 | 11:16 |
| tool-iface | tool-iface-tools/staged | 165 | 09:51 |
| runtime-seam | runtime-seam-tools/staged | 44 | 11:28 |
| storage-seam | storage-seam-tools/staged | 21 | 09:33 |
| pm-seam | pm-seam-tools/staged | 24 | 10:35 |
| backend-contract | backend-contract-tools/staged | 28 | 10:50 |
| hot-cache | round6-tools/staged/hot-cache | 105 | 10:31 |
| polyglot-units | round6-tools/staged/polyglot-units/staged | 25 | 10:26 |
| ide-free | ide-free-tools/staged | 56 | 09:32 |
| langs-conc | langs-conc-tools/staged | 19 | 09:32 |
| graph-mem | graph-mem-tools/staged (round6-tools/staged/graph-mem is older, 08:36, see N2) | 18 | 09:39 |
| rcache-server | rcache-server-tools/staged | 12 | 09:33 |
| cli-latency | round6-tools/staged/cli-latency/staged | 11 | 08:32 |

Not in scope of this audit: agent-session, perf-slo, cache-gc, legacy-purge (checked only where it touches tool-iface).

## ✅️ Blocker status (round 1 list)

| # | Blocker | Status | Evidence |
|---|---|---|---|
| 1 | tool-iface compat module and barrel re-export | RESOLVED | No `compat` path in manifest; barrel after/0061 has no re-export |
| 2 | tool-iface tolerated spawns and ledgers; runtime-seam law ledger | PARTIAL | Runtime-seam ledger removed (no `⚖️law/📒️.json`). Guard has no `tolerated` option. Production allowance is 0 (after/0112:100). Still open: test-scope ledger after/0115 with 589 rows, consumed at after/0112:98-104 |
| 3 | langs-conc stub bridges | RESOLVED | No `toolchains-stub-*` files in langs-conc-tools or round6 langs-conc; layout.py has no stub mapping |
| 4 | Raw `spawn("git")` in runtime code | RESOLVED | backend-contract semio_scheduler.ts has no spawn; polyglot ⌨️command has no `ls-files` spawn. Every remaining spawn of git/go/python/npm is in test code |
| 5 | hot-cache dev/bench macOS paths | RESOLVED | No `/private/tmp` or `/Users/ueli` in dev/bench. Residual test-only paths: N4 |
| 6 | Legacy support (polyglot Go, tool-iface 0034, 0042) | PARTIAL | Polyglot Go: legacy `// +build` removed, only `//go:build` now (go/🟦️.ts:68). OPEN: tool-iface after/0034 still has `LEGACY_ENGINE_BIN`, `ensureLegacyBuilt`, `generateLegacy`, `LEGACY_RECIPE` (lines 16-19, 64-66, 82-87, 100, 108-189). after/0042 still has `ShimFileReader` and `ShimProgressEvent` global polyfills (lines 60-76). legacy-purge-tools/src/edits.py:11, 16, 81 only renames them (`PATTERN_SHELL_ENGINE_BIN`, `generatePatternShell`) and moves the shim into `installOffBrowserGlobals()`; behavior is unchanged |
| 7 | Emoji-led docstrings | RESOLVED (residual L) | JS and Go: 0 failures in hot-cache, polyglot, graph-mem (current), tool-iface. Python: 0 failures in hot-cache, polyglot, graph-mem (current), storage-seam. Residual L: pm-seam new/w/c1318bbfafca.ts:61 keycap; tool-iface after/0088:85 keycap; cli-latency staged/apply.py module docstring (ticket tool) |
| 8 | Comments inside Python definitions | RESOLVED | No `#` comment lines in after/0008, after/0022, after/0047. Residual M: 0003:53 and 0004:56 comment still says "one shim block" |
| 9 | Default language `"en"` | RESOLVED for the audited files | hot-cache service index: no `?? "en"` (0 hits). graph-mem memory.mjs:29 gone. Residual L: `?? "en"` defaults in polyglot tests (tests/🌐️languages/🟦️.ts:28, tests/⌨️command/🟦️.ts:21). Open for N2 |
| 10 | repo-launcher i18n leaks | RESOLVED | No English reason fragments in blobs. Registry message `unexpected` is EN+DE (32 messages). Raw error text now only a `{reason}` parameter inside the translated template (L) |
| 11 | ide-free: 61 permanent deletes need user approval | OPEN | plan.json still has 61 deletes (extension 59, plans 1, leftovers 1). No approval evidence in the snapshot, so NO-GO until you confirm |
| 12 | Schema-first gaps | PARTIAL | graph-mem RESOLVED: memory.mjs reads `schema.definitions.Policy` and `Messages` (lines 2, 8, 12). rcache-server OPEN: schema exists only as rcache-server-tools/staged/schema-defs.txt, a JSON fragment in a .txt file, not a schema file |
| 13 | serde and serde_json in providers crate | RESOLVED | ide-free plan has no Cargo.toml write; no `serde = ` anywhere in ide-free staged |
| 14 | Ticket-copy path length | RESOLVED by characters | Max now 208 chars (runtime-seam), was 235 (storage-seam). UTF-8 bytes: runtime-seam 264 (L), others under 256 |
| 15 | tool-iface migrated entries and old paths | RESOLVED | Kinds now: edit 136, new 27, generated 2. No `migrated` entries left, so no old-path removal is required |

## 🚦️ New or residual findings

- N1 (H, tool-iface): after/0034 legacy engine and after/0042 global polyfills remain. legacy-purge only renames them. Remove them, or justify each as required (blocker 6).
- N2 (H, graph-mem): round6-tools/staged/graph-mem is an older copy (08:36) that still has the old violations: `staged/new/memory/module/memory.mjs:29` default `"en"`, 10 emoji-less Python docstrings, and `/private/tmp` and `/Users/ueli` paths (scale-bench.ts:4-5, verify.sh:2-6). If that copy is the apply source, the set fails. Delete it or refresh it from graph-mem-tools/staged.
- N3 (M, polyglot-units): round6-tools/staged/polyglot-units/staged/🧩️units/🧪️tests/🌐️languages/🟦️.ts:222 defaults `POLYGLOT_WIT_ORACLE` to `/private/tmp/...`.
- N4 (M, hot-cache): test assertions hard-code the Unix socket prefix `/tmp/semio-repo-1000/` (dev/go/tests/contract.go:78, dev/resident/tests/protocol.test.ts:122). Gate these to Unix hosts.
- N5 (L, tool-iface): after/0097 has a `[DEBUG]` temporary log (`t.Logf`). Remove it before apply.
- N6 (L, hot-cache): round6 hot-cache staged root contains ticket tooling (apply.py, mirror.py, verify.sh, mutation-check.py). Keep it out of the repo.
- N7 (L, round6 polyglot): units-cli.ts at the staged root spawns git. It is a scratch driver (its header says so). Keep it out of the repo.
- N8 (M, tool-iface): 589 test-scope ledger rows (blocker 2 residual). Migrate these call sites, or drop the ledger.

## 🟢️ Per-set verdict

| Set | Verdict | Reason |
|---|---|---|
| repo-launcher | GO | All checks pass on the blob snapshot |
| tool-iface | NO-GO | Blocker 6 open (N1). Blocker 2 partial (N8). Residual N5, 0003/0004 comment |
| runtime-seam | GO | Ledger gone; docstrings and tool spawns clean |
| storage-seam | GO | Python docstring fixed; path copies shorter |
| pm-seam | GO | Only the L keycap docstring |
| backend-contract | GO | Git spawn removed; no `/tmp` paths |
| hot-cache | GO | Follow-up N4 (M), N6 (L) |
| polyglot-units | GO | Follow-up N3 (M) |
| ide-free | NO-GO | Blocker 11: 61 deletes, approval not evidenced |
| langs-conc | GO | Stubs removed; layout clean |
| graph-mem | GO only if graph-mem-tools/staged is the apply source | N2: the round6 copy is stale and fails |
| rcache-server | NO-GO | Blocker 12: schema is a .txt fragment, not a schema file. Cheap fix |
| cli-latency | GO | L only |

## 🧾️ Caveats

- Sample-based for the docstring and comment checks in tool-iface (all 165 manifest bodies scanned by content, not by extension).
- Several sets were changing during this audit. The verdicts describe the snapshot at 11:52. Re-check after the final staging.
- Owners' READY claims were verified against the files, not taken at face value. Where they disagree, the file evidence above governs.

## 🔁️ Round 3 addendum (11:50 snapshot)
- (a) agent-session (edits.json, 68): NO-GO, Go test docstring `reportScript stands in…` (mcp/tests/build-session) lacks an emoji lead; 8 `prog` function edits carry no inline text, so content is unverifiable. perf-slo: NO-GO, oracle.py module docstring lacks an emoji lead (else clean). cache-gc staged3 (staged2 superseded): GO; its "ledger" is size bookkeeping and deletes are limited to cache roots.
- (a) legacy-purge (241 entries): NO-GO, after/0141:37-49 keeps a legacy `{x, y}` canvas wire through `bridge`; after/0173:59-60 says an engine shim survives; after/0161:22 is a dangling "Compatibility alias" doc; sampled ticket.json rewrites drop `started`/`finished`; 2 repo files under `🔨️modules/🎫️tickets/` changed after the manifest (stale apply).
- (b) tool-iface #6 + legacy-purge combined: NOT clean. Names are fixed (0034 becomes the real `pattern-shell` recipe, legacy-purge after/0188). The shim survives: the mesh generator (legacy-purge after/0194:53,56) still installs the globals, imported from `🧪️tests/🧰️support/…` into a production script. #6 stays OPEN.
- (c) ide-free confirmed: apply.py OPT_IN=[extension, manifests]; default deletes = 2 (plans-duplicate-1 `.cursor/plans/neural_schemas…`, leftovers `.github/hooks/repo.json`). Default apply GO; those 2 permanent deletes still need your go-ahead. The 59 extension deletes stay opt-in.
- (d) graph-mem apply source = graph-mem-tools/staged (09:39, clean); round6-tools/staged/graph-mem is stale and must not be applied.
