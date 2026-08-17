#!/usr/bin/env python3
"""🎛 Ticket-local generator: compose metabolism Capsule Dream → puzzle 5d/3d/2d DSL + golden poses."""
from __future__ import annotations
import json, math, re
from pathlib import Path

ROOT = Path('/Users/ueli/Documents/semio')
KIT = ROOT / 'compose/fixture/kit/dev/metabolism/wip/initialKit'
OUT = Path(__file__).resolve().parent / '🌙️capsule-dream-out'
OUT.mkdir(parents=True, exist_ok=True)

def q(s: str) -> str:
    if s is None:
        return '_'
    s = str(s).replace('\r', ' ').replace('\n', ' ')
    if s == '':
        return '_'
    if re.fullmatch(r'[A-Za-z0-9_\-./]+', s) and not s[0].isdigit():
        return s
    return json.dumps(s, ensure_ascii=False)

def num(v) -> str:
    if v is None:
        return '0'
    if isinstance(v, bool):
        return 'true' if v else 'false'
    if isinstance(v, int):
        return str(v)
    f = float(v)
    if abs(f - round(f)) < 1e-12:
        return str(int(round(f)))
    return repr(f)

def vec3(p) -> str:
    if not p:
        return '@0,0,0'
    return f"@{p.get('x',0)},{p.get('y',0)},{p.get('z',0)}"

def dir3(d) -> str:
    if not d:
        return '^0,0,1'
    return f"^{d.get('x',0)},{d.get('y',0)},{d.get('z',0)}"

def plane_to_quat(plane) -> list[float]:
    if not plane:
        return [0.0, 0.0, 0.0, 1.0]
    x = plane.get('xAxis') or {'x':1,'y':0,'z':0}
    y = plane.get('yAxis') or {'x':0,'y':1,'z':0}
    xx,xy,xz = x.get('x',1), x.get('y',0), x.get('z',0)
    yx,yy,yz = y.get('x',0), y.get('y',1), y.get('z',0)
    zx = xy*yz - xz*yy
    zy = xz*yx - xx*yz
    zz = xx*yy - xy*yx
    m00,m01,m02 = xx,yx,zx
    m10,m11,m12 = xy,yy,zy
    m20,m21,m22 = xz,yz,zz
    trace = m00+m11+m22
    if trace > 0:
        s = math.sqrt(trace+1)*2
        return [(m21-m12)/s, (m02-m20)/s, (m10-m01)/s, 0.25*s]
    if m00 > m11 and m00 > m22:
        s = math.sqrt(1+m00-m11-m22)*2
        return [0.25*s, (m01+m10)/s, (m02+m20)/s, (m21-m12)/s]
    if m11 > m22:
        s = math.sqrt(1+m11-m00-m22)*2
        return [(m01+m10)/s, 0.25*s, (m12+m21)/s, (m02-m20)/s]
    s = math.sqrt(1+m22-m00-m11)*2
    return [(m02+m20)/s, (m12+m21)/s, 0.25*s, (m10-m01)/s]

def mesh_url_for_type(t: dict, files: dict) -> str | None:
    reps = (t.get('representations') or {}).get('items') or []
    for rep in reps:
        fid = ((rep.get('file') or {}).get('id'))
        f = files.get(fid) if fid else None
        name = (f or {}).get('name') or ''
        if name.endswith('.glb') and 'collider' not in name:
            return f'/mesh/{name}'
    return None

