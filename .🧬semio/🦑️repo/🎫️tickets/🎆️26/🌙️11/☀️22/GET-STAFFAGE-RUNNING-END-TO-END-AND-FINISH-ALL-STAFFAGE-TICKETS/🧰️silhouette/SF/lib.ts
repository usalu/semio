/** 🗺️ Paths and the file access of the furniture of batch SF. */
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const catalogFile = (slug: string): string => `${S}/🗂️catalog/🛋️furniture/${slug}/🔣️.json`;
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));

/** 📖️ Reads an asset document and the indentation of its file. */
export async function readDoc(arg: string): Promise<{ doc: any; indent: number; path: string }> {
  const path = fileOf(arg);
  const text = await Bun.file(path).text();
  const second = text.split("\n", 2)[1] ?? "";
  return { doc: JSON.parse(text), indent: second.length - second.trimStart().length, path };
}

/** 💾️ Writes an asset document back in the indentation it had. */
export async function writeDoc(path: string, doc: any, indent: number): Promise<void> {
  await Bun.write(path, JSON.stringify(doc, null, indent) + "\n");
}

/** ✏️ The drawn shape `id` of a view of a scale; throws when it is missing. */
export function shapeOf(asset: any, scale: number, view: string, id: string): any {
  const shape = asset.representations[String(scale)].geometry[view].find((s: any) => s.id === id);
  if (shape === undefined) throw new Error(`${asset.id} 1:${scale} ${view}: no shape "${id}"`);
  return shape;
}

/** 🧊️ The solid `id` of a list of the volume of a scale; throws when it is missing. */
export function solidOf(asset: any, scale: number, id: string, list = "body"): any {
  const solid = asset.volumes[String(scale)][list].find((s: any) => s.id === id);
  if (solid === undefined) throw new Error(`${asset.id} 1:${scale} ${list}: no solid "${id}"`);
  return solid;
}

/** 🔁️ Replaces the members of an object in place by `next` (same key order as `next`), so that a repair states the whole shape or solid and is idempotent. */
export function restate(target: any, next: Record<string, unknown>): void {
  for (const key of Object.keys(target)) delete target[key];
  Object.assign(target, next);
}

/** 🧽️ A drawn shape as a plain polygon of the view: keeps the identity and the paint of the shape (id, bone, role, z, filled, tone, region, material, pen) and states `points`; a `solid` placement and every other geometry member are dropped. */
export function asPoly(shape: any, points: readonly (readonly [number, number])[]): void {
  const keep = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "penMm", "hatch", "title"];
  const next: Record<string, unknown> = {};
  for (const key of Object.keys(shape)) if (keep.includes(key)) next[key] = shape[key];
  restate(shape, { ...next, kind: "poly", points: points.map(([x, y]) => [x, y]), closed: true });
}

/** 🏁️ Runs the repair of one asset: reads the catalog file (or `--file <path>`), applies `repair`, writes it back in its indentation when something changed (not with `--dry`) and says so. */
export async function run(slug: string, repair: (asset: any) => void): Promise<void> {
  const at = process.argv.indexOf("--file");
  const { doc, indent, path } = await readDoc(at < 0 ? slug : process.argv[at + 1]!);
  const before = JSON.stringify(doc);
  repair(doc.asset);
  const changed = JSON.stringify(doc) !== before;
  if (changed && !process.argv.includes("--dry")) await writeDoc(path, doc, indent);
  console.log(`${slug}: ${changed ? "repaired" : "already in place"}${process.argv.includes("--dry") ? " (dry)" : ""}`);
}

/** ➕️ States the drawn shape `shape.id` of a view of a scale: replaces the shape of that id, or inserts it after the shape `after` when it is not there yet. */
export function stateShape(asset: any, scale: number, view: string, after: string, shape: Record<string, unknown>): void {
  const list: any[] = asset.representations[String(scale)].geometry[view];
  const at = list.findIndex((s) => s.id === shape.id);
  if (at >= 0) { restate(list[at], shape); return; }
  const anchor = list.findIndex((s) => s.id === after);
  if (anchor < 0) throw new Error(`${asset.id} 1:${scale} ${view}: no shape "${after}"`);
  list.splice(anchor + 1, 0, shape);
}
