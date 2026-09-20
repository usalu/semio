/** 🔭️ Isolates which member of `createServer`'s `InlineConfig | ResolvedConfig` parameter the dev
 * freshness suite's literal fails, by asserting the same literal against `InlineConfig` alone. */
import type { InlineConfig } from "vite";
import { semioSourceFreshnessVitePlugins } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";

export const inline: InlineConfig = {
  configFile: false,
  root: "/tmp/sandbox",
  logLevel: "silent",
  cacheDir: "/tmp/sandbox/.vite",
  optimizeDeps: { noDiscovery: true, include: [] },
  server: { host: "127.0.0.1", port: 0, hmr: false, watch: null },
  plugins: semioSourceFreshnessVitePlugins({ repoRoot: "/tmp/sandbox" }),
};
