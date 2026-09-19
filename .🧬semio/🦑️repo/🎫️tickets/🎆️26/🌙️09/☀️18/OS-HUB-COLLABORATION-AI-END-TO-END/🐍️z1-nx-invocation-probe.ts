#!/usr/bin/env bun
/** 🔎️ Z1 probe: what `bun nx run <target>` (root `package.json` "nx" wrapper) really resolves every
 * `.vscode/launch.json` dev row to — target, selected plugin, selected renderer, port. */
const { resolveNxInvocation } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts");

const cases: { label: string; env: Record<string, string>; argv: string[] }[] = [
  { label: "row 🛠️dev🪐️space⚛️react", env: { S_OS_PORT: "6070", SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react" }, argv: ["run", "workspace:dev", "--", "s"] },
  { label: "row 🛠️dev🪐️space🧊️wgpu🌐️wasm", env: { S_OS_PORT: "6071", SEMIO_PLUGIN: "s", SEMIO_RENDERER: "wgpu" }, argv: ["run", "workspace:dev", "--", "s"] },
  { label: "row 🛠️dev🖥️s👤️1⚛️react", env: { S_OS_PORT: "6072", SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react" }, argv: ["run", "workspace:dev", "--", "s"] },
  { label: "row 🛠️dev🖥️s👤️2⚛️react", env: { S_OS_PORT: "6073", SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react" }, argv: ["run", "workspace:dev", "--", "s"] },
  { label: "row ✏️draw⚛️react", env: { S_OS_PORT: "6088", SEMIO_PLUGIN: "draw", SEMIO_RENDERER: "react" }, argv: ["run", "workspace:dev", "--", "draw"] },
  { label: "row ✏️draw🧊️wgpu🌐️wasm", env: { S_OS_PORT: "6089", SEMIO_PLUGIN: "draw", SEMIO_RENDERER: "wgpu" }, argv: ["run", "workspace:dev", "--", "draw"] },
  { label: "alias :dev + SEMIO_PLUGIN=draw (pre-fix shape)", env: { S_OS_PORT: "6088", SEMIO_PLUGIN: "draw", SEMIO_RENDERER: "react" }, argv: ["run", "@semio-tech/framework-os-dev:dev"] },
  { label: "alias :dev bare (no env)", env: {}, argv: ["run", "@semio-tech/framework-os-dev:dev"] },
  { label: "row 🛠️dev🪐️space⚛️react📦️served", env: { S_OS_PORT: "6070", SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react" }, argv: ["run", "workspace:dev", "--", "s", "served"] },
  { label: "cli workspace:dev -- dag (react)", env: { SEMIO_RENDERER: "react" }, argv: ["run", "workspace:dev", "--", "dag"] },
];

for (const row of cases) {
  for (const key of ["S_OS_PORT", "SEMIO_PLUGIN", "SEMIO_RENDERER", "SEMIO_BUILD_MODE"]) delete process.env[key];
  Object.assign(process.env, row.env);
  try {
    const resolved = resolveNxInvocation(row.argv);
    console.log(`${row.label}\n  nx ${resolved.args.join(" ")}\n  plugin=${resolved.env.SEMIO_PLUGIN} renderer=${resolved.env.SEMIO_RENDERER} port=${resolved.env.S_OS_PORT}${resolved.watch ? ` watch=${resolved.watch}` : ""}`);
  } catch (error) {
    console.log(`${row.label}\n  THREW ${(error as Error).message}`);
  }
}
