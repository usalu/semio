/** 👥️ Generation3dViewPresence — typed twin of `🦀️.rs` / `🔣️.json`. What a co-viewer sees of this
 * viewer: where its read-only preview camera is looking and how it is shading. Hover and selection
 * broadcast separately through the framework's own `PresenceInteraction`. */

import type { Generation3dViewCamera } from "../../🎚️config/🧬️schema/🟦️";

/** 🧬️ Generation3dViewPresence */
export interface Generation3dViewPresence {
  /** @state presence */
  previewCamera: Generation3dViewCamera;
  /** @state presence */
  showMode: string;
}

export type { Generation3dViewCamera };
