import subprocess,os,re,collections,json
os.chdir("/Users/ueli/Documents/semio")
out=subprocess.run(r"""/usr/bin/grep -rn "ArtifactCommandWork<[^>]*>\{1,2\} for [A-Za-z0-9_]*\|impl ArtifactCommandWork<.*> for" ✏️s --include=🦀️.rs | /usr/bin/grep -v "/🧪️tests/\|BoundedArtifact" """,shell=True,capture_output=True,text=True).stdout.split("\n")
files=collections.OrderedDict()
for l in out:
    m=re.match(r"^(.*?\.rs):(\d+):.* for ([A-Za-z0-9_<>:]+)",l)
    if not m: continue
    f,_,t=m.groups()
    if "<" in t or "::" in t: continue
    files.setdefault(f,[])
    if t not in files[f]: files[f].append(t)
def pkg(f):
    d=os.path.dirname(f)
    while d and d!="✏️s":
        c=os.path.join(d,"📦️packages/🦀️rust/Cargo.toml")
        if os.path.exists(c):
            for line in open(c):
                m=re.match(r'name = "(.*)"',line)
                if m: return m.group(1),c
        d=os.path.dirname(d)
    return None,None
plan=[];mods=0
for f,ts in files.items():
    p,c=pkg(f)
    s=open(f).read()
    if "mod work_frame_census" in s: 
        plan.append((p,c,f,ts,"exists")); continue
    body="\n".join(f"        semio_plugin::retained_command::assert_work_frame_fits::<{t}>();" for t in ts)
    add=f"\n#[cfg(test)]\nmod work_frame_census {{\n    use super::*;\n    /// 📏️ Every command work of this file keeps its inline frame at or below the close release bound, so a large owner is boxed or paged and never carried inline.\n    #[test]\n    fn command_work_frames_stay_below_the_close_release_bound() {{\n{body}\n    }}\n}}\n"
    open(f,'w').write(s.rstrip("\n")+"\n"+add); mods+=1
    plan.append((p,c,f,ts,"added"))
json.dump(plan,open(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/runtime/sweep/plan.json","w"),ensure_ascii=False,indent=0)
pk=collections.Counter(p for p,_,_,_,_ in plan)
print(len(files),"files",mods,"added",len(pk),"packages",sum(1 for p,_,_,_,_ in plan if p is None),"nopkg")
