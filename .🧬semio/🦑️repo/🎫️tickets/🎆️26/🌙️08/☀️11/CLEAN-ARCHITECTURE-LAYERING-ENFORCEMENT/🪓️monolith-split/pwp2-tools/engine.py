#!/usr/bin/env python3
# ⚙️ Carve engine of PLUGIN-WP2: item cuts, path resolution (`crate::m::N` -> final crate path) and import headers for moved text.
import re, os, sys, collections, json
from common import *
import model
from rs import blank, uses, expand_use, item_names
import plan_tools as pt

M = model

# root-level names (facade root re-exports) that WP2 files reach through `crate::NAME`; value = absolute path valid in every crate
ROOT_NAMES = {
    'MountedOwnerPolicyV1': 'semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1',
    'MountedOwnerPhaseV1': 'semio_framework_plugin_mounted_owner::MountedOwnerPhaseV1',
    'MountedOwnerTurnV1': 'semio_framework_plugin_mounted_owner::MountedOwnerTurnV1',
    'OriginalRenderResult': 'semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult',
    'ComponentTree': 'semio_framework_ui_runtime::ComponentTree',
    'ArtifactOwnedDisposer': 'semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer',
    'Fault': 'semio_framework::Fault',
    'AppIo': 'semio_framework::AppIo',
    'ConfigSpec': 'semio_framework::ConfigSpec',
    'CommandGrammar': 'semio_framework::CommandGrammar',
}
ALIAS_PATHS = {'io': 'semio_framework_os_kernel::io', 'host': 'semio_framework_plugin_actor_host'}

INPUT_DIR = [PLUG]

def set_input(path):
    INPUT_DIR[0] = path

def src_text(rel):
    p = INPUT_DIR[0] + '/' + rel
    if os.path.exists(p):
        return read(p)
    return read(SNAP + '/' + rel)

def has_body(text):
    return any(l.strip() and not l.lstrip().startswith('//!') for l in text.split('\n'))

# -------------------------------------------------------------------------------- homes
FILE_MOD = {f: n for n, f in M.MOD_FILE.items()}
LIFT_MODS = {c: {m for m, _, _ in l} for c, l in M.LIFTS_IN.items()}
CUT_BY = {(c['file'], n): c for c in M.CUTS for n in c['names']}

HOME_OVERRIDE = {
    'project_media_error': ('decl-app', 'media_operation_refusal'),
    'key_media_error': ('decl-app', 'media_operation_refusal'),
    'media_projection_error': ('decl-app', 'media_intrinsic'),
    'plugin_internal_fault': ('reactor-app', None),
    'EphemeralParts': ('window-transient', 'window_transient'),
}

class Homes:
    def __init__(self):
        self.by_name = collections.defaultdict(list)
        for r in M.UNITS:
            f = r['file']
            c = CUT_BY.get((f, r['name']))
            self.by_name[r['name']].append(dict(dst=c['to'] if c else r['dst'], src=r['src'], src_mod=FILE_MOD.get(f), file=f, kind=r['kind'], cut=c))

    def home(self, mod, name, cur):
        """-> (crate, module-in-crate or None) of item `name` referenced as `crate::mod::name`, or None when unknown"""
        if name in HOME_OVERRIDE:
            return HOME_OVERRIDE[name]
        rows = [r for r in self.by_name.get(name, []) if r['kind'] != 'impl']
        if not rows:
            return None
        same = [r for r in rows if r['src_mod'] == mod] if mod else []
        cand = same or rows
        if len({r['dst'] for r in cand}) > 1:
            pref = [r for r in cand if r['dst'] == cur]
            cand = pref or cand
        if len({(r['dst'], r['file']) for r in cand}) > 1:
            return None
        r = cand[0]
        dst = r['dst']
        if r['cut']:
            return (dst, r['cut']['module'] if dst in M.WP2 else None)
        src_mod = r['src_mod']
        if dst == r['src']:
            if src_mod is None:
                return (dst, None)
            return (dst, None if M.ROOT.get(dst) == M.MOD_FILE.get(src_mod) else src_mod)
        if dst in M.WP2:
            if r['file'] == '🧵️retained-command/🧬️context/📸️fields/🦀️.rs':
                return (dst, 'owned_context')
            if src_mod in LIFT_MODS.get(dst, ()) or src_mod in M.CHILDREN.get(dst, {}):
                return (dst, src_mod)
        return (dst, None)

