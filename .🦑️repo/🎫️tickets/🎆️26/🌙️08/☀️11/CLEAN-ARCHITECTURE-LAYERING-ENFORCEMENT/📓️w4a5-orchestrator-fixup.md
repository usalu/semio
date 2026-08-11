# Wave 4a.5 orchestrator fix-up

Fixed the 2 remaining consumer gaps the verify agent found (both blocked
enum deletion):

1. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
   — `sync_cad_computer_contributions` now dual-reads `topic_contribution`
   (topic `cad.computer`) before falling back to `Contribution::CadComputer`.
2. `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
   — this file defines its OWN local `ProgramContributionEntry` shadow
   struct (not the real framework one) with no `topic_contribution` field
   at all; added the field + dual-read for topic `flow.extension`.

Also confirmed the `semio-framework-math` `TokenKind` blocker (hit by
several wave-4a.5 agents) resolved on its own — the other concurrent
session finished its edit.

`cargo check -p semio-s-plugin-cad -p semio-s-plugin-procedural` — both
blocked only by the still-live "document" module churn (unrelated,
confirmed via error content), no new error class from either fix.

With this, every real producer AND consumer of `Contribution` supports the
open `TopicContribution` shape. Proceeding directly to the final cut:
deleting the closed enum, the old `contribution(s)` fields, and every
closed-path branch, in one coordinated wave.
