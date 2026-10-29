from pathlib import Path
here = Path(__file__).parent
api = here.parent / "api.md"
text = api.read_text(encoding="utf-8")
new = (here / "api1.md").read_text(encoding="utf-8")
start = text.index("## 1. Nakagin")
end = text.index("## 2. Nakagin")
api.write_text(text[:start] + new + "\n" + text[end:], encoding="utf-8")
print("spliced")
