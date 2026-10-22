import os,re,sys
mine=['➗️mathematical','🗒️note','📕️norm','📖️playbook','🌀️procedural','🏭️process','🧩️puzzle','🖨️raster','💡️reasoning','📸️remodel','🎬️sequence','🎥️shooting','🪵️sourcing','🪐️space','🗄️stdio','🔱️trinity','🌿️vcs','🀄️wfc','✒️writer']
os.chdir('/Users/ueli/Documents/semio')
dry='--dry' in sys.argv
pat=re.compile(r'#\[mutations\(([^\]]*?)\)\]')
tot=0;files=0;skipped=[]
for p in mine:
    for r,ds,fs in os.walk('✏️s/🔌️plugins/'+p):
        ds[:]=[d for d in ds if d not in('node_modules','target')]
        for f in fs:
            if not f.endswith('.rs'): continue
            fp=os.path.join(r,f)
            try:s=open(fp,encoding='utf-8').read()
            except:continue
            if '#[mutations(' not in s: continue
            def rep(m):
                global tot
                a=m.group(1)
                if 'reference' in a: return m.group(0)
                sn=re.search(r'\bsnapshot\s*=\s*([^,]+?)\s*(?:,|$)',a); df=re.search(r'\bdiff\s*=\s*([^,]+?)\s*(?:,|$)',a)
                if not sn or not df: skipped.append(fp); return m.group(0)
                tot+=1
                return f"#[mutations({a}, reference = {sn.group(1)}, reference_diff = {df.group(1)})]"
            t=pat.sub(rep,s)
            if t!=s:
                files+=1
                if not dry: open(fp,'w',encoding='utf-8').write(t)
print('attrs',tot,'files',files,'skipped',skipped[:5])
