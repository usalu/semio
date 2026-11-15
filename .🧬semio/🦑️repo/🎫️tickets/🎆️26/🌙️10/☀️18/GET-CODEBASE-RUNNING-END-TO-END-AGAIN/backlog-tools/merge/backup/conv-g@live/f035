import {createHash} from "node:crypto";
import {access, lstat, readlink, realpath, stat} from "node:fs/promises";
import * as os from "node:os";
import {resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

const policy = schema.definitions.Policy.const;

const ambientNamesV1 = new Set(policy.ambientEnvironment.names.map(name => name.toUpperCase())), ambientPrefixesV1 = policy.ambientEnvironment.prefixes.map(prefix => prefix.toUpperCase());

/** 🌫️ Tells whether a name is an ambient host, agent or session variable: the terminal, the desktop session, the IDE and the agent that started the build are not inputs of any plan or action, whatever their values are, so they leave no observation and never break a witness. Names are compared without regard to case, as Windows does. */
export function isAmbientEnvironmentNameV1(name) { const upper = name.toUpperCase(); return ambientNamesV1.has(upper) || ambientPrefixesV1.some(prefix => upper.startsWith(prefix)); }

/** 🔭️ The environment variable that switches plan reuse: `on` (default), `off`, or `verify`, which plans completely every time and reports when the reused plan would have differed; anything else is refused in both languages. */
export const WORLD_WITNESS_SWITCH_V1 = "SEMIO_REPO_PLAN_REUSE";

/** 🎛️ Reads the plan reuse switch; an unknown value is a refusal, never a silent default. */
export function readWorldWitnessSwitchV1(environment = process.env) {
 const raw = environment[WORLD_WITNESS_SWITCH_V1] ?? "on", value = raw.toLowerCase();
 if (value === "on" || value === "off" || value === "verify") return value;
 throw Error(`Expected ${WORLD_WITNESS_SWITCH_V1}=on|off|verify, got "${raw}" / Erwartet ${WORLD_WITNESS_SWITCH_V1}=on|off|verify, erhalten "${raw}"`);
}

/** 💬️ Renders a message or a reason in one declared language; there is no hidden default language. */
export function renderWorldWitnessTextV1(table, language, name, values = {}) {
 const messages = schema.definitions[table].const[language];
 if (messages === undefined || typeof messages[name] !== "string") throw Error("Declared language and message required");
 return messages[name].replace(/\{(\w+)\}/gu, (_match, key) => String(values[key] ?? ""));
}

/** 🕵️ Records, inside the process that runs it, every fact of the world one computation read: stat identities, directory identities, file identities, entry kinds asked through `entry`, link targets, real paths, environment digests and host facts; whatever it cannot record (spawned processes, network, native readers, volatile host facts, writes outside the declared places) it names as `tainted`, so a witness that is not tainted is a complete list of what the computation looked at. The function is self-contained: its source text is the preload of the planner child. */
export function installWorldWitnessV1(configText) {
 const config = JSON.parse(configText), load = (name) => process.getBuiltinModule(name);
 const fs = load("node:fs"), fsp = load("node:fs/promises"), os = load("node:os"), cp = load("node:child_process"), path = load("node:path"), crypto = load("node:crypto"), net = load("node:net"), http = load("node:http"), https = load("node:https"), url = load("node:url");
 const original = {statSync: fs.statSync, writeFileSync: fs.writeFileSync, renameSync: fs.renameSync, rmSync: fs.rmSync, lstat: fsp.lstat, stat: fsp.stat};
 const rows = new Map(), unstable = [], tainted = [], ownedFiles = new Set(), ownedDirectories = new Set(), written = new Map();
 let racy = null, sealed = false, allowance = 0, environmentEnumerated = false;
 const infrastructure = [path.resolve(config.workspace), path.resolve(config.root, ".cache")], writable = config.writable.map(value => path.resolve(value));
 const within = (child, parent) => child === parent || child.startsWith(parent + path.sep);
 const absolute = (value) => {
  if (typeof value === "string") return path.resolve(value);
  if (value instanceof URL) return value.protocol === "file:" ? url.fileURLToPath(value) : undefined;
  if (value !== null && typeof value === "object" && typeof value.length === "number" && Buffer.isBuffer(value)) return path.resolve(value.toString());
  return undefined;
 };
 const owned = (target) => {
  if (ownedFiles.has(target)) return true;
  for (let current = path.dirname(target), depth = 0; depth < 64; depth++, current = path.dirname(current)) { if (ownedDirectories.has(current)) return true; if (path.dirname(current) === current) break; }
  return false;
 };
 const probeFiles = new Set(config.hostProbeFiles), probeDirectories = new Set(config.hostProbeDirectories), probes = config.hostProbes.map(row => [row.program, ...row.arguments]), placeholders = {"{pid}": /^[0-9]{1,10}$/u, "{pids}": /^[0-9]{1,10}(,[0-9]{1,10})*$/u};
 const skipped = (target) => infrastructure.some(parent => within(target, parent)) || owned(target) || probeFiles.has(target);
 const callerOf = () => { const frame = String(new Error().stack).split("\n").slice(3).find(line => !line.includes("world-witness") && !line.includes("node:") && !line.includes("native:") && line.includes("/")); return frame === undefined ? "" : " @ " + frame.trim().replace(/^at\s+/u, "").slice(-160); };
 const taint = (reason) => { if (sealed || tainted.length >= 64) return; const located = reason + callerOf(); if (!tainted.some(known => known.startsWith(reason))) tainted.push(located); };
 const put = (kind, subject, value) => {
  if (sealed) return;
  const key = kind + "\0" + subject, known = rows.get(key);
  if (known === undefined) { if (rows.size >= config.maximumObservations) { taint("more than " + config.maximumObservations + " observations"); return; } rows.set(key, value); }
  else if (known !== value && unstable.length < 64) unstable.push(kind + " " + subject);
 };
 const identity = (facts, target) => {
  const big = typeof facts.mtimeNs === "bigint", modified = big ? facts.mtimeNs : facts.mtimeMs, changed = big ? facts.ctimeNs : facts.ctimeMs;
  const latest = Math.max(Number(big ? modified / 1000000n : modified), Number(big ? changed / 1000000n : changed));
  if (racy === null && !sealed && Math.floor(latest) > Date.now() - config.racyMilliseconds) racy = target;
  return [facts.dev, facts.ino, facts.mode, facts.size, modified, changed].join(":");
 };
 const failure = (error) => "!" + (error !== undefined && error !== null && typeof error.code === "string" ? error.code : "ERR");
 const wrapped = Symbol.for("semio.world-witness.wrapped");
 const patch = (target, name, make) => {
  const descriptor = Object.getOwnPropertyDescriptor(target, name), function_ = descriptor !== undefined && typeof descriptor.get === "function" ? descriptor.get.call(target) : target[name];
  if (typeof function_ !== "function" || function_[wrapped]) return;
  const replacement = make(function_);
  Object.defineProperty(replacement, wrapped, {value: true});
  try {
   if (descriptor !== undefined && typeof descriptor.get === "function") Object.defineProperty(target, name, {get: () => replacement, configurable: true, enumerable: descriptor.enumerable});
   else target[name] = replacement;
  } catch { taint("cannot observe " + name); }
 };
 const observeResult = (kind, subject, outcome) => { const target = absolute(subject); if (target === undefined || skipped(target)) return; put(kind, target, typeof outcome === "function" ? outcome(target) : outcome); };
 const resultKinds = {lstat: "l", stat: "s", readlink: "r", realpath: "p"};
 const recordResult = (target, key, value, error, bigint) => {
  if (key === "lstat" || key === "stat") { const kind = bigint ? resultKinds[key].toUpperCase() : resultKinds[key]; observeResult(kind, target, at => error !== undefined ? failure(error) : value === undefined ? "!ENOENT" : identity(value, at)); }
  else observeResult(resultKinds[key], target, error !== undefined ? failure(error) : String(value));
 };
 const wrapResult = (target, name, key) => patch(target, name, function_ => function (...args) {
  const subject = args[0], bigint = args[1] !== null && typeof args[1] === "object" && args[1].bigint === true, last = args[args.length - 1];
  if (typeof last === "function") { args[args.length - 1] = (error, value) => { recordResult(subject, key, value, error ?? undefined, bigint); return last(error, value); }; return function_.apply(this, args); }
  let outcome;
  try { outcome = function_.apply(this, args); } catch (error) { recordResult(subject, key, undefined, error, bigint); throw error; }
  if (outcome !== null && typeof outcome === "object" && typeof outcome.then === "function") return outcome.then(value => { recordResult(subject, key, value, undefined, bigint); return value; }, error => { recordResult(subject, key, undefined, error, bigint); throw error; });
  recordResult(subject, key, outcome, undefined, bigint);
  return outcome;
 });
 const observeBefore = (kind, subject) => {
  const target = absolute(subject);
  if (target === undefined || skipped(target)) return;
  try { put(kind, target, identity(original.statSync(target), target)); } catch (error) { put(kind, target, failure(error)); }
 };
 const wrapBefore = (target, name, kind, select = (args) => args[0]) => patch(target, name, function_ => function (...args) { observeBefore(kind, select(args)); return function_.apply(this, args); });
 const writeFlags = (flags) => typeof flags === "string" ? /[wa+]/u.test(flags) : typeof flags === "number" ? (flags & (fs.constants.O_WRONLY | fs.constants.O_RDWR | fs.constants.O_APPEND | fs.constants.O_CREAT | fs.constants.O_TRUNC)) !== 0 : false;
 const markWrite = (subject, directory = false) => {
  const target = absolute(subject);
  if (target === undefined) return;
  if (!infrastructure.some(parent => within(target, parent)) && !writable.some(parent => within(target, parent)) && !owned(target)) { taint("wrote outside the declared places: " + target); return; }
  (directory ? ownedDirectories : ownedFiles).add(target);
 };
 const wrapWrite = (target, name, positions = [0], directory = false) => patch(target, name, function_ => function (...args) { for (const position of positions) markWrite(args[position], directory); return function_.apply(this, args); });
 for (const module_ of [fs, fsp]) {
  for (const name of ["lstat", "stat", "lstatSync", "statSync"]) wrapResult(module_, name, name.replace("Sync", ""));
  for (const name of ["readlink", "readlinkSync"]) wrapResult(module_, name, "readlink");
  for (const name of ["realpath", "realpathSync"]) wrapResult(module_, name, "realpath");
  for (const name of ["readdir", "readdirSync", "opendir", "opendirSync"]) patch(module_, name, function_ => function (...args) {
   const options = args[1];
   if (options !== null && typeof options === "object" && options.recursive === true) taint("recursive listing of " + String(args[0]));
   observeBefore("d", args[0]);
   return function_.apply(this, args);
  });
  for (const name of ["readFile", "readFileSync", "createReadStream"]) wrapBefore(module_, name, "f");
  for (const name of ["copyFile", "copyFileSync"]) patch(module_, name, function_ => function (...args) { observeBefore("f", args[0]); markWrite(args[1]); return function_.apply(this, args); });
  for (const name of ["open", "openSync"]) patch(module_, name, function_ => function (...args) { if (writeFlags(args[1])) markWrite(args[0]); else observeBefore("f", args[0]); return function_.apply(this, args); });
  for (const name of ["access", "accessSync"]) patch(module_, name, function_ => function (...args) {
   const mode = typeof args[1] === "number" ? args[1] : 0, last = args[args.length - 1], subject = args[0];
   if (typeof last === "function") { args[args.length - 1] = (error) => { observeResult("a" + mode, subject, error ? "0" : "1"); return last(error); }; return function_.apply(this, args); }
   let outcome;
   try { outcome = function_.apply(this, args); } catch (error) { observeResult("a" + mode, subject, "0"); throw error; }
   if (outcome !== null && typeof outcome === "object" && typeof outcome.then === "function") return outcome.then(value => { observeResult("a" + mode, subject, "1"); return value; }, error => { observeResult("a" + mode, subject, "0"); throw error; });
   observeResult("a" + mode, subject, "1");
   return outcome;
  });
  for (const name of ["writeFile", "writeFileSync", "appendFile", "appendFileSync", "createWriteStream", "unlink", "unlinkSync", "rm", "rmSync", "rmdir", "rmdirSync", "chmod", "chmodSync", "utimes", "utimesSync", "lutimes", "lutimesSync", "chown", "chownSync"]) wrapWrite(module_, name);
  for (const name of ["mkdir", "mkdirSync", "mkdtemp", "mkdtempSync"]) wrapWrite(module_, name, [0], true);
  for (const name of ["rename", "renameSync"]) wrapWrite(module_, name, [0, 1]);
  for (const name of ["symlink", "symlinkSync", "link", "linkSync"]) wrapWrite(module_, name, [1]);
  for (const name of ["cp", "cpSync", "glob", "globSync", "watch", "watchFile", "truncate", "truncateSync"]) patch(module_, name, function_ => function (...args) { taint("unobserved " + name + " of " + String(args[0])); return function_.apply(this, args); });
 }
 patch(fs, "existsSync", function_ => function (...args) { const outcome = function_.apply(this, args); observeResult("a0", args[0], outcome ? "1" : "0"); return outcome; });
 if (typeof fs.realpathSync === "function" && typeof fs.realpathSync.native === "function") patch(fs.realpathSync, "native", function_ => function (...args) { try { const outcome = function_.apply(this, args); recordResult(args[0], "realpath", outcome); return outcome; } catch (error) { recordResult(args[0], "realpath", undefined, error); throw error; } });
 const environmentWrites = new Set(), authority = new Set(config.authorityEnvironment), ignoredNames = new Set([...config.authorityEnvironment, ...config.ignoredEnvironment]), ambientNames = new Set(config.ambientEnvironment.names.map(name => name.toUpperCase())), ambientPrefixes = config.ambientEnvironment.prefixes.map(prefix => prefix.toUpperCase());
 const ignoredForDigest = { has: name => { if (ignoredNames.has(name)) return true; const upper = name.toUpperCase(); return ambientNames.has(upper) || ambientPrefixes.some(prefix => upper.startsWith(prefix)); } };
 const digestOfEnvironment = () => Object.keys(process.env).filter(name => !ignoredForDigest.has(name)).sort().map(name => name + "=" + crypto.createHash("sha256").update(String(process.env[name])).digest("hex").slice(0, 8)).join("\n");
 const observeEnvironment = (name, target) => { if (environmentWrites.has(name) || ignoredForDigest.has(name)) return; const value = target[name]; put("e", name, value === undefined ? "-" : crypto.createHash("sha256").update(value).digest("hex")); };
 const environment = new Proxy(process.env, {
  get(target, key) { if (typeof key === "string") observeEnvironment(key, target); return target[key]; },
  has(target, key) { if (typeof key === "string") observeEnvironment(key, target); return key in target; },
  getOwnPropertyDescriptor(target, key) { if (typeof key === "string") observeEnvironment(key, target); return Reflect.getOwnPropertyDescriptor(target, key); },
  set(target, key, value) { if (typeof key === "string") environmentWrites.add(key); target[key] = value; return true; },
  deleteProperty(target, key) { if (typeof key === "string") environmentWrites.add(key); return delete target[key]; },
  ownKeys(target) { environmentEnumerated = true; return Reflect.ownKeys(target); }
 });
 try { process.env = environment; } catch { taint("cannot observe the environment"); }
 for (const name of ["homedir", "tmpdir", "hostname", "platform", "arch", "release", "type", "version", "machine", "endianness", "availableParallelism", "totalmem"]) patch(os, name, function_ => function (...args) { const outcome = function_.apply(this, args); put("h", name, String(outcome)); return outcome; });
 patch(os, "userInfo", function_ => function (...args) { const outcome = function_.apply(this, args); put("h", "username", String(outcome.username)); return outcome; });
 patch(os, "cpus", function_ => function (...args) { const outcome = function_.apply(this, args); put("h", "cpus", String(outcome.length)); return outcome; });
 for (const name of config.volatileHostFacts) patch(os, name, function_ => function (...args) { taint("volatile host fact " + name); return function_.apply(this, args); });
 const commandOf = (label, args) => {
  const first = args[0];
  if (label === "exec" || label === "execSync" || label === "fork" || label === "Bun.$") return undefined;
  if (Array.isArray(first)) return first.map(String);
  if (first !== null && typeof first === "object" && Array.isArray(first.cmd)) return first.cmd.map(String);
  return typeof first === "string" ? [first, ...(Array.isArray(args[1]) ? args[1].map(String) : [])] : undefined;
 };
 const hostProbe = (label, args) => {
  const command = commandOf(label, args);
  if (command === undefined || command.length === 0 || (command[0].includes("/") && !probeDirectories.has(path.dirname(command[0])))) return false;
  const program = path.basename(command[0]);
  const given = [program, ...command.slice(1)];
  return probes.some(row => row.length === given.length && row.every((token, at) => token === given[at] || (placeholders[token] !== undefined && placeholders[token].test(given[at]))));
 };
 const spawned = (label) => function_ => function (...args) { if (allowance <= 0 && !hostProbe(label, args)) { const command = commandOf(label, args); taint("spawned " + label + " " + (command !== undefined ? command.slice(0, 8).map(part => part.slice(0, 60)).join(" ") : String(typeof args[0] === "string" ? args[0] : ""))); } return function_.apply(this, args); };
 for (const name of ["spawn", "spawnSync", "exec", "execSync", "execFile", "execFileSync", "fork"]) patch(cp, name, spawned(name));
 if (typeof Bun !== "undefined") {
  for (const name of ["spawn", "spawnSync"]) patch(Bun, name, spawned("Bun." + name));
  patch(Bun, "$", spawned("Bun.$"));
  for (const name of ["which", "resolve", "resolveSync", "build", "write"]) patch(Bun, name, function_ => function (...args) { if (name === "write") markWrite(typeof args[0] === "string" || args[0] instanceof URL ? args[0] : undefined); else taint("unobserved Bun." + name); return function_.apply(this, args); });
  patch(Bun, "file", function_ => function (...args) { if (typeof args[0] === "string" || args[0] instanceof URL) observeBefore("f", args[0]); else if (args[0] !== 1 && args[0] !== 2) taint("unobserved Bun.file of " + (args[0] === null ? "null" : typeof args[0] === "object" ? (args[0].constructor && args[0].constructor.name) : typeof args[0] + " " + String(args[0]))); return function_.apply(this, args); });
  try { const glob = Bun.Glob; if (glob !== undefined && glob.prototype !== undefined) for (const name of ["scan", "scanSync"]) patch(glob.prototype, name, function_ => function (...args) { taint("unobserved Bun.Glob." + name); return function_.apply(this, args); }); } catch { taint("cannot observe Bun.Glob"); }
 }
 if (typeof globalThis.fetch === "function") patch(globalThis, "fetch", function_ => function (...args) { taint("network fetch"); return function_.apply(this, args); });
 for (const [module_, label] of [[http, "http"], [https, "https"]]) for (const name of ["request", "get"]) patch(module_, name, function_ => function (...args) { taint("network " + label + "." + name); return function_.apply(this, args); });
 for (const name of ["connect", "createConnection"]) patch(net, name, function_ => function (...args) { const first = args[0], socket = first !== null && typeof first === "object" ? first.path : typeof first === "string" ? first : undefined; if (typeof socket !== "string" || !infrastructure.some(parent => within(path.resolve(socket), parent))) taint("network connection"); return function_.apply(this, args); });
 try {
  const sqlite = load("bun:sqlite");
  if (sqlite !== undefined && typeof sqlite.Database === "function" && !sqlite.Database[wrapped]) {
   const Base = sqlite.Database;
   const Guarded = class extends Base { constructor(...args) { const file = args[0]; if (typeof file === "string" && file !== ":memory:" && file !== "") { const target = path.resolve(file); if (!infrastructure.some(parent => within(target, parent))) taint("opened a database outside the declared places: " + target); } else if (typeof file !== "string" && file !== undefined) taint("opened an unnamed database"); super(...args); } };
   Object.defineProperty(Guarded, wrapped, {value: true});
   sqlite.Database = Guarded;
  }
 } catch { /* a runtime without bun:sqlite has nothing to observe */ }
 patch(process, "chdir", function_ => function (...args) { taint("changed the working directory"); return function_.apply(this, args); });
 const api = Object.freeze({
  version: 1,
  allow(label, body) {
   allowance++;
   let outcome;
   try { outcome = body(); } catch (error) { allowance--; throw error; }
   if (outcome !== null && typeof outcome === "object" && typeof outcome.then === "function") return outcome.finally(() => { allowance--; });
   allowance--;
   return outcome;
  },
  observe(kind, subject, value) { put(kind, subject, value); },
  async entry(subject, follow) {
   const target = absolute(subject);
   let value;
   try { const facts = await (follow ? original.stat : original.lstat)(target); value = facts.isSymbolicLink() ? "link" : facts.isFile() ? "file" : facts.isDirectory() ? "directory" : "other"; } catch (error) { value = failure(error); }
   observeResult(follow ? "K" : "k", target, value);
   return value;
  },
  taint(reason) { taint(reason); },
  finish() {
   if (sealed) return;
   if (environmentEnumerated) {
    if ([...environmentWrites].some(name => !ignoredForDigest.has(name))) taint("the environment was changed after it was enumerated");
    else rows.set("E\0*", digestOfEnvironment());
   }
   const observations = [...rows].map(([key, value]) => { const split = key.indexOf("\0"); return [key.slice(0, split), key.slice(split + 1), value]; });
   sealed = true;
   const reasons = [...tainted];
   const body = JSON.stringify({version: 1, observations, racy, unstable, tainted: reasons}), temporary = config.out + "." + process.pid + ".tmp";
   original.writeFileSync(temporary, body, {mode: 0o600});
   original.renameSync(temporary, config.out);
  }
 });
 Object.defineProperty(globalThis, "__semioWorldWitnessV1", {value: api, enumerable: false});
 return api;
}

/** 📜️ The source text of the preload that installs the witness in a child process: the module header, the self-contained function and its call with the configuration the parent passed in the declared environment variable, which is removed before any other code runs. */
export function worldWitnessPreloadV1() {
 return `const configText = process.env.${policy.config};\ndelete process.env.${policy.config};\nif (configText !== undefined) (${installWorldWitnessV1.toString()})(configText);\n`;
}

/** 🧾️ The environment a child needs to run under the witness; the output file receives the finished witness. */
export function worldWitnessEnvironmentV1(options) {
 const config = {out: options.out, root: options.root, workspace: options.workspace, writable: options.writable, racyMilliseconds: options.racyMilliseconds ?? policy.racyMilliseconds, maximumObservations: policy.maximumObservations, volatileHostFacts: policy.volatileHostFacts, hostProbes: options.hostProbes ?? policy.hostProbes, hostProbeDirectories: policy.hostProbeDirectories, hostProbeFiles: options.hostProbeFiles ?? policy.hostProbeFiles, authorityEnvironment: policy.authorityEnvironment, ignoredEnvironment: policy.ignoredEnvironment, ambientEnvironment: policy.ambientEnvironment};
 return {[policy.config]: JSON.stringify(config)};
}

/** 🔭️ The witness of the process that runs here, when it was started under the preload; otherwise nothing. */
export function worldWitnessV1() {
 const api = globalThis.__semioWorldWitnessV1;
 return api !== undefined && api.version === 1 ? api : undefined;
}

/** 📖️ Admits a witness file the planner child wrote; anything malformed is refused. */
export function readWorldWitnessV1(text) {
 if (Buffer.byteLength(text) > policy.maximumBytes) throw Error("Bounded world witness required");
 const value = JSON.parse(text);
 if (value === null || typeof value !== "object" || value.version !== 1 || !Array.isArray(value.observations) || !Array.isArray(value.tainted) || !Array.isArray(value.unstable) || (value.racy !== null && (typeof value.racy !== "string" || value.racy === ""))) throw Error("Closed world witness required");
 for (const row of value.observations) if (!Array.isArray(row) || row.length !== 3 || row.some(part => typeof part !== "string")) throw Error("Closed world observation required");
 return Object.freeze(value);
}

const digestOf = (value) => createHash("sha256").update(value).digest("hex");
const failureOf = (error) => "!" + (error !== null && typeof error === "object" && typeof error.code === "string" ? error.code : "ERR");
const entryKindOf = (facts) => facts.isSymbolicLink() ? "link" : facts.isFile() ? "file" : facts.isDirectory() ? "directory" : "other";

/** 🧱️ Reads only the kind of an entry (`file`, `directory`, `link`, `other`, or `!CODE` when it cannot be read; `follow` resolves a link first): under the witness it leaves a kind row (`k`, `K`) that holds however the bytes, times or mode of the entry change, so a computation that only depends on which paths exist and what they are is not invalidated by a content edit; without the witness it asks the file system directly. */
export async function worldEntryKindV1(path, follow = false) {
 const witness = worldWitnessV1();
 if (witness !== undefined && typeof witness.entry === "function") return witness.entry(path, follow);
 try { return entryKindOf(await (follow ? stat : lstat)(path)); } catch (error) { return failureOf(error); }
}

const identityOf = (facts) => {
 const big = typeof facts.mtimeNs === "bigint";
 return [facts.dev, facts.ino, facts.mode, facts.size, big ? facts.mtimeNs : facts.mtimeMs, big ? facts.ctimeNs : facts.ctimeMs].join(":");
};
const hostFact = (name) => {
 if (name === "username") return os.userInfo().username;
 if (name === "cpus") return os.cpus().length;
 if (name === "availableParallelism") return typeof os.availableParallelism === "function" ? os.availableParallelism() : os.cpus().length;
 if (name === "machine") return typeof os.machine === "function" ? os.machine() : "";
 return os[name]();
};

async function checkOne(kind, subject, expected, ports, environment) {
 try {
  if (kind === "l" || kind === "L" || kind === "s" || kind === "S") {
   const big = kind === "L" || kind === "S";
   return identityOf(await (kind === "l" || kind === "L" ? lstat : stat)(subject, {bigint: big})) === expected;
  }
  if (kind === "d" || kind === "f") return identityOf(await stat(subject)) === expected;
  if (kind === "k" || kind === "K") return entryKindOf(await (kind === "k" ? lstat : stat)(subject)) === expected;
  if (kind === "r") return (await readlink(subject)) === expected;
  if (kind === "p") return (await realpath(subject)) === expected;
  if (kind[0] === "a") { try { await access(subject, Number(kind.slice(1))); return expected === "1"; } catch { return expected === "0"; } }
  if (kind === "e") { if (isAmbientEnvironmentNameV1(subject)) return true; const value = environment[subject]; return (value === undefined ? "-" : digestOf(value)) === expected; }
  if (kind === "h") return String(hostFact(subject)) === expected;
  if (kind === "E") {
   const ignored = new Set([...policy.authorityEnvironment, ...policy.ignoredEnvironment]), now = Object.keys(environment).filter(name => !ignored.has(name) && !isAmbientEnvironmentNameV1(name)).sort().map(name => name + "=" + digestOf(String(environment[name])).slice(0, 8));
   if (now.join("\n") === expected) return true;
   const before = new Set(expected.split("\n")), after = new Set(now), moved = [...new Set([...before].filter(row => !after.has(row)).concat([...after].filter(row => !before.has(row))).map(row => row.split("=")[0]))];
   return "changed: " + moved.slice(0, 12).join(", ");
  }
  if (kind === "g") return ports?.git !== undefined && await ports.git(subject, expected);
  if (kind === "i") return ports?.ignore !== undefined && await ports.ignore(subject, expected);
  if (kind === "x") return ports?.runtime !== undefined && await ports.runtime(subject, expected);
  return false;
 } catch (error) {
  return kind.length === 1 && "lLsSdfkK".includes(kind) ? failureOf(error) === expected : false;
 }
}

/** ✅️ Tells whether every observation of a witness still holds: the same answers from the same questions asked now, in bounded parallel batches, stopping at the first that differs; stat-like observations that failed before must fail the same way again. A witness holds exactly when the computation that wrote it would read the same world and therefore compute the same result. */
export async function verifyWorldWitnessV1(witness, options) {
 const rows = witness.observations, concurrency = options.concurrency ?? policy.concurrency, environment = options.environment ?? process.env;
 let next = 0, checked = 0, broken;
 const lane = async () => {
  while (broken === undefined) {
   options.signal?.throwIfAborted();
   const position = next++;
   if (position >= rows.length) return;
   const [kind, subject, expected] = rows[position];
   const outcome = await checkOne(kind, subject, expected, options.ports, environment);
   if (outcome !== true) { broken ??= {kind, subject: typeof outcome === "string" ? subject + " " + outcome : subject}; return; }
   checked++;
   if (checked % policy.progressObservations === 0) await options.progress?.(checked);
  }
 };
 await Promise.all(Array.from({length: Math.min(concurrency, Math.max(1, rows.length))}, lane));
 options.signal?.throwIfAborted();
 return broken === undefined ? Object.freeze({holds: true, checked}) : Object.freeze({holds: false, checked, kind: broken.kind, subject: broken.subject});
}

/** 🧮️ Digests the tracked-file listing and the ignore files a git universe depends on; the caller passes the output of the listing so the same function serves recording and checking. */
export function worldWitnessGitDigestV1(listing) { return digestOf(listing); }

/** 🗂️ The names the observations of a witness group under, with their counts: the shape a report or a law can read. */
export function summarizeWorldWitnessV1(witness) {
 const counts = {};
 for (const [kind] of witness.observations) counts[kind] = (counts[kind] ?? 0) + 1;
 return Object.freeze(counts);
}

/** 🔒️ Resolves a path the way the witness records it. */
export function worldWitnessPathV1(value) { return resolve(value); }
