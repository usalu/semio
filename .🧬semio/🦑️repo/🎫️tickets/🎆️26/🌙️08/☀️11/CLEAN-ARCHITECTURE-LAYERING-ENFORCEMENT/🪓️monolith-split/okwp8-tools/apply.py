"""🚀 Builds everything, validates the staged output and writes it through the snapshotting writer."""
import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import genall as G, validate as V, wp8lib as L
def main(do_apply):
    D = G.build_all()
    out, dele = G.assemble(D)
    ke, removed = G.kernel_edits(D)
    for p, t in ke.items():
        assert p not in out, p
        out[p] = t
    want = G.widen_plan(out)
    wout, rep = G.widen_apply(want)
    for p, t in wout.items():
        assert p not in out, p
        out[p] = t
    print('files', len(out), 'delete', len(dele), 'widened files', len(wout), 'widen items', len(want), 'problems', D['problems'], 'prelude missing', D['prelude_missing'])
    if not do_apply:
        return out, dele
    for p, t in out.items():
        if os.path.exists(p) and L.read(p) == t: continue
        L.write(p, t)
    for p in dele: L.delete(p)
    return out, dele
if __name__ == '__main__':
    main(len(sys.argv) > 1 and sys.argv[1] == '--apply')