HOMES = Homes()

# -------------------------------------------------------------------------------- scanning helpers
def inline_mod_spans(clean):
    spans = []
    for m in re.finditer(r'\bmod\s+[A-Za-z_]\w*\s*\{', clean):
        depth, j = 1, m.end()
        while j < len(clean) and depth:
            depth += (clean[j] == '{') - (clean[j] == '}')
            j += 1
        spans.append((m.end(), j))
    return spans

def in_spans(pos, spans):
    return any(a <= pos < b for a, b in spans)

PATH_START = re.compile(r'(?<![A-Za-z0-9_$:])(crate|super)::')
SEG = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')

def parse_path(clean, i):
    """parse `seg(::seg)*` starting at i; returns (segs, end) ; a following `::{`/`::*` ends the path"""
    segs = []
    j = i
    while True:
        m = SEG.match(clean, j)
        if not m:
            break
        segs.append(m.group(0))
        j = m.end()
        if clean.startswith('::', j) and SEG.match(clean, j + 2):
            j += 2
            continue
        break
    return segs, j

class Resolver:
    def __init__(self, cur):
        self.cur = cur

    def build(self, crate, mod, name, rest):
        tail = ('::' + '::'.join(rest)) if rest else ''
        if crate == self.cur:
            return 'crate::' + (mod + '::' if mod else '') + name + tail
        return M.libname(crate) + '::' + (mod + '::' if mod else '') + name + tail

    def resolve_segs(self, segs):
        """segs after `crate::`/`super::` (first = module, root name or alias). -> full replacement path or None to keep"""
        m0 = segs[0]
        rest0 = segs[1:]
        if m0 in M.MOD_FILE and rest0 and rest0[0] in M.MOD_FILE and M.MOD_CRATE.get(rest0[0]) in M.WP2 and M.MOD_CRATE.get(m0) not in M.WP2:
            return self.resolve_segs(rest0)
        if m0 in M.MOD_FILE:
            if rest0:
                h = HOMES.home(m0, rest0[0], self.cur)
                if h:
                    return self.build(h[0], h[1], rest0[0], rest0[1:])
            mc = M.MOD_CRATE.get(m0)
            if mc is None or mc.startswith('TOP') or mc.startswith('UNMAPPED'):
                return None
            is_root = M.ROOT.get(mc) == M.MOD_FILE[m0]
            if mc == self.cur:
                return 'crate::' + '::'.join(rest0) if is_root and rest0 else ('crate' if is_root else 'crate::' + '::'.join(segs))
            if is_root:
                return M.libname(mc) + (('::' + '::'.join(rest0)) if rest0 else '')
            return M.libname(mc) + '::' + '::'.join(segs)
        if m0 in ROOT_NAMES:
            return ROOT_NAMES[m0] + (('::' + '::'.join(rest0)) if rest0 else '')
        if m0 in ALIAS_PATHS:
            return ALIAS_PATHS[m0] + (('::' + '::'.join(rest0)) if rest0 else '')
        if m0 in ('store', 'protocol', 'dsl', 'vcs'):
            return None
        rows = [r for r in HOMES.by_name.get(m0, []) if r['kind'] != 'impl']
        if rows:
            h = HOMES.home(rows[0]['src_mod'], m0, self.cur)
            if h:
                return self.build(h[0], h[1], m0, rest0)
        return None

