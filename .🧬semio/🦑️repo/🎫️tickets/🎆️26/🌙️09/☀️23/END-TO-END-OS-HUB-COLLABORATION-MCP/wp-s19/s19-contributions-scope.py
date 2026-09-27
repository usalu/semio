#!/usr/bin/env python3
"""✂️ S19 one-off codemod (set `flow-extensions`, host side): host→guest contributions are scoped by the receiver's
`consumes` row alone. The operator-reachability cut is gone (a document was only ever offered the extensions it
already used — measured 2026-09-27: a fresh flow document could never gain a brep/bim/draw node, the flow catalogue
"Extensions" group stayed empty), and with it the per-instance guest document read the push used to await
(`resolveScope`, `readDocumentOperatorScope`, `documentSourcesFromPack`, `reachableKindsFromUnknown`,
`resolveDocumentOperatorKinds`, `exampleArtifactSources`, `documentFlowGraphPresent`, `contributionIsCapabilityPack`).
The pack is a function of (receiver plugin, loaded closure, consumed topics) only. Idempotent.
usage: s19-contributions-scope.py <root>
"""
import json
import os
import re
import sys

root = sys.argv[1]
KERNEL = "🧰️framework/🔨️modules/🎠️kernel"
ENGINE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
HELPER = f"{ENGINE}/🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts"
PUSH_FIXTURE = f"{ENGINE}/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push/🔣️.json"
PUSH_TEST = f"{ENGINE}/🧪️tests/🧩️contributions-push/🟦️.ts"


def path(rel):
    return os.path.join(root, rel)


def read(rel):
    return open(path(rel), encoding="utf-8").read()


def write(rel, text):
    if read(rel) != text:
        open(path(rel), "w", encoding="utf-8").write(text)
        print(f"edited {rel}")


SCOPE = '''/**
 * ✂️ Host→guest contributions cut to exactly the topics the receiver consumes, plus its own.
 *
 * ⚖️ `consumedTopics` is the receiver's `consumes` row in the plugin registry — the one authority for
 * what a plugin acts on — and an empty set forwards NO foreign contribution: passing every pack put
 * `gis`'s 196 400-byte `stdio.artifact-catalog.v1` (a topic no plugin consumes) into all four
 * demonstrator apps and blew their wire admission (measured 2026-09-16: 226 310-byte pack).
 *
 * 🧩️ A consumed topic crosses WHOLE. A palette can only offer an extension the push carried, so the
 * operator-reachability cut this replaces offered a document only the extensions it already used
 * (measured 2026-09-27: a fresh flow document could never gain a brep node, its catalogue's extension
 * group stayed empty), and made the pack depend on a guest document read the push had to await.
 */
export function scopeContributionsJson(
  loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: Pick<PluginManifest, "topicContributions"> }>,
  receiverPluginId: string,
  consumedTopics: readonly string[],
): string {
  const consumed = new Set(consumedTopics);
  const entries: ProgramContributionEntry[] = [];
  for (const entry of loaded) {
    const own = entry.pluginId === receiverPluginId;
    for (const topicContribution of entry.manifest.topicContributions ?? []) {
      if (own || consumed.has(topicContribution.topic)) entries.push({ pluginId: entry.pluginId, topicContribution });
    }
  }
  return JSON.stringify(entries);
}
'''


def kernel(text):
    if "export function scopeContributionsJson(\n  loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: Pick<PluginManifest, \"topicContributions\"> }>,\n  receiverPluginId: string,\n  consumedTopics: readonly string[],\n)" in text:
        return text
    start = text.index("const CONTRIBUTION_KIND_KEYS = ")
    tail = text.index("export function resolveDocumentOperatorKinds(")
    end = text.index("\n}\n", tail) + 3
    return text[:start] + SCOPE + text[end:]


write(f"{KERNEL}/🟦️.ts", kernel(read(f"{KERNEL}/🟦️.ts")))

