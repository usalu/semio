STATUS: plugin-decl-surface, plugin-peer, plugin-child GREEN (wasm32-wasip2 --lib, hub manifest, `C/🗑️generated/1101/plugfam-2.rc`=0, no errors in `plugfam-2.txt`). Full framework gate NOT green: `plugfam-gate-1.txt` rc=1, refused before compiler because plugin-job (lane plugjob) source extent exceeded: `👷️job/🏛️registration/🦀️.rs` 14657 bytes > 11964 max (plugjob must split it, no limit raise). Nothing behind plugin-job (decl-app, typed-job family) could be judged.

## Done
- peer / child: `.result` on `artifact_retirement_*` outcomes (cycle autofix had already covered most; child pump `map_err` at the active-box close fixed).
- decl-surface `📥️natural-file`: InteractiveJob `step(cx, claim) -> InteractiveJobStep` via `stepsplit.py` (`step_original` + `from_callback`); 8 `admit_owned_retirement(..)` matches take `.result`.
- decl-surface `🌱️original` errors from gaterp-62 were already gone in plugfam-2.

## Open
- Re-run the gate once plugjob shrinks `registration/🦀️.rs`; crates behind plugin-job unverified.
- Tests of these crates not ported (only --lib checked).
