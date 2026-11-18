/** 🩹 In-memory application of comma-separated patches (JSON ops files like humans-ops.py, or default-export .mjs functions) to an asset. */
export async function applyPatch(asset: any, patches: string | undefined): Promise<void> {
  for (const patch of (patches ?? "").split(",").filter((p) => p && p !== "-")) {
    const path = patch.startsWith("/") ? patch : process.cwd() + "/" + patch;
    if (!patch.endsWith(".json")) { (await import(path)).default(asset); continue; }
    const find = (list: any[], id: string) => list.find((x) => x.id === id);
    for (const o of JSON.parse(await Bun.file(path).text())) {
      const list: any[] = o.op.startsWith("shape") ? asset.representations[o.scale].geometry[o.view] : asset.volumes[o.scale].body;
      const key = o.op.startsWith("shape") ? "shape" : "solid";
      if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
      else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
      else list[list.indexOf(find(list, o.id))] = o[key];
    }
  }
}
