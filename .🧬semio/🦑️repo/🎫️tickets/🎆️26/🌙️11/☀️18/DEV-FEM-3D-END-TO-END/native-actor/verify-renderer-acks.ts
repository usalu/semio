import { strict as assert } from "node:assert";

const pagePrefix = Buffer.from("semio.typed-operation-page.v1\0");
const ackPrefix = Buffer.from("semio.typed-operation-ack.v1\0");
const capture = await Bun.file(process.argv[2]!).text();
const expectSettlement = process.argv.includes("--settled");
const argument = (name: string, fallback: string) => process.argv.find(value => value.startsWith(`--${name}=`))?.split("=")[1] ?? fallback;
const operation = BigInt(argument("operation", "64"));
const generation = BigInt(argument("generation", "0"));
const acceptedLanes = new Map<string, { epoch: number; lane: number; text: string; documentEffect: boolean }>();
const messages: Buffer[] = [];
const messageEpochs: number[] = [];
const issuedEpochs: number[] = [];
const returnedAcks = new Map<number, Buffer>();
let refusalEpoch = 0;

for (const line of capture.split(/\r?\n/)) {
  if (line.startsWith("[DEBUG] guest effects ")) {
    const result = JSON.parse(line.slice("[DEBUG] guest effects ".length));
    for (const effect of result.effects) {
      if (effect.tag !== "send-message" || effect.val.target.tag !== "shell") continue;
      const bytes = Buffer.from(effect.val.payload);
      if (!bytes.subarray(0, pagePrefix.length).equals(pagePrefix)) continue;
      const page = bytes.subarray(pagePrefix.length);
      assert.equal(page.readUInt32LE(0), effect.val.target.val);
      acceptedLanes.set(page.subarray(0, 25).toString("hex"), {
        epoch: Number(result.epoch), lane: page[25]!, text: page.subarray(30).toString("utf8"),
        documentEffect: result.effects.some((item: { tag: string }) => item.tag === "load-document"),
      });
    }
  } else if (line.startsWith("[DEBUG] guest [DEBUG] command ")) {
    const request = JSON.parse(line.slice("[DEBUG] guest [DEBUG] command ".length));
    for (const event of request.events) {
      if (event.kind !== "message" || event.payload.source.tag !== "shell") continue;
      const bytes = Buffer.from(event.payload.payload);
      if (bytes.subarray(0, ackPrefix.length).equals(ackPrefix)) messages.push(bytes.subarray(ackPrefix.length));
    }
  } else if (line.startsWith("[DEBUG] guest grant ")) {
    const poll = JSON.parse(line.slice("[DEBUG] guest grant ".length));
    const epoch = Number(poll.slice.input.epoch);
    issuedEpochs.push(epoch);
    if (poll.events.includes("message")) messageEpochs.push(epoch);
  } else if (line.startsWith("[DEBUG] guest ack-returned ")) {
    const returned = JSON.parse(line.slice("[DEBUG] guest ack-returned ".length));
    returnedAcks.set(Number(returned.epoch), Buffer.from(returned.payload));
  } else if (line.startsWith("[DEBUG] guest poll refusal ")) {
    const refusal = JSON.parse(line.slice("[DEBUG] guest poll refusal ".length));
    assert.equal(refusal.payload.failure.tag, "semantic");
    refusalEpoch = Number(refusal.payload.input.epoch);
    break;
  }
}

assert.ok(messageEpochs.length <= messages.length);
const operationAcks = messages.map((token, index) => ({ token, epoch: messageEpochs[index]! }))
  .filter(({ token }) => token.readBigUInt64LE(4) === operation && token.readBigUInt64LE(12) === generation);
assert.equal(operationAcks.length, 3);
const lanes = operationAcks.map(({ token, epoch }, sequence) => {
  assert.equal(token.length, 25);
  assert.equal(token.readUInt32LE(0), 1);
  assert.equal(token.readUInt32LE(20), sequence);
  assert.equal(token[24], 1);
  const page = acceptedLanes.get(token.toString("hex"));
  assert.ok(page && page.epoch < epoch);
  if (sequence === 0) assert.equal(page.documentEffect, true);
  if (sequence === 2) assert.match(page.text, /typed-operation-complete/);
  return page.lane;
});
assert.deepEqual(lanes, [6, 8, 10]);
const terminal = operationAcks[2]!;
const terminalEpoch = terminal.epoch;
if (expectSettlement) {
  assert.equal(refusalEpoch, 0);
  for (const { token, epoch } of operationAcks) assert.deepEqual(returnedAcks.get(epoch), Buffer.concat([ackPrefix, token]));
  assert.ok(issuedEpochs.some((epoch) => epoch > terminalEpoch));
  console.log(`Browser accepted the exact terminal ACK at epoch ${terminalEpoch} and issued a later poll.`);
} else {
  assert.equal(refusalEpoch, terminalEpoch);
  console.log(`Browser emitted Effect, UI and Terminal pages. The matching terminal ACK was issued at epoch ${terminalEpoch}, which returned a semantic refusal.`);
}
