# Native projected instance hover

The WGPU scene bridge now accepts the complete authored `hoveredIds` projection and keeps `hoveredId` as the distinct primary address. The renderer matches projected IDs to draw instances, even when several rows share one geometry allocation or interaction alias. An explicit empty array clears all whole-instance hover marks. Scenes without the array preserve their singular hover path, and component targeting retains its own lane.

A native pointer hit or explicit miss overrides the projection across identical scene refreshes. A changed valid projection takes ownership again. A graph-only authored hover does not create pointer-leave debt. The native pointer publication remembers its own pending leave even when a fresh scene has taken over painting.

The registered `world_host_unit_2` law replays all six shared neutral cases over the actual scene bridge and WGPU draw flags. It also checks that an empty list with a component and primary address leaves whole geometry unpainted, that selected flags remain correct, and that the draw buffers and geometry generations stay fixed. The isolated Rust run compiled the actual draw-flag function and matched all six cases. A Three.js instanced mesh independently mapped ray hits and per-instance colors to the same six expected sets. The registered native test remains pending while the shared Flow build uses compiler capacity.

The repository source-closure check parsed 27 included Rust sources. All touched packages and the registered test remained within source admission limits.
