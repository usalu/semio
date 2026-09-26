/** 🔁️ The declared all-plugin rebuild chain is well formed, builds each component once for describe AND staging, proves
 * its `s` staging before the catalog, and selects contiguous step ranges. */
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { readRebuildChain, REBUILD_STAGES, selectRebuildSteps } from "../../🔁️rebuild/🟦️.ts";

describe("rebuild-all chain", () => {
  it("declares descriptors, registry, guests and catalog in dependency order", () => {
    const chain = readRebuildChain();
    expect(chain.map((step) => step.id)).toEqual(["components", "generate", "check", "activate-s", "verify-s", "publish-catalog"]);
    expect([...new Set(chain.map((step) => step.stage))]).toEqual([...REBUILD_STAGES]);
    expect(chain[0]!.command.slice(0, 5)).toEqual(["nx", "run-many", "-t", "describe", "materialize-dev"]);
    expect(chain.find((step) => step.id === "verify-s")?.command.slice(-2)).toEqual(["--variant", "s"]);
    expect(chain.find((step) => step.id === "publish-catalog")?.command.slice(-2)).toEqual(["--packages", "all"]);
  });

  it("selects the contiguous range --from/--to name and refuses unknown or inverted bounds", () => {
    const chain = readRebuildChain();
    expect(selectRebuildSteps(chain, []).map((step) => step.id)).toEqual(chain.map((step) => step.id));
    expect(selectRebuildSteps(chain, ["--to", "verify-s"]).map((step) => step.id)).toEqual(["components", "generate", "check", "activate-s", "verify-s"]);
    expect(selectRebuildSteps(chain, ["--from", "generate", "--to", "check"]).map((step) => step.id)).toEqual(["generate", "check"]);
    expect(() => selectRebuildSteps(chain, ["--from", "check", "--to", "generate"])).toThrow(/after --to/);
    expect(() => selectRebuildSteps(chain, ["--to", "nowhere"])).toThrow(/names no step/);
    expect(() => selectRebuildSteps(chain, ["--to", "check", "--to", "generate"])).toThrow(/usage/);
    expect(() => selectRebuildSteps(chain, ["--from"])).toThrow(/usage/);
  });

  it("refuses a chain whose stages run backwards or repeat a step", () => {
    const root = mkdtempSync(join(tmpdir(), "rebuild-chain-"));
    const write = (steps: unknown[]): string => {
      const path = join(root, `${steps.length}-${Math.random()}.json`);
      writeFileSync(path, JSON.stringify({ schema: "semio.plugin-registry.rebuild-chain/v1", steps }));
      return path;
    };
    const describeStep = { id: "components", stage: "descriptors", command: ["nx", "run-many", "-t", "describe", "materialize-dev"] };
    const catalogStep = { id: "publish-catalog", stage: "catalog", command: ["nx", "run", "os-hub:trusted-catalog-bootstrap"] };
    expect(() => readRebuildChain(write([catalogStep, describeStep]))).toThrow(/stage order/);
    expect(() => readRebuildChain(write([describeStep, describeStep]))).toThrow(/duplicated/);
    expect(() => readRebuildChain(write([{ ...describeStep, command: ["cargo", "build"] }]))).toThrow(/nx command/);
  });
});
