# 1030 review-today (review1030, read-only, no builds)

Scope read: derive(Mutations) const redesign, set-field leaf, StepContext rebase, jobpage, hostfix H5, actinfra queue/lender/grow/graph-cache, deps headroom, fem SealedShared Clone, energy RetainedRowPatchApply, perf catalogue cache, i18n. Not reviewed in depth: tsfix record codec/poll-lanes, scratch leases, ramp idle grants, cache-grow resize/restamp bodies, puzzle5d projection memo, TS pager (spot-read only). Every item re-read at the cited lines.

## Findings, most severe first

### 1. `SealedShared<T>: Clone` is an unfunded alias that panics on plain drop (HIGH, confidence high)
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs`, `impl Clone for SealedShared` (line ~106) next to `impl Drop` (line ~108).
- Clone creates a live alias via `source.duplicate().expect(..)` with no grant (the funded form is `try_duplicate(grant)`), and `Drop` asserts `source.is_none()` unless panicking. A `.clone()` that compiles (any `let id = node.id.clone();` used as `&str`, `Display`, map key by deref, `==`) and is later dropped normally aborts with "sealed source requires original granted handback" at runtime, not compile time. Also two `.expect` in non-test code (policy), and a transferred source panics inside `clone`.
- The fem lane fixed only compile-visible sites (5). Silent sites cannot be grepped; candidates: `SealedShared<String>` rows in fem engine (`FemNode.id`, `FemMaterial.{id,name}`, 2d engine/mesh/editor `node.id.clone()` users) and energy native rows (`🔋️energy/.../🧬️mutations/🧳️native*`, `🧳️state`, `🔺️diff`).
- Failure scenario: any generic helper taking `impl Into<String>/AsRef<str>` or `format!("{}", id.clone())` on those rows panics on the first edit of a loaded document.
- Fix direction: no `Clone` (remove impl; keep `try_duplicate`), or make `Clone` impossible to drop unfunded by design; do not ship alias-Clone for a type whose Drop panics.
- Owner: fem (introduced), energy + fem2d (audit sites).

### 2. Wait-marker lending probably never fires and, where it does, can over-admit (MEDIUM, confidence medium)
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts:116` `hostTreeOnlyWaitsV1`, used at `:291`.
- Rule: EVERY live descendant of the holder (full descendant list from `liveChildPidsV1`, wrappers included) must own a marker. Intermediate processes (`bun --eval` body, `repo` CLI, shells) that merely await their waiting child declare no wait edge, so they have no marker and the tree never counts as "only waits". The 14:56 convoy (holder -> bun body -> waiter) is exactly this shape. Tests use synthetic flat markers, so they cannot show it. Unverified live.
- Opposite risk: marker files of a killed (SIGKILL, no `exit` handler) waiter stay until pid death; pid reuse by a running compile makes `alive(pid)` true and the stale marker counts it as waiting, lending a reservation while it compiles (memory over-admission).
- Fix direction: treat a descendant as waiting if it or all its own descendants wait (recursive), or have `awaiting child` edges declared; validate marker content against process start time.
- Owner: actinfra.

### 3. Fuel accounting: a zero-fuel context still funds work, and `None` is unreachable (MEDIUM, confidence high)
- `✏️s/.../geometry/⚙️compute/🦀️.rs:45` `fuel_budget` = `fuel_remaining().max(1)`; `🧱️mesh-support/🦀️.rs:255-265,348` (`MachineJob::step`, `Modeling::advance`) pass `fuel_budget(cx) * BATCH` (=256 kernel units) to the kernel even when `fuel_remaining()==0`.
- Doc says `None` = grant admitted no progress; `MachineJob::step` always returns `Ok(Some(..))`. So an exhausted/zero-fuel step silently runs 256 units and `spend` then saturates at 0 (no accounting trace beyond `work_units_spent`). Also: Working charges the whole budget, Done charges 1 regardless of kernel units used (under-billing), and brep/tessellate charge `budget` while other paths charge `1` or `fuel.min(3)` (inconsistent law).
- Fix direction: `if cx.fuel_exhausted() { return Ok(None) }` before any kernel call; bill kernel-reported units.
- Owner: stepctx.

### 4. Hostfix H5 lend: leak/stall if the stepping handle is dropped while holding the job (MEDIUM-LOW, confidence medium)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs:56-80,116`.
- `held` is `ManuallyDrop` and `Drop` asserts only `state.is_none()`. A stepping handle dropped without `return_held` (cancel path that never reaches `close_step`, panic unwound elsewhere) leaves `state.lent=true` forever: every clone yields (`admit_yield`) endlessly, and the boxed job leaks silently. `begin_close` ignores `return_held()==false` and then `try_lock` failure skips `inner.begin_close()`, with no retry later in `close_step` (close_step never calls `inner.begin_close()`), so a busy gate at that instant loses the begin-close for the inner job.
- Owner: hostfix.

