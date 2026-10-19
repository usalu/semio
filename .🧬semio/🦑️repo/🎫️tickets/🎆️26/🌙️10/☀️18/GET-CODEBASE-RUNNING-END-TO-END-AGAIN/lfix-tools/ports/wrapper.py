"""🔧 Generic wrapper-fixture codemod: rewrites stale framework fixture calls whose app is a wrapper that owns an OriginalFixtureScope.
Profile: inner(root) -> expr of the VcsArtifactApp (e.g. `app.0`), cx(root) -> expr of the lent context (e.g. `app.1.cx()`)."""
import re, sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools')
from portlib import *

POLICY = 'semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT'

class Profile:
    def __init__(self, inner, cx, policy=POLICY, roots=('app',), boxed=True):
        self.inner = inner; self.cx = cx; self.policy = policy; self.roots = roots; self.boxed = boxed

def _strip(expr):
    e = expr.strip()
    changed = True
    while changed:
        changed = False
        for pre in ('&mut ', '&', '*'):
            if e.startswith(pre):
                e = e[len(pre):].lstrip(); changed = True
    return e

def _root_name(expr):
    e = _strip(expr)
    m = re.match(r'[A-Za-z_][A-Za-z0-9_]*', e)
    return m.group(0) if m else None

def method_rewrite(src, prof, name, old_arity, new_args, *, retarget=True):
    """`ROOT.name(args)` -> `INNER(ROOT).name(new_args(args, ROOT))`."""
    counts = [0]
    calls = find_calls(src, name, 'method', old_arity)
    for c in reversed(calls):
        recv = c.recv(src)
        if recv is None:
            continue
        root = _root_name(recv)
        if root not in prof.roots or _strip(recv) not in (root, prof.inner(root)):
            continue
        args = new_args(list(c.args), root)
        if args is None:
            continue
        text = ', '.join(args)
        inner = prof.inner(root)
        # replace receiver text and args
        recv_start = c.name_start - 1 - len(recv)
        # c.name_start is index of name; the '.' sits at name_start-1
        recv_span_start = src.rfind(recv, 0, c.name_start)
        src = src[:recv_span_start] + (inner if retarget else recv) + src[recv_span_start + len(recv):c.open + 1] + text + src[c.close:]
        counts[0] += 1
    return src, counts[0]

def func_rewrite(src, name, old_arity, new_args, pre_ok=('', '::')):
    n = 0
    calls = find_calls(src, name, 'func', old_arity)
    for c in reversed(calls):
        args = new_args(list(c.args))
        if args is None:
            continue
        src = src[:c.open + 1] + ', '.join(args) + src[c.close:]
        n += 1
    return src, n

def append_cx_func(src, prof, name, old_arity, app_arg_index=0):
    """free fn whose first arg is the app expression: append the wrapper's cx."""
    def f(a):
        root = _root_name(a[app_arg_index])
        if root not in prof.roots:
            return None
        return a + [prof.cx(root)]
    return func_rewrite(src, name, old_arity, f)

def close_fix(src, prof, expect='original caller paid app close'):
    """`close_registered_fixture_app(&mut ROOT.0, POLICY);` -> `...(…, POLICY, CX).expect(..);`"""
    n = 0
    calls = find_calls(src, 'close_registered_fixture_app', 'func', 2)
    for c in reversed(calls):
        root = _root_name(c.args[0])
        if root not in prof.roots:
            continue
        end = c.close + 1
        tail = src[end:end + 12]
        ins = ', ' + prof.cx(root) + ')'
        suffix = ''
        if re.match(r'\s*;', tail):
            suffix = f'.expect("{expect}")'
        src = src[:c.close] + ', ' + prof.cx(root) + ')' + suffix + src[c.close + 1:]
        n += 1
    return src, n

root_name = _root_name
strip_ref = _strip

DFA = 'semio_framework_plugin::artifact_app_laws::dispatch_fixture_action'

