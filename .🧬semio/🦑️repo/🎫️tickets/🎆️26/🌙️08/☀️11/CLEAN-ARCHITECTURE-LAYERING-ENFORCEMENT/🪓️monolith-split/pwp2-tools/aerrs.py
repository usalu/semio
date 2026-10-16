#!/usr/bin/env python3
# 🩺️ Reads the rustc JSON of sandbox A only and prints the resolution errors that are not "external crate missing" noise, grouped; usage: aerrs.py <jsonl> [--all]
import json, sys, re, collections
ARGS = list(sys.argv[1:])
RES = {'E0412', 'E0425', 'E0422', 'E0423', 'E0432', 'E0433', 'E0603', 'E0616', 'E0624', 'E0252', 'E0405', 'E0407', 'E0599', 'E0742', 'E0220', 'E0428', 'E0255', 'E0659', 'E0560', 'E0609', 'E0614', 'E0603', 'E0364', 'E0365', 'E0117', 'E0119'}
EXT = re.compile(r'`(?:semio_framework(?!_plugin_)[a-z_0-9]*|semio_framework_plugin_(?:ownership|mounted_owner)|serde(?:_json)?|pack|replication|wit_bindgen|store|protocol|dsl|vcs|tokio|futures|base64|sha2|thiserror)`')
def main():
    c = collections.Counter()
    where = {}
    for line in open(ARGS[0]):
        line = line.strip()
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        if d.get('level') != 'error':
            continue
        code = (d.get('code') or {}).get('code') or ''
        msg = d['message']
        if '--all' not in ARGS and (code not in RES):
            continue
        if EXT.search(msg) and 'pl_' not in msg:
            continue
        sp = next((s for s in d.get('spans', []) if s.get('is_primary')), None)
        f = sp['file_name'].split('/🔌️plugin/')[-1] if sp else ''
        lab = (sp.get('label') or '') if sp else ''
        key = (code, msg[:140], lab[:60])
        c[key] += 1
        where.setdefault(key, collections.Counter())[f[-70:]] += 1
    tot = 0
    for k, v in sorted(c.items(), key=lambda kv: -kv[1]):
        tot += v
        print(v, k[0], k[1], '|', k[2], '|', where[k].most_common(2))
    print('signal-class errors:', tot)

if __name__ == '__main__':
    main()
