# Sol Independent P8 Draw Exact-Owner and Staged-Commit Re-Audit — 2026-08-23

## Verdict

**REJECT — Draw retained-load source cohort.** The latest remediation removes the previous shared
history-ID clone, asset `iter().nth` rescan, semantic-copy multiplier, and fabricated
`exact_for_test` seam. It does not yet establish the requested exact owner boundary: derived
allocation capacity is still projected rather than observed, BTreeMap backing is represented by a
synthetic pointer constant, aggregate admission occurs after large fixed authority owners already
exist, duplicate identity hashing does not frame the ID/name boundary, and the adversarial fixtures
do not drive the live initializer through aggregate-byte, container, commit, and publication
failure stages.

This is an independent Sol High source re-audit. I did not author the remediation and made no
production edit. Terra admission remained scheduler-blocked, so no Terra verdict is claimed.
Cargo, Nx, native, Wasm, browser, network, root lint, allocator/runtime timing, and hostile-valid
payload timing were not run.

Phase 8 remains **RED at 0/884 admitted commands, 18 global failure classes, and runtime
unverified**. Because this cohort is rejected, this report does not accept or decrement the raw
thirteen-caller structural remainder.

## Blocking Findings

### 1. The fixed census records projected sizes, not exact simultaneous allocation ownership

`DrawFixedOwnerCensus` is genuinely fixed at 4,096 slots, but the values written into those slots
are not the actual owners requested by the repair:

- `DrawSnapshotBoundsAuthority::string_owner` counts the source `String::capacity()` but predicts
  the derived String with `value.len()` (`owned/component.rs:818-820`).
- `vec_owner` likewise counts source `Vec::capacity()` but predicts the derived allocation with
  `value.len()` (`:822-829`), and `DrawSemanticDigestCredit::{derived_string,derived_vec}` repeats
  the same length-based projection (`:1976-1990`).
- The live clone then uses `try_reserve_exact` or `Vec::with_capacity` (`:992`,
  `:1023-1029`, `:1077-1078`, `:1128`, `:1364`, `:1628-1632`, `:1752-1760`, `:1836`).
  Rust guarantees at least the requested capacity, not equality. The code never reads the returned
  String/Vec capacities and never reconciles excess backing against the aggregate claim before
  publication.
- BTreeMap allocation is not observed at all. Every source/candidate asset entry is charged the
  hard-coded `DRAW_ASSET_MAP_NODE_POINTERS = 4` (`:670`, `:962`), while the candidate inserts into
  a live `BTreeMap` (`:1406`). That constant is neither allocator-returned backing nor a
  schema-first fixed map/page authority.

There is also an ordering inversion. `DrawMutationCandidateAuthority::new` constructs the first
4,096-slot source census at `:3127`; `PreflightMutation` constructs the second 4,096-slot digest
census at `:3249`. Only after both exist does
`DrawMutationAggregateReservation::admit` add
`size_of::<DrawMutationCandidateAuthority>()` as `authority_bytes` (`:2852`). Thus aggregate
rejection observes already-created authority ownership rather than reserving it before
construction.

The container cursor prevents logical growth beyond its requested element counts, which is useful,
but it does not cure the missing actual-capacity/map accounting. The asserted 4,096-item /
262,144-byte boundary is therefore not exact for every simultaneously retained original,
candidate, mutation, container, map, page, index, digest, and output owner.

### 2. Duplicate rewrite is staged, but its alleged length-framed identity is ambiguous

`DrawDuplicateRewriteAuthority` now separates ID copy, name copy, hash initialization, hash pages,
new-ID allocation/install, suffix reservation/append, and old-ID retirement across retained
phases. However, `id_len` and `name_len` are only assigned and reset (`:2903-2904`,
`:2954`, `:2965`, `:3024-3025`). They are never observed by the hasher.

Hash initialization observes only the combined `material_len` (`:2972`), then the hash cursor
writes the raw concatenation `id || name` (`:2980`). For the same layer variant, owners
`id="ab", name="c"` and `id="a", name="bc"` therefore present the same total length and
identical bytes and mint the same replacement ID. This contradicts the implementation report's
“length-framed hash initialization” claim and does not preserve a schema-exact ID/name boundary.

The permanent predicate requires the authority name, fixed material array, and hash cursor, but it
does not require separate ID/name length frames. No differential fixture changes only that
boundary split.

### 3. Boundary and cancellation fixtures do not exercise the claimed live owners

The new aggregate fixture is not an exact live aggregate boundary:

- the 4,096/4,097 check is a **single field** limit, not the 262,144-byte aggregate limit;
- the layer loop accepts either item or byte rejection and records only the last sub-cap totals, so
  it never constructs an exact aggregate-byte boundary plus one;
- `source.layers.as_ptr()` is checked while `live_reservation` merely borrows the source, making
  that pointer preservation tautological. It does not hand a mutation/candidate/container owner
  into admission and recover the exact rejected backing;
- the final assertion checks `source.id`, not the recursive mutation, candidate, rebuild buffers,
  or displaced owner identities.

