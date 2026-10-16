#!/usr/bin/env python3
"""🧲️ Privacy-faithful union oracle: loads several crates as modules of one sandbox crate (crate:: and sibling-crate paths remapped, pub(crate) narrowed per crate) and runs the analysis-only rustc on it."""
import os, re, sys, json, shutil, subprocess, collections, hashlib
from pathlib import Path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod
from rsitems import lex

REPO = '/Users/ueli/Documents/semio'
SCRATCH = os.environ.get('PHC_SCRATCH', '/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad')
RUSTC = ['rustc', '+nightly-2026-07-20']
STD_ROOTS = {'std', 'core', 'alloc'}


def mod_files(root_path, ws_fs):
    hcmod.FS = ws_fs
    root, modmap = hcmod.load_crate(root_path)
    return root, modmap


def transform(src, ident, firstparty, under_test, fm_mod, neutral_externs=True):
    toks = [t for t in lex(src)]
    patches = []
    n = len(toks)
    T = lambda i: toks[i]
    for i, t in enumerate(toks):
        k, x, sl, el, a = t
        if k != 'id':
            continue
        prev = toks[i - 1][1] if i > 0 else ''
        prevk = toks[i - 1][0] if i > 0 else ''
        nxt = toks[i + 1][1] if i + 1 < n else ''
        nxt2 = toks[i + 2][1] if i + 2 < n else ''
        if x == 'crate':
            if prev == '$':
                continue
            if prev == '(' and i > 1 and toks[i - 2][1] == 'pub' and nxt == ')':
                patches.append((a, a + len(x), 'in crate::' + ident))
            elif nxt == ':' and nxt2 == ':':
                patches.append((a, a + len(x), 'crate::' + ident))
            continue
        if x in firstparty and ((nxt == ':' and nxt2 == ':') or (nxt == 'as' and prev in ('use', '{', ','))):
            if prev == ':' and i > 1 and toks[i - 2][1] == ':':
                continue
            if (prev == '.' and not (i > 1 and toks[i - 2][1] == '.')) or prev == '$':
                continue
            patches.append((a, a + len(x), 'crate::' + x))
            continue
        if x in ('cfg', 'cfg_attr') and not under_test:
            j = i + 1
            if j < n and toks[j][1] == '!':
                j += 1
            if j < n and toks[j][1] == '(':
                d = 0
                k2 = j
                while k2 < n:
                    xk = toks[k2][1]
                    if toks[k2][0] == 'p' and xk == '(':
                        d += 1
                    elif toks[k2][0] == 'p' and xk == ')':
                        d -= 1
                        if d == 0:
                            break
                    elif toks[k2][0] == 'id' and xk == 'test' and toks[k2 - 1][1] in ('(', ',') and k2 + 1 < n and toks[k2 + 1][1] in (')', ','):
                        patches.append((toks[k2][4], toks[k2][4] + 4, '__nt'))
                    k2 += 1
    return patches


def apply_patches(src, patches):
    out = src
    for a, b, r in sorted(patches, key=lambda p: -p[0]):
        out = out[:a] + r + out[b:]
    return out


def inline_descendants(fm):
    for c in fm.children.values():
        if c.inline:
            yield c
            yield from inline_descendants(c)


def strip_item_patches(mod, src):
    patches = []
    for m in [mod] + list(inline_descendants(mod)):
        for it in m.all_items:
            if it.kind == 'extern_crate':
                patches.append((it.a, it.b, '\n' * src[it.a:it.b].count('\n')))
        for u in m.uses:
            if u.glob and u.path:
                first = u.path[0]
                if first in ('crate', 'self', 'super') or first in STD_ROOTS:
                    continue
                if hcmod.lookup(m, first) is not None:
                    continue
                it = u.item
                text = src[it.a:it.b]
                # single glob use only
                if text.count('*') == 1 and '{' not in text:
                    patches.append((it.a, it.b, 'const _: () = ();' + '\n' * text.count('\n')))
    return patches


BINDGEN = re.compile(r'wasmtime::component::bindgen!\(\{.*?\}\);', re.S)
BINDGEN_STUB = 'pub mod semio { pub mod framework { pub mod effects {} pub mod ui {} pub mod types {} pub mod pure {} pub mod capabilities {} pub mod events {} pub mod host_async {} pub mod instance_lifetime {} } } pub mod exports { pub mod semio { pub mod framework { pub mod reactor {} pub mod jobs {} pub mod codec {} } } }'
INCLUDE_LIT = re.compile(r'(?:include_str|include_bytes|include)!\(\s*"([^"]+)"')