def _owned_args(a):
    a = a.strip()
    if a == 'None':
        return 'None'
    m = re.match(r'^Some\(\s*&\s*(.*)\)$', a, re.S)
    if m:
        inner = m.group(1).strip()
        if re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*', inner):
            return f'Some({inner}.clone())'
        return f'Some({inner})'
    m = re.match(r'^(Some\(.*\))\.as_ref\(\)$', a, re.S)
    if m:
        return m.group(1)
    m = re.match(r'^Some\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)$', a)
    if m:
        return f'Some({m.group(1)}.clone())'
    m = re.match(r'^(\w+)\.as_ref\(\)$', a)
    if m:
        return f'{m.group(1)}.clone()'
    return None

def handle_action_rewrite(src, prof):
    """`R.handle_action(action, args, meta, identity)` -> `dispatch_fixture_action(&mut INNER(R), action, owned_args, meta, identity, CX(R), POLICY.close)`."""
    n = 0
    for c in reversed(find_calls(src, 'handle_action', 'method', 4)):
        recv = c.recv(src)
        root = root_of(recv) if recv else None
        if recv is None or root not in prof.roots or strip_ref(recv) not in (root, prof.inner(root)):
            continue
        args = _owned_args(c.args[1])
        if args is None:
            print('   !! handle_action args shape not handled:', c.args[1][:60])
            continue
        recv_start = src.rfind(recv, 0, c.name_start)
        new = f'{DFA}({app_ref(prof, root)}, {c.args[0]}, {args}, {c.args[2]}, {c.args[3]}, {prof.cx(root)}, {prof.policy}.close)'
        src = src[:recv_start] + new + src[c.close + 1:]
        n += 1
    return src, n

IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'

def append_policy_identity(src, name, old_arity, policy=POLICY):
    """law helpers that gained `(mounted_policy, identity)` (they own their original caller): append both when missing."""
    n = 0
    for c in reversed(find_calls(src, name, 'any', old_arity)):
        src = src[:c.open + 1] + c.raw.rstrip().rstrip(',') + f', {policy}, &mut {IDENT}' + src[c.close:]
        n += 1
    return src, n

SCOPE_T = 'semio_framework_plugin::artifact_app_laws::OriginalFixtureScope'
LAWS = 'semio_framework_plugin::artifact_app_laws'

def norm_first(arg, root, prof):
    """first-arg app expression -> `&mut INNER(root)` when it names the wrapper (`app`, `&mut app`, `&mut *app`, `&mut app.0`)."""
    e = strip_ref(arg)
    if prof.inner(root) == root:
        return arg.strip() if e == root else None
    if e in (root, prof.inner(root)):
        return '&mut ' + prof.inner(root)
    return None

def func_calls_with_cx(src, prof, specs, report=None):
    """specs: [(name, old_arity, expect_suffix_or_None)] free/UFCS calls whose first arg is the app; append cx (and `.expect`) and normalise arg0."""
    total = 0
    for name, arity, expect in specs:
        n = 0
        for c in reversed(find_calls(src, name, 'any', arity)):
            if c.pre == '.':
                continue
            root = root_name(c.args[0])
            if root not in prof.roots:
                continue
            first = norm_first(c.args[0], root, prof)
            if first is None:
                continue
            args = [first] + c.args[1:] + [prof.cx(root)]
            text = ', '.join(args)
            tail = src[c.close + 1:c.close + 14]
            suffix = ''
            if expect and re.match(r'\s*;', tail):
                suffix = f'.expect("{expect}")'
            src = src[:c.open + 1] + text + ')' + suffix + src[c.close + 1:]
            n += 1
        total += n
        if report is not None:
            report[name] = n
    return src, total

