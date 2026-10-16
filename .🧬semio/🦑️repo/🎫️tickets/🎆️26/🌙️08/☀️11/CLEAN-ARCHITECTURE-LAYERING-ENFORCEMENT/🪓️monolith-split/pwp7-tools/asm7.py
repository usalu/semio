import sys; sys.path.insert(0,'.')
from bundle7 import *
from src7 import load_pre, SNAPDIR
from use7 import leaves, leaf_stmt
EMO=[ch for ch in __import__('emoji7').pool(0)][120:]   # my slice of the unused-emoji pool
_emo_i=[0]
EMOJI_MAP=GEN+'/pwp7-emoji.json'
def _emoji_map():
    return json.load(open(EMOJI_MAP,encoding='utf-8')) if os.path.exists(EMOJI_MAP) else {}
def next_emoji(key=None):
    m=_emoji_map()
    if key is not None and key in m: return m[key]
    # an existing file keeps the emoji it already carries (snapshot first, then the live file)
    if key is not None:
        for base in (SNAPDIR+'/'+key, PLUG+key):
            if os.path.exists(base):
                first=open(base,encoding='utf-8').readline()
                mm=re.match(r'//!\s*(\S+)',first)
                if mm:
                    e=mm.group(1).replace('\ufe0f','')
                    m[key]=e; json.dump(m,open(EMOJI_MAP,'w',encoding='utf-8'),ensure_ascii=False,indent=1); return e
    taken=set(m.values())
    while EMO[_emo_i[0]] in taken: _emo_i[0]+=1
    e=EMO[_emo_i[0]]; _emo_i[0]+=1
    if key is not None:
        m[key]=e; json.dump(m,open(EMOJI_MAP,'w',encoding='utf-8'),ensure_ascii=False,indent=1)
    return e
def header_extras(rel):
    f=load_pre(rel)
    stmts=[]; cur=[]; attrs=[]
    for l in f['header']:
        s=l.strip()
        if not cur:
            if not s or s.startswith('//!'): continue
            if s.startswith('#[') and 'use ' not in s: attrs.append(l); continue
            if s.startswith('///') or s.startswith('//'): continue
        cur.append(l)
        if s.endswith(';'):
            stmt='\n'.join(attrs+cur); attrs=[]; cur=[]
            if re.sub(r'\s','',stmt)!='usesuper::*;': stmts.append(stmt)
    return stmts
def norm(stmt): return re.sub(r'\s+','',stmt)
def impl_parts(head_line):
    """returns (generics_text, where_text)"""
    h=head_line.strip()
    m=re.match(r'impl\s*<',h)
    M=mask(h)
    lt=h.index('<'); gt=match_angle(M,lt)
    gen=h[lt+1:gt]
    rest=h[gt+1:]
    mm=re.match(r'\s*VcsArtifactApp\s*<[^>]*>\s*(where[^{]*)?\{?\s*$',rest)
    assert mm, head_line
    return gen.strip(), (mm.group(1) or '').strip()
def new_head(gen, where, crate, view):
    name=(VIEW if view else CTX)[crate]
    lt="'v" if view else "'ctx"
    gen2=gen if 'X' not in gen else gen
    g="%s, %s, X: InstanceExtension"%(lt,gen)
    h="impl<%s> %s<%s, A, M, X>"%(g,name,lt)
    if where: h+=' '+where
    return h+' {'

