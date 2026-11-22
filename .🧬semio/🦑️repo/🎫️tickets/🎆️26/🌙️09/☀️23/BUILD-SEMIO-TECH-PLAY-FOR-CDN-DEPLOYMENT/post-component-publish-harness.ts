// Prototype-mode smoke of the writer and reader halves of the Play pages cohort with a synthetic site: split, stage under the exclusive cohort lease, read back, serve.
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import { withArtifactScratchV1, withArtifactDestinationTaskV1 } from "@semio-tech/repo-lib/artifacts";
import { commitPlayPagesV1, withPublishedPlayPagesV1 } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/🟦️.ts";
import { publishPlayPages } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/🟦️.ts";
import { servePublishedPlay } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/📦️release/🟦️.ts";
import { PLAY_HOST } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📇️catalog/🟦️.ts";

const workspace = process.env.SEMIO_WORKSPACE_ROOT!, mode = process.env.SEMIO_BUILD_MODE as "prototype" | "release";
if (mode !== "prototype") throw new Error("Harness refuses to write the release cohort");
await receiveScriptProcessInvocation(process.env, async invocation => {
  const phase = scriptCompilerAdmissionControlV1(invocation, "vite:play-pages", 0);
  await withRepositoryCacheStoreV1(workspace, "products", invocation, phase, (store, storePhase) => withArtifactScratchV1({ workspace, invocation, storage: store.storage, control: store.control, phaseControl: storePhase, request: { version: 1, mode, owner: "play/pages-assembly" } }, token => withArtifactDestinationTaskV1(token, invocation, async sink => {
    const site = join(sink.directory, "site"), pages = join(sink.directory, "pages");
    const put = (path: string, text: string) => { mkdirSync(join(site, path, ".."), { recursive: true }); writeFileSync(join(site, path), text); };
    put("index.html", "<!doctype html><title>play</title>"); put("assets/app.js", "export default 1;\n"); put("osm/0/0/0.png", "png"); put("vt/0/0/0.pbf", "pbf"); put("mesh/a.glb", "glb"); put("🖼️assets/x/🔭️y.3dm", "3dm"); put("🔌️plugin-modules/🧵️shard/🟨️shard-worker.js", "export {};\n"); put("🔌️plugin-modules/🖍️draw/🌉️bridge.js", "export {};\n");
    const published = publishPlayPages(site, pages, PLAY_HOST);
    await commitPlayPagesV1({ workspace, invocation, store, sink, control: sink.control, pages, published, mode });
  })));
  await withPublishedPlayPagesV1(workspace, invocation, async read => {
    console.log("[harness] families", read.families);
    const service = servePublishedPlay(read.root, read.families);
    try {
      const get = async (url: string) => { const response = await fetch(url); return `${response.status} ${response.headers.get("content-type")} ${response.headers.get("access-control-allow-origin")}`; };
      const base = service.baseURL, origins = service.origins;
      console.log("[harness] play index", await get(base), "| missing", await get(base + "nope.js"));
      for (const [host, origin] of Object.entries(origins)) console.log("[harness]", host, "->", origin);
      const media = Object.entries(origins).find(([host]) => host.startsWith("https://media."))![1];
      console.log("[harness] media 3dm", await get(`${media}/${encodeURIComponent("🖼️assets")}/x/${encodeURIComponent("🔭️y.3dm")}`));
    } finally { await service.stop(); }
  });
  console.log("[harness] OK");
});
