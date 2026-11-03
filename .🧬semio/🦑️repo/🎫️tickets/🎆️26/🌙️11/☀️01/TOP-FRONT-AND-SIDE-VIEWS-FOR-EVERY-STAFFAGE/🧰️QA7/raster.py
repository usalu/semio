import glob, os, sys, warnings
warnings.filterwarnings('ignore')
import fitz
HERE = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(HERE, '..', '🗑️generated', 'QA7')
n = 0
for svg in glob.glob(os.path.join(G, '**', '*.svg'), recursive=True):
    png = svg[:-4] + '.png'
    if os.path.exists(png) and os.path.getmtime(png) >= os.path.getmtime(svg):
        continue
    fitz.open(svg)[0].get_pixmap(dpi=150).save(png)
    n += 1
print('rasterized', n)
