/** ♨️ Keeps the TypeScript programs of a long-lived deriver between questions: every parsed source file is kept by its text and parse options, so an unchanged file is never parsed again, and the previous program of a configuration is handed to `createProgram` as `oldProgram` only while the deriver has armed the store, i.e. after it proved that every probe the previous program's module resolution read (existence, directories, real paths, configuration and package files) still answers the same; TypeScript's own structure reuse then re-resolves exactly the files whose text changed.
 * @see https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API#incremental-build-support-using-the-language-services */

let active;

/** 🏭️ A fresh reuse store: programs by configuration, parsed files by configuration, the files of the programs of the running question, and the arm switch. */
export function createTypeScriptProgramReuseV1() {
  const programs = new Map(), parsed = new Map();
  let armed = false, sources = new Set(), used = new Map(), reused = [];
  return Object.freeze({
    begin(arm) { armed = arm === true; sources = new Set(); used = new Map(); reused = []; },
    end() { armed = false; for (const [config, keys] of used) { const files = parsed.get(config); if (files) for (const key of [...files.keys()]) if (!keys.has(key)) files.delete(key); } const done = { sources, reused }; sources = new Set(); used = new Map(); reused = []; return done; },
    sourceFile(config, original, readFile) {
      let files = parsed.get(config); if (!files) parsed.set(config, files = new Map());
      let keys = used.get(config); if (!keys) used.set(config, keys = new Set());
      return (fileName, languageVersionOrOptions, onError, shouldCreateNewSourceFile) => {
        const text = readFile(fileName);
        if (text === undefined) return original(fileName, languageVersionOrOptions, onError, shouldCreateNewSourceFile);
        const options = typeof languageVersionOrOptions === "object" && languageVersionOrOptions !== null ? languageVersionOrOptions : { languageVersion: languageVersionOrOptions };
        const key = `${fileName}\0${options.languageVersion}\0${options.impliedNodeFormat ?? ""}\0${options.jsDocParsingMode ?? ""}`, known = files.get(key);
        keys.add(key);
        if (known && known.text === text && !shouldCreateNewSourceFile) return known.file;
        const file = original(fileName, languageVersionOrOptions, onError, shouldCreateNewSourceFile);
        if (file && file.text === text) files.set(key, { text, file }); else files.delete(key);
        return file;
      };
    },
    oldProgram(config) { return armed ? programs.get(config) : undefined; },
    keep(config, program) { programs.set(config, program); for (const file of program.getSourceFiles()) sources.add(file.fileName); if (armed) reused.push(program.structureIsReused); },
  });
}

/** 🔌️ Installs the store the program questions of this process use (`undefined` uninstalls it); only a long-lived deriver installs one. */
export function useTypeScriptProgramReuseV1(store) { active = store; }

/** 🔎️ The installed store, if any. */
export function activeTypeScriptProgramReuseV1() { return active; }
