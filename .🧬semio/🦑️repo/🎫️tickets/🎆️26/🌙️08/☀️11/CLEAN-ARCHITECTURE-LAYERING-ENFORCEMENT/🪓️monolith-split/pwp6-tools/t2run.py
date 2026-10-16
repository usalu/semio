import sys; sys.path.insert(0, '.')
import rw6
from t2plan import *
from sig6 import parse_sig
from comps6 import generics_of

def build_comp_generics():
    ft = field_table()
    byc = collections.defaultdict(list)
    for n, d in ft.items():
        if d['cf']: byc[d['cf']].append(d['f']['decl'])
    return {cf: generics_of(byc[cf]) for cf in COMPONENTS}

def impl_head(w, cf):
    g = w.comp_generics[cf]
    bounds = {'A': 'A: ArtifactApp', 'M': "M: SpaceMember + MemberFactory + 'static"}
    return 'impl' + ('<' + ', '.join(bounds[x] for x in g) + '>' if g else '') + ' ' + w.comp_type[cf] + ('<' + ', '.join(g) + '>' if g else '') + ' {'

def param_decl(w, p, mut):
    if p == 'app': return 'app: ' + ('&mut A' if mut else '&A')
    if p == 'mounted_policy': return 'mounted_policy: &crate::MountedOwnerPolicyV1'
    g = w.comp_generics[p]
    return p + ': ' + ('&mut ' if mut else '&') + w.comp_type[p] + ('<' + ', '.join(g) + '>' if g else '')

FN_LEAD = re.compile(r'^(?P<lead>(?:[ \t]*(?:///[^\n]*|//[^\n]*|#\[[^\n]*\])\n)*)(?P<ind>[ \t]*)(?P<vis>pub(?:\([^)]*\))?[ \t]+)?(?P<qual>(?:const[ \t]+|async[ \t]+|unsafe[ \t]+|extern[ \t]+"[^"]*"[ \t]+)*)fn[ \t]')

def convert_member(text, info, w):
    """text: one member of an `impl VcsArtifactApp` block; returns the member as a method of the component impl"""
    sg = parse_sig(text)
    head, body = text[:sg['body_open']], text[sg['body_open']:]
    own_g = set(w.comp_generics[info.cf])
    mut = info.recv in ('mut', 'val') if info.recv else True
    decls = [param_decl(w, p, mut) for p in info.params]
    probe = mask(head + body); types_in_params = ' '.join(decls)
    need = [g for g in ('A', 'M') if (re.search(r'\b' + g + r'\b', probe) or re.search(r'\b' + g + r'\b', types_in_params)) and g not in own_g]
    bounds = {'A': 'A: ArtifactApp', 'M': "M: SpaceMember + MemberFactory + 'static"}
    edits = []
    if sg['recv']:
        b = sg['recv']['span'][1]
        if decls: edits.append((b, b, ''.join(', ' + d for d in decls)))
    elif decls:
        a = sg['paren'][0] + 1
        edits.append((a, a, ', '.join(decls) + (', ' if sg['params'] else '')))
    if need:
        if sg['gen']:
            ge = sg['gen'][1] - 1
            inner = head[sg['gen'][0] + 1:ge].strip()
            edits.append((ge, ge, (', ' if inner else '') + ', '.join(bounds[x] for x in need)))
        else:
            e = sg['name_span'][1]
            edits.append((e, e, '<' + ', '.join(bounds[x] for x in need) + '>'))
    # visibility
    lm = FN_LEAD.match(head)
    assert lm, head[:200]
    vis = (lm.group('vis') or '').strip()
    if vis == '' or vis == 'pub(super)':
        vs, ve = (lm.start('vis'), lm.end('vis')) if lm.group('vis') else (lm.end('ind'), lm.end('ind'))
        edits.append((vs, ve, 'pub(crate) '))
    for a, b, s in sorted(edits, key=lambda e: (-e[0], -e[1])): head = head[:a] + s + head[b:]
    return head, body, need


REC_PARAM = re.compile(r'\b(\w+)\s*:\s*&\s*(?:\'\w+\s+)?(?:mut\s+)?VcsArtifactApp\b')

def convert_all():
    model, methods, table, known = build_world()
    cg = build_comp_generics()
    w = rw6.World(methods, cg); w.known_composer = known
    new_texts = {}
    stats = collections.Counter()
    for rel, f in model.items():
        R = f['R']
        out = list(f['header'])
        for pi, p in enumerate(f['pieces']):
            gap = [g for g in p.gap if g.strip()]
            if p.kind != 'vcsimpl':
                seg = '\n'.join(R[p.s:p.e])
                names = sorted(set(REC_PARAM.findall(mask(seg))))
                for nm in names:
                    seg = rw6.rewrite(seg, w, 'composer', recv=nm); stats['free-item-rewrites'] += 1
                out += gap + seg.split('\n') + ['']
                continue
            conv = collections.OrderedDict(); plain = []
            for mi, m in enumerate(p.members):
                info, r = table[(rel, pi, mi)]
                txt = '\n'.join(R[m.a:m.e]).rstrip('\n')
                if info.cf:
                    head, body, need = convert_member(txt, info, w)
                    body = rw6.rewrite(body, w, 'component', own=info.cf, params=set(info.params), recv='self')
                    for sm in re.finditer(r'\bSelf\b', mask(body)):
                        w.flag('Self left in component body of %s: %s' % (info.name, body[sm.start():sm.start() + 40].replace(chr(10), ' ')))
                    conv.setdefault(info.cf, []).append(head + body)
                    stats['converted'] += 1
                else:
                    plain.append(rw6.rewrite(txt, w, 'composer', recv='self'))
                    stats['composer-level'] += 1
            attrs = []
            for hl in p.head_lines:
                if re.match(r'\s*(#\[|///|//)', hl): attrs.append(hl)
                else: break
            blocks = []
            if plain:
                blocks += list(p.head_lines) + ['\n'.join(x for x in plain).replace('\n\n\n', '\n\n')] if False else []
            seg_out = []
            if plain:
                seg_out += list(p.head_lines)
                for x in plain: seg_out += x.split('\n') + ['']
                while seg_out and not seg_out[-1].strip(): seg_out.pop()
                seg_out.append(R[p.close_idx]); seg_out.append('')
            for cf in COMPONENTS:
                if cf in conv:
                    seg_out += attrs + [impl_head(w, cf)]
                    for x in conv[cf]: seg_out += x.split('\n') + ['']
                    while seg_out and not seg_out[-1].strip(): seg_out.pop()
                    seg_out.append('}'); seg_out.append('')
            out += gap + seg_out
        while out and not out[-1].strip(): out.pop()
        out += [l for l in f['tail'] if l.strip()]
        new_texts[rel] = '\n'.join(out) + '\n'
    return new_texts, w, stats, model, table

if __name__ == '__main__':
    new_texts, w, stats, model, table = convert_all()
    print(stats, 'flags', len(w.flags))
    c = collections.Counter(w.flags)
    for k, v in c.most_common(80): print(v, k)
