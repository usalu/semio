# Findings E25
- mtimes cannot show recent edits: at 05:16:35 every landing.lock owner file and many sources got the same mtime (mass touch / replica). Use landings.md and lock owners instead.
- `🧭️invocation/🕸️graph/🧠️memo/✏️edit`: "rows.slice is not a function (rows.slice(1))" in the derived-serve path: a real runtime error behind 3 failing tests (owner: memo/graph lane).
- memo `soundness against the engine` and `output directory ... rename` tests: `observationsHold(rows,64,new Set(),true)` answers true for a file rewritten with the same size and a replaced directory; check the racy-window stamp shortcut of `👁️observation/🟨️.mjs` (recordedFactHoldsV1 / stableFactHoldsV1).
- Budget drift in the startup law fixture is silent because the single test stops at the first failing row: fleet-entry masked six more failing rows (leases, capacity, doctor, caller, watchdog, fleet-door). Suggest one assertion per row.
- `⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🧪️tests/🌳️rooted` and `🏞️task-view/🦀️cargo` fail outside the suite runner ("scratch Cargo workspace outside every enclosing Cargo configuration", launcher required): they need `sf.sh`/the suite.
- Doctor registry builds `DOCTOR_REPAIRS_V1` synchronously from all probes, which blocks a lazy probe load (closure 152 modules).
