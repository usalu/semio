import os,re,shutil,sys,glob
root=os.getcwd()
C=glob.glob('.🧬semio/🦑*/🎫*/🎆*/🌙*10/☀*18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN')[0]
ws=os.path.join(C,'🗑️generated','bzl-rust-ws')
src=glob.glob('🧰*/🔨*/📐*')[0]
if os.path.exists(ws): shutil.rmtree(ws)
os.makedirs(ws)
crates={}
for name in os.listdir(src):
    if name.startswith('⚙') or name.startswith('➗') or name.startswith('🌙'):
        crates[name]=os.path.join(src,name)
assert len(crates)==3,crates
dst_root=os.path.join(ws,'geometry')
def ignore(d,names): return [n for n in names if n.startswith('🧪') or n=='node_modules' or n=='dist' or n.endswith('project.json')]
for n,p in crates.items(): shutil.copytree(p,os.path.join(dst_root,n),ignore=ignore)
rename={'semio-framework-geometry-primitives':'bzlrust-primitives','semio-framework-geometry-vector':'bzlrust-vector','semio-framework-geometry-bulge':'bzlrust-bulge'}
members=[]
for dp,dn,fn in os.walk(dst_root):
    if 'Cargo.toml' in fn:
        f=os.path.join(dp,'Cargo.toml'); t=open(f,encoding='utf8').read()
        t=re.sub(r'^workspace = ".*"\n','',t,flags=re.M)
        t=re.sub(r'^rust-version.workspace = true\n','',t,flags=re.M)
        t=re.sub(r'\n\[lints\]\nworkspace = true\n','\n',t)
        t=re.sub(r'\n\[dev-dependencies\].*\Z','\n',t,flags=re.S)
        t=re.sub(r'\n\[package\.metadata\.semio\.rust-source-boundary\].*?(?=\n\[)','',t,flags=re.S)
        own=re.search(r'^name = "([^"]+)"',t,re.M).group(1)
        t=re.sub(r'^name = "[^"]+"','name = "%s"'%rename[own],t,count=1,flags=re.M)
        t=re.sub(r'^(semio-framework-geometry-[a-z]+)\s*=\s*\{\s*path\s*=\s*',lambda m:'%s = { package = "%s", path = '%(m.group(1),rename[m.group(1)]),t,flags=re.M)
        open(f,'w',encoding='utf8').write(t)
        members.append(os.path.relpath(dp,ws))
wasm=os.path.join(ws,'wasm-demo'); os.makedirs(wasm)
open(os.path.join(wasm,'Cargo.toml'),'w').write('[package]\nname = "bzlrust-wasm"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\ncrate-type = ["cdylib"]\npath = "lib.rs"\n\n[dependencies]\n')
open(os.path.join(wasm,'lib.rs'),'w').write('#[unsafe(no_mangle)]\npub extern "C" fn bzlrust_add(left: u32, right: u32) -> u32 {\n    left.wrapping_add(right)\n}\n')
members.append('wasm-demo')
open(os.path.join(ws,'Cargo.toml'),'w',encoding='utf8').write('[workspace]\nresolver = "2"\nmembers = [\n'+''.join('    "%s",\n'%m for m in sorted(members))+']\n')
print(sorted(members))
