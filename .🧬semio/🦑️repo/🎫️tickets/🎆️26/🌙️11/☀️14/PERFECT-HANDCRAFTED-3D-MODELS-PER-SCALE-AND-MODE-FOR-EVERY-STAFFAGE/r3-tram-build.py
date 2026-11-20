"""🏗️ Round 3 tram: writes the handcrafted lists of every scale into `volumes/vehicles/tram.json` (`python3 r3-tram-build.py [50 100 200 500]`)."""
import importlib.util, json, os, sys

T = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(T, 'volumes', 'vehicles', 'tram.json')


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace('-', '_'), os.path.join(T, name + '.py'))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


def lists(scale):
    if scale == 50:
        m = load('r3-tram-s50')
        out = {'body': m.body50()}
        if hasattr(m, 'thermal50'):
            out['thermal'] = m.thermal50(out['body'])
            out['structure'] = m.structure50(out['body'])
            out['decisions'] = m.DECISIONS
        return out
    return load('r3-tram-coarse').scale(scale)


if __name__ == '__main__':
    scales = [int(a) for a in sys.argv[1:]] or [50]
    d = json.load(open(SRC))
    for s in scales:
        for k, v in lists(s).items():
            ids = [x['id'] for x in v] if k != 'decisions' else []
            assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]
            d['volumes'][str(s)][k] = v
        print(s, {k: len(d['volumes'][str(s)][k]) for k in ('body', 'thermal', 'structure', 'decisions')})
    tmp = SRC + '.r3.tmp'
    json.dump(d, open(tmp, 'w'), ensure_ascii=False)
    os.replace(tmp, SRC)