write(f"{KERNEL}/🧫️fixtures/🔬️scope-contributions/🔣️.json", json.dumps({
    "schema": "semio.kernel.scope-contributions/1",
    "note": "Host→guest contributions pass iff they are the receiver's own or their topic is in the receiver's consumes row; a consumed topic crosses whole, whatever the open document reaches.",
    "loaded": [
        {"pluginId": "procedural", "topic": "flow.extension"},
        {"pluginId": "flow-extension-brep", "topic": "flow.extension"},
        {"pluginId": "flow-extension-bim", "topic": "flow.extension"},
        {"pluginId": "flow-extension-math", "topic": "flow.extension"},
        {"pluginId": "process-extension-wood", "topic": "process.machines"},
        {"pluginId": "cad-extension-aec-building", "topic": "cad.computer"},
        {"pluginId": "sourcing-module-beams", "topic": "sourcing.module"},
        {"pluginId": "imperative-extension-effect", "topic": "imperative.module"},
        {"pluginId": "gis", "topic": "stdio.artifact-catalog.v1"},
    ],
    "cases": [
        {"id": "flow-receives-every-extension", "receiver": "flow", "consumes": ["flow.extension"], "expect": ["procedural", "flow-extension-brep", "flow-extension-bim", "flow-extension-math"]},
        {"id": "procedural-receives-its-own-and-every-extension", "receiver": "procedural", "consumes": ["forms.questionKind", "flow.extension"], "expect": ["procedural", "flow-extension-brep", "flow-extension-bim", "flow-extension-math"]},
        {"id": "demonstrator-receives-its-five-topics-and-never-gis", "receiver": "demonstrator", "consumes": ["forms.questionKind", "process.machines", "cad.computer", "sourcing.module", "flow.extension"], "expect": ["procedural", "flow-extension-brep", "flow-extension-bim", "flow-extension-math", "process-extension-wood", "cad-extension-aec-building", "sourcing-module-beams"]},
        {"id": "imperative-receives-every-module", "receiver": "imperative", "consumes": ["imperative.module"], "expect": ["imperative-extension-effect"]},
        {"id": "a-receiver-consuming-nothing-receives-only-its-own", "receiver": "procedural", "consumes": [], "expect": ["procedural"]},
        {"id": "an-unconsumed-topic-never-crosses", "receiver": "sourcing", "consumes": ["sourcing.module"], "expect": ["sourcing-module-beams"]},
    ],
}, ensure_ascii=False, indent=2) + "\n")

KERNEL_TEST = '''import { describe, expect, it } from "vitest";
import { scopeContributionsJson } from "../../🟦️.ts";
import { dialectFromLegacyExampleAppId, examplesForApp, normalizeManifestExampleRow, normalizeManifestExamples, type PluginManifest } from "../../../🛂️manifest/🟦️.ts";
import fixture from "../../🧫️fixtures/🔬️scope-contributions/🔣️.json";

describe("normalizeManifestExamples", () => {
  it("stamps dialect from a legacy appId so the navbar picker can resolve the row", () => {
    const legacy = { id: "demo", appId: "s.sourcing.curation@1/*#editor", artifactJson: "semio …" };
    const normalized = normalizeManifestExampleRow(legacy);
    expect(normalized.appId).toBeUndefined();
    expect(examplesForApp([normalized], { dialect: dialectFromLegacyExampleAppId("s.sourcing.curation@1/*#editor") }).map((row) => row.id)).toEqual(["demo"]);
  });
  it("leaves rows that already carry dialect unchanged", () => {
    const dialect = { artifactKind: "s.draw.drawing", standard: "1", subset: "*" };
    const row = { id: "demo", dialect, artifactJson: "…" };
    expect(normalizeManifestExamples({ examples: [row] }).examples?.[0]).toEqual(row);
  });
});

describe("scopeContributionsJson", () => {
  const loaded = fixture.loaded.map((entry) => ({ pluginId: entry.pluginId, manifest: { topicContributions: [{ topic: entry.topic, payload: { pluginId: entry.pluginId } }] } as Pick<PluginManifest, "topicContributions"> }));
  for (const row of fixture.cases) {
    it(row.id, () => {
      const scoped = JSON.parse(scopeContributionsJson(loaded, row.receiver, row.consumes)) as { pluginId: string; topicContribution: { topic: string } }[];
      expect(scoped.map((entry) => entry.pluginId)).toEqual(row.expect);
      for (const entry of scoped) expect(entry.pluginId === row.receiver || row.consumes.includes(entry.topicContribution.topic)).toBe(true);
    });
  }
  it("forwards a consumed topic whole, independent of any open document", () => {
    const flow = fixture.cases.find((row) => row.id === "flow-receives-every-extension")!;
    const extensions = fixture.loaded.filter((entry) => entry.topic === "flow.extension").map((entry) => entry.pluginId);
    expect(JSON.parse(scopeContributionsJson(loaded, flow.receiver, flow.consumes)).map((entry: { pluginId: string }) => entry.pluginId)).toEqual(extensions);
  });
});
'''
write(f"{KERNEL}/🧪️tests/🔬️scope-contributions/🟦️.ts", KERNEL_TEST)

