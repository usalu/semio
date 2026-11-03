"""Blesses the scene fixture of the consolidation curve: the average degree of consolidation at selected time factors from the finite-volume Crank-Nicolson solution of the oracle (independent of the Terzaghi series of the Rust scene). Usage: python3 -I bless_consolidation.py <fixture path>"""
import importlib.util, json, pathlib, sys

ORACLE = next(p for p in pathlib.Path("/home/deck/semio").glob("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/⚖️compliance/📇️catalogue.py"))
spec = importlib.util.spec_from_file_location("catalogue_oracle", ORACLE)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
factors = [0.01, 0.05, 0.1, 0.197, 0.3, 0.5, 0.848, 1.2, 2.0]
rows = [{"timeFactor": t, "degree": module._consolidation_pde(t, 400, 400)} for t in factors]
out = {"note": "Average degree of consolidation U(Tv) of a layer drained on one side, finite-volume Crank-Nicolson (400 cells, 400 steps, Rannacher start); blessed by 🧰️scripts/en1997-w3/bless_consolidation.py", "rows": rows}
path = pathlib.Path(sys.argv[1])
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text(json.dumps(out, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(len(rows), "rows")
