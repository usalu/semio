#!/usr/bin/env python3
"""🔁️ Round-trip check for `🐍️semio-pattern-a-vectors.py`'s decoders: re-encode each decoded real
artifact back to `.dsl.semio` text with an independently written Python ENCODER and compare it byte
for byte with the committed file. The committed text is this repository's own `print_dsl` output, so
an exact match is evidence that the Python decoder read the real grammar correctly — which is what
every committed specification vector's `before` snapshot rests on."""

import importlib.util, struct, sys

spec = importlib.util.spec_from_file_location("g", "🐍️semio-pattern-a-vectors.py")
g = importlib.util.module_from_spec(spec)
spec.loader.exec_module(g)

def hx(s):
    return s.encode("utf-8").hex()

def f(v):
    return str(int(v)) if float(v).is_integer() else repr(float(v))

def bits(v):
    return str(struct.unpack("<Q", struct.pack("<d", float(v)))[0])

def b(v):
    return "1" if v else "0"

def opt(v, enc):
    return "[0]" if v is None else f"[1,{enc(v)}]"

def lst(items, enc):
    return "[" + ",".join(enc(i) for i in items) + "]"

def p2(p):
    return f"[{f(p['x'])},{f(p['y'])}]"

# flow
def e_flow(s):
    def node(n):
        return f"[{hx(n['id'])},{hx(n['kind'])},{hx(n['label'])},{lst(n['params'], lambda p: f'[{hx(p[chr(107)+chr(101)+chr(121)])},{hx(p[chr(118)+chr(97)+chr(108)+chr(117)+chr(101)])}]')},{p2(n['position'])}]"
    def port(r):
        return f"[{hx(r['node'])},{hx(r['port'])}]"
    def edge(e):
        return f"[{hx(e['id'])},{port(e['from'])},{port(e['to'])},{hx(e['kind'])}]"
    return f"semio stdio.semio.flow.dsl v1\nschema={hx(s['schema'])}\nnodes={lst(s['nodes'], node)}\nedges={lst(s['edges'], edge)}"

# cad
def e_cad_entity(e):
    k = e["kind"]
    if k == "line":
        return f"L[{p2(e['a'])},{p2(e['b'])}]"
    if k == "arc":
        return f"A[{p2(e['center'])},{f(e['radius'])},{f(e['start_angle'])},{f(e['end_angle'])}]"
    if k == "circle":
        return f"C[{p2(e['center'])},{f(e['radius'])}]"
    if k == "ellipse":
        return f"E[{p2(e['center'])},{p2(e['major_axis_end'])},{f(e['ratio'])},{f(e['start_param'])},{f(e['end_param'])}]"
    if k == "polyline":
        return f"P[{lst(e['vertices'], p2)},{b(e['closed'])}]"
    if k == "text":
        return f"T[{p2(e['position'])},{f(e['height'])},{f(e['rotation'])},{hx(e['content'])}]"
    if k == "insert":
        return f"I[{hx(e['block_name'])},{p2(e['insertion_point'])},{p2(e['scale'])},{f(e['rotation'])}]"
    if k == "solid":
        return f"S[{p2(e['p1'])},{p2(e['p2'])},{p2(e['p3'])},{p2(e['p4'])}]"
    if k == "dimension":
        return f"D[{p2(e['def_point'])},{p2(e['text_position'])},{f(e['measurement'])},{hx(e['text'])}]"
    raise ValueError(k)

def e_cad(s):
    rec = lambda r: f"[{hx(r['handle'])},{hx(r['layer'])},{e_cad_entity(r['entity'])}]"
    layer = lambda l: f"[{hx(l['name'])},{l['colorIndex']},{hx(l['lineType'])},{b(l['visible'])}]"
    block = lambda bl: f"[{hx(bl['name'])},{p2(bl['basePoint'])},{lst(bl['entities'], rec)}]"
    return ("semio stdio.semio.cad.dsl v1\n"
            f"schema={hx(s['schema'])}\nlayers={lst(s['layers'], layer)}\nblocks={lst(s['blocks'], block)}\nentities={lst(s['entities'], rec)}")

# document
def e_style(st):
    return f"[{b(st['bold'])},{b(st['italic'])},{b(st['underline'])},{opt(st['size'], bits)},{opt(st['font'], hx)},{opt(st['color'], hx)},{opt(st['link'], hx)}]"

def e_run(r):
    return f"[{hx(r['text'])},{e_style(r['style'])}]"

def e_doc_block(bl):
    k = bl["kind"]
    if k == "paragraph":
        return f"P[{opt(bl['style_id'], hx)},{lst(bl['runs'], e_run)}]"
    if k == "heading":
        return f"H[{bl['level']},{opt(bl['style_id'], hx)},{lst(bl['runs'], e_run)}]"
    if k == "list":
        return f"L[{b(bl['ordered'])},{lst(bl['items'], lambda i: '[' + lst(i['blocks'], e_doc_block) + ']')}]"
    if k == "table":
        cell = lambda c: "[" + lst(c["blocks"], e_doc_block) + "]"
        row = lambda r: "[" + lst(r["cells"], cell) + "]"
        return f"T[{lst(bl['rows'], row)}]"
    if k == "code":
        return f"C[{opt(bl['language'], hx)},{hx(bl['text'])}]"
    if k == "quote":
        return f"Q[{lst(bl['blocks'], e_doc_block)}]"
    if k == "image":
        return f"I[{hx(bl['image_id'])},{hx(bl['alt'])},{opt(bl['width'], bits)},{opt(bl['height'], bits)}]"
    if k == "pageBreak":
        return "B[]"
    raise ValueError(k)

def e_doc(s):
    style = lambda st: f"[{hx(st['id'])},{hx(st['name'])},{opt(st['basedOn'], hx)}]"
    image = lambda im: f"[{hx(im['id'])},{hx(im['mime'])},{bytes(im['bytes']).hex()}]"
    return ("semio s.stdio.semio.document.dsl v1\n"
            f"schema={hx(s['schema'])}\nstyles={lst(s['styles'], style)}\nimages={lst(s['images'], image)}\nblocks={lst(s['blocks'], e_doc_block)}")

if __name__ == "__main__":
    CASES = [
        ("flow  ", "🌊️pipeline", g.decode_flow, e_flow),
        ("cad   ", "📐️drawing", g.decode_cad, e_cad),
        ("doc   ", "📄️memo", g.decode_document, e_doc),
    ]

    failed = 0
    for name, example, decode, encode in CASES:
        committed = g.read(f"{g.EXAMPLES}/{example}/🖼️assets/🗣️example.dsl.semio")
        produced = encode(decode(committed))
        if produced == committed:
            print(f"{name} re-encode is byte-identical to the committed artifact ({len(committed)} B)")
        else:
            failed += 1
            print(f"{name} MISMATCH")
            for i, (a, bch) in enumerate(zip(committed, produced)):
                if a != bch:
                    print(f"   first difference at byte {i}: committed {committed[i-40:i+40]!r} vs produced {produced[i-40:i+40]!r}")
                    break
            else:
                print(f"   length differs: committed {len(committed)} vs produced {len(produced)}")
    sys.exit(1 if failed else 0)
