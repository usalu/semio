import { mkdirSync } from "node:fs";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import storageSchema from "../🧬️schema/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🧊️ Freezes a declaration recursively. */
const freeze = value => { if (value !== null && typeof value === "object") { for (const child of Object.values(value)) freeze(child); Object.freeze(value); } return value; };

/** 🧾️ The declared limits of deferred optional accounting and of worker grants: the schema file is the single source. */
export const OPTIONAL_LEAVES_POLICY_V1 = freeze(structuredClone(schema.$defs.Policy.const));
const P = OPTIONAL_LEAVES_POLICY_V1, IMMUTABLE = /(?:^|\/)ob\/[a-f0-9]{64}\.br$/u, optional = new RegExp(storageSchema.definitions.OptionalQuota.optionalPath, "u");

/** 🏷️ Whether a name relative to the inference namespace is an optional cache leaf. */
export function isOptionalLeafNameV1(name) { return typeof name === "string" && optional.test(name); }

const waitSync = milliseconds => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

/** 🧮️ Accounts optional leaves against the quota and the durable ledger in batches (a content addressed leaf that two workers produced is accounted once and never removed because of the other report): a leaf written inside a batch or by a worker process is admitted by the in-memory quota at once and owed to the ledger, which is paid by one publication when enough bytes, leaves or time have gathered (a crash loses only this bookkeeping, the next ledger opening measures the physical bytes). Without a batch every leaf is published through the ledger exactly as before. */
export function createOptionalAccountingV1({ ledger, quota, policy = P, clock = () => performance.now(), wait = waitSync }) {
  const stats = { published: 0, deferred: 0, adopted: 0, refused: 0, duplicates: 0, flushes: 0, postponed: 0 }, accepted = new Set(), declined = new Set();
  let bytes = 0, leaves = 0, since = 0;
  const owe = delta => { if (leaves === 0) since = clock(); bytes += delta; leaves++; };
  const flush = final => {
    if (leaves === 0) return true;
    const attempts = final ? policy.finalFlushAttempts : 1;
    for (let attempt = 0; attempt < attempts; attempt++) {
      if (attempt > 0) wait(policy.flushRetryMilliseconds);
      if (ledger.publish({ peakBytes: Math.max(bytes, 0), retainedDeltaBytes: bytes }, () => {})) { bytes = 0; leaves = 0; stats.flushes++; return true; }
    }
    stats.postponed++;
    return false;
  };
  const maybeFlush = () => { if (bytes >= policy.flushBytes || leaves >= policy.flushLeaves || clock() - since >= policy.flushMilliseconds) flush(false); };
  return Object.freeze({
    publish(plan, write, deferred = false, held) {
      const reservation = held ?? quota.reserve(plan);
      if (!reservation) return false;
      try {
        if (!deferred) {
          if (!ledger.publish(plan, write)) return false;
          reservation.commit();
          stats.published++;
          return true;
        }
        const value = write();
        if (value && typeof value.then === "function") throw Error("Synchronous optional publication required");
        reservation.commit();
      } finally { if (held === undefined) reservation.release(); }
      stats.deferred++;
      owe(plan.retainedDeltaBytes);
      maybeFlush();
      return true;
    },
    adopt(items, remove) {
      const taken = [], refused = [];
      for (const { name, bytes: size } of items) {
        if (!isOptionalLeafNameV1(name) || !Number.isSafeInteger(size) || size < 0 || size > policy.maximumLeafBytes) throw Error("Optional graph leaf name and size required: " + String(name));
        const immutable = IMMUTABLE.test(name);
        if (immutable && (accepted.has(name) || declined.has(name))) { stats.duplicates++; continue; }
        const reservation = quota.reserve({ peakBytes: size, retainedDeltaBytes: size });
        if (!reservation) { if (immutable) declined.add(name); remove(name); refused.push(name); stats.refused++; continue; }
        try { reservation.commit(); } finally { reservation.release(); }
        if (immutable) accepted.add(name);
        taken.push(name); stats.adopted++;
        owe(size);
      }
      if (taken.length) maybeFlush();
      return { accepted: taken, refused };
    },
    grant(size) {
      const reservation = quota.reserve({ peakBytes: size, retainedDeltaBytes: 0 });
      return reservation === undefined ? undefined : () => reservation.release();
    },
    flush: final => flush(final === true),
    facts: () => ({ ...stats, owedBytes: bytes, owedLeaves: leaves }),
  });
}

/** 🛰️ The writer scope of one graph worker job: fact directories are the shared graph directories the parent named, every optional leaf is written there directly within a byte grant, and the leaves are reported for the parent to account; a leaf outside the grant is declined like a leaf the quota refuses, and a write that finds its content addressed target already there answers `false` and reports nothing. */
export function createWorkerWriterV1({ roots, grantBytes }) {
  if (roots === null || typeof roots !== "object" || typeof roots.inference !== "string" || !isAbsolute(roots.inference) || !Number.isSafeInteger(grantBytes) || grantBytes < 0) throw Error("Absolute graph worker roots and a byte grant required");
  const inference = resolve(roots.inference), written = [], declined = [], receipts = new Set(), made = new Set(), presence = new Set();
  let used = 0;
  return Object.freeze({
    directory(namespace) {
      const root = roots[namespace];
      if (typeof root !== "string" || !isAbsolute(root)) throw Error("Declared graph worker namespace required: " + namespace);
      if (!made.has(namespace)) { mkdirSync(root, { recursive: true }); made.add(namespace); }
      return root;
    },
    publish(path, plan, write) {
      const name = relative(inference, resolve(path)).split(sep).join("/");
      if (name === "" || name.startsWith("..") || isAbsolute(name) || !isOptionalLeafNameV1(name)) throw Error("Explicit optional graph cache leaf required");
      if (used + plan.peakBytes > grantBytes) { declined.push(name); return false; }
      const value = write();
      if (value && typeof value.then === "function") throw Error("Synchronous optional publication required");
      if (value === false) return true;
      used += Math.max(plan.retainedDeltaBytes, 0);
      written.push({ name, bytes: Math.max(plan.retainedDeltaBytes, 0) });
      return true;
    },
    presence,
    defer(manifest) { receipts.add(manifest); },
    report: () => ({ leaves: written.slice(), declined: declined.slice(), receipts: [...receipts] }),
  });
}
