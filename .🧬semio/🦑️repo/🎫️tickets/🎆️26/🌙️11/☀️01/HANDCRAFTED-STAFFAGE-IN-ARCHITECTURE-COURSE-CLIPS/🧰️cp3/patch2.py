import sys
from pathlib import Path
p = Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
s = s.replace('("r2-ap-report.md", "r2-fb-report.md")', '("r2-ap-report.md", "r2-fb-report.md", "r2-eb-report.md")')
s = s.replace('''    gate = GATES.get(video_id, ())
''', '''    gate = tuple((GATES.get(video_id, ),) if isinstance(GATES.get(video_id), str) else GATES.get(video_id, ())) + (("r2-eb-report.md",) if video_id in HELD else ())
''')
s = s.replace('''    return all((TICKET / name).is_file() for name in ((gate,) if isinstance(gate, str) else gate))''', '''    return all((TICKET / name).is_file() for name in gate)''')
s = s.replace("def gate_open(", 'HELD = set()\nEXPORT_ERROR = "Spread syntax requires"\n\n\ndef gate_open(', 1)
s = s.replace('''            if result == "memory":''', '''            if result == "fail" and EXPORT_ERROR in (GEN / f"{video_id}.log").read_bytes()[-40000:].decode("utf-8", "replace"):
                result = "export"
                break
            if result == "memory":''')
s = s.replace('''        state[video_id] = {"status": result''', '''        if result == "export":
            HELD.add(video_id)
            pending.append(video_id)
            log(f"{video_id} cast export error, held for r2-eb-report.md")
            continue
        state[video_id] = {"status": result''')
p.write_text(s, encoding="utf-8")
