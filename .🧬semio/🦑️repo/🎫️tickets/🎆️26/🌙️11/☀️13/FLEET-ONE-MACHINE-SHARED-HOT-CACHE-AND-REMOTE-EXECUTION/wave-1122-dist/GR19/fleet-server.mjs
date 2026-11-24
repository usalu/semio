const { startRemoteCacheServerV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts");
const server = await startRemoteCacheServerV1({ directory: process.env.DIR, host: "127.0.0.1", port: Number(process.env.PORT), maximumBytes: 2 ** 29, maximumEntryBytes: 2 ** 28, maximumAgeDays: 1, defaultInstance: "main", tokens: { read: "r", write: "w" }, validation: "off" });
console.log("[fleet] " + server.url); setInterval(() => {}, 1 << 30);
