import { fileURLToPath } from "node:url";
import staffageConfig from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts";

export default function configure(environment: Parameters<typeof staffageConfig>[0]) {
  process.env.SEMIO_VITE_CACHE_DIRECTORY = fileURLToPath(new URL("../../../../../../../.cache/b/p/m/phone-staffage", import.meta.url));
  process.env.TEACHING_ARCHITECTURE_STAFFAGE_WATCH = "off";
  console.error("[DEBUG] Staffage Vite configuration loaded");
  const config = staffageConfig(environment);
  return {
    ...config,
    envFile: false,
    optimizeDeps: { ...config.optimizeDeps, include: ["react", "react-dom", "react-dom/client", "react/jsx-runtime", "react/jsx-dev-runtime"] },
    server: { ...config.server, warmup: { clientFiles: ["./🟦️.ts"] } },
  };
}
