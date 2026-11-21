# Play Verification Read-Only Audit

Evidence inspected on 20 November; no browser, build, server, or activation was started for this audit. These are historical observations, not a claim about the current live run.

## Acceptance surfaces

- The restart handoff requires actual Play routes with example selection and editor/viewer interaction; a ready shell, component receipt, or typed DSL round trip alone does not establish usability. The handoff calls for all 15 routes; reconcile that count with the current canonical suite's 14-route selection before claiming full coverage.
- For the Eurocode surface fixture, require current retained 3D geometry, a matching draw receipt and visible model pixels, blank and restore through the picker, viewport zoom, the last paged Insert item for EN 1991 and EN 1993, and 2D diagram pixels for EN 1990 and EN 1992. Rerun first/last diagram-gallery page subtree-limit assertions and real UI probes against the final gallery version. See `eurocode-surface-and-example-audit.md` and `handoff-2026-11-20-restart.md`.
- The DIN 16798 picker has two identically labeled examples. Select exact IDs scoped to the combobox and verify `aria-selected` on switch and restore, as described in `din4108-archive-play.md`.

## Structural preview and staged history

`🗑️generated/wa041jP-preview.jsonl` contains exactly EN 1992, EN 1993 and EN 1994, using the matching `wa041jP` structural bindings and core. Each reports `shellReady`, five windows, empty `faults` and empty `failedRequests`; elapsed times are 100421, 25202 and 24892 ms respectively. This is a paired preview, not proof of final canonical publication or the complete interactive journey.

The older `🗑️generated/restart-staged-routes.jsonl` contains 15 failed shells: 12 initial `original-turn.source-not-ready` archive failures and three structural example-ingress failures. EN 1992 and EN 1993 also reported `ui.fixed-capacity`. This historical sweep must not be mistaken for a result against later published components.

## Publication and logs

`native-publication-audit-2026-11-20.md` identifies canonical Base, Eurocode, Structural and Materials receipts matching successfully linked runs and provenance rechecks. Wrapper exit 75 after publication is not by itself a publication verdict; check the receipt, digest, complete inventory and provenance together. Conversely, a successful compiler/link pass alone does not establish publication. The Energy `wdxiPax` linked digest differs from canonical, and its final provenance recheck was not reached after a source extent mismatch. It is not an authoritative Energy publication and must not be substituted into the canonical bundle.

The known UI quota failures were a Checks body of 129 nodes against 128 and Inspection explanations above the 512-byte text limit. Check the stage and cause of any `ui.fixed-capacity` semantic failure; shell readiness or a generic command refusal alone cannot localize it. The historical reports describe corrections, but the handoff specifically requests final-version bounded-page assertions and integrated probes.
