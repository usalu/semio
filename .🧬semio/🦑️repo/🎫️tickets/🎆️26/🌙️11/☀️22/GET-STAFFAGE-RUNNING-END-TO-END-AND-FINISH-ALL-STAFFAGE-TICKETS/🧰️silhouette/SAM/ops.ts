/** 🧷️ The repair ops of the assets of batch SAM and how they apply to an asset: `shape`, `shape-add`, `shape-set`, `shape-remove` (a drawn shape of `representations[scale].geometry[view]`), `solid`, `solid-add`, `solid-set`, `solid-remove` (a solid of `volumes[scale][list]`, `list` defaults to `body`), `region-set` (a thermal region), `dims` (declared dimensions) and `set` (a value at a JSON path whose steps are keys, indices or `{ id }` matches). */
export type Op = { readonly op: string; readonly scale?: string; readonly view?: string; readonly list?: string; readonly id?: string; readonly after?: string; readonly shape?: any; readonly solid?: any; readonly set?: any; readonly path?: readonly (string | number | { readonly id: string })[]; readonly value?: unknown };

const at = (list: any[], id: string, what: string): number => {
  const i = list.findIndex((x) => x.id === id);
  if (i < 0) throw new Error(`${what} "${id}" not found`);
  return i;
};

/** 🧷️ Applies an op that is not a plain shape op of the working scale (the greedy repair applies those itself). */
export function applyOther(asset: any, o: Op): void {
  applyOp(asset, o);
}

/** 🧷️ Applies one op to the asset in place; throws when what it names is missing. */
export function applyOp(asset: any, o: Op): void {
  if (o.op.startsWith("shape")) {
    const l = asset.representations[o.scale!].geometry[o.view!] as any[];
    if (o.op === "shape-add") { if (l.some((x) => x.id === o.shape.id)) l[at(l, o.shape.id, "shape")] = o.shape; else l.splice(at(l, o.after!, "shape") + 1, 0, o.shape); }
    else if (o.op === "shape") l[at(l, o.id!, "shape")] = o.shape;
    else if (o.op === "shape-set") Object.assign(l[at(l, o.id!, "shape")], o.set);
    else if (o.op === "shape-remove") l.splice(at(l, o.id!, "shape"), 1);
    else throw new Error(`unknown op ${o.op}`);
  } else if (o.op.startsWith("solid")) {
    const l = asset.volumes[o.scale!][o.list ?? "body"] as any[];
    if (o.op === "solid-add") { if (l.some((x) => x.id === o.solid.id)) l[at(l, o.solid.id, "solid")] = o.solid; else l.splice(o.after === undefined ? l.length : at(l, o.after, "solid") + 1, 0, o.solid); }
    else if (o.op === "solid") l[at(l, o.id!, "solid")] = o.solid;
    else if (o.op === "solid-set") Object.assign(l[at(l, o.id!, "solid")], o.set);
    else if (o.op === "solid-remove") l.splice(at(l, o.id!, "solid"), 1);
    else throw new Error(`unknown op ${o.op}`);
  } else if (o.op === "region-set") Object.assign(asset.thermal.regions[at(asset.thermal.regions, o.id!, "region")], o.set);
  else if (o.op === "dims") Object.assign(asset.dimensions, o.set);
  else if (o.op === "set") {
    let node = asset;
    const steps = o.path!;
    const step = (n: any, s: string | number | { readonly id: string }): any => (typeof s === "object" ? n[at(n, s.id, "item")] : n[s]);
    for (const s of steps.slice(0, -1)) node = step(node, s);
    const last = steps[steps.length - 1]!;
    if (typeof last === "object") node[at(node, last.id, "item")] = o.value; else if (o.value === undefined) delete node[last]; else node[last] = o.value;
  } else throw new Error(`unknown op ${o.op}`);
}

const signature = (o: Op): string => JSON.stringify([o.op.replace(/-add$/u, ""), o.scale, o.view, o.list, o.id ?? o.shape?.id ?? o.solid?.id, o.path, o.op === "shape-set" || o.op === "solid-set" || o.op === "region-set" || o.op === "dims" ? Object.keys(o.set).sort() : undefined]);

/** 🧷️ Merges new ops into a list: an op that names the same target replaces the earlier one in place (a `shape` replaces an earlier `shape-add` of the same id by a `shape-add` of the new shape), others are appended. */
export function mergeOps(list: readonly Op[], added: readonly Op[]): Op[] {
  const out = list.slice();
  for (const o of added) {
    const i = out.findIndex((x) => signature(x) === signature(o));
    if (i < 0) out.push(o);
    else if (out[i]!.op.endsWith("-add") && !o.op.endsWith("-add")) out[i] = { ...out[i]!, [o.op]: (o as any)[o.op] };
    else out[i] = o;
  }
  return out;
}

/** 📥️ The ops of a file, empty when it does not exist. */
export async function readOps(file: string): Promise<Op[]> {
  return (await Bun.file(file).exists()) ? (JSON.parse(await Bun.file(file).text()) as Op[]) : [];
}

/** 📤️ Writes ops one per line. */
export async function writeOps(file: string, ops: readonly Op[]): Promise<void> {
  await Bun.write(file, `[\n${ops.map((o) => JSON.stringify(o)).join(",\n")}\n]\n`);
}

/** 🧷️ Applies a list of ops in order and returns what it skipped: a removal whose target is gone and a shape op whose target a later op of the same list removes are skipped when the list runs a second time on its own result (the list stays idempotent); every other missing target throws. */
export function applyOps(asset: any, ops: readonly Op[]): string[] {
  const skipped: string[] = [];
  const removedLater = (index: number, id: string | undefined, o: Op): boolean => ops.slice(index + 1).some((x) => x.op === "shape-remove" && x.scale === o.scale && x.view === o.view && x.id === id);
  ops.forEach((o, index) => {
    if (o.op.startsWith("shape")) {
      const list = asset.representations[o.scale!].geometry[o.view!] as any[];
      const has = (id: string | undefined): boolean => list.some((x) => x.id === id);
      if ((o.op === "shape" || o.op === "shape-set") && !has(o.id) && removedLater(index, o.id, o)) { skipped.push(`${o.op} ${o.scale} ${o.view} ${o.id}`); return; }
      if (o.op === "shape-remove" && !has(o.id)) { skipped.push(`${o.op} ${o.scale} ${o.view} ${o.id}`); return; }
      if (o.op === "shape-add" && !has(o.shape.id) && o.after !== undefined && !has(o.after)) { list.push(o.shape); skipped.push(`${o.op} ${o.scale} ${o.view} ${o.shape.id} (appended)`); return; }
      if (o.op === "shape-add" && !has(o.shape.id) && o.after === undefined) { list.unshift(o.shape); return; }
    }
    applyOp(asset, o);
  });
  return skipped;
}
