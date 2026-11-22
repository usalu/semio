# Result E75 (graph-keys-2), 16:03 CET

Landing: `T/landings.md` "Wave 1122 E75" (graph lock, released).

## 1. In-process keys follow the per-row law
- `memoContext` (pass memo of the owner loop, `project-nodes/🟨️.mjs`) no longer hashes `context.registry`. The memo entry already carries the stored observation rows, which since E62 include the registry questions (`global` rows), and `replay` evicts by `holds(rows)`; so an edit evicts only owners whose recorded question it moves. `registryDigest` (catalog providers) stays in the key as before (head of the owner record has it too).
- `hubKeyV1` (project farm hub) no longer hashes `context.registry`. Because workers hold the registry of their session, every job now carries `registryDigest` (digest of the registry of its construction, `ownerRegistryDigestOnceV1`, memoized per registry object). The worker (`👷️service run`) calls `adoptProjectInferenceRegistryV1(environment, digest)`: same digest as held: nothing; different: read the current admitted registry of the checkout (`currentOwnerRegistryV1`, file-stamp memoized) once and remember the asked digest (no read per job); unreadable: keeps the held registry. `scripts.generatedOutput` of the environment now reads `context.registry` live. Soundness: records carry rows with the answers actually given, validated against the current file, so a worker on a slightly newer registry is as valid as one on the pinned one.
- Effect: a registry edit inside one long process no longer restarts the project farm (pools stay, a worker re-reads the registry once per edit) nor the pass memo (only moved owners go stale).

## 2. Proof
- `🧾️registry-reads/🧪️tests/♻️memo` (3/0): for each of 8 edits of the E62 fixture, a taken memo holding one owner per recorded question, with the file registry on disk edited, keeps exactly the owners the Python oracle (`🐍️.py`, independent) says are not moved, and `evictedChanged` equals the moved count; digest-once test (oracle digest, null, reorder = same); source-text test that neither key names the registry and jobs carry the digest.
- `project-farm/🧪️tests/🧾️registry` (2/0): two constructions with an edited registry share the pools (no new worker started), jobs carry oracle digest and edited digest; worker adoption: no read when digest equals held, one read when different, none the second time, unreadable keeps held.
- Dependants all pass: project-farm 30 files (incl. hub, captured, authority, no-inline registry-follow), registry-reads 4, taken-memo 4, generation-identity 3, 🛂️admission 20, verdict-wait 2, look-job 4, owner-harness 1.

## 3. Three red tests (claimed in claims-tests.md)
- `admission-memo` "facts lent ...": harness `Function(...)` lacked the binding `observeOwnerWithRetryV1` that `observeOwnerFactsMemoizedV1` now uses (moved to `🛟️owner-isolation`): test harness bug, binding imported and passed. 10/0.
- `selected-exchange` wiring: product refactor (`suffix` const then `key=scope+...+suffix`) is behaviour-identical; test text updated. 3/0.
- `show-project-scope`: product intentionally changed (`show project` reads the sealed graph, selection `{kind:"all"}` with `closure:[p]` via `sealedProject`, `policy??(whole?"current":"fresh")`); test text updated to the live command. 3/0. (The function-level scope tests pass untouched.)
- Also fixed taken-memo text test that pinned the old key.

## 4. Whole read
Whole read pid 62092 (and 62120/21/299) from E62's note are gone; a `repo show projects` pid 11561 (parent 11559) started 15:55:56 and its seal progress (`.cache/b/g/m/generations/s/4b07...progress.json`, phase facts 90112/841640 at 15:59) advances; owner dir `generations/o` was last written 15:41. My graph code edits landed 15:52:35-46, 3 min before that start; I cannot prove the earlier read ended independently of my landing (E54: a plain code edit does not restart it). Pid 72392 untouched and alive. I ran no graph command and killed nothing.

## 5. Pilot m12
Not observable from the shared store: the selection dir lives in m12's local checkout. At 15:59 `S/origins/g1-m12/ready.json` = waiting, stage workspace, attempts 14, refusal `sidecar-restarting` (supervisor restarting on updated code), so m12 is adopting the newest code now. Procedure for the next `📋️build-owners.json` edit: on m12 note `.cache/b/g/m/inference/selections/<selection digest>` (name = digest of the selection only, since E62) and its `np` count before; after the edit the dir name and the `present` count are unchanged, `[graph-owner]` re-lookups only for owners with a stale registry row.

## NOT verified
- Live pilot above; a real farm run across a real registry edit (only scripted workers); real worker adoption of a registry read from a 4836-owner file (digest 27 ms measured by E62).
- The `candidates`/`digest` rows are coarse (any owner add moves them), as E62 said.
- Before-hashes of the edited files were not recorded.

## Others must know
- Jobs sent to farm workers have a new field `registryDigest`; a farm test that stubs the worker and compares job payloads exactly must tolerate it.
