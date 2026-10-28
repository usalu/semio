/** 🧪️ @vitest-environment node */
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { repositoryCachePartitionRootDataV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url)), config = resolve(here, "../../🏗️builder/🌐️vite/🟦️.ts"), root = resolve(here, "../../📦️packages/🟦️typescript"), repository = resolve(root, "../../../../..");

describe("hub admin build configuration", () => {
  for (const loader of ["runner", "native"] as const) {
    it(`loads through Vite's ${loader} config loader without evaluating the whole repository library`, async () => {
      const vite = await import("vite"), saved = { mode: process.env.SEMIO_BUILD_MODE, cache: process.env.SEMIO_VITE_CACHE_DIRECTORY };
      process.env.SEMIO_BUILD_MODE = "prototype";
      process.env.SEMIO_VITE_CACHE_DIRECTORY = join(repositoryCachePartitionRootDataV1(repository, "products"), "hub-admin-config-law");
      try {
        const loaded = await vite.loadConfigFromFile({ command: "build", mode: "prototype" }, config, root, "silent", undefined, loader);
        expect(loaded?.config.base).toBe("/admin/");
        expect(loaded?.config.root).toBe(root);
      } finally {
        for (const [key, value] of [["SEMIO_BUILD_MODE", saved.mode], ["SEMIO_VITE_CACHE_DIRECTORY", saved.cache]] as const) {
          if (value === undefined) delete process.env[key]; else process.env[key] = value;
        }
      }
    });
  }
});
