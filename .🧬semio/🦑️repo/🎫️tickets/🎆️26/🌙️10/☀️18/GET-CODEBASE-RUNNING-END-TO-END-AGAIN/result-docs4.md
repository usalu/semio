# result-docs4 (lane docs4, 10-27): the guides describe today's build behaviour

## ROUND 2 (14:35): G36-G43 documented, status refreshed

FOR coordinator: 7 doc batches ready in `C/docs4-tools/staged2/` (all on `📓️build-guide.md`, EN and DE in sync, base = live 14:30, guide sha e88fdd1b3edd, `check` = 7 x `result: pending`). Nothing applied live. Round 1 (`staged/`, 9 batches) is landed and untouched.
- check  : `sh C/docs4-tools/apply2.sh check`
- apply  : `sh C/docs4-tools/apply2.sh apply`   (or one: `python3 -I C/docs4-tools/staged2/<name>/stage.py apply [--root DIR]`; modes of the guide are kept, the law checks them)
- revert : `sh C/docs4-tools/apply2.sh revert`  (independent batches, any subset)
- rebuild after drift: `python3 -I C/docs4-tools/build2.py` (anchored on the live guide, exit 1 names the anchor)
- laws   : mirror `python3 -I C/mirror-tools/mirror.py create d4h --only "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md" "🧰️framework/🛍️products/🦑️repo/README.md" --no-closure --no-nm`, then `DOCS4_STAGED=staged2 python3 -I C/docs4-tools/verify.py <mirror root> 1 2 3 4 5`: GREEN, 0 failures (5 seeds, 11 batches = 7 + the 4 still-pending docs3 batches fleet, nxfix-core, nxlock, plan-once; bytes AND modes restored by revert; independent applier equal; drift refused; EN/DE parity; 46 claims found in the source tree).
- No product proof needed (docs only). Two statements depend on runs not finished at 14:30: `derived` (gate run g43: course edit served with the `build:graph-derive` child) and "verified in gate runs g34-g42" for the store directory. If g43 reverts memoedit `derived` or `memoout`, revert `memo-serve`.
- Note: `xplat5-tools/tests/test_stage_mode.py` tests the manifest-style stage.py; the docs engine (`docs4-tools/engine.py`, anchored edit lists) is a different tool that copies the replaced file's mode (`os.chmod` of the temp file), and verify.py asserts the modes are unchanged after apply and after revert.

