/** 🧫 The store module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export { default as storePresenceSchema } from "../../👥️presence/🧬️schema/🔣️.json" with { type: "json" };
export { default as workerVisibleSurfacesDocument } from "../../👷️worker/🪟️visible-surfaces/🔣️.json" with { type: "json" };
export { default as syncCommandRejectionSchema } from "../../🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json" with { type: "json" };
export const groupHistoryCasesFixtureUrl = new URL("../../🧩️composition/📬️publication/🤝️group/🧪️tests/🧫️fixtures/📚️history-cases/🔣️.json", import.meta.url);
export { default as compositionOpenGenesisFixture } from "../../🧩️composition/🚪️open/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
export { default as storeCommandRejectionFixture } from "./🧫️command-rejection/🔣️.json" with { type: "json" };
export { default as storeNativeIdentitySchema } from "../../🧬️schema/📸️native-identity/🔣️.json" with { type: "json" };
