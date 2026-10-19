import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
CUR = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()')
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'

p = CUR + '🧪️tests/🔬️unit/🦀️.rs'

# ---- test bodies (unit file)
s = read(p)
CXA = lambda a, root: a + [prof.cx(root)]
s, n = method_rewrite(s, prof, 'dispatch_typed', 3, CXA); print('dispatch_typed', n)
s, n = method_rewrite(s, prof, 'maintenance_step', 2, lambda a, root: [prof.cx(root), prof.policy + '.maintenance']); print('maintenance_step', n)
s, n = method_rewrite(s, prof, 'advance_typed_operation_publication', 2, lambda a, root: [a[0], prof.cx(root)]); print('advance', n)
s, n = method_rewrite(s, prof, 'close_step', 2, lambda a, root: [prof.cx(root), prof.policy + '.close']); print('close_step', n)
# UFCS PluginApp::... (&mut *app, ..)
def ufcs(name, old_arity, new):
    global s
    calls = find_calls(s, name, 'any', old_arity)
    n = 0
    for c in reversed(calls):
        if s[max(0, c.name_start - 11):c.name_start] != 'PluginApp::':
            continue
        root = root_name(c.args[0])
        if root not in prof.roots:
            continue
        s = s[:c.open + 1] + ', '.join(new(c.args, root)) + s[c.close:]
        n += 1
    print(name, n)
ufcs('poll_document_archive_load', 3, lambda a, r: ['&mut ' + prof.inner(r), a[1], a[2], prof.cx(r)])
ufcs('maintenance_step', 3, lambda a, r: ['&mut ' + prof.inner(r), prof.cx(r), prof.policy + '.maintenance'])
ufcs('begin_document_archive_load', 3, lambda a, r: ['&mut ' + prof.inner(r), a[1], a[2]])
ufcs('acknowledge_document_archive_load', 2, lambda a, r: ['&mut ' + prof.inner(r), a[1]])
write(p, s)
