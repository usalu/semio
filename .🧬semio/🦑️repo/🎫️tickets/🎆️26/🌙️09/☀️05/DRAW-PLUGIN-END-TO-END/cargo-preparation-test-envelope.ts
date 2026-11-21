import { join } from "node:path";
const root = process.cwd();
const { declareBuildTestEnvelopeV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts"));
declareBuildTestEnvelopeV1({ owner: "cargo-source-handoff", maximumElapsedMilliseconds: 180000, capabilities: { artifacts: true } });
