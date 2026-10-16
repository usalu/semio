import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const FOLDER = join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const ORIGINAL = join(FOLDER, "met-tools/time-travel-original.ts.txt");
const DRY = process.argv.includes("--dry");
const ns = (emoji: string, slug: string): string => emoji.replace(/️/gu, "") + "️" + slug;
const FILE = ns("🟦", "") + ".ts";

const H = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel");
const S = join(REPO, "✏️s/🔌️plugins/🧩️puzzle", ns("🧪", "tests"), ns("⏪", "time-travel"));
for (const dir of [H, join(REPO, "✏️s/🔌️plugins/🧩️puzzle")]) if (!existsSync(dir)) throw new Error(`missing ${dir}`);

const text = readFileSync(ORIGINAL, "utf8");
const sf = ts.createSourceFile("original.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const lineOf = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const statements = [...sf.statements];

const declNames = (statement: ts.Statement): string[] => {
  const out: string[] = [];
  if (ts.isVariableStatement(statement)) {
    for (const declaration of statement.declarationList.declarations) {
      if (ts.isIdentifier(declaration.name)) out.push(declaration.name.text);
      else if (ts.isObjectBindingPattern(declaration.name) || ts.isArrayBindingPattern(declaration.name)) for (const element of declaration.name.elements) if (ts.isBindingElement(element) && ts.isIdentifier(element.name)) out.push(element.name.text);
    }
  } else if ((ts.isFunctionDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name) out.push(statement.name.text);
  return out;
};

type ModuleKey = string;
type ModuleSpec = { readonly key: ModuleKey; readonly dir: string; readonly kind: "harness" | "scenario"; readonly emoji: string; readonly doc: string };
const mod = (key: string, kind: "harness" | "scenario", emoji: string, slug: string, doc: string): ModuleSpec => ({ key, kind, emoji, doc, dir: join(kind === "harness" ? H : S, ns(emoji, slug)) });

const MODULES: ModuleSpec[] = [
  mod("state", "harness", "🧠", "state", "The run state every harness module shares: the CLI-derived configuration, the clock, the current locale and step, and the browser handles. Reads are live bindings; every write goes through the setter beside the variable."),
  mod("scenario", "harness", "🧩", "scenario", "The contract a plugin scenario fulfils to run on the time-travel harness: its route, its steps and the hooks the harness calls around them."),
  mod("arguments", "harness", "🧭", "arguments", "The CLI configuration of one time-travel run: the flags it reads and the output files it opens."),
  mod("report", "harness", "🧾", "report", "The report of one run: verdicts, notes, the timeline and the ndjson stream."),
  mod("copy", "harness", "🌍", "copy", "The localized texts every verdict compares against, byte-equal with their Rust and TypeScript producers."),
  mod("console", "harness", "🎧", "console", "Console and page-error capture of every page and worker the run opens."),
  mod("page", "harness", "📄", "page", "The browser pages of a run: evaluation, polling, screenshots, devices and fresh peers."),
  mod("mirror", "harness", "🧊", "wgpu-mirror", "The wgpu shell's accessibility mirror and the band words it announces."),
  mod("chrome", "harness", "🧱", "chrome", "Shell chrome: DOM ids, dock panels, the boot wait and the tour dialog."),
  mod("history", "harness", "📜", "history", "The history panel: windowed rows, entries, row actions and the inventory."),
  mod("band", "harness", "🎗", "band", "The time-travel band, its trace, notices, chords and the finalize dialog."),
  mod("editor", "harness", "🖊", "editor", "The draft editor of a history session: its number controls, text inputs and focus."),
  mod("aria", "harness", "♿", "aria-oracle", "The structural ARIA oracle run over the band, the history panel and the prompt."),
  mod("replay", "harness", "🔁", "replay-arm", "The replay arm: an in-page recorder of the band while a replay runs."),
  mod("reprojection", "harness", "🔄", "reprojection", "Deferred reprojection of the document: steps, loads and remote changes."),
  mod("session", "harness", "🧵", "history-session", "History sessions shared by every scenario: opening the editor, the alternatives list and leaving a session."),
  mod("universal", "harness", "🌐", "universal", "The plugin-neutral universal step: the mirror's controls and verbs, one clean change and the journey over them."),
  mod("digest", "harness", "📟", "console-digest", "The console digest step: uncaught errors, hard faults and the notices of a locale."),
  mod("run", "harness", "🎪", "run", "One locale pass and the whole run with its markdown report."),
  mod("board", "scenario", "🎲", "board", "The board 2d surface the scenario drives: positions, selection, camera and pointer gestures."),
  mod("pcopy", "scenario", "🌍", "copy", "The board-specific localized texts, merged over the harness copy."),
  mod("drag", "scenario", "🌄", "drag-journey", "Steps 1 to 5: boot, drag, edit, accept and finalize of one drag mutation."),
  mod("reload", "scenario", "🔂", "reload-check", "The reload half of step 5: the folder archive, the reconnect band and the forgotten folder."),
  mod("alternatives", "scenario", "🌿", "alternatives", "Steps 6 to 8: undo, redo, alternatives and the fatal path."),
  mod("gaps", "scenario", "🧮", "gaps", "Steps 10 to 15: input controls, further edits, the lock warning and the phone and tablet journeys."),
  mod("lanes", "scenario", "🛣", "lanes", "Steps 16 to 19: long history, two peers, history steps, configuration and list inputs."),
  mod("interaction", "scenario", "🕹", "interaction", "Steps 20 to 23: row actions, the goal sentence, handles and the pointer journeys."),
];
const moduleOf = new Map(MODULES.map((m) => [m.key, m]));

const idsIn = (...ranges: (number | readonly [number, number])[]) => ranges.flatMap((range) => (typeof range === "number" ? [range] : Array.from({ length: range[1] - range[0] + 1 }, (_unused, i) => range[0] + i)));
const ASSIGN: Record<string, number[]> = {
  state: idsIn(10, 12, 13, [22, 36], 44, 45, [64, 75], 82, 83),
  arguments: idsIn(37, 38, 39),
  report: idsIn([40, 43], [46, 50]),
  copy: idsIn(51),
  console: idsIn([54, 63]),
  page: idsIn([76, 81], [84, 89]),
  mirror: idsIn(90, [92, 100], [103, 106]),
  chrome: idsIn(15, 16, 117, [127, 142]),
  history: idsIn([143, 171]),
  band: idsIn([172, 204]),
  editor: idsIn([205, 223]),
  aria: idsIn([224, 231]),
  replay: idsIn([232, 239]),
  reprojection: idsIn([240, 252]),
  session: idsIn(255, 256, [271, 274], 288, 289, 325, 262, 263),
  universal: idsIn([328, 334]),
  digest: idsIn(278),
  run: idsIn(336, 337, 338),
  board: idsIn(11, 14, 17, 21, 91, 101, 102, [107, 116], [118, 126]),
  pcopy: idsIn(52, 53),
  drag: idsIn(253, 254, [257, 261]),
  reload: idsIn(264, 265, 266, 267, 268),
  alternatives: idsIn(269, 270, 275, 276, 277),
  gaps: idsIn([279, 287], [290, 302]),
  lanes: idsIn([303, 314]),
  interaction: idsIn([315, 324], 326, 327),
};
const owner = new Map<number, ModuleKey>();
for (const [key, ids] of Object.entries(ASSIGN)) for (const id of ids) {
  if (owner.has(id)) throw new Error(`statement ${id} assigned twice`);
  owner.set(id, key);
}
const SKIP = new Set([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 18, 19, 20, 335, 339]);
statements.forEach((_statement, id) => {
  if (!owner.has(id) && !SKIP.has(id)) throw new Error(`statement ${id} (${declNames(statements[id]!).join(",")} @${lineOf(statements[id]!.getStart(sf))}) is unassigned`);
});

const STATE_VARS = new Set(["renderer", "serveOrigin", "only", "reloadSelected", "locales", "chordLocales", "explore", "folderAt", "longHistory", "OUT", "stamp", "base", "ndjsonPath", "t0", "cancelled", "currentLocale", "currentStep", "browser", "context", "page", "mod", "navigations", "expectedReloads", "probeReloadAt", "invalidatedBy", "wgpuSeedTerminology", "universalRoute", "universalEditor", "lastBootFault", "scenario"]);
const setterName = (name: string) => "set" + name[0]!.toUpperCase() + name.slice(1);
const identifiersOf = (node: ts.Node, out: Set<string>) => {
  if (ts.isIdentifier(node)) {
    const parent = node.parent;
    const property = (ts.isPropertyAccessExpression(parent) && parent.name === node) || (ts.isPropertyAssignment(parent) && parent.name === node) || (ts.isQualifiedName(parent) && parent.right === node) || (ts.isPropertySignature(parent) && parent.name === node) || (ts.isMethodSignature(parent) && parent.name === node);
    if (!property) out.add(node.text);
  }
  node.forEachChild((child) => identifiersOf(child, out));
};

const rebase = (spec: string, fromDir: string, toDir: string): string => {
  if (!spec.startsWith(".")) return spec;
  let rel = relative(toDir, resolve(fromDir, spec));
  if (!rel.startsWith(".")) rel = "./" + rel;
  return rel;
};

type ExternalImport = { readonly name: string; readonly spec: string; readonly typeOnly: boolean; readonly alias?: string };
const externals = new Map<string, ExternalImport>();
for (const statement of statements) {
  if (!ts.isImportDeclaration(statement) || !statement.importClause) continue;
  const spec = (statement.moduleSpecifier as ts.StringLiteral).text;
  const clause = statement.importClause;
  if (clause.namedBindings && ts.isNamedImports(clause.namedBindings)) for (const element of clause.namedBindings.elements) externals.set(element.name.text, { name: element.name.text, spec, typeOnly: clause.isTypeOnly || element.isTypeOnly });
}

const stripRegionLines = (value: string) => value.split("\n").filter((line) => !/^\s*\/\/\s*#(end)?region/u.test(line)).join("\n");
const REGION_RE = /^\s*\/\/\s*#(end)?region/u;

const rawStatement = (id: number): string => {
  const node = statements[id]!;
  const start = node.getStart(sf);
  const comments = (ts.getLeadingCommentRanges(text, node.getFullStart()) ?? []).filter((range) => !REGION_RE.test(text.slice(range.pos, range.end)));
  const lead = comments.map((range) => text.slice(range.pos, range.end)).join("\n");
  return (lead ? lead + "\n" : "") + text.slice(start, node.end);
};


//#region patches
type Patch = readonly [string, string];
const patches = new Map<number, Patch[]>();
const patch = (id: number, ...list: Patch[]) => patches.set(id, [...(patches.get(id) ?? []), ...list]);
const idOfName = (name: string): number => {
  const found = statements.findIndex((statement) => declNames(statement).includes(name));
  if (found < 0) throw new Error(`no statement declares ${name}`);
  return found;
};

patch(idOfName("serveOrigin"), ['let serveOrigin = "http://127.0.0.1:6012";', 'let serveOrigin = "";']);
patch(idOfName("configure"),
  ["const configure = (segments: readonly string[], defaultOutDir: string, signal: AbortSignal) => {\n", "const configure = (segments: readonly string[], defaultOutDir: string, signal: AbortSignal, scenarioDefinition: TimeTravelScenarioV1<object> | undefined) => {\n  scenario = scenarioDefinition;\n"],
  ['serveOrigin = new URL(flagValue(segments, "--serve") ?? DEFAULT_SERVES[renderer]).origin;', 'serveOrigin = new URL(flagValue(segments, "--serve") ?? defaultServe(scenarioDefinition, renderer)).origin;'],
  ['universalRoute = segments.includes("--universal") ? (flagValue(segments, "--serve") ?? DEFAULT_SERVES[renderer]) : null;', 'universalRoute = segments.includes("--universal") ? (flagValue(segments, "--serve") ?? defaultServe(scenarioDefinition, renderer)) : null;'],
);
patch(idOfName("BENIGN_RE"), ["activate-puzzle2d-react-prototype", "activate-[a-z0-9]+-react-prototype"]);
patch(idOfName("diagnose"), ["vitals: await vitals(), ", "...(await scenario?.diagnostics?.()), "]);
patch(idOfName("wgpuBootShape"), ['.filter((id) => id.startsWith("2d-")).length', '.filter((id) => id.startsWith(scenario?.bootWindowPrefix ?? "")).length']);
patch(idOfName("Editor"), ["dx: StepperRead | null; dy: StepperRead | null;", "numbers: Record<string, StepperRead | null>;"]);
patch(idOfName("editor"),
  ["the dx/dy controls and the targets list. */", "the scenario's number controls and the targets list. */"],
  ['(renderer === "wgpu" ? wgpuEditor() : reactEditor())', '(renderer === "wgpu" ? wgpuEditor(editorNumbers()) : reactEditor(editorNumbers()))'],
);
patch(idOfName("wgpuEditor"),
  ["const wgpuEditor = async (): Promise<Editor | null> => {", "const wgpuEditor = async (pointers: readonly string[]): Promise<Editor | null> => {"],
  ['    dx: number("dx"),\n    dy: number("dy"),\n', '    numbers: Object.fromEntries(pointers.map((pointer) => [pointer, number(pointer)])),\n'],
);
patch(idOfName("reactEditor"),
  ["const reactEditor = () =>\n  evalSafe(\n    () => {", "const reactEditor = (pointers: readonly string[]) =>\n  evalSafe(\n    (pointers) => {"],
  ['        dx: number("dx"),\n        dy: number("dy"),\n', '        numbers: Object.fromEntries(pointers.map((pointer) => [pointer, number(pointer)])),\n'],
  ["    null as Editor | null,\n  );", "    null as Editor | null,\n    pointers,\n  );"],
);
patch(idOfName("readsAsOpText"), ["(`create-node node { id=… }`)", "(`<mutation-kind> node { id=… }`)"]);
patch(idOfName("MirrorNode"), [" The board's positions, camera and selection are read from\n * `semioWgpuIntrospection.dumpBoard2d(windowId)`, the wgpu twin of React's `data-board-*` vitals.", ""]);
patch(idOfName("step9"), ["const step9 = async (_ctx: Ctx) => {", "const step9 = async (_ctx: object) => {"]);
patch(idOfName("stepU"), ["const stepU = async (_ctx: Ctx) => {", "const stepU = async (_ctx: object) => {"]);
patch(idOfName("runProbe"), ["for (const step of STEP_NUMBERS) {", "for (const step of stepNumbers()) {"]);
patch(idOfName("runLocale"),
  ["board: await wgpuBoard(), boardContract: WGPU_BOARD_CONTRACT, ", "...(await scenario?.exploreInventory?.()), "],
  ['fill(locale === "de" ? "Drehen" : "Rotate", ', 'fill(scenario?.paletteQuery[locale] ?? "", '],
  ["band: await band(), vitals: await vitals(), aria:", "band: await band(), ...(await scenario?.diagnostics?.()), aria:"],
  ["band: await band(), editor: await editor(), dialog, vitals: await vitals(), console:", "band: await band(), editor: await editor(), dialog, ...(await scenario?.diagnostics?.()), console:"],
  ["for (const step of ORDER) {", "for (const step of runOrder()) {"],
  ["await STEPS[step](ctx);", "await stepFor(step)(ctx);"],
);
//#endregion patches

//#region transforms
const applyPatches = (id: number, value: string): string => {
  let result = value;
  for (const [from, to] of patches.get(id) ?? []) {
    const pieces = result.split(from);
    if (pieces.length !== 2) throw new Error(`patch for statement ${id} (${declNames(statements[id]!).join(",")}) matched ${pieces.length - 1} times: ${from.slice(0, 80)}`);
    result = pieces.join(to);
  }
  return result;
};

const rewriteStateWrites = (value: string, label: string): string => {
  const snippet = ts.createSourceFile("snippet.ts", value, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const edits: { start: number; end: number; value: string }[] = [];
  const locals = new Set<string>();
  const collect = (node: ts.Node) => {
    if ((ts.isVariableDeclaration(node) || ts.isParameter(node) || ts.isBindingElement(node)) && ts.isIdentifier(node.name) && STATE_VARS.has(node.name.text)) locals.add(node.name.text);
    node.forEachChild(collect);
  };
  collect(snippet);
  const visit = (node: ts.Node) => {
    if (ts.isBinaryExpression(node) && node.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && node.operatorToken.kind <= ts.SyntaxKind.LastAssignment && ts.isIdentifier(node.left) && STATE_VARS.has(node.left.text) && !locals.has(node.left.text)) {
      const name = node.left.text;
      const right = node.right.getText(snippet);
      const operator = node.operatorToken.kind;
      let expression: string;
      if (operator === ts.SyntaxKind.EqualsToken) expression = right;
      else if (operator === ts.SyntaxKind.QuestionQuestionEqualsToken) expression = `${name} ?? ${right}`;
      else if (operator === ts.SyntaxKind.PlusEqualsToken) expression = `${name} + ${right}`;
      else if (operator === ts.SyntaxKind.MinusEqualsToken) expression = `${name} - ${right}`;
      else throw new Error(`${label}: unhandled assignment operator on ${name}`);
      if (!ts.isExpressionStatement(node.parent)) throw new Error(`${label}: assignment to ${name} is used as an expression`);
      edits.push({ start: node.getStart(snippet), end: node.end, value: `${setterName(name)}(${expression})` });
    }
    if ((ts.isPostfixUnaryExpression(node) || ts.isPrefixUnaryExpression(node)) && (node.operator === ts.SyntaxKind.PlusPlusToken || node.operator === ts.SyntaxKind.MinusMinusToken) && ts.isIdentifier(node.operand) && STATE_VARS.has(node.operand.text) && !locals.has(node.operand.text)) {
      if (!ts.isExpressionStatement(node.parent)) throw new Error(`${label}: increment of ${node.operand.text} is used as an expression`);
      edits.push({ start: node.getStart(snippet), end: node.end, value: `${setterName(node.operand.text)}(${node.operand.text} ${node.operator === ts.SyntaxKind.PlusPlusToken ? "+" : "-"} 1)` });
    }
    node.forEachChild(visit);
  };
  visit(snippet);
  edits.sort((a, b) => b.start - a.start);
  let result = value;
  for (const edit of edits) result = result.slice(0, edit.start) + edit.value + result.slice(edit.end);
  return result;
};

const rebaseSee = (value: string, fromDir: string, toDir: string): string => value.replace(/@see (\.{1,2}\/\S+)/gu, (_all, spec: string) => `@see ${rebase(spec, fromDir, toDir)}`);
//#endregion transforms

//#region assembly
type Piece = { readonly names: readonly string[]; text: string; readonly origin: number | null; readonly typeOnly?: boolean };
type Body = { readonly spec: ModuleSpec & { readonly file?: string }; pieces: Piece[]; readonly extraImports: ExternalImport[] };
const isScenarioKind = (key: ModuleKey): boolean => key === "sassembly" || moduleOf.get(key)?.kind === "scenario";
const captured: { prepare: string } = { prepare: "" };
const bodies = new Map<ModuleKey, Body>();
const fileOf = (spec: ModuleSpec): string => join(spec.dir, FILE);
for (const spec of MODULES) bodies.set(spec.key, { spec, pieces: [], extraImports: [] });

const ownerStatementIds = new Map<string, number>();
for (const [id] of statements.entries()) for (const name of declNames(statements[id]!)) ownerStatementIds.set(name, id);

const moduleDirOf = (key: ModuleKey): string => (key === "main" ? H : key === "sassembly" ? S : moduleOf.get(key)!.dir);
const piece = (id: number, key: ModuleKey, override?: string): Piece => {
  const base = override ?? rawStatement(id);
  const patched = applyPatches(id, base);
  const editorReads = isScenarioKind(key) ? patched.replace(/\b(value|ed|cEd)(\??)\.(dx|dy)\b/gu, "$1$2.numbers.$3").replace(/\(await editor\(\)\)(\??)\.(dx|dy)\b/gu, "(await editor())$1.numbers.$2") : patched;
  const rewritten = rewriteStateWrites(editorReads, `statement ${id}`);
  return { names: declNames(statements[id]!), text: rebaseSee(rewritten, H, moduleDirOf(key)), origin: id, typeOnly: ts.isTypeAliasDeclaration(statements[id]!) };
};

const extra = (key: ModuleKey, names: readonly string[], value: string, before?: string): void => {
  const body = bodies.get(key) ?? (key === "main" || key === "sassembly" ? bodies.get(key) : undefined);
  if (!body) throw new Error(`no body ${key}`);
  const created: Piece = { names, text: value, origin: null };
  if (before === undefined) body.pieces.push(created);
  else {
    const index = body.pieces.findIndex((candidate) => candidate.names.includes(before));
    if (index < 0) throw new Error(`extra ${names.join(",")}: no ${before} in ${key}`);
    body.pieces.splice(index, 0, created);
  }
};

for (const key of ["main", "sassembly"]) bodies.set(key, { spec: { key, kind: key === "main" ? "harness" : "scenario", emoji: "⏪", doc: "", dir: moduleDirOf(key) }, pieces: [], extraImports: [] });

//#region state module
const stateStatements = ASSIGN.state!.map((id) => statements[id]!);
for (const id of ASSIGN.state!) {
  const node = statements[id]!;
  const names = declNames(node);
  if (ts.isVariableStatement(node)) {
    const body = piece(id, "state");
    bodies.get("state")!.pieces.push(body);
    for (const name of names) if (STATE_VARS.has(name)) bodies.get("state")!.pieces.push({ names: [setterName(name)], text: `/** ✏️ Replaces \`${name}\`; the state module is the only place the binding is assigned. */\nconst ${setterName(name)} = (value: typeof ${name}): void => {\n  ${name} = value;\n};`, origin: null });
  } else bodies.get("state")!.pieces.push(piece(id, "state"));
}
void stateStatements;
bodies.get("state")!.pieces.push({ names: ["scenario"], text: "/** 🧩️ The scenario of the running pass; absent for a universal run. */\nlet scenario: TimeTravelScenarioV1<object> | undefined = undefined;", origin: null });
bodies.get("state")!.pieces.push({ names: ["setScenario"], text: "/** ✏️ Replaces `scenario`. */\nconst setScenario = (value: TimeTravelScenarioV1<object> | undefined): void => {\n  scenario = value;\n};", origin: null });
//#endregion state module

for (const [key, ids] of Object.entries(ASSIGN)) {
  if (key === "state" || key === "copy" || key === "pcopy") continue;
  for (const id of ids) bodies.get(key)!.pieces.push(piece(id, key));
}

//#region copy split
{
  const copyNode = statements[idOfName("COPY")]!;
  const variable = (copyNode as ts.VariableStatement).declarationList.declarations[0]!;
  const literal = (variable.initializer as ts.AsExpression).expression as ts.ObjectLiteralExpression;
  const PUZZLE_KEYS = new Set(["items", "drag", "createNode", "changeLocked", "rotateCommand", "scaleCommand", "rotate", "scale", "createNodeRow", "itemCount", "example", "exampleQuery", "manifest"]);
  const entries = new Map<string, { generic: string[]; puzzle: string[] }>();
  for (const localeProperty of literal.properties) {
    const locale = (localeProperty as ts.PropertyAssignment).name.getText(sf);
    const inner = (localeProperty as ts.PropertyAssignment).initializer as ts.ObjectLiteralExpression;
    const generic: string[] = [];
    const puzzle: string[] = [];
    for (const property of inner.properties) {
      const key = (property as ts.PropertyAssignment).name.getText(sf);
      const leading = text.slice(lineStartOfPos(property.getStart(sf)), property.getStart(sf));
      const chunk = leading + text.slice(property.getStart(sf), property.end) + ",";
      (PUZZLE_KEYS.has(key) ? puzzle : generic).push(chunk);
    }
    entries.set(locale, { generic, puzzle });
  }
  const leadStart = copyNode.getStart(sf);
  const header = (ts.getLeadingCommentRanges(text, copyNode.getFullStart()) ?? []).filter((range) => !REGION_RE.test(text.slice(range.pos, range.end))).map((range) => text.slice(range.pos, range.end)).join("\n");
  void leadStart;
  const genericHeader = header.replace("\n * drag/create labels (`✋️drag-selection`, `🌱create-node`), history-edit rows", "\n * history-edit rows");
  if (genericHeader === header) throw new Error("COPY header anchor not found");
  const genericText = `${genericHeader ? genericHeader + "\n" : ""}const COPY = {\n${[...entries].map(([locale, rows]) => `  ${locale}: {\n${rows.generic.join("\n")}\n  },`).join("\n")}\n} as const;`;
  bodies.get("copy")!.pieces.push({ names: ["COPY"], text: genericText, origin: idOfName("COPY") });
  const puzzleText = `/** 🌍️ The harness copy with the board-specific labels merged over it (drag, create, lock, rotate, scale and the example commands). */\nconst COPY = {\n${[...entries].map(([locale, rows]) => `  ${locale}: {\n    ...FRAMEWORK_COPY.${locale},\n${rows.puzzle.join("\n")}\n  },`).join("\n")}\n} as const;`;
  bodies.get("pcopy")!.pieces.unshift({ names: ["COPY"], text: puzzleText, origin: null });
  for (const id of ASSIGN.pcopy!) bodies.get("pcopy")!.pieces.push(piece(id, "pcopy"));
}
function lineStartOfPos(pos: number): number {
  return text.lastIndexOf("\n", pos - 1) + 1;
}
//#endregion copy split

//#region routeUrl
{
  const body = bodies.get("arguments")!;
  const index = body.pieces.findIndex((candidate) => candidate.names.includes("routeUrl"));
  body.pieces[index] = { names: ["routeUrl"], origin: idOfName("routeUrl"), text: "/** 🌐️ The route of the serve under test: the universal route, else the scenario's playground at the serve origin. */\nconst routeUrl = () => universalRoute ?? `${serveOrigin}/?plugin=${scenario?.variant ?? \"\"}`;" };
  extra("arguments", ["defaultServe"], "/** 🧭️ The serve a run drives when `--serve` is absent: the scenario's default for `renderer`. */\nconst defaultServe = (scenarioDefinition: TimeTravelScenarioV1<object> | undefined, renderer: Renderer): string => {\n  const serve = scenarioDefinition?.defaultServes[renderer];\n  if (serve === undefined) throw new Error(\"a run without a scenario names its serve with --serve\");\n  return serve;\n};", "configure");
}
//#endregion routeUrl

//#region editor extras
extra("editor", ["editorNumbers"], "/** 🔢️ The number inputs of the mutation under edit that the scenario asks the editor read to carry. */\nconst editorNumbers = (): readonly string[] => scenario?.editorNumbers ?? [];", "editor");
//#endregion editor extras

//#region universal helpers hoisted out of stepU
{
  const body = bodies.get("universal")!;
  const index = body.pieces.findIndex((candidate) => candidate.names.includes("stepU"));
  const stepU = body.pieces[index]!;
  const from = stepU.text.indexOf("  const settledControl = async (pointer: string) => {");
  const reviewed = stepU.text.indexOf("  const reviewed = () =>");
  if (from < 0 || reviewed < from) throw new Error("stepU helper block not found");
  const block = stepU.text.slice(from, reviewed);
  stepU.text = stepU.text.slice(0, from) + stepU.text.slice(reviewed);
  const dedented = block.split("\n").map((line) => (line.startsWith("  ") ? line.slice(2) : line)).join("\n").trimEnd();
  const names = ["settledControl", "visibleNoticeCodes", "visibleNoticeTexts", "dismissNotices"];
  const target = body.pieces.findIndex((candidate) => candidate.names.includes("universalChangeOneInput"));
  body.pieces.splice(target, 0, { names, text: "/** 🧯️ The clean-input helpers of the universal journey: a control's settled value and the shown notices. */\n" + dedented, origin: null });
}
//#endregion universal helpers

//#region run module
{
  const body = bodies.get("run")!;
  const runLocale = body.pieces.find((candidate) => candidate.names.includes("runLocale"))!;
  const start = runLocale.text.indexOf("  const ctx: Ctx = {};");
  const loop = runLocale.text.indexOf("  let reloaded = false;\n");
  if (start < 0 || loop < start) throw new Error("runLocale prepare block not found");
  captured.prepare = runLocale.text.slice(start + "  const ctx: Ctx = {};\n".length, loop);
  runLocale.text = runLocale.text.slice(0, start) + "  const ctx: object = scenario?.context() ?? {};\n  const blocked = scenario ? await scenario.prepare(ctx) : null;\n" + runLocale.text.slice(loop + "  let reloaded = false;\n".length);
  const reloadStart = runLocale.text.indexOf("    if ((step === 14 || step === 9) && !reloaded");
  const reloadEnd = runLocale.text.indexOf("    if (only && !only.has(step)) continue;");
  if (reloadStart < 0 || reloadEnd < reloadStart) throw new Error("runLocale reload block not found");
  runLocale.text = runLocale.text.slice(0, reloadStart) + "    await scenario?.beforeStep(step, ctx, blocked);\n" + runLocale.text.slice(reloadEnd);
  const blockedStart = runLocale.text.indexOf("    if (boardBlocked && step !== 1 && step !== 9) {");
  const blockedEnd = runLocale.text.indexOf("      continue;\n    }\n", blockedStart);
  if (blockedStart < 0 || blockedEnd < blockedStart) throw new Error("runLocale blocked block not found");
  runLocale.text = runLocale.text.slice(0, blockedStart) + "    if (blocked !== null && step !== 9 && !blocked.exempt.includes(step)) {\n      setCurrentStep(step);\n      verdict(blocked.name, false, { prerequisite: blocked.prerequisite });\n" + runLocale.text.slice(blockedEnd);
  const tailStart = runLocale.text.indexOf("  if (reloadSelected && !reloaded && !boardBlocked");
  const tailEnd = runLocale.text.indexOf("  emit({ kind: \"context\", ctx });");
  if (tailStart < 0 || tailEnd < tailStart) throw new Error("runLocale tail block not found");
  runLocale.text = runLocale.text.slice(0, tailStart) + "  await scenario?.afterSteps(ctx, blocked);\n" + runLocale.text.slice(tailEnd);
  extra("run", ["SHARED_STEPS", "runOrder", "stepFor", "stepNumbers"], "/** 📟️ The steps every run carries whatever its scenario: the universal journey and the console digest. */\nconst SHARED_STEPS: Readonly<Record<number, (context: object) => Promise<void>>> = { 9: step9, 24: stepU };\n\n/** 🧭️ The order of one locale pass: the scenario's steps, then the universal journey, then the console digest. */\nconst runOrder = (): readonly number[] => [...(scenario?.order ?? []), 24, 9];\n\n/** 🔎️ The step function of `step`: the scenario's own, else a shared one. */\nconst stepFor = (step: number): ((context: object) => Promise<void>) => scenario?.steps[step] ?? SHARED_STEPS[step]!;\n\n/** 🔢️ Every step number the report lists, ascending. */\nconst stepNumbers = (): readonly number[] => [...new Set([...(scenario?.stepNumbers ?? []), 9, 24])].sort((a, b) => a - b);", "runLocale");
}
//#endregion run module

//#region main module
{
  const main = bodies.get("main")!;
  const id = idOfName("runTimeTravelCli");
  let value = rawStatement(id);
  const replace = (from: string, to: string) => {
    const pieces = value.split(from);
    if (pieces.length !== 2) throw new Error(`runTimeTravelCli patch matched ${pieces.length - 1}: ${from.slice(0, 70)}`);
    value = pieces.join(to);
  };
  replace("export async function runTimeTravelCli(invocation:ScriptInvocation,repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {\n  checkScriptInvocation(invocation);\n  configure(segments, defaultOutDir, invocation.control.signal);", "export async function runTimeTravelCli(invocation: ScriptInvocation, repoRoot: string, defaultOutDir: string, segments: readonly string[], scenarioDefinition?: TimeTravelScenarioV1<object>): Promise<void> {\n  checkScriptInvocation(invocation);\n  if (scenarioDefinition === undefined && !segments.includes(\"--universal\")) throw new Error(\"a run without a scenario is universal: pass --universal --variant <declared variant> --serve <url>, or run the time-travel command of the plugin that owns a scenario\");\n  configure(segments, defaultOutDir, invocation.control.signal, scenarioDefinition);");
  replace("measured: { serve: serveUrl, renderer, variant, cancelled", "measured: { serve: serveUrl, renderer, variant: variant ?? \"none\", cancelled");
  replace("measured: { renderer, variant, serve: routeUrl(),", "measured: { renderer, variant: variant ?? \"none\", serve: routeUrl(),");
  replace("explicit: flagValue(segments, \"--variant\") }, playgroundCatalog);", "explicit: flagValue(segments, \"--variant\"), scenarioVariant: scenarioDefinition?.variant }, playgroundCatalog());");
  replace("activatePlaygroundRuntime(invocation,selectedVariant, \"dev\", renderer)", "activatePlaygroundRuntime(invocation, selectedVariant, \"prototype\", renderer)");
  value = value.replace("/** 🚦️ `verify time-travel [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,…]\n * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--universal] [--variant <declared variant>] [--explore]` — reuses the serve answering at `--serve` (default\n * :6012 React, :6112 wgpu) or starts the catalog-admitted universal variant (puzzle 2d for specialized steps) there for the run ({@link ensureDevServe}; stopped\n * after it)", "/** 🚦️ `time-travel [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,…]\n * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--universal] [--variant <declared variant>] [--explore]` — reuses the serve answering at `--serve` (the\n * scenario's default per renderer) or starts the catalog-admitted variant there for the run ({@link ensureDevServe}; stopped\n * after it)");
  main.pieces.push(piece(idOfName("CHECK_ID"), "main"));
  main.pieces.push({ names: ["runTimeTravelCli"], text: rebaseSee(rewriteStateWrites(value, "runTimeTravelCli"), H, H), origin: id });
}
//#endregion main module
//#endregion assembly

//#region scenario contract and assembly
bodies.get("scenario")!.pieces.push({ names: ["TimeTravelBlockV1"], origin: null, typeOnly: true, text: "/** 🚧️ A prerequisite the scenario cannot meet: every step but the `exempt` ones fails with the verdict `name` naming the `prerequisite`. */\nexport type TimeTravelBlockV1 = Readonly<{ name: string; prerequisite: string; exempt: readonly number[] }>;" });
bodies.get("scenario")!.pieces.push({ names: ["TimeTravelStepV1"], origin: null, typeOnly: true, text: "/** 👣️ One scenario step; the method form keeps a step over a narrower context assignable to the harness's. */\nexport type TimeTravelStepV1<Context extends object = object> = { run(context: Context): Promise<void> }[\"run\"];" });
bodies.get("scenario")!.pieces.push({ names: ["TimeTravelScenarioV1"], origin: null, typeOnly: true, text: [
  "/** 🧩️ What a plugin supplies to the time-travel harness: the playground it drives, where its serve listens, the steps of its journey and the hooks the harness calls around them. The harness owns the CLI, the browser, the report and the plugin-neutral universal step. */",
  "export type TimeTravelScenarioV1<Context extends object = object> = Readonly<{",
  "  variant: string;",
  "  defaultServes: Readonly<Record<Renderer, string>>;",
  "  bootWindowPrefix: string;",
  "  editorNumbers: readonly string[];",
  "  paletteQuery: Readonly<Record<Locale, string>>;",
  "  stepNumbers: readonly number[];",
  "  order: readonly number[];",
  "  steps: Readonly<Record<number, TimeTravelStepV1<Context>>>;",
  "  context(): Context;",
  "  prepare(context: Context): Promise<TimeTravelBlockV1 | null>;",
  "  beforeStep(step: number, context: Context, blocked: TimeTravelBlockV1 | null): Promise<void>;",
  "  afterSteps(context: Context, blocked: TimeTravelBlockV1 | null): Promise<void>;",
  "  diagnostics(): Promise<Record<string, unknown>>;",
  "  exploreInventory(): Promise<Record<string, unknown>>;",
  "}>;",
].join("\n") });

{
  const prepareBody = captured.prepare;
  const lines = prepareBody.split("\n");
  const cleaned = lines.filter((line) => line !== "  let reloaded = false;").join("\n").trimEnd();
  const sassembly = bodies.get("sassembly")!;
  sassembly.pieces.push({ names: ["reloaded"], origin: null, text: "/** 🔁️ The contexts whose reload check already ran. */\nconst reloaded = new WeakSet<object>();" });
  sassembly.pieces.push({ names: ["prepare"], origin: null, text: [
    "/** 🧱️ Once per locale after boot: on wgpu the board must project its introspection and carry the example; otherwise the board steps are blocked. */",
    "const prepare = async (): Promise<TimeTravelBlockV1 | null> => {",
    cleaned,
    "  return boardBlocked ? { name: boardEmpty ? \"blocked-by-the-empty-board\" : \"blocked-by-missing-board-introspection\", prerequisite: boardEmpty ? \"a board with nodes (wgpu-loads-the-example-at-boot / wgpu-navbar-example-loads-the-board)\" : \"semioWgpuIntrospection.dumpBoard2d\", exempt: [1] } : null;",
    "};",
  ].join("\n") });
  sassembly.pieces.push({ names: ["beforeStep"], origin: null, text: [
    "/** 🔁️ Takes step 5's reload half before the fresh-context steps (14) and the console digest (9). */",
    "const beforeStep = async (step: number, ctx: Ctx, blocked: TimeTravelBlockV1 | null): Promise<void> => {",
    "  if ((step === 14 || step === 9) && !reloaded.has(ctx) && reloadSelected && blocked === null && invalidatedBy === null) {",
    "    reloaded.add(ctx);",
    "    setCurrentStep(5);",
    "    try {",
    "      await reloadCheck(ctx);",
    "    } catch (error) {",
    "      verdict(\"reload-check-threw\", false, { error: String(error).slice(0, 400) });",
    "    }",
    "  }",
    "};",
  ].join("\n") });
  sassembly.pieces.push({ names: ["afterSteps"], origin: null, text: [
    "/** 🔚️ Takes the reload half after the last step when no earlier step already did. */",
    "const afterSteps = async (ctx: Ctx, blocked: TimeTravelBlockV1 | null): Promise<void> => {",
    "  if (reloadSelected && !reloaded.has(ctx) && blocked === null && !cancelled.aborted && invalidatedBy === null) {",
    "    setCurrentStep(5);",
    "    await reloadCheck(ctx);",
    "  }",
    "};",
  ].join("\n") });
  sassembly.pieces.push({ names: ["puzzle2dTimeTravelScenario"], origin: null, text: [
    "/** ⏪️ The reference scenario of the history-editing design: the puzzle 2d playground driven through a drag, its edit, alternatives, input controls, phone and tablet journeys, long history, two peers and the row actions. */",
    "export const puzzle2dTimeTravelScenario: TimeTravelScenarioV1<Ctx> = {",
    "  variant: PLUGIN_VARIANT,",
    "  defaultServes: DEFAULT_SERVES,",
    "  bootWindowPrefix: \"2d-\",",
    "  editorNumbers: [\"dx\", \"dy\"],",
    "  paletteQuery: { en: \"Rotate\", de: \"Drehen\" },",
    "  stepNumbers: [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],",
    "  order: [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 19, 20, 21, 22, 23, 14, 15, 16, 17, 18],",
    "  steps: { 1: step1, 2: step2, 3: step3, 4: step4, 5: step5, 6: step6, 7: step7, 8: step8, 10: step10, 11: step11, 12: step12, 13: step13, 14: step14, 15: step15, 16: step16, 17: step17, 18: step18, 19: step19, 20: step20, 21: step21, 22: step22, 23: step23 },",
    "  context: () => ({}),",
    "  prepare,",
    "  beforeStep,",
    "  afterSteps,",
    "  diagnostics: async () => ({ vitals: await vitals() }),",
    "  exploreInventory: async () => ({ board: await wgpuBoard(), boardContract: WGPU_BOARD_CONTRACT }),",
    "};",
  ].join("\n") });
}
//#endregion scenario contract and assembly

//#region imports exports and emission
const declaredIn = new Map<string, ModuleKey[]>();
for (const [key, body] of bodies) for (const item of body.pieces) for (const name of item.names) declaredIn.set(name, [...(declaredIn.get(name) ?? []), key]);
const ownerFor = (name: string, from: ModuleKey): ModuleKey | undefined => {
  const candidates = declaredIn.get(name);
  if (!candidates) return undefined;
  if (candidates.includes(from)) return from;
  if (candidates.length === 1) return candidates[0];
  return candidates.find((candidate) => (candidate === "pcopy") === isScenarioKind(from));
};
const usedOf = new Map<ModuleKey, Set<string>>();
for (const [key, body] of bodies) {
  const used = new Set<string>();
  for (const item of body.pieces) identifiersOf(ts.createSourceFile("piece.ts", item.text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), used);
  usedOf.set(key, used);
}
const needsExport = new Set<string>();
const importsOf = new Map<ModuleKey, Map<ModuleKey, Set<string>>>();
const externalsOf = new Map<ModuleKey, Map<string, ExternalImport[]>>();
for (const [key, used] of usedOf) {
  const local = new Map<ModuleKey, Set<string>>();
  const external = new Map<string, ExternalImport[]>();
  for (const name of used) {
    const owner = ownerFor(name, key);
    if (owner !== undefined && owner !== key) {
      needsExport.add(`${owner}:${name}`);
      if (!local.has(owner)) local.set(owner, new Set());
      local.get(owner)!.add(name);
      continue;
    }
    if (owner === key) continue;
    const entry = externals.get(name);
    if (entry) {
      const spec = rebase(entry.spec, H, moduleDirOf(key));
      external.set(spec, [...(external.get(spec) ?? []), entry]);
    }
  }
  importsOf.set(key, local);
  externalsOf.set(key, external);
}

const specFor = (key: ModuleKey): string => (key === "main" ? FILE : key === "sassembly" ? FILE : FILE);
void specFor;
const fileForModule = (key: ModuleKey): string => join(moduleDirOf(key), FILE);

const regionNameOf = (id: number | null): string[] => {
  if (id === null) return [];
  const pos = statements[id]!.getStart(sf);
  const stack: string[] = [];
  for (const match of text.slice(0, pos).matchAll(/^[ \t]*\/\/\s*#(end)?region[ \t]+(.*)$/gmu)) {
    if (match[1]) stack.pop();
    else stack.push(match[2]!.trim());
  }
  return stack;
};

const exportize = (item: Piece, key: ModuleKey): string => {
  if (!item.names.some((name) => needsExport.has(`${key}:${name}`))) return item.text;
  const snippet = ts.createSourceFile("piece.ts", item.text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const first = snippet.statements[0];
  if (!first) return item.text;
  const at = first.getStart(snippet);
  const head = item.text.slice(at);
  if (head.startsWith("export ")) return item.text;
  return item.text.slice(0, at) + "export " + head;
};

const importLines = (key: ModuleKey): string[] => {
  const lines: string[] = [];
  const dir = moduleDirOf(key);
  const external = externalsOf.get(key)!;
  for (const [spec, entries] of external) {
    const typeNames = [...new Set(entries.filter((entry) => entry.typeOnly).map((entry) => entry.name))];
    const valueNames = [...new Set(entries.filter((entry) => !entry.typeOnly).map((entry) => entry.name))];
    if (valueNames.length) lines.push(`import { ${valueNames.join(", ")} } from ${JSON.stringify(spec)};`);
    if (typeNames.length) lines.push(`import type { ${typeNames.join(", ")} } from ${JSON.stringify(spec)};`);
  }
  for (const [owner, names] of importsOf.get(key)!) {
    let spec = relative(dir, fileForModule(owner));
    if (!spec.startsWith(".")) spec = "./" + spec;
    const typeOnly = [...names].filter((name) => bodies.get(owner)!.pieces.some((item) => item.names.includes(name) && item.typeOnly));
    const valueNames = [...names].filter((name) => !typeOnly.includes(name));
    if (valueNames.length) lines.push(`import { ${valueNames.join(", ")} } from ${JSON.stringify(spec)};`);
    if (typeOnly.length) lines.push(`import type { ${typeOnly.join(", ")} } from ${JSON.stringify(spec)};`);
  }
  return lines;
};

const originalDocstring = (() => {
  const ranges = ts.getLeadingCommentRanges(text, statements[1]!.getFullStart()) ?? [];
  const doc = ranges.map((range) => text.slice(range.pos, range.end)).find((value) => value.startsWith("/**"));
  if (!doc) throw new Error("module docstring not found");
  return doc;
})();

const DOC_EMOJI: Record<string, string> = { page: "📃", chrome: "🪟", arguments: "🚦", replay: "⏺", pcopy: "🗣" };
const docFor = (key: ModuleKey): string => {
  if (key === "main") return "/** ⏪️ `time-travel` — the plugin-neutral end-to-end harness of non-destructive history editing. One headless Chromium, one fresh browser context per UI locale (`en`, then `de`, chosen through `navigator.language`), the steps of a plugin scenario and the shared universal journey, each verdict compared against the localized texts of the framework's own history panel.\n *\n * A scenario ({@link TimeTravelScenarioV1}) names the playground it drives, where its serve listens and the steps of its journey; a run without a scenario is universal and drives any declared playground through the mirror's own controls. Outputs under `--out`: `probe-<renderer>-<stamp>.md` (step lines, verdicts, notes, timeline, console digest), `.ndjson` (one record per verdict/note/dump) and `probe-<renderer>-<stamp>-<locale>-s<step>-<tag>.png|json`; the acceptance record `time-travel`.\n * @see ./🧩️scenario/🟦️.ts\n * @see ../../../🔌️plugin/⏪️time-travel/🦀️.rs\n * @see ../👥️two-human/🟦️.ts */";
  if (key === "sassembly") {
    let doc = originalDocstring;
    doc = doc.replace(/^\/\*\* ⏪️ `verify time-travel` — /u, "/** 🕰️ The puzzle 2d scenario of `time-travel` — ");
    doc = doc.replace(/\n \* Run: `verify time-travel \[/u, "\n * Run: `bun repo run @semio-tech/puzzle-js:time-travel -- [");
    doc = doc.replace(/\n \* @see \.\.\/\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/🔨️modules\/⏪️time-travel\/🦀️\.rs\n \* @see \.\.\/\.\.\/\.\.\/🔌️plugin\/⏪️time-travel\/🦀️\.rs\n \* @see \.\.\/\.\.\/\.\.\/📺️renderer\/🧑‍🎨engine\/🧱️elements\/🛠️ShellHelpers\/⏪️time-travel\/🟦️\.tsx\n \* @see \.\.\/👥️two-human\/🟦️\.ts \*\//u, "\n * @see ../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts */");
    return doc;
  }
  const spec = moduleOf.get(key)!;
  return `/** ${DOC_EMOJI[key] ?? spec.emoji}️ ${spec.doc} */`;
};

const renderModule = (key: ModuleKey): string => {
  const body = bodies.get(key)!;
  const out: string[] = [docFor(key)];
  const imports = importLines(key);
  if (key === "pcopy") imports.unshift(`import { COPY as FRAMEWORK_COPY } from ${JSON.stringify(relative(moduleDirOf("pcopy"), fileForModule("copy")).startsWith(".") ? relative(moduleDirOf("pcopy"), fileForModule("copy")) : "./" + relative(moduleDirOf("pcopy"), fileForModule("copy")))};`);
  if (key === "main") {
    for (const name of ["runProbe"]) void name;
  }
  out.push(...imports);
  out.push("");
  let stack: string[] = [];
  const bodyParts: string[] = [];
  const closeTo = (target: string[]) => {
    let common = 0;
    while (common < stack.length && common < target.length && stack[common] === target[common]) common += 1;
    while (stack.length > common) bodyParts.push(`//#endregion ${stack.pop()}`);
    for (const region of target.slice(common)) {
      bodyParts.push(`//#region ${region}`);
      stack.push(region);
    }
  };
  let lastStack: string[] = [];
  for (const item of body.pieces) {
    const regions = item.origin === null ? lastStack : regionNameOf(item.origin);
    if (item.origin !== null) lastStack = regions;
    closeTo(regions);
    bodyParts.push(exportize(item, key));
  }
  closeTo([]);
  out.push(bodyParts.join("\n\n").replace(/\n\n(\/\/#(end)?region)/gu, "\n$1"));
  out.push("");
  return out.join("\n");
};

const outputs = new Map<string, string>();
for (const key of bodies.keys()) outputs.set(fileForModule(key), renderModule(key));

const report = { modules: [...bodies].map(([key, body]) => `${key}: ${body.pieces.length} pieces, ${renderModule(key).split("\n").length} lines`), exports: [...needsExport].length };
writeFileSync(join(FOLDER, "🗑️generated/met-tt-split-report.json"), JSON.stringify(report, null, 1));
if (DRY) console.log(JSON.stringify(report, null, 1));
else {
  for (const [file, content] of outputs) {
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, content);
    console.log("wrote", relative(REPO, file), content.split("\n").length);
  }
  writeFileSync(join(FOLDER, "met-tools/tt-new-files.txt"), [...outputs.keys()].map((file) => relative(REPO, file)).join("\n") + "\n");
}
//#endregion imports exports and emission
