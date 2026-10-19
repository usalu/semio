"""🔧 Template port: `type XApp = VcsArtifactApp<..>` + `app()`/`settle()`/`close()` mounted-tool fixtures -> XApp struct owning an OriginalFixtureScope."""
import re, sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools')
from portlib import *

SCOPE = 'semio_framework_plugin::artifact_app_laws::OriginalFixtureScope'
POLICY = 'semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT'

def port(path, alias, label):
    s = read(path)
    m = re.search(r'type ' + alias + r' = (VcsArtifactApp<.*>);\n', s)
    assert m, (path, 'alias')
    ty = m.group(1)
    s = s.replace(m.group(0), f'''struct {alias} {{
    app: {ty},
    scope: {SCOPE},
}}

impl std::ops::Deref for {alias} {{
    type Target = {ty};
    fn deref(&self) -> &Self::Target {{
        &self.app
    }}
}}

impl std::ops::DerefMut for {alias} {{
    fn deref_mut(&mut self) -> &mut Self::Target {{
        &mut self.app
    }}
}}
''')
    m = re.search(r'fn app\(\) -> ' + alias + r' \{\n    let mut app = block_on\(', s)
    assert m, (path, 'app()')
    s = s.replace(m.group(0), f'fn app() -> {alias} {{\n    let scope = {SCOPE}::admit();\n    let mut app = block_on(')
    m = re.search(r'(    block_on\(app\.bind_instance_id\(1\)\);\n)    app\n\}', s)
    assert m, (path, 'app() tail')
    s = s.replace(m.group(0), m.group(1) + f'    {alias} {{ app, scope }}\n}}')
    s = s.replace(f'fn settle(app: &mut {alias}, result: Result<InvocationResult, semio_framework_plugin::Fault>) -> InvocationResult {{', f'fn settle<E: std::fmt::Debug>(app: &mut {alias}, result: Result<InvocationResult, E>) -> InvocationResult {{')
    s, n = rewrite(s, 'maintenance_step', 'func', 3, lambda a, c, src: ['&mut app.app', 'app.scope.cx()', POLICY + '.maintenance'] if a[0] == 'app' else None)
    assert n == 1, (path, 'maintenance', n)
    def adv(a, c, src):
        return [a[0], 'app.scope.cx()']
    s, n = rewrite(s, 'advance_typed_operation_publication', 'method', 2, adv, only_if=lambda c, src: c.recv(src) == 'app')
    assert n == 1, (path, 'advance', n)
    s = s.replace('block_on(app.advance_typed_operation_publication(', 'block_on(app.app.advance_typed_operation_publication(')
    s, n = rewrite(s, 'dispatch_typed', 'method', 3, lambda a, c, src: a + ['app.scope.cx()'], only_if=lambda c, src: c.recv(src) == 'app')
    assert n >= 1, (path, 'dispatch')
    s = re.sub(r'block_on\(app\.dispatch_typed\(', 'block_on(app.app.dispatch_typed(', s)
    m = re.search(r'fn close\(app: &mut ' + alias + r'\) \{', s)
    assert m
    end = match_paren(s, m.end() - 1)
    s = s[:m.start()] + f'fn close(app: &mut {alias}) {{\n    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.app, {POLICY}, app.scope.cx()).expect("{label} app paid close");\n}}' + s[end + 1:]
    write(path, s)
    return s
