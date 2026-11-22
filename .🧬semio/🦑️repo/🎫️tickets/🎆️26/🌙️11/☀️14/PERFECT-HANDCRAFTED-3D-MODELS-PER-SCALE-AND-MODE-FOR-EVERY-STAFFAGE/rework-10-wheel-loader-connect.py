"""🚜 Wheel loader: every part joined to what carries it (rule `volumes-connected`): axles that reach the rims and sit in cradles under the rear rails, fender brackets, lamp stalks, a seat pedestal, the lift cylinder crossmember and steering lugs at the coarse scales, and a structure whose load path runs tyre, rim, axle, frame, hitch pin, frame."""
exec(open('rework-10-lib.py').read())
s = Source('wheel-loader')
PS = 'paint-steel'
F50 = {i: s.get(50, i) for i in ('link-pin',)}


def member(sid, of, bone):
    return {"id": sid, "of": of, "member": bone, "part": "member"}


for scale in (50, 100, 200, 500):
    body = s.ids(scale)
    for axle, frame, x in (('axle-r', 'rear-frame', -1.675), ('axle-f', 'front-frame', 1.675)):
        if scale == 500:
            s.put(scale, [cyl(axle, frame, PS, [x, 0.8075, -0.86], [0, 0, 1], 0.12, 1.72, tone=0.4, role='body')], region='front-frame')
            s.put(scale, [member(f'member-{axle}', axle, frame)], lst='structure')
        elif scale == 200:
            s.get(scale, axle).update(profile=[[0, 0], [0, 0.13], [1.7, 0.13], [1.7, 0]], origin=[x, 0.8075, -0.85])
        else:
            s.get(scale, axle)['profile'] = [[(0.84 if a > 0 else -0.84) if abs(abs(a) - 0.77) < 1e-9 else a, r] for a, r in s.get(scale, axle)['profile']]
    if 'axle-cradle-r' not in body:
        s.put(scale, [box('axle-cradle-r', 'rear-frame', PS, [0.5, 0.15, 0.92], [-1.675, 0.975, 0.0], tone=0.45, role='body')], region='front-frame')
    s.put(scale, [member('member-axle-cradle-r', 'axle-cradle-r', 'rear-frame')], lst='structure')
    for w in ('r-l', 'r-r', 'f-l', 'f-r'):
        s.put(scale, [member(f'member-tyre-{w}', f'tyre-{w}', f'wheel-{w[0]}')], lst='structure')
    if scale != 500:
        s.put(scale, [box('fender-f-bracket', 'front-frame', PS, [0.08, 0.08, 0.34], [0.885, 1.278, 0.59], tone=0.45, role='body', repeat={"count": 2, "translation": [0, 0, -1.18]})], region='front-frame')
    if scale in (200, 500):
        s.put(scale, [cyl('lift-cyl-crossmember', 'front-frame', PS, [0.45, 0.9, -0.7], [0, 0, 1], 0.06, 1.4, tone=0.45, role='body')], region='front-frame')
        for side, tag in ((1, 'l'), (-1, 'r')):
            s.put(scale, [
                box(f'steer-lug-rear-{tag}', 'rear-frame', PS, [0.16, 0.13, 0.08], [-1.05, 1.0, side * 0.5], tone=0.45, role='body'),
                box(f'steer-lug-front-{tag}', 'front-frame', PS, [0.16, 0.13, 0.13], [0.42, 1.0, side * 0.505], tone=0.45, role='body'),
            ], region='front-frame')
    s.put(scale, [member('member-lift-cyl-crossmember', 'lift-cyl-crossmember', 'front-frame')], lst='structure')
    for tag in ('l', 'r'):
        s.put(scale, [member(f'member-steer-lug-rear-{tag}', f'steer-lug-rear-{tag}', 'rear-frame'), member(f'member-steer-lug-front-{tag}', f'steer-lug-front-{tag}', 'front-frame')], lst='structure')
        if scale in (50, 100):
            s.put(scale, [{"id": f"joint-steer-cyl-eye-{tag}", "of": f"steer-cyl-eye-{tag}", "member": f"steer-cyl-{tag}", "part": "joint"}, {"id": f"joint-steer-rod-eye-{tag}", "of": f"steer-rod-eye-{tag}", "member": f"steer-rod-{tag}", "part": "joint"}], lst='structure')
    s.drop(scale, ['joint-articulation'], lists=('structure',))
    s.put(scale, [dict(cyl('member-articulation', 'articulation', PS, [0, 0, 0], [1, 0, 0], 0.055, 0.19), member='articulation', part='member'),
                  dict(cyl('joint-articulation', 'articulation', PS, [0.71, 0, 0], [1, 0, 0], 0.055, 0.19), member='articulation', part='joint')], lst='structure')

for axle in ('axle-r', 'axle-f'):
    p = s.get(50, axle)['profile']
    s.get(50, axle)['profile'] = [[-1.09, 0.0], [-1.09, 0.1], [-0.84, 0.1]] + p[1:-1] + [[0.84, 0.1], [1.09, 0.1], [1.09, 0.0]]
for w in ('r-l', 'r-r', 'f-l', 'f-r'):
    s.put(50, [member(f'member-hub-{w}', f'hub-{w}', f'wheel-{w[0]}')], lst='structure')
for w in ('r-l', 'r-r', 'f-l', 'f-r'):
    pad = s.get(50, f'support-contact-{w}', 'structure')
    pad['size'][1] = 0.04
    pad['transform']['translation'][1] = 0.02
s.put(50, [box('seat-pedestal', 'rear-frame', PS, [0.3, 0.3, 0.3], [-0.8, 2.11, 0.0], tone=0.3)], region='cab')
for tag, z in (('l', 0.86), ('r', -0.86)):
    s.get(50, f'tail-lamp-{tag}')['transform']['translation'] = [-3.73, 1.5895, z]
for i in ('work-lamp-l-0', 'work-lamp-l-1', 'work-lamp-r-0', 'work-lamp-r-1'):
    s.get(50, i)['transform']['translation'][0] = 0.375
for scale in (50, 100):
    s.put(scale, [box('head-lamp-stalk', 'front-frame', PS, [0.05, 0.045, 0.05], [1.58, 1.745, 1.05], tone=0.4, repeat={"count": 2, "translation": [0, 0, -2.1]})], region='front-frame')
s.put(100, [dict(F50['link-pin'])], region='lift-arm')
s.put(500, [member('member-rear-frame', 'rear-body', 'rear-frame')], lst='structure')

for scale in (50, 100, 200, 500):
    s.decide(scale, "Rework 10: nothing floats: axles reach the rims and sit in a cradle under the rear rails, fenders hang on brackets, lamps stand on stalks or on the counterweight, cylinders end in lugs or on their crossmember; the structure carries from the tyres through rims, axles, frames and the hitch pins.", "Überarbeitung 10: nichts schwebt: die Achsen reichen bis zu den Felgen und sitzen in einem Sattel unter den Hinterrahmenholmen, Kotflügel hängen an Konsolen, Leuchten stehen auf Haltern oder auf dem Gegengewicht, Zylinder enden in Laschen oder auf ihrer Traverse; das Tragwerk trägt von den Reifen über Felgen, Achsen, Rahmen und die Gelenkbolzen.")
s.save()
