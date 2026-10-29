"""🏗️ Forecast roof-post continuous RC joint changes in memory before source publication."""
import json,runpy
from pathlib import Path
ROOT=next(p for p in Path(__file__).parents if (p/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json").is_file())
source=json.loads((ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json").read_text())
for obj in source["elements"]:
    if obj["tag"]=="10-FIRST":
        section=obj["geometry"]["sections"][0]
        section["top_level"]=9.03
        obj["geometry"]["sections"]=[section]
    if obj["tag"]=="15-FIRST":
        obj["geometry"]["sections"][1]["base_level"]=6.34
context=runpy.run_path(str(Path(__file__).with_name("joints-r42-oracle.py")),init_globals={"source_override":source})
result=context["result"]
Path(__file__).with_name("🗑️generated").joinpath("roof-bearing-forecast-r43.json").write_text(json.dumps(result,indent=2))
