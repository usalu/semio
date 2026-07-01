---
technology: infinite
path: ♾️infinite🏙️world
bundle:
 name: world
 emoji: 🏙️
 description: Infinite world area.
 kind: library
---

# World

An infinite world with chunk system (grid division), view radius (loading & unloading), memory management (object pooling) and precision managment (no floating point precision errors).

Generic r3f engine: [@semio-tech/infinite-world-r3f](r3f/index.tsx) — composable {@link WorldLayerStack}, {@link WorldCanvas}, chunking, LOD/grid, pooling, CAD precision. Specializations (e.g. puzzle 3d) compose content layers on top, mirroring `gis/2d` on `infinite/cavas`.