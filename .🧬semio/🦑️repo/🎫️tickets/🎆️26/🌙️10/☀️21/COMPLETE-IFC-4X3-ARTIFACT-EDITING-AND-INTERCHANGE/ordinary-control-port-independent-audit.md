# Ordinary Control Port Independent Audit

Read-only review of Interpreter/control-port TS/TSX, input-views, display-views, action-dispatch, stable DOM IDs and node-tree routing. Root reports pure12tests/43assertions plus narrow strict check. Actual renderer execution remains unavailable under Explicit Vitest policy required; no override, fake registration or native/browser proof is claimed.

No new confirmed P1/P2 was established in this narrow source review. documentControlContext retains exact real store identity, reads current resident nodes without graph copies and refuses stale record identity, disabled nodes and ambiguous duplicate trigger bindings before buildIntent. It preserves the real store sequence/revision mint and exact sink Promise/failure. Node-tree switches ordinary components through this adapter; other scene/container routes remain their existing interpreter path.

Button callbacks reread exact current record, require reference equality and type, and use source/owner/key-scoped pending state. A Promise from an older pending state cannot clear a newer state. Unmount suppresses state updates. DOM IDs derive actual source surface plus scope/key/id rather than a fabricated store projection.

Input blur behavior still uses useCommitDraft and the original commit lane: current input is reread, disabled/kind/draft-target checked, active draft proof checked, subscription combines actual publication and draft lifecycle, and commit remains a commit trigger. Number/color continuous edits retain the long-lived lane/ref and existing gesture payload, release and retirement abort behavior. Presentation override reads only scalar value; document adapter returns undefined, preserving existing document display. Select and other immediate controls dispatch the rendered record, so the adapter rejects obsolete callbacks instead of rebinding old args to a replacement.

## Boundaries to preserve in the owned implementation

The current document adapter is not an owned-read authority. Its exact getNodeSnapshot comparison is suitable for UiDocumentStore but cannot be replaced by canReadSnapshot alone: active older lease roots are deliberately retained. Owned callbacks must compare exact latest subscription/publication and synchronously consume reserved binding under that live lease. No borrowed resident args may escape to a generic async fallback without original owned input custody.

Keep stable controls.identity across context objects and per-control renders; the new adapter uses real store object for this purpose. OrdinaryControlView also exposes continuous controls; a staged-only owned port must explicitly refuse unsupported triggers/payloads instead of treating all ordinary controls as staged text.

## Required executable follow-up

The pure adapter vectors cover stale/removed/disabled/ambiguous bindings, exact args identity and Promise propagation, but not React lifecycle. Run existing Interpreter continuous-press, number-keyboard and commit-draft suites through actual renderer React long/exhaustive policy once the sanctioned policy is available. Add authentic owned wire Input/Select/Button fixture covering old snapshot callback after replacement, disabled replacement, pending button owner switch, blur during publication replacement, continuous release/retirement, text-preserving staged refresh and structural page replacement. Use existing OwnedRootSource fixture and acknowledgement path; do not loadSnapshot/project a complete tree as owned proof.
