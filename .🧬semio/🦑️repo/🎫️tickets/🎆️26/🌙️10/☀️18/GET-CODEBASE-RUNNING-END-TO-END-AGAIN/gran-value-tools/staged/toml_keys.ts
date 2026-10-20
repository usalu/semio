/** 🔎️ Third-party oracle for the manifest cases: Bun's own TOML parser lists the dependency keys of a manifest. */
import { readFileSync } from "node:fs";
const doc = Bun.TOML.parse(readFileSync(process.argv[2]!, "utf8")) as Record<string, Record<string, unknown> | undefined>;
console.log(JSON.stringify({ dependencies: Object.keys(doc["dependencies"] ?? {}), devDependencies: Object.keys(doc["dev-dependencies"] ?? {}) }));
