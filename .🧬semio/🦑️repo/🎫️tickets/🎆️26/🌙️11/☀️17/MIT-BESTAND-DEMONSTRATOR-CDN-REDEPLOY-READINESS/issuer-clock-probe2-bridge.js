/** @generated semio actor jco component bridge */
const U64=0xffffffffffffffffn;const currencies=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"]];const ORIGINAL_SELF_ISSUED_GRANT={"maximumItems":1048576,"maximumCopyBytes":67108864,"maximumCapacityBytes":67108864,"maximumReleaseBytes":67108864,"maximumDepth":64};const ORIGINAL_JOB_WORK_FUEL=16777216;const ORIGINAL_JOB_OUTPUT_PAGE_BYTES=16384;const ORIGINAL_JOB_CONTROL_WALL_US=30000000n;const ORIGINAL_JOB_PENDING_PAGES=1024;const ORIGINAL_SELF_ISSUED_RESERVE_BYTES=4194304;const ORIGINAL_SELF_ISSUED_RESERVE_ITEMS=4096;const ORIGINAL_SELF_ISSUED_ELAPSED_US=4503599627370496n;function amount(value) {
  if (!Number.isSafeInteger(value) || value < 0)
    throw Error("original-turn.invalid-amount");
  return value;
}
function identity(value) {
  if (typeof value !== "bigint" || value <= 0n || value > U64)
    throw Error("original-turn.invalid-identity");
  return value;
}
function grant(value) {
  return Object.freeze({ maximumItems: amount(value.maximumItems), maximumCopyBytes: amount(value.maximumCopyBytes), maximumCapacityBytes: amount(value.maximumCapacityBytes), maximumReleaseBytes: amount(value.maximumReleaseBytes), maximumDepth: amount(value.maximumDepth) });
}
function optionValue(value) {
  if (value === null || value === void 0)
    return;
  if (typeof value === "object" && (value.tag === "none" || value.tag === "some"))
    return value.tag === "none" ? void 0 : value.val;
  return value;
}
class OriginalOperationReceiptRefusal extends Error {
  receipt;
  constructor(receipt) {
    super("original-turn.receipt-refused");
    this.receipt = receipt;
  }
}
class OriginalOperationSource {
  operation;
  generation;
  deadlineUs;
  #remaining;
  #fuel;
  #epoch = 1n;
  #outstanding;
  #cancelled = !1;
  #closed = !1;
  #receiptRefusal;
  #producerRefusal;
  #nativeRefusal;
  #lastClock;
  #physical = Object.freeze({ copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0, workUnitsSpent: 0 });
  clock;
  constructor(admission, clock) {
    this.clock = clock;
    this.operation = identity(admission.operation);
    this.generation = identity(admission.generation);
    this.#remaining = grant(admission.grant);
    this.#fuel = amount(admission.fuel);
    const now = clock();
    if (typeof now !== "bigint" || now < 0n || now > U64 || typeof admission.maximumElapsedUs !== "bigint" || admission.maximumElapsedUs < 0n || admission.maximumElapsedUs > U64 - now)
      throw Error("original-turn.deadline-overflow");
    this.#lastClock = now;
    this.deadlineUs = now + admission.maximumElapsedUs;
  }
  operationClockUs() {
    const now = this.clock();
    if (typeof now !== "bigint" || now < this.#lastClock || now > U64)
      throw Error("original-turn.clock-refused");
    this.#lastClock = now;
    return now;
  }
  get fuelRemaining() {
    return this.#fuel;
  }
  get retainedRemaining() {
    return this.#remaining;
  }
  get outstanding() {
    return this.#outstanding !== void 0;
  }
  get receiptRefusal() {
    return this.#receiptRefusal;
  }
  get nativeRefusal() {
    return this.#nativeRefusal;
  }
  holdNativeRefusal(call) {
    if (this.#nativeRefusal !== void 0 || call === void 0)
      throw Error("original-turn.native-return-refused");
    this.#nativeRefusal = call;
  }
  get producerRefusal() {
    return this.#producerRefusal;
  }
  holdProducerRefusal(report) {
    if (this.#producerRefusal !== void 0 || report === void 0)
      throw Error("original-turn.producer-return-refused");
    this.#producerRefusal = report;
  }
  get physicalSpent() {
    return this.#physical;
  }
  cancel() {
    if (this.#closed)
      throw Error("original-turn.closed");
    this.#cancelled = !0;
  }
  issue() {
    if (this.#closed || this.#outstanding || this.#receiptRefusal || this.#producerRefusal !== void 0 || this.#nativeRefusal !== void 0)
      throw Object.assign(Error("original-turn.source-not-ready"), { cause: this.#nativeRefusal ?? this.#producerRefusal ?? this.#receiptRefusal });
    const input = Object.freeze({ operation: this.operation, generation: this.generation, epoch: identity(this.#epoch), grant: this.#remaining }), turn = Object.freeze({ input, fuel: this.#fuel, deadlineUs: this.deadlineUs, cancelled: this.#cancelled });
    this.#outstanding = turn;
    return turn;
  }
  receive(receipt) {
    const issued = this.#outstanding;
    if (!issued || receipt.input.operation !== issued.input.operation || receipt.input.generation !== issued.input.generation || receipt.input.epoch !== issued.input.epoch)
      throw Error("original-turn.foreign-receipt");
    for (const key of Object.keys(issued.input.grant))
      if (receipt.input.grant[key] !== issued.input.grant[key])
        throw Error("original-turn.foreign-grant");
    amount(receipt.workUnitsSpent);
    for (const [, axis] of currencies)
      amount(receipt.spent[axis]);
    const physical = { ...this.#physical };
    for (const [, axis] of currencies)
      physical[axis] = amount(physical[axis] + receipt.spent[axis]);
    physical.workUnitsSpent = amount(physical.workUnitsSpent + receipt.workUnitsSpent);
    let refused = receipt.workUnitsSpent > this.#fuel;
    if (receipt.nativeRemaining) {
      for (const [axis, key] of currencies)
        refused ||= receipt.nativeRemaining[axis] + receipt.spent[key] !== receipt.input.grant[axis];
      refused ||= receipt.nativeRemaining.maximumDepth !== receipt.input.grant.maximumDepth;
    }
    for (const [axis, spent] of currencies)
      refused ||= receipt.spent[spent] > this.#remaining[axis];
    this.#physical = Object.freeze(physical);
    this.#outstanding = void 0;
    if (refused) {
      this.#receiptRefusal = receipt;
      throw new OriginalOperationReceiptRefusal(receipt);
    }
    const next = { ...this.#remaining };
    for (const [axis, spent] of currencies)
      next[axis] -= receipt.spent[spent];
    this.#remaining = Object.freeze(next);
    this.#fuel -= receipt.workUnitsSpent;
    if (this.#epoch === U64)
      throw Error("original-turn.epoch-exhausted");
    this.#epoch += 1n;
  }
  close() {
    if (this.#outstanding || this.#receiptRefusal || this.#producerRefusal !== void 0 || this.#nativeRefusal !== void 0)
      throw Error("original-turn.original-return-pending");
    this.#closed = !0;
  }
}
function originalGrantWit(grant) {
  return { maximumItems: BigInt(grant.maximumItems), maximumCopyBytes: BigInt(grant.maximumCopyBytes), maximumCapacityBytes: BigInt(grant.maximumCapacityBytes), maximumReleaseBytes: BigInt(grant.maximumReleaseBytes), maximumDepth: BigInt(grant.maximumDepth) };
}
function originalTurnWit(turn) {
  const input = turn.input;
  return { operation: input.operation, generation: input.generation, epoch: input.epoch, work: { clockBinding: input.operation, absoluteDeadlineUs: turn.deadlineUs, fuel: BigInt(turn.fuel) }, grant: originalGrantWit(input.grant) };
}
const actorInputWire=(function createActorInputWireV1() {
  const text = new TextEncoder, compare = (left, right) => {
    const length = Math.min(left.length, right.length);
    for (let at = 0;at < length; at++)
      if (left[at] !== right[at])
        return left[at] - right[at];
    return left.length - right.length;
  }, varint = (out, value) => {
    let rest = BigInt(value);
    for (;; ) {
      const byte = Number(rest & 0x7fn);
      rest >>= 7n;
      if (rest === 0n) {
        out.push(byte);
        return;
      }
      out.push(byte | 128);
    }
  }, collect = (value, counts) => {
    if (typeof value === "string")
      counts.set(value, (counts.get(value) ?? 0) + 1);
    else if (Array.isArray(value))
      for (const item of value)
        collect(item, counts);
    else if (value !== null && typeof value === "object" && !ArrayBuffer.isView(value))
      for (const item of Object.values(value))
        collect(item, counts);
  }, inline = (out, value) => {
    const bytes = text.encode(value);
    out.push(7);
    varint(out, bytes.length);
    for (const byte of bytes)
      out.push(byte);
  }, write = (value, index, out) => {
    if (value === null || value === void 0) {
      out.push(18);
      return;
    }
    if (typeof value === "boolean") {
      out.push(value ? 2 : 1);
      return;
    }
    if (typeof value === "string") {
      const at = index.get(value);
      if (at === void 0)
        inline(out, value);
      else {
        out.push(6);
        varint(out, at);
      }
      return;
    }
    if (value instanceof Uint8Array) {
      out.push(12);
      varint(out, value.length);
      for (const byte of value) {
        out.push(4);
        varint(out, byte);
      }
      return;
    }
    if (Array.isArray(value)) {
      out.push(12);
      varint(out, value.length);
      for (const item of value)
        write(item, index, out);
      return;
    }
    const exact = typeof value === "bigint" ? value : typeof value === "number" && Number.isSafeInteger(value) ? BigInt(value) : void 0;
    if (exact !== void 0) {
      if (exact < 0n || exact > 0xffffffffffffffffn)
        throw Error("actor-input.integer");
      out.push(4);
      varint(out, exact);
      return;
    }
    if (typeof value === "object") {
      const entries = Object.entries(value).map(([key, item]) => [text.encode(key), key, item]).sort((left, right) => compare(left[0], right[0]));
      out.push(16);
      varint(out, entries.length);
      for (const [, key, item] of entries) {
        inline(out, key);
        write(item, index, out);
      }
      return;
    }
    throw Error("actor-input.value");
  }, lift = (value) => {
    if (value === void 0 || value === null)
      return null;
    if (value instanceof Uint8Array)
      return value;
    if (Array.isArray(value))
      return value.map(lift);
    if (typeof value === "object")
      return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, lift(item)]));
    return value;
  };
  return {
    encode(value) {
      const counts = new Map;
      collect(value, counts);
      const interned = [...counts].filter(([symbol, count]) => text.encode(symbol).length <= 128 || count >= 2).map(([symbol]) => text.encode(symbol)).sort(compare), index = new Map(interned.map((bytes, at) => [new TextDecoder().decode(bytes), at])), out = [];
      varint(out, interned.length);
      for (const bytes of interned) {
        varint(out, bytes.length);
        for (const byte of bytes)
          out.push(byte);
      }
      varint(out, 1);
      varint(out, 1);
      out.push(17);
      write(value, index, out);
      return new Uint8Array(out);
    },
    event: lift
  };
})();
function originalContextBudgetWit(turn) {
  return { input: originalTurnWit(turn), retainedPolicy: originalGrantWit(turn.input.grant), fuel: BigInt(turn.fuel), deadlineUs: turn.deadlineUs, cancelled: !1 };
}
function originalJobWallUs(budget) {
  return budget === void 0 ? ORIGINAL_JOB_CONTROL_WALL_US : BigInt(budget.deadlineMs) * 1000n;
}
function originalReceiptFromWit(receipt, workUnitsSpent) {
  const extent = (value) => {
    if (typeof value !== "bigint" || value < 0n || value > BigInt(Number.MAX_SAFE_INTEGER))
      throw Error("original-turn.native-extent-refused");
    return Number(value);
  }, raw = receipt.input, input = { operation: raw.operation, generation: raw.generation, epoch: raw.epoch, grant: { maximumItems: extent(raw.grant.maximumItems), maximumCopyBytes: extent(raw.grant.maximumCopyBytes), maximumCapacityBytes: extent(raw.grant.maximumCapacityBytes), maximumReleaseBytes: extent(raw.grant.maximumReleaseBytes), maximumDepth: extent(raw.grant.maximumDepth) } }, spent = { copiedItems: extent(receipt.spent.copiedItems), copiedBytes: extent(receipt.spent.copiedBytes), retainedCapacityBytes: extent(receipt.spent.retainedCapacityBytes), releasedBytes: extent(receipt.spent.releasedBytes) }, remaining = receipt.remaining, nativeRemaining = remaining ? { maximumItems: extent(remaining.maximumItems), maximumCopyBytes: extent(remaining.maximumCopyBytes), maximumCapacityBytes: extent(remaining.maximumCapacityBytes), maximumReleaseBytes: extent(remaining.maximumReleaseBytes), maximumDepth: extent(remaining.maximumDepth) } : void 0;
  return { input, spent, workUnitsSpent: extent(workUnitsSpent), nativeReceipt: receipt, nativeRemaining };
}
class OriginalJobFunds {
  #input;
  #deadlineUs;
  #granted;
  #policy;
  #work;
  #fuel;
  #clock;
  #drive;
  constructor(operation, nowUs, wallUs, clock, turn, drive) {
    this.#clock = clock;
    this.#drive = drive;
    if (turn === void 0) {
      const grant = originalGrantWit(ORIGINAL_SELF_ISSUED_GRANT);
      this.#input = { operation: operation > 0n ? operation : 1n, generation: 1n, epoch: 1n, work: { clockBinding: 1n, absoluteDeadlineUs: nowUs + wallUs, fuel: BigInt(ORIGINAL_JOB_WORK_FUEL) }, grant };
      this.#work = BigInt(ORIGINAL_JOB_WORK_FUEL);
    } else {
      this.#input = originalTurnWit(turn);
      this.#work = BigInt(turn.fuel);
    }
    const wall = nowUs + wallUs, issuedDeadline = this.#input.work.absoluteDeadlineUs;
    this.#deadlineUs = wall < issuedDeadline ? wall : issuedDeadline;
    this.#fuel = this.#work;
    this.#granted = this.#input.grant;
    this.#policy = { ...this.#input.grant };
  }
  crossing() {
    if (this.#work === 0n)
      throw Error("original-job.work-exhausted");
    return { input: this.#input, retainedPolicy: this.#policy, fuel: this.#work, deadlineUs: this.#deadlineUs, cancelled: !1 };
  }
  outputCrossing() {
    const budget = this.crossing(), page = BigInt(ORIGINAL_JOB_OUTPUT_PAGE_BYTES), policy = budget.retainedPolicy;
    return { ...budget, retainedPolicy: { ...policy, maximumCopyBytes: policy.maximumCopyBytes < page ? policy.maximumCopyBytes : page } };
  }
  receipt(turn) {
    const spent = (axis) => Number(this.#granted[axis] - this.#policy[axis]);
    return { input: turn.input, spent: { copiedItems: spent("maximumItems"), copiedBytes: spent("maximumCopyBytes"), retainedCapacityBytes: spent("maximumCapacityBytes"), releasedBytes: spent("maximumReleaseBytes") }, workUnitsSpent: Number(this.#fuel - this.#work) };
  }
  settle(call, policy = this.#policy) {
    const input = this.#input;
    if (call.input.operation !== input.operation || call.input.generation !== input.generation || call.input.epoch !== input.epoch || call.input.work?.clockBinding !== input.work.clockBinding || call.input.work?.absoluteDeadlineUs !== input.work.absoluteDeadlineUs || call.input.work?.fuel !== input.work.fuel || call.input.grant?.maximumDepth !== input.grant.maximumDepth || currencies.some(([axis]) => call.input.grant?.[axis] !== input.grant[axis]))
      throw Error("original-job.foreign-receipt");
    const next = { ...this.#policy };
    for (const [axis, spent] of currencies) {
      const used = call.spent[spent];
      if (typeof used !== "bigint" || used < 0n || used > policy[axis] || used > next[axis])
        throw Error("original-job.receipt-refused");
      next[axis] -= used;
    }
    if (typeof call.workUnitsSpent !== "bigint" || call.workUnitsSpent < 0n || call.workUnitsSpent > this.#work)
      throw Error("original-job.receipt-refused");
    this.#policy = next;
    this.#work -= call.workUnitsSpent;
    if (optionValue(call.refusal) !== void 0)
      throw Object.assign(Error("original-job.call-refused"), { refusal: call.refusal });
  }
  #pending(attempt, message) {
    if (attempt >= ORIGINAL_JOB_PENDING_PAGES || this.#clock !== void 0 && this.#clock() >= this.#deadlineUs)
      throw Error(message);
  }
  async#request(call, output = !1) {
    if (this.#drive !== void 0)
      return this.#drive(call, output);
    const budget = output ? this.outputCrossing() : this.crossing(), reply = await call(budget);
    this.settle(reply, budget.retainedPolicy);
    return reply;
  }
  async start(guest, job, kind, input) {
    let source = { job, kind, input, checkpoint: null };
    for (let pending = 0;; pending++) {
      const wire = actorInputWire.encode({ operation: "start-job", source }), admission = await this.#request((budget) => guest.reactor.admitActorInput(wire, budget));
      if (!admission.ready || admission.disposition === "returned")
        throw Error("original-job.input-refused");
      if ((await this.#request((budget) => guest.jobs.startJob(job, budget))).admitted)
        return;
      this.#pending(pending, "original-job.start-stalled");
      source = null;
    }
  }
  async step(jobs, job, control = {}) {
    const call = await this.#request((budget) => jobs.stepJob(job, budget)), outcome = optionValue(call.outcome);
    if (outcome === void 0)
      return { status: "running" };
    const bytes = await this.#collect(jobs, job, outcome.byteLength, control);
    if (outcome.state === "running")
      return { status: "running", progress: bytes.length > 0 ? bytes : void 0 };
    if (outcome.state === "done")
      return { status: "done", value: bytes };
    if (outcome.state === "failed")
      return { status: "failed", value: bytes };
    throw Error("original-job.outcome-state");
  }
  async cancel(jobs, job, control = {}) {
    const call = await this.#request((budget) => jobs.cancelJob(job, budget));
    control.held?.delete(job);
    if (!call.admitted)
      throw Error("original-job.cancel-refused");
  }
  async#collect(jobs, job, expected, control) {
    const held = control.held?.get(job) ?? { parts: [], length: 0n };
    control.held?.set(job, held);
    let pending = 0;
    for (;; ) {
      if (control.signal?.aborted)
        throw Error("original-job.aborted");
      const call = await this.#request((budget) => jobs.readJobOutput(job, budget), !0), chunk = optionValue(call.chunk);
      if (chunk === void 0)
        break;
      const page = optionValue(chunk.bytes);
      if (page !== void 0) {
        if (chunk.offset !== held.length)
          throw Error("original-job.output-offset");
        held.parts.push(page);
        held.length += BigInt(page.length);
        pending = 0;
        control.onProgress?.({ job, bytes: held.length, expected });
      }
      if (chunk.outputClosed)
        break;
      if (page === void 0)
        this.#pending(pending++, "original-job.output-stalled");
    }
    if (held.length !== expected)
      throw Error("original-job.output-length");
    for (let pending = 0;; pending++) {
      if ((await this.#request((budget) => jobs.acknowledgeJobOutput(job, budget))).acknowledged)
        break;
      this.#pending(pending, "original-job.output-ack-stalled");
    }
    const bytes = new Uint8Array(Number(held.length));
    let at = 0;
    for (const part of held.parts) {
      bytes.set(part, at);
      at += part.length;
    }
    control.held?.delete(job);
    return bytes;
  }
}
async function originalSourceJob(source, report, wallUs, run) {
  const clock = () => source.operationClockUs(), started = clock(), drive = async (call, output) => {
    if (source.fuelRemaining === 0)
      throw Error("original-job.work-exhausted");
    const turn = source.issue(), funds = new OriginalJobFunds(source.operation, started, wallUs, clock, turn), budget = output ? funds.outputCrossing() : funds.crossing();
    let reply;
    try {
      reply = await call(budget);
    } catch (failure) {
      source.holdNativeRefusal(failure);
      throw failure;
    }
    let refusal;
    try {
      funds.settle(reply, budget.retainedPolicy);
    } catch (failure) {
      if (!(failure instanceof Error) || failure.message !== "original-job.call-refused") {
        source.holdNativeRefusal(reply);
        throw failure;
      }
      refusal = failure;
    }
    const receipt = funds.receipt(turn);
    source.receive(receipt);
    report(receipt);
    if (refusal !== void 0)
      throw refusal;
    return reply;
  };
  return run(new OriginalJobFunds(source.operation, started, wallUs, clock, void 0, drive));
}
class OriginalSelfIssuedTurns {
  #guest;
  #source;
  #clock;
  #held = new Map;
  #ready = !1;
  constructor(guest, clock) {
    this.#guest = guest;
    this.#clock = clock;
    this.#source = new OriginalOperationSource(Object.freeze({ operation: 1n, generation: 1n, grant: ORIGINAL_SELF_ISSUED_GRANT, fuel: Number.MAX_SAFE_INTEGER, maximumElapsedUs: ORIGINAL_SELF_ISSUED_ELAPSED_US }), clock);
  }
  #stock() {
    const left = this.#source.retainedRemaining;
    if (left.maximumItems < ORIGINAL_SELF_ISSUED_RESERVE_ITEMS || left.maximumCopyBytes < ORIGINAL_SELF_ISSUED_RESERVE_BYTES || left.maximumCapacityBytes < ORIGINAL_SELF_ISSUED_RESERVE_BYTES || left.maximumReleaseBytes < ORIGINAL_SELF_ISSUED_RESERVE_BYTES)
      throw Object.assign(Error("original-turn.stock-exhausted"), { remaining: left });
  }
  #receive(call, workUnitsSpent) {
    this.#source.receive(originalReceiptFromWit(call, workUnitsSpent));
  }
  #hold(failure) {
    try {
      if (this.#source.outstanding && this.#source.nativeRefusal === void 0)
        this.#source.holdNativeRefusal(failure ?? Error("original-turn.native-return-refused"));
    } catch {}
  }
  async#context() {
    while (!this.#ready) {
      this.#stock();
      const turn = this.#source.issue();
      let call;
      try {
        call = await this.#guest.reactor.admitActorContext(originalContextBudgetWit(turn));
      } catch (failure) {
        this.#hold(failure);
        throw failure;
      }
      this.#receive(call, call.workUnitsSpent);
      if (optionValue(call.refusal) !== void 0)
        throw Object.assign(Error("original-turn.context-refused"), { originalCall: call });
      this.#ready = call.ready;
      if (!this.#ready && call.spent.copiedItems === 0n && call.spent.copiedBytes === 0n && call.spent.retainedCapacityBytes === 0n && call.spent.releasedBytes === 0n)
        throw Object.assign(Error("original-turn.context-held"), { originalCall: call });
    }
  }
  async#admit(value) {
    this.#stock();
    const turn = this.#source.issue();
    let call;
    try {
      call = await this.#guest.reactor.admitActorInput(actorInputWire.encode(value), originalContextBudgetWit(turn));
    } catch (failure) {
      this.#hold(failure);
      throw failure;
    }
    this.#receive(call, call.workUnitsSpent);
    if (optionValue(call.refusal) !== void 0 || !call.ready || call.disposition === "returned")
      throw Object.assign(Error("original-turn.input-refused"), { originalCall: call });
  }
  async poll(events, shape) {
    if (events.length > 1)
      throw Error("actor-poll.event-count: a reactor turn admits at most one event");
    await this.#context();
    await this.#admit({ operation: "poll", events: events.map(actorInputWire.event), shape });
    this.#stock();
    const turn = this.#source.issue();
    let result;
    try {
      result = await this.#guest.reactor.poll(originalContextBudgetWit(turn));
    } catch (failure) {
      try {
        if (failure && failure.input && failure.spent && typeof failure.workUnitsSpent === "bigint")
          this.#receive(failure, failure.workUnitsSpent);
      } catch {}
      this.#hold(failure);
      throw failure;
    }
    this.#receive(result.retainedReceipt, result.retainedReceipt.workUnitsSpent);
    return result;
  }
  async startJob(job, kind, input) {
    await this.#context();
    return originalSourceJob(this.#source, () => {}, originalJobWallUs(), (funds) => funds.start(this.#guest, job, kind, input));
  }
  stepJob(job, budget, control = {}) {
    return originalSourceJob(this.#source, () => {}, originalJobWallUs(budget), (funds) => funds.step(this.#guest.jobs, job, { ...control, held: this.#held }));
  }
  cancelJob(job) {
    return originalSourceJob(this.#source, () => {}, originalJobWallUs(), (funds) => funds.cancel(this.#guest.jobs, job, { held: this.#held }));
  }
}

const ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = 44;
const encodeActorInstanceLifecycle = function encodeActorInstanceLifecycle(value) {
  const body = value.kind === "ack" ? value.receipt : value, tag = value.kind === "ack" ? body.kind === "captured" ? 5 : body.kind === "accepted" ? 6 : body.kind === "retired" ? 7 : -1 : body.kind === "open" ? 0 : body.kind === "captured" ? 1 : body.kind === "close" ? 2 : body.kind === "accepted" ? 3 : body.kind === "retired" ? 4 : -1;
  if (tag === -1)
    throw Error("actor-lifecycle.tag");
  const generation = body.kind === "open" ? body.activationGeneration : body.lifetime.activationGeneration, instance = body.kind === "open" ? body.instanceId : body.lifetime.instanceId, validGeneration = (field) => typeof field === "bigint" && field > 0n && field <= 0xffffffffffffffffn;
  if (!validGeneration(generation) || !Number.isInteger(instance) || instance < 0 || instance > 4294967295 || !Number.isSafeInteger(body.requestSequence) || body.requestSequence <= 0)
    throw Error("actor-lifecycle.invalid-authority");
  if (body.kind !== "open" && !validGeneration(body.lifetime.guestLifetime))
    throw Error("actor-lifecycle.invalid-guest-lifetime");
  if ((body.kind === "accepted" || body.kind === "retired") && !validGeneration(body.closeGeneration))
    throw Error("actor-lifecycle.invalid-close-generation");
  const output = new Uint8Array(44);
  let length = 0;
  const put = (initial) => {
    let rest = initial;
    do {
      const byte = Number(rest & 127n);
      rest >>= 7n;
      output[length++] = byte | (rest === 0n ? 0 : 128);
    } while (rest !== 0n);
  };
  output[length++] = tag;
  put(generation);
  put(BigInt(instance));
  if (body.kind !== "open")
    put(body.lifetime.guestLifetime);
  put(BigInt(body.requestSequence));
  if (body.kind === "accepted" || body.kind === "retired")
    put(body.closeGeneration);
  return output.slice(0, length);
};
const ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = 35;
const encodeActorUiPatchReceipt = function encodeActorUiPatchReceipt(receipt) {
  const lifetime = receipt.lifetime, valid = (value) => typeof value === "bigint" && value > 0n && value <= 0xffffffffffffffffn;
  if (!valid(lifetime.activationGeneration) || !valid(lifetime.guestLifetime) || !valid(receipt.patchSequence) || !Number.isInteger(lifetime.instanceId) || lifetime.instanceId < 0 || lifetime.instanceId > 4294967295)
    throw Error("actor-ui-patch.invalid-authority");
  const output = new Uint8Array(35);
  let length = 0;
  const put = (initial) => {
    let rest = initial;
    do {
      const byte = Number(rest & 127n);
      rest >>= 7n;
      output[length++] = byte | (rest === 0n ? 0 : 128);
    } while (rest !== 0n);
  };
  put(lifetime.activationGeneration);
  put(BigInt(lifetime.instanceId));
  put(lifetime.guestLifetime);
  put(receipt.patchSequence);
  return output.slice(0, length);
};
const validateActorUiPatchPairing = function validateActorUiPatchPairing(patchCount, receipt) {
  if (!Number.isSafeInteger(patchCount) || patchCount < 0 || patchCount > 0 !== (receipt != null))
    throw Error("actor-ui-patch.pairing");
  if (receipt != null)
    encodeActorUiPatchReceipt(receipt);
};
const commandIngressKinds = new Map([[0, "idle"], [1, "page-accepted"], [2, "backpressure"], [3, "command-pending"], [4, "command-complete"], [5, "fault"]]);
const {admitActorCodecRequestV1,admitActorCodecAnswerV1,actorCodecAnswer:codecAnswer}=(function createActorCodecBoundaryV1() {
  const brand = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), Symbol.toStringTag).get, bytes = (value) => ArrayBuffer.isView(value) && brand.call(value) === "Uint8Array", fields = (value, names) => value !== null && typeof value === "object" && !Array.isArray(value) && !ArrayBuffer.isView(value) && Reflect.ownKeys(value).length === names.length && names.every((name) => {
    const descriptor = Object.getOwnPropertyDescriptor(value, name);
    return descriptor !== void 0 && "value" in descriptor;
  }), mirror = (value) => Array.isArray(value) && value.length === 2 && Reflect.ownKeys(value).length === 3 && ["0", "1"].every((name) => {
    const descriptor = Object.getOwnPropertyDescriptor(value, name);
    return descriptor !== void 0 && "value" in descriptor && typeof descriptor.value === "string";
  });
  function admitActorCodecRequestV1(value) {
    const descriptor = value !== null && typeof value === "object" ? Object.getOwnPropertyDescriptor(value, "operation") : void 0;
    if (!descriptor || !("value" in descriptor) || typeof descriptor.value !== "string")
      throw Error("actor-codec.request");
    const operation = descriptor.value;
    if (operation !== "pack-schema-hash" && operation !== "genesis" && operation !== "print-mirror")
      throw Error("actor-codec.operation:" + operation);
    if (fields(value, operation === "pack-schema-hash" ? ["operation", "artifactKind"] : operation === "genesis" ? ["operation", "artifactKind", "documentId"] : ["operation", "artifactKind", "pair"]) && typeof value.artifactKind === "string" && value.artifactKind.length > 0 && (operation === "pack-schema-hash" || operation === "genesis" && typeof value.documentId === "string" && value.documentId.length > 0 || operation === "print-mirror" && fields(value.pair, ["pack", "spr"]) && bytes(value.pair.pack) && bytes(value.pair.spr)))
      return value;
    throw Error("actor-codec.request");
  }
  function admitActorCodecAnswerV1(operation, value) {
    if (operation !== "pack-schema-hash" && operation !== "genesis" && operation !== "print-mirror")
      throw Error("actor-codec.operation" + (typeof operation === "string" ? ":" + operation : ""));
    if (fields(value, ["fault"]) && bytes(value.fault))
      return value;
    if (fields(value, ["ok"])) {
      const success = value.ok;
      if (operation === "pack-schema-hash" && bytes(success) || operation === "genesis" && fields(success, ["pack", "spr"]) && bytes(success.pack) && bytes(success.spr) || operation === "print-mirror" && mirror(success))
        return value;
    }
    throw Error(`actor-codec.answer:${operation}`);
  }
  async function actorCodecAnswer(codec, request) {
    admitActorCodecRequestV1(request);
    const { operation, artifactKind } = request;
    if (!codec)
      throw Error("actor-codec.unexported");
    try {
      let success;
      if (operation === "pack-schema-hash") {
        if (typeof codec.packSchemaHash !== "function")
          throw Error("actor-codec.unexported");
        success = await codec.packSchemaHash(artifactKind);
      } else if (operation === "genesis") {
        const documentId = request.documentId;
        if (typeof codec.genesis !== "function")
          throw Error("actor-codec.unexported");
        success = await codec.genesis(artifactKind, documentId);
      } else if (operation === "print-mirror") {
        const pair = request.pair;
        if (typeof codec.printMirror !== "function")
          throw Error("actor-codec.unexported");
        success = await codec.printMirror(artifactKind, pair);
      } else
        throw Error("actor-codec.operation");
      return admitActorCodecAnswerV1(operation, { ok: success });
    } catch (error) {
      const payload = error !== null && typeof error === "object" && "payload" in error ? error.payload : void 0;
      if (payload?.tag === "fault")
        try {
          return admitActorCodecAnswerV1(operation, { fault: payload.val });
        } catch {
          throw error;
        }
      throw error;
    }
  }
  return { admitActorCodecRequestV1, admitActorCodecAnswerV1, actorCodecAnswer };
})();
const HOST_CHANNEL = 25;
const admitGuestChannelVersion = function admitGuestChannelVersion(guest, host) {
  if (guest === host)
    return null;
  return { origin: "framework", code: "plugin.channel-mismatch", severity: "error", message: `the guest speaks app channel ${guest}, the host app channel ${host}`, scope: {}, params: { guest: String(guest), host: String(host) }, retryable: !1 };
};


function lifecycleBody(value) {
  return { lifetime: value.lifetime, requestSequence: BigInt(value.requestSequence), ...("closeGeneration" in value ? { closeGeneration: value.closeGeneration } : {}) };
}

function lifecycleEvent(kind, payload, activationGeneration) {
  if (kind === "patch-ack" || kind === "patch-rejected") {
    encodeActorUiPatchReceipt(payload.receipt);
    if (payload.receipt.lifetime.activationGeneration !== activationGeneration || payload.surface?.instance !== payload.receipt.lifetime.instanceId) throw new Error("actor-ui-patch.activation-mismatch");
    return { tag: kind, val: payload };
  }
  if (kind === "instance-open") {
    encodeActorInstanceLifecycle({ kind: "open", activationGeneration: payload.activationGeneration, instanceId: payload.instance, requestSequence: payload.requestSequence });
    if (payload.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
    return { tag: kind, val: { ...payload, requestSequence: BigInt(payload.requestSequence) } };
  }
  if (kind === "instance-close" || kind === "instance-lifecycle-ack") {
    if (payload?.kind !== (kind === "instance-close" ? "close" : "ack")) throw new Error("actor-lifecycle.event-kind");
    encodeActorInstanceLifecycle(payload);
    const receipt = kind === "instance-close" ? payload : payload.receipt;
    if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
    return { tag: kind, val: kind === "instance-close" ? lifecycleBody(receipt) : { tag: receipt.kind, val: lifecycleBody(receipt) } };
  }
  return kind === "wake" ? ({ tag: kind }) : ({ tag: kind, val: payload });
}

function lifecycleReceipt(raw, activationGeneration) {
  const value = optionValue(raw);
  if (value === undefined || value === null) return undefined;
  if (value.tag !== "captured" && value.tag !== "accepted" && value.tag !== "retired") throw new Error("actor-lifecycle.receipt-required");
  const body = value.val;
  if (typeof body?.requestSequence !== "bigint" || body.requestSequence <= 0n || body.requestSequence > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("actor-lifecycle.request-sequence");
  if (body.lifetime?.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
  return encodeActorInstanceLifecycle({ kind: value.tag, lifetime: body.lifetime, requestSequence: Number(body.requestSequence), ...(value.tag === "captured" ? {} : { closeGeneration: body.closeGeneration }) });
}

function normalizeCommandIngress(status) {
  const originalMediaAck = optionValue(status.originalMediaAck);
  if (originalMediaAck !== undefined && originalMediaAck !== null) return { tag: "original-media-ack", val: originalMediaAck };

  const tag = commandIngressKinds.get(status.kind);
  if (!tag) throw new Error(`unknown command ingress kind: ${status.kind}`);
  if (tag === "idle") return { tag };
  if (tag === "fault") return { tag, val: { cursor: status.cursor, fault: { tag: "fault", val: status.fault } } };
  return { tag, val: status.cursor };
}

function uiPatchReceipt(result, activationGeneration) {
  if (!Array.isArray(result.uiPatches)) throw new Error("actor-ui-patch.envelope");
  const receipt = optionValue(result.uiPatchReceipt);
  validateActorUiPatchPairing(result.uiPatches.length, receipt);
  if (receipt === undefined || receipt === null) return undefined;
  if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error("actor-ui-patch.activation-mismatch");
  return encodeActorUiPatchReceipt(receipt);
}

function nativeRefusalError(failure) {
  const tag = (failure?.payload ?? failure)?.failure?.tag;
  if (typeof tag === "string") return new Error("original-turn.native-return-refused: " + tag, { cause: failure });
  return failure instanceof Error ? failure : undefined;
}

export async function createActorApi(actorId, activationGeneration, originalAdmission, operationClockUs, originalReturn) {
  if (typeof activationGeneration !== "bigint" || activationGeneration <= 0n || activationGeneration > 0xffffffffffffffffn) throw new Error("actor-close.invalid-activation-generation");
  const componentUrl = new URL("./semio_hub_procedural_component.js", import.meta.url);
  const rebuildVersion = new URL(import.meta.url).searchParams.get("v");
  componentUrl.searchParams.set("actor", actorId);
  componentUrl.searchParams.set("activation", activationGeneration.toString());
  if (rebuildVersion) componentUrl.searchParams.set("v", rebuildVersion);
  const hostUrl = new URL("./🟨️.js", import.meta.url);
  hostUrl.search = componentUrl.search;
  const hostShim = await import(hostUrl.href);
  const { reactor, jobs, checkpoint, describe, codec } = await import(componentUrl.href);
  hostShim.__bindOriginalClock(() => { const v = operationClockUs(); const g = globalThis; g.__cl ??= { last: v, calls: 0, big: 0, back: 0 }; g.__cl.calls++; if (v < g.__cl.last) g.__cl.back++; const d = Number(v - g.__cl.last); if (d > 7500 && g.__cl.big++ < 15) console.log('[PROBE] clock jump us=' + d + ' calls=' + g.__cl.calls); g.__cl.last = v; if (g.__cl.calls % 200000 === 0) console.log('[PROBE] clock calls=' + g.__cl.calls + ' big=' + g.__cl.big + ' back=' + g.__cl.back); return v; });
  const originalSource = new OriginalOperationSource(originalAdmission, () => hostShim.operationClockUs());
  function receiveOriginal(call, workUnitsSpent) {
    let receipt;
    try { receipt = originalReceiptFromWit(call, workUnitsSpent); originalSource.receive(receipt); }
    catch (failure) { if (receipt) originalReturn.receipt(receipt); originalSource.holdNativeRefusal(call); originalReturn.hold(call); throw failure; }
    originalReturn.receipt(receipt);
  }
  const heldOutput = new Map();
  let sourceTail = Promise.resolve();
  const exclusive = (run) => {
    const work = sourceTail.then(() => {
      const refusal = nativeRefusalError(originalSource.nativeRefusal);
      if (refusal !== undefined) throw refusal;
      return run();
    });
    sourceTail = work.then(() => {}, () => {});
    return work;
  };
  const fundedJob = (budget, run) => exclusive(() => originalSourceJob(originalSource, (receipt) => originalReturn.receipt(receipt), originalJobWallUs(budget), run));
  let contextReady = false;
  async function admitWire(value) {
    const turn = originalSource.issue();
    let call;
    const wireBytes = actorInputWire.encode(value);
    try { call = await reactor.admitActorInput(wireBytes, originalContextBudgetWit(turn)); }
    catch (failure) { originalSource.holdNativeRefusal(failure); originalReturn.hold(failure); throw failure; }
    receiveOriginal(call, call.workUnitsSpent);
    if (optionValue(call.refusal) !== undefined || !call.ready || call.disposition === "returned") throw Object.assign(new Error("original-turn.input-refused: refusal=" + JSON.stringify(optionValue(call.refusal) ?? null) + " ready=" + call.ready + " disposition=" + call.disposition + " wireBytes=" + (wireBytes?.length ?? 0) + " grant=" + JSON.stringify(turn.input.grant, (k, v) => typeof v === "bigint" ? v.toString() : v)), { originalCall: call });
  }
  const unreceipted = (wire, call) => exclusive(async () => {
    if (wire !== undefined) await admitWire(wire);
    const turn = originalSource.issue();
    try { return await call(originalContextBudgetWit(turn)); }
    finally { const receipt = { input: turn.input, spent: { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 }, workUnitsSpent: 0 }; originalSource.receive(receipt); originalReturn.receipt(receipt); }
  });
  const pairWire = (pair) => ({ pack: pair.pack, spr: pair.spr });
  const codecCall = (method, wire) => typeof codec?.[method] === "function" ? unreceipted(wire, (budget) => codec[method](budget)) : Promise.reject(new Error("actor-codec.unexported"));
  const codecExports = {
    packSchemaHash: (artifactSchema) => codecCall("packSchemaHash", { operation: "pack-schema-hash", artifactSchema }),
    genesis: (artifactSchema, documentId) => codecCall("genesis", { operation: "genesis", artifactSchema, documentId }),
    printMirror: (artifactSchema, pair) => codecCall("printMirror", { operation: "print-mirror", artifactSchema, pair: pairWire(pair) }),
  };
  async function receiveContext(method) {
    const turn = originalSource.issue();
    const call = await reactor[method](originalContextBudgetWit(turn));
    receiveOriginal(call, call.workUnitsSpent);
    if (optionValue(call.refusal) !== undefined) throw Object.assign(new Error("original-turn.context-refused"), { originalCall: call });
    return call;
  }
  async function admitContext() {
    while (!contextReady) {
      const call = await receiveContext("admitActorContext");
      contextReady = call.ready;
      if (!contextReady && call.spent.copiedItems === 0n && call.spent.copiedBytes === 0n && call.spent.retainedCapacityBytes === 0n && call.spent.releasedBytes === 0n) throw Object.assign(new Error("original-turn.context-held"), { originalCall: call });
    }
  }
  const channelRefusal = admitGuestChannelVersion(await reactor.channelVersion(), HOST_CHANNEL);
  if (channelRefusal !== null) throw Object.assign(new Error(`${channelRefusal.code}: ${channelRefusal.message}`), { fault: channelRefusal });
  return {
    originalSource,
    closeOriginal: async () => {
      if (originalSource.outstanding || originalSource.nativeRefusal !== undefined || originalSource.producerRefusal !== undefined || originalSource.receiptRefusal !== undefined) throw new Error("original-turn.original-return-pending");
      for (;;) {
        const call = await receiveContext("closeActorContext");
        if (call.complete) {
          const acknowledgement = originalSource.issue();
          if (acknowledgement.input.grant.maximumItems < 1 || acknowledgement.input.grant.maximumDepth < 1 || acknowledgement.fuel < 1) throw new Error("original-turn.source-acknowledgement-held");
          const receipt = { input: acknowledgement.input, spent: { copiedItems: 1, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 }, workUnitsSpent: 1 };
          originalSource.receive(receipt);
          originalReturn.receipt(receipt);
          originalSource.close();
          return { operation: originalSource.operation, generation: originalSource.generation, sourceClosed: true };
        }
        if (call.blocked || (call.spent.copiedItems === 0n && call.spent.copiedBytes === 0n && call.spent.retainedCapacityBytes === 0n && call.spent.releasedBytes === 0n)) return { operation: originalSource.operation, generation: originalSource.generation, sourceClosed: false };
      }
    },
    poll: (events, commandPage, coldPairPage, budget) => exclusive(async () => {
      await admitContext();
      if (events.length > 1) throw new Error("actor-poll.event-count: a reactor turn admits at most one event");
      const wireEvents = events.map(({ kind, payload }) => lifecycleEvent(kind, payload, activationGeneration));
      await admitWire({ operation: "poll", events: wireEvents.map(actorInputWire.event), shape: { maxEffects: budget.maxEffects, maxPatchBytes: budget.maxPatchBytes, maxFrames: budget.maxFrames } });
      const turn = originalSource.issue();
      hostShim.__checkoutOriginal(turn);
      let result;
      try {
        if (commandPage) await reactor.stageCommandPage(commandPage.cursor, commandPage.bytes);
        if (coldPairPage) await reactor.stageColdPairPage(coldPairPage);
        const slice = originalContextBudgetWit(turn);
        if (typeof budget.fuel === "number" && budget.fuel > 0 && BigInt(budget.fuel) < slice.fuel) slice.fuel = BigInt(budget.fuel);
        const __t0 = performance.now();
        result = await reactor.poll(slice);
        { const ms = performance.now() - __t0; globalThis.__n = (globalThis.__n ?? 0) + 1; if (ms > 6 && (globalThis.__slow = (globalThis.__slow ?? 0) + 1) <= 40) console.log('[PROBE] poll#' + globalThis.__n + ' ms=' + ms.toFixed(1) + ' epoch=' + turn.input.epoch); if (globalThis.__n % 500 === 0) console.log('[PROBE] polls=' + globalThis.__n + ' slow=' + (globalThis.__slow ?? 0)); }
      } catch (failure) {
        try {
          const receipt = failure?.payload ?? failure;
          if (receipt && receipt.input && receipt.spent && typeof receipt.workUnitsSpent === "bigint") receiveOriginal(receipt, receipt.workUnitsSpent);
          if (originalSource.nativeRefusal === undefined) { originalSource.holdNativeRefusal(failure); originalReturn.hold(failure); }
        } finally {
          const raw = hostShim.__returnOriginal(turn);
          if (raw !== undefined) { originalSource.holdProducerRefusal(raw); originalReturn.hold(raw); }
        }
        throw nativeRefusalError(failure) ?? failure;
      }
      let raw;
      try { receiveOriginal(result.retainedReceipt, result.retainedReceipt.workUnitsSpent); }
      finally { raw = hostShim.__returnOriginal(turn); if (raw !== undefined) { originalSource.holdProducerRefusal(raw); originalReturn.hold(raw); } }
      if (raw !== undefined) throw Object.assign(new Error("original-turn.producer-held"), { originalProducer: raw });
      return { ...result, nextWake: optionValue(result.nextWake) ?? null, lifecycleReceipt: lifecycleReceipt(result.lifecycleReceipt, activationGeneration), uiPatchReceipt: uiPatchReceipt(result, activationGeneration), commandIngress: normalizeCommandIngress(result.commandIngress) };
    }),
    startJob: async (job, kind, input) => fundedJob(undefined, (funds) => funds.start({ reactor, jobs }, job, kind, input)),
    stepJob: async (job, budget, control = {}) => fundedJob(budget, (funds) => funds.step(jobs, job, { ...control, held: heldOutput })),
    cancelJob: async (job) => fundedJob(undefined, (funds) => funds.cancel(jobs, job, { held: heldOutput })),
    takeSegmentedDownloadChunk: async (instanceId, operationId) => optionValue(await jobs.takeSegmentedDownloadChunk(instanceId, operationId)),
    checkpoint: () => unreceipted(undefined, (budget) => checkpoint.checkpoint(budget)),
    restore: (state) => unreceipted({ operation: "restore", state }, (budget) => checkpoint.restore(budget)),
    describe: () => unreceipted(undefined, (budget) => describe.describe(budget)),
    codec: (request) => codecAnswer(codecExports, request),
    resolveEffect: (requestId, value) => hostShim.__resolveEffect(requestId, value),
    rejectEffect: (requestId, message) => hostShim.__rejectEffect(requestId, message),
  };
}
