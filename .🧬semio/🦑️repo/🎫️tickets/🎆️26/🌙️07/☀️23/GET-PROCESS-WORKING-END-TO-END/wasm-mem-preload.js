() => {
  window.__mems = [];
  const track = (inst, tag) => { try { for (const [k, v] of Object.entries(inst.exports)) if (v instanceof WebAssembly.Memory) window.__mems.push({ tag, k, mem: new WeakRef(v), at: Math.round(performance.now()) }); } catch {} return inst; };
  const NativeMemory = WebAssembly.Memory;
  WebAssembly.Memory = new Proxy(NativeMemory, { construct(target, args) { const m = new target(...args); window.__mems.push({ tag: "Memory", k: JSON.stringify(args[0]), mem: new WeakRef(m), at: Math.round(performance.now()) }); return m; } });
  window.__isWorker = false;
  const NativeWorker = window.Worker;
  window.__workers = [];
  window.Worker = new Proxy(NativeWorker, { construct(target, args) { window.__workers.push(String(args[0]).slice(-80)); return new target(...args); } });
  const NativeInstance = WebAssembly.Instance;
  WebAssembly.Instance = new Proxy(NativeInstance, { construct(target, args) { return track(new target(...args), "ctor"); } });
  const nativeInstantiate = WebAssembly.instantiate;
  WebAssembly.instantiate = async function (...args) { const r = await nativeInstantiate.apply(this, args); return r instanceof WebAssembly.Instance ? track(r, "inst") : (track(r.instance, "inst"), r); };
  const nativeStreaming = WebAssembly.instantiateStreaming;
  if (nativeStreaming) WebAssembly.instantiateStreaming = async function (...args) { const r = await nativeStreaming.apply(this, args); track(r.instance, "stream"); return r; };
  window.__memReport = () => JSON.stringify([window.__workers, window.__mems.map(m => { const v = m.mem.deref(); return [m.tag, m.k, m.at, v ? Math.round(v.buffer.byteLength / 1048576) : "gc"]; })]);
}
