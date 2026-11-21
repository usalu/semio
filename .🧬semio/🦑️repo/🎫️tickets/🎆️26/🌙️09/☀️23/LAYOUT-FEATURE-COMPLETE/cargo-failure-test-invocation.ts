import { resolve } from "node:path";

const { declareBuildTestEnvelopeV1 } = await import(resolve("🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts"));
declareBuildTestEnvelopeV1({ owner: "cargo-failure-verification", maximumElapsedMilliseconds: 120000, capabilities: { artifacts: true } });
