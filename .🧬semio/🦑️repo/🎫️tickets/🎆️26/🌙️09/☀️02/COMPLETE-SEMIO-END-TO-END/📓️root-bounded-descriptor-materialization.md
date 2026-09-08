# Bounded Descriptor Materialization

## Current Qualification — 2026-09-08 05:50 UTC

At 06:08 UTC root sampled only its verified Stdio compiler PID47337 for one
second. Active sampled stacks are compiler arena/type-check-result teardown
and allocator frees; this is not evidence of a Cargo lock deadlock or a
passing build. CPU time continues to advance. macOS reports a 7.7 GiB
physical footprint and 9.3 GiB peak, despite much smaller resident RSS.
System-wide free memory was35% and disk free123 GiB. No process, cache or
other agent output was modified. The sample is ticket-generated diagnostic
output and must be removed when this ticket is actually finished.

Verbose retry 5506 subsequently exited zero through the complete bootstrap
source gate: wire rows21, typed dependency rows25, source-epoch physical
laws17, process-owner laws8, generation-stage rows28, publication child
cases12 and the final two-package/28-codec/one-Map contract. This is source
and isolated-oracle evidence only. Native catalog session30472 reached build
at `exact-cargo-laws-B3F3qd/00`; Stdio26394 reached current Stdio compilation.

Full bootstrap source session 1141 stopped before assertions at Nx graph
loading with the existing erased `undefined` diagnostic. It is not a source
assertion failure. Retry 5506 enables verbose diagnostics. The public root
wrapper deliberately uses the shared workspace database; the separate Nx
daemon qualification explicitly tests that policy. This task has not
changed it or deleted shared caches.

Home has corrected the two inference compilation errors from 94155 without
changing the crate edition or API. Fresh native catalog-selection session
30472 is queued behind the current Stdio build 26394 and includes all eight
catalog, dependency, materialization and profile laws. Neither is a native
pass yet. WGPU independently executed the landed Pack fixture oracle and
confirmed all 21 rows; this remains independent source/oracle evidence.

## Implemented and Source-Qualified Frontier — 2026-09-08 05:43 UTC

The strict Pack-owner entry point, shared symbol-header/string decoding,
fixed logical storage accounting, exact store facade, and 32 MiB Hub policy
are implemented. The zero-argument wire decoder also delegates to the strict
default-options facade; no permissive compatibility route was retained.
Hub checks duplicates and raw canonical encoding before consuming the value
into its descriptor, then checks the typed schema projection. This removes
the explicit `value.clone()`; later encoder/projection allocations are not
claimed clone-free or covered by the decoder budget.

WGPU's schema-first fixture packet contains 21 literal wire rows. The landed
long-symbol boundary is 352/351 logical credits (32-byte symbol slot, 32-byte
symbol, three 64-byte array slots, three 32-byte referenced copies). The
earlier 228/227 example below is illustrative, not the landed fixture value.
The Pack owner's `📜️script.ts` checks these bytes with independent BigInt,
UTF-8 and DataView logic, AJV, JSON and fast-deep-equal, and the TypeScript
Pack decoder for accepted values. It is called from existing Hub catalog
selection/bootstrap gates, not a new executable command.

Source session 59372 failed because the runner selected the AJV 2020 class
for a draft-07 schema. After selecting AJV's matching draft-07 entry point,
session 22780 exited zero: 21 wire rows, 25 dependency claims, nine neutral
native dependency vectors and 23 registry selection rows. Native behavior
remains unqualified.

The authored Hub exact law consumes all 21 rows through the actual store
facade and compares admitted values through Serde JSON. Malformed bridge
rows also exercise the zero-argument facade. A separate bounded hostile has
one 2 MiB symbol referenced 32 times, still below the 4 MiB raw cap, and must
be rejected by the actual Hub 32 MiB materialization fence before descriptor
schema rejection. Both existing native provider and catalog-selection exact
groups include this law. It has not executed.

Hub publication session 94155 completed BUILD RED with three errors: a
Rust-2024 let-chain in the Rust-2021 inference crate, an inference checkpoint
error-type mismatch (both assigned to Home), and the new store facade absent
from the older OS-kernel compilation captured before this patch. No native
assertion ran. Stdio retry 26394 is now building in the serialized target.
Full bootstrap source rerun 1141 is live. No native, current-catalog, mounted
Shell, or end-to-end pass is inferred from source-only checks.

## Original Decision and Audit Trail

The current native descriptor entry point caps raw Pack at 4 MiB, but the
eager decoder can repeatedly clone interned strings before canonical/schema
validation. The default `max_total_alloc` is not consumed on that path.
This is a real resource gap, independently identified in the Terra audit.

The implementation direction is a strict single-DslValue record-body entry
point in the existing Pack owner, exposed through the existing store wire
facade. It must reject extra root fields and non-Value root tags before
entering the general record decoder. Its shared DslValue decoder will debit
owned symbol/string bytes and collection storage before materializing them,
returning LimitExceeded on overflow or budget exhaustion. Hub must not parse
Pack bytes itself. The facade must transfer the decoded value without a
second unconditional clone.

The claim is bounded materialization of this exact schema-less value bridge,
not complete allocation accounting for generic expression parsing, TableSoA,
chunk decompression, every Pack document decoder, allocator bookkeeping, or
the complete retained catalog. A neutral corpus and native laws must qualify
repeated symbols, inline values, nested containers, count/byte/depth limits,
and malformed bridge structure before any runtime pass is claimed.

The original note above predates the strict decoder implementation. The
current source review and remaining native qualification boundary are recorded
below.

## Strict Value Bridge And Neutral Corpus — Source Review (2026-09-08)

### Current narrow boundary

The landed Pack entry point is appropriately narrower than the old generic
record decoder:

- `decode_value_record_body_exact` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:2262`
  reads the inline symbol header, then requires exactly one field, the
  caller-specified field id, `TAG_VALUE`, and EOF.  It therefore rejects a
  duplicate/foreign root field, a non-Value root tag, and a suffix before
  entering generic field decoding.
- `TAG_VALUE` leads only to `DslValue`'s null/bool/number/string/list/map
  grammar (`1714–1748`).  It cannot reach `TAG_EXPR`, `TAG_TABLE_SOA`, packed
  vectors, document chunks, or generic unknown-field preservation.
- The strict path reserves the symbol vector only after a logical 32-byte
  per-slot debit and `try_reserve_exact` (`2234–2256`); copies every symbol,
  inline string, and resolved symbol only after a debit (`1444–1449`,
  `1473–1479`, `1491–1498`); and checks/debits every `DslValue` list/map count
  before reserving (`1460–1469`, `1729–1745`). Checked arithmetic and the
  64-byte list/map-slot ceiling make the stated logical accounting
  architecture-independent.

That is source evidence only. No Pack/Hub native execution was run in this
audit.

### Resolved follow-up: Hub tree clone and default bridge strictness

The initial review found a complete-tree clone in the Hub descriptor path and
a permissive zero-argument Store bridge. Both are now repaired in current
source:

1. `decode_package_descriptor` rejects duplicate descriptor fields, encodes
   `&value` and compares it to raw bytes, then consumes `value` into
   `PackageDescriptor`, followed by the descriptor projection comparison
   (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1305–1322`).
   The former `value.clone()` is gone.
2. `os_store::pack_rt::decode_wire_value` delegates to
   `decode_wire_value_with_options` (`🏪️store/🦀️.rs:5005–5012`), so all
   zero-argument bridge callers now receive the exact one-Value-field,
   trailing-byte-rejecting entry point.

This ordering retains both canonical checks while removing the avoidable
copy. It does not claim to bound later descriptor/projection construction,
which is a separate concern.

The default bridge remains deliberately *varint-permissive*: the existing
dynamic-integer corpus asserts that redundant-but-decodable varints decode and
are rejected only by an explicit encode/decode canonicality comparison
(`🏪️store/🦀️.rs:26718–26730`). “Exact” here means one bridge field and EOF,
not minimal-varint canonicality.

The direct caller and co-located test census found no intentional acceptance
of a missing root field, extra/unknown root field, suffix, or non-`TAG_VALUE`
root. The existing callers all consume producer-generated bridge bytes or
intentionally map a decode failure to a protocol failure/default. The neutral
corpus must nevertheless bind these four rejection rows through the public
`decode_wire_value` wrapper, not only the Pack-inner function, so a future
wrapper regression cannot restore permissive behavior.

### Hub-limit branch and overflow audit

Under the trusted descriptor configuration—4 MiB raw/per-string,
131,072 symbols, 262,144 items, depth 64, 32 MiB logical storage
(`trusted-catalog.rs:1305–1313`)—each accepted `DslValue` branch is bounded
before the relevant allocation:

- scalar null/bool/numbers allocate no tree backing;
- `TAG_STR` converts the index with `usize::try_from`, validates the symbol
  reference, debits the cloned UTF-8 bytes, then reserves/copies;
- `TAG_STR_INLINE` validates the length and UTF-8, debits, then copies;
- list/map call `check_items`, perform checked `count * 64`, debit, convert to
  `usize`, and `try_reserve_exact` before decoding children; map keys use the
  same string path;
- header symbols debit checked `count * 32`, use `try_reserve_exact`, and
  debit each UTF-8 copy before allocation.

No `u64` or `usize` conversion/credit overflow is reachable at those Hub
ceilings. Recursive depth starts at zero and cannot approach `u16` overflow
with a maximum of 64. A caller that deliberately supplies
`PackLimits.max_depth == u16::MAX` remains a generic API edge: a sufficiently
deep nested value can make `depth + 1` overflow before the next depth check.
That is outside Hub's fixed profile, but the Pack owner should use
`checked_add` if it advertises the arbitrary caller-supplied limit as
overflow-safe.

### Language-neutral fixture packet

The neutral corpus should use one raw, replayable record-body fixture, not a
test-side encoder. Suggested schema identity and closed shape:

```json
{
  "schema": "semio.pack.value-record-materialization.v1",
  "wire": "SPK/1.0-record-body",
  "bridge": { "fieldId": 1, "rootTag": "11", "exact": true },
  "cases": [
    {
      "id": "interned-repeat-at-limit",
      "rawHex": "0101610101110c03060006000600",
      "limits": {
        "maxSymbols": 1,
        "maxDepth": 8,
        "maxItems": 3,
        "maxTotalAlloc": 228
      },
      "expect": { "outcome": "accepted" }
    }
  ]
}
```

The example byte body is source-derived, not a native receipt: one inline
symbol `a`, one bridge field, `TAG_VALUE`, then a three-item `DslValue` list
whose members are all `TAG_STR` symref zero. Under the landed accounting it is
exactly `32 + 1 + 3×64 + 3×1 = 228` credits; its paired over-limit row uses
227. `maxTotalAlloc` means the Pack-defined logical owned credits—UTF-8 bytes
plus the fixed 32/64 slot credits—not platform allocator capacity or `size_of`
values, so the JSON remains language-neutral.

Required small rows are:

| Row pair or row | Raw grammar/property | Required outcome |
| --- | --- | --- |
| `interned-repeat-at-limit`, `interned-repeat-over-limit` | One short symbol referenced N/N+1 times in a list | accept exactly at the fixed logical credit; `LimitExceeded` before the next string clone |
| `inline-string-at-limit`, `inline-string-over-limit` | Inline UTF-8 string(s), no symbol table | same boundary for direct owned string copies |
| `nested-array-map-at-depth`, `nested-array-map-over-depth` | alternating `TAG_LIST`/`TAG_MAP` | max depth accepted; first deeper node is `LimitExceeded` |
| `list-count-over-items`, `map-count-over-items` | declared count exceeds `maxItems`, deliberately truncated immediately after count | `LimitExceeded`, proving count rejection precedes body allocation/read |
| `symbols-over-limit`, `symbol-bytes-over-limit` | header count or one header string exceeds its configured limit | `LimitExceeded` before vector/string allocation |
| `root-empty`, `root-duplicate`, `root-foreign`, `root-non-value` | field count 0/2, wrong id, or a root tag other than `11` | malformed exact bridge; no generic value decode |
| `trailing-byte` | valid canonical value plus one byte | malformed exact bridge |

The canonical source-derived minimal roots are `0001011112` for `null` and
`000101110403` for unsigned `3`. The corpus should not include expression,
TableSoA, packed-vector, or chunk rows as if this facade supported them: the
strict `DslValue` root rejects those tags by construction.

### Explicitly unqualified eager paths

The strict facade properly avoids these paths; its fixture must not overclaim
them as bounded:

- Generic `decode_value` still accepts `TAG_EXPR`, `TAG_TABLE_SOA`, packed
  f64/varint, chunks and unknown values (`1583–1632`). Expression parsing
  additionally invokes DSL parsing under that parser's own default limits.
- Generic `decode_table_soa` separately allocates row records, dense/sparse
  presence vectors and bitmaps, and row-field maps (`1934–2004`). Independent
  row/column maxima do not bound their product.
- Complete-document chunk concatenation uses `read_chunked_bytes`
  (`1504–1524`); it is unavailable in a record body but remains outside the
  strict bridge proof.
- Generic `decode_record_body` and `decode_document` deliberately construct
  `DecCtx { materialization: None }` (`2186`, `2276–2280`). That preserves
  existing behavior, but means no inference from the new bridge law to all
  Pack decoders.

### Correct qualification seam

The eager value decoder is OS-kernel source, mounted by
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:152`, not by the
product-neutral `semio-framework-pack` crate. Its co-located record-body
tests remain useful source anchors, but
`nx run @semio-tech/framework-pack-rs:test` would not qualify this code and
must not be presented as its test target. The narrow native law should instead
run in the existing Hub catalog-selection exact-law group through the real
Store facade/`decode_package_descriptor`, or use a registered
`@semio-tech/framework-os-kernel` target with package
`semio-framework-os-kernel`, lib target. Neither was run here.

## Private GIS Approval Undo Authority and Rebootstrap Audit (source-only)

### Verified durable-route properties

The Hub undo route is correctly target- and owner-bound on the source
inspected. It takes the same private ingress authority as approval
(`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7543–7560`), retains sorted
User/Session/DirectorySpaceAuthority/Membership guards during the operation
(`:7441–7456`), and revalidates before returning a receipt (`:7555–7557`).
The runtime then serializes the exact document gate, resolves the target and
original job through the current session reader, checks live Author authority
*before* a replay lookup, and verifies the exact expected frontier twice
(`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3445–3496`).

SQLite binds the target to the approval witness and to original
user/session/authorization-generation/space/document
(`🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:822–851`).  Both target lookup and replay
require those exact reader coordinates (`:861–921`, `:924–968`).  A committed
replay is additionally keyed by the exact target plus idempotency key, while
`prepare_gis_map_approval_undo` rejects a second key or any substituted
operation/proposal/mutation/command tuple (`:973–1046`).  This is a genuine
per-entry durable target; the page receives neither inverse bytes nor the
idempotency key.

Shell dispatch is also narrowly scoped: only the one currently mounted
document session is considered; zero or multiple matches do not send an undo
request (`ShellHost/🟦️.tsx:4527–4538`, `:6455–6464`).  The worker validates
history epoch, client instance and scope before it uses its private handle
(`🧵️backbone-worker.ts:4041–4053`).

### Current rebootstrap/revocation boundary

The initial two P0s were repaired in the current source, but remain
source-only: the scoped-4401 callback now calls `closeArtifactRuntime(key)`
before it notifies Shell (`🧵️backbone-worker.ts:3198–3208`), and normal close
aborts the exact undo owner (`:4310–4327`). `undoInferenceApproval` now runs
`sameApprovalUndoMountV1` after the network/receipt awaits and before it can
publish `applied` (`:4059–4083`). A stale callback whose original owner was
replaced is also rejected by object identity (`:4073–4078`).

Rebootstrap takes a deliberate retained-retry approach: it aborts and emits
`unavailable` for the old object, then creates a new `awaiting-mount` owner
with the same server target and idempotency key
(`:3719–3741`). That keeps a request that crossed the network safely
recoverable through the server's exact replay record, rather than treating a
timeout as a fresh inverse.

### Revalidation and changed-frontier repair

The current source now captures all four `DocumentOpenRevalidationV1`
coordinates—directory revision, membership, session, and share generation—at
first retention; preserves them through reissue; records them in the mount;
and requires equality before availability or post-await publication
(`🧵️backbone-worker.ts:3643–3715`, `:3735–3810`, `:4091–4123`). It also
retires a reissued owner when the exact replacement mount has a changed
frontier. This closes the prior stale-availability finding, source-only.

The previous cross-document liveness regression is also repaired. The bind
helper now returns on another scope/client before it evaluates a frontier or
authority mismatch (`🧵️backbone-worker.ts:3751–3753`); only the exact
owner's state can retire its private owner. Thus a mounting document B cannot
erase a waiting owner for A merely because this is a process-global slot.

The preceding behavioral-test P1 is now repaired in the current source: the
real worker test first binds unrelated B and asserts that A remains
`awaiting-mount` (`🧵️backbone-worker.ts:8028–8042`), then independently
rotates the optional share generation after absence of session generation
(`:8062–8078`). This is still unrun source evidence. A narrow new source
compile blocker was found in the adjacent delayed undo-receipt test:
`backbone-worker.ts:8117–8118` repeats the `replayed: false` object key in one
literal. Remove the duplicate before treating the test repair as typechecked.

Required deterministic worker/Shell rows:

1. available and submitting owner → scoped 4401 → exact abort, absent owner,
   no late `applied`, while an unrelated document history remains live;
2. rebootstrap with identical artifact but rotated directory/membership/session/share
   generation →
   unavailable and no undo request;
3. rebootstrap with changed head frontier → unavailable and no hidden pending
   owner; and
4. a real transport ambiguity → rebootstrap with the same revalidated pair →
   same idempotency key replay, one `applied` status only.
5. A awaiting reissued owner + an unrelated document B reaching mounted state
   → A remains pending; B cannot consume, retire or receive A's private target.

## Next Genuine Map Journey: Registered-Gate Composition (source-only)

### What the current executable gates actually cover

| Existing launch | Concrete current coverage | Deliberate nonclaim |
| --- | --- | --- |
| `⚖️gate🧬️trusted-stdio-gis-bundle-browser🌎️hub` (`.vscode/launch.json:7395–7413`) | One newly built stdio+GIS closure is materialized, independently candidate-validated, published current, cold-map checked, then its **same published** closed GIS actor is described in Chromium. `TrustedStdioGisBundleCheckScript` routes this through `--browser` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:11085–11260`); `proveTrustedGisClosedActorV1` reads and rechecks the exact current token before and after Chromium (`:9265–9350`). | No authenticated worker, cold-pair application, Shell renderer, mutation, or peer. The script itself prints those exclusions. |
| `⚖️gate🗺️gis-map-proposal🌎️hub🔁️process` (`.vscode/launch.json:7703–7719`) | `proveGisMapProposalProcess` materializes current, starts one real Hub, creates two actual Author credentials/document sockets, uses MCP clients, exercises B's paused cancellation and A's private proposal/approval/undo/replay (`📜️script.ts:10512–10575`, `:10882–10886`). | No Shell scene, WGPU render, or collaborative redo. It makes a fresh ticket-local `dataRoot`; it does not consume the browser gate's selected generation. |
| `⚖️gate🌉️gis-component-cold-map-patch🌍️gis🦀️native` (`.vscode/launch.json:7634–7641`) | Native component cold-map law only. | It is not authenticated Hub, browser, Shell, or two-peer proof. |
| `⚖️gate🧬️trusted-stdio-gis-bundle-process🌎️hub` (`.vscode/launch.json:7416–7434`) | Current publication/candidate failure/rotation plan behavior. | It is not the two-author AI/Shell journey. |

The launch-specific artifact directories are intentionally distinct. More
importantly, every `--native`, `--process`, and `--browser` bundle mode creates
`<ticket artifact>/hub-target` and invokes Cargo itself
(`📜️script.ts:11220–11253`). Running native followed by browser therefore
does **not** reuse a prior native build/current. The smallest existing
single-command bridge is the **browser** gate: it builds once and puts native
cold-map plus Chromium proof behind the one materialized selected generation.
It must not be relabelled an end-to-end map journey.

### Lowest sound next permanent composition target

Add one registered, ticket-owned process target—not a sequence of the two
existing targets—that owns one `dataRoot`, one exact
`TrustedBootstrapMaterializationV1` receipt, and one `os-hub` binary path.
Its order should be:

1. Run the existing materialize → cold-map → candidate readiness → current
   publication fence once, exactly as
   `validateAndPublishTrustedStdioGisCandidate` orders it
   (`📜️script.ts:9900–9920`). Refuse if current's generation, bundle SHA,
   profile, or publication revision differs from that receipt.
2. Start the real Hub against that **same** `dataRoot` and binary, with two
   real Authors. The existing process harness provides the credential/socket
   and MCP lifecycle scaffolding; it must not silently rematerialize its own
   generation.
3. Open one real authenticated Shell/worker using the plan/lease for the
   published v14 GIS descriptor, wait for its genuine component's cold pair to
   be mounted, and observe a semantic `UiDocumentStore`/`TiledMapHost` region
   projection—not the controlled scene fixture.
4. Use the inherited test-only FD4 checkpoint gate already designed for B's
   real MCP job: wait `entered`, cancel B through B's private MCP handle,
   release, and require terminal cancelled with no offer. Then run A's normal
   approve and ordinary Shell Undo route against the same mounted worker.
5. Observe B's separate authenticated socket/pair sees the approved shared
   frontier/region but cannot address A's private job or undo target; require
   the mounted A scene to lose the approved region after the ordinary undo.

This adds the missing *connection* only: current browser proof plus current
two-author process proof. It must retain existing independent unit laws;
neither a synthetic actor scene nor a browser actor description proves
Shell→worker→Hub mutation delivery.

### Fail-closed prerequisites and reuse boundary

- Do not start this target until the pending catalog-selection/native closure
  reports all six published files—GIS's four plus Stdio's two. Root's recent
  preflight found that old publication fixtures/CLI copied only four GIS files;
  the corrected neutral fixture now requires the exact six-file closure, but
  its Rust reload proof is still unrun.
- Carry `executionProtocol.appChannelVersion` only from the decoded exact
  descriptor/lease. Refuse a plan, actor, or mounted worker whose exact
  descriptor/component/actor hashes differ; never host-stamp the protocol.
- Pass the candidate's exact `dataRoot` to both the Hub process and Shell test
  harness. A default root or a new random process root is a different current
  generation and invalidates the assertion.
- The shared browser gate leaves current valid after browser failure by design;
  it therefore proves selected-current native materialization, not a
  browser-qualified publication invariant. The composition target must state
  its own browser/Shell outcome separately.
- FD4 is test-support inherited control only. EOF/truncation before `release`
  must fail the test and never become an HTTP or MCP capability.

## Same-Data-Root Two-Author Map Composition Packet (design only)

The proposed closed neutral contract is now authored under the ticket:

- [fixture](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️same-data-root-gis-map-composition-v1/🔣️.json>)
- [schema](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️same-data-root-gis-map-composition-v1/🧬️.schema.json>)

It requires exactly two Authors, the stdio→GIS selected closure, execution
protocol 14, and one receipt/current identity tuple
`(generationId, bundleSha256, profileId, publicationRevision)`. It explicitly
rejects a second data root, changed current token, a synthetic scene in place
of the worker patch, direct HTTP undo in place of Shell Undo, peer private
handle access, and restart without a post-undo pair check. It is proposed
test input only; no AJV/native/browser execution has been claimed.

### Correct MCP/Shell owner split

The composition must not forward A's MCP approval receipt or its `undo` handle
to a browser relay. The server returns that handle only in the original
session's approval response (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:10683–10724`),
and the Hub undo route derives the original-job identity from the authenticated
reader before it looks up the target
(`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3445–3478`). A separately authenticated
browser relay is therefore intentionally not an MCP client capable of adopting
that secret.

The honest composed journey is consequently: B submits and cancels **B's own**
MCP job through FD4; A's mounted Shell worker submits and approves **A's own**
job; that same worker retains the approval receipt privately in
`retainInferenceApprovalUndo` (`🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3796–3825`)
and ordinary Shell Undo sends only its history epoch, client instance and scope
(`🏛️ShellHost/🟦️.tsx:4527–4538`). The worker later supplies its private target,
frontier and stable idempotency key. This retains the intended A-private
approval/undo boundary while still allowing B to be an equal Author who submits
their own job.

The proposed fixture now records this split, rejects an MCP-to-Shell private
handle handoff, and pins explicit UI locales A=`en`, B=`de`; it declares no
fallback/default locale. It remains an unrun design artifact.

Strict AJV 2020 validation of that proposed fixture/schema was run after the
schema's `closure` tuple received its explicit two-item bounds. This validates
only the language-neutral design file, not any process, native, browser, or
Shell behavior.

## Stdio Headless Import Metadata Capture (current-source audit)

The switch from Cargo's `.rlib` to the sole `.rmeta` reported by the same
`compiler-artifact` record is the right immediate correction. The current
selector binds the Stdio root source path and sorted `full-artifact-catalog`
feature set before it accepts one metadata filename
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:4802–4815`). It also rejects an
in-proof replacement by comparing the metadata inode, size, mtime and ctime
after every import row (`:4831–4835`). The `.rlib` was not a valid metadata
input for this `--emit=metadata` import probe.

It is not yet immutable enough when the reported filename is the un-hashed
`libsemio_s_plugin_stdio.rmeta`. `runExactCargoLaws` releases Cargo after the
build completes, then `proveHeadlessStdioImports` first opens the reported
path. A second Cargo command sharing that target can therefore replace the
un-hashed file *before* the initial `lstat`; the before/after comparison then
observes a stable but different build. The current launch configuration also
assigns `native-catalog-selection` the provider target directory, and assigns
the full and `--stdio-only` provider commands the same target
(`.vscode/launch.json:7059–7082`, `:7148–7157`). Cargo's build lock does not
cover the later standalone `rustc` import loop.

### Bounded repair

1. Give every independently launchable exact-Cargo command a distinct,
   ticket-generated `CARGO_TARGET_DIR`. In particular separate
   `native-catalog-selection` from the provider target and give `--stdio-only`
   its own target/artifact pair if it can run beside the full provider gate.
   The script should require that target to be an absolute generated child of
   the command's owned artifact root. This is a build-output isolation rule,
   not a source-epoch or whole-compiler-provenance scheme.
2. Immediately after the verified Cargo record is read, capture its one `.rmeta`
   into `headless-imports-*/stdio.rmeta` using the existing descriptor-stable,
   bounded `readStableBuildFile` helper
   (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:52–75`),
   with a fixed metadata byte ceiling, `wx` destination, SHA-256 and a small
   capture receipt recording source path, sorted features, original path,
   byte length and digest. Point `--extern semio_s_plugin_stdio=...` at that
   private copy. The target is already private, so its sole `debug/deps`
   directory remains the resolver `-L dependency=...` for transitive metadata;
   no attempt to reconstruct Cargo's transitive compiler closure is needed.
