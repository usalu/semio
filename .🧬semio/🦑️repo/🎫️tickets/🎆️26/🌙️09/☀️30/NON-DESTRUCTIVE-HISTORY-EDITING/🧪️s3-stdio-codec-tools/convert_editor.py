import re,sys
for path in sys.argv[1:]:
    s=open(path,encoding='utf-8').read()
    o=s
    # call site
    pat=re.compile(r'(semio_s_artifact_stdio_contract::)?editing::snapshot_edit_set_snapshot\(event, snapshot, \|snapshot\| (\w+)::SetSnapshot\(([\w:]*?)::?SetSnapshot \{ snapshot(?:: (?:snapshot|Box::new\(snapshot\)))? \}\)\)')
    def call(m):
        prefix=m.group(1) or ''
        enum=m.group(2); module=m.group(3)
        if module.endswith('snapshot_edit_set_snapshot'):
            target='patch_snapshot::PatchSnapshot'
        elif module.endswith('set_snapshot'):
            target=module[:-len('set_snapshot')]+'patch_snapshot::PatchSnapshot'
        elif module=='':
            target='patch_snapshot::PatchSnapshot'
        else:
            target=module+'::patch_snapshot::PatchSnapshot'
        return f'{prefix}editing::snapshot_edit_patch(event, snapshot, |patch| {enum}::PatchSnapshot({target} {{ patch }}))'
    s=pat.sub(call,s)
    # alias import
    if 'snapshot_edit_set_snapshot::' not in s.replace('editing::snapshot_edit_set_snapshot','') :
        s=s.replace('set_snapshot as snapshot_edit_set_snapshot','patch_snapshot')
    left=[l for l in s.split('\n') if 'snapshot_edit_set_snapshot' in l]
    open(path,'w',encoding='utf-8').write(s)
    print(('changed ' if s!=o else 'unchanged ')+path[-90:], ('LEFT: '+' || '.join(x.strip()[:160] for x in left)) if left else '')
