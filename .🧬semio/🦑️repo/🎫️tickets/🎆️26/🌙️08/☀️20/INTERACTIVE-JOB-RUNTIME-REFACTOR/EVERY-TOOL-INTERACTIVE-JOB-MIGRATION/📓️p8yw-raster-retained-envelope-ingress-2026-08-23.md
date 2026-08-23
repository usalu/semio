# Phase 8yw — Raster Retained Envelope Ingress

Date: 2026-08-23  
Owner: `/root/p3_browser_worker_repair`  
Verdict: source-audit-ready; acceptance remains pending an independent audit.

## Pre-edit exact caller census

The exact production scan

```text
rg -n "reject_whole_buffer_artifact_envelope_ingress" --glob '*.rs' .
```

returned 13 occurrences before this packet: one shared fail-closed definition and 12 live raw whole-buffer callers.

| Role | Exact source |
|---|---|
| Shared definition (out of scope) | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7926` |
| Dag caller (out of scope) | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:8925` |
| Flow caller (out of scope) | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:844` |
| Procedural3d caller (out of scope) | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |
| Fem2d caller (out of scope) | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` |
| Procedural2d caller (out of scope) | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |
| Fem3d caller (out of scope) | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` |
| Process3d caller (out of scope) | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:36` |
| Cad caller (out of scope) | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:24` |
| Puzzle5d caller (out of scope) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` |
| Puzzle3d caller (out of scope) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` |
| Shooting caller (out of scope) | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:31` |
| Raster caller (this packet) | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |

The required post-edit structural target is therefore exactly 12 occurrences: the unchanged shared definition plus the unchanged 11 non-Raster callers.

## Pre-edit Raster owner census

The only Raster raw ingress is the WASM `RasterArtifactVcs::new(envelope_json: &str)` path. It materializes a borrowed whole JSON string into `RasterEnvelope`, then constructs `RasterStore` in the same call. The owners reachable from this boundary are:

- `RasterEnvelope = store::ArtifactEnvelope<RasterSnapshot, RasterMutation>` at `.../🧬️schema/🧬️mutations/🦀️component.rs:69`;
- `RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>` at the same file's line 70;
- recursive `RasterSnapshot` at `.../🧬️schema/📸️snapshot/🦀️component.rs:30`, including `String`, `Vec<RasterLayerNode>`, and `BTreeMap<String, RasterAssetChild>` owners;
- twelve-variant `RasterMutation` at `.../🧬️schema/🧬️mutations/🦀️component.rs:41`, including nested layer/asset owners;
- the JS-facing `RasterArtifactVcs.store: RefCell<RasterStore>` owner in the scoped WASM component.

No second Raster raw caller or compatibility constructor exists in the pre-edit census.

## Packet invariants

- Raster alone changes; the shared fail-closed definition and other raw callers remain byte-for-byte out of scope.
- Ingress is fixed-page and pre-admitted before copying bytes.
- Operation/page/byte credits, generation, checkout, ACK, cancellation, handback, and close are explicit fixed authorities.
- Decode/build advances only through bounded poll grants; ordinary constructor/request paths do no whole-buffer decode.
- Rejection returns the exact generation/page owner; stale/duplicate tokens cannot mutate a reused slot.
- Terminal close retires one page/root/scalar per grant and releases admission only after a terminal-empty witness.

## Implemented source packet

### Raster Wasm caller

The Raster Wasm bridge no longer accepts or parses a whole envelope string. `RasterArtifactVcs`
owns the established `VcsArtifactApp<EditorApp<RasterPlayApp>>` runtime and exposes only the retained
page lifecycle:

1. `beginEnvelopeLoad(maximum_pages, maximum_bytes)` admits nonzero credits no larger than the
   framework's fixed 64-page / 262,144-byte operation authority and returns the by-value
   `(operation, generation)` handle.
