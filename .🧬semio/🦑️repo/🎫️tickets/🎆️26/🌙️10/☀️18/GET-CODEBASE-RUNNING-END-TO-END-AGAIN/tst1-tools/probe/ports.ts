const m = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts");
const rows = Object.entries(m.PLAYGROUND_PORTS).map(([k, v]: any) => [k, v.dev, v.test]);
console.log("[DEBUG] count", rows.length, "unique dev", new Set(rows.map((r) => r[1])).size);
console.log("[DEBUG]", JSON.stringify(rows.slice(0, 12)), "reserved", m.allPlaygroundReservedPorts().size);
