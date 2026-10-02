const base = "https://localhost:18445";
const post = (bytes: number) => fetch(`${base}/commands`, { method: "POST", headers: { origin: "https://quizze.architektur-und-technologie.de", "content-type": "application/json" }, body: new Uint8Array(bytes).fill(0x20), tls: { rejectUnauthorized: false } } as RequestInit).then((r) => r.status);
console.log("at the limit:", await post(16384), "one over:", await post(16385), "empty:", await post(0), "small:", await post(10));
const stream = new ReadableStream({ start(controller) { controller.enqueue(new Uint8Array(20000).fill(0x20)); controller.close(); } });
console.log("chunked, over the limit:", await fetch(`${base}/commands`, { method: "POST", headers: { "content-type": "application/json" }, body: stream, duplex: "half", tls: { rejectUnauthorized: false } } as RequestInit).then((r) => r.status, (e) => `error ${e.message}`));
console.log("GET:", await fetch(`${base}/instance`, { tls: { rejectUnauthorized: false } } as RequestInit).then((r) => r.status));
