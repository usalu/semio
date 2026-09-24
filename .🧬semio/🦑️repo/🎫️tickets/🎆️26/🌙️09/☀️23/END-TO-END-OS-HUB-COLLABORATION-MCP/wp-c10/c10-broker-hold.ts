/** 🎫️ C10 live proof of the session broker without restaging the hub binary: boots a loopback hub from an existing binary
 * on a catalog clone with the development profiles, starts the SAME `startLocalSessionBroker` the local hub owner runs,
 * and holds. Usage: bun c10-broker-hold.ts <port> <dataDir> <binary> */
import { LOCAL_HUB_DEVELOPMENT_PROFILES, finishLocalHub, startLocalHub, waitForReadiness, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS } from "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { startLocalSessionBroker } from "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
const [port, dataDir, binaryPath] = process.argv.slice(2);
process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
const run = await startLocalHub("/Users/ueli/Documents/semio", "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust", LOCAL_HUB_DEVELOPMENT_PROFILES, { port: Number(port), dataDir, binaryPath, adminToken: "c10-admin", capture: true });
console.log(`SPAWNED hold=${process.pid} hub=${run.child.pid}`);
await waitForReadiness(run, false, 4 * TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
const broker = startLocalSessionBroker(run, dataDir, LOCAL_HUB_DEVELOPMENT_PROFILES, 2);
console.log(`READY broker profiles=${broker.record.profiles.join(",")} port=${broker.record.port}`);
const stop = () => { broker.stop(); void finishLocalHub(run).then(() => process.exit(0)); };
process.once("SIGTERM", stop);
process.once("SIGINT", stop);
await new Promise((resolve) => run.child.once("exit", resolve));
console.log(`EXITED ${run.child.exitCode}`);
broker.stop();