2. `admitEnvelopePage(handle, Uint8Array)` checks the 4,096-byte page cap and calls
   `preflight_artifact_envelope_ingress_page` before the producer closure copies into the fixed
   `[u8; 4_096]` page. The callback is never entered on stale/saturated/oversized admission, so the
   JS caller retains its exact source owner.
3. `sealEnvelopeLoad` transfers the exact page authority to the retained decoder.
4. `pollEnvelopeLoad` gives maintenance one item / one page-byte grant, returns explicit progress,
   and acknowledges the exact operation-generation only after a replacement candidate is ready.
5. `cancelEnvelopeLoad` and `closeStep` route through the same retained runtime; there is no direct
   store constructor, direct dispatch, projection JSON, envelope JSON, bulk loop, or inline fallback.

### Raster-owned domain catalog and initializer

`RasterEnvelopeOwnedFieldCatalog` now owns snapshot, mutation, VCS, edit-history, and conflict field
authorities. Snapshot and mutation pack fields are admitted into the fixed 4,096-byte
`OwnedSchemaHexAuthority` before domain decode. Publication transfers the sole decoded owner; fault
and cancellation transfer it to `RasterOwnedRetirement`.

The retirement taxonomy is exhaustive for:

- the snapshot's schema/id/title, layer vector, asset map, and composed child handles;
- Pixel, Group, and Adjustment layers, including nested children and recursive `DslValue` params;
- all twelve Raster mutations, boxed layers, image-asset bytes, and all strings; and
- snapshot `Arc` roots and store/displaced-store ownership.

All recursive owners sit behind `ManuallyDrop` terminal assertions. A close grant advances at most
one nested owner, one string, one byte vector, one empty container backing, or one scalar shell.
The retained store initializer validates operation/generation and edit identities, clones one
admitted scalar/layer/asset per grant, seeds and replays history one entry or mutation per grant,
retires each displaced snapshot incrementally, and publishes only through checked
`generation.checked_add(1)`. Cancellation, stale authority, and fault all pump the same terminal
retirement path before returning their terminal result. Its entry guard observes cancellation and
the `StepContext` deadline/yield authority before either the active owner or retirement cursor is
advanced; a zero-fuel grant advances neither phase nor owner.

`RasterPlayApp` now supplies the Raster envelope catalog, member-store owners, retained
initialization job, and document-store disposer. There is no compatibility constructor.

## Focused fixtures and permanent mutations

Rust source fixtures cover:

- next-generation candidate publication followed by one-owner document-store close;
- cancellation and stale-generation fault reaching terminal-empty;
- a zero-fuel/deadline grant advancing neither initializer phase nor retained owner before a later
  cancellation pumps the same owner to terminal-empty;
- nested Group/Adjustment/`DslValue` and composed-child retirement with one-owner grant assertions;
- all twelve mutation variants, zero-fuel ownership, per-turn item/byte ceilings, and terminal-empty;
- exact 4,096-byte / 64-page / 262,144-byte caps and a page-size +1 rejection returning the same
  fixed owner contents; and
- 4,096-byte domain-string max/+1 admission.

The permanent verifier now rejects, independently: missing preflight; copy-before-preflight; fixed
page replacement with `Vec`; operation-generation handle erasure; unchecked generation increment;
missing deadline/yield admission; missing worker-cancellation admission; a weakened zero-budget
fixture; one missing mutation variant; recursive-value deep drop; missing nested-close fixture;
missing domain cap/terminal fixture; missing page +1 fixture; dynamic post-lift slice input;
whole-buffer string reintroduction; missing exact ACK; cancel-by-drop; bulk close; missing
fixed-registry +1 exact handback; and missing interrupted one-page close.

## Post-edit exact census

The same production scan now returns exactly **12** occurrences: the unchanged shared fail-closed
definition plus the unchanged eleven non-Raster callers. Raster has zero occurrences. The remaining
callers are Dag, Flow, Procedural3d, Fem2d, Procedural2d, Fem3d, Process3d, Cad, Puzzle5d, Puzzle3d,
and Shooting.

## Source gates

