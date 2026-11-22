const assert = require("node:assert/strict");
const {readFileSync} = require("node:fs");
const {join} = require("node:path");
const {runInNewContext} = require("node:vm");

const source = readFileSync(join(__dirname, "bearbeiten-ingress-probe.cjs"), "utf8");
const observer = source.slice(source.indexOf("function observeWire("), source.indexOf("/** 🎞️"));
for (const capture of [true, false]) {
  const sends = [], rows = [];
  class Worker {
    addEventListener() {}
    postMessage(message) { sends.push(message); }
  }
  const window = { Worker };
  runInNewContext(observer + "\nobserveWire(capture);", {
    window, capture, performance, setInterval() {}, Uint8Array, btoa,
    console: {info(...args) { rows.push(args); }},
  });
  const worker = new window.Worker();
  const bytes = Uint8Array.of(9, 8, 7);
  const commandPage = {cursor: {seq: 4n, pageIndex: 0}, page: {length: 3}, bytes};
  const message = {actorId: "demonstrator#probe", requestId: "r1", commandPage};
  worker.postMessage(message);
  worker.postMessage(message);
  assert.deepEqual(sends, [message, message]);
  const dumps = rows.filter(row => row[1] === "command-page-bytes");
  assert.equal(dumps.length, Number(capture));
  if (capture) assert.equal(JSON.parse(dumps[0][2])[1], Buffer.from(bytes).toString("base64"));
}
