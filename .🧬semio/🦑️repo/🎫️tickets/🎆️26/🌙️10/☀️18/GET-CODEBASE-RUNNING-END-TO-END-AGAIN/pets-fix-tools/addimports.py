import re,sys,collections,os,subprocess
G=sys.argv[1]; write=len(sys.argv)>2 and sys.argv[2]=='--write'
root='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets'
mods=os.path.join(root,'🔨️modules')
files=[f for f in subprocess.check_output(['git','ls-files','*.rs'],cwd=root,text=True).split('\n') if f and '🧪️' not in f]
def modpath(f):
    d=f.split('/')
    parts=[p for p in d]
    s=f
    if '🚶️locomotion/' in f and f.count('/')>=2: return 'crate::locomotion'
    table=[('🧗️climbing','semio_framework_pets_traversal::climbing'),('🪢️swing','semio_framework_pets_traversal::swing'),
     ('🚧️clearance','semio_framework_pets_clearance'),('💗️feeling','semio_framework_pets_character::feeling'),
     ('🧠️behavior','semio_framework_pets_character::behavior'),
     ('🏞️terrain','semio_framework_pets_kinematics::terrain'),('📐️trigonometry','semio_framework_pets_kinematics::trigonometry'),
     ('🎲️randomness','semio_framework_pets_kinematics::randomness'),('🎞️animation','semio_framework_pets_kinematics::animation'),('🦴️rig','semio_framework_pets_kinematics::rig'),
     ('👆️gesture','semio_framework_pets_interaction::gesture'),('✨️effects','semio_framework_pets_interaction::effects'),
     ('🏷️scalars','semio_framework_pets_contract::schema'),('🎬️stage','semio_framework_pets_contract::schema'),
     ('📝️draft','crate::draft'),('📏️spacing','crate::spacing'),('🗓️schedule','crate::schedule'),('👀️attention','crate::attention'),
     ('✋️hand','crate::locomotion'),('🧬️schema','semio_framework_pets_contract::schema'),('💞️sociability','crate::sociability'),('🎯️choice','crate::choice'),('👥️population','crate::population'),('🕰️clock','crate::clock'),('🎥️projection','crate::projection'),('🎪️stage','crate::stage')]
    for k,v in table:
        if k in s: return v
    if '🪄️mischief' in s and '🚶️' not in s: return 'semio_framework_pets_interaction::mischief'
    return None
pat=re.compile(r'^\s*(pub(?:\([^)]*\))?\s+)?(?:const\s+fn|async\s+fn|unsafe\s+fn|fn|const|static|struct|enum|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)')
idx=collections.defaultdict(list)
for f in files:
    mp=modpath(f)
    if mp is None: continue
    for l in open(os.path.join(root,f),encoding='utf8'):
        m=pat.match(l)
        if m and (m.group(1) or '').strip() in ('pub','pub(crate)'):
            idx[m.group(2)].append(mp)
STAGING=('💞️sociability','🎯️choice','👥️population','🕰️clock','🎥️projection','🎪️stage')
DRAFTMODS=('crate::draft','crate::spacing','crate::schedule','crate::attention','crate::locomotion')
def in_staging(f): return '🧪️' in f or any(m in f for m in STAGING)
def retarget(f,c):
    if in_staging(f):
        if any(c==m or c.startswith(m+'::') for m in DRAFTMODS): return c.replace('crate::','semio_framework_pets_staging_draft::',1)
        return c
    return c
prefer={'circled':'semio_framework_pets_interaction::gesture'}
need=collections.defaultdict(lambda:collections.defaultdict(set))
for l in open(G+'/errs.txt'):
    m=re.match(r'(.*?):\d+:\d+: error\[(E\d+)\]: cannot find ([^`]*?)`([^`]+)`',l)
    if not m: continue
    f,_,kind,name=m.groups()
    if f=='🦀️.rs': f='📝️draft/🦀️.rs'
    cands=sorted(set(idx.get(name,[])))
    if name in prefer: cands=[prefer[name]]
    if len(cands)!=1: print('AMBIG/NONE',f,name,cands); continue
    need[f][retarget(f,cands[0])].add(name)
for f,byp in need.items():
    full=os.path.join(mods,f)
    own=modpath(f)
    src=open(full,encoding='utf8').read().split('\n')
    # insertion line: after last top-level use before first //#region
    last=-1
    for i,l in enumerate(src):
        if l.startswith('//#region'): break
        if re.match(r'^(pub(\([^)]*\))? )?use ',l) or (last>=0 and l.startswith('    ') and not l.strip().startswith('//')): last=i if re.match(r'^(pub(\([^)]*\))? )?use ',l) or src[i-1].rstrip().endswith(',') or src[i-1].rstrip().endswith('{') else last
    # robust: find first //#region or first item; use last 'use' terminator ';'
    last=-1
    for i,l in enumerate(src):
        if l.startswith('//#region') or re.match(r'^(pub(\([^)]*\))? )?(fn|const|static|struct|enum|mod|type|trait|impl)\b|^#\[|^///',l): break
        if l.rstrip().endswith(';') and re.match(r'^(pub(\([^)]*\))? )?use ',l): last=i
        elif l.rstrip().endswith('};'): last=i
    existing='\n'.join(src[:last+1] if last>=0 else src[:20])
    lines=[]
    for p,names in sorted(byp.items()):
        ns=[n for n in sorted(names) if not re.search(r'\b'+n+r'\b',existing.split('//!')[-1])]
        if not ns: continue
        lines.append(f'use {p}::{{{", ".join(ns)}}};' if len(ns)>1 else f'use {p}::{ns[0]};')
    print(f, '->', len(lines))
    for ln in lines: print('   ',ln)
    if write and lines:
        if last<0:
            # after //! doc block
            j=0
            while j<len(src) and src[j].startswith('//!'): j+=1
            last=j-1
            src[last+1:last+1]=['']
            last+=1
        src[last+1:last+1]=lines
        open(full,'w',encoding='utf8').write('\n'.join(src))