| Gate | Result |
|---|---|
| exact whole-buffer symbol census | **PASS**: 13 → **12** occurrences; Raster zero |
| edition-2021 scoped `rustfmt --check` | **PASS** on the Raster codec, editor, and Wasm bridge |
| Bun TypeScript/parser + permanent tool-job verifier | **PASS**: **309** self-tests clean |
| broad `bun ./📜️script.ts verify interactivity --format json` | **BLOCKED by concurrent unrelated source drift**: the rerun stopped in the live-reconcile self-test because the mounted `PatchTracker` predicate no longer matched; the Raster-focused verifier remained clean |
| live Raster retained-route predicate | **PASS**: neither deterministic ledger contains a Raster-specific failure |
| deterministic ledgers | **PASS**: byte-identical `📊️p8yw-raster-ledger-a.json` / `-b.json`, SHA-256 `ed38c717f71a6b0bc05da53925953fb33ad427b1c6788a3748b1da8db1ee3fbf` |
| full tool-job verifier | expected global **RED**: **0/884**, 18 failure classes; no Raster-specific failure |
| scoped and whole working-tree `git diff --check` | **PASS** |
| Cargo / Nx / native / Wasm / browser / runtime / network | **not run by instruction** |

The packet is **source-audit-ready, not accepted**. Rust typechecking, the Rust fixtures, Wasm
generation, and real browser/runtime behavior remain explicitly unverified until the serialized
build lane is opened. Phase 8 remains globally RED for the eleven raw callers and the ledger's
other declared cohorts.

## Independent RED remediation

The five source blockers in
`coordinator-independent-p8yw-raster-retained-ingress-final-audit-2026-08-23.md` are repaired in
the scoped Raster codec and permanent verifier. This is an implementation handback for independent
re-audit, not an acceptance claim.

### Recursive preflight and construction

`RasterSnapshotBoundsAuthority`, `RasterLayerBoundsAuthority`, and
`RasterDslValueBoundsAuthority` now census the recursive owner graph with retained fixed paths and
frames. They account source and simultaneous candidate item/allocation-capacity bytes before the
corresponding construction phase. `RasterSnapshotCloneAuthority`, `RasterLayerCloneAuthority`, and
`RasterDslValueCloneAuthority` then construct one admitted field, key, value, child, or asset field
per granted turn. All governed work calls `raster_reserve_unit` before allocation/copy/traversal;
zero fuel, insufficient low fuel, and an expired deadline advance no owner or cursor.

The production retained route contains no `serde_json::to_vec(source)`, recursive
`source.clone()`, `operation.diff(current)`, or `diff.apply(current)` admitted unit. Asset child
hashing/construction is also retained: MIME, capped byte owner, key, child ID, artifact ID, and each
dialect field advance separately before the exact child is published.

### Typed history replay and atomic candidate publication

`RasterMutationDigestAuthority` hashes the complete twelve-variant mutation taxonomy one bounded
scalar/string/page/layer unit at a time. `RasterMutationCandidateAuthority` cursor-clones the
current snapshot, locates nested targets with a fixed address, stages strings/layers/assets, and
applies vector shifts one swap per grant. The prior snapshot is unchanged until the complete
candidate swaps atomically into the retained initialization runtime; the displaced snapshot enters
the existing one-owner retirement path before the next history mutation.

Insert preparation has its own `BeginInsert` turn, so parent lookup and vector publication never
share a fuel opportunity. Initializer active-owner retirement also returns immediately after one
retirement action rather than continuing into another phase in the same step.

### Exact terminal ownership

An unclaimed completed store is now included in terminal retirement through
`ArtifactDocumentStoreDisposer`; cancel/stale/fault after `Complete` cannot declare terminal while
`candidate`, `mutation_candidate`, or `candidate_disposer` remains. The public generic initializer
wrapper uses `ManuallyDrop`, refuses Drop until exact candidate handoff/terminal retained close, and
the mounted ingress recovery path routes every non-taken terminal through
`retain_initializer_for_close(job)`. This proves fail-closed retained ownership rather than a silent
candidate destructor; the eventual runtime gate still has to exercise the mounted abandon path.

