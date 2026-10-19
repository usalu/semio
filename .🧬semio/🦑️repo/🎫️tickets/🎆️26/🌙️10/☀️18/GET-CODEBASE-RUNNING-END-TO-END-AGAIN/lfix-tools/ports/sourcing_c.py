import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
CUR = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()')
CXA = lambda a, root: a + [prof.cx(root)]

p = CUR + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
old = '''    pub async fn render(app: &mut SourcingTestApp, body_key: &str) -> String {'''
new = '''    /// 📤️ One media port exported through the app's original caller, its pending turns driven until the port answers.
    pub async fn export_media(app: &mut SourcingTestApp, port: &str) -> semio_framework_plugin::Media {
        for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            if let Some(media) = app.0.export_media(port, app.1.cx()).await.expect("media export turn") {
                return media;
            }
        }
        panic!("media export of {port} stayed pending under its original caller");
    }

    pub async fn render(app: &mut SourcingTestApp, body_key: &str) -> String {'''
assert s.count(old) == 1
s = s.replace(old, new)
old = '''let media = ::semio_framework_async::poll::resolve_ready(app.export_media("catalog:out")).expect("catalog export");'''
assert s.count(old) == 1
s = s.replace(old, '''let media = context::export_media(&mut app, "catalog:out").await;''')
s, n = method_rewrite(s, prof, 'dispatch_typed', 3, CXA); print('unit dispatch', n)
write(p, s)

for rel in ['🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs', '🎮️commands/📥️load-document-json/🧪️tests/🔬️unit/🦀️.rs']:
    p = CUR + rel
    s = read(p)
    s, n = method_rewrite(s, prof, 'dispatch_typed', 3, CXA); print(rel[-40:], 'dispatch', n)
    if 'load-document-json' in rel:
        old = '''    let admitted = app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("undo");'''
        assert s.count(old) == 1
        s = s.replace(old, '''    let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "undo", None, &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("undo");''')
    write(p, s)
