import subprocess,tomllib,os,sys,json
root='/Users/ueli/Documents/semio'
out=subprocess.run(['rg','-l','semio-framework-plugin-decl-views','-g','Cargo.toml',root],capture_output=True,text=True).stdout.split()
out=[o for o in out if 'node_modules' not in o and '/.cache/' not in o]
jobs=[]
for m in out:
    d=os.path.dirname(m)
    try: t=tomllib.load(open(m,'rb'))
    except Exception as e: print('ERR',m,e); continue
    entries=[]
    lib=t.get('lib',{}).get('path') or ('src/lib.rs' if os.path.exists(d+'/src/lib.rs') else None)
    if lib: entries.append(os.path.normpath(os.path.join(d,lib)))
    for k in ('test','bin','example','bench'):
        for x in t.get(k,[]):
            if 'path' in x: entries.append(os.path.normpath(os.path.join(d,x['path'])))
    jobs.append({'manifest':m,'entries':entries})
json.dump(jobs,open(sys.argv[1],'w'))
print(len(jobs))
