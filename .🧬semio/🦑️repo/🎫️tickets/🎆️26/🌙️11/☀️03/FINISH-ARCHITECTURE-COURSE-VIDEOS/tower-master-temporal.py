import sys
from pathlib import Path
sys.path.insert(0, str(next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir()) / '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media'))
import lib
p = lib.base('en', 'speaker') / 'rendered/Grasshopper_tower_speaker_en_480p15.mp4'
d = lib.duration(p)
for offset in (-5, -1, 0, 1, 5):
    t = d * .75 + offset
    lib.frame(p, t / d, (854, 480)).save(Path(__file__).parent / '🗑️generated' / f'tower-master-{offset:+d}.png')
    print(offset, t, flush=True)
