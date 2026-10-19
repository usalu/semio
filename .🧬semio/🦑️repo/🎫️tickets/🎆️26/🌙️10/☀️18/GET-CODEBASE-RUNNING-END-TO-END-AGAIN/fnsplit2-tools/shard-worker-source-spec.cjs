const { R, dir, doc, write } = require("./lib_ps.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts";
const part = (e, n, docText, name, pe, fdoc, fromLine) => ({ dir: dir(e, n), doc: doc(e, docText), name, emoji: pe, fdoc, fromLine });
write({
  file, function: "shardWorkerSource",
  parts: [
    part("🥾", "worker-bootstrap-and-faults", "The shard worker bootstrap text: the clock, the JSPI gate, fault state and the fault reporter.", "shardWorkerBootstrapSource", "🩺", "Returns the worker text from the clock and the stack limit through the JSPI capability gate to the fault reporter.", 70),
    part("🎬", "actor-state-and-lanes", "The shard worker actor state, pacing constants, lane order and frame interpretation text.", "shardWorkerActorStateSource", "🗂", "Returns the worker text of the actor tables, the pacing constants, the lane ordering and the frame interpretation.", 132),
    part("🚦", "request-beats-and-drive-pacing", "The shard worker request beats, turn status helpers and drive pacing text.", "shardWorkerDrivePacingSource", "⏱", "Returns the worker text of the request beats, the turn status helpers and the drive step pacing.", 241),
    part("🔌", "actor-loading-and-replies", "The shard worker reply, runtime diagnostics and actor loading text.", "shardWorkerActorLoadingSource", "📦", "Returns the worker text of the reply helpers, the runtime diagnostics and the actor loading and closing.", 289),
    part("📨", "message-handler", "The shard worker lifecycle turn classification and message handler text.", "shardWorkerMessageHandlerSource", "📬", "Returns the worker text of the lifecycle turn classification and the message handler.", 458),
  ],
}, process.argv[2]);
