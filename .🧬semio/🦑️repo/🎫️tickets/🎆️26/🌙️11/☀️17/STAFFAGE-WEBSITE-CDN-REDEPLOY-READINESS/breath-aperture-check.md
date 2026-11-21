# Breath aperture regression check

The earlier selected library run reported that an ellipsoid translated away from its tissue surface passed the feature rule. The live implementation now checks the aperture centre’s distance to a distinct tissue surface in the same bone frame. Its regression fixture supplies that surface, then moves the negative variant away from it.

The isolated physical breath-aperture test passed on the current files using the repository-pinned Bun and Vitest. The case exercises both accepted aperture constructions and the rejected away-from-surface ellipsoid. This check required no source changes. The catalog-wide run covers the authored animal apertures separately.
