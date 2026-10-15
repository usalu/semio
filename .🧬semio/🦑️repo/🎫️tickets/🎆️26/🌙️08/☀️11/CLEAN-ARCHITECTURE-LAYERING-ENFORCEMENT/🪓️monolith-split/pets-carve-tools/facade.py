import os,re,glob,sys
from plan import *

PUBLIC={m for m in OWNER if not (OWNER[m]=='staging' and m!='stage')}

def adapter_names():
    names=set()
    for f in glob.glob(PETS+'/🧪️tests/*/🦀️.rs'):
        s=open(f).read()
        for m in re.finditer(r'use pets::\{(.*?)\};',s,re.S):
            for n in re.split(r'[,\s]+',m.group(1)):
                if n and n!='self': names.add(n)
        for m in re.finditer(r'use pets::(\w+);',s):
            if m.group(1)!='serde_json': names.add(m.group(1))
    return names

def definitions():
    d={}
    for m in PUBLIC:
        s=open(src(m)).read()
        for mm in re.finditer(r'^pub (?:const |unsafe )?(?:fn|const|static|struct|enum|type|trait)\s+(\w+)',s,re.M):
            d.setdefault(mm.group(1),[]).append((m,'def'))
        for mm in re.finditer(r'^pub use [\w:]+?(?:::)?(\w+) as (\w+);',s,re.M):
            d.setdefault(mm.group(2),[]).append((m,'alias'))
        for mm in re.finditer(r'^pub use crate::\w+::(\w+);',s,re.M):
            d.setdefault(mm.group(1),[]).append((m,'reexport'))
        for mm in re.finditer(r'^pub use [\w:]+::(\w+);',s,re.M):
            d.setdefault(mm.group(1),[]).append((m,'reexport'))
    return d

def resolve():
    names=adapter_names(); defs=definitions()
    out={}; problems=[]
    for n in sorted(names):
        cand=defs.get(n,[])
        cand=list(dict.fromkeys(cand))
        real=[c for c in cand if c[1] in('def','alias')]
        if not real:
            exp=[c for c in cand if c[1]=='reexport']
            if exp: real=exp
        if len(real)!=1: problems.append((n,cand)); continue
        out[n]=real[0][0]
    return out,problems

def render(out):
    by={}
    for n,m in out.items(): by.setdefault(m,[]).append(n)
    lines=[]
    order=['schema','validation','trigonometry','randomness','rig','animation','terrain','swing','climbing','gesture','effects','mischief','clearance','behavior','feeling','stage']
    for m in order:
        if m not in by: continue
        c=OWNER[m]
        path=f'{ident(c)}::{m}' if m!='clearance' else ident(c)
        lines.append(f'pub use {path}::{{{", ".join(sorted(by[m]))}}};')
    return lines

if __name__=='__main__':
    out,problems=resolve()
    print(len(out),'resolved; problems:',problems)
    for l in render(out): print(l[:160])

HEAD='''//! 🐾️ Pets product façade — render-independent deterministic pets and their pure core, as the one crate the subject hosts of the language-agnostic cases link (`🧪️tests/<case>/🦀️.rs`, lib `pets`). The core itself is carved into producer crates along its domain seams, each below the next:
//!
//! - `semio-framework-pets-contract` — the contract types (menageries of rigged species, the stage, its events and frames) and their validation;
//! - `semio-framework-pets-kinematics` — trigonometry in turns, counter-based randomness, the forward kinematics of a rig, animation (clips, springs, blinks) and terrain (perches, walls, strides, falls, hops);
//! - `semio-framework-pets-clearance` — the bodies and claimed corridors that keep pets apart, and the proof that no two overlap;
//! - `semio-framework-pets-traversal` — the swing of ropes and parachutes, and climbing (walls, ladders, grappling ropes, routes);
//! - `semio-framework-pets-interaction` — the press, the heat and the gestures of the learner's hand, the particles of the effects and the mischief with the page;
//! - `semio-framework-pets-character` — behaviour (weights, dwells, encounters, needs, casts) and feeling (moods, states, tricks, the chemistry between species);
//! - `semio-framework-pets-staging` — the stage that folds events into frames.
//!
//! Every function is pure, shares its vocabulary with the TypeScript core (`@semio-tech/pets`, snake_case here) and evaluates the same expressions in the same order, so both cores yield the same bits. This crate holds no code of its own: it re-exports exactly the names its subject hosts call.
//!
//! @see README.md — the domain model
//! @see 🧬️schema/🔣️.json — the normative contract

'''
TAIL='''
/// 🧪️ The JSON codec the Protocol v2 subject hosts decode the shared vectors with.
pub use serde_json;
'''
def write_root():
    out,problems=resolve()
    assert not problems,problems
    body='\n'.join(render(out))+'\n'
    open(PETS+'/🦀️.rs','w').write(HEAD+body+TAIL)
    print(len(out),'names')
if len(sys.argv)>1 and sys.argv[1]=='write': write_root()
