# Required-Service Backend Identity Reachability Review — 2026-10-17

Read-only Source audit. No runtime, native, build, or tooling store operation executed.

## Confirmed Reachable Defect

P2. `SpawnGroup::from_launch` in `🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs:159–164` drops required processes solely on equal command_id before effective backend binding. This is reachable through supported registry declarations, without hand-constructing invalid Launch values.

A single ordinary target or tool may declare requires as two objects: `{run:"service:dev",env:{SEMIO_BUILD_TOOL:"nx"}}` and `{run:"service:dev",env:{SEMIO_BUILD_TOOL:"bazel"}}`. `🎛️dashboard/🎮️registry/🦀️.rs:640–653` rejects only identical run/parameters/env triples, so both objects are valid. `Member` equality at line 453 includes env. `launch` at 1128–1132 resolves each requirement and overlays its environment on its processes. The resulting two service processes have the same command ID and different selected backends. The collector discards the second. A compound with separate members requiring these variants reaches the same collector: requirement union at 1119 uses Member equality including env and preserves both.

## Small Repair and Meaningful Regression

The collector must retain distinct unresolved LaunchProcess values until actual inherited selection is available. The smallest safe fix is deduplicating by complete LaunchProcess equality (`known == process`) rather than command_id. LaunchProcess already derives PartialEq and Eq. Exact repeated services remain one entry; explicit backend differences and an inherited-versus-explicit request survive. Later daemon service reuse already compares effective backend, so equivalent unresolved forms can be consolidated only when inheritance is known.

Do not collapse absent backend and explicit nx inside this collector by assuming default nx: a requesting client may inherit bazel, making those two service requests distinct. Do not merely compare environment-only selected tool with an artificial None parent for the same reason.

Regression should resolve a registry Facts declaration with the two requires objects, then call SpawnGroup::from_launch and assert two required SessionCommand entries with distinct backend env and unique generated session IDs. A duplicate identical requirement via two nested dependency branches should still yield one entry. Add an absent-versus-explicit-nx case with inherited bazel at the later binding stage. This tests registry reachability plus collector behavior, not only a hand-built launch.

## Adjacent Registry Filter

`🎛️dashboard/🎮️registry/🦀️.rs:1122` removes a requirement whenever any compound member resolves to the same command ID, ignoring Member.env. A compound member `service:dev` pinned to nx can therefore suppress another member's required `service:dev` pinned to bazel even before collection. This is the same backend identity loss at an earlier boundary. It is reachable with supported member.env and requires.env declarations. Repair must compare the resolved process selection/environment or defer this elimination until semantic identity is bound; command ID alone is insufficient. Include this compound-member-versus-required-service case in the regression if addressing complete group backend identity.

No production changes made. Implementing agent owns patch and runtime validation.
