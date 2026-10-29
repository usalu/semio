import jsonpatch from "fast-json-patch";
import fixture from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🧫️fixtures/🧭️source-drain.json";

import expiryFixture from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🧫️fixtures/⌛️source-expiry.json";

const axes = ["maximumItems", "maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes"];
const debits = ["copiedItems", "copiedBytes", "retainedCapacityBytes", "releasedBytes"];
const same = (actual: unknown, expected: unknown, label: string) => {
  jsonpatch.applyPatch({actual}, [{op: "test", path: "/actual", value: expected}], true, false);
};
const transition = (previous: any, record: any) => {
  const operations: any[] = [
    {op: "test", path: "/identity", value: record.identity},
    {op: "test", path: "/stock", value: record.beforeStock},
    {op: "test", path: "/fuel", value: record.fuelBefore},
  ];
  if (record.epoch <= previous.epoch) throw Error("epoch must advance under the retained issuer");
  for (let index = 0; index < axes.length; index++) {
    const axis = axes[index];
    const remaining = previous.stock[axis] - record.receipt[debits[index]];
    if (!Number.isSafeInteger(remaining) || remaining < 0) throw Error("quoted stock exceeded");
    operations.push({op: "replace", path: `/stock/${axis}`, value: remaining});
  }
  const fuel = previous.fuel - record.workSpent;
  if (!Number.isSafeInteger(fuel) || fuel < 0) throw Error("original fuel exceeded");
  operations.push({op: "replace", path: "/fuel", value: fuel}, {op: "replace", path: "/epoch", value: record.epoch}, {op: "replace", path: "/role", value: record.role});
  return jsonpatch.applyPatch(previous, operations, true, false).newDocument;
};
const compare = (packet: any) => {
  const initial = packet.records[0];
  let model = {identity: initial.identity, stock: initial.beforeStock, fuel: initial.fuelBefore, epoch: 0, role: "admitted"};
  same(packet.records.map((record: any) => record.role), fixture.expectedPhases, "phase output");
  for (const record of packet.records) {
    model = transition(model, record);
    same(model.stock, record.afterStock, "exact remaining stock");
    same(model.fuel, record.fuelAfter, "exact remaining fuel");
  }
  same(packet.refused.map((row: any) => row.axis), fixture.refusedAxes, "negative cases");
  for (const row of packet.refused) {
    const actual = row.supplied;
    const expected = row.input;
    same(expected.operation, model.identity.operation, "negative original operation");
    same(expected.generation, model.identity.generation, "negative original generation");
    same(expected.work.clockBinding, model.identity.clock, "negative original clock");
    same(expected.work.absoluteDeadlineUs, model.identity.deadline, "negative original deadline");
    same(expected.grant, model.stock, "negative original stock");
    same(expected.work.fuel, model.fuel, "negative original fuel");
    same(expected.epoch, model.epoch + 1, "negative unissued epoch");
    const pointer: Record<string, string> = {operation: "/operation", generation: "/generation", clock: "/work/clockBinding", deadline: "/work/absoluteDeadlineUs", fuel: "/work/fuel", stock: "/grant/maximumItems"};
    const address = pointer[row.axis];
    const original = address.split("/").slice(1).reduce((value: any, part: string) => value[part], expected);
    const altered = jsonpatch.applyPatch(expected, [{op: "replace", path: address, value: original + 1}], true, false).newDocument;
    same(actual, altered, "one authentic altered axis");
    const replacements: any[] = [
      {op: "test", path: "/operation", value: expected.operation},
      {op: "test", path: "/generation", value: expected.generation},
      {op: "test", path: "/work/clockBinding", value: expected.work.clockBinding},
      {op: "test", path: "/work/absoluteDeadlineUs", value: expected.work.absoluteDeadlineUs},
      {op: "test", path: "/work/fuel", value: expected.work.fuel},
      {op: "test", path: "/grant", value: expected.grant},
    ];
    let accepted = true;
    try { jsonpatch.applyPatch(actual, replacements, true, false); }
    catch (error) { if (!(error instanceof Error) || error.name !== "TEST_OPERATION_FAILED") throw error; accepted = false; }
    same(row.accepted, accepted, row.axis);
    same(row.receipt, {copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0}, "refused effects");
  }
  return {records: packet.records.length, refused: packet.refused.length, nativeCompared: true};
};
if (process.argv[2]) {
  const log = await Bun.file(process.argv[2]).text();
  const line = log.split(/\r?\n/).find(line => line.includes("[SOURCE-LIFETIME] "));
  if (!line) throw Error("native source lifetime records absent");
  const compared = compare(JSON.parse(line.slice(line.indexOf("[SOURCE-LIFETIME] ") + 18)));
  const expiryLine = log.split(/\r?\n/).find(line => line.includes("[SOURCE-EXPIRY] "));
  if (!expiryLine) throw Error("native source expiry records absent");
  const expiry = JSON.parse(expiryLine.slice(expiryLine.indexOf("[SOURCE-EXPIRY] ") + 16));
  same(expiry.unscopedBlocked, expiryFixture.unscopedBlocked, "expired scope requires actual retirement");
  same(expiry.unscopedReceipt, {copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0}, "expired unscoped effects");
  same(expiry.zeroFuelBlocked, expiryFixture.zeroFuelBlocked, "zero fuel retains source");
  same(expiry.zeroFuelReceipt, {copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0}, "zero fuel effects");
  same(expiry.records.map((row: any) => ({name: row.name, cancelled: row.cancelled})), expiryFixture.cases, "actual expiry phases");
  for (const record of expiry.records) {
    same(record.deadline, expiryFixture.work.absoluteDeadlineUs, "immutable expired deadline");
    same(record.clockBinding, expiryFixture.work.clockBinding, "immutable original clock binding");
    same(record.observedClock, expiryFixture.expiredClock, "actual expired issuer observation");
    if (!(record.observedClock > record.deadline) || !(record.releasedBytes > 0)) throw Error("expired source did not release under remaining authority");
  }
  console.log(JSON.stringify({...compared, expiryCompared: true}));
} else {
  const identity = {parent: "actual-seed", decoder: ["actual-ledger", "actual-recipient"]};
  const stock = {maximumItems: 10, maximumCopyBytes: 20, maximumCapacityBytes: 30, maximumReleaseBytes: 40, maximumDepth: 5};
  let reference: any = {identity, stock, fuel: 20, epoch: 0, role: "admitted"};
  for (const [index, role] of fixture.expectedPhases.entries()) {
    reference = transition(reference, {identity, beforeStock: reference.stock, fuelBefore: reference.fuel, receipt: {copiedItems: 1, copiedBytes: 2, retainedCapacityBytes: 3, releasedBytes: 4}, workSpent: 1, epoch: index + 1, role});
  }
  console.log(JSON.stringify({reference, nativeCompared: false, awaiting: "native SOURCE-LIFETIME records"}));
}
