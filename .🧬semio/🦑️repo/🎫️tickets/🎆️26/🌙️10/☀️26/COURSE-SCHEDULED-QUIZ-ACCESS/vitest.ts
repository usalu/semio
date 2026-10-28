import path from "node:path";
import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import { aliases, core, root, ticket } from "./aliases";

export default defineConfig({
  root,
  cacheDir: path.resolve(ticket, "🗑️generated/vitest"),
  plugins: [react()],
  resolve: { alias: aliases, dedupe: ["react", "react-dom"] },
  test: {
    environment: "jsdom",
    include: [path.resolve(core, "🧪️tests/📇️learner-pages/🟦️.tsx"), path.resolve(core, "🧪️tests/🪜️challenge-views/🟦️.tsx"), path.resolve(core, "🧪️tests/💬️challenge-hint-views/🟦️.tsx"), path.resolve(core, "🧪️tests/⏱️challenge-clock-views/🟦️.tsx"), path.resolve(root, "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️course/🟦️.ts"), path.resolve(core, "🧪️tests/🔓️access/🟦️.tsx"), path.resolve(core, "🧪️tests/🎫️quiz-card-challenges/🟦️.tsx"), path.resolve(core, "🧪️tests/🚏️navigation/🟦️.tsx"), path.resolve(core, "🧪️tests/🏠️home-grid/🟦️.tsx")].map(file => file.replaceAll("\\", "/")),
    setupFiles: [path.resolve(root, "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts"), path.resolve(core, "🧪️tests/🌐️preloaded-locales/🟦️.ts")],
    css: { include: [/🎨️\.css(?:\?|$)/u] },
  },
});
