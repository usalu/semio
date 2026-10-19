import sys, re, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wrapper import *
D = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
ROOTS = ('app', 'a', 'b', 'replica', 'remote', 'local', 'fresh', 'reopened', 'receiver', 'sender')
for rel in ['🧪️tests/🧪️typing-runs/🦀️.rs', '🧪️tests/✂️concurrent-typing/🦀️.rs']:
    p = D + rel
    s = read(p)
    s, ty = alias_to_tuple(s, 'Replica', vis='')
    s, n = ctor_fix(s)
    m = re.search(r'let \(mut a, mut b\) = (paired_registered_apps_with_members::<[^;]*?>\([^;]*?\))\.await;', s, re.S)
    if m:
        call = m.group(1)
        call = call[:-1] + f', {POLICY}, &mut {IDENT})'
        s = s.replace(m.group(0), f'let (a, b) = {call}.await;\n    let (mut a, mut b) = (Replica(a, {SCOPE_T}::admit()), Replica(b, {SCOPE_T}::admit()));')
    old = 'let mut app = new_app_with_registry_and_members::<EditorApp<WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>('
    if old in s:
        s = s.replace(old, f'let scope = {SCOPE_T}::admit();\n    let mut app = new_app_with_registry_and_members::<EditorApp<WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(')
        s = s.replace('    app.bind_instance_id(meta("local").instance_id).await;\n    app\n}', '    app.bind_instance_id(meta("local").instance_id).await;\n    Replica(app, scope)\n}', 1)
    write(p, s)
    s, c, r = user_port(p, 'Replica', 'Replica', rename_raw=False, extra_roots=ROOTS)
    print(rel, c, r)
