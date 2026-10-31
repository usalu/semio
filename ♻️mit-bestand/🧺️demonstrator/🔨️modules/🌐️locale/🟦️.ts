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

/** 🔒️ The one locale the whole demonstrator is locked to. */
export const DEMONSTRATOR_LOCKED_LOCALE: DemonstratorLocale = catalog.lockedLocale === "en" ? "en" : catalog.lockedLocale === "de" ? "de" : unsupportedLocale(catalog.lockedLocale);

function unsupportedLocale(locale: string): never {
  throw new Error(`Unsupported demonstrator locale: ${locale}`);
}

/** 📖️ The text of `key` in `locale`; there is no fallback, an unknown key is a defect. */
export function demonstratorText(key: DemonstratorTextKey, locale: DemonstratorLocale): string {
  return catalog.texts[key][locale];
}

/** 🧵️ The text of `key` in `locale` with every `{{name}}` placeholder filled from `params`. */
export function demonstratorFormat(key: DemonstratorTextKey, locale: DemonstratorLocale, params: Readonly<Record<string, string>>): string {
  return demonstratorText(key, locale).replace(/\{\{(\w+)\}\}/g, (_, name: string) => params[name] ?? "");
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
