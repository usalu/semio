# Open Section Capture Audit

Read-only review of ticket visual-audit-gallery.py. The former clipper gathered all intersection vertices and angularly sorted them into a single inferred cap. That is not a valid general material-boundary reconstruction for concave sections or multiple loops.

This is relevant to actual source pieces, not merely a hypothetical future mesh: the podium capture planeY−2.7876133739814066 crosses bottom/top fascia pieces on edges0,1,2 (podium-fascia-1-0-bottom/top,1-1-bottom/top,1-2-bottom/top). Their source profiles follow outer and inner curved arcs, so a one-loop angular cap cannot be assumed to represent their thin material or preserve voids. No numerical false-cap area is claimed; the cap construction lacks the needed boundary topology.

Root removed synthetic caps. Independently reread the saved clipped helper: it retains clipped original faces/normals only, adds no inferred cut polygon, and the gallery explicitly labels Open cuts / Offene Schnitte. This resolves the artificial-cap evidence risk without changing physical model geometry or inventing closed material in a capture. Final cut images should be judged as open clipped views, not proof of a watertight section solid. Production typed-face/tessellation coverage remains a separate renderer gate.
