import { readFileSync } from "node:fs";
for (const f of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) {
  const d = Bun.JSONC.parse(readFileSync(f, "utf8")) as any;
  console.log("##", f, Object.keys(d));
  console.log("configurations", d.configurations.length, "compounds", d.compounds?.length, "inputs", d.inputs?.length);
  const groups = new Map<string, number>();
  const types = new Map<string, number>();
  const placeholders = d.configurations.filter((c: any) => typeof c === "string").length;
  for (const c of d.configurations) { if (typeof c === "string") continue; const g = c.presentation?.group ?? "(none)"; groups.set(g, (groups.get(g) ?? 0) + 1); types.set(c.type ?? "?", (types.get(c.type ?? "?") ?? 0) + 1); }
  console.log("placeholders", placeholders);
  console.log([...groups].sort().map(([k, v]) => `${k}:${v}`).join("  "));
  console.log([...types].map(([k, v]) => `${k}:${v}`).join("  "));
  console.log("compounds:", (d.compounds ?? []).map((c: any) => (typeof c === "string" ? c : c.name)).join(" | "));
  console.log("inputs ids:", (d.inputs ?? []).length, (d.inputs ?? []).slice(0, 3).map((i: any) => typeof i === "string" ? i : i.id));
  console.log("sre:", d.configurations.filter((c: any) => c.serverReadyAction).length);
  if (d.devLaunchers) console.log("devLaunchers", Object.keys(d.devLaunchers).length, "projectLaunchers", Object.keys(d.projectLaunchers ?? {}));
}
