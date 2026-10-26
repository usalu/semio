"""🖼️ Sample completed corrected scene clips without touching active media."""
from pathlib import Path
import hashlib
import json
import sys
import av
from PIL import Image, ImageDraw, ImageFont

TICKET = Path(__file__).resolve().parent
TEACHING = Path('C:/git/semio/🎓️teaching')
ROOT = TICKET / '🗑️generated/g/s'
OUTPUT = TICKET / '🗑️generated/corrected-visual'
OUTPUT.mkdir(parents=True, exist_ok=True)
FONT_HASH = '00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2'
font = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 21)
targets = [(6, 'Beat5_Strahlung', .72), (7, 'Beat6_ThermischeMasse', .74), *[(8, 'Beat7_SensibelLatent', f) for f in (.55, .65, .75, .85, .94, .985)]]
rows = []
frames = []
for ordinal, scene, fraction in targets:
    path = ROOT / str(ordinal) / 'v' / (scene + '.mp4')
    metadata = json.loads(path.with_suffix('.render.json').read_text(encoding='utf-8'))
    assert metadata['sources'][str(TEACHING / 'manim_fonts.py').replace('\\', '/')] == FONT_HASH
    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        duration = float(stream.duration * stream.time_base)
        target = duration * fraction
        container.seek(int(target / stream.time_base), stream=stream)
        for frame in container.decode(stream):
            if frame.time is not None and frame.time >= target:
                image = frame.to_image()
                actual = float(frame.time)
                break
        else:
            raise RuntimeError(f'No frame at {target}: {path}')
    name = f'{ordinal}-{fraction:.2f}.png'
    image.save(OUTPUT / name)
    tile = Image.new('RGB', (854, 530), '#151515')
    tile.paste(image, (0, 50))
    ImageDraw.Draw(tile).text((10, 8), f'{ordinal}: {scene} | local {actual:.3f}s ({fraction:.0%})', font=font, fill='white')
    frames.append(tile)
    rows.append({'ordinal':ordinal,'scene':scene,'fraction':fraction,'target_seconds':target,'actual_seconds':actual,'duration_seconds':duration,'path':str(path),'image':name})
sheet = Image.new('RGB', (854 * 2, 530 * 4), '#151515')
for i, frame in enumerate(frames):
    sheet.paste(frame, ((i % 2) * 854, (i // 2) * 530))
sheet.save(OUTPUT / 'contact.jpg', quality=94)
(OUTPUT / 'frames.json').write_text(json.dumps(rows, ensure_ascii=False, indent=2), encoding='utf-8')
print('[DEBUG] Sampled ' + str(len(rows)) + ' completed scene frames')
