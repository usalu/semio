# Encoding unit marker sites

The remaining mounted source sites are in `🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs`:

- Line 76: the denied and funded-loop `recipient.close_step` calls lack a close marker; that function has no existing close marker.
- Lines 83–84: two `control.close_retirement_recipient` calls lack one; that function has no existing close marker.
- Line 90: the recipient close loop lacks one; that function has no existing close marker.
- Lines 97, 102, 106, 108 and 111: respectively two, two, one, one and one one-argument `ControlledRetirement::step` call sites. Their existing `actual` variables belong to prior allocation effects and may already contain real outcomes; they are not reusable as empty close-turn markers.

The local erased Owner at lines 65–72 still implements one-argument `close_step` and does not override `next_computational_work`; the current erased default returns `UnsupportedOwner` (`♻️retirement/🧬️contract/🦀️.rs:103`). Its current implementation reports one work unit even for denied item/copy grants, so a new quote and actual receipt must reflect the admitted branch rather than assert constant work without analysis.

The shared helper mounts were already reviewed separately. This inventory excludes the typed integration and unmounted paged tests. No test was launched.
