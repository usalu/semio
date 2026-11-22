"""🚜 Forklift root state, pass 1 (second round): the pallet bone and the overall length stay as authored (pallet-x at 2.446 m, 3.646 m long); the pallet meets the forks through the 45 mm fork shanks of the volumes instead.

Run from the ticket folder: python3 rework-forklift-p1-rig.py
"""
exec(open("rework-10-lib.py").read())
C = Catalog("forklift")
bone = next(b for b in C.asset["rig"]["bones"] if b["id"] == "pallet-x")
bone["x"] = 2.446
C.asset["dimensions"]["widthM"] = 3.646
C.touch("rig", "dimensions")
print("[DEBUG] wrote", C.write())
