"""♨️ Repair of `used-trench-convector` at 1:50: the front elevation did not draw the four levelling brackets (25 x 50 mm angles beside the trough, two at each end, seen in pairs one behind the other), which stand 26.3 mm out of the drawn trough: the pair of each side is drawn as the side elevation draws them, together with the 1.25 mm trough wall it is fixed to (the drawn end plate ends inside that wall). Idempotent: python3 fix_used_trench_convector.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "used-trench-convector"
text, doc = sc.load(ID)
side = sc.shape(doc, 50, "elevation", "levelling-bracket-0")
for name, x in (("levelling-bracket-0", -0.145), ("levelling-bracket-1", 0.11875)):
    bracket = copy.deepcopy(side)
    bracket.update({"id": name, "z": 6.9, "x": x, "y": 0.08, "w": 0.02625, "h": 0.05})
    sc.put(sc.shapes(doc, 50, "front"), bracket, before="trough-end-connection")
sc.save(ID, text, doc)
