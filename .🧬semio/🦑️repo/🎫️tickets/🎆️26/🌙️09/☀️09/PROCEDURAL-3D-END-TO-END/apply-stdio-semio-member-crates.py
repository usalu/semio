import os,re,shutil,subprocess,glob
R="/Users/ueli/Documents/semio"
A=R+"/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
os.chdir(R)
P="semio-s-artifact-stdio-semio-"
SUBS="🏅️standards/🔖️v1/🪆️subsets"
rows=sorted(["animation","audio","brep","cad","document","drawing","flow","graph","image","kit","mesh","model","object","presentation","table","text","value","video"])
dirs={}
for d in os.listdir(f"{A}/{SUBS}"):
    n=d.split("️",1)[-1] if "️" in d else d
    for r in rows:
        if d.endswith(r) and os.path.isdir(f"{A}/{SUBS}/{d}") and d!="✉️base": dirs[r]=d
assert sorted(dirs)==rows,dirs
deps_of={"audio":["audio-model","audio-mutations"],"table":["table-model","table-mutations"],"text":["text-model","text-mutations"],"value":["value-model","value-mutations"],"video":["video-model","video-mutations"]}
common=["semio-framework-job","semio-framework-value","semio-framework-artifact-reference","semio-os-kernel",P+"base"]
def camel(r):return r.capitalize()
def cname(r):return f"{P}{r}-member"
def rel(r):return f"{SUBS}/{dirs[r]}/👥️member"
oldgroups=["animation-brep","cad-drawing","flow-image","kit-model","object-table","text-video"]
# row text from existing groups
body={}
for g in oldgroups:
    s=open(f"{A}/{SUBS}/✉️base/👥️members/🧩️{g}/🦀️.rs",encoding='utf8').read()
    for m in re.finditer(r'^        (\w+)\("s\.stdio\.semio", "v1", "(\w+)", "stdio\.semio"\) => \(([^\n]*)\),\n',s,re.M): body[m.group(2)]=m.group(3)
assert sorted(body)==rows
# 1 remove groups
shutil.rmtree(f"{A}/{SUBS}/✉️base/👥️members")
# 2 create crates
for r in rows:
    d=f"{A}/{rel(r)}"; os.makedirs(d+"/📦️packages/🦀️rust",exist_ok=True)
    ty=f"Semio{camel(r)}Member"
    open(d+"/🦀️.rs","w",encoding='utf8').write(f'''//! 👥️ The typed member store of the `{r}` subset of the `stdio.semio` artifact. One subset is one crate so the typed stores of the whole artifact are never instantiated in a single compilation unit; see the master plan's "Architecture > The semio artifact" section.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;

dsl::space_members! {{
    native_producer:&::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    history_limits: ::semio_s_artifact_stdio_semio_base::SEMIO_MEMBER_HISTORY_LIMITS,
    maximum_history_allocation_bytes: ::semio_s_artifact_stdio_semio_base::SEMIO_MEMBER_HISTORY_ALLOCATION_BYTES,
    pub enum {ty}, {ty}Open {{
        {camel(r)}("s.stdio.semio", "v1", "{r}", "stdio.semio") => ({body[r]}),
    }}
}}
''')
    deps=sorted(set(common+[P+x for x in deps_of.get(r,[r])]))
    open(d+"/📦️packages/🦀️rust/Cargo.toml","w",encoding='utf8').write(f'''[package]
workspace = "../../../../../../.."
name = "{cname(r)}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Member store of the {r} subset of semio-s-artifact-stdio-semio"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
'''+"".join(f"{x} = {{ workspace = true }}\n" for x in deps))
# 3 workspace + artifact manifests
p=f"{A}/Cargo.toml"; s=open(p,encoding='utf8').read()
for g in oldgroups:
    s=s.replace(f'    "{SUBS}/✉️base/👥️members/🧩️{g}/📦️packages/🦀️rust",\n','')
    s=s.replace(f'[workspace.dependencies.{P}members-{g}]\npath = "{SUBS}/✉️base/👥️members/🧩️{g}/📦️packages/🦀️rust"\n\n','')
