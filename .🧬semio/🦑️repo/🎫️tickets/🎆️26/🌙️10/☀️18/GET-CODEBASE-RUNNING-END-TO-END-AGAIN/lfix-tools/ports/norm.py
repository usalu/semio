import sys, re, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wrapper import *
R = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/'
SUF = '🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
EXPORT = re.compile(r'::semio_framework_async::poll::resolve_ready\(PluginApp::export_media\(&mut app, ("[^"]+")\)\)\.expect\(')
def port(path):
    s = read(path)
    s, ty = alias_to_tuple(s, 'NormApp', vis='pub ')
    i = s.index('/// Field `1` is the fixture')
    j = s.index('impl std::ops::DerefMut for NormApp')
    k = s.index('}\n', s.index('fn deref_mut', j)) 
    k = s.index('}\n', k + 2) + 2
    block = s[i:k]
    block = block.replace('\n', '\n    ').rstrip(' ') 
    s = s[:i] + block + s[k:]
    m = re.search(r'(?P<ind>[ \t]*)let mut app = new_app_with_registry(?:_and_members)?::<', s)
    assert m, path
    s = s[:m.start()] + f"{m.group('ind')}let scope = {SCOPE_T}::admit();\n" + s[m.start():]
    fi = s.index('fn app_with_registry')
    ti = s.index('\n        app\n    }', fi)
    s = s[:ti] + '\n        NormApp(app, scope)\n    }' + s[ti + len('\n        app\n    }'):]
    roots = ('app', 'reopened', 'fresh', 'receiver', 'sender', 'instance_a', 'instance_b')
    prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=roots)
    s, counts = standard_calls(s, prof)
    n = 0
    def exp(mm):
        nonlocal n; n += 1
        return f'::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::export_fixture_media(&mut app.0, {mm.group(1)}, app.1.cx(), {POLICY}.close)).expect('
    s = EXPORT.sub(exp, s)
    counts['export'] = n
    write(path, s)
    return counts
if __name__ == '__main__':
    for d in sorted(os.listdir(R)):
        p = R + d + '/' + SUF
        if os.path.exists(p):
            print(d, port(p))