| batch | where | content (latest landing groups) |
|---|---|---|
| `memo-serve` | "What happens on a run" item 2 (sub-bullet), "When it goes wrong" row | memo carry (`still describes generation`), reseal across generations, memoedit (`re-checked the N facts`, what is served and what still plans), derived facts in a `build:graph-derive` child (`asked again by a child process`), output directories (`.🧾️store`), the info lines are not warnings |
| `plan-walk` | item 5 bullets | git row digests only admitted files (staged ticket files no longer end a plan), excluded-name directories asked of `git check-ignore`, `.🧾️store` result of planfp3 now measured |
| `dashboard` | Rust lanes | dashboard no-op: installed binary returned when Cargo's dep-info holds (`semio.sources.json`, `installed.json` v4, profile policy in the digest, nested plan reusable), EN/DE line, 10.0 s / 1.15 GB |
| `rust-lanes` | Rust lanes, lock-order row | incremental workspace crates (print recompile 46-76 s to 8 s), lane quota ignores session bytes, selnear/witdelta/planmemo/nxmeta start-of-action savings, the two remaining lease-order pairs fixed |
| `disk-memo` | Disk budget | graph memo 1 GiB budget (EN/DE line), sweep of abandoned journal state directories (7 days / 10 minutes) |
| `sealed-capacity` | many agents, editing, troubleshooting | stale "default flip pending gate batch 8" replaced (`SEMIO_REPO_SEAL` opt-in, default still `live`, `SEMIO_REPO_IMPLEMENTATION` is the dashboard's `rust|go` selector); grown-reservation row now says the run keeps the jobs memory carries |
| `status` | Status, Measuring a build, memory table | the 10-26 header, the G34 note, the "Met since" and the 10-26 "Not met" baseline lines replaced by a target table with the 10-27 numbers (latest gate logs g34-g42 incl. peak RSS from `run/*.json`), a short 10-26 baseline, a refreshed pending-gate list, the measured line, the graph-builder row (355-owner refresh 10.3 GB) |

Already documented by landed docs (checked, nothing added): quietfc round 3 / capgrow / grownow (full-request bound, growing a reduced grant, compile cores, accounting), sealflip's `SEMIO_REPO_SEAL` bullet, jdir (round 1 `g34`), C1 enforcement and purge bullets, actenv (no change since round 1: 46 names, `check_runtime.py` LEAKS none at g33).
Not documented on purpose: hotpath2 keyparts diagnostic (`SEMIO_REPO_PLAN_KEY_PARTS`, only for debugging), audit4 gatelock/agedlight and audit2 mutual-grow internals, buildmem releasefirst, walk-views, peeredit internals (the user-visible effect is in the memo and status text).

Numbers used in the table (from `gate-g-tools/run/*.json`): g42-hot3 10.1 s 1062 MB; g41-dash3 10.0 s 1147 MB; g42-print3 15.1 s 1615 MB; g42p-edit 85.5 s 1929 MB; g42p-revert 15.2 s; g42c-edit 50.5 s 5179 MB; g42c-revert 40.3 s; g42-print1 599 s 10336 MB (355 refreshed); g42-dash1 95.5 s 4413 MB (38 refreshed); course edit 25.2/35.2 s from the g34c verdict in HANDOVER-1027b.md.

---

# Round 1 (landed in G35)

FOR coordinator: 9 doc batches ready in `C/docs4-tools/staged/` (8 on `📓️build-guide.md`, 1 on `README.md`; EN and DE in sync; base = live at 10:41, guide sha 7b768aa704cb, README be08003eff0f). Nothing applied live.
- check  : `sh C/docs4-tools/apply.sh check`   (all 9 print `result: pending`; exit 2 = drift, nothing written)
- apply  : `sh C/docs4-tools/apply.sh apply`   (order: graph-memo, plan-reuse, action-env, lock-order, capacity, rust-path, measuring, readme-bazel, g34; or any batch alone: `python3 -I C/docs4-tools/staged/<name>/stage.py apply`)
- revert : `sh C/docs4-tools/apply.sh revert`  (the batches are independent, so any subset can be reverted; revert `g34` together with a code revert of the G34 group (planfp staged3, hotpath2 rowid/jdir/memolist), the others with their code batches listed in each `README.md`)
- laws   : mirror `python3 -I C/mirror-tools/mirror.py create d4g --only "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md" "🧰️framework/🛍️products/🦑️repo/README.md" --no-closure --no-nm`, then `python3 -I C/docs4-tools/verify.py <mirror root> 1 2 3 4 5` (or `sh C/docs4-tools/staged/<any>/verify.sh <mirror root>`): GREEN, 0 failures (5 seeds, 26 batches = 9 docs4 + the 17 still-pending docs3 batches, mirror untouched afterwards, `mirror.py diff` 0).
- rebuild: if the guide moved between now and landing and a batch says DRIFT: `python3 -I C/docs4-tools/build.py` regenerates all batches from the live documents (exit 1 with the anchor when one is gone).
- product proof: none needed (documentation only). The text of `g34` makes claims that gate run g34 still has to prove (course revert replans nothing, memo hit after an in-place sibling edit, one journal service, no runtime-row door); if g34 reverts any of the four, revert the matching `g34` doc batch.

## What each batch adds (guide: EN section and DE section, same anchors; README: Bazel implementation bullets)

| batch | where in the guide | content |
|---|---|---|
| `graph-memo` | "What happens on a run" item 2 (sub-bullet), "When it goes wrong" row | when a repeated request is served without a graph child (light, sealed, third run), the key and what stays out of it, the served line and the `cannot serve (<reason>)` line (EN/DE texts as in the memo schema), the reasons `light changed pointer journal stale generation plan pin error`, `SEMIO_BUILD_GRAPH_MEMO=on|off`, profile marks `graph.memo` / `graph.memo-seal`, the macOS clone-event fix |
| `plan-reuse` | item 5 Bazel bullets, "When it goes wrong" rows, "Warm caches" table | measured result replaces the "not measured yet" sentence of the host-probes bullet; what keeps a plan (content edits after planfp, job count, handover env, ambient/agent env, diagnostic switches, slot choice after dashplan) and what ends it (witness kinds `l s d f k K g i e E x h`, key part, racy 3 s window); rows for `planning again: <reason>` and `cannot be reused later: <file> was modified moments before ...`; two warm-cache rows (TypeScript and Rust byte edits) |
| `action-env` | item 5 bullets, "Warm caches" table, "Troubleshooting" row | the action environment allowlist (classes, credential-like names withheld, 72 -> 46 names), how a task declares a variable (`{"env": NAME}` input row, `valueEnv`, `env` map; `declared-environment.json`; part of the key), `--skip-cache` to prove it, runtime rows keyed by stdout only (profile no longer re-keys) |
| `lock-order` | "Priority, leases and stuck holders" bullet, "When it goes wrong" row | the lease classes and ranks (pin 0 ... bazel-picker 9), `SEMIO_LEASE_ORDER=off|observe|enforce`, the `[lease-order]` line, the violation registry, which rules are enforced, how to read a warning |
| `capacity` | item 4 quiet follow-up sentence, "Troubleshooting" row | the stale "200% of default" sentence replaced by the bound and the hot-no-op numbers; row for `reservation grown from X MB to Y MB of the Z MB it asked for` (sole run grows to what fits) |
| `rust-path` | "Rust lanes" (new bullets) | Cargo preparation witness (no child unless an answer differs), compiler-writer lock order, admission memos for repository tests |
| `measuring` | new section "Measuring a build" before Status; Status bullet | a no-op is the third run, measure from one clean environment (what `product.py` drops), read the proof lines (`served from the memo`, `plan reused`, `INFO: 1 process`), `SEMIO_REPO_PROFILE` is safe, one-line edit probe, the 10-27 measurements |
| `readme-bazel` | README "Bazel implementation" (EN) and its German twin, guide pointer line | bullets "Plan reuse" and "Action environment", pointer lists the new section |
| `g34` | item 2 sub-bullet, journal section, item 5 bullets, Status | memo stamps for unread siblings (memolist), the journal's new place (jdir), runtime rows without a child (rowid), the plan walk skipping git-ignored directories (planfp staged3), Status note that the g34 verdict is pending |

## Laws (`C/docs4-tools/verify.py`, run on a mirror)
1. every batch is pending on the pristine documents, base hashes equal the staged ones;
2. five seeded random application orders, mixed with all 17 pending docs3 batches: all apply (exit 0), re-apply as a no-op, report applied; the sorted lines are equal for every order; reverting in another random order restores the original bytes;
3. independent oracle: a line-based applier written in verify.py (no code shared with `stage.py`) yields the same lines and length as the engine, per document;
4. drift: a moved anchor makes every batch refuse with exit 2 and leaves the file untouched;
5. EN/DE parity per edit: same ids, same SEMIO_ names, same digit runs (decimal comma, thousands separators and date order folded), same identifier spans, every table row exactly three pipes, same inserted headings;
6. claims: all 40 needles (every SEMIO_ name plus identifiers and message fragments such as `declared-environment.json`, `violations.jsonl`, `graph.memo-seal`, `bazel-picker`, `reservation grown from`) exist in the live source tree (`rg -F -f`, streamed; the guide and README themselves are excluded).
Conflict check against docs3 (pending batches bench2, fdleak-guard, jobsrv, plan-once ... replace lines in the guide): no `old` text of docs3 overlaps an anchor or `old` of docs4 (first draft collided on three lines: the ENFILE row, the Rust paragraph and the Graph item; the anchors moved to unmodified neighbours); docs4 never replaces a line another batch replaces.
There is no markdown, prose or catalog law for the guides in the repo (the language-catalog law scans source literals only); the only doc contract is the one verify.py enforces.

## Facts used and where they come from (so the next editor can re-check)
- memo: `🧠️memo/🧬️schema/🔣️.json` (texts, switch, denied env, reasons), `🚪️command/🟦️.ts` (marks, notice), result-boot-g §3, memokey/memokey2/memonotice, result-factsound, result-hotpath(2).
- plan reuse: `🔭️witness/🧬️schema/🔣️.json` (messages EN/DE, reasons, kinds, ignored/ambient env), `♻️plan-reuse`, result-racy-c, grantenv/ignoredenv/jobskey lines in coordination, result-dashplan, result-planfp, result-rustedit (universe).
- action env: `🌿️environment/🏃️action` schema, result-actenv (72 -> 46 names), result-envleak (keyed runtime rows, ambient policy).
- leases: `🏃️process/🔒️leases/🧭️order/🔣️policy.json` (classes, ranks, rules, texts), result-leaseord, result-cargofix.
- capacity: `🌡️host-capacity/🧬️schema/🔣️.json` (messages), result-quietfc2.
- numbers: coordination.md gate verdicts g29-g33 (hot no-op 10.0-10.1 s at 1.07-1.58 GB, course edit 30.5-35.2 s / revert 45-50 s, print edit 105 s / revert 20 s, dashboard 140-206 s).

## Not done / follow-ups
- The guide's older sections still carry 10-26 wording that this lane did not touch because pending docs3 batches replace the same lines (bench2 replaces the "Not met or not shown" status line; the Status header still says "Measured 10-26"). After docs3 lands, one more pass can merge the Status bullets.
- The guide has no section on `bun repo doctor` output after the lease/memo additions; not changed.
- Windows/Linux paths of the journal directory (jdir) are quoted from result-hotpath2, not run.
- README.md otherwise untouched; its long contract sections (Neutral model ... Bazel implementation) describe designs and were not rewritten.
