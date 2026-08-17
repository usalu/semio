import { readFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";
const root = join(import.meta.dir, "../../../../../..", "✏️s/🔌️plugins/🎞️animate");
console.log("packages", readdirSync(join(root, "📦️packages")));
const pkg = join(root, "📦️packages/🟦️typescript/package.json");
console.log(readFileSync(pkg, "utf8"));
const react = join(root, "🎛️apps/🎬️present/📺️renderer/⚛️react/🟦️component.tsx");
console.log("react exists", existsSync(react));
