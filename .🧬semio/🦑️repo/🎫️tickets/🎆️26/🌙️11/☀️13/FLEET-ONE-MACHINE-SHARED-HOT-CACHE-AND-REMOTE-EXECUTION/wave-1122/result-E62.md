# Result E62 (owner-registry-scope), 15:50 CET

Landing: `T/landings.md` "Wave 1122 E62" (lock graph, released). Scripts kept in `T/🧰️scripts/E62/` (land1.py, land2.py, two real-registry probes).

## 1. Where the registry entered keys (before)
`buildGraphScopeKeyV1(selection,registry)` = sha256(JSON[selection, whole registry]) = `scope`. `scope` named: the store `inference/selections/<scope>/` (np owner records, sa, oc checkpoint, members), `selected/<scope>.json` + `.authority.json` + `.admission.json`, the fleet head key and slot (`selectedHeadKeyV1`), the in-process admission-facts key, writer resource names, and the registry-check (2 sites in `inventory`). Elsewhere: owner record key head `registryDigest` = catalog providers only (E1, already row-level); in-process `memoContext` (taken memo) and `hubKeyV1` still hold the whole registry (not stores; left).

## 2. Design (landed)
- Scope = digest of the selection alone. Registry edits never change a store, graph, memo or head name.
- What a read asked of the registry travels in its records as `global` rows (E1's mechanism): `row`, `package`, `root`, `within`, `providers`, `candidates`, `wasm-index`, and one `digest` row for the closure and the selected graph (they read the registry whole). Resolver reads the current `📋️build-owners.json` (memoized per file stamp). Edit of row A: only records that asked about A (or a question A's change alters) go stale; the closure/graph re-derive cheaply from valid owner records.
- Schema first: `🕸️graph/🧾️registry-reads/🧬️schema/🔣️.json`. Registry-check now compares registry digests.

## 3. Proof
New test `🧾️registry-reads/🧪️tests` (4/0): Python oracle (`🐍️.py`, hashlib, own resolution) agrees on row digests, registry digest, every answer and, per edit, which rows/answers move; scope key equals oracle and is unchanged by every edit; disk resolver flips only the affected questions across 8 edits; recording view test. Real registry (4836 owners): digest 27 ms, 20k `within` questions 120 ms; one-row edit moves exactly one row digest. Recorded `wasm-index` digest equals resolver digest on the real registry (1.4 s observed / 0.36 s plain).
Dependants (repo test-file, final run, 15 files, all pass): registry-reads 4, admission 10, 🛂️admission 20, adopt-registry 3, stale-facts 2, seal-under-churn 3, repair-rounds 28, candidate 2, inventory 10, discovery observation 4, owner-names 1, authority-round 1, contract-reads 4, generation-identity 3, taken-memo 4. Harness mocks repaired for the new bindings.

## 4. Coordinator / landing
Whole read pids 62120/62121/62299 stayed alive through the landing (checked 15:44); no graph command was run, so no structural claim competes (E54: a plain code edit does not restart it). Effect is one re-key: first selected read per machine opens `selections/<selection digest>`; old registry-keyed dirs go to cache gc. Workers adopt via replica.

## 5. Pilot plan (one machine)
Pilot m12 (fastest slice): after its supervisor restarts on the new code, note `selections/<scope>` dir name and `np` count; wait for the next `📋️build-owners.json` edit (reconcile adds rows several times a day); expect same dir, `present` ledger not reset, `[graph-owner]` re-lookups only for owners with a stale row. Then fleet-wide is already in effect.

## NOT verified
- A real selected read across a real registry edit (not done: whole read sealing, load 40-50).
- Pre-existing red tests not caused here: show-project-scope (command text), admission-memo "facts lent … own row" (ReferenceError observeOwnerWithRetryV1 in harness), selected-exchange wiring key text (`key=scope+…` suffix refactor).
- `candidates` question is coarse: any owner add/root change re-derives the few catalog-consumer owners.
- Layer-B records still do not record `playground`/generator reads beyond existing rows.

## Others must know
In-process `memoContext`/`hubKeyV1` keep the whole registry: a registry move inside one process still restarts the farm/pass memo (disk stores survive now).
