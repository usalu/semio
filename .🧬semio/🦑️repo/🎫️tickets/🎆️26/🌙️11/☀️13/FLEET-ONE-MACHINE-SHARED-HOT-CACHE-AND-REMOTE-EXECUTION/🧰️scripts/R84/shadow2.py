#!/usr/bin/env python3
"""R84: shadow a test from a full replacement text file: shadow2.py <live rel> <text file> -> prints scratch path"""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import shadow
print(shadow.shadow(sys.argv[1],open(sys.argv[2],encoding="utf8").read(),sys.argv[3] if len(sys.argv)>3 else None))
