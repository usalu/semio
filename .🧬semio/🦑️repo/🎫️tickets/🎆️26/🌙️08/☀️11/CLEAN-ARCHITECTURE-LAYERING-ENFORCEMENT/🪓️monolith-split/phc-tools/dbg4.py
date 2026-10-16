import sys, os, json, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, oracle_host as O, union_host as U
ws, C, roots, base_ws = plan_host.prepare()
s = O.split('native', ws, roots, None)
pickle.dump(s[1], open(O.CACHE + '/split-native-diags.pkl', 'wb'))
for d in s[1]:
    k = U.key_of(d)
    if k and k[0] in ('E0603', 'E0423') :
        print(json.dumps({'msg': d['message'], 'code': d['code'], 'spans': [(x['file_name'], x['line_start'], x['is_primary'], x.get('label')) for x in d['spans']], 'children': [(c['message'], [(x['file_name'], x['line_start'], x.get('label')) for x in c['spans']]) for c in d.get('children', [])]}, ensure_ascii=False)[:900])
