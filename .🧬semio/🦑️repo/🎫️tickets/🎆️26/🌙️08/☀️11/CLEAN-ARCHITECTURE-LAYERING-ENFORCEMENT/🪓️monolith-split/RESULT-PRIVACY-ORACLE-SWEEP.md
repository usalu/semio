# Result PRIVACY-ORACLE-SWEEP (prefix `pos`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command (read-only `git cat-file` only), no ticket tool. Tools: `pos-tools/`. Logs: `🗑️generated/pos-*`. Sandboxes (all writes): `scratchpad/pos-work/<family>/{cur,base}` outside the repo.

STATUS: IN PROGRESS (interim record, rewritten after each milestone).

## 0. Verdict so far

* `pos-tools/pos.py` is a reusable union oracle with a stub layer (section 1). It reaches body type-check (E0616/E0624 visible) for the families `rcr`, `phc`, `osm`; it does NOT yet reach it for `okernel`, `rhs`, `smg`, `ri2` (a rustc abort after coherence checking that I could not yet attribute; section 4). Those families are reported as "resolution-level only" until the blocker is found.
* Real carve defects found and fixed in `phc` (PLUGIN-HOST-CARVE): 73 privacy errors (E0616/E0624) reduced to 0, plus 4 more type-level defects (section 3).
* `rcr` (REPO-CRATE-ROOT-SPLITS, six roots): lib 850 = 850, test 928 = 928 errors against HEAD, 0 new, canary reached. Negative control detects a removed `pub(super)`.
* `osm` (kernel crate, OS-SERVICES-MCP-SPLITS): lib 106 = 106, 0 new; test 272 vs 27,477 (0 new).

## 1. Tool

(filled below)
