import protocol from "./🔣️.json" with { type: "json" };

/** 🌍️ The environment a language is read from. */
export type LanguageEnvironmentV1 = Readonly<Record<string, string | undefined>>;

/** 🧭️ Where the chosen language came from. */
export type LanguageResolutionV1<T extends string = string> = Readonly<{ language: T; source: string }>;

/** 🚧️ Why no language was chosen: no source names an offered one, the flag or the explicit variable names one that is not offered. */
export type LanguageRefusalV1 = Readonly<{ kind: keyof typeof protocol.refusal; name?: string; value?: string }>;

/** 🔌️ The language tags of the operating system in preference order, or none when it names none. */
export type OsLanguageListV1 = () => readonly string[];

/** 📜️ The one language protocol every runtime reads (`🔣️.json`, schema `🧬️schema/🔣️.json`). */
export const LANGUAGE_PROTOCOL_V1 = protocol;

const subtagSplit = new RegExp(`[${protocol.subtagSeparators.replace(/[\\\]^-]/gu, "\\$&")}]`, "u");
let remembered: readonly string[] | undefined;

/** 🔤️ Reads the language subtag of a locale or language tag such as `de_CH.UTF-8@euro` or `en-US`; the neutral locales `C` and `POSIX` carry none. */
export function languageSubtagV1(value: string | undefined): string | undefined {
  const tag = value?.trim().toLowerCase().split(subtagSplit)[0];
  return tag === undefined || tag === "" || protocol.neutralTags.includes(tag) ? undefined : tag;
}

/** 🏳️ Reads the last `--lang <value>` or `--lang=<value>` before the `--` separator. */
export function languageFlagV1(args: readonly string[]): string | undefined {
  const boundary = args.indexOf(protocol.argumentsEnd), head = args.slice(0, boundary < 0 ? undefined : boundary), prefix = `${protocol.flag}=`;
  let explicit: string | undefined;
  for (let index = 0; index < head.length; index++) {
    const argument = head[index]!;
    if (argument === protocol.flag) explicit = head[++index] ?? "";
    else if (argument.startsWith(prefix)) explicit = argument.slice(prefix.length);
  }
  return explicit;
}

/** 🖥️ Asks the operating system for its language list once per process: the macOS preferences command of the protocol, the Windows user locale; other systems state theirs in the environment. */
export const osLanguageListV1: OsLanguageListV1 = () => {
  if (remembered !== undefined) return remembered;
  if (process.platform === "darwin") {
    const row = protocol.osLocale.darwin, done = process.getBuiltinModule("node:child_process").spawnSync(row.command, row.args, { encoding: "utf8", timeout: row.timeoutMs, windowsHide: true });
    return (remembered = done.status === 0 ? [...done.stdout.matchAll(new RegExp(row.pattern, "gu"))].map((match) => match[1]!) : []);
  }
  return (remembered = process.platform === "win32" ? [Intl.DateTimeFormat().resolvedOptions().locale] : []);
};

/** ⚖️ Chooses the language without throwing: `--lang`, `SEMIO_LANGUAGE` (both must name an offered language), `LANGUAGE` (first offered entry), the first non-empty of `LC_ALL`, `LC_MESSAGES`, `LANG`, then the operating system list; a source that names no offered language is skipped. */
export function chooseLanguageV1<T extends string>(languages: readonly T[], args: readonly string[], environment: LanguageEnvironmentV1, osLanguages: OsLanguageListV1 = osLanguageListV1): LanguageResolutionV1<T> | Readonly<{ refusal: LanguageRefusalV1 }> {
  const offered: readonly string[] = languages, chosen = (tag: string | undefined): tag is T => tag !== undefined && offered.includes(tag);
  const flag = languageFlagV1(args);
  if (flag !== undefined) return chosen(flag) ? { language: flag, source: protocol.flag } : { refusal: { kind: "flag", value: flag } };
  const explicit = environment[protocol.explicitVariable];
  if (explicit !== undefined && explicit !== "") {
    const tag = languageSubtagV1(explicit);
    return chosen(tag) ? { language: tag, source: protocol.explicitVariable } : { refusal: { kind: "variable", name: protocol.explicitVariable, value: explicit } };
  }
  for (const entry of (environment[protocol.preferenceListVariable] ?? "").split(protocol.preferenceListSeparator)) {
    const tag = languageSubtagV1(entry);
    if (chosen(tag)) return { language: tag, source: protocol.preferenceListVariable };
  }
  const name = protocol.environmentOrder.find((candidate) => (environment[candidate] ?? "") !== "");
  if (name !== undefined) {
    const tag = languageSubtagV1(environment[name]);
    if (chosen(tag)) return { language: tag, source: name };
  }
  for (const entry of osLanguages()) {
    const tag = languageSubtagV1(entry);
    if (chosen(tag)) return { language: tag, source: "os" };
  }
  return { refusal: { kind: "none" } };
}

/** 🚫️ The refusal of a command, one line per offered language (a language without its own text reads the English one); it never depends on a language being chosen. */
export function languageRefusalV1(languages: readonly string[], refusal: LanguageRefusalV1 = { kind: "none" }): Error {
  const row: Readonly<Record<string, string>> = protocol.refusal[refusal.kind], values: Readonly<Record<string, string>> = { choice: languages.join("|"), name: refusal.name ?? "", value: refusal.value ?? "" };
  return new Error(languages.map((language) => (row[language] ?? row.en!).replace(/\{(choice|name|value)\}/gu, (_whole, key: string) => values[key]!)).join("\n"));
}

/** 🗣️ The language of a command; when none is chosen the bilingual refusal of the protocol is thrown. */
export function resolveLanguageV1<T extends string>(languages: readonly T[], args: readonly string[], environment: LanguageEnvironmentV1, osLanguages: OsLanguageListV1 = osLanguageListV1): LanguageResolutionV1<T> {
  const choice = chooseLanguageV1(languages, args, environment, osLanguages);
  if ("refusal" in choice) throw languageRefusalV1(languages, choice.refusal);
  return choice;
}

/** 🌐️ The languages a notice is written in: the one the arguments and the environment choose, or every offered language; a notice never reads the operating system, so one environment gives the same notice on every machine (`withoutChoice.notices`). */
export function noticeLanguagesV1<T extends string>(languages: readonly T[], environment: LanguageEnvironmentV1 = process.env, args: readonly string[] = []): readonly T[] {
  const choice = chooseLanguageV1(languages, args, environment, () => []);
  return "refusal" in choice ? languages : [choice.language];
}
