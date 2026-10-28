/** 🕊️ The builder drops the inference caches (memoized closures, parsed manifests, observation images, interned rows) as soon as the partial inference is shaped, BEFORE the fact index is patched, packed and written: the caches only save recomputation and the index work needs none of them, so its peak no longer rides on the inference working set. Order recorded through a host that wraps the mock. */
import {expect, test} from "bun:test";
import {mkdtempSync, realpathSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import {createMockHost, ringFiles, seed} from "../../../🧪️tests/🏃️world/🟦️.ts";

test("the inference caches are released after the shaping and before the generation is published, and once more when the load ends", async () => {
  const dir = mkdtempSync(join(realpathSync(tmpdir()), "graph-release-")), order: string[] = [];
  try {
    await seed(dir, ringFiles(6));
    const mock = createMockHost(dir);
    const host = {
      ...mock.host,
      discoverPart: async (...args: Parameters<typeof mock.host.discoverPart>) => { order.push("discover"); return mock.host.discoverPart(...args); },
      say: (key: string, fields: Record<string, string | number>) => { order.push("say:" + key); mock.host.say(key, fields); },
      release: async () => { order.push("release"); },
    };
    await loadGenerationGraphV1(host as never);
    const at = (name: string) => order.indexOf(name);
    expect(at("discover")).toBeGreaterThanOrEqual(0);
    expect(at("release")).toBeGreaterThan(at("discover"));
    expect(at("release")).toBeLessThan(at("say:published"));
    expect(order.filter(name => name === "release").length).toBe(2);
    expect(order.at(-1)).toBe("release");
  } finally { rmSync(dir, {recursive: true, force: true}); }
});