DESC={
 ('render','🛠️tool-machine'):'The tool overlays: gesture, press and typing state of the continuous tools, the overlay refold and its retirement, and the test clock.',
 ('render','📜️history-view'):'The history projection: the command log backfill, the history view and its patches, the projection cache refresh, the history route commands and the reprojection status.',
 ('render','🧭️interaction'):'Interaction state: topology resolution, selection persistence, inline interaction verbs and the interaction action.',
 ('render','🏷️reserved-jobs'):'The inputs of the reserved action and media jobs.',
 ('dispatch','📤️emit'):'`dispatch_emit` and the emit transaction groups: how one app emit lands as commands, effects and history rows.',
 ('dispatch','🎟️producers'):'The reserved emit producers: lease admission, owner mounting and the producer admission fixtures.',
 ('dispatch','📣️host-events'):'Host events: the window facts delivered to the app as ordinary typed operations.',
 ('dispatch','⌨️typed-command'):'Typed command admission and start: roots capture, wire admission, import media and configuration commands.',
 ('dispatch','🤝️group-history'):'Group history actions over the composed children.',
 ('typed-ops','📬️publication'):'Typed operation publication: the mounted publication units, the latest-wins ladder and the stall witness.',
 ('typed-ops','♻️retirement'):'Typed operation retirement under the granted lifecycle step.',
 ('typed-ops','🪆️child-group'):'Private child group publication.',
 ('typed-ops','⌨️typing'):'Tool dispatch and typing commits: continuous-control arguments, typing runs and their publication.',
 ('commands','🧊️reserved-routes'):'Framework-reserved routes: the history, clipboard and shared host routes and their commit units.',
 ('commands','🚦️actions'):'Action and command dispatch, the addressed action preview and the agent lane preview.',
 ('commands','🗃️archive-merge'):'The whole-document archive merge.',
}
def to_free(m, T):
    txt=m['new']
    ded='\n'.join(l[4:] if l.startswith('    ') else l for l in txt.split('\n'))
    toks=[x.strip() for x in T.free.get(m['name'],'').replace('::<','').replace('>','').split(',') if x.strip()]
    need=[]
    if 'A' in toks: need.append('A: ArtifactApp')
    if 'M' in toks: need.append("M: SpaceMember + MemberFactory + 'static")
    sg=parse_sig(ded)
    a,b=sg['name_span']
    if sg['gen']:
        ga,gb=sg['gen']
        inner=ded[ga+1:gb-1]
        out=ded[:ga]+'<'+', '.join(need+[inner])+'>'+ded[gb:]
    else:
        out=ded[:b]+(('<'+', '.join(need)+'>') if need else '')+ded[b:]
    m2=re.search(r'^(?P<pre>(?:[ \t]*(?:///[^\n]*|//[^\n]*|#\[[^\n]*\])\n)*)(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<rest>(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\b)',out)
    assert m2, out[:200]
    if not m2.group('vis'):
        out=out[:m2.end('pre')]+'pub(crate) '+out[m2.end('pre'):]
    elif m2.group('vis').strip()=='pub(super)':
        out=out[:m2.start('vis')]+'pub(crate) '+out[m2.end('vis'):]
    return out
def group_impls(members):
    """ordered impl blocks: [(gen, where, view, [members])] ctx first then view, order of first appearance"""
    blocks=[]
    for m in members:
        if m['recv'] is None: continue
        gen,where=impl_parts(m['head'][-1])
        view=bool(m['view'])
        key=(re.sub(r'\s','',gen),where,view)
        for b in blocks:
            if b[0]==key: b[1].append(m); break
        else: blocks.append((key,[m],gen,where,view))
    blocks.sort(key=lambda b:b[0][2])
    return blocks
def impl_text(crate, members):
    out=[]
    for key,ms_,gen,where,view in group_impls(members):
        out.append(new_head(gen,where,crate,view))
        for k,m in enumerate(ms_):
            out.append(m['new'].rstrip('\n'))
            if k+1<len(ms_): out.append('')
        out.append('}')
        out.append('')
    return '\n'.join(out)
def word_in(name, masked):
    return re.search(r'(?<![\w])'+re.escape(name)+r'(?![\w])',masked) is not None
_srcmask={}
def src_mask(rel):
    if rel not in _srcmask: _srcmask[rel]=mask(__import__('finish7').pre_text(rel) if False else open((SNAPDIR+'/'+rel) if os.path.exists(SNAPDIR+'/'+rel) else PLUG+rel,encoding='utf-8').read())
    return _srcmask[rel]