String and byte owners now compare/report allocation `capacity()`, as do vector backings. The first
remediation introduced a fixed iterative retirement stack and an empty-map shell fixture. The
second remediation below supersedes both mechanisms with a fixed-page map authority and paged
retirement stack that account populated-map and control backing explicitly.

### Added focused source fixtures and mutations

Rust fixtures now cover:

- a 48-level snapshot clone with fuel `1` and exact source/candidate equality;
- an expired deadline that advances neither bounds, clone, nor mutation ownership;
- a small rename against a 40-level snapshot with unchanged source until atomic publication;
- cancel after `Complete` with a 24-level unclaimed candidate and exact disposer terminal;
- allocation-capacity string/byte retirement and fixed iterative layer/value depth;
- exact nested item/byte maximum and `+1` preflight failure; and
- allocation-free empty asset-map shell retirement.

The permanent verifier now discriminates removal of the recursive bounds authority, pre-work fuel,
typed mutation candidate/digest, completed-candidate disposer, capacity accounting, fixed iterative
frames, each hostile fixture, public initializer Drop refusal, and reintroduction of whole recursive
encode/clone or monolithic diff/apply.

## Remediation gates

| Gate | Result |
|---|---|
| exact whole-buffer symbol census | **PASS**: exactly **12**, one shared definition plus eleven non-Raster callers; Raster zero |
| scoped edition-2021 `rustfmt --check` | **PASS**: Raster codec, editor, and Wasm bridge |
| permanent tool-job verifier self-test | **PASS**: **318** self-tests |
| live tool-job source verifier | expected global **RED**: **0/884**, 19 declared failure classes; no Raster-specific failure |
| deterministic ledgers | **PASS**: byte-identical A/B, SHA-256 `0619e32db7b59923e5e4df3038249db97172d0ff717fbd9b39cb79e14c51be9b` |
| scoped and whole working-tree `git diff --check` | **PASS** |
| Cargo / Nx / native / Wasm / browser / runtime / network | **not run by instruction** |

Final remediation verdict: **source-audit-ready, not accepted**. Rust typechecking and execution of
the new Rust fixtures remain unrun; the serialized native/Wasm/browser gate remains mandatory.

## Second independent remediation

The four blockers in
`coordinator-independent-p8yw-raster-second-remediation-reaudit-2026-08-23.md` are repaired in the
Raster-owned schema containers, retained initializer, retirement authority, fixtures, and permanent
verifier. This remains an implementation handback for another independent source audit.

### Fixed-page map ownership

Retained asset and adjustment maps no longer depend on `BTreeMap` node allocation or deallocation.
`RasterOwnedMap` admits at most 64 semantic entries in eight-entry pages. Every page has a
conservative 16 KiB credit independent of standard-library or allocator layout; the concrete
code-owned page must fit inside that credit before allocation. Populated retirement removes one
semantic entry per grant and subsequently detaches and releases one empty page backing per grant.
The final empty shell owns no hidden node allocation.

Replacement is exact-owner preserving. The incoming `(String, V)` pair atomically replaces the
stored pair and returns the exact displaced `(String, V)` owner. Retained consumers cursor-retire
both the displaced key and value. The pointer fixture uses a deliberately over-capacity incoming
key and proves both the installed incoming allocation and returned old key/value allocations are
unchanged. Capacity `+1` still returns the exact rejected key/value pair.

The permanent verifier rejects whole-map Clone, serde, or Dsl materialization in the retained codec
and rejects ordinary populated-map Drop. General schema trait implementations remain outside this
retained initialization route; the retained root is `ManuallyDrop`-owned and can only release its
populated pages through `RasterOwnedRetirement`.

### Observed allocation capacity

Strings are built from exact boxed byte slices and retain their observed capacity. Candidate Vec
allocations are inspected immediately after allocation; any allocator-provided capacity above the
requested amount is added to the simultaneous candidate ledger before construction continues. An
over-credit candidate remains owned by the clone authority and is retired incrementally. The
allocator-over-capacity discriminator and requested-vs-observed mutation remain in the permanent
verifier.

