import fs from "node:fs";
import path from "node:path";
import { paths, root, ticket, ui } from "./aliases";

const source = path.resolve(ui, "📦️packages/🟦️typescript/tsconfig.json");
const config = JSON.parse(fs.readFileSync(source, "utf8"));
const directory = path.dirname(source);
const generated = path.resolve(ticket, "🗑️generated");
fs.mkdirSync(generated, { recursive: true });
fs.writeFileSync(path.join(generated, "tsconfig.json"), JSON.stringify({
  ...config,
  extends: path.resolve(directory, config.extends),
  compilerOptions: { ...config.compilerOptions, incremental: false, tsBuildInfoFile: undefined, paths },
  include: [...config.include.map((file: string) => path.resolve(directory, file)), path.resolve(root, "🧰️framework/🛍️products/📚️course/🧪️tests/🔓️access/🟦️.tsx")],
}, null, 2));
