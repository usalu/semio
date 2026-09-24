/** 🔍️ C10: decodes a package descriptor (lease fixture by default, or a `descriptor.semio` path) and prints each app's window and panel-tab bodies. */
import { readFileSync } from "node:fs";
import { decodePackValue } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
const path = process.argv[2];
const bytes = path
  ? new Uint8Array(readFileSync(path))
  : Uint8Array.from((JSON.parse(readFileSync(new URL("../../../../../../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", import.meta.url), "utf8")).descriptorHex as string).match(/../g)!.map((h) => parseInt(h, 16)));
const guest = decodePackValue(bytes) as Record<string, any>;
const leaves = (tabs: any[]): any[] => tabs.flatMap((tab) => (tab.children?.length ? leaves(tab.children) : [tab]));
for (const app of guest.manifest.apps) console.log(JSON.stringify({ appId: app.id, windowKinds: app.windowKinds?.map((w: any) => [w.id, w.bodyKey]), panels: app.panelTabs === undefined ? "absent" : leaves(app.panelTabs).map((tab) => [tab.kind, tab.bodyKey ?? null]) }));
