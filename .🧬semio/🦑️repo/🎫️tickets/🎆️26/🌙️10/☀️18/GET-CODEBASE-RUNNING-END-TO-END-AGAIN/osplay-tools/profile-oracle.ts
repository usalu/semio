import { sourceDeploymentOwnersV1 } from "../../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧩️scope/🟦️.ts";
const t = performance.now();
const o = await sourceDeploymentOwnersV1({ signal: new AbortController().signal, progress: () => {} });
console.log("[DEBUG] oracle", o.length, (performance.now() - t).toFixed(0), "ms");