`retained_draw_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid` does use
a real cancellation token and mismatched generation, but only against
`DrawMutationCandidateAuthority`. It directly calls the candidate close helper afterward. It
does not drive `DrawStoreInitializationAuthority` through forward/inverse digest, applied/redo
preparation, `CommitApplied`, `CommitRedo`, candidate installation, displaced-store retirement,
generation swap, or ACK. The live editor fixtures cover one successful rename/ACK, partial ingress
cancel, and hostile edit ID; they do not supply those missing commit-stage cancellation/stale or
aggregate-saturation handback cases.

The all-fourteen-candidate fixture proves that each variant reaches terminal without error and can
be retired, but it does not assert direct-vs-retained semantic results for every mutation. The
schema-digest fixture is materially stronger and should be preserved; it does not substitute for
the missing live owner/cancellation boundary.

### 4. The shared audited source does not pass its declared formatting gate

Edition/style-2021 rustfmt passes the owned Draw file, editor, and subset glue. The same scoped
command fails on the shared store file with live drift at its import order, public VCS re-export
order, and the linked-document fixture around lines 26, 1933, and 22036. I made no formatting or
production edit. The semantic rejection above is independent of this gate failure.

## Accepted Source Evidence

The following improvements are present and should be retained:

| Requirement | Source result |
| --- | --- |
| Fixed recursion and census shells | PASS structurally: depth 64, fixed 4,096 census slots, fixed traversal/path arrays, no resizable census. |
| Retained clone taxonomy | PASS structurally: seven layer variants, Group children, fills, both gradients/stops, stroke strings/dashes, paths/segments/points, Boolean operands, Trace/Text/Image fields, and assets advance through typed phases without a whole-tree clone/serde route. |
| Schema-complete mutation digest | PASS structurally: repository-owned SHA-256 frames the fourteen mutation discriminants and nested layer/style/geometry fields. |
| Shared applied/redo ID ownership | PASS narrowly: `CursorRevisionRecord` stores fixed `[u8; 32]` digests; `push_applied`/`push_redo` hash borrowed IDs and move each prepared history String into its lane without `id.clone()`. |
| Duplicate retirement stages | PASS except framing defect: new/old IDs are retained in terminal-asserting slots and one owned String is retired per grant. |
| Asset iteration | PASS: fixed key cursor plus ordered `BTreeMap::range`; no `iter().nth` rescan. |
| Forbidden retained replay seams | PASS: zero source/snapshot whole clone, whole operation encode, diff/apply, serde reconstruction, whole metadata scan, `exact_for_test`, or `structural_copies` in the owned file. |
| Browser-facing route | PASS structurally: begin/page/seal/poll/generation replacement/idempotent ACK/cancel/one-grant close remain wired, with no whole-buffer placeholder in Draw. Runtime behavior is unexecuted. |

## Source Census and Gates

| Gate | Result |
| --- | --- |
| Draw owned/editor/glue rustfmt, edition/style 2021 | **PASS** |
| Shared store rustfmt, edition/style 2021 | **FAIL**: live formatting drift; no audit edit |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | **PASS: 245 self-tests clean** |
| Draw retained predicate in full verifier | **PASS mechanically**; no Draw-specific verifier failure |
| Full tool-job verifier | Expected **RED**: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importers, 34 globals, **18** failures |
| Broad interactivity self-test and plain DENY | **PASS**: one recorded allowlisted test bridge, zero unlisted findings |
| Production placeholder census | Raw source is one shared definition plus 13 live callers; Draw has zero. Cohort count is not accepted because verdict is REJECT. |
| Deterministic ledgers | Four Draw owner/tool current/repeat files are byte-identical, 312,305 bytes, SHA-256 `c6285afecde02b6005349bc05f24009996ab9c3a4842ce34fd5c9f1008617472` |
| Scoped and whole working/staged/HEAD `git diff --check` | **PASS** |
| Cargo, Nx, native, Wasm, browser, network, root lint, runtime/allocator timing | Not run; **RED/unverified** |

## Required Repair

1. Replace projected derived lengths and synthetic BTree node costs with schema-first fixed
   String/sequence/map pages, or reserve and reconcile every allocator-returned capacity while exact
   owners remain recoverable and before any publication. Reserve the fixed authority/census/digest
   shells before constructing them.
2. Frame duplicate ID and name independently (domain, ID length+pages, name length+pages) and add a
   split-boundary collision fixture plus verifier mutation.
3. Add real aggregate item and byte boundary/+1 fixtures that enter ownership admission, recover
   exact source/mutation/candidate/container pointers/leases, and prove no growth or publication.
4. Drive the live store initializer with cancellation and stale generation at candidate,
   container, applied/redo commit, candidate installation, displaced retirement, and ACK stages;
   assert exact terminal handback and unchanged last-valid generation.
5. Canonically format the shared store under its manifest edition and rerun the serialized Rust and
   browser/runtime gates when authorized.

Until those source and evidence gaps close, Draw remains **REJECTED** and Phase 8 remains
**RED: 0/884, 18 failure classes, runtime unverified**.
