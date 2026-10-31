# Fixture Mounts and Interior Fit

The existing reference interior contained disconnected television, audio, speaker grille, shelf, plumbing and sanitary components. Thirty-nine discrete supports now join their actual material surfaces, including a curved-door handle fixed to the door itself. All concealed attachment shapes are explicitly marked assumed with the Domus reference as their context; they are not presented as surveyed original engineering. Acoustic cavities, sanitary bowls and the curved wetroom opening remain open.

Floor-level wetroom and bed joinery is clipped to the same faceted clear-lining profile used by the physical casing. This removes material intruding into the rounded lining without raising the whole fixture or changing the declared usable dimensions. Handed layouts and explicitly overridden usable interiors share the same mapping.

Validation began with fourteen failed assertions for missing connections and corner intrusion. The completed implementation passes four NumPy and independent mesh-oracle tests in 12.764 seconds. The portable JSON contract records all thirty-nine named connections across all six layouts and two dimension-control cases. Full containment, clipped-mesh closedness and positive-volume checks also pass across those layouts and controls.

The tap extension was also developed from failing contact expectations. Both mixer bodies now connect their spouts and valves; the basin mixer bears on its back rim, and two discrete tub-deck legs support the tub mixer. The expanded four-test suite passes in 12.764 seconds; AJV separately validates the portable contract and rejects three malformed cases.

Rhino/OpenNURBS now independently confirms the same closedness result as the project edge-incidence check for every mount and every clipped fixture, using the actual independently ear-clipped triangles. The four-test suite including this comparison passes in 12.764 seconds. No production runtime dependency was added.
