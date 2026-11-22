"""✏️ Shopping trolley, second round pass 1: the 1:500 drawings follow the model (front struts drawn in elevation and front, the plan rim stroke laid inside the basket edge so the drawn sign is as wide as the basket).

Usage: python3 rework-shopping-trolley-2d.py [--apply]
"""
import sys

from rework_15_kit import Kit

kit = Kit("objects/shopping-trolley")
STROKE = {"bone": "chassis", "role": "body", "z": 2.7, "filled": False, "tone": 0.3, "region": "frame-tube", "material": "zinc-tube", "penMm": 0.13, "kind": "poly", "closed": False}
kit.put_shape("500", "elevation", dict(STROKE, id="front-strut", points=[[0.595, 0.0125], [0.557, 0.3945]]), after="chassis-line")
kit.put_shape("500", "front", dict(STROKE, id="front-strut-l", points=[[0.1987, 0.18], [0.186, 0.562]]), after="chassis-line")
kit.put_shape("500", "front", dict(STROKE, id="front-strut-r", points=[[-0.1987, 0.18], [-0.186, 0.562]]), after="front-strut-l")
kit.shape("500", "plan", "rim")["points"] = [[-0.3625, 0.2211], [0.4875, 0.1839], [0.4875, -0.1839], [-0.3625, -0.2211]]
kit.finish(sys.argv)
