# Norm-Complete Visual Eurocode Plugins

Ticket summary. Detail lives in the linked audits; this file only points.

## Briefs

- [Fleet brief](fleet-brief.md), [wave 2 brief](fleet-wave2-brief.md), [wave 3 preamble](fleet-wave3-preamble.md)
- [Norm completion](norm-completion-brief.md), [oracle wiring](oracle-wiring-brief.md), [visual vertical](visual-vertical-brief.md)

## Coordination

- Running log: [coordinator](🔍️audits/wave2/coordinator.md)
- Open requests and locks: [requests](📌️important/requests.md), post-build steps: [post-build](📌️important/post-build.md)
- Values to verify against the printed standards: [checklist](🔍️audits/wave2/printed-standards-checklist.md) (section 5 lists the wave 3 additions)

## Wave 3

Per-code status (norm coverage, 3D routing/grips/diagrams, editor features, oracles, pending schema applies, compile state), cross-cutting work and open safety items: [wave 3 status](🔍️audits/wave3/status.md). Individual agent reports are in [🔍️audits/wave3](🔍️audits/wave3/).

Infrastructure state at the time of writing:

- The re-entrant `heavy.sh` fix is staged as `🗑️generated/build-runner/heavy.sh.new` and awaits the developer's decision ([twins-speed](🔍️audits/wave3/twins-speed.md)).
- `NX_DAEMON=false` and the lowered runner memory floor take effect after the next runner restart ([build-memory](🔍️audits/wave3/build-memory.md)).
