import { readFileSync } from "node:fs";
import TOML from "@iarna/toml";
import { readFile } from "node:fs/promises";
import { dirname, relative, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root = resolve(process.cwd(), "🧰️framework/🔨️modules/🖱️ui/🧬️contract");
const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🧱️owners/🔣️.json"), "utf8").toString());
const framework = resolve(root, "../../..");
const cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
const physical = new Map<string, string>();
for (const owner of [{ name: "document", root: "." }, ...fixture.owners]) {
  const directory = resolve(root, owner.root);
  const manifest = resolve(directory, "📦️packages/🦀️rust/Cargo.toml"), parsed = TOML.parse(readFileSync(manifest, "utf8")) as any;
  for (const mode of [[], [...Object.keys(parsed.features ?? {}).map((n: string) => "feature=" + n), "test"]]) {
  const entries = [resolve(directory, "🦀️.rs"), ...(mode.length ? (parsed.test ?? []).map((t: any) => resolve(dirname(manifest), t.path)) : [])];
  const closure = await readRustSourceClosureV1(entries, framework, [...cfg, ...mode], async (p) => await readFile(p).catch((e: any) => e.code === "ENOENT" ? null : Promise.reject(e)));
  for (const input of closure.filter((i) => i.kind === "source")) {
    const path = resolve(framework, input.path);
    if (relative(directory, path).startsWith("..")) console.log("ESCAPE", owner.name, input.path.slice(-100));
    const prev = physical.get(path);
    if (prev !== undefined && prev !== owner.name) console.log("DUP", owner.name, "vs", prev, input.path.slice(-100));
    physical.set(path, owner.name);
  }
  }
}
console.log("done");
