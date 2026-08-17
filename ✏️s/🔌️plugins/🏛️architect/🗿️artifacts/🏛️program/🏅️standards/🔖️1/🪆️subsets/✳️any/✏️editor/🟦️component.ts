/** ✏️ Architect editor — subset-level typed twin. Re-exports every window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️component.rs`'s `create_architect_app()` stitching every window/mode module together. Namespaced
 * (not a blanket `export *`) even though none of the five windows' exported names collide today —
 * matches the sibling plugins' established convention so a future window addition cannot silently
 * introduce an ambiguous re-export. */

export const ARCHITECT_EDITOR_DIALECT = { artifactKind: "s.architect.program", standard: "1", subset: "*" } as const;

export const ARCHITECT_MODE_EDIT = "edit" as const;
export const ARCHITECT_MODE_REVIEW = "review" as const;
export const ARCHITECT_MODE_REPORT = "report" as const;

export * as adjacencyWindow from "./🎭️modes/✏️edit/🪟️windows/↔️adjacency/🟦️component";
export * as graphWindow from "./🎭️modes/✏️edit/🪟️windows/🕸️graph/🟦️component";
export * as registerWindow from "./🎭️modes/✏️edit/🪟️windows/📋️register/🟦️component";
export * as reportWindow from "./🎭️modes/✏️edit/🪟️windows/📄️report/🟦️component";
export * as traceWindow from "./🎭️modes/✏️edit/🪟️windows/🧭️trace/🟦️component";