def render_patterns(src, prof):
    """`project_and_retire_fixture_tree(R.render(K, None, V).await.<tail>)` and the `let tree = ..; let json = project..(tree)..;` pair."""
    n = 0
    for c in reversed(find_calls(src, 'project_and_retire_fixture_tree', 'any', 1)):
        arg = c.args[0]
        m = re.match(r'^(\w+)\.render\((.*)\)\.await\s*(?:\.expect\([^()]*\)|\.unwrap_or_else\((?:[^()]|\([^()]*\))*\))\s*$', arg, re.S)
        if not m:
            continue
        root = m.group(1)
        if root not in prof.roots:
            continue
        rargs = split_top(m.group(2))
        parts = [m.group(2)[a:b].strip() for a, b in rargs]
        if len(parts) != 3:
            continue
        new = render_call(prof, root, parts)
        start = src.rfind('project_and_retire_fixture_tree', 0, c.open)
        pre_path = src[:start]
        # strip a qualifying path before the call name
        m2 = re.search(r'(?:[A-Za-z_][A-Za-z0-9_]*::)+$', pre_path)
        if m2:
            start = m2.start()
        src = src[:start] + new + src[c.close + 1:]
        n += 1
    return src, n

def render_pair_patterns(src, prof):
    """`let T = R.render(K, None, V).await..?;` + next statement `let J = ...project_and_retire_fixture_tree(T)..?;` (T in tree|node)."""
    pat = re.compile(r'(?P<ind>[ \t]*)let (?P<tv>\w+) = (?P<bp>Box::pin\()?(?P<root>\w+)\.render\((?P<args>[^;]*?)\)(?(bp)\))\.await(?P<tail>[^;]*?);\n[ \t]*let (?P<jv>\w+) = (?:[A-Za-z_][A-Za-z0-9_]*::)*project_and_retire_fixture_tree\((?P=tv)\)(?P<tail2>[^;]*?);', re.S)
    n = 0
    def rep(m):
        nonlocal n
        root = m.group('root')
        if root not in prof.roots:
            return m.group(0)
        parts = [m.group('args')[a:b].strip() for a, b in split_top(m.group('args'))]
        if len(parts) != 3:
            return m.group(0)
        n += 1
        tail2 = m.group('tail2').strip()
        handler = tail2 if tail2 else m.group('tail').strip()
        handler = handler.replace('.map_err(str::to_string)', '.map_err(|error| format!("{error:?}"))')
        return f"{m.group('ind')}let {m.group('jv')} = {render_call(prof, root, parts)}{handler};"
    return pat.sub(rep, src), n

def ufcs_calls(src, prof, name, old_arity, new_args):
    """`PluginApp::name(APP, ..)` -> args rewritten by new_args(args, root); APP must name the wrapper."""
    n = 0
    for c in reversed(find_calls(src, name, 'any', old_arity)):
        if src[max(0, c.name_start - 11):c.name_start] != 'PluginApp::':
            continue
        root = root_name(c.args[0])
        if root not in prof.roots:
            continue
        if norm_first(c.args[0], root, prof) is None:
            continue
        src = src[:c.open + 1] + ', '.join(new_args(c.args, root)) + src[c.close:]
        n += 1
    return src, n

