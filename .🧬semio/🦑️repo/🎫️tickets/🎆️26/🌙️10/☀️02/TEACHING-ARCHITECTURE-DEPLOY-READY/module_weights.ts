/** ⚖️ One-off production build that records rendered module sizes of the architecture quiz site. */
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { build, type Plugin } from "vite";
import factory from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts";

const ticket = dirname(fileURLToPath(import.meta.url));
const out = join(ticket, "🗑️generated", "module-weights.txt");

const weights: Plugin = {
  name: "module-weights",
  generateBundle(_options, bundle) {
    const rows: string[] = [];
    for (const item of Object.values(bundle)) {
      if (item.type === "asset") {
        const size = typeof item.source === "string" ? item.source.length : item.source.byteLength;
        rows.push(`${String(size).padStart(10)} asset ${item.fileName}`);
        continue;
      }
      rows.push(`${String(item.code.length).padStart(10)} chunk ${item.fileName}`);
      const modules = Object.entries(item.modules)
        .map(([id, meta]) => ({ id, rendered: meta.renderedLength }))
        .sort((left, right) => right.rendered - left.rendered);
      for (const module of modules) {
        if (module.rendered < 500) continue;
        rows.push(`${String(module.rendered).padStart(10)}   ${item.fileName}  ${module.id}`);
      }
    }
    writeFileSync(out, `${rows.join("\n")}\n`);
  },
};

const user = await factory({ command: "build", mode: "production" });
await build({
  ...user,
  plugins: [...(user.plugins ?? []), weights],
  logLevel: "warn",
});
console.log(out);
