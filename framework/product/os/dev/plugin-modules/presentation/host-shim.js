/** @generated semio program host shim */
const BACKBONE_STORAGE_PREFIX = "semio.backbone.";

export function log(level, message) {
  if (level === "error") console.error(`[plugin] ${message}`);
  else console.log(`[plugin] ${message}`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function readDocument(handle) {
  throw `read-document unsupported: ${handle}`;
}

export function writeDocument(handle, payloadJson) {
  throw `write-document unsupported: ${handle}`;
}

export function openWindow(kind, paramsJson) {
  throw `open-window unsupported: ${kind}`;
}

export function invokeAction(target, invocationJson) {
  throw `invoke-action unsupported: ${target}`;
}

export function readAsset(handle) {
  throw `read-asset unsupported: ${handle}`;
}

export function networkFetch(origin, path) {
  throw `network-fetch unsupported: ${origin}${path}`;
}

function syncBackboneRequest(method, uri, body) {
  const request = new XMLHttpRequest();
  request.open(method, `/semio-backbone?uri=${encodeURIComponent(uri)}`, false);
  request.send(body);
  return request;
}

function readBackboneEntry(uri) {
  if (typeof localStorage !== "undefined") {
    return localStorage.getItem(`${BACKBONE_STORAGE_PREFIX}${uri}`);
  }
  const response = syncBackboneRequest("GET", uri, null);
  return response.status === 200 ? response.responseText : null;
}

function writeBackboneEntry(uri, payload) {
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(`${BACKBONE_STORAGE_PREFIX}${uri}`, payload);
    return;
  }
  const response = syncBackboneRequest("PUT", uri, payload);
  if (response.status !== 200) throw `backbone write failed: ${uri}`;
}

const backbonePolled = new Set();

/** @emoji 📨 Mirrors vcs::storage_receive — first poll after attach yields the stored snapshot, then goes quiet until the next send. */
export function backbonePoll(uri) {
  if (backbonePolled.has(uri)) return [];
  backbonePolled.add(uri);
  const stored = readBackboneEntry(uri);
  if (stored == null) return [];
  return [JSON.stringify({ kind: "snapshot", envelopeJson: stored })];
}

/** @emoji 📨 Mirrors vcs::storage_send — a snapshot message overwrites, an operations message appends into vcs.edits deduped by id. */
export function backboneSend(uri, messageJson) {
  const message = JSON.parse(messageJson);
  if (message.kind === "snapshot") {
    writeBackboneEntry(uri, message.envelopeJson);
    return;
  }
  if (message.kind === "operations") {
    const stored = readBackboneEntry(uri);
    if (stored == null) throw `cannot append operations before a snapshot exists: ${uri}`;
    const envelope = JSON.parse(stored);
    const edits = envelope?.vcs?.edits;
    if (!Array.isArray(edits)) throw `stored envelope missing vcs.edits: ${uri}`;
    const seen = new Set(edits.map((edit) => edit?.id).filter((id) => typeof id === "string"));
    for (const operationEnvelope of message.envelopes ?? []) {
      const editJson = operationEnvelope?.diff?.payload;
      const id = editJson?.id;
      if (typeof id === "string") {
        if (seen.has(id)) continue;
        seen.add(id);
      }
      edits.push(editJson);
    }
    writeBackboneEntry(uri, JSON.stringify(envelope));
    return;
  }
  throw `unsupported backbone message kind: ${message.kind}`;
}
