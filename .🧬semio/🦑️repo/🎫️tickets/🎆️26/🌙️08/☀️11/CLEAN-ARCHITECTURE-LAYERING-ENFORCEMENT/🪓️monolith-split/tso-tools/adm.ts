import { fixtureOriginalWorkStock } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";
const a = await fixtureOriginalWorkStock().admit();
console.log(a, Object.getPrototypeOf(a)?.constructor?.name);
