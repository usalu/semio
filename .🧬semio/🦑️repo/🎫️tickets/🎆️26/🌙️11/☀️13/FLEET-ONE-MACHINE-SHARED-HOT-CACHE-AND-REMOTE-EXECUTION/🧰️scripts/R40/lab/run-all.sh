#!/bin/zsh
# usage: run it inside an empty scratch dir OUTSIDE the worktree (needs `brotli`, the persisted generation of .cache/b/g/m read only, and the toolchain nightly-2026-07-20). Writes about 700 MB while it runs; delete model.jsonl and rust-nodes.jsonl afterwards.
HERE=${0:A:h}; cp $HERE/*.py $HERE/*.ts $HERE/*.sh . 2>/dev/null
mkdir -p iface/🧬️schema; T=${HERE:h:h}
cp "$HERE/iface-r39-masker-copy.ts.txt" iface/🟦️.ts
python3 dump.py && python3 prune.py && rm rust-nodes.jsonl
python3 oracle_meta.py
python3 survey2.py && python3 keysets.py
python3 cmp_deps.py && python3 analyze3.py && python3 top10.py && python3 extras.py
python3 replay.py && python3 replay2.py
python3 manifest_jobs.py && BUN manifest_proj.ts
python3 -c "import pickle,json;m,k,t=pickle.load(open('keys.pkl','rb'));json.dump(sorted({f for l in t.values() for f in l if f.endswith('.rs')}),open('rsfiles.json','w'),ensure_ascii=False)"
BUN timing.ts; BUN splice.ts
