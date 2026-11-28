"""🧮️ Sorts the findings of a `🧊️model-catalog` log by owner: `python3 mine.py <log>` prints the count of the kinds of lane CL-T3 (self-intersection, watertight, triangles that grow, GLB size) and of the kinds of the lanes CX-GLB (the quantized GLB that the glTF-Transform reader of the oracle does not open yet), CX-FBX and CX-ID, and every other finding line."""
import re
import sys

text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
lines = sorted(set(re.findall(r'^\s*[+-]\s+"((?:model|geometry|closed|bounds|glb|variants|fbx|json|budget|determinism|set) [^"]*)"', text, re.M)))
kinds = {"CL-T3": r"watertight|triangles grow|inside out|self-intersection|degenerate|non-finite|GLB of", "CX-GLB": r"Missing required extension", "CX-FBX": r"^fbx .*assimp", "CX-ID": r"must NOT have more than 64"}
counts = {k: sum(1 for line in lines if re.search(p, line)) for k, p in kinds.items()}
other = [line for line in lines if not any(re.search(p, line) for p in kinds.values())]
invalid = len(set(re.findall(r"ModelInvalidError: [^\n]*", text)))
print(f"findings: {', '.join(f'{k} {n}' for k, n in counts.items())}, ModelInvalidError {invalid}, other {len(other)}{': ' + ' | '.join(other)[:400] if other else ''}")
