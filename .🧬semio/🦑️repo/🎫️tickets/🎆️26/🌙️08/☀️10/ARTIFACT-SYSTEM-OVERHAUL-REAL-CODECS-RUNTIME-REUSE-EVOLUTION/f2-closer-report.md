# F2 — C2 Closer Report

Wave: F2 (5 standards — stl/ascii, obj/3.0, ply/1.0, las/1.0, bmp/v3; tiff deliberately deferred,
live external edit). Role: C2 closer — the only F2 agent authorized to touch `📦️glue.rs` and
`📜️script.ts`.

## 1. Inputs read

All 5 fan-out reports (`f2-stl-report.md`, `f2-obj-report.md`, `f2-ply-report.md`,
`f2-las-report.md`, `f2-bmp-report.md`) and the independent verification report
(`f2-verify-report.md`), all in this ticket folder.

## 2. `glue_followup` items applied

**None requested a new top-level directory or a `📦️glue.rs` mount.** All 5 fan-out reports
confirmed (per S2's Task 1 resolution) that every real diff/mutation/absorb rewrite fit inside
already-mounted `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs`,
`⚙️engine/🦀️component.rs`, and sibling facet leaves. `glue.rs` was not touched.

ply's and las's reports flagged stale `📜️script.ts` policy-allowlist entries (not `glue.rs`
edits) for this closer to prune — handled in §5 below, alongside the other 3 artifacts' equally
stale entries discovered by this closer's own cross-check.

## 3. Closer-found-and-fixed defect (1, real, own-code, found during this closer's own
verification — not flagged by any fan-out or verify report)

While cross-checking `git status` for new/untracked directories (§6), this closer found two
untracked stray files inside ply's own tree:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🅰️component.g4`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔤️component.ebnf`

