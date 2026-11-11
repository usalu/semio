from pathlib import Path
import av
from PIL import Image, ImageDraw
root = next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir())
for num, scene in [(122, 'Beat3_Tower'), (123, 'Beat4_Casting')]:
    files = sorted((root / f'.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/{num}/p/{scene}').glob('uncached_*.mp4'))
    files = [p for p in files if p.stat().st_mtime > 1794027600]
    for start in range(0, len(files), 12):
        subset = files[start:start+12]
        sheet = Image.new('RGB', (1281, 1080), 'white')
        draw = ImageDraw.Draw(sheet)
        for i, file in enumerate(subset):
            try:
                with av.open(str(file)) as c:
                    st = c.streams.video[0]
                    d = float(st.duration * st.time_base)
                    c.seek(int(d * .5 / st.time_base), stream=st, backward=True)
                    frame = next(f for f in c.decode(st) if f.time is not None and f.time >= d * .5)
                    x, y = (i%3)*427, (i//3)*270
                    sheet.paste(frame.to_image().resize((427,240)), (x,y+30))
                    draw.text((x+5,y+5), file.stem, fill='black')
            except Exception as e:
                print(file.name, type(e).__name__, flush=True)
        sheet.save(Path(__file__).parent / '🗑️generated' / f'partial-{num}-{start:03}.png')
    print(num,len(files),flush=True)
