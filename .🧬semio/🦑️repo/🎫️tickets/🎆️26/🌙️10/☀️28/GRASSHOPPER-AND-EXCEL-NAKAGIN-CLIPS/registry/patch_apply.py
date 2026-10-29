import re
from pathlib import Path
p = Path("apply.py")
t = p.read_text(encoding="utf-8")
start = t.index("def upsert")
end = t.index("def main")
t = t[:start] + '''def place(items: list, entries: list, key=lambda item: item["id"]) -> list:
    ids = {key(entry) for entry in entries}
    kept = [item for item in items if key(item) not in ids]
    anchor = max(index for index, item in enumerate(kept) if key(item).startswith("bim_")) + 1
    return kept[:anchor] + entries + kept[anchor:]


''' + t[end:]
t = t.replace('''    for row in rows:
        upsert(manifest["videos"], "id", {"id": row["id"], "script": row["entry"], "chapters": [{"scene": row["chapter"], "segments": [{"script": row["script"], "scene": row["scene"]}]}], "output": row["output"]})
''', '''    manifest["videos"] = place(manifest["videos"], [{"id": row["id"], "script": row["entry"], "chapters": [{"scene": row["chapter"], "segments": [{"script": row["script"], "scene": row["scene"]}]}], "output": row["output"]} for row in rows])
''')
a = t.index('    base = len(')
b = t.index('    save(CASES["contract"], data)')
t = t[:a] + '''    known = {name: (chapters, segments) for name, chapters, segments in zip(curriculum["ids"], curriculum["chapter_counts"], curriculum["segment_counts"])}
    known.update({row["id"]: (1, [1]) for row in rows})
    order = place([{"id": name} for name in curriculum["ids"]], [{"id": row["id"]} for row in rows])
    curriculum["ids"] = [item["id"] for item in order]
    curriculum["chapter_counts"] = [known[name][0] for name in curriculum["ids"]]
    curriculum["segment_counts"] = [known[name][1] for name in curriculum["ids"]]
    totals = [sum(counts) for counts in curriculum["segment_counts"]]
    curriculum["first_segment_ordinals"] = [1 + sum(totals[:index]) for index in range(len(totals))]
    curriculum["render_segment_count"] = sum(totals)
    curriculum["logical_clip_count"] = sum(curriculum["chapter_counts"])
''' + t[b:]
t = t.replace('''    clips = [clip for clip in data["clips"] if clip["topic"] not in SERIES]
    for row in rows:
        clips.append({"id": row["id"], "topic": row["topic"], "chapters": [{"scene": row["chapter"], "segments": [{"script": row["script"], "scene": row["scene"]}]}]})
    data["clips"] = clips''', '''    clips = place(data["clips"], [{"id": row["id"], "topic": row["topic"], "chapters": [{"scene": row["chapter"], "segments": [{"script": row["script"], "scene": row["scene"]}]}]} for row in rows])
    data["clips"] = clips''')
p.write_text(t, encoding="utf-8", newline="\n")