class Union:
    def __init__(self, ws, crates, scope_rel, tag='u'):
        """crates: dict ident -> absolute root file path; scope_rel: repo-relative directory whose tracked files are copied into the sandbox."""
        self.ws, self.crates, self.scope_rel, self.tag = ws, crates, scope_rel, tag
        self.scope = os.path.join(REPO, scope_rel)

    def build(self, under_test=None, extra_files=()):
        sb = Path(SCRATCH) / f'union-{self.tag}'
        if sb.exists():
            shutil.rmtree(sb)
        sb.mkdir(parents=True)
        files = {p for p in self.ws.cur if p.startswith(self.scope + '/')}
        files -= set(self.ws.deleted)
        hcmod.FS = self.ws
        written = set()
        firstparty = set(self.crates)
        includes = set()
        for ident, root in self.crates.items():
            croot, modmap = hcmod.load_crate(root)
            for m in hcmod.walk(croot):
                if m.inline or getattr(m, 'missing', False):
                    continue
                if m.file in written:
                    continue
                written.add(m.file)
                src = m.src
                patches = transform(src, ident, firstparty, ident == under_test, m)
                patches += strip_item_patches(m, src)
                new = apply_patches(src, patches)
                new = BINDGEN.sub(lambda mm: BINDGEN_STUB, new)
                for lit in INCLUDE_LIT.findall(src):
                    includes.add(os.path.normpath(os.path.join(os.path.dirname(m.file), lit)))
                dst = sb / os.path.relpath(m.file, REPO)
                dst.parent.mkdir(parents=True, exist_ok=True)
                dst.write_text(new)
        for f in sorted(files | includes | set(extra_files)):
            if f in written or not (os.path.exists(f) or f in self.ws.cur):
                continue
            dst = sb / os.path.relpath(f, REPO)
            dst.parent.mkdir(parents=True, exist_ok=True)
            if f in self.ws.cur:
                dst.write_text(self.ws.cur[f])
            else:
                try:
                    shutil.copy2(f, dst)
                except Exception:
                    pass
        mounts = []
        for ident, root in self.crates.items():
            mounts.append(f'#[path = "{os.path.relpath(root, REPO)}"]\npub mod {ident};\n')
        (sb / '__union.rs').write_text('\n'.join(mounts))
        return sb

    def run(self, under_test=None, cfgs=(), tag_extra=''):
        sb = self.build(under_test)
        cmd = RUSTC + ['--edition', '2021', '--crate-type', 'lib', '--crate-name', 'host_union', '--emit=metadata', '-o', '/dev/null', '--error-format=json', '-W', 'unused-imports']
        if under_test:
            cmd.append('--test')
        for c in cfgs:
            cmd += ['--cfg', c]
        cmd.append('__union.rs')
        env = dict(os.environ, CARGO_MANIFEST_DIR=str(sb), CARGO_PKG_NAME='host_union', CARGO_PKG_VERSION='0.1.0')
        p = subprocess.run(cmd, cwd=sb, capture_output=True, text=True, env=env)
        diags = []
        for line in p.stderr.splitlines():
            line = line.strip()
            if not line.startswith('{'):
                continue
            try:
                d = json.loads(line)
            except Exception:
                continue
            diags.append(d)
        return diags, p.stderr


def prim(d):
    for s in d.get('spans', []):
        if s.get('is_primary'):
            return s
    return (d.get('spans') or [None])[0]


PATH_PREFIX = re.compile(r'crate::[A-Za-z0-9_]+::')


def key_of(d, strip_crate=True):
    if d.get('level') not in ('error',):
        return None
    msg = d.get('message', '')
    if msg.startswith('aborting due to') or 'previous error' in msg:
        return None
    s = prim(d)
    code = (d.get('code') or {}).get('code') or ''
    f = os.path.normpath(s['file_name']) if s else ''
    msg = PATH_PREFIX.sub('', msg)
    msg = re.sub(r'`crate::[A-Za-z0-9_]+`', '`crate`', msg)
    return (code, msg, f)


def multiset(diags):
    c = collections.Counter()
    for d in diags:
        k = key_of(d)
        if k:
            c[k] += 1
    return c


def diff(a, b):
    only_a = a - b
    only_b = b - a
    return only_a, only_b


EXTERN_RE = re.compile(r"(cannot find module or crate `(\w+)`|unresolved imports? `(\w+)`$|cannot find attribute `\w+` in this scope|use of unresolved module or unlinked crate|cannot find `(\w+)` in the crate root: could not find)")


RETIRED = set()


EXTERNAL = set()


def is_extern_noise(code, msg, crates):
    if msg.startswith("couldn't read"):
        return True
    m = re.search(r"(?:cannot find module or crate|use of unresolved module or unlinked crate|use of undeclared crate or module|failed to resolve: use of unresolved module or unlinked crate) `(\w+)`", msg)
    if m:
        return m.group(1) in EXTERNAL
    m = re.search(r"unresolved imports? (`\w+`(?:, `\w+`)*)$", msg)
    if m:
        names = re.findall(r'`(\w+)`', m.group(1))
        return all(n in EXTERNAL for n in names)
    if re.search(r"cannot find attribute `\w+` in this scope", msg):
        return True
    m = re.search(r"cannot find `(\w+)` in the crate root", msg)
    if m:
        return m.group(1) in EXTERNAL
    return False
