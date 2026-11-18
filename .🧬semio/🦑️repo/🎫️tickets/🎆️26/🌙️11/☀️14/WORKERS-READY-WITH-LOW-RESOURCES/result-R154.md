# R154: the tail of the take pass ("taken 588 of 590", "52 of 53"), and the `Original project inference observations required` failure (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R154/` (`patches.py` anchored edits, `land.py` apply/restore tested byte-identical on a scratch mirror, `window.sh`, `battery.sh`, `compare.py`, `tests.list`, `fast.list`). Landing entry: `landings.md` "R154 landing 1" (22:12:19 to 22:12:55 UTC, hold 36 s).

## 1. What the "tail" is (evidence, no owner is missing)

The lab's own timeline (`timeline.jsonl` of R147 lab1 and R143 run6, one sample per 2 s with the service activity) shows the read of `repo-mcp` (53 owners) as a loop of whole graph attempts: `build:source-admission`, `command-source-imports`, `selected-*`, `build:projects` (the owners), `project-assembly`, `build:modules`, `build:graph`, `build:refresh`, and again from `source-admission`, many times in 540 to 1300 s. The owners phase of one attempt is a few seconds (records present: `[owners] owners 53 of 53 done: 53 present`); the rest of an attempt is admission, assembly and validation, which print NO progress line. So:

- `taken 52 of 53` (lab) and `taken 588 of 590` (workers) are the LAST value the progress timer (`startSliceProgressV1`, every 5 s) saw before the owner source of the pass was removed. The final increments of a pass, and every later phase, are never told. Nothing computes the "last 2 owners" because there are none: the pass is over, `processes.count 0` and a parent of 1.3 GB are an attempt in admission. Same reason R142's tail report ("no owner arrived for ...") never prints: its timer is cleared with the pass.
- Why the attempts repeat: the R143 service log (run6, `[graph-repair]` rounds 1 to 4, then `[graph-retry] reason observed-facts`, three times in a row for 20 minutes) names ONE stale fact every time: `project-nodes/🟨️.mjs`, recorded `3b2b5c56…`, on disk `9d5a6aba…`, and the recorded value is the same in every round. The process reads its implementation files pinned (`withImplementationPinV1` + `useProcessWideImplementationPinV1` in the slice worker), so a repeated inference reads the pinned bytes again and the fact can never hold: ONE edit of a loaded implementation file (the fleet lands one every few minutes) makes every later repair round and attempt of that process fail for ever, each repeating the 100 to 400 s start phase. Neither the repair rounds of R151 nor the 30-minute `code-stale` restart can end it earlier.
- `Original project inference observations required` (g1-m07, 21:32 UTC, reference 83ec1075): thrown by `observeProjectInferenceV1` itself, not by the tagging (`tagProjectFailureV1` only appears in the async stack because it awaits the call). The publication of the inference is looked up again by the record path in a module-wide map after the awaits of the computation; another start (`delete` at the top of `observeProjectInferenceV1` and `observeInferenceComputationV1`) or `releaseProjectInferenceRetentionV1` of the same record path in between removes it (the take pass looks at owners the loop is inferring; the loop does not register in `inflight`). The race is closed by handing the publication to the caller (`published`); a law reproduces it (`🤝️publication-capture`, red before: the exact message).

## 2. What changed (landed 22:12 UTC)

| change | law (red before, green after) |
| --- | --- |
| a repair round that leaves the same stale source facts (same path, same recorded fact) as the round before ends the attempt as a stale implementation (`staleSourceFactsErrorV1`, code `BUILD_AUTHORITY_CHANGED`, `[graph-retry] reason unrefreshed-facts`): the slice worker exits 75 and the pool starts it again with the current code | `🧪️tests/🕸️graph/🧊️stale-facts` (2: pinned fact ends after one repair; a refreshed fact and a fact that moves every round are not mistaken) |
| the final owner count is told: the source's last value is kept at removal and the timer emits `owners done N of N` once, `taken` again for the next pass | `🧠️memory/📋️slice-report/🧪️tests/🏁️final-line` (2) |
| the observations an inference published are handed to its caller, a lost one is the typed `BUILD_INFERENCE_PUBLICATION_LOST` naming owner and record (EN/DE) | `👁️observation/🧪️tests/🤝️publication-capture` (1) |
| `tagProjectFailureV1` (exported) wraps an error that cannot be extended or a plain value, keeping owner, code, text and cause; it reads no observation | `project-nodes/🧪️tests/🏷️owner-tag` (4) |
| `failureMessageV1` names the owner of a failure in the refusal line | `🏡️resident/🧪️tests/🧯️failure/🏷️owner-named` (2) |

Battery of 19 test directories before/after: nothing worse; reds that were red before stay (observation main 51/1, candidate 1/2).

## 3. Lab run on the new code (R154 lab1, 22:19 to 22:37 UTC, `--slice=repo-mcp --machines=1`, `SEMIO_GRAPH_PHASES=1`, `SEMIO_BUILD_TRACE_AUTHORITY=true`, load 48 to 68, scratch `R154/lab1`)

- The status line now says `owners done 53 of 53` for the 282 s after the owners (before: frozen at `taken 49/51/52 of 53`). No `[graph-retry]` happened (one repair round, then `[graph-seal]` quiet); so the stale-fact restart was not triggered in this run (no peer edit of a file the process had loaded during its attempt); it is proven by the law only.
- The read published its graph at about 525 s and then started the whole read again (`build:source-admission` at 609 s and 893 s: three complete reads in 1015 s, each 270 s of source admission, command-source imports, `selected-*` and portable inputs under load 60); no `ready` verdict within 1015 s. After the first publication the stage re-reads the graph two more times; why (second and third read of one `run-many` dry run) is NOT established. This is the wall that remains in the lab: time, not a stuck owner.

## 4. Workers after the landing (store `origins/<m>`, 22:30 to 22:37 UTC)

- g1-m07 already runs the new code: its status line reads `owners done 589 of 590` (then the read ends). That is a REAL missing owner, not a display value: the stage failed at 22:33:45 with the refusal `host-graph-owner-too-heavy` whose names are EMPTY and limit `0 MB` ("The project graph could not be read on this machine yet:  need(s) more memory than this machine can give one project (0 MB)"). `heavyRefusalOfV1` (`🏛️service/🧯️failure`) builds `named`/`limit` from the error values or the JSON at the end of the message; here neither arrived, so the refusal names no owner. Next package: make the heavy refusal carry the owner names across the process boundary (the worker error must put the JSON at the end of its message or the host must read `error.values`), and find why one owner is classified `never` on a 16 GB machine (`neverHere` uses `row.saturated === true` or the live solo grant; R147's pool/read budget).
- g1-m07 and g1-m08 had `Original project inference observations required` twice (reference 08a57583, 22:04 and 22:1x, old code, same stack): it repeats, so it is a frequent race, not an accident; after the landing it has not been seen in the store.
- No worker reached `ready: true` (m01 587 of 590, m08 586 of 590 on old code; m03 stuck 4735 s on old code, restart pending, m10 restarted).

## 5. Not done / unverified

- The take-loop invariant of the brief is not built: the evidence has no stuck loop; the owner pass ends (the heavy owner of m07 ends it with a typed refusal, only with a nameless text).
- No lab or worker log yet shows `[graph-retry] reason unrefreshed-facts` followed by exit 75 and a restart (law only). R156's rewrite of the attempt (`graph-seal`) kept the guard (`📥️load` line 137) and its repair-rounds law (11) is green with mine.
- The heavy refusal without names, the "589 of 590" owner, and the three whole reads of one dry run are the next walls.