### 5. Retained set-field leaf (UNCOMPILED) risks (MEDIUM-LOW, confidence medium)
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🎛️set-field/🦀️.rs`.
- `Drop` (line ~143) requires `closing`: a leaf that is created (`new`) and dropped without `begin_close` (admission refused before custody) panics even though it holds nothing; `terminal_is_empty` shares the `closing &&` clause. Other leaves in-tree test emptiness only.
- `decide` phase 3 entered with `authored.value().is_some()` reports `receipt(d)` (an exchange-demand charge) for a phase that did no work, then `admit_retained_clone_progress` accepts it: over-billing, not unsafe.
- `unsafe { ManuallyDrop::drop }` x2 in Drop is only sound if empty; correct, but the `inverse_phase`/`phase` machine has no test (energy law test still queued). Not proven by compile; treat as unverified.
- Owner: mutderive.

### 6. Cache grow resume trusts stale journal `oldDevice` / skips drain on resume (LOW-MEDIUM, confidence low-medium)
- `🛍️products/🦑️repo/.../📦️budget/📏️grow/🟦️.ts:145,150-171`.
- Journal records `oldDevice` at planning. After an aborted attempt the fleet resumes (image may be re-mounted with a new device) and phase `drained` is already persisted, so a resume skips the holder check and uses the stale `oldDevice` in the restamp (`restampChildReceiptTextV1(text, oldDevice, newDevice)`), rewriting wrong device ids if the mount device changed between attempts. Detach retry then fails busy again instead of draining first.
- Fix direction: on resume re-read `deviceOf(mount)` when the image is attached and phase < `detached`; re-run the holder check when resuming from `drained`.
- Owner: actinfra.

### 7. Holder-reentrant shared admission past writers (LOW, confidence medium)
- `🔨️modules/🏃️process/🔒️leases/⚖️fairness/🟦️.ts:20-25` `queueMayProceedV1`: `requester.mode==="shared" && requester.holdsShared` passes any waiting writer. Deadlock argument is sound for the holder tree, but a long-lived holder tree that keeps issuing new shared tickets (cargo test fan-out) starves the grow/exclusive writer unboundedly; the 1 h preference no longer bounds it. Drain times 20-34 min and the aborted 29 min wait are consistent with this. Add a writer-starvation bound (e.g. after N minutes only already-admitted descendants, not new tickets, pass).
- Owner: actinfra.

### 8. Derive redesign: completeness is a count, hash doc drifted (LOW, confidence medium)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔁️mutations-expansion/🦀️.rs:217-282`: `scope_counts[idx] == registered` is a bijection only if `validate_leaf` pins each leaf to a distinct registered (owner,kind) and a leaf validating in two overlapping scopes is counted for the first only (loud false failure, not silent). Sound; relies on the roster uniqueness check using the aggregate-level `owner_layout` while components may carry different layouts: verify uniqueness is keyed by owner only.
- Result file says "last 48 bytes"; code is head 16 + 32-stride samples + tail 32 (`🪪️leaf-descriptor-validation/🦀️.rs:168-201`), tests updated by a peer. Docs stale, behavior fine; worst-case probing is O(n^2) with collisions, bounded by the `>= SLOTS` panic.
- Owner: mutderive.

### 9. i18n (LOW, confidence high)
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🟦️.ts`: `DEMONSTRATOR_LOCKED_LOCALE` from catalog `lockedLocale: "de"` is itself a default language (explicitly allowed only by the German lock; brand content is German-only). `demonstratorFormat` turns a missing placeholder into `""` silently (`params[name] ?? ""`); `demonstratorText` returns `undefined` for an unregistered locale at runtime (type-only guard). Same `?? ""` pattern in headroom `headroomTextV1`.
- `🧰️framework/.../🌊️flow/🗂️catalogue/🦀️.rs:132-` static catalogue sections are English literals ("Inputs", "Slider", ...); the new `catalogue_sections` cache (`✏️s/.../🌊️flow/.../✏️editor/🖥️host/🦀️.rs:24`) is one process-global slot keyed by (registry generation, extras text) and hands out the English literals to every locale; two windows with different extras thrash the single slot (perf only).
- Owner: i18n (notes), perf (cache slot).

### 10. Energy RetainedRowPatchApply boxing (looks correct, one note)
- `🧰️framework/.../🧳️retained/🩹️patch/🚚️apply/🦀️.rs:28,39,68-70`: phase-0 quote charges `capacity_bytes=size_of::<Fields>()`, close releases the same; `child()` subtractions are guarded by the preceding demand-fit check. `Box::new(Fields::new(..))` builds `Fields` on the stack first (several KB for energy), then moves: transient stack use not covered by the grant; acceptable but note for wasm stacks. No defect found.

## Checked, no defect found
- jobpage: guest `read_job_output_step_with` offset/ack ordering, native `collect_job_output` pending bound (1024) and offset check, `for_output_page` narrowing only the copy axis.
- graph cache own-group LRU: header-first unit order, protected idle time, index compaction (note: eviction is by write time, not read time; an unlink error other than ENOENT aborts before `compactGroupIndexV1`, leaving a stale index until the next pass).
- headroom `stranded grace`: only affects the `alone` branch, requires `reclaimed`, outstanding bytes of an old row are already 0; reasonable.
- RetainedRowPatchApply, pagers, deps registry reader selection (read via result file only; code not re-read).
