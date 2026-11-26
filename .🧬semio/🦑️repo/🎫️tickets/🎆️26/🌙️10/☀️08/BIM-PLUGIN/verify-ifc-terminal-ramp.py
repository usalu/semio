"""🏗️ Rebuild affected IFC writers against one captured Cargo graph and export both schema fixtures."""
from pathlib import Path
from functools import cache
import os,json,re,subprocess,sys
t=Path(__file__).resolve().parent
g=t/'🗑️generated'
target=sys.argv[1] if len(sys.argv)>1 else 'n'
r=json.loads(max((g/'compiler-retry').glob(f'{target}-ifc-*.success.json'),key=lambda p:p.stat().st_mtime).read_text())
a=r['arguments']; rust=r['compiler']; files={}
for i,arg in enumerate(a[:-1]):
 if arg=='-L' and a[i+1].startswith('dependency='):
  for file in Path(a[i+1][11:]).glob('lib*'): files[file.name]=file
out=g/('ifc-closed-shells-'+target);out.mkdir(exist_ok=True)
src=Path(r['cwd'])
manifests={}; roots={'semio_s_artifact_bim_model_modify','semio_s_artifact_bim_model_tests_5','semio_s_artifact_bim_model_components'}
for p in src.rglob('Cargo.toml'):
 m=re.search(r'^name = "([^"]+)"',p.read_text(),re.M)
 if m:manifests[m[1].replace('-','_')]=p
@cache
def deps(origin):
 return dict(re.findall(r'^\d+ (\w+)-(\w+) hash ',subprocess.check_output([rust,'-Zls=root',str(origin)],text=True),re.M))
def archive(name,ident,ext='.rmeta'):return files.get('lib'+name+'-'+ident+ext)
@cache
def build(name,origin):
 manifest=manifests[name]; ids=deps(origin); overrides={}
 cached=out/('lib'+name+'-ifcterminals.rmeta')
 if cached.with_suffix('.rlib').is_file() and (name != r['crate'] or cached.stat().st_mtime_ns >= (manifest.parent/'../../🔬️projection/🦀️.rs').stat().st_mtime_ns):return cached
 if name not in roots and not roots.intersection(ids):return origin
 fields=re.findall(r'^([\w-]+)(?:\.workspace)?\s*=',manifest.read_text().split('[dependencies]')[1].split('\n[')[0],re.M)
 args=[rust,'--crate-name',name,'--crate-type','rlib','--edition=2021','--emit=metadata,link','-Cmetadata=ifcterminals','-Cextra-filename=-ifcterminals','-Zno-parallel-backend','-Zthreads=1','-Copt-level=1','-Ccodegen-units=64','--out-dir',str(out),'-L','dependency='+str(out)]
 if target=='w':args+=['--target','wasm32-wasip2']
 for d in dict.fromkeys(f.parent for f in files.values()):args+=['-L','dependency='+str(d)]
 for field in fields:
  key=field.replace('-','_')
  key={'semio_framework_pack':'pack'}.get(key,key)
  if key not in ids:continue
  path=archive(key,ids[key])
  if path and key in manifests:path=build(key,path)
  if path and path.parent==out:
   for ext in ['.rlib','.rmeta']:args+=['--extern',key+'='+str(path.with_suffix(ext))]
  else:
   for ext in ['.rlib','.rmeta','.dylib','.so','.dll']:
    p=archive(key,ids[key],ext)
    if p:args+=['--extern',key+'='+str(p)]
 print('Rebuilding',name,flush=True)
 env=dict(os.environ,**r['cargo']);env.update(CARGO_MANIFEST_DIR=str(manifest.parent),CARGO_PKG_NAME=name.replace('_','-'))
 for attempt in range(5):
  result=subprocess.run([*args,str(manifest.parent/'🦀️.rs')],env=env,cwd=manifest.parent)
  if result.returncode not in (-9,-15) or attempt==4:
   result.check_returncode();break
 return out/('lib'+name+'-ifcterminals.rmeta')
name=r['crate']; origin=next(Path(a[a.index('--out-dir')+1]).glob('lib'+name+'-*.rmeta')); fresh=build(name,origin)
snapshot='semio_s_artifact_bim_model_snapshot'; snap=archive(snapshot,deps(origin)[snapshot])
source=out/'export.rs'
source.write_text((t/'exchange-fixtures.rs').read_text().replace('use bim_svg::standards::v1::subsets::any::io::export::svg::export_svg;','').replace('"svg" => (export_svg(&model).unwrap().into_bytes(), vec![]),',''))
cmd=[rust,'--crate-type','bin','--cfg','feature="measure-ifc4"','--edition=2024','-Zno-parallel-backend','-Zthreads=1',str(source),'-o',str(out/'export'),'-L','dependency='+str(out)]
for i,arg in enumerate(a[:-1]):
 if arg=='-L':cmd+=['-L',a[i+1]]
if sys.platform=='darwin' and target=='n':
 sysroot=Path(subprocess.check_output([rust,'--print','sysroot'],text=True).strip());linker=next(sysroot.glob('lib/rustlib/*/bin/gcc-ld/ld64.lld'));cmd+=['-Clink-arg=-fuse-ld='+str(linker),'-Clink-arg=-Wl,--threads=1,--read-workers=0']
for key,p in [('bim_ifc',fresh),('bim_snapshot',snap),('bim_schema',archive('semio_s_artifact_bim_model_schema',deps(origin)['semio_s_artifact_bim_model_schema']))]:
 for ext in ['.rmeta','.rlib']:cmd+=['--extern',key+'='+str(p.with_suffix(ext))]
subprocess.run(cmd,check=True)