### Control backing and combined retirement depth

The recursive retirement stack is now paged instead of embedding 403 owner frames in one opaque
Box. Its root and pending owner are retained inline; the remaining frames use 51 fixed eight-frame
pages. Each page must fit the same conservative 16 KiB control credit, is allocated only when the
next admitted push needs it, and is released only after all eight semantic slots are empty. The
fixed simultaneous control census is therefore 51 stack pages plus 13 non-stack Box/Arc controls,
exactly 64; capacity `+1` is rejected.

`RasterSnapshotRootRetirement` separates Arc control release, owned snapshot handoff, inner
retirement construction, payload retirement, and inner Box release across distinct turns. Packed
field, digest, mutation candidate, and initializer active/envelope retirement owners likewise hold
a terminal flag and release a completed Box control on a later credited turn. Runtime fixtures
assert the outer retirement, stack page, snapshot clone, layer clone, and value clone control types
fit the 16 KiB credit before exercising Box/Arc release.

Layer and DSL admission now share the exact formula `layer depth + 2 * value depth + wrappers`.
The admitted capacity is 400 frames with three fixed rejected-owner frames, while the physical
paged stack holds 403. The hostile maximum Group-to-Adjustment-to-nested-value fixture reaches
terminal-empty; maximum `+1` is rejected before any frame push.

### Second-remediation source gates

| Gate | Result |
|---|---|
| exact whole-buffer symbol census | **PASS**: exactly **12** source occurrences, one shared definition plus eleven non-Raster callers; Raster zero |
| retained standard-map / `pop_last` / `mem::forget` scans | **PASS**: zero in the Raster retained codec |
| exact replacement handback census | **PASS**: three retained `Ok(Some((previous_key, previous)))` consumers |
| scoped edition-2021 `rustfmt --check` | **PASS** on all nine changed Raster Rust files |
| permanent tool-job verifier self-test | **PASS**: **323** self-tests clean |
| live tool-job source verifier | expected global **RED**: **0/884**, 19 declared failure classes; no Raster-specific predicate failure |
| scoped `git diff --check` | **PASS** |
| whole working-tree `git diff --check` | **PASS**, with only the pre-existing DXF CRLF normalization warning |
| Cargo / Nx / native / Wasm / browser / runtime / network | **not run by instruction** |

Second-remediation verdict: **source-audit-ready, not accepted**. Rust typechecking and execution of
the Rust fixtures remain unrun. The serialized native/Wasm/browser matrix remains mandatory after
independent source acceptance. P2a1 was not started.

## Fourth independent remediation

The four blockers in
`terra-independent-p8yw-raster-third-remediation-final-audit-2026-08-23.md` are repaired in the
Raster content/control ledger, semantic work budget, owned fixed map, retirement control
authorities, mounted-shaped fixtures, and permanent verifier. This is an implementation handback
for independent source audit; no build/runtime acceptance is claimed.

### Separate payload and real control admission

`RasterOwnerTotals` now keeps source/candidate payload items and bytes separate from
source/candidate control items and bytes. The 262,144-byte payload ceiling is therefore available
to the snapshot rather than being consumed a second time by the 64 fixed controls. The control
census remains exact: 51 maximum stack pages plus 13 non-stack controls, with 262,144 bytes of
separate conservative control backing.

The mounted initializer claims a real process-wide reservation for all 13 non-stack controls after
the empty/content bounds authority succeeds and before any child clone Box is constructed. The
reservation records both remaining items and bytes, stays held through clone, history replay,
mutation candidates, cancellation/fault disposal, and envelope retirement, and returns exactly one
4,096-byte backing per semantic turn. Normal success is not visible until all 13 credits return;
terminal cancel/fault also requires the reservation to be absent.

