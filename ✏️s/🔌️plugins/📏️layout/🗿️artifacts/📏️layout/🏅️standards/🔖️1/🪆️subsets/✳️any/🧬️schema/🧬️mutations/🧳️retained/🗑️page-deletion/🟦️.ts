import type { LayoutSnapshot } from "../../../📸️snapshot/🟦️.ts";
import type { Grant, Receipt } from "../../../🔺️diff/🧳️retained/📋️keyed-plan/🟦️.ts";
import { LayoutTargetLookup } from "../🔎️target/🟦️.ts";

type FrameSource = Pick<LayoutSnapshot["pages"][number]["frames"][number], "id" | "kind"> & { readonly threadNext?: string | null };
type PageSource = Pick<LayoutSnapshot["pages"][number], "id" | "spreadId"> & { readonly frames: readonly FrameSource[] };
type SpreadSource = { readonly id: string; readonly pageIds: readonly string[] };
type Base = { readonly pages: readonly PageSource[]; readonly spreads: readonly SpreadSource[] };
type DeletePage = { readonly id: string };
type Phase = "page" | "threads" | "spread" | "member";
export type DeletePageDecision =
  | { readonly page: number; readonly spread: number; readonly member: number | null }
  | { readonly error: "missing-page"; readonly code: "mutation.target-missing" }
  | { readonly error: "surviving-thread"; readonly code: "mutation.invariant" }
  | { readonly error: "missing-spread"; readonly code: "mutation.target-missing"; readonly page: number };

const noProgress = Object.freeze({ copyBytes: 0, capacityBytes: 0, releaseBytes: 0, done: false });

function funded(grant: Grant, work: number, demand: ReturnType<LayoutTargetLookup["demand"]>) {
  return work >= 1 && grant.maximumItems >= 1 && grant.maximumCopyBytes >= demand.copyBytes
    && grant.maximumCapacityBytes >= demand.capacityBytes && grant.maximumReleaseBytes >= demand.releaseBytes
    && grant.maximumDepth >= demand.depth;
}

/** 🗑️ Resolves an original page deletion without materializing candidate collections. */
export class DeletePagePlan {
  private mutation: DeletePage | undefined;
  private base: Base | undefined;
  private baseBound = false;
  private lookup = new LayoutTargetLookup();
  private phase: Phase = "page";
  private page = 0;
  private spread = 0;
  private survivingPage = 0;
  private survivingFrame = 0;
  private decision: DeletePageDecision | undefined;
  private closing = false;

  result(): DeletePageDecision | undefined { return this.decision; }
  demand() { return this.lookup.demand(); }

  private go(phase: Phase) {
    this.phase = phase;
    this.lookup = new LayoutTargetLookup();
  }

  private find<T>(rows: readonly T[], key: (row: T) => string, target: string, grant: Grant) {
    this.lookup.advance({ identity: 1, count: rows.length, key: index => key(rows[index]!) }, 1, target, grant);
    return this.lookup.result();
  }

  advance(mutation: DeletePage, base: Base, grant: Grant, work: number): Receipt {
    if (this.closing || (this.mutation !== undefined && this.mutation !== mutation) || (this.base !== undefined && this.base !== base)) {
      throw Error("layout.delete-page.source-changed");
    }
    if (this.decision !== undefined) return { ...noProgress, done: true };
    const demand = this.demand();
    if (!funded(grant, work, demand)) return noProgress;
    if (this.mutation === undefined) {
      this.mutation = mutation;
      this.base = base;
    } else if (!this.baseBound) this.baseBound = true;
    else this.advanceBound(mutation, base, grant);
    return { ...noProgress, copyBytes: demand.copyBytes, done: this.decision !== undefined };
  }

  private advanceBound(mutation: DeletePage, base: Base, grant: Grant) {
    switch (this.phase) {
      case "page": {
        const found = this.find(base.pages, page => page.id, mutation.id, grant);
        if (found === undefined) return;
        if (found.index === undefined) this.decision = { error: "missing-page", code: "mutation.target-missing" };
        else {
          this.page = found.index;
          this.go(base.pages[this.page]!.frames.length ? "threads" : "spread");
        }
        return;
      }
      case "threads": this.checkSurvivingThread(base, grant); return;
      case "spread": {
        const found = this.find(base.spreads, spread => spread.id, base.pages[this.page]!.spreadId, grant);
        if (found === undefined) return;
        if (found.index === undefined) this.decision = { error: "missing-spread", code: "mutation.target-missing", page: this.page };
        else {
          this.spread = found.index;
          this.go("member");
        }
        return;
      }
      case "member": {
        const found = this.find(base.spreads[this.spread]!.pageIds, id => id, base.pages[this.page]!.id, grant);
        if (found !== undefined) this.decision = { page: this.page, spread: this.spread, member: found.index ?? null };
      }
    }
  }

  private checkSurvivingThread(base: Base, grant: Grant) {
    if (this.survivingPage === base.pages.length) { this.go("spread"); return; }
    const page = base.pages[this.survivingPage]!;
    if (this.survivingPage === this.page || this.survivingFrame === page.frames.length) {
      this.survivingPage++;
      this.survivingFrame = 0;
      return;
    }
    const frame = page.frames[this.survivingFrame]!;
    if (frame.kind !== "text" || frame.threadNext == null) { this.survivingFrame++; return; }
    const found = this.find(base.pages[this.page]!.frames, frame => frame.id, frame.threadNext, grant);
    if (found === undefined) return;
    if (found.index !== undefined) this.decision = { error: "surviving-thread", code: "mutation.invariant" };
    else { this.survivingFrame++; this.go("threads"); }
  }

  beginClose() { this.closing = true; this.decision = undefined; this.lookup.beginClose(); }
  closeDemand() { return this.lookup.closeDemand(); }
  close(grant: Grant, work: number): Receipt {
    if (!this.closing) throw Error("layout.delete-page.close-not-started");
    if (this.terminalIsEmpty()) return { ...noProgress, done: true };
    const demand = this.closeDemand();
    if (!funded(grant, work, demand)) return noProgress;
    if (!this.lookup.terminalIsEmpty()) this.lookup.close(grant);
    else if (this.base !== undefined) { this.base = undefined; this.baseBound = false; }
    else this.mutation = undefined;
    return { ...noProgress, copyBytes: demand.copyBytes, done: this.terminalIsEmpty() };
  }
  terminalIsEmpty() { return this.closing && this.lookup.terminalIsEmpty() && this.base === undefined && this.mutation === undefined; }
}
