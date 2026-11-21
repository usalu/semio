"""🧮️ Check the demo's Live Load has a compressive frame buckling direction."""

from contextlib import redirect_stdout
import io
import json
from pathlib import Path
import re
import runpy
import sys

import numpy as np


with redirect_stdout(io.StringIO()) as output:
    runpy.run_path(str(Path(__file__).with_name("demo-reaction-oracle.py")))
reaction = json.loads(output.getvalue())
source = Path(sys.argv[1]).read_text(encoding="utf-8")
support, compression = next((key.removesuffix(".Ty"), force) for key, force in reaction.items() if force > 0)
members = re.findall(r"(?m)^  beam .*? start=(\S+) end=(\S+) material-id=(\S+) section-id=(\S+)$", source)
[(start, end, material, section)] = [member for member in members if support in member[:2]]
nodes = {node: np.array([float(x), float(y)]) for node, x, y in re.findall(r"(?m)^  (\S+) (-?[\d.]+)m (-?[\d.]+)m$", source)}
assert nodes[start][0] == nodes[end][0], "The loaded roller attaches only to a vertical member"
assert "Rz" not in re.search(rf"(?m)^  \S+ {re.escape(support)} \[ ([^\]]+) \]$", source)[1].split()
length = np.linalg.norm(nodes[end] - nodes[start])
modulus = float(re.search(rf'(?m)^  {re.escape(material)} "[^"]+" ([\d.]+)Pa', source)[1])
inertia = float(re.search(rf'(?m)^  {re.escape(section)} "[^"]+" [\d.]+m2 ([\d.]+)m4', source)[1])

points, weights = np.polynomial.legendre.leggauss(4)
coordinate = (points + 1) / 2
first_derivative = 1 - 4 * coordinate + 3 * coordinate**2
second_derivative = (-4 + 6 * coordinate) / length
elastic_energy = modulus * inertia * length / 2 * (weights @ second_derivative**2)
negative_geometric_energy = compression * length / 2 * (weights @ first_derivative**2)
assert elastic_energy > 0 and negative_geometric_energy > 0
print(json.dumps({
    "support": support,
    "member": [start, end],
    "compressionN": compression,
    "positiveTrialBucklingFactor": elastic_energy / negative_geometric_energy,
}))
