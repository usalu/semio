"""EN 1992 v8: adds `length` (m, 0 = derived) and `bend` (string vocabulary auto|straight|hook|u-hook) to BarPoint in the model and its snapshot twins. Idempotent."""
import pathlib, re
A = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema')
def rd(p): return p.read_text()
def wr(p, s): p.write_text(s)
def sub(p, old, new):
    s = rd(p)
    if new in s: print('skip', p.name); return
    assert s.count(old) == 1, (str(p), old[:80], s.count(old))
    wr(p, s.replace(old, new)); print('edit', p.relative_to(A))

sub(A / '📸️snapshot/🦀️.rs', '    pub z: f64,\n    pub diameter: f64,\n}\n\n/// 📍 TendonPoint', '    pub z: f64,\n    pub diameter: f64,\n    pub length: f64,\n    pub bend: String,\n}\n\n/// 📍 TendonPoint')

LEN = '''        "length": {
          "type": "number",
          "x-semio-ui": {
            "label": {
              "en": "Bar length",
              "de": "Stablänge"
            },
            "widget": "stepper",
            "unit": "m",
            "displayUnit": "mm",
            "displayFactor": 1000,
            "precision": 0,
            "step": 0.05
          }
        },
        "bend": {
          "type": "string"
        }
'''
for p in (A / '📸️snapshot/🔣️.json', A / '🔣️.json'):
    s = rd(p)
    if '"bend": {' in s and s.index('"bend": {') > s.index('"BarPoint": {'): print('skip', p.name); continue
    i = s.index('    "BarPoint": {'); j = s.index('    "TendonPoint": {')
    blk = s[i:j]
    blk = blk.replace('        "z",\n        "diameter"\n      ],', '        "z",\n        "diameter",\n        "length",\n        "bend"\n      ],', 1)
    k = blk.index('"precision": 1,\n            "step": 0.005\n          }\n        }\n      },\n      "x-semio-formats"')
    tail = '"precision": 1,\n            "step": 0.005\n          }\n        }\n      },\n      "x-semio-formats"'
    assert blk.count(tail) == 1
    blk = blk.replace(tail, '"precision": 1,\n            "step": 0.005\n          }\n        },\n' + LEN + '      },\n      "x-semio-formats"', 1)
    wr(p, s[:i] + blk + s[j:]); print('edit', p.relative_to(A))

for p in (A / '📸️snapshot/🟦️.ts', A / '🟦️.ts'):
    sub(p, '  z: number;\n  diameter: number;\n}\n\nexport interface TendonPoint', '  z: number;\n  diameter: number;\n  length: number;\n  bend: string;\n}\n\nexport interface TendonPoint')
    sub(p, 'diameter: normWireRequired(normWireNumber) });\nexport const parseTendonPoint', 'diameter: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), bend: normWireRequired(normWireString) });\nexport const parseTendonPoint') if 'export const parseTendonPoint' in rd(p) else None
for p in (A / '📸️snapshot/🔗️.graphql', A / '🔗️.graphql'):
    sub(p, 'type BarPoint { id: String! layerId: String! y: Float! z: Float! diameter: Float! }', 'type BarPoint { id: String! layerId: String! y: Float! z: Float! diameter: Float! length: Float! bend: String! }')
for p in (A / '📸️snapshot/🛰️.proto', A / '🛰️.proto'):
    sub(p, 'double z = 4; double diameter = 5; }', 'double z = 4; double diameter = 5; double length = 6; string bend = 7; }') if 'message BarPoint' in rd(p) else None
