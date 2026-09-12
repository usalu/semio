import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testTrunkLockfile } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts"));
await testTrunkLockfile(root);
