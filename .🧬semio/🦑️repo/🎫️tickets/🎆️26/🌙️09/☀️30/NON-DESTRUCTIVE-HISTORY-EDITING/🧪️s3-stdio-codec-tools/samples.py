import re,sys
C='semio_s_artifact_stdio_contract::editing'
for p in sys.argv[1:]:
    s=open(p,encoding='utf-8').read(); lines=s.split('\n'); out=[]; n=0; in_list=False
    for i,l in enumerate(lines):
        out.append(l)
        if re.search(r'let (samples|one_per_variant|variants|all|cases|mutations|every)\b[^=]*= (vec!)?\[\s*$',l): in_list=True; continue
        if in_list and re.match(r'^\s*\];?\s*$',l): in_list=False
        m=re.match(r'^(\s*)(\w+)::SetSnapshot\((\w+)::SetSnapshot \{.*\}\),\s*$',l)
        if in_list and m and not any('PatchSnapshot(' in x for x in lines[i+1:i+3]):
            out.append(f'{m.group(1)}{m.group(2)}::PatchSnapshot(patch_snapshot::PatchSnapshot {{ patch: {C}::SnapshotPatch::Set {{ path: "/schema".into(), value: dsl::DslValue::String("stdio.patch-snapshot.witness".into()) }} }}),'); n+=1
    open(p,'w',encoding='utf-8').write('\n'.join(out)); print(n,p.split('🗿️artifacts/')[1][:70])