Both contain the real, handcrafted PLY 1.0 grammar content ply's fan-out report describes — but
at the WRONG path. The correct path (matching every sibling leaf's convention) is one level
deeper, under `📸️snapshot/📝️text/`. At that correct path, the actual `🅰️component.g4` and
`🔤️component.ebnf` files were still the OLD scaffolded placeholders (`grammar Stdio_ply_snapshot;
DOCUMENT: 'schema' [ ]+ 'stdio.ply' ;` / the equivalent 3-line ebnf stub) — despite ply's own
report (§1, "Grammar leaves") explicitly claiming "the snapshot facet's full 6-file set
(text: g4/ebnf/grammar.semio; binary: ksy/spicy/abnf/protocol.semio) is handcrafted honestly."
This was a real authoring-tool path mistake (the two files were written one directory level too
shallow), not a deliberate scope cut.

**Fix applied**: overwrote the correct `📝️text/🅰️component.g4` and `📝️text/🔤️component.ebnf`
with the real content from the misplaced stray copies, then deleted the two stray files. Diffed
byte-for-byte before deleting to confirm no content was lost.

**Verification**: `cargo test -p semio-s-plugin-stdio --lib "artifacts::ply"` → still 23 passed, 0
failed after the fix (grammar leaves aren't exercised by Rust tests, so this couldn't have broken
compilation, but re-ran anyway as a sanity check). `bun ./📜️script.ts policy`, re-run
immediately after: both leaves flipped from "real, unallowlisted, would be a `missing` breach if
not for an allowlist entry that happened to already cover the OLD placeholder path" to correctly
`-stale-` (fixed, allowlist entry pending removal) — confirming the fix is real and detected by
the same S-8 grammar-honesty checker every other leaf in this wave was graded by. Pruned in the
same pass as the other 49 pre-existing-stale grammar-honesty entries (§5).

## 4. Full-crate gate (`cargo test -p semio-s-plugin-stdio --lib`)

**Final, real, on-disk result: 795 passed, 0 failed, crate-wide.** Ran this gate twice: once
before the ply grammar-leaf-path fix and the `📜️script.ts` allowlist edits (795/0, matching the
verify agent's own independent run exactly), and once after (795/0 again — confirming neither the
ply fix nor the allowlist edits caused any regression, expected since grammar `.g4`/`.ebnf` files
and `📜️script.ts`'s TypeScript allowlist arrays are outside the Rust crate's compilation unit
entirely).

Per-artifact filter, independently re-run by this closer (not just trusting the fan-out/verify
reports' own numbers):

| artifact | tests | result |
|---|---|---|
| stl | 21 | 21 passed, 0 failed |
| obj | 17 | 17 passed, 0 failed |
| ply | 23 | 23 passed, 0 failed |
| las | 21 | 21 passed, 0 failed |
| bmp | 14 | 14 passed, 0 failed |
| **total** | **96** | **96 passed, 0 failed** |

Unlike F1's closing session (which hit ~15 minutes of external-wave compile blockage), this
closer's own gate runs never saw any external-wave interference — the concurrent "subset
multiplicities" wave (docx/ifc/jpg/pdf/tiff/xlsx, plus tiff specifically per this ticket's own
note) had already settled into a compiling state by the time this closer ran its own gates. The
795/0 result matches exactly what the independent verify agent (`f2-verify-report.md`) reported
before this closer started.

## 5. Policy shrink (`bun ./📜️script.ts policy`)

Ran the real policy check (not `verify`) and cross-checked the regenerated
`.🦑️repo/⚡️cache/breaches/compose.json` directly (not just the CLI's priority-filtered stdout,
since low-priority stale-allowlist breaches don't print by default) — same methodology F1's
closer used.

### 5.1 Before this closer's edits

Filtering `compose.json` for the 4 S-8 rule kinds
(`stdio-artifacts/{diff-algebra,field-sweep-presence,grammar-honesty,facet-mirror-drift}`) scoped
to `🟪️stl`/`🧊️obj`/`☁️ply`/`☁️las`/`🖼️bmp`: **59 breaches, every single one `-stale-`** (low
priority) — 5 diff-algebra + 5 field-sweep + 49 grammar-honesty. `facet-mirror-drift` showed **0**
hits (neither real nor stale) for all 5 artifacts — not investigated further, since F1's own
closer already root-caused this rule's checker to two structural false-positive sources (test-code
identifier pollution, proto's idiomatic snake_case never matching a camelCase substring search)
and explicitly declined to touch its allowlist; nothing in this wave's data contradicted that
finding, so it wasn't re-litigated.

**Zero real (non-stale/"missing") breaches existed for F2 even before this closer's edits** — net
of the one ply grammar-leaf-path defect found and fixed in §3 (which, once fixed, immediately
surfaced 2 *more* now-stale entries, pruned in the same pass as the other 49).

### 5.2 Allowlist edits applied to `📜️script.ts`

All edits were scoped precisely to the line range of each rule's own allowlist array literal
(`POLICY_DIFF_ALGEBRA_ALLOWLIST` lines 8546-8571, `POLICY_FIELD_SWEEP_ALLOWLIST` lines 8614-8639,
`POLICY_GRAMMAR_HONESTY_ALLOWLIST` lines 8703-9253, before edits) — **not** a global
string-replace. This distinction mattered in practice: an early attempt at a global replace over-
removed 5 extra lines, traced to the fact that `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` (a 4th,
untouched array) independently contains identically-formatted key strings for the same artifacts
(e.g. `"stdio/las/standards#1.0-subsets-any-schema-diff-component"` appears once in
`POLICY_DIFF_ALGEBRA_ALLOWLIST` and once, coincidentally, in
`POLICY_FACET_MIRROR_DRIFT_ALLOWLIST`) — a global match would have silently deleted entries from
the wrong (untouched, per F1 precedent) allowlist. Caught this before writing, verified with a
targeted `grep -c`/`grep -n` cross-check, and switched to the line-range-scoped removal that
correctly split 59→(5,5,49) and, after the ply fix, 61→(5,5,51) across the right 3 arrays only.

- **`POLICY_DIFF_ALGEBRA_ALLOWLIST`**: removed 5 F2 entries (bmp/las/obj/ply/stl, one each — all
  now implement `DiffAlgebra`, confirmed `-stale-` by the breach cache).
- **`POLICY_FIELD_SWEEP_ALLOWLIST`**: removed 5 F2 entries (same 5 — all now have a passing
  `field_sweep`-named test).
- **`POLICY_GRAMMAR_HONESTY_ALLOWLIST`**: removed 51 F2 entries total — 49 confirmed `-stale-`
  before this closer's own defect fix (stl 11, obj 12, ply 7, las 7, bmp 12), plus 2 more that
  became stale only after the §3 fix (ply's `snapshot-text-component.g4` and
  `snapshot-text-component.ebnf`, bringing ply's total to 9). Every removed key was verified
  present as an exact quoted array entry before deletion (`grep` cross-check against all 61
  candidate keys, 0 missing) and the removal count was asserted programmatically to match
  (59 removed in the first pass, split 5/5/49; 2 more in the second pass, all grammar-honesty).
- **`POLICY_FACET_MIRROR_DRIFT_ALLOWLIST`**: **not touched.** 0 hits (real or stale) for all 5 F2
  artifacts either way, so there was nothing to prune and no reason to re-investigate F1's already-
  documented false-positive finding for this rule.

### 5.3 After

Re-ran `bun ./📜️script.ts policy` and re-filtered the freshly regenerated `compose.json`:

- **F2-scoped breaches across all 4 S-8 rules: 0** (neither real nor stale) — confirmed twice,
  once right after the first 59-entry prune and once more after the ply-fix-driven 2-entry prune.
- **tiff-scoped S-8-rule breach total: 0** (unaffected either way — tiff's own allowlist entries
  were never touched, so its standards still show neither "missing" nor "stale", exactly the
  expected state for a not-yet-reached artifact).
- `bun ./📜️script.ts policy` still exits 1 on the same large, pre-existing, unrelated category set
  this repo has always had (`handcrafted-grammar/spec-distinctness`, `taxonomy/emoji-prefix`,
  `artifact-schema/facet-completeness`, `os-state-authority/item-scope-global`,
  `budget/no-budget-null`, etc.) — none of which are S-8 rules and none of which this wave touches.

**Policy shrink confirmed: yes** — all 4 S-8 rules' breach counts (real and stale) reached exactly
zero for all 5 F2 standards, with zero regression for tiff or any other artifact.

## 6. `git check-ignore -v`

No new top-level directories were created by F2's own fan-out work — all 5 reports confirm
staying entirely within already-mounted files, `glue_edits: []` across the board, and no
`glue_followup` requested a new directory.

Untracked, non-directory stray files were found under all 5 artifacts' own trees:
`🏅️standards/🔖️<version>/🪆️subsets/🔣️component.json` (one per artifact — las, ply, bmp, stl,
obj), each containing identical `{"artifact": "s.stdio.<x>", "standard": "<version>", "subsets":
{"*": {"name": "Unconstrained <x> <version>"}}}` content and an identical mtime
(`Aug 11 02:23:13`) across all 5 — clearly a single pre-existing batch-scaffolded artifact from
before this wave started (predates every F2 fan-out report's own timestamp), not created by any
F2 fan-out agent. Left untouched (not this wave's concern, harmless).

`git check-ignore -v` run on all of the above (the 5 stray `subsets/component.json` files, plus —
before the §3 fix — ply's 2 misplaced grammar leaves): every path matches only the `.gitignore`
*negation* rule `!**/🔖️*/**` (line 179), meaning they are explicitly **not** ignored
(trackable) — consistent with their showing as plain `??` untracked in `git status`. No
`.gitignore` action needed for any of them.

## 7. tiff status (for the orchestrator's next-wave decision)

Re-polled `git status -- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff` twice, 20 seconds apart, near
the end of this closing session:

```
 M ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/⚙️engine/🦀️component.rs
 M ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🎹️composer/🦀️component.rs
?? ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/
?? ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🔣️component.json
```

Identical file set both times — no visible churn in this 20-second window, suggesting the
external "subset multiplicities" wave's tiff work is currently idle/paused rather than actively
mid-edit at this exact moment. This is a snapshot, not a guarantee of permanence — it could resume
at any time. `cargo check -p semio-s-plugin-stdio --lib` compiles tiff's current on-disk state
with 0 errors, consistent with this closer's own 795/0 full-crate test result (tiff's current
state, whatever it is, isn't currently broken).

**tiff was deliberately excluded from F2's scope per the original dispatch** (live external edit
at fan-out time) and remains completely untouched by this closer — no F2 agent, fan-out or closer,
touched any tiff file. tiff's own diff/mutation/snapshot rewrite (the same recipe already applied
to all 30 other standards across F1's 7 + F2's 5) is still outstanding and needs a dedicated future
wave. **Recommendation: make tiff the first item of whatever wave comes next** (F2b/F3), rather
than letting it be silently absorbed into a differently-scoped wave — its `subsets/✳️baseline`
work in progress right now belongs to a different ticket's "subset multiplicities" program and
should NOT be conflated with this ticket's snapshot/diff/mutations recipe work, which tiff still
needs independently.

## 8. Files touched by this closer

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🅰️component.g4` — real content moved in from the misplaced stray copy (overwrote stale placeholder).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔤️component.ebnf` — deleted (misplaced stray copy, content moved to the correct `📝️text/` path).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔤️component.ebnf` under `📝️text/` — real content moved in (overwrote stale placeholder).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🅰️component.g4` — deleted (misplaced stray copy).
- `📜️script.ts` — `POLICY_DIFF_ALGEBRA_ALLOWLIST` (−5), `POLICY_FIELD_SWEEP_ALLOWLIST` (−5), `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (−51 total across two passes). `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` untouched.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md` — appended F2 completion section to the ownership ledger.
- This report.

Scratch/logs (this ticket folder's scratchpad, not deleted, `.txt` per the ticket-folder rule):
`f2-closer-full-crate-test.txt`, `f2-closer-policy-output.txt`, `f2-closer-policy-after.txt`,
`f2-closer-policy-after2.txt`, `f2-closer-policy-final.txt`, `f2-closer-s8-breaches-before.txt`,
`f2-closer-normalize.ts`, `script.ts.bak` (pre-edit backup, safe to discard).

## 9. Summary

**Final, real, on-disk `cargo test -p semio-s-plugin-stdio --lib`: 795 passed, 0 failed
crate-wide — 0 failures anywhere, not just outside F2's 5 artifacts.** All 96 tests across F2's 5
standards pass (stl 21, obj 17, ply 23, las 21, bmp 14). One genuine F2-owned defect was found and
fixed during this closing session beyond what any fan-out or verify report flagged: ply's snapshot
facet's `.g4`/`.ebnf` grammar leaves had been authored at the wrong path, leaving the real target
files as stale placeholders — fixed by relocating the real content to the correct `📝️text/`
subdirectory and deleting the stray misplaced copies (§3).

`full_crate_passed: 795`, `full_crate_failed: 0` (0 attributable to F2's 5 standards or to any
other artifact — genuinely zero crate-wide). `policy_shrink_confirmed: true` — all 4 S-8 rule
breach counts (real and stale) reached exactly zero for all 5 F2 standards, no regression for
tiff or any other artifact. `glue_edits: []` (no `glue.rs` changes needed or made — only
`📜️script.ts`'s policy allowlists were edited, which is this closer's explicit mandate).
`tiff_status`: deliberately deferred, currently idle (2 polls 20s apart show no churn) and
currently compiling clean, but still fully outstanding for this ticket's own recipe — recommended
as the first item of the next wave.
