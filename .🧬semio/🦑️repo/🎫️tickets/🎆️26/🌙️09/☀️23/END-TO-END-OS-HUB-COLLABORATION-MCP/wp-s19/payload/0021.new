/** 🧩️ The host→guest contributions push as an OWNED unit, driven by the language-neutral fixture
 * `🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push/🔣️.json`.
 *
 * 🏁️ The unit used to await a guest document read to cut the pack by operator reachability, and every
 * refresh that started a push was superseded inside that read (measured live on 2026-09-12: 45 s of
 * console with zero contributions lines, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The pack is now a
 * function of the receiver's `consumes` row and the loaded closure, so the laws are about the UNIT:
 * overlapping refreshes join one crossing and exactly one `setContributions` crosses per
 * `(instanceId, content)`. */

import { describe, expect, it } from "vitest";
import { createContributionsPublisher, type ContributionsPublishOutcome, type ContributionsSessionKey } from "../../🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts";
import fixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push/🔣️.json";

type Scenario = (typeof fixture.scenarios)[number] & { readonly retireInstanceId?: number; readonly nextInstanceId?: number };

type Environment = { readonly generation: string };

/** 🧪️ One run of the unit against a guest crossing the test holds open by hand, so "a refresh starts
 * while the first crossing is in flight" is a state the law reaches deterministically. */
function harness(scenario: Scenario) {
  const pushes: { readonly instanceId: number; readonly json: string }[] = [];
  let releaseCrossing: (() => void) | undefined;
  const crossingGate = new Promise<void>((resolve) => {
    releaseCrossing = resolve;
  });
  const publisher = createContributionsPublisher<Environment>({
    registryGeneration: (environment) => environment.generation,
    buildPack: () => (scenario.packEmpty ? "[]" : fixture.pack),
    install: async (session, json) => {
      pushes.push({ instanceId: session.instanceId, json });
      if (scenario.overlapping) await crossingGate;
    },
  });
  return { publisher, pushes, releaseCrossing: () => releaseCrossing?.() };
}

const sessionKey = (instanceId: number): ContributionsSessionKey => ({ pluginId: fixture.session.pluginId, instanceId });
const environment: Environment = { generation: "boot" };

describe("contributions push unit", () => {
  for (const scenario of fixture.scenarios as readonly Scenario[]) {
    it(scenario.id, async () => {
      const { publisher, pushes, releaseCrossing } = harness(scenario);
      const outcomes: ContributionsPublishOutcome[] = [];
      if (scenario.overlapping) {
        const runs = Array.from({ length: scenario.refreshes }, () => publisher.publish(sessionKey(fixture.session.instanceId), environment));
        releaseCrossing();
        outcomes.push(...(await Promise.all(runs)));
      } else if (scenario.retireInstanceId !== undefined) {
        outcomes.push(await publisher.publish(sessionKey(fixture.session.instanceId), environment));
        publisher.retire(scenario.retireInstanceId);
        outcomes.push(await publisher.publish(sessionKey(scenario.nextInstanceId!), environment));
      } else {
        for (let index = 0; index < scenario.refreshes; index++) outcomes.push(await publisher.publish(sessionKey(fixture.session.instanceId), environment));
      }
      expect(pushes.length, `setContributions crossings for ${scenario.id}`).toBe(scenario.expected.pushes);
      expect(pushes.map((entry) => `${entry.instanceId}::PACK`)).toEqual(scenario.expected.installed);
      expect(outcomes.map((outcome) => outcome.status)).toEqual(scenario.expected.outcomes);
      for (const push of pushes) expect(push.json).toBe(fixture.pack);
    });
  }

  it("cuts the pack once per registry generation and re-cuts when it moves", async () => {
    let cuts = 0;
    const publisher = createContributionsPublisher<Environment>({
      registryGeneration: (environment) => environment.generation,
      buildPack: () => {
        cuts += 1;
        return fixture.pack;
      },
      install: async () => {},
    });
    expect((await publisher.publish(sessionKey(1), { generation: "boot" })).status).toBe("installed");
    expect((await publisher.publish(sessionKey(1), { generation: "boot" })).status).toBe("unchanged");
    expect((await publisher.publish(sessionKey(1), { generation: "one-plugin-more" })).status).toBe("unchanged");
    expect(cuts).toBe(2);
    expect(publisher.installedFor(1)).toBe(fixture.pack);
  });

  it("releases the installed content when the guest crossing throws, so a failure can be retried", async () => {
    let attempts = 0;
    const publisher = createContributionsPublisher<Environment>({
      registryGeneration: (environment) => environment.generation,
      buildPack: () => fixture.pack,
      install: async () => {
        attempts += 1;
        if (attempts === 1) throw new Error("guest instance busy");
      },
    });
    expect((await publisher.publish(sessionKey(3), environment)).status).toBe("failed");
    expect(publisher.installedFor(3)).toBe(null);
    expect((await publisher.publish(sessionKey(3), environment)).status).toBe("installed");
    expect(attempts).toBe(2);
  });
});
