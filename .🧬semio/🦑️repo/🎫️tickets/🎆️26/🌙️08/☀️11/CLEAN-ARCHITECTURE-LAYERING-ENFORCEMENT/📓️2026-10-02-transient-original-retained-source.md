# Original Retained Transient Laws

Physical original describe/it bodies immediately before owner relocation. Canonical class rename was already applied. No assertion or test name is changed in the relocation.

```typescript
  describe("ephemeralBox", () => {
    it("stores a function-typed init as the current value (not as a lazy factory)", () => {
      const identity = (id: string) => id;
      const box = ephemeralBox<(id: string) => string>(`test.ephemeralBox.fn.${Math.random()}`, identity);
      expect(typeof box.current).toBe("function");
      expect(box.current("ui.nav.back")).toBe("ui.nav.back");
    });

    it("stores a no-op function init without invoking it", () => {
      let calls = 0;
      const noop = () => {
        calls += 1;
      };
      const box = ephemeralBox<() => void>(`test.ephemeralBox.noop.${Math.random()}`, noop);
      expect(typeof box.current).toBe("function");
      expect(calls).toBe(0);
      box.current();
      expect(calls).toBe(1);
    });

    it("is owned by an isolatable, resettable TransientStore lane", () => {
      const left = new TransientStore();
      const right = new TransientStore();
      const leftBox = left.box("cursor", { x: 1 });
      leftBox.current.x = 2;
      expect(left.box("cursor", { x: 99 })).toBe(leftBox);
      expect(right.box("cursor", { x: 3 }).current.x).toBe(3);

      const oldMap = left.map<string, number>("measurements");
      oldMap.set("width", 42);
      left.reset();
      expect(left.map<string, number>("measurements")).not.toBe(oldMap);
      expect(left.map<string, number>("measurements").size).toBe(0);
      expect(oldMap.get("width")).toBe(42);
    });
  });

```
