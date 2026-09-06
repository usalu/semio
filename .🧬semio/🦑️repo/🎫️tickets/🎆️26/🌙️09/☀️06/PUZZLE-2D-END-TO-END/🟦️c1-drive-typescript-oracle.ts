/** 🔧 Standalone driver for the puzzle-2d TypeScript third-party oracle adapter. */
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const OWNER = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any";

const adapter = (await import(join(REPO, OWNER, "🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts"))).default;
const ctx = { fixture: (uri: string) => join(REPO, OWNER, uri.split("://")[1]!), scenario: { id: "driver", steps: [] }, repoRoot: REPO } as any;

let failed = 0;
for (const [id, entry] of Object.entries(adapter.scenarios) as [string, any][]) {
  try {
    const outcome = await entry.oracle(ctx);
    console.log(`PASS ${id.padEnd(22)} checked=${outcome.projection.checked} vectors=${outcome.projection.vectors.length}`);
  } catch (error) {
    failed += 1;
    console.log(`FAIL ${id.padEnd(22)}\n${(error as Error).message}`);
  }
}
console.log(`driver: ${failed} scenario(s) failed`);
process.exit(failed === 0 ? 0 : 1);
