# F27 result: persisted graph stores re-serve a stale row to every fresh worker

## Finding
- Persisted path-keyed stores: the owner checkpoint (`oc` packs, key = hash of scope, format, owner question, no forget-by-path) and the admission memo (`<scope>.admission.json`, owners lent to the in-process memo). The closure graph store (`.cache/f/...`) is stamp-keyed (path stamp = size, mtime, ctime, inode) and is not the carrier.
- Mechanism (proven at store level, `🪦️forgotten-paths/🧪️tests`, "a stale row the in-round judge keeps holding is served by every fresh worker ..."): the repair path (F23) evicted only the in-process owner facts memo. A repaired or fresh worker then asks the checkpoint (`recover`) and the memo seed; both hand back the same stored row, and whenever the in-round hold (shared verdicts, `memo-check`) accepts it while the seal (outside any round) judges it stale, the same path and fact come back: `unrefreshed-facts`, exit 75, three workers in a row. Three fresh workers in the test all answer the old facts with zero discoveries.
- NOT reproduced live: I did not run a real slice (load 29-38, seal window) and did not find why the in-round judge accepts the row on m02. The fix therefore does not depend on that asymmetry: whatever the judge says, a row the seal proved stale cannot be re-adopted from disk.

## Fix (tombstones, not stamps)
- `📥️load/🪦️forgotten-paths`: `<scope>.forgotten.json` (schema first) holds path -> time proven stale (max 1024 paths, 7 days, atomic merge). A stored record is refused when one of its rows names such a path (or lists its directory) and the record was stored at or before that time (pack mtime for the checkpoint, memo file mtime for the memo). A row observed after the repair is stored later and is kept, so the next worker adopts the fresh row.
- Called by the three repair branches of `loadBuildGraphAttemptV1` with the moved paths; consulted by the checkpoint `read` (new port `refused`), the admission memo owner seeding and the memo `remembered` check.
- Chosen over stamp-keying: the row's own judge is what disagrees, so a key made of input stamps would need the same judge; a tombstone keyed by the seal's verdict does not.

## Proof after the fix (unit level)
- `forgotten-paths` test "after the seal forgot the stale path no later worker adopts the old row": three fresh workers answer the new facts, the first discovers once, the other two reuse the row observed after the repair (no repeat of the stale signature).
- Python oracle cross-check of the refusal rule; merge, bound, expiry, corrupt and foreign file cases; stale-facts test asserts the repair persists the moved path. Numbers and file list in `T/landings.md` (Wave 1125 F27).

## Not verified
- No live slice run, no fleet effect measured (check m02 `ready.json` and for `[graph-retry] refresh-facts` followed by convergence after the supervisors restart). The memo seed path is covered by the wiring through the extracted-function harness (stubs), not by a real memo file test.
- `📁️inventory` test (load) fails before reaching this code because of a peer's cache-partition policy change (`Canonical workspace and declared cache partition required`).
