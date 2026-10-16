import os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, state, dump
ws, C, roots, base_ws = plan_host.prepare()
print('vis applied', C.vis_log[0], 'missed', C.vis_log[1][:3])
for k, v in C.fix_log.items():
    print(k, v if not isinstance(v, tuple) else (v[0], len(v[1])) if k == 'opaque' else v)
state.save(ws)
print('dumped', dump.dump(ws))
