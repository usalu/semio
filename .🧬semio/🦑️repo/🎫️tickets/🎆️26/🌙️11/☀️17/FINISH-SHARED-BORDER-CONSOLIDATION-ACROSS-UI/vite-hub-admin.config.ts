import { readFileSync } from "node:fs";
import factory from "../../../../../../../🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts";

const fixture = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🧫️fixtures/📊️observability-v1/🔣️.json", "utf8"));
const user = (i: number) => ({ id: `u-${i}`, email: `user${i}@semio.dev`, displayName: `User ${i}`, createdAtMs: 1760000000000 + i });
const space = (i: number) => ({ id: `sp-${i}`, name: `Space ${i}`, kind: "studio", visibility: i % 2 ? "public" : "private", ownerUserId: "u-1", memberCount: 2, documentCount: 2, activeConnections: 2, createdAtMs: 0, updatedAtMs: 0 });
const connection = (i: number, s: number) => ({ syncSessionId: `sync-${s}-${i}`, scope: { spaceId: `sp-${s}`, documentId: `doc-${s}-${i % 2}` }, authenticatedUserId: "u-1", email: `user${i}@semio.dev`, role: "author", connectedAtMs: 0, source: "recorded-sync-session" });
const bodies: Record<string, unknown> = {
  "/admin/api/overview": { counts: { spaces: 3, users: 4, connections: 5 }, backends: { sqlite: true, postgres: false, neo4j: false }, dataDirBytes: 123456, headSeq: 3, openArtifacts: 2 },
  "/admin/api/observability": fixture.body ?? fixture,
  "/admin/api/spaces": { rows: [1, 2, 3].map(space), observedAtMs: 0 },
  "/admin/api/users": { rows: [1, 2, 3, 4].map(user), observedAtMs: 0 },
  "/admin/api/connections": { rows: [1, 2, 3, 4].map((i) => connection(i, 1 + (i % 2))), observedAtMs: 0, source: "recorded-sync-sessions", headSeq: 3 },
  "/admin/api/documents": { rows: [1, 2].map((i) => ({ descriptor: { spaceId: "sp-1", documentId: `doc-1-${i}` }, headSeq: 5, commitSeq: 4, epoch: 1 })), observedAtMs: 0 },
  "/admin/api/events": { rows: [1, 2, 3].map((i) => ({ id: `e-${i}`, seq: i, body: { kind: "space-created" }, actor: { kind: "user", id: "u-1" }, recordedAtMs: 1760000000000 })), observedAtMs: 0 },
};
const mock = (unauthorized: boolean) => ({
  name: "ticket-admin-mock",
  configureServer(server: { middlewares: { use: (fn: (req: { url?: string }, res: { statusCode: number; setHeader: (k: string, v: string) => void; end: (b?: string) => void }, next: () => void) => void) => void } }) {
    server.middlewares.use((req, res, next) => {
      const url = new URL(req.url ?? "/", "http://x");
      const path = url.pathname.replace(/^\/admin(?=\/admin\/api)/u, "");
      if (!path.startsWith("/admin/api/")) return next();
      if (unauthorized) { res.statusCode = 401; return res.end("{}"); }
      const member = /^\/admin\/api\/spaces\/([^/]+)$/u.exec(path);
      const body = member ? { ...space(1), id: decodeURIComponent(member[1]!), members: { rows: [1, 2].map((i) => ({ userId: `u-${i}`, email: `user${i}@semio.dev`, role: i === 1 ? "author" : "spectator" })), observedAtMs: 0 } } : bodies[path];
      res.setHeader("content-type", "application/json");
      if (!body) { res.statusCode = 404; return res.end("{}"); }
      res.end(JSON.stringify(body));
    });
  },
});
export default async (env: { command: "serve" | "build"; mode: string }) => {
  const config = await (typeof factory === "function" ? factory(env) : factory);
  const mode = process.env.ADMIN_MOCK === "unauthorized" ? mock(true) : process.env.ADMIN_MOCK === "unreachable" ? null : mock(false);
  return { ...config, server: process.env.HUB_FROZEN ? { ...(config.server ?? {}), watch: null, hmr: false } : config.server, plugins: [...(mode ? [mode] : []), ...(config.plugins ?? [])] };
};
