"""One-off body additions of delivery-van 1:50 (author 32): hood panel, silencer."""
import sys; sys.path.insert(0, '.')
from author_32_kit import *
A = 'vehicles/delivery-van'
d = load(A)
v = d['volumes']['50']
v['body'] = [s for s in v['body'] if s['id'] not in ('hood-panel', 'silencer', 'exhaust-pipe', 'slider-track', 'slider-track-low', 'slider-track-high')]
RQ = [0.70711, 0, 0.70711, 0]
top = [(1.88, 1.075), (2.06, 1.045), (2.3, 1.0), (2.55, 0.915), (2.62, 0.89)]
v['body'].append(loft('hood-panel', 'chassis', 'paint', 'body', 0.55, [sec((x, y - 0.0125, 0), 0.82 if x < 2.5 else 0.76, 0.014, 8, RQ) for x, y in top]))
v['body'].append(box('slider-track-low', 'chassis', 'paint', 'body', 0.4, (1.4, 0.03, 0.04), (-0.4, 0.5, -0.925)))
v['body'].append(box('slider-track-high', 'chassis', 'paint', 'body', 0.4, (2.0, 0.03, 0.04), (-0.7, 1.8, -0.925)))
save(A, d)
