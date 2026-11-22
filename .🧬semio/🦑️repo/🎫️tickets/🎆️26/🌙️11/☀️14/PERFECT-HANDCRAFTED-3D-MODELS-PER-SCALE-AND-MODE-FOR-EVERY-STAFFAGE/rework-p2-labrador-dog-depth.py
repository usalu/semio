"""📏 Labrador: declared lateral dimension follows the narrowed trunk (0.28 m -> 0.258 m, measured across the upper arms).

Usage: python3 rework-p2-labrador-dog-depth.py
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('girdle', os.path.join(T, 'rework-p2-labrador-dog-girdle.py'))
girdle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(girdle)

text = open(girdle.CATALOG, encoding='utf-8').read()
dimensions = json.loads(text)['asset']['dimensions']
dimensions['depthM'] = 0.258
open(girdle.CATALOG, 'w', encoding='utf-8').write(girdle.splice(text, 'dimensions', dimensions, girdle.insert().dump))
