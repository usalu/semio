# gate-g queue (dry-run 1026 18:58, live tree, nothing applied)
6b (applied, laws 0 fail): win-withheld, upgrade-before-custody, keys3 -> course:build pending the quiet window.
Dry-run states: all fresh unless noted.
- rev3fix: 51 before / 9 absent (anchors present; needs upgrade-before-custody = applied).
- route2: 2 of 44 files CHANGED-SINCE-STAGING (graph closure-facts/json = peer added routerKeptFromSourceV1 + observedSourceAnalysisV1 entries; lib budget/headroom session.ts = only import order). Live is newer in both: staged copies moved to backups/route2-skipped, 42 apply.
- laws-g build-tools: --check ok (after route2).
- gram2 index-chunks 7 edits old / row-table 10 old / farm-listener status source+law true (already live).
- walk-g walks 29 old, footprint 26 old; share only repo README.md (anchored edits, apply sequentially).
- nxfix core 13 pending, nx-patch 2 pending; nxlock 10 pending + 2 needs-core (apply AFTER core).
- ORDER NOTE: nxfix core replaces whole 📓️build-guide.md (base sha must equal live) and testmem2 anchored-edits the same file: apply nxfix core BEFORE testmem2 (anchors verified present in core's bytes: 1/1); the reverse order makes core refuse.
- testmem2 9 base + 1 new; obsrows 10 base + 1 new; setup 12 anchors ok; jobsrv 9 NEW + 5 UNCHANGED-BASE; store managed-parent 8 anchors ok, foldsafe 14 unique.

## Group plan (20:25, rules: high-risk (graph/capacity/leases/unit-cache/key routing) max 3; low-risk disjoint up to 5; own verify green + revert tool each)
- G0 (running g4-course, 6b): GROUP A + GROUP 1 + GROUP 2.  [earlier attempts: g3 killed 19:36 externally, g3b died 20:06 on host ENFILE (module-not-found), g4 parked on 7168 MB reserve]
- G1: quietfc (+ revert set incl. bench2-g's 3 host-capacity files) [+ guard-g/purge-g when ready]  -> append 'GATE quietfc LANDED'
- G2: verify2 journal-fix -> verify2 vouch, toolsetup probes (probes first; conv-g then needs the afterconv merged witness copies)
- G3: guard-g/purge-g (P0), dashlock, graphlat plan-once (high risk)
- G4 (low risk, disjoint): oracle2 -> conv-g, xplat3 -> xplat4 (freespace, corpus, platforms), obs-g, setup-g, obsrows-g
- G5: route2 -> laws-g build-tools, gram3/index-chunks (cap issue: rev3fix after restaged), row-table, walks -> footprint
- G6: nxfix core -> nxlock -> (census2 xor testmem2), store-g after setup-g, jobsrv, rev3fix (planner laws on a mirror first, then land only on green)
- G7: nxfix nx-patch; G8: self-g sealed flip LAST.

## CHECKPOINT 22:20 (exact state; resume from here)
LANDED + product-GREEN: 6b = GROUP A + GROUP 1 + GROUP 2 (g4-course exit 0, 20:07-21:15).
LANDED live, graph-free laws green, NOT yet product-built (next session: ONE `course:build` verdict covers them all; if RED bisect by reverting half, revert tools below):
 1. purge-g purge (+docs purge): `python3 -I C/purge-g-tools/stage.py revert`; `python3 -I C/docs3-g-tools/staged/purge/stage.py revert`. Laws 0 fail except 2 documented baseline reds.
 2. verify2 journal-fix (+docs journal-fix): `sh C/verify2-g-tools/staged/journal-fix/revert.sh`; docs `.../docs3-g-tools/staged/journal-fix/stage.py revert`. Laws 6/1/1 pass.
 3. toolsetup probes (+docs probes): `sh C/toolsetup-g-tools/staged/probes/revert.sh`; docs probes revert. Laws 8/0 + 4/0. (conv-g's two 🔭️witness files now drift: use toolsetup-g-tools/mirror/afterconv merged copies when landing conv-g.)
 4. verify2 vouch + rev4-g vouch-anchor overlay (+docs vouch): revert = copy C/gate-g-tools/run/backups/g2/vouch-module-post-vouch.ts over 🪪️vouch/🟦️.ts, delete 🪪️vouch/🧪️tests/⚓️anchor/🟦️.ts, then `sh C/verify2-g-tools/staged/vouch/revert.sh` and docs vouch revert. Laws 7/9/15 + anchor 1 pass. (verified live, not on a mirror)
 5. oracle2 (re-staged 14 files) (+docs oracle2): `python3 -I C/oracle2-g-tools/stage.py revert`; docs revert. run-laws 20 pass 1 fail = 5 s default timeout of the 30k-file cover test under load 47 (environmental, rerun with --timeout).
REVERTED: quietfc (upgrade asks 28672 MB with 13 GB free, g5-course starved 25 min). Backup tar C/gate-g-tools/run/backups/g1/quietfc-before.tar; waiting for quietfc-g FIX-OF-RED batch; re-land via `sh C/quietfc-g-tools/apply.sh apply` (+ docs3 quietfc).
HELD: graphlat plan-once (rev4 MID, re-staging), rev3fix (needs planner laws on mirror + gram3 index-chunks restage; cap 65536 B conflict), red-g inferfix (RISK: land ALONE).
NOT YET LANDED (dry-run fresh, see list above): route2 (42 files; apply-staged.mjs --check) -> laws-g build-tools; setup-g, xplat3 -> xplat4 (freespace, corpus, platforms), conv-g (after oracle2, needs afterconv witness merge), obs-g fleet, obsrows-g, dashlock, jobsrv, store-g (after setup-g), gram2 row-table/index-chunks, walk-g walks -> footprint, nxfix core -> nxlock -> census2 xor testmem2 (nxfix core replaces the WHOLE build-guide.md: needs the docs3 nxfix-core variant instead since other guide edits landed), fdleak guard (after census2), sweep2, keys4 precision-rules, red-g batches, nxfix nx-patch (second to last), self-g sealed flip (LAST).
NEXT COMMAND: `python3 C/chain-tools/detach.py C/gate-g-tools/run/p15.out python3 C/gate-g-tools/product.py g6-course '@teaching/architecture-course:build'` (no product build is running; flatten-g may have moved some staged dirs: re-check paths).
