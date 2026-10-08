import { readFileSync, writeFileSync } from "node:fs";
const r = "C:/git/semio/";
const sub = (file: string, from: string, to: string) => {
  const s = readFileSync(r + file, "utf8");
  if (!s.includes(from)) throw new Error(`${file}: ${from}`);
  writeFileSync(r + file, s.replace(from, to));
};
const A = "🧰️framework/🔨️modules/🖼️assets/";
sub(A + "🔣️icons/🏗️builder/📽️projection/🟦️.ts", "then run the assets build launch configuration (\\`bun nx run @semio-tech/assets:build\\`)", "then run the assets build dashboard command (\\`@semio-tech/assets:build\\`)");
sub(A + "README.md", "then run the assets build launch configuration (`bun nx run @semio-tech/assets:build`)", "then run the assets build dashboard command (`@semio-tech/assets:build`)");
const P = "🧰️framework/🛍️products/";
sub(P + "🎤️presentation/README.md", "points as nx targets, and `.vscode/🧩️launch.seed.jsonc` as launch configurations.", "points as nx targets, which the dashboard offers as commands (`<project>:<target>`, `bun run dashboard`).");
sub(P + "📓️print/README.md", "entry points as nx targets, and `.vscode/🧩️launch.seed.jsonc` as launch configurations.", "entry points as nx targets, which the dashboard offers as commands (`<project>:<target>`, `bun run dashboard`).");
