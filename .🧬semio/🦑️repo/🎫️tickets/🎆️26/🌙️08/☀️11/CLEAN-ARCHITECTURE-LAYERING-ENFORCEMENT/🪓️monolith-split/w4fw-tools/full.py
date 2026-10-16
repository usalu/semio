import sys, os, shutil, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from apply import *
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/sb/"
BASE = 'base2'
def run_sandbox():
    shutil.rmtree(S + 'work', ignore_errors=True)
    shutil.copytree(S + BASE, S + 'work')
    for f in SUPPORT_FILES:
        os.makedirs(os.path.dirname(S + 'work/' + f), exist_ok=True)
        shutil.copy(S.replace('sb/', 'baseline-support/') + f, S + 'work/' + f)
    cut = Cut(S + 'work/')
    builds, rw = apply(cut)
    api = api_names(cut)
    problems = audit(cut, builds)
    texts = write_out(cut, builds, rw)
    infos = write_support(cut, builds, api, S + 'work/', texts)
    json.dump(infos, open(S + 'infos.json', 'w'), indent=1, ensure_ascii=False)
    return cut, builds, rw, infos, problems
if __name__ == '__main__':
    cut, builds, rw, infos, problems = run_sandbox()
    print('audit', [p for p in problems if p[0] not in ('unemitted', 'pub use dropped')])
    print(rw.log)
    for c, i in infos.items(): print(c, i)
