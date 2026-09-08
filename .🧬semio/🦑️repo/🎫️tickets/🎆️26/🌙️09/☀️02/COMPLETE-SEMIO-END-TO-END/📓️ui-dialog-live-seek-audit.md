# UIDialog And Live-Seek Audit

## Scope

Read-only audit of the current tutorial drive and UIDialog composition. No
browser or native execution was performed here.

## Resolved In Source, Unqualified: New-Director Seek Race

The initial finding was that `seekTutorial` claimed its token and began the
first asynchronous mutation without pausing playback or moving `tutorialLastAppliedMsRef`
([`ShellHost:5196`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5196>)). A 90-ms
director turn reads the same old frontier, constructs an overlapping slice and
submits the identical mutation before the seek's token invalidation can take
effect ([`ShellHost:5163`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5163>)).

The current source now introduces `runPausedTutorialSeekV1`, which pauses the
actual clock synchronously and resumes only under unchanged owner/user intent.
The authored neutral fixture covers playing, user pause and owner close. This
is source-only evidence; its pre-existing-director operation gap remains below.

## Resolved In Source, Unqualified: Shell-Scoped Portal

The new primitive composition gives UIDialog an accessible title, description,
focus containment and scoped chords. The real Shell renderer consumes the new
field binding ([`ShellHost:8626`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8626>)).

The initial version allowed `DialogContent` to create its implicit `document.body` portal
([`UIDialog:76`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx:76>)). This contradicts the per-Shell dialog
portal contract ([`ShellScope:55`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🟦️.tsx:55>)): `ShellScope.query` only walks the
Shell root ([`ShellScope:87`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🟦️.tsx:87>)), and tutorial event discovery is
already root-scoped ([`ShellHost:5143`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5143>)).
Consequently a dialog action/field was invisible to root-scoped tutorial or
automation lookup.

The current source now uses the existing `DialogPortal` with
`container={shellScope?.portalLayerRef.current}` and an explicit
`DialogOverlay` around `DialogContent`; fall back only outside a Shell. The
primitive already proves custom-container behavior
([`Dialog test:270`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🟦️.tsx:270>)). Add a ShellScope row that checks the
portal parent and that `scope.query("#kindChoice")` resolves the live trigger.
The Shell layer is deliberately `pointer-events-none`
([`ShellHost:1510`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1510>)). `DialogContent` overrides that via its glass
recipe, but `DialogOverlay` does not; add `pointer-events-auto` to the
primitive overlay. `DialogPortal` itself now supplies `pointer-events-auto`,
which covers both the veil and nested portaled controls below the Shell layer.

## Resolved In Source, Unqualified: Field Binding Consumers

The initial story renderer and a renderer origin test were three-argument functions
([`UIDialog story:20`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️.story.tsx:20>),
[`renderer test:231`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts:231>)). TypeScript permits this, so neither
exercise the new binding. The current source consumes all four arguments and
applies `field.id`, `field.labelledBy` and `field.required` to their focusable
controls. No test execution is claimed by this audit.

## Remapped Escape: Source Repair Pending Test Outcome

The initial scoped handler resolved `ui.dialog.cancel` from the configured bindings,
but the primitive's document-capture handler closes every topmost dialog on
`Escape` before the UIDialog React key handler can decide otherwise
([`UIDialog:47`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx:47>),
[`Dialog:429`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:429>)). A configured `alt+x` cancel
therefore retained a second, unconfigured Escape cancellation path.

The current source passes `onEscapeKeyDown` to `DialogContent`, prevents the
primitive dismissal when Escape is not the configured cancel chord, and adds
the remapped Escape assertion. Parent reported its expected RED is running; no
pass claim is made here.

## Resolved In Source, Unqualified: Pre-Existing Director Operation

The original risk was a director bridge call already in flight when a seek
claimed a newer token. A later seek could then infer an uncommitted source
frontier and send an inverse before the director's first mutation committed.

The current `TutorialDriveV1` has one `#running` physical batch and at most one
coalesced `#pending` successor. Director and seek both submit through
`enqueue`, and each reads `tutorialLastAppliedMsRef` inside its admitted
closure. The director publishes that cursor only after `applyTutorialSliceToShell`
returns ([`ShellHost:5164`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5164>)). Thus a started batch finishes before the next
batch derives its cursor; invalidation prevents additional old events but does
not pretend to cancel an admitted bridge call. This is the right ownership
boundary. Its drain handoff remains defective below.

The corresponding controlled row must defer director mutation `M`, enqueue a
seek to `0`, prove no inverse before `M` resolves, then prove exactly one
ordered reconciliation and final cursor `0`. An owner-close row must prove no
successor write after close and wait for the active bridge call before restore.

## Resolved In Source: Serial Drain And Direct-Claim Handoff

`TutorialDriveV1.drain()` now starts a retained successor when necessary,
awaits the observed active promise, and rereads `#running` plus `#pending`
until the lane is quiescent ([`tutorial contract:27`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🎥️tutorial/🟦️.ts:27>)).
`OwnedTutorialRunV1.stop()` therefore restores only after a delayed successor
finishes. `claim()` also refuses to replace an admitted physical document batch
([`tutorial contract:15`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🎥️tutorial/🟦️.ts:15>)).

The owning agent reports a fresh full renderer run GREEN with delayed-successor
drain, snapshot-after-drain, fault quarantine, bounded pending replacement,
requested-play intent, and rapid coalesced-seek rows. This audit did not rerun
that gate; no residual source race was found in the final lane.

## Superseded: Resume Eligibility Across Coalesced Seeks

