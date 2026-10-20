// differential for interactivityMountedFrameTransactionFailures: old (snapshot) versus new (live) on the clean inputs and on deterministic mutations of every input
import { test, expect } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";
import { INTERACTIVITY_AUDIT_SHELL_FILE, INTERACTIVITY_AUDIT_FIND_ITEMS_FILE, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_WORLD3D_FILES, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE, INTERACTIVITY_AUDIT_OS_SERVICES_FILE } from "@semio-tech/repo-lib/laws";
import { policyReadRustPolicySource } from "@semio-tech/repo-lib/inventory";
const repoRoot = "/Users/ueli/Documents/semio";
const liveFile = process.env.FT_REAL_FILE!;
const oldFile = process.env.FT_OLD_FILE!;
Bun.plugin({ name: "ft-old-resolve", setup(build) { build.onResolve({ filter: /^\.\.?\// }, (args) => (args.importer === oldFile ? { path: path.resolve(path.dirname(liveFile), args.path) } : undefined)); } });
const oldMod = await import(oldFile);
const newMod = await import(liveFile);
const read = (file: string): string => policyReadRustPolicySource(repoRoot, file);
type Fn = typeof newMod.interactivityMountedFrameTransactionFailures;
  const clean: Parameters<Fn> = [
    read("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"),
    read("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs"),
    read("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs"),
    read("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs"),
    read("🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️.rs"),
    read(INTERACTIVITY_AUDIT_SHELL_FILE) + "\n" + read(INTERACTIVITY_AUDIT_FIND_ITEMS_FILE),
    read(INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE),
    INTERACTIVITY_AUDIT_WORLD3D_FILES.map(read).join("\n"),
    read(INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE),
    read(INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE),
    read(INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE),
    read("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"),
    read("🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs"),
    read("🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs"),
    read("🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📨️scene_slots/🦀️.rs"),
    read("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"),
    read(INTERACTIVITY_AUDIT_OS_SERVICES_FILE),
  ];
let seed = 12345;
const rnd = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296; };
test("old and new agree on clean and mutated inputs", () => {
  const results: string[] = [];
  const inputs: Parameters<Fn>[] = [clean];
  for (let round = 0; round < 60; round += 1) {
    const variant = [...clean] as Parameters<Fn>;
    const k = Math.floor(rnd() * variant.length);
    const text = variant[k] as string;
    const cut = Math.floor(rnd() * Math.max(1, text.length - 400));
    variant[k] = (text.slice(0, cut) + text.slice(cut + 100 + Math.floor(rnd() * 3000))) as never;
    inputs.push(variant);
  }
  inputs.push(clean.map(() => "") as Parameters<Fn>);
  let nonEmpty = 0;
  for (const input of inputs) {
    const a = oldMod.interactivityMountedFrameTransactionFailures(...input);
    const b = newMod.interactivityMountedFrameTransactionFailures(...input);
    expect(b).toEqual(a);
    if (a.length) nonEmpty += 1;
    results.push(String(a.length));
  }
  expect(nonEmpty).toBeGreaterThan(0);
});
