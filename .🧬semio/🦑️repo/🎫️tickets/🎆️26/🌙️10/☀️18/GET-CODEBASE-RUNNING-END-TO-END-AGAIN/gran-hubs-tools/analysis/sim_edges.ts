/** 🧮 Dependency view of manifest texts for the plan simulation: stdin JSON [{path, text}] -> JSON {path: {name, features, deps:[{key,pkg,kind,optional,features,target,path,ws}]}} (Bun TOML). */
const items: { path: string; text: string }[] = JSON.parse(await Bun.stdin.text());
const out: Record<string, any> = {};
for (const { path, text } of items) {
  try {
    const d: any = Bun.TOML.parse(text);
    const deps: any[] = [];
    const push = (kind: string, target: string | null, t: any) => {
      for (const [key, raw] of Object.entries(t ?? {})) {
        const v: any = typeof raw === "string" ? { version: raw } : raw;
        deps.push({ key, pkg: v.package ?? key, kind, target, optional: v.optional === true, features: v.features ?? [], path: v.path ?? null, ws: v.workspace === true });
      }
    };
    const K: Record<string, string> = { dependencies: "normal", "build-dependencies": "build", "dev-dependencies": "dev" };
    for (const [k, kind] of Object.entries(K)) push(kind, null, d[k]);
    for (const [cfg, t] of Object.entries((d.target ?? {}) as any)) for (const [k, kind] of Object.entries(K)) push(kind, cfg, (t as any)[k]);
    out[path] = { name: d.package?.name ?? null, features: d.features ?? {}, deps };
  } catch { out[path] = null; }
}
console.log(JSON.stringify(out));