HELPER_TEXT = '''// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHelpers/contributions/component.ts
/** @emoji 🧩️ The host→guest contributions push as an owned, per-instance unit — kept in its OWN module
 * (no React, no shell imports) so a law can drive it without pulling the shell's element graph, the way
 * `🏛️ShellHost/🩺️fault` is kept apart from `🏛️ShellHost` itself. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
 */
// #endregion 🧲️Header

/** 🧩️ The session a contributions closure is installed INTO — the receiver, never the contributor. */
export type ContributionsSessionKey = { readonly pluginId: string; readonly instanceId: number };

/** 🧩️ What ONE `publish` call actually did, so a caller logs a decision instead of inferring one. */
export type ContributionsPublishOutcome =
  | { readonly status: "installed"; readonly chars: number }
  | { readonly status: "unchanged" }
  | { readonly status: "empty" }
  | { readonly status: "failed"; readonly reason: string };

/** 🔌️ Everything the publisher needs from its host, as three ports over an `environment` the caller
 * captures per publish (the loaded plugin closure, the host/focused mode, the disabled extensions). */
export type ContributionsPublisherPorts<E> = {
  readonly registryGeneration: (environment: E) => string;
  readonly buildPack: (session: ContributionsSessionKey, environment: E) => string;
  readonly install: (session: ContributionsSessionKey, json: string, environment: E) => Promise<void>;
};

export type ContributionsPublisher<E> = {
  readonly publish: (session: ContributionsSessionKey, environment: E) => Promise<ContributionsPublishOutcome>;
  readonly retire: (instanceId: number) => void;
  readonly installedFor: (instanceId: number) => string | null;
};

/**
 * 🧩️ The host→guest contributions push as its OWN owned, per-instance unit.
 *
 * 🏁️ It used to live inside `ShellHost.refreshUi`, behind that function's refresh-generation guard,
 * and to await a guest document read to cut the pack by operator reachability — so each push was
 * superseded inside its own read and the closure never crossed (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 * The pack now depends on the receiver's `consumes` row and the loaded closure alone, so nothing is
 * awaited before the crossing.
 *
 * The pack is cut once per `(receiver plugin, registry generation)` and installed per
 * `(instanceId, content)`: the content is claimed BEFORE the guest crossing, so two overlapping
 * publishes never push it twice, and released when the crossing throws so a failure can be retried.
 * `retire(instanceId)` forgets what an instance that went away had installed.
 */
export function createContributionsPublisher<E>(ports: ContributionsPublisherPorts<E>): ContributionsPublisher<E> {
  // 📦️ The scoped pack is a quarter-megabyte cut of the whole loaded closure and depends on nothing
  // but `(receiver, registry generation)` — cutting it again on every refresh burned real CPU for a
  // byte-identical answer (71 rebuilds in one 60 s boot, measured).
  const packByKey = new Map<string, string>();
  const installedByInstance = new Map<number, string>();
  return {
    async publish(session, environment) {
      try {
        const packKey = `${session.pluginId}::${ports.registryGeneration(environment)}`;
        let json = packByKey.get(packKey);
        if (json === undefined) {
          json = ports.buildPack(session, environment);
          packByKey.set(packKey, json);
        }
        if (json.length === 0 || json === "[]") return { status: "empty" };
        const displaced = installedByInstance.get(session.instanceId);
        if (displaced === json) return { status: "unchanged" };
        installedByInstance.set(session.instanceId, json);
        try {
          await ports.install(session, json, environment);
        } catch (error) {
          if (displaced === undefined) installedByInstance.delete(session.instanceId);
          else installedByInstance.set(session.instanceId, displaced);
          throw error;
        }
        return { status: "installed", chars: json.length };
      } catch (error) {
        return { status: "failed", reason: error instanceof Error ? error.message : String(error) };
      }
    },
    retire(instanceId) {
      installedByInstance.delete(instanceId);
    },
    installedFor: (instanceId) => installedByInstance.get(instanceId) ?? null,
  };
}
'''
write(HELPER, HELPER_TEXT)

