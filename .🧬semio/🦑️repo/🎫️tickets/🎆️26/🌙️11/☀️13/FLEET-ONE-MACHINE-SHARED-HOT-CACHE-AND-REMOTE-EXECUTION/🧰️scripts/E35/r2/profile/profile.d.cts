/** 🪪️ Digests the receipt-relevant projection of one workspace manifest, or `null` when it is not a manifest object. */
export function manifestProfileSha256V1(path: string, content: string | Uint8Array): string | null;
/** 📜️ The declared policy of the projection. */
export const manifestProfilePolicyV1: Readonly<{ version: 1; fileName: string; excludedTopLevelFields: readonly string[]; maximumBytes: number }>;
