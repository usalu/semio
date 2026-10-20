/** 🧭 Post-carve ownership: the live policy owners overlaid with module-root packages that the waves create. */
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { lane } from "../exports-stage-tools/model.ts";

export type CarveRef = { name: string; dir: string };
export class Ownership {
  private policy = loadPolicy();
  private cache = new Map<string, { name: string; dir: string } | null>();
  constructor(public carves: CarveRef[]) { this.carves = [...carves].sort((a, b) => b.dir.length - a.dir.length); }
  owner(file: string): { name: string; dir: string } | null {
    if (this.cache.has(file)) return this.cache.get(file)!;
    const s = ownerOfFile(this.policy, file);
    const base = s ? { name: lane(s.owner)?.name ?? s.names[0]!, dir: s.owner } : null;
    let best = base;
    for (const c of this.carves) if (file.startsWith(c.dir + "/") && (!best || c.dir.length > best.dir.length)) { best = { name: c.name, dir: c.dir }; break; }
    this.cache.set(file, best);
    return best;
  }
  pk(file: string): string { return this.owner(file)?.name ?? "-"; }
}
