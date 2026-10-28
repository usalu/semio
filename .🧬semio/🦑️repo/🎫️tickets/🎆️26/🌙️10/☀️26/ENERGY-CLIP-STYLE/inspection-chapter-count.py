"""🔢 Retain independent actual source, chapter and lesson clocks."""
import importlib.util
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("collection_verify", TICKET / "inspection-chapter-verify.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
scene = "PhysicalFundamentals_FullSection"
paths = [module.BASE / "1/v" / (scene + ".mp4"), module.chapter_path(scene),
         module.BASE / "rendered/Full_Physical_Fundamentals_spectator_de_480x270p4.mp4"]
result = {"media": [module.STREAMS.info(path) for path in paths],
          "clock": module.read(module.chapter_path(scene).with_suffix(".chapter.json"))}
text = json.dumps(result, ensure_ascii=False, indent=2)
(TICKET / "inspection-chapter-clock-count.md").write_text("# Independently Decoded Physical Clocks\n\n" + "\n".join("    " + line for line in text.splitlines()) + "\n", encoding="utf-8")
print(text)
