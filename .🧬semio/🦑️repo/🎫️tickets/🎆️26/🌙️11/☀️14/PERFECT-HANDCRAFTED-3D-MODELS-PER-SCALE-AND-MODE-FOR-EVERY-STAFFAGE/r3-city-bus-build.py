"""🏗️ Round 3 city-bus: writes the handcrafted lists of every scale into `volumes/vehicles/city-bus.json` (`python3 r3-city-bus-build.py [50 100 200 500]`)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import r3_city_bus_body50 as B50

T = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(T, 'volumes', 'vehicles', 'city-bus.json')


def lists(scale):
    mods = {}
    if scale == 50:
        mods['body'] = B50.body50()
        try:
            import r3_city_bus_lists50 as L50
            mods['thermal'] = L50.thermal50(mods['body'])
            mods['structure'] = L50.structure50()
            mods['decisions'] = L50.DECISIONS
        except ImportError:
            pass
    else:
        import r3_city_bus_coarse as C
        mods = C.scale(scale)
    return mods


if __name__ == '__main__':
    scales = [int(a) for a in sys.argv[1:]] or [50]
    d = json.load(open(SRC))
    for s in scales:
        for k, v in lists(s).items():
            d['volumes'][str(s)][k] = v
        print(s, {k: len(d['volumes'][str(s)][k]) for k in ('body', 'thermal', 'structure', 'decisions')})
    tmp = SRC + '.r3.tmp'
    json.dump(d, open(tmp, 'w'), ensure_ascii=False)
    os.replace(tmp, SRC)