def standard_calls(src, prof, report=None):
    """the full mechanical set for an app wrapper `prof`; returns (src, counts)."""
    counts = {}
    src, counts['dispatch_typed'] = method_rewrite(src, prof, 'dispatch_typed', 3, lambda a, r: a + [prof.cx(r)])
    src, counts['dispatch_typed2'] = method_rewrite(src, prof, 'dispatch_typed', 2, lambda a, r: a + ['&mut ' + IDENT, prof.cx(r)])
    src, counts['maintenance_step'] = method_rewrite(src, prof, 'maintenance_step', 2, lambda a, r: None if 'cx' in a[0] else [prof.cx(r), prof.policy + '.maintenance'])
    src, counts['close_step'] = method_rewrite(src, prof, 'close_step', 2, lambda a, r: None if 'cx' in a[0] else [prof.cx(r), prof.policy + '.close'])
    src, counts['advance'] = method_rewrite(src, prof, 'advance_typed_operation_publication', 2, lambda a, r: None if 'cx' in a[1] else [a[0], prof.cx(r)])
    src, counts['tick_backbone'] = method_rewrite(src, prof, 'tick_backbone', 1, lambda a, r: a + [prof.cx(r)])
    src, counts['ingest_text'] = method_rewrite(src, prof, 'ingest_operations_text', 1, lambda a, r: a + ['&mut ' + IDENT], retarget=False)
    src, counts['ingest_operations'] = method_rewrite(src, prof, 'ingest_operations', 2, lambda a, r: a + [prof.cx(r)])
    src, counts['poll_archive'] = method_rewrite(src, prof, 'poll_document_archive_load', 2, lambda a, r: a + [prof.cx(r)])
    src, counts['seal'] = method_rewrite(src, prof, 'seal_artifact_envelope_ingress', 1, lambda a, r: a + [prof.cx(r)])
    src, counts['advance_load'] = method_rewrite(src, prof, 'advance_artifact_envelope_load', 1, lambda a, r: a + [prof.cx(r)])
    src, counts['handle_action'] = handle_action_rewrite(src, prof)
    src, counts['render1'] = render_patterns(src, prof)
    src, counts['render2'] = render_pair_patterns(src, prof)
    src, counts['render_ready'] = render_ready_patterns(src, prof)
    rep = {}
    src, _ = func_calls_with_cx(src, prof, [('settle_registered_typed_operation', 4, None), ('close_registered_fixture_app', 2, 'original caller paid app close'), ('settle_history_verb', 5, None), ('assert_undo_redo_round_trip', 7, None), ('load_document', 3, None), ('load_document_text', 3, None)], rep)
    counts.update(rep)
    src, counts['ufcs_advance1'] = ufcs_calls(src, prof, 'advance_typed_operation_publication', 1, lambda a, r: [norm_first(a[0], r, prof), '&mut ' + IDENT, prof.cx(r)])
    src, counts['ufcs_maint'] = ufcs_calls(src, prof, 'maintenance_step', 3, lambda a, r: [norm_first(a[0], r, prof)] + a[1:] if 'cx' in a[1] else [norm_first(a[0], r, prof), prof.cx(r), prof.policy + '.maintenance'])
    src, counts['ufcs_maint2'] = ufcs_calls(src, prof, 'maintenance_step', 2, lambda a, r: [norm_first(a[0], r, prof), prof.cx(r), a[1]])
    src, counts['ufcs_close'] = ufcs_calls(src, prof, 'close_step', 2, lambda a, r: [norm_first(a[0], r, prof), prof.cx(r), a[1]])
    src, counts['ufcs_close3'] = ufcs_calls(src, prof, 'close_step', 3, lambda a, r: [norm_first(a[0], r, prof)] + a[1:])
    src, counts['ufcs_poll'] = ufcs_calls(src, prof, 'poll_document_archive_load', 3, lambda a, r: [norm_first(a[0], r, prof), a[1], a[2], prof.cx(r)])
    for nm, ar in [('begin_document_archive_load', 3), ('acknowledge_document_archive_load', 2), ('document_archive', 1)]:
        src, counts['ufcs_' + nm] = ufcs_calls(src, prof, nm, ar, lambda a, r: [norm_first(a[0], r, prof)] + a[1:])
    src, counts['reserved'] = func_rewrite_first(src, prof, 'settle_framework_reserved_admission', 2)
    return src, {k: v for k, v in counts.items() if v}

