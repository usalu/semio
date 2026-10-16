import sys,os,subprocess,difflib,glob
def fmt(path):
    r=subprocess.run(['rustfmt','+nightly-2026-07-20','--edition','2021','--config-path','/Users/ueli/Documents/semio/rustfmt.toml','--config','skip_children=true','--emit','stdout',path],capture_output=True,text=True)
    if r.returncode!=0:raise Exception(r.stderr[:500])
    out=r.stdout
    # rustfmt --emit stdout prefixes the file name
    lines=out.split('\n')
    if lines and lines[0].endswith(':') and path in lines[0]:
        lines=lines[2:] if lines[1]=='' else lines[1:]
    return '\n'.join(lines)
def run(stage):
    changed=0
    for p in sorted(glob.glob(stage+'/*/🦀️.rs')):
        old=open(p,encoding='utf-8').read()
        try:new=fmt(p)
        except Exception as e:
            print('fmt failed',p,str(e)[:200]);continue
        a=old.split('\n');b=new.split('\n')
        out=[]
        sm=difflib.SequenceMatcher(None,a,b,autojunk=False)
        for tag,i1,i2,j1,j2 in sm.get_opcodes():
            if tag=='equal':out.extend(a[i1:i2])
            elif tag=='replace' and any('pub(super)' in l for l in a[i1:i2]) and not any('//' in l for l in a[i1:i2]):
                out.extend(b[j1:j2]);changed+=1
            else:out.extend(a[i1:i2])
        res='\n'.join(out)
        if res!=old:open(p,'w',encoding='utf-8').write(res)
    return changed
if __name__=='__main__':print(sys.argv[1],'hunks reformatted',run(sys.argv[1]))
