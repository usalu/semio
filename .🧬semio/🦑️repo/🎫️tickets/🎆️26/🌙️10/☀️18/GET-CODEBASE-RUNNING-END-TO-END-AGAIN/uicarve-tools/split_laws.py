import os, re, sys, json
sys.path.insert(0, 'lib')
import carve, rs, peel
from carve import R, UI, rd, wr
W = UI + '/🎯️targets/🧊️wgpu'
spec = carve.Spec('specs/wgpu.json')
def rel(f, t): return os.path.relpath(t, os.path.dirname(f))
# 1. allocator extraction
pp = UI + '/🧪️tests/♻️physical-job-close/🦀️.rs'
ap = UI + '/🧪️tests/♻️physical-job-close/⏱️allocator/🦀️.rs'
s = rd(pp)
if not os.path.exists(ap):
    a = s.index('use std::alloc::{GlobalAlloc, Layout};')
    b = s.index('fn grant(copy: usize, release: usize)')
    block = s[a:b]
    block = block.replace('pub(crate) fn measured', 'pub fn measured')
    wr(ap, '//! ⏱️ Physical allocator witness: counts the bytes a closure births and frees on its thread, the global allocator of the test binary that mounts it.\n\n' + block.rstrip('\n') + '\n')
    s = s[:a] + 'use super::physical_allocator::measured;\n\n' + s[b:]
    wr(pp, s)
    print('allocator extracted')
# helper: companion root text from an owner header
def header_of(key):
    root = spec.root_file(spec.crates[key])
    t = rd(root)
    uses = [t[u.astart:u.end] for u in peel.header_uses(t)]
    return uses
def mounts_of(key):
    root = spec.root_file(spec.crates[key])
    return [(n, p) for n, p, a, pub in carve.mounts(root) if n]
def remove_from(key, names):
    root = spec.root_file(spec.crates[key])
    t = rd(root)
    for it in peel.top_items(t):
        if it.kind == 'mod' and it.name in names:
            t = t.replace(it.text, '')
    wr(root, re.sub(r'\n{3,}', '\n\n', t))
def write_companion(key, owner_key, glob, mounts, extra=''):
    c = spec.crates[key]; root = spec.root_file(c)
    hdr = header_of(owner_key + '-laws')
    txt = '//! 🥣️ ' + c['desc'] + '.\n\n#![cfg(test)]\n\n' + '\n'.join(hdr) + '\n' + extra
    for n, p, cfg in mounts:
        txt += '%s#[path = "%s"]\nmod %s;\n\n' % ((cfg + '\n') if cfg else '', rel(root, p), n)
    wr(root, txt)
# 2. events control laws
ev = {n: p for n, p in mounts_of('events-laws')}
if 'control_commit_tests' in ev:
    write_companion('events-control-laws', 'events', None, [(n, ev[n], '') for n in ('control_commit_tests', 'scrub_press_tests')])
    remove_from('events-laws', {'control_commit_tests', 'scrub_press_tests'})
    print('events control laws split')
# 3. engine scene laws
en = {n: p for n, p in mounts_of('engine-laws')}
if 'retained_select_origin_tests' in en:
    ms = [(n, en[n], '') for n in ('retained_document_hostile_fixtures', 'retained_section_collapse_tests', 'retained_select_origin_tests', 'original_presented_source_tests')]
    ms.insert(0, ('physical_allocator', ap, '#[cfg(not(target_arch = "wasm32"))]'))
    ms.append(('physical_job_close_tests', en['physical_job_close_tests'], '#[cfg(not(target_arch = "wasm32"))]'))
    write_companion('engine-scene-laws', 'engine', None, ms, extra='use semio_framework_ui_wgpu_hosting::host::{ClipboardIoJob, ClipboardIoOperation};\n')
    remove_from('engine-laws', {'retained_document_hostile_fixtures', 'retained_section_collapse_tests', 'retained_select_origin_tests', 'original_presented_source_tests', 'physical_job_close_tests'})
    er = spec.root_file(spec.crates['engine-laws'])
    t = rd(er)
    if 'physical_allocator' not in t:
        t = t.rstrip('\n') + '\n\n#[cfg(not(target_arch = "wasm32"))]\n#[path = "%s"]\nmod physical_allocator;\n' % rel(er, ap)
        wr(er, t)
    # the measuring fragment now names the allocator module
    fp = UI + '/🧪️tests/🔬️targets-wgpu-engine-unit/📐️surface-content-height-and-worker-close-laws/🦀️.rs'
    f = rd(fp)
    wr(fp, f.replace('crate::physical_job_close_tests::measured', 'crate::physical_allocator::measured'))
    print('engine scene laws split')
