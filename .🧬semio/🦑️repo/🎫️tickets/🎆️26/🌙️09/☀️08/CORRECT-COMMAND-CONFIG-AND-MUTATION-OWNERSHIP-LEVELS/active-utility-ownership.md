# Active Utility Ownership

## Architecture

Active utility selection belongs to the OS view model because it is ephemeral, local, and scoped to a mounted window. Plugin editor configuration must not serialize it or expose config mutation leaves for it. Rendering reads the window-projected `ViewModel.active_utility_id`; action handling reads `ActionMeta.view_state` when the selected utility affects a domain interaction. The framework-owned `setActiveUtility` view action remains the only selection route.

Plugin commands may request a host utility selection through `Effect::SetActiveUtility` after a domain operation. Gesture cleanup remains a domain interaction concern: switching utility must cancel plugin scratch through a typed cancellation path or when the next host-projected view state is observed, without recreating a plugin config field.

## Initial Audit

The ownership report identified serialized `activeUtilityId` config fields in Generation3d, Shooting, Process3d, Lowpoly, CAD, Remodel, Draw, Raster, and Note. It also identified plugin config mutation or editor command routes in Generation3d, Shooting, Remodel, Draw, Raster, Note, Puzzle2d, and Block3d; source inspection additionally found utility command routes in Process3d, Lowpoly, and CAD.

Presence fields and framework `WindowMeasure.active_utility_id` filters are separate concerns. They are not configuration authority and are not removed merely by matching the field name.

## Validation Status

- The pre-change Bun/Nx ownership report completed and reported the misplaced declarations above.
- Focused Rust compilation is running with a ticket-local Cargo target and has not completed.
- No active-utility implementation check has completed yet.
