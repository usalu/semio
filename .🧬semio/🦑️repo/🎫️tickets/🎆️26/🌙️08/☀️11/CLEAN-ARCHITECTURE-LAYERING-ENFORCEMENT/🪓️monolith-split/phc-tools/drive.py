#!/usr/bin/env python3
"""🚦️ One end-to-end generation: plan, rewrite passes, privacy repair rounds against the oracle, final differential."""
import os, sys, json, collections, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, oracle_host as O, autovis
from carve import REPO

mode = sys.argv[1] if len(sys.argv) > 1 else 'native'
ws, C, roots, base_ws = plan_host.prepare()
print('prepared; changed files', len(ws.changed()), flush=True)
if mode in ('native', 'all'):
    O.repair(ws, C, roots, rounds=int(os.environ.get('ROUNDS', '3')))
    O.run_native(ws, roots)
if mode in ('test', 'all'):
    for c in (sys.argv[2:] or list(roots)):
        O.run_test(ws, roots, c)
import dump
print('dumped', dump.dump(ws))
