import glob, os, re, sys, warnings
sys.stdout.reconfigure(encoding="utf8")
warnings.filterwarnings('ignore')
import fitz
import numpy as np
from PIL import Image, ImageDraw
HERE = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(HERE, '..', '🗑️generated', 'QA7')
PXM = float(os.environ.get('PXM', '300'))
ROWS = int(os.environ.get('ROWS', '5'))


def runs(mask):
    out, start = [], None
    for i, v in enumerate(mask):
        if v and start is None:
            start = i
        if not v and start is not None:
            out.append((start, i))
            start = None
    if start is not None:
        out.append((start, len(mask)))
    return out


def cluster(idx, gap):
    out = []
    for i in idx:
        if out and i - out[-1][1] <= gap:
            out[-1][1] = i
        else:
            out.append([i, i])
    return out


def crops(svg, scale):
    page = fitz.open(svg)[0]
    text = open(svg, encoding='utf8').read()
    dpi = int(round(PXM * scale * 25.4 / 1000.0))
    mm = 72.0 / 25.4
    out = []
    parts = re.split(r'<g transform="translate\(([-\d.]+) ([-\d.]+)\)">', text)
    for k in range(1, len(parts), 3):
        ox, oy, body = float(parts[k]), float(parts[k + 1]), parts[k + 2]
        xs, ys = [], []
        for m in re.finditer(r'<g id="(?:shape-|outline-)[^"]*"[^>]*>.*?</g>|<path id="ground-line" d="([^"]*)"', body, re.S):
            s = m.group(0)
            for d in re.findall(r' d="([^"]*)"', s):
                nums = [float(n) for n in re.findall(r'-?\d+(?:\.\d+)?(?:e-?\d+)?', d)]
                xs += nums[0::2]
                ys += nums[1::2]
        if not xs:
            continue
        pad = 1.2
        rect = fitz.Rect((ox + min(xs) - pad) * mm, (oy + min(ys) - pad) * mm, (ox + max(xs) + pad) * mm, (oy + max(ys) + pad) * mm)
        p = page.get_pixmap(dpi=dpi, clip=rect)
        out.append(Image.frombytes('RGB', (p.width, p.height), p.samples))
    return out


def main(assets):
    for asset in assets:
        for sd in sorted(glob.glob(os.path.join(G, asset, '*'))):
            if not os.path.isdir(sd):
                continue
            scale = int(os.path.basename(sd))
            svgs = sorted(glob.glob(os.path.join(sd, '*.svg')))
            rowsimg = []
            for svg in svgs:
                label = os.path.basename(svg).split('--')[1]
                try:
                    cs = crops(svg, scale)
                except Exception as e:
                    print('ERR', svg, e)
                    continue
                if not cs:
                    continue
                W = sum(c.width for c in cs) + 12 * (len(cs) - 1)
                H = max(c.height for c in cs) + 18
                row = Image.new('RGB', (W, H), 'white')
                ImageDraw.Draw(row).text((2, 2), label, fill='black')
                x = 0
                for c in cs:
                    row.paste(c, (x, 18 + (H - 18 - c.height)))
                    x += c.width + 12
                rowsimg.append(row)
            for gi in range(0, len(rowsimg), ROWS):
                g = rowsimg[gi:gi + ROWS]
                W = max(r.width for r in g)
                H = sum(r.height + 6 for r in g)
                m = Image.new('RGB', (W, H), 'white')
                y = 0
                for r in g:
                    m.paste(r, (0, y))
                    y += r.height + 6
                out = os.path.join(sd, f'zoom-{gi // ROWS}.png')
                m.save(out)
                print(out.replace(G, ''), m.size)


main(sys.argv[1:])
