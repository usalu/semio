#!/usr/bin/env bun
/** 🔗️ Builds the golden hash-chain fixture with Node `crypto` — the third-party oracle for `semio.mcp.event/1`. */
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";

type Input = { kind: string; peer: string; generation: number; requestId?: string; payload: string };

const INPUTS: Input[] = [
  { kind: "session.opened", peer: "fixture", generation: 1, payload: '{"generation":1}' },
  { kind: "request.received", peer: "fixture", generation: 1, requestId: "n:1", payload: '{"jsonrpc":"2.0","id":1,"method":"ping","params":{}}' },
  { kind: "response.sent", peer: "fixture", generation: 1, requestId: "n:1", payload: '{"jsonrpc":"2.0","id":1,"result":{}}' },
  { kind: "notification.received", peer: "fixture", generation: 1, payload: '{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}' },
  { kind: "request.received", peer: "fixture", generation: 1, requestId: "s:tool", payload: '{"jsonrpc":"2.0","id":"tool","method":"tools/call","params":{"name":"echo","arguments":{"text":"a<b>c&d"}}}' },
  { kind: "response.sent", peer: "fixture", generation: 1, requestId: "s:tool", payload: '{"jsonrpc":"2.0","id":"tool","result":{"content":[{"type":"text","text":"a<b>c&d"}]}}' },
  { kind: "session.closed", peer: "fixture", generation: 1, payload: '{"reason":"closed"}' },
];

/** 🔤️ Escapes a string the way Go's `encoding/json` does, HTML escaping included. */
function goString(value: string): string {
  let out = '"';
  for (const character of value) {
    if (character === '"') out += '\\"';
    else if (character === "\\") out += "\\\\";
    else if (character === "\n") out += "\\n";
    else if (character === "\r") out += "\\r";
    else if (character === "\t") out += "\\t";
    else if (character === "<") out += "\\u003c";
    else if (character === ">") out += "\\u003e";
    else if (character === "&") out += "\\u0026";
    else if (character === "\u2028") out += "\\u2028";
    else if (character === "\u2029") out += "\\u2029";
    else if (character.codePointAt(0)! < 0x20) out += `\\u${character.codePointAt(0)!.toString(16).padStart(4, "0")}`;
    else out += character;
  }
  return out + '"';
}

/** 🗜️ Strips insignificant whitespace and applies the same HTML escaping to a JSON document. */
export function compact(raw: string): string {
  let out = "";
  let inString = false;
  let escaped = false;
  for (const character of raw) {
    if (inString) {
      if (escaped) escaped = false;
      else if (character === "\\") escaped = true;
      else if (character === '"') inString = false;
    } else if (character === '"') inString = true;
    else if (character === " " || character === "\t" || character === "\n" || character === "\r") continue;
    if (character === "<") out += "\\u003c";
    else if (character === ">") out += "\\u003e";
    else if (character === "&") out += "\\u0026";
    else if (character === "\u2028") out += "\\u2028";
    else if (character === "\u2029") out += "\\u2029";
    else out += character;
  }
  return out;
}

/** 🔗️ Renders one event record with the given hash value. */
export function render(event: { sequence: number; kind: string; peer: string; generation: number; requestId: string; payload: string; previous: string }, hash: string): string {
  let out = `{"schema":${goString("semio.mcp.event/1")},"sequence":${event.sequence},"kind":${goString(event.kind)},"peer":${goString(event.peer)},"generation":${event.generation}`;
  if (event.requestId !== "") out += `,"requestId":${goString(event.requestId)}`;
  out += `,"payload":${event.payload}`;
  if (event.previous !== "") out += `,"previous":${goString(event.previous)}`;
  return out + `,"hash":${goString(hash)}}`;
}

/** 🔗️ Builds the whole chain, hashing each record over its own rendering with an empty hash. */
export function chain(inputs: Input[]): { lines: string[]; hashes: string[] } {
  const lines: string[] = [];
  const hashes: string[] = [];
  let previous = "";
  inputs.forEach((input, index) => {
    const event = { sequence: index + 1, kind: input.kind, peer: input.peer, generation: input.generation, requestId: input.requestId ?? "", payload: compact(input.payload), previous };
    const hash = createHash("sha256").update(render(event, ""), "utf8").digest("hex");
    lines.push(render(event, hash));
    hashes.push(hash);
    previous = hash;
  });
  return { lines, hashes };
}

if (import.meta.main) {
  const { lines, hashes } = chain(INPUTS);
  writeFileSync(process.argv[2], JSON.stringify({ schema: "semio.mcp.event.chain/1", inputs: INPUTS, hashes, jsonl: lines.join("\n") + "\n" }, null, 2) + "\n");
  console.log(`[chain] ${hashes.length} events -> ${process.argv[2]}`);
}
