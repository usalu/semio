# Cargo Wait Snapshot — 2026-11-05

Read-only owned lease snapshot 1793875757429: shared Cargo registry holders 45536 and 35368; captured workspace 46992 holds configuration-view 3, Hub 26756 view 17, component 46216 view 60. No repo cargo waiters, cycles or violations shown in this snapshot. These repo leases do not identify the owner of Cargo operating-system package-cache filelock.

Surface reports law 78614 last delivered planning-capture output said Blocking waiting for file lock on package cache at approximately 620 seconds. Doctor processfacts 1793875784460 confirms parent 56536 and captured child 45536; no Cargo executable descendant appears under that subtree at this instant, so the old wait is not asserted as current. The 57092 to 35368 subtree is separate and 35368 shared registry holder reports source admission unit 2/33 in lease snapshot. JPG outer PID 53880 is separate. Captured-root ledger helper at 1793875772242 contains no marks for 45536, 35368 or 53880; best-effort retention and bootstrapping make this no execution verdict.

Root workspace 46992, Hub 26756 and component 46216 processfacts 1793875749590 show captured Bun commands with source SHA256 895e41b299e599742ab44a3529ba2c3e329dd3cc256ae2dff62ffb9d687aabb1; no Cargo executable descendants appear under those three at that instant. No OS lock owner, cross-resource cycle, deadlock or terminal result established. No process or lock was changed.

## Post-Source Snapshot

Root reports workspace 1764 source admission 1118/1118 and Hub 58834 1022/1022. At lease snapshot 1793881616420 captured workspace PID46992 and Hub PID26756 hold shared configuration view, registry and own build/store leases plus exclusive own-lane custody; no reported repo waiters or cycles. Admitted parent 47400 retains four jobs and 55588 one job. Process snapshots 1793881619469 and 1793881635819 show no Cargo/rustc executable descendants under those two at those instants. Direct Bun children are unit-cache gc --single-flight; latter snapshot shows four under workspace and seven under Hub. Their coexistence does not by itself prove blocking. Best-effort phase ledger has no retained marks for these exact PIDs. No compiler/test success or deadlock is established.

Second process sample 1793881702243, 66.4 seconds after 1793881635819, retains the same 11 GC child PIDs: workspace 22532, 33316, 53032, 56456; Hub 9156, 28140, 37316, 42780, 55868, 56860, 62288. Doctor reports cpuPercent 0 for each in this sample only; no CPU delta, deadlock or staleness follows. No Cargo/rustc descendant appears in that snapshot.

Superseding ledger location: Engine identified actual .cache/b/r/m/timing/cargo-phases.jsonl. Root reports PID46992 source:end and profile:ready then capacity:begin 1793881596073; PID26756 capacity:begin 1793881602871, with no retained capacity:ready or child:begin at that observation. Earlier empty helper ledger used a different ledger root and is not evidence of phase absence.

Root reports actual Hub 58834 compiler RED with four E0433 missing direct native-owned dependency; manifest repair and dual TOML owner verification followed. Doctor processfacts 1793883483615 show parent 55588 and captured 26756 remain, with direct unit-cache GC children 9156, 10968, 28140, 37316, 42780, 55868, 56860, 59820, 62288; no Cargo or rustc executable descendant appears at that instant. Snapshot is consistent with cleanup but does not establish outer terminal status. No process was cancelled.

WASM 13885 captured PID46216 correct-ledger bounded read: source:end 1793886546467, profile:ready 1793886586937, latest capacity:begin 1793886586938 (elapsed 11322085ms). Lease snapshot 1793887035652: shared registry/view60/compiler-writer and own-lane build/store, exclusive custody; outer PID43336 retains one-job grant, no reported waiter/cycle for captured PID. Doctor processfacts 1793887043862 shows captured PID and direct unit-cache GC child40152, no Cargo/rustc descendant at that instant. No retained capacity:ready or child begin in this ledger read; appends are best effort. No compiler/link result or specific capacity-wait cause follows.

## Current multi-root snapshot

At process snapshot 1793891956978, captured root children were 62600→50264, 65528→63752, 66516→22872, 20360→62356, 68048→68016, 48364→53708, 65632→61480, 40592→64552, 67944→66400, 61328→57824, and 63416→51700. Captured 63752 had Cargo child 70476, rustc wrapper 60016, and scheduler 61424. At 1793891981980 no rustc.exe descendant of 61424 was observed. Cargo has started for this root; this observation does not establish compiler progress or completion.

The current .cache/b/r/m/timing/cargo-phases.jsonl shows latest observed marks: 63752 child.begin (1793890882452); 62356 source.begin (1793890025979); 64552, 50264, 57824, 22872, and 53708 inputs.begin; 61480, 51700, 68016, and 66400 store.ready. The ledger is best effort, so absence of a later mark is not proof of a wait or terminal state.

Lease snapshot 1793892012295 explicitly shows 68016 queued for cargo:configuration-publication, held exclusively by 53708; 57824 holds exclusive cargo:registry-fetch. The 63752 process holds shared compiler host capacity and exclusive compiler CPU slot 2. No current nested scheduler queue or OS package-cache lock owner was established by these public observations. Different observed stages and the Cargo descendant disprove a claim that all roots share one demonstrated precompiler wait. No cycle or stale owner is established.

HTTP localhost:6063 remained ECONNREFUSED at 2026-11-05T15:21:39Z; no application readiness is established.

## Current global prerequisite trace

Public lease snapshot 1793892796714 shows exclusive registry-fetch held by 57824, standing resource-directory-admission:v1; configuration-publication had no holder or waiter in that snapshot. Thus the earlier 68016 waiting for 53708 is no longer observed.

Process snapshot 1793892852048 has 57824 under 61328 and 53708 under 48364, both Bun processes with no observed descendants; 68016 under 68048 has PowerShell 61464 and conhost 70252 descendants. Their ledger command is cargo:test. Last retained ledger marks are inputs.begin for 57824 and 53708 and store.ready for 68016. No conclusion of active fetch, successful completion, or failed cleanup follows from these observations. The registry-fetch lease is constructed by Cargo ready storage at 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts:114; a held lease alone does not identify active work.

Public snapshot 1793892900127 explicitly lists 50264 waiting for registry-fetch with blocker 57824/357. No holder or waiter has resource-directory-admission as its actual resource; that string appears in 57824 standing phase and cannot establish a current gate wait or gate owner. Current source 🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:299 and :374 acquires a shared directory gate to open a requested lease and releases it after acquisition. Captured implementation digest 8c274a8966893a8d is not proven identical to current source. No cycle is demonstrated.
