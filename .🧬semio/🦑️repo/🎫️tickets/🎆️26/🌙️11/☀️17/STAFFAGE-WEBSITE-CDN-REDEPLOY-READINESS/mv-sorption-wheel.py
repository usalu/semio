"""Sorption wheel 1:50: front and plan draw the eight spokes per face as the open strokes they are on the faces (front: vertical lines at z = +0.095 and -0.09, plan: horizontal lines), so the slender volume spokes follow a 10 mm pen stroke instead of the 0.18 m wide disc strip."""
from mvlib import *
d = D("🏗️machinery/sorption-wheel")
def spoke(i, path): return {"id": i, "bone": "rotor", "role": "detail", "z": 3.45, "filled": False, "tone": 0.2, "kind": "path", "d": path, "region": "disc", "material": "silica-gel"}
n = d.shape(50, "front", "disc")
d.insert_after(n, spoke("spoke-front-face", "M 0.095 0.033 L 0.095 0.265 M 0.095 0.355 L 0.095 0.587"))
d.insert_after(n, spoke("spoke-rear-face", "M -0.09 0.033 L -0.09 0.265 M -0.09 0.355 L -0.09 0.587"))
n = d.shape(50, "plan", "disc")
d.insert_after(n, spoke("spoke-front-face", "M -0.285 0.095 L -0.045 0.095 M 0.045 0.095 L 0.285 0.095"))
d.insert_after(n, spoke("spoke-rear-face", "M -0.285 -0.09 L -0.045 -0.09 M 0.045 -0.09 L 0.285 -0.09"))
d.commit()
