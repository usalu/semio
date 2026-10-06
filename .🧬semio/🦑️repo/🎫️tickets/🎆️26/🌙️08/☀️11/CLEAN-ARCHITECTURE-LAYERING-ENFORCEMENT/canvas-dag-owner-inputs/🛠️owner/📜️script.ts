import { existsSync, readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { collectIconShortcodeSources, generateIconShortcodeTables } from "../../🔎️shortcodes/🛠️generator/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";

const controller = new AbortController();
process.once("SIGINT", () => controller.abort()); process.once("SIGTERM", () => controller.abort());

/** 🔎️ Owns the static shortcode bindings and direct original SVG asset identities. */
class GenerateShortcodesScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("generate-shortcodes accepts no arguments");
    const assets = resolve(this.repoRoot, "🧰️framework/🔨️modules/🖼️assets"), source = resolve(assets, "🔣️icons/🔣️shortcodes.json"), schema = JSON.parse(readFileSync(resolve(this.root, "../../🔎️shortcodes/🧬️schema/📇️sources/🔣️.json"), "utf8")), data = JSON.parse(readFileSync(source, "utf8"));
    const errors = validateJsonSchemaSubset(schema, data); if (errors.length) throw Error("shortcode source contract: " + JSON.stringify(errors));
    const directory = resolve(this.root, "../../🔎️shortcodes/🤖️generated"), progress = (directory: string, entries: number) => console.error("[DEBUG] shortcode source directory " + directory + " entries=" + entries);
    const catalog = collectIconShortcodeSources(resolve(assets, "🔣️icons"), this.repoRoot, controller.signal, progress), themed = collectIconShortcodeSources(resolve(assets, "🌱️metabolism/🔣️icons"), this.repoRoot, controller.signal, progress);
    if (catalog.length !== data.catalog.length || new Set(data.catalog).size !== data.catalog.length || !themed.length) throw Error("shortcode catalog identity closure differs");
    for (const id of data.catalog) if (!catalog.some(row => row.key === id)) throw Error("missing shortcode catalog identity: " + id);
    const after = generateIconShortcodeTables({ directory: relative(this.repoRoot, directory).replaceAll("\\", "/"), emoji: data.emoji, catalog, themed }, controller.signal).source, destination = resolve(directory, "🦀️.rs");
    controller.signal.throwIfAborted(); if (!existsSync(destination) || readFileSync(destination, "utf8") !== after) { mkdirSync(dirname(destination), { recursive: true }); writeFileSync(destination, after); }
    console.log("[DEBUG] shortcode sources emoji=" + Object.keys(data.emoji).length + " themed=" + themed.length + " catalog=" + catalog.length);
  }
}

/** 🧪️ Runs the full defining Canvas library cohort without native law filters. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test accepts no arguments");
    await new GenerateShortcodesScript(this.root).run([]);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-ui-canvas"], cwd: this.root, extraArgs: ["--lib", "--no-fail-fast"] }, readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate-shortcodes", GenerateShortcodesScript).register("test", NativeTestScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test" });
