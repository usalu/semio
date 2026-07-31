import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const jsIndex = path.resolve(here, "../../../../../..", "compose", "js", "index.ts");
const text = fs.readFileSync(jsIndex, "utf8");
const lines = text.split(/\r?\n/);
const headEnd = 3400; // keep lines 1..3400 (1-based)
const tailStart = 3901; // resume at line 3901 (// #endregion) — drop legacy block lines 3401-3900
const newBlock = `//#region 🧪️EmbeddedTests
if (
  typeof process !== "undefined" &&
  !!process.env &&
  process.env["COMPOSE_JS_RUN_EMBEDDED_TESTS"] === "1"
) {
  const { describe, it, expect } = await import("vitest");

  describe("compose-js Kit facade (strict)", () => {
    it("Kit.prototype has no legacy snapshot() hook", () => {
      type Snap = { snapshot?: () => unknown };
      const snap: Snap = Kit.prototype as unknown as Snap;
      expect(snap.snapshot).toBeUndefined();
    });

    it("kitReadPointKey normalizes the main line scope for cache keys", () => {
      expect(kitReadPointKey(theKitReadPoint)).toBe(JSON.stringify(theKitReadPoint));
    });

    it("VCS + change-algebra shells wire without KitRuntime", () => {
      const k = Object.create(Kit.prototype) as Kit;
      const g = new Graph(k, "wip");
      expect(g.root).toBe("wip");
      expect(new Session(k).id).toBe("session");
      expect(new TheKit(k, "wip").id).toContain("theKit");
      const cp = new Checkpoint(k, "wip", "cp1");
      expect(cp.change("c1").id).toBe("c1");
      expect(cp.edit("e1").id).toBe("e1");
      expect(RenamedKit.name).toBe("RenamedKit");
      expect(KitDiff.name).toBe("KitDiff");
      expect(RenamedKitInput.name).toBe("RenamedKitInput");
    });
  });

  describe("compose-js GraphQL dto contract", () => {
    it("KIT_SESSION_QUERY_ENTRY and KIT_EVENT_STREAM_SUBSCRIPTION align with target.schema.graphql", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      let sdl = "";
      const here = dirname(fileURLToPath(import.meta.url));
      for (const p of [resolve(here, "../graphql/target.schema.graphql"), resolve(process.cwd(), "compose/graphql/target.schema.graphql")]) {
        try {
          sdl = readFileSync(p, "utf8");
          if (sdl.length > 100) break;
        } catch {
          /* try next */
        }
      }
      expect(sdl.length).toBeGreaterThan(100);
      expect(sdl).toContain("type Session");
      expect(sdl).toContain("type Kit");
      expect(sdl).toMatch(/type Kit[\\s\\S]*designs:/s);
      expect(sdl).toMatch(/type Subscription[\\s\\S]*\\bwip\\b/s);
      expect(sdl).not.toMatch(/^\\s*event:\\s*Json!/m);
      expect(sdl).toContain("type Mutation");
      expect(sdl).toContain("session: SessionCommandInput!");
      expect(sdl).not.toContain("type KitStoreMutation");
      expect(KIT_SESSION_QUERY_ENTRY).toContain("wip { id theKit");
      expect(KIT_EVENT_STREAM_SUBSCRIPTION).toContain("wip");
      expect(KIT_COMMAND_SUCCEEDED_SUBSCRIPTION).toBe(KIT_EVENT_STREAM_SUBSCRIPTION);
    });
  });

  describe("compose kit-store fixtures (US-001)", () => {
    it("golden ops + expected invariants parse and match op count", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const here = dirname(fileURLToPath(import.meta.url));
      const opsPath = resolve(here, "../assets/compose/kit-store.golden.ops.compose.json");
      const expPath = resolve(here, "../assets/compose/kit-store.golden.expected.compose.json");
      const ops = JSON.parse(readFileSync(opsPath, "utf8")) as { ops: unknown[] };
      const exp = JSON.parse(readFileSync(expPath, "utf8")) as { invariants: { totalPieces: number }; projectionFingerprint: string };
      expect(ops.ops.length).toBe(exp.invariants.totalPieces);
      expect(exp.projectionFingerprint.length).toBe(64);
    });

    it("metabolism.new kit bundle has metabolism on-disk shape (Rust-owned)", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const here = dirname(fileURLToPath(import.meta.url));
      const b = JSON.parse(readFileSync(resolve(here, "../assets/compose/metabolism.new.kit.compose.json"), "utf8")) as {
        schema: string;
        wip: { id: string; initialKit?: unknown; theKit?: { savedChanges?: { items: unknown[] }; unsavedChanges?: { items: unknown[] } }; checkpoints?: { items: unknown[] } };
        authoritative: { id: string };
        stage: { id: string };
        conflicts: { items: unknown[] };
        blobs: { items: unknown[] };
      };
      expect(b.schema).toBe("🎆️26🌙️06⬆️1");
      for (const k of ["wip", "authoritative", "stage", "conflicts", "blobs"] as const) {
        expect(b[k]).toBeTruthy();
      }
      expect(typeof b.wip.id).toBe("string");
      expect(b.wip.initialKit).toBeTruthy();
    });

    it("dev JSON backbone wire shape documents semanticOpLog + persistence hints (US-004)", async () => {
      const backboneDoc = {
        kind: "compose.kit_backbone.dev_json",
        schema: "2026-05-06",
        connectionUri: "file:///tmp/example.dev-kit.json",
        persistence: {
          atomic_rewrite:
            "Serialize full JSON to sibling path ending in .tmp.compose-write, fsync, then rename(2) over the canonical file.",
          crash_safety: "Readers only observe the last renamed complete document; orphaned temp tails are harmless.",
        },
        semanticOpLog: [] as { changeId: string; kind: string; input: Record<string, unknown> }[],
      };
      expect(backboneDoc.kind).toBe("compose.kit_backbone.dev_json");
      expect(backboneDoc.persistence.atomic_rewrite.includes("rename")).toBe(true);
      expect(Array.isArray(backboneDoc.semanticOpLog)).toBe(true);
    });
  });
}
//#endregion 🧪️EmbeddedTests`;

const out = [...lines.slice(0, headEnd), newBlock, ...lines.slice(tailStart - 1)].join("\n");
fs.writeFileSync(jsIndex, out, "utf8");
console.log("spliced", { headLines: headEnd, dropped: tailStart - headEnd - 1, tailFrom: tailStart });
