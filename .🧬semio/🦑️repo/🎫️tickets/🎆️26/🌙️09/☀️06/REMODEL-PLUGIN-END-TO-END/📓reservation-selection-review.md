# Reservation selection review

Source review of list/🎟️reservation/🔎️selection/🦀️.rs, before native execution. The selection bound covers two initial events, at most H nibble events, H branch pairs plus the terminal node, and the final result: 3H+4 where H=ceil(usize::BITS/4). The reservation bound covers H branch pairs, terminal inspection, allocation and update events, and the sixteen branch-header initializations; 2H+16+8 covers the counted paths.

The physical and work denials precede allocation; the marker is unchanged when initially None. Binding checks the list address, root pointer/length/capacity, logical dimensions, byte policy and reservation epoch. Post-allocation excess capacity remains owned in the list and is reported with its bytes on the normal installed-node paths. These are source observations, not a native test result.

The separately allocated replacement root is local until installed; the unsafe PageAllocation contract must ensure an error cannot return while retaining a successful new allocation in that local Vec, otherwise this error path drops it without a paid release. The ordinary ExactAllocation and policy contracts require independent validation; no such faulty policy was established by this review. The currently running native gate is required to establish compilation and behavior.
