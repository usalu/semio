# Controls Adoption Contract

Controls use `data-shared-border-frame=""` and `data-shared-border-edges="top right bottom left"` on painted rectangular outer frames; direct cells use `data-shared-border-cell=""`. Group roots declare `data-orientation="horizontal|vertical"`. Toggle shells (including secondary action shells) are the direct cells; labels and drag handles are never independent bordered cells.

Navbar declares only `data-shared-border-edges="bottom"`; footer only `top`, each also a frame. Their layout wrappers, ribbon strips and panel tab rows use `data-shared-border-group=""` as attachment discovery boundaries but do not claim full rectangular painted frames. Panel tabs are cells and preserve existing silhouette/chip geometry. No `data-shared-border-attached` is emitted speculatively: geometry runtime alone marks measured coincident sides.

Button outline variants delegate to the group's outer outline rather than drawing a second item rectangle. Detached standalone actions retain their complete border and register as frames.

Requested integration: border-core styles explicit direct `[data-shared-border-cell]` separators (including vertical block-end), avoiding inset-shadow duplicates; silhouette runtime measures frames/edge descriptors and group boundaries. Navbar/name-chip partial joins must be segment-based, never suppress an entire bar edge merely because one chip touches it.