write(PUSH_FIXTURE, json.dumps({
    "format": "semio.shell.contributions-push",
    "version": 1,
    "note": "The contributions push is an owned per-instance unit, not a projection of one refreshUi, and awaits nothing before the crossing: the pack is a function of the receiver's consumes row and the loaded closure. Every scenario drives repeated or overlapping refreshes; the push must land exactly once per (instanceId, content).",
    "session": {"pluginId": "procedural", "instanceId": 7},
    "pack": "[{\"pluginId\":\"flow-extension-brep\"},{\"pluginId\":\"flow-extension-math\"}]",
    "scenarios": [
        {"id": "overlapping-refreshes-install-once", "note": "Refreshes that start while the first crossing is still in flight must not push the same content twice.", "refreshes": 4, "overlapping": True, "packEmpty": False, "expected": {"pushes": 1, "installed": ["7::PACK"], "outcomes": ["installed", "unchanged", "unchanged", "unchanged"]}},
        {"id": "repeat-refresh-installs-once", "note": "Once installed, later refreshes must not re-push the same content into the same instance.", "refreshes": 3, "overlapping": False, "packEmpty": False, "expected": {"pushes": 1, "installed": ["7::PACK"], "outcomes": ["installed", "unchanged", "unchanged"]}},
        {"id": "retired-instance-installs-again-under-a-new-id", "note": "A session switch retires the instance that went away; the next instance installs under its own key.", "refreshes": 1, "overlapping": False, "packEmpty": False, "retireInstanceId": 7, "nextInstanceId": 8, "expected": {"pushes": 2, "installed": ["7::PACK", "8::PACK"], "outcomes": ["installed", "installed"]}},
        {"id": "an-empty-pack-pushes-nothing", "note": "A receiver that consumes nothing the closure contributes gets no crossing at all.", "refreshes": 2, "overlapping": False, "packEmpty": True, "expected": {"pushes": 0, "installed": [], "outcomes": ["empty", "empty"]}},
    ],
}, ensure_ascii=False, indent=2) + "\n")

PUSH_TEST_TEXT = '''/** 🧩️ The host→guest contributions push as an OWNED unit, driven by the language-neutral fixture
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
'''
write(PUSH_TEST, PUSH_TEST_TEXT)

SHELL_HOST = f"{ENGINE}/🧱️elements/🏛️ShellHost/🟦️.tsx"


