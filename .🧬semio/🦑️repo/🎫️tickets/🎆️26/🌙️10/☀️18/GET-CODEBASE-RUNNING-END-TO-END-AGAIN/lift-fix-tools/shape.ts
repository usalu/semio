// 🧱 Single owner of the lifted-program shape shared by every router-lift generator and the live repair: argv pass-through for routers, bare entry for test programs.
export const BIRTH_OPENER = "receiveScriptProcessInvocation";
const TEST_PATH = /\/(?:🧪️tests|👷️child)\//u;

/** 🧪️ Whether a script is a test program: spawned by its test with its own argv and environment, it reports after its own birth drained and must not depend on a router birth. */
export const isTestProgram = (scriptPath: string): boolean => TEST_PATH.test(scriptPath);

/** 🚦️ Options of the router terminal: the program parses its own arguments, so one fixed command is always dispatched. */
export const dispatchOptions = (command: string): string => `{ invocation: original, argv: [${JSON.stringify(command)}] }`;

/** 🪺️ Bare entry of a test program; the program itself opens its birth when it needs one. */
export const bareScript = (header: string, moduleSpecifier: string): string => `${header ? header + "\n" : ""}import { runScriptProgram } from "${moduleSpecifier}";\nif (import.meta.main) await runScriptProgram();\n`;
