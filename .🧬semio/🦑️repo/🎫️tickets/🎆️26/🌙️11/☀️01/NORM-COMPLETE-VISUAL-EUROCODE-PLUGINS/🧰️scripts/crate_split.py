import os,re,sys
def relp(a,b):return os.path.relpath(a,b)
def mk_crate(artifact,crate_dir,name,desc,lib_text,base_toml,extra_deps=(),dev=True,drop_deps=()):
    pkg=os.path.join(crate_dir,'📦️packages','🦀️rust');os.makedirs(pkg,exist_ok=True)
    src=open(base_toml).read();bpkg=os.path.dirname(base_toml)
    def fix(m):
        p=m.group(2);ab=os.path.normpath(os.path.join(bpkg,p));return m.group(1)+relp(ab,pkg)+'"'
    deps=re.search(r'\[dependencies\]\n(.*?)(?:\n\[|\Z)',src,re.S).group(1)
    deps=re.sub(r'(path = ")([^"]+)"',fix,deps).strip().split('\n')
    deps=[d for d in deps if d and not any(d.startswith(x+' ') for x in drop_deps)]
    deps+=[f'{n} = {{ workspace = true }}' for n in extra_deps]
    deps=sorted(set(deps))
    t=f'''[package]
workspace = "{relp(artifact,pkg)}"
name = "{name}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "{desc}"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
'''+'\n'.join(deps)+'\n'
    if dev:
        m=re.search(r'\[dev-dependencies\]\n.*',src,re.S)
        if m:t+='\n'+m.group(0).strip()+'\n'
    open(os.path.join(pkg,'Cargo.toml'),'w').write(t)
    open(os.path.join(crate_dir,'🦀️.rs'),'w').write(lib_text)
    return pkg
def register(artifact,pkg,name):
    f=os.path.join(artifact,'Cargo.toml');s=open(f).read()
    rel=relp(pkg,artifact)
    if f'"{rel}"' not in s:
        s=s.replace('members = [\n',f'members = [\n    "{rel}",\n',1)
    if f'[workspace.dependencies.{name}]' not in s:
        s=s.replace('[workspace.lints.rust]',f'[workspace.dependencies.{name}]\npath = "{rel}"\n\n[workspace.lints.rust]',1)
    open(f,'w').write(s)
def add_dep(toml,name):
    s=open(toml).read()
    if name+' =' not in s:
        s=s.replace('[dependencies]\n',f'[dependencies]\n{name} = {{ workspace = true }}\n',1)
        open(toml,'w').write(s)