def shell_host(text):
    def once(old, new):
        nonlocal text
        if new in text and old not in text:
            return
        assert text.count(old) == 1, f"ShellHost anchor count {text.count(old)}: {old[:90]!r}"
        text = text.replace(old, new)

    once("  exampleArtifactSources,\n  examplesForApp,\n  resolveDocumentOperatorKinds,\n  scopeContributionsJson,\n", "  examplesForApp,\n  scopeContributionsJson,\n")
    once("import { createContributionsPublisher, type ContributionsOperatorScope, type ContributionsPublishOutcome, type ContributionsSessionKey } from ", "import { createContributionsPublisher, type ContributionsPublishOutcome, type ContributionsSessionKey } from ")
    start = text.find("/** 📄️ Decode a live document pack for operator-kind reachability — pack value, then UTF-8 of pack/spr. */\nfunction documentSourcesFromPack(")
    if start >= 0:
        end = text.index("\n}\n", start) + 3
        text = text[:start] + text[end + 1:]
    once(
        """   * with zero `[DEBUG] contributions …` lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The unit
   * now lives in {@link createContributionsPublisher}: per `(pluginId, instanceId)`, joined on an
   * unmoved registry generation, cancelled only by `retire` (a session switch), and installed keyed
   * by `(instanceId, content)`.""",
        """   * with zero `[DEBUG] contributions …` lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The unit
   * now lives in {@link createContributionsPublisher} and awaits nothing before the crossing: the pack
   * is cut by the receiver's `consumes` row alone (never by a document read), once per registry
   * generation, and installed keyed by `(instanceId, content)`.""",
    )
    block_start = text.find("      resolveScope: async (session, environment): Promise<ContributionsOperatorScope> => {")
    if block_start >= 0:
        block_end = text.index("      buildPack: (session, kinds, environment) => {", block_start)
        text = text[:block_start] + text[block_end:]
    once("      buildPack: (session, kinds, environment) => {", "      buildPack: (session, environment) => {")
    once(
        "const scopedContributionsJson = scopeContributionsJson(loadedForScope, receiver === undefined ? session.pluginId : programPluginIdV1(receiver), kinds, environment.consumedTopics);",
        "const scopedContributionsJson = scopeContributionsJson(loadedForScope, receiver === undefined ? session.pluginId : programPluginIdV1(receiver), environment.consumedTopics);",
    )
    once("      install: async (session, json, kinds, environment) => {", "      install: async (session, json, environment) => {")
    read_start = text.find("  /** 📄️ The receiver's OWN open document, as an operator scope — a genesis envelope, a plugin that\n")
    if read_start >= 0:
        read_end = text.index("  }, []);\n\n", read_start) + len("  }, []);\n\n")
        text = text[:read_start] + text[read_end:]
    once("[captureProgramEffectOwner, hostMode, readDocumentOperatorScope, registry, resolvedTargetViewState]", "[captureProgramEffectOwner, hostMode, registry, resolvedTargetViewState]")
    once(
        """`framework-os-dev verify catalog`. Beside it, `__semioOsInstalledContributions()` reads the contributions closure the
   * host last pushed through `setContributions` (`<instanceId>::<json>`, {@link createContributionsPublisher}'s
   * `installedKey`) — the witness that a topic-only extension reached its parent. Never defined in a production build. */""",
        """`framework-os-dev verify catalog`. Beside it, `__semioOsInstalledContributions(instanceId)` reads the contributions
   * closure the host last pushed into that instance through `setContributions` ({@link createContributionsPublisher}'s
   * `installedFor`) — the witness that a topic-only extension reached its parent. Never defined in a production build. */""",
    )
    once(
        """    const installed = window as unknown as { __semioOsInstalledContributions?: () => string | null };
    installed.__semioOsInstalledContributions = () => contributionsPublisherRef.current?.installedKey() ?? null;""",
        """    const installed = window as unknown as { __semioOsInstalledContributions?: (instanceId: number) => string | null };
    installed.__semioOsInstalledContributions = (instanceId) => contributionsPublisherRef.current?.installedFor(instanceId) ?? null;""",
    )
    return text


write(SHELL_HOST, shell_host(read(SHELL_HOST)))

BRIDGE = f"{ENGINE}/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts"


