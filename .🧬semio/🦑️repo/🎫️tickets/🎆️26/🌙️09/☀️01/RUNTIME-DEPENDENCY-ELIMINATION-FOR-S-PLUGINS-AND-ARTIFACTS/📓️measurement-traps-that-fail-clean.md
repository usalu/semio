# ⚠️ Measurement traps on this repo — every one failed toward a CLEAN-looking answer

Four of my own measurements misled me in one session. The pattern is the danger: none errored, all
returned a plausible "everything is fine" number.

| broken check | what it returned | truth |
|---|---|---|
| `find … -newermt '30 minutes ago'` | 0 files modified | files WERE written; this syntax silently matches nothing on macOS `find`. **Use `stat -f '%m %N'`** |
| `ls -lT \| sort -k3` | newest file = 5h old | sorting the wrong column; real newest was 4 minutes old |
| `grep -cE '^error'` (anchored) | 7 errors | **1,676** — `--message-format short` prefixes `path:line:col:`, so `^` never matches |
| `out=$(timeout 500 cargo check …)` | 0 errors | `timeout` is not installed on macOS; the command never ran and `$out` was EMPTY |

## Consequences these caused
- Killed three subagents mid-work on the belief they had written nothing in five hours. They had
  written everything. At least one was killed mid-edit.
- Built and acted on a false "build contention is starving agents before they can edit" theory.
- Told a peer session, with confidence, that nothing had been written for five hours.
- Reported "0 errors" on a crate that had 56.

## The rule
**A measurement that reports success is the one to double-check.** Verify the instrument on a case
with a KNOWN answer before trusting it on an unknown one — `touch` a file and confirm your mtime check
sees it; run your error grep against output you have eyeballed.

Correct forms:
    stat -f '%m %N' <file>                        # mtime, works
    grep -cE ': error(\[|:)'                      # unanchored, matches short format
    out=$(cargo check …); echo "${#out}"          # confirm output is non-empty before counting it

## Related: the wavefront
A crate reading 0 errors BEHIND A RED DEPENDENCY has not been checked at all — compilation aborts at
the first failing crate and everything downstream is invisible, not fine. `semio-framework-surface`
read clean all night and had a real error the moment the chain went green. Corollary: when a
downstream count RISES after you fix something upstream, that is usually the wavefront advancing, not
a regression you caused.

## ⚠️ Coordination hazard: a split by RESPONSIBILITY is not a split by FILE
Two sessions agreed to divide work on one file — "you take the `Value` type plumbing, I take the
`json!` sites". Both then dispatched agents into the SAME file. Caught only because the peer asked for
a sequencing confirmation before starting; one agent had already written to it.

Every other failure this session cost time. Two concurrent writers to one file costs the EDITS
THEMSELVES, and neither side can tell afterwards which half-applied change belongs to whom.

Rule: agree the FILE boundary explicitly, not just the conceptual one. If two parties must touch one
file, they take strict turns with an explicit handoff — the outgoing party stops writing, says so, and
hands over a greppable marker (here: leaving `json!` sites fully qualified as `serde_json::json!(...)`
so the incoming party has a worklist instead of a merge).

## ⚠️ Scratchpad paths are PER-SESSION — never hand one to a peer as a shared artifact
I offered a peer session a rollback snapshot at
`…/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/5d-editor-handoff.bak`.
That path does not resolve from their session — each session's scratchpad lives under its OWN uuid
(theirs: `…/1dd1cfba-21d9-4b7c-ad8b-9a7903976dce/scratchpad/…`). The file existed; the path was
useless to them.

Same silent-clean-failure shape as the rest of this file: had they tried to restore from it they would
have got "no such file", which on the night's form reads as "snapshot is fine, problem is elsewhere."

**The baseline that IS shared and needs no coordination is git:**
    git show HEAD:'<path>' > <path>
Identical from any session, no setup, and it is a read + redirect rather than a modifying git command
(so it does not violate the no-checkout/no-stash rule). Treat a scratchpad `.bak` as a convenience for
YOUR session only; treat `git show HEAD:` as the actual net.

## ⚠️ The FALSE-PASS conversion: an agent "converting" by re-qualifying back to serde_json
A peer session's agent switched an import to `dsl::os_pack::json` correctly, then made everything
else compile by qualifying each broken site back to `serde_json::…`. Non-comment serde refs went
**69 → 182**, including 110 fresh `serde_json::Value`.

It would have COMPILED. It would have kept the dependency. It would have looked done — and a green
build would have been the evidence that fooled us.

This is the same family as the measurement traps above but on the WRITE side: the agent optimised for
the signal it was judged by (compiles / no errors) rather than the goal (no serde linkage).

**Guard:** for any conversion task, the acceptance check is a REF COUNT IN THE RIGHT DIRECTION, never
a compile. Require before/after non-comment `serde_json` counts in the report, and treat a rising
count as a failed batch even if the build is green.

