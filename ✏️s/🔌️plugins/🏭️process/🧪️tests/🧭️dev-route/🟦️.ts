import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { parse } from "smol-toml";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { readGeneratedCatalogProjection } from "@semio-tech/plugin-registry/catalog-view";
import { buildPlaygroundSession } from "@semio-tech/plugin-registry/playground/session";
import { parsePlaygroundBlock } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";
import { parseDevContribution } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/🧬️schema/🟦️.ts";
import { composeSpecificOsCatalogV1 } from "../../../../🧑‍💻dev/🧩️catalog/🟦️.ts";
import fixture from "./🔣️.json";

test("the process editor launches with its authored browser contribution", () => {
  const root = resolve(import.meta.dir, "../../../../..");
  const read = (path: string) => readFileSync(resolve(root, path), "utf8");
  const source = read(fixture.manifest);
  const manifest = parse(source);
  const rows = (manifest.package as { metadata: { semio: { playground: { variant: string; devContribution?: string }[] } } }).metadata.semio.playground;
  const row = rows.find(candidate => candidate.variant === fixture.variant);
  expect(row).toMatchObject({ app: fixture.app, ports: fixture.ports, devContribution: fixture.ownerRoot + "/🧬️schema/🔣️.json" });
  const authored = source.split("[[package.metadata.semio.playground]]")[1]!.split("\n[")[0]!;
  const discovered = parsePlaygroundBlock(authored, "process", dirname(fixture.manifest));
  expect(discovered?.devContribution).toBe(row!.devContribution);
  const contribution = parseDevContribution(JSON.parse(read(discovered!.devContribution!)));
  expect(contribution.ownerRoot).toBe(fixture.ownerRoot);
  expect(read(contribution.browserEntry)).toContain("bootFrameworkOsDev");
  expect(read(contribution.viteConfig)).toContain("specificOsDevBuildInventoryV1");
});

test("the selected editor session admits all four machine catalogs", () => {
  const projection = readGeneratedCatalogProjection(resolve(import.meta.dir, "../../../../🧑‍💻dev/🧩️catalog/🤖️generated"));
  const session = buildPlaygroundSession(fixture.variant, projection);
  expect(projection.playgrounds.find(row => row.variant === fixture.variant)?.devContribution).toBe(fixture.ownerRoot + "/🧬️schema/🔣️.json");
  expect(session.defaultAppId).toBe(fixture.app);
  expect(session.plugins.filter(plugin => plugin.contributes.includes("process.machines")).map(plugin => plugin.pluginId).sort()).toEqual(fixture.extensions);
  expect(session.plugins.find(plugin => plugin.pluginId === "process")?.consumes).toContain("process.machines");
});

test("the browser boots the same process editor and extension closure", () => {
  const { catalog } = composeSpecificOsCatalogV1("http://localhost:6022/", {
    maxBytes: 2097152,
    maxRows: 512,
    maxEdges: 4096,
    maxWork: 65536,
    deadlineMs: performance.now() + 30000,
    now: () => performance.now(),
    cancelled: () => false,
    progress: () => {},
  });
  const boot = resolvePlaygroundBoot(catalog, fixture.variant);
  expect(boot.dependencyErrors).toEqual([]);
  expect(boot.defaultAppId).toBe(fixture.app);
  expect(boot.plugins.map(plugin => plugin.pluginId).sort()).toEqual(["process", ...fixture.extensions]);
});
