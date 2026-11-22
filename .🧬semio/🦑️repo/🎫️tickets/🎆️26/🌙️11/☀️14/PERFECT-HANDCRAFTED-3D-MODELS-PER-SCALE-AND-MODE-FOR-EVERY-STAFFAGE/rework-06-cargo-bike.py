"""🚲 Cargo bike: the seat post that the 1:200 and 1:500 lists had lost (the saddle hung 0.19 m above the seat tube) and the pedals of 1:500; everything else was measured against a 2.6 m long-john and kept."""
exec(open('rework-06-lib.py').read())
A = 'vehicles/cargo-bike'
d = load(A)
AXIS = [-0.374607, 0.927184, 0]
TOP = [0.2827, 0.7636, 0]
NOTE = {'en': 'Rework 2026-11-22: the seat post (215 mm of it showing between seat tube and saddle) is modelled at this scale too, so the saddle is carried by the frame.', 'de': 'Überarbeitung 2026-11-22: Die Sattelstütze (215 mm sichtbar zwischen Sitzrohr und Sattel) ist auch in diesem Maßstab modelliert, sodass der Sattel vom Rahmen getragen wird.'}
post = {'id': 'seat-post', 'kind': 'revolve', 'bone': 'frame', 'material': 'aluminium', 'role': 'body', 'tone': 0.7, 'profile': [[0, 0.0158], [0.215, 0.0158]], 'axis': AXIS, 'origin': TOP}
insert_after(d, '200', 'body', 'seat-tube', post)
insert_after(d, '200', 'thermal', 'th-seat-tube', {'id': 'th-seat-post', 'of': 'seat-post', 'region': 'frame'})
insert_after(d, '200', 'structure', 'st-seat-tube', {'id': 'st-seat-post', 'of': 'seat-post', 'member': 'frame', 'part': 'member'})
post500 = dict(post, profile=[[0, 0.0375], [0.215, 0.0375]])
insert_after(d, '500', 'body', 'seat-tube', post500)
insert_after(d, '500', 'thermal', 'th-seat-tube', {'id': 'th-seat-post', 'of': 'seat-post', 'region': 'frame'})
insert_after(d, '500', 'structure', 'st-seat-tube', {'id': 'st-seat-post', 'of': 'seat-post', 'member': 'frame', 'part': 'member'})
for sc in ('200', '500'):
    d['volumes'][sc]['decisions'].append(NOTE)
for side, sg in (('n', -1), ('f', 1)):
    arm = find(d, '500', 'body', 'crank-arm-' + side)
    x = 0.17 if arm['transform']['translation'][0] > 0 else -0.17
    z = 0.15 if arm['transform']['translation'][2] > 0 else -0.15
    ped = {'id': 'pedal-' + side, 'kind': 'box', 'bone': 'crank', 'material': arm['material'], 'role': 'body', 'tone': 0.12, 'size': [0.09, 0.075, 0.075], 'transform': {'translation': [x, 0, z]}}
    insert_after(d, '500', 'body', 'crank-arm-' + side, ped)
    for lst in ('thermal', 'structure'):
        ref = [s for s in d['volumes']['500'][lst] if s.get('of') == 'crank-arm-' + side]
        if ref:
            n = dict(ref[0]); n['of'] = 'pedal-' + side; n['id'] = ref[0]['id'].replace('crank-arm', 'pedal')
            insert_after(d, '500', lst, ref[0]['id'], n)
save(A, d)
