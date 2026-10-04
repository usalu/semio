import re,sys
# usage: codec.py file enum leaf [parse] [decode-string|decode-proto] [encode-push]
path, enum, leaf = sys.argv[1:4]; modes=set(sys.argv[4:])
s=open(path,encoding='utf-8').read(); log=[]
C='semio_s_artifact_stdio_contract::editing'
def before(pattern, make, tag):
    global s
    m=re.search(pattern, s, re.M)
    if not m: log.append(f"MISS {tag}"); return
    ind=re.match(r'\s*', m.group(0).lstrip('\n')).group(0)
    start=m.start() if not m.group(0).startswith('\n') else m.start()+1
    s=s[:start]+make(ind)+s[start:]; log.append(f"ok {tag}")
if 'parse' in modes and f'"patch-snapshot" =>' not in s:
    before(r'^\s*"set-snapshot" =>', lambda i: f'{i}"patch-snapshot" => {C}::snapshot_patch_from_text(line).map(|patch| {enum}::PatchSnapshot({leaf} {{ patch }})),\n', 'parse')
if 'decode-string' in modes and 'TAG_PATCH_SNAPSHOT =>' not in s:
    before(r'^\s*TAG_SET_SNAPSHOT =>', lambda i: f'{i}TAG_PATCH_SNAPSHOT => {enum}::PatchSnapshot({leaf} {{ patch: {C}::snapshot_patch_from_bytes(reader.read_bytes(reader.remaining()).map_err(|e| e.to_string())?)? }}),\n', 'decode-string')
if 'decode-proto' in modes and 'TAG_PATCH_SNAPSHOT =>' not in s:
    before(r'^\s*TAG_SET_SNAPSHOT =>', lambda i: f'{i}TAG_PATCH_SNAPSHOT => {enum}::PatchSnapshot({leaf} {{ patch: <{C}::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed {{ what: "patch-snapshot payload", offset: reader.position() as u64, detail: e.to_string() }})?)? }}),\n', 'decode-proto')
if 'encode-push' in modes and 'out.push(TAG_PATCH_SNAPSHOT)' not in s:
    before(r'^\s*'+enum+r'::SetSnapshot\(set_snapshot::SetSnapshot \{ snapshot \}\) => \{\n\s*out\.push\(TAG_SET_SNAPSHOT\);', lambda i: f'{i}{enum}::PatchSnapshot({leaf} {{ patch }}) => {{\n{i}    out.push(TAG_PATCH_SNAPSHOT);\n{i}    out.extend(protocol::OpBinary::encode_op(patch)?);\n{i}}}\n', 'encode-push')
PROTO=f'<{C}::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed {{ what: "patch-snapshot payload", offset: reader.position() as u64, detail: e.to_string() }})?)?'
if 'decode-proto-ok' in modes and 'TAG_PATCH_SNAPSHOT =>' not in s:
    before(r'^\s*TAG_SET_SNAPSHOT =>', lambda i: f'{i}TAG_PATCH_SNAPSHOT => Ok({enum}::PatchSnapshot({leaf} {{ patch: {PROTO} }})),\n', 'decode-proto-ok')
if 'decode-string-ok' in modes and 'TAG_PATCH_SNAPSHOT =>' not in s:
    before(r'^\s*TAG_SET_SNAPSHOT =>', lambda i: f'{i}TAG_PATCH_SNAPSHOT => Ok({enum}::PatchSnapshot({leaf} {{ patch: {C}::snapshot_patch_from_bytes(reader.read_bytes(reader.remaining()).map_err(|e| e.to_string())?)? }})),\n', 'decode-string-ok')
if 'demo' in modes:
    at=s.find('fn demo_mutation_cases')
    if at<0: log.append('MISS demo-fn')
    elif f'{enum}::PatchSnapshot(' in s[at:s.find('\n}\n',at)]: log.append('skip demo')
    else:
        m=re.compile(r'^(\s*)'+enum+r'::SetSnapshot\(', re.M).search(s, at)
        if not m: log.append('MISS demo')
        else:
            i=m.group(1).lstrip('\n')
            st=m.start()+ (1 if s[m.start()]=='\n' else 0)
            s=s[:st]+f'{i}{enum}::PatchSnapshot({leaf} {{ patch: {C}::SnapshotPatch::Set {{ path: "/schema".into(), value: dsl::DslValue::String("stdio.patch-snapshot.witness".into()) }} }}),\n'+s[st:]
            log.append('ok demo')
if 'TAG_PATCH_SNAPSHOT' in s and 'const TAG_PATCH_SNAPSHOT' not in s:
    m=re.search(r'^(\s*)(pub(?:\(crate\))? )?const TAG_SET_SNAPSHOT: u8 = (.*)$', s, re.M)
    if m:
        decl=m.group(0).replace('TAG_SET_SNAPSHOT','TAG_PATCH_SNAPSHOT').replace('"set-snapshot"','"patch-snapshot"')
        s=s.replace(m.group(0), m.group(0)+'\n'+decl,1); log.append("ok const")
    else: log.append("MISS const")
open(path,'w',encoding='utf-8').write(s); print(' '.join(log))
