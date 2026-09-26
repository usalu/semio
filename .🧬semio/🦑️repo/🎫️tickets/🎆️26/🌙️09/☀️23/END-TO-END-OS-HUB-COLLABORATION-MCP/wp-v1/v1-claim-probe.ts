import { claimHubBackend } from "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { spawnSync } from "node:child_process";
const repo = "/Users/ueli/Documents/semio";
for (const name of ["postgres", "neo4j"] as const) {
  const t = Date.now();
  const claim = await claimHubBackend(repo, name, `v1probe_${process.pid}`, { onProgress: (p) => console.log(`progress ${JSON.stringify(p)}`) });
  const query = name === "postgres" ? "SELECT current_database()" : "MATCH (n) RETURN count(n) AS nodes";
  const out = spawnSync(claim.client[0]!, [...claim.client.slice(1), query], { encoding: "utf8" });
  console.log(`${name} claimed in ${Date.now() - t} ms env=${JSON.stringify(claim.env)} probe=${JSON.stringify(out.stdout.trim())} rc=${out.status}`);
  claim.release();
}
const list = spawnSync("docker", ["exec", "semio-hub-backend-postgres", "psql", "--username=semio", "--dbname=semio", "--tuples-only", "--no-align", "--command", "SELECT datname FROM pg_database ORDER BY 1"], { encoding: "utf8" });
console.log(`databases after release: ${list.stdout.trim().split("\n").join(",")}`);
