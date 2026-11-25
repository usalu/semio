
## Recheck 11:23

Cause: `advance_builder` in `🧵️instance/👷️jobs/📥️staging/🏷️reserved/🦀️.rs` (lines 150-151) had `if/else` branches whose `build_*_job(...)?` calls returned `bool` while the statement context expected `()`; the 11:05 wasm release build saw that version. A concurrent edit at 11:09 added the missing semicolons (`...cx)?;`), so both E0308 errors are gone.
Check: `repo cargo --store=shared --configuration=prototype -- check -j 1 -p semio-plugin-instance-jobs --message-format short` finished with no errors (warnings only). No further edit was needed from this session.