def func_rewrite_first(src, prof, name, arity):
    """normalise only the first (app) argument of a free fn to `&mut INNER`."""
    n = 0
    for c in reversed(find_calls(src, name, 'any', arity)):
        if c.pre == '.':
            continue
        root = root_name(c.args[0])
        if root not in prof.roots:
            continue
        first = norm_first(c.args[0], root, prof)
        if first is None or first == c.args[0]:
            continue
        src = src[:c.open + 1] + ', '.join([first] + c.args[1:]) + src[c.close:]
        n += 1
    return src, n

def import_media_rewrite(src, prof):
    """`R.import_media(port, media, meta)` -> `INNER.import_media(port, media, meta, CX)`."""
    return method_rewrite(src, prof, 'import_media', 3, lambda a, r: a + [prof.cx(r)])

def alias_to_tuple(src, alias, vis='pub '):
    """`type NAME = TYPE;` -> `struct NAME(TYPE, Scope)` + Deref/DerefMut to the app (field 0) — the scope is field 1."""
    m = re.search(r'(?:pub(?:\([a-z]+\))? )?type ' + alias + r' = (.+);\n', src)
    assert m, alias
    ty = m.group(1).strip()
    new = f'''/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the app's paid close.
{vis}struct {alias}(pub {ty}, pub {SCOPE_T});

impl std::ops::Deref for {alias} {{
    type Target = {ty};
    fn deref(&self) -> &Self::Target {{
        &self.0
    }}
}}

impl std::ops::DerefMut for {alias} {{
    fn deref_mut(&mut self) -> &mut Self::Target {{
        &mut self.0
    }}
}}
'''
    return src.replace(m.group(0), new), ty

def detect_roots(src, wrapper, raw, extra=()):
    """names of variables / parameters that hold the wrapper (or its raw app) in this file."""
    roots = set(extra)
    for m in re.finditer(r'\b(\w+)\s*:\s*&(?:mut\s+)?(?:\w+::)*(?:' + re.escape(wrapper) + '|' + re.escape(raw) + r')\b', src):
        roots.add(m.group(1))
    ctors = set(m.group(1) for m in re.finditer(r'(?:pub )?(?:async )?fn (\w+)\s*\([^)]*\)\s*->\s*(?:\w+::)*' + re.escape(wrapper) + r'\b', src))
    for c in ctors:
        for m in re.finditer(r'let (?:mut )?(\w+)\s*=\s*(?:\w+::)*' + re.escape(c) + r'\s*\([^)]*\)(?:\.await)?\s*;', src):
            roots.add(m.group(1))
    return tuple(sorted(r for r in roots if r not in ('self', '_')))

