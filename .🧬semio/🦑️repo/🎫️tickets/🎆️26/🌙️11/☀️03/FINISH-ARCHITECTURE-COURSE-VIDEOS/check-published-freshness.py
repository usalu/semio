import sys
from pathlib import Path
sys.path.insert(0, str(next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir()) / '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media'))
import lib
for video, output in [('grasshopper_trees', 'Grasshopper_trees')]:
    try:
        s = lib.video_status(video, 'en', 'speaker', output)
        print(video, lib.master_fresh(s), sum(x['fresh'] for x in s['segments']), len(s['segments']))
    except Exception as e:
        print(video, type(e).__name__, str(e))
