"""🔬️ [nxdeadline] adds or removes [DEBUG] probes in a MIRROR copy of the Nx storage owner and graph store: instrument.py <mirror-root> on|off"""
import sys, pathlib, shutil
root = pathlib.Path(sys.argv[1]); mode = sys.argv[2]
N = root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx"
L = 'globalThis.console.error("[DEBUG] "+(performance.now()/1000).toFixed(1)+"s '
probes = {
 N / "🟦️.ts": [
  ("finally{live=false;await releaseNxGraphNamespacesV1(held.reverse(),primary);}", "finally{live=false;"+L+"storage finally "+'"+held.length);await releaseNxGraphNamespacesV1(held.reverse(),primary);'+L+'storage released");}'),
 ],
 N / "📦️graph/🟦️.ts": [
  ("releasing=(async()=>{const failures=[...await untilAbortedV1(readers,entry.control.signal,()=>[entry.control.signal.reason])];", "releasing=(async()=>{"+L+'ns readers");const failures=[...await untilAbortedV1(readers,entry.control.signal,()=>[entry.control.signal.reason])];'+L+'ns readers closed");'),
  ("await untilAbortedV1(released,control.signal,()=>undefined);", "await untilAbortedV1(released,control.signal,()=>undefined);"+L+'holder exit aborted=" + control.signal.aborted);'),
  ("try{await child!.close();}catch(error){failures.push(error);}", L+'ns child close");try{await child!.close();}catch(error){failures.push(error);}'+L+'ns child closed");'),
  ("finally{const failures=[...await children.readers.close()];", "finally{"+L+'store readers");const failures=[...await children.readers.close()];'+L+'store readers closed");'),
 ],
 N.parent / "📦️budget/📥️opening/🏪️store/🟦️.ts": [
  ("frame.accepting=false;while(frame.pending.size)await Promise.allSettled([...frame.pending]);", "frame.accepting=false;let ndSpins=0;if(frame.pending.size)"+L+'store settle waits "+frame.pending.size+" owner "+frame.owner.id+" failed "+failed);while(frame.pending.size){if(++ndSpins%100000===0)'+L+'store settle spins "+ndSpins);await Promise.allSettled([...frame.pending]);}'),
 ],
}
for path, edits in probes.items():
    backup = path.with_name(path.name + ".nd-orig")
    if mode == "off":
        if backup.exists(): shutil.move(backup, path); print("restored", path.name)
        continue
    if not backup.exists(): shutil.copy2(path, backup)
    text = backup.read_text()
    for old, new in edits:
        if text.count(old) != 1: sys.exit(f"anchor x{text.count(old)}: {old[:60]}")
        text = text.replace(old, new)
    path.write_text(text); print("instrumented", path.name)
