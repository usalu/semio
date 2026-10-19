import os, re, sys
sys.path.insert(0, 'lib')
import carve, peel
from carve import UI, rd, wr
W = UI + '/🎯️targets/🧊️wgpu'
# (a) physical_job_close_tests: host mount -> engine-laws (the test binary owning the global allocator witness)
hp = W + '/🏃️host/🦀️.rs'
s = rd(hp)
items = peel.top_items(s)
for it in items:
    if it.kind == 'mod' and it.name == 'physical_job_close_tests':
        keep = '\n'.join(l for l in it.text.split('\n') if re.match(r'\s*//\s*#(end)?region\b', l))
        s = s[:it.start] + keep + s[it.end:]
        wr(hp, s); print('host mount removed')
        break
ep = W + '/⚙️engine/🧪️tests/🥣️companion/🦀️.rs'
e = rd(ep)
if 'physical_job_close_tests' not in e:
    rel = os.path.relpath(UI + '/🧪️tests/♻️physical-job-close/🦀️.rs', os.path.dirname(ep))
    e = e.rstrip('\n') + '\n\n#[cfg(not(target_arch = "wasm32"))]\n#[path = "%s"]\nmod physical_job_close_tests;\n' % rel
    e = e.replace('use semio_framework_ui_wgpu_engine::*;', 'use semio_framework_ui_wgpu_engine::*;\nuse semio_framework_ui_wgpu_hosting::host::{ClipboardIoJob, ClipboardIoOperation};', 1)
    wr(ep, e); print('engine-laws mount added')
tp = UI + '/🧪️tests/🔬️targets-wgpu-engine-unit/📐️surface-content-height-and-worker-close-laws/🦀️.rs'
t = rd(tp)
t2 = t.replace('semio_framework_ui_wgpu_hosting::host::physical_job_close_tests::measured', 'crate::physical_job_close_tests::measured')
if t2 != t: wr(tp, t2); print('engine test measured re-pointed')
# physical-job-close test: super:: imports
pp = UI + '/🧪️tests/♻️physical-job-close/🦀️.rs'
p = rd(pp)
# (b) input-laws: retained hit helpers the unit root used unqualified
ip = W + '/📥️input/🧪️tests/🥣️companion/🦀️.rs'
i = rd(ip)
if 'mounted_layout::hit' not in i:
    i = i.replace('use semio_framework_ui_wgpu_input::*;', 'use semio_framework_ui_wgpu_input::*;\nuse semio_framework_ui_wgpu_mounted_layout::hit::*;', 1)
    wr(ip, i); print('input-laws hit glob added')
