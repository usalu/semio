"""📐️ Repair ops of the four machines whose views stay red but whose declared dimensions miss the body by a plain, scale-independent amount: the declared value becomes the extent of the body of 1:50 (to the millimetre); every coarser body then lies inside its own limit.
- `concrete-mixer-truck`: 9.011 m long at 1:50 (9.013 at 1:100, 9.000 at 1:200 and 1:500) against 9.00 m.
- `forklift`: 3.662 m long at 1:50 and 1:100 (3.646 at 1:200 and 1:500) against 3.646 m.
- `oil-pumpjack`: 9.100 m long at every scale against 9.20 m; 6.8777 m high against 6.87 m.
- `tower-crane`: 71.822 m long at every scale against 71.8 m.
"""
from oplib import write

DIMS = {"concrete-mixer-truck": {"widthM": 9.011}, "forklift": {"widthM": 3.662}, "oil-pumpjack": {"widthM": 9.1, "heightM": 6.878}, "tower-crane": {"widthM": 71.822}}
for ident, dims in DIMS.items():
    write(ident, [{"op": "dims", "set": dims}])
