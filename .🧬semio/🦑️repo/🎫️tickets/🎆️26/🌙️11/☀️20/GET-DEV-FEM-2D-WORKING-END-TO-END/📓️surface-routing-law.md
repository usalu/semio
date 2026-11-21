# Funded Surface Routing Law

The framework's existing surface-routing case now supplies its original caller context to mounted render and host refresh, waits for the ready result, and uses the funded render owner to retire each response. The case still checks exact per-surface view state, selection presence addressing, and the refusal of a synthetic window.

The scheduling assertions cover first mount, window render, refused mount, and hide. The panel render leaves the latch clear. The test clears the latch between observations so the refused synthetic mount begins without earlier work still armed.

The scoped whitespace and Rust syntax checks passed. Native execution of `every_mounted_surface_renders_against_its_own_view_state_while_one_pick_reaches_every_body` remains pending.