def bridge(text):
    def once(old, new):
        nonlocal text
        if new in text and old not in text:
            return
        assert text.count(old) == 1, f"bridge anchor count {text.count(old)}: {old[:90]!r}"
        text = text.replace(old, new)

    once("  fetchPackageDescriptor,\n  reachableKindsFromUnknown,\n  scopeContributionsJson,\n", "  fetchPackageDescriptor,\n  scopeContributionsJson,\n")
    once(
        """/** @emoji 🎛️ The receiver's `consumes` row from this product's generated registry — what scopes a
 * capability pack (a contribution no operator graph can reach). An id the catalog does not list
 * consumes nothing, which forwards no foreign capability pack. */""",
        """/** @emoji 🎛️ The receiver's `consumes` row from this product's generated registry — the one thing that
 * scopes a contributions pack. An id the catalog does not list consumes nothing, which forwards no
 * foreign contribution. */""",
    )
    once(
        """/** @emoji 📦️ One pack-sized contributions payload — receiver plus flow-graph-reachable operators only. */
export function wgpuBuildScopedContributionsPack(
  receiverPluginId: string,
  reachabilityValues: readonly unknown[],
  loadedManifests?: ReadonlyArray<{ readonly pluginId: string; readonly manifest: Pick<PluginManifest, "topicContributions"> }>,""",
        """/** @emoji 📦️ One pack-sized contributions payload — the receiver's own contributions plus every topic it consumes. */
export function wgpuBuildScopedContributionsPack(
  receiverPluginId: string,
  loadedManifests?: ReadonlyArray<{ readonly pluginId: string; readonly manifest: Pick<PluginManifest, "topicContributions"> }>,""",
    )
    once(
        """  const reachableKinds = reachableKindsFromUnknown(reachabilityValues);
  const json = scopeContributionsJson(loaded, receiverPluginId, reachableKinds, consumedTopics);""",
        """  const json = scopeContributionsJson(loaded, receiverPluginId, consumedTopics);""",
    )
    once(
        "  readonly pushScopedContributions: (instanceId: number, appId: string, reachabilityJson: string, viewStateJson: string) => Promise<InvocationResponse>;",
        "  readonly pushScopedContributions: (instanceId: number, appId: string, viewStateJson: string) => Promise<InvocationResponse>;",
    )
    once(
        """  const pushScopedContributions = async (instanceId: number, appId: string, reachabilityJson: string, viewStateJson: string): Promise<InvocationResponse> => {
    let reachability: unknown = [];
    try {
      reachability = JSON.parse(reachabilityJson);
    } catch {
      reachability = [];
    }
    let viewState: unknown = {};""",
        """  const pushScopedContributions = async (instanceId: number, appId: string, viewStateJson: string): Promise<InvocationResponse> => {
    let viewState: unknown = {};""",
    )
    once(
        """    const values = Array.isArray(reachability) ? reachability : [reachability];
    const pack = wgpuBuildScopedContributionsPack(pluginId, values);""",
        """    const pack = wgpuBuildScopedContributionsPack(pluginId);""",
    )
    once(
        "  readonly pushScopedContributions: (instanceId: number, appId: string, reachabilityJson: string, viewStateJson: string) => Promise<string>;",
        "  readonly pushScopedContributions: (instanceId: number, appId: string, viewStateJson: string) => Promise<string>;",
    )
    once(
        "    pushScopedContributions: (instanceId, appId, reachabilityJson, viewStateJson) => handle.pushScopedContributions(instanceId, appId, reachabilityJson, viewStateJson).then(invocationResponseJson),",
        "    pushScopedContributions: (instanceId, appId, viewStateJson) => handle.pushScopedContributions(instanceId, appId, viewStateJson).then(invocationResponseJson),",
    )
    return text


write(BRIDGE, bridge(read(BRIDGE)))

PROGRAM_BRIDGE = f"{ENGINE}/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"


def program_bridge(text):
    def once(old, new):
        nonlocal text
        if new in text and old not in text:
            return
        assert text.count(old) == 1, f"ProgramBridge anchor count {text.count(old)}: {old[:90]!r}"
        text = text.replace(old, new)

    once(
        """    pub async fn push_scoped_contributions(&self, instance_id: u32, app_id: &str, reachability_json: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, String> {
        match &self.backend {
            ProgramBridgeBackend::Js(handle) => push_scoped_contributions_js(handle, instance_id, app_id, reachability_json, view_state_json).await,""",
        """    pub async fn push_scoped_contributions(&self, instance_id: u32, app_id: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, String> {
        match &self.backend {
            ProgramBridgeBackend::Js(handle) => push_scoped_contributions_js(handle, instance_id, app_id, view_state_json).await,""",
    )
    once(
        """async fn push_scoped_contributions_js(handle: &Rc<JsValue>, instance_id: u32, app_id: &str, reachability_json: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, String> {
    let push = get_fn(handle.as_ref(), "pushScopedContributions")?;
    let args = Array::new();
    args.push(&JsValue::from_f64(instance_id as f64));
    args.push(&JsValue::from_str(app_id));
    args.push(&JsValue::from_str(reachability_json));
    args.push(&JsValue::from_str(view_state_json));""",
        """async fn push_scoped_contributions_js(handle: &Rc<JsValue>, instance_id: u32, app_id: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, String> {
    let push = get_fn(handle.as_ref(), "pushScopedContributions")?;
    let args = Array::new();
    args.push(&JsValue::from_f64(instance_id as f64));
    args.push(&JsValue::from_str(app_id));
    args.push(&JsValue::from_str(view_state_json));""",
    )
    return text


write(PROGRAM_BRIDGE, program_bridge(read(PROGRAM_BRIDGE)))

WGPU_SHELL = f"{ENGINE}/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"


