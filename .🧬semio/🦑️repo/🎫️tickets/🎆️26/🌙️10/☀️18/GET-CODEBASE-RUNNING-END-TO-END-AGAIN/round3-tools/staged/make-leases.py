#!/usr/bin/env python3
"""Derives the merged copy of conc-leases from the CURRENT original: conc-harness `trace` (live) already wrapped `acquireResourceLease` and `acquireQueuedResourceLease` in `traceLeaseV1(...)` and renamed the queued
body, so the two anchors of conc-leases' stage.py (the original one-line forms) no longer exist. In the merged copy the public functions stay the traced forms and the lease-order wrappers become private functions the
trace calls (`acquireOrderedResourceLeaseV1`, `acquireOrderedQueuedResourceLeaseV1`); the rest of the set is untouched. The copy lives in C/round3-tools/staged/conc-leases; running its stage.py re-stages from the live files."""
import os, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SOURCE, TARGET = C + "/conc-leases-tools", HERE + "/conc-leases"
shutil.rmtree(TARGET, ignore_errors=True)
shutil.copytree(SOURCE, TARGET, ignore=shutil.ignore_patterns("__pycache__", "backup", "backup-*"))
path = TARGET + "/stage.py"
text = open(path, encoding="utf8").read()


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)


swap('''LEASES=P+"🔒️leases/🟦️.ts"''', '''def merged_acquire():
    w=load("acquire-wrapper.ts.txt").rstrip("\\n")
    head='export async function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{'
    assert w.count(head)==1
    return 'export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return traceLeaseV1(options,()=>acquireOrderedResourceLeaseV1(options));}\\n'+w.replace(head,'async function acquireOrderedResourceLeaseV1(options:LeaseOptions):Promise<ResourceLease>{')
def merged_queued():
    w=load("queued-wrapper.ts.txt").rstrip("\\n")
    head='export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {'
    assert w.count(head)==1
    return 'export function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {return traceLeaseV1(options,()=>acquireOrderedQueuedResourceLeaseV1(options));}\\n'+w.replace(head,'async function acquireOrderedQueuedResourceLeaseV1(options: QueuedLeaseOptions): Promise<ResourceLease> {')+'\\nasync function acquireQueuedResourceLeaseBody(options: QueuedLeaseOptions,guard?:LeaseOrderGuardV1): Promise<ResourceLease> {\\n  options=Object.freeze({...options});'
TRACED_ACQUIRE='export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return traceLeaseV1(options,()=>acquireResourceLeaseBody(options));}'
TRACED_QUEUED='export function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {return traceLeaseV1(options,()=>acquireQueuedResourceLeaseBody(options));}\\nasync function acquireQueuedResourceLeaseBody(options: QueuedLeaseOptions): Promise<ResourceLease> {\\n  options=Object.freeze({...options});'
LEASES=P+"🔒️leases/🟦️.ts"''')
swap(''' ('export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return acquireResourceLeaseBody(options);}',load("acquire-wrapper.ts.txt").rstrip("\\n")),''', ''' (TRACED_ACQUIRE,merged_acquire()),''')
swap(''' ('export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {\\n  options=Object.freeze({...options});',load("queued-wrapper.ts.txt").rstrip("\\n")+'\\nasync function acquireQueuedResourceLeaseBody(options: QueuedLeaseOptions,guard?:LeaseOrderGuardV1): Promise<ResourceLease> {\\n  options=Object.freeze({...options});'),''', ''' (TRACED_QUEUED,merged_queued()),''')
open(path, "w", encoding="utf8").write(text)
print("merged conc-leases: lease-order wrappers sit under conc-harness' traceLeaseV1")

# second pass: catalogue edits that depend on the exact operation lists other sets extend (--json: graph/show/incremental/concurrency/capacity/cache) become list-independent
text = open(path, encoding="utf8").read()
old0 = ''' ('"operations": ["graph", "show", "incremental", "capacity"],','"operations": ["graph", "show", "incremental", "capacity", "leases"],'),'''
assert text.count(old0) == 1
text = text.replace(old0, '')
text = text.replace('patch(OPS+"🔣️catalog.json",[', 'patch(OPS+"🔣️catalog.json",[', 1)
hook = '''patch(OPS+"🔣️catalog.json",['''
assert text.count(hook) == 1
text = text.replace(hook, '''import re as _re
def _json_leases(text):
    m=_re.search(r'("name": "--json",\\n\\s+"value": "none",\\n\\s+"operations": \\[)([^\\]]*)(\\])',text)
    if m is None:raise Drift("catalog: --json flag not found")
    if '"leases"' in m.group(2):return text
    return text[:m.end(2)]+', "leases"'+text[m.end(2):]
_orig_patch=patch
def patch(rel,edits,optional=False):
    if rel==OPS+"🔣️catalog.json" and not optional:
        base=read(rel);text=_json_leases(base)
        for i,(old,new) in enumerate(edits):text=once(text,old,new,"%s edit %d"%(rel,i))
        write(rel,text,base,optional);return
    return _orig_patch(rel,edits,optional)
patch(OPS+"🔣️catalog.json",[''')
open(path, "w", encoding="utf8").write(text)
print("merged conc-leases: --json operations list made list-independent")
