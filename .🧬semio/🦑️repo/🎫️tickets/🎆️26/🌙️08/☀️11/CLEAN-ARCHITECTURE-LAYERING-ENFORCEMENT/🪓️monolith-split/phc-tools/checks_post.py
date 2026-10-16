#!/usr/bin/env python3
"""🔍️ Runs a check module's `main` after the flush: every base workspace reads the pre-carve text kept in the persisted state instead of the carved disk (usage: checks_post.py checks_host|bodycheck)."""
import os, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, ws as wsm

ws, model, roots = state.load()
make = wsm.WS


def base_factory(*a, **k):
    b = make(*a, **k)
    b.cur = dict(ws.orig)
    return b


state.load = lambda: (ws, model, roots)
wsm.WS = base_factory
importlib.import_module(sys.argv[1] if len(sys.argv) > 1 else 'checks_host').main()