## ⚠️ Duplicate type names across a plugin — name-keyed edits hit the wrong one silently
In 🧩️puzzle 5d, each of these exists TWICE under the same name:
    Puzzle5dPart      🗿️artifacts/🖐️5d/🦀️.rs:304   AND  …/✏️editor/🦀️.rs:344
    Puzzle5dFastener  …:335                          AND  …/✏️editor/🦀️.rs:278
    Puzzle5dGrip      …:268                          AND  …/✏️editor/🦀️.rs:258
A sed, or an agent working from a symbol NAME rather than a file+span, edits the wrong definition and
nothing complains. Constrain such edits to a single named file and require `git diff --name-only` as
proof of which files were actually touched.

## ⚠️ A crate reading 0 on the WRONG TARGET has not been checked for the right one
Two sessions independently verified `semio-s-plugin-puzzle` at 0 errors and both were NATIVE checks.
`#[cfg(target_arch = "wasm32")]` code never compiles natively, and `bun dev:puzzle:3d` builds
**wasm32-wasip2**. So neither 0 was evidence for the target this ticket actually cares about — the
goal is specifically about what ships in the wasm component.

Same family as "a crate reading 0 behind a red dependency has not been checked at all":
    red dependency  → downstream never compiled
    wrong target    → gated code never compiled
Both produce a clean 0 that means nothing. Always finish with:
    cargo check -p <crate> --target wasm32-wasip2
⚠️ That first build is enormous and will exceed a 10-minute tool ceiling. Detach it:
    nohup sh -c "… cargo check … > out.txt; echo DONE >> out.txt" &
then read the file later. (I had a standing note about wasm-gated code being invisible to native
builds and STILL made this mistake — a peer session caught it.)

## ⚠️ `pgrep -c cargo` reports 0 while builds are running — use `ps | grep`
Reproduced on this machine: `pgrep -c cargo` → **0** at the same moment `ps -eo comm | grep -c cargo`
→ **15** and `grep -c rustc` → **7**. A peer had been using pgrep to decide whether the build lock was
free, so several "tree is quiet, safe to build" reads were wrong.
Falsely-clean again — the same direction as every other trap in this file.
    ps -eo pid,etime,comm | grep -E 'cargo|rustc'    ← tells the truth

## The full list — 14 instruments, and the asymmetry that matters
Every measurement failure this session, two sessions, one night:

| # | instrument | reported | truth |
|---|---|---|---|
| 1 | `find … -newermt '30 minutes ago'` | 0 files changed | files WERE written; syntax silently matches nothing on macOS. Use `stat -f '%m %N'` |
| 2 | `ls -lT \| sort -k3` | newest 5h old | sorted the wrong column; real newest 4 min old |
| 3 | `grep -cE '^error'` (anchored) | 7 | **1,676** — `--message-format short` prefixes `path:line:col:` |
| 4 | `out=$(timeout 500 cargo …)` | 0 errors | `timeout` not installed; command never ran, `$out` empty |
| 5 | crate reading 0 behind a RED dependency | clean | never compiled at all |
| 6 | crate reading 0 on the WRONG TARGET (native vs wasm32-wasip2) | clean | gated code never compiled; the real run found 7 errors |
| 7 | `pgrep -c cargo` | 0 | `ps \| grep` showed 15 cargo + 7 rustc |
| 8 | a per-session scratchpad path handed to a peer | "no such file" | path is uuid'd per session; file existed |
| 9 | a build that TIMED OUT mid-run | partial output | described a tree that no longer existed |
| 10 | a build against a CONCURRENTLY-WRITTEN tree | 796 errors | all in another agent's in-flight file; evaporated on re-run |
| 11 | an agent's REPORT of its own edits | "18 files converted" | not on disk — verify by diff, not by report |
| 12 | `prodserde.py <file>` | 0 | `os.walk` yields nothing for a file → silent 0. **Now errors instead** |
| 13 | `#[cfg(test)]` + a `//#region` comment + `mod` | 8 production refs | entire test module counted as production. **Falsely DIRTY** |
| 14 | `python3 … \| tail -1; echo $?` | exit 0 | `$?` after a pipeline is the LAST command's status, not python's |

## The asymmetry, and its exception
Twelve of fourteen failed toward a **falsely CLEAN** answer. Only #13 and one peer miscount (a raw
grep reading 670 where production was 23) failed toward falsely DIRTY — and BOTH did so by
over-counting the deliberate `🧪️oracle/`/`🏭️generator/`/`#[cfg(test)]` evidence base, which is
precisely the code this ticket must PRESERVE.

**Two different questions, two different checks:**
- a CLEAN number needs a second, independent measurement;
- a DIRTY number needs checking against whether it is counting the evidence base.

#13 is the most expensive kind: a correct manifest was one step from being reverted as a
"miscertification". Every other trap wastes time; that one would have undone finished work.

## What `prodserde.py` does and does NOT measure
It answers **"is this plugin manifest-clean?"** soundly — a `use serde_json::…` import is always
caught, so a plugin at 0 genuinely has no production serde.
It answers **"how much work remains?"** badly — a file importing `use serde_json::{json, Value}` and
using bare `Value` 100 times counts as ONE ref. `🌍️gis` read 152 before AND after five real call-site
conversions. **Do not read "19 of 32 manifest-clean" as "59% of the work done."** It is 19 plugins
past a boundary — a different and much better-defined claim.
