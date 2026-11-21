# Coordinator Stage Publication Observation

At 09:18:37 UTC on 2026-11-21 the coordinator daemon PID 50009 had a fresh boot beat in run phase, activity git and 149 ms lag. Its journal head remained at 09:13:59 and attestation at 09:14:25, reporting running code d0cb1e1f505c14d07704592246ad1007d78afea70c44d1a2c8493e8007530d2f and staged tree 41cfdebf5190cfa4c28e0b38b0294119d053503f. The activity label covers multiple git-watch and healing operations and does not identify an exact stack or a particular wait. A single process-table sample without a Git child does not establish the cause.

The replica permits only one attester tick at a time; a pending tick prevents another tick. Code detection, admission and adoption occur within that tick, so an unfinished tick is one possible explanation for the stale attestation and old code, but the observed heartbeat and files cannot prove it. The boot watchdog detects silent beats or excessive event-loop lag; a fresh beat does not prove progress in asynchronous operations and does not by itself trigger that watchdog. Normal code adoption requires detection, admission and self-check; the presence of new sealed code does not prove it is executing. No safe explicit nonmutating recovery was established, and no process or runtime state was changed.

Relevant source: 🤝️shared-worktree/🔁️replica/🟦️.ts around lines 2520–2550 and 2661–2670; 🤝️shared-worktree/🩺️attest/🟦️.ts around lines 633 and 683–713; 🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts around lines 265–273.

## 09:22 UTC Recheck

The daemon beat remained fresh while health.json was last updated at 09:14:04, git-index.json at 09:14:09, and the local state WAL and shared journal head at 09:13:59. The Git index mtime was 09:14:31 and index.lock was absent at the sample. These timestamps delimit observed progress but do not identify the awaited operation. The last health record said store ok and pending null at its own timestamp; it cannot establish either condition now.

The store guard abandons a turn when a probe itself is overdue and the work beat is also overdue. It does not impose a general deadline on a local operation while probes continue answering. Existing beat activity git covers multiple git-watch and healing operations. Existing records and a single process snapshot do not distinguish a long pending operation from a permanently stuck one or identify its exact await. No nonmutating supported per-await observation was found in the inspected interface.
