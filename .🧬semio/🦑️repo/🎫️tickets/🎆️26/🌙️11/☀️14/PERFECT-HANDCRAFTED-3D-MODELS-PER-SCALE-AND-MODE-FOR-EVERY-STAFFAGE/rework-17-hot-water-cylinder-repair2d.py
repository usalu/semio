"""🩹 Author 17: one-off repair of the stub shapes of the cylinder drawings, which an early run of the author script shifted more than once (its done-test compared to 1e-6 where the rect origin is rounded to 1e-5): shifts them back so that they sit exactly once at the new tapping heights."""
import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rework-17-kit.py')).read())
SHIFT = {'3': 0.76 - 0.9227, '4': 0.26 - 0.5102}


def repair(asset):
    y = shape(asset, 50, 'elevation', 'stub-3')['y']
    n = round((0.91067 - y) / -SHIFT['3'])
    print('shifted', n, 'times')
    if n <= 1:
        return
    for rep in asset['representations'].values():
        if rep['status'] != 'authored':
            continue
        for view in ('elevation', 'front'):
            for s in rep['geometry'].get(view, []):
                sid = s['id']
                if sid[-1] in SHIFT and sid[:-1] in ('stub-', 'nut-', 'bore-stub-'):
                    dy = -SHIFT[sid[-1]] * (n - 1)
                    if 'points' in s:
                        s['points'] = [[p[0], r5(p[1] + dy)] for p in s['points']]
                    elif 'cy' in s:
                        s['cy'] = r5(s['cy'] + dy)
                    else:
                        s['y'] = r5(s['y'] + dy)


edit_catalog('hot-water-cylinder', ['representations'], repair)
