import glob, os, sys
from PIL import Image
sys.stdout.reconfigure(encoding='utf8')
HERE = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(HERE, '..', '🗑️generated', 'QA7')
MAXH = 2400
for asset in sys.argv[1:]:
    for sd in sorted(glob.glob(os.path.join(G, asset, '*'))):
        if not os.path.isdir(sd):
            continue
        files = sorted(glob.glob(os.path.join(sd, '*.png')))
        groups, cur, h = [], [], 0
        for f in files:
            im = Image.open(f)
            if cur and h + im.height > MAXH:
                groups.append(cur)
                cur, h = [], 0
            cur.append(f)
            h += im.height + 4
        if cur:
            groups.append(cur)
        for gi, g in enumerate(groups):
            ims = [Image.open(f).convert('RGB') for f in g]
            W = max(i.width for i in ims)
            H = sum(i.height + 4 for i in ims)
            m = Image.new('RGB', (W, H), 'white')
            y = 0
            for i in ims:
                m.paste(i, (0, y))
                y += i.height + 4
            out = os.path.join(sd, f'montage-{gi}.jpg')
            m.save(out, quality=88)
            print(out, m.size, len(g))
