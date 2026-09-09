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

## Shell Cold Pair and Two-Author Composition Follow-Up (Source Audit)

The new `OpenDocumentSession` path has the right owner boundary: the session
retains a private port and a bounded pre-bind input queue, `snapshotReplaced`
does the cold load, and `entry.ready` settles only after the binding method
returns.  In particular, the 64-message/4 MiB queue in
[`ShellHost:2001-2010`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2001>) is a legitimate bounded pre-Bound reservation: the actual port serializes its delivery behind `bind()`.

Four defects existed at this audit point and were handed to the Shell owner:

1. `runDocumentOpeningAttemptV1` cleared its only deadline immediately after
   `socket()` ([`document opening:36-42`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts:36>)), while a Hub opening subsequently awaited `entry.ready`
   ([`ShellHost:4686`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4686>)).  A socket actor with no first
   snapshot could retain the document admission forever.  The deadline must
   cover socket, cold-ready attach, and commit; the regression needs a
   socket-success/ready-never-settles row proving close, exact lane retirement,
   and no residual timer/session.
2. A second snapshot during an active cold replacement raised
   `document-backbone.snapshot-overlap`
   ([`ShellHost:2053`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2053>)) and was converted into a nonretryable bootstrap failure
   ([`ShellHost:2436-2457`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2436>)).  Rebootstrap and watcher traffic can naturally overlap.  The
   replacement owner needs one active pair plus one latest bounded pending pair,
   with close/rebootstrap invalidation and a paused-first/second-wins law.
3. `DocumentAttachmentLaneV1.replace` marked `#attached` before `apply`
   ([`document opening:101-102`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts:101>)); the owned cold-load branch did not release that exact owner on
   loader/bind failure ([`ShellHost:2059-2066`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2059>)).  A failure can leave the lane logically attached
   without a live port.  Cleanup must use an intent token so it cannot release a
   same-string successor; add failing cold-load and refused-bind retry rows.
4. The Shell's `prepared` callback publishes its port and map before Bound
   ([`ShellHost:2026-2034`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2026>)).  If the bind later refuses, PluginRuntime can retire its
   private map while Shell retains the failed candidate.  The exact candidate
   must be removed from Shell only after the binding method has completed its
   own refusal/indeterminate reconciliation.

The two-author acceptance command is deliberately stronger than its source
fixture.  `trusted-stdio-gis-bundle-check --two-author-shell` performs one
combined Hub/MCP build, a real verified-GIS seed law, materialization,
candidate/current validation, Chromium closed-actor proof, and then the
same-data-root Browser/MCP/Shell process
([`Hub script:12629-12670`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12629>)).  The staged browser host rejects non-ticket-owned or digest-mismatched GIS
component/descriptor inputs before it creates module roots
([`dev script:457-488`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:457>)).  Thus its first practical prerequisite is a fresh exact artifact/binary
closure; the neutral fixture is not evidence that an actual map mounted.

Two acceptance-contract discrepancies remain to be corrected before claiming
that process journey:

- The fixture says the two MCP clients and sockets open before either Shell
  mounts, but real code mounts Author A's ordinary creation Shell first—only
  then can it discover the Directory-issued artifact id, open clients/sockets,
  and mount Author B ([`Hub script:11704-11736`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11704>)).  Update the fixture's ordering rather than
  documenting an impossible sequence.
- The fixture claims restart proves `no-retained-running-job`; after restart
  the harness proves only the durable checkpoint equality and both mounted maps
  ([`Hub script:11811-11816`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11811>)).  It correctly disposes private MCP handles before
  restart, so it cannot read the old job.  Make this an explicit nonclaim for
  the browser journey, rely on the separate native supervisor-recovery law, or
  add a bounded server-owned terminal-count diagnostic that exposes neither a
  handle nor job contents.

No build, browser, or native process was run during this audit.

## P0: Bootstrap Deadline Needs an Actual Owner-Tied Timer

The worker currently stores `artifactBootstrapDeadlineMs` when a Bootstrap
frame arrives ([`backbone-worker:2872-2875`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2872>)) and clears it on abort or successful finish
([`backbone-worker:2678-2685`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2678>), [`backbone-worker:2840-2845`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2840>)).  There is no timer, scheduled watchdog, or other reader of
that state outside construction of `ArtifactBootstrapAssembler`.  Therefore a
server that sends Bootstrap and then sends no chunk/done frame leaves the
15-second deadline unobserved indefinitely.  This affects both initial
bootstrap and a `artifact-rebootstrap-required` recovery: the Shell's new
60-second opening deadline covers initial attach, but not a later open session
whose port has been deliberately retired for rebootstrap.

The minimal repair is an owner-tied timer: capture the exact
`DocumentArtifactBootstrapOwner`, assembler, and deadline when starting; when
it fires, first prove all three are still current, then call
`rejectArtifactBootstrap(state, Error("artifact bootstrap deadline exceeded"),
owner)`.  Clear precisely that timer in both `abortArtifactBootstrap` and the
success path.  A generic state lookup is unsafe because a late first timer
could otherwise reject a successor transfer.  The worker law needs a
non-inline Bootstrap with no further frames, one `deadline-exceeded` event,
cleared assembler/owner/deadline and closed socket; then a successor bootstrap
must complete after the old timer would have fired.

The binding contract has since clarified that Ack is a guest-to-Shell ingest
receipt, not a new actor command completion.  It must be made an explicit
terminal Shell consumption rather than falling through a generic
`kind !== "mutations"` drop.  Foreign-URI backbone effects must fail their
own old turn, and a stale/closing exact port must not retry through a successor
owner.  These semantics need a final effect-path law.

## P0 Follow-Up: Rebootstrap Must Be Timed Before Bootstrap Exists

The downstream Bootstrap watchdog has since been repaired in current source:
[`backbone-worker:2681-2701`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2681>) schedules an exact
owner/assembler/deadline timer, and the abort and successful-install paths
clear it.  That closes the no-chunk/no-done transfer hole, but it deliberately
cannot cover the earlier **no Bootstrap at all** recovery state.

`requireArtifactRebootstrap` drops the verified pair, pack, SPR, frontier and
resume token, emits the rebootstrap event, then closes the live socket
([`backbone-worker:2778-2792`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2778>)).  Its reconnect loop retries `connectHubOnce` until document close
([`backbone-worker:2306-2407`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2306>),
[`backbone-worker:2415-2417`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2415>)).  The individual grant requests have their own 10-second limit, but no
single owner or deadline spans repeated grants, socket establishment, Hello,
and the first Welcome.  The existing Shell opening deadline cannot help: the
opening already committed when a later rebootstrap retires its port.

There is also a concrete fail-open successor state.  After those fields are
dropped, `Welcome.None` still marks the transport live at
[`backbone-worker:2952-2959`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2952>), and `Welcome.Tail` installs a tail request at
[`backbone-worker:2961-2966`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2961>) despite no retained verified pair from which that tail can be
applied.  Neither result is a valid recovery after `RebootstrapRequired`.

The narrow repair is a distinct `DocumentRebootstrapOwner` (or an explicitly
separate phase of the existing Bootstrap owner) retained on `ArtifactState`
and armed **before** closing the old socket.  It must capture the exact state
identity, runtime key, client instance, config/binding selection, execution
lease/open generation, and an incrementing rebootstrap epoch.  Its single
overall timer rechecks that retained owner/epoch before emitting one
`deadline-exceeded` bootstrap failure and closing only the socket that is
current for that epoch.  It is cleared by document close/config replacement,
an explicit newer rebootstrap, or only after an authenticated new canonical
pair has installed; it must not clear on socket open, Hello, or a bare
Welcome.  `None` and `Tail` while this recovery owner exists and no pair is
present must be rejected rather than transition to live.

Required worker rows are: rebootstrap then no Welcome reaches one terminal
failure; an old epoch timer cannot close a successor that receives and installs
a pair; `None` and `Tail` after pair erasure are refused; a valid replacement
pair clears the timer; and close/config replacement clears the timer without a
post-close event.  This conclusion is source-only; no worker runtime was run
for this audit.

## Plugin Runtime Disposal Must Retain Pending Opens

The current raw runtime `dispose` is not a physical retirement boundary.
`createApp` publishes `actorIdByInstance` before awaiting activation
([`PluginRuntime:1257-1262`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1257>)), then captures the lifecycle and performs guest open only after
that await ([`PluginRuntime:1263-1275`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1263>)).  Meanwhile, `dispose` starts unretained
`destroyApp` tasks for the current map and immediately completes the outcome
broadcast ([`PluginRuntime:1301-1304`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1301>)).
Disposal during activation can therefore remove the provisional actor, then
let the suspended create path resume, open a guest instance, and return it
after the handle's outcomes are already terminal.

`ActorDocumentBindingV1` itself has the right local sequencing: its port
retirement waits for the bind promise, then performs the exact retire exchange
([`backbone binding`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📡️backbone/🟦️.ts:168>)).  The outer runtime must not bypass that
ownership by closing its observable handle first.

The smallest sound refactor changes the kernel and rich handle `dispose`
contracts from synchronous void to one idempotent `Promise<void>`
([`kernel handle`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:218>),
[`rich adapter`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1596>)).  It needs a synchronous Closing bit and a retained
`disposePromise`, plus an `InstanceOpenOwner` captured before activation.
After every asynchronous create step, that owner must prove it remains current;
if invalidated before an open was sent it only disposes the actor, while an
already-issued open schedules its one exact retirement after its own attempt
settles.  Per-instance `retirementByInstance` must return the same promise to
concurrent destroy/dispose calls.  Only after every opening and retirement
owner is settled may the raw layer complete `turnOutcomes`; the rich adapter
also disposes its `AppChannelClient`s exactly once around the same retained
shutdown promise.

Required regression rows: paused activation then dispose, released activation
with no guest open/returned instance; paused bind or retire, with terminal
outcomes delayed until the exact retire settles; duplicate dispose/destroy
sharing one physical close; post-close create/bind/enqueue refusal; and a
retired old port unable to send to a successor.  React cleanup can initiate the
promise without awaiting it, but must retain it as the handle's terminal
lifetime rather than describe that initiation as completed disposal.

### Lifecycle Close Is a Required Precondition, Not a Best-Effort Cleanup

The new ShardClient guard makes the missing composition observable:
`disposeActivation` rejects while an instance or close owner remains
([`shard-client:1949-1952`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1949>)).  A real instance must therefore use its existing
`ShardInstanceLifecycleLease`, whose exact lawful phases are already pinned by
the lifecycle scheduler fixture
([`lifecycle-scheduler.json:6-8`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧪️fixtures/⏱️lifecycle-scheduler.json:6>)):

1. Retire the document port while its captured operational activation still
   exists.  `beginClose` increments the operation generation, so binding
   retirement after that point cannot lawfully exchange its exact guest
   `retire` command.
2. Call `lease.beginClose()` synchronously, which blocks new ordinary actor
   work but preserves the separately captured close authority.
3. Submit lifecycle `close`, acknowledge `Accepted`, then obtain `Retired`.
4. Begin and drain the exact host `OwnedUiInstance`; only its retirement
   witness may acknowledge `Retired`.
5. Once the lease reports `complete`, call `lease.dispose()` / ShardClient
   disposal.

The host proof is mandatory: a retired acknowledgement rejects unless it
matches a host bound to the exact lease/lifetime and that host is fully retired
([`shard-client:1515-1522`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1515>)).  Production PluginRuntime currently has no
`bindHostRetirement` call (only tests do); its separate `retainedWindowByActor`
map cannot be replaced by a freshly created empty close-time owner.  The
lifecycle capture/open path must instead bind an owner that represents the
actual retained UI descendants, and closing must drain that same owner.

There is an independent return barrier.  `activation.returned` prevents both
lifecycle turns and final actor disposal
([`shard-client:1724-1730`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1724>),
[`shard-client:1949-1952`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1949>)).  Although `lease.pendingReturn` exposes the exact
return owner, source audit found no production terminal release that clears
`activation.returned`: accepting a return merely sets `state.retired`
([`shard-client:1700-1702`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1700>)).  A disposition cannot invent a replacement response
owner.  It must retain the original return supervisor, cancel/poll and
physically retire its exact response/page/content, then use a narrowly owned
empty-proof release that clears the activation return slot.  Until that exists,
async dispose must correctly remain pending/fail rather than remove maps or
pretend the actor was released.

### Production UI Ownership Is Not Yet Connected to the Lifecycle Witness

There is no production construction of `OwnedUiInstance`: a source-wide
`new OwnedUiInstance` search finds the owner implementation and test suites
only.  Plugin Runtime instead retains a separate plain
`Map<string, UiDocumentState>` in
[`PluginRuntime:1058-1125`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1058>).
Its render path decodes guest patch operations and applies them to that plain
map at
[`PluginRuntime:1519-1521`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1519>),
while `patchAckEvents` constructs generic acknowledgements.  Consequently it
does not use `lease.captureUiPatchAuthority`, `OwnedUiInstance.beginPatch`,
input-retirement, or `lease.submitUiAcknowledgement`.  Merely binding a new
empty host at destruction would therefore certify a different object from the
one that actually retained/rendered guest UI.

The smallest non-synthetic integration starts immediately after the captured
lifecycle receipt was acknowledged and its exact guest lifetime is available:
construct one `OwnedUiInstance` with the captured lifecycle activation,
lifetime, selected document limits and host profile, then call
`lease.bindHostRetirement(owner)`.  The runtime must retain that same owner
per instance.  Each actual guest `uiPatch` is admitted through
`lease.captureUiPatchAuthority(turn, index)`, its matching instance surface
and `owner.beginPatch`; its input receipt is released and its exact
acknowledgement submitted through the lease.  The renderer must read the
owner's issued `OwnedUiInstanceSurface` rather than maintain an independent
plain map.  The existing `useOwnedUiView`/`useOwnedUiNode` consumers in the
UiDocumentStore tests demonstrate the intended read surface without a second
state projection.

That makes the real close sequence feasible: first retire the document port
under the still-live operational activation; then `lease.beginClose`, guest
close/Accepted acknowledgement, `owner.beginClose` and bounded
`owner.closeStep`, extract its sole witness, acknowledge Retired, and only
then dispose the completed lease.  The exact owner and witness APIs are at
[`instance owner:221-289`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts:221>)
and the lifecycle identity gate is at
[`shard-client:1515-1523`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1515>).
An open cancelled before its captured receipt needs a separate pre-open
reservation reconciliation; the host cannot be constructed before that
receipt defines the guest lifetime.

### Captured Return Still Has No Physical Terminal Release

No existing return-discharge primitive can clear `activation.returned`.
`OwnedShardReturn` exposes execute/retry/poll/cancel and accessors only
([`shard-client:683-707`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:683>));
its page has no release method
([`shard-client:709-729`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:709>)).
`OwnedActorTurnOutputs` only marks itself closed at
[`output:156`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:156>),
without a roster close/terminal witness.  Likewise return content and fields
only expose `beginClose`, not a physical close progression
([`return content:47-97`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts:47>)).

The required owner is a private, original-return supervisor retained by the
same `ShardInstanceOwner`, not a synthetic replacement.  It must reject a
release while a request is in flight, a result is retryable/refused/faulted,
a page/content/input is retained, or any response roster/cell/record/facade
is non-terminal.  It drives only the original `OwnedShardReturn` through
cancel/poll until the actor supplies its `retired` result, then physically
drains those exact page, content/field, and response roots under their
existing resident accounting.  Only after all those conditions and the exact
`state.retired` are true may a private compare-and-clear remove
`activation.returned`.  `acceptCapturedReturn` currently only records
`state.retired = true`
([`shard-client:1700-1703`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1700>));
both lifecycle sends and actor disposal intentionally refuse while the slot is
non-null ([`1724-1730`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1724>),
[`1949-1952`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1949>)).

This remains source-only.  The focused disposal result currently exercises a
fake shard sequence and does not establish real host/return retirement.

### Audit: Empty Actor-Output Roster Retirement

The new `OwnedActorTurnOutputs.closeStep` is appropriately narrow.  It
refuses pending, response-bearing, returned, and faulted slots before touching
their roots ([`output:164-171`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:164>)).
Only a reserved unused slot is first changed to `cancelled`; its facade/owner
links are detached before the exact resident record is detached, retired and
then unlinked ([`174-202`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:174>)).
The record detachment is tied to the original queue, and the final predicate
retains every admission/root/fault field until it is empty
([`191-204`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:191>)).

Source review found no lost-prefix, stale-record-alias, or raw-root discharge
in that narrow path.  In particular, a formerly reserved external output
facade becomes non-matching before record detachment, but it has no outcome,
response, or fault and therefore cannot expose a returned root; a returned
facade instead remains the head and blocks at the first guard.  The new
fixture correctly exercises unused admission prefixes and separately checks
pending/returned/faulted roots remain retained.  It does not, and must not,
qualify the still-missing page/content/returned-data discharge or final
`activation.returned` release.  No test result is claimed here; the requested
runner was still blocked in graph construction at review time.

### P0: Rebootstrap Fences Inbound Tail but Not Local Raw Outbound Batches

The fresh rebootstrap watchdog correctly captures a document/config/runtime/
client/Hub selection owner at
[`backbone-worker:2744-2790`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2744>) and old timers cannot close a replacement owner; the focused watchdog fixture at
[`space-artifact-creation-owner:733-858`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:733>) exercises that successor fence.  This is source-level controlled evidence only.

It does not fence all mutation traffic.  `captureArtifactRebootstrapOwner`
sets `artifactRebootstrapRequired = true` before
`requireArtifactRebootstrap` awaits retirement of the folder canonical mirror
([`backbone-worker:2785-2789`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2785>),
[`2911-2918`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2911>)).  A local bound-port message can concurrently reach
`admitLocalMutations` and `relayMutationsToHub`
([`4927-4958`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4927>),
[`2485-2532`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2485>)).  That relay checks only read-only authority, actor readiness and socket openness—not bootstrap, rebootstrap or catch-up state—so it can publish through the still-live old actor before the old pair is dropped.

