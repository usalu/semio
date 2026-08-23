# Coordinator Independent P1q Retained-byte Credit Reopen Audit — 2026-08-23

## Verdict

**RED — the earlier source-only P1q acceptance is reopened.** The retained I/O scheduling and exact
top-level `Vec` handback improvements remain useful, but the byte/page accounting is logical rather
than actual and the terminal disposer recursively drops uncensused owners. P1q cannot be a final
Phase 1 foundation in its current form. No Cargo, Nx, Wasm, browser, runtime, timing, or network gate
was run.

## Trigger

The P1w initial-catalog CAS census requires credit for the exact allocated backing of its
`DbIoPages`, not merely its logical byte length. Independent inspection of the shared P1q boundary
shows that the current `DbIoPages` contract cannot provide that proof.

## Blocking Findings

### P1q-R1 — `DbIoPages` contains one ordinary `Vec`, not owned 16 KiB pages

`DbIoPages::try_new` checks `owner.len()` and computes `page_count` from the length, but retains the
original `Vec<u8>` unchanged. Its allocation capacity, allocator rounding, and control owner are not
measured. A one-byte vector with a very large capacity is admitted as one logical page. `page()`
returns slices into that same unrelated allocation; it does not prove that one exact page is the
actual storage.

`try_range` has the same defect and `into_vec` allocates a new suffix when `start != 0`, outside any
visible page reservation. The max/+1 fixture checks only logical length and value equality, so it
does not discriminate this failure.

Required repair: use actual owned <=16 KiB page allocations assembled through pre-admitted page
writers, or credit the exact observed live allocation capacity/control owner and retain an exact
incremental disposer. Do not copy an already-retained range after admission.

### P1q-R2 — operation admission trusts estimates rather than the transferred owner graph

`DbIoRequest::admitted_bytes` adds caller-supplied `input_bytes`, `output_bytes`, and an estimated
list scalar size. It does not census the captured closure, input vector capacity, path/document/key
strings, backend/control owners, nested list strings, or actual result allocation. The fixed 16 KiB
base surcharge is not an exact authority for those owners.

The generic `DbIoOperation<T>` then accepts any closure and `T`; neither is required to carry the
reservation that owns its storage. A caller can understate the request or a backend can return a
larger/capacity-heavy result while the operation retains only the estimated credit. Exact item/byte
and process caps are therefore not enforced by construction.

Required repair: schema-first typed I/O operations must admit each exact input/control/output page
before transfer and construction. Output allocation must use an admitted retained writer rather
than allocating arbitrary `T` and checking afterward. External backend types stay behind owned
interfaces.

### P1q-R3 — close drops whole captured/result graphs

`DbIoState::close_one` takes and drops a retry job, work closure, terminal work closure, or generic
result in one grant. A closure may capture page vectors, strings, paths, keys, and Arcs; a result may
contain nested vectors/strings/maps. Dropping the one outer box/result recursively releases all of
those allocations. `terminal_is_empty` witnesses absence after the bulk destruction, but there is no
per-owner/page credit return.

Required repair: terminal work/result must be typed retained authorities with field/page close
cursors. One close grant releases at most one exact owner/page/control backing; the operation slot
and process byte credit return only after an exhaustive nonopaque empty witness.

### P1q-R4 — backend calls still allocate/copy whole buffers inside one opportunity

Memory, filesystem, and SQLite-shaped paths still reach operations such as `extend_from_slice`,
`to_vec`, whole file reads/writes, atomic catalog-buffer construction, hashing, and generic closure
execution in one worker opportunity. The historical reports correctly listed indivisible syscalls as
residuals, but whole in-memory allocation/copy is also source-visible and remains outside the 8 ms
and cancellation contract.

Required repair: cursorize owned in-memory copies/hashes/encodes through admitted pages. Platform
syscalls must move to the final owned platform boundary with explicit observed latency evidence and
must not conceal additional whole buffer construction on either side.

## Required Fixtures and Verifier Mutations

- vector length 1 with capacity above every operation/process cap;
- range start with multi-page backing and zero post-admission suffix allocation;
- exact page max/max + 1, aggregate process max/max + 1, and identical rejected owner/page identity;
- captured path/key/document strings and result/list nested capacities at max/max + 1;
- producer that attempts to emit more bytes/pages than reserved and cannot allocate outside its
  writer authority;
- rejection, cancellation, panic, retry saturation, receiver/future Drop, and shutdown with
  interrupted one-page close and exact aggregate credit return;
- mutations restoring logical-length credit, decorative page views, arbitrary generic closure/result
  ownership, `into_vec` suffix copy, bulk closure/result drop, and whole in-worker buffer copies.

## Dependency Consequence

P1w and P1x both transfer catalog encodings into `DbIoPages`; they must not claim exact retained byte
ownership until P1q-R1 through P1q-R3 are repaired or they use a stronger owned-page authority that
also replaces the shared deficient boundary. The already-removed inline/fallback storage routes
remain removed and need not be reintroduced.

P1q and Phase 1 remain open.
