import json, pathlib, collections, re
AUDIT = pathlib.Path(r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/audit2")
rows = [json.loads(l) for l in (AUDIT/"paren-scan.jsonl").read_text(encoding="utf-8").splitlines() if l.strip()]
NONSCREEN = {"print","search","findall","match","fullmatch","assertIn","assertNotIn","assertRegex","assertNotRegex","assertEqual","assertRaisesRegex","assertRaises","add_argument","compile","write_text","split","replace","join","extend","append","len","sorted","zip","frozenset","next","skipUnless","run","run_command","TypeError","ValueError","FileNotFoundError","fail","subTest","ProfileError","ResolveError","SpeechError","WorkMismatch","_UnitError","unit_items_speech","_colour","_din_ref","_say"}
def keep(r):
    if r.get("kind") in ("docstring",) or "error" in r: return False
    c = r.get("call","")
    if c in NONSCREEN or c.endswith("Error") or c.endswith("Mismatch") or c.endswith("Exception"): return False
    return True
kept = [r for r in rows if keep(r)]
print("kept", len(kept))
by = collections.defaultdict(list)
for r in kept: by[r["folder"]].append(r)
for f, lst in sorted(by.items()):
    print(f, len(lst), collections.Counter(r["call"] or "<none>" for r in lst).most_common(12))
(AUDIT/"paren-kept.jsonl").write_text("\n".join(json.dumps(r, ensure_ascii=False) for r in kept), encoding="utf-8")
