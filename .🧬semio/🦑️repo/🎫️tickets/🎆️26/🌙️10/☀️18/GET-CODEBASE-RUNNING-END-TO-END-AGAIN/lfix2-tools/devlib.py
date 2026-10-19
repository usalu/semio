"""🔧 lfix2 dev codemod library: lifts the older dev fixture call shapes to the shapes lfix's wrapper.standard_calls understands, then applies it."""
import re, sys, os
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools')
from wrapper import *
from portlib import *
import wrapper as _W
def _owned_args(a):
    r = _W._owned_args(a)
    if r is not None:
        return r
    a = a.strip()
    m = re.match(r'^([A-Za-z_][\w\.]*)\.as_ref\(\)$', a)
    if m:
        return f'{m.group(1)}.clone()'
    return None

DFA_T = 'semio_framework_plugin::artifact_app_laws::dispatch_fixture_action'
LAWS_T = 'semio_framework_plugin::artifact_app_laws'
EXT_ROOT = '/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/'

def lift_old_arities(src, prof):
    """older dev call shapes -> the intermediate arities standard_calls expects. returns (src, counts)."""
    counts = {}
    def bump(k, n):
        if n:
            counts[k] = counts.get(k, 0) + n
    def lift(name, old, extra, only_roots=True, first=0):
        nonlocal src
        n = 0
        for c in reversed(find_calls(src, name, 'any', old)):
            if c.pre == '.':
                continue
            root = root_name(c.args[first])
            if only_roots and root not in prof.roots:
                continue
            src = src[:c.open + 1] + ', '.join(c.args + extra) + src[c.close:]
            n += 1
        bump(name, n)
    P = prof.policy
    lift('settle_registered_typed_operation', 2, [P, '&mut ' + IDENT])
    lift('settle_history_verb', 3, [P, '&mut ' + IDENT])
    lift('close_registered_fixture_app', 1, [P])
    lift('load_document', 2, ['&mut ' + IDENT])
    lift('assert_undo_redo_round_trip', 5, [P, '&mut ' + IDENT])
    n = 0
    for c in reversed(find_calls(src, 'handle_action', 'any', 4)):
        if c.pre != '::' or not src[max(0, c.name_start - 11):c.name_start].endswith('PluginApp::'):
            continue
        root = root_name(c.args[0])
        if root not in prof.roots:
            continue
        a = c.args
        src = src[:c.open + 1] + ', '.join([a[0], a[1], a[2], a[3], '&mut ' + IDENT]) + src[c.close:]
        n += 1
    bump('ufcs_handle_action', n)
    return src, counts

def handle_action3(src, prof):
    """`R.handle_action(action, args, meta)` (3 args) -> dispatch_fixture_action over the wrapper's inner app."""
    n = 0
    for c in reversed(find_calls(src, 'handle_action', 'method', 3)):
        recv = c.recv(src)
        root = root_of(recv) if recv else None
        if recv is None or root not in prof.roots or strip_ref(recv) not in (root, prof.inner(root)):
            continue
        args = _owned_args(c.args[1])
        if args is None:
            print('   !! handle_action args shape not handled:', c.args[1][:80])
            continue
        recv_start = src.rfind(recv, 0, c.name_start)
        new = f'{DFA_T}({app_ref(prof, root)}, {c.args[0]}, {args}, {c.args[2]}, &mut {IDENT}, {prof.cx(root)}, {prof.policy}.close)'
        src = src[:recv_start] + new + src[c.close + 1:]
        n += 1
    return src, n

def ufcs_handle_action(src, prof):
    """`PluginApp::handle_action(APP, action, args, meta, identity)` -> dispatch_fixture_action."""
    n = 0
    for c in reversed(find_calls(src, 'handle_action', 'any', 5)):
        if not src[max(0, c.name_start - 11):c.name_start].endswith('PluginApp::'):
            continue
        root = root_name(c.args[0])
        if root not in prof.roots:
            continue
        args = _owned_args(c.args[2])
        if args is None:
            print('   !! ufcs handle_action args shape:', c.args[2][:80]); continue
        first = norm_first(c.args[0], root, prof)
        start = src.rfind('PluginApp::', 0, c.name_start)
        new = f'{DFA_T}({first}, {c.args[1]}, {args}, {c.args[3]}, {c.args[4]}, {prof.cx(root)}, {prof.policy}.close)'
        src = src[:start] + new + src[c.close + 1:]
        n += 1
    return src, n

def let_roots(src, ctors=('app', 'app_with_registry')):
    """variables bound by `let [mut] X = [Box::new(][context::]CTOR().await`."""
    out = set()
    for ctor in ctors:
        for m in re.finditer(r'let (?:mut )?(\w+)\s*=\s*(?:Box::new\()?(?:\w+::)*' + ctor + r'\(\)(?:\.await)?\)?\s*;', src):
            out.add(m.group(1))
    return out

def port_dev_file(path, wrapper, raw, *, extra_roots=(), ctors=('app', 'app_with_registry'), dry=False, rename=True):
    s = read(path)
    if rename:
        s = re.sub(r'(?<![A-Za-z0-9_])' + re.escape(raw) + r'(?![A-Za-z0-9_])', wrapper, s)
    roots = set(detect_roots(s, wrapper, raw, tuple(extra_roots))) | let_roots(s, ctors) | set(extra_roots)
    roots.discard('self')
    prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=tuple(sorted(roots)))
    s, c0 = lift_old_arities(s, prof)
    s, c1 = handle_action3(s, prof)
    s, c2 = ufcs_handle_action(s, prof)
    s, c3 = standard_calls(s, prof)
    s, c4 = ctor_fix(s)
    counts = {'lift': c0, 'ha3': c1, 'ufcs_ha': c2, 'std': c3, 'ctor': c4}
    if not dry:
        write(path, s)
    return s, counts, prof.roots
