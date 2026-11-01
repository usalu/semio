// #region 🧲️Header
/** 🌐️ The demonstrator page's own texts: `🔣️.json` is the one source, every key carries an English and a German cell, and no lookup falls back to another locale. */
// #endregion 🧲️Header

import catalog from "./🔣️.json" with { type: "json" };

/** 🌍️ A locale the demonstrator ships texts for. */
export type DemonstratorLocale = "en" | "de";

/** 🗝️ A key of {@link catalog}. */
export type DemonstratorTextKey = keyof typeof catalog.texts;

/** 🏷️ A key naming a label with `normal` and `beginner` wording. */
export type DemonstratorLabelKey = DemonstratorTextKey extends infer K ? (K extends `${infer P}.label.normal` ? P : never) : never;

/** 🌎️ Every locale the demonstrator ships, English first and German second; none is preselected. */
export const DEMONSTRATOR_LOCALES: readonly DemonstratorLocale[] = ["en", "de"];

/** 🗣️ Each locale named in its own words, shown to a viewer who has not chosen a language yet. */
export const DEMONSTRATOR_LOCALE_ENDONYMS: Readonly<Record<DemonstratorLocale, string>> = { en: "English", de: "Deutsch" };

/** 🧭️ The first candidate language tag (`en`, `de-AT`, ...) naming a shipped locale; `undefined` when none does, so the viewer is asked instead of served a guessed language. */
export function resolveDemonstratorLocale(candidates: readonly (string | null | undefined)[]): DemonstratorLocale | undefined {
  for (const candidate of candidates) {
    const primary = candidate?.toLowerCase().split("-")[0];
    const match = DEMONSTRATOR_LOCALES.find((locale) => locale === primary);
    if (match !== undefined) return match;
  }
  return undefined;
}

/** 📖️ The text of `key` in `locale`; there is no fallback, an unknown key is a defect. */
export function demonstratorText(key: DemonstratorTextKey, locale: DemonstratorLocale): string {
  return catalog.texts[key][locale];
}

/** 🧵️ The text of `key` in `locale` with every `{{name}}` placeholder filled from `params`; a placeholder without a value is a defect and throws. */
export function demonstratorFormat(key: DemonstratorTextKey, locale: DemonstratorLocale, params: Readonly<Record<string, string>>): string {
  return demonstratorText(key, locale).replace(/\{\{(\w+)\}\}/g, (_, name: string) => {
    const value = params[name];
    if (value === undefined) throw new Error(`demonstrator text ${key} lacks the placeholder value ${name}`);
    return value;
  });
}

/** 🏷️ The wire shape of a manifest `LocalizedLabel`: one cell per terminology and locale, here the same text for every terminology. */
export type DemonstratorLabel = Readonly<Record<"native" | "reuse", Readonly<Record<DemonstratorLocale, string>>>>;

/** 🗂️ Key prefix of the manifest labels (introductions, tutorial, brand); they are resolved by the shell from the wire shape, never through the UI translation port. */
const BRAND_KEY_PREFIX = "demonstrator.brand.";

/** 🏷️ The manifest label of `key`, carrying its English and German cell for every terminology so the shell resolves it in whatever locale it stands at. */
export function demonstratorLabel(key: DemonstratorTextKey): DemonstratorLabel {
  const cells = { en: demonstratorText(key, "en"), de: demonstratorText(key, "de") };
  return { native: cells, reuse: cells };
}

type TextTree<K extends string> = K extends `${infer Head}.${infer Rest}` ? { readonly [P in Head]: TextTree<Rest> } : { readonly [P in K]: string };
type MergedTree<U> = (U extends unknown ? (value: U) => void : never) extends (value: infer I) => void ? I : never;

/** 🌲️ The translation tree of {@link catalog}, typed by its own dot-path keys so the UI port can derive the registered key union. */
export type DemonstratorTranslationTree = MergedTree<TextTree<Exclude<DemonstratorTextKey, `demonstrator.brand.${string}`>>>;

/** 🌳️ The nested translation tree of one locale, the shape the UI translation port registers. */
export function demonstratorTranslationTree(locale: DemonstratorLocale): DemonstratorTranslationTree {
  const tree: Record<string, unknown> = {};
  for (const [key, cells] of Object.entries(catalog.texts)) {
    if (key.startsWith(BRAND_KEY_PREFIX)) continue;
    const path = key.split(".");
    const leaf = path.pop()!;
    let node = tree;
    for (const part of path) node = (node[part] ??= {}) as Record<string, unknown>;
    node[leaf] = cells[locale];
  }
  return tree as DemonstratorTranslationTree;
}
