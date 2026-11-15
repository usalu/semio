import importlib, json
B = "🎓️teaching.🏛️architecture.⚡️energy."
rt = importlib.import_module(B + "🏗️authoring.🎬️clip.⚙️runtime.🐍️")
videos = rt.load_manifest()
plan = rt.segment_plan()
c = {
    "ids": [v["id"] for v in videos][:5],
    "chapter_counts": [len(v["chapters"]) for v in videos][:5],
    "segment_counts": [[len(ch["segments"]) for ch in v["chapters"]] for v in videos][:5],
    "n": len(videos),
    "render_segment_count": len(plan),
    "logical": len({(j["video_index"], j["chapter_scene"]) for j in plan}),
    "first": [min(j["ordinal"] for j in plan if j["video_id"] == v["id"]) for v in videos],
    "unique_sources": len(rt.source_plan(plan)),
}
print(json.dumps(c, ensure_ascii=False))
