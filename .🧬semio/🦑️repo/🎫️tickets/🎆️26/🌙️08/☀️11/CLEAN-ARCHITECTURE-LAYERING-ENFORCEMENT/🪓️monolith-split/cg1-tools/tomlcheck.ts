// 🧾️ tomlcheck.ts <repo-rel manifest>... : parses each manifest with Bun's TOML reader and reports whether it lists the named dependency.
import {readFileSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const needle = process.env.DEP ?? "semio-framework-job-native-decode";
for (const rel of process.argv.slice(2)) {
  const parsed = Bun.TOML.parse(readFileSync(ROOT + rel, "utf8")) as any;
  const deps = {...(parsed.dependencies ?? {}), ...(parsed["dev-dependencies"] ?? {})};
  console.log("ok", rel.slice(-52), needle in deps ? "has " + needle : "-");
}
