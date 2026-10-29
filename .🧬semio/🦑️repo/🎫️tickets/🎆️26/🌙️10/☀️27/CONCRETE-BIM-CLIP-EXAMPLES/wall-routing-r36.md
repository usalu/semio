# Path Wall Routing Verification R36

The closed native roof stair wall is an IfcWall/STANDARD with an authored multi-segment curved path. Routing only by the IFC predefined enum had incorrectly reduced it to its first straight segment. Geometry dispatch now uses the actual path topology for all native IfcWall classes. Linear walls remain in the linear geometry group; multi-segment and curved walls use the path group, without changing their native class or enum.

Path-wall quantities distinguish gross occupied material,cavity,envelope and opening material removal. Public plan,section,elevation,axon,selected representation and English/German native exchange use the same path geometry. Hosted fillings on nonlinear wall hosts are emitted once. The closure of an explicitly closed path is preserved.

Four language-agnostic-backed wall-routing test functions cover actual source geometry,all seven representation routes,native English/German exchange and the west closure occupancy. The first three actual-source tests passed97.541s; the fourth passed9.793s after failing on the omitted closure. Occupancy at(2.85,8)m is wall material; interior(3.2,8)m stays empty. Independent GEOS polygon checks validate occupied regions. These tests validate geometry and selected fragments; native clip frames remain behind the repo runtime gate.

The standard test-hook registration is being checked with the final clip handoff. Retained tests live under the model test module 🌀️wall-routing; no checked-in source is copied into the ticket.
