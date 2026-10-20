/** 🟠️ The lease pool evict now phase of registerPlaygroundActionAndPluginSourceTests. */
import type { registerPlaygroundResolutionTests } from "../🔶️playground-resolution/🟦️.ts";

/** 🟡 Registers the lease pool evict now tests of registerPlaygroundActionAndPluginSourceTests. */
export function registerLeasePoolEvictNowTests({ describe, it, createLeasePool, expect }: Pick<ReturnType<typeof registerPlaygroundResolutionTests>, "describe" | "it" | "createLeasePool" | "expect">) {
  describe("LeasePool evictNow (hot-swap reload eviction)", () => {
    it("disposes a fully-released key immediately", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
        { lingerMs: 30_000 },
      );
      const lease = await pool.acquire("url-v1");
      lease.release();
      expect(disposed).toEqual([]);
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });

    it("skips (does not throw) a key with an active lease, matching a reload that hasn't released the old handle yet", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
      );
      const lease = await pool.acquire("url-v1");
      expect(() => pool.evictNow("url-v1")).not.toThrow();
      expect(disposed).toEqual([]);
      lease.release();
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });

    it("treats two cache-busted URLs of the same pluginId as independent keys", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
      );
      const oldLease = await pool.acquire("note.js?v=1");
      const newLease = await pool.acquire("note.js?v=2");
      oldLease.release();
      pool.evictNow("note.js?v=1");
      expect(disposed).toEqual(["value:note.js?v=1"]);
      newLease.release();
      pool.evictNow("note.js?v=2");
      expect(disposed).toEqual(["value:note.js?v=1", "value:note.js?v=2"]);
    });
  });
}
