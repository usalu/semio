"""[DEBUG] Renders a verify.ts dump into contact sheets (rows: queries, columns: side, front, top).
usage: python render.py <asset> <scale> [label-substring ...]"""
import json, os, sys
from PIL import Image, ImageDraw, ImageFont

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, '..', '🗑️generated', 'A1')
asset, scale = sys.argv[1], sys.argv[2]
subs = sys.argv[3:]
dump = json.load(open(os.path.join(OUT, f'{asset}-dump-{scale}.json'), encoding='utf-8'))
if subs:
    dump = [d for d in dump if any(s in d['label'] for s in subs)]
VIEWS = ['elevation', 'front', 'plan']
PPM = {'50': 1, '100': 1, '200': 1, '500': 1}


def bbox(view):
    xs, ys = [], []
    for d in dump:
        for s in d['views'][view]:
            for r in s['rings']:
                for x, y in r:
                    xs.append(x); ys.append(y)
    return min(xs), max(xs), min(ys), max(ys)


boxes = {v: bbox(v) for v in VIEWS}
span_x = max(b[1] - b[0] for b in boxes.values())
span_y = max(b[3] - b[2] for b in boxes.values())
cell_w = 520
ppm = (cell_w - 30) / span_x
cell_h = int(span_y * ppm) + 36
if cell_h > 420:
    ppm = (420 - 36) / span_y
    cell_w = int(span_x * ppm) + 30
    cell_h = 420
W = 110 + 3 * cell_w
H = 20 + len(dump) * cell_h
img = Image.new('RGB', (W, H), 'white')
dr = ImageDraw.Draw(img)
font = ImageFont.load_default()
for c, v in enumerate(VIEWS):
    dr.text((110 + c * cell_w + 5, 4), v, fill='black', font=font)
for r, d in enumerate(dump):
    top = 20 + r * cell_h
    dr.text((4, top + 10), d['label'], fill='black', font=font)
    for c, v in enumerate(VIEWS):
        x0, x1, y0, y1 = boxes[v]
        ox = 110 + c * cell_w + 15 - x0 * ppm
        oy = top + cell_h - 18 + y0 * ppm
        tx = lambda x: ox + x * ppm
        ty = lambda y: oy - y * ppm
        if v != 'plan':
            dr.line([(tx(x0) - 5, ty(0)), (tx(x1) + 5, ty(0))], fill=(200, 200, 200))
        for s in sorted(d['views'][v], key=lambda s: s['z']):
            g = int(60 + 180 * min(1, max(0, s['tone'])))
            for ring in s['rings']:
                pts = [(tx(x), ty(y)) for x, y in ring]
                if len(pts) < 2:
                    continue
                if s['filled'] and s['closed']:
                    dr.polygon(pts, fill=(g, g, g), outline='black')
                else:
                    dr.line(pts + ([pts[0]] if s['closed'] else []), fill='black', width=1)
name = os.path.join(OUT, f'{asset}-{scale}' + ('-' + '_'.join(subs) if subs else '') + '.png')
img.save(name)
print(name, img.size)
