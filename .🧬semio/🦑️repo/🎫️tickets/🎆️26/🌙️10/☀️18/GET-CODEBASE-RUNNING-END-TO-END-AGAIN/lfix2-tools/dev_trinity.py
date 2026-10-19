from devlib import *
T = EXT_ROOT + '🔱️trinity/🪆️graph/🧪️tests/✏️editor/'
files = ['🔎️query-ownership-and-examples/🦀️.rs', '🔭️lod-catalogue-and-examples/🦀️.rs', '🖼️scene-rendering-and-text-edit/🦀️.rs']
render_re = re.compile(r'(?P<ind>[ \t]*)let (?P<n>\w+) = app\.render\((?P<k>"[^"]*"), None, (?P<v>.*?)\)\.await(?P<t>[^;\n]*);\n')
proj_re = r'artifact_app_laws::project_and_retire_fixture_tree\(%s\)(?:\.expect\("[^"]*"\)|\.map_err\(str::to_string\)\?)'
def mapped(t):
    return t.replace('.map_err(str::to_string)?', '.map_err(|error| format!("{error:?}"))?')
for f in files:
    p = T + f
    s = read(p)
    out = []
    pos = 0
    n_pairs = n_inline = 0
    while True:
        m = render_re.search(s, pos)
        if not m:
            out.append(s[pos:]); break
        out.append(s[pos:m.start()])
        name = m.group('n')
        head = f"{m.group('ind')}let {{J}} = artifact_app_laws::render_registered_fixture_app(&mut app.app, {m.group('k')}, {m.group('v')}, cx).await{mapped(m.group('t'))};\n"
        rest_start = m.end()
        window = s[rest_start:rest_start + 1500]
        pm = re.match(r'(?P<ind2>[ \t]*)let (?P<j>\w+) = ' + proj_re % re.escape(name) + r';\n', window)
        if pm:
            out.append(head.replace('{J}', pm.group('j')))
            pos = rest_start + pm.end()
            n_pairs += 1
            continue
        out.append(head.replace('{J}', name))
        pos = rest_start
        # inline project(name) in the next lines
        nxt = re.search(proj_re % re.escape(name), s[pos:pos + 1200])
        assert nxt, (f, name, s[pos:pos + 200])
        s = s[:pos + nxt.start()] + name + s[pos + nxt.end():]
        n_inline += 1
    s = ''.join(out)
    write(p, s)
    print(f[:30], 'pairs', n_pairs, 'inline', n_inline)
# rail verbs
p = T + '🩹️rail-verb-laws/🦀️.rs'
s = read(p)
a = 'semio_framework_plugin::artifact_app_laws::fixture_identity()).await;\n        assert_eq!(node_names(&app, Some(&ids)).await'
print(s.count(', &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;'))
s = s.replace('artifact_app_laws::settle_history_verb(&mut app.app, "undo", JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;', 'artifact_app_laws::settle_history_verb(&mut app.app, "undo", JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await;')
s = s.replace('artifact_app_laws::settle_history_verb(&mut app.app, "redo", JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;', 'artifact_app_laws::settle_history_verb(&mut app.app, "redo", JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await;')
write(p, s)
