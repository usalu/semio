"""♻️ Restores the carve sources from the pre-edit snapshot so the carve tools can run again (own files only): the time-travel root, the two lift stubs and the tool-run root; removes the piece files the carve created. usage: python3 restore5.py"""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lib5 as L
for rel in ['⏪️time-travel/🦀️.rs', '🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs', '⏯️tool-run/🦀️.rs', '👷️job/🛬️lifted/⏯️tool-run/🦀️.rs']:
    shutil.copy2(L.SNAP + '/' + rel, L.PLUG + '/' + rel)
for rel in ['⏪️time-travel/🪟️panel/🔢️pointer/🦀️.rs', '⏪️time-travel/🪟️panel/🖼️sections/🦀️.rs']:
    p = L.PLUG + '/' + rel
    if os.path.exists(p): os.remove(p)
    d = os.path.dirname(p)
    if os.path.isdir(d) and not os.listdir(d): os.rmdir(d)
print('restored')
