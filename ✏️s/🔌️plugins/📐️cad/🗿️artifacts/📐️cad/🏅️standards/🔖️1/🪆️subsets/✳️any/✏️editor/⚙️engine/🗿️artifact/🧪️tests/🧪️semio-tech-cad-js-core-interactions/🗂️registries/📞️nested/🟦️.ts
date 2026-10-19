/** 📞️ The nested interaction call rollback, output mapping and arbitrary nesting tests. */
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { RegistriesTestKit } from "../🧰️kit/🟦️.ts";

/** 🪆 Registers the nested interaction call abort rollback, the child output mapping and the arbitrarily nested session tests. */
export function registerNestedInteractionCallTests(kit: RegistriesTestKit): void {
  const { BrepjsKernel, Model, compileInteraction, createInteractionRuntime, expect, it, mergeInteractionCallOutputs, modelDefinitionInteractionRegistry, parseInteractionSpec, registerInteractionSpec } = kit;
  it("aborting nested interaction.call rolls back the calling transition", async () => {
    const pickChild = parseInteractionSpec({
      schema: "spatial.interaction",
      id: "test.nested.pick",
      version: "1",
      invocation: "callable",
      machine: {
        initial: "pick",
        states: [
          {
            name: "pick",
            selection: { accept: ["face"], multiple: false, prompt: "Pick surface" },
            on: [
              {
                event: "selection.changed",
                transitions: [
                  {
                    target: "committed",
                    effects: [
                      {
                        mutation: "assign",
                        target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
                        value: {
                          kind: "path",
                          root: "event",
                          segments: [
                            { kind: "field", name: "targets" },
                            { kind: "index", index: 0 },
                            { kind: "field", name: "id" },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
          { name: "committed", final: true },
        ],
      },
      commit: { fromStates: ["committed"], operation: { kind: "action", action: "command.finish", params: {} } },
    })!;
    const host = parseInteractionSpec({
      schema: "spatial.interaction",
      id: "test.nested.host",
      version: "1",
      machine: {
        initial: "choose_mode",
        states: [
          {
            name: "choose_mode",
            on: [
              {
                event: "mode.surface",
                transitions: [
                  {
                    target: "committed",
                    effects: [
                      { mutation: "assign", target: { root: "context", segments: [{ kind: "field", name: "constructMode" }] }, value: { kind: "const", value: "surface" } },
                      {
                        mutation: "interaction.call",
                        interaction: "test.nested.pick",
                        outputs: [
                          {
                            target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
                            value: { kind: "path", root: "context", segments: [{ kind: "field", name: "faceId" }] },
                          },
                        ],
                      },
                    ],
                  },
                ],
              },
            ],
          },
          { name: "committed", final: true },
        ],
      },
      commit: { fromStates: ["committed"], operation: { kind: "action", action: "command.finish", params: {} } },
    })!;
    let interactions = modelDefinitionInteractionRegistry();
    interactions = registerInteractionSpec(interactions, compileInteraction(pickChild));
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const rt = createInteractionRuntime(compileInteraction(host), {
      kernel,
      document: { model: new Model(), nodes: [] },
      interactions,
    });
    await rt.send({ kind: "mode.surface" });
    expect(rt.getSnapshot().interactionId).toBe("test.nested.pick");
    rt.cancel();
    const snap = rt.getSnapshot();
    expect(snap.interactionId).toBe("test.nested.host");
    expect(snap.state).toBe("choose_mode");
    expect(snap.context.constructMode).toBeUndefined();
    expect(snap.context.faceId).toBeUndefined();
  });
  it("mergeInteractionCallOutputs maps child context through PathTarget bindings", () => {
    const host: Record<string, unknown> = {};
    mergeInteractionCallOutputs(host, { faceId: "f-42", extra: 1 }, [
      {
        target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
        value: { kind: "path", root: "context", segments: [{ kind: "field", name: "faceId" }] },
      },
    ]);
    expect(host.faceId).toBe("f-42");
  });
  it("interaction.call supports arbitrarily nested interaction sessions", async () => {
    const grandchild = parseInteractionSpec({
      schema: "spatial.interaction",
      id: "test.pick.grandchild",
      version: "1",
      invocation: "callable",
      machine: {
        initial: "go",
        states: [
          { name: "go", on: [{ event: "confirm", transitions: [{ target: "done" }] }] },
          { name: "done", final: true },
        ],
      },
      commit: {
        fromStates: ["done"],
        operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
      },
    })!;
    const child = parseInteractionSpec({
      schema: "spatial.interaction",
      id: "test.pick.child",
      version: "1",
      invocation: "callable",
      machine: {
        initial: "call",
        states: [
          {
            name: "call",
            on: [
              {
                event: "confirm",
                transitions: [
                  {
                    target: "done",
                    effects: [
                      {
                        mutation: "interaction.call",
                        interaction: "test.pick.grandchild",
                        outputs: [
                          {
                            target: { root: "context", segments: [{ kind: "field", name: "token" }] },
                            value: { kind: "const", value: "ok" },
                          },
                        ],
                      },
                    ],
                  },
                ],
              },
            ],
          },
          { name: "done", final: true },
        ],
      },
      commit: {
        fromStates: ["done"],
        operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
      },
    })!;
    const host = parseInteractionSpec({
      schema: "spatial.interaction",
      id: "test.pick.host",
      version: "1",
      machine: {
        initial: "call",
        states: [
          {
            name: "call",
            on: [
              {
                event: "go",
                transitions: [
                  {
                    target: "done",
                    effects: [
                      {
                        mutation: "interaction.call",
                        interaction: "test.pick.child",
                        outputs: [
                          {
                            target: { root: "context", segments: [{ kind: "field", name: "token" }] },
                            value: { kind: "path", root: "context", segments: [{ kind: "field", name: "token" }] },
                          },
                        ],
                      },
                    ],
                  },
                ],
              },
            ],
          },
          { name: "done", final: true },
        ],
      },
      commit: {
        fromStates: ["done"],
        operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
      },
    })!;
    let reg = modelDefinitionInteractionRegistry();
    reg = registerInteractionSpec(reg, compileInteraction(grandchild));
    reg = registerInteractionSpec(reg, compileInteraction(child));
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const rt = createInteractionRuntime(compileInteraction(host), {
      kernel,
      document: { model: new Model(), nodes: [] },
      interactions: reg,
    });
    await rt.send({ kind: "go" });
    let snap = rt.getSnapshot();
    expect(snap.interactionId).toBe("test.pick.child");
    expect(snap.nested?.hostInteractionId).toBe("test.pick.host");
    await rt.send({ kind: "confirm" });
    snap = rt.getSnapshot();
    expect(snap.interactionId).toBe("test.pick.grandchild");
    expect(snap.nested?.hostInteractionId).toBe("test.pick.child");
    await rt.send({ kind: "confirm" });
    snap = rt.getSnapshot();
    expect(snap.interactionId).toBe("test.pick.host");
    expect(snap.state).toBe("done");
    expect(snap.context.token).toBe("ok");
  });
}
