import * as bun from "bun:test";
import { resolve } from "node:path";
const pages = await import(resolve("🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/🟦️.ts"));
const { registerTests1 } = await import(resolve("🏢️semio-tech/🎡️play/🧪️tests/🧪️playpages/🟦️.ts"));
await registerTests1(bun as any, pages);
