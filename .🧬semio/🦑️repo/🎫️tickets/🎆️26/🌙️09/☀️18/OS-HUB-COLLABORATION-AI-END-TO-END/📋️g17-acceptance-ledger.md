# G17 — Acceptance ledger, re-score over G14's 35+NEW rows plus session 7 (13:40→outage) and session 8's launch

Auditor G17 (Sonnet, high effort, read-only, no builds/servers/edits outside this file, no
sub-agents). Read in full: `📋️g14-acceptance-ledger.md`, `📓️status.md` from "### Session 7 — 13:40
actual" to EOF (line 688), and every report G14/status.md cite that postdates G14: S10, PZ1, U3, U3b,
C5, C6, C7, PR1, M8, M9, TC3, TC3b, TC3c, TC3d, DB2, DB3, HT13, HT14, HT15, FP4–FP10, JB1, FL2,
KN1–KN3, CE2, RB1, G15, G16 — all 34 exist and were read (headline/handoff + honest-gaps sections in
full; long measurement bodies skimmed for the exact numbers cited below).

**Method note — captures.** `ls 🗑️generated | grep <slice>` returns non-zero files for every session-7
slice above (counts recorded per-slice while building this ledger; e.g. s10 54, c5 60, pz1 15, db3
16, ht13 4 — all postdate the 2026-09-21 02:30 wipe G14 documented, so **all are real and
independently listable**, unlike G14's own S5–CE1 chain which G14 could only score OBSERVED-per-report).
G15 and G16 (pure audits) legitimately wrote zero `🗑️generated` files — verified, not a gap.
Session-8 slices (C8, S11, TC3e, CE3, CA1, HT16, FP11) each already have 1–3 capture files on disk,
confirming they are live and have started producing output, not just launched on paper.

**Method note — live ports**, `curl -s -m 2 -o /dev/null -w '%{http_code}' http://127.0.0.1:<port>/readyz`,
run at audit time:

| port | code | reading |
|---|---|---|
| 7611 | 000 | down |
| 7621 | 000 | **down** — this is C8's named target hub; either mid-restart or its holder died between launch and this read |
| 7631 | 000 | down (M8's hub, unheld since) |
| 7641 | **503** | up, not-ready — S11's target hub, alive |
| 7651 | 000 | down — TC3e's target, not yet bootstrapped at read time |
| 7661 | 000 | down |
| 7671 | 000 | down (DB3's Postgres/Neo4j hub, its docker stack was torn down — DB3 §8.7) |
| 6071 | **200** | up — S11's serve |
| 6081 | 000 | down (RB1's release-bundle serve target; bundle never built) |
| 6190 | **200** | up — C8's `s` serve |
| 6191 | **200** | up — C8's `gis2d` serve |
| 6196 | **200** | up — CE3's serve |

**This matters for scoring below**: C8's own hub (7621) answering `000` while C8's two serves answer
`200` means the ten-step scenario cannot be running against a live document *right now*, at the
moment of this audit, whatever C8's eventual report says — I score the collaboration-scenario row on
the chain of fixes up through C7 (all independently re-verified above) and flag the live 7621 outage
as a fact for the coordinator, not as a verdict on C8's unfinished work.

Status legend (unchanged from G11/G13/G14): OBSERVED-AT-RUNTIME / TEST-ONLY / COMPILED-ONLY / PARTIAL
/ CLAIMED-UNVERIFIED / OPEN. Where two reports contradict, both are cited.

## Outcome 1 — `dev s` frontend hosting every plugin/artifact

| item | G14 → NOW | evidence | what is still missing | owner session 8 |
|---|---|---|---|---|
| Cold, uncontended `dev s` completes; `plugins.length===60` | OBSERVED-AT-RUNTIME → **unchanged** | no session-7/8 evidence against it; S10/S11 both build directly on the same staged tree | nothing — closed | none |
| `capabilities_search` differentiated hits / catalog diagnostics | 2 diagnostics (puzzle, stdio) → **PZ1: stdio closed (§1, `describe` rc=0, 1 504 747 B), puzzle's cliff root-fixed (§2.3, capsule-dream dereference) but re-describe never completed a mutex hold (§4.4) — 1 diagnostic remains, `client-e2e` 30/32** → **CE2 found the SAME wfc/puzzle "cliff" was starvation not runaway (§5.3, a no-fuel-progress bound fixed in `🔌️plugin/🖥️host/🦀️.rs`), landed but COMPILE-UNVERIFIED (five cargo runs died in the flock pile-up, CE2 §5.4)** | re-describe under CE2's fix + verify compile; CA1's own 29 findings / 1 diagnostic baseline (`🗑️generated/ca1-audit-before.txt` exists, in progress) | **CA1** (running) |
| Foreign-kind spawn+mutate+undo+redo inside `s`, local | 22/35 → **S10: 27/35 confirmed at runtime after one guest rebuild (§2.7); three more roots (space, playbook-module-procedural, norm) fixed in source, staleness-proven by mtime (§2.8b), queued as `restage4` but not yet re-measured — ceiling ≤30/35** | S10 §2.5/§2.7/§2.8b/§2.9; **LIVE, this audit**: `🗑️generated/s11-sweep-a.txt` still shows `norm`'s `setSnapshot` refused and `playbook-module-procedural`'s presence refusal VERBATIM as S10's pre-rebuild readings — S11 is measuring against a guest that has not yet picked up `restage4` | S11's own restage + re-sweep; 4 still-unfixed FAILs (energy, sourcing declared HostOnly, stdio/trinity/writer — a third, undiagnosed `edits[0,0,0,0]`-on-`Artifact`-lane shape, S10 §2.7) | **S11** (running, live capture shows pre-rebuild state as of this read) |
| Foreign-kind open via the **hub catalog**, inside `s` | blocked mid-flight (S9 killed by the wipe) → **C5/C6/C7 chain: jco 1.34 confirmed live (C5 §4.0), the load-hang root-caused+fixed (C6, actor reaches `active` in 7.5 s), `Welcome.bootstrap` dead-code root found + 4/5 defects fixed at runtime (C7 §2) — 5th needs the gis2d guest rebuilt. Document still does NOT mount (C7 §0)** | C5 §0/§8, C6 §0, C7 §0/§5 — each next blocker found only after the prior one cleared, at runtime, on hub 7621 | the gis2d rebuild carrying defect 5; then the mount itself, never yet observed | **C8** (launched on 7621 + 6190/6191, but **7621 answers `000` at this audit's own curl check** — the hub is down right now, so no scenario step can be running against it this instant) |
| `block` boots and renders; `gis`'s four verbs dispatch | OBSERVED-AT-RUNTIME, closed (both) → **unchanged** | no contradicting evidence in session 7/8 | nothing — closed | none |
| wgpu renderer hub sign-in/spaces/workspace surface | PARTIAL, UNOWNED → **unchanged, still UNOWNED** | absent from `📓️fleet-7-agents.md` (G14) and absent from the session-8 roster (C8/S11/TC3e/CE3/CA1/HT16/FP11 — none touch wgpu) | rerun the 7 written laws; observe end to end | **UNOWNED**, third consecutive audit |
| wasm32 browser wgpu shell lazy-install + foreign-kind open | PARTIAL, UNOWNED → **unchanged, still UNOWNED** | same | a live lazy-install + open run | **UNOWNED**, same reason |
| `MutationKind::label()`'s 2690 call sites localized | assigned, unstarted → **OBSERVED-AT-RUNTIME** | U3 codemod (2795 sites, 2791 files, §3); U3b whole-tree green (102/106 crates natively + wasm32 + framework `--all-targets`, §1–§3); **S10 §2.10 photographed the History panel in BOTH locales on the rebuilt guests** — `EN rows [...] / DE rows [..., "Panel-Tab wechseln"]`, non-empty in both, first live confirmation | one new bug found live: a locale switch does NOT re-render rows already in the ledger (S10 §2.10 item 2, named not fixed); English prose (~324 kebab-case ids) untouched; 4 peer-owned crates still red (U3b §7.1) | **UNOWNED** for the re-render bug — no session-8 slice names it |

## Outcome 2 — hub backend with db/presence/auth

| item | G14 → NOW | evidence | what is still missing | owner session 8 |
|---|---|---|---|---|
| `cargo check -p semio-hub` green | COMPILED-ONLY, closed → **unchanged, reconfirmed ~8 more times** | TC3b (0 err, 338 warn), TC3c (0 err, 338 warn), PR1 (`--all-targets` ✅), M8/M9 (0 err each), HT13/HT14 (0 err, 34/34 targeted laws), DB2/DB3 (`--features postgres,neo4j[,native-artifact-execution]` all green) | every one of these needs a fresh `semio-hub` binary before its runtime claim is real; several are STILL untested against each other on one binary | HT16 (rebuild + rerun) |
| `cargo test -p semio-hub` green, exact number | 318/3 → 319/2 best-of-day, rerun in flight → **HT13 fixed the sync-hello lost wakeup (04:03-area red), HT14 fixed all 22 post-TC3b-genesis reds (34/34 targeted laws green), s7g 327/327 (16:38), a regression to 4 inference reds at s7h (21:41) was root-caused+fixed by HT15 (wall-clock-vs-work-bound), rerun s7i = 327/327 green at 02:01 2026-09-22 — the LAST coordinator-confirmed number** | HT13 §7, HT14 §0/§3, HT15 §7/§9, `📓️status.md` line 671 | s7i predates PR1/M8/M9/TC3c's hub-fence edits, KN3, and FL2's replication fix — **none of those has been proven against a suite run yet**; hub was down 04:17→10:52 with every in-flight rebuild cut | **HT16** ("hub startup stall-bound + frontier identity + hub suite" — exactly this) |
| `os-hub:test` wall-clock budget | UNOWNED → **still UNOWNED, 7th consecutive audit** | no report since G11 measures it | an actual timed run | **UNOWNED** |
| `dev` boots to `/readyz` all-true | OBSERVED-AT-RUNTIME, closed → **unchanged** | no contradiction | nothing — closed | none |
| `POST /auth/sessions` mints; revokes; rate-limited | OBSERVED-AT-RUNTIME, closed (all three) → **unchanged, DB3 independently reconfirms rate-limiting on a Postgres-backed hub** (§5a row 9: 429 + positive retry-after) | DB3 §5a | nothing — closed | none |
| Hub emits structured trace for WS handlers + directory command path | PARTIAL, UNOWNED → **OBSERVED-AT-RUNTIME — G16 re-audited this from scratch and found it real**: `semio-framework-trace` 49/49 unit tests, wired into WS/directory/auth/credential/rate-limit paths, **live counters observed on a real booted hub** (`server.artifact.maintenance`, `server.auth.session.mint`, etc., G16 §h) | `server.auth.agent.*` spans emitted but not yet listed in the vocabulary fixture; `checkpoint-publications` decision untested at runtime; `🌎️hub/README.md:543` still falsely claims "no metrics, no request tracing" (G16 §h, stale-doc drift) | **UNOWNED** — no session-8 slice re-verifies this or fixes the README drift, but the underlying gap G14 carried as OPEN/UNOWNED is actually CLOSED |
| Postgres and Neo4j backends actually run once | nobody, recorded USER decision → **OBSERVED-AT-RUNTIME — DB2 + DB3 closed this, the single biggest closure in outcome 2 this window.** DB2: both directory lanes proven against real servers, three defects found+fixed (D1 projection-rebuild identity loss, D2 no deadlock retry, D3 silent identity loss). DB3: **both halves on Postgres, 42 PASS / 0 FAIL** incl. a `SIGTERM`+restart with membership surviving byte-exact; **first-ever Neo4j-backed hub booted** (41 PASS/1 FAIL, the 1 being a reason-code non-defect); 39 real DB objects created and verified with `\dt` | DB3 §8: no artifact was ever CREATED on the Postgres hub (no trusted catalog published there — publishing into Postgres is a real chunked-CAS write, not a copy job); `OS_HUB_STORAGE_BACKEND=neo4j` (documents-on-Neo4j) never booted; `dev postgres` nx route never driven end to end (only the boot script) | **UNOWNED** — no session-8 slice continues DB2/DB3's work |
| `HubInstance` durable stores wired to a real route | OBSERVED-AT-RUNTIME, unchanged → **unchanged, reconfirmed by every sign-in in C5/M8/M9/PR1/DB2/DB3** | — | sagas still register no deciders | none |

## Outcome 3 — collaboration between users over the hub

| item | G14 → NOW | evidence | what is still missing | owner session 8 |
|---|---|---|---|---|
| Trusted catalog publishes; `artifactAuthority` ready | OBSERVED-AT-RUNTIME, closed → **unchanged, and now generalised**: TC3/TC3b/TC3c/TC3d landed catalog-carried genesis — **any** plugin kind is creatable via the guest's own `codec.genesis`, not only the two hardcoded native codecs (TC3 §1.4/§2, TC3b §0/§1, TC3c §0 built stdio+gis+note cold and staged all three) | TC3c died at 16:14 on a guest `codec.genesis` trap for `note` (root: every codec app of a >1-app bundle was constructed-then-dropped, tripping a terminal-empty-store assertion); **TC3d root-fixed it, type-checked native+wasm32, but the fix was never proven at 7651 — the mutex queue never let the driver hold long enough (TC3d §5, "NOT REACHED")** | 7651 bootstrap + note-creation proof; sweep stdio/gis too (TC3d only asked note, §6b) | **TC3e** ("stdio/gis/note hub 7651 + note creation" — exactly this) |
| Two distinct hub-authenticated identities | OBSERVED-AT-RUNTIME, closed → **unchanged, reconfirmed in C5/C6/C7/M8/M9** | — | nothing — closed | none |
| `collabRunScenario`'s 10 steps, real numbers | PARTIAL, C5 running with 0 measured → **STILL 0/10, but the single blocking chain moved three more links: C5 (jco confirmed live, load-hang found) → C6 (load-hang fixed, actor reaches `active` 7.5 s, next blocker = `Welcome.bootstrap` never produced) → C7 (bootstrap dead-code root found, 4/5 defects fixed at runtime, 5th needs a guest rebuild). Document still does not mount (C7 §0: "NO, not yet claimed")** | C5 §0/§5 "0 of 10 — not run, nothing claimed", C6 §4 "NOT RUN", C7 §3 "0 of 10 — not run" | the gis2d guest rebuild carrying C7's 5th fix; then the mount; then all 10 steps | **C8** — but **hub 7621 answers `000` at this audit's live curl check**, so whatever C8 has done since launch is not reachable from outside right now |
| Connection loss / two-writer convergence / per-user crossing undo, as scenario steps | transitively blocked on C5 → **unchanged, transitively blocked on C8's still-unproven mount** | C5 §8, C7 §5 — the gate itself is deliberately NOT wired (≥8 green steps required, 0 are green) | same as scenario row | C8 |
| Presence: two distinct session colours, symmetric and stable | observed once, unstable → **PR1 root-caused BOTH defects and fixed them in source: asymmetry = a join-replay gap (a late joiner gets 0 presence frames until a peer's bytes next change, PR1 §0.1); decay = an unbounded client beat + a 15 s hub TTL erasing a still-open socket (§0.2). A THIRD finding: the `agent` roster row was never missing, just never reached — OBSERVED for the first time (§5.3, browser-free)** | PR1 §0/§5.1-5.3 | **the hub half of both fixes is compile+law-proven ONLY — not observed live.** Every running hub (7611, 7621) executes a binary built BEFORE PR1's fix (PR1 §6, explicit: "rule 26 forbids this slice from building semio-hub"). No browser reaches a document socket on PR1's own catalog either (§2) | **HT16**'s hub rebuild would carry this fix for the first time; no session-8 slice re-verifies presence itself |
| wgpu native shell collaboration path | UNOWNED → **unchanged, still UNOWNED** | absent from session-7 and session-8 rosters both | a second wgpu session in one space, observed | **UNOWNED**, third consecutive audit |

## Outcome 4 — AI integration over the semio MCP

| item | G14 → NOW | evidence | what is still missing | owner session 8 |
|---|---|---|---|---|
| `.mcp.json`'s `semio` server answers `initialize`/`tools/list`/`resources/list` | OBSERVED, closed → **unchanged** | reconfirmed transitively by CE2's gates | nothing | none |
| `capabilities_search` / catalog diagnostics | same row as Outcome 1 | — | — | CA1 |
| Undeclared gesture routes (puzzle×3, block×1, writer×1) | PARTIAL, block/writer UNOWNED → **unchanged**: PZ1's scope was puzzle+stdio+wfc only (§0), never touched block/writer | PZ1 §0 | block/writer's 2 routes | **UNOWNED**, unchanged since G14 |
| Full mutation chain e2e (`prepare→invoke→snapshot→undo/redo→rollback→export`) | OBSERVED-AT-RUNTIME, closed → **unchanged, further reconfirmed**: CE2's `live-agent-loop-check` **21/21** live against a real `s` shell on 6196 (CE2 §0/§3.2), superseding S7's own 21/21 with a cleaner run (no (e3) race) | CE2 §3 | `client-e2e` itself regressed to **5/6** purely because CE1's freshness gate fail-closes on the FIRST step once wfc/note components are rebuilt without a fresh describe (JB1 §5, CE2 §1) — the mutation-chain claim itself is not what regressed | **CE3** ("all four MCP gates") |
| `wfc`'s `job.explicit-state-machine-required` refusal | named, not fixed → **JB1 fixed the actual root far beyond wfc**: the opaque-future job path was DELETED tree-wide; all 5 builtins (incl. `semio.infer`) are explicit bounded state machines now, in EVERY production build, not just test (`--lib jobs` 39/39). wfc's OWN descriptor went stale as a side effect of the rebuild it needed (JB1 §6.1) | JB1 §0/§6.1; CE2 §5 found the "cliff" was starvation, not a real wfc defect, and landed a no-fuel-progress fix — compile-unverified | wfc + note re-describe under CE2's fix; verify compile | **CE3** |
| A destructive-capability approval, live round trip | OBSERVED-AT-RUNTIME, closed → **unchanged, and the underlying count kept growing**: destructive-flagged actions now number ~50 files / 25 descriptors (G15 #1, re-verified from source, git-diff-confirmed) | G15 #1 | nothing on the round-trip claim | none |
| React shell renders live agent tool-call transcript | OBSERVED-AT-RUNTIME, superseded → **unchanged, superseded again by CE2's cleaner 21/21** | CE2 §3.2 | nothing | none |
| MCP agent edits inside a real hub space, human sees it | PARTIAL, owned-unstarted (M8) → **PARTIAL, materially advanced, 2 of 3 gates open**: M8 opened D1 (kind no longer refused) and D3 (bytes off the verified canonical pair) live against hub 7621's real `gis.map` document (M8 §5.1/§5.2); **M9 closed D2 partway**: a hub-bound gateway now fetches+hash-verifies+compiles the HUB-authorized component (M9 §0/§2.3, live) and stops one step further in, at `wasmtime` instantiate, on a **tree-wide** world-export boundary (TC3b's `codec` export in `world actor`, landed in source but not yet carried by ANY built+republished component on 7621) | M8 §0/§5, M9 §0/§2.4/§7 | `features.mcpWorkspace` compiled-not-observed (needs HT16's rebuild); the LoadDocument-of-canonical-pair step + socket-grants (M8 §8.1 steps 3-4, untouched); a rebuilt+republished 7621 catalog carrying TC3b's world (the SAME sweep TC3e is running for 7651, but 7621 specifically is nobody's) | **M9's own next step is unowned in session 8** — no slice named continues D2's dispatch chain or the LoadDocument step; TC3e rebuilds 7651, not 7621 |
| `resources/subscribe` | OBSERVED-AT-RUNTIME, closed → unchanged | — | crate's own unit-test lane still not obtained | none |

## C. Items no running or freshly-assigned session-8 slice owns — UNOWNED, ranked by blast radius

Ranked by how much of the four outcomes each one blocks, highest first.

1. **wgpu renderer — outcome 1 rows 8/9, outcome 3's native-collaboration row.** Third consecutive
   audit (G13, G14, now G17) with zero session touching it. Blocks a whole rendering surface across
   two outcomes simultaneously; nobody has even re-confirmed G13's 7 written laws still pass.
2. **M9's own next step: the dispatch chain past `wasmtime` instantiate, on a rebuilt+republished
   7621.** This is the single largest remaining distance in outcome 4's headline claim ("an MCP agent
   edits a hub document and a human sees it") — M9 itself calls it "the whole remaining distance,"
   and it is the same wasm-mutex-sweep-on-a-specific-hub problem C8/TC3e are independently fighting
   for 7621/7651, but nobody's stated scope is "finish M9's D2 on 7621."
2b. **7621 is down right now** (this audit's own `curl` — `000`, not `503`), which is C8's stated
   target port for the ten-step scenario. Whatever C8 has measured since launch cannot currently be
   reproduced or extended by anyone until 7621 (or an equivalent hub) is back up.
3. **`os-hub:test` wall-clock budget.** Unowned across seven consecutive audits (G11 through G17) —
   the longest-standing item on this whole ledger.
4. **PR1's hub-side presence fix, unverified live.** Both root-caused defects (asymmetry, decay) and
   the first-ever `agent` roster row are compile+law-proven only; every running hub predates the fix.
   HT16's rebuild would carry it, but HT16's own stated scope (hub startup stall-bound + frontier
   identity + hub suite) does not mention re-running PR1's own probe against the fresh binary —
   someone needs to.
5. **DB2/DB3's continuation.** No document was ever created on a Postgres-backed hub (publishing a
   trusted catalog into Postgres chunk storage is real, unstarted work); `OS_HUB_STORAGE_BACKEND=neo4j`
   never booted at all. Outcome 2's headline claim ("Postgres and Neo4j backends actually run") is now
   materially true for the directory+session half and still open for the artifact-publication half.
6. **block/writer's 2 undeclared gesture routes** (of the 5 G14 named). PZ1's scope was explicitly
   puzzle+stdio+wfc only; nobody has ever picked up the other two, across three audits now.
7. **The locale re-render bug S10 found live** (§2.10 item 2): switching locale does not re-render
   rows already in the History ledger. Freshly discovered this window, zero owners.
8. **`semio-hub`'s structured-trace README drift** (G16 §h): `README.md:543` still claims "no
   metrics, no request tracing," which is now false and actively misleading to an operator. Trivial
   fix, zero owners — the underlying capability G14 scored UNOWNED is actually done; only the
   documentation lie remains.
9. **`build-s-react-release`, G15 item #7.** RB1 spent its slice queued behind the fleet mutex for
   the entire session and never got a bundle out (run 1 failed on peer churn at 33/99 tasks; runs
   2–7 were repeatedly killed/requeued by mutex churn and the account outage; "RB1 run 7 (never
   started)" at the 04:17 outage, `📓️status.md` line 685). This is the single remaining button-press
   for outcome 1's release-distribution claim and nobody in session 8 is pressing it.
10. **The stdio/gis `codec.genesis` sweep TC3d itself named as unowned-by-it** (TC3d §6b): TC3d's fix
    was proven only against `note`'s trap; stdio and gis have linked native codecs so the trap never
    fired for them, but nothing has DRIVEN `codec.genesis` against their components to confirm the
    shared resolver fix actually covers them too. TC3e's stated scope (stdio/gis/note hub 7651 + note
    creation) may incidentally cover this — worth confirming explicitly rather than assuming.