def wgpu_shell(text):
    start = text.find("    fn contributions_reachability_json(&self, session: &ActiveSession) -> String {")
    if start >= 0:
        end = text.index("\n    }\n", start) + len("\n    }\n")
        text = text[:start] + text[end:].lstrip("\n")
    old_call = """            let reachability = self.contributions_reachability_json(&session);
            let view_json = serde_json::to_string(&self.live_view_state(&session)).unwrap_or_else(|_| "{}".into());"""
    if old_call in text:
        text = text.replace(old_call, """            let view_json = serde_json::to_string(&self.live_view_state(&session)).unwrap_or_else(|_| "{}".into());""")
    text = text.replace(', "crossings": 1, "reachableChars": reachability.len() })', ', "crossings": 1 })')
    text = text.replace("plugin.push_scoped_contributions(session.instance_id, &session.app.id, &reachability, &view_json)", "plugin.push_scoped_contributions(session.instance_id, &session.app.id, &view_json)")
    assert "contributions_reachability_json" not in text and "reachability" not in text.split("fn push_contributions", 1)[1].split("\n    }\n", 1)[0], "wgpu shell reachability leftovers"
    return text


write(WGPU_SHELL, wgpu_shell(read(WGPU_SHELL)))

WINDOW_FAULT = f"{ENGINE}/🧪️tests/🩺️window-fault/🟦️.ts"


def window_fault(text):
    text = text.replace(
        'import { exampleArtifactSources, reachableKindsFromUnknown, resolveDocumentOperatorKinds, scopeContributionsJson, type PluginManifest } from "@semio-tech/framework";',
        'import { scopeContributionsJson, type PluginManifest } from "@semio-tech/framework";',
    )
    start = text.find('\ndescribe("scopeContributionsJson", () => {')
    if start < 0:
        return text
    end = text.index('\ndescribe("contributions pack crossing", () => {', start)
    close = text.index("\n});\n", end) + len("\n});\n")
    replacement = '''
describe("scopeContributionsJson", () => {
  const manifest = (topic: string, payload: unknown): Pick<PluginManifest, "topicContributions"> =>
    ({ topicContributions: [{ topic, payload }] });
  const loaded = [
    { pluginId: "procedural", manifest: manifest("flow.extension", { operators: [{ kind: "procedural.example" }] }) },
    { pluginId: "flow-extension-brep", manifest: manifest("flow.extension", { operators: [{ kind: "brep.solid.extrude" }, { kind: "brep.curve.polygon" }] }) },
    { pluginId: "flow-extension-bim", manifest: manifest("flow.extension", { operators: [{ kind: "bim.wall" }] }) },
    { pluginId: "gis", manifest: manifest("stdio.artifact-catalog.v1", { catalogJson: "x".repeat(64) }) },
  ];
  it("forwards every contribution of a consumed topic and none of an unconsumed one", () => {
    const scoped = JSON.parse(scopeContributionsJson(loaded, "flow", ["flow.extension"])) as { pluginId: string }[];
    expect(scoped.map((entry) => entry.pluginId)).toEqual(["procedural", "flow-extension-brep", "flow-extension-bim"]);
  });

  it("an empty loaded table serializes as [] and that payload is not installable", async () => {
    const json = scopeContributionsJson([], "procedural", ["flow.extension"]);
    expect(json).toBe("[]");
    const installed: string[] = [];
    const publisher = createContributionsPublisher({
      registryGeneration: () => "fixture",
      buildPack: () => json,
      install: async (_session, pack) => { installed.push(pack); },
    });
    expect(await publisher.publish({ pluginId: "procedural", instanceId: 1 }, undefined)).toEqual({ status: "empty" });
    expect(installed).toEqual([]);
    expect(publisher.installedFor(1)).toBeNull();
  });
});

describe("contributions pack crossing", () => {
  it("the live shell sends one pack-encoded setContributions scoped by consumes alone, never 4 KiB string pages", () => {
    expect(shellSource).not.toContain("publicInvocationStringPages");
    expect(shellSource).not.toContain("resolveDocumentOperatorKinds");
    expect(shellSource).not.toContain("exampleArtifactSources");
    expect(shellSource).not.toContain("readDocumentOperatorScope");
    expect(shellSource).toContain("scopeContributionsJson(loadedForScope, receiver === undefined ? session.pluginId : programPluginIdV1(receiver), environment.consumedTopics)");
    expect(shellSource).toContain('encodeAppCommandInvocation(pluginEntry.handle.pluginId, targetApp, "setContributions", args)');
    expect(shellSource).toContain("pluginEntry.handle.handleCommand(instanceId, wire, environment.targetViewState)");
    expect(shellSource).toContain("page: 0, pageCount: 1");
  });
});
'''
    return text[:start] + replacement + text[close:]


