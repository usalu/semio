# Original straight Sweep and Pipe

The retained Sweep and Pipe nodes accept one original open wire with one finite line trim and a planar profile face. They use the directed displacement between the trim endpoints and keep the profile in its own world position, matching the cold straight-path branch. Axial circular faces publish an analytic cylinder. Transverse line polygons use the funded prism cursor. Both preserve the existing `solid` output. Pipe accepts an absent guide and explicitly refuses a supplied guide. Multi-edge, curved or degenerate paths, oblique conic sweeps and holed profiles remain outside this original route.

The native Flow fixture constructs the profiles and path wires through the registry. It covers a reversed tilted cylinder sweep along a line placed away from the profile and an oblique rectangular pipe. It checks opposing edge uses, funded preview bounds and exact volume. It also verifies that a bent path and a supplied Pipe guide leave no live result.

Manifold reproduced both cases as an oriented cylinder and the hull of a translated rectangular profile. Both oracle tests passed all 16 assertions for bounds, enclosed volume and genus. Rust syntax parsing passed across nine modules. The full native Flow fixture remains unexecuted because the host has insufficient space for Cargo admission.