Standalone owned-retirement Boxes and the Arc-root retirement now claim a separate real one-item,
4,096-byte process credit before their Box is allocated. Each credit is stored behind
`ManuallyDrop`, reports `held_items` and `held_bytes`, and returns only after payload and inner
retirement terminal witnesses are empty. The fixed process ceiling is 13 controls for each of the
64 admitted outer operations. Stack pages use their own checked process page ledger: claim is a
separate CAS turn, allocation can occur only while that exact page credit is held, and empty-page
release returns the process credit in the same turn that reports one item/4,096 bytes.

### Semantic fuel is independent from byte admission

`raster_reserve_unit` now consumes exactly one semantic fuel unit. String capacities, 16 KiB map
pages, 4 KiB controls, and aggregate byte ceilings remain independently checked by the admitted
payload/control ledgers; their byte sizes are never used as worker fuel. The mounted-shaped fixture
uses the production budget of 64, includes a real layer and nine asset entries so construction
crosses into a second fixed map page, reaches `Complete`, observes the process non-stack counter at
zero, takes the exact candidate, and closes it through the retained disposer.

### Exact fixed-map ownership

`RasterOwnedMap` stores its page array behind `ManuallyDrop` and refuses ordinary Drop unless
every semantic entry and every actual page backing has been explicitly detached. The key-discarding
`remove` API is deleted. Pair removal returns `RasterOwnedMapEntry<V>`, whose Drop refuses until
the exact key/value pair is taken once. Replacement returns
`RasterOwnedMapInsert::Replaced(RasterOwnedMapEntry<V>)`; all retained clone/mutation consumers
move both displaced owners directly into the bounded retirement cursor.

Public unique insertion rejects duplicates without mutating the installed owner. Legacy direct
diff asset removal now fails before cloning/mutation and points callers to the retained authority.
Populated whole-map `Clone`, serde decode, and Dsl decode no longer allocate/copy page graphs:
Clone accepts only an empty shell, while populated decode is fail-closed in favor of the retained
page decoder. The retained initialization codec still contains zero whole-map Clone/serde/Dsl
materialization.

### Fourth-remediation discriminators

Rust fixtures added or strengthened:

- empty snapshot bounds succeed with a nonzero payload shell plus the separate 64-control census;
- mounted 64-fuel initialization crosses a second 16 KiB map page and returns process credit to
  zero before candidate handoff;
- stack page credit claim, allocation, semantic use, page release, and process-credit return are
  distinct observable transitions;
- exact pair removal preserves the original key pointer and a populated ordinary map Drop refuses;
- replacement preserves both incoming and displaced key/value allocations; and
- Box and Arc retirement assertions require exact held item/byte counters to be empty before their
  terminal witness.

Permanent verifier mutations now reject payload/control double-counting, byte-valued semantic
fuel, removal of the mounted-64 fixture, operation or standalone process control ledgers,
non-`ManuallyDrop` control shells, uncredited stack allocation, restoration of key-discarding
remove, restoration of populated whole-map Clone/serde, and removal of the exact terminal control
fixtures.

### Fourth-remediation source gates

| Gate | Result |
|---|---|
| scoped edition-2021 `rustfmt --check` | **PASS** on all nine changed Raster Rust files |
| permanent tool-job verifier self-test | **PASS**: **328** self-tests clean |
| live tool-job source verifier | expected global **RED**: **0/884**; no Raster-specific predicate failure |
| semantic-fuel argument scan | **PASS**: zero `raster_reserve_unit(cx, ...)` byte-valued calls |
| key-discard / whole-map clone scans | **PASS**: zero public key-discarding remove and zero clone insertion loop |
| exact raw whole-buffer census | observed **9** current-tree occurrences during concurrent P8 caller reductions; Raster remains zero |
| scoped `git diff --check` | **PASS** |
| whole working-tree `git diff --check` | **PASS**, with only the unrelated DXF CRLF normalization warning |
| Cargo / Nx / native / Wasm / browser / runtime / network | **not run by instruction** |

