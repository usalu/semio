# Selection Repair Scope and Invariants

Read-only source review, 2026-11-04. No tests launched and no implementation changed.

## Established Scope

All ten EN1990–EN1999 editor main modules construct `LiveView::<Family>::with_lanes` with the persisted checks config index, current picked addresses and current transient lane. The shared `with_lanes` now restores explicit `transient.view.check` priority using `resolve_with_priority(..., true)` after the ordinary owner-aware resolution. Persisted check fallback remains owner-filtered; explicit interaction check-row picks also retain priority. This correctly separates the two currently available sources of check choice.

## Concrete Remaining Coherence Issue

The explicit-lane override preserves `selection.picked` from the old interaction entity owners. `LiveView::checks_view` always uses these owners to narrow the displayed checks. Keyboard stepping in `view-steps::resolve_view_step` walks the default whole-report ordering (not this owner-filtered list) and emits only `LaneEvent::ViewCheck`. Therefore with member b1 picked, a step to a c2 check can select/focus c2 while checks remain narrowed to b1 and omit the selected row. Preserving explicit check priority alone does not restore the all-windows-consistent invariant. Reconcile the checks owner filter with explicit lane selection or define stepping over the actual visible list, and test both next-check and governing-check across owners. This finding was sent to the visual agent.

A further temporal behavior needs an explicit fixture: pick b1, step to c2, then pick b3. A retained transient check remains explicit for every later paint until a lane event clears it; the pure resolver has no information about which entity pick occurred later. Establish through the host route whether a fresh pick clears/replaces the lane before claiming latest-user-intent priority.

## Read-Only Viewer Bypass

Every one of the ten Eurocode viewer main modules bypasses LiveView. `render_with_request_context` ignores `_interaction`, forwards the ephemeral lane to viewport and selected-check rendering, and falls back to ordinary `render` for diagrams. That diagram call receives only the document, window context and locale, not the view lane or picked owner. Consequently this editor selection repair does not establish viewer viewport/outliner/check/diagram focus coherence. All fifteen viewer modules ignore the interaction parameter; the concrete no-lane diagram fallback was confirmed for the ten Eurocodes. The read-only mutation guarantees remain separate and were not challenged by this review.

## Required Witnesses

Add cases for explicit stepping across a retained owner filter, fresh owner picks following a step, empty/cleared picks after persisted check selection, invalid/removed check indices after reevaluation, and viewer check stepping updating diagrams/highlights without artifact mutation. Existing six-case selection fixtures do not alone establish these host sequences or viewer behavior. Native execution remains blocked/pending as recorded in the completion matrix.

## Concurrent Repair Update

The visual agent subsequently implemented checks_owners and wired LiveView::checks_view to it: when the selected check belongs outside actual picked owners, the checks owner filter is empty while actual picked entities remain intact. Its fixture now has seven cases with explicit checkOwners expectations and a native assertion that plan_checks contains the selected row. This addresses the concrete hidden-selected-row issue above at source level; native execution evidence is still required. Viewer bypass and temporal fresh-pick sequencing remain separate scope questions.

## Minimal Shared Viewer Integration Map

The viewer render callback already owns both InteractionView and TransientView. Read the existing interaction selection from `interaction.selection(&interaction_domain_id(variant)).ids`, and the existing ephemeral lane from LaneTransient::lane(transient.snapshot).view. NormViewerSurface::view_report returns the shared Arc report; a pure shared resolver can combine report, lane.check and picks into NormSelection without cloning a document, using NormHost, importing editor modules or introducing persisted state. Preserve explicit-lane priority and apply checks_owners when rendering checks.

EN1990 concrete window gaps: viewport render_with passes an empty highlight slice to render_viewport_panel; diagrams passes None/None focus arguments and a constant NUL-separated memo key; checks render_selected carries only selected index with default ChecksViewState and default ChecksWindowSpec (no interaction domain). Resolve focus once and pass it plus highlights/filter ownership into the three viewer window functions. The diagram memo key must vary with focus. These exact helpers and gaps were sent to the visual agent.

Temporal caution: a retained lane.check carries no recency relative to newer interaction picks. A pure rendering helper cannot infer the latest event; the host route must define when a fresh pick clears/replaces the view check or otherwise explicitly define the priority. This requires a host-sequence witness rather than adding arbitrary timestamps or persisted state.