3. Remove `-L dependency=${dirname(receipt.executable)}`. The snapshot's
   original `debug/deps` directory is the relevant dependency resolver; the
   executable directory is an unrelated second search root that can silently
   offer a different dependency artifact. Require the selected `.rmeta` to be
   a regular non-symlink child of that exact private `debug/deps` directory,
   and retain the current exact target/source/features checks.

The import loop should use only the copied metadata and private dependency
directory. A change to the live target after capture is then irrelevant rather
than a race the test merely notices. This intentionally does **not** claim that
the metadata copy proves every source input Cargo consumed; the existing Cargo
artifact event plus actual positive/negative `rustc` imports are the scoped
proof.

### Required deterministic laws

- A capture-helper law with a regular metadata fixture: replace the source
  pathname after capture and prove the private copy's SHA/bytes are used;
  replace it during the descriptor-stable capture and require refusal.
- A native import law that records its full metadata SHA and proves the
  positive exports and negative `E0432`/`E0433` rows against the private copy.
  A source-path replacement after capture must not affect either row.
- An exact-runner/launch law: two provider-like commands with distinct
  generated target roots each produce a capture receipt under their own
  artifact root; a target outside that root and a second `-L` executable
  directory are refused. This tests concurrency isolation without running a
  broad provenance compiler matrix.

The composition fixture/schema was validated with `Ajv2020({ strict: true,
allErrors: true })` after adding the previously missing two-item bounds to its
`closure` tuple. It is schema evidence only.

### Minimal owner and reusable boundaries

Add one registered launch configuration, but **do not** add a second Nx/Cargo
owner: extend the existing target with
`bun nx run os-hub:trusted-stdio-gis-bundle-check -- --two-author-shell`.
`TrustedStdioGisBundleCheckScript` already owns the isolated `hub-target` and
the fresh producer. Its new branch should materialize/validate/publish once,
run the retained closed-actor proof, then call one
`proveGisMapTwoAuthorShellProcess(repoRoot, hubRoot, prepared)` with the same
absolute ticket artifact root, `dataRoot`, receipt, current pointer and binary
path. It must not shell out to either existing gate because those create
independent artifact/data roots and invoke their own Cargo build.

Extract these exact *existing* pieces without changing their authority model:

| Need | Reuse | Required new parameter/result boundary |
| --- | --- | --- |
| one materialized selected current | `materializeTrustedStdioGisBundle`, `validateAndPublishTrustedStdioGisCandidate`, `trustedBootstrapCurrent` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:8970`, `:9900–9921`) | `prepareTrustedGisMapSelectionV1(repoRoot, hubRoot, artifactPath): { dataRoot, receipt, current, binaryPath, validation }`; compare all four current coordinates before every external start/restart. |
| one bounded Hub plus FD4 | `startLocalHub` / `finishLocalHub` (`📜️script.ts:780–886`, `:982+`) | profiles A/B must each allow `native`, `mcp`, and `react-relay`; use `{ dataDir, binaryPath, isolatedSecuritySmoke: true, inferenceCheckpointControl: true }`. The returned `LocalHubRun` already owns descriptor 4 and its one entered/release reader. |
| real initial map pair | `checkpointPublicationProcessFixture`, `commitCheckpointPublicationProcessMutation`, `publishCheckpointPublicationProcessPairV1` (`📜️script.ts:1234–1415`) | retain the returned plan/frontier/receipt only long enough to seed the authorized, announced GIS document. It is real authenticated command/socket/publication traffic, not an ArtifactStore write. |
| two MCP owners and private cancel | `startGisMapProcessMcpClient`, `callGisMapProcessMcp`, `waitForGisInferenceCheckpointControl`, `releaseGisInferenceCheckpointControl`, `readGisMapProcessCheckpoint` (`📜️script.ts:1418–1545`) | retain two child handles, zero their captured credentials at terminal, and require B's `running` receipt → FD4 `entered` → B exact cancel → FD4 `release` → terminal no-offer. |
| two real document sockets | `openGisMapProcessDocumentSocket`, `waitForCheckpointSocketFrame` (`📜️script.ts:1168–1232`) | use both as server-side peer ordering witnesses. Require equal RebootstrapRequired payloads before starting each browser re-open, never merely equal eventual REST pages. |
| two authenticated browser/Shell surfaces | `issueLocalCredential(..., "react-relay")` plus `startLocalBrowserRelay` (`📜️script.ts:500–639`) and the two-context/diagnostic/teardown pattern in `runCollabE2eVerify` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2876–3014`) | extract `startAuthenticatedReactShellV1({ uiPort, hubOrigin, relay, relayProof, renderer: "react" })`, which **only serves** the already selected shell and never calls the old `collabPrebuildPlugins`. Start one relay and one isolated browser context per Author. |

Do **not** reuse `collabStartHub`, `collabPrebuildPlugins`, or
`collabStartUserDevServer` as correctness owners: they make a new default Hub
root, build a different plugin set, and authenticate via their historical
`S_USER` setup (`🧑‍💻dev/📜️script.ts:2392–2525`). They are useful only as
Playwright/daemon cleanup examples.

### Required smallest new Shell observation seam

