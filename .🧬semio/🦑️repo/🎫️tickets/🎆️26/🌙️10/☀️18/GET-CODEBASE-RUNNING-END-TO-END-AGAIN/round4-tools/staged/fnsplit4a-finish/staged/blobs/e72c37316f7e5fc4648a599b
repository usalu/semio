/** 🟠️ The actor instance close receipts phase of registerTests1. */
import type { ActorInstanceCloseRequest, ActorInstanceLifecycleReceipt, ActorInstanceOpenRequest } from "../../../🟦️.ts";
import type { registerActorInstanceCloseFaultTests } from "../🔶️actor-instance-close-fault/🟦️.ts";
import type { TestSource } from "../🟢️shared/🟦️.ts";

/** 🟡 Registers the actor instance close receipts tests of registerTests1. */
export function registerActorInstanceCloseReceiptsTests({ it, source, expect, actorInstanceLifetimeEquals, actorInstanceCloseReceiptMatches, actorInstanceCapturedReceiptMatches, actorInstanceLifecycleReceiptEquals }: Pick<ReturnType<typeof registerActorInstanceCloseFaultTests>, "it" | "expect" | "actorInstanceLifetimeEquals" | "actorInstanceCloseReceiptMatches" | "actorInstanceCapturedReceiptMatches" | "actorInstanceLifecycleReceiptEquals"> & { readonly source: TestSource }) {
  it("actor instance close receipts reject reused IDs and premature terminal messages", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const prior = { ...fixture.reopen.prior, activationGeneration: BigInt(fixture.reopen.prior.activationGeneration), guestLifetime: BigInt(fixture.reopen.prior.guestLifetime) };
    const current = { ...fixture.reopen.current, activationGeneration: BigInt(fixture.reopen.current.activationGeneration), guestLifetime: BigInt(fixture.reopen.current.guestLifetime) };
    const request: ActorInstanceCloseRequest = { kind: "close", lifetime: current, requestSequence: 9 };
    const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: current, requestSequence: 9, closeGeneration: 13n };
    expect(actorInstanceLifetimeEquals(prior, current)).toBe(fixture.reopen.oldRequestAccepted);
    expect(actorInstanceCloseReceiptMatches(request, null, { ...accepted, lifetime: prior })).toBe(fixture.reopen.oldReceiptAccepted);
    expect(actorInstanceCloseReceiptMatches(request, null, { ...accepted, kind: "retired" })).toBe(false);
    expect(actorInstanceCloseReceiptMatches(request, null, accepted)).toBe(true);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired" })).toBe(true);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired", closeGeneration: 12n })).toBe(false);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired", requestSequence: 8 })).toBe(false);
    const open: ActorInstanceOpenRequest = { kind: "open", activationGeneration: current.activationGeneration, instanceId: current.instanceId, requestSequence: 8 };
    const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: current, requestSequence: 8 };
    expect(actorInstanceCapturedReceiptMatches(open, captured)).toBe(true);
    expect(actorInstanceCapturedReceiptMatches(open, accepted)).toBe(false);
    expect(actorInstanceCapturedReceiptMatches(open, { ...captured, requestSequence: 7 })).toBe(false);
    expect(actorInstanceCapturedReceiptMatches(open, { ...captured, lifetime: { ...current, activationGeneration: current.activationGeneration + 1n } })).toBe(false);
    expect(actorInstanceLifecycleReceiptEquals(captured, { ...captured })).toBe(true);
    expect(actorInstanceLifecycleReceiptEquals(captured, { ...captured, lifetime: prior })).toBe(false);
    expect(actorInstanceLifecycleReceiptEquals(accepted, { ...accepted, kind: "retired" })).toBe(false);
    expect(actorInstanceLifecycleReceiptEquals(accepted, { ...accepted, closeGeneration: 14n })).toBe(false);
  });
}
