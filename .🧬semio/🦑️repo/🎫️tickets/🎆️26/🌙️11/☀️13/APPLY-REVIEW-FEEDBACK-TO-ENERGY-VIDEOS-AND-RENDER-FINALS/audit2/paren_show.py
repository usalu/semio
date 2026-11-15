import json, pathlib, re
AUDIT = pathlib.Path(r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/audit2")
rows = [json.loads(l) for l in (AUDIT/"paren-kept.jsonl").read_text(encoding="utf-8").splitlines() if l.strip()]
for r in rows:
    if r["call"] in ("math_label","math_panel","math_readout","math_row","_readout","_live","_place_row"):
        continue
    short = r["file"].split("⚡️energy/")[-1]
    print(f'{r["folder"][:4]}|{short[-60:]}:{r["line"]}|{r["call"] or "-"}|{r["text"][:90]}')
