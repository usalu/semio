# WGPU graph hover ownership

The native WGPU graph scene cache tracks when it owns the visible hover. A successful local pointer move claims the mark before a delayed scene can clear it. An absent scene hover clears a mark still owned by the scene. A scene echo of a local mark leaves it local. The synchronizer preserves the effective mark across graph rebuilds for the Flow and Dag hosts.

The shared JSON now includes the Generation 3D sequence with viewport hover on Profile, a local graph pointer move onto Extrude, an absent scene update before the pointer echo, the pointer echo, and a later viewport hover and exit. The Rust regression enters the actual WGPU retained document painter, invokes the native bounded pointer input route, synchronizes each scene update, and asserts the live Flow host mark. It awaits execution because native build storage admission is blocked.

The shared Bun fixture suite passed 5 tests and 58 assertions. The exact native reconciliation function passed seven isolated Rust fixture steps. Rust parsed all five changed sources and git diff --check passed.
