/** 📝️ Text representation for `procedural.assembly.snapshot`. */
export type AssemblySnapshotText = string;

/** 🚪️ Refusal of one instance position, the shape `parseAssemblySnapshotText` rejects with. */
export class AssemblySnapshotTextRefusal extends Error {
  constructor(
    readonly at: string,
    readonly why: string,
  ) {
    super(`${at}: ${why}`);
  }
}

/** 📖️ Accepts one serialized `.assembly` document, refusing anything that is not text. */
export function parseAssemblySnapshotText(value: unknown, at = "$"): AssemblySnapshotText {
  if (typeof value !== "string") throw new AssemblySnapshotTextRefusal(at, "value is not a string");
  return value;
}
