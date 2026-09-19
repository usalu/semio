/** 🔍️ Full audit of react dev launchers vs playground variants: every mismatch (never just the first),
 * with the seed `devLaunchers` row shape and the rendered command/env for each failing variant. */

import { generatePlaygroundRegistry } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";
import { generateLaunchJson } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts";

type Entry = { name?: string; command?: string; env?: Record<string, string>; presentation?: { group?: string; order?: number } };

const repoRoot = process.argv[2]!;
const playgrounds = generatePlaygroundRegistry(repoRoot);
const launch = Bun.JSONC.parse(generateLaunchJson(repoRoot, playgrounds, [])) as { readonly configurations: readonly Entry[] };
const react = launch.configurations.filter((entry) => String(entry.name ?? "").endsWith("⚛️react"));
const wgpu = launch.configurations.filter((entry) => String(entry.name ?? "").endsWith("🧊️wgpu🌐️wasm"));

console.log(`playgrounds=${playgrounds.length} reactLaunchers=${react.length} wgpuLaunchers=${wgpu.length}`);
if (playgrounds.length === 0 || react.length === 0) throw new Error("discovery returned an empty set");

const conformant = (entry: Entry, variant: string, renderer: "react" | "wgpu", _pluginId: string): boolean =>
  !/👤️\d+/u.test(String(entry.name ?? "")) &&
  String(entry.command ?? "").trimEnd().endsWith(`-- ${variant}`) &&
  entry.env?.SEMIO_PLUGIN === variant &&
  entry.env?.SEMIO_RENDERER === renderer &&
  typeof entry.env?.S_OS_PORT === "string";

for (const renderer of ["react", "wgpu"] as const) {
  const pool = renderer === "react" ? react : wgpu;
  const bad = playgrounds
    .map((playground) => ({ playground, matches: pool.filter((entry) => conformant(entry, playground.variant, renderer, playground.pluginId)) }))
    .filter((row) => row.matches.length !== 1);
  console.log(`\n=== ${renderer}: ${bad.length} of ${playgrounds.length} variant(s) without exactly one conformant launcher ===`);
  for (const row of bad) {
    const near = pool.filter((entry) => String(entry.command ?? "").includes(row.playground.variant) || entry.env?.SEMIO_PLUGIN === row.playground.pluginId);
    console.log(`\n- ${row.playground.variant} (plugin=${row.playground.pluginId} app=${row.playground.app ?? "-"} port=${row.playground.ports[renderer]}) matches=${row.matches.length}`);
    for (const entry of near.slice(0, 4)) console.log(`    near: ${entry.name} | ${entry.command} | env=${JSON.stringify(entry.env)}`);
    if (near.length === 0) console.log("    near: (none)");
  }
}

const portUse = new Map<number, string[]>();
for (const playground of playgrounds) {
  for (const renderer of ["react", "wgpu"] as const) {
    const port = playground.ports[renderer];
    portUse.set(port, [...(portUse.get(port) ?? []), `${playground.variant}:${renderer}`]);
  }
  for (const renderer of ["react", "wgpu"] as const) {
    for (const port of playground.userPorts?.[renderer] ?? []) portUse.set(port, [...(portUse.get(port) ?? []), `${playground.variant}:${renderer}:user`]);
  }
}
const collisions = [...portUse.entries()].filter(([, owners]) => owners.length > 1);
console.log(`\n=== ports: ${portUse.size} distinct, ${collisions.length} collision(s) ===`);
for (const [port, owners] of collisions) console.log(`  ${port}: ${owners.join(", ")}`);