No current browser gate connects a live Hub plan to two mounted Shell maps:
`proveBrowserDocumentOpenRuntime` runs a genuine worker against a synthetic
authority server (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2380–2563`), while
`proveTrustedGisClosedActorV1` only describes a closed actor. The new harness
therefore needs one dev/test-only, read-only probe exposed from the existing
`browserActorUiByRuntimeKeyRef` owner in
`ShellHost/🟦️.tsx:1721–1779`. Its *closed return* must be no more than:

```ts
type MountedGisMapProbeV1 = Readonly<{
  scope: { spaceId: string; documentId: string };
  clientInstanceId: string;
  activationGeneration: string;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  uiRevision: number;
  rootKind: "tiled-map";
  regionIds: readonly string[];
}>;
```

It must be constructed from the actual retained `UiDocumentStore` after the
worker's accepted patch ACK, with the store scope recorded at installation.
It must expose no plan receipt, socket grant, session credential, approval
target, idempotency key, or action dispatch. A Chromium page reads it only to
assert both A and B have the same real scoped map/region after rebootstrap and
that both lack the approved region after A's normal Shell action. This is the
smallest factual bridge from `TiledMapHost` semantics to a process test; DOM
text, a controlled `UiDocumentStore`, or the closed actor describe result is
not evidence of that path.

### Restart and exact relay ownership

After the durable undo receipt and the two absent-region observations, stop
both browser relays, finish the Hub, then restart `startLocalHub` on the same
port, binary path, and `dataRoot`. Before restart, snapshot the full current
identity tuple; after readiness, require byte-for-byte equality of the pointer
fields. Do not reuse old browser sessions: issue fresh `react-relay` and MCP
envelopes, recreate each relay with its prior `{ port, secret }` binding (the
supported rotation form used by the browser document-open proof at
`📜️script.ts:2447–2483`), and navigate each page with a newly minted broker
fragment. Then reopen both documents and require each new MCP checkpoint pair
and each new map probe equal the same post-undo frontier. This proves session
revalidation rather than preserving a stale live browser authority through a
Hub restart.

Every owned child must close in reverse order: pages/contexts → relays (which
zero their capability/proof) → MCP children → document sockets → Hub. On an
FD4 failure, end the control pipe first and fail; never leave a paused retained
inference owner while proceeding to approval or teardown.

## Two-Author Shell Mount and Creation Boundary (current-source audit)

The former proposed observation seam is now present in source. In a development
build only, `ShellHost` exposes
`window.__semioMountedGisMapProbe(spaceId, documentId)` from its retained
`browserActorUiByRuntimeKeyRef` entry
([probe](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8010>)).
It returns `null` until the precise scope has an installed identity and then
derives the closed tiled-map/region projection from the actual `UiDocumentStore`.
The worker posts `browser-actor-ui-mounted` only after the UI patch result
acknowledges the exact revision
([worker reconciliation](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1500>)).
This is source-qualified read-only observability, not an executed two-Shell
process proof.

### Reusable real mount/control mechanics

Use two isolated Vite child processes, one per author, rather than two servers
inside one process: the Vite dev configuration and broker endpoint values are
process environment inputs. The useful lifecycle model is the existing
two-daemon Playwright harness
([server spawn](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2513>),
[two contexts and diagnostics](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2874>)).
It must supply each process its own loopback port, `S_HUB_URL`,
`S_LOCAL_RELAY_URL`, `S_LOCAL_RELAY_SECRET`, `SEMIO_PLUGIN=s`,
`SEMIO_RENDERER=react`, `SKIP_PLUGIN_BUILD=1`, and an explicit locale lock
(A=`en`, B=`de`). Do not reuse its historical `S_USER` credentials or its
prebuild/default-Hub setup as authority.

The minimally honest browser sequence after a selected current and a live Hub
exist is:

1. Mint one real `react-relay` credential envelope and one proof per author;
   start one local relay per author, then navigate a fresh Chromium context to
   its own Vite origin with a one-use `#semio-broker=...` fragment. Wait for
   the real page readiness marker, then assert `document.documentElement.lang`
   equals the configured locale.
2. Navigate each page to `/spaces/{spaceId}`. That route mounts the actual
   Space index over a Hub opening
   ([route](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4138>)),
   rather than a harness `openDocument` call.
3. Drive the real Space action that emits `ReplayShellCommand
   os.open-artifact`; Shell resolves the installed app and invokes its ordinary
   `openDocument` path
   ([effect sink](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3964>),
   [open owner](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4222>)).
   Wait until the two per-page mounted probes both report the same selected
   catalog/component/descriptor/browser-actor coordinates, same scoped map
   root, and the expected acknowledged revision. A DOM label or a controlled
   store is not a substitute.
4. A then uses the rendered tiled-map context-menu action **Propose Bounds
   Region** (English only in this row). Its guest request is resolved into the
   normal private worker `inference-open` / `inference-propose` messages by
   Shell ([host effect](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3915>)).
   Wait for the real `section[data-semio-inference-port]` preview, approve
   there, and read both probes after the server-triggered rebootstrap.
5. Invoke the ordinary History **Undo** button in A's Shell. It sends only
   `historyEpoch`, `clientInstanceId`, and scope to the worker
   ([Shell dispatch](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4603>)); the worker retains the Hub undo capability and verifies its mounted
   identity/frontier before posting the durable undo
   ([owner check](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4108>)).
   Neither the harness nor B may see or forward that private handle.
6. Close pages and contexts first; then relays (which erase their proof and
   capability); then MCP children and document sockets; then Hub. On restart,
   recreate every browser context/relay/credential and require fresh probes
   against the same post-undo frontier.

### Current P0: no catalog-authoritative ordinary GIS creation/open

The existing Space index cannot create a GIS Map. Its sole guest-visible kind
set is the static four-member `draw/note/dag/writer` fallback
([table](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:31>)).
`CreateArtifact` rejects every other `kindId`, mints a document id itself, and
immediately emits `os.open-artifact`
([handler](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:26>)).
`SpaceIndexConfig` has only visibility, members and presence
([config](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:45>)); it carries no trusted artifact-kind projection.

More importantly, no normal Shell mapping creates `DirectoryCommand::AnnounceDocument`:
the ordinary action map contains only space/member/invite commands
([mapping](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1178>)).
The existing process seeder's direct `announce-document` plus socket mutation
therefore creates no Space index row and no normal open effect
([seeder](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1245>)). Conversely, a
Space `createArtifact` action has no way to announce its newly minted id before
its immediate open effect asks Hub for a plan. The two directions cannot be
joined by selecting an existing fixture id because that command accepts no id.

The narrow repair is a host-owned, schema-first creation transaction, not an
extra client-side opening helper:

- Project read-only `ArtifactCreationKindV1` records into `SpaceIndexConfig`
  from the selected trusted catalog: `{kindId, labels{en,de}, artifactKind,
  standard, subset, schema, role, catalogGenerationId}`. Do not expose paths,
  arbitrary package metadata, or a user-selected document id.
- Given `{kindId,name}`, the authenticated host revalidates the Space/Author
  and selected catalog generation, mints the document id, constructs the
  `AnnounceDocument` descriptor from the catalog's exact owner/dialect/schema/
  pack binding, then seals a one-use `ArtifactCreationGrantV1` bound to scope,
  kind, catalog generation and document id.
- Only that host may consume the grant to append the Space index row using the
  same id. Emit `os.open-artifact` after the row and directory document are
  durable. No public guest action receives an arbitrary pre-existing id and no
  client file path enters the flow.

`SpaceIndexConfig` is explicitly a whole-record host-folded view model, not a
security capability ([definition](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:45>)).
The projected list can drive the guest dialog, but the Hub must treat its
`kindId` as an untrusted UI hint and resolve it again from the exact selected
catalog under its creation fence. A guest/config snapshot cannot authorize the
descriptor, generation, document id, or host grant.

There is a separate current UI P0: merely adding that config field will **not**
put GIS in the ordinary dialog. The plugin's `createArtifact` dialog is a
compile-time `DialogDefinition` whose select options come only from
`KNOWN_ARTIFACT_KINDS` ([options](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:58>),
[dialog](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:481>)).
`create_space_index_editor()` has no `ConfigView`; its static manifest cannot
read a later host fold. The command also declares the artifact publication lane
and locally mints the id. A catalog-derived normal journey therefore requires
one coherent UI/command reclassification: an effect-only creation request
whose rendered options are host-projected at runtime (or an equivalent
runtime-configurable dialog facility), and a Shell/Hub handler that starts the
private creation record. Do not add GIS to the static fallback or only add it
to config: the former is stale/over-broad and the latter changes no selectable
option. The new tests need one positive rendered GIS option for the selected
catalog and its absence after a generation that lacks GIS, as well as the
server-side re-resolution tests above.

The concrete Shell insertion is the `replayShellCommand` effect funnel
([branches](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3948>)).
It presently recognizes only administration, the seven `os.directory.*`
commands, and `os.open-artifact{,-with}`. Give creation its own exact,
identity-required branch before the generic directory prefix and post a closed
worker request containing only `{kindId,name}`. Neither
`directoryCommandFromAction` ([mapping](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1178>))
nor a replay effect may carry a descriptor, document id, catalog generation, or
server grant from the browser.

### No existing cross-owner atomic operation

There is not yet an atomic owner that can consume that grant across the
Directory and Space-document writes. `DirectoryService::execute_idempotent`
has one `directory.write` guard and atomically owns only the Directory command
receipt/events ([implementation](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2134>)).
It cannot call the Space document command pipeline. Conversely, the current
guest `CreateArtifact` produces its `SSpaceMutation::CreateArtifact` plus the
open relay locally; it has no host-grant parameter or Directory transaction
([handler](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:26>)).
The Hub binary has no `semio_s_plugin_space`/`SpaceIndex` dependency, so it
also cannot construct that plugin-owned `SSpaceMutation` itself. Its generic
socket write path deliberately authorizes envelopes without interpreting their
schema-specific payload ([admission](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3994>)).
A bare Hub grant cannot prove that an arbitrary generic envelope created the
specific same-id row. The design therefore must name one real consumer: either
move a minimal, server-verifiable Space-index creation schema/codec into a
shared host domain, or bind the grant to an exact trusted guest-produced
operation with server-verifiable semantics. Do not imply that the existing
Directory command service can append a plugin row.
The direct seeder is likewise a sequence, not a transaction: it announces,
then obtains a document plan/socket, then publishes a checkpoint
([sequence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1245>)).

The host grant therefore needs a durable, private creation record with an
explicit `Preparing → Ready` (and terminal reconciliation) state. It must bind
the immutable scope, server-minted document id, complete descriptor digest,
selected catalog generation/identity, initiating principal/session generation,
and request-id command digest. The record survives request cancellation. A
retained server driver, not the HTTP/Shell request, progresses the only safe
order: exact map genesis/checkpoint durable → exact descriptor announced →
same-id Space row durable → `Ready`/one open effect. Each retry rereads the
record and exact durable facts; it never remints an id, reuses a grant outside
the record, or replaces a descriptor.

That record cannot be an in-memory Shell/Hub map or an ordinary
`DirectoryCommandReceipt`: the latter only models command outcome/event range
and cannot retain per-step descriptor/document facts. A production resumable
creator needs a closed `HubDirectories` persistence seam with SQLite,
Postgres, and Neo4j implementations plus a bounded pending-record recovery
cursor. Otherwise it is only a non-resumable test seeder and must not be used
as the normal `Create Artifact` path.

There is no need to invent a two-document atomic lock for this saga. For each
single durable step acquire the sorted initiator `User`, `Session`,
`DirectorySpaceAuthority`, `Membership`, plus **that step's own**
`DocumentWrite` key before revalidating and committing. `SocketBindingGatesV1`
already sorts/deduplicates this union
([implementation](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:877>),
[order](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:723>)).
The existing checkpoint publisher itself acquires and holds the map
`DocumentWrite` around its authoritative publication
([publisher](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3317>));
an outer map lock would self-deadlock. The later Space-row append must acquire
its own index `DocumentWrite` with fresh principal/role revalidation. Do not
hold any of these guards across component/bootstrap work. This retains the
revocation cutoff per physical commit without falsely claiming a cross-document
transaction.

`AnnounceDocument` is immutable/idempotent only for an exactly equal
descriptor and has no compensating document-delete command
([decision](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1781>)).
Thus a post-announcement failure cannot honestly be reported as “nothing was
durable.” It is a non-visible `Preparing` record requiring bounded retained
reconciliation, and it must emit no `os.open-artifact` until the row and
genesis are exact. A current-generation change before the first durable step
can refuse cleanly; after a step, reconciliation must retain the captured
catalog/descriptor identity or resolve a typed stale/indeterminate outcome—do
not silently substitute the new current generation.

The first tests must prove: success has exactly one same-id genesis,
descriptor, index row and opening tuple; unknown, stale, foreign and duplicate
catalog kinds are refused before any durable step; forged/replayed/cross-space
grants cannot append; each pause after genesis/announcement/index append has
no early opening effect and resumes only the original record; and a demotion
before each durable boundary stops progress without a newly visible row. This
is the lowest actual prerequisite to the planned same-data-root Shell proof.

### Current composition-owner cleanup defect

The new composition source creates observational document sockets in
`openGisMapProcessDocumentSocket`, but its `closePeers` loop merely calls
`socket.close()` and immediately clears the plan receipt
([open](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10426>),
[cleanup](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10801>)).
It never observes the close event or a bounded terminal error before it kills
the MCP children and closes the Hub. That is an owner-lifetime race during the
restart claim: an old socket can still be live/receiving rebootstrap frames as
the same data root and port are reused. Add a private bounded
`closeGisMapProcessDocumentSocket` owner that first disables frame retention,
sends the normal close if needed, waits for its exact `close` event (or reports
its error/timeout), then erases the plan receipt/grant. `closePeers` must await
that helper before MCP/Hub shutdown. This does not alter normal authority, but
makes the “fresh peer after restart” evidence causally meaningful.

The forthcoming `startGisMapShellPeerV1` must also receive a parent-owned
monotonic local-bootstrap credential issuer, rather than invoking
`issueLocalCredential` with its default `sequence = 2`. The inherited bootstrap
server requires exactly previous incoming sequence plus one before issuing a
credential ([check](</Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🦀️.rs:431>)).
The composition has already used sequences for native and MCP peers before
opening the Shells. Each Shell/start-after-restart therefore needs a fresh
`react-relay` exchange and the next sequence under the parent pipe owner; a
reused default is a real authentication-protocol failure, not a retriable
browser bootstrap.

### Current Space creation command cannot be made host-owned by a relay hook

The current `CreateArtifact` handler does not merely request an opening. It
creates an `SSpaceMutation::CreateArtifact` in its `Artifact` publication lane
and returns it in the same `Emit` as the immediate `os.open-artifact` effect
([handler](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:24>)).
It also mints the id inside the guest. A Shell or Hub hook after that artifact
lane has committed is too late to claim the server minted the same id or that
the index row was the final creation step. The existing unit law deliberately
pins that local mutation, locally minted id, and immediate opening relay
([law](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:50>)); it is not a law for catalog-derived creation.

The normal GIS journey consequently needs a distinct host-only creation
request (or one fully reclassified replacement command with all callers and
publication contracts moved together). Its guest side may select the
host-projected presentation row, but it must publish neither an index mutation
nor an opening relay. The private server record is then the only source of the
server-minted id, index row and final opening effect. A minimal hostile matrix
for that new command is: selected GIS success; an otherwise valid static
legacy `kindId` refused when absent from selected catalog; selected generation
rotates before first durable step; replay of the guest request; duplicate
delivery after `Preparing`; and a forged server response/document id. Existing
legacy `CreateArtifact` laws remain unchanged unless the command is explicitly
replaced as a wire-breaking, coherent change.

The framework already has a suitable *presentation* vocabulary:
`ActionArgDef::artifact_kind` derives `ActionArgControl::ArtifactKind`
([definition](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:372>)),
and `ArtifactKindChoice` has the bounded neutral UI shape
`{kindId,schema,dialect,label{en,de}}`
([wire](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4115>)).
The existing resolver is **not** usable as the creation authority: it walks all
in-process `PluginManifest`s
([resolver](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4201>)),
which can differ from the selected trusted Hub generation. A selected-catalog
resolver may reuse this exact presentation shape, while retaining the selected
generation and complete catalog binding privately and re-resolving `kindId` at
the server commit boundary.

### Descriptor round-trip P0: owned Stdio vectors

The current GIS/VCS large-byte round-trip rejection has a concrete loss path
independent of catalog selection. `ArtifactKindSpec` owns
`export_stdio_kinds` and `import_stdio_kinds` as `Vec<&'static str>`, then
uses `ignore_stdio_kinds_on_decode` to replace either decoded vector with an
empty one ([definition](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:5120>)).
Thus a raw descriptor with nonempty GIS import/export kinds cannot re-encode
to the same bytes after a typed parse. The appropriate fix is the planned
owned `Vec<String>` migration with ordinary `ToValue`/`FromValue`, not a
special descriptor encoder.

This field is also live outside trusted-catalog decoding: OS media
compatibility intersects the two vectors
([consumer](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3487>))
and the plugin-registration catalog projects them into `ArtifactKindEntry`
([projection](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:6480>)).
After the owned migration that projection must clone strings rather than
dereference static strings. The required narrow law is a nonempty
`ArtifactKindSpec` raw `to_value → from_value → to_value` equality plus a
registered host-catalog assertion preserving both vectors. This is source
evidence only; the parent owns the migration and its native qualification.

**Current source update.** The migration is now applied: both fields are
`Vec<String>` in `ArtifactKindSpec`; the decode-erasure hook is gone, GIS and
the other nonempty artifact constructors own their literals, and the OS
registry projection uses `clone()` directly. A bounded scan found no remaining
static-slice `.to_vec()` assignment or erased-field hook. The new closed,
nonempty fixture lives at
[`artifact-kind-formats.json`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🗄️artifact-kind-formats.json>)
and is consumed by the linked-provider law, which checks typed `FromValue`,
Serde and Pack equality plus array/scalar refusals for both fields
([law](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:312>)).
This is a source audit only; the fresh native run was active at the time of
review.

## Ordered Directory Publication Oracle (current-source audit)

The present topology check is no longer blocked by `DirectoryStreamMessage`
boxing: its `publish_persisted_locked` assertion correctly expects
`Event { event: Box::new(event.clone()) }`
([oracle](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:125>),
[service](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2053>)).
It must remain writer-specific. `redeem_invite` uses its required backend
atomic `redeem_invite_atomic` transaction then publishes the returned event
while its write guard remains live
([path](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2355>)); it does not use the generic
`append_and_publish_locked`. `execute_idempotent` similarly uses
`append_decided_events`, completes the durable receipt, and only then fans out
under the same guard ([path](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2134>)).
The current oracle models both paths explicitly, which is correct. Future
writer additions must add an equivalent exact topology predicate rather than
forcing every transaction shape through the convenience helper.

## Two-Author Shell Composition And Retained-Browser Re-Audit (Current Source)

### Socket, cancellation, and restart ownership

The former composition socket-retirement finding is **superseded**. The
current `finishGisMapProcessDocumentSocket` removes message/open/error
handlers, waits for the exact `close` event or a bounded error/timeout, and
wipes both the plan receipt and retained frames in its `finally`
([owner](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10511>)).
`openGisMapProcessDocumentSocket` routes every post-socket open failure through
that same owner ([open failure path](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10503>)).
The composition's `closePeers` awaits Shell stop, each exact socket close, and
each MCP child before it clears credential strings
([composition owner](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10847>)).
It invokes that owner before the pre-restart Hub close
([restart order](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10949>))
and repeats it from the outer `finally`
([failure cleanup](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10964>)).
The socket close helper has a neutral closed-at-entry, close-event, timeout,
Bun WebSocket, and independent `ws` case
([fixture execution](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11490>)).
This is current-source evidence only: the recently observed physical socket
opening failure keeps the full composition unqualified until its registered
gate completes.

The FD4 control is process-owned and single-use. It records the exact entered
job before the MCP cancellation call, verifies that the returned cancellation
page names that same private job and has `cancelRequested`, then writes only
the matching release frame ([runner](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10735>)).
The native gate emits the entered frame and accepts only that exact release;
EOF, truncation, or a substituted job becomes `Denied`, rather than a release
([gate](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2514>)).
If the runner fails after `entered` but before its release, the outer cleanup
ends FD4 before bounded child retirement
([Hub owner close](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:993>));
the paused codec consequently fails closed rather than continuing as a
silently released job. No success/cancel receipt is emitted on that path, so
there is no current false acceptance.

The restart consumes all arrays with `splice`, closes the first Hub, then
issues fresh credential exchanges to a newly started Hub on the same data root
and port ([restart](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10949>)).
`startLocalHub` itself owns a failed bootstrap child and removes its temporary
run root ([failure path](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:862>)).
I found no present owner handoff that permits an old socket, MCP child, or
checkpoint-control pipe to be treated as the restarted peer.

**P1 regression-fence gap.** The neutral composition fixture currently only
asserts that the process owner retains the prepared data root/current, and
that it does not materialize or publish another current
([fixture source fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11591>)).
It does not make the now-important teardown ordering hostile. Add a
source-level hostile that fails if the first restart lacks the ordered
`await closePeers()` → `await finishLocalHub(run)` → second
`startLocalHub(... dataDir: prepared.dataRoot ...)` sequence, or if
`closePeers` ceases to call both `finishGisMapProcessDocumentSocket` and
`finishGisMapProcessMcpClient`. Add another hostile for removing the
open-socket catch's call to `finishGisMapProcessDocumentSocket`. This pins the
existing bounded lifetime behavior without treating the planned Shell facade
or Space-row work as complete.

### Browser candidate root/generation fence

No current cross-root acceptance bypass was found in the private fresh-browser
path. `trustedBootstrapCurrent(dataRoot)` parses only the four-field current
pointer and **derives** the bundle location under that supplied data root; it
does not accept a bundle path from pointer JSON
([current reader](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9792>)).
Before Chromium receives actor bytes, `proveTrustedGisClosedActorV1` captures
the selected current, requires the exact GIS+Stdio closure and component /
descriptor identities, and verifies the complete generation
([pre-child fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9453>)).
Its second fence rejects a changed pointer SHA/revision/profile/generation or
bundle path and re-hashes all selected leaves after the child returns
([post-child fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9474>)).

Candidate validation copies only the fixed five selected leaves into a
candidate-owned data root, verifies those leaves before and after staging, and
writes a new probe-only pointer that cannot replace a live pointer
([candidate stage](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9512>)).
The normal path runs cold-map proof and a Hub process against that distinct
candidate root before the one live publication attempt
([ordering](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10079>)).
The browser proof intentionally happens after live current publication; its
own output expressly excludes a browser-qualified-publication claim
([qualification boundary](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9501>)).
That is an honest boundary, not a source-generation mix.

The browser helper is private and its `current` argument is internal
materialization state. If it is ever exported or fed from a caller-provided
receipt, first require `current.bundlePath === join(dataRoot, "trusted-catalog",
"generations", current.generationId, "trusted-catalog.json")` *before* the
first `trustedBootstrapReadCurrentBundle(current, ...)`. Today the later
retained fence detects a substituted path before acceptance, but this
precondition would avoid even reading an arbitrary caller path. This is a
future API-hardening requirement, not a current publication bypass.

## Host-Owned Space Artifact Creation And Index Boundary (Current Source)

### Concrete current cut and P0 openability gap

The guest-side cut is now correct. `create-artifact` emits only the bounded
`name` and opaque `kindChoice` relay and asserts that its local snapshot stays
empty ([guest command](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:22>)). In particular, it no longer
mints an id or writes an artifact row. The Shell/Hub must resolve catalog
choice and mint the identity; a browser request must never supply either.

There is one security and consistency cut that the Hub creation saga must
close: **a durable descriptor is already enough to issue a document-open
plan.** The route reads the descriptor at
[bin.rs:2378](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2378>),
but reads the active checkpoint as an `Option` at
[bin.rs:2388](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2388>).
`DocumentOpenPlanAuthorityV1::validate` accepts `checkpoint: None`
([bin.rs:1247](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1247>)). Thus a
descriptor-first create/then-checkpoint sequence gives an authenticated client
an open plan for an incompletely created document. It is not sufficient to
delay only the Shell open effect.

### Do not append an SSpace mutation from the Hub

`SSpaceSnapshot` is a persisted index document with a UI row containing id,
title, kind/schema, dialect, and audit metadata
([snapshot](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:24>)). It is not a safe generic
server index write target: `SSpaceDiff.artifacts` replaces the *entire* vector
([diff](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:17>)). A stale server-read plus
`SSpaceMutation::CreateArtifact` can therefore erase a concurrent row. It
also makes the generic Directory service depend on the Space plugin's codec
and semantic mutation vocabulary. Do not add a raw Pack/opaque mutation
append, and do not reuse the guest's collision-probing `mint_artifact_id`
([snapshot helper](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:110>))
as a server identity authority.

The existing shared, domain-neutral durable index is already
`DirectoryEventBody::DocumentAnnounced { descriptor }`. The OS directory fold
keeps per-space `documents`, updates the count, and is idempotent
([fold](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs:54>),
[announcement projection](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs:165>)).
Use that event as the sole server-owned index operation binding. The Space
HostOnly/Shell owner can derive its local row/read model from the directory
event rather than asking Hub to mutate `SSpaceSnapshot`.

`DocumentDescriptor` currently holds immutable document and codec/bootstrap
identity but not the title or full immutable dialect needed to reproduce a
`SpaceArtifactRow` ([schema](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1438>)).
The smallest truthful extension is a domain-neutral, descriptor-owned
`DocumentIndexIdentityV1` (or equivalent fields) containing a bounded human
`display_name` and exact `{ artifact_kind, standard, subset }` dialect. It is
selected and validated by the server catalog, included in the canonical
descriptor digest encoding, and carried by `DocumentAnnounced`. The Space
projection then deterministically maps it to `{id,name,kindId,schema,dialect}`;
the event header supplies initial created/updated time and actor. Do not
re-resolve an old row through the mutable current catalog, or names/dialects
will drift after catalog rotation.

### Smallest correct generic durable creation seam

Add a **server-only, typed document-genesis operation**, not a public
`DirectoryCommand` and not a generalized artifact mutation API. A concise
shape is:

```rust
struct DocumentGenesisCommitV1 {
    descriptor: DocumentDescriptor,
    descriptor_event: NewDirectoryEvent,
    checkpoint: ArtifactCheckpoint, // genesis, parent=None
    checkpoint_event: NewDirectoryEvent,
    reservation: ArtifactCasReservation,
    completion: DocumentCreationCompletionV1,
}

DirectoryService::commit_document_genesis(
    DirectoryActor::system(...),
    DocumentGenesisCommitV1,
) -> DirectoryResult<DocumentCreationReceiptV1>
```

`DocumentCreationCompletionV1` is a durable claim keyed by exact authenticated
actor/session generation, request id, selected catalog generation, canonical
descriptor digest, and genesis checkpoint id/aggregate. It must have the same
strict release rule already used by directory/checkpoint receipts: release only
for a transaction-proven pre-commit rejection; I/O/commit ambiguity remains
queryable/reconcilable. The request-local owner may release its HTTP guards on
an accepted durable claim, but a retained server owner must resume the exact
claim/candidate after a dropped request or restart.

Validate all of the following under one `DirectoryService.write` acquisition:

1. server-derived selected catalog identity, exact descriptor/digest, genesis
   frontier (`parent=None`), and CAS plan;
2. current `User → Session → DirectorySpaceAuthority → Membership` fence at
   admission and immediately before the durable commit (never through WASM,
   CAS staging, or component work);
3. descriptor absent-or-byte-identical, reservation live/exact, and no active
   checkpoint; and
4. both events in this order: `DocumentAnnounced`, then
   `ArtifactCheckpointPublished`.

The **single backend transaction** must persist both ordered events, project
the descriptor before projecting the checkpoint, write private checkpoint
authority, consume the CAS reservation, and complete the creation receipt
before commit. Publish both events only after commit. Then change document-open
and execution-target selection to require `Some(active_checkpoint)` rather
than serializing `None`. This makes the public state transition atomic from
``not found`` to ``descriptor + usable genesis``.

This is a small extension of the existing CAS transaction machinery, but it
must be a distinct typed method rather than overloading its current single
event parameter. `decide_verified_checkpoint` currently requires the
descriptor to be already durable
([directory decider](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1847>)), and
`append_reserved_artifact_checkpoint` validates exactly one checkpoint event
and emits one persisted event
([service](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2309>)). An optional
second `DocumentAnnounced` argument would otherwise evade its current
`validate_verified_checkpoint_append` contract.

All three existing backends already have the needed transactional primitive;
extend each in lockstep with `append_document_genesis`, not a second
post-commit call:

- SQLite begins an `Immediate` transaction in
  `append_reserved_artifact_checkpoint`, then persists and projects the
  checkpoint before committing
  ([SQLite seam](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2055>)).
  Persist/project `DocumentAnnounced` first in that same `Transaction`, then
  persist/project the checkpoint and private/CAS records.
- Postgres already holds its per-space CAS transaction and writes the event,
  checkpoint authority, projections, CAS reference, and completion before
  `commit`
  ([Postgres seam](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2171>)).
  Allocate/persist two ordered directory sequences in the same transaction,
  then project them in order.
- Neo4j's `start_txn` branch does the same ordered graph mutation and commits
  only at the end
  ([Neo4j seam](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2053>)).
  Create the descriptor node/relationship before its checkpoint and retain
  every query result until it is dropped before `txn.commit()`.

The generic Directory fold and all three projections already treat a
`DocumentAnnounced` as the authoritative document index, so no parallel Space
backend table is needed. The existing current `fold-directory-events` command
is deliberately config-only; Home should add a separate derived directory
document-row projection, not smuggle durable index mutations through it
([command contract](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇fold-directory-events/🦀️.rs:1>)).

### Minimum acceptance laws before exposing `os.open-artifact`

Use an actual three-backend Directory fixture and a paused commit seam, with
the production creation route rather than a direct table write:

1. Pause after the descriptor event has been constructed but before the
   transaction commits: descriptor/open-plan/index are all absent. Resume:
   both ordered events, private checkpoint, CAS reference, and creation receipt
   become visible together.
2. Pause after CAS staging/reservation but before the final transaction;
   revoke member/session through the production path. Resume must write no
   descriptor/event/row/open effect; only bounded staged-CAS recovery remains.
3. Drop HTTP after durable commit and restart. The exact completion claim
   replays the one document/checkpoint identity; no second descriptor/event,
   no fresh id, and no duplicate CAS reference.
4. A forged document id, catalog kind, title/dialect, descriptor digest,
   checkpoint parent, or reservation is refused before append. An equal retry
   returns the same receipt; a substituted request/identity conflicts.
5. A source/physical route law confirms `DocumentOpenPlan` and
   `DocumentExecutionTarget` both reject a descriptor with no active genesis;
   the normal host relay occurs only after the receipt is Ready. This explicitly
   covers the currently openable descriptor-only state.
6. SQLite/Postgres/Neo4j replay/rebuild preserve descriptor-before-checkpoint
   order and the derived Space row has the immutable descriptor title/dialect.

No creation implementation or law has been run in this audit.

### Amendment: keep mutable index presentation out of `DocumentDescriptor`

The preceding descriptor-field suggestion is superseded by the cleaner
event-projection shape below. `DocumentDescriptor` is immutable codec and
bootstrap identity. A human title is mutable presentation metadata, so it
does not belong in its digest.

Add a typed, domain-neutral second event:

```rust
struct DocumentIndexEntryV1 {
    name: String,
    dialect: os_io::ArtifactDialect,
}

DirectoryEventBody::DocumentIndexed {
    scope: DocumentScope,
    descriptor_digest_v1: ArtifactHash,
    entry: DocumentIndexEntryV1,
}
```

`ArtifactDialect` is already the persisted/wire, `ToValue`/`FromValue` exact
type ([owner](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:68>)).
At projection and fold, require a preceding descriptor in the same scope,
require its digest to match, and require
`entry.dialect.artifact_kind == descriptor.artifact_kind`. The server derives
both from its chosen catalog; the client controls neither. Require
`NewDirectoryEvent.user_id` for this event and derive `createdAt`, `updatedAt`,
`createdBy`, and `updatedBy` from its immutable event header. That prevents a
system actor from fabricating a display creator field.

Give `DocumentIndexEntryV1` its own strict validator: non-empty, trimmed,
control-free `name`; each dialect component non-empty, trimmed, control-free,
and at the existing 256-byte public identity ceiling; and the canonical
artifact-kind grammar already required by the selected catalog. The reusable
Directory helper only checks non-empty/byte length
([`validate_bounded_auth_text`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:837>)),
while `ArtifactDialect` itself is an owned wire type rather than a complete
untrusted-event validator. Do not make either fact the sole validation for
this new public event.

The genesis transaction's public order becomes
`DocumentAnnounced → DocumentIndexed → ArtifactCheckpointPublished`. Each
backend persists/projects that sequence before its private checkpoint,
reservation consumption, receipt completion, and commit. A backend/rebuild
must refuse an index event that has no prior exact descriptor rather than
silently accepting an orphan. An equal replay is idempotent; a changed entry
for the same descriptor is a conflict. Future rename/delete work needs its
own typed directory event and authorizer; it must not silently fall back to
the old local `SSpaceMutation` row as authoritative metadata.

Extend `DirectoryReadModel` with an explicit `DirectoryIndexedDocumentViewV1`
or a keyed `document_index` alongside existing `documents`, then make the
Home config wire serialize it. This is necessary because present Space open
commands look only in `doc.snapshot.artifacts`
([open command](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️open-artifact/🦀️.rs:21>)); a
new Directory event alone cannot cause that old document vector to show or
open the row. The Space HostOnly owner should move table/open lookup to the
typed Directory-derived view. This removes the separate Space-index document
creation saga without making directory code depend on the Space plugin.

### Durable creation receipt, backend projection, and terminal recovery packet

**Do not reuse `CheckpointPublicationReceipt` as the creation operation.** It
is deliberately only `{ actorUserId, correlationId, commandSha256,
pending|completed, checkpointId }`
([model](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:457>),
[SQLite table](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:235>)).
It neither binds a session/generation or scope/document id nor retains a
catalog selection, descriptor/index payload, staged private locators, or CAS
plan. It cannot safely resume a dropped/restarted genesis. The generic
directory-command receipt is likewise an atomic short-command record with
only event range/result kind. Reuse their *exact key/digest validators and
`release only a proven pre-commit rollback` rule, not either table or wire
type.

Add a private, backend-owned `DocumentGenesisOperationV1` record keyed by
`(actor_user_id, request_id)` with a unique generated `operation_id` and an
exact immutable request digest. Its bounded fields must include:

- original user id, exact session id and authorization generation;
- scope plus server-minted document id, request name, selected catalog
  generation/selection digest, and the immutable descriptor digest once
  prepared;
- a closed phase: `Accepted`, `Prepared`, `Committed`, or
  `CancelledBeforeCommit`/`RejectedBeforeCommit`;
- in `Prepared`, the canonical descriptor, `DocumentIndexEntryV1`, exact
  private checkpoint, and CAS ownership plan/reservation facts needed to
  resume the *same* candidate; and
- in `Committed`, first/last directory event sequence, exact checkpoint id,
  and a redacted result digest. No capability plaintext, bearer, or browser
  receipt belongs in the record.

The record's initial `Accepted` insert is the sole point at which the HTTP
route may return an accepted/running creation receipt. An in-memory retained
task, a `tokio::spawn`, or a CAS write is not durable acceptance. After
admission, the task is only a bounded accelerator: startup/close reconciliation
must page `Accepted`/`Prepared` records and either reconstruct the exact
candidate from the record or terminalize it after a proven rollback. It may
not silently mint a new document id, re-resolve a newer catalog, or turn an
unknown commit outcome into rejection.

The durable record also needs a bounded **attempt epoch/lease** separate from
its request identity. A request retry, startup reconciler, and close-time
maintenance may all observe the same `Accepted` row; only the atomically
claimed current attempt may move it to `Prepared`, and that update must match
the operation id plus attempt epoch. A cancelled/expired attempt may leave
staged CAS data for ordinary bounded cleanup, but it cannot later publish. An
in-memory supervisor may coalesce live work; it is not the interprocess or
post-restart exclusion proof. Cap both retained records and live attempts, and
make a timed-out attempt recoverable by a new durable epoch rather than
unbounded task accumulation.

The final `commit_document_genesis` transaction receives the durable operation
id and exact prepared payload, verifies its phase and all identity fields, and
atomically writes:

1. `DocumentAnnounced` and its `hub_document_descriptor` projection;
2. `DocumentIndexed` and a generic `hub_document_index` projection keyed by
   `(space_id, document_id)` with descriptor digest/name/dialect/event seq;
3. `ArtifactCheckpointPublished`, private authority journal, and CAS reference;
4. the operation `Committed` completion.

Only after that commit may `DirectoryService` publish the three events or the
Hub emit `os.open-artifact`. A durable `hub_document_index` projection is
useful for bounded server administration/read windows; it is still a generic
Directory projection, not an SSpace table. It needs the same delete cascade as
the descriptor and must be wiped/replayed by all projection rebuilds. The
event log remains the reconstruction source; the private operation table and
private checkpoint journal are **not** wiped as ordinary event projections.

The current backend methods make this an additive, bounded extension:

- SQLite currently begins `TransactionBehavior::Immediate`, persists one
  event, projects it, writes checkpoint private authority/CAS, then commits
  ([transaction body](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2055>)).
  Persist/project the descriptor and index events before its present checkpoint
  sequence and update the operation row before `tx.commit()`.
- Postgres already uses one transaction with per-space CAS lock,
  `FOR UPDATE` reservation check, event sequence allocation, projections,
  private authority, CAS reference and receipt completion
  ([transaction body](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2171>)).
  Allocate the three event sequences and project them in order in that one
  transaction.
- Neo4j's same operation is one `start_txn` chain
  ([transaction body](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2053>)).
  It must drop each `neo4rs` result before the next query/commit while creating
  descriptor relation, generic index node/fields, checkpoint/private authority,
  CAS reference, and committed operation state.

All three projection rebuilders currently reconstruct descriptor/checkpoint
from events and private checkpoint journal (for example SQLite
[replay loop](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2370>)).
Add the index-table truncate and `DocumentIndexed` projection to each replay.
A replayed index event without an already projected exact descriptor digest
must fail the whole rebuild transaction. This catches an impossible ordering
rather than rendering a row from unbound metadata.

### Authority and cancellation cut

At HTTP creation admission, resolve user/session and the catalog choice,
mint the new id server-side, then acquire the sorted existing gates
`User → Session → DirectorySpaceAuthority → Membership → DocumentWrite(scope)`
and revalidate user/session plus current `Author` role. This mirrors the
current real directory command fence, which holds User/Session plus space and
member bindings before the writer
([route fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4864>));
the key enum makes `DocumentWrite` the final sorted binding
([key order](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:716>)).
Do not retain these guards during catalog materialization or CAS staging.

Before `Prepared → Committed`, reacquire the same five gates and revalidate
the recorded exact session/generation, space existence/non-archive state, and
current Author membership. The backend commit also needs an in-transaction
space/membership check (not merely an earlier HTTP read), so a winning
membership removal/archive has a defined serial result. Extend the intrinsic
projection guard: it presently examines only archive/member events
([current scope](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1916>)),
therefore it would not reject a direct `DocumentAnnounced`/`DocumentIndexed`
write into an archived space. Give document announce/index/checkpoint writes a
typed `ArchivedDocumentWrite` rejection under the same backend writer
transaction. This is defense in depth for every backend and rebuild, while
the Hub gate provides the ordinary revocation race linearization.

Cancellation is a durable compare-and-transition, not task abort. The private
status/cancel endpoint authenticates the record's exact owner/session and
acquires the same binding union. It may change only `Accepted` or `Prepared`
to `CancelledBeforeCommit` after the prepared candidate/reservation is known
not committed; final commit verifies non-cancelled state in its own transaction.
If commit wins, cancellation reads `Committed` and returns settled. If a
session/member/archive revocation wins, reconciliation terminalizes cancellation
without events. A transport/commit failure after the final transaction starts
is **Indeterminate to the caller**, leaves the durable record intact, and
reconciles by exact operation id/descriptor/checkpoint; it must never delete
the claim or report `RejectedBeforeCommit`.

Required three-backend laws add: paused `Accepted` cancellation before any
side effect; paused `Prepared` versus member removal/archive; final commit
versus cancel with exactly one winner; crash/reopen on each phase; exact
reconcile of `Committed`; an injected pre-commit rollback that alone produces
`RejectedBeforeCommit`; and rebuild rejection of index-before-descriptor or
digest substitution. None has been run in this audit.

### Exact backend writer and public-fold boundary

The existing generic idempotent command receipt is **not** a suitable creation
receipt even though it has the right request-key vocabulary. Its pipeline
appends in one backend transaction and then calls
`complete_directory_command_receipt` in a second transaction
([service](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2134>),
[SQLite completion](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1567>)).
A disconnect at that boundary is intentionally reconciled by the established
generic command flow, but it cannot establish the exact prepared
descriptor/index/checkpoint/CAS identity required here. The closest reusable
*shape*, not storage record, is the checkpoint append: it writes public event,
private authority, CAS reference, and its completion before one commit
([SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2055>),
[Postgres](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2171>),
[Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2053>)).

`append_document_genesis` should therefore be a required, separate
`HubDirectory` authority method and must reject use through
`append_decided_events`: that convenience seam expressly excludes
`ArtifactCheckpointPublished` ([SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2294>)).
Each implementation needs this one transaction shape:

1. claim the existing backend writer/CAS-space fence in the same order as the
   checkpoint append, then read current `hub_space` and the initiating
   membership under that transaction; require non-archive space and `Author`;
2. verify the operation id, request digest, attempt epoch, exact prepared
   payload, and still-live CAS reservation; allocate a contiguous three-event
   sequence range while that writer is held;
3. persist/project `Announced`, then `Indexed` (including exact descriptor
   digest), then `CheckpointPublished`; insert private authority/CAS reference
   and flip the genesis operation to `Committed` in that same transaction;
4. commit; only the service holding its `HubClock` writer may then fan out the
   three committed events.

The ordering is important for PostgreSQL and Neo4j: do not pre-lock membership
before their current head/CAS writer path, or a membership event that already
holds the head can form a new lock inversion. SQLite's `Immediate` transaction
already serializes writers. In Neo4j every result cursor from the counter,
membership, and CAS checks must be dropped before subsequent query or commit.
The backend transaction, not a retained in-memory task, is the linearization
point for the role and archive decision.

Add `hub_document_index` as a generic child of
`hub_document_descriptor`: `(space_id, document_id)` primary/foreign key,
`descriptor_digest` fixed 32 bytes, bounded name, canonical serialized
`ArtifactDialect`, and its `event_seq`. The projection must delete it before
the descriptor in per-space cleanup and full rebuild in SQLite
([per-space cleanup](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:904>)),
Postgres ([per-space cleanup](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:821>)),
and Neo4j (the corresponding descriptor/index node relation). The current
descriptor child foreign keys already make space deletion cascade; no parallel
SSpace index table or opaque mutation event is needed.

For public Rust/TS folds, `descriptor_digest_v1` is an **opaque server
witness**, not a value that the browser must recompute with a new SHA
implementation. The server append and all backend rebuilds verify it exactly.
The pure fold projects an index row only after a matching folded descriptor at
the same scope exists and direct, non-cryptographic fields agree
(`dialect.artifact_kind == descriptor.artifact_kind`, bounded valid entry).
An index-before-descriptor or mismatch yields no visible row/recovery signal;
it never manufactures an open right. The subsequent Hub open-plan and target
routes must still revalidate current descriptor plus an active checkpoint, so
a tampered local view cannot obtain access. Required pure fold rows are:
`Announced → Indexed` one visible row; index-first/mismatched kind none; and
no fold path issues an open request. This avoids a second, unreviewed TS hash
implementation without weakening server log integrity.

One further backend detail is non-negotiable: the present descriptor projectors
use idempotent `INSERT … DO NOTHING`/`MERGE` behavior
([SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:930>),
[Postgres](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:851>),
[Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:571>)).
The genesis path must not use that as a substituted-id success. Before the
first event it requires the scope absent (or an already `Committed` operation
with byte-identical prepared identity); after its descriptor insert it must
verify the stored descriptor/digest equals the prepared one. A collision is a
typed pre-commit refusal, never a three-event log whose projection silently
points at an older descriptor. Extend `MemoryArtifactProjection`'s atomic fold
with the same descriptor→index→checkpoint ordering check so its parity laws
cover the new invariant rather than only physical backends.

### Current `DocumentIndexed` implementation audit (source only)

The newly landed Directory shape is coherent at the schema/fold boundary. The
Rust owner is [document-index-v1](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🦀️.rs:8>), re-exported by the Directory schema, and the event is a first-class
`DocumentIndexed { scope, descriptor_digest_v1, entry }` variant. The JSON
schema, TypeScript discriminated union/parser, Rust fold, TypeScript fold, and
Home `DirectorySpaceWire` now all contain the field. In particular, Home maps
`indexed_documents` in both directions rather than silently dropping it
([wire conversion](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:79>)).

The three physical projectors have also landed: each reads the already
projected descriptor in its writer transaction, runs the shared exact
descriptor/digest/kind helper, and only then stores an immutable per-scope
index payload ([SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:944>),
[Postgres](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:868>),
[Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:586>)). All three add cleanup/rebuild handling;
Neo4j explicitly drops each result cursor before its next query. The transient
`directory_message_matches_scope` exhaustiveness hole was also fixed: a
document socket now delivers `DocumentIndexed` only when its exact
`DocumentScope` matches ([scope filter](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5691>)).

Two bounded follow-ups remain before treating the new module as qualified:

1. The new language-neutral corpus exists at
   [document-index-v1 fixture](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🔣️.json:1>). It is now consumed by
   `proveSpaceArtifactCreationContractV1` in Hub `📜️script.ts` and by the
   second creation-module Rust law, which exercise the Rust pure fold and
   `MemoryArtifactProjection`. The remaining qualification work is narrower:
   wire the same oracle to the TypeScript fold and to every physical
   SQLite/Postgres/Neo4j projection **and rebuild** law. Its useful rows cover
   ordered, replayed, index-before-announcement, kind/document/space mismatch,
   missing author, invalid name, and forged digest. Add the actual
   document-socket same-scope/foreign-scope delivery rows to the existing
   socket law rather than a synthetic filter test.
2. Rust server validation rejects an all-zero index digest
   ([validator](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:281>)); the TypeScript page parser and both public folds currently accept it because
   `hash()` establishes only a 32-byte array. This is not a server-side
   authority bypass—the backend helper rejects it before persistence—but it is
   a wire-language parity gap. Reject all-zero in the TypeScript parser too
   and add a `zero-digest` corpus row (`clientRows: 0`,
   `backendAccepted: false`). The intentional `forged-digest` row should
   remain visible client-side: the browser has no SHA implementation and its
   digest is an opaque protected-server witness; the backend is the verifier.

The creation writer still needs to make `Announced → Indexed → Checkpoint` one
durable transaction. `directory_projection_space_v1` now includes all three
and rejects direct writes into archived spaces
([intrinsic guard](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1942>)), but ordinary individual appends can still leave an index without a
checkpoint. Home must therefore derive/open only from the active checkpoint
plus index; the index itself is never proof that a descriptor became openable.

### Native 60628 catalog conflict diagnosis

The reported native run `60628`/`KBimMp` did execute the first four selected
laws and then failed the fifth, exact `linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure`, with
`document codec registration conflicts for schema gis.map`. This is a real
same-process ownership conflict, not a fixture ordering artefact.

`prepared_gis_binding_fixture` builds the real GIS plugin merely to obtain the
descriptor ([fixture](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1869>)). However, `PluginBuilder::try_build` commits its declared artifacts—including
the two document codecs—to the process registry
([builder commit](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:724>)). The fixture's observed `initial-public-codecs=2` is exactly those
GIS map/terrain registrations. Later the trusted loader legitimately attempts
to publish the receipt closure atomically.

The two construction paths disagree on a structural identity field. Ordinary
declaration construction preserves `ArtifactCodec::of`'s record-spec
`pack_schema_hash`; the private GIS receipt constructs the same typed codec
but overwrites that field with SHA-256 of `*.protocol.semio`
([receipt](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs:76>)). The registry deliberately compares schema, extension, structural hash,
and every erased function pointer ([identity rule](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9640>)), so rejecting the duplicate is correct.

The repair must not weaken that rule. Make the native receipt obtain and retain
the *actual* `ArtifactCodec::of::<Snapshot, Mutation>` structural hash, and
assert it agrees with the declared codec. If the binary protocol digest remains
necessary, carry it as a separately named receipt/descriptor commitment rather
than relabeling `pack_schema_hash`. Route declaration registration and receipt
consumption through one package-private canonical factory, or at minimum add a
law that compares schema, extension, structural hash, and all four function
pointers before loader registration. A side-effect-free descriptor projection
would make this fixture less contaminating, but does not replace that
runtime-correctness repair: a real host may have an installed GIS plugin before
loading the trusted current, and only truly identical codec ownership may be
idempotent.

### Exact zero-history artifact genesis (implementation packet, source only)

The requested genesis must mean **an empty edit/commit history**, not an empty
or host-invented GIS Map. The selected artifact application's own initial
snapshot may legitimately contain domain-provided starter content. For GIS Map
that authority is `Gis2dPlayApp::initial_snapshot`
([app implementation](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:747>)), not the Hub choosing either
`default_document` or an empty-map helper. The factory must wrap that snapshot
with the normal `create_document_envelope` and `print_document_pack` path
([envelope constructor](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10025>)); it produces a nonempty valid `.pack/.spr` pair with no
edit. No synthetic GIS mutation is involved.

Use the existing exact zero actor frontier as the sole genesis value:

```text
documentId = scope.document_id
headEditOrdinal = 0
headEditId = ""
lastCommitSeq = 0
chainHash = [0; 32]
parentCheckpointId = None
```

This is already the only no-current baseline accepted by the GIS publisher
([predicate](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3429>)) and is how an unopened DB actor represents zero history. It must become a
**domain-valid `ArtifactGenesis` only inside the private creation authority**;
do not relax the public `CheckpointPublicationFrontierV1` decoder, which
correctly requires a nonzero normal edit frontier
([wire type](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:388>)). Likewise, keep
`CanonicalArtifactAuthority::materialize_checkpoint` normal-only: its
`validate_request` rejects the zero/empty baseline
([authority validation](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:349>)).

The minimal new private authority operation is
`materialize_genesis(descriptor, scope, document_id, context)`. It must:

1. validate the server-minted scope/document id and descriptor digest, resolve
   the exact trusted codec identity, and invoke a new **codec-owned initial
   pair factory**; current `TrustedArtifactCodec` exposes only validation and
   application ([trait](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:241>)), so the Hub cannot honestly create a pair today;
2. require both pair files nonempty, run the same codec input/output validation
   and pair byte limits as normal materialization, and derive all blob hashes,
   aggregate, CAS ownership, descriptor digest, and checkpoint id from those
   server-owned bytes;
3. emit no accepted operation and no mutation envelope. The type-erased
   `ArtifactCodec` is insufficient by itself because it has decoding,
   printing, and apply pointers but no initial snapshot factory
   ([codec fields](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9464>)). Keep the initial-pair closure in the
   package-owned native receipt/binding, then carry it privately to
   `VerifiedNativeArtifactCodec`; do not make Hub name GIS snapshot types.

`checkpoint_id_encoding_v1` already includes the empty id, zero ordinal,
zero commit, zero chain, descriptor, and both individual blob identities
([encoding](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:366>)). Add a distinct
`GENESIS_CHECKPOINT_ID_V1_DOMAIN` selected only for the complete five-field
zero-history form. This makes the initial pair's identity domain explicit
without pretending its empty head is an edit. A partial zero form is never
canonical.

#### Required shape and lineage hardening

Introduce one shared predicate at the Directory schema/authority boundary:
`is_exact_zero_history_frontier(scope, frontier)`. It requires all five fields
above; no partial zero values are accepted. Then use its complement for the
normal form: nonempty `head_edit_id`, nonzero chain, and positive
`head_edit_ordinal` plus `last_commit_seq` (they need not be equal; existing
GIS frontiers legitimately differ). Apply it to:

- `validate_checkpoint_shape` in
  [Hub Directory](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1437>): zero is valid **iff** `parent_checkpoint_id` is `None`; every
  normal checkpoint requires a parent. This closes today's parentless edited
  first checkpoint path.
- both `MemoryArtifactProjection` and `decide_verified_checkpoint`
  ([memory lineage](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1573>),
  [durable decider](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1886>)). With no
  active checkpoint they accept only exact genesis; with an active checkpoint
  they reject genesis, require `parent == active.id`, and keep the existing
  strict ordinal/commit advance. Thus an ordinary first user edit cannot skip
  genesis.
- `ArtifactRetention` shape if genesis is retainable. It may accept the exact
  zero frontier only because later lineage lookup proves it equals the named
  genesis checkpoint; all partial zeros remain invalid. The current retention
  validator rejects empty/zero unconditionally
  ([validator](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1472>)).

Do **not** map zero to a string such as `"genesis"`: that turns a history root
into an apparent user edit and would make the ordinary `ArtifactFrontier`
wire ambiguous. The temporary structural reuse is safe only because the
context is private genesis materialization plus the exact all-zero predicate.

#### Pair, actor, and bootstrap handoff

There is a concrete asymmetry to preserve. Generic
`HubState::ensure_document` only lazily creates a blank `db::ArtifactHandle`
([method](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1853>)); it does **not** consume a persisted checkpoint pair. Therefore
the creation transaction must not claim that merely adding a Directory
checkpoint initialized every generic DB actor.

The current GIS path is nevertheless compatible with exact zero genesis:
`map_base` reads the active verified pair, including its frontier, from
`VerifiedRebootstrapSource` ([pair reader](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3177>));
`mount_document` decodes that same pack and compares it with the blank actor's
checkpoint snapshot before constructing retained GIS stores
([mount](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:936>)). The
zero actor snapshot and exact zero Directory genesis match. Socket delivery
itself sends only a `RebootstrapRequired` control; the canonical-pair HTTP
route independently calls the same verified active-pair reader before it
streams bytes ([route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2960>),
[active reader](</Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:219>)). Thus the
browser worker must still fetch and acknowledge that pair; the control frame
alone does not initialize it.

This gives the required first normal approval sequence: it reads pair+zero
baseline, commits a real first mutation in the actor, and publishes a normal
checkpoint whose parent is the genesis checkpoint. The current publisher
already requires `Some(current)` to equal the request base exactly, so it will
fence this transition correctly. A nonempty synthetic genesis would instead
conflict with the blank actor; do not use that design.

#### One required durable creation seam

`CheckpointPublicationOrchestrator::publish_candidate` cannot be reused as
the final writer unchanged: its publisher consumes a reservation and appends
only a checkpoint event after a descriptor already exists
([orchestrator](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:440>),
[`publish_reserved_artifact_checkpoint`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2343>)). Add the previously proposed
`append_document_genesis` backend operation instead. Under the one Directory
writer and one physical backend transaction it consumes the live CAS
reservation and writes exactly:

1. `DocumentAnnounced` by the revalidated initiating Author;
2. `DocumentIndexed` by that Author, with the exact descriptor digest;
3. `ArtifactCheckpointPublished` by the private System authority, with the
   exact zero-history genesis and private locators/CAS references;
4. the exact creation operation/receipt completion.

The index event needs a user actor because the current projection records
`created_by` from `event.user_id` ([projection](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1508>)); the checkpoint must remain System-only. The
service may publish the three events only after that transaction commits.
Cancellation before commit has no ready coordinate and may release an
uncommitted receipt/reservation; any error after commit admission is
Indeterminate and reconciles the exact creation key—never a failed receipt.

Minimum native matrix: actual selected GIS creation produces a codec-validated
nonempty pair with empty history; exact zero genesis passes all three
backends/rebuild/reopen and rebootstrap returns byte-identical pair; each of
the four partial-zero variants, parented genesis, and parentless normal edit
is refused with zero descriptor/index/checkpoint/CAS-reference append; the
first GIS approval has `parent == genesis.id` and a strictly advanced normal
frontier; and cancellation at pre-append versus ambiguous post-commit follows
the typed creation receipt rule. None of these genesis laws has been run in
this audit.

#### First-edit validation and verified-native factory boundary

`CheckpointRequest.base_frontier` is both the authority operation baseline and,
when `operations` is empty, the materialized checkpoint baseline. That makes
two first-edit forms materially different:

1. The existing GIS approval path has already applied its accepted mutation to
   the actor before calling `materialize_checkpoint`. Its publisher supplies
   `parent_checkpoint_id = Some(genesis.id)`, the **normal post-WAL actor
   frontier** as `base_frontier`, and no authority operations
   ([publisher request construction](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3450>)). This remains valid with the
   current `validate_request`; it never presents zero history to that method.
2. A future authority-applies-operation caller starts from the verified genesis
   pair. It must submit `parent_checkpoint_id = Some(genesis.id)`, exact zero
   `base_frontier`, and at least one operation. The first has sequence `1`; its
   resulting frontier, and therefore the final materialized frontier, must be
   fully normal (positive ordinal/commit, nonempty head id, nonzero chain).

The second form cannot work while `validate_request` unconditionally rejects
an empty `head_edit_id` ([current guard](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:349>)). Do not relax normal validation. Add only an
internal `is_exact_zero_history(scope, frontier)` branch before the ordinary
input-frontier check: it permits zero **only** with a nonzero supplied parent
and a nonempty operation list; the existing sequence and resulting-frontier
loop then makes the first output normal. An empty operation list with zero,
any partial-zero tuple, and any `parent=None` request remain refused. The
directory writer—not a hash-shaped request alone—must prove that that parent
is the active exact-zero genesis checkpoint before appending it. This is the
necessary complementary change to make a real authority-applied first edit
possible without a parentless-checkpoint loophole.

The public/durable lineage state also needs an explicit `Genesis {
checkpoint_id }` arm beside `None` and `Active`, rather than encoding zero in
`CheckpointPublicationFrontierV1`, whose validation deliberately requires a
normal positive frontier ([wire validator](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:390>)). The normal
first-edit command then carries `Genesis { id }`; its current/replay checks
require current id equality and `parent == id`. `None` must not authorize a
normal first checkpoint once document creation guarantees genesis.

Keep genesis-pair construction out of generic live plugin authority. The
generic `TrustedArtifactCodec` currently exposes validation and operation
application only ([trait](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:241>)); extending it would allow a live
`PluginHostTrustedArtifactCatalog` to create a document. Instead add a
private, receipt/binding-owned initial-pair function to
`NativeCodecBinding`/`VerifiedNativeArtifactCodec` and expose a private
`VerifiedTrustedCatalog` resolver used solely by the server creation owner.
It must resolve the selected verified native receipt and call that selected
artifact app's own initial-snapshot/print path. `ConfiguredArtifactAuthority`
already starts from `VerifiedTrustedCatalog`, not the live plugin host
([configuration](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:413>)); preserve that separation and offer no fallback.

Required focused laws: (a) current GIS post-WAL approval with a genesis parent
still calls normal `validate_request`; (b) a direct authority-applied first
operation succeeds only with exact zero input, named active genesis parent,
sequence one, and normal output; (c) zero plus no operation, zero plus absent
or non-genesis parent, all partial zeros, and normal parentless requests append
nothing; (d) a generic/live `PluginHostTrustedArtifactCatalog` cannot invoke
the genesis factory, while the verified-native selected codec can; and (e) a
nonempty application initial snapshot still yields empty history rather than a
fabricated edit. These are proposed laws, not executed results.

### Landed genesis candidate audit and durable-append packet (source only)

The new [`materialize_genesis`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:24>) has the right local candidate shape: it creates
the descriptor and `parent=None` exact-zero checkpoint from one pair, enforces
the pair ceiling before both semantic validations, uses a separate genesis
checkpoint-id domain, and keeps the pair in a local success-only candidate.
`OperationContext::report` checkpoints after each reported stage, so
cancellation/deadline before a returned candidate causes no Directory or CAS
write ([context](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:159>)). A
codec factory must nevertheless be specified as pure/deterministic with respect
to its verified identity, document id, and dialect: after a dropped
pre-publication attempt the durable owner must be able to regenerate an
identical pair/checkpoint, rather than silently bind a new random/timed
snapshot to the same creation request.

One bounded preflight detail remains: `published_at_ms` currently comes from
`context.now_ms()` after pair work, while Directory later requires it to fit
`DIRECTORY_WIRE_INTEGER_MAX`. Capture/check that value at the last candidate
checkpoint before constructing the checkpoint, so an out-of-wire-range test
clock is a local `InvalidFrontier`/resource refusal rather than a completed
factory candidate that cannot ever commit. The value is excluded from checkpoint
identity, so this does not affect deterministic pair or checkpoint-id replay.

#### P0: the present factory capability is not native-selected yet

The method is currently public for **every** `C: TrustedArtifactCatalog` whose
codec implements the public `TrustedArtifactGenesisCodec`
([generic implementation](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:22>),
[public trait](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:256>)). Its request also
contains public raw `identity`, `dialect`, and bootstrap frontier fields.
Syntactic `SpaceArtifactCreationReadyV1` validation proves only that the two
given kind strings agree; it does not prove that they came from the immutable
verified current open selection. A future live
`PluginHostTrustedArtifactCatalog` could therefore acquire creation power just
by implementing the extension trait.

Make the creation port private/sealed and concrete: only
`VerifiedNativeArtifactCodec`, constructed from a selected
`NativeCodecBinding`/native receipt, implements the internal initial-pair
function; only `VerifiedTrustedCatalog` exposes selection-derived genesis
coordinates; and only the Hub server creation owner invokes it. Construct the
request from the exact `VerifiedDocumentOpenSelectionV1` plus the server-minted
document id/name—not from a caller-provided `TrustedArtifactIdentity` or
`ArtifactDialect`. `ConfiguredArtifactAuthority` already retains both the
verified catalog and normal authority
([configuration](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:413>)); add a
non-public verified-native creation member there rather than extending
`PluginHostTrustedArtifactCatalog`. The native binding must be checked against
the selected identity, schema, pack-schema hash, and parent dialect before its
factory executes, and must not register a codec or publish any global state.

The `DocumentFrontier` `epoch` is a DB/runtime epoch, not the artifact edit
history represented by `ArtifactFrontier`; existing descriptor fixtures have
zero head/commit with epoch one. Do not globally redefine no-history as epoch
zero. For this **new server-minted initial document**, have the concrete Hub
owner construct `{ head_seq: 0, commit_seq: 0, epoch: 0 }`, as the current GIS
selection fixtures do. The factory must not accept that frontier from a client.

#### Required Directory acceptance split

The candidate cannot yet be appended: `validate_checkpoint_shape` rejects an
empty head id and zero chain
([validator](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1437>)); `MemoryArtifactProjection`
and `decide_verified_checkpoint` meanwhile accept an arbitrary normal
parentless checkpoint ([memory](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1587>),
[durable](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1896>)). Replace all three with
the same exhaustive lineage rule:

| Persisted active lineage | Candidate required before any append |
| --- | --- |
| absent | `parent=None` and `baseline_frontier.is_genesis_for(scope)` |
| present | `baseline_frontier.is_edited_for(scope)`, `parent=active.id`, strictly higher ordinal and commit |

The shape validator first accepts either exact predicate, then retains all
nonzero checkpoint/descriptor/blob/aggregate hashes, positive blob lengths,
canonical checkpoint-id encoding, and bounded fields. No partial zero is
valid. Apply the same split to retention only after its named lineage lookup
proves the retained checkpoint is the genesis root.

Both [`DocumentOpenPlanV1::validate`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1712>) and the execution-target lease still
unconditionally deny an empty/zero checkpoint. They must accept the same
exact-root predicate or fully edited predicate, never merely omit the
head/hash checks. The public generic checkpoint-publication command needs the
previously proposed `Current::Genesis { checkpoint_id }` rather than a normal
frontier encoding; its current `None` branch would otherwise admit an
unrelated parentless normal publication.

#### One durable `append_document_genesis` operation, not three calls

Do not compose `AnnounceDocument`, `DocumentIndexed`, and
`publish_reserved_artifact_checkpoint`: the existing special append has room
for only one checkpoint event and assumes the descriptor is already durable
([service](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2343>),
[backend port](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2727>)). Add a required
`HubDirectory::append_document_genesis` and one `DirectoryService` wrapper
that holds the clock writer once, derives exactly three events in order, calls
one backend transaction, then broadcasts the returned three events before
releasing the writer:

```text
Author DocumentAnnounced(descriptor)
Author DocumentIndexed(scope, descriptorDigest, indexEntry)
System ArtifactCheckpointPublished(exact genesis)
private ArtifactCheckpoint + CAS reference/ledger consume + creation receipt=Committed
```

The backend input must be a typed `DocumentGenesisAppendV1`, not a free event
vector: exact selected catalog generation/identity, descriptor and digest,
index entry, verified checkpoint/private locators, live exact CAS reservation,
actor/session/space/membership fence identity, and durable operation key/digest.
It preflights all semantic refusal cases before `COMMIT`; only then is a
rejection rollback-proven and eligible to release/retry. A post-`COMMIT`
transport error, or cancellation after the backend begins its commit, is
`Indeterminate`; reconciliation reads the exact creation key and returns the
same ready coordinates only for the byte-identical committed tuple. Staged but
unreferenced CAS objects are permitted crash residue and swept through their
unconsumed/expired reservation; they must never make a descriptor or open plan
visible.

Claim a durable creation record before expensive native work, with the
server-minted id, canonical user request digest, selected catalog generation,
identity/dialect, and current session/membership generations. Before staging,
bind its exact derived descriptor/checkpoint/CAS plan; retries must either
regenerate byte-identically and match that binding or reconcile/return
Indeterminate. This prevents a nondeterministic initial factory from turning a
retry into a second unseen genesis. Revalidate the short-lived authority fence
again immediately before `append_document_genesis`; do not retain it through
factory execution or physical blob staging.

Backend implementation seams are the existing single-transaction special
checkpoint appenders: SQLite
[`append_reserved_artifact_checkpoint`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1660>), Postgres
([`FOR UPDATE`/space lock path](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2187>)), and Neo4j
([transactional space lock](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2072>)). Each must persist the dense event sequence,
descriptor/index projections, private authority journal, CAS ledger/reference,
and completion receipt in that same transaction; rebuild replays the public
triple and checks the private checkpoint journal at the checkpoint event just
as it already does for ordinary verified checkpoints. No in-memory accepted
task is a substitute for this record.

Do not reuse the ordinary `DocumentAnnounced` projection's SQLite `INSERT OR
IGNORE` for this operation
([projection](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:937>)). Before reserving
the three event sequence, each backend must read/lock the scope's descriptor
and index row: absent is the only new-create state; an exact matching
**Committed creation receipt** returns its original triple; every other
descriptor/index/checkpoint presence is a conflict. This same check is needed
in PostgreSQL and Neo4j despite their different projection upsert syntax. It
prevents a conflicting preexisting descriptor from being silently retained
while the special append records a different genesis receipt or checkpoint.

Minimum acceptance laws, unrun: native selected GIS factory twice yields the
same exact pair/checkpoint; foreign selected identity/dialect/schema and live
catalog factory attempts are refused before factory execution; cancellation at
every candidate progress boundary leaves no event, index, receipt completion,
or CAS reference; three backends append exactly the ordered triple or zero
rows; duplicate exact request reconciles one triple after restart while a
substituted request conflicts; a fault after transaction commit is
Indeterminate and later resolves Ready; and missing/partial genesis plus a
parentless normal checkpoint leave all projections and socket publication
unchanged.

### Current source correction: verified catalog selection now landed

The production method has since moved from the generic extension described
above to the concrete
[`ValidatingCanonicalArtifactAuthority<Arc<VerifiedTrustedCatalog>>::materialize_genesis`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:21>).
It derives package identity and dialect from the catalog's sole Editor target,
then resolves that exact identity. The generic helper is private, so a live
`PluginHostTrustedArtifactCatalog` no longer has a callable production genesis
route. That closes the earlier generic-capability P0.

The initial two implementation blockers are now repaired in the current source:
`ArtifactGenesisRequest` has only scope and kind, and the helper constructs
exact `{ head_seq: 0, commit_seq: 0, epoch: 0 }`
([request and construction](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:9>)).
It also checks the clock before factory work and again immediately before
candidate construction, using the latter checked observation as the published
timestamp
([preflight](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:42>),
[candidate](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:66>)).
This closes the caller-supplied epoch and late out-of-wire publication-clock
risks. A focused native law should still prove an out-of-range clock is refused
before `initial_pair` is called and the returned descriptor always has exact
zero `DocumentFrontier`.

The Stdio capability split is likewise now structurally correct: a
`NativeCodecBinding` has an optional genesis factory, `new` leaves it absent,
and `with_genesis` is used only for GIS/VCS
([binding](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:305>),
[Stdio provider](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:167>),
[GIS provider](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:91>)).
`artifact_creation_selection` now additionally matches the full immutable
package/artifact/schema/hash tuple against a genesis-capable codec
([selection](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:526>)).
That preserves the complete 26-codec Stdio catalog while denying an attempt to
create an artifact whose selected Editor lacks a receipt-owned initial factory.
The remaining test obligation is a catalog with an Editor target that matches a
codec without genesis: creation must refuse before any factory/codec validation
and must not alter the registered codec closure.

`native_artifact_genesis_for_editor` correctly rejects a mismatched dialect and
checks that the envelope has no edits/changes/checkpoints/alternatives and a
zero cursor
([factory](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27283>)).
It is nevertheless an opaque async factory with no internal
`OperationContext`; the verified wrapper can check only before and after its
await ([wrapper](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:451>)).
For the current small static initial snapshot that is a bounded implementation
contract, not an interruptible CPU proof. If a factory gains unbounded work,
pass a bounded context/progress port into the factory rather than claiming
mid-factory cancellation.

### Exact durable backend transaction shape (current appenders)

The three ordinary special appenders are the correct implementation bases:
SQLite takes `TransactionBehavior::Immediate` and performs its CAS reservation,
event, private journal, projections, CAS ledger/reference, optional receipt,
then commit in one transaction
([SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2073>));
Postgres explicitly locks the space and reservation under one transaction
([Postgres](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2187>));
and Neo4j begins a graph transaction and acquires the same per-space CAS fence
([Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2072>)).
`append_document_genesis` should mirror each exactly, but persist three dense
events in order. Its private authority-journal and CAS-ledger rows must name
the third (checkpoint) event sequence, while descriptor/index projections run
after their respective first/second events in the same transaction. The
backend must not call the normal single-event appender three times.

Preflight inside those transactions is required before allocating any event
sequence: lock/read the space, descriptor, index, active checkpoint,
creation-receipt and reservation. New state means all are absent except the
live reservation; exact completed creation receipt means return its stored
three-event receipt; any other presence is conflict. The normal SQLite
`INSERT OR IGNORE` descriptor/index projections
([descriptor/index](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:940>)
are insufficient as a creation oracle, and PostgreSQL/Neo4j have analogous
conflict-ignore index upserts. The creation appender must read/compare before
using them. This prevents a substituted existing descriptor, a stale index,
or a completed-but-different receipt from being silently retained.

The public shape/decider now use the required exact two-branch frontier rule;
the remaining backend transaction issue is recorded in the current lower-seam
section below. Genesis append still needs three event actors: the
authenticated/revalidated Author for announce/index (the index projection
requires `user_id`), and System for the checkpoint because
`validate_verified_checkpoint_append` rejects any other actor
([validator](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1484>)).

### Codec extension is a suffix, not a Pack envelope identifier

The current `ArtifactCodec::of` stores `P::envelope_id()` in its `extension`
field ([implementation](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9595>)). That conflicts with the type's own contract, which calls the
field `P::EXTENSION`, and with `ArtifactDsl`: `EXTENSION` is expressly the
single-segment suffix used by codecs, while `envelope_id` is the dotted
`plugin.artifact` identity used by `.semio` envelopes
([trait](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4906>)).

This is an actual no-clobber failure, not a receipt-policy discrepancy. The
plugin builder writes `DocumentCodecSpec` and `codec-extension` claims from
`ArtifactDsl::EXTENSION` ([builder](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:3404>)), then the receipt validates the typed
codec against that suffix. GIS therefore declares `gismap` while the generic
codec produces `gis.gismap`; VCS similarly declares `vcs`
([GIS receipt](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs:35>),
[VCS receipt](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🦀️.rs:34>)).
`same_document_codec` intentionally includes the field, so registration must
not be relaxed to conceal the divergence
([registry equality](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9642>)).

The narrow repair is `extension: P::EXTENSION` in `ArtifactCodec::of`; do not
add an `envelopeId` to `ArtifactCodec` or reinterpret this field. Pack
encoding/decoding already compares `P::envelope_id()` directly, independently
of the registry, so that change leaves every binary/text envelope identifier
unchanged. One generic law with a type whose suffix differs from its envelope
ID (GIS Map suffices) should assert all of: generated codec extension equals
the descriptor/receipt suffix; registration admits the identical codec; and
the printed Pack still carries and decodes `gis.gismap`. This is the precise
guard against collapsing the two identities again.

### Genesis is an exact durable parent, never a public empty-current route

This section supersedes the preceding historical observation that the
Directory shape/decider still rejected genesis. Current source has repaired
that predicate: [`validate_checkpoint_shape`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1437>) admits only an exact
[`ArtifactFrontier::is_genesis_for`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2577>) or exact edited frontier, and both the
in-memory fold and service decider require an empty lineage to begin with a
parentless exact genesis. The next concern is the lower backend append seam,
described separately below.

The smallest public checkpoint-current grammar is now correctly closed as:

```text
Genesis { checkpoint_id }
Active  { checkpoint_id, baseline_frontier: edited-frontier }
```

There is deliberately no `None`. A `Genesis` value identifies a real durable
parent and does not let the caller spell an empty frontier. The already-landed
Rust enum and validator are at
[`schema/🦀️.rs`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:425>), the strict TypeScript union/parser are at
[`schema/🟦️.ts`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:406>), and the JSON `oneOf` is at
[`checkpoint-publication-command-v1`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📣️checkpoint-publication-command-v1/🧬️.schema.json:46>). The neutral
current corpus already contains an explicit rejected `none` row and accepted
exact-genesis row
([`artifact-genesis-v1/📤️current.json`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️artifact-genesis-v1/📤️current.json:1>)).

The HTTP fence is correspondingly exact:
[`checkpoint_publication_current_matches`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3193>) requires the named active checkpoint to be
parentless and genesis, while replay requires the new checkpoint to parent on
the supplied genesis or active ID
([`checkpoint_publication_replay_matches`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3221>)). This is the right division: a public command can only create a
normal child of the durable genesis; the separate verified-native
[`materialize_genesis`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:22>) is the sole parentless constructor.

#### Remaining caller migration

Three Rust test command constructors still name the removed enum variant:
[`🚀️bin.rs:8626`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8626>),
[`🚀️bin.rs:13848`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:13848>), and
[`🚀️bin.rs:13875`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:13875>). Each fixture must create and durably append a real genesis first, retain its
checkpoint ID, and pass `Genesis { checkpoint_id }`; replacing it with a
synthetic `Active` is invalid because `CheckpointPublicationFrontierV1` is
intentionally edited-only.

The real process helper still emits the now-invalid wire value at
[`publishCheckpointPublicationProcessPairV1`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1388>). Its parameters must carry the checkpoint ID from the
normal host-owned creation receipt/plan and emit
`{ state: "genesis", checkpointId }`. Its nearby `none` mutation at
[`📜️script.ts:13843`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:13843>) should remain a hostile rejection, not be migrated. No ordinary
public process seed should use a fabricated checkpoint ID.

#### First normal edit versus factory-only genesis

The present public checkpoint route is already an edited-post-WAL route:
after a persisted socket/actor command it constructs `base_frontier` from the
actor snapshot and calls `materialize_checkpoint` with no operations
([`🚀️bin.rs:3572`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3572>)). With a genesis current it must therefore use
`parent=Some(genesis_id)` and an edited base; it must not route zero history
through `materialize_checkpoint`.

For a future authority that actually applies its first accepted operation
inside `CheckpointRequest`, the correct generic rule is narrower: exact zero
base is allowed only with `parent_checkpoint_id=Some(real_genesis)`, at least
one operation, and an edited first resulting frontier (sequence one). An
edited base also requires a parent but may have an empty operation list for a
post-WAL snapshot publication. Reject parentless normal candidates, zero base
with no operation, and every partial-zero frontier before codec work. The
current [`validate_request`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:358>) still treats an empty head as invalid and does not require a
parent; it is correct for the current public route but remains a generic
parentless-normal loophole until those rules are added.

GIS has one additional no-current bypass to remove:
[`GisMapApprovalCheckpointPublisherV1Impl::current_matches_base`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3429>) currently treats `None` plus exact zero as current. Under durable creation it
must return false for `None`; for a zero base it must require the active
parentless genesis checkpoint, and for an edited base require exact current
frontier equality. Its subsequent authority materialization uses the
post-WAL edited frontier, so this is a current-fence repair, not a new
zero-history materialization path.

#### Lower backend verified-append bypass — P0 before creation exposure

The service decider is not the last authority. All three backend
[`append_reserved_artifact_checkpoint`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2073>) implementations validate only public/private shape/event
equality, then persist the public event and run a projection which replaces
the active checkpoint before the private journal check:

| Backend | Transaction seam |
| --- | --- |
| SQLite | [`🪶️sqlite/🦀️.rs:2081`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2081>) |
| PostgreSQL | [`🐘️postgres/🦀️.rs:2195`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2195>) |
| Neo4j | [`🌐️neo4j/🦀️.rs:2080`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2080>) |

Their `project_verified_checkpoint` helpers only rerun
`validate_verified_checkpoint_append` (SQLite
[`🪶️sqlite/🦀️.rs:710`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:710>), PostgreSQL
[`🐘️postgres/🦀️.rs:762`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:762>), Neo4j's corresponding helper), which does not load the durable descriptor or
active lineage. A direct backend caller with a valid CAS reservation can thus
replace the active pointer with a wrong-digest checkpoint, a parentless edited
checkpoint, a second genesis, a wrong-parent child, or a non-advancing child.
The same omission survives projection rebuild because it calls `project` and
then that shape-only private helper.

Add one shared pure preflight, e.g.
`validate_verified_checkpoint_lineage(descriptor, active, event, checkpoint)`,
which first performs the existing exact event/shape test, then checks the
descriptor digest, and finally enforces:

```text
active = None  => candidate is parentless exact genesis
active = Some  => candidate.parent == active.id
                   && candidate frontier is edited
                   && frontier strictly advances active
```

It should take the existing same-ID private/public record as a separate input
as well: an exact already-published CAS reference remains the existing
idempotence return; any orphan or byte-different same-ID public/private record
is a conflict before the active pointer can move. Do not let a later unique
constraint provide this authority after public projection work has begun.

Run it inside the backend transaction *after* the existing CAS-reference
idempotence return but *before* allocating/persisting the public event or
flipping the active projection. SQLite's `Immediate` transaction already
serializes this read; PostgreSQL must retain its `cas_lock_space` then query
descriptor/current with `FOR UPDATE`; Neo4j must read the descriptor and
`ACTIVE_CHECKPOINT` edge in the existing graph transaction before deleting that
edge. Put the same helper at the start of each backend's
`ArtifactCheckpointPublished` `project` branch so rebuild observes precisely
the live rule before it changes the active pointer. Retain the private helper
for public/private equivalence, but it cannot substitute for lineage.

The new host-only `append_document_genesis` must be the only backend operation
allowed to append a parentless event. It needs the same transaction-level
preflight with **no** descriptor/current/index for a fresh scope, then the
ordered `DocumentAnnounced`, `DocumentIndexed`, and exact-genesis checkpoint
events plus the private journal/CAS ledger/creation receipt. Ordinary
`append_reserved_artifact_checkpoint` should reject `parent=None` even when
the candidate shape is exact genesis.

This requires two transition modes, not a single helper that blindly treats
`active=None` as genesis: ordinary `DirectoryService::publish_reserved…` is
backed by the public `HubDirectory` append trait and would otherwise remain an
alternate parentless route. Its transaction preflight must require an existing
active parent and an edited child. Only `append_document_genesis` selects the
`None → exact genesis` mode after its absence/creation-receipt checks. During
projection rebuild, the replayed ordered genesis event is allowed to make that
state transition; it is replaying a trusted historical special append, not
authorizing a live caller.

Minimum native laws per backend: direct `append_reserved` attempts for wrong
descriptor digest, parentless edited, duplicate genesis, wrong-parent, and
non-advancing child each leave event head, active checkpoint, private journal,
and CAS reference unchanged; two independently connected writers racing two
children of one genesis commit exactly one; rebuild rejects a journal/event
lineage violation before exposing an active checkpoint; the special creation
append commits exactly Announced → Indexed → genesis or commits none. These
laws cover the lower seam; the neutral current matrix only proves parser
closure.

### Durable creation fact port: one persisted owner, not a receipt wrapper

The proposed `Accepted → Prepared → Committed | Failed | Cancelled` record is
the right boundary, but it cannot reuse the ordinary directory-command receipt
as-is. `DirectoryService::execute_idempotent` holds only an *instance-local*
`write` mutex while it claims, decides, and appends
([`directory/🦀️.rs:2167`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2167>)). The
three receipt implementations persist a short `pending` row and then allow a
release on a pre-append error (SQLite
[`🪶️sqlite/🦀️.rs:1553`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1553>),
PostgreSQL [`🐘️postgres/🦀️.rs:1479`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1479>),
Neo4j [`🌐️neo4j/🦀️.rs:1316`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1316>)). That is correct for a
freshly recomputable directory decision, but releasing an Accepted creation
claim would allow a second native factory run after an owner crash.

Create a distinct, backend-owned `ArtifactCreationFactV1` port. It must be the
only production entry that can append the initial `DocumentAnnounced`,
`DocumentIndexed`, and genesis checkpoint triple. Do not compose three public
`DirectoryService` calls or call generic `HubDirectory::append_*` from a
retained worker: those calls have separate transactions and expose partial
creation.

The fact key should be `(actor_user_id, request_id)`, with a unique constraint
or node key in every backend. The row/node carries the immutable comparison
tuple, rather than trusting an incoming retry:

```text
actor user, exact session id, accepted authorization generation,
space id, canonical command digest, selected catalog generation/receipt digest,
server-minted document id, creation deadline, accepted-at
```

The kind/editor/descriptor selector belongs in the canonical command digest;
the minted document ID is server-written at first acceptance and is not a
client-supplied retry parameter. An equal key with any unequal immutable value
is `Conflict`; it never adopts a new session, catalog, document ID, or deadline.
An endpoint may additionally require the original live session before it
returns a fact to a caller, so a replacement session cannot use a guessed old
request ID as a cross-session receipt lookup.

`Prepared` needs more than CAS locator metadata. The existing reservation
tables retain only an ownership plan and object digests (for example
[`hub_artifact_cas_reservation`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:304>) and
[`cas_project_reserve`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:555>)); their bytes are staged later through
the independent `ArtifactChunkBlobStore::stage`
([`directory/🦀️.rs:4234`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4234>)). A process can therefore die after a
factory has returned but before a CAS manifest is durable. Neither a plan nor a
content hash lets recovery reproduce those bytes without re-running the
factory.

Persist a fact-owned, bounded, chunked pair retention alongside `Prepared`:
the canonical descriptor and checkpoint bytes; exact Pack and SPR bytes; each
length and SHA-256; the computed ownership plan; and, when acquired, the
current reservation fence fields. Use a fact-id/part/ordinal unique key, not a
generic payload-store reference. SQLite can store bounded BLOB chunks in the
same `Immediate` transaction, PostgreSQL bounded `BYTEA` chunks in the same
transaction, and Neo4j bounded byte-array chunk nodes under the fact node. The
fact becomes `Prepared` only in the transaction that has stored and re-read
all of those chunks and hashes. Afterwards any worker may restage the exact
pair into `ArtifactChunkBlobStore`; an expired CAS lease can be renewed or
re-reserved for the **same stored plan**, but no factory is invoked. Terminal
cleanup marks the fact first and deletes retained chunks/CAS leftovers only
after a fresh no-reference check. That handles an external filesystem CAS
staging crash without pretending it was part of the directory transaction.

#### Required transitions and recovery query

The port should expose closed operations, not a mutable row API:

```text
claim_or_read_creation_fact(intent, now)
prepare_creation_fact(fact_id, exact prepared bytes, now)
claim_prepared_creation_recovery(after_fact_id, limit <= 64, coordinator, now)
commit_prepared_creation_fact(fact_id, recovery_lease, now)
cancel_creation_fact(fact_id, caller binding, now)
read_creation_fact_exact(actor, request_id, command_digest)
```

`claim_or_read` verifies the current session, generation, active space, and
Author membership inside its transaction before writing `Accepted`. Equal
retries return the exact record; they do not start another owner. An Accepted
record is deliberately *not* a factory work queue: only the original retained
owner may materialize it. A recovery scan may transition an Accepted record to
`Failed(owner-deadline-expired)` after its immutable deadline, but must never
restart materialization. A new intent requires a new request ID.

`prepare` is a conditional `Accepted → Prepared` transition after the
factory's pre/post checkpoints. It rechecks the live authority transactionally
before writing any pair byte; a demotion, revocation, expiry, deletion, or
deadline win produces `Cancelled`/`Failed` with no public event and no CAS
reference. Once Prepared, recovery lists only compact identities ordered by
`(deadline_ms, fact_id)` and claims one short recovery lease at a time. It
fetches the retained chunks only after that lease succeeds. The lease prevents
two maintenance instances from staging/committing together; it is never an
authorization substitute.

`commit` must append the exact three events, private checkpoint journal, CAS
reference/ledger transfer, document/index projections, and the fact's three
event IDs/sequences in **one backend transaction**. The fact receipt is the
authoritative reconciliation witness, not a post-commit HTTP response. A
Committed retry returns that immutable triple without a factory or new append.
The current reserved-checkpoint implementations demonstrate the useful atomic
subparts—event, private journal, CAS projection, and publication receipt are
co-located (SQLite [`🪶️sqlite/🦀️.rs:2077`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2077>),
PostgreSQL [`🐘️postgres/🦀️.rs:2191`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2191>), Neo4j
[`🌐️neo4j/🦀️.rs:2076`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2076>)); the creation port must make the
fact receipt part of that same transaction rather than completing it later.

Do not classify a driver error after a commit attempt as `Failed` or
`Cancelled`. The present admin-effect implementations already use an honest
`Indeterminate` result when `commit` itself errors (SQLite
[`🪶️sqlite/🦀️.rs:647`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:647>), PostgreSQL
[`🐘️postgres/🦀️.rs:690`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:690>), Neo4j
[`🌐️neo4j/🦀️.rs:375`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:375>)). Preserve that distinction here. The
smallest model-compatible approach is a `Prepared` record with a
`commit_attempted`/reconciliation-only marker and a public `Indeterminate`
outcome; an explicit `Indeterminate` phase is clearer if the schema is allowed
to add it. Either way it is non-cancellable and non-releasable until a fresh
read proves `Committed` or a backend rollback is proven. A generic PG/Neo
driver error after the first durable statement is not rollback proof. SQLite
may classify an error as rollback-proven only after the transaction has been
explicitly rolled back successfully; an error from `commit` remains
indeterminate as well.

The bounded reconciler therefore reads the fact by its unique key first. If it
is Committed, its receipt wins even if the original HTTP response was lost. If
it remains Prepared after a fresh transaction has acquired the fact lock and
confirmed no receipt/event triple, it may re-run the *idempotent commit
transaction* from retained bytes; it must not re-run the factory. If the
database cannot yet establish either observation, leave it reconciliation-only
and return unavailable/indeterminate. A terminal cancellation/deadline failure
is permitted only while the locked fact remains Accepted/Prepared with no
commit-attempt ambiguity and no creation receipt.

#### Authority race proof must be inside storage transactions

Current session validation is a read-only socket helper: SQLite reads
`hub_auth_session` and then membership at
[`🪶️sqlite/🦀️.rs:1430`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1430>), PostgreSQL does the same at
[`🐘️postgres/🦀️.rs:1411`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1411>), and Neo4j's graph read is at
[`🌐️neo4j/🦀️.rs:1250`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1250>). These are appropriate socket
delivery fences, but they are not a commit fence. In parallel, revocation
updates sessions in independent transactions (SQLite
[`🪶️sqlite/🦀️.rs:609`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:609>), PostgreSQL
[`🐘️postgres/🦀️.rs:643`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:643>), Neo4j
[`🌐️neo4j/🦀️.rs:305`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:305>)). Two `DirectoryService` instances do not share the
service mutex.

The existing per-space CAS lock is also insufficient: PostgreSQL's
`pg_advisory_xact_lock` is taken only by CAS paths
([`🐘️postgres/🦀️.rs:516`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:516>)), and Neo4j's
`ArtifactCasSpaceBarrier` is mutated only by CAS paths
([`🌐️neo4j/🦀️.rs:173`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:173>)). Neither is acquired by session
revocation or membership projection writes. Reusing either would still permit
“last authorization read, then revoked, then genesis append.”

Add one private `lock_creation_authority_tx` and call it from every fact
transition *and* from session revoke plus membership/space role/delete writers
that can affect a creator. Its deterministic typed order must be shared:

```text
Session(session_id) → Space(space_id) → Membership(space_id, user_id)
    → CreationFact(actor_user_id, request_id)
```

For PostgreSQL, select the exact session, space, and membership rows `FOR
UPDATE` in that order (and use a fact row `FOR UPDATE`, creating it with a
unique key). The relevant writer updates then naturally conflict with those
row locks; do not rely solely on a 64-bit advisory hash. For SQLite start
`TransactionBehavior::Immediate` before any authority read and re-read every
field in that transaction; its per-instance connection mutex alone is not the
cross-service proof. For Neo4j, use stable unique authority-barrier nodes and
increment each in the same sorted order in both fact and revocation/membership
writers; a read of an `AuthSession` or `MEMBER_OF` relationship alone does not
create the required write-lock contract. After locking, validate exact session
ID/user/generation, `revoked_at IS NULL`, expiry, active non-deleted/non-archived
space, and Author role. The catalog generation remains an immutable accepted
selection: compare factory output to that pinned receipt; do not silently
reselect whatever catalog is current during recovery.

This gives the two required linearizations:

* If cancellation/revocation/demotion acquires the authority/fact locks first,
  `prepare`/`commit` sees it and writes no triple; a prepared fact becomes
  Cancelled with only later garbage collection possible.
* If commit acquires them first, it writes all three events plus Committed
  receipt atomically; a later cancel waits, reads Committed, and returns the
  same receipt. It never deletes a CAS reservation or marks a committed fact
  Cancelled.

#### Closed corpus and native proofs

Add a schema-first `artifact-creation-fact-v1` corpus with these rows: equal
claim replay; same key/different digest conflict; Accepted owner crash reaching
deadline without a factory rerun; Prepared crash/resume with byte-for-byte
identical Pack/SPR and factory count one; revoke/demote between factory and
prepare; cancellation-wins and commit-wins paused interleavings; expired CAS
reservation restaged from the prepared payload; and a commit-ack-loss row
whose reconciliation returns the one exact three-event receipt.

The native law should run each row against SQLite, PostgreSQL, and Neo4j where
available, with two independently constructed Directory services/connections.
For every rejection/cancellation assert unchanged event head, descriptor/index,
active checkpoint, private journal, CAS reference, and fact phase. The
commit-loss row must inject failure only after the backend commit attempt and
assert an honest indeterminate response followed by reconciliation—not a
second factory or a terminal Failed. Add a restart row for each backend and a
prepared-payload digest corruption row which remains fail-closed, preserving
the fact for forensic/terminal handling rather than constructing a new pair.

### Exact-zero canonical pair follow-up

The Rust canonical-pair transport is correctly repaired at its materialization
boundary. Its encoder and decoder use the same scope-bound closed predicate
`is_genesis_for || is_edited_for` ([`lag-rebootstrap/🦀️.rs:428`](</Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:428>),
[`🦀️.rs:462`](</Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:462>), and
[`🦀️.rs:620`](</Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:620>)), and its MCP receiver does so before calculating
pair length or allocating Pack/SPR vectors
([`remote/pair/🦀️.rs:955`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🦀️.rs:955>)).
`ArtifactFrontier::is_genesis_for` confines an empty head and zero chain to
the exact scope, ordinal zero, and commit zero
([`directory schema/🦀️.rs:2582`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2582>)); edited
frontiers require positive counters, a nonempty head, and a nonzero chain.

One TS mirror/oracle still rejects that valid genesis form: the document-open
plan structure validator calls `documentOpenNeutralText` on the head and then
unconditionally rejects an all-zero chain
([`Hub 📜️script.ts:4360`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4360>)). Replace that pair of checks with the
same two exact shapes. It must allow empty head/zero chain only together with
scope document, ordinal zero, and commit zero; it must retain the current
positive/nonempty/nonzero requirements for edited history. This is a concrete
cross-language denial, not a transport allocation escape. The direct search
found no other `chainHash.every(byte === 0)` branch in the worker/script
folder-mirror path.

### Lineage transaction and creation-reducer re-audit (current source)

The new shared
[`validate_published_checkpoint_lineage`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1508>) is correctly placed before the active-pointer
mutation in SQLite ([`🪶️sqlite/🦀️.rs:955`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:955>)), PostgreSQL
([`🐘️postgres/🦀️.rs:877`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:877>)), and Neo4j
([`🌐️neo4j/🦀️.rs:603`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:603>)). It binds descriptor scope and digest, count,
the exact parentless zero frontier, the descriptor bootstrap Pack digest, and
an edited child to the active parent. Rebuild replays through this same branch
before restoring the private checkpoint record in all three stores, so a
historical bad lineage rolls its rebuild transaction back rather than exposing
a pointer.

Neo4j briefly took `cas_lock_space` in that projection branch. That formed an
ABBA: a normal reserved append acquired CAS then `DirectoryCounter`
([`🌐️neo4j/🦀️.rs:2098`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2098>),
[`🦀️.rs:2134`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2134>)), while rebuild owned `DirectoryCounter` first
([`🦀️.rs:2564`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2564>)) and replay entered the projection. The current
patch removed the second CAS acquisition. That is correct: every mutating
`project` caller has already acquired `DirectoryCounter`—normal/admin event
appenders increment it before projection, the reserved append increments it
before projection, and rebuild owns it across replay. CAS-only sweeper/fence
transactions do not acquire the counter, so they can conflict/retry against a
rebuild's CAS-node truncation but no longer form a lock cycle. Preserve this
contract: do not add a per-space CAS lock in `project`; if a future projection
needs one, establish a single global-before-space order across *all* writers.
Add a paused Neo law with an append stopped after its CAS claim while rebuild
claims the counter; after release both operations must finish and rebuild must
not attempt a second CAS claim during replay.

Two correctness boundaries remain open. First,
[`MemoryArtifactProjection`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1588>) still duplicates a weaker lineage check: it omits
the descriptor bootstrap-frontier and bootstrap-Pack-digest checks. Make it
call the shared validator with its active head and count so embedded/rebuild
parity cannot drift. Second, the ordinary verified append/service decider
still permits `active=None` plus parentless genesis
([`directory/🦀️.rs:1515`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1515>),
[`🦀️.rs:1937`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1937>)). This is the explicitly tracked gap: once the dedicated
genesis appender is present, ordinary append must reject `parent=None`; the
dedicated transaction alone must prove Announced → Indexed → exact genesis,
including index binding, private journal, CAS and creation receipt atomically.

#### Creation fact reducer

The new reducer has a useful immutable `Accepted → Prepared → Committed`
shape and prevents replacement of already-prepared bytes
([`operation-v1/🦀️.rs:123`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/📚️operation-v1/🦀️.rs:123>)). The current policy is coherent if it is made explicit at the port:

* `Prepared` and client-visible `Failed` require the originally accepted exact
  actor, including session and authorization generation
  ([`🦀️.rs:129`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/📚️operation-v1/🦀️.rs:129>),
  [`🦀️.rs:134`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/📚️operation-v1/🦀️.rs:134>)). A host-supervisor deadline/recovery failure needs a distinct private
  transition, not the client failure route.
* Cancellation intentionally permits a fresh session of the same user. The
  backend must therefore reauthenticate the *current* caller/session and
  Author membership inside its fact transaction before appending `Cancelled`;
  a stale/revoked session or demoted same user must not be able to use this
  reducer's user-id equality as authority.
* Cancellation versus commit must lock the fact revision and the same
  authority rows/barriers transactionally. If cancel wins, no public triple;
  if commit wins, return the existing exact receipt and never append
  `Cancelled`.

`ArtifactCreationReceiptV1` currently proves only a contiguous sequence range
([`🦀️.rs:45`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/📚️operation-v1/🦀️.rs:45>)). That is insufficient for its stated
three-event guarantee: an unrelated dense triple could satisfy the reducer.
The durable commit port must derive—not accept—a receipt with the exact three
event IDs (and validate in the same transaction their ordered
`DocumentAnnounced`, `DocumentIndexed`, and checkpoint bodies, descriptor
digest, scope, checkpoint ID and private journal). Add hostile rows for a
contiguous wrong-kind triple, substituted event ID, index-before-announcement,
and a post-commit acknowledgement loss reconciled to the one stored receipt.
No creation persistence port is landed yet, so the reducer source proof alone
does not establish claim/retry/cancel atomicity or an indeterminate-commit
outcome.

**Current-source correction.** The creation fact port and dedicated genesis
transaction began landing while this audit was in progress: SQLite now has
`claim_artifact_creation`, `append_artifact_creation_fact`, recovery scanning,
and `append_document_genesis`
([`🪶️sqlite/🦀️.rs:1026`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1026>),
[`🦀️.rs:1077`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1077>)); the common packet validator
binds the exact prepared descriptor/pair/reservation and author/author/system
triple ([`directory/🦀️.rs:1524`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1524>)). The remaining
conclusions above apply to the three-backend completed port, not as a claim
that this newly landed SQLite source is absent.

The earlier stale-time concern is historical, not a current finding: the
SQLite transaction now samples its own `observed_now` and uses it both for
deadline comparison and `creation_authority`. The corrected current audit and
the required regression rows are recorded below.

Finally, recovery scanning deliberately returns expired Accepted/Prepared
intents ([`🪶️sqlite/🦀️.rs:1065`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1065>)). A client transition correctly cannot
turn a revoked original actor into a failure, but this means a host-supervisor
deadline/rollback-proof transition remains required to terminate an
unrecoverable Accepted fact. It must neither call the client `Failed` path nor
rerun the factory.

#### SQLite genesis transaction: corrected clock finding and native matrix

**Correction.** The stale-time defect described above has now been repaired in
SQLite. `claim_artifact_creation`, `append_artifact_creation_fact`, and
`append_document_genesis` each open `BEGIN IMMEDIATE`, read `now_ms()` inside
that transaction, reject future caller timestamps, and use that observed value
for the current Session/User/Space/Membership query
([`🪶️sqlite/🦀️.rs:1026`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1026>),
[`🦀️.rs:1052`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1052>),
[`🦀️.rs:1081`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1081>)). An old accepted timestamp can no
longer make an expired session appear live. `append_document_genesis` also
checks the exact Accepted/Prepared pair, current author authority, no public
occupancy, deletion-fence absence, exact reservation generation/epoch/expiry
and plan, then commits the dense event triple, private checkpoint journal,
CAS ledger/reference and revision-three receipt in one SQLite transaction.

The native SQLite law should use a single reusable `genesis_fixture` setup,
not raw hand-written rows:

1. `SqliteDirectory::connect(":memory:")` plus `seed()` provides the live
   `seed` Author/default Space. `AuthSessionIssue` as used by the socket tests
   at [`🪶️sqlite/🦀️.rs:3068`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:3068>) provides a real current session; construct
   `ArtifactCreationActorV1` from its record id and authorization generation.
2. Use the schema-first creation fixture for the exact accepted/prepared
   pair, but replace its actor/scope/time only before calculating its command
   digest. Claim and append the immutable `Prepared` fact through the public
   backend methods. Derive the reservation using
   `prepare_artifact_cas_ownership_v1`, then reserve it through
   `DirectoryService::reserve_artifact_cas` with the existing System actor
   helper ([`directory/🦀️.rs:2392`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2392>)). A separate service-path row can stage the same Pack/SPR with the
   existing `stage_reserved_checkpoint` test helper
   ([`directory/🦀️.rs:4359`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4359>)); the SQLite transaction law itself need not pretend that
   physical staging is database-atomic.
3. Invoke `DirectoryService::publish_document_genesis`, rather than calling
   the backend for the success/replay rows, so the test also proves exactly
   one ordered three-message broadcast. Call the backend directly only for
   fault injection/state snapshots.

For every refusing row, snapshot the fact vector, directory head, descriptor,
index, active and verified checkpoint, authority journal, CAS ledger/reference
and reservation before the call and require byte-for-byte unchanged state
after it. The closed matrix is:

* **Success/replay:** one `Accepted → Prepared → Committed` call yields three
  dense event IDs/bodies in `Announced, Indexed, CheckpointPublished` order;
  the three derived projections, private checkpoint and CAS reference exist.
  An exact retry returns `Existing`, advances neither head nor ledger and
  emits no second broadcast.
* **Live authority:** revoke the session, alter its generation, expire it,
  archive the space, and remove/demote the membership after `Prepared` but
  before genesis. Each must refuse with the Prepared fact and live reservation
  intact. This is the important current-session fence, not a stale route
  authorization test.
* **Clock:** claim with future `accepted_at`, claim at deadline, prepare at
  deadline, genesis with a future caller `now_ms`, and genesis at the current
  deadline. Each has no partial Accepted/Prepared/triple as appropriate.
* **Packet and reservation binding:** independently alter descriptor digest,
  Pack or SPR hash/length/locator, checkpoint ID/frontier, event order/body,
  actor/user/scope, index dialect/name, reservation plan/generation/write
  epoch/expiry, live deletion lease, and pre-existing descriptor/index/CAS
  reference. Each must leave the pre-call Prepared operation retriable and
  publish no event prefix.
* **Transactional fault cuts:** install one test-only SQLite `RAISE(ABORT)`
  trigger at each durable cut: descriptor projection, index projection,
  checkpoint projection, authority journal, CAS ledger/reference and
  revision-three fact insertion. Each row proves all three public events and
  all private projections rolled back together—not merely that the reply is
  an error. The existing scoped trigger pattern
  (`install_invite_projection_failure` at [`🪶️sqlite/🦀️.rs:623`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:623>)) is the right test-only
  mechanism; make trigger predicates unique to the genesis fixture.
* **Commit acknowledgement loss:** the existing
  `fail_next_append_commit_ack` hook only fires in ordinary append/admin
  paths ([`🪶️sqlite/🦀️.rs:617`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:617>),
  [`🦀️.rs:2469`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2469>)); it does **not** cover genesis today. Add a
  genesis-specific one-shot *after* `tx.commit`, return a typed
  indeterminate outcome, and prove a subsequent exact reconciliation reads
  the one stored receipt without re-executing or broadcasting.

There are two required follow-on ports, not test-only concerns. First, a raw
`tx.commit().map_err(backend)` in genesis has unavoidable commit/response
ambiguity; the core service must map it to a typed `Indeterminate` and
reconcile by `(accepted actor user, request id, command digest)` before ever
reporting failure. Second, recovery scanning already deliberately returns
expired Accepted and Prepared facts, while public `append_artifact_creation_fact`
requires a live current actor and the `Failed` reducer requires the original
actor. Add a separate host-supervisor-only terminal transition under the same
Immediate/request lock. It may write `Failed` for an expired Accepted only,
or for Prepared only after a backend-proven rollback/absence outcome; it must
never invoke the factory, accept client credentials, or convert an ambiguous
post-commit state to Failed. A fresh authenticated session of the same user
remains intentionally valid for the public `Cancelled` route.

This is not only an Accepted-fact issue: `recover_prepared` eventually enters
`GenesisPublisher::reserve`, which reauthorizes the **original** session and
generation ([`service-v1/🦀️.rs:106`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:106>),
[`🦀️.rs:141`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:141>)). A revoked or expired accepted
actor therefore cannot resume a durable Prepared pair, and its present
`terminal(Failed)` retry would fail the same backend authority check. The
private supervisor must first reconcile the exact creation key; if no receipt
exists, it needs the rollback-proof terminal transition above. It must not
present this source path as a generally resumable Prepared recovery.

#### PostgreSQL and Neo4j creation-port audit

Both newly landed backends correctly retain the immutable fact payload as
their request identity and perform the public genesis sequence in the same
database transaction: PostgreSQL takes a request advisory lock and fact row
locks ([`🐘️postgres/🦀️.rs:458`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:458>),
[`🦀️.rs:1117`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1117>)); Neo4j uses the unique mutable
`ArtifactCreationRequest` node and a unique `documentScopeKey`
([`🌐️neo4j/🦀️.rs:52`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:52>),
[`🦀️.rs:352`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:352>)). Their genesis paths validate the same prepared packet,
reservation generation/epoch/expiry/encoded plan, no active delete lease,
no public scope/CAS-reference occupancy, then write all three events, private
checkpoint, CAS ledger/reference and Committed fact before `commit`. A
projection, journal or fact-insertion error is inside the backend transaction
and should roll back the full prefix; each backend still needs the same
post-commit typed indeterminate/reconciliation outcome as SQLite.

Two concrete cross-backend repairs are required before that claim is safe:

1. **Genesis lock order is currently ABBA-prone.** PostgreSQL and Neo4j
   genesis take request → authority (including the membership row/edge) →
   CAS-space → DirectoryCounter/head
   ([`🐘️postgres/🦀️.rs:1119`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1119>),
   [`🌐️neo4j/🦀️.rs:860`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:860>)). A normal directory
   append takes Counter/head first and `SpaceArchived` then writes all Author
   memberships ([`🌐️neo4j/🦀️.rs:2615`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2615>),
   [`🌐️neo4j/🦀️.rs:600`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:600>)). Therefore genesis can hold
   membership while waiting for Counter, while archive holds Counter while
   waiting for membership. Moving Counter before CAS is not safe either:
   normal reserved checkpoint publication is CAS → Counter. Use the sole
   genesis order **request → CAS-space → Counter/head → authority locks →
   facts/reservation/triple**. No normal path takes request after Counter or
   CAS, so this preserves the existing CAS → Counter order and removes the
   membership cycle. Add a paused archive/genesis native law; no sleeps and no
   generic retry claim.
2. **Neo4j's current time is sampled too early.** It reads `now_ms()` before
   starting the transaction in claim, fact append and genesis
   ([`🌐️neo4j/🦀️.rs:781`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:781>),
   [`🦀️.rs:824`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:824>),
   [`🦀️.rs:864`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:864>)). A wait for the request or authority lock can cross
   session expiry while the old timestamp still satisfies `expiresAt > now`.
   PostgreSQL currently samples later, after request lock, but its `FOR
   UPDATE` authority reads can themselves wait across expiry. Neither backend
   should only move a clock call: split authority into (a) raw current
   User/Session/Space/Membership lock-and-read and (b) validation against a
   timestamp sampled after those locks. This is the closest equivalent to
   SQLite's post-`BEGIN IMMEDIATE` observed clock. Add an exact paused
   authority-row-holder row that crosses expiry and then releases; it must
   reject with no new fact/triple.

The shared recovery query is intentionally a page, not a lease: PostgreSQL
uses the same Accepted-deadline-or-Prepared predicate
([`🐘️postgres/🦀️.rs:1098`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1098>)); Neo4j derives the latest fact and applies the same
terminal exclusion ([`🌐️neo4j/🦀️.rs:842`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:842>)). The planned private terminal/recovery
operation must reacquire the respective request lock and re-read the complete
fact history; the scan result itself is never an execution grant. A
cross-backend native law should let two services receive the same Prepared
candidate, allow only one to obtain the request writer, and prove the loser
re-reads its terminal/receipt rather than replaying factory bytes.

#### Genesis commit acknowledgement and live-directory delivery follow-up

The later backend repair has corrected the two findings immediately above:
PostgreSQL and Neo4j now acquire **request → CAS-space → DirectoryCounter/head
→ User/Session/Space/Membership authority**, and their authority helper samples
server time only after it owns those state locks. SQLite's `BEGIN IMMEDIATE`
paths likewise sample their clock inside the transaction. The new private
`artifact_creation_terminate_uncommitted` operation takes the request writer,
returns an already terminal/Committed operation unchanged, and writes Failed
only when the exact original authority is demonstrably gone or the backend's
post-lock clock has reached the immutable deadline. This is the required
rollback-proven path; a generic backend/commit error remains an error rather
than a fabricated Failed fact.

`DirectoryService::publish_document_genesis` now also correctly treats a
post-commit acknowledgement as uncertain: it re-reads the exact fact chain,
loads the three receipt-sequenced events, reconstructs the expected Completed
fact against the immutable Prepared packet and candidate, checks the stored
fact, then fans out the verified triple while it owns `write`. That is the
right **readable-receipt** repair.

There remains one delivery P0 if either recovery read itself fails. A directory
socket performs `events_since` only while subscribing
([`🚀️bin.rs:6081`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6081>));
heartbeats are deliberately invisible to callers
([`🚀️bin.rs:5782`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5782>)),
and its normal live branch accepts a later higher event sequence without
fetching the intervening range
([`🚀️bin.rs:6128`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6128>)).
Thus an ACK-loss followed by a transient `read_artifact_creation` or
`events_since` error can leave a committed triple unannounced indefinitely to
already subscribed peers: `publish_prepared` subsequently reads Ready and
returns it without a second publication attempt.

Do not use `RebootstrapRequired` as the fallback before the receipt is read:
that control asserts a particular active checkpoint, which is not proven while
the commit remains ambiguous. The bounded ownership repair is a separate
scope-only `DirectoryResyncRequired`/equivalent private live-socket invalidator
emitted before the writer guard is released. Its Hub socket handler must
reauthorize the affected scope and close active scoped/global directory streams
with `1013 rebootstrap-required`; reconnect uses the existing subscribe-then-
durable-replay path. No timer or unbounded retry is required. The caller may
return Ready only after either (a) receipt reconstruction and exactly one local
fanout or (b) that forced-resync terminal action. A native row should inject
the post-commit ACK failure plus the first receipt/events read failure, assert
the affected subscribed peers are forced to reconnect before Ready is exposed,
then assert reconnect replays exactly the three receipt event IDs once.

#### Per-space delivery-epoch handoff review

The landed DirectoryService epoch/invalidation owner is a good bounded
replacement for an unbounded receipt-retry task: on a failed exact
reconciliation it advances the affected space fence while retaining its single
directory writer. Its two delivery proof obligations are stricter than a
snapshot plus ordinary later checks, however.

1. **A snapshot/check alone is not a send fence.** Socket delivery performs
authority and visibility awaits before its final `sender.send(...).await`
([`🚀️bin.rs:5950`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5950>)).
An invalidation can therefore occur between a last `delivery_epoch` comparison
and the actual frame write. The peer could receive a later event across the
unknown triple. Provide a DirectoryService delivery **read lease** acquired
only after authority/visibility work, rechecked against the captured epoch,
and retained through the bounded socket send. Invalidation must await the
matching write lease, advance its epoch, release it, then wake consumers. A
plain `std::sync::Mutex` getter cannot provide that linearization and should
not be held across the preceding database awaits.
2. Subscribe to invalidations before replay; compare the captured epoch again
immediately after replay and before entering the live loop. This covers an
invalidation in snapshot→subscription and subscription→replay gaps. The
invalidation broadcast is advisory only: `RecvError::Lagged` is itself a
mandatory `1013 rebootstrap-required`, because an ignored lost invalidation
would recreate the delivery hole.
3. A scoped directory socket compares its space entry. A global directory
socket must initially compare the global epoch (`delivery_epoch(None)`), hence
conservatively reconnects on any space invalidation; per-space preservation
for global streams needs a separately proven relevant-space set and is not
free. The existing removal-in-A/preserve-B rule remains about authorization
and ordinary event filtering, not about this exceptional unknown-commit
recovery fence.

The SQLite neutral rows `ack-receipt-unreadable` and
`ack-events-unreadable` should use one-shot failures *after* the real commit
ack is dropped. Each must prove: durable `Ready`, the exact three rows and
receipt IDs exist, no ordinary live event is emitted, exactly one scoped epoch
advances and one invalidation is received; a later Existing retry changes
neither epoch nor live stream; a fresh durable replay reads the exact triple.
The physical socket law additionally pauses immediately after its final epoch
comparison and before frame write, triggers invalidation, and proves either
the already leased old send completes before invalidation advances or the
socket closes before it can send any subsequent event. This test must include
invalidation-channel overflow (`Lagged`) and reconnect, rather than relying on
timeouts as evidence.

The subsequently landed `DirectoryDeliveryLeaseV1` has the correct immediate
lock order: its reader and invalidator both take the async delivery lock before
the epoch mutex, and invalidation waits only for a final bounded frame write,
not for authority or database work. One non-blocking resource follow-up remains:
the per-space epoch `BTreeMap` grows for every exceptional invalidated space.
It needs active-socket scope refcount retirement, or an explicitly conservative
global-epoch fallback, before it can claim a hard memory bound. Removing an
entry while an old scoped socket can still compare it would be unsafe.

#### SQLite genesis fixture, recovery and bounded epoch fallback review

The current SQLite law is structurally compile-sound: its child module may
access the private ancestor `DirectoryService` test seams, and
`DirectoryDeliveryLeaseV1` has no missing trait or lifetime requirement. Its
paused invalidator is deterministic: it takes the read lease, polls the writer
once to its real `RwLock::write()` pending point, drops the lease, and only then
allows invalidation. The one-shot SQLite hooks also fail the intended points:
the commit has already succeeded when `genesis_commit_ack_failure` returns
`Indeterminate`, and flags 2/3 are consumed respectively by the service's
receipt read and exact three-event read. Thus the two reconnect rows prove the
service chooses durable reconciliation or a scoped invalidation, rather than
silently rebroadcasting an uncertain triple.

The 4,096-scope fallback is safe in its current form. The tuple is
`(global, fallback, scoped)`: on the first new scope at capacity it increments
`global`, clears `scoped`, sets `fallback` to that new generation, and inserts
the triggering scope. Any old scoped snapshot whose entry was evicted now reads
the newer fallback, so it cannot regain admission; unaffected scopes remain
selective until that conservative transition. A global socket compares
`global`, so it also reconnects. Do not replace this with ordinary map eviction.
The invalidation receiver must, however, compare its own captured epoch on
**every** received invalidation, before filtering the notification's space id.
The capacity transition sends only the triggering new space id even though it
invalidates every evicted scope through `fallback`; a socket for an evicted A
that ignores a B notification would otherwise remain connected with an old A
snapshot. Add a cap-crossing socket row (A is live, 4,096 distinct other scopes
are invalidated, then B triggers fallback) requiring A to close 1013 before a
later A event can be sent. `broadcast::RecvError::Lagged` remains the same
fail-closed result.

Two qualifications remain for the SQLite law:

1. It establishes the **DirectoryService** lease, not a real socket send. The
   future local socket path must acquire the lease only after its final
   authorization/visibility reads, retain it through one bounded frame send,
   and subscribe before replay then compare again after replay. A write
   invalidation currently waits while `DirectoryService.write` is retained, so
   an unbounded channel send would turn a disconnected socket into a global
   directory-writer stall. The handler must time-bound/close that final send;
   the present fixture honestly does not prove it.
2. `GenesisFixture::create` always appends `Prepared` and reserves CAS before
   the tested service call. It therefore does **not** cover a host crash after
   Accepted but before factory/preparation. The actual retained recovery loop
   does supply a fresh maintenance `OperationContext` and therefore reaches
   `artifact_creation_terminate_uncommitted` once the immutable Accepted
   deadline expires ([`🚀️bin.rs:4702`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4702>),
   [`service-v1/🦀️.rs:99`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:99>)); it is not blocked by the
   request deadline. This is source evidence only. Add one real SQLite
   Accepted-only row: claim the immutable intent, omit Prepared/reservation,
   advance the backend-observed clock past the deadline, run `recover` using a
   fresh maintenance control, and require exactly `Accepted → Failed`, no
   factory/authority acquire, no CAS reservation, no public event, and a
   repeated recovery no-op. This closes the liveness proof without permitting a
   factory retry.

The service's competing transition behavior itself is sound: the reducer first
recognizes an already terminal operation and returns `Ok(None)`, before it
requires the caller's stale revision. Therefore a fresh same-user session can
win `Cancelled` while the original task is producing a private pair, and the
later original `Prepared` append observes `Cancelled` rather than overwriting
it or returning a spurious mutation. Conversely a commit winner makes a stale
cancel return Ready. The HTTP cancel endpoint persists that result before it
sets the retained task's cancellation flag
([`🚀️bin.rs:4910`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4910>)), which is the right order.

One small route-level idempotency gap remains: `post_space_artifact_creation`
reserves its per-key retained slot **before** calling `service.accept`
([`🚀️bin.rs:4866`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4866>)). While the first execution is running,
`reserve` rejects the same key and a duplicate create request returns 503 even
though the durable operation exists and `service.accept` would return its
Accepted/Preparing status without an execution grant. This does not re-execute
the factory, but it violates the intended same-request durable replay contract.
Make the retained owner distinguish `AlreadyRunning(key)` from capacity/closing:
the former may call `accept` and must observe `execution: None`; only the owner
that inserted Accepted may activate the task. Add a paused-first-task HTTP law
requiring the duplicate request to return the exact durable status and leave
factory invocation count one. It must not move generic acceptance ahead of
capacity reservation, because an Accepted fact whose owner then cannot reserve
a slot can only fail at deadline and can never safely rerun its factory.

**Correction before implementing that route change:** a bare
`AlreadyRunning → service.accept` branch is not sufficient. The first holder
can be between local reservation and its durable `claim_artifact_creation`.
If it later errors while a joiner calls `accept`, the joiner can win
`Accepted { execution: Some }` without owning the only retained slot, losing
the non-cloneable factory grant. The task owner needs a per-key
**admission-completion slot**, not merely a `tasks.contains_key` distinction:

1. the first reservation alone invokes `accept`;
2. a duplicate joins that slot and waits for its outcome, never calls `accept`
   itself while the result is unknown;
3. the first owner atomically transfers Accepted+the execution grant into the
   retained task before notifying joiners; an Existing result publishes only
   its durable status; and
4. a pre-accept error/drop removes the slot and wakes joiners to retry
   reservation, rather than returning a fabricated Accepted result.

The paused law must cover exactly that interleaving: pause the first call
before durable acceptance, admit the duplicate, make the first fail, then
require exactly one successor admission/one task or one explicit durable
Existing result. A second row pauses after Accepted but before activation and
requires the joiner never receive an execution grant or invoke the factory.

For the pending WebSocket integration, the only race-free handoff is:

1. subscribe to delivery invalidations, then capture the scoped (or global)
   epoch, then subscribe to live events;
2. perform durable replay, then read the epoch again before entering the live
   loop; and
3. after every live frame's authority/visibility reads, acquire the matching
   `DirectoryDeliveryLeaseV1` and hold it only through a time-bounded final
   `sender.send`.

Every invalidation notification must compare the socket's own epoch before
looking at the notification scope; `Lagged` and a mismatch close 1013. The
epoch check inside `acquire_delivery_lease` makes a select race linear: an
already completed invalidation rejects the send, while an invalidator that
arrives later waits for the bounded existing send and then forces reconnect.
Replay needs no lease across its whole database/read loop—its final direct
epoch check makes an intervening commit reconnect rather than silently enter
live delivery with a stale cursor.

#### Current bin admission/recovery and delivery-lease reread

The landed `ArtifactCreationHttpPendingV1` corrects the earlier duplicate
request race. Its disposition is still zero while the first reservation is
calling the durable accept; it becomes one only after that reservation has
either moved the non-cloneable execution into `tasks` or observed an Existing
durable operation. A joiner consequently cannot mint an execution grant. Its
notify loop registers before the second atomic read, avoiding a lost wake, and
the predecessor-failure relay is now bounded to two retries. The recovery
control also now observes the owner shutdown flag at each
`OperationContext` checkpoint, so retained recovery is cooperatively
cancelled rather than only aborted at the outer deadline.

There is one remaining **P0 close race** in this owner. `shutdown` waits for
pending reservations only through its 32-second deadline; on timeout it takes
and joins/aborts `tasks` and `recovery`, but leaves the pending reservation map
live. A request stalled in `service.accept` can then complete after the Hub has
continued teardown. `ArtifactCreationHttpReservationV1::activate` currently
removes that reservation and spawns a task without checking `state.closing`,
so it can start factory/publication work against closing or already closed
resources. Make `activate` require both the exact pending identity and
`!state.closing` under the same owner mutex. At the timeout, retire/clear the
remaining pending slots and notify their joiners; an eventual accepted grant is
dropped/recovered durably, never activated after close. Add a paused
accept-across-shutdown-deadline row that requires no new task/factory after
close, no retained registry entry, and only the durable Accepted record (or
its later supervisor terminalization).

That P0 is repaired in the current owner. `shutdown_with_deadline` now takes
the remaining reservation map after its bounded wait, cancels and CASes each
unresolved pending disposition to retired, then wakes joiners. `activate` and
`finish` compare the exact pending `Arc` under the owner mutex and require the
same state not be closing; an owner that returns after the timeout drops its
future rather than spawning. The focused paused-reservation law calls late
`activate` after a 10ms shutdown, proves its future never executes, disposition
is retired, the owner count is zero, and new admission is unavailable. This
closes the post-close factory escape. A future service-level expansion may bind
that mechanism to a real Accepted fact, but no production ownership hole
remains in the source transition.

The current socket wiring now meets the core delivery linearization:
`handle_directory_ws_v1` subscribes to invalidations before capturing its
scope/global epoch, subscribes to live events before durable replay, verifies
the epoch after replay, and compares it on every invalidation regardless of
the message scope. `send_socket_directory_message` and the scoped lag-control
sender acquire the delivery read lease only after their authority/visibility
awaits and retain it through a two-second final send. This satisfies the
4,096-entry fallback because an A socket rechecks itself even when the
broadcast's payload names B. I found no ownership or lifetime escape in that
sequence. It still needs an actual socket law that pauses after the lease is
obtained and before final send, induces the unreadable-ACK invalidation, then
proves writer progress waits for that one bounded send and the next A frame is
never emitted; separately cover post-replay invalidation and invalidation
broadcast `Lagged` closing 1013.

#### PostgreSQL and Neo4j post-lock-time reread

The current PostgreSQL and Neo4j creation paths implement the required order
in the real genesis transaction: request key, CAS-space, directory
counter/head, then the User/Session/Space/Membership authority reads. Both
authority helpers sample `observed_now` only after those raw state locks, and
their ordinary fact append paths pass that exact backend observation into the
shared transition decider. Commit acknowledgement loss remains
`Indeterminate`; it does not terminalize or rewrite the durable operation.
The private terminalizer serializes on the request key, returns an exact
already committed/terminal state, and produces Failed only from an expired or
authoritatively lost identity; a generic backend failure remains an error. I
found no additional transaction-order or post-commit outcome defect in this
revision.

The newly authored backend laws correctly cover post-commit receipt/retry,
paused lock-to-authority expiry and two-service terminalization.  The earlier
wall-clock sleep was replaced by a backend-private test observation: the law
persists a real future session expiry, pauses after request/CAS/counter locks,
then advances only the fixture sample before the authority read.  Production
passes no override and samples `SystemTime` after the actual locks.  This is
deterministic source evidence, pending Pg/Neo native execution.

#### Current Creation-to-Two-Shell Acceptance Boundary

The intended composition fixture is correctly schema-first: it requires one
same-data-root current, two Authors with explicit English/German locales,
ordinary creation, B's own paused MCP cancellation, A's Shell-owned approval
and ordinary Shell Undo, two acknowledged `UiDocumentStore` Map observations,
then a same-root restart.  The closed desired sequence is in
[`🤝️two-author-shell-v1/🔣️.json`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🤝️two-author-shell-v1/🔣️.json>).
Its ownership policy is also right: an MCP private job/undo handle must never
be injected into the browser; B cancels its *own* MCP job, while A creates,
proposes, approves, and undoes through its own Shell worker.

The current executable process implementation does **not yet** enact its
creation step.  After creating the Space and admitting B,
[`proveGisMapTwoAuthorShellProcess`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11107>) still calls
`commitCheckpointPublicationProcessMutation` and
`publishCheckpointPublicationProcessPairV1`, which directly announce a
fixture `documentId`, append a first edit, and publish its pair before either
Shell opens.  It then calls both peers with that preselected document scope.
That bypasses the normal HostOnly creation command, server-minted artifact id,
`DocumentIndexed` row, native genesis factory, and genesis checkpoint.

The smallest honest replacement is:

1. after membership is durable, start A with
   `startGisMapShellPeerV1({ scope: { spaceId }, creation: { kindId:
   "s.gis.gismap", name }, locale: "en", ... })`;
2. take the returned immutable `A.scope`—which is discovered only from the
   `artifact:artifact-*` Directory row and then requires the acknowledged
   Map probe—as the sole document scope;
3. only then start B's MCP client, both document sockets, and B's existing-map
   Shell with locale `de`; obtain the initial durable pair from the two MCP
   reads, rather than the synthetic checkpoint-publication helper; and
4. retain all existing B MCP cancellation, A Shell approval/Undo, rebootstrap,
   same-pair, and restart assertions unchanged against that discovered scope.

This is not a cosmetic refactor: the current low-level helper begins with a
standalone `announce-document` and then requests an open plan
([`📜️script.ts:1271`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1271>)).
Under the new lineage rule it is not a valid root—the first durable public
state must be the dedicated atomic
`DocumentAnnounced → DocumentIndexed → ArtifactCheckpointPublished` genesis
transaction.  Reusing it for creation would both bypass the transaction and
try to open before there is an active checkpoint.

The browser-side creation path itself has the right narrow shape but remains
unqualified: `ShellHost` converts only the closed `kindChoice,name` action,
the worker polls only the closed creation status, and Ready is accepted only
for the exact request/space/kind owner before it opens the returned document
([`ShellHost/🟦️.tsx:1182`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1182>),
[`backbone-worker.ts:3033`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3033>),
[`ShellHost/🟦️.tsx:1972`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1972>)).
`startGisMapShellPeerV1` already exercises the ordinary dialog and waits for
the actual dev-only acknowledged-store probe
([`📜️script.ts:10911`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10911>)); the process gate only needs to use that branch once durable creation is wired.

There is a current native-test compilation cleanup at the same boundary:
`CheckpointPublicationCurrentV1` is now closed to `Genesis|Active`, but three
bin test callers still construct removed `CheckpointPublicationCurrentV1::None`
([`🚀️bin.rs:9239`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9239>),
[`🚀️bin.rs:14461`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:14461>),
[`🚀️bin.rs:14488`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:14488>)).
Each test must create/persist an exact Genesis or Active predecessor; restoring
`None` would reopen the forbidden parentless publication path.

#### Genesis Fixture and Cross-Backend Clock Reread

The current PostgreSQL/Neo4j test-only clock is appropriately scoped.  The
only override is obtained inside `append_document_genesis`, after request,
CAS-space, and directory-counter locks, and is behind `#[cfg(test)]`; production
passes `None`, so `validate_artifact_creation_authority` samples `SystemTime`
only after the actual User/Session/Space/Membership lock/query.  The law first
changes the real persisted session expiry, pauses at that exact boundary, then
advances the fixture observation past it.  It therefore proves lock-crossing
expiry deterministically without granting any production caller a clock or
auth bypass.  It remains source-only until Pg/Neo native execution.

The root fixture must similarly stop using
`commitCheckpointPublicationProcessMutation` as its initial-pair setup.  Its
standalone `announce-document → open-plan → first socket mutation` sequence
is incompatible with the dedicated genesis-only boundary.  The native fixture
should construct one accepted intent, exact Prepared genesis bytes, and one
`DocumentGenesisAppendV1` triple; only after its returned receipt may it use an
ordinary socket to append the first edited frontier and then publish an Active
successor.  Required denial rows are: standalone `DocumentAnnounced` has no
open plan, a root through ordinary `append_reserved` is refused, a substituted
or expired CAS reservation produces no events, and an ACK-loss retry returns
the exact three ids without rebroadcasting.

#### Persisted-Genesis Fixture Reread

The current fixture now follows that dedicated path. `publish_fixture_genesis`
issues a real session, writes Accepted then Prepared, stages the exact pair in
the supplied CAS, and calls the dedicated three-event append
([`📇️directory/🦀️.rs:4243`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4243>)).
The apparent locator discrepancy is intentional: `checkpoint` retains CAS
manifest locators for staging/publication, while only the independently retained
`prepared.checkpoint` is rewritten to the raw `sha256/...` locators committed by
Prepared. `validate_document_genesis_append_v1` normalizes a clone for that
Prepared comparison and separately validates the manifest reservation; neither
stage nor publication receives the raw-locator clone.

The fixture now replaces the generic projection fixture's Unicode schema with
the Ready-valid ASCII identity `s.gis.gismap/1/any`, while retaining Unicode
data in the pure projection oracle. Genesis has the exact zero frontier; every
later fixture checkpoint derives its parent from the persisted receipt. The
process child rereads that lineage before building its successor.

The in-memory projection fixture now inserts a creator-owned `DocumentIndexed`
event between announcement and genesis and separately refuses a checkpoint
without a descriptor or without its index
([`📇️directory/🦀️.rs:5167`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:5167>)).
SQLite likewise loads and validates the stored index before projection. At this
reread PostgreSQL and Neo4j still lacked that validation in their checkpoint
arms ([`🐘️postgres/🦀️.rs:997`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:997>),
[`🌐️neo4j/🦀️.rs:718`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:718>));
the assigned parity repair must add index decode/`validate_checkpoint_index_v1`
before lineage validation, plus missing/mismatched-index rebuild denials. Until
then either backend can materialize an active but undiscoverable document from
a malformed history.

Filesystem fixture roots are ticket-owned and unique. The successful restart
and process-race paths drop the blob store, SQLite/service owners, and child
processes before exact-root removal, so I found no cross-platform handle escape.
The SQLite Accepted-only expiry row deliberately waits for the real persisted
30-second deadline; it is elapsed-time evidence rather than a synthetic-clock
claim and asserts no Prepared, CAS, private-journal, or public-event side effect.

#### Current Two-Shell Rewire and Credential Boundary

The earlier direct-preseed observation is superseded. Current
`proveGisMapTwoAuthorShellProcess` has A create through the ordinary Shell
dialog, adopts only its minted `artifact-*` scope, and then binds B's Shell,
both MCP readers, and both sockets to that exact scope
([`📜️script.ts:11107`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11107>)).
It requires a genesis MCP frontier before B's own MCP cancellation and A's
Shell-owned proposal/approval/Undo. No MCP private job or undo handle crosses
into the browser session. The process/browser proof remains unrun.

Relay-secret cleanup is already centralized: `startLocalBrowserRelay` copies
and immediately erases the input envelope capability, then wipes its retained
capability, secret, and proof on stop. The Shell facade uses that path; no
extra caller-side clearing is needed.

#### P0: Protected Open Still Accepts a Descriptor Without Genesis

The protected open routes have not yet adopted the new active-checkpoint
boundary. Both `issue_document_open_plan_inner` and
`document_execution_target_selection` load
`get_active_artifact_checkpoint(...).map(document_open_checkpoint)` and pass
the resulting `Option` into an otherwise valid plan/lease
([`🚀️bin.rs:2394`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2394>),
[`🚀️bin.rs:2589`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2589>)).
The OS wire model also declares `checkpoint` optional, and the current Rust
authority validation only checks it when present. Consequently an authorized
descriptor-only document—whether from a standalone announcement or malformed
legacy/event history—can obtain an open-plan receipt and execution-target
bytes despite having no committed genesis pair.

The smallest correct repair is schema-first: make the checkpoint required on
both plan and execution-target lease, require the active checkpoint before any
catalog resolution/receipt creation, and revalidate exact scope plus descriptor
digest/frontier after the existing authorization and directory-revision fence.
No `None` compatibility variant should remain. Required route laws are:

1. descriptor-only and indexed-but-no-checkpoint both deny without a plan
   receipt, socket grant, or asset bytes;
2. the exact committed genesis triple opens and preserves its zero frontier;
3. a changed/replaced active checkpoint or revoked session across the paused
   issue fence returns Stale/Denied and exposes no assets.

This is the highest-risk remaining protected endpoint before a Shell-created
GIS Map can be treated as an ordinary mounted document.

The schema conversion must also retire the misleading test-only shortcut
`issue_document_socket_grant_fixture` ([`🚀️bin.rs:2729`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2729>)).
It currently checks only descriptor presence and issues a direct document grant;
many socket tests pair it with `announce_document_for_test`. It is not compiled
into production, but retaining it for mutation/semantic socket laws would hide
the exact pre-genesis case the production repair forbids. Convert those laws to
committed-genesis plus real plan→exchange admission, or make the helper
explicitly transport-only and unable to drive document state. The already
shared neutral plan/lease fixtures carry a checkpoint and are the appropriate
positive root.

#### GIS Structural-Hash Receipt Repair (Source Review Only)

The GIS repair addresses the actual refusal mechanism without relaxing codec
identity.  `ArtifactCodec::of` derives `pack_schema_hash` solely from
`ArtifactPack::record_spec`, using zero only for an opted-out handwritten
pack ([`🏪️store/🦀️.rs:9368`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9368>),
[`🏪️store/🦀️.rs:9600`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9600>)).
Both handwritten GIS snapshot packs now supply their exact body specs—six
fields for Map and three for Terrain—rather than inheriting that zero default
([`🗺️gismap snapshot/🦀️.rs:238`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:238>),
[`🗺️gismap snapshot/🦀️.rs:289`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:289>),
[`🏔️gisterrain snapshot/🦀️.rs:204`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:204>),
[`🏔️gisterrain snapshot/🦀️.rs:248`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:248>)).

The new receipt law checks the produced codec’s nonzero hash against both the
private receipt and the direct `schema_hash(record_spec)`, while retaining
function-pointer identity and envelope-versus-extension separation
([`📇️native-codecs tests/🦀️.rs:49`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/🦀️.rs:49>)).
That is the right closure: a changed snapshot field layout changes the
registered codec fingerprint and the verified receipt rather than allowing
the earlier schema-agnostic collision. I found no further source-level
identity bypass in this delta. The asserted law is not native-qualified yet.

#### Open-Checkpoint Migration Reread

The schema-side conversion is now already present: the Rust/TypeScript plan,
lease, and Rust client authority all require a concrete checkpoint. The only
remaining optional owner is Hub-private
`DocumentOpenPlanAuthorityV1`, and both protected routes still build it from
`get_active_artifact_checkpoint(...).map(...)`
([`🚀️bin.rs:1243`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1243>),
[`🚀️bin.rs:2398`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2398>),
[`🚀️bin.rs:2589`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2589>)).
Those reads must be `Some` before catalog selection or asset loading, returning
the existing redacted `Stale` failure. This retains the route’s later exact
checkpoint reread in socket validity rather than substituting a merely
revision-based fence.

The intended option-removal must also update the explicit hostile fixture
assignments at `🚀️bin.rs:10686`, `10724`, `11246`, and `11325`: retain the
negative cases as malformed required checkpoint values, not a restored absent
variant. Until this Hub-private conversion lands, the workspace is deliberately
mid-migration and source compilation is not evidence of closed admission.

#### CAS Race-Test Sweep Bounds and Independent Admission Reread

The two CAS-race laws now use the full request cap
`ARTIFACT_CAS_SWEEP_OBJECT_MAX`, so the established genesis objects cannot
starve the deliberately stale orphan before the delete barrier. The production
sweep still bounds every request and retains its delete fence across the
external conditional deletion; a successor reservation advances the shared
physical epoch, so the old delete must receive the stale-fence result
([`📇️directory/🦀️.rs:2535`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2535>),
[`📇️directory/🦀️.rs:4898`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4898>)).

There are two test-harness boundedness gaps, not a production CAS defect:

1. The two-service law bounds waiting for barrier entry, but after
   `release.notify_one()` awaits `sweep_task` without a deadline and only
   asserts `is_err`. Wrap that join in the same finite test deadline and
   require the precise stale-delete-fence error. A failure before release also
   leaves the spawned task blocked until test-runtime teardown; an owned
   cleanup guard should notify release and await/abort+join exactly once on
   all assertion exits.
2. The process law has bounded successful `wait_with_output` calls, but its
   readiness poll is a fixed count of delayed turns rather than one owning
   timeout future. It does use `kill_on_drop(true)`, so an assertion failure
   does not leave a live child by design; nevertheless, model the old child
   plus release file as one cleanup owner and perform a bounded final reap on
   every branch. This makes root deletion a verified postcondition rather than
   relying on test-process teardown.

No separate runtime execution admission bypass was found outside Home’s
in-flight open-plan/target routes. Lag rebootstrap requires descriptor,
active public checkpoint, and exact private checkpoint before exposing a pair
([`🛰️lag-rebootstrap/🦀️.rs:319`](</Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:319>));
GIS inference enters through that rebootstrap base before frozen identity or
ledger acceptance ([`💡inference runtime/🦀️.rs:3296`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3296>)).
The scoped Directory socket can still be minted for a descriptor-only scope,
but it is an event/rebootstrap stream only—not an execution asset or document
mutation grant—and rebootstrap itself rejects the missing active pair. It
should remain explicitly covered as a stream-only route, rather than being
silently treated as evidence of document execution readiness.

#### Genesis Attach, First Edit, and Shell Undo Reread

The current normal path has no zero-history contradiction. The dedicated
creation transaction persists an exact scope-bound zero frontier with its
descriptor/index/checkpoint triple. The document-open parser admits only that
exact genesis shape or an edited positive-head shape
([`directory schema TS:1251`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1251>),
[`directory schema Rust:2577`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2577>)).
The worker compares the full checkpoint frontier from the authenticated lease
to the received bootstrap before installing a cold pair, then retains that
exact pair as the actor’s current state
([`backbone-worker.ts:650`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:650>),
[`backbone-worker.ts:2671`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2671>)).

Hub inference independently reads the verified active pair rather than any
client pack, preserving that same genesis frontier in `InferenceMapBaseV1`
([`inference runtime:3177`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3177>)).
The retained committer requires the first post-WAL actor snapshot to advance
both ordinal and commit sequence by exactly one and to name the server-stamped
mutation as its head. Publication then uses the existing genesis checkpoint as
its parent and publishes the strictly edited child
([`inference runtime:1774`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1774>),
[`directory service:1935`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1935>)).
The public checkpoint-command grammar intentionally rejects a zero candidate
frontier: its `Current::Genesis { checkpoint_id }` is the durable parent
proof, while its candidate is the first real edit
([`directory schema Rust:390`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:390>)).

Shell Undo also cannot apply an inverse to the old genesis pair. It retains
the server-minted private handle, goes unavailable while a rebootstrap is in
flight, and becomes available only when the new mounted verified pair equals
the approval receipt’s exact after-frontier and the lease identity remains
unchanged ([`backbone-worker.ts:3894`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3894>),
[`backbone-worker.ts:4297`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4297>)).
The Hub route repeats the exact-current check before rebuilding and committing
the server-owned inverse ([`inference runtime:3463`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3463>)).

One residual source drift must be removed. The GIS-specific publisher still
accepts a missing active directory checkpoint when the submitted base happens
to be the zero sentinel, and its current native vector asserts that behavior
([`Hub bin:3464`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3464>),
[`Hub bin:9724`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9724>)).
That is incompatible with the dedicated-genesis rule: an ordinary child must
have an active committed parent, and the backend correctly rejects a missing
parent ([`directory service:1935`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1935>)).
It is not the normal creation journey—`map_base` already requires the active
pair—but after an active-lineage loss between base read and publish it can let
the retained approval reach its post-WAL publisher before failing as a
parentless ordinary child. Make `None` a conflict in
`current_matches_base`, replace the positive missing-current vector with a
refusal, and add one integration law that removes/revokes the active lineage
after base capture and proves no root candidate/publication is attempted.

The former open-plan P0 paragraphs above describe the pre-migration frontier.
Current Hub routes now require and scope/digest/frontier-check the active
checkpoint before plan or target selection at
[`Hub bin:2394`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2394>)
and [`Hub bin:2600`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2600>).
That correction is source-qualified only; parent reports the associated source
gate GREEN and no native qualification yet.

#### Required-Checkpoint Hub Route and Consumption Reread

The current protected cores are correctly ordered.  Both plan issuance and
direct execution-target selection authenticate/revalidate first, read the
durable descriptor, then require an active checkpoint before touching the
catalog or selected bytes.  The plan path calls
`get_active_artifact_checkpoint` before `resolve_document_open`
([`🚀️bin.rs:2390`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2390>),
[`🚀️bin.rs:2406`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2406>));
the target path does the analogous read before
`assets_for_current_selection`
([`🚀️bin.rs:2596`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2596>),
[`🚀️bin.rs:2615`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2615>)).
Both reject absent checkpoint as `NotFound`, reject a mismatched
scope/digest/frontier as `Stale`, retain the exact converted checkpoint in the
authority/lease, and repeat subject plus directory-revision revalidation after
the testable final fence ([`🚀️bin.rs:2393`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2393>),
[`🚀️bin.rs:2446`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2446>),
[`🚀️bin.rs:2652`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2652>)).

The exact checkpoint survives plan exchange.  Exchange requires the retained
authority equal byte-for-byte to the ledger record, rereads the descriptor,
re-resolves the same catalog selection, and requires the recorded directory
revision before it can consume the one-use plan
([`🚀️bin.rs:2508`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2508>),
[`🚀️bin.rs:2520`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2520>),
[`🚀️bin.rs:2538`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2538>),
[`🚀️bin.rs:1431`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1431>)).
There is deliberately no second active-checkpoint read at that intermediate
exchange.  That does not open an execution path: before consuming the resulting
socket grant, `document_plan_socket_validity` rereads and compares the full
active checkpoint to the retained authority; mismatch rejects the pending grant
([`🚀️bin.rs:3768`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3768>),
[`🚀️bin.rs:3813`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3813>),
[`🚀️bin.rs:3820`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3820>)).
Thus a checkpoint advancing in the narrow exchange window can consume the old
plan and mint only a non-consumable short-lived grant; it cannot activate a
socket, serve an execution asset, or bypass current-lineage validation.  That
is a retry/availability consequence, not an authority bypass.

The newly added `document_open_and_execution_target_refuse_descriptor_or_index_without_genesis`
law is a real-Hub-state core law: it creates an authenticated author,
descriptor, optional durable `DocumentIndexed` event, and a populated test
catalog, then invokes the same production inner plan and target selection
functions and observes `404/not-found`
([`🚀️bin.rs:11190`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:11190>)).
It is therefore not a pure DTO/authority-validator assertion and correctly
exercises the absence returned by the Directory checkpoint read.  It does not,
however, traverse Axum's route wrappers: `issue_document_open_plan` and the
four asset handlers add request-body/URI timeout and response mapping around
those cores ([`🚀️bin.rs:2471`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2471>),
[`🚀️bin.rs:2658`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2658>),
[`🚀️bin.rs:8178`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8178>)).
The adjacent positive target suite does cover those actual HTTP handlers
([`🚀️bin.rs:11002`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:11002>)),
but not this negative state.  The smallest remaining qualification law is one
raw `POST` for `/open-plan` and one for `/execution-target/manifest` in each
descriptor-only/indexed-without-genesis row, asserting `404`, an empty plan
ledger, and no response body containing selected bytes.  A test catalog left
unavailable in one row should still return `404`, pinning the required
checkpoint read ahead of catalog availability.  This is a bounded endpoint
coverage gap, not evidence of a current source bypass.

The older GIS `None`-current concern above is also now stale: current
`current_matches_base` vectors explicitly require a missing Directory
checkpoint to be rejected, including the zero-frontier variants
([`🚀️bin.rs:9724`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9724>),
[`🚀️bin.rs:9733`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9733>)).
That prior source finding should not be carried forward as an open defect.

#### Space Artifact-Creation Dialog and Ready-Owner Reread

**P0 — ordinary dialog submission loses the selected kind.** The Space
manifest declares `kindChoice`, and its handler relays that exact field in
`os.create-space-artifact`
([`Space editor:432`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:432>),
[`create-artifact:35`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:35>)).
The Shell submits that declared argument unchanged
([`ShellHost:8445`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8445>)).
But `command_from_action` reads only obsolete `kindId`, and its test supplies
that obsolete field ([`Space editor:329`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:329>),
[`Space editor:630`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:630>)).
The actual UI submission therefore makes `kind_choice` empty and opens the
dialog again, rather than creating. Use `kindChoice` only and add one law from
the declared dialog args to the exact two-field replay command.

The accepted-request Ready path is otherwise correctly fenced: it requires
the exact request/space/kind owner, current index runtime/client/session/scope,
and repeats that check after awaiting target resolution before opening a
document ([`ShellHost:1972`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1972>)).
Runtime close deletes the owner before sending cancellation
([`ShellHost:1780`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1780>)).
Thus a late Ready after a submitted request cannot mount a replacement.

**P1 — pre-submit dialog ownership is absent.** `OverlayState.dialog` retains
only `dialogId` and seed args ([`Shell state:509`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:509>));
the opening effect captures no session/runtime identity
([`ShellHost:3953`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3953>)),
and rendering/submission uses whichever session is current later
([`ShellHost:8445`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8445>)).
`SET_SESSION` does not invalidate the overlay
([`Shell state:747`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:747>)).
Consequently A’s open dialog can be submitted as B after an active
index-session/scope replacement—before the Ready owner exists. Capture
`{pluginId, instanceId, runtimeKey, clientInstanceId, scope}` or clear on any
mismatch, and gate both display and submit. Reuse the renderer target’s
existing catalog/owner rows and `SET_DIALOG` reducer row, adding A-dialog →
B-replace → submit/A-Ready assertions: no B request and no B open.

The submitted value can also survive the replacement: `UIDialog` holds staged
arguments in component-local state and the Shell supplies no key derived from
the origin ([`UIDialog:42`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx:42>),
[`ShellHost:8445`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8445>)).
When both spaces permit GIS, A’s staged choice can therefore create in B;
when B does not, the server rejects it. The fix must key/remount or clear the
staged form on origin change as well as preventing render/submit.

**P1 — transport progress/cancel is not a UI capability.** The worker owns
the full `accepted|preparing|ready|indeterminate|failed|cancelled` state and
exact cancel ([`creation schema:19`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts:19>),
[`backbone-worker:3150`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3150>)).
The Shell status branch stores neither accepted nor preparing in React state,
renders neither progress/error nor cancel, and only deletes an owner or opens
on Ready ([`ShellHost:1972`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1972>)).
Close/unmount cancellation is not a user-visible cancellation control. Retain
a presentation snapshot under the existing exact owner, render a polite status
and exact cancel action, and erase it only on its terminal/replacement fence.

Viewer mutation actions are stopped before guest dispatch
([`ShellHost:4804`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4804>)),
and only the current exact index client/scope receives selected catalog kinds
([`ShellHost:6493`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6493>));
no spectator authority bypass was found.

**P1 accessibility and locale debt.** `UIDialog` supports Escape and submit
keybindings, but its field captions are unassociated `span`s and its modal has
no dialog role, `aria-modal`, accessible name, initial focus, focus trap, or
focus restoration ([`UIDialog:35`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx:35>)).
The bounded repair is labelled controls plus modal semantics and focus lifetime
tests for submit, cancel, and origin invalidation. Space provides both English
and German labels, and unknown kind-choice locale uses the stable kind id
([`Space editor:432`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:432>),
[`ShellHelpers:2089`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2089>)).
But global locale normalization hard-defaults missing or non-German language
to English ([`ui react:4038`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:4038>)),
which violates the no-default-language requirement. Locale needs explicit
shell/brand/user choice with a neutral untranslated state rather than English
fallback.

#### Dialog Owner Lifecycle, Restoration, and Delayed-Effect Audit

The minimal existing identity sources are sufficient, but their lifetimes are
not interchangeable:

- `sessionRef` is the active UI session, but is assigned in a passive effect,
  not synchronously during reducer dispatch
  ([`ShellHost:1661`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1661>),
  [`ShellHost:3073`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3073>)).
  A session comparison must include `pluginId`, `app.id`/controller, and
  `instanceId`; instance id alone is not a cross-plugin identity.
- `openDocumentSessionsRef` holds the private mount authority. A new open
  mints a fresh `clientInstanceId`, closes the old runtime, and installs the
  replacement ([`ShellHost:4377`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4377>)).
  A document-bound dialog must therefore retain `runtimeKey`, that client id,
  and exact `{spaceId, documentId}` in addition to its session tuple. The map
  is ref-only, so a reducer-only `SET_SESSION` cleanup cannot observe an
  in-place runtime/client replacement; `openDocument`, `closeDocument`, and
  rebootstrap retirement must explicitly invalidate a matching dialog owner.
- Current-session lookup itself scans this ref by plugin plus instance
  ([`ShellHost:6481`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6481>)).
The new dialog owner must use the stronger tuple above, rather than reusing
that convenience lookup as an authority predicate.

The same tuple bug exists in the rebootstrap branch: it has the exact runtime
entry but nulls the active session when only numeric instance ids match
([`ShellHost:2120`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2120>)).
It must compare plugin/app/instance exactly before discarding a visible
session; centralizing that pure comparison with dialog ownership prevents an
A rebootstrap from retiring an unrelated B with the same local instance id.

**P0 — delayed guest effects are currently admitted under their captured
`baseSession`.** Generic `onAction` awaits `handleAction` then calls
`applyHostEffects` with the captured target session, without testing that it
is still current ([`ShellHost:4799`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4799>)).
`applyHostEffects` immediately installs `openDialog` from that stale base
([`ShellHost:3919`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3919>),
[`ShellHost:3953`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3953>)).
Its final view-state commit also compares only `instanceId`
([`ShellHost:4234`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4234>));
an equal numeric instance owned by another plugin/app can accept old state.
This is broader than dialog rendering. The active-session dialog branch and
the final primary-session write need strict session-tuple revalidation.
Do not blindly discard every stale-base effect: spawned extension effects have
their own primary-or-still-listed-spawned liveness rule
([`ShellHost:4167`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4167>)).
The existing refresh path is separately generation-fenced before it applies
pending effects ([`ShellHost:3579`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3579>)); the direct action reply is the uncovered route.

For the dialog submit specifically, display-time ownership alone is
insufficient: after user submission, the guest action may await while its
origin mount is replaced, then emit `os.create-space-artifact`. Pass an
immutable private submit owner through the internal action/effect path; verify
it before `handleAction`, after its reply and before every host effect, and
again at the creation replay branch.  Never use a global current session as a
substitute for that captured owner.  The worker/Hub still reauthorize users;
the owner is solely Shell routing lifetime.

Owner operations must be compare-and-clear. A stale Cancel/Submit handler
cannot unconditionally dispatch `SET_DIALOG(null)`, because a newer dialog
may already occupy the singleton overlay. Add a conditional clear action or
generation token. Render `UIDialog` with an origin-derived key so it resets
staged values exactly when its valid origin changes; this also closes the
cross-scope staged-value transfer noted above.

Tutorial restoration is a separate required decision. The reducer currently
turns `TutorialUiSnapshot.openDialogId` into an unowned dialog
([`Shell state:952`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:952>)).
It must not bind a restored document dialog to whatever mount happens to be
active later. Either record and validate a non-persisted current owner at the
restore boundary, or fail closed by declining document-bound dialog restore;
silently attaching it to the current session recreates the same scope bug.

The smallest deterministic corpus extends the existing
`@semio-tech/framework-renderer-react:test` rows in
[`renderer React index.test.ts`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts):
the selected-catalog and ready-owner examples already provide English/German,
A/B scopes, canonical choice, and Ready tuple; the reducer test already
covers `SET_DIALOG`. Add delayed A open-dialog after B session, A submit then
B replacement before reply, A runtime client replacement with unchanged
session, stale cancel after B dialog, and tutorial restore without a valid
mount. No browser or native claim follows from this controlled renderer suite.

#### Ordinary Artifact-Creation Progress, Cancellation, and Accessible Presentation

**Current bounded gap.** The worker is already the retained transport owner:
it retains an exact request, abort controller, worker epoch, deadline, cancel
latches, and its latest sealed status
([`backbone-worker.ts:2997`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2997>),
[`backbone-worker.ts:3109`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3109>)).
It emits the closed `accepted|preparing|ready|indeterminate|failed|cancelled`
wire state ([`creation schema:19`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts:19>)),
and its cancel request is exact `{requestId, spaceId}`
([`backbone-worker.ts:3175`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3175>)).
The Shell retains only the private owner map and catalog state
([`ShellHost:1778`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1778>));
its status handler deletes terminal owners or begins Ready opening without a
presentation state ([`ShellHost:1991`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1991>)).
Consequently an ordinary author sees no creation progress, no terminal reason,
and no user Cancel affordance. This remains a source finding only.

The smallest coherent insertion is a bounded request-id keyed Shell
presentation map, with each record retaining the *existing* immutable
`SpaceArtifactCreationOwnerV1`, the last worker phase, and a local
`cancelRequested` latch. Insert `accepted` synchronously with the existing
owner at action admission; then reduce a worker status only after the current
owner predicate and index-runtime/client/session/scope reread that the Ready
branch already performs ([`ShellHost:1204`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1204>),
[`ShellHost:1993`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1993>)).
At most eight records can exist because the worker has that capacity
([`backbone-worker.ts:2997`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2997>)); no unbounded Shell list is needed.

**Cancellation ownership rule.** A user click must *not* call
`cancelSpaceArtifactCreationsForRuntime`: that lifecycle helper intentionally
deletes the owner before posting cancellation
([`ShellHost:1784`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1784>)),
which is correct only when the runtime is being destroyed. The user route must
retain owner and presentation, atomically mark the local latch, send one
exact cancel, and leave the worker/Hub terminal result authoritative. This
prevents a late `ready` from being silently ignored after a cancellation race:
if Hub reports Ready, the already-fenced normal opening remains the durable
result; if Hub reports cancelled/failed/indeterminate, clear exactly that
record. Runtime close, rebootstrap, revocation, and unmount may retain their
existing delete-then-cancel behavior because their UI and mount are gone.

**Accessible reusable pattern.** Do not overload administration state. A
small `SpaceArtifactCreationStatusNotice` can use the existing retained
operation conventions:

- `BootstrapStatusNotice` supplies a polite labelled `role=status`, a real
  `<progress>` only when the protocol has counters, and an ordinary button
  ([`host-bootstrap:85`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx:85>)).
  Creation presently has phase-only receipts, so it must announce phase text,
  not invent a numeric progress value.
- `InferencePortPanel` supplies the closer model: a labelled section,
  focus to its heading while present and restoration on close, polite work
  updates, assertive terminal outcomes, and a disabled-after-click cancel
  control ([`host-bootstrap:199`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx:199>)).
- `SpaceAdministrationPane` is the directly registered EN/DE status,
  labelled-control, focus-restoration precedent
  ([`SpaceAdministration:217`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx:217>),
  [`space-administration.test.tsx:102`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🏛️space-administration.test.tsx:102>)).

Use explicit EN and DE phase/control strings and fail closed to a
`locale-missing` alert for another locale, as the existing directory bootstrap
notice does ([`directory-bootstrap:271`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx:271>)).
Do not propagate the global `en` fallback into this new authority-adjacent UI
surface. The existing generic `UIDialog` semantics/locale debt remains a
separate framework repair; current dialog-origin fencing is now owned by the
root lane and should not be duplicated here.

**Catalog availability is a distinct visible state.**
`openSpaceArtifactCreationCatalog` silently catches capacity, network,
non-OK, decode, and stale-owner failures
([`backbone-worker.ts:3024`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3024>)).
The Shell consequently gives an author an empty disabled kind choice with no
distinction between loading, unavailable catalog, and an intentionally empty
selection ([`ShellHost:6533`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6533>)).
Carry a bounded local `loading|ready|unavailable` catalog presentation keyed
by exact `{spaceId, clientInstanceId, runtimeKey}`. It is not a permission
decision: the server remains authoritative. It gives a signed-in author an
accessible reason/retry state while keeping a missing selected-current catalog
fail-closed.

**Minimum registered law set.** Extend the existing renderer target
`@semio-tech/framework-renderer-react:test`, whose
[`index.test.ts`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts)
already has exact catalog, request, owner, and Ready vectors at lines 52–168,
and reuse worker owner laws at
[`backbone-worker.ts:4781`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4781>):

1. exact A owner accepts accepted/preparing; a mismatched request, space,
   kind, client, session, or index scope cannot change A’s visible row;
2. one Cancel click posts one exact request, disables its button, retains the
   row, and no post-cancel Ready is opened when the owner has been invalidated;
   a non-cancel terminal removes only that request, preserving a concurrent B
   row;
3. terminal cancelled/failed/indeterminate yields an assertive EN and DE
   announcement and restores focus; accepted/preparing remain polite; no fake
   numeric `<progress>` is rendered;
4. catalog `loading`, `unavailable`, and exact selected-current `ready` are
   visibly distinct; a cross-client or cross-space catalog response cannot
   enable the dialog; and an unsupported locale renders the explicit
   locale-missing state rather than English.

These are controlled renderer/worker laws only. They do not qualify the
Hub-backed native catalog, actual Shell browser creation, or process journey.

#### Dialog-Origin Lifecycle Re-Audit (Implementation In Progress)

The current root-origin implementation is an in-progress source fence, not a
completed lifecycle qualification. Its contract now carries exact plugin, app,
controller, numeric session, runtime, client, and optional scope identity
([`dialog-origin:4`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🟦️.ts:4>)).
It correctly rejects changed client/runtime/scope tuples, uses an opening id
for compare-and-close, remounts `UIDialog` by that id, and now closes a dialog
when its exact mount is removed
([`ShellHost:1840`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1840>),
[`ShellHost:4481`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4481>),
[`ShellHost:8491`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8491>)).
The observed direct `applyHostEffects` callers now all supply the fourth
provenance parameter: refresh, recursive file/delayed/media dispatch,
extension callback, URI route, utility/tool/general action, and command
([`ShellHost:3606`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3606>),
[`ShellHost:4045`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4045>),
[`ShellHost:4363`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4363>),
[`ShellHost:4859`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4859>),
[`ShellHost:7034`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:7034>)).

**P0 — presentation origin does not establish spawned-effect source
liveness.** `onAction` captures `actionOrigin` from the visible primary
session *before* resolving a different spawned `targetSession`
([`ShellHost:4599`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4599>),
[`ShellHost:4768`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4768>)).
After B has been removed from A’s spawned panel, B’s delayed `handleAction`
reply remains admitted by the still-current A origin. `applyHostEffects` only
tests that presentation origin ([`ShellHost:3942`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3942>)); it can therefore still apply B’s non-dialog effects such as navigation,
panel/utility changes, external opening, or a follow-on spawn. Recursive
`requestFileOpen`, delayed `dispatchAction`, and media dispatch use the same
primary-only predicate ([`ShellHost:4045`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4045>),
[`ShellHelpers:490`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:490>)).
`invokeExtension` already has a separate partial source check for primary or
still-listed spawned source, which confirms the distinction
([`ShellHost:4194`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4194>)).

Retain two predicates rather than overloading dialog identity: (1) existing
exact presentation/mount origin; (2) a source owner captured for the actual
handler target. A source is current only if it is the exact active
plugin/app/controller/instance, or an exact current panel entry
`{pluginId, appId, instanceId}`. Recheck both before issuing an async handler,
after it resolves, before each recursively submitted action, and before every
effect. Do not rely on plugin id alone: two instances of the same plugin are
distinct. The minimal controlled row starts a B action under primary A,
removes B from the panel before resolving a reply containing `navigate` or
`setPanel`, and asserts no navigation, no A view-state update, no refresh and
no dialog.

**P1 — tutorial dialog refusal remains session-rebindable.** The bridge
correctly declines a dialog restoration while the current session has a
document mount ([`ShellHost:4965`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4965>)).
However a tutorial records only `openDialogId`; it does not retain the tutorial
source session or mount. `applyTutorialUiSnapshotToShell` and its delta helper
ask the *current* bridge to restore that id
([`ShellHelpers:2223`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2223>),
[`ShellHelpers:2288`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2288>)).
If a tutorial begun in documentless A survives a switch to documentless B and
both apps declare the same dialog id, B can receive the dialog on the next
tick/seek. The tutorial lifecycle effect explicitly does nothing when the
tutorial id is unchanged across the new session
([`ShellHost:5009`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5009>)).

Capture one immutable tutorial owner at start (the exact session and, where a
document is involved, exact mount), and stop/refuse tutorial UI and document
application when it is no longer current; a snapshot cannot rebind it. Test
A documentless tutorial with dialog `edit` → B documentless same-id session
before a director tick and before a seek: neither path may install B’s dialog
or apply B document effects. The existing controlled tutorial bridge tests are
the right target; this is not a browser/native claim.

**P0 — current stop can restore A’s snapshot into replacement B.** Sandbox
start stores only raw pack/SPR in `tutorialDocumentSnapshotRef` after reading
the current plugin. Stop then derives a plugin afresh from whichever `session`
is current and loads those raw bytes into it
([`ShellHost:5006`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5006>),
[`ShellHost:5034`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5034>)).
After A’s `readAppDocumentPack` settles, a switch to B followed by Stop can
therefore invoke B’s `loadAppDocumentPack` using A’s bytes. The present
`previousId === activeTutorialId` early return also fails to notice the
same-id session replacement ([`ShellHost:5009`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5009>)).

The smallest safe owner is an ephemeral monotonic `TutorialRunV1`, captured
before `SET_TUTORIAL`: `{epoch, tutorialId, exactOrigin, sourceSession,
sourcePlugin, snapshot?}`. Its owner test is the full dialog-origin equality,
not tutorial id nor numeric instance. A run is invalidated explicitly by the
exact mount-close/rebootstrap path as well as by session replacement; stopping
the clock and clearing the visible tutorial state must happen once per epoch.
Do not clear the run object before guarded cleanup completes, because an old
async finalizer must not affect a new run of the same tutorial id.

The required guards are bounded and specific:

1. **Start/sandbox:** after `readAppDocumentPack`, before storing the
   snapshot, before base UI/camera/seek, and after `refreshUi`, require the
   exact run. On failure zero/drop any captured byte arrays and do not apply a
   base snapshot to the replacement.
2. **Tick and recursive slice:** the clock subscription, each UI change,
   every document-operation continuation, and the `finally` which clears
   `tutorialDrivenRef` must require the same epoch. `onActionRef` history
   calls occur only while the owner is still current; an old slice must not
   capture B as a new action origin.
3. **Seek and converge:** recheck after each awaited mutation/refresh and on
   every rAF tween before setting cameras or re-enabling playback. This keeps
   an A rAF from moving B’s camera after a switch.
4. **Stop:** restore only through the retained A plugin/session *and only if
   A’s exact owner is still current*. If it is not, zero/drop A’s snapshot;
   never rediscover B’s plugin and never write A’s snapshot anywhere. The
   source itself may be retired, so dropping is the correct fail-closed
   outcome rather than attempting a hidden restore.

`TutorialUiBridgeContext` should be constructed for this run, rather than
using the render-updated current-session ref. Its `restoreDialog` continues to
refuse document-bound tutorial dialogs and additionally requires the run
owner. A language-neutral controlled corpus should cover: (a) A snapshot
paused before completion then B switch; (b) A snapshot complete then B switch
then Stop, asserting B never receives `loadAppDocumentPack`; (c) A same-id
restart while old cleanup is paused, asserting old finalization cannot stop or
clear the new epoch; (d) document-mount close during tick/seek, asserting no
UI/dialog/camera/document continuation. These are lifecycle source laws only.

#### Effect-Source Propagation Re-Audit (Current Source)

The newly added owner has the right two independent predicates: presentation
must retain its exact origin, while the producer must be the current primary
or an exact `{pluginId, appId, instanceId}` spawned member
([`dialog-origin:71`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🟦️.ts:71>)).
`captureEffectOwner` also pins the exact loaded plugin-handle identity, and
the top of `applyHostEffects` now tests that composite owner for every
top-level returned effect
([`ShellHost:1851`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1851>),
[`ShellHost:3967`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3967>)).
All direct callers observed in the current source provide the fourth owner
parameter, including refresh, recursive file/delayed/media dispatch, URI,
utility/tool/general action and command. This is a source audit only; the
focused helper test does not exercise the full host-effect lifecycle.

**Resolved in current source, still source-only — same-plugin spawned effects
are classified by full session identity.** The final branch now uses
`!shellDialogSessionIsCurrentV1(baseSession, currentPrimary)`, rather than a
plugin-id inequality, before taking the spawned-state/refresh path
([`ShellHost:4306`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4306>)). The required controlled row remains a primary and spawned
instance sharing `pluginId` but differing app/instance: it must update only
the spawned panel and use `refreshSpawnedUi`. No browser/runtime acceptance is
implied by the focused source result.

**P0 — artifact opening needs admission *before* its intentional primary
handoff, not an old-source check after it.** `openArtifactWithAppRef` creates
the target app, synchronously dispatches the newly created B primary session,
and returns B as the explicit `DocumentOpeningTarget`
([`ShellHost:5831`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5831>),
[`opening:3`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🟦️.ts:3>)). A naïve
`isCurrentEffectOwner(A)` check after that await would falsely reject the
valid A→B transition and strand B before `openDocument` attaches its explicit
target. Instead, give `openArtifactWithAppRef` an optional admission callback
(defaulting to true for ordinary Open-with) and check it before install, after
install, and after `createApp`. The relay supplies its exact effect owner; the
creation Ready route supplies its existing request/index/runtime owner
predicate. If admission becomes false after `createApp`, destroy that exact
instance and return `null`; only a still-admitted call may dispatch B and
return its target. The command notification belongs after the post-install
check.

The controlled host lifecycle rows must pause: (1) install, then retire A and
prove no B creation/session/document; (2) `createApp`, then retire A and prove
exact B destruction with no session/document; and (3) the creation Ready
route, then replace its index/runtime owner and prove the same cleanup. A
normal A→B row must prove the explicit B target remains attachable. The pure
[`opening.test.ts`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🚪️opening.test.ts>) target-selection tests do not
exercise this async ownership boundary.

`ensureSpawnedPlugin` is separate: it now accepts an exact-current callback
and destroys a just-created instance on either post-await failure
([`ShellHost:3945`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3945>)). This is source-reviewed repair evidence only; its focused suite does not qualify browser/runtime behavior.

#### Current Creation Root-to-Mount Preconditions

The ordinary browser route is structurally coherent once it has an actual
fresh Space host artifact: the Space editor emits only the exact
`{kindChoice,name}` relay and does not mint a client document
([`create-artifact:22`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:22>));
the Shell accepts it only from one exact mounted Space-index runtime
([`ShellHost:4130`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4130>));
and a Ready receipt is rechecked against that same index owner both before
and after resolving the selected open target
([`ShellHost:2010`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2010>)).
`openDocument` then waits for the socket actor before directory/catalog use
([`ShellHost:4470`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4470>)).

**P0 preflight blocker — the composition runner serves a stale Space
descriptor/component.** `--two-author-shell` builds Hub and MCP only
([`Hub script:12088`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12088>))
and each Shell dev server sets `SKIP_PLUGIN_BUILD=1`
([`Hub script:10990`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10990>)).
That dev pathway serves the component/descriptor previously staged by the
plugin builder, whose staging copies the plugin-root `🔣️.json` verbatim
([`dev script:232`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:232>)).
The current checked-in Space descriptor still declares `createArtifact` field
`kindId` ([`Space descriptor:3500`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🔣️.json:3500>)),
whereas current Rust emits `kindChoice`
([`Space editor:433`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:433>)),
and the harness deliberately selects `#kindChoice`
([`Hub script:11049`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11049>)).
Therefore the next process run needs a bounded, explicit preflight that
rebuilds/describes/stages the exact Space host component plus descriptor and
verifies the emitted descriptor contains precisely `[name,kindChoice]` before
starting either browser. It must not be inferred from the trusted GIS/stdio
current: Space is the ordinary host UI artifact and is outside that selected
two-package current. Until that preflight is native-qualified, creation and
mount are unproven.

#### Current Two-Author Shell Process Preflight

The current `--two-author-shell` route is strong at the selected GIS boundary:
it builds Hub and MCP with the test-support feature, seeds one native verified
checkpoint pair, materializes and validates one owned GIS/stdio current, runs
the retained closed-actor proof, and gives Hub that same data root
([`Hub script:12083`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12083>),
[`Hub script:12113`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12113>),
[`Hub script:12119`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12119>)). The eventual process owner is also correctly fail-closed: it checks the retained current before opening peers and after durable changes, compares each mounted probe’s GIS component,
descriptor, browser actor and generation to that selected current, observes a
real MCP-owned cancellation, drives ordinary Shell proposal/approval/Undo,
and repeats the two-peer observations after a same-root Hub restart
([`Hub script:11139`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11139>),
[`Hub script:11222`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11222>),
[`Hub script:11294`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11294>)). Those are source-path facts; the process has not been accepted here.

**P0 preflight blocker — browser host bytes are neither ticket-owned nor
bound to the prepared current.** Each peer invokes `bun … dev` with
`SKIP_PLUGIN_BUILD=1`, but React `ServeScript` accepts the shared development
activation receipt and Vite reads the fixed repository browser-module root;
neither uses `prepared.artifactRoot` or the selected current
([`Hub script:10943`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10943>),
[`dev script:1293`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1293>),
[`Vite config:25`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts:25>),
[`Vite config:120`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts:120>)). `S_DATA_DIR` is only an injected browser environment value, not a module-root selector. The mounted GIS probe rejects a byte mismatch, so this is fail-closed rather than a false selected-GIS acceptance; nevertheless a stale Space host can run, and a passing run would not prove current-source host behavior.

The smallest coherent harness boundary is an explicitly test-only pair of
absolute, ticket-owned paths for the browser module root and activation root.
Require both below `SEMIO_TEST_ARTIFACT_DIR`, as regular non-link directories,
or retain today’s normal defaults; never silently fall back. Build/materialize
the exact `s` host and support/shard bytes into
`<artifactRoot>/browser-host/modules`, activate that exact staged session into
`<artifactRoot>/browser-host/activation`, and verify a closed receipt before
each peer starts. The receipt should bind the `s` component and descriptor
digests, complete activation-receipt digest, selected GIS current SHA and
generation. Pass both roots to Vite only for this test harness and record that
receipt in the final process observation. A substituted host module,
descriptor, activation receipt, or a missing override must refuse before
Chromium starts. Existing `collabPrebuildPlugins` proves the necessary
component-presence shape but writes the same shared global root, so it cannot
be used directly for a concurrent ticket-owned process law
([`dev script:2415`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2415>)).

#### Tutorial Invalidation Re-Audit

`OwnedTutorialRunV1.stop()` closes synchronously before returning its restore
promise, so document close’s post-removal stop intentionally discards rather
than restores a snapshot into a closed/replaced mount
([`tutorial run:31`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🎥️tutorial/🟦️.ts:31>),
[`ShellHost:4547`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4547>)). That part does not admit a late document write.

**P1 — close/replace can strand the global tutorial-driven suppression bit.**
Seek sets `tutorialDrivenRef` then returns early if its run is retired, without
clearing it; the convergence animation does the same on an invalid run
([`ShellHost:5188`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5188>),
[`ShellHost:5195`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5195>),
[`ShellHost:5253`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5253>)). That leaves later ordinary actions invisible to both deviation auto-pause and recording. Do not add indiscriminate `finally { false }`: director, seek, convergence and start can overlap. Give each driven async operation a monotonic token tied to its exact run and transition epoch; completion or early return clears only its own still-current token, while close/stop/start invalidates the epoch/token synchronously. Also make the missing-definition branch stop the run before clearing UI state (`ShellHost:5073`). Add paused-seek and paused-convergence close/replacement rows, then a normal user action proving recording/deviation observation was restored.

#### Tutorial Drive Token Re-Audit (Current Source)

**Resolved source-only:** `TutorialDriveV1` now holds a monotonic active token
instead of the shared suppression boolean
([`tutorial:4`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🎥️tutorial/🟦️.ts:4>)). Stale completion can release only its own still-active token;
`retire()` invalidates all old work. Initialization, director slices, seek, and
convergence test exact run/readiness/token before post-await UI or document
work; close, stop, unmount, and successor start retire synchronously
([`ShellHost:5068`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5068>),
[`ShellHost:5113`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5113>),
[`ShellHost:5205`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5205>),
[`ShellHost:5262`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5262>)). The prior restore rejection is now caught/logged before successor
admission ([`ShellHost:5303`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5303>)). I found no stale-release or stale-run write bypass.

**P1 — a live seek can duplicate its first mutation.** Seek claims the token,
composes UI, and awaits its first `applyMutations` while leaving both the
clock playing and `tutorialLastAppliedMsRef` at the old frontier
([`ShellHost:5196`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5196>),
[`ShellHost:5210`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5210>)). A 90-ms director turn consequently builds the same
`oldFrontier→liveTime` slice, claims a newer token, and submits the same
mutation before token invalidation can stop the paused seek
([`ShellHost:5163`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5163>),
[`ShellHost:5174`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5174>)). This is a real duplicate mutation, not merely a stale UI completion.

The smallest repair is to capture `wasPlaying` and pause the clock before
claiming seek ownership; after exact seek mutations, `lastApplied`, clock seek,
refresh, and a final exact token/run check, resume only when `wasPlaying`.
Test with deferred mutation M: begin live playback at applied time zero, seek
across M, advance clock past the heavy director threshold while M is paused,
and assert exactly one M submission plus no competing frontier change; release
M, then assert selected playhead/camera and a single resume. This belongs beside
the neutral drive corpus and a controlled renderer law; the current three
token rows do not exercise it.

#### UIDialog Accessible-Modal Reuse Packet

**P0 accessibility gap:** `UIDialog` hand-rolls a sibling veil and glass
surface, without `role=dialog`, `aria-modal`, title/description associations,
focus initialisation/trap/return, scroll isolation, or topmost nested-dialog
dismissal ([`UIDialog:35`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx:35>)). Recompose it using the existing owned
`Dialog`, `DialogContent`, `DialogTitle`, optional `DialogDescription`, and
`DialogFooter`; that primitive already owns portal isolation, focus behavior,
topmost Escape/outside handling, and semantic associations
([`Dialog:310`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:310>),
[`Dialog:353`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:353>)). Keep
`OwnedShellDialog.settle` outside it: exact `openingId` close before dispatch
remains the only Shell action authority
([`OwnedShellDialog:16`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🌐️browser/🟦️.tsx:16>)).

Pitfalls:

1. Drop UIDialog's global cancel keybinding when `DialogContent` owns Escape;
   two document listeners otherwise call `onCancel` twice for standalone
   callers. Keep submit only as a scoped chord. Do not add an `aria-label`
   which suppresses the localized `DialogTitle` association.
2. `DialogContent` owns `data-slot="dialog-content"`; existing UIDialog CSS
   selects `data-slot="dialog-box"` ([`ui.css:7070`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css:7070>)). Update that CSS contract rather than
   trying to override the primitive's slot.
3. Current field labels are visual spans only. The real renderer returns
   controls keyed by `def.id` ([`UIDialog:62`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx:62>),
   [`ShellHelpers:2835`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2835>)); text labels alone do not name injected
   select/slider/toggle controls. Pass generated label/description ids to
   `renderField` and require its focusable control to attach
   `aria-labelledby`/`aria-describedby`; `kindChoice` must not rely on a
   placeholder as its accessible name.

Reuse the language-neutral dialog composite expectation
([`dialog expect`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🖥️composite/💭️dialog/🎯️expect.json>))
and the registered primitive matrix
([`Dialog tests`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🟦️.tsx>),
[`ui-react registry`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts:25>)). Add UIDialog rows for
localized dialog name/description and field name; first focus, Tab/Shift+Tab
containment and restoration; Escape/veil exact-once cancel; scoped submit only
when required args exist; and `openingId` replacement while old focus is active.

**P1 — a restore failure must not prevent a successor tutorial from being
admitted.** `OwnedTutorialRunV1.stop()` memoizes the restore promise, including
its rejection, while `startTutorial` awaits the old run without handling that
rejection ([`tutorial run:31`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🎥️tutorial/🟦️.ts:31>),
[`ShellHost:5291`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5291>)). The old run is already closed, so a failed restore must be logged and disowned by exact reference, not block an otherwise-current successor. Catch the old terminalization locally, clear only if the ref still denotes that old run, then perform the transition epoch/owner check and create the new run. Add a restore-reject → immediate new tutorial row; it must produce one logged old failure and a live new run, never clear a replacement.
