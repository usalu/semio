#!/usr/bin/env python3
"""💾️ Persists the in-memory workspace of the carve so later tools (law map, manifests, flush, verification) start from the exact generated tree."""
import os, sys, pickle, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ws as wsm, hcmod, plan_host
from carve import Model

PATH = '/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/phc-state.pkl'


def save(ws):
    pickle.dump(dict(cur=ws.cur, created=ws.created, deleted=ws.deleted, orig=ws.orig), open(PATH, 'wb'))


def load():
    st = pickle.load(open(PATH, 'rb'))
    ws = wsm.WS()
    ws.cur, ws.created, ws.deleted, ws.orig = st['cur'], st['created'], st['deleted'], st['orig']
    roots = plan_host.prepare_roots()
    hcmod.FS = ws
    model = Model(ws, roots)
    return ws, model, roots


def base_ws(ws):
    """A workspace holding the monolith as it was before the flush: every file the carve read, with its original text."""
    b = wsm.WS()
    b.cur = dict(ws.orig)
    return b
