"""🦊 Red fox body: the shoulder as two rounded muscle masses, one on the scapula and one on the humerus.

Usage: python3 rework-red-fox-body.py   (edits the volumes source in place, idempotent; publish with volumes-insert.py)
Stations are (x along the bone, y towards the bone normal, rx lateral, ry across); z is the lateral offset of the near side.
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, 'volumes/animals/red-fox.json')
LIMB = [0, -0.70711, 0, 0.70711]

BLADE = {
    '50': [(-0.006, 0, 0.008, 0.012), (0.010, 0, 0.014, 0.024), (0.035, 0, 0.018, 0.030), (0.065, 0, 0.020, 0.030), (0.090, 0, 0.021, 0.027), (0.105, 0, 0.019, 0.022), (0.113, 0, 0.012, 0.014)],
    '100': [(-0.004, 0, 0.009, 0.014), (0.020, 0, 0.016, 0.027), (0.060, 0, 0.020, 0.030), (0.098, 0, 0.020, 0.025), (0.112, 0, 0.012, 0.014)],
    '200': [(0.0, 0, 0.0165, 0.020), (0.045, 0, 0.019, 0.030), (0.100, 0, 0.019, 0.024)],
}
ARM = {
    '50': [(-0.016, 0, 0.010, 0.012), (-0.006, 0, 0.019, 0.022), (0.010, -0.002, 0.023, 0.028), (0.040, -0.006, 0.023, 0.031), (0.075, -0.009, 0.021, 0.029), (0.105, -0.008, 0.017, 0.023), (0.124, -0.006, 0.0135, 0.0175), (0.134, -0.005, 0.008, 0.010)],
    '100': [(-0.014, 0, 0.011, 0.013), (0.004, -0.001, 0.022, 0.027), (0.045, -0.006, 0.023, 0.031), (0.095, -0.008, 0.018, 0.025), (0.124, -0.006, 0.0135, 0.0175), (0.134, -0.005, 0.009, 0.010)],
    '200': [(-0.008, 0, 0.0165, 0.0165), (0.020, -0.003, 0.022, 0.030), (0.075, -0.008, 0.020, 0.028), (0.130, -0.005, 0.0165, 0.0165)],
}
ARM_INSET = 0.004


def loft(ident, bone, stations, z):
    sections = [{'at': [x, y, z], 'rx': rx, 'ry': ry, 'exponent': 2.1, 'rotation': LIMB} for x, y, rx, ry in stations]
    return {'id': ident, 'kind': 'loft', 'bone': bone, 'material': 'red-fur', 'role': 'body', 'tone': 0.7, 'sections': sections, 'capStart': True, 'capEnd': True}


def main():
    source = json.load(open(SOURCE, encoding='utf-8'))
    for scale in ('50', '100', '200'):
        volume = source['volumes'][scale]
        for name, tag, inward in (('near', 'fn', 1), ('far', 'ff', -1)):
            body = [s for s in volume['body'] if s['id'] != name + '-shoulder-blade']
            at = next(i for i, s in enumerate(body) if s['id'] == name + '-shoulder')
            body[at] = loft(name + '-shoulder', 'humerus-' + tag, ARM[scale], inward * ARM_INSET)
            body.insert(at + 1, loft(name + '-shoulder-blade', 'scapula-' + tag, BLADE[scale], 0))
            volume['body'] = body
            thermal = [s for s in volume['thermal'] if s['id'] != 'th-' + name + '-shoulder-blade']
            at = next(i for i, s in enumerate(thermal) if s['id'] == 'th-' + name + '-shoulder')
            thermal.insert(at + 1, {'id': 'th-' + name + '-shoulder-blade', 'of': name + '-shoulder-blade', 'region': 'coat-trunk', 'layer': 'fur'})
            volume['thermal'] = thermal
    json.dump(source, open(SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
    print('body written', SOURCE)


main()