The current policy deliberately treats `tutorialPlaying` as the authoritative,
current user request—not an observation of the physically paused clock. While a
drive is busy, the Shell effect suspends the clock; after the final admitted
operation, `runPausedTutorialSeekV1` resumes only when the exact owner still
exists and `wantsPlaying()` is currently true
([`tutorial contract:67`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🎥️tutorial/🟦️.ts:67>), while the Shell effect is
`tutorialPlaying && !drive.busy` ([`ShellHost:5017`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5017>)).

An inherited `wasPlaying`/resume-eligibility bit would be weaker: it would
discard a deliberate Play pressed while a seek is waiting. The required fixture
rows are instead initial-clock-paused with requested Play, user Pause during a
pending seek, rapid coalesced seeks, owner retirement, and lane fault. This
policy is source-only pending its expanded fixture qualification.

## Superseded: UIDialog Portal Placement Did Not Scope Isolation

The current Dialog primitive implements the recommended explicit inherited
`isolationRoot`, per-root stacks/snapshots/event admission, root-relative
absolute Dialog and Select geometry, and `aria-modal=false` for scoped mode.
The two-Shell UIDialog and nested Dialog rows now cover local inerting, B
interaction, independent Escape and Select dismissal. The owning agent reports
focused source tests GREEN; this audit did not rerun them. The historical
diagnosis below records why the now-landed policy was required.

`DialogPortal` is now correctly placed inside a Shell's portal layer and marked
`pointer-events-auto`, so its modal veil and nested Select menu can receive
input below the layer's `pointer-events-none` parent. The remaining global
behavior is in the Dialog primitive: `syncModalEnvironment` walks from the
portal to `document.body`, setting `inert` and `aria-hidden` on every sibling
at every ancestor ([`Dialog:157`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:157>)). Consequently a dialog in Shell A also
inerts a sibling Shell B, and the global `dialogStack` makes B's dialog compete
for topmost status. Merely selecting A's portal container does not establish
per-Shell modal ownership.

The intended policy must be explicit. If dialogs are page-modal, add a
two-Shell test that asserts B is intentionally blocked. If ShellScope's
no-cross-shell-collision contract applies, add an explicit `isolationRoot` to
the Dialog primitive, pass `scope.rootRef.current`, stop sibling isolation at
that root, and maintain topmost stacks per root. A two-Shell row must prove A's
inner application is inert while B remains usable. Do not infer the boundary
from a portal path; standalone dialogs still need page-level isolation.

Per-root inerting alone is not enough for that B-usability claim. Dialog's veil
and content are `position: fixed`, and SelectContent uses fixed viewport
coordinates ([`Dialog:360`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:360>),
[`Select:603`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:603>)). A portal within A's absolute layer does not
by itself constrain fixed descendants; its veil can still physically cover B.
Scoped mode must either be restricted to viewport-filling Shell roots, or use
an A-root-relative absolute coordinate space for veil, dialog and nested menu
placement (including layer-rect conversion and clamping). Test actual pointer
delivery to B, not only the absence of `inert`.

`aria-modal=true` is also document-wide semantics, not an isolation-root
semantic ([`Dialog:483`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:483>)). If B is intentionally interactive,
the scoped variant cannot tell assistive technology that the A dialog is a
global modal. The design must either retain page-modal semantics or make the
scoped variant non-`aria-modal` while preserving its local label/focus policy;
the two-Shell accessibility row must exercise the chosen policy.

The scoped isolation identity should belong to Dialog context and inherit into
nested Dialogs. A portal-only option would let a nested default portal silently
return to page-global isolation. Thread the root key through stack selection,
Escape/focus gating and nested-boundary ordering.

## Resolved During Review: UIDialog Layer Composition

The shared `GLASS_OVERLAY_BOX_CLASS` contains `z-tutorial`, while UIDialog adds
`z-dialog`. This is safe: the repository `cn` helper reverse-deduplicates the
`z` class group, retaining the last `z-dialog` token
([`class-name composition:253`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🟦️.ts:253>)). The computed class therefore
places the dialog below its owned `z-menu` Select content. The previous
stylesheet-order concern is withdrawn. `DialogPortal` now has
`pointer-events-auto`, which correctly gives the veil and nested portaled
controls an input surface inside the Shell's otherwise `pointer-events-none`
portal layer.

## Superseded: Page-Scoped Select Could Observe a Scoped Shell

The current per-root Select registry correctly isolates two non-null Shell
roots: it filters the top token by the exact `isolationRoot`, and the global
pointer, focus and Escape listeners reject targets outside that root
([`Select:121`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:121>),
[`Select:285`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:285>)).

The prior null-root hole is now closed: `isInsideSelectIsolation` delegates
page scope to `isInsideActiveScopedDialogIsolation`, so a page-level Select
rejects events originating inside an active scoped Dialog
([`Select:239`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:239>),
[`Dialog:263`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:263>)).
The owning agent reports a mixed page-Select/scoped-dialog row and focused UI,
typecheck and full UI gates GREEN. This audit reread the source but did not run
those gates.

## Partially Resolved: Unavailable Or Rotated Artifact Catalog Selection

The original raw-input failure is resolved in current source. An
`artifactKind` or `surfaceApp` is now always rendered as a `Select`, disabled
when it has no options ([`Shell helpers:2812`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2812>)), and the shared
`unresolvedActionArgs` gate refuses a missing or non-member choice
([`action argument resolution:33`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩️action-argument-resolution/🟦️.ts:33>)). This audit did not run the reported renderer qualification.

The remaining source-in-progress boundary is the Shell dispatch snapshot: the
current builder still accepts every canonical self-consistent choice rather
than an exact member of the mounted selected catalog
([`ShellHost:1180`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1180>)), and the current presentation reduction still discards
`catalogGenerationId` ([`ShellHost:6658`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6658>)). A
`ready` status is not itself evidence of catalog contents—the status wire is
separate from the catalog message ([`OS wire:948`](</Users/ueli/Documents/semio/🛍️products/💻️os/🟦️.ts:948>)). Home owns this exact selected-member gate.

The worker catalog already has an exact immutable identity:
`clientInstanceId`, `spaceId`, `catalogGenerationId` and ordered full kind
tuples ([`backbone worker:3025`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3025>),
[`OS wire:916`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:916>)).
The Shell reduces that identity to `readonly ArtifactKindChoice[] | undefined`.
For a mounted Space index, loading, unavailable, a closed index, or an owner
mismatch becomes `[]` ([`ShellHost:6658`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6658>)).

At discovery, `resolveActionArgDef` then gave `artifactKind` zero options
([`Shell helpers:2093`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2093>)).
The former generic control identified that as host-resolved `artifactKind`,
whose exhaustive fallback was a writable text input
([`manifest:519`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:519>),
[`Shell helpers:2921`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2921>)).
Any nonempty text then passed `missingRequiredArgs`; the Shell request builder
still checks only a canonical decodable choice, not membership in the selected
catalog
([`ShellHost:1180`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1180>)).

The Hub still resolves the submitted `kindId` again under its current verified
catalog, so this was never a server authority-minting bypass. The remaining
stale-selection lifecycle defect is that the dialog is keyed solely by
`openingId`, so staged input survives catalog unavailability or a
`catalogGenerationId` rotation. The global catalog notice is outside a scoped
UIDialog and is inert/hidden behind that modal
([`ShellHost:8498`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8498>)).

The smallest coherent repair is Shell-owned, not a domain-specific exception
in generic form validation:

- Preserve a discriminated Space catalog availability snapshot through action
  resolution: loading/unavailable, or ready with its exact generation and full
  canonical choices. Keep `undefined` separately for non-Space manifest
  choices.
- The generic non-editable Select and submit predicate are now in source. Pass
  its local status/error into UIDialog and associate it with the field; retain
  the global notice only as secondary status.
- Bind a ready dialog stage to `catalogGenerationId` and exact full encoded
  choice membership. On a rotation or unavailable transition, discard the old
  stage or reject it before dispatch. The creation dispatch must make the same
  membership check, then deliberately send only `kindId`; the Hub remains the
  final current-catalog resolver.
- Apply the same host-resolution state to all three field consumers, not only
  `OwnedShellDialog`: the window action pane, command pane, and owned dialog
  all call `renderStagedArgControl` ([`Shell helpers:3070`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:3070>),
  [`Shell helpers:3557`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:3557>),
  [`ShellHost:8632`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8632>)).

Required fixture rows are ready-valid, loading, unavailable, ready-empty,
generation rotation with the same `kindId` but a changed full tuple, and a
stale `seedArgs.kindChoice`. Each unavailable/stale row must prove no raw
input, disabled submission, an in-modal accessible explanation and no worker
request. The valid row must prove that the exact selected tuple dispatches.
This audit did not run the proposed rows.

The current renderer host test still expressly admits a canonical but foreign
`s.gis.viewer` choice ([`renderer host test:621`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts:621>)). That row must become a selected-catalog denial when the owner snapshot
lands; preserve its malformed-kind row separately.

## P1: Durable Ready Can Lose Its Ordinary Automatic Open

The created document is durable before Shell receives `ready`, but Shell's
automatic open is only an ephemeral best effort. On a router, plugin install,
actor or document-opening rejection, the `catch` only logs a console warning;
the `finally` then retires the exact creation owner and clears the visible
progress state ([`ShellHost:2012`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2012>),
[`ShellHost:2054`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2054>)). No creation cancellation or durable failure is correct here: Directory has
already indexed the artifact. However, an ordinary user is left with an
unannounced row that can only be recovered by manually rediscovering and
opening it.

For the same-root composition gate, do not inject a relay or reinterpret this
as a failed creation. After Ready, assert the new ordinary Directory row and
click its normal Open route if the product does not promise automatic open.
If automatic open is promised, retain a bounded local `created-open-failed`
presentation state with an accessible ordinary Open retry, bound to the exact
request/runtime/client/session owner. A focused row should force
`openArtifactWithAppRef` or `openDocument` to reject after Ready and prove: no
extra create/cancel request; exact owner cleanup; durable row remains; and its
ordinary row Open mounts the same document. This is a journey availability
gap, not a catalog or backend-authority bypass.

There is also a concrete cleanup condition for that runner. The current
`openArtifactWithAppRef` commits the new primary session before `openDocument`
begins ([`ShellHost:5871`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5871>)), while `openDocument` registers its runtime route and session entry before
awaiting the socket actor ([`ShellHost:4526`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4526>)). A timeout or attach failure therefore cannot merely show guidance: it
must retire its exact worker waiter and route. The runner must deliberately
choose either restoring the original Space (and retiring its newly admitted
app) or retaining an explicitly unbound selected app with visible retry; the
normal `closeDocument` path itself does not destroy an app
([`ShellHost:4595`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4595>)). It must not undo durable Ready. The minimal runner receipt is a non-reusable
`{requestId, runtimeKey, clientInstanceId, openingId}` owner. A late failed
automatic open after an ordinary manual retry has succeeded must be inert.

## Current Read-Only Recheck: Catalog Dispatch Is Now Fenced

The selected catalog is now captured from the effect owner's exact mounted
Space-index document and rechecked at effect replay. The capture requires the
same mounted runtime, client instance, Space, ready status and catalog
([`ShellHost:1216`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1216>)). The request builder rejects a mismatched runtime,
client, Space, generation, non-canonical choice, or a choice absent from
either the captured or the live exact list
([`ShellHost:1257`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1257>)). The replay branch obtains that live
snapshot immediately before emitting the worker request
([`ShellHost:4249`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4249>)).

The neutral corpus covers the exact member plus withdrawn, generation-rotated,
replaced-client, other-Space and absent/non-member cases
([`catalog-authority fixture`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🌱️artifact-creation/🪪️catalog-authority/🔣️.json>)).
The prior dispatch-bypass claim is therefore superseded; no source-level
catalog authority bypass was found in this reread. The reported focused test
outcome was not independently run here.

One qualified presentation inconsistency remains: the real Hub/TS catalog
contract intentionally refuses zero choices
([`OS wire:923`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:923>),
[`creation schema fixture:377`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json:377>)), while the generic
dialog notice still has an “authorized empty” branch. Thus real worker data
can only reach `unavailable`, not ready-empty. That is safe and does not
weaken selection; either identify empty as an intentionally generic-only
presentation state or make it valid consistently across the Hub schema,
worker decoder and selection policy.

## P0: A Retired Opening Can Reattach Its Backbone

`runDocumentOpeningAttemptV1` correctly clears its timeout, closes an
uncommitted exact entry, and retires the socket waiter
([`document opening runner:13`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🟦️.ts:13>)). It does not, however, own a compensating detach for
an `attachBackbone` that completes after its second `current()` check can no
longer succeed. If `closeDocument` or a replacement happens while the caller's
awaited `plugin.attachBackbone()` is paused, close removes the exact entry and
detaches once. The old attach may then resolve and reattach the retired app;
the runner observes `current() === false`, but its `closeDocument` callback
finds no matching entry and cannot detach that late attachment
([`ShellHost:4606`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4606>)).

Make attachment an attempt-owned resource: record that attach began/completed
and call a direct, exact `plugin.detachBackbone(targetSession.instanceId)` in
the uncommitted/retired finalizer, independently of the open-session map. It
must be idempotent with `closeDocument` and not detach a successor. A
deterministic row must pause attach, close or replace the entry, then release
attach and assert no route/waiter/session survives and that the final operation
is a detach after the late attach.

## P0: Background Index Opening Bypasses the New Attempt Owner

`touchSpaceIndexArtifact` still opens a hidden Space-index app directly. It
creates the app, open-session entry, route and socket waiter, then races an
uncleared ten-second timeout
([`ShellHost:6806`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6806>),
[`ShellHost:6841`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6841>)). On timeout or a rejection it removes
only the waiter; the outer catch logs the failure and leaves the open entry,
route and worker-side open live. It separately has the late-attach race above.

This is not covered by the `openDocument` helper, which has one call site
([`ShellHost:4585`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4585>)). Route the background index admission through the same
attempt owner, with an explicit owned-app destroy on unsuccessful admission;
or extract one shared low-level admission. Pin timeout and paused-attach /
replacement rows for that hidden owner as well.

## Follow-Up Recheck: Same-Owner Late Attach Is Now Serialized

The immediate same-owner late-attach defect above is addressed in the current
opening helper. Its finalizer invokes both exact document close and an
attempt-owned detach after any started, uncommitted attachment
([`document opening runner:13`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🟦️.ts:13>)). `DocumentAttachmentLaneV1` serializes the actual plugin calls;
`closeDocument` and the failed attempt use that same lane
([`ShellHost:1861`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1861>),
[`ShellHost:4615`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4615>)). This source reread did not run its
new test command.

## P0: Attachment Lane Does Not Retire a Different Current Owner

The lane is keyed by plugin handle and app instance, but its `attach` method
sets `#attached = owner` and invokes the new attach without first detaching a
different `#attached` owner
([`document opening runner:50`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🟦️.ts:50>)). The open-session registry is instead
keyed by document runtime key. Thus two different documents can be admitted to
the same existing app instance through the current-session/manual-open path;
the second physical `attachBackbone` replaces the first plugin attachment
without retiring its first document entry or route.

Before applying a successor, the lane must either establish that an app
instance owns at most one document, or serialize
`detach(previous) -> attach(successor)` and only then replace `#attached`.
The latter needs an exact row: A commits, B opens through the same app instance,
then A closes. The physical sequence must be `attach(A), detach(A), attach(B)`;
A's close must not detach B, and only B's runtime/session route may remain.

## P0: Ready-Opening Release Can Run Twice After a Release Error

The newly introduced durable Ready runner correctly defers publishing its
private target until a committed exact open and an owner-current recheck
([`ready-opening runner:13`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🌱️artifact-creation/🚪️ready-opening/🟦️.ts:13>)). But either stale-owner branch awaits
`release(target, disposition)` inside its `try`. If release rejects, control
falls into `catch`, which invokes `release` again for the same exact app,
document attachment and instance ([`ready-opening runner:22`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🌱️artifact-creation/🚪️ready-opening/🟦️.ts:22>)). That can double-close/detach/destroy an
already partially retired target.

Use an at-most-once `releaseOnce` latch set before awaiting any destructive
release; aggregate its first error with the opening failure but never retry the
release. Add rows for a release failure before open and after an already
committed open whose origin has retired. Each must observe one release and
must not alter the durable Ready record or a successor.

## Current Read-Only Recheck: Opening Receipts And Background Sessions

The Ready-opening helper now has the required pre-await release latch. It sets
`released = true` before its first `await`, aggregates the first cleanup
failure with the opening failure, and its neutral rows include both current
and retired release-failure cases
([`ready opening helper:13`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🌱️artifact-creation/🚪️ready-opening/🟦️.ts:13>)).
The prior double-release finding is superseded in source; this audit did not
run its fixture.

`openDocument` now translates the boolean low-level attempt result into the
non-reusable exact receipt `{ committed: true, runtimeKey, clientInstanceId }`
before returning it
([`ShellHost:4605`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4605>)).
The deferred Ready flow uses that receipt before it closes, drains and destroys
only the private unactivated target
([`ShellHost:6038`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6038>)).
This removes the former boolean-as-ownership ambiguity. The remaining claim is
source-only until owner-replacement, rejection and retry integration rows run.

The old direct hidden-index opener is replaced by `BackgroundDocumentSessionsV1`,
which serializes per-Space create/reuse/release and makes Shell unmount await
pending pool work
([`background pool:102`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🟦️.ts:102>),
[`Shell unmount:3497`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3497>)).
Background `openDocument` now refuses an already occupied exact runtime before
the same-app/previous-owner sweep, so a delayed hidden opener cannot evict a
foreground Space-index document
([`ShellHost:4567`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4567>)).
`closeDocument` also schedules exact-client pool retirement, allowing a
foreground successor to reap a displaced background instance without touching
the successor
([`ShellHost:4674`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4674>)).

### P1: Cached Background Index Sessions Survive A Plugin Or Identity Replacement Until Another Touch

The pool's `current` predicate compares the captured identity, plugin handle
and exact document client id, but only `run()` evaluates it
([`ShellHost:6839`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6839>)).
There is no identity/loaded-plugin change effect that scans an already cached
pool session. Consequently a cached hidden Space-index app whose identity
changes or whose plugin handle is replaced remains open, with its route and
worker scope, until a later checkpoint touch or Shell unmount.

Add a bounded `retireMatching`/sweep operation to the pool and invoke it from
the identity and loaded-plugin replacement lifecycle. It should queue exact
per-Space retirement through the existing release callback rather than destroy
directly. A pending create is already safe because its post-create `current`
check releases it; the new row only needs a cached successful admission then
an identity or plugin-handle replacement, asserting one exact
close/detach/destroy before any later touch. This is a reclamation gap, not a
foreground authority bypass.

## Follow-Up Recheck: Background Authority Replacement Now Reaps The Exact Cache

The pool now exposes `retain(current)`, which snapshots both cached and pending
Space keys and serializes exact retirement for values no longer current
([`background pool:151`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🟦️.ts:151>)).
Shell invokes it on the identity base URL, user id and loaded-plugin set
([`ShellHost:6886`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6886>)).
Because a pending create retains its own post-create `current` check, and the
retirement work is appended per Space, the sweep neither resurrects an old
admission nor releases a successor under the new identity. The former cached
identity/plugin replacement P1 is superseded in source. The reported focused
test result was not independently rerun by this audit.

## Source Inventory: Renderer Schema Relocations

`renderer72014`'s remaining failures are stale test-source imports, not
ShellHost/Ready/pool type errors. The current schema ownership convention is
the semantic owner's `🧬️schema/🔣️.json`; fixture directories own vectors,
not a sibling `🧬️.schema.json`.

`UiDocumentStore/🟦️.tsx` has only these verified relocation classes:

- `…/🧪️fixtures/<case>/🧬️.schema.json` is the surrounding semantic schema:
  retained-root cases use
  `🖱️ui/🧬️contract/🧵️retained/🧬️schema/🔣️.json`, while retained-wire
  cases use `…/🧵️retained/📦️wire/🧬️schema/🔣️.json`.
- `…/🧪️tests/🧬️schema/🔣️.json` for resident scalar uses
  `…/💾️resident/🔢️scalar/🧬️schema/🔣️.json`.
- Every stale same-owner `📐️schema/🔣️.json` or `🧬️schema.json` maps to
  its existing `🧬️schema/🔣️.json`. This covers actor page; retained
  operations/input/pages; resident scalar, slot, page, binding, reader,
  builder, evidence, payload and metadata; and value/resident.
- The sole nonstandard spelling,
  `…/🧵️retained/🎬️scene/🧾️typed/🔣️.schema.json`, maps to
  `…/🧵️retained/🎬️scene/🧾️typed/🧬️schema/🔣️.json`.

`PluginRuntime/🟦️.tsx` has one equivalent stale path:
`🌱️value/💾️resident/🧬️schema.json` maps to the existing
`🌱️value/💾️resident/🧬️schema/🔣️.json`.

The React top-level fixture imports separately map as follows:

- descriptor-load → `🎠️kernel/🧬️schema/🔣️.json`
  (`$defs.DescriptorLoadFixture`);
- action-semantics and tutorial-document-track →
  `🛂️manifest/🧬️schema/🔣️.json`
  (`ActionSemanticsFixture` and `TutorialDocumentTrackFixture`);
- puzzle session-factory → the exact selected subset root
  `…/🧩️puzzle/…/◻️2d/…/✳️any/🧬️schema/🔣️.json`;
- action-argument choices vector/schema → respectively
  `🧩️action-argument-resolution/🧫️fixtures/🔽️choices/🔣️.json` and
  `🧩️action-argument-resolution/🧬️schema/🔣️.json`.

The old `presence-overlay` schema import has no current JSON schema owner in
the UI contract: its vector exists, but the contract's generated JSON schema
has no `PresenceUpdate`/`PresenceOverlayFixture` definition. It is therefore
not a safe relocation; a narrow canonical definition is required. The Rust
manifest `include_str!` for choices is already current at
`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:5943`.

## Source Boundary: Legacy Backbone Route Replacement

The page-global route map remains keyed only by URI-derived runtime key
([`kernel relay:1660`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1660>)).
Shell currently writes the successor session and replaces that route before it
awaits its attachment lane
([`ShellHost:4597`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4597>),
[`ShellHost:4640`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4640>)).
If an old direct-import guest emits after replacement, this map cannot
distinguish its old app instance from the new entry.

That is not source proof of a current Actor-runtime route bleed. The actual
kernel actor handle owns `enqueue`/`outcomes`, not attach/detach
([`kernel handle:261`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:261>));
the current runtime contains only optional legacy `attachBackbone`/
`detachBackbone` declarations and no implementation assignment. The kernel
also documents the main-thread inbound queue as undrained after the ABI flip.
No first-party source caller of the registered global outbound function was
found; the generated frame worker only installs it. Therefore changing the
legacy route timing cannot be represented as an actor-runtime correctness
repair. Either retire that bridge, or bind source-owner correlation at the
real actor effect/outcome delivery seam and qualify it with a real guest.

## P0 Source Boundary: Mounted Actor Has No Backbone Port

Source-only review of the current `PluginRuntime` factory separates the
working app/document API from the retired backbone API.

`createApp`, `handleAction`, `handleCommand`, `applyMutations`,
`readAppDocumentPack`, and `loadAppDocumentPack` are concrete named adapter
members, not proxy-generated fallbacks. The adapter creates an
`AppChannelClient` after the real raw `handle.createApp`, and each named
method calls a concrete channel command
([`PluginRuntime:1489`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1489>),
[`PluginRuntime:1501`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1501>),
[`PluginRuntime:1517`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1517>),
[`PluginRuntime:1536`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1536>)).
`loadPluginModule` also owns the real per-instance actor id and serial
`submitTurn` closure ([`PluginRuntime:1111`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1111>),
[`PluginRuntime:1200`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1200>)).

By contrast, the only three retired fields are deliberately literal
`undefined`: `attachBackbone`, `detachBackbone`, and `ephemeralSnapshot`
([`PluginRuntime:1547`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1547>)).
They are not dynamically synthesized. Shell still optional-calls the first
one inside its successful attachment lane
([`ShellHost:4636`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4636>)),
so that lane currently does no guest attachment at all.

This is a real mounted-Shell execution blocker, not merely an unused legacy
route. `PluginRuntime::ensure_plugin_initialized` explicitly says no
per-instance `EffectBackbone` channel is registered and that backbone-dependent
VCS paths fail with `no host backbone linked`
([`plugin runtime:29794`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29794>)).
The only wasm port implementation is intentionally uninhabited; its
`PortBackbone::new` has no channel and `send`/`receive` return that same
failure ([`store:17183`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17183>),
[`store:17201`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17201>)).
The existing VCS law establishes the behavioural consequence: an attached
local edit reaches its peer, while the same edit after detach changes only the
local graph ([`plugin:37630`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37630>)).
Thus the current Shell can apply worker-originated updates through its real
`applyMutations`/`loadAppDocumentPack` calls, but cannot establish the guest
side required for an ordinary local edit, approval mutation, or Undo to travel
back through the worker and converge with the peer.

The WIT already owns the replacement vocabulary: an outbound
`effect.send-message { target: backbone(uri), payload }` and inbound
`event.message { source: backbone(uri), payload }`
([`plugin WIT:24`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:24>),
[`plugin WIT:199`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:199>),
[`plugin WIT:755`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:755>)).
However, current renderer execution strips only shell-targeted
`send-message` values into `AppFrame` replies
([`PluginRuntime:388`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:388>));
a backbone-targeted value becomes a leftover and `wireEffectToFriendly` has no
case for it, logging and dropping it
([`PluginRuntime:642`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:642>)).
Inbound `Event::Message` conversion exists, but the reactor handles only the
typed shell ACK and ignores all other sources, including Backbone
([`reactor turn:299`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:299>)).

The smallest correct implementation boundary is therefore a new, named,
per-live-instance actor message port owned by `loadPluginModule`, rather than
reviving the retired `attachBackbone` method or its process-global route. It
must retain the exact `(actorId, activation/lifecycle lease, instanceId,
runtimeKey, clientInstanceId, document scope)` at the point Shell's
worker-backed document attachment is committed.

1. During `runQueuedTurn`, consume only a raw `send-message` whose target is
   the port's exact `backbone(uri)`; dispatch it to the already-open document
   worker under the retained client/scope. Do this before generic friendly
   conversion, so it cannot be silently dropped.
2. Route the worker's bounded document delta into one serialized
   `event.message { source: backbone(uri), payload }` on that same actor and
   activation; reject after detach, replacement, scope/client mismatch, or
   generation revocation. The reactor/app boundary must then consume that
   event; current `Event::Message { .. } => {}` cannot make inbound backbone
   semantics true.
3. Dispose the port before the old document attachment or actor is released,
   await its exact lane, and only then permit a replacement owner. This is the
   real source-owner fence, rather than changing
   `registerPluginBackboneRoute` timing.

The standalone `💻️os/⚡️effect-backbone.ts` is not a safe drop-in repair:
no current `PluginRuntime`/`ShellHost` import wires it into the actor, and its
worker helper opens a synthetic `os.effect.backbone` document with a new random
client id ([`effect backbone:456`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts:456>),
[`effect backbone:490`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts:490>)).
That cannot stand in for the already-authorized GIS document/session.

Minimum qualification rows should use one real mounted document actor (not a
legacy global): (a) local command emits one worker send with exact scope/client
and a second actor receives the resulting mutation; (b) inbound worker delta
submits exactly one backbone event to the current actor; (c) stale client,
replaced activation, foreign URI, and post-detach messages produce neither a
worker send nor a guest turn; and (d) dispose waits for port/attachment
retirement before a same-URI successor may emit. A native twin must prove that
the VCS channel is nonempty for (a), because source-only direct
`applyMutations` does not prove a mounted guest can publish.

## P0 Design Packet: Canonical Actor Backbone Port V1

### The current TypeScript codec is not the Store codec

The TypeScript `encodeBackboneMessage` writes a hand-rolled leading tag
(`0` snapshot, `1` mutations, `2` acknowledgement), then its own byte/vector
format ([`OS TypeScript:347-386`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:347>)). The Store's declared channel contract is instead `OpBinary`:
`format=1 | variant-ordinal uLEB128 | canonical Pack record body`
([`Store:16956-17058`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16956>)). Thus a current TS snapshot starts with `00`, which Rust rejects as an unsupported format, and the current TS mutations type is additionally wrong: Rust carries the opaque result of `encode_envelopes`, not a TypeScript `WireMutationEnvelope[]` ([`Store:16960-16965`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16960>)).

The shared encoder must therefore replace—not wrap—the raw-tag codec. It must
emit one canonical `BackboneMessage::encode_op` byte string and type mutation
payload as opaque `Uint8Array` until a separately qualified exact twin of
`encode_envelopes` exists. The current handwritten Rust `BackboneMessage`
decoder also differs from the generic `op_rt` decoder: it uses
`decode_record_body`, does not demand terminal bytes, and does not re-encode
to establish canonicality ([`Store:17055-17066`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17055>)). The new port must use the generic exact behaviour (or make this
specialized impl equally exact) on **both** ingress directions; otherwise TS
strictness cannot protect a native path that accepts trailing or alternate
record encodings.

The derivation is deterministic: declaration order supplies variant ordinals
([`Dsl derive:1741-1830`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:1741>)), and field declaration position is its
u16 id ([`Dsl derive:1176-1210`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:1176>)). The exact V1 layout is:

| Variant | Ordinal | Record fields |
| --- | ---: | --- |
| `Snapshot` | 0 | `0: Bytes64 pack`, `1: Bytes64 spr` |
| `Mutations` | 1 | `0: Bytes64 envelopes` |
| `Ack` | 2 | `0: List<Text> op_ids` |

Record bodies are `symbolCount uLEB128`, sorted UTF-8 symbols, then sorted
`fieldCount` / `fieldId` / tagged fields ([`Pack:2187-2222`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:2187>)). The relevant tags are `Bytes64=08`,
`List=0c`, interned text `06`, and inline text `07`
([`Pack:23-35`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:23>)). Strings at most 128 bytes are symbolized and symbols are lexicographically sorted
([`Pack:185-207`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:185>)).

The following small hex vectors are source-derived from that grammar and are
independently calculable; they are **not native-executed evidence** yet:

| Value | Canonical bytes (hex) |
| --- | --- |
| `Snapshot { pack: aa, spr: bb cc }` | `01000002000801aa010802bbcc` |
| `Mutations { envelopes: dd }` | `01010001000801dd` |
| `Ack { op_ids: [] }` | `01020001000c00` |
| `Ack { op_ids: ["a"] }` | `010201016101000c010600` |

The decoder law must reject empty input, format other than `01`, an ordinal
outside `0..=2`, overlong/nonminimal uLEB128, invalid UTF-8 symbols,
duplicate/unknown/missing fields, a tag inconsistent with the fixed field
shape, trailing bytes, and bytes that decode but fail canonical re-encoding.
The existing TS Pack helpers (`packBuildSymbols` and string helpers near
[`OS TypeScript:1453`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1453>)) are appropriate primitives; the old `readBytes`/`writeVecEnvelope`
backbone codec is not.

### Agreed control envelope and acknowledgement rules

The agreed minimal schema-first control plane is sound if it remains separate
from both retired AppChannel attach commands and instance lifecycle receipts:

```
Event::Message {
  source: Shell(instance),
  payload: strict Pack semio.plugin.document-backbone-binding.v1 {
    operation: bind | retire,
    instanceId: PackUInt(u32),
    bindingGeneration: PackUInt(u64),
    uri: UTF-8 string <= 1280 bytes
  }
}

Effect::SendMessage {
  target: Shell(instance),
  payload: strict Pack semio.plugin.document-backbone-binding-receipt.v1 {
    operation: bound | retired | refused,
    instanceId, bindingGeneration, uri,
    code?: bounded closed refusal code
  }
}
```

The complete raw control Pack, including its schema envelope, must be at most
4096 bytes, reject unknown/duplicate/trailing fields, and use only integral
`PackUInt` values—no numeric coercion. This is consistent with the existing
fixed-page control boundary (`ACTOR_BYTE_PAGE_BYTES=4096`)
([`actor page:2-44`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📃️page/🦀️.rs:2>)). `Event::Message` is suitable for control because the control is
processed by the actor; it is not sufficient by itself as a binding authority.

The host must retain, privately, the exact `ShardInstanceLifecycleLease`
(`actor id`, activation generation, instance id, captured guest lifetime),
runtime key, client instance id, and local-or-Hub document scope. The guest
does not need those secret host fields. On every bind or receipt, the host
compares the command/receipt triple against the retained owner and also
revalidates that the worker open receipt, selected document, and current Shell
session still equal that private owner. `captureInstanceLifecycle` already
provides the correct immutable activation/lifetime source
([`ShardClient:1484-1532`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1484>)); its existing `captured/accepted/retired` receipt state must not be
overloaded with this independent port protocol.

Admission and retirement need these exact fences:

1. Root's `loadPluginModule.bindDocumentPort(instanceId, binding)` captures the
   live lease **after** worker document open and cold-pair application are
   confirmed. It installs a private `BindingPending` owner, but forwards no
   Backbone effect or inbound worker bytes.
2. Guest accepts `bind` only when `source == Shell(instanceId)`, the actor's
   currently live instance equals `instanceId`, and its generation is newer
   than any retained active/terminal binding. It constructs the concrete
   per-instance Store channel then returns `bound`; an equal duplicate bind may
   resend the retained `bound` receipt, while a different or stale triple is
   `refused` and cannot replace an active channel.
3. The host activates forwarding only after an exact `bound` receipt and a
   second local owner-current check. A guest `backbone(uri)` effect before that
   point, from another URI, or after retirement starts is dropped fail-closed.
   The current reactor deliberately ignores non-Shell messages
   ([`reactor turn:299-306`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:299>)), so Home must make the new
   command handler and Store channel insertion explicit.
4. Host retirement first closes outbound/inbound admission under the same
   private owner, then sends exact `retire`. Guest stops new Store sends and
   drains/retains its exact channel owner until terminal before `retired`.
   Only that exact receipt lets the host await worker attachment retirement and
   close/reuse the app/lifecycle. A late bound/retired/refused receipt, old
   generation, replacement client/scope, or duplicate URI cannot revive the
   port. Keep an exact terminal-receipt ledger for duplicate retire delivery
   until the instance lifetime itself closes.

This uses the WIT message vocabulary already present
([`plugin WIT:24-38`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:24>), [`plugin WIT:755-804`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:755>)) without inventing a legacy host API. It deliberately avoids the
existing lifecycle receipt because that receipt enforces a different close
state machine ([`ShardClient:1819-1872`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1819>)).

### Data-plane bound and snapshot distinction

`message-event.payload` is currently an unconstrained WIT `list<u8>`
([`plugin WIT:755-770`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:755>)). Control's 4096-byte cap does not bound
`BackboneMessage` bytes. Do not make the new port silently unbounded.

For the smallest safe first slice, hot-port `Mutations` and `Ack` must be
bounded to the existing Store envelope ceiling, 262,144 bytes / 64 × 4096-byte
pages ([`Store:7772-7774`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:7772>)). The current WIT message has no cursor/page
field, so this is a total-byte admission limit, not a claim that it has already
been paged. A future fragmentation protocol must add one owned cursor and
acknowledgement; it must not split bytes ad hoc.

`Snapshot` is different: current Store `attach_backbone` immediately sends
one ([`Store:25090-25099`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:25090>)), while the real cold-pair ingress is the existing
bounded snapshot transport (64 × 64 KiB = 4 MiB)
([`cold pair:2-44`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📥️cold-pair/🦀️.rs:2>)). Consequently the initial binding must attach an
already-cold-pair-materialized Store **without** emitting a second raw
Snapshot, or it must use the existing cold-pair route for a resync. It must
not truncate a Snapshot to the hot-port 256 KiB limit or pass up to 4 MiB as
an unbounded `message` list. The OpBinary codec still needs Snapshot vectors
because Store channel semantics retain that variant, but the first hot-port
law should refuse it and require cold-pair resynchronization.

### Reusable implementations and qualification

`EffectBackbone` is not a transport to wrap: it is unconnected to
`PluginRuntime`/`ShellHost` and allocates a synthetic document/client. Reuse
only the ownership pattern of its subscription teardown. The Store's
`BackboneChannelPort` is the correct domain seam, but `BackboneChannelPorts`
is intentionally uninhabited today ([`Store:17164-17221`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17164>)); Home should add a
bounded, per-instance concrete channel owned by the binding—not a process
global port. Existing Store channel laws already prove attach, incoming
mutation, acknowledgement, and detach locally
([`Store:25090-25135`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:25090>)), but do not prove WIT ownership.

Add a neutral `document-backbone-binding-v1` fixture alongside the plugin WIT
contract with bind/bound, stale instance/generation, URI mismatch, duplicate
bind, worker/client/scope replacement, inbound-before-bound,
post-retire outbound, terminal duplicate retire, and successor-after-retire
rows. Add companion `op-binary` golden and hostile rows above. Qualification
requires (1) strict TS and Rust codec goldens, (2) a TS `ShardClient` owner
law covering every private-owner mismatch, and (3) a native Store/reactor/WIT
law proving one real local mutation crosses the active port, one inbound
mutation is ingested and acknowledged, and retirement leaves neither channel
nor stale delivery owner. This packet is source-only; no new port or runtime
law has been run.

## P0 Source Boundary: Envelope Batches Must Stay Opaque Across the Actor Port

The current Rust batch grammar is positional, not Pack-record based:

```
batch = envelope_count:uLEB128 | envelope*
envelope = mutation_id:str | document_id:str | actor:str |
           dependency_count:uLEB128 | dependency_id:str* |
           diff_schema:str | diff_payload:bytes |
           inverse_schema:str | inverse_payload:bytes |
           hlc_actor:uLEB128 | hlc_physical_ms:uLEB128 | hlc_logical:uLEB128
```

This is exactly what [`encode_envelopes`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:883>) emits and what the Rust Store consumes from `BackboneMessage::Mutations` ([`Store pump`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16702>)). It has **no field ids, bools, or option tags**. Those belong to surrounding frame/Pack grammars and must not be invented in this byte blob. Rust's `read_bool` also intentionally accepts every nonzero byte as true ([`wire:552-560`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs:552>)); it is not relevant to the batch itself.

The following are source-derived, not Rust-executed, exact batch vectors:

| Batch | Canonical hex |
| --- | --- |
| no envelopes | `00` |
| one: `m,d,a,[],s,[aa],i,[bb cc],HLC(3,4,5)` | `01016d0164016100017301aa016902bbcc030405` |
| one: `m,d,a,[p],s,[01],i,[02],HLC(3,4,5)` | `01016d016401610101700173010101690102030405` |
| maximal `u64` HLC limb | `ffffffffffffffffff01` |
| first non-precise JS integer, `2^53` | `8080808080808010` |

The last two vectors establish why a `number` is not an exact TS representation of a Rust `u64`—all three HLC limbs are `u64` in Rust ([`causal:815-825`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:815>)), but `WireMutationEnvelope.timestamp` exposes `number` ([`replication TS:149-157`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:149>)). The current generic TS reader also permits precision loss, accepts a tenth LEB byte payload above Rust's permitted `1`, does not require minimal encodings, and uses replacement UTF-8 ([`replication TS:256-313`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:256>)). In contrast, Rust rejects the tenth-byte overflow and invalid UTF-8 ([`varint`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs:106>), [`string`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs:513>)).

More importantly, the current `decode_envelopes` allocates `Vec::with_capacity(count as usize)` and each dependency vector from untrusted counts, then returns without terminal-byte or canonical validation ([`causal:845-903`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:845>)). The TS counterpart likewise does not terminal-check and maps into a lossy application shape ([`replication TS:781-789`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:781>)). The byte-vector-to-DSL worker bridge itself copies every supplied item before decoding and has no byte ceiling ([`sync envelope serde:47-76`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:47>)). Therefore the new 262,144-byte **outer** message cap is necessary but not sufficient for bounded native batch materialization.

The corrected `BinaryBackboneMessage` public shape should remain `{ kind: "mutations", envelopes: Uint8Array }`, as now defined ([`OS:347-354`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:347>)). It must not expose `MutationEnvelope[]` at the plugin/Shell port. That application type omits HLC and other wire facts; the generic adapters fabricate HLCs (`0,0,index`) on encode and derive inverse/base values on decode ([`replication TS:42-73`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:42>), [`replication TS:781-789`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:781>)). Re-encoding a guest-originated batch through that shape changes causal identity.

There are two live examples of that loss today:

1. Shell sends a remote event to the plugin after rebuilding it through `encodeCausalEnvelopeBatch` ([`ShellHost:2329-2347`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2329>)).
2. Plugin outbound bytes are decoded to `MutationEnvelope[]` and then sent through the worker's actor-domain request ([`ShellHost:3461-3485`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3461>)). The browser fallback then creates fresh causal timestamps when it emits hub `Commands` ([`backbone worker:2401-2406`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2401>)).

The smallest coherent port is consequently: admit a `Uint8Array` only after one strict, bounded batch validation; retain and forward the exact bytes unchanged through `BinaryBackboneMessage`; decode once at the Store/actor’s owned ingress; and use a separate, clearly named local-command construction path for UI-originated mutations that is allowed to mint an HLC. Do not make the plugin port call `mutationEnvelopeFromWire`/`mutationEnvelopeToWire`, `ReplicationPackCodec`, or the `applyMutations` convenience pack wrapper. The current Rust worker bridge already describes `LocalMutations`/`RemoteMutations` as `encode_envelopes` bytes ([`sync envelope serde:51-75`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:51>)); the direct port should carry that same byte witness rather than reconstruct a domain envelope in Shell.

Add a replication-owner `decode_envelopes_exact_with_limits` for this port rather than altering unrelated broad wire consumers: reject raw batches above 262,144 bytes before parsing; use `BigInt` in the TS twin for the three `u64` HLC limbs; reject nonminimal/overflow varints, invalid UTF-8, count/identifier/dependency excess, and trailing bytes; debit each copied string/payload/container before allocation; then canonical-reencode and compare exact bytes. Existing causal constraints support a maximum of 8,192 DAG entries and 256-byte mutation/dependency ids ([`causal:178-179`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:178>)); schema/payload limits still need an explicitly owned port limit. The native Store must call this exact bounded decoder, not the present `decode_envelopes`, at its WIT/worker boundary.

## Current OpBinary and Binding-Schema Review

The newly landed TS outer OpBinary codec has the right public data model and does enforce format `01`, variant ordinal `0..2`, prescribed field count/order/id/tag, terminal position, and canonical re-encoding ([`OS:356-462`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:356>)). Its `packReadVarintBigInt` correctly enforces Rust's ten-byte/u64 overflow rule ([`OS:1487-1505`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1487>)); every decoded count is reduced below the 4 MiB/256 KiB caps before conversion back to `number`. The schema U64 regex is also an exact decimal partition of `0..=18446744073709551615`: the `1[0-7]…` through `…5161[0-4]` alternatives partition each lexicographic 20-digit prefix and the final literal admits only the maximum ([`binding schema:6-8`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧬️schema/🔣️.json:6>)). No acceptance hole was found. Pin acceptance of `0`, `9999999999999999999`, `10000000000000000000`, and `18446744073709551615`, and refusal of `00`, `+1`, `-1`, `18446744073709551616`, and a JSON number.

Two remaining concrete points need closure:

1. Native `BackboneMessage::decode_op` still calls permissive `decode_record_body`, not `decode_record_body_exact`, and does not terminal-check or canonical-reencode itself ([`Store:17020-17035`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17020>), [`Pack exact API`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:2222>)). TS strictness therefore does not prove strict host admission. Make the Rust OpBinary use exact record decoding and compare `encode_op()` with input before exposing the port.
2. TS obtains `symbolCount` and materializes every symbol before it knows that Snapshot/Mutations must have no symbols ([`OS:428-432`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:428>)). A 4 MiB Snapshot can request millions of empty symbol records and allocate a disproportionate JS string array before canonical rejection. Reject a nonzero symbol count immediately for variants 0/1; for Ack apply an explicit symbol/item ceiling before `symbols.push`, not merely a raw-byte ceiling.

No build or runtime law was run during this audit.

## P0 Implementation Packet: First-Class Raw Document-Backbone Worker Frames

The raw port cannot be repaired only at `ShellHost`.  The current worker
contract deliberately makes `send` carry an `ArtifactActorMsg` and makes
`event` carry an `ArtifactEvent` ([`OS worker types:647-692`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:647>), [`worker request/response:1039-1120`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1039>)).  Its only special conversion is
`localMutations`/`remoteMutations`, which converts a domain envelope array to
and from a byte list ([`OS worker bridge:1197-1222`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1197>)).  Reusing that generic `send` branch for the new port would make the raw
packet another unbounded, permissively cast actor message.  It also obscures
which messages must use strict OpBinary validation.

The minimal authoritative owner is therefore the existing **OS backbone-worker
wire** (`products/os/🟦️.ts` plus Rust
`store/sync::backbone_worker_wire`), not the WIT binding-control schema.  The
control schema only decides whether a given Shell/instance/generation/URI owns
the data port.  Add two dedicated, schema-first worker-frame variants there:

```
DocumentBackboneSendV1 {
  kind: "document-backbone-send",
  documentId, spaceId?: string, clientInstanceId,
  message: Uint8Array // one canonical hot BackboneMessage
}
DocumentBackboneReceiveV1 {
  kind: "document-backbone-receive",
  documentId, scope?: DocumentScope, clientInstanceId,
  message: Uint8Array // one canonical hot BackboneMessage
}
```

`spaceId`/`scope` and `clientInstanceId` deliberately mirror the current
`send`/`event` shapes: `dispatchBackboneWorkerRequest` already resolves their
complete runtime key and rejects a non-current client
([`worker dispatch:202-244`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:202>)).  Plugin instance, binding generation, URI, activation, and capability
remain private to the Shell's successful WIT bind receipt; duplicating them in
a worker packet would create a second authority source.  `message` is always
the existing outer `BackboneMessage` OpBinary, whose `Mutations.envelopes` is
the raw `encode_envelopes` batch.  The existing outer exact decoder and hot
cap are reusable ([`Store:17024-17056`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17024>)).

For this first slice, **only `Mutations` is admitted or emitted**.  `Snapshot`
is already a cold-pair responsibility, and `Ack` currently has no actor
completion semantics—`relay_one_backbone` explicitly drops it
([`sync actor:1768-1778`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1768>)).  Accepting an Ack at the new port would falsely acknowledge an
operation.  Thus the new frame parser must reject both variants even though
the general `decode_hot_backbone_message_exact` currently refuses only
Snapshot.  A later Ack feature needs its own retained completion owner and
law; it must not be smuggled into this mutation-preservation patch.

### Required simultaneous changes

1. **OS TypeScript wire owner.** Extend `BackboneWorkerRequest` and
   `BackboneWorkerResponse`, then give the two new kinds dedicated exact
   encode/decode branches in `encode/decodeBackboneWorker{Request,Response}`
   ([`OS:707-847`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:707>)).  The parser must require the exact field set, validate every packed
   byte before copying it, reject raw input over 262,144 bytes before inner
   decoding, require `decode_hot_backbone_message_exact`-equivalent outer
   canonicality, require `Mutations`, and call the new bounded canonical
   `decode_envelopes_exact_with_limits`.  Do not use the current final
   `return parsed as BackboneWorker{Request,Response}` for this variant.

2. **Router and both worker implementations.** Include
   `document-backbone-send` in the current `send|close` owner-routing branch
   in `backbone-worker.ts`; otherwise a Hub-bound document can accidentally
   select the wrong execution owner.  In the Rust worker, add matching
   `backbone_worker_wire::{Request,Response}` variants and handle the send by
   strict-decoding the outer message and its envelope batch at the actor
   ingress before forwarding the resulting owned envelopes to the existing
   actor command.  The wasm worker currently forwards every `Send` as a boxed
   `ArtifactActorMsg` and wraps every subscription event uniformly
   ([`worker:43-88`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs:43>)); the new distinct packet consequently needs its own two
   match arms, not an unchecked new `ArtifactActorMsg` variant.

   In the TypeScript fallback, add the same `handleTsRequest` branch and
   decode its raw packet once before it reaches `handleLocalMsg`.  Hub-bound
   documents deliberately select this fallback even when a Rust worker is
   available ([`worker dispatch:224-228`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:224>)), so a Rust-only change is not a usable
   browser implementation.

3. **Receive at the canonical source, not by rehydrating in Shell.** The Rust
   worker should derive a `document-backbone-receive` packet from each actor
   `RemoteMutations` event with `encode_envelopes(envelopes)` before it posts
   to JS.  The actor already creates that canonical raw batch when it delivers
   remote operations ([`sync actor:2501-2509`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2501>)); the worker must use the same causal
   codec, wrap it once with `BackboneMessage::Mutations.encode_op`, and never
   pass through `MutationEnvelope[]` at the Shell boundary.

   The TypeScript fallback must create the equivalent raw receive frame in
   `handleHubFrame`'s `Commands` branch *before* `fromWireEnvelope`
   ([`worker:2878-2897`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2878>)).  This requires the causal TS twin to retain all three HLC limbs as
   `bigint` and to use the strict batch encoder; its current `number` model
   cannot reproduce Rust `u64` timestamps.  Continue emitting the existing
   domain event only for host projections/non-bound consumers.  When the
   exact document binding is active, Shell must forward the raw receive frame
   and **not** also call `plugin.applyMutations` for that same mutation,
   otherwise the guest applies it twice.

4. **Shell data edge.** Replace the two lossy calls at
   `ShellHost:2329-2347` and `ShellHost:3461-3485`: a bound port forwards the
   `document-backbone-receive.message` directly to
   `postPluginBackboneInbound`, and plugin outbound bytes become one
   `document-backbone-send` frame after an outer exact validation.  It must
   not call `encode/decodeCausalEnvelopeBatch`,
   `mutationEnvelopeToWire`, `mutationEnvelopeFromWire`, or
   `applyMutations` on that bound data path.  Shell rejects the packet unless
   its runtime key, client, and separately retained bound WIT owner are all
   current; retirement closes that admission before the WIT retire request.

5. **Honest authority boundary.** Exact bytes can be proved from plugin →
   Shell → worker → actor ingress and from remote actor delivery → worker →
   Shell → plugin.  They cannot presently be claimed identical from a local
   plugin submission through the Hub: both the Rust actor
   ([`sync actor:2444-2494`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2444>) and TS fallback
   ([`worker:2385-2406`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2385>)) assign the authenticated actor and fresh HLC when creating
   a `ClientFrame::Commands`.  That is an intentional host-authority
   transition, not a byte transport.  Preserve raw bytes until the actor has
   validated/decoded them; then make this stamping boundary explicit in the
   law.  Do not weaken it merely to assert equality across a semantic rewrite.

### Qualification packet

- Extend the existing plugin binding fixture with data-admission rows:
  before-bound, stale/replaced client, stale binding generation, after-retire,
  oversized, Snapshot, and Ack all refuse; one current `Mutations` frame
  reaches exactly one bound owner.  Keep causal byte goldens/hostiles in the
  replication-owned fixture rather than duplicating its grammar in the binding
  fixture.
- Extend [`backbone-envelope-io`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts) and the renderer
  `actor-backbone` test with the raw worker send/receive round trip, canonical
  mismatch, wrong field set, raw limit, and no Shell envelope conversion
  assertion.
- Add one TS-fallback worker law using a Hub `Commands` frame with a HLC limb
  above `Number.MAX_SAFE_INTEGER`; the receive message must equal the strict
  causal batch/outer OpBinary and reach the active port once.
- Add one native wasm-worker/actor law: open current client, submit a raw
  mutation frame, observe one actor admission; inject a remote mutation and
  observe the exact canonical raw receive frame; then close/retire and prove
  neither a late send nor a late receive reaches the successor.  Include
  malformed/noncanonical/trailing/oversized inner batch rows.  No such runtime
  law was run in this audit.

### Implementation-shape correction: use the dedicated actor fields

The active ownership split selects the narrower representation within the
existing worker `send`/`event` envelopes, rather than new top-level worker
kinds.  The same data contract is therefore represented as these two new
fields:

```
ArtifactActorMsg::DocumentBackbone { message: Vec<u8> }
ArtifactEvent::DocumentBackbone { message: Vec<u8> }
// TypeScript: { kind: "documentBackbone", message: Uint8Array }
```

This keeps the current `send` request's runtime-key/client admission and the
wasm worker's generic subscription ownership, while making the raw lane
unambiguous in both request and event schemas.  It is safe only if the fields
use a dedicated strict byte-vector serializer/parser in
`wireArtifactActorMsg`/`parseArtifactActorMsg` and
`wireArtifactEvent`/`parseArtifactEvent`; they must not take the current
permissive cast branch.  Add the raw message length to
`artifact_actor_message_bytes`, enforce the 262,144-byte hot cap before an
inner decode, and update both native and wasm `handle_cmd`/remote-delivery
matches.  The `send|close` owner router then needs no new top-level variant,
but both TypeScript and Rust worker serializers still change together because
the nested actor/event discriminants are part of their Pack wire shape.

The same non-duplication and authority rules above remain: `DocumentBackbone`
is the bound plugin's sole mutation ingress/egress; `RemoteMutations` may stay
only for explicitly non-port host consumers, never as a second plugin delivery
for the same active binding.  This correction supersedes the illustrative
top-level `document-backbone-send`/`receive` shapes above; their exact field
set remains the ownership model carried by the existing surrounding
`send`/`event` envelopes.
