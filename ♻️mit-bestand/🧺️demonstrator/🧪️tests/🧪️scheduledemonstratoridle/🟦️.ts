type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { demonstratorPaneBootVariants, scheduleDemonstratorIdle } = dependencies;
  type DemonstratorIdleScheduler = any;

  const { describe, expect, it } = vitest;

  //#region 🧪️DemonstratorPaneBootTests
  describe("scheduleDemonstratorIdle", () => {
    it("never enters the idle queue before the minimum delay", () => {
      const delayed: (() => void)[] = [];
      let idleCalls = 0;
      let callbackCalls = 0;
      const scheduler: DemonstratorIdleScheduler = {
        setTimeout: (callback) => {
          delayed.push(callback);
          return 1;
        },
        clearTimeout: () => undefined,
        requestIdleCallback: (callback) => {
          idleCalls += 1;
          callback();
          return 2;
        },
      };
      scheduleDemonstratorIdle(() => {
        callbackCalls += 1;
      }, 1_500, scheduler);
      expect({ idleCalls, callbackCalls }).toEqual({ idleCalls: 0, callbackCalls: 0 });
      delayed.shift()?.();
      expect({ idleCalls, callbackCalls }).toEqual({ idleCalls: 1, callbackCalls: 1 });
    });
  });

  describe("demonstratorPaneBootVariants", () => {
    it("keeps Generator's branded app id while loading the standalone procedural module", () => {
      expect(demonstratorPaneBootVariants("generator")).toEqual({ runtime: "generation3d", manifest: "generator" });
      expect(demonstratorPaneBootVariants("koordinator")).toEqual({ runtime: "koordinator", manifest: "koordinator" });
    });
  });
  //#endregion 🧪️DemonstratorPaneBootTests

}
