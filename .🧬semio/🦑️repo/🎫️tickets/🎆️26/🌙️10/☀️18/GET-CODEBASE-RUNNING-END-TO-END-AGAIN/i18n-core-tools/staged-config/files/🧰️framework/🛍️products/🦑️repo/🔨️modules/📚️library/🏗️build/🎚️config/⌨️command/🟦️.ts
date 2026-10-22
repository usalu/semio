import { CATALOG_LANGUAGES_V1, textV1, type TextValuesV1 } from "@P/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { isRepoErrorV1, renderRepoErrorV1, repoErrorExitCodeV1 } from "@P/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import { languageFlagV1, resolveLanguageV1, resolveLanguagesV1, type LanguageEnvironmentV1, type OsLanguageListV1 } from "@P/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import { appendLocalConfigEventV1, localConfigDirectoryV1, localConfigEventV1, readConfiguredLanguageV1, readLocalConfigV1 } from "../🟦️.ts";
import { CONFIG_TEXTS_V1, refuseConfigV1, type ConfigTextKeyV1 } from "../📖️texts/🟦️.ts";

/** 🎛️ What `repo config` needs besides its arguments: the environment, the output, the help of the operations catalogue and the pinned clock and id of a test. */
export type ConfigCliContextV1 = Readonly<{ environment: LanguageEnvironmentV1; args?: readonly string[]; osLanguages?: OsLanguageListV1; write?: (line: string) => void; writeError?: (line: string) => void; help: (language: string) => string; clock?: () => Date; id?: () => string }>;

const CHOICE = CATALOG_LANGUAGES_V1.join("|");

/** 🚪️ Runs `repo config set language <en|de> | unset language | show [--json]` and answers the exit code. It runs before any language is resolved, because it is how a language gets chosen: its output is in the language it just set, else in the chosen one, else in every language. */
export async function runConfigCliV1(root: string, argv: readonly string[], context: ConfigCliContextV1): Promise<number> {
  const write = context.write ?? ((line: string) => console.log(line)), writeError = context.writeError ?? ((line: string) => console.error(line));
  const words: string[] = [];
  for (let index = 0; index < argv.length; index++) {
    if (argv[index] === "--lang") index++;
    else if (!argv[index]!.startsWith("--lang=")) words.push(argv[index]!);
  }
  const directory = localConfigDirectoryV1(root, context.environment);
  const languagesOf = (configured: string | undefined, forced?: string): readonly string[] => forced !== undefined ? [forced] : resolveLanguagesV1(CATALOG_LANGUAGES_V1, context.args ?? argv, context.environment, context.osLanguages, configured);
  const say = (key: ConfigTextKeyV1, values: TextValuesV1, languages: readonly string[]): string => languages.map((language) => textV1(CONFIG_TEXTS_V1.catalog, key, language, values)).join("\n");
  try {
    const current = ((): string | undefined => { try { return readLocalConfigV1(directory).config.language; } catch { return undefined; } })();
    if (words.length === 0 || words.includes("--help") || words.includes("-h")) {
      write(languagesOf(current).map((language) => context.help(language)).join("\n\n"));
      return 0;
    }
    const [verb, key, value] = words;
    if (verb === "show") {
      const rest = words.slice(1);
      if (rest.some((word) => word !== "--json")) throw refuseConfigV1("usage", "usage", { choice: CHOICE });
      if (directory === undefined) throw refuseConfigV1("missing", "no-location", { root });
      const { config, file } = readLocalConfigV1(directory), languages = languagesOf(config.language);
      const effective = ((): Readonly<{ language: string; source: string }> | undefined => { try { return resolveLanguageV1(CATALOG_LANGUAGES_V1, context.args ?? argv, context.environment, context.osLanguages, config.language); } catch { return undefined; } })();
      if (rest.includes("--json")) {
        write(JSON.stringify({ version: 1, ...(config.language === undefined ? {} : { language: config.language }), events: config.events, file: file ?? null, effective: effective ?? null }, null, 2));
        return 0;
      }
      write(config.language === undefined ? say("show-none", {}, languages) : say("show-language", { language: config.language }, languages));
      write(say("show-log", { file: file ?? "-", events: config.events }, languages));
      write(effective === undefined ? say("show-effective-none", { choice: CHOICE }, languages) : say("show-effective", { language: effective.language, source: effective.source }, languages));
      return 0;
    }
    if (verb === "set" || verb === "unset") {
      if (key !== "language") throw refuseConfigV1("usage", key === undefined ? "usage" : "unknown-key", key === undefined ? { choice: CHOICE } : { key });
      if (directory === undefined) throw refuseConfigV1("missing", "no-location", { root });
      if (verb === "unset") {
        if (value !== undefined) throw refuseConfigV1("usage", "usage", { choice: CHOICE });
        const event = localConfigEventV1("language-cleared", undefined, context.clock, context.id), file = appendLocalConfigEventV1(directory, event);
        write(say("language-unset", { id: event.id, file }, languagesOf(undefined)));
        return 0;
      }
      if (value === undefined || words.length !== 3) throw refuseConfigV1("usage", "usage", { choice: CHOICE });
      const event = localConfigEventV1("language-chosen", value, context.clock, context.id), file = appendLocalConfigEventV1(directory, event);
      write(say("language-set", { language: value, id: event.id, file }, [value]));
      return 0;
    }
    throw refuseConfigV1("usage", "usage", { choice: CHOICE });
  } catch (error) {
    if (!isRepoErrorV1(error)) throw error;
    writeError(languagesOf(undefined).map((language) => renderRepoErrorV1(error, language)).join("\n"));
    return repoErrorExitCodeV1(error.code);
  }
}

/** 🗣️ The language for the CLI door: undefined when a flag or SEMIO_LANGUAGE decides first; otherwise what the repo config names. A broken log is reported in every language and does not stop the door, which then asks for a language by itself. */
export function configuredLanguageForDoorV1(root: string, args: readonly string[], environment: LanguageEnvironmentV1, writeError: (line: string) => void = (line) => console.error(line)): string | undefined {
  if (languageFlagV1(args) !== undefined || (environment.SEMIO_LANGUAGE ?? "") !== "") return undefined;
  try { return readConfiguredLanguageV1(root, environment); } catch (error) {
    if (!isRepoErrorV1(error)) throw error;
    writeError(CATALOG_LANGUAGES_V1.map((language) => renderRepoErrorV1(error, language)).join("\n"));
    return undefined;
  }
}
