import re,sys,json
# usage: arms.py file enum snapshot leafpath
path, enum, snap, leaf = sys.argv[1:5]
s=open(path,encoding='utf-8').read()
lines=s.split('\n')
out=[]; log=[]
mk=f"<{leaf} as protocol::MutationKind<{snap}, {enum}>>"
i=0
for idx,line in enumerate(lines):
    out.append(line)
    m=re.match(r'^(\s*)(\()?'+enum+r'::SetSnapshot\((.*?)\)(, _\))? => (.*)$', line)
    if not m or 'PatchSnapshot' in line: continue
    ind=m.group(1); rhs=m.group(5)
    if m.group(2):  # tuple arm, manual
        log.append(f"MANUAL tuple {idx+1}: {line.strip()[:120]}"); continue
    new=None
    if re.search(r'diff_set_snapshot\(|::between\(base|MutationOutcome::new\(diff_set_snapshot|blocked_snapshot_violation', rhs):
        new=f"{ind}{enum}::PatchSnapshot(patch) => return {mk}::diff(patch, base),"
    elif re.search(r'SetSnapshot\(set_snapshot::SetSnapshot \{ snapshot: (Box::new\()?base\.clone\(\)', rhs) or re.search(r'snapshot: base\.c', rhs):
        new=f"{ind}{enum}::PatchSnapshot(patch) => return {mk}::inverse(patch, base),"
    elif re.fullmatch(r'"set-snapshot",', rhs.strip()):
        new=f'{ind}{enum}::PatchSnapshot(_) => "patch-snapshot",'
    elif re.fullmatch(r'"setSnapshot",', rhs.strip()):
        new=f'{ind}{enum}::PatchSnapshot(_) => "patchSnapshot",'
    elif re.fullmatch(r'TAG_SET_SNAPSHOT,', rhs.strip()):
        new=f'{ind}{enum}::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,'
    elif rhs.startswith('format!("set-snapshot'):
        new=f"{ind}{enum}::PatchSnapshot({leaf} {{ patch }}) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),"
    elif re.match(r'enc_\w*snapshot_bin\(snapshot, &mut out\),', rhs) or rhs.startswith('write_bin_snapshot('):
        new=f"{ind}{enum}::PatchSnapshot({leaf} {{ patch }}) => out.extend(protocol::OpBinary::encode_op(patch)?),"
    if new and line.rstrip().endswith(('{', '(', '=>')):
        out.insert(len(out)-1, new); log.append(f"added-before {idx+1}: {new.strip()[:110]}")
    elif new:
        out.append(new); log.append(f"added {idx+1}: {new.strip()[:110]}")
    else:
        log.append(f"MANUAL {idx+1}: {line.strip()[:140]}")
s2='\n'.join(out)
if 'TAG_PATCH_SNAPSHOT' in s2 and 'const TAG_PATCH_SNAPSHOT' not in s2:
    m=re.search(r'^(\s*)(pub(?:\(crate\))? )?const TAG_SET_SNAPSHOT: u8 = (.*)$', s2, re.M)
    if m:
        decl=m.group(0).replace('TAG_SET_SNAPSHOT','TAG_PATCH_SNAPSHOT').replace('"set-snapshot"','"patch-snapshot"')
        s2=s2.replace(m.group(0), m.group(0)+'\n'+decl,1); log.append("added const "+decl.strip()[:120])
    else: log.append("MANUAL: TAG_SET_SNAPSHOT const not found")
open(path,'w',encoding='utf-8').write(s2)
print('\n'.join(log))
