"""🧪 Probe the rhino3dm calls the fit test needs."""
import math

import rhino3dm as rh

phi, eps = math.radians(55), math.radians(24)
right = rh.Vector3d(-math.sin(phi), math.cos(phi), 0)
up = rh.Vector3d(-math.sin(eps) * math.cos(phi), -math.sin(eps) * math.sin(phi), math.cos(eps))
plane = rh.Plane(rh.Point3d(0, 0, 0), right, up)
to_view = rh.Transform.PlaneToPlane(plane, rh.Plane.WorldXY())
p = rh.Point3d(1, 2, 3)
p.Transform(to_view)
print("[DEBUG]", p.X, p.Y, p.Z, -math.sin(phi) * 1 + math.cos(phi) * 2)
cloud = rh.PointCloud()
cloud.Add(rh.Point3d(0, 0, 0))
cloud.Add(rh.Point3d(1, 2, 3))
box = cloud.GetBoundingBox()
print("[DEBUG]", box.Min.X, box.Min.Y, box.Max.Z)