def extras_for(body_text, srcs):
    """leaf imports of the source headers that the new text uses, plus the ones the source never named textually (trait or dead imports)"""
    bm=mask(body_text)
    seen={}; order=[]
    for rel in srcs:
        sm=None
        for s in header_extras(rel):
            ls=leaves(s)
            if ls is None: continue
            for l in ls:
                attrs,vis,p,alias,name=l
                if name=='*':
                    key=('*',p,attrs)
                else: key=(name,attrs)
                if key in seen: continue
                if name!='*':
                    # name used in the new text?
                    used=word_in(name,bm)
                    if not used:
                        # keep only when the source file never names it either (method-resolution traits and dead imports)
                        body_of_src=src_mask(rel)
                        occurrences=len(re.findall(r'(?<![\w])'+re.escape(name)+r'(?![\w])',body_of_src))
                        # the import statement itself accounts for one occurrence (or more for multi-line forms)
                        if occurrences>1: continue
                seen[key]=l; order.append(key)
    return [leaf_stmt(seen[k]) for k in order]
def domain_file(crate, sdir, members, emoji, T):
    srcs=sorted({m['file'] for m in members})
    emo_dir=sdir.split('\ufe0f')[0]+'\ufe0f'
    slug=sdir[len(emo_dir):]
    desc=DESC[(crate,sdir)]
    body=[]
    frees=[m for m in members if m['recv'] is None]
    for m in frees:
        body.append(to_free(m,T)); body.append('')
    body.append(impl_text(crate,members))
    body_txt='\n'.join(body)
    if all(s.endswith('🛠️tool-machine/🦀️.rs') for s in srcs): body_txt='//#region 🔖️Driver\n'+body_txt.rstrip('\n')+'\n//#endregion 🔖️Driver'
    extras=extras_for(body_txt,srcs)
    out=['//! %s\ufe0f Staged `%s` members of the `instance-%s` use cases, as methods of the borrowed bundle (WP7 of the instance family).'%(emoji,slug,crate),
         '//!','//! %s %s'%(emo_dir,desc),'use super::*;']+extras+['',body_txt]
    txt='\n'.join(out)
    return re.sub(r'\n{3,}','\n\n',txt).rstrip('\n')+'\n'

def inplace(W, rel, crate, extra=()):
    """regenerate one in-place file: members of `crate` stay (rewritten), others are dropped; `extra` members are appended"""
    f=load_pre(rel)
    R=f['R']
    mine=[m for m in W.ms if m['file']==rel]
    byk={(m['piece'],m['idx']):m for m in mine}
    out=list(f['header'])
    last_vcs=None
    for pi,p in enumerate(f['pieces']):
        out+=p.gap
        if p.kind!='vcsimpl':
            out+=R[p.s:p.e]; continue
        ms_=[byk[(pi,mi)] for mi in range(len(p.members)) if (pi,mi) in byk and byk[(pi,mi)]['crate']==crate]
        if not ms_: continue
        if crate=='app' or p.trait is not None:
            out+=[l for l in p.head_lines]
            for k,m in enumerate(ms_): out+=m['new'].split('\n')
            out.append(R[p.close_idx])
        else:
            attrs=[l for l in p.head_lines[:-1]]
            body=impl_text(crate,ms_)
            frees=[to_free(m,W.T) for m in ms_ if m['recv'] is None]
            # head attributes/docs/regions of the original impl precede the first block
            out+=attrs
            out+=body.rstrip('\n').split('\n')
            for ft in frees: out+=['']+ft.split('\n')
        last_vcs=len(out)
    if extra:
        out+=['']+impl_text(crate,list(extra)).rstrip('\n').split('\n')
        for ft in [to_free(m,W.T) for m in extra if m['recv'] is None]: out+=['']+ft.split('\n')
    out+=f['tail']
    txt='\n'.join(out)
    return re.sub(r'\n{3,}','\n\n',txt).rstrip('\n')+'\n'
