import re, sys
from pathlib import Path

root = Path(__file__).parent / "🗑️generated"
picks = [("walking-commuter-line", 0), ("walking-commuter-line", 5), ("walking-commuter-line", 9), ("radiator-rated-solid", 0), ("human-thermal-walk", 0), ("human-thermal-walk", 6), ("radiator-heatup-thermal", 0), ("radiator-heatup-thermal", 8)]
cells = []
for cast, frame in picks:
    svg = (root / "cast" / cast / f"{frame:04d}.svg").read_text(encoding="utf-8")
    svg = re.sub(r"<\?xml[^>]*\?>", "", svg)
    svg = re.sub(r'width="[^"]*mm" height="[^"]*mm"', 'style="height:280px;outline:1px dashed #345"', svg, count=1)
    cells.append(svg)
(root / "sheet.html").write_text('<body style="background:#001117;margin:0;display:flex;flex-wrap:wrap;gap:12px;padding:12px">' + "".join(cells) + "</body>", encoding="utf-8")
