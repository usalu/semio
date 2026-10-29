- model agent: sections 1-2 final (JSON v2, core functions, car_path, demo_event, resolve_link, clearance(spacing=)); details T/model-milestone-1.md

R20 roof: path_wall_geometry(model, wall) publishes disjoint mitered screen parts, station apertures and tapered cap; all seven reps and plan/section/elevation/axon use them. solarium_detail exposes the physical aperture section. Native STEP preserves tapered screen cap profiles. See roof-remodel-r20.md.

R21 services: service_geometry(key,model) resolves native hollow material on X/Y/Z axes, with optional true mitered endpoints; public part interfaces preserve closed convex polyhedra and native IFC Breps. Historical state/evidence is schema-backed and preserved by information/native STEP. See services-remodel-r21.md.
