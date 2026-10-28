/** 🎚️ Writes the generated UI axis projections of a MIRROR root (the Rust dashboard crates need them to compile): bun axes.ts <mirror-root> */
const root = process.argv[2]!;
if (!root.includes("/sm/") && !root.includes("/tmp/m/")) throw Error("a mirror root is required");
const { readUiAxes } = await import(`${root}/🧰️framework/🔨️modules/🖱️ui/🎚️axes/📥️source/🟦️.ts`);
const { uiAxesTargets } = await import(`${root}/🧰️framework/🔨️modules/🖱️ui/🎚️axes/📋️plan/🟦️.ts`);
const { publishUiAxes } = await import(`${root}/🧰️framework/🔨️modules/🖱️ui/🎚️axes/📤️publication/🟦️.ts`);
const { mkdirSync, writeFileSync } = await import("node:fs");
const { dirname } = await import("node:path");
publishUiAxes(uiAxesTargets(root, readUiAxes(root)), (path: string, text: string) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, text); return true; });
console.log("[DEBUG] axes generated in", root);
