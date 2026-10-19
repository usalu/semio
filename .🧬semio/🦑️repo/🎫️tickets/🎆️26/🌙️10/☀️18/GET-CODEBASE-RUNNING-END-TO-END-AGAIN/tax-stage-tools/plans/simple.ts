// 🧹 Small library edits: the three dangling references, the four emoji-only directory renames with their referencers, and the two queued law-sweep edits.
import {readdirSync, statSync} from "node:fs";
import {join} from "node:path";
import {LIB, REPO, Vfs} from "../vfs.ts";

const E = (cp: number): string => String.fromCodePoint(cp) + "️";
const TESTS = E(0x1F9EA) + "tests", TS = E(0x1F7E6) + ".ts", JSON_ = E(0x1F523) + ".json";
const FRAMEWORK = E(0x1F9F0) + "framework", MODULES = E(0x1F528) + "modules", PROCESS = E(0x1F3C3) + "process", BUILD = E(0x1F3D7) + "build";

function files(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(join(REPO, dir))) { const p = `${dir}/${name}`; if (statSync(join(REPO, p)).isDirectory()) out.push(...files(p)); else out.push(p); }
  return out;
}

export function planSimple(v: Vfs): void {
  const mode = `${LIB}/${BUILD}/${E(0x1F39A)}mode/${E(0x1F980)}profile/${TESTS}`;
  const caching = `${LIB}/${E(0x26A1)}caching`;
  const renames: [string, string][] = [
    [`${caching}/${E(0x1F680)}bootstrap/${E(0x1F6E0)}tools/${E(0x1F4E6)}sdk/${E(0x1F4E5)}source/${E(0x1F517)}derivative/${TESTS}/${E(0x1F6C2)}`, `${E(0x1F6C2)}sdk-fences`],
    [`${LIB}/${PROCESS}/${E(0x1F980)}cargo/${E(0x1F9EA)}testing/${E(0x1F4E6)}executable/${TESTS}/${E(0x1F33F)}`, `${E(0x1F33F)}runtime-selection`],
    [`${mode}/${E(0x2699)}arguments/${E(0x1F4D0)}context/${E(0x1F980)}`, `${E(0x1F980)}rlib-context`],
    [`${mode}/${E(0x1F980)}`, `${E(0x1F980)}profile-precedence`],
  ];
  for (const [oldDir, name] of renames) {
    const parent = oldDir.split("/").slice(0, -1).join("/"), newDir = `${parent}/${name}`;
    for (const f of files(oldDir)) { const target = newDir + f.slice(oldDir.length); v.write(target, v.read(f)); v.remove(f); }
  }
  const [d1, d2, d3, d4] = renames.map(([o, n]) => [o, o.split("/").slice(0, -1).join("/") + "/" + n] as const);
  const swap = (file: string, pairs: readonly (readonly [string, string])[]): void => { for (const [a, b] of pairs) v.edit(file, a, b); };
  swap(`${LIB}/${caching.slice(LIB.length + 1)}/${E(0x1F4BE)}storage/${E(0x1F512)}enforcement/${E(0x1F4E6)}budget/${E(0x1F4E5)}opening/${E(0x1F3EA)}store/${TESTS}/${E(0x1F4E5)}phase/${E(0x1F4CB)}callers.json`, [[`${d1[0]}/${TS}`, `${d1[1]}/${TS}`]]);
  swap(`${FRAMEWORK}/${E(0x1F6CD)}products/${E(0x1F991)}repo/${MODULES}/${E(0x1F39B)}dashboard/${TESTS}/${E(0x1F9ED)}authority/${TS}`, [[d2[0].slice(d2[0].indexOf(`${E(0x1F4DA)}library`)) + `/${TS}`, d2[1].slice(d2[1].indexOf(`${E(0x1F4DA)}library`)) + `/${TS}`]]);
  swap(`${LIB}/${PROCESS}/${E(0x1F980)}cargo/${E(0x1F9EA)}testing/${E(0x1F4E6)}executable/${TESTS}/${TS}`, [[`"./${E(0x1F33F)}/${TS}"`, `"./${d2[1].split("/").at(-1)}/${TS}"`]]);
  const capacity = `${FRAMEWORK}/${MODULES}/${PROCESS}/${BUILD}/${E(0x1F39F)}capacity/${TESTS}/${E(0x1FA86)}scope/${E(0x1F9EB)}fixtures/${JSON_}`;
  v.edit(capacity, `${d3[0]}/${TS}`, `${d3[1]}/${TS}`, 2);
  const contextTests = `${mode}/${E(0x2699)}arguments/${E(0x1F4D0)}context/${TESTS}/${TS}`;
  v.edit(contextTests, `new URL("../${E(0x1F980)}.ts",import.meta.url)`, `new URL("../${d3[1].split("/").at(-1)}/${TS}",import.meta.url)`, 2);
  v.edit(`${d4[1]}/${TS}`, `${E(0x2699)}arguments/${E(0x1F4D0)}context/${E(0x1F980)}/${TS}`, `${E(0x2699)}arguments/${E(0x1F4D0)}context/${d3[1].split("/").at(-1)}/${TS}`);
  v.edit(`${mode}/${JSON_}`, `"${TESTS}/${E(0x1F980)}.ts"`, `"${TESTS}/${d4[1].split("/").at(-1)}/${TS}"`);

  const buildInputs = `${LIB}/${BUILD}/${E(0x1F4E5)}inputs/${E(0x1F4BE)}storage/${E(0x1F4DC)}script.ts`;
  v.edit(buildInputs, `"${TESTS}/${E(0x1F4D2)}ledger.ts"`, `"${TESTS}/${E(0x1F4D2)}ledger/${TS}"`);
  const cachingScript = `${caching}/${E(0x1F4DC)}script.ts`;
  v.edit(cachingScript, `"${E(0x1F4E6)}artifacts/${E(0x1F512)}bounded/${TESTS}/${E(0x1F34E)}image.ts"`, `"${E(0x1F4E6)}artifacts/${E(0x1F512)}bounded/${TESTS}/${E(0x1F34E)}image/${TS}"`);
  v.edit(cachingScript, `"${E(0x1F4BE)}storage/${E(0x1F512)}enforcement/${E(0x1F4E6)}budget/${E(0x1F4E5)}opening/${TESTS}/${E(0x1F3EA)}scope.ts"`, `"${E(0x1F4BE)}storage/${E(0x1F512)}enforcement/${E(0x1F4E6)}budget/${E(0x1F4E5)}opening/${TESTS}/${E(0x1F3EA)}scope/${TS}"`);

  const composed = `${LIB}/${E(0x1F39B)}commands/${E(0x2705)}verify/${E(0x1F9F7)}composed-child-refs/${TS}`;
  v.edit(composed, `"${E(0x1F5D1)}generated"]);`, `"${E(0x1F5D1)}generated", "${E(0x1F3AB)}tickets"]);`);
  const verify = `${LIB}/${E(0x1F39B)}commands/${E(0x2705)}verify/${TS}`;
  const prettierMany = `runCmd("bun", [this.root + "/node_modules/prettier/bin/prettier.cjs", "--check", ...["${E(0x25FB)}2d", "${E(0x1F9CA)}3d"].map((dimension) => this.root + "/${FRAMEWORK}/${MODULES.replace(E(0x1F528), E(0x1F528))}/${E(0x1F5B1)}ui/${E(0x1FA9F)}viewport/" + dimension + "/${E(0x1F9EC)}schema/${E(0x1F517)}.graphql")], { cwd: this.root });`;
  const prettierOne = `runCmd("bun", [this.root + "/node_modules/prettier/bin/prettier.cjs", "--check", this.root + "/${FRAMEWORK}/${MODULES}/${E(0x1F5B1)}ui/${E(0x1FA9F)}viewport/${E(0x1F9CA)}3d/${E(0x1F9EC)}schema/${E(0x1F517)}.graphql"], { cwd: this.root });`;
  v.edit(verify, prettierMany, `await parseViewportGraphqlSchemasV1([...["${E(0x25FB)}2d", "${E(0x1F9CA)}3d"].map((dimension) => this.root + "/${FRAMEWORK}/${MODULES}/${E(0x1F5B1)}ui/${E(0x1FA9F)}viewport/" + dimension + "/${E(0x1F9EC)}schema/${E(0x1F517)}.graphql")]);`);
  v.edit(verify, prettierOne, `await parseViewportGraphqlSchemasV1([this.root + "/${FRAMEWORK}/${MODULES}/${E(0x1F5B1)}ui/${E(0x1FA9F)}viewport/${E(0x1F9CA)}3d/${E(0x1F9EC)}schema/${E(0x1F517)}.graphql"]);`);
  v.edit(verify, `//#region ${E(0x1F516)}VerifyScript\n`, `/** ${E(0x1F9FE)} Parses each viewport GraphQL schema with the reference parser and refuses the first syntax error. */\nasync function parseViewportGraphqlSchemasV1(paths: readonly string[]): Promise<void> {\n  const { parse } = await import("graphql");\n  for (const path of paths) parse(readFileSync(path, "utf8"));\n}\n\n//#region ${E(0x1F516)}VerifyScript\n`);
}
