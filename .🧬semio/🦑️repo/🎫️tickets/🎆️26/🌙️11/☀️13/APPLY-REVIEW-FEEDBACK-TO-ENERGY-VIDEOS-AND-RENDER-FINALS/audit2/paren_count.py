import json, pathlib, collections, re
AUDIT = pathlib.Path(r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/audit2")
rows = [json.loads(l) for l in (AUDIT/"paren-kept.jsonl").read_text(encoding="utf-8").splitlines() if l.strip()]
ONSCREEN = {"Text","body_text","centered_body_text","chip","window_chip","section_heading","legend","tr","math_panel_text"}
MATHCALLS = {"math_label","math_panel","math_readout","math_row","_readout","_live","_place_row"}
ALNUM_MATH = re.compile(r"[$\^_{}]")
cat = collections.Counter(); ex = collections.defaultdict(list); total_on = 0
out = []
for r in rows:
    f = r["file"]
    if "🧪️tests" in f or "🧬" in f: cat["excluded: tests"] += 1; continue
    if "📝️narration" in f or "🔊️speech" in f or "🗺️roadmap" in f: cat["excluded: narration/speech/roadmap"] += 1; continue
    if r["call"] in MATHCALLS or ALNUM_MATH.search(r["text"]):
        cat["excluded: math/LaTeX strings"] += 1; continue
    if not f.endswith("🐍️.py"): cat["excluded: non-py"] += 1; continue
    if r["folder"] == "🏗️authoring" and "🎬️clip" in f and "🧪" not in f and "📤" not in f and "🪟️presentation" not in f:
        pass
    if r["call"] in ONSCREEN or r["call"] == "" and r["kind"] in ("fstring",):
        key = "on-screen literal (" + (r["call"] or "fstring") + ")"
    elif r["call"] == "":
        key = "plain literal, unresolved (tuple/dict/assignment)"
    else:
        key = "other call: " + r["call"]
    cat[(r["folder"], key)] += 1
    if len(ex[(r["folder"], key)]) < 5:
        ex[(r["folder"], key)].append(f'{f.split("⚡️energy/")[-1]}:{r["line"]} "{r["text"][:70]}"')
for k, v in sorted(cat.items(), key=lambda kv: str(kv[0])):
    print(k, v)
print("EXAMPLES")
for k, v in ex.items():
    print(k)
    for e in v: print("   ", e)
