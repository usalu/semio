/** 🧭️ Architect editor — Trace window: typed twin of `🦀️.rs`'s document-owned EventFeed boundary. */

/** 🧭️ The Trace window reads the whole program audit trail; renderer hosts own temporal labels. */
export interface ArchitectTraceViewModel {
  windowKindId: "architect-trace";
  bodyKey: "architect.trace";
}

export const ARCHITECT_WINDOW_TRACE = "architect-trace" as const;
export const ARCHITECT_BODY_TRACE = "architect.trace" as const;
