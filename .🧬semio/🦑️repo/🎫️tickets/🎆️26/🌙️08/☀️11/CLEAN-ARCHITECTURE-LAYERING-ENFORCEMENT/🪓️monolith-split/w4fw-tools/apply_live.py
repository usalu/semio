import sys, os, json, hashlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from apply import *
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
def main(write):
    cut = Cut(REPO)
    builds, rw = apply(cut)
    INTENDED = ('manifest', 'm_surface')
    problems = [p for p in audit(cut, builds) if not (p[0] in ('unemitted', 'pub use dropped') and p[1] in INTENDED)]
    if problems: raise SystemExit(f'audit problems {problems}')
    api = api_names(cut)
    if not write:
        print('dry run ok'); return
    texts = write_out(cut, builds, rw)
    infos = write_support(cut, builds, api, REPO, texts)
    json.dump(infos, open(S + 'infos-live.json', 'w'), indent=1, ensure_ascii=False)
    print('written')
if __name__ == '__main__':
    main(len(sys.argv) > 1 and sys.argv[1] == 'write')
