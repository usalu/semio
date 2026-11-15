# Cargo Owners Growing the Plan Heap (Concurrent In-Process Admission)

Paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Log: `host-74145656f35a.log` lines 1946-2175 and 3587-3737 (two runs).

## Findings
1. The farm WAS used: "inferring projects with 3 worker processes (1536 MB each)" (log 1948, 3589), workers staged 580-820 files each (`semio-graph-farm-*/w0..w2-g1`). It is not "identity" mode here (`REFUSED`, `project-farm/🟨️.mjs:80` only after a refused worker identity; no such line in these runs). The 8 "running" owners are `active` (all owners from admission until they finish), including owners waiting `away` for a worker.
2. What runs in the parent is the part after the worker: re-reading the adopted record and, with `late`, `resolveClosureRefsV1` (`🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:484`), which binds every closure ref of the owner (thousands of command source groups, each a whole closure file list) and compacts them: millions of strings/Objects per owner (log: 16M strings, 10M Objects, heap and external grow together).
3. Root cause 1, `🧠️memory/🟨️.mjs` `run` -> `away`: after `away(work)` the owner was re-added to `owners` unconditionally (`finally{owners.add(owner)}`), without any admission. So the governor admitted owners while they were away (waiting for a worker costs nothing), and all of them came back into the process at once, up to `workers` (8), whatever the headroom ("waiting for memory headroom (1528 of 176 MB reserved, 7 projects running)" while 7 were already in process).
4. Root cause 2, estimates: admission used only the learned peak of the KIND (`language:cargo`), learned from `owner.taken`, which under concurrency is a proportional SHARE of the process growth (`sample()` splits the delta over the owners). Every concurrent sample lowered the peak (needs 101-187 MB in the log), so more owners were admitted, whose shares were smaller again (positive feedback). The recorded cost per owner key (430-470 MB in `owner-costs.json`) was used only for the heavy classification, never for admission. 8 x ~450 MB plus the ~1.3 GB base matches the 6-7 GB growth: there is no hidden unattributed shared structure in the cargo path itself (`nativeDependencyRoots` memoises per directory in `nativeClosures`, `readToml` per file; no `cargo metadata`), the "missing" bytes are the attribution split and the unadmitted concurrency.
5. Per-owner enforcement (`enforce`) only compares one owner's SHARE against its limit (1756 MB), so seven owners can each stay below it while the process reaches 7 GB; the plan guard then stops the whole plan (exit 75). Not changed here: the limits stay, the admission is now sound.

## Fix
- `🧠️memory/🟨️.mjs`: new `estimate(key,kind)` port; an owner costs `max(kind peak, recorded cost of its key)` in admission, in-flight room and the "smallest pending" choice; a recorded cost makes a kind count as known.
- `away` re-admission: `resume` waits (poll, bilingual "waiting" line, abortable) until a slot is free and the headroom less the remaining estimates covers the owner's cost, or nobody else runs (so it never deadlocks); new admissions wait while owners resume (`resuming`).
- Learning: a sample taken while owners ran together (`owner.shared`) only raises the kind peak, never lowers it.
- `🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs`: `graphOwnerEnforcementV1` returns `recordedBytes(name)` (recorded estimates only), passed as `estimate` to the governor. `🧠️memory/🟨️.d.mts` typed.

## Tests
`🧠️memory/🧪️tests/🛡️resume/🟦️.ts` (3 pass): owners returning from workers never exceed the headroom with the recorded cost; the last owner always returns; a shared share does not lower the learned peak. Also pass: `🚪️away` 1, `🛑️overrun` 7, `project-nodes/🧪️tests/🛑️too-heavy` 6, `🚦️governed-dispatch` 5, `🧹️parent-budget` 3.
Not caused by this change: `🧠️memory/🧪️tests/🟦️.ts` 1 failure (`psProcessRowsV1` missing), `🧠️memory/🧪️tests/🧹️parent-budget` stopped by the test ceiling (3.4-4.3 GB, spawns its own allocating processes; this change can only lower concurrency).

## Not verified / open
No live run (service untouched). Item 2 (the transient memory of `resolveClosureRefsV1` per owner) is inferred from code and the object counts, not profiled; `cargo-heap-profile.mjs` (this folder, copy of `profile-owner.mjs` for the Documents workspace) is a start but needs cargo-owner JSON built through `cargoTargets` first. Next step if growth persists after admission is sound: stream/intern the per-owner `bind` and `compactSourceInputs` strings.
