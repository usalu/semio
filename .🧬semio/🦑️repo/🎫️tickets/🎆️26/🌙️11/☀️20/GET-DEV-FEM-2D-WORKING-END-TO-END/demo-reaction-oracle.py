"""🔮️ Solve the bundled demo frame's live-load support equilibrium with NumPy."""

import json
from pathlib import Path
import re
import sys

import numpy as np


source = Path(sys.argv[1]).read_text(encoding="utf-8")
nodes = {
    node: float(x)
    for node, x in re.findall(r"(?m)^  (\S+) (-?[\d.]+)m -?[\d.]+m$", source)
}
members = re.findall(r"(?m)^  beam .*? start=(\S+) end=(\S+)", source)
frame = {"n1"}
while True:
    expanded = frame | {node for pair in members if frame.intersection(pair) for node in pair}
    if expanded == frame:
        break
    frame = expanded

supports = [
    node
    for node, restraints in re.findall(r"(?m)^  \S+ (\S+) \[ ([^\]]+) \]$", source)
    if node in frame and "Ty" in restraints.split()
]
assert len(supports) == 2, supports
live = re.search(r'(?ms)^  live "[^"]+" \{\n(.*?)^  \}\n  false$', source)
assert live, "Expected a live load case without self-weight"
forces = re.findall(r"node-id=(\S+) dof=Ty value=(-?[\d.]+)", live[1])
assert forces and all(node in frame for node, _ in forces), forces

loads = [(nodes[node], float(value)) for node, value in forces]
moments = [sum(value for _, value in loads), sum(x * value for x, value in loads)]
reactions = np.linalg.solve([[1, 1], [nodes[node] for node in supports]], -np.array(moments))
print(json.dumps({f"{node}.Ty": float(value) for node, value in zip(supports, reactions)}))
