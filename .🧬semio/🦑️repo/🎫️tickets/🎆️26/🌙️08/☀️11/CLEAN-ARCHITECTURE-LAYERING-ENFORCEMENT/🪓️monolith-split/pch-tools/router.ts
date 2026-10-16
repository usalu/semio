// 🛣️ Read-only: does each artifact's TypeScript router list every carved Rust crate of the artifact in its `packages` option (patchRouterPackages), and does the router exist?
import {readFileSync, existsSync} from "node:fs";
import {join, dirname} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
for (const artifact of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const facadeManifest = `${artifact}/📦️packages/🦀️rust/Cargo.toml`;
  const facade = (Bun.TOML.parse(readFileSync(join(ROOT, facadeManifest), "utf8")) as any).package.name as string;
  const ws = Bun.TOML.parse(readFileSync(join(ROOT, artifact, "Cargo.toml"), "utf8")) as any;
  const carved: string[] = [];
  for (const m of ws.workspace.members as string[]) {
    const p = join(ROOT, artifact, m, "Cargo.toml");
    const name = (Bun.TOML.parse(readFileSync(p, "utf8")) as any).package?.name as string;
    if (name && name !== facade && name.startsWith(facade + "-")) carved.push(name);
  }
  const script = `${artifact}/📦️packages/🦀️rust/📜️script.ts`;
  if (!existsSync(join(ROOT, script))) { console.log(`${artifact.split("/").slice(-1)[0]}\tNO-ROUTER\tcarved=${carved.length}`); continue; }
  const text = readFileSync(join(ROOT, script), "utf8");
  const missing = carved.filter(n => !text.includes(`"${n}"`));
  console.log(`${artifact.split("/").slice(-1)[0]}\trouter\tcarved=${carved.length}\tmissing=${missing.length}${missing.length ? " " + missing.slice(0, 3).join(",") : ""}`);
}