def wrapper_port(path, wrapper, raw, *, extra_roots=(), ctor_var='app', helpers_raw_to_wrapper=True, policy=POLICY, close_method='close', dry=False):
    """port a `struct W(RAW);` fixture file generically. Returns (new_src, counts, roots)."""
    s = read(path)
    # 1. struct field + scope
    pat = re.compile(r'(?P<vis>pub(?:\([a-z]+\))? )?struct ' + wrapper + r'\((?P<fvis>pub(?:\([a-z]+\))? )?' + re.escape(raw) + r'\);')
    m = pat.search(s)
    assert m, (path[-50:], 'struct ' + wrapper)
    vis = m.group('vis') or ''
    fvis = m.group('fvis') or 'pub '
    if fvis.strip() != 'pub':
        fvis = 'pub '
    s = s.replace(m.group(0), f"/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.\n    {vis}struct {wrapper}({fvis}{raw}, {fvis}{SCOPE_T});")
    # 2. close inside Drop / close methods: `close_registered_fixture_app(&mut self.0, POLICY);`
    s, n = re.subn(r'((?:semio_framework_plugin::)?(?:artifact_app_laws::)?close_registered_fixture_app\(&mut self\.0, ' + re.escape(policy) + r')\)\s*;', r'\1, self.1.cx()).expect("original caller paid app close");', s)
    if re.search(r'fn close\(app: &mut ' + re.escape(raw) + r'\b', s):
        s = s.replace('close(&mut self.0);', 'close(self);')
    # 3. ctor: scope before the app, wrapper construction with scope
    k = 0
    def ctor_fix(mm):
        nonlocal k
        k += 1
        return f"{mm.group(1)}let scope = {SCOPE_T}::admit();\n{mm.group(1)}let mut {mm.group(2)} = "
    cm = list(re.finditer(r'(?P<w>fn \w+\([^)]*\)\s*->\s*(?:\w+::)*' + wrapper + r'\s*\{\n)(?P<body>(?:(?!\n    \}\n|\n\}\n).)*?)\b' + wrapper + r'\((?P<var>\w+)\)', s, re.S))
    for mm in reversed(cm):
        body = mm.group('body')
        im = re.search(r'(?P<ind>[ \t]*)let mut (?P<var>\w+) = ', body)
        assert im, ('ctor body', path[-40:])
        newbody = body[:im.start()] + f"{im.group('ind')}let scope = {SCOPE_T}::admit();\n" + body[im.start():]
        s = s[:mm.start('body')] + newbody + f"{wrapper}({mm.group('var')}, scope)" + s[mm.end():]
    # 4. helper signatures
    if helpers_raw_to_wrapper:
        s = re.sub(r'(fn \w+\(\s*app: &mut )' + re.escape(raw) + r'\b', r'\g<1>' + wrapper, s)
    roots = detect_roots(s, wrapper, raw, extra_roots + (ctor_var,))
    prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', policy=policy, roots=roots)
    s, counts = standard_calls(s, prof)
    if not dry:
        write(path, s)
    return s, counts, roots

def user_port(path, wrapper, raw, *, rename_raw=True, extra_roots=(), policy=POLICY, dry=False):
    """port a file that USES an already-ported wrapper: raw alias -> wrapper in type positions, then every call site."""
    s = read(path)
    if rename_raw:
        s = re.sub(r'(?<![A-Za-z0-9_])' + re.escape(raw) + r'(?![A-Za-z0-9_])', wrapper, s)
    roots = detect_roots(s, wrapper, raw, extra_roots)
    prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', policy=policy, roots=roots)
    s, counts = standard_calls(s, prof)
    if not dry:
        write(path, s)
    return s, counts, roots