write(WINDOW_FAULT, window_fault(read(WINDOW_FAULT)))

DISPATCH_TEST = f"{ENGINE}/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts"
DISPATCH_FIXTURE = f"{ENGINE}/🧫️fixtures/🔬️wgpu-extension-dispatch/🔣️.json"


def dispatch_fixture(text):
    data = json.loads(text)
    pack = data["laws"]["scopedContributionsPack"]
    pack.pop("expectEmptyGraphPluginIds", None)
    pack.pop("expectManifestJsonPluginIds", None)
    pack["receiverConsumes"] = ["forms.questionKind", "flow.extension"]
    pack["expectPluginIds"] = ["flow-extension-bim", "flow-extension-brep", "flow-extension-math", "procedural"]
    pack["expectConsumingNothingPluginIds"] = ["procedural"]
    return json.dumps(data, ensure_ascii=False, indent=2) + "\n"


write(DISPATCH_FIXTURE, dispatch_fixture(read(DISPATCH_FIXTURE)))


def dispatch_test(text):
    start = text.find('describe("wgpu scoped contributions pack", () => {')
    if start < 0:
        return text
    end = text.index('\ndescribe("wgpu contributions command ingress", () => {', start)
    replacement = '''describe("wgpu scoped contributions pack", () => {
  const manifest = (kind: string) => ({ topicContributions: [{ topic: "flow.extension", payload: { operators: [{ kind }] } }], apps: [], workflows: [] });
  const loaded = [
    { pluginId: "procedural", manifest: manifest("procedural.example") },
    { pluginId: "flow-extension-brep", manifest: manifest("brep.solid.extrude") },
    { pluginId: "flow-extension-bim", manifest: manifest("bim.wall") },
    { pluginId: "flow-extension-math", manifest: manifest("math.vector") },
  ];
  const law = laws.laws.scopedContributionsPack;
  it("sends one body-bounded pack of the receiver plus every contribution of the topics it consumes", () => {
    const pack = wgpuBuildScopedContributionsPack(law.receiverPluginId, loaded, law.receiverConsumes);
    expect(pack).not.toBeNull();
    expect(pack?.crossings).toBe(1);
    expect(pack?.bytes.byteLength).toBeLessThanOrEqual(PUBLIC_INVOCATION_BODY_BYTES);
    expect(PUBLIC_INVOCATION_STRING_BYTES).toBeLessThan(PUBLIC_INVOCATION_BODY_BYTES);
    expect([...pack!.pluginIds].sort()).toEqual(law.expectPluginIds);
  });
  it("sends only the receiver's own contributions when it consumes nothing", () => {
    const pack = wgpuBuildScopedContributionsPack(law.receiverPluginId, loaded, []);
    expect(pack).not.toBeNull();
    expect(pack?.crossings).toBe(1);
    expect([...pack!.pluginIds]).toEqual(law.expectConsumingNothingPluginIds);
  });
});
'''
    return text[:start] + replacement + text[end:]


write(DISPATCH_TEST, dispatch_test(read(DISPATCH_TEST)))

RUNTIME_TEST = f"{ENGINE}/🧪️tests/🔌️plugin-runtime/🟦️.tsx"
write(RUNTIME_TEST, read(RUNTIME_TEST).replace(
    """      /** 📜️ `ops` is the third half of the frame, not decoration: `🏛️ShellHost`'s contributions push
       * reads it (`documentSourcesFromPack` → `resolveDocumentOperatorKinds`) and treats a MISSING
       * one as `{ status: "unresolved", reason: "document-ops-missing" }`, so an adapter that drops
       * it silently unresolves every operator scope. This law therefore pins the text crossing
       * verbatim, not merely that a document was read. */""",
    """      /** 📜️ `ops` is the third half of the frame, not decoration: a reader of the live document gets the
       * pack, the spr AND the op text, so an adapter that drops it silently loses the document's ops.
       * This law therefore pins the text crossing verbatim, not merely that a document was read. */""",
))
