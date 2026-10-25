// usage: bun socket-vectors.ts <out.json>   writes the socket path vectors (digests from Bun's reference xxh3)
const key = "0123456789abcdef", fallback = "/tmp/semio-dashboard-501", limits = { macos: 104, linux: 108 };
const encoder = new TextEncoder();
const extra = `/${key}.sock`.length;
const sized = (total: number) => "/" + "r".repeat(total - extra - 1);
const specs: [string, string][] = [
  ["the default per-user directory", fallback],
  ["one byte below the macOS capacity", sized(103)],
  ["exactly the macOS capacity", sized(104)],
  ["one byte below the Linux capacity", sized(107)],
  ["exactly the Linux capacity", sized(108)],
  ["a deep Unicode runtime directory", "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tmp/st/d/1d994ca0/artifacts/dashboard-journeys/j16381.14"],
  ["a very deep ASCII runtime directory", "/" + "d".repeat(300)],
];
const cases = specs.map(([name, runtime]) => {
  const preferred = `${runtime}/${key}.sock`;
  const bytes = encoder.encode(preferred).length;
  return { name, runtime, preferredBytes: bytes, preferred: { macos: bytes < limits.macos, linux: bytes < limits.linux }, fallbackFile: `${Bun.hash.xxHash3(encoder.encode(preferred)).toString(16).padStart(16, "0")}.sock` };
});
await Bun.write(process.argv[2]!, JSON.stringify({ schemaVersion: 1, limits, fallbackDirectory: fallback, key, cases }, null, 2) + "\n");