def raw_scoped_port(path, helpers, *, top_roots=('app', 'reopened'), anchor=r'let mut (?:app|reopened) = Box::new\(', policy=POLICY, dry=False, cx_type="semio_framework_job::StepContext<'_>"):
    """raw (unwrapped) app tests: a local `scope` is admitted before the first app is built; named helper fns gain a trailing `cx` parameter
    (their bodies thread `cx`), and every call site / direct framework call at the top level threads `scope.cx()`."""
    s = read(path)
    # 1. split helper spans
    spans = []
    for h in helpers:
        for m in re.finditer(r'(?P<ind>[ \t]*)(?P<pre>(?:pub )?(?:async )?fn ' + re.escape(h) + r')\s*\(', s):
            open_idx = m.end() - 1
            close = match_paren(s, open_idx)
            brace = s.index('{', close)
            end = match_paren(s, brace)
            spans.append((m.start(), open_idx, close, brace, end, h))
    spans.sort()
    helper_prof = Profile(lambda r: r, lambda r: 'cx', policy=policy, roots=('app',))
    top_prof = Profile(lambda r: r, lambda r: 'scope.cx()', policy=policy, roots=tuple(top_roots))
    out = []
    pos = 0
    total = {}
    for start, open_idx, close, brace, end, h in spans:
        top = s[pos:start]
        top, c = standard_calls(top, top_prof)
        for k, v in c.items(): total[k] = total.get(k, 0) + v
        out.append(top)
        sig = s[start:close]
        params = s[open_idx + 1:close].rstrip().rstrip(',')
        newsig = s[start:open_idx + 1] + params + f', cx: &mut {cx_type}'
        body = s[close:end + 1]
        body, c = standard_calls(body, helper_prof)
        for k, v in c.items(): total['helper_' + k] = total.get('helper_' + k, 0) + v
        out.append(newsig + body)
        pos = end + 1
    top = s[pos:]
    top, c = standard_calls(top, top_prof)
    for k, v in c.items(): total[k] = total.get(k, 0) + v
    out.append(top)
    s = ''.join(out)
    # 2. helper call sites (any call of a helper that is not its definition): append `scope.cx()`
    for h in helpers:
        for c in reversed(find_calls(s, h, 'func')):
            before = s[max(0, c.name_start - 16):c.name_start]
            if re.search(r'fn\s+$', before):
                continue
            if 'cx' in c.args[-1] and ('scope.cx()' in c.args[-1] or c.args[-1].strip() == 'cx'):
                continue
            s = s[:c.open + 1] + c.raw.rstrip().rstrip(',') + ', scope.cx()' + s[c.close:]
            total['site_' + h] = total.get('site_' + h, 0) + 1
    # 2b. inside helper bodies the lent context is `cx`, not the test's `scope`
    for m in list(re.finditer(r'fn \w+\s*(?:<[^>]*>)?\s*\(', s))[::-1]:
        o = m.end() - 1; c = match_paren(s, o)
        if c < 0 or not re.search(r'cx:\s*&mut\s+' + re.escape(cx_type.split('<')[0]), s[o:c]):
            continue
        b = s.find('{', c); e = match_paren(s, b)
        if b < 0 or e < 0:
            continue
        s = s[:b] + s[b:e + 1].replace('scope.cx()', 'cx') + s[e + 1:]
    # 3. scope declaration
    ams = list(re.finditer(r'(?P<ind>[ \t]*)(?P<a>' + anchor + ')', s))
    assert ams, ('anchor', path[-50:])
    test_marks = [m.start() for m in re.finditer(r'#\[(?:test|semio_framework_async_macros::async_test)\]', s)]
    for am in reversed(ams):
        mark = max([t for t in test_marks if t < am.start()], default=0)
        if 'OriginalFixtureScope::admit()' in s[mark:am.start()]:
            continue
        s = s[:am.start()] + f"{am.group('ind')}let mut scope = {SCOPE_T}::admit();\n" + s[am.start():]
    # 4. render helpers: the `let tree = ..;` + project pair inside helper bodies is handled by standard_calls' render patterns
    if not dry:
        write(path, s)
    return s, total


def app_ref(prof, root):
    if prof.inner(root) == root:
        return f'&mut *{root}' if prof.boxed else f'&mut {root}'
    return f'&mut {prof.inner(root)}'

def ctor_fix(src):
    """old app constructors lacking `(mounted_policy, identity)`: new_app_with_registry(_and_members) with 2 args, artifact_app_laws::new_app with 1."""
    total = 0
    for name, arity in [('new_app_with_registry_and_members', 2), ('new_app_with_registry', 2), ('new_registered_app_with_members', 2), ('new_registered_app', 2)]:
        src, n = append_policy_identity(src, name, arity)
        total += n
    return src, total

def std_drop(src, wrapper, policy=POLICY):
    """replace a hand-rolled `impl Drop for W` close loop by the framework's exact retained close under the scope."""
    m = re.search(r'(?P<ind>[ \t]*)impl Drop for ' + re.escape(wrapper) + r' \{', src)
    if not m:
        return src, 0
    brace = src.index('{', m.start())
    end = match_paren(src, brace)
    body = src[brace:end + 1]
    if 'close_registered_fixture_app' in body and 'self.1.cx()' in body:
        return src, 0
    ind = m.group('ind')
    new = f'''{ind}impl Drop for {wrapper} {{
{ind}    fn drop(&mut self) {{
{ind}        if !std::thread::panicking() {{
{ind}            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, {policy}, self.1.cx()).expect("original caller paid app close");
{ind}        }}
{ind}    }}
{ind}}}'''
    return src[:m.start()] + new + src[end + 1:], 1


