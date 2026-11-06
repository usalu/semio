# Ordered entry denial review

Canceled advance and not-closing close are checked before zero-item and work denial. Denials do not move any owner; advance clears per-turn receipts and allocation marker on these denial paths, and actual child allocation attempts propagate when child work is nonzero.

Concrete accessor gap: work-quote errors return before clearing previous progress and last_work (and advance allocation marker), so public last-turn accessors may report stale prior-turn data despite the returned zero-work error. close_step does not clear last_allocation_attempt on any path. Native owner notified. No tests run.