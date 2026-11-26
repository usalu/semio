"""🧮 Observe the paid native evaluation cursor over repeated original continuations."""
from pathlib import Path
import os
import subprocess
import sys
ticket=Path(__file__).resolve().parent
g=ticket/"🗑️generated/bridge-current"
root=Path.cwd()
sdk=root/"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm"
mirror=root/".cache/g3d-native/math-trace"
mirror.mkdir(exist_ok=True)
request=Path("📥️request/🦀️.rs")
for p in sdk.iterdir():
    dest=mirror/p.name
    if p.name==request.parts[0]:
        dest.mkdir(exist_ok=True)
        for child in p.iterdir():
            if child.name!=request.name and not (dest/child.name).exists():(dest/child.name).symlink_to(child)
    elif not dest.exists():dest.symlink_to(p)
marker="let pending=owner.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);"
index=(sdk/request).read_text()
start=index.index(marker,index.index("pub fn evaluate_invoke_json"))
trace="""static COUNT:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);let count=COUNT.fetch_add(1,std::sync::atomic::Ordering::Relaxed);if count%4000==0{if let Some(c)=owner.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref(){eprintln!("[DEBUG] invoke {} yield={} grant={:?} decoded={} slot={:?} req={} root={} lookup={} ready={} active={} fault={} closing={} field={} response={} continuation={}",count,cx.should_yield(),grant,c.decoded,c.slot,c.request.is_some(),c.root.is_some(),c.lookup.is_some(),c.ready_wire.is_some(),c.active.is_some(),c.fault.is_some(),c.closing,c.field,c.response.is_some(),c.continuation.is_some());if let Some(slot)=c.slot{let jobs=evaluation_jobs().lock().unwrap();if let Some((_,r))=jobs.jobs.slot_entry(slot){eprintln!("[DEBUG] owner units={} phase={:?} response={} compact={} cleanup={} id={} cancel={} fault={} posted={} receipt={:?}",r.units,r.progress().phase,r.response.is_some(),r.compact_request.is_some(),r.compact_cleanup,r.continuation_identity.is_some(),r.cancelled,r.fault.is_some(),r.normal_context_posted,r.normal_step_progress());}}}}"""
(mirror/request).write_text(index[:start]+trace+index[start:])
build=(ticket/"relink-current-math-graph.py").read_text().replace('if key not in changes and all(p.parent!=out for p in children.values()):','if key!=target and (out/("lib"+name+".rlib")).exists():\n        built[key]=out/("lib"+name+".rlib")\n        return built[key]\n    if key not in changes and all(p.parent!=out for p in children.values()):')
build=build.replace('cmd=[args[0]', 'if key==target:source=mirror/"🦀️.rs"\n    dest_out=(g/"trace-math-sdk") if key==target else out\n    dest_out.mkdir(exist_ok=True)\n    cmd=[args[0]').replace('"--out-dir",str(out)','"--out-dir",str(dest_out)').replace('built[key]=out/("lib"+name+".rlib")\n    return built[key]\nassert','built[key]=dest_out/("lib"+name+".rlib")\n    return built[key]\nassert').replace('assert build(target).parent==out','build(target)')
exec(build,dict(__file__=str(ticket/"relink-current-math-graph.py"),mirror=mirror))
subprocess.run([sys.executable,str(ticket/"test-current-math-native.py"),"installed_math_evaluate"],env={**os.environ,"MATH_TEST_OUT":"trace-math-test","MATH_TEST_SDK":str(g/"trace-math-sdk/libsemio_os_flow_extensions.rlib")},check=True)
