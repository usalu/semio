from pathlib import Path
p = Path("apply.py")
t = p.read_text(encoding="utf-8")
t = t.replace('''    print(f"[DEBUG] registered''', '''    for key, series in SERIES.items():
        mine = [row for row in rows if row["topic"] == key]
        cases = {"series": key, "folder": series["folder"], "languages": ["en", "de"], "chapters": [
            {"folder": SERIES[key]["videos"][index][2], "scene": row["scene"], "video_id": row["id"], "entry": row["entry"], "chapter": row["chapter"], "title": row["title"], "output": row["output"]}
            for index, row in enumerate(mine)]}
        save(ARCH / series["folder"] / "🎬️clip" / "🧪️tests" / "📇️registry" / "🔣️.cases.json", cases)
    print(f"[DEBUG] registered''')
p.write_text(t, encoding="utf-8", newline="\n")
