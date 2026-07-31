# Celebrating mesh material on catalogue drop

After a successful catalogue → world drop (`addObjectKind`), newly added instance ids are stamped via `celebrateWorldInstances` for `CELEBRATE_STAMP_DURATION_MS`.

While stamped, `resolveMeshStyle` returns `celebrated` (above `selected`). `PaintTexturedMesh` / `GlbInstanceMesh` then use `CelebratingConicMaterial` — a spinning primary/secondary/tertiary conic-gradient ShaderMaterial (object-space `atan`) instead of the solid selected color.
