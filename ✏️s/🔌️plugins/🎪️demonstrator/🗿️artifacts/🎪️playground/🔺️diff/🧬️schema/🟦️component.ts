/** 🧬️ Playground diff schema — sparse field delta over the artifact. */

export interface PlaygroundDiff {
  /** @state persistent */
  artifact?: PlaygroundArtifact;
  /** @state persistent */
  schema?: string;
}

export interface PlaygroundArtifact {
  schema: string;
}
