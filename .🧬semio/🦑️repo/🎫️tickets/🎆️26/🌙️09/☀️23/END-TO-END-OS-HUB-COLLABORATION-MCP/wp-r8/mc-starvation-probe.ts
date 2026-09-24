/** 🔬️ R8 probe: does a self-reposting MessageChannel chain starve setTimeout in this runtime? */
const channel = new MessageChannel();
let rounds = 0;
const started = Date.now();
channel.port1.onmessage = () => { rounds += 1; if (Date.now() - started < 3000) channel.port2.postMessage(0); else { console.log(`chain stopped after ${rounds} rounds, timer fired: ${fired}`); channel.port1.close(); } };
let fired = false;
setTimeout(() => { fired = true; console.log(`timer fired after ${Date.now() - started} ms at round ${rounds}`); }, 10);
channel.port2.postMessage(0);