Fourth-remediation verdict: **source-audit-ready, not accepted**. Rust typechecking and execution of
the mounted-shaped Rust fixtures remain unrun because the serialized build window was not assigned.
P2a1 was not started.

## Fifth independent remediation

The two remaining blockers in
`terra-independent-p8yw-raster-fourth-remediation-final-audit-2026-08-24.md` are repaired in the
Raster standalone/Arc retirement admission, the populated owned-map DSL boundary, focused fixtures,
and the permanent verifier. This is a source-audit handback; it does not claim build or runtime
acceptance, and P2a1 was not started.

### Saturation-retained standalone and Arc ownership

`RasterOwnedRetirement::new` and `RasterSnapshotRetirementFactory::retire` no longer place an
`expect` between a `ManuallyDrop` owner capture and its retained cursor. Each constructor captures
the exact producer owner directly in the fail-closed cursor and opportunistically claims its typed
standalone control. A saturated claim leaves `control: None`; the first later close opportunity
retries one compare-exchange and yields without advancing the owner whether the retry remains full
or succeeds. The Arc root has an explicit `control_returned` terminal witness, so it cannot reclaim
its returned credit or report terminal before the Arc owner, inner Box retirement, and root control
all close.

Standalone claim and release now use checked, single-attempt compare-exchange transitions. A
contended release retains its held item/byte witness and resumes on the next close step; underflow,
overflow, and duplicate release cannot mutate the process counter. Saturated standalone and Arc
factories therefore return a live retained cursor instead of panicking, deep-dropping, leaking, or
losing the exact producer allocation.

The focused max/+1 fixtures fill all 832 standalone controls, construct the 833rd owner under
`catch_unwind`, prove its allocation pointer is unchanged, prove zero release while full, return one
exact credit, resume the same cursor, and close every cursor to `held == returned`, process zero,
page-process zero, and terminal-empty. The Arc fixture repeats the boundary through root admission,
Arc-to-value transfer, saturated inner-Box construction, resumed inner retirement, root-control
return, weak allocation extinction, and terminal-empty.

### Populated DSL materialization removed

The public `RasterOwnedMap<V>::to_value` implementation contains no capacity allocation, entry
loop, key clone, or recursive value materialization. Empty maps still produce an empty DSL map.
Populated maps fail closed before inspecting the first semantic entry, with an explicit message
that interactive production routes require retained page output authority. The current Raster
production census has no direct populated map `to_value` caller; even a generic derived call now
reaches the pre-work refusal rather than the former whole-map loop.

The hostile populated fixture installs the exact 64-entry maximum across all fixed pages with
nested object/array/string owners, verifies max+1 returns the original key/value, proves ordinary
DSL output produces no result under panic containment, proves the first installed key allocation
remains exact, exercises zero-grant cancellation and populated-input fault refusal, and retires the
rejected pair plus the full nested map through one-owner grants to standalone/page process zero and
terminal-empty.

### Fifth-remediation permanent mutations

The Raster source predicate now requires both saturation-safe optional constructor claims, the
retryable standalone admission seam, the Arc control-return witness, all three hostile fixture
names, the explicit populated-DSL refusal, and empty-only DSL output. It rejects restored constructor
`expect`, panic-on-saturation retry, removal of the Arc return witness, removal of either saturation
fixture, removal of the populated DSL fixture/refusal, and reintroduction of the exact key-clone /
recursive-value materialization loop.

### Fifth-remediation source gates

| Gate | Result |
|---|---|
| scoped edition-2021 `rustfmt --check` | **PASS** on the Raster owned-map and retained binary codec files |
| scoped Raster/root-verifier `git diff --check` | **PASS** |
| permanent tool-job verifier self-test | **PASS**: **328** self-tests clean |
| live tool-job source verifier | expected global **RED**: 884 remaining commands and unrelated global failure classes; no Raster predicate failure |
| standalone panic / populated DSL loop scans | **PASS**: zero production saturating constructor `expect`; zero production key-clone/value-materialization loop |
| Cargo / Nx / native / Wasm / browser / runtime / network / broad builds | **not run by instruction** |

