import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
let workspace = ticket;
while (!existsSync(join(workspace, ".git"))) {
  const parent = dirname(workspace);
  if (parent === workspace) throw Error("Repository root required");
  workspace = parent;
}

export default {
  root: join(workspace, "🎓️teaching/🏛️architecture/📚️course"),
  cacheDir: join(ticket, "🗑️generated/vitest-cache"),
  test: {
    environment: "node",
    include: ["🧪️tests/📖️assignment-pdfs/🟦️.ts"],
    testTimeout: 30000,
    passWithNoTests: false,
  },
};
