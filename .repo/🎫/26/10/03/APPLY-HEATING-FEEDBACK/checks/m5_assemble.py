from pathlib import Path
C = Path(__file__).resolve().parent
tpl = (C / "m5_scene_template.py.txt").read_text()
out = tpl.replace("<<SHARED>>\n", (C / "m5_shared_block.txt").read_text()).replace("<<BEAT6>>\n", (C / "m5_beat6_block.txt").read_text())
assert "<<" not in out
(C.parents[6] / "tutorial/energy/demand/Heating/5_solar_heat_gain/scene_5.py").write_text(out)
print("written", len(out.splitlines()), "lines")