def rewrite_paths(text, cur, report=None):
    """rewrite every `crate::..`/`super::..` path (not inside inline mods for `super`) that leaves or enters the final crate layout"""
    clean = blank(text)
    spans = inline_mod_spans(clean)
    res = Resolver(cur)
    out, last = [], 0
    for m in PATH_START.finditer(clean):
        if m.start() < last:
            continue
        kw = m.group(1)
        if kw == 'super' and in_spans(m.start(), spans):
            continue
        j = m.end()
        if j < len(clean) and clean[j] == '{':
            continue
        segs, end = parse_path(clean, j)
        if not segs:
            continue
        rep = res.resolve_segs(segs)
        if rep is None and kw == 'super':
            rep = 'crate::' + '::'.join(segs)
        if rep is None:
            continue
        if report is not None:
            report.append((kw, '::'.join(segs), rep))
        out.append(text[last:m.start()])
        out.append(rep)
        last = end
    out.append(text[last:])
    return ''.join(out)

USE_STMT = re.compile(r'(?m)^([ \t]*)((?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)((?:pub(?:\([^)]*\))?[ \t]+)?)use[ \t]+((?:crate|super)::[^;]*);')

def expand_group_uses(text):
    """expand grouped `use crate::{..}` / `use super::{..}` into one statement per leaf"""
    clean = blank(text)
    out, last = [], 0
    for m in USE_STMT.finditer(clean):
        body = m.group(4)
        if '{' not in body and '*' not in body:
            continue
        first = re.match(r'(?:crate|super)::(\w+)', body)
        if first and first.group(1) in ('store', 'protocol', 'dsl', 'vcs'):
            continue
        s, e = m.start(), m.end()
        body_text = text[m.start(4):m.end(4)]
        leaves = expand_use(body_text)
        lines = []
        for path, alias, glob in leaves:
            p = '::'.join(path)
            if glob:
                lines.append(f'{m.group(1)}{text[m.start(2):m.end(2)]}{m.group(3)}use {p}::*;')
            else:
                a = f' as {alias}' if alias else ''
                lines.append(f'{m.group(1)}{text[m.start(2):m.end(2)]}{m.group(3)}use {p}{a};')
        out.append(text[last:s]); out.append('\n'.join(l.replace('\n', ' ') for l in lines)); last = e
    out.append(text[last:])
    return ''.join(out)

# -------------------------------------------------------------------------------- cuts
def unit_rows_for(file_rel, names):
    return [r for r in M.UNITS if r['file'] == file_rel and r['name'] in names]

def spans_of(rows, file_rel):
    sp = []
    for r in rows:
        for m in r['members']:
            if m['file'] == file_rel:
                sp.append((m['s'], m['e']))
    return sorted(set(sp))

def cut_text(text, spans):
    """remove the 1-based inclusive line spans; returns (remaining text, [cut item texts in order])"""
    lines = text.split('\n')
    cuts = []
    drop = set()
    for s, e in spans:
        cuts.append('\n'.join(lines[s - 1:e]))
        drop.update(range(s - 1, e))
    keep = []
    for i, l in enumerate(lines):
        if i in drop:
            continue
        keep.append(l)
    out = []
    for l in keep:
        if l.strip() == '' and out and out[-1].strip() == '':
            continue
        out.append(l)
    return '\n'.join(out), cuts

def widen_text(text):
    """pub(crate) / pub(super) -> pub on items, fields and methods of moved text; `pub(in ..)` left alone"""
    return re.sub(r'\bpub\((?:crate|super)\)', 'pub', text)

def tokens_of(text):
    clean = blank(text)
    return set(re.findall(r'(?<![A-Za-z0-9_$])[A-Za-z_][A-Za-z0-9_]*', clean))

