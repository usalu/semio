#!/usr/bin/env bun
/** 🩺️Reconstructs this task's rename evidence without restoring historical file contents. */
import { existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync, renameSync, readdirSync } from "node:fs";
import { createHash } from "node:crypto";
import { basename, dirname, join, posix } from "node:path";
import { strict as assert } from "node:assert";
import emojiRegex from "emoji-regex";

const root = process.cwd();
const ticket = dirname(import.meta.dir);
const generated = join(ticket, "🗑️generated", "🩺️repair");
const session = "/Users/ueli/.codex/sessions/2026/09/03/rollout-2026-09-03T18-16-50-01a0680f-071e-7a31-aa35-410ec3443065.jsonl";
const baseline = "03100691d5";
const command = Bun.argv[2] ?? "inventory";
if (command !== "test") throw new Error("Retired after the recovery batches. All further naming and reference repairs must be individually reviewed, handpicked, and applied as precise edits; no batch mutation commands are permitted.");
const stemCache = new Map<string, string>();
const git = (...args: string[]): string => {
  const result = Bun.spawnSync(["git", ...args], { cwd: root, maxBuffer: 128 * 1024 * 1024 });
  if (result.exitCode) throw new Error(result.stderr.toString());
  return result.stdout.toString();
};
const stem = (name: string): string => {
  if (stemCache.has(name)) return stemCache.get(name)!;
  let rest = name.normalize("NFC");
  for (;;) {
    const match = emojiRegex().exec(rest);
    if (match?.index !== 0) { stemCache.set(name, rest); return rest; }
    rest = rest.slice(match[0].length).replace(/^[\uFE0E\uFE0F]+/u, "");
  }
};
const identity = (path: string): string => path.split("/").map(stem).join("/");
const indexed = (paths: string[]): Map<string, string[]> => {
  const output = new Map<string, string[]>();
  for (const path of paths) {
    const key = identity(path), values = output.get(key) ?? [];
    if (!values.includes(path)) values.push(path);
    output.set(key, values);
  }
  return output;
};
const withParents = (paths: string[]): string[] => {
  const result = new Set(paths);
  for (const path of paths) for (let parent = dirname(path); parent !== "." && parent !== ""; parent = dirname(parent)) result.add(parent);
  return [...result];
};
const replace = (text: string, pairs: [string, string][]): string => {
  const replacements = new Map(pairs);
  const escaped = [...replacements.keys()].sort((a, b) => b.length - a.length).map(value => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
  return escaped.length ? text.replace(new RegExp(escaped.join("|"), "gu"), match => replacements.get(match)!) : text;
};
const compile = (pairs: [string, string][]): ((text: string) => { text: string; count: number }) => {
  type Node = { next: Map<string, Node>; value?: string };
  const root: Node = { next: new Map() };
  for (const [source, destination] of pairs) {
    let node = root;
    for (let index = 0; index < source.length; index++) {
      const unit = source[index]!;
      if (!node.next.has(unit)) node.next.set(unit, { next: new Map() });
      node = node.next.get(unit)!;
    }
    node.value = destination;
  }
  return text => {
    const candidates = /[^\x00-\x7F]/g, chunks: string[] = [];
    let cursor = 0, count = 0, candidate: RegExpExecArray | null;
    while ((candidate = candidates.exec(text))) {
      let node = root, index = candidate.index, end = index, value: string | undefined;
      while (index < text.length && node.next.has(text[index]!)) {
        node = node.next.get(text[index++]!)!;
        if (node.value !== undefined) { value = node.value; end = index; }
      }
      if (value === undefined) continue;
      chunks.push(text.slice(cursor, candidate.index), value);
      cursor = end; count++; candidates.lastIndex = end;
    }
    return { text: count ? chunks.join("") + text.slice(cursor) : text, count };
  };
};
if (command === "test") {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧪️cases.json"), "utf8"));
  const Ajv = (await import("ajv")).default;
  const ajv = new Ajv();
  for (const test of fixture.cases) {
    const actual = replace(test.input, test.pairs);
    assert.equal(actual, test.expected, test.name);
    assert.equal(compile(test.pairs)(test.input).text, test.expected, test.name);
    assert(ajv.compile({ const: test.expected })(actual), test.name);
  }
  assert.equal(stem("🧪️🧪️🏔️🦋️tests"), "tests");
  assert.equal(stem("🌳️bäume"), "bäume");
  console.log(JSON.stringify({ cases: fixture.cases.length, oracle: "Ajv exact-output equality", result: "passed" }));
} else if (command === "inventory") {
  mkdirSync(generated, { recursive: true });
  const rows = readFileSync(session, "utf8").trim().split("\n").map(line => JSON.parse(line));
  const recorded = rows.find(row => row.timestamp === "2026-09-04T00:41:00.366Z")?.payload.item.stdout;
  const findings = [...recorded.matchAll(/\{\s*"kind":\s*"[^"]+",\s*"path":\s*"(?:[^"\\]|\\.)*"[^{}]*\}/gu)].flatMap(match => { try { return [JSON.parse(match[0])]; } catch { return []; } });
  const audit = { findings, evidence: "Complete finding objects recovered from this task's recorded output; truncated tail is excluded." };
  const exactEvidence = new Set(audit.findings.filter((finding: any) => finding.kind !== "oracle").map((finding: any) => identity(finding.path)));
  const original = withParents(git("ls-tree", "-r", "--name-only", "-z", baseline).split("\0").filter(Boolean));
  const inventory = [...new Set(git("ls-files", "-co", "--exclude-standard", "-z").split("\0").filter(Boolean))].filter(path => existsSync(path));
  const current = withParents(inventory);
  const oldByIdentity = indexed(original), currentByIdentity = indexed(current);
  const moves: { source: string; destination: string; directory: boolean; evidence: string }[] = [];
  const conflicts: unknown[] = [];
  for (const [key, paths] of oldByIdentity) {
    if (key.startsWith(".🧬semio/") || key.startsWith(".semio/")) continue;
    const now = currentByIdentity.get(key);
    if (!now) continue;
    if (paths.length !== 1 || now.length !== 1) {
      if (paths.some(path => !now.includes(path))) conflicts.push({ key, old: paths, current: now });
      continue;
    }
    if (basename(paths[0]!) === basename(now[0]!)) continue;
    moves.push({ source: now[0]!, destination: paths[0]!, directory: lstatSync(now[0]!).isDirectory(), evidence: exactEvidence.has(key) ? "recorded-second-pass-finding" : "historical-path-and-current-stem" });
  }
  const mappings = new Map<string, Set<string>>();
  for (const move of moves) {
    const name = basename(move.source), names = mappings.get(name) ?? new Set<string>();
    names.add(basename(move.destination)); mappings.set(name, names);
  }
  const document = { baseline, audit, moves, conflicts, ambiguousNames: [...mappings].filter(([, names]) => names.size > 1).map(([name, names]) => ({ name, originals: [...names] })), pairs: [...mappings].filter(([, names]) => names.size === 1).map(([name, names]) => [name, [...names][0]]), inventory };
  writeFileSync(join(generated, "inventory.json"), JSON.stringify(document, null, 2));
  console.log(JSON.stringify({ moves: moves.length, recorded: moves.filter(move => move.evidence.startsWith("recorded")).length, conflicts: conflicts.length, ambiguousNames: document.ambiguousNames, pairs: document.pairs.length, samples: moves.filter(move => !move.evidence.startsWith("recorded")).slice(0,25) }, null, 2));
} else if (command === "preview-recovery" || command === "apply-recovery") {
  if (existsSync(join(generated, "apply-recovery.json"))) throw new Error("This recovery batch has already been applied. Never replay a historical rename batch.");
  const evidence = JSON.parse(readFileSync(join(generated, "inventory.json"), "utf8"));
  const oldNames = new Set(withParents(git("ls-tree", "-r", "--name-only", "-z", baseline).split("\0").filter(Boolean)).map(path => basename(path)));
  const multiple = (name: string): boolean => [...name.slice(0, name.length - stem(name).length).matchAll(emojiRegex())].length > 1;
  const reserved = (name: string): boolean => /^(?:README(?:\..+)?|LICENSE(?:\..+)?|package\.json|tsconfig\.json|\.vscode-test\.mjs|route\.ts|Trunk\.toml)$/u.test(stem(name));
  const moves = evidence.moves.filter((move: any) => multiple(basename(move.source)) || reserved(basename(move.source)));
  const selectedNames = new Set(moves.map((move: any) => basename(move.source)));
  const pairs = evidence.pairs.filter(([source]: [string, string]) => selectedNames.has(source) && !oldNames.has(source));
  const conflicts = moves.filter((move: any) => existsSync(join(dirname(move.source), basename(move.destination))));
  if (conflicts.length) throw new Error(`Recovery destination conflicts: ${JSON.stringify(conflicts.slice(0,10))}`);
  const reverse = compile(pairs);
  const changed: { path: string; before: string; after: string; replacements: number }[] = [];
  const skipped: string[] = [];
  const digest = (bytes: Buffer): string => createHash("sha256").update(bytes).digest("hex");
  const inventory = [...new Set(git("ls-files", "-co", "--exclude-standard", "-z").split("\0").filter(Boolean))];
  const stash = join(generated, "current-bytes");
  if (command === "apply-recovery") mkdirSync(stash, { recursive: true });
  for (const path of inventory) {
    if (path.startsWith(".🧬semio/") || path.startsWith(".🧬️semio/") || basename(path) === "AGENTS.md") continue;
    let stat;
    try { stat = lstatSync(path); } catch { continue; }
    if (!stat.isFile() || stat.size > 8 * 1024 * 1024) continue;
    const bytes = readFileSync(path);
    if (bytes.includes(0)) continue;
    const text = bytes.toString("utf8");
    if (!Buffer.from(text).equals(bytes)) { skipped.push(path); continue; }
    const repaired = reverse(text), updated = repaired.text, replacements = repaired.count;
    if (updated === text) continue;
    const after = Buffer.from(updated), beforeHash = digest(bytes);
    changed.push({ path, before: beforeHash, after: digest(after), replacements });
    if (command === "apply-recovery") {
      const backup = join(stash, beforeHash);
      if (!existsSync(backup)) writeFileSync(backup, bytes, { flag: "wx" });
      if (!readFileSync(path).equals(bytes)) throw new Error(`Concurrent content edit: ${path}`);
      writeFileSync(path, after);
      if (!readFileSync(path).equals(after)) throw new Error(`Content verification failed: ${path}`);
    }
  }
  const completed: unknown[] = [];
  if (command === "apply-recovery") {
    for (const move of moves.sort((a: any,b: any) => b.source.split("/").length-a.source.split("/").length || b.source.localeCompare(a.source))) {
      const destination = join(dirname(move.source), basename(move.destination));
      if (!existsSync(move.source) || existsSync(destination)) throw new Error(`Concurrent path edit: ${move.source}`);
      const before = lstatSync(move.source);
      renameSync(move.source, destination);
      const after = lstatSync(destination);
      if (before.ino !== after.ino || before.size !== after.size) throw new Error(`Rename verification failed: ${destination}`);
      completed.push({ source: move.source, destination });
    }
  }
  const report = { mode: command, moves, completed, pairs, changed, skipped };
  writeFileSync(join(generated, `${command}.json`), JSON.stringify(report, null, 2));
  console.log(JSON.stringify({ mode: command, moves: moves.length, completed: completed.length, pairs: pairs.length, changed: changed.length, replacements: changed.reduce((total,row) => total+row.replacements,0), skipped }, null, 2));
} else if (command === "semantic-preview" || command === "semantic-apply") {
  const evidence = JSON.parse(readFileSync(join(generated, "inventory.json"), "utf8"));
  const pairs: [string,string][] = evidence.pairs.filter(([, original]: [string,string]) => stem(original) === original);
  const reverse = compile(pairs);
  const literals = (text: string): { raw: string; index: number }[] => [...text.matchAll(/"(?:[^"\\\r\n]|\\.)*"|'(?:[^'\\\r\n]|\\.)*'/gu)].map(match => ({ raw: match[0], index: match.index! }));
  const tree = git("ls-tree", "-r", "-z", baseline).split("\0").filter(Boolean).map(row => { const tab = row.indexOf("\t"); return { path: row.slice(tab+1), hash: row.slice(0,tab).split(" ")[2]! }; });
  const treeByIdentity = new Map<string, typeof tree>();
  for (const row of tree) { const key = identity(row.path), rows = treeByIdentity.get(key) ?? []; rows.push(row); treeByIdentity.set(key,rows); }
  const candidates: { path: string; original: string; hash: string }[] = [];
  for (const path of [...new Set(git("ls-files", "-co", "--exclude-standard", "-z").split("\0").filter(Boolean))]) {
    if (path.startsWith(".🧬semio/") || path.startsWith(".🧬️semio/") || basename(path) === "AGENTS.md") continue;
    const original = treeByIdentity.get(identity(path));
    if (original?.length !== 1) continue;
    let stat; try { stat = lstatSync(path); } catch { continue; }
    if (!stat.isFile() || stat.size > 8*1024*1024) continue;
    const bytes = readFileSync(path);
    if (bytes.includes(0)) continue;
    const text = bytes.toString("utf8");
    if (!Buffer.from(text).equals(bytes)) continue;
    if (literals(text).some(literal => reverse(literal.raw).count)) candidates.push({ path, original: original[0]!.path, hash: original[0]!.hash });
  }
  const edits: { path: string; before: string; after: string; tokens: { before: string; after: string; index: number }[] }[] = [];
  const unresolved: { path: string; literal: string; reason: string }[] = [];
  for (let start = 0; start < candidates.length; start += 80) {
    const batch = candidates.slice(start,start+80);
    const response = Bun.spawnSync(["git", "cat-file", "--batch"], { cwd: root, stdin: Buffer.from(batch.map(item => item.hash).join("\n")+"\n"), maxBuffer: 128*1024*1024 });
    if (response.exitCode) throw new Error(response.stderr.toString());
    let offset = 0;
    for (const item of batch) {
      const eol = response.stdout.indexOf(10,offset), header = response.stdout.subarray(offset,eol).toString(), size = Number(header.split(" ")[2]);
      if (!Number.isFinite(size)) throw new Error(`Invalid historical blob header: ${header}`);
      const old = response.stdout.subarray(eol+1,eol+1+size).toString("utf8"); offset=eol+1+size+1;
      const oldLiterals = new Map<string, Set<string>>();
      for (const literal of literals(old)) { const key = reverse(literal.raw).text, choices = oldLiterals.get(key) ?? new Set<string>(); choices.add(literal.raw); oldLiterals.set(key,choices); }
      const bytes = readFileSync(item.path), current = bytes.toString("utf8"), replacements: { before:string; after:string; index:number }[] = [];
      for (const literal of literals(current)) {
        const reverted = reverse(literal.raw);
        if (!reverted.count) continue;
        const choices = oldLiterals.get(reverted.text);
        if (!choices?.has(reverted.text) || choices.size !== 1) {
          if (!literal.raw.includes("/")) unresolved.push({ path:item.path,literal:literal.raw,reason: choices?.size ? "ambiguous-historical-literal" : "no-historical-literal" });
          continue;
        }
        const value = literal.raw.slice(1,-1);
        if (value.includes("/") && (existsSync(value) || existsSync(join(dirname(item.path),value)) || /^\.{1,2}\//u.test(value))) continue;
        replacements.push({before:literal.raw,after:reverted.text,index:literal.index});
      }
      if (!replacements.length) continue;
      let updated = current;
      for (const edit of [...replacements].reverse()) updated=updated.slice(0,edit.index)+edit.after+updated.slice(edit.index+edit.before.length);
      const before=createHash("sha256").update(bytes).digest("hex"),after=createHash("sha256").update(updated).digest("hex");
      edits.push({path:item.path,before,after,tokens:replacements});
      if (command === "semantic-apply") {
        const backup=join(generated,"current-bytes",before);
        if(!existsSync(backup))writeFileSync(backup,bytes,{flag:"wx"});
        if(!readFileSync(item.path).equals(bytes))throw new Error(`Concurrent semantic edit: ${item.path}`);
        writeFileSync(item.path,updated);
      }
    }
    process.stderr.write(`semantic evidence ${Math.min(start+80,candidates.length)}/${candidates.length}\n`);
  }
  writeFileSync(join(generated,`${command}.json`),JSON.stringify({edits,unresolved},null,2));
  console.log(JSON.stringify({files:edits.length,tokens:edits.reduce((sum,row)=>sum+row.tokens.length,0),unresolved:unresolved.length,samples:edits.slice(0,10).map(row=>({path:row.path,tokens:row.tokens.slice(0,3)}))},null,2));
} else if (["handpicked-preview", "handpicked-apply", "reserved-preview", "reserved-apply"].includes(command)) {
  const reserved = command.startsWith("reserved-"), apply = command.endsWith("-apply");
  const groups = reserved ? [] : JSON.parse(readFileSync(join(import.meta.dir,"🖐️names.json"),"utf8")).groups;
  const allMoves: {source:string;destination:string}[] = reserved ? JSON.parse(readFileSync(join(import.meta.dir,"🔒️reserved.json"),"utf8")).moves : groups.flatMap((group:any) => group.names.map((name:any) => ({source:posix.join(group.parent,name.from),destination:posix.join(group.parent,name.to)})));
  const moves = allMoves.filter(move => existsSync(move.source) && move.source !== move.destination);
  for(const group of groups) {
    const prefixes=new Map<string,string>();
    for(const child of readdirSync(group.parent).filter(name=>!name.startsWith("."))) {
      const target=group.names.find((entry:any)=>entry.from===child)?.to??child;
      const emojis=[...target.matchAll(emojiRegex())];
      assert(emojis.length===1&&emojis[0]!.index===0,`Review every sibling: ${group.parent}/${target}`);
      const prefix=emojis[0]![0].replaceAll("\uFE0F","");
      assert(!prefixes.has(prefix),`Handpicked sibling collision: ${group.parent}/${target} and ${prefixes.get(prefix)}`);
      prefixes.set(prefix,target);
    }
  }
  const project = (path:string):string => {
    const move = [...moves].sort((a,b)=>b.source.length-a.source.length).find(move=>path===move.source||path.startsWith(move.source+"/"));
    return move ? move.destination+path.slice(move.source.length) : path;
  };
  for(const move of moves) {
    assert(!existsSync(move.destination),`Destination exists: ${move.destination}`);
    if (reserved) {
      assert(basename(move.source)==="🟥️src" && basename(move.destination)==="src" && dirname(move.source)===dirname(move.destination) && existsSync(join(dirname(move.source),"Cargo.toml")), `Unproven Cargo source convention: ${move.source}`);
      continue;
    }
    const emojis=[...basename(move.destination).matchAll(emojiRegex())];
    assert(emojis.length===1&&emojis[0]!.index===0,`Exactly one leading emoji required: ${move.destination}`);
    assert(!["📁","📂","📄"].includes(emojis[0]![0].replaceAll("\uFE0F","")),`Generic emoji: ${move.destination}`);
  }
  const changed: any[] = [], unresolved:any[]=[];
  for(const path of [...new Set(git("ls-files","-co","--exclude-standard","-z").split("\0").filter(Boolean))]) {
    if(path.startsWith(".🧬semio/")||path.startsWith(".🧬️semio/")||basename(path)==="AGENTS.md")continue;
    let stat;try{stat=lstatSync(path)}catch{continue}
    if(!stat.isFile()||stat.size>8*1024*1024)continue;
    const bytes=readFileSync(path);if(bytes.includes(0))continue;
    const content=bytes.toString("utf8");if(!Buffer.from(content).equals(bytes))continue;
    if(!moves.some(move=>content.includes(basename(move.source))))continue;
    const replacements:any[]=[];
    let updated=content.replace(/(["'`])([^"'`\r\n]+)\1/gu,(literal,quote:string,value:string,index:number)=>{
      if(!moves.some(move=>value.includes(basename(move.source))))return literal;
      let replacement:string|undefined;
      if(project(value)!==value)replacement=project(value);
      else {
        const resolved=new Map<string,string>();
        for(let base=dirname(path);;base=dirname(base)) {
          const target=posix.normalize(posix.join(base,value)),next=project(target);
          if(next!==target&&existsSync(target)) {
            let specifier=posix.relative(project(base),next);
            if(value.startsWith("./")&&!specifier.startsWith("."))specifier="./"+specifier;
            resolved.set(next,specifier);
          }
          if(base===".")break;
        }
        if(resolved.size===1)replacement=[...resolved.values()][0];
      }
      if(replacement===undefined){unresolved.push({path,literal});return literal}
      replacements.push({before:value,after:replacement,index});return quote+replacement+quote;
    });
    for(const move of moves)updated=updated.replaceAll(move.source+"/",move.destination+"/");
    if(updated===content)continue;
    const before=createHash("sha256").update(bytes).digest("hex"),after=createHash("sha256").update(updated).digest("hex");
    changed.push({path,before,after,replacements});
    if(apply) {
      const backup=join(generated,"current-bytes",before);if(!existsSync(backup))writeFileSync(backup,bytes,{flag:"wx"});
      if(!readFileSync(path).equals(bytes))throw new Error(`Concurrent handpicked reference edit: ${path}`);
      writeFileSync(path,updated);
    }
  }
  if(apply)for(const move of [...moves].sort((a,b)=>b.source.length-a.source.length)) {
    assert(!existsSync(move.destination),`Concurrent destination: ${move.destination}`);
    const before=lstatSync(move.source);renameSync(move.source,move.destination);assert.equal(lstatSync(move.destination).ino,before.ino);
  }
  const report=JSON.stringify({moves,changed,unresolved},null,2);
  writeFileSync(join(generated,`${command}-${Date.now()}.json`),report,{flag:"wx"});
  writeFileSync(join(generated,`${command}.json`),report);
  console.log(JSON.stringify({moves:moves.length,files:changed.length,unresolved},null,2));
} else throw new Error(`Unknown command: ${command}`);