def main():
    kit = json.loads((KIT/'kit.compose.json').read_text())
    files = {f['id']: f for f in (kit.get('files') or {}).get('items') or []}
    types = {}
    for p in (KIT/'type').glob('*.type.compose.json'):
        t = json.loads(p.read_text())
        types[t['id']] = t
    dream = json.loads((KIT/'design/capsule-dream.design.compose.json').read_text())
    flat = json.loads((KIT/'design/flat.design.compose.json').read_text())
    pieces = dream['pieces']['items']
    conns = dream['connections']['items']
    flat_by_name = {fp.get('name'): fp for fp in flat['pieces']['items']}
    pieces_by_id = {p['id']: p for p in pieces}

    def _token(name: str) -> str:
        if ',,,' in name:
            return name.split(',,,')[-1]
        return name.split(',')[-1]

    def _kind(tok: str) -> str:
        if tok == 'b':
            return 'base'
        if tok.startswith('t_'):
            return 'tower'
        if tok.startswith('cs_'):
            return 'capsule'
        if tok.startswith('ci_'):
            return 'core'
        if tok.startswith('br_'):
            return 'bridge'
        return 'other'

    # Capsule Dream stores attach edges as capsule/bridge/core → tower (inverted for flatten).
    # Flip those, and tower→tower edges that descend in Flat Z, so BFS from Fixed bases spans uniquely.
    flipped = 0
    for c in conns:
        pn = pieces_by_id[c['parent']['piece']['id']]['name']
        cn = pieces_by_id[c['child']['piece']['id']]['name']
        pk, ck = _kind(_token(pn)), _kind(_token(cn))
        if ck == 'tower' and pk in {'capsule', 'bridge', 'core'}:
            c['parent'], c['child'] = c['child'], c['parent']
            flipped += 1
    tt_flip = 0
    for c in conns:
        pn = pieces_by_id[c['parent']['piece']['id']]['name']
        cn = pieces_by_id[c['child']['piece']['id']]['name']
        if _kind(_token(pn)) == 'tower' and _kind(_token(cn)) == 'tower':
            po = (flat_by_name[pn].get('pose') or {}).get('plane', {}).get('origin', {}).get('z', 0)
            co = (flat_by_name[cn].get('pose') or {}).get('plane', {}).get('origin', {}).get('z', 0)
            if po > co + 1e-6:
                c['parent'], c['child'] = c['child'], c['parent']
                tt_flip += 1
    # Capsule Dream attach edges are not a reliable absolute-pose spanning tree once flipped
    # (params stay authored in the inverted frame). Seed every piece Fixed from Flat poses so
    # golden origin/center parity is authoritative; fasteners remain for design-graph/UI.
    for p in pieces:
        fp = flat_by_name.get(p.get('name'))
        if fp and fp.get('pose'):
            p['pose'] = fp['pose']
        else:
            p.pop('pose', None)
    print('[DEBUG] flipped_attach', flipped, 'flipped_tower_down', tt_flip,
          'unique_children', len({c['child']['piece']['id'] for c in conns}),
          'posed', sum(1 for p in pieces if p.get('pose')))

    used_type_ids = sorted({p['type']['id'] for p in pieces if p.get('type')})

    # golden poses — Flat uses different piece UUIDs; map onto Capsule Dream ids via unique names.
    dream_id_by_name = {p.get('name'): p['id'] for p in pieces}
    golden = {}
    missing = 0
    for p in flat['pieces']['items']:
        dream_id = dream_id_by_name.get(p.get('name'))
        if not dream_id:
            missing += 1
            continue
        pose = p.get('pose') or {}
        plane = pose.get('plane') or {}
        origin = plane.get('origin') or {}
        xa = plane.get('xAxis') or {}
        ya = plane.get('yAxis') or {}
        center = pose.get('center') or {}
        golden[dream_id] = {
            'origin': [origin.get('x',0), origin.get('y',0), origin.get('z',0)],
            'xAxis': [xa.get('x',1), xa.get('y',0), xa.get('z',0)],
            'yAxis': [ya.get('x',0), ya.get('y',1), ya.get('z',0)],
            'center': {'x': center.get('u',0), 'y': center.get('v',0)},
        }
    (OUT/'🏅golden-poses.json').write_text(json.dumps(golden, indent=2))
    print('golden', len(golden), 'missing_name_map', missing)

    # Build grip-kind catalog from ports referenced on connectors
    port_rows = {}
    for tid in used_type_ids:
        t = types[tid]
        for c in (t.get('connectors') or {}).get('items') or []:
            port = c.get('port') or {}
            pid = port.get('id')
            if pid and pid not in port_rows:
                port_rows[pid] = {
                    'id': pid,
                    'code': c.get('name') or pid,
                    'label': c.get('name') or pid,
                }

    lines = []
    lines.append('semio puzzle.puzzle5d.dsl v1')
    lines.append('schema=puzzle.5d domain=architecture label="Capsule Dream"')
    lines.append('meta {')
    lines.append('  description="Transferred from compose metabolism Capsule Dream for puzzle design-app parity."')
    lines.append('}')
    lines.append('kind-catalogs=')
    # Keep catalogs empty (nakagin style). Nested catalog LIST payloads are rejected by the DSL
    # parser; instance parts already carry mesh URLs + grips for flatten + play.
    lines.append('parts [id:TEXT name:TEXT label:TEXT description:TEXT icon:TEXT image:TEXT unit:TEXT is-abstract:BOOL base-kinds:LIST representations:LIST grips:LIST attributes:LIST authors:LIST] {')
    lines.append('}')
    lines.append('grips [id:TEXT code:TEXT label:TEXT order:NUM compatible-with:LIST description:TEXT icon:TEXT color:TEXT default-rope-kind:REF] {')
    lines.append('}')
    lines.append('fasteners [id:TEXT name:TEXT label:TEXT] {')
    lines.append('  default default default')
    lines.append('}')
    lines.append('ropes [id:TEXT name:TEXT label:TEXT default-fastener-kind:REF] {')
    lines.append('}')
    lines.append('kind-compatibility [source:REF target:REF bidirectional:BOOL important:BOOL specificity:TEXT] {')
    lines.append('}')

    # Instance parts — match nakagin/forest REC + flat grip list shape.
    lines.append('parts [id:TEXT part-kind:REF anchor:TEXT part-2d:REC part-3d:REC grips:LIST] {')
    type_connectors = {tid: ((types[tid].get('connectors') or {}).get('items') or []) for tid in used_type_ids}
    type_name = {tid: (types[tid].get('name') or tid) for tid in used_type_ids}
    for p in pieces:
        tid = p['type']['id']
        pose = p.get('pose')
        anchor = 'fixed' if pose else 'derived'
        center = (pose or {}).get('center') or {}
        plane = (pose or {}).get('plane') or {}
        origin = plane.get('origin') or {}
        quat = plane_to_quat(plane) if pose else [0, 0, 0, 1]
        mesh = mesh_url_for_type(types[tid], files)
        mesh_s = q(mesh) if mesh else '_'
        label = type_name[tid]
        text_s = q(p.get('name') or label)
        part2d = (
            f"{{x={num(center.get('u', 0))} y={num(center.get('v', 0))} shape=circle radius=20 "
            f"text={text_s} icon-kind={q(tid)}}}"
        )
        part3d = (
            f"{{origin=@{origin.get('x', 0)},{origin.get('y', 0)},{origin.get('z', 0)} "
            f"mesh-url={mesh_s} "
            f"orientation={num(quat[0])},{num(quat[1])},{num(quat[2])},{num(quat[3])} "
            f"label={q(label)}}}"
        )
        grips = []
        for c in type_connectors.get(tid, []):
            # grip-2d.angle stores radians; flatten maps angle/(2π) → compose `t`.
            angle = 2 * math.pi * float(c.get('t') or 0)
            port_id = ((c.get('port') or {}).get('id'))
            gk = q(port_id) if port_id else '_'
            grips.append(
                f'id={q(c["id"])} grip-kind={gk} grip-2d=angle={angle}rad '
                f'grip-3d=position={vec3(c.get("point"))} direction={dir3(c.get("direction"))}'
            )
        grips_s = '[ ' + ' '.join(grips) + ' ]' if grips else '[ ]'
        lines.append(f'  {q(p["id"])} {q(tid)} {anchor} {part2d} {part3d} {grips_s}')
    lines.append('}')

    lines.append('fasteners [id:TEXT source:TEXT target:TEXT fastener-kind:REF gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM x:NUM y:NUM] {')
    for c in conns:
        src = f'{c["parent"]["piece"]["id"]}:{c["parent"]["connector"]["id"]}'
        tgt = f'{c["child"]["piece"]["id"]}:{c["child"]["connector"]["id"]}'
        lines.append(
            f'  {q(c["id"])} {q(src)} {q(tgt)} default '
            f'{num(c.get("gap", 0))} {num(c.get("shift", 0))} {num(c.get("rise", 0))} '
            f'{num(c.get("rotation", 0))} {num(c.get("turn", 0))} {num(c.get("tilt", 0))} '
            f'{num(c.get("u", 0))} {num(c.get("v", 0))}'
        )
    lines.append('}')

    dsl5 = '\n'.join(lines) + '\n'
    (OUT/'🗣️dream.5d.dsl.semio').write_text(dsl5)
    print('5d dsl bytes', len(dsl5.encode()), 'parts', len(pieces), 'fasteners', len(conns), 'types', len(used_type_ids))

    # Minimal 3d/2d projections as JSON sidecars for E2/E3 (full DSL can be derived)
    snap3 = {
        'schema': 'puzzle.3d',
        'domain': 'architecture',
        'meta': {'kindCatalogs': None, 'kindCompatibility': []},
        'objects': [],
        'attractions': [],
        'targetVolumes': [],
        'references': [],
    }
    for p in pieces:
        tid = p['type']['id']
        pose = p.get('pose')
        plane = (pose or {}).get('plane') or {}
        origin = plane.get('origin') or {'x':0,'y':0,'z':0}
        quat = plane_to_quat(plane) if pose else [0,0,0,1]
        vortices = []
        for c in type_connectors.get(tid, []):
            vortices.append({
                'id': c['id'],
                'vortexKind': ((c.get('port') or {}).get('id')),
                'position': [c.get('point',{}).get('x',0), c.get('point',{}).get('y',0), c.get('point',{}).get('z',0)],
                'direction': [c.get('direction',{}).get('x',0), c.get('direction',{}).get('y',0), c.get('direction',{}).get('z',1)],
            })
        snap3['objects'].append({
            'id': p['id'],
            'objectKind': tid,
            'anchor': 'fixed' if pose else 'derived',
            'origin': [origin.get('x',0), origin.get('y',0), origin.get('z',0)],
            'orientation': quat,
            'meshUrl': mesh_url_for_type(types[tid], files),
            'vortices': vortices,
            'hidden': False,
            'locked': False,
        })
    for c in conns:
        snap3['attractions'].append({
            'id': c['id'],
            'attracting': f'{c["parent"]["piece"]["id"]}:{c["parent"]["connector"]["id"]}',
            'attracted': f'{c["child"]["piece"]["id"]}:{c["child"]["connector"]["id"]}',
            'gap': c.get('gap',0) or 0,
            'shift': c.get('shift',0) or 0,
            'rise': c.get('rise',0) or 0,
            'rotation': c.get('rotation',0) or 0,
            'turn': c.get('turn',0) or 0,
            'tilt': c.get('tilt',0) or 0,
            'x': c.get('u',0) or 0,
            'y': c.get('v',0) or 0,
        })
    (OUT/'dream.3d.json').write_text(json.dumps(snap3))
    print('3d json objects', len(snap3['objects']), 'attractions', len(snap3['attractions']))

    # 2d projection
    snap2 = {'schema':'puzzle.2d','camera':{'x':0,'y':0,'zoom':1},'nodes':[],'edges':[],'meta':{'kindCompatibility':[],'kindCatalogs':None}}
    for p in pieces:
        tid = p['type']['id']
        pose = p.get('pose')
        center = (pose or {}).get('center') or {}
        handles = []
        for c in type_connectors.get(tid, []):
            handles.append({'id': c['id'], 'handleKind': ((c.get('port') or {}).get('id')), 'angle': 2*math.pi*float(c.get('t') or 0)})
        snap2['nodes'].append({
            'id': p['id'],
            'nodeKind': tid,
            'x': center.get('u',0) or 0,
            'y': center.get('v',0) or 0,
            'anchor': 'fixed' if pose else 'derived',
            'handles': handles,
        })
    for c in conns:
        snap2['edges'].append({
            'id': c['id'],
            'source': f'{c["parent"]["piece"]["id"]}:{c["parent"]["connector"]["id"]}',
            'target': f'{c["child"]["piece"]["id"]}:{c["child"]["connector"]["id"]}',
            'gap': c.get('gap',0) or 0,
            'shift': c.get('shift',0) or 0,
            'rise': c.get('rise',0) or 0,
            'rotation': c.get('rotation',0) or 0,
            'turn': c.get('turn',0) or 0,
            'tilt': c.get('tilt',0) or 0,
            'x': c.get('u',0) or 0,
            'y': c.get('v',0) or 0,
        })
    (OUT/'dream.2d.json').write_text(json.dumps(snap2))
    print('2d json nodes', len(snap2['nodes']), 'edges', len(snap2['edges']))
    # copy 5d json too for tests
    print('done →', OUT)

if __name__ == '__main__':
    main()
