import sys, json
sys.path.insert(0, ".")
from migrate import main as build_plan
plan, name_to_module = build_plan()
p = next(x for x in plan if x["struct_name"] == "ConnectAdjacency")
print("inv_uses:", p["_inv_uses"])