def import_header(cut_texts, src_rel, src_remaining, dest_crate):
    """`use` lines the moved text needs in its destination crate (imports of the source file, remaining source items)"""
    toks = set()
    for t in cut_texts:
        toks |= tokens_of(t)
    src = src_text(src_rel)
    lines = []
    seen = set()
    for cfg, vis, path, local, glob in uses(src, top_only=True):
        if vis.startswith('pub'):
            continue
        p = '::'.join(path)
        if glob:
            stmt = f'use {p}::*;'
        elif local in toks and local != '_':
            stmt = f'use {p}' + (f' as {local};' if path[-1] != local else ';')
        else:
            continue
        stmt = rewrite_paths(expand_group_uses(stmt), dest_crate)
        key = (cfg, stmt)
        if key in seen:
            continue
        seen.add(key)
        lines.append((cfg + '\n' if cfg else '') + stmt)
    # items remaining in the source module
    src_mod = next((n for n, f in M.MOD_FILE.items() if f == src_rel), None)
    remaining_names = set(item_names(src_remaining))
    for name in sorted(remaining_names & toks):
        h = HOMES.home(src_mod, name, dest_crate)
        if h is None:
            continue
        if h[0] == dest_crate:
            continue
        stmt = f'use {M.libname(h[0])}::' + (h[1] + '::' if h[1] else '') + f'{name};'
        if (None, stmt) in seen:
            continue
        seen.add((None, stmt))
        lines.append(stmt)
    return lines


# -------------------------------------------------------------------------------- lifts delivered by other packages
LIFT_SOURCE = {
    'media_operation_refusal': '🎞️media/⚠️turn/🦀️.rs',
    'media_intrinsic': '🎞️media/🛫️intrinsic/🦀️.rs',
    'artifact_inference': '💡️inference/🦀️.rs',
    'inference_gateway': '💡️inference/🚪️gateway/🦀️.rs',
    'plugin_runtime': '⚛️reactor/🏃️runtime/🦀️.rs',
}
SKIP_NAMES = set('Self self crate super std core alloc String Vec Option Result Box Arc Rc Some None Ok Err Default Clone Copy Debug Send Sync Sized Drop Fn FnMut FnOnce Into From TryFrom ToString PartialEq Eq Ord PartialOrd Hash Iterator IntoIterator'.split())

def auto_imports(body, own_src_mod, dest, explicit_locals):
    """`use` lines for names the body uses that are units of the item graph living in another module/crate (replaces `use super::*`)"""
    toks = tokens_of(body)
    own = set(item_names(body))
    lines = []
    for name in sorted(toks):
        if name in SKIP_NAMES or name in own or name in explicit_locals or name in pt.GENERIC:
            continue
        if re.fullmatch(r'[a-z][a-z0-9]{0,12}', name):
            continue
        rows = [r for r in HOMES.by_name.get(name, []) if r['kind'] != 'impl']
        if not rows:
            continue
        h = HOMES.home(own_src_mod, name, dest)
        if h is None or h[0] == dest and h[1] is None:
            if h is None:
                continue
        if h[0] == dest:
            if h[1] is None:
                continue
            lines.append(f'use crate::{h[1]}::{name};')
        else:
            lines.append(f'use {M.libname(h[0])}::' + (h[1] + '::' if h[1] else '') + f'{name};')
    return lines

def transform_lift(rel, dest, widen=True):
    """a lift file delivered by its source owner: `use super::*` -> explicit imports, old `crate::` paths -> final crate paths, pub(crate) -> pub"""
    text = src_text(rel)
    mod = next(m for m, r, st in M.LIFTS_IN[dest] if r == rel)
    if re.search(r'(?m)^use super::\*;\s*$', text):
        src_rel = LIFT_SOURCE.get(mod)
        body = re.sub(r'(?m)^use super::\*;\s*\n', '', text)
        hdr = []
        if src_rel and os.path.exists(INPUT_DIR[0] + '/' + src_rel):
            hdr = import_header([body], src_rel, src_text(src_rel), dest)
        locals_ = {m.group(1) for m in re.finditer(r'(?m)^use [^;]*?(?:as\s+)?(\w+);', body)}
        auto = auto_imports(body, FILE_MOD.get(src_rel) if src_rel else None, dest, locals_)
        head = '\n'.join(l for l in body.split('\n') if l.startswith('//!'))
        rest = '\n'.join(l for l in body.split('\n') if not l.startswith('//!'))
        seen, lines = set(), []
        for l in hdr + auto:
            if l not in seen:
                seen.add(l); lines.append(l)
        text = head + '\n' + '\n'.join(lines) + '\n' + rest
    text = rewrite_paths(expand_group_uses(text), dest)
    return widen_text(text) if widen else text
