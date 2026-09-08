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