Fifth-remediation verdict: **source-audit-ready, not accepted**. Rust typechecking and execution of
the new Rust fixtures remain unrun until the serialized build lane is assigned. P2a1 was not
started.

## Sixth independent remediation

The sole blocker in
`terra-independent-p8yw-raster-fifth-remediation-final-audit-2026-08-24.md` is repaired in the
Raster owned-map serde boundary, the three derived schema paths, the exact-owner fixture, and the
permanent verifier. This is a source-audit handback; it does not claim build or runtime acceptance,
and P2a1 was not started.

### Public populated map serde is absent

`RasterOwnedMap<V>` no longer implements `serde::Serialize` and exposes no public serialization
bound. The former `serialize_map(Some(self.length))` / `for (key, value) in self` /
`serialize_entry` loop is deleted. The remaining `RasterOwnedMap` traits are ownership access,
empty-only Clone, fail-closed Deserialize, and fail-closed populated DSL conversion.

The three serde-derived fields that previously made the map implementation publicly reachable now
use the generic `serialize_empty_owned_map` field function:

- `RasterLayerNode::Adjustment.params`;
- `RasterSnapshot.assets`; and
- `RasterArtifact.assets`.

That function has no `V: serde::Serialize` bound. It rejects a populated map before reading its
first key, value, or page and only emits an empty map for the empty shell. The unused public
`semio_example_json` whole-snapshot helper was deleted. Existing Raster snapshot/layer callers now
either serialize map-free/empty shapes or receive the explicit retained-output-required serde
error; none can recover a populated `RasterOwnedMap: Serialize` bound or invoke a hidden generic
map loop.

### Hostile populated serde fixture and faithful mutation

`raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact` installs
the full 64-entry map across all admitted pages with nested object/array/string owners, proves the
65th pair returns the exact input allocation, calls the public derived adjustment-layer serde path
under panic containment, and requires an ordinary serde error before a whole result exists. It
then proves the first retained owner is unchanged, applies a zero-item/zero-byte cancellation-shaped
close grant, and retires the rejected pair plus the populated snapshot through the existing one-owner
cursor to terminal-empty with standalone-control and retirement-page process counters at zero.

The permanent Raster predicate now reads the retained codec, owned-map source, artifact schema, and
snapshot schema together. It requires all three derive guards, the unbounded-value-free empty helper,
and the hostile fixture. It rejects a public `RasterOwnedMap` serialize bound and the actual
length-based `serialize_map` / `serialize_entry(key, value)` mechanism. Its new self-test restores
the complete former public implementation and loop; acceptance of that mutation is an immediate
verifier failure. A second mutation removes one derived-field guard, and a third removes the hostile
fixture.

### Sixth-remediation source gates

| Gate | Result |
|---|---|
| scoped edition-2021 `rustfmt --check` | **PASS** on the four touched Raster Rust files |
| scoped Raster/root-verifier `git diff --check` | **PASS** |
| public populated-map serde loop/bound scan | **PASS**: zero production implementations, length-based map serializers, or key/value `serialize_entry` loops |
| derived Raster owned-map serde guard census | **PASS**: exactly three guarded fields; no derived field retains the map Serialize requirement |
| permanent tool-job verifier self-test | **PASS**: **328** self-tests clean, including the restored public-loop mutation |
| live tool-job source verifier | First scoped run reached expected global **RED** with 884 remaining commands and no Raster predicate failure. A final rerun after concurrent peer drift was blocked before completion by unrelated `toolJobFem2dMountedSessionExact` input wiring (`frameworkPlugin` was `undefined` at `📜️script.ts:3205`); no Raster source or predicate failure was emitted. |
| Cargo / Nx / native / Wasm / browser / runtime / network / broad builds | **not run by instruction** |

Sixth-remediation verdict: **source-audit-ready, not accepted**. Rust typechecking and execution of
the new Rust fixture remain unrun until the serialized build lane is assigned. P2a1 was not started.
