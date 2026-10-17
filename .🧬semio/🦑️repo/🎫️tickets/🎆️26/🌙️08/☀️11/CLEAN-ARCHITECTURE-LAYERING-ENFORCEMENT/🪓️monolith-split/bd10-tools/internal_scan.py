import json,os,re,subprocess,sys
R='/Users/ueli/Documents/semio/'
F=R+'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/'
crates=json.load(open(F+'bd10-tools/crates.json'))['crates']
only=sys.argv[1].split(',') if len(sys.argv)>1 else list(crates)
KEEP=re.compile(r'\bcrate::|\bsuper::|\bself::|in the crate root|in `crate`|in `super`|in `self`|in this scope')
CODES={'E0255','E0428','E0583','E0603','E0616','E0624','E0412','E0425','E0422','E0423','E0432','E0433','E0599','E0609','E0560','E0061'}
for name in only:
    root=R+crates[name]['root']
    for test in (False,True):
        cmd=['rustc','+nightly-2026-07-20','--edition','2021','--crate-type','lib','--crate-name','x_'+name,'--emit=metadata','-o','/dev/null','--error-format=short','--cap-lints','allow']
        if test: cmd.append('--test')
        p=subprocess.run(['nice','-n','10']+cmd+[root],capture_output=True,text=True,cwd=os.path.dirname(root))
        out=[l for l in p.stderr.split('\n') if ': error' in l]
        keep=[]
        for l in out:
            m=re.search(r'error\[(E\d+)\]',l)
            code=m.group(1) if m else ''
            if code in ('E0463',) : continue
            msg=l
            if re.search(r"(unresolved import|cannot find module or crate|failed to resolve: use of unresolved module or unlinked crate|could not find) `?(semio_\w+|base64_codec|machine|serde\w*|dsl|protocol|store|schema|pack|png|image|std)`",msg): continue
            if KEEP.search(msg) or code in ('E0252','E0254','E0255','E0428','E0124','E0201','E0583'):
                keep.append(l)
        print(f'== {name} {"test" if test else "lib"}: {len(out)} errors, {len(keep)} internal')
        for l in keep[:40]: print('  ',l[:300].replace(R,''))
