import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const stores: Record<string, string> = JSON.parse(readFileSync(join(here, "store-paths.json"), "utf8"));
const names = Object.keys(stores).filter((name) => !name.startsWith("@types/")).sort((left, right) => right.length - left.length);

/** 🧪 Emulates root-level declarations for out-of-package importers by resolving the undeclared bare names into the dependency store (diagnostic only, never shipped). */
Bun.plugin({
  name: "tst1-emulated-root-declarations",
  setup(build) {
    build.onResolve({ filter: /^[@a-z]/ }, (args) => {
      const name = names.find((candidate) => args.path === candidate || args.path.startsWith(candidate + "/"));
      if (!name) return undefined;
      try { Bun.resolveSync(args.path, dirname(args.importer || here)); console.log("[DEBUG] native", args.path, args.importer); return undefined; } catch {}
      return { path: Bun.resolveSync(args.path, stores[name]!) };
    });
  },
});
