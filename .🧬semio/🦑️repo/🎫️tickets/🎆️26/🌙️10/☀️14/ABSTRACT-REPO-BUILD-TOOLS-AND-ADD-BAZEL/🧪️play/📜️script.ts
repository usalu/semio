import { describe, expect, it } from "bun:test";
import { resolve } from "node:path";
const root = process.cwd();
const dependencies = await import(resolve(root, "🏢️semio-tech/🎡️play/🔨️modules/📦️site/🆕️fresh-build/🟦️.ts"));
const { registerTests1 } = await import(resolve(root, "🏢️semio-tech/🎡️play/🧪️tests/🧪️playfreshbuild/🟦️.ts"));
await registerTests1({ describe, expect, it }, dependencies);
