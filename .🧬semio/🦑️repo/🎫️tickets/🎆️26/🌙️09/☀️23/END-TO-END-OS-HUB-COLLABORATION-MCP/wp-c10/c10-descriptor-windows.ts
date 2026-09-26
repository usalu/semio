import { decodePackValue } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";
const fx = JSON.parse(await Bun.file(process.argv[2]).text());
const hex: string = fx.descriptorHex;
const bytes = Uint8Array.from({ length: hex.length / 2 }, (_u, i) => Number.parseInt(hex.slice(i * 2, i * 2 + 2), 16));
const d = decodePackValue(bytes) as any;
const app = d.manifest.apps.find((a: any) => a.id === fx.manifest.surface.appId);
console.log(fx.manifest.surface, app.windowKinds.map((w: any) => [w.id, w.bodyKey]), (app.panelTabs ?? []).length);
