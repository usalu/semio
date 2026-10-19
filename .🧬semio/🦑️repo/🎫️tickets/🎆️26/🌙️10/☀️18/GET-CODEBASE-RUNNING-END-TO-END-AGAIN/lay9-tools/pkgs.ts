import { readFileSync, existsSync } from "node:fs";
export const ROOT = "/Users/ueli/Documents/semio/";
export type Pkg = { name: string; owner: string; sourceOwner: string; exports: Record<string, string | null> };
export function workspacePackages(): Pkg[] {
  const root = JSON.parse(readFileSync(ROOT + "package.json", "utf8"));
  const out: Pkg[] = [];
  for (const w of root.workspaces as string[]) {
    const f = ROOT + w + "/package.json";
    if (!existsSync(f)) continue;
    const j = JSON.parse(readFileSync(f, "utf8"));
    const ex: Record<string, string | null> = {};
    const e = j.exports ?? {};
    for (const [k, v] of Object.entries(e)) ex[k] = typeof v === "string" ? v : typeof v === "object" && v !== null ? (Object.values(v as any).find((x) => typeof x === "string") as string | undefined) ?? null : null;
    out.push({ name: j.name, owner: w, sourceOwner: w.includes("/📦️packages/") ? w.split("/📦️packages/")[0]! : w, exports: ex });
  }
  return out;
}
