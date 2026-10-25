import { JSDOM } from "/Users/ueli/Documents/semio/node_modules/jsdom/lib/api.js";
const dom = new JSDOM("<!doctype html><html><head></head><body></body></html>");
globalThis.DOMParser = dom.window.DOMParser;
globalThis.document = dom.window.document;
const { it, expect, describe } = await import("bun:test");
const mod = await import("/Users/ueli/Documents/semio/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🪞️zukunft-bau-mirror/🟦️.ts");
const dir = "/Users/ueli/Documents/semio/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript";
mod.registerZukunftBauMirrorTests({ describe, expect, it, source: { directory: dir, url: "file://" + dir + "/📦️index.ts" } });
