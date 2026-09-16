type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, _source: TestSource): Promise<void> {
  const { demonstratorActivationLanes, demonstratorRuntimeComponentIds, mergeDemonstratorActivationReceipts } = dependencies;

  const { describe, expect, it } = vitest;

  //#region 🧪️DemonstratorUnionReceiptTests
  /** 🔑️ A receipt-shaped sha256 — the merge's own validator refuses anything that is not 64 hex digits. */
  const sha = (seed: string): string => seed.repeat(64).slice(0, 64);
  const lane = (name: string, plugins: readonly (readonly [string, string, number])[]) => ({
    lane: name,
    receipt: { schema: "semio.dev.activation/v1", variant: name, profile: "dev", plugins: plugins.map(([pluginId, seed, rebuiltAt]) => ({ pluginId, artifactSha256: sha(seed), rebuiltAt })) },
  });

  describe("demonstratorActivationLanes", () => {
    it("adds a lane only for a pane plugin the primary closure cannot reach", () => {
      expect(demonstratorActivationLanes()).toEqual(["generator", "energy", "fem3d"]);
    });

    it("covers every component of the runtime union across its lanes", () => {
      const ids = demonstratorRuntimeComponentIds();
      expect(ids).toContain("energy");
      expect(ids).toContain("fem");
      expect(new Set(ids).size).toBe(ids.length);
    });
  });

  describe("mergeDemonstratorActivationReceipts", () => {
    it("unions every lane's rows into one sorted generator receipt", () => {
      const merged = mergeDemonstratorActivationReceipts(
        [lane("generator", [["procedural", "b", 7], ["demonstrator", "c", 7]]), lane("energy", [["energy", "d", 9]]), lane("fem3d", [["fem", "e", 9]])],
        ["demonstrator", "energy", "fem", "procedural"],
      );
      expect(merged.schema).toBe("semio.dev.activation/v1");
      expect(merged.variant).toBe("generator");
      expect(merged.profile).toBe("dev");
      expect(merged.plugins.map((row: any) => row.pluginId)).toEqual(["demonstrator", "energy", "fem", "procedural"]);
    });

    it("keeps the earliest rebuiltAt when two lanes agree about one component", () => {
      const merged = mergeDemonstratorActivationReceipts(
        [lane("generator", [["procedural", "b", 12]]), lane("energy", [["procedural", "b", 3], ["energy", "d", 4]])],
        ["energy", "procedural"],
      );
      expect(merged.plugins.find((row: any) => row.pluginId === "procedural").rebuiltAt).toBe(3);
    });

    it("refuses a stale lane that disagrees about one component's artifact", () => {
      expect(() => mergeDemonstratorActivationReceipts(
        [lane("generator", [["procedural", "b", 7]]), lane("energy", [["procedural", "f", 7], ["energy", "d", 7]])],
        ["energy", "procedural"],
      )).toThrow(/Stale Demonstrator activation lane: procedural/);
    });

    it("refuses a union missing one required component", () => {
      expect(() => mergeDemonstratorActivationReceipts([lane("generator", [["demonstrator", "c", 7]])], ["demonstrator", "energy"]))
        .toThrow(/does not contain its exact runtime union — missing: energy; extra: \(none\)/);
    });

    it("refuses a union carrying a component the demonstrator never asked for", () => {
      expect(() => mergeDemonstratorActivationReceipts([lane("generator", [["demonstrator", "c", 7], ["puzzle", "a", 7]])], ["demonstrator"]))
        .toThrow(/does not contain its exact runtime union — missing: \(none\); extra: puzzle/);
    });

    it("refuses a lane directory holding another variant's receipt", () => {
      expect(() => mergeDemonstratorActivationReceipts([{ ...lane("energy", [["energy", "d", 7]]), lane: "fem3d" }], ["energy"]))
        .toThrow(/Demonstrator activation lane fem3d carries a energy receipt/);
    });

    it("refuses a release receipt in a development lane", () => {
      const release = lane("energy", [["energy", "d", 7]]);
      expect(() => mergeDemonstratorActivationReceipts([{ ...release, receipt: { ...release.receipt, profile: "release" } }], ["energy"]))
        .toThrow(/Demonstrator activation lane energy is not a dev activation: release/);
    });
  });
  //#endregion 🧪️DemonstratorUnionReceiptTests

}