def render_call(prof, root, parts):
    """`render_registered_fixture_app(..)` (or the `_over` variant when a snapshot override is passed) for `R.render(key, override, view)` parts."""
    if parts[1].strip() == 'None':
        return f'{LAWS}::render_registered_fixture_app({app_ref(prof, root)}, {parts[0]}, {parts[2]}, {prof.cx(root)}).await'
    return f'{LAWS}::render_registered_fixture_app_over({app_ref(prof, root)}, {parts[0]}, {parts[1]}, {parts[2]}, {prof.cx(root)}).await'


def maintain_rewrite(src, prof):
    """`...testing::maintain(APP, bytes)` / `maintain_items(APP, items, bytes)` -> `(&mut INNER, .., CX)`."""
    n = 0
    for name, arity in (('maintain', 2), ('maintain_items', 3)):
        for c in reversed(find_calls(src, name, 'any', arity)):
            if not src[max(0, c.name_start - 9):c.name_start].endswith('testing::'):
                continue
            root = root_name(c.args[0])
            if root not in prof.roots:
                continue
            first = norm_first(c.args[0], root, prof)
            if first is None:
                continue
            src = src[:c.open + 1] + ', '.join([first] + c.args[1:] + [prof.cx(root)]) + src[c.close:]
            n += 1
    return src, n


def with_registry_fix(src, policy=POLICY):
    """`X::with_registry(app, registry, actor)` -> `(.., actor, POLICY, &mut IDENTITY)` and the constructor settle after the instance bind."""
    n = 0
    for c in reversed(find_calls(src, 'with_registry', 'any', 3)):
        stmt_start = src.rfind(';', 0, c.name_start) + 1
        lm = re.search(r'let (?:mut )?(\w+)\s*=', src[stmt_start:c.name_start])
        if not lm:
            continue
        var = lm.group(1)
        src = src[:c.open + 1] + ', '.join(list(c.args) + [policy, '&mut ' + IDENT]) + src[c.close:]
        c_end = src.index(';', c.open) + 1
        bm = re.compile(r'[^\n]*' + re.escape(var) + r'\.bind_instance_id\([^\n]*;').search(src, c_end)
        if bm and bm.start() - c_end < 400:
            ind = re.match(r'[ \t]*', src[src.rfind('\n', 0, bm.start()) + 1:]).group(0)
            ins = f'\n{ind}semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut {var}, {policy});'
            src = src[:bm.end()] + ins + src[bm.end():]
            n += 1
    return src, n


def render_ready_patterns(src, prof):
    """`project_and_retire_fixture_tree(RR(R.render(K, None, V)).expect(..))` (RR = resolve_ready) -> `RR(render_registered_fixture_app(..))`."""
    n = 0
    for c in reversed(find_calls(src, 'project_and_retire_fixture_tree', 'any', 1)):
        arg = c.args[0]
        m = re.match(r'^(?P<rr>(?:::)?semio_framework_async::poll::resolve_ready)\((?P<root>\w+)\.render\((?P<args>.*)\)\)\s*\.expect\([^()]*\)\s*$', arg, re.S)
        if not m or m.group('root') not in prof.roots:
            continue
        parts = [m.group('args')[a:b].strip() for a, b in split_top(m.group('args'))]
        if len(parts) != 3:
            continue
        root = m.group('root')
        call = render_call(prof, root, parts)[:-len('.await')]
        new = f"{m.group('rr')}({call})"
        start = src.rfind('project_and_retire_fixture_tree', 0, c.open)
        m2 = re.search(r'(?:[A-Za-z_][A-Za-z0-9_]*::)+$', src[:start])
        if m2:
            start = m2.start()
        src = src[:start] + new + src[c.close + 1:]
        n += 1
    return src, n