a=f'    "{SUBS}/✉️base/📚️examples/📦️packages/🦀️rust",\n'; assert s.count(a)==1
s=s.replace(a,"".join(f'    "{rel(r)}/📦️packages/🦀️rust",\n' for r in rows)+a)
b='[workspace.dependencies.semio-s-artifact-stdio-semio-animation]\n'; assert s.count(b)==1
s=s.replace(b,"".join(f'[workspace.dependencies.{cname(r)}]\npath = "{rel(r)}/📦️packages/🦀️rust"\n\n' for r in rows)+b)
assert "members-" not in s
open(p,"w",encoding='utf8').write(s)
p=f"{A}/📦️packages/🦀️rust/Cargo.toml"; s=open(p,encoding='utf8').read()
s=re.sub(r'semio-s-artifact-stdio-semio-members-[a-z-]+ = \{ workspace = true \}\n','',s)
a='semio-s-artifact-stdio-semio-kit = { workspace = true }\n'; assert s.count(a)==1
s=s.replace(a,a+"".join(f"{cname(r)} = {{ workspace = true }}\n" for r in rows)); open(p,"w",encoding='utf8').write(s)
# 4 lib
p=f"{A}/🦀️.rs"; s=open(p,encoding='utf8').read()
i=s.index("// 🧬️ Eighteen composable subsets"); j=s.index("/// 🏭️ Mints a typed Semio child")
mod=lambda r:cname(r).replace("-","_")
s=s[:i]+'''// 🧬️ Eighteen composable subsets, bound to their exact kind, standard, subset and schema; every subset crate `👥️member` instantiates only its own typed store, this enum routes by dialect.
dsl::space_member_groups! {
    pub enum SemioMembers, SemioMembersOpen {
'''+"".join(f"        {camel(r)}({mod(r)}::Semio{camel(r)}Member),\n" for r in rows)+'''    }
}

'''+s[j:]
open(p,"w",encoding='utf8').write(s)
# 7 test
p=f"{A}/{SUBS}/🌊️flow/🚪️io/💾️binary/📸️snapshot/🧪️tests/💾️binary/🦀️.rs"; s=open(p,encoding='utf8').read()
a="SemioMembers::FlowImage(group) if matches!(group.as_ref(), semio_s_artifact_stdio_semio_members_flow_image::SemioMembersFlowImage::Flow(_))"
assert a in s
s=s.replace(a,"SemioMembers::Flow(group) if matches!(group.as_ref(), semio_s_artifact_stdio_semio_flow_member::SemioFlowMember::Flow(_))"); open(p,"w",encoding='utf8').write(s)
# 8 TS
p=R+"/🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🏭️member-factory-identity/🟦️.ts"; s=open(p,encoding='utf8').read()
i=s.index("    const membersRoot"); j=s.index("    assert.equal(declarations.length, 18);")
s=s[:i]+'''    const subsetsRoot = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets");
    const semioTable = readdirSync(subsetsRoot).filter(subset => existsSync(join(subsetsRoot, subset, "👥️member", "🦀️.rs"))).map(subset => readFileSync(join(subsetsRoot, subset, "👥️member", "🦀️.rs"), "utf8")).join("\\n");
    const declarations = [...semioTable.matchAll(/\\w+\\("([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"\\) =>/g)].map(match => match.slice(1)).sort((left, right) => (left[2]! < right[2]! ? -1 : left[2]! > right[2]! ? 1 : 0));
'''+s[j:]
s=s.replace('import { readFileSync, readdirSync } from "node:fs";','import { existsSync, readFileSync, readdirSync } from "node:fs";'); open(p,"w",encoding='utf8').write(s)
# 5 build-owners
p=R+"/📋️build-owners.json"; s=open(p,encoding='utf8').read()
for g in oldgroups:
    root=f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/{SUBS}/✉️base/👥️members/🧩️{g}/📦️packages/🦀️rust"
    blk=f'    {{\n      "name": "{P}members-{g}",\n      "root": "{root}",\n      "manifest": "{root}/Cargo.toml",\n      "packages": []\n    }},\n'
    assert blk in s; s=s.replace(blk,"")
anchor_names=re.findall(r'^      "name": "(semio-s-artifact-stdio-semio-[^"]+)",$',s,re.M)
for r in rows:
    n=cname(r); root=f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/{rel(r)}/📦️packages/🦀️rust"
    blk=f'    {{\n      "name": "{n}",\n      "root": "{root}",\n      "manifest": "{root}/Cargo.toml",\n      "packages": []\n    }},\n'
    nxt=next((x for x in anchor_names if x>n),None); assert nxt
    a=f'    {{\n      "name": "{nxt}",\n'; assert s.count(a)==1
    s=s.replace(a,blk+a); anchor_names.append(n); anchor_names.sort()
open(p,"w",encoding='utf8').write(s)
import json; json.loads(s)
# 6 locks
files=[f for f in subprocess.run(["git","ls-files","-z"],capture_output=True,text=True).stdout.split("\0") if f.split("/")[-1]=="Cargo.lock" and "🎫️tickets" not in f]
n=0
for f in files:
    s=open(f,encoding='utf8').read()
    if '\nname = "semio-s-artifact-stdio-semio"\n' not in s: continue
    head,*blocks=s.split("\n[[package]]\n")
    out=[]
    for b in blocks:
        nm=re.match(r'name = "([^"]+)"',b).group(1)
        if nm.startswith(P+"members-"): continue
        if nm=="semio-s-artifact-stdio-semio":
            m=re.search(r'dependencies = \[\n((?: "[^"]+",?\n)*)\]',b)
            ds=[d for d in re.findall(r'"([^"]+)"',m.group(1)) if not d.startswith(P+"members-")]
            ds=sorted(set(ds+[cname(r) for r in rows]))
            b=b[:m.start(1)]+"".join(f' "{d}",\n' for d in ds)+b[m.end(1):]
        out.append((nm,b))
    names={x for x,_ in out}
    for r in rows:
        ds=sorted(set(common+[P+x for x in deps_of.get(r,[r])]))
        assert all(d in names for d in ds),(f,ds)
        blk=f'name = "{cname(r)}"\nversion = "0.1.0"\ndependencies = [\n'+"".join(f' "{d}",\n' for d in ds)+']\n'
        k=0
        while k<len(out) and out[k][0]<cname(r): k+=1
        out.insert(k,(cname(r),blk))
    open(f,"w",encoding='utf8').write(head+"".join("\n[[package]]\n"+b for _,b in out)); n+=1
print("locks",n)
