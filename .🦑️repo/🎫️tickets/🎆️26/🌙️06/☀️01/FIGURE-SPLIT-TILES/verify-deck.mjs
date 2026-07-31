import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const deckRoot = resolve(ticketDir, "../../../../mit-bestand/präsentation/33.projektetage");
const { execSync } = await import("node:child_process");
execSync("bunx vitest run", {
  cwd: deckRoot,
  stdio: "inherit",
  env: {
    ...process.env,
    VITEST_CONFIG: resolve(ticketDir, "vitest.deck.config.ts"),
  },
});
