"""🌱 Crate root generator: pruned import header from candidate parents + mounts with re-exports."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify,parse_use,gather_text
from usage import toks,leaves
def read_use_items(path):
    """list of dict(attrs, vis, names[(alias,full)], anon, globs, raw)"""
    t=open(path,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);out=[]
    for it in top_items(L,I):
        classify(it)
        if it['kind']!='use':continue
        txt=gather_text(L,it)
        nm,anon,globs=parse_use(txt)
        vis='pub' if re.match(r'^pub\s',it['text']) and not it['text'].startswith('pub(') else ('crate' if it['text'].startswith('pub(') else '')
        out.append(dict(attrs=it['attrs'],vis=vis,names=nm,anon=anon,globs=globs,raw=txt.strip()))
    return out
def fmt_leaf(full,name):
    last=full.split('::')[-1]
    return full if last==name else f"{full} as {name}"
def emit(items,used_tokens,rewrite,keep_globs=()):
    """items: use dicts. emit grouped `use` lines for used leaves; keep globs listed in keep_globs (after rewrite)."""
    groups=collections.OrderedDict()
    lines=[]
    for u in items:
        cfg=''.join(a+'\n' for a in u['attrs']) if u['attrs'] else ''
        leaves_=[]
        for n,full in u['names']:
            if n in used_tokens:leaves_.append((n,rewrite(full)))
        for n,full in u['anon']:
            leaves_.append(('_',rewrite(full)))
        for g in u['globs']:
            rg=rewrite(g)
            if rg in keep_globs:leaves_.append(('*',rg))
        for n,full in leaves_:
            key=(cfg,u['vis'],'::'.join(full.split('::')[:-1]) if n!='*' else full)
            groups.setdefault(key,[]).append((n,full))
    for (cfg,vis,prefix),ls in groups.items():
        v={'':'', 'pub':'pub ','crate':'pub(crate) '}[vis]
        if ls[0][0]=='*':
            lines.append(f"{cfg}{v}use {prefix}::*;")
        elif len(ls)==1:
            n,full=ls[0];lines.append(f"{cfg}{v}use {fmt_leaf(full,n)};")
        else:
            inner=", ".join(fmt_leaf(full,n).split('::')[-1] if not full.endswith(' as '+n) else fmt_leaf(full,n).split('::')[-1] for n,full in ls)
            lines.append(f"{cfg}{v}use {prefix}::{{{inner}}};")
    return lines