There is a second, independently reachable bypass.  A fresh authenticated
`Session` unconditionally removes and relays the outbox at
[`3239-3244`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3239>),
although the preceding `Welcome` starts a non-inline artifact transfer without
awaiting its completion ([`3053-3076`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3053>).  The inbound `Commands` path rejects that same
window ([`3176-3186`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3176>)); outbound delivery currently does not.  The existing
`finishCatchupIfReady` is the right final release point, but its guarantee is
undermined by the direct Session flush.

An in-flight old-socket `Ack` is also not fenced by
`artifactRebootstrapRequired`: it can run after the asynchronous control
handler yields but before the close event updates `state.socket`, because the
Ack branch only tests bootstrap/required-tail
([`3189-3196`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3189>).  It can therefore release the exact raw-byte ledger and update the
frontier during a transition.  The correct transition entry must synchronously
move `pendingBatches` to the retained outbox before its first await, and must
ignore old control-plane terminal frames thereafter; retrying the same
mutation identifiers after the verified new pair is the already-established
lost-Ack reconciliation shape.

The smallest safe repair is one `flushOutboxWhenLive` predicate used by every
outbound site.  It must require: authenticated actor; open current socket;
`artifactBootstrap === null`; `artifactRebootstrapRequired === false`;
`requiredTailFrontier === null`; and a verified installed pair/current
frontier.  `relayMutationsToHub` queues rather than sends while that predicate
is false, and `Session` never splices the queue directly.  `finishCatchupIfReady`
is then the only normal post-bootstrap release.  New raw `documentBackbone`
ingress while rebootstrap/catch-up is pending must be rejected *before*
`exactLocalEnvelopes` is allocated: the current wire message carries no
binding-generation capable of proving it belongs to the future bound guest
port.  Already-admitted exact bytes remain retained in the outbox for exactly
one post-catch-up replay; they must not be discarded or re-encoded.

Required controlled rows belong with the existing worker fixture above: (1)
RebootstrapRequired paused at mirror retirement plus a local raw message:
zero Commands send and no byte-ledger admission; (2) a sent batch followed by
control/requeue, then a fresh Session before Welcome/Bootstrap: zero send;
(3) a non-inline bootstrap followed by Session and a local raw message: no
send/no new admission; (4) bootstrap completion plus exact tail equality:
one send of only the original retained raw batch; (5) delayed old-socket Ack
after control: no ledger release/frontier mutation.  These rows qualify the
TS fallback path only; they do not establish the WASM worker or a browser
runtime result.

### Audit: Narrow Never-Executed Return Retirement

The new `ShardInstanceLifecycleLease.retireUnusedReturn` is scoped correctly
to an original return that has never acquired domain output.  It blocks an
in-flight request and every origin, identity, event, page, content, retry or
fault root before it begins detachment
([`shard-client:1626-1633`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1626>)).  It first drives the original
output roster empty, then clears `state.instance`, `state.client`, and
`state.facade`, while the exact `activation.returned === state` relation is
still held.  Only after the original cell/record retirement proof does it
compare that relation and clear `activation.returned`
([`1641-1662`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1641>)).

That order preserves identity continuity: while the state is being detached,
the activation still owns the same source and `returnPhase` is `closing`, so a
new admission refuses.  A previously captured `OwnedShardReturn` has no
capability after detachment—its public reserve/submit routes both require the
cleared `state.client` ([`OwnedShardReturn:700-709`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:700>)).  The new
13-prefix test checks exact resident refund, stale-facade refusal, and that a
later admission returns a distinct source
([`reserved-response-settlement:716-754`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts:716>)).

No lost-return or stale-alias defect was found in this narrow primitive.  It
must remain unavailable for returned, page-owned, content-owned, retryable,
faulted, or executing sources; those roots still need the original-return
supervisor described above.  The test/source state is not a native lifecycle
qualification.

### P0: Plugin Runtime Has Two Pre-Owner Disposal Windows

The new composition correctly preserves the exact lifecycle turn in
`coerceTurnResult`, mints an `OwnedUiInstance` from the captured lifetime,
binds it through `lease.bindHostRetirement`, and routes a UI acknowledgement
through `lease.captureUiPatchAuthority`/
`submitUiAcknowledgement` rather than the former generic patch event
([`PluginRuntime:1187-1238`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1187>),
[`1400-1421`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1400>)).  `settlePluginTurn` respects that special
acceptance result and does not add a second generic patch acknowledgement
([`1008-1016`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1008>)).

But `createApp` checks cancellation immediately after `registry.activate`,
before it captures a lifecycle lease, and again immediately after its native
open turn, before it mints/binds the actual host owner
([`1398-1413`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1398>)).  `destroyApp` first makes that
check fail, waits for the opening promise, then throws
`plugin-ui.native-owner-required` unless both the lifecycle and host owner
exist ([`1435-1446`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1435>)).  The actor id, closing bit and
partial activation maps consequently remain live, and the outer create error
path recursively observes the same failed retirement rather than reconciling
the exact stage.

The remedy needs a retained per-instance opening stage, not an empty
close-time UI owner.  A cancellation before lifecycle capture must use only a
pre-lifecycle activation reservation rollback.  Once the open request has
been issued and its captured receipt is returned, the code must first mint and
bind the *real* owner from that receipt, then honor cancellation by the normal
document-port retire → `lease.beginClose` → Accepted → owner witness → Retired
sequence.  Every stage must converge by deleting its exact maps and closing
bit only after its own release succeeds.  Required focused rows pause: (1)
after activate/before capture; (2) after capture/before native open completes;
(3) after captured open/before host bind; and (4) activation failure.  Each
must prove one cleanup, no retained actor/closing entry, no guest work after
close, and no synthetic owner.

`destroyApp` does issue `documentPort.retire()` before `lease.beginClose`, and
the port uses its captured activation for the exchange rather than its public
current predicate ([`backbone binding:130-174`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📡️backbone/🟦️.ts:130>),
[`PluginRuntime:1437-1446`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1437>)); that intended ordering is sound once
the pre-owner stage is reconciled.

### P0: Retirement Witness Still Precedes the Actual Presentation Root

Although patch admission now has a real host owner, it also maintains a
parallel `retainedWindowByActor` `UiDocumentState` projection and refreshes
from that map ([`PluginRuntime:1198-1205`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1198>),
[`1494-1523`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1494>)).  The exact owner surface is instead put in
`uiSurfaceByInstance`, which has no production reader.  Lifecycle close
obtains and submits the host retirement witness before `destroyApp` later
deletes the parallel map ([`1240-1282`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1240>),
[`1447-1458`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1447>)).  The witness therefore does not prove retirement of the
state actually driving presentation.

The agreed correction is to use the exact owned surface as the sole
production presentation source.  `refreshUi` may create a transient
`BuiltNode` response only while the exact instance/lease is current; it must
not cache a plain `UiDocumentState`.  All subscriptions/reads used for that
projection must be acknowledged, unsubscribed and terminal before
`owner.closeStep`/Retired.  A stale asynchronous read is discarded, not
published.  The old map/helpers may remain test-only, but cannot be a
production cache that survives the witness.  All statements in this section
are source audit; the real-authority fake-worker target was still launching.

### Supersession: Owned Presentation and Opening-Stage Cleanup

The two preceding PluginRuntime P0s are no longer accurate on the current
source frontier.  Production `refreshUi` now projects from the exact
`OwnedUiInstanceSurface` collection, subscribes/acknowledges/unsubscribes each
read, checks the exact `(actor, lease, owner)` owner before returning, and
tracks every in-flight read so close can wait for its retirement
([`PluginRuntime:1234-1318`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1234>)).
`retainedWindowByActor` and its patch helpers remain declared solely for
colocated test registration; no production call reaches them.  The retained
presentation map is cleared only as part of exact owner close, before the
sole retirement witness is extracted
([`PluginRuntime:1319-1354`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1319>)).

Opening cancellation is likewise stage-aware.  `createApp` records the
opening promise before external code can destroy the instance, captures the
lifecycle before issuing its open turn, and creates/binds the real
`OwnedUiInstance` before a subsequent cancellation check.  `destroyApp`
first marks the exact instance closing, waits the exact opening and port
retirement, and uses an activation-only rollback only when no lifecycle was
captured ([`PluginRuntime:1470-1548`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1470>)).
I found no remaining pre-owner leak in those paths.

The earlier Retired-witness retry concern is also superseded: lifecycle close
reads `lease.pendingReceipt` and retries accepted or retired receipts before
consulting a blocked progress state
([`PluginRuntime:1329-1351`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1329>)).
The requested wrapper-level refusal/retry test is useful coverage, but this
source audit does not find a retry-blocking branch.

These are source conclusions only; they do not qualify a real guest, shard,
WASM, or browser lifecycle.

### P1: Legacy `localMutations` Bypasses the Rebootstrap Admission Fence

The repaired raw `documentBackbone` route rejects ingress before parsing or
retaining bytes unless the current pair, tail, and rebootstrap owner are all
ready ([`backbone-worker:4987-5002`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4987>)).
It also requeues sent batches synchronously at rebootstrap entry and funnels
flush through the verified-current predicate
([`backbone-worker:2480-2498`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2480>),
[`2937-2956`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2937>)).

However, the still-exported `ArtifactActorMsg.localMutations` variant enters
`admitLocalMutations` directly, without `documentBackboneAdmissionReady`
([`OS wire:660-665`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:660>),
[`backbone-worker:5004-5006`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5004>)).
For a Hub-bound document this can append a newly supplied domain envelope to
`pendingMutations`/the outbox during rebootstrap or unresolved tail and later
relay it under the successor Session.  It does not violate the new exact-raw
ledger because it is the older domain route, but it violates the broader
"no new mutation admission before the verified current pair" rule.  The
current Shell use found is identity configuration, not the bound GIS path;
that is not a reason to leave the generic document-facing wire variant
unfenced.

Either reject `localMutations` for a Hub-bound document unless the same
admission predicate holds, or split it into an explicit local/identity-only
route and remove it from document ingress.  Add one controlled rebootstrap
row proving this variant leaves pending/outbox/ledger and Commands sends
unchanged.  This finding is source-only; it does not negate the reported
controlled raw-worker repair.

### Supersession: Legacy Local Mutation Admission

The owner subsequently put the Hub-bound `localMutations` branch behind the
same `documentBackboneAdmissionReady` predicate
([`backbone-worker:5004-5010`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5004>)).  Home reports a focused controlled
worker law for the canonical-pair gate.  This audit did not run that law, so
the qualification remains source-plus-owner-reported test evidence rather
than a browser or component-runtime result.

### P0: The WGPU Plugin Bridge Erases Receipt Authority and Has No Document Port

The materialized component shim itself preserves both WIT receipts: its
`createActorApi().poll` spreads the component result and replaces
`lifecycleReceipt` and `uiPatchReceipt` with the validated/canonical bridge
forms ([`plugin package:583-585`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:583>)).  The direct browser-actor path also treats those fields as authority: it checks the
captured lifecycle receipt before cold transfer
([`backbone-worker:1423-1460`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1423>)) and decodes the raw UI-patch receipt before an
acknowledged renderer result feeds back into the guest
([`1543-1627`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1543>)).

The parallel WGPU bridge takes a different path.  Its `submitTurn` calls the
generic `coerceTurnResult`, whose declared result contains only patches,
effects, wake, command ingress, and cold-pair status
([`wire-turn:58-75`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:58>)).  It creates a new object and drops the original turn,
`lifecycleReceipt`, and `uiPatchReceipt`.  `runQueuedTurn` then creates a
second aggregate of exactly those stripped fields and publishes UI patches to
a plain retained window projection
([`WGPU bridge:429-470`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:429>)).  The shipped
`frame-worker.js` contains the same stripping coercer
([`frame worker:23193-23197`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js:23193>)).

This is not merely a missing acknowledgement: `WgpuPluginHandle` explicitly
excludes transactions, backbone, and presence, and exposes neither
`loadAppDocumentPack` nor `bindDocumentPort`
([`WGPU bridge:372-387`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:372>)).  Therefore a WGPU producer/browser success cannot be offered as evidence for the
receipt-owned PluginRuntime/Shell path or the raw Backbone port.  It must
either be deliberately limited to its narrow rendering target, or use the
same retained receipt/owned-surface/document-port protocol; preserving the
bytes alone is insufficient.

Required focused coverage for any unification is: a real component turn with
both receipts reaches the renderer; exactly one matching lifecycle/UI ACK is
sent; a receipt for an old instance is rejected after replacement; and a
retire blocks both a later UI publication and raw-port send.  No existing WGPU
test establishes those facts.

### P1: Cold-Pair WIT Ingress Is Implemented in the Browser Actor, but Not in ShardClient/WGPU

The WIT export deliberately has a dedicated third poll argument and an
observable `cold-pair-ingress` result
([`plugin WIT:1122-1205`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1122>)); the Rust reactor refuses an attempted event-based bypass
([`turn:204-210`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:204>)).  The authenticated direct browser actor uses that
contract correctly: it passes each owned page as poll argument three, requires
the exact accepted cursor or final applied receipt, and wipes transferred
bytes ([`backbone-worker:1468-1510`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1468>)).

By contrast, the general ShardClient `turn` message and captured activation
only carry `events`, optional `commandPage`, and budget
([`shard-client:360-363`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:360>),
[`613-617`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:613>)).  The generated worker calls the materialized
`poll` with `undefined` for its third argument on every turn
([`plugin package:389-399`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:389>)).  It can parse a status produced by another path, but it cannot submit a
page.  Hence the WGPU bridge's `coldPairIngress` is necessarily `idle` for
its own turns and cannot prove cold materialization.

React Shell currently loads pack/spr on the separate AppChannel route
([`PluginRuntime:1922-1926`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1922>),
[`plugin Rust:28977-28989`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28977>)).  That route may be the intended ordinary
Shell load mechanism, but it is not evidence that the WIT cold-pair ingress
ran or reached `Applied`.  Acceptance must choose and state one authoritative
route: either wire an owned bounded page argument through ShardClient and
require the final exact `Applied` receipt, or keep AppChannel load as the
authoritative Shell mechanism and remove any claim that its test proves WIT
cold-pair ingress.  Do not merge the two paths implicitly.

### P1: AppChannel Publishes a Candidate Document Cache Before Load Confirmation

`AppChannelClient.loadDocument` stores caller-owned `pack` and `spr` into the
live cache before it sends `LoadDocument`
([`OS channel:3280-3287`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3280>)).  `PluginRuntime.loadAppDocumentPack` detects an
error frame only after that call returns
([`PluginRuntime:1922-1926`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1922>)).  A rejected load can therefore leave
`documentPack()` exposing bytes the guest never accepted.  This cache is used
by the transaction coordinator as its target snapshot
([`PluginRuntime:2218-2224`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:2218>)).

Keep the prior cache until a load response is conclusively non-error, then
replace it with a defensive owned copy (or with an echoed/read-back document
frame where the protocol supplies one).  Add a channel-level error-frame row
that proves `documentPack()` remains the previous accepted pair, followed by
a successful replacement row.  This is a source finding; no component or
browser test was run here.

### P0: The Mounted Browser Actor and Shell Action Actor Are Disconnected

The authenticated browser actor is the source actually rendered for a mounted
document.  The worker opens its own child at WIT instance `0`
([`backbone-worker:1417-1460`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1417>)), transfers the verified cold pair to that child, and
retains its patched `UiDocumentStore`.  Shell selects that store in preference
to its ordinary window UI when the document is mounted
([`ShellHost:8242-8271`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8242>)).

But an interpreted map action invokes the normal active Shell session's
`plugin.handleAction(instanceId, ...)`
([`ShellHost:5201-5211`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5201>)), which is the separate PluginRuntime/AppChannel actor.  The only renderer-to-worker
browser-actor messages are view-state and UI-patch-result
([`ShellHost:3585-3602`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3585>),
[`2183-2204`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2183>)).  Correspondingly the
browser actor only polls `surface-visible`, `wake`, lifecycle/patch feedback,
and cold pages; it has no action or raw Backbone delivery method
([`backbone-worker:1468-1627`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1468>)).

This leaves two separate guest states for the same mounted scope: an
initially-rendered browser actor and the actor that receives ordinary Shell
actions and document-port traffic.  Since the mounted actor store wins the
render selection, a successful action on the PluginRuntime side has no
defined incremental route to update the visible map actor.  A later bootstrap
may replace the cold pair, but that is not an action/result correlation and
cannot supply ordinary interactive convergence.

The smallest coherent choice is one owner, not a relay of unauthenticated UI
JSON: either bind the mounted browser actor as the actual document port/action
actor (with its captured lifetime, raw exact Backbone ingress, and action
receipt), or render the receipt-owned PluginRuntime surface and retire the
parallel browser actor for this route.  Required end-to-end rows must prove:
(1) a map UI action reaches the exact actor whose store is displayed;
(2) its mutation/action receipt flows through the bound raw port; (3) the
same actor receives the resulting current update and emits the next UI
revision; and (4) a stale sibling actor cannot alter the mounted store after
replacement.  The current cold/render fixture proves only initial mounting,
not this interaction loop.

### Decision: The Direct Browser Actor Cannot Yet Be Replaced by PluginRuntime

Adding a `coldPairPage` field to generic `ShardClient` would not make it an
equivalent replacement for the direct browser actor. The direct path is the
only current path with both document selection and authenticated actor
authority. `DocumentExecutionTargetLease.admitBrowserActor` captures the
server-issued socket actor/grant and exact open binding
([`backbone-worker:870-920`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:870>)); its reservation obtains the plan-selected browser
bundle, hashes it, validates child `describe` against the verified descriptor,
and refuses stale socket/lease/grant before opening WIT instance `0`
([`1290-1395`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1290>)). It then transfers the exact verified pair
with the third WIT poll argument ([`1468-1510`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1468>)).

The ordinary React `PluginRuntime` instead obtains a module URL from the
global development/extension source and opens a generic app instance. It has
no `DocumentExecutionTargetLease`, socket grant, selected browser-bundle
asset, or verified-component-derived activation source. Its attachment uses
the separate AppChannel `loadDocumentPair`/`bindDocumentBackbone` route
([`ShellHost:2042-2111`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2042>)), and its generated
component worker supplies `undefined` as poll argument three
([`plugin package:389-399`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:389>)). No current API turns the
lease's verified component bytes into a `ShardClient` activation.

There is already a schema-first action format: [`browser-bundle/action-handoff`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts:1>) carries the exact scope,
verified surface, execution version, activation generation, instance,
surface revision, and action sequence. A production-source search finds no
producer or consumer outside that module and its test. WIT already provides
the typed ingress points: `event.ui-intent` for UI interactions and
`event.message` for bounded endpoint payloads
([`plugin WIT:688-695`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:688>),
[`757-805`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:757>)).

The smallest complete same-owner route is to keep
`DocumentBrowserActorReservation` as the document actor, make the existing
action-handoff tuple select its live lifetime/instance, and have that
reservation invoke WIT `ui-intent` plus the existing bounded `message`
ingress. Exact raw document-backbone bytes must enter and leave that same
reservation under its current lease/socket/grant predicate; retirement must
synchronously refuse both action and raw ingress before child/lease teardown.
That completes the selected actor route without a JSON relay or a second
actor.

Replacing the direct child would require a new private lease-derived
`ShardClient` activation whose executable source, descriptor, scope, socket
actor/grant, and retirement all derive from the verified lease. Reusing
ordinary `loadPluginModule` is insufficient and weakens selected-component
proof. Neither route is runtime qualified by this audit.

### P1: AppChannel `loadDocument` Cache Is a Pre-Commit Candidate

The cache assignment still occurs before `LoadDocument` has a correlated
successful result ([`OS channel:3280-3287`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3280>)); `pumpOutcomes` rejects the pending caller on an
error before it can produce a successful frame set
([`3085-3115`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3085>)). It is a candidate cache, not an accepted document cache.

The narrow fix belongs in `AppChannelClient.loadDocument`: retain the prior
defensive pair until its exact waiter resolves without error, then store
copied request bytes unless a matching `AppFrame::Document` supplied accepted
bytes. On error, channel close, decode failure, or superseded waiter it must
leave the prior cache unchanged. The existing test owner is
[`tests/backbone-envelope-io`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts:788>), which already exercises this cache. A
language-neutral companion corpus should cover empty+error, accepted
A+error/close, accepted A+success B, and an accepted `Document` echo
overriding the candidate. No test ran in this audit.

### P0 Implementation Packet: Direct Actor Action and Raw Backbone Ownership

The current direct child opens WIT instance `0`
([`backbone-worker:1416-1445`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1416>)), but the new document-backbone binding contract rejects that valid WIT
instance at three layers: the TypeScript control and port owner require
`instanceId >= 1` ([`PluginRuntime backbone:23-30`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📡️backbone/🟦️.ts:23>),
[`203-221`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📡️backbone/🟦️.ts:203>)), the neutral
schema has minimum `1`, and native command decoding rejects zero
([`binding Rust:100-124`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🦀️.rs:100>)).  Make the shared domain range
`0..=u32::MAX`: zero is a real live WIT child here, not an absent-instance
sentinel.  The existing guest-side binding and raw delivery logic already
works for a live numeric instance without a different code path
([`plugin runtime:27617-27720`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27617>)).

The action handoff needs to carry the exact UI contract `UiIntent`, not an
app-command invocation. `UiDocumentStore.buildIntent` stamps the rendered
surface, revision, node, trigger, binding arguments/input, and its monotonic
sequence ([`UiDocumentStore:504-521`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx:504>)).  The native reactor accepts this as
`Event::UiIntent`, rejects a surface for another instance, enforces revision
staleness, and routes it through the ordinary typed dispatch path
([`reactor turn:218-230`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:218>),
[`654-700`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:654>)).  Converting it to Shell's action descriptor would discard precisely
the surface/revision evidence the direct actor needs.  The isolated
`browser-actor-action` schema should therefore be revised in place to own
bounded canonical intent bytes and the existing exact owner tuple; no
compatibility second payload is needed.

Add the resulting direct-action request/result variants to
`BackboneWorkerRequest`/`BackboneWorkerResponse` and the specialized strict
request/response parser arms in [`OS wire:721-875`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:721>).  They must join the existing direct
TypeScript-only dispatch exception next to UI patch/view-state
([`backbone-worker:205-248`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:205>)); otherwise an installed Rust host dispatches the unknown
request to Rust and the reservation never receives it.  The request must
also carry `clientInstanceId`, while the worker reconstructs the artifact
state and validates the lease/grant/current predicate; caller bytes are not
actor authority.

`BrowserActorChild` permits exactly one active invocation
([`child:93-102`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:93>)).  A direct action cannot call `poll` concurrently with
cold transfer, rendering, or patch feedback. `DocumentBrowserActorReservation`
therefore needs one owner-local serialized turn lane covering every child
invocation, a bounded action queue (or immediate refusal while occupied), and
current checks before and after every await. Its close path must first refuse
new action/raw admission, then close that lane and child.

Finally, direct child results currently inspect only lifecycle/cold/UI-patch
fields ([`backbone-worker:1255-1287`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1255>)); they do not drain `Effect::SendMessage`.
After the exact WIT `ui-intent` poll and before reporting success, consume
only `send-message → backbone` effects addressed to the reservation's exact
URI, reject a foreign URI or snapshot, and feed their unchanged raw bytes to
the existing `handleLocalMsg(documentBackbone)` admission path.  The guest
already handles the complementary WIT messages: `Shell{0}` controls binding,
and `Backbone{exact-uri}` receives hot raw bytes
([`reactor turn:309-328`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:309>)).  Reconcile the same result's UI-patch
receipt and feedback before acknowledging the action.

Minimum executable laws: direct-zero bind/bound/replay/retire; direct-zero
raw inbound; a canonical UI intent yielding the exact raw mutation batch and
the next accepted UI revision; wrong scope/surface/generation/sequence and
foreign URI denial; an action during cold/patch wait is bounded and cannot
interleave invokes; and retire prevents later action/raw admission and Hub
send. These are design/source recommendations, not executed evidence.

### Supersession: AppChannel Candidate Cache

The implementation is now source-repaired. `sendCommand` captures owned
candidate bytes in its exact waiter, and `captureDocumentFrames` replaces the
cache only for a correlated non-error reply containing `Done` or an actual
`Document` frame ([`OS channel:3109-3117`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3109>),
[`3144-3157`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3144>)).  Disposal also clears the cache. The preceding P1 is therefore
superseded as a source defect; this audit did not execute the newly added
qualification tests.

### Correction: Direct Instance Zero Is Now a TypeScript-Only Barrier

The earlier direct-action packet overstated the scope of the zero-instance
problem. The schema now permits `0` ([`binding schema:11-15`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧬️schema/🔣️.json:11>)); the neutral fixture has the
`instance-zero-first-bind` positive row ([`fixture:6-10`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧪️fixture/🔣️.json:6>)); and the native reducer explicitly decodes and
returns that zero owner ([`unit test:30-34`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧪️tests/🔬️unit-standalone/🦀️.rs:30>)).

Only the current TypeScript implementation excludes the authenticated direct
child: `validateDocumentBackboneControlV1` and
`ActorDocumentMessagePortV1` require `instanceId >= 1`
([`PluginRuntime backbone:23-30`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📡️backbone/🟦️.ts:23>),
[`203-221`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📡️backbone/🟦️.ts:203>)). The canonicalized shared binding module should instead
admit `0..=u32::MAX` consistently. Its TypeScript law needs to execute the
existing zero fixture through encode, decode, bind, and retire, rather than
adding another schema variant.

### P0: Displayed Direct UI Still Sends Actions Through the Wrong Contract

The direct worker UI is accepted and identity-bound before it becomes
displayable: patch handling creates the per-runtime `UiDocumentStore` only
after scope/client/surface validation, then records a mounted identity only
after an acknowledged revision
([`ShellHost:2183-2225`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2183>)).
But the selected `browserActorStore` is rendered with `onIntentStable`
([`ShellHost:8242-8271`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8242>)), which calls
`uiIntentToActionDescriptor` ([`5283-5288`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5283>)). That conversion discards the
original UI surface, revision, node key, trigger, and monotonic sequence,
then directs the request into the unrelated normal Shell action path.

The direct store must instead receive an owner-captured callback that posts
the new strict worker `dispatchIntent` request. It must capture the exact
mounted record (`runtimeKey`, `scope`, `clientInstanceId`, activation
generation, instance `0`, verified surface id, and acknowledged UI revision)
and recheck that record immediately before worker admission. The regular
Shell store may continue using `onIntentStable`; this is not a global action
API migration.

### P0: Canonical Direct `UiIntent` Wire Must Preserve `seq` and Native Surface

`UiDocumentStore.buildIntent` produces a JavaScript `bigint` sequence
([`UiDocumentStore:504-521`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx:504>)), while native
`UiIntent.seq` is a `u64` ([`ui action:1494-1516`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:1494>)). The direct encoder must
encode it with `packUInt(intent.seq)`, not JSON serialization or a JS number.
It must keep every other `UiIntent` field intact.

The rendered store surface is the exposed window id (for example `map`), but
the reactor accepts intent and patch identity only under the internal
`"<instance>:<surface>"` format
([`reactor pending:367-372`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs:367>),
[`turn:218-226`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:218>)). For the direct child the sole lawful
translation is `map` → `0:map`, performed after confirming the original
surface equals the captured mounted window/surface. Arbitrary prefixed
surfaces and a surface already containing `:` must be refused; otherwise the
reactor's `parse_surface_instance` check could be satisfied for a different
tree. The request schema should contain the original exposed intent bytes or
its typed fields, not a pre-authorized arbitrary native surface.

The minimal law is one rendered `map` intent with a sequence above
`2^53-1`, verifying the exact Pack `UInt`, mapped `0:map`, current revision,
and instance `0` at the guest boundary; hostile rows cover fractional/string
sequence, `1:map`, `0:other`, stale mounted revision, and replacement client.

### Cache Review: Current AppChannel Patch Is Narrowly Correct

The six-row neutral fixture is at
[`os fixtures/document-cache`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📦️document-cache/🔣️.json), consumed by the
`backbone-envelope-io` law
([`1045-1099`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts:1045>)). The current source snapshots LoadDocument's
two input arrays into its waiter ([`OS channel:3171-3184`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3171>)); only the correlated reply is scanned
([`3109-3117`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3109>)); cache replacement is refused on error/disposal, requires `Done` for the
candidate, and lets an exact returned `Document` supersede it
([`3144-3157`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3144>)). Returned cache bytes are copied. I found no new cache
publication defect within that stated one-command/reply boundary.

The corpus does not establish per-command attribution for a bare
`TurnOutcome.error`: the kernel outcome type has only `{instanceId,error}`
([`kernel:156`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:156>)), and AppChannel currently rejects the oldest pending waiter
([`OS channel:3088-3093`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3088>)). That is a pre-existing channel-wide error model,
not a regression introduced by the cache fix; without a sequence in the
underlying error it cannot be honestly repaired as command-specific cache
semantics. The cache law should therefore not claim concurrent error
attribution proof.

### Supersession: Canonical Binding Now Admits Direct Instance Zero

The immediately preceding correction described the pre-canonicalized
TypeScript copy. Current shared
[`plugin/backbone/binding TypeScript`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts:23>) now accepts
`instanceId >= 0` in both the control validator and the message-port owner
([`203-221`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts:203>)). It agrees with the existing schema, fixture, and
native reducer. The zero-range source defect is therefore superseded. The
remaining obligation is an integration law that has the actual authenticated
`DocumentBrowserActorReservation` bind, receive, and retire instance `0`;
the existing isolated binding fixture alone cannot prove that route.

### P0: Direct Browser Reservation Still Bypasses Guest Lifecycle Retirement

The authenticated child does not currently shut down through the same guest
lifecycle it opened.  `openGuest` sends `instance-open`, captures the receipt,
and acknowledges it ([`backbone-worker:1414-1465`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1414>)), but
`DocumentBrowserActorReservation.close` immediately sets `lifetime = null`,
calls `child.close()`, and releases the state slot
([`1640-1662`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1640>)).  It sends neither `instance-close` nor an
exact lifecycle acknowledgement, and it never observes a `Retired` receipt.

That is not equivalent to the native lifecycle contract.  A normal close first
needs the exact `Accepted` acknowledgement and then a `Retired` acknowledgement
after the guest proves its native owner empty; the native law retains the
structural owner until that final acknowledgement
([`reactor lifetime runtime law:36-61`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs:36>)).
The neutral lifecycle fixture also explicitly says a retired receipt alone does
not permit host disposal ([`fixture:31`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧫️fixture/🔣️.json:31>)).

The direct route therefore needs a retained asynchronous close owner.  Its
synchronous entry must first mark the reservation closing and refuse all new
cold/render/action/raw work.  The sole child poll lane then performs, in order:

1. Exact document-backbone `retire` control when it was Bound.
2. `instance-close` for the captured lifetime; receive and exactly acknowledge
   the close receipt.
3. Bounded empty-driving polls until the matching `Retired` receipt, then its
   exact final acknowledgement.
4. Only then clear the lifetime, close the child, and remove the reservation.

An abort or deadline can force-kill an isolated child, but must be recorded as
unconfirmed forced termination rather than a retirement proof.  The required
controlled direct-child law pauses each receipt boundary, proves no later
action/raw/cold invoke reaches the child after close admission, and distinguishes
the forced path from a successful `Retired` witness.  This is source evidence;
no browser-child runtime law was run in this audit.

### P1: Keep Outer Action Correlation Separate From `UiIntent.seq`

`UiIntent.seq` is a `u64`, whereas the new handoff's `actionSequence` decoder is
a JavaScript safe integer ([`action handoff:70-79`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts:70>)).
The latter must remain a distinct, bounded owner-local worker request counter;
it must never be formed with `Number(intent.seq)`.  The actual intent body uses
`packUInt(intent.seq)`.  Add a direct child law with `seq = 2^53 + 1`, outer
`actionSequence = 1`, and a byte-exact guest-side decode, so these two identities
cannot be silently conflated.

### P0: Direct Worker Must Bind the Nested Intent Bytes Before Acknowledging

The new producer is sound as a producer: it maps only the captured exposed
surface to `"<instance>:<window>"` and uses `packUInt(intent.seq)`
([`intent helper:12-30`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧭️intent/🟦️.ts:12>)).  The outer
handoff parser, however, deliberately validates only the bounded byte-array
shape ([`handoff parser:60-87`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts:60>)).
That is insufficient at worker admission: worker input is an untrusted
structured-clone value, and the reactor silently ignores a malformed Pack
intent instead of returning a distinct action failure
([`reactor turn:218-228`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:218>)).

Before the reservation puts a request on its sole poll lane, it must strict
decode the nested Pack body, require canonical re-encoding byte equality, and
then require all of the following against the captured reservation owner:

- body surface is exactly `"0:<captured-window-kind>"`, not merely a prefix;
- body revision equals the outer/mounted `surfaceRevision`;
- node and action-version have their declared unsigned bounds; and
- body `seq` is an actual unsigned `u64` Pack carrier, not a rounded number.

Only then may the result be reported as acknowledged.  Add hostile direct
worker rows for trailing/noncanonical body bytes, malformed Pack, `1:map`,
`0:other`, stale revision, and a number-form sequence.  The existing producer
law already covers the maximum valid `u64`; these are admission laws, not a
second encoder.

### Update: Direct Reservation Has Acknowledged Lifecycle Retirement, But Not Yet Full-Turn Serialization

The earlier lifecycle-bypass finding is superseded in current source.  The
reservation now has a `retireGuest` path which retires the document binding,
sends `instance-close`, accepts the exact `Accepted` receipt, boundedly drives
to `Retired`, and sends the final acknowledgement
([`backbone-worker:1915-1962`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1915>)).  Importantly, a child that has
been allocated but has no captured lifetime now reports `unconfirmed`, rather
than falsely claiming `retired`
([`1988-1995`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1988>)).  This is source evidence only; no browser-child
runtime qualification was performed.

The new `turnTail`/`enqueueTurn` is currently only declared
([`1379-1408`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1379>)); it has no call site.  `invokePoll` consequently
serializes an individual `reactor.poll`, then releases `pollTail` before
`driveTurnResult` has consumed the returned effects or waited for its required
UI-patch acknowledgement ([`1896-1912`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1896>)).  A peer `Commands` delivery
([`3542-3561`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3542>)) or a host-view refresh can therefore invoke the
same child while a prior turn is awaiting `patch-ack`.  The action path happens
to reject while `pendingUiPatch` is set, but raw ingress and render are not
subject to that guard.

Make the *whole turn*, rather than each individual poll, the unit of the
reservation lane: poll, exact effect admission, patch offer/settlement,
feedback, and any `more-work` continuation must remain inside one
`enqueueTurn` operation.  The port may retain bounded raw ingress while that
operation is waiting, but it must not start another guest turn.  Close must
synchronously refuse admission and reject the outstanding patch offer, then
enqueue binding retirement and instance retirement behind the current full
turn.  A direct-child law should pause after a patch offer, inject peer
Commands and a view refresh, and prove neither reaches the guest until the
matching patch feedback completes.

### P0: Do Not Convert Failed Direct Effect Admission Into An Acknowledged Action

`routeTurnEffects` waits for `localEffectTail`
([`backbone-worker:1448-1475`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1448>)), but the binding sender replaces that
tail with `work.then(() => {}, () => {})`
([`1487-1492`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1487>)).  A rejection from `handleLocalMsg` is thus swallowed;
the turn can return an `acknowledged` action disposition even though its exact
outbound backbone mutations were not admitted.

Keep the rejecting work as the tail through the current turn and make an
admission failure terminal for that retained reservation (the closure sequence
then reports only its real `Retired` witness or `unconfirmed`).  Do not retain
a successful action disposition after guest state has emitted a payload the
worker failed to admit.  Add a controlled `handleLocalMsg` rejection law that
asserts no action acknowledgement, no next guest turn, and no forwarding to a
successor binding.

### P1: Bound The Socket-Facing Side Of Delayed Direct Ingress

`handleHubFrame` currently awaits `reservation.receiveBackbone`
([`3550-3557`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3550>)); a direct delivery can legitimately wait for a
host UI acknowledgement.  Although `ActorDocumentMessagePortV1` bounds its
own queued copies ([`252-265`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts:252>)), blocking the
websocket frame chain first leaves later browser message events outside that
ledger.

After full-turn admission exists, `Commands` handling should synchronously
validate/copy into the bounded port queue, attach a terminal failure handler to
the retained owner, and advance the frame frontier without awaiting guest
execution.  Queue-full, stale, or delivery failure must fail the exact current
document/rebootstrap path; it must never fall back to the generic
`documentBackbone` event.  A law needs a paused patch, more than the port
capacity of peer batches, and verifies a bounded failure rather than an
unbounded websocket-chain backlog.

### P0: Native Actor Ready-to-Idle Handoff Can Lose I/O Readiness

The retained native actor’s `Poll::Pending` path is correctly ordered: it
stores its future, releases `scheduled`, then consumes `wake_requested`
([`sync:3022-3026`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3022>)).  The corresponding completed-turn path is not.  For
both `MoreWork` and `Idle`, it consumes `wake_requested` while the runner is
still marked scheduled, then releases that mark
([`sync:3051-3059`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3051>)).

An `ArtifactReadinessWake` from the retained Hub read or connect future can
therefore occur between those two atomics.  It records its flag, fails to
enqueue because `scheduled` is still true, and is then missed by the earlier
exchange.  If the outcome is `Idle { deadline: None }`, no timer repairs the
loss.  Release `scheduled` before consuming the wake flag; then a wake either
owns the new job or is observed and schedules one.  Add a deterministic native
law that pauses exactly at this handoff, invokes a socket/connect Waker, and
requires one subsequent poll.  The existing retained-turn fixture is the
right colocated target
([`native actor fixtures`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️native-actor-retained-turn-fixtures/🦀️.rs:39>)).

### P0: Any Idle Wake Currently Defeats Native Reconnect Backoff

`schedule_reconnect` records a future `reconnect_at`
([`sync:2177-2183`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2177>)), but
`start_connect_hub` has no equivalent future-deadline refusal
([`sync:2077-2103`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2077>)).
The next finite-turn cycle begins at `Connect`, whereas only the later
`Reconnect` phase clears an expired deadline
([`sync:1729-1778`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1729>)).  Thus an ordinary mailbox wake, filesystem wake, or
late socket readiness wake before the timer can start a new connection early.

Refuse `start_connect_hub` while `reconnect_at > Instant::now()`; the
`Reconnect` phase remains the only place that clears the deadline and starts
the attempt.  A native law should schedule a reconnect, inject an unrelated
mailbox/readiness wake before the deadline, complete one full phase rotation,
and prove no admission/socket attempt occurs until the deadline turn.

The stable readiness closure itself is appropriately weak and actor-lifetime
scoped: it upgrades a `Weak<ActorRunner>` and calls `schedule`
([`sync:3258-3266`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3258>)); terminal and complete runners refuse
the request ([`sync:2920-2926`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2920>)), and final close removes the
self-retain ([`sync:3168-3179`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3168>)).  It therefore does not revive a
cancelled/completed actor.  It should *not* instead capture the runner’s
per-turn generation: an I/O registration valid across phase rotations would
then discard legitimate readiness.  With the start guard above, an already
queued old source wake is harmless scheduling work, not stale connection
admission.  Native compilation was active during this source audit; neither
repair is runtime-qualified here.

### P0: Cancellation Must Not Start A Second Terminal Runner Beside An Active Turn

`ActorRunner::cancel` calls `begin_terminal`, forcibly clears `scheduled`, and
then enqueues a terminal close job
([`sync:3121-3125`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3121>)).  This is unsafe if the already scheduled
`run_job` has taken the `Actor` or `ActorTurnFuture` out of `turn` but has not
yet returned it.  `begin_terminal` sees no owner to transfer; the newly queued
terminal job can see no `terminal_turn`, conclude the runner is empty, and
clear `self_retained` ([`sync:2979-2988`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2979>),
[`3160-3179`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3160>)).  When the original job next observes terminal it
does transfer its local owner into `terminal_turn`, but its `enqueue(true)` is
then refused because the runner is already complete.  The owner is no longer
closed by the retained terminal progression.

Do not clear `scheduled` in `cancel` when it represents an active job.  After
`begin_terminal`, terminal work should be enqueued only when no job owns that
slot; an active job already has the terminal branch that returns its exact
local actor/future into terminal ownership and schedules one-close-at-a-time
drain.  Add a controlled in-flight future law: pause after the runner has
taken its owner, cancel from another thread, assert `complete == false` until
the original poll transfers the owner, then prove that exact owner is drained.
The existing cancellation test only covers a future still stored in `turn`
([`native actor fixture:53-76`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️native-actor-retained-turn-fixtures/🦀️.rs:53>)) and cannot expose this race.

### Update: Native Wake And Cancel Source Repairs Landed; Exercise The Real Enqueue Edge

Current source releases `scheduled` before consuming the flag through
`release_scheduled_after_turn_with`, guards a live Hub, live connect future, or
future reconnect deadline at `start_connect_hub`, and no longer clears
`scheduled` from `cancel`
([`sync:2077-2081`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2077>),
[`sync:3022-3056`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3022>),
[`sync:3125-3128`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3125>)).  These source changes directly address the
three preceding defects.

The new wake fixture currently writes `wake_requested` directly in its
after-release hook ([`native actor fixture:54-65`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️native-actor-retained-turn-fixtures/🦀️.rs:54>)).  It proves the
flag exchange, but not the real Waker property that matters: `request_wake`
must see released `scheduled`, acquire it, and enqueue exactly one successor.
Use a live controlled pool and invoke `request_wake(current_generation)` from
the hook (or use an actual `ArtifactReadinessWake`); assert one subsequent
poll/job.  Retain the separate in-flight-cancel law described above.  Native
compilation was still active, so this is a source-only re-audit.

### Update: In-Flight Cancellation Regression Exercises The Correct Interleaving

The newly added cancellation law holds the pool worker, manually runs a
future after `run_job` has taken it, calls `cancel` while `scheduled` remains
true, and proves no terminal owner or completion appears until that poll
returns `Pending`; only then does it release the queued terminal drain
([`native actor fixture:93-137`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️native-actor-retained-turn-fixtures/🦀️.rs:93>)).  This exercises
the previously missing active-local-owner race and matches the corrected
`cancel` ordering.  A drop sentinel would make physical future destruction
observable, but the retained `terminal_turn` and no-early-`complete` assertions
already detect the premature-finalization failure.  Execution remains pending
the active native compile.

### P0: Direct Browser-Actor Intents Reject Every Normal Native Emit And Cannot Run GIS Host Effects

The authenticated child path is not yet a complete replacement for the Shell
action path.  Every successful `UiIntent` is framed by native
`plugin_dispatch_intents` as `AppFrame::Emit` and then has its side effects and
events collected ([`plugin:28721-28753`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28721>)).  A rejected intent is similarly
framed as `AppFrame::Error`, rather than disappearing
([`plugin:28742`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28742>)).  The reactor routes both frames to
`Effect::SendMessage { Shell }`; only `UiPatch` receives its special retained
patch route ([`reactor turn:917-949`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:917>)).  It also admits each
requested host effect and turns every app event into `PublishEvent`
([`reactor turn:666-680`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:666>)).  All of those variants survive the
WIT turn-result boundary ([`reactor:1560-1595`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1560>)).

`DocumentBrowserActorReservation.routeTurnEffects` currently accepts only a
`send-message` effect aimed at the exact backbone URI.  A Shell target is
accepted only for the bind/retire control exchange (`shellReceipts=true`), not
for an ordinary turn ([`backbone worker:1512-1541`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1512>)).  `reconcileUiPatches` calls
that filter *before* it captures or offers the patch
([`backbone worker:1939-1950`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1939>)), and `dispatchIntent` acknowledges only after that
reconciliation returns ([`backbone worker:1649-1686`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1649>)).  Consequently even a successful
native intent reaches `foreign shell effect` on its mandatory `Emit`; a native
fault reaches the same rejection through `Error`.  It cannot truthfully report
an action acknowledgement today.

This is immediately material for the real GIS Map: `ProposeBoundsRegion`
emits `RequestInferenceProposal(GisMapBoundsRegion)`
([`GIS inference command:22-28`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💡️inference/🦀️.rs:22>)), while
`OpenSource` can emit `OpenExternalUrl`
([`GIS Shell command:22-27`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️shell/🦀️.rs:22>)).  Neither is a
backbone mutation.  There is no current `Emit::event` in the GIS Map command
tree, but the generic native event route is active and must not be silently
dropped when the direct path becomes the supported actor route.

The existing Shell has the required semantic interpreter, but it is wired only
to `PluginWasmHandle` action results: `applyHostEffects` owns effects such as
`OpenExternalUrl`, and its `RequestInferenceProposal` branch resolves the one
exact mounted document scope before opening/proposing through the worker
([`ShellHost:4255-4401`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4255>)).  In contrast,
`onBrowserActorIntent` only dispatches an intent and renders a generic failure
notice; it receives no actor-side frame/effect result
([`ShellHost:5312-5328`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5312>)).

The minimal truthful replacement is a schema-owned, owner-fenced direct-turn
output channel from the reservation to Shell, separate from the document
binding control receipt.  It must carry the exact captured document owner
tuple (scope, client instance, activation generation, instance `0`, surface,
and action sequence) plus ordered raw canonical `AppFrame` bytes, the complete
WIT `Effect` union, and `PublishEvent` records.  The Shell must first verify
that tuple is still current, then decode/project each normal `Emit`/`Error` and
admit each effect/event through the existing exact-owner host path.  A stale
or failed projection is terminal for that action and must never be replayed to
a successor.  Control exchanges must remain stricter: only their bound/retired
receipt is legal; a normal `AppFrame`, event, or backbone mutation there is a
protocol fault.

An action may be marked acknowledged only after (1) direct backbone admission,
(2) the matching Shell frames are projected, and (3) effects/events have been
admitted or explicitly terminally rejected.  This does **not** require waiting
for the eventual inference computation: `applyHostEffects` deliberately
acknowledges the host-owned inference-port admission by posting `inference-open`
and `inference-propose` ([`ShellHost:4391-4399`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4391>)); its durable
completion belongs to that retained port.  Required laws are: a real native
success with `Emit` + patch + backbone mutation; a real `Error`; GIS inference
effect admission; GIS external-URL admission; one `PublishEvent`; stale-owner
rejection without successor delivery; and control-frame/data mixing denial.
This is source audit only; no browser or native execution is claimed.

### P0: New Direct Host-Effect Publication Can Orphan A First Inference Operation

The landed publication vocabulary currently has exactly one supported member,
`requestInferenceProposal(gis-map-bounds-region)`
([`publication:1-51`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts:1>)).  However, the action result admits up to 64
canonical host-effect payloads ([`action handoff:67-81`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts:67>)), and the
publisher intentionally invokes its callback for every member
([`publication:53-61`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts:53>)).  Shell's current callback is a
single-owner inference opener: every invocation increments one
`inferencePortEpoch`, replaces `inferencePortOwnerRef`, and posts one
`inference-open` plus `inference-propose`
([`ShellHost:4256-4270`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4256>)).  Two valid duplicate effects from one guest
turn therefore start two operations while immediately losing the first
operation's retained owner; it cannot subsequently be cancelled, approved, or
closed by the UI owner.

Until host effects have per-effect request identities and independently
retained owners, this closed vocabulary must enforce *at most one* entry in
both the neutral schema and strict decoder (not merely a byte/count cap).  Add
a hostile two-identical-canonical-effect row and prove no worker inference
request is posted.  This concerns the new publication code; mandatory
Emit/Error/effect accumulation inside the worker remains an in-flight WGPU
change and is not characterized here as completed.

### P1: Intent Error Admission Is Broader Than The Native Producer

`decodeBrowserActorIntentPublicationV1` admits `AppFrame::Error` with either
`in_reply_to: null` or `0` ([`publication:28-38`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts:28>)).  The native
intent dispatcher creates its error using `push_app_fault(..., None, ...)`
([`plugin:28730-28743`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28730>)).  The direct route has no command sequence `0` to
support; accepting it blurs the strict separation between unsolicited intent
failure and a command-like reply.  Require `null` exactly and add the
canonical `Some(0)` rejection row beside the existing foreign-frame cases.

### P1: Worker Acknowledgement Still Precedes Shell Host-Effect Admission

The mailbox resolves an acknowledged result as soon as the worker disposition
arrives ([`action mailbox:44-60`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts:44>)).  Shell only subsequently calls
the exact-current publication routine in the promise continuation
([`ShellHost:5307-5331`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5307>)).  An owner replacement, missing identity, or
publication fault in that continuation therefore leaves a worker action result
whose `outcome: acknowledged` did not result in a Shell host-effect admission.
If that outcome means only “the guest applied the intent,” document/name it as
such.  If it means end-to-end action acceptance, retain the completion until a
separate exact-owner Shell publication receipt returns; the inference's later
computation need not be awaited, but the `inference-open`/`inference-propose`
admission must be.  Add replacement and missing-identity rows distinguishing
guest application from final host admission.  No runtime execution is claimed.

### P0: Publication Fixture Is Not Admitted By Its Own Closed Schema

The current action-handoff fixture's `publication` contains
`externalHostEffect` and `externalProjectedEffect` in addition to the required
inference vectors ([`fixture`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧫️fixture/🔣️.json>)).  Its schema defines `publication` with
`additionalProperties: false` and only `emit`, `error`, `hostEffect`, and
`projectedEffect` ([`schema`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json>)).  The colocated strict-AJV
fixture law must consequently reject its own input.  Either declare the two
external-URL vectors in the schema's required/properties set or move them to a
separate fixture; do not loosen the closed object.  This is a current
source-only defect, independently of the in-flight worker integration.

### Update: Publication Schema Fixture Finding Superseded

The current action-handoff schema now declares both external-URL vectors as
required closed `publication` members
([`action-handoff schema:357-366`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json:357>),
[`527-568`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json:527>)).  The preceding fixture/schema mismatch was observed during a
concurrent relocation and is no longer current; it is superseded rather than
an outstanding defect.

### P0: A Second Inference Effect Locally Aborts, But Does Not Cancel, The First Durable Job

The current one-port replacement is not a server cancellation.  On any second
`inference-open`, the worker synchronously calls `closeInferencePort` on the
prior port before installing the successor
([`backbone worker:4964-4977`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4964>)).  That close goes directly to
`terminateInferencePort`, which clears the timer, aborts the local fetch,
removes `inferencePort`, and publishes a local `inference.cancelled` failure
([`backbone worker:4944-4953`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4944>),
[`5177-5185`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5177>)).  It never enters
`cancelInferenceJob`, the only function that posts the authenticated
`/jobs/{jobId}/cancel` request and waits for the Hub-owned page
([`backbone worker:5061-5083`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5061>)).

This is reachable across *separate* otherwise valid guest actions, even after
the per-action host-effects vector is tightened to one.  The Shell callback
increments `inferencePortEpochRef` and overwrites
`inferencePortOwnerRef` before it posts the new open/propose pair
([`ShellHost:4256-4270`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4256>)).  The old
terminal status is then deliberately ignored by the exact new-owner fence
([`ShellHost:2255-2258`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2255>)).
If the first job already has a receipt, the browser has lost the only private
job id it could use to cancel it.  The Hub job continues until its own
lifetime/worker termination; it is not cancelled by a browser abort.

There is a more important submit race.  A close during `submitting` aborts the
POST before the worker stores the receipt.  Yet the Hub has already made
`POST /jobs` idempotent for the exact authenticated
`(user, authorization generation, scope, requestId)` tuple
([`inference SQLite:291-307`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:291>)), and the
route may have persisted/installed the retained job before its response is
lost ([`inference runtime:3203-3265`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3203>)).  The
current worker immediately discards that exact request id with its operation;
the public route offers only POST `/jobs`, job-id events, job-id cancel, and
job-id approval—no request-id lookup
([`Hub router:8051-8055`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8051>)).  Thus a response-lost accepted job
can be left running even though the UI has declared it cancelled and opened a
second job.

Posting an `inference-close` from Shell before overwriting the ref does **not**
fix this: that message currently has no close receipt, performs the same local
abort, and the next message can be processed immediately.  The correct
bounded ownership boundary is in the worker, with Shell retaining a pending
replacement rather than replacing its owner eagerly:

1. A close of `idle` is locally terminal.  A close with a known `jobId` retains
   the exact old operation and sends the existing job-id cancel until the
   server returns its terminal page; only then may it release the old owner.
2. A close during submit retains the sealed request id/body and admission
   identity.  It must reconcile the same idempotent POST to obtain the exact
   receipt, then use the normal cancel path.  A transport-indeterminate result
   stays a retained `closing/indeterminate` operation; it must not create a
   successor merely because local I/O was aborted.  This reuses the existing
   server idempotency law rather than minting a second job or exposing a
   private handle.
3. `inference-open` needs an exact worker acknowledgement (opened, refused, or
   predecessor-indeterminate) after lease validation and predecessor
   retirement.  Shell can keep a separate pending tuple, but must set
   `inferencePortOwnerRef`, dispatch `OPEN_INFERENCE_PORT`, and send
   `inference-propose` only after the matching **opened** acknowledgement.
   A status message cannot serve this purpose today because it is accepted
   only after Shell has already installed that owner.

Required controlled worker laws: (a) receipt-known running A then open B:
exactly one A cancel HTTP call and no B submit before A terminal page; (b)
POST A accepted at the Hub but response paused/lost, then B open: exact same A
request id is reconciled and cancelled, never a second A POST identity; (c)
retryable/indeterminate close: B gets no opened acknowledgement and the old
private owner remains retained; (d) identity/document-close while A is
submitting follows the same reconciliation rather than reporting a fabricated
cancelled terminal; and (e) a late A page never updates B.  The existing
worker tests cover user-initiated cancellation after an exact receipt
([`space-artifact owner test:2017-2036`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:2017>)),
but none of these replacement/response-loss interleavings.  This is source
audit only; no execution claim is made.

### P0: Response-Lost Inference Must Reconcile Through An Exact Reader-Bound Lookup, Not `POST /jobs`

The preceding recommendation to “reconcile the same idempotent POST” is too
weak and is superseded.  The current SQLite idempotency key omits the session
id: it is `(user_id, authorization_generation, space_id, document_id,
request_id)` ([`inference SQLite:11-35`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:11>),
[`276-316`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:276>)).  A re-POST derives a new
identity and map base before `accept`; after browser-broker rotation it may
therefore either conflict or create a different principal's job.  It is not a
safe cleanup operation.

The existing private reader is the correct server authority boundary:
`InferenceReaderV1::matches` compares **user, session, authorization
generation, space, and document** ([`inference SQLite:215-226`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:215>)).  Existing
job-id reads and cancellation already construct that reader from the current
authenticated session and revalidate live Author authority
([`runtime:3280-3289`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3280>),
[`3316-3327`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3316>)).  The smallest safe missing
route is consequently an owner-private, read-only reconcile endpoint:

```
POST /spaces/{spaceId}/documents/{documentId}/inference/gis-map/jobs/reconcile
{ "schema": "semio.hub.inference-job-reconcile/v1", "version": 1,
  "requestId": "<exact lower 32-hex>" }
```

It needs a closed Rust/TS/JSON `InferenceJobReconcileRequestV1` bounded below
the existing `INFERENCE_REQUEST_MAX_BYTES` (the fixed payload itself should be
at most 256 bytes).  The ledger operation should select only by the existing
five-column unique prefix, decode the frozen identity, then apply
`InferenceReaderV1::matches`; it must return the existing job receipt/id only
on that exact match.  It must not call `accept`, select a catalog binding,
capture a map base, or install a runtime operation.  The runtime endpoint can
reuse `inference_context`, `authenticated_session`, `session_reader`, the
document gate, and `check_live_inference_author`, then hand the returned job
id to the already-correct `cancel_gis_map_job` path.  The current router has
only submit, event, cancel, approval, and approval-undo routes
([`Hub router:8049-8055`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8049>)); no request-id lookup exists today.

The route is safe to call with a rotated bearer because it is side-effect-free
and the server performs the exact owner proof.  A nonmatching reader must
produce an owner-denied result which the worker stores as **indeterminate**;
it must never retry submit or admit a successor.  The worker currently has no
private authenticated-session pin: `browserBrokerFetch` holds only a rotating
capability proof, and `/auth/sessions/me` exposes the generation but no session
binding ([`backbone worker:4908-4929`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4908>),
[`SessionMeResponse:6488-6501`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6488>)).
`sessionBindingSha256` is available on authenticated directory pages, but is
not retained as a broker identity credential; it may be a secondary local
fence only if captured by the worker itself, never supplied by Shell.  If a
literal client-side proof *before even the lookup POST* is required, a
server-attested binding must be added to the original private submit receipt
and to a broker-owned refresh response; no current API provides that proof.
Server reader-bound reconciliation is the minimal correct current boundary.

There is one important result-classification edge for that close lane.  The
known-job cancel route first durably calls `request_cancel`, then obtains its
HTTP page through `events` ([`runtime:3316-3340`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3316>)).
`events` rejects `now >= expires_at` before returning a page
([`inference SQLite:546-570`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:546>)).  Consequently a cancel
POST can have committed its request/terminalization but still return
`Expired`; neither that error nor a dropped response proves the final state.
The close owner must classify both as indeterminate and keep its exact job id
and sealed request.  The reconcile/terminal-read API should return a bounded
owner-private state independently of the live-progress expiry gate (or invoke
the existing retained-runtime/server recovery then return its terminal page).
It must not report `cancelled` merely because the client issued a cancel.

Worker ownership changes are correspondingly narrow: retain the sealed
`requestId`, scope, and a close/tombstone owner until either (a) a known job-id
is cancelled to a terminal Hub page or (b) reconcile is exact-owner denied or
transport-indeterminate.  Do not use `docAbort` for that retained close lane:
the current `closeArtifactRuntime` aborts it before `closeInferencePort`
([`backbone worker:5430-5445`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5430>)),
while `inferenceBrokerFetch` refuses any request once it is aborted
([`backbone worker:4908-4913`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4908>)).  Artifact disposal must be two-stage:
block new document work, drain the retained inference close/reconcile owner,
then abort/drop the document state only after terminal proof.  A new identity
or failed reconcile leaves a durable local tombstone and blocks a successor;
a browser timeout is not cancellation proof.

The relay also currently makes all live browser inference impossible:
`localRelayUpstreamPath` allowlists no inference path
([`Hub script:463-476`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:463>)),
whereas `inferenceBrokerFetch` emits `/spaces/.../documents/.../inference/gis-map/jobs...`
([`backbone worker:4902-4915`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4902>)).
Add one strict decoded-and-reencoded inference route helper there: submit and
reconcile are POST/no query; event is GET with exactly one canonical bounded
`after` (the ledger bound is 16, not merely three decimal characters);
cancel/approval and approval-undo are POST/no query.  Reject encoded slash,
unknown query, malformed/zero job id, wrong method, and every other inference
suffix.  The appropriate existing source law is
`GisMapProposalCheckScript`/`proveGisMapProposalApprovalFixture` in
[`🌎️hub/📦️packages/🦀️rust/📜️script.ts:7222-7377`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7222>)
and command `gis-map-proposal-check` ([`12096-12149`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12096>)); extend its existing
`🗺️gis-inference-job-v1` fixture with closed HTTP-route and reconcile rows,
rather than a disconnected source oracle.

Required native/browser-controlled rows are: accepted response deliberately
lost then same-reader reconcile gives the original job id and one cancel;
each of the five reader fields mismatching denies without a new accepted row;
reconcile does not run the factory/install a second worker; close during submit
keeps the close owner past document-input retirement; and A→B→C admits only C
after A has a terminal page.  A mocked `browserBrokerFetch` alone cannot
qualify the relay; one controlled live local-relay request must exercise its
strict allowlist separately.

### Opening Admission Update: Current Handshake Is Correctly Non-Replacing, With One Required Completion Boundary

The current opening change has fixed the old eager replacement: worker
`openInferencePort` refuses capacity before installing a new port
([`backbone worker:4973-4990`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4973>)); Shell retains a private mailbox and
only publishes `inferencePortOwnerRef`, `OPEN_INFERENCE_PORT`, and the propose
request after the exact `opened` result and its current entry/client/session
checks ([`ShellHost:4255-4291`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4255>)).
The mailbox itself verifies exact epoch and scope before settlement
([`opening:44-94`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🚪️opening/🟦️.ts:44>)).

This is a sound admission-only step, not close correctness: current worker
open cannot yet return `indeterminate`, and `closeInferencePort` still locally
terminates an accepted/running job without attempting the known job-id cancel
([`backbone worker:5191-5199`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5191>)).
When the retained predecessor/tombstone lands, keep the following invariant:
an `opened` receipt means capacity has been freed by **terminal remote proof**
or there was no job; `refused`/`indeterminate` must settle only the candidate
mailbox and must not send a close to, mutate, or erase the predecessor.  The
current Shell catch's `inference-close` is harmless only for an opened
candidate that has not yet proposed; after a candidate can inherit/queue
work, its close request must be exact-candidate-owned.  Add controlled rows
for opened-then-owner-replaced (one exact candidate close), stale/foreign
opening receipt (no port close), capacity predecessor (no candidate propose),
and timeout after worker installation (candidate close, predecessor untouched).
No remote/browser integration is claimed from these mailbox or mock-fetch
laws.

### P1: Opening Disposition Does Not Yet Bind Its Code To Its Semantic Class

The new opening envelope correctly rejects `opened` with a non-null code, but
both the runtime parser and neutral schema admit all remaining combinations:
`{ outcome: "indeterminate", code: "inference.capacity" }` and
`{ outcome: "refused", code: "inference.transport" }` pass today
([`opening parser:35-40`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🚪️opening/🟦️.ts:35>),
[`opening schema:50-300`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🚪️opening/🧬️schema/🔣️.json:50>)).  That distinction becomes
security/liveness relevant once a capacity predecessor is retained: capacity,
invalid input, and an unverified lease are exact refusals (no remote state
unknown), while transport is the sole current indeterminate result.

Make the closed mapping exact in both schema and `parse...ResultV1`:
`opened ↔ null`; `refused ↔ capacity|invalid|lease-unverified`; and
`indeterminate ↔ transport`.  Add the two swapped-code hostile rows plus an
end-to-end worker-wire decode refusal in the existing
`backbone-envelope-io` opening test
([`test:1686-1718`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts:1686>)).  The current focused green mailbox
law covers only foreign epoch/scope, duplicate receipt, replay, and an opened
non-null-code hostile; it does not prove this semantic partition.

### Current Relay Review: Strict Inference Allowlist Is Now Present

This supersedes the earlier finding that the relay had no inference route.
The current relay admits only the five implemented Hub operations: submit,
owner-private event page, cancel, approval, and approval undo
([`Hub relay:466-489`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:466>),
[`Hub router:8049-8056`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8049>)).
It decodes each scope segment, requires the exact re-encoding, prohibits dot
segments and encoded slash/control bytes, and allows only the scope grammar
the inference schema owns (1--96 `[A-Za-z0-9._:-]` bytes)
([`inference TS schema:28-36`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🟦️.ts:28>)).
The GET path has exactly one canonical `after` query parameter and is capped
at 16, matching the ledger's persisted cursor bound
([`SQLite ledger:545-549`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:545>)).

The relay applies the same 1,024-byte request and 16,384-byte response caps
to the inferred path before it forwards the bearer; it strips the browser
proof header at the upstream boundary and returns no-store responses
([`relay limits and proof boundary:621-706`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:621>)).
The neutral schema independently recognizes the same method/path grammar
([`relay schema`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🌐️relay/🔣️.json:1>)).
Per root's reported result, `inference-relay-check` completed 34 checks
(29 live route rows, 3 response-bound rows, 2 request-bound rows); this audit
did not execute that gate.

One bounded test gap remains, not an authorization bypass: the live relay law
only exercises an oversized declared `Content-Length`.  The streamed reader
does enforce its running total, but the one-time browser proof is deliberately
ratcheted before that body is consumed, so a chunked 1,025-byte request will
receive `413` **with** an advanced-proof header.  Add one `ReadableStream`
no-`Content-Length` row to lock the intended behavior: upstream call count
unchanged, 413, proof advanced exactly once, then a request with the next
proof succeeds.  That tests the actual `readLocalRelayBody` path without
changing its safe bounded behavior.

When the owner-private request-id reconciliation route is added, it must be
added atomically to all four current representations: the Hub router, worker
`inferenceBrokerFetch` route permission, `localRelayInferencePath`, and the
relay schema/fixture/live upstream assertion.  A route only in the Hub router
will remain browser-unreachable; a broad suffix pattern would weaken the
present strict surface.

### P0: Current Two-Author Provisioning Still Mixes App Protocol 14 And 15

The live OS admission authority is now channel **15**:
[`CHANNEL_VERSION`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:24>),
[`DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1130>),
and native trusted-catalog descriptor validation
([`trusted catalog:1145-1150`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1145>))
all reject a selected component declared as 14.  The following acceptance
closure still hardcodes 14, so its source fixture can be self-consistent while
the current materialized GIS descriptor is correctly refused.

Required positive-v15 reissue:

- `🌎️hub/🧪️fixtures/🤝️two-author-shell-v1/🔣️.json:10` and its source
  assertion [`script:12553-12557`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12553>) must say 15 (prefer the imported
  directory constant in the script).  This is the registered source gate's
  direct preflight.
- `🌎️hub/🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json:12` must say 15;
  its `expectedDigest` at line 25 must be regenerated through the existing
  Rust/TS frozen-binding digest oracle.  Keep its hostile value **13** at line
  34 unchanged: that is a deliberate rejection row.
- The frozen inference consumer is presently itself typed as version 14:
  [`inference schema TS:529-533`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🟦️.ts:529>) and
  [`inference JSON schema:503-507`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🔣️.json:503>).  Both must become 15 before an actual
  selected binding can be decoded in the browser/Hub inference path.
- The trusted two-package bootstrap's GIS and Stdio package entries
  [`bootstrap fixture:20,42`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🔣️.json:20>)
  and the positive `generation-stage` descriptors
  [`stage fixture:5-8`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧱️generation-stage/🔣️.json:5>)
  must become 15.  Recompute the bootstrap profile `generationId` and the
  rotation aliases derived from it (`initialGenerationId`,
  `currentAfterFailedCandidate`, `stalePlan.issuedGenerationId`); its selected
  closure hash does not encode the protocol and should not be changed merely
  because this field changed.  Do not mechanically alter the synthetic failed
  or next generation sentinels unless their own assertions require it.
- The independent Hub oracle has three document-open checks hardcoded to 14
  ([`script:4334-4339,4487-4490,4543-4546`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4334>)),
  plus the compiled-dependency and generation-stage positive descriptor setup
  ([`script:9075-9079,9477-9484`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9075>)).
  Bind these to the imported protocol authority rather than a replacement
  literal.  `document-open-plan-v1.json` has three positive 14 occurrences
  ([`110,146,292`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json:110>)); after changing them,
  regenerate its `catalogEncoding.expectedHex` and
  `catalogEncoding.expectedGenerationId`, because the catalog framing includes
  this four-byte field ([`script:4312-4341`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4312>)).  Its explicit mismatch hostile
  value **13** remains 13.
- `browser-document-open-v1.json`'s positive installed target and plan
  ([`33,140`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json:33>) must become 15.  Its
  descriptor digest and receipt digest encodings do not contain protocol, so
  they do not need invented new values merely for this transition.
- The actual browser-host staging description and its fixture both pin 14
  ([`browser describe schema:19-22`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🧬️schema/🔣️.json:19>),
  [`fixture:4-7`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🧫️fixtures/🔣️.json:4>));
  reissue together with the component descriptor.  The producer test fixtures
  `fresh-component` and `catalog-complete` have positive descriptor setup at
  lines 134 and 136 respectively and need 15.  Preserve
  `catalog-complete`'s `{appChannelVersion: 14, extra: true}` as a malformed
  extra-field hostile; add a plain 14 unsupported-version hostile if its
  intent is to prove rejection of the preceding current version.
- The MCP live-catalog schema/fixture used by the two authenticated MCP
  readers is also fixed at 14
  ([`MCP schema:145-159`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧬️schema/🔣️.json:145>),
  [`fixture:10-38`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧫️fixtures/🔐️hub-live-catalog/🔣️.json:10>)).
  Move both positive package projections to 15 and leave the hostile 13 row
  as the mismatch proof.

The generic manifest codec test's literal 14 and unrelated numeric `14`
values are outside this acceptance closure; do not sweep them as part of the
protocol reissue.  Conversely, the trusted-catalog `two-package` fixture is
an active native provider fixture and its two positive package records must
be reissued to 15 or its provider law will no longer model a selectable
current package.

### Registered Full-Browser Acceptance Invocation (Not Yet A Passing Claim)

The actual launch configuration is
[`launch.json:7870-7888`](</Users/ueli/Documents/semio/.vscode/launch.json:7870>):
`bun nx run os-hub:trusted-stdio-gis-bundle-check -- --two-author-shell`, with
the ticket-owned `trusted-stdio-gis-browser` artifact directory and isolated
one-job Cargo environment.  It first runs the source fixture fences, builds
`semio-hub` and `semio-framework-os-mcp` binaries from one target directory,
runs the genuine checkpoint-pair native law, materializes and publishes one
trusted current, checks the closed GIS browser actor, then passes that exact
current/data root and binaries to the Playwright two-Shell process
([`bundle gate:12624-12793`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12624>)).

The process creates the map through Shell A before either document-scoped MCP
or peer connection, then uses the same data root for two auth-scoped MCP
clients, two document sockets, English Shell A and German Shell B, ordinary
Shell proposal/approval/Undo, and a Hub restart
([`two-Shell owner:11757-11946`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11757>)).
Its receipt explicitly excludes browser-qualified publication, private-job
restart recovery, WGPU rendering, external-model-provider, and collaborative
redo; it must not be read as proof of those behaviors.

Before starting that expensive command, the positive v15 closure above must
be reissued and qualified, the current component/closed actor must be freshly
materialized and selected, the direct authenticated actor/worker bridge must
be integrated, and the retained inference close/reconcile path must provide
remote terminal proof.  The source-only companion launch
`--two-author-source` is useful for fixture topology but cannot qualify any
of those runtime prerequisites.

### Protocol 15 Derived Golden Reissue (Read-Only, 2026-09-09)

This supersedes the preceding inventory's description of the positive rows as
still being 14: the current checkout has already changed the selected
bootstrap packages and the document-open catalog rows to 15, while their
derived generation values remain 14-era values.  I cloned the fixture objects
in memory, used the current producer framing exactly, and did not change
source files or run a project target.

#### Trusted Stdio + GIS bootstrap generation

The canonical producer is
[`trustedBootstrapProfileEncoding`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8273>).  Its immutable input is:

1. ASCII `semio/hub/trusted-profile-generation/v1\\0`;
2. an eight-byte big-endian length plus profile ID;
3. a four-byte big-endian selected-closure count, followed in fixture closure
   order by each selected package's framed plugin/package/version/role and its
   three raw 32-byte digests;
4. each package's protocol as `u64-be(4) || u32-be(15)`, framed browser actor,
   UTF-8 tuple-sorted dependency vector, then the tuple-sorted native codec
   rows `(kind, schema, raw 32-byte Pack hash)`;
5. the one open target with the same framed 32-bit protocol and grant bytes.

The exact codec projection is still 26 Stdio plus 2 GIS rows from
[`native-codec-factories.json`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json)
and [`native-codecs.json`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json), via
[`projectTrustedBootstrapCodecsV1`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9166>).  Captured generation bytes are
3,439 bytes.  Node `createHash("sha256")` and WebCrypto `subtle.digest` agree:

```text
profile.generationId (v15): e53729da5bfb25770f2601ed575e0fa3eeb7903230239df46ee5d6170d9d4e52
node == webcrypto: true
```

As a formula cross-check, changing only the two package protocol fields back
to 14 produces exactly the present stale fixture value
`9d010f1a012a471c7ceaae923472b3687018b02c4aff475db7db4ee6142a2565`.
The selected-closure framing deliberately excludes protocol and remains
`848e9d6c38a1b1202398214a9138e5e289ee5564bc0bd1fdf61d622c9d4f5588`.
Therefore update only the generation-derived aliases that denote the current
profile (`generationId`, `rotation.initialGenerationId`,
`rotation.currentAfterFailedCandidate`, and
`rotation.stalePlan.issuedGenerationId`), not the closure hash or synthetic
failed/next sentinels.

#### Document-open catalog generation and exact bytes

[`documentOpenCatalogEncoding`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4323>) frames ASCII
`semio/hub/openable-document-catalog/v1\\0`, a `u32-be` row count, then 19
`u64-be(length)||payload` fields per catalog row.  Only field seven changes:
the payload stays four bytes but becomes `0000000f` rather than `0000000e`.
With the two current positive rows at 15, the 893-byte payload is:

```text
73656d696f2f6875622f6f70656e61626c652d646f63756d656e742d636174616c6f672f76310000000002000000000000000c732e6769733ae59cb0e59bb30000000000000012732e6769732e6769736d61703a636f6465630000000000000008312e302e303aceb200000000000000202222222222222222222222222222222222222222222222222222222222222222000000000000002044444444444444444444444444444444444444444444444444444444444444440000000000000020555555555555555555555555555555555555555555555555555555555555555500000000000000040000000f000000000000000c732e6769733a6769736d61700000000000000017732e6769732e6769736d617040312f2a3ae7b2bee5af8600000000000000201111111111111111111111111111111111111111111111111111111111111111000000000000000c732e6769733a6769736d617000000000000000013100000000000000012a0000000000000012737572666163652e6769732e656469746f7200000000000000076170702e676973000000000000000f77696e646f772e646f63756d656e740000000000000006656469746f72000000000000000572656163740000000000000003010101000000000000000c732e6769733ae59cb0e59bb30000000000000012732e6769732e6769736d61703a636f6465630000000000000008312e302e303aceb200000000000000202222222222222222222222222222222222222222222222222222222222222222000000000000002044444444444444444444444444444444444444444444444444444444444444440000000000000020555555555555555555555555555555555555555555555555555555555555555500000000000000040000000f000000000000000c732e6769733a6769736d61700000000000000017732e6769732e6769736d617040312f2a3ae7b2bee5af8600000000000000201111111111111111111111111111111111111111111111111111111111111111000000000000000c732e6769733a6769736d617000000000000000013100000000000000012a0000000000000012737572666163652e6769732e76696577657200000000000000076170702e676973000000000000000f77696e646f772e646f63756d656e740000000000000006766965776572000000000000000572656163740000000000000003010001
```

Both SHA-256 implementations agree on:

```text
catalogEncoding.expectedGenerationId (v15): 2bb0378fb7e9c4ae59a3c5af0d7dddcd67f27a30d8ec7aca09c8fd888ef2978f
node == webcrypto: true
```

Changing only both protocol payloads back to 14 reconstructs both current
stale values byte-for-byte: the fixture's existing `expectedHex` and
`de543c145295fc3122d590888bde5e6c07439f420e17dbd31e9febdf5c2d2904`.
The v15 generation must be propagated to both document-plan and browser-plan
catalog fields; descriptor and plan receipt hashes are not catalog encodings
and must not be regenerated merely for this change.

#### Exact validation gates

No permanent generator or migration script is warranted: the existing source
oracles already regenerate and compare these bytes.  After applying the exact
values, use the registered checks, in increasing runtime scope:

- `bun nx run os-hub:trusted-stdio-gis-bootstrap` verifies the full profile
  with Node and WebCrypto at [`script:8911-8918`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8911>).
- `bun nx run os-hub:open-plan-source-check`, then
  `bun nx run os-hub:open-plan-check`, and
  `bun nx run os-hub:browser-document-open-check` cover the exact catalog,
  server selection, and browser-plan propagation (registered in
  [`Hub project.json:475-528`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:475>)).
- `bun nx run os-hub:trusted-stdio-gis-bundle-check -- --source` verifies the
  staged v15 profile and its source fences.  Native selection and full process
  work remain separately qualified by the existing `native-catalog-selection`
  and `--two-author-shell` launches.

#### Negative-version collision audit

A negative that meant “current plus one” at protocol 14 must now explicitly
be 16 (or `CURRENT + 1`), not 15.  The active compiled-dependency mutation is
correct in the current script: it now assigns
`DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 + 1` at
[`script:9107-9109`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9107>).  The other relevant hostile fields I inspected
are deliberately 13 and remain noncurrent: document-open mismatch
[`fixture:469-472`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json:469>),
MCP live-catalog [`fixture:38`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧫️fixtures/🔐️hub-live-catalog/🔣️.json:38>),
frozen binding [`fixture:34`](</Users/ueli/Documents/semio/🌎️hub/🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json:34>),
and trusted generation-stage mutations [`script:9540,9555-9558`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9540>).  They are not accidental v15 admissions.  No additional current-15 negative collision was found in the audited trusted Stdio/GIS, document-open, MCP, and browser describe acceptance closure.

### Native WGPU Inference Close Must Retain an Unconfirmed Submit (Read-Only, 2026-09-09)

The browser worker's newly qualified cancel-pending behavior needs an equivalent native
WGPU owner before a native execution-target lease is ever enabled.  Today no native submit
can start: `document_execution_target_lease` is deliberately always `None`
([`WGPU Shell:2366-2369,2864-2868`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2366>)), and
`execution_target_lease_verified` therefore refuses the port
([`WGPU Shell:4517-4521`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4517>)).  This is an honest current refusal, not runtime evidence for the lifecycle below.

#### Concrete orphan seam

`open_inference_port` first calls `cancel_inference_port`, then immediately admits a
replacement ([`WGPU Shell:4531-4571`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4531>)).  The latter simply removes the
only `Arc<ShellInferenceRunner>` and invokes `OperationContext::cancel_now()`
([`WGPU Shell:4589-4595`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4589>)).

That is unsafe once calls are live:

- The submit turn mints `request_id` *inside* its detached I/O future, and neither the
  runner nor the shell retains the request or a later receipt/job locator
  ([`WGPU Shell:1439-1477`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1439>)).
- Completion upgrades only a `Weak<ShellInferenceRunner>`.  After close/replacement the
  runner can be gone even when the HTTP submit already linearized remotely, so it neither
  folds the receipt nor performs a server cancellation
  ([`WGPU Shell:1477-1503`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1477>)).
- A user cancel is correctly not terminal in the pure reducer: it only becomes a
  `Cancel { job_id }` once a receipt supplies a job id
  ([`WGPU Shell:1296-1329`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1296>)).  `cancel_inference_port` bypasses
  that rule by aborting the whole context before the submit outcome is known.

The native client has only submit, event-page, cancel-by-known-job, and approval calls
([`Directory client:1063-1089`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1063>)).
There is presently no owner-pinned lookup by request id.  The Hub SQLite ledger already
keys acceptance by the identity-derived binding and supplied request bytes
([`inference ledger:276-315`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:276>)); a cleanup replay under a replacement
browser/session client would instead be a different authenticated request.  The WGPU shell
also replaces/clears `directory_client` during identity bootstrap/auth failures, so a
reconcile path must retain the original client/identity rather than dereference the current
shell client.

#### Smallest consistent lifecycle packet

Keep an exact `Closing` tombstone rather than dropping `inference_port`.  It owns the
request id minted *before* dispatch, immutable scope, original client/identity/lease owner,
known `job_id: Option`, and the physical runner/future drain.  A replacement is refused until
that tombstone reaches one of these proven endpoints:

1. the retained submit resolves to a receipt, then the existing cancel-by-job operation
   returns a terminal server page;
2. a new owner-pinned **lookup-by-request-id** route says the retained request did not
   linearize; or
3. original authority cannot be proved, which is `indeterminate` and blocks a successor —
   it must never resubmit the original body with a newer session/auth-generation.

The typed wire body is already schema-owned as
`InferenceJobReconcileRequestV1`/`InferenceJobReconcileResultV1`
([`inference schema:158-176,480-497`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🦀️.rs:158>)); it deliberately carries
only `requestId`.  The still-missing runtime route and DirectoryClient method must bind that
request to the original caller's user, authorization generation, space, and document, and return
only the ordinary private receipt/terminal state.  There is currently no consumer outside the
schema (`rg` finds the type only in that schema), so it cannot yet reconcile either browser or
WGPU ownership.  After a receipt, the current `cancel_gis_map_inference_job` client method is
the right server operation.  A cancel response remains nonterminal until the reducer sees that
exact terminal page.  No new generic browser/native replay API is needed.

Add a neutral close/reconcile corpus plus one WGPU I/O-lane law, not merely another pure
driver test.  Required rows are: cancel before submit receipt; close after a lost submit
receipt; close before linearization/lookup absent; identity rotation that forbids use of a
new client; stale late terminal response; and successor admission only after exact terminal
cleanup.  The WGPU law should pause submit after the request write, then assert exactly one
submit, lookup using the captured client, one cancel after receipt discovery, no successor
submit before the terminal page, and reaping of the old future.  Existing driver-only rows in
[`WGPU command registry tests:230-352`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs:230>) do not exercise this
transport/owner boundary.

This packet is source audit only; no native WGPU inference submit or close law has been run.

#### Post-apply v15 literal sweep

The inspected positive fixtures now consistently carry 15: trusted bootstrap/document-open,
browser document-open, MCP live catalog, frozen binding, and execution-target lease.  The only
named unsupported current-plus-one mutation derives `CURRENT + 1`, hence is now 16.  No stale
v14 positive or accidental current-15 negative was found in that closure.  One source oracle
still compares the frozen candidate to the literal `15` rather than the exported constant
([`Hub script:5682`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5682>)); it is correct today but must be
constant-derived before the next protocol increment.

### Two-Author Mounted Shell Acceptance Frontier (Read-Only, 2026-09-09)

The current `--two-author-shell` owner is a materially real composition candidate, not merely
the neutral fixture: it builds the Hub/MCP bins and runs the dedicated persisted-genesis seed
law before materializing and publishing the one current
([`Hub script:12761-12802`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12761>)).  It starts the Hub on the
prepared data root, creates the private space, admits Author B, has Author A create the Map
through the ordinary dialog, and only then opens two independent MCP children, two socket
witnesses, and the second ordinary Shell
([`Hub script:11873-11922`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11873>)).  The two browser peers use separate
Playwright contexts, relay credentials and client instance IDs; `waitMaps` requires two mounted
`tiled-map` probes with the selected component/descriptor/actor hashes and equal region state
([`Hub script:11924-11945`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11924>)).

It also genuinely exercises the intended ownership split:

- Author B submits through its actual MCP child, waits the inherited real-codec checkpoint,
  cancels with its private job handle, releases the checkpoint, and polls the exact job until a
  cancelled/no-offer page ([`Hub script:11473-11527`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11473>)).
- Author A proposes, approves and undoes through ordinary visible Shell controls.  The harness
  awaits two server socket controls, reads matching canonical pairs through both MCP children,
  and requires the region appear, then disappear, on both mounted Maps
  ([`Hub script:11947-11991`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11947>)).
- It closes all peers, restarts the Hub against the same prepared root, reissues identities, then
  reopens both Shells/MCP clients and requires the post-Undo canonical pair and both Map states
  to match ([`Hub script:11992-12012`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11992>)).  The receipt correctly
  leaves private running-job recovery and WGPU rendering as nonclaims.

#### P0 acceptance-correlation gap: side witness control is not the mounted-worker ACK

`changedPair` proves that two separately opened witness sockets received equal
`RebootstrapRequired` controls and that two MCP readers see the corresponding durable pair
([`Hub script:11947-11958`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11947>)).  Separately, `waitMaps` proves only a
bounded probe containing scope, content hashes, `uiRevision`, and region IDs
([`Hub script:11530-11559,11928-11945`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11530>)); its real projection reads only
the acknowledged UI store revision and the Map document bytes
([`ShellHost:664-709`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:664>)).  No mounted Shell probe
contains an acknowledged checkpoint id, frontier, or control identity.  Thus the fixture claim
`same-peer-rebootstrap-control` is not yet an exact proof that either mounted worker consumed
*that* server control: an unrelated reload/fetch/patch producing the same regions could satisfy
the current independent assertions.

The smallest repair is to extend the existing closed, dev-only acknowledged
`UiDocumentStore` probe with the exact applied `activeCheckpointId` and frontier (or an opaque
control digest) *only after the worker acknowledges it*.  At each `changedPair`, assert both
mounted probes equal the witness control's checkpoint/frontier and the MCP pair.  Add those
fields and the equality law to the two-author neutral fixture; do not expose private job or
undo handles.

#### P1: the cancellation scene proves running/cancel, not an observed progress sample

The MCP cancellation helper requires a `running` submit receipt, the FD4 callback entry, a
cancel request, and a terminal cancelled page, but never validates a nonempty `progress` page
or `completed > 0` ([`Hub script:11473-11527`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11473>)).  This is structural:
the inherited checkpoint gate pauses before the outer inference callback persists its ledger
heartbeat ([`runtime:2769-2781,3169-3180`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2769>)).  The present test truthfully
proves cancellable admitted computation, but not MCP progress delivery.  If the acceptance
criterion includes progress, add a deterministic gate after a real persisted positive-progress
heartbeat and assert its bounded page before requesting cancel; do not synthesize progress in
the process runner.

No two-author browser/native process was started by this audit.  The assertions above describe
current source; the acceptance journey remains unqualified until its registered process target
actually completes.

### Browser Inference Retained Closing (Read-Only, 2026-09-09)

The focused browser-worker test reported by the implementation owner as GREEN exercises only
the newly added narrow closing slice.  It is **not** evidence for the three fixture scenarios
`lost-submit`, `retired-document`, and `rotated-session`; those remain outside that focused
selection.  The current browser worker does correctly seal `operation.request` before the first
submit transport, keeps it through an indeterminate result, and gives the port its own abort,
scope, original local session epoch, client id, lease snapshot, timer and one-shot cancel state
([`backbone worker:4785-4804,5115-5149`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4785>)).  Queued broker admission checks the
operation owner both before and after constructing a next proof
([`backbone worker:542-576`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:542>)); it does not optimistically report a Cancel terminal.

#### P0: a late approval recreates a browser-private Undo owner after document retirement

`closeArtifactRuntime` deliberately retires the matching Undo owner before it drops the
document lease and removes the artifact state
([`backbone worker:5634-5656`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5634>)).  A pre-existing approval response, or a reconcile response, can arrive
after that transition.  Both paths unconditionally call
`retainInferenceApprovalUndo` ([`backbone worker:5171-5184,5320-5326`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5171>)).  That helper accepts a missing
artifact state (`state?.browserActorReservation?...`) and installs an `awaiting-mount` owner
using the already-retired client id
([`backbone worker:4961-4988`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4961>)).  A normal reopen has another client id; the bind path merely returns on
the mismatch rather than retiring that newly resurrected owner
([`backbone worker:4920-4928`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4920>)).

This is a retained private receipt/undo-handle tombstone with no presentation owner.  It is
bounded to one owner but survives until another approval happens to replace it.  The repair must
make the post-response handoff prove that the original document/client/session owner is still
eligible for browser presentation; otherwise dispose only that presentation material while the
Hub's durable fact remains authoritative.  A targeted law must pause an approval (and separately
a recovered approval) across `closeArtifact`, then assert: no `inference-history-status`
available message, no new undo owner, one exact port close, and no successor exposure.  The same
row should cover session rotation, not only document deletion.

#### P1: broker-proof installation lacks a post-response owner fence

`browserBrokerFetch` checks `admit` before starting the network operation, clears the old proof,
and then installs the response's successor proof without checking the owner again
([`backbone worker:559-586`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:559>)).  During that await, a successor identity can install its own proof because
the old slot is empty; the stale response then overwrites it.  The old inference request still
uses its already-captured old proof, so this does not authorize it as the new user, but it loses
the successor proof and makes the next identity's request consume an old-session proof and fail
closed/rebootstrap.

Fence proof replacement after the response with the same captured owner/session generation.
If it changed, zero the stale `next` bytes and leave the successor's installed proof intact.
Required deterministic row: pause reconcile/cancel after its second admission check, rotate the
directory/identity owner and install a successor proof, release an advanced old response, then
assert the successor proof remains and the old operation cannot post a status, Undo owner, or
effect into the successor.

#### Reconcile route is intentionally still unqualified, not a terminal-absence proof

The browser/relay admission grammar permits `POST .../jobs/reconcile`
([`backbone worker:5024-5038`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5024>)), and the SQLite reader has an exact
reader-bound lookup ([`inference SQLite:545-651`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:545>)).  The current Hub router, however, only installs submit,
events, cancel, approval and undo—not reconciliation
([`Hub binary:8048-8056`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8048>)).  This is active Home-owned work, so it is an explicit unqualified boundary rather than a
new regression in the browser patch.

Importantly, `found:false` must **not** release the browser tombstone yet.  Submit holds the
per-document runtime gate through expensive frozen-base validation and `ledger.accept`
([`inference runtime:3211-3253`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3211>)); a concurrent lookup can commit an empty immediate
read before that original handler accepts.  The route needs the same causal admission fence (or
an explicit pending result) before absence becomes terminal.  Until then, original-session loss
correctly remains indeterminate and blocks a successor; it must not replay the sealed request
under a newer identity.

The Shell's close receipt itself is scoped correctly for a live owner: it matches operation epoch
and exact scope before clearing UI ([`ShellHost:2261-2276`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2261>)).  Document and identity
retirement intentionally clear that presentation owner first
([`ShellHost:2655-2665,4904-4913`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2655>)); therefore worker-side closure/tombstone
ownership, not a late Shell message, is the only lawful reconciliation path after retirement.

#### Current-source correction and two remaining browser races

The earlier P0/P1 paragraphs describe the pre-fix frontier.  Current source now refuses a late
Undo retention unless the exact artifact state remains live with the same client and local
directory session ([`backbone worker:4975-5003`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4975>)), and checks the captured broker owner plus
`admit` again after the network await before installing its next proof
([`backbone worker:545-600`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:545>)).  Those two prior races are therefore repaired in source; the
focused test result still does not qualify the broader lost-submit/retired-document/rotated-
session corpus.

The native reconcile route is also now mounted and takes the same per-document gate as submit
([`Hub binary:7991-8000,8080-8087`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7991>),
[`runtime:3203-3270,3275-3285`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3203>)).  It must nevertheless preserve
`found:false` as *indeterminate*: a lost original POST may not yet have reached that gate, so an
empty lookup can precede a later accepted submit.  Current browser code does preserve the sealed
request by converting that response through `terminateInferencePort` rather than releasing it
([`backbone worker:5176-5182,5087-5100`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5176>)).  A terminal absence needs an explicit
request-id arrival/reservation fence in the Hub; merely sharing the gate serializes only handlers
which have already arrived.  This is active Hub work, not an additional browser regression.

**P0 — an aborted queued Undo consumes the global broker proof.**
`browserBrokerFetch` tests an `AbortSignal` before it enters its serialized queue, but not again
after `await prior` ([`backbone worker:545-570`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:545>)).  Undo does pass its owner abort signal but has no
owner `admit` predicate ([`backbone worker:5056-5063`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5056>)).  Thus: keep a prior broker turn in flight; queue
Undo; retire the document (which aborts the owner); release the prior turn.  The queued Undo
clears the otherwise valid proof before `fetchWithTimeout` immediately observes its already
aborted signal, then wipes its replacement proof.  It sends no stale HTTP request, but it forces
an unrelated successor into rebootstrap.  Recheck `signal.aborted` immediately after the queue
await, before reading or clearing the proof.  Undo should also use an exact retained owner/
state/session admission predicate, so an identity or mount change cannot consume shared proof
capacity.  Add the paused-turn law asserting no Undo fetch, unchanged successor proof, no status
resurrection, and no later rebootstrap induced by the cancelled request.

**Current-source correction.** The owner applied this packet after the finding: the broker now
rechecks abort both immediately after queue admission and after next-proof derivation
([`backbone worker:545-569`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:545>)). Undo now retains the captured local session and supplies exact
owner/mount admission plus session-only proof retention
([`backbone worker:4838-4856,4981-5009,5065-5080`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4838>)). The stale queued-abort
proof-consumption defect is therefore repaired in current source. This audit did not run the
new test command.

**Qualified non-finding — `directory-close` is a stream-owner transition, not identity loss.**
Although `closeDirectory` advances `directoryWorkerEpoch` without advancing the inference
session, this is intentional: `openDirectoryBootstrap` calls it before every ordinary directory
stream bootstrap ([`backbone worker:4306-4325`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4306>)).  Retiring a document inference port at
that point would turn an unrelated stream rebootstrap into an orphaning cancel.  The epoch is
therefore directory-operation-local; inference correctly relies on document lease and session
ownership.  Identity/document retirement still needs to go through its own exact close paths.
No change is recommended from this observation.

### Browser Identity-to-Inference Authority Fence (Read-Only, 2026-09-09)

`directorySessionEpoch` is currently a useful *local test/legacy-open* fence, but not an
observable production authenticated-session transition. Its sole production increment is in
the legacy `openDirectory` helper ([`backbone worker:4227-4229`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4227>)). No current Shell production emitter
sends `directory-open`; the active Home path sends `directory-bootstrap-open`, whose
`openDirectoryBootstrap` deliberately calls stream cleanup without advancing that epoch
([`backbone worker:4306-4325`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4306>),
[`ShellHost:2771-2791`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2771>)).

The Shell does retire the visible inference port whenever its local identity state changes
([`ShellHost:2655-2665`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2655>)), but it sends no worker authority-generation replacement.
Identity changes can be delivered from the local identity document's remote mutations
([`ShellHost:2473-2492`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2473>)). Browser-broker proof is instead read once from the
`#semio-broker` fragment and transferred once during worker construction
([`ShellHost:186-192,2184-2194`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:186>)); no current identity transition
reinitializes it. Finally, `GET /auth/sessions/me` exposes only
`userId`, `email`, `displayName`, and expiry ([`OS directory client:3674`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3674>)); Shell reads only the user-facing
fields ([`ShellHost:2719-2725`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2719>)). It provides no stable
server session/generation with which the worker could prove a replacement.

Consequently the current `rotated-session` row, which directly changes the worker test seam,
is a valuable stale-owner unit law but **not** end-to-end evidence for an authenticated session
rotation. The existing fail-closed broker behavior still prevents an old proof accepted as a
new session after a 401; it does not give a successful successor-session handoff.

The smallest sound protocol is a first-class, private browser-broker authority binding:

1. after an authenticated server handoff supplies a fresh proof plus opaque server session
   generation, Shell retires the old visible inference owner and tells the worker an exact
   `authority-replaced` generation;
2. the worker advances a private authority epoch, rejects queued/late prior-owner admissions,
   and installs the replacement proof only for that generation;
3. new work cannot open until the fresh binding is installed; a sealed old submit stays
   indeterminate and can only reconcile/cancel with a proof of its original authority—never
   replay under the replacement;
4. the worker reports one exact closed terminal to the old Shell owner, while the successor
   cannot receive its status or Undo capability.

A language-neutral fixture needs at least: old queued submit plus successful new binding; late
old advanced response; remote identity-config mutation without server binding (must not claim a
rotation); and server 401/revocation. The browser law must use the real Shell broker-port
handoff, not direct mutation of `directorySessionEpoch`.

### Hub Inference DTO Nullability and Exact-Tag Audit (Read-Only, 2026-09-09)

**P0 — the normal browser inference HTTP path rejects the normal Hub JSON it invokes.** This is
not an MCP-only or synthetic mismatch. The protected Hub routes return their Rust DTOs without a
translation layer: submit returns `Json(receipt)`, events/cancel return `Json(page)`, and approval
returns `Json(receipt)` ([`Hub binary:7977-8055`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7977>)).
The browser worker directly feeds each successful body to the OS parsers
([`backbone worker:5154-5156,5263-5265,5296-5298,5343-5345`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5154>)).

`InferenceJobReceiptV1.proposal_hash` and `InferenceEventPageV1.proposal_hash` are bare Rust
`Option<String>` fields, not `skip_serializing_if` fields
([`Hub inference schema:373-383,417-432`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🦀️.rs:373>)).
Serde therefore emits required JSON `proposalHash: null` while there is no proposal. This is
intentional and live-native test evidence: the actual authenticated submit asserts a running
receipt with `null`, and the actual cancel response asserts `null`
([`Hub bin unit:1377-1404`](</Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️rs:1377>)).
The normal constructors also preserve that `Option` in the submit receipt, event page, and cancel
page ([`runtime:3288-3291,3314-3326,3342-3354`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3288>)).

OS currently describes these fields as optional strings and only handles `undefined`:

- `GisMapInferenceJobReceiptV1` and `GisMapInferenceEventPageV1` use `proposalHash?: string`
  ([`OS directory schema:1639-1687`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1639>));
- `parseGisMapInferenceJobReceiptV1` / `parseGisMapInferenceEventPageV1` allow absence but pass
  any present value (including `null`) to the hex-string decoder
  ([`OS directory schema:2030-2042,2047-2093`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:2030>));
- the OS JSON definitions also omit `proposalHash` from `required` and accept only a hex string
  ([`OS schema:473-571`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json:473>)).

Thus the first successful accepted/running/cancelled response from a genuine Hub fails before the
worker can publish its port status. Align the OS contract with the already-owned Hub schema:
make `proposalHash` required and `hex64 | null` in both DTO types and JSON definitions; reject
absence; preserve `null` in the parsed object and reducer. No legacy omission support is sound.
The independent OS package oracle still casts both fields as optional strings
([`OS TypeScript script:89-105`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts:89>)),
so it must be changed with the production type rather than perpetuating the old wire.

**Exact schema tags are another current fail-open seam.** Hub's schema establishes exact tags:
`semio.hub.inference-job-receipt/v1`, `semio.hub.inference-job-events/v1`,
`semio.hub.gis-map-inference-preview/v1`, and
`semio.hub.inference-approval-receipt/v1`
([`Hub schema JSON:205-217,326-420`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🔣️.json:205>)).
The Hub runtime emits those exact strings ([`runtime:3080,3288-3290,3314-3316,3342-3344,3382-3389`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3080>)).
Preview is already exact in both OS parser and schema
([`OS directory schema:1935-1956`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1935>)); receipt, page, and approval instead accept any nonempty schema text in both the parser and
the OS JSON schema ([`OS directory schema:2030-2109`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:2030>),
[`OS schema:473-605`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json:473>)).
Make those three OS fields literal types/JSON `const`s and reject foreign tags before the
state-machine transition. The current test already intends this but cannot enforce it because the
parser does not inspect `schema` ([`space artifact creation owner test:2019-2035`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:2019>)).

#### Positive old-wire inventory

Only two OS sources still contain the obsolete normal tags, but they feed multiple positive
lifecycle paths. Every following record must use the current exact tag and an explicit
`proposalHash: null` when no proposal exists:

- The worker HTTP harness factories use the obsolete receipt/page tags and omit a hash whenever
  their optional parameter is absent
  ([`space artifact creation owner test:1997-2017`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:1997>)).
  Those factories cover accepted submit, running poll, cancellation, and reconciliation scenarios;
  they are not hostile fixtures.
- The neutral positive corpus
  [`gis-map-inference-port-v1/🔣️.json`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json)
  contains old tags at lines **187, 256, 280, 315, 350, 489, 513, and 564**. The omitted-null
  positive records are `uncertainLifecycle[3] reconciliation-discovers-original-job`,
  `successLifecycle[1] server-accepts-and-mints-the-job`,
  `successLifecycle[2] first-bounded-progress-page`,
  `successLifecycle[3] monotonic-progress-page`,
  `cancelLifecycle[1] server-accepts-and-mints-the-job`,
  `cancelLifecycle[2] first-bounded-progress-page`, and
  `cancelLifecycle[4] server-confirms-cancellation`. `successLifecycle[4]` has an offered hash
  but still needs the page tag changed.
- The same corpus's `hostileTransitions` has old tags at **655, 689, 811, 909, and 980**. It
  should not retain old tags as accidental compatibility fixtures: update the normal envelope
  tag/null field while preserving each intended hostile transition (wrong job, illegal phase, or
  stale offer) as the sole rejection cause.

The separate `wire.receipt` / `wire.page` fixture is already the correct production shape
(current tags plus explicit `null`) and is used by the direct OS-vs-Hub schema/parser test at
[`space artifact creation owner test:2019-2035`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:2019>).
It is valuable evidence of the intended contract, but current parsers cannot consume that exact
fixture, which exposes the integration failure rather than qualifying it.

#### Bounds and preview/approval parity

- Hub runtime and OS parser agree on the values observed on the real route: Hub ledger admits at
  most 16 monotonic progress cursors and returns at most 8 rows per events read
  ([`Hub SQLite:400-424,654-701`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:400>));
  OS limits cursor to 16, events to 8, and verifies ordered event/progress rows plus
  `completed <= total` ([`OS directory schema:2047-2073`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:2047>)).
  The browser body's shared cap is 16 KiB ([`backbone worker:5077-5082`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5077>)).
- Hub's exported JSON schema is looser than its own live response law: progress cursor and
  `nextCursor` are merely safe integers, total has no positive lower bound, and a normal events
  page allows 16 progress items instead of the route's 8
  ([`Hub schema JSON:370-420`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🔣️.json:370>)).
  This does not make the current Rust route emit an unparseable page, but it lets schema-only
  consumers bless a response the browser will reject. Tightening those declared bounds to the
  route contract is a P1 schema-parity follow-up.
- Preview is correctly optional only as a field: when present it is strict in both owners
  (five closed coordinates, exact `inference-{jobId}` and exact hash binding in OS; runtime only
  emits it for a live, non-cancelled succeeded/offered job)
  ([`OS directory schema:1935-1956,2074-2080`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1935>),
  [`Hub runtime:3307-3313`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3307>)).
  The missing element is only receipt/page/approval exact-tag and nullable-hash parity, not a
  preview format change.
- Approval is always non-null hash and Hub makes it available only after the committed result
  ([`Hub runtime:3358-3390`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3358>)).
  Keep that hash required; make only its `schema` exact on the OS side. Do not incorrectly make
  the approval receipt nullable or accept the old generic tags.

No commands were run for this audit.

### Broker Initialization Must Leave Time for the First Bootstrap `/me`

Current source has correctly separated the private port acknowledgement deadline from an active
broker request: `BrowserBrokerPortClientV1` keeps an initializing request timer at `null`, starts
the two-second request timer only in `dispatch` after a valid `initialized` receipt, and uses a
separate initialization ceiling
([`broker port:3-91`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️session-refresh/🌐️broker-port/🟦️.ts:3>)).
That split is required.  Shell transfers the port immediately after constructing a module worker
([`Shell host:2158-2164`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2158>)),
but `backbone-worker` registers its handler only after its static module graph has evaluated
([`worker:274-289`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:274>)).
`rustHostPromise` is not awaited for that handler, so the cold risk is module fetch/evaluation,
not native host readiness.  A two-second initialization ceiling would falsely reject a valid cold
worker.

The chosen initialization ceiling is currently exactly 120 seconds, which is also the one-use
relay bootstrap-proof expiry.  That equality is unsound as a guarantee: an allowed acknowledgement
at the final initialization instant leaves no budget for the Shell's up-to-two-second cached
identity wait and the first active two-second `/me`; the relay can then reject the first request
before it reaches the upstream.  Retain a long initialization phase, but require the invariant
`initializationDeadline + cachedIdentityWait + firstRequestDeadline + schedulingMargin <
bootstrapProofDeadline`.  A 90-second initialization ceiling is a simple bounded value under the
present 120-second bootstrap and two-second active deadlines.  If the 120-second value is retained,
bootstrap must be raised by at least those downstream budgets; merely naming the two periods
differently is not enough.

The existing MessageChannel law already checks that a queued request emits no `request` before the
acknowledgement and that an acknowledgement dispatches it
([`broker client law:60-113`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:60>)).
Make its timing assertion explicit: advance beyond the two-second active timeout but below the
initialization deadline, then acknowledge and prove exactly one request receives a *new* two-second
deadline.  A separate row must cross the initialization deadline and prove all queued callers
reject with no outbound request.  This avoids a green fixture that merely compares constants.

No commands were run for this audit.

### Five-Second `/me` Refresh Is Liveness Maintenance, Not a Recovery Handoff

The new refresher is correctly single-flight and schedules its next turn only after a successful,
canonical, unexpired exact-200 authority.  Any transport, 401, 428, non-200, malformed body, or
expiry closes it with zero retry ([`session refresh:19-54`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️session-refresh/🟦️.ts:19>)).
That is the right fail-closed response for an invalid broker proof.

It does **not** by itself establish durable browser-proof renewal.  The active proof is only 15
seconds in both the worker ([`worker proof expiry:499-506,613-641`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:499>))
and relay ([`relay active expiry:643-658`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:643>)).
A foreground five-second timer normally advances it, but browser timer throttling/suspension,
long process pauses, or a response lost after the relay advanced its digest can still reach 428.
The refresh module then intentionally stops and the current Shell has no fresh-proof handoff
([`Shell refresh unavailable:2766-2772`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2766>)).

Therefore the proposed 120-second bootstrap phase qualifies only first-page startup; it must not
be described as a background/resume recovery mechanism.  Recovery after active-proof invalidation
needs a future, separately authenticated trusted launcher/host handoff and a new worker-port
generation, or the UI must remain correctly unavailable.  It must never turn the Vite proxy into a
proof issuer.  The immediate implementation should expose a localized unavailable state and retain
sealed old jobs indeterminate; it must not call `takeBrowserBootstrapProof` again from browser
code or retry `/me` after 428.

A future recovery law should explicitly pause the page/worker past the active TTL, resume, prove
that the refresh owner reports unavailable exactly once and no Hub call occurs under a fabricated
successor proof, then exercise whatever distinct trusted handoff is eventually introduced.  No
such recovery is currently qualified.

No commands were run for this audit.

### Browser Broker Bootstrap Must Be Armed After UI Readiness

The two production launcher paths currently start the active 15-second browser-proof timer before
their UI is ready.  `startGisMapShellPeerV1` creates the random proof and arms
`startLocalBrowserRelay` before it spawns the Vite daemon, then waits for UI readiness before it
navigates Chromium ([`Hub runner:11718-11780`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11718>)).
`secure-suite` has the same ordering ([`Hub script:12556-12567`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12556>)).
The relay begins `browserProofExpiresAtMs` immediately at construction
([`relay:570-577`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:570>)), while it
rejects any expired proof before the upstream request ([`relay:643-658`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:643>)).
Consequently a cold Vite build can exhaust the proof before the first Shell `/me`; the Shell's
five-second refresh cannot rescue a proof that was never admitted.

Use a two-phase, private relay lifetime:

1. `startLocalBrowserRelay` starts **unarmed**.  Its in-process return value alone exposes
   `takeBrowserBootstrapProof(): Buffer`; there is no HTTP route, Vite environment value, log, or
   browser-visible configuration that mints a proof.
2. After `waitForUiReadiness`, each launcher invokes that one-shot method, converts the returned
   bytes only to the top-level `#semio-broker=` fragment, and fills the `Buffer`.  The method
   atomically installs only the SHA-256 digest in the relay.  It rejects after issue, after an
   accepted broker request, or after stop.
3. The first matching request consumes the **bootstrap** digest and installs its supplied next
   digest under the existing active ratchet.  A bounded 120-second bootstrap expiry is appropriate
   for cold page/module/React/worker startup; every subsequent proof retains the existing
   15-second expiry.  This is not start-on-first-use: unarmed/expired/replayed bootstrap requests
   return 401 without upstream work.

A 120-second bootstrap phase matters even when issue moves after Vite readiness: the actual
runner permits a 30-second navigation and 120-second Shell-ready wait after the fragment is
created ([`Hub runner:11776-11782`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11776>)).
Starting the regular 15-second active interval merely at Vite readiness therefore still cannot
prove a zero-touch first `/me` on a cold machine.  The bootstrap proof remains bounded and
one-use; the active proof is short-lived as soon as it is consumed.

Do not add `/_semio/.../bootstrap` (or any equivalent proxy route).  The Vite proxy already adds
the relay secret on behalf of every same-origin browser request.  A mint endpoint would thus let a
same-origin hostile shard obtain a fresh proof, defeating the private port boundary demonstrated
by the hostile-shard oracle ([`relay hostile probe:2217-2237`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2217>)).

The Shell currently sends `initialize` but discards the worker's `{kind:"initialized",ok}` receipt
([`Shell broker setup:2200-2211`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2200>));
the worker does emit that exact receipt ([`worker port:676-681`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:676>)).
The broker wrapper must retain one bounded initialization waiter and make the first `/me` await
`ok === true`; `false`, close, timeout, or a replacement port must reject it and leave identity
offline.  FIFO delivery happens to make the current initial route likely work, but it is not a
receipt fence for a fresh handoff.

Required laws: hold Vite readiness longer than the active TTL while the relay is unarmed, then
issue/bootstrap and prove the first `/me` reaches upstream; prove unarmed/expired/replayed
bootstrap never reaches upstream; prove active proof expiry remains 15 seconds after the first
accepted request; prove duplicate issue fails; and prove a hostile shard has neither a proof nor a
mint route.  These are source-level requirements until the registered two-peer process runs.

No commands were run for this audit.

### Shell Must Capture Verified Authority Before Opening an Inference Port

The new refresh owner correctly makes `/me` serial, canonical, exact-200, expiry-checked and
terminal on refusal ([`session refresh:1-56`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️session-refresh/🟦️.ts:1>)).
Its Shell callback updates the verified authority state and correctly avoids persisted identity
events overriding it ([`Shell identity route:2501-2512`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2501>),
[`Shell refresh wiring:2738-2772`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2738>)).

However `requestInferenceProposal` currently requires only a persisted `identityRef`, not an exact
current `verifiedSessionAuthorityRef`; it creates the mailbox and sends `inference-open` before
any session binding/generation is captured ([`Shell inference open:4298-4320`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4298>)).
The present worker admits `inference-open` from a writable execution lease alone
([`worker inference admission:5228-5244`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5228>)).
Thus an identity snapshot can enable an inference-opening effect before `/me`; the Shell may show
an opened port even though the planned broker-operation fence will later refuse all authenticated
work.

Require the Shell to capture `{sessionBindingSha256, authorizationGeneration}` from the exact
verified authority before it creates the opening mailbox.  Recheck that tuple after the opening
receipt and at every propose/cancel/approve/Undo dispatch.  On unavailable or replacement it must
close the exact pending mailbox and clear the current presentation immediately, while the worker
retains any already-sealed job as indeterminate.  The worker-side session-operation fence remains
the authoritative no-successor defense; this Shell gate prevents an unauthorised pre-`/me` UI
admission and stale control presentation.

Add a Shell law with a persisted identity and no accepted `/me`: an inference effect emits no
`inference-open`.  Add a second row that holds the opening receipt, rotates the verified binding,
then releases it and proves no `OPEN_INFERENCE_PORT`/propose dispatch.  Keep an already-submitted
old request in the worker's indeterminate capacity slot, rather than silently replacing it.

No commands were run for this audit.

### Uninstalled `/me` Authority Lets a Retained Inference Cross a Proof Replacement

The port-detach repair is effective for a delayed `/me` body: it rotates the admission, consumes
the proof, aborts all tracked controllers, and its new law holds the old stream through replacement
([`backbone worker:660-710`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:660>),
[`authority port law:2377-2406`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:2377>)).
The exact-200 restriction is also present before any authority parse
([`backbone worker:556-575`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:556>)).

There remains a distinct proof-to-operation boundary failure when the first `/me` has **not yet**
succeeded.  `retireBrowserSessionAuthority` returns immediately when
`browserSessionAuthority === null`; therefore proof invalidation/replacement does not increment
`directorySessionEpoch`, close a port, or retire an Undo/artifact owner in that state
([`backbone worker:536-554`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:536>)).
But opening an inference requires only the live writable execution-target lease, not an installed
`browserSessionAuthority`, and stores only that mutable directory epoch
([`backbone worker:5119-5129,5215-5234`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5119>)).
Its submit/reconcile/poll/cancel route consequently treats the operation as current whenever that
epoch is unchanged ([`backbone worker:5135-5153`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5135>)).
The existing inference suite itself demonstrates that inference mechanics can run with a verified
lease without first installing a broker authority; ordinary rows open and submit directly through
the worker harness ([`owner test:2073-2088`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:2073>)).

Interleaving: a leased document opens/submits an inference before `/me`; the broker proof is
consumed for the submit, then transport/proof replacement makes it indeterminate; a fresh proof is
installed.  Because there was no accepted authority, `clearLocalBrowserBrokerProof` does not
advance the operation epoch.  The scheduled original reconciliation has
`operation.sessionEpoch === directorySessionEpoch`, so it can call `browserBrokerFetch` with the
fresh proof ([`backbone worker:5194-5207,5273-5319`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5194>)).
That violates the stated no-old-uncertain-inference-under-successor-credential property.  The
current authenticated-replacement law begins by successfully calling `/me`, so it cannot cover
this initial-authority window ([`owner test:2277-2317`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:2277>)).

The correct owner is a **broker-admission/session binding**, not merely `directorySessionEpoch`.
Make every broker invalidation/replacement advance one session-operation fence even if `/me` is
currently null, and capture that fence (then, once the planned authority handoff is connected, the
exact `sessionBindingSha256` plus generation) in inference/Undo and every authenticated retained
operation.  A mismatched fence must leave a sealed request indeterminate and block all
submit/reconcile/cancel/approval calls; it must never merely sample the current proof.  Do not
solve this by letting an operation silently adopt the new `browserSessionAuthority`.

Add a worker law with no initial `/me`: establish a writable lease, submit a sealed request until
its response is lost, rotate/install a fresh proof, and drive its old epoch.  Assert there is no
second `/jobs`, no `/reconcile`, `/cancel`, or approval request under the successor proof, and that
the original remains indeterminate/capacity-blocking.  A second row should establish `/me` first
and prove the same result, preserving the existing covered path.

No commands were run for this audit.

### Broker-Port Replacement Leaves an Old `/me` Turn Authorized

The new worker-side authority install correctly gives every broker request an admission object,
serializes it, bounds its body to 2 KiB/2 seconds, and makes an authenticated replacement retire
the old Directory/inference/Undo/artifact owners
([`backbone worker:530-655`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:530>)).
That protects a queued request after a *proof* replacement.  It does **not** protect a request
after a broker-port replacement.

`attachLocalBrokerPort` explicitly supports replacement by closing the preceding port, but neither
rotates `localBrowserBrokerAdmission` nor aborts the old RPC controllers
([`backbone worker:659-695`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:659>)).
The older `browserBrokerFetch` therefore retains the same admission object through its delayed
fetch/body read and `acceptBrowserSessionAuthority` accepts and assigns
`browserSessionAuthority` after the successor port is already installed
([`backbone worker:598-646`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:598>),
[`backbone worker:556-574`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:556>)).
The result is sent only to the closed old port, but the security-relevant worker mutation has
already occurred.  The present Shell creates its production port once, so this is a replacement
path defect rather than evidence that the ordinary initial bootstrap currently takes the bad
branch; the replacement code itself establishes that this is a supported lifetime boundary.

Required atomic boundary: when a distinct replacement port is accepted, first rotate the broker
admission, abort and remove every old-port request controller, consume the current proof, and
retire accepted session authority; only then close/set/start the new port.  The successor must
initialize with a new proof.  Do not simply compare `localBrowserBrokerPort` at `postMessage`:
that would hide the response while leaving its authority side effect live.

Add one deterministic worker law: hold the old `/me` body after its request has consumed the
proof, attach a second port, release the body, and assert that (a) no authority is installed or
replaced by the old response, (b) the old request cannot advance a proof for the successor, and
(c) only a fresh successor `initialize` followed by `/me` can install its authority.  This is the
missing stale-response case alongside the existing proof-replacement/queued-turn laws in
[`space-artifact creation owner test:2280-2334`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:2280>)).

As a smaller closed-route tightening, this `accept` callback is used only for `GET
/auth/sessions/me`, whose Hub route returns `200` on success
([`Hub route:6515-6531`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6515>)).
The bounded reader already rejects non-OK responses, but permits another hypothetical 2xx status
with a canonical authority body.  Require `response.status === 200` before parsing so a future
relay/route drift cannot establish a session through a non-contract success response.  This is a
P1 contract hardening; the port-replacement interleaving is the substantive stale-owner defect.

No commands were run for this audit.

### Two-Author Browser-Host Receipt Does Not Bind Its GIS Module Inputs

After the locale selector repair, the next concrete acceptance-proof gap is in the ticket-owned
browser-host staging receipt.  The runner correctly hashes the selected *source* GIS component
and descriptor before and after materialization
([`OS dev staging:461-464,483-486`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:461>)),
and it stages a closed module tree.  But the receipt used to authorize Vite contains only
`selectedGis.generationId` and `selectedGis.currentSha256`; its only per-module byte identities
are for the freshly compiled **Space** host
([`browser-host receipt type/parser:18-78`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🟦️.ts:18>)).

This is materially weaker than the later mounted-map assertions.  `closeTestBrowserHostStagingV1`
validates the Space descriptor/core identity, then incorporates GIS only into an aggregate module
set hash; it does not parse or rehash the staged GIS descriptor against the selected
`componentSha256`/`descriptorSha256`
([`browser-host close:169-210`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🟦️.ts:169>)).
`resolveTestBrowserHostRootsV1` repeats exactly the Space-only identity check
([`browser-host resolve:229-251`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🟦️.ts:229>)).
The two-author runner subsequently compares the **server/open-plan-derived** mounted probe
fields to selected-current metadata, not a GIS identity attested by the Vite host receipt
([`Hub runner:11883-11895,11968-11991`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11883>)).

Consequently a closed staged module set can be internally immutable while its GIS module/bridge is
not demonstrated to derive from the exact selected component and descriptor.  This does not show
that the current staging code substitutes bytes; it means a passing mounted journey must **not**
claim that stronger browser-byte provenance yet.  The existing unit hostile substitutions only
replace Space files after closure, so they do not exercise this missing GIS input binding
([`browser-host staging test:88-105`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts:88>)).

The smallest reliable remedy is an explicit GIS materialization receipt written by the same
component transpilation owner and incorporated into the host receipt: selected component SHA-256,
selected descriptor SHA-256, staged descriptor SHA-256, bridge SHA-256, and the generated core
file digest(s), all verified before Vite serves.  A parser/hostile law must reject (1) a selected
hash changed in the receipt and (2) a staged GIS descriptor/bridge substitution, independently of
the aggregate module-set hash.  Do not pretend an aggregate self-hash establishes an input/output
derivation.  This is an acceptance qualification blocker, not evidence that a presently mounted
run has used substituted bytes.

No commands were run for this audit.

### Two-Author Shell Process: German Existing-Artifact Open Selector

The registered `--two-author-shell` route is an actual-process candidate, not a qualified run in
this audit.  It does materially stage selected-current GIS bytes, start a native Hub at the
prepared same data root, launch Chromium, use local credential envelopes, create A's document
through the ordinary Shell, then mount B and both restart peers
([`Hub script:11866-12047`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11866>)).
Its MCP checkpoint/socket assertions and the mounted `UiDocumentStore` probe are useful actual
process evidence **only when the command runs**; the adjacent `--two-author-source` fixture
checker is explicitly a source oracle and cannot qualify that journey
([`Hub script:12659-12711`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:12659>)).

The first independently reproducible harness defect is the existing-artifact open control.  The
runner deliberately locks B to German, checks `html[lang="de"]`, and runs B both on the initial
peer join and after restart ([`Hub script:11761,11896-11900,11963-11966`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11761>)).
Yet its existing-document branch always asks Playwright for an exact English accessible name:

```ts
artifactRow.getByRole("button", { name: "Open", exact: true }).click()
```

([`Hub script:11792-11797`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11792>)).
This is not a theoretical translation choice: the real UI vocabulary maps the `open` command to
English `Open` and German `Öffnen`
([`React UI target:2211-2246`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:2211>)).
The very same runner already chooses the locale-specific space button (`Open Studio` vs
`Studio öffnen`) ([`Hub script:11762-11765`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11762>)).

Therefore the documented German second peer cannot truthfully complete its ordinary document-open
path: a properly localized UI causes the candidate to time out, while an English-only button would
mask the localization contract breach.  This is earlier than checkpoint-to-probe correlation
(owned separately) and unrelated to progress evidence.

The smallest sound repair is for this test-only process owner to select the existing artifact's
stable action identity scoped to the already exact `[data-row-id="artifact:<documentId>"]`, if the
row exposes one; otherwise it must take the closed locale label from the same two-language test
contract (`Open` / `Öffnen`) and add a source/DOM hostile that swaps the locale label.  Do not
weaken it to a text substring or a global button lookup: that could activate a different row or a
stale dialog.  The neutral fixture already commits to A=`en`, B=`de` and both initial/restart
mounts, so add this control identity/locale expectation there as part of the registered route
([`two-author fixture`](</Users/ueli/Documents/semio/🌎️hub/🧪️fixtures/🤝️two-author-shell-v1/🔣️.json)).

No commands were run for this audit.

### Nx `css-color` Graph Failure Is a Cascaded Missing External-Node Set

`Source project does not exist: npm:@asamuzakjp/css-color` is not evidence of a corrupt Bun
installation or an absent lockfile package.  The current root lock contains the package at
`3.2.0`, its installed package directory exists, and the installed Nx Bun parser independently
produces both `npm:@asamuzakjp/css-color` and its versioned node from that lock.  No package,
lockfile, `node_modules`, Nx cache, or daemon mutation is warranted.

The graph configuration deliberately disables the built-in `@nx/js` lockfile producer
([`nx configuration`](</Users/ueli/Documents/semio/nx.json)), so the custom
`@repo/emoji-project-json` plugin is the sole source of Bun external nodes.  Its node producer
adds the full Bun external-node result only after it has mapped *every* project/config input
([`custom nodes:821-881`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:821>)).
If any project setup import throws first, this aggregate create-nodes operation returns none of
those external nodes.  Nx nevertheless invokes `createDependencies` concurrently; the custom
lock model then emits an ordinary external-to-external edge such as `css-color → css-calc`
([`custom dependencies:885-962`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:885>)).
The Nx graph builder correctly rejects that edge because its source node was never installed.
Thus `css-color` is the follower error; the earliest `AggregateCreateNodesError` stack is the
actionable cause.

The previously cached graph identifies that first error as the removed sibling fixture import
`./🔣️.json` in the ownership field-parity test.  Current source instead points at the existing
canonical `../../🧫️fixtures/🪪️field-parity/🔣️.json`
([`field parity test`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts:6>)).
The observed graph failure predates that repair; a new graph is necessary to establish whether it
was the last create-nodes exception.  The minimal safe retry is exactly one `NX_DAEMON=false`
owner with a fresh ticket-local `NX_WORKSPACE_DATA_DIRECTORY` and verbose producer capture.  The
current repository bootstrap wrapper preserves a supplied workspace-data directory
([`Nx bootstrap:50-53`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:50>));
there is no need to bypass the registered runner.  Keep intentional dependent commands serialized
within that one directory, but do not share it with unrelated graph producers.

No commands were run for this audit; the parser/lock checks were read-only Node inspections.

### Native GIS Inference Hub DTO Ingress: Nullable Hash Coverage and Approval Contradiction

The current receipt and event-page DTOs have the intended *structural* ingress fence.  Both
native OS types use `#[value(rename_all = "camelCase", deny_unknown_fields)]`, and both make the
otherwise-defaultable `Option<String>` hash explicitly present with `#[value(required)]`
([`receipt/page definitions`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2033>)).  The shared `FromValue` derive emits an unknown-key
loop and, independently, an error for a missing required field
([`derive expansion`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:464>),
[`required-field branch`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:515>)).
Present `null` decodes to `Option::None`; any non-string/non-null `proposalHash` reaches the
`Option<String>` decoder and fails.  Nested event/progress/preview and undo shapes are likewise
closed, and the subsequent validators enforce exact current tags, the 8-event/16-progress
bounds, coordinate equality, and preview ownership
([`page validation`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2096>)).

The actual native response ingress is not the pure reducer: every public `DirectoryClient` route
byte-caps the body, decodes through `os_pack::json::from_json_str`, then validates *before it
returns a DTO* to any caller.  This covers submit, events, cancel, and approval
([`client ingress and public fences`](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1040>)).
Thus a substituted receipt/page schema is not applied: it may structurally decode, but the
public method returns `inference.invalid`.  Conversely, the Hub has no receipt/page/approval
receipt **request** ingress: its real inbound JSON is the submit and approval request body, both
direct `serde_json` closed request decoders with their own schema/version/hash checks
([`submit request decode`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🦀️.rs:129>),
[`approval request decode`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/✅️approval/🦀️.rs:7>)).
The Hub only *emits* the three DTOs, with the exact current tags on every submit/read/cancel and
approval construction path ([`runtime emission`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3244>),
[`page/cancel emission`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3270>),
[`approval emission`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3338>)).
Neither nullable `proposal_hash` field has `skip_serializing_if`, so `None` serializes as the
required JSON `null`; only the independently optional page `preview` is omitted.

The queued native law correctly establishes a positive receipt/page round trip and missing-hash
rejection, but it does not yet prove the whole actual ingress contract.  Its present scope is
only receipt/page, and it tests substituted tags only by validating already-created values
([`current law`](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:42>)).
The existing public-client law proves a wrong receipt tag and a wrong page job id, but has no
approval, wrong `proposalHash` type, missing page tag, or unknown-field row
([`current client law`](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs:571>)).

Minimum native completion packet for the current neutral corpus:

- Extend its `wire` section with one exact valid `approval` receipt (including the closed undo
  handle), rather than relying on the Hub-only reconcile fixture.  Keep receipt/page
  `proposalHash: null` as the positive nullable rows.
- For receipt and page, execute the real `os_pack::json::from_json_str` path for: missing
  `proposalHash`; `proposalHash: false`; `proposalHash: []`; an extra outer field; and the other
  DTO's schema tag.  Require decode failure for the first four and decode-plus-`validate` failure
  for the tag substitution.  Apply equivalent wrong-tag/type/unknown-field tests to the approval
  receipt and its nested `undo` object.
- Feed the same hostile bodies through the public `DirectoryClient` submit/read/cancel/approve
  methods with `FakeTransport`, asserting `inference.invalid`; this establishes rejection before
  the native application/reducer, not only a DTO round trip.  Retain the existing response byte
  ceiling test separately.

There is one concrete approval-contract defect to resolve before adding that positive approval
row.  The published JSON schema intentionally permits `applied: false`, and the Hub's durable
reconciliation deliberately returns it on an already-committed outbox after it verifies the
retained undo witness ([`committed branch`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:934>)).
That value propagates directly into a successful `200` approval receipt
([`approval route result`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3337>)).
But the OS native receipt validator unconditionally requires `self.applied`
([`native approval validator`](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2195>)),
so `DirectoryClient::approve_gis_map_inference_job` converts that legitimate reconciled 200 into
`inference.invalid`; the TypeScript reducer instead accepts the schema and maps `false` to its
commit-unavailable terminal.  Pick one coherent contract before native acceptance:

- if a 200 approval receipt is always durable success, normalize/relabel the Hub's committed
  recovery result as successful (or express replay separately) and make `applied` a schema
  constant true; or
- if `applied:false` remains a valid 200 outcome, remove the native `self.applied` rejection and
  specify the shared terminal/Undo semantics consistently across Rust and TypeScript.

Do not silently loosen the tag, owner, mutation/hash, or undo-frontier checks in either choice.
Add an exact committed-replay transport law that proves the chosen result instead of treating a
locally constructed `applied:false` object as sufficient.

A read-only literal search found **no** remaining OS/Hub occurrence of the obsolete
`semio.hub.inference-receipt/v1` or `semio.hub.inference-events/v1` tags.  The current positive
wire fixture carries both required nullable hashes
([`neutral wire`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json:4>)).
The current positive code/fixture users have the `inference-job-*` tags; the coverage gap is
approval absence from that neutral `wire` vector, not a stale tag or omitted nullable field.

One deliberately narrow residual: the OS response decoder's JSON `Object` overwrites duplicate
known keys before `FromValue` sees them ([`last-value-wins object`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:166>),
[`generic bridge`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1440>)).
`deny_unknown_fields` therefore rejects an extra *name* but cannot detect a duplicate allowed
name.  This does not affect Hub-generated responses, and it is outside the currently declared
JSON-schema contract, but a future claim of raw closed-response parsing requires a Pack-owned
duplicate-key-rejecting parse option plus a client ingress test; schema-only hostiles cannot
establish it.

No builds or source edits were performed for this audit.

### Ordinary GIS Creation Loses the Selected Catalog Generation Before Hub Admission

The ordinary browser route has a real selected-current presentation fence, but its client intent
does not carry the generation that fence selected.  The Space index receives a catalog containing
`catalogGenerationId`; `spaceArtifactCreationRequestFromAction` compares the captured and live
generation and membership, then returns only `{ requestId, spaceId, kindId, name }`
([`Shell request projection`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1242>)).  The worker serializes that same four-field payload
([`worker submission`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4201>)), and the shared TypeScript/Rust
`SpaceArtifactCreateV1` deliberately admits precisely `schema`, `requestId`, `kindId`, and
`name` ([`shared schema twin`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts:12>),
[`native twin`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs:103>)).

Consequently a trusted-current rotation between the browser's last local comparison and Hub
`accept` changes what is created.  `ArtifactCreationServiceV1::accept` selects from the catalog
that is current **when the POST arrives** and records that generation in the durable intent;
there is no equality check against a client-selected generation
([`admission selection`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:49>)).
The later `catalog_matches` checks correctly prevent a G1 accepted operation from materializing
after a subsequent G2 rotation, but they cannot prevent a G1 dialog from being accepted as G2 in
the first place ([`execution revalidation`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:45>)).

This is a real selected-trusted-GIS semantic mismatch, not an executable-authority injection:
G2 can be valid and native-verified, but it is not the catalog row the author saw and chose.
The durable Ready status likewise exposes only the resulting kind/schema/dialect, not the
intent's generation ([`ready/status surface`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts:21>)).

The two-author process candidate hides this interval rather than qualifying it: it deliberately
keeps the publication pointer byte-identical through `assertGisMapCompositionCurrent`, then has
A choose only the `kindId` from the ordinary dialog
([`current assertion`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11947>),
[`ordinary selection`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11840>)).  It is an
unexecuted actual-process candidate in this audit; even a future positive completion would prove
the stable-current case only, not this race.

Minimum fail-closed packet:

- Make `catalogGenerationId` a required 64-hex *expected selection* in `SpaceArtifactCreateV1`,
  its canonical TS/Rust parsers, worker request codec, and command digest.  This is not a
  client-supplied factory/descriptor/space authority: Hub must require exact equality with the
  active verified catalog before minting an intent or document ID.  The existing neutral
  `injected-generation` negative row must become a valid matching expected-generation row; add
  stale, zero, malformed, and unknown-field rows rather than retaining a false claim that an
  expected generation is forbidden ([`neutral corpus`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json:1>)).
- Carry the accepted generation in every durable creation status (or at minimum every Ready
  status) and require the retained Shell owner/worker operation to compare it to its captured
  request before it schedules `openReadySpaceArtifactCreation`.  This makes the browser's
  ordinary Ready-to-open handoff independently auditable instead of inferring provenance from a
  later global mounted probe.
- Add a native transaction pause immediately before selection/claim: fetch G1 catalog, submit a
  G1 request after replacing active catalog with a different G2, and require conflict with no
  accepted fact, no `DocumentAnnounced`/`DocumentIndexed`/`CheckpointPublished` triple, no CAS
  reference, and no Directory artifact row.  Keep the existing post-accept catalog-change test:
  it covers a distinct later failure mode.
- Add a Shell/worker law where a G1 choice is captured, G2 arrives before dispatch, and the POST
  body remains G1; the Hub refusal must yield no Ready/open attempt.  In the registered
  `--two-author-shell` route, pause after A sees the catalog and rotate before its actual POST;
  the negative must produce no artifact row/probe.  The existing positive must assert the same
  generation in A's creation status, the B reopen plan, and both mounted probes.

No commands, browser sessions, builds, or source edits were performed for this audit.

### Catalog-Generation Creation Fix: Exact Propagation Inventory

`catalogGenerationId` already has a single appropriate public shape: a canonical lower-case,
non-zero SHA-256 identity (`^(?!0{64}$)[0-9a-f]{64}$`).  Reuse the `digest` check in the shared
creation schema and the native creation `hash`/`ArtifactHash` check; do not reuse the generic
worker `workerWireSha256V1`, which currently permits all-zero hashes.  This field is an expected
catalog-selection precondition, not a client-selected package, executable, descriptor, scope, or
document ID.

The implementation boundary is below.  Fields named **request** must be extended in the same
canonical order in TypeScript and Rust; `JSON.stringify` and the Rust canonical serializer use
that declaration order, so every raw JSON literal and command digest must be deliberately
regenerated rather than accepted under an alternate ordering.

| Owner | Required change |
| --- | --- |
| Shared creation contract | Add required `catalogGenerationId` to `SpaceArtifactCreateV1`, `sealSpaceArtifactCreateV1`, and exact request fields in [`space-artifact-creation-v1/🟦️.ts:12-129`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️:12>) and its Rust `SpaceArtifactCreateV1::validate` twin ([`🦀️.rs:103-129`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️:103>)).  Add it to the generic `$defs/SpaceArtifactCreationRequest` required/properties list ([`directory schema:3457`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json:3457>)). |
| Hub acceptance and durable key | In [`ArtifactCreationServiceV1::accept`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:49>), compare request generation with `self.catalog.generation_id()` **before** entropy/mint/claim.  Keep the existing stored `intent.catalog_generation`, but ensure it is copied from the matched request/current value.  `artifact_creation_command_digest_v1` already hashes canonical request bytes ([`creation schema:207`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:207>)); it changes automatically once the request wire is complete. |
| Hub HTTP route | [`post_space_artifact_creation`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4947>) needs no separate authority parameter: its canonical `SpaceArtifactCreateV1::parse_canonical_json` already reaches `service.accept`.  The route law must prove a G1 request after active G2 gets conflict *before* task activation. |
| Durable status correlation | `ArtifactCreationOperationV1` already retains `intent.catalog_generation`, but its `status()` discards it ([`creation schema:290`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:290>)).  Add required `catalogGenerationId` to shared `SpaceArtifactCreationStatusV1`, every canonical parser and generic status schema.  Update all three native status construction sites: `ArtifactCreationOperationV1::status`, the `intent.validate` synthetic Accepted status, and service's manual Indeterminate response ([`service:152`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:152>)). |
| OS worker wire | Extend `BackboneWorkerRequest` create arm, strict request key set/parser, and returned object in [`os/🟦️.ts:918-929,1121`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:918>).  Extend strict worker-visible creation status parsing/types ([`os/🟦️.ts:933-957`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:933>)).  In the worker, compare a received status generation to the retained operation request before posting it, and include it in duplicate-request equality ([`backbone worker:4145-4256`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4145>)). |
| Shell selected-row owner | Keep the existing captured/live catalog membership comparison, but return `capturedCatalog.catalogGenerationId` in the request ([`ShellHost:1242-1261`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1242>)).  Store it on `SpaceArtifactCreationOwnerV1`; `spaceArtifactCreationOwnerAcceptsStatus` currently checks only request/space/kind and must require status generation equality before Ready can trigger open ([`ShellHost:1264-1295`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1264>). |

There is already a later, independent document-open generation fence: a genuine open plan carries
`catalog.generationId` ([`DocumentOpenPlanV1`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1186>)), and Hub resolves the descriptor against current
catalog selection before minting it ([`open-plan selection`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2608>)).  It does **not** receive the creation owner’s expected generation, however.  Thus it can fail a later stale opening, but cannot repair the pre-accept G1→G2 substitution.  A status/owner equality check is still required; changing the generic document-open intent is not needed for this narrow repair.

Handcrafted current four-field request sites:

- The shared neutral corpus has 14 `requests` values and eight request `rawJson` literals that
  must gain the canonical field while retaining the original hostile reason.  In particular,
  retain exactly one new *missing-generation* negative; do not accidentally turn every existing
  malformed-name/order/duplicate case into the same missing-field rejection
  ([`fixture request rows`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json:5>),
  [`raw literals`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json:488>)).
- [`Hub script:1338`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1338>) creates a real genesis through direct HTTP.  It must first read the selected creation catalog and seal its returned generation; retaining the current direct three-field helper would make the registered checkpoint/process fixtures invalid.
- The two direct Rust constructors are the native HTTP route law at
  [`Hub bin tests:191`](</Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:191>) and the persisted-genesis fixture helper at
  [`Hub bin tests:764`](</Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:764>).  The latter already receives `catalog_generation`; set the request field to that exact value, so its static command digest is recomputed through the shared helper rather than hand-copied.
- The durable operation fixture request needs the field, and its literal `commandSha256` must be
  regenerated from the canonical completed request—not preserved from the four-field input
  ([`operation fixture`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🌱️creation/🧫️fixtures/📚️operation-v1/🔣️.json:12>)).
- Update the Shell engine expectation ([`engine contract:673`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:673>)), the worker ownership cases ([`space creation owner:447`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:447>)), and the wire round trip ([`backbone envelope:2107`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts:2107>)).  The catalog values already present in these test fixtures provide valid non-zero values; do not invent a second generator.

Required tests beyond parser/schema parity:

1. Native route/service gate: G1 catalog fetch, pause before `accept` reads selection, replace with
   a descriptor-distinct G2, submit G1.  Require 409, no durable accepted fact, no execution
   reservation, no CAS ownership, no event triple, and no indexed row.
2. Worker/Shell gate: encode/decode preserves non-zero G1; a G2 status for a retained G1 request
   is dropped before UI/open; a same-G1 Ready reaches exactly one opening attempt.
3. Registered two-author process negative: pause after the ordinary dialog's catalog response and
   before its POST, rotate selected current, then require no new artifact row or mounted map.  The
   existing positive should compare its A creation status generation, both open-plan/mounted probe
   generations, and prepared current.  This remains source-only until the registered browser/native
   command emits its completed receipt.

No commands, builds, or source edits were performed for this inventory.

### G1 Ready to G2 Open: Existing Fail-Closed Paths and the Missing Immutable Correlation

After a legitimate G1 creation is Ready, the present open path does **not** retain G1 as an
opening constraint.  `openReadySpaceArtifactCreation` derives only
`artifactRef/documentId/spaceId/schema`, creates a private app, and calls `openDocument`; neither
the opening arguments nor `DocumentOpeningReceiptV1` contain a creation generation
([`Ready projection`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1285>),
[`Ready open call`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6356>),
[`opening receipt`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts:13>)).

Hub's generic document-open boundary is correctly current-selection based, not creation-selection
based.  It resolves the persisted descriptor against **current** catalog G2 when it issues a plan
([`open-plan selection`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2403>)); the plan does carry its
own G2 `catalog.generationId` ([`DocumentOpenPlanV1`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1186>)).
The worker verifies the plan/lease coherence, but has no G1 input to compare it with.

This produces three distinct outcomes:

- If G2 no longer provides an exact selection for the G1 descriptor owner/hash/schema, plan issue
  is `component-unavailable` (503).  The Ready target is not published; the existing opening
  runner releases the private app and leaves Ready retryable.
- If G1 plan issuance succeeds and G2 replaces current before socket-grant exchange, Hub compares
  the retained plan authority generation with current and returns `stale` (409)
  ([`exchange revalidation`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2522>)).  If replacement instead occurs before
  execution-target reads, the target route reselects current assets and the worker's plan/lease
  equality fails.  These are already fail-closed.
- If G2 is descriptor-compatible (for example a republished catalog generation retaining the same
  exact GIS package/component/descriptor), plan issue and mount legitimately proceed under G2.
  No current line compares that mounted G2 identity to the G1 creation intent, so a private app can
  reach `commit` under G2.  This is the remaining immutable-generation mismatch.

Do **not** solve this by adding a generation selector to generic `DocumentOpenIntentV1`: it would
turn a server-resolved descriptor route into caller-selected catalog authority.  The smallest safe
correlation is entirely inside the creation-specific Shell owner:

1. Retain required `catalogGenerationId` on `SpaceArtifactCreationOwnerV1` and require the Ready
   status to equal it, as in the preceding packet.
2. Add a private `expectedCatalogGenerationId` only to the prepared target passed by
   `openReadySpaceArtifactCreation` into its own `openDocument` attempt.  It is an assertion,
   never serialized into the Hub plan request.
3. Have the attempt wait for the exact acknowledged `browser-actor-ui-mounted` identity and
   compare its `catalogGenerationId` with that private expected value **before**
   `runDocumentOpeningAttemptV1.commit`.  On mismatch, throw through the existing no-commit path;
   it closes/detaches the exact runtime, retires the port, destroys the unactivated app, and keeps
   the durable creation at Ready with `open-failed`.

The third point must not use `entry.ready` alone.  That promise has two successful writers:
`bindDocumentBackbone` resolves it after a document-port bind
([`port bind`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2023>)), while the
actual mounted identity resolves it later from a validated `browser-actor-ui-mounted` message
([`mounted handler`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2203>)).
Use a second exact mounted-identity promise/value on `OpenDocumentSession`; reject it in the same
close/fault paths that currently reject `entry.ready`.  Otherwise a port-first turn can commit
before any catalog identity exists, defeating the proposed check.

The new expected-generation POST mismatch has a simple current HTTP outcome.  A service
`DirectoryError::Conflict` maps to a bare 409 ([`route mapping`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4907>)); it is not a creation
status body.  `driveSpaceArtifactCreation` maps every 4xx to a local `failed` status without
attempting to parse a body ([`worker 4xx path`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4201>)).
That safely prevents Ready/open, but it leaves the old G1 catalog presentation retained: catalog
fetch is only opened on the Space mounting path, not after failed creation
([`catalog open call sites`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4105>)).
The creation-specific Shell status handler should therefore invalidate its exact G1 presentation
and ask the worker to refetch the catalog for that exact retained Space-index client after this
terminal failed result.  It must not resend the G1 request or auto-create under G2.  A subsequent
human action uses the fresh G2 row.

Bounded laws:

- G1 Ready + compatible G2 mount: port becomes ready first, then G2 `browser-actor-ui-mounted`;
  assert no commit/publish and exact close/detach/destroy once.  The G1 version commits once.
- G1 plan → G2 before grant and G1 plan → G2 before asset read retain existing Hub 409/lease
  mismatch refusal; add an explicit no mounted-identity/no Shell publication assertion.
- Expected-G1 creation POST after G2: assert bare 409, no status decoder invocation, no durable
  creation fact; then assert one catalog refetch and no automatic POST.  The refreshed G2 choice
  requires a new user action.

No commands, builds, or source edits were performed for this audit.
