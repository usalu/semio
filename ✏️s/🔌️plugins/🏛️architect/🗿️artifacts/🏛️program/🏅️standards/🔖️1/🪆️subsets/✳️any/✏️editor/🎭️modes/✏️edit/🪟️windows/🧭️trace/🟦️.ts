/** 🧭️ Architect editor — Trace window: typed twin of `🦀️.rs`'s view boundary. Mirrors
 * `render(program: &ProgramSnapshot) -> UiNode`'s signature — the document-wide audit trail. No
 * config parameter: unlike its four siblings this window reads only the program document, since the
 * audit feed has no per-session view state (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
 * removed the selection-scoped trace chain/impact sections this window used to also carry). */

/** 🧭️ The Trace window's typed view-model — mirrors the Rust `render()` boundary's sole input: the
 * whole program document (read for its `audit_events`/`traces`; rendered as the last 12 events). */
export interface ArchitectTraceViewModel {
  windowKindId: "architect-trace";
  bodyKey: "architect.trace";
}

export const ARCHITECT_WINDOW_TRACE = "architect-trace" as const;
export const ARCHITECT_BODY_TRACE = "architect.trace" as const;
