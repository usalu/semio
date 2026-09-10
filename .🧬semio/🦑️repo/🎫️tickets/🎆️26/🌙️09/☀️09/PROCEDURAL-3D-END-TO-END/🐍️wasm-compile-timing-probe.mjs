/** 🧪️ Times the renderer-wasm phases the boot's module cache removes, against the LIVE artifact served on
 * 6118 — `compileStreaming` (what a cold boot pays), a buffered `WebAssembly.compile` of the same bytes,
 * and reuse of an already-compiled `WebAssembly.Module` (what a cached boot pays). Run with
 * `bun "🐍️wasm-compile-timing-probe.mjs" [url]`; prints one JSON line. */
const url = process.argv[2] ?? "http://127.0.0.1:6118/semio-framework-os-renderer-wgpu_bg.wasm";
const head = await fetch(url, { method: "HEAD" });
const tag = head.headers.get("etag") ?? `mtime:${head.headers.get("last-modified")}:${head.headers.get("content-length")}`;
const streamStart = performance.now();
const streamed = await WebAssembly.compileStreaming(await fetch(url));
const streamMs = performance.now() - streamStart;
const bytes = new Uint8Array(await (await fetch(url)).arrayBuffer());
const bufferStart = performance.now();
const buffered = await WebAssembly.compile(bytes);
const bufferMs = performance.now() - bufferStart;
const reuseStart = performance.now();
const reused = WebAssembly.Module.imports(streamed).length + WebAssembly.Module.exports(buffered).length;
const reuseMs = performance.now() - reuseStart;
console.log(JSON.stringify({ url, tag, byteLength: bytes.byteLength, streamingCompileMs: Number(streamMs.toFixed(1)), bufferedCompileMs: Number(bufferMs.toFixed(1)), cachedModuleReuseMs: Number(reuseMs.toFixed(3)), descriptorRows: reused }));
