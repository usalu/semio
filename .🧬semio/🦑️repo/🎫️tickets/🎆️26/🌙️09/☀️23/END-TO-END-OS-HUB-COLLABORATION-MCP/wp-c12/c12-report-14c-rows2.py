"""🧾️ C12 14c (late): appends rows 8–9 and the log lines after the 22:42 reboot to 📓️wp-c12.md (idempotent)."""
p = "/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-c12.md"
s = open(p, encoding="utf-8").read()
anchor = "**Session 14c log**"
rows = """| 8 | STEP 15 on 8010 (H13 binary 2155, declared-batch fix) | full harness `c12collab-8010-en2` (22:51): **7/15** again (1,2,3,4,5,7,12); STEP 15 "the 5000 ms cut rebuilt the document" — contaminated by STEP 13's concurrent whole-text SETs (the worker's reorder backstop rebuilds). Standalone probe `c12short-8010` (`wp-c12/c12-shortage-run.sh`: proxies 8023/8024 → 8010, serves 6525/6526 via `ensureDevServe`, A types alone, en + de): **5 s cut: all PASS** (no freeze — worst frame 25/32 ms, en/de link state, convergence, every keystroke in both editors, **no rebuild**); **15 s / 60 s: FAIL** — no freeze and localized states hold, but both clients rebuild ("fresh authoritative restore", A 3/7×, B 3/4×), A's pill "Pending (64)" drains, user2's browser actor child faults on a resumed hub frame (`malformed hub frame … invocation rejected: invoke reactor/poll: Error: [object Object] (see error.payload)`), the editors converge EMPTY (15 s) or B stays empty (60 s); user1 also hit a pageerror `SyntaxError … "A " is not valid JSON` after a 500 from its dev serve (URL not captured). Next: read the child's payload (row 9 patch), then root-cause the ≥ 15 s rebuild in the worker (resume refused → reopen path) |
| 9 | Guest fault payload lost in the child | prepared `wp-c12/child-payload/c12-child-payload-patch.py` (browser bundle → L1's train): `childRejectionReason` carries a component-model error's typed `payload` (JSON; bigints decimal, bytes as length; bounded) instead of "[object Object] (see error.payload)"; law case added; dry run 3 hunks / 0 problems; projection checked on the law's own input (`child-payload/check.ts`). SeedHistory law amended per H13: a local edit before AND after the tail (STEP 8's mixed shape: 15 remote + 2 local); re-dry-run 15/0 |
| 10 | collab de | `c12collab-8010-de2` (23:16, load 63): **1/15** — STEP 2's share dialog timed out (30 s), everything after it skipped/failed; under that load not a product verdict. de on 7800 (Home fix) and a de re-run on 8010 wait for load < 40 (coordinator rule) |

"""
if "| 8 | STEP 15 on 8010" not in s:
    assert s.count(anchor) == 1
    s = s.replace(anchor, rows + anchor)
log_anchor = "**Window-3 runbook (C12 set)**"
log = """- 22:42 machine reboot (every process died, incl. `c12collab-8010-de`). Torn-write check: every C12 script dry-runs clean, every edited tree file parses (`bun build --no-bundle`), JSON fixtures parse — nothing torn.
- 22:4x SeedHistory set (`wp-c12/seed/`), relayed; H13 approved + amendment (mixed local/remote shape) applied. 22:51 `c12collab-8010-en2`; 23:05 `c12short-8010`; 23:16 `c12collab-8010-de2`. Serves/proxies/browsers of every run stopped by their wrappers (ports 6520–6526, 8023–8026 free at 23:30).

"""
if "- 22:42 machine reboot" not in s:
    assert s.count(log_anchor) == 1
    s = s.replace(log_anchor, log + log_anchor)
open(p, "w", encoding="utf-8").write(s)
print("ok")
