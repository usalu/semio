const root = process.argv[2], library = process.argv[3], name = process.argv[4], fleetRoot = process.argv[5];
const { runOwnerPoolV1 } = await import(library + "/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🏃️runner/🟨️.mjs");
const started = Date.now();
await runOwnerPoolV1({ root, environment: process.env, signal: new AbortController().signal, say: line => console.log(JSON.stringify({ at: Date.now() - started, machine: name, pid: process.pid, line })), pollMilliseconds: 500, once: true, cores: 2, fleetRoot });
process.exit(0);
