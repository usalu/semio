self.onmessage = async ({ data: { guest, root, lifetime, policy, ids, sequence, wire } }) => {
  try {
    if (typeof WebAssembly.Suspending !== "function" || typeof WebAssembly.promising !== "function") throw Error("Browser worker lacks WebAssembly JSPI");
    const load = (name: string) => import("/@fs" + name);
    const [{ createActorApi }, { decodeActorInstanceLifecycle }, { exercise }, ...commandModules] = await Promise.all([load(guest + "/bridge.js"), load(lifetime), load(root + "/guest-scenarios.ts"), ...wire.map(load)]);
    await exercise(createActorApi, decodeActorInstanceLifecycle, ids, policy, () => BigInt(Math.floor(performance.now() * 1000)), async (url: string) => {
      const response = await fetch(url);
      if (!response.ok) throw Error("Missing core module " + url);
      return WebAssembly.compile(await response.arrayBuffer());
    }, sequence, Object.assign({}, ...commandModules));
    self.postMessage({ ids, command: "ReadConfig" });
  } catch (error) {
    self.postMessage({ error: error instanceof Error ? error.stack || error.message : String(error) });
  }
};
