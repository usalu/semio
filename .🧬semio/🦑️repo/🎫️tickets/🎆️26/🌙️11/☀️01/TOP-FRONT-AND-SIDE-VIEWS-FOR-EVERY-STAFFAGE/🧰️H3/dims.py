import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *
doc = load("skateboarder")
doc["asset"]["dimensions"]["widthM"] = 1.057
doc["asset"]["dimensions"]["depthM"] = 1.03
save("skateboarder", doc)
print("ok")

doc = load("pregnant-person")
doc["asset"]["dimensions"]["widthM"] = 0.69
doc["asset"]["dimensions"]["depthM"] = 0.5
save("pregnant-person", doc)
print("pregnant dims")
