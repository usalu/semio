# Stopped Ticket Output Disk Audit

Read-only observation2026-10-14. No deletion, process termination or Rust-output modification. All candidate paths below are inside this ticket's 🗑️generated directory.

Initial df showed135MiB available. Ticket top allocated non-Cargo files were rw/source-maps.json70,221,824bytes, rw/project-graph.json58,216,448, rw/inference/emoji-project-json.json56,578,048, publication53M and test-cases publication32M. Source-maps/project-graph disappeared between inventory and lsof; concurrent cleanup was observed, not performed here. A subsequent df showed284MiB available. rw/rc directories still exist, so do not assume whole caches were deleted or safely inactive.

Concrete stopped root-route outputs: semio-text-native-nx-repaired.log7,108,415bytes and semio-text-original-native.log6,963,587bytes. Both tails declare stage complete and script nx exited1. lsof on the repaired log found no open descriptor (command exit1 with no matching entry). Their combined logical reclaim is14,072,002bytes (~13.4MiB). These are generated failed historical logs, not source/input/report/Rust artifacts. Preserve needed diagnosis in ticket reports before root-authorized removal. They provide only modest immediate headroom.

Other large logs include JPEG paged-source6.04MB, JSON context-repaired5.93MB, JSON store-order5.82MB, JSON canonical-step5.81MB, JPEG public-utf85.79MB and erased-context5.78MB; tails all show terminal nx exit1. These belong to other agent scopes or require owner evidence, so this audit does not nominate them for removal.

cargo-native-job allocated228MiB in the first inventory contains actual rlib/rmeta outputs and is excluded entirely. rc allocated81MiB and rw allocated454MiB were initial generated-cache totals, but their current authorizer/source ownership is shared with live qualification routes; no safe cache-removal recommendation follows from names or age alone.

Process snapshot ps -axo pid,ppid,command filtered for cargo/rustc/nextest/EPW/JPEG/currentticket found no matching actual runner at the later instant, only this audit shell. This is time-local evidence; root reported active routes before the read and must resolve their actual terminal handles. Absence from one process list is not authorization to remove Rust artifacts or cache paths used by a subsequently admitted route. No broad workspace inventory or unrelated cache cleanup was done.
