/** 🧬️ Playground diff schema — sparse field delta over the artifact. */

export interface PlaygroundDiff {
  /** @state artifact */
  artifact?: PlaygroundArtifact;
  /** @state artifact */
  schema?: string;
}

export interface PlaygroundArtifact {
  schema: string;
}
