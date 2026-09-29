// Moves the ui-react i18n port (locale storage, chrome bundles, port, shell instances, locale resolution) out of the
// 11 800-line React target barrel into `🎯️targets/⚛️react/🌐️i18n/🟦️.ts`, leaving an import + re-export in its place.
// Usage: node extract_ui_i18n_port.mjs <repo root>. Blocks are located by exact marker lines; nothing is written
// unless every marker is found in order and the barrel is unchanged between the read and the write.
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = process.argv[2];
const barrel = join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx");
const directory = join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n");
const moduleFile = join(directory, "🟦️.ts");
if (existsSync(moduleFile)) throw new Error(`${moduleFile} already exists`);

const original = readFileSync(barrel, "utf8");
const eol = original.includes("\r\n") ? "\r\n" : "\n";
const lines = original.split(/\r?\n/u);

function find(predicate, from, label) {
  const index = lines.findIndex((line, position) => position >= from && predicate(line));
  if (index < 0) throw new Error(`marker not found: ${label}`);
  return index;
}

const aStart = find((line) => line === "/** @emoji 🌐️ Storage key for the active UI locale. */", 0, "locale storage key");
if (!lines[aStart + 1].startsWith("export const UI_CHROME_LOCALE_STORAGE_KEY")) throw new Error("unexpected locale storage block");
const aFunction = find((line) => line.startsWith("export function writeStoredUiChromeLocale("), aStart, "writeStoredUiChromeLocale");
const aEnd = find((line) => line === "}", aFunction, "end of writeStoredUiChromeLocale");
const bStart = find((line) => line === "// #region 🇩️🇪️ German Bundle", aEnd, "German bundle");
const bEnd = find((line) => line === "} satisfies Record<UiLocale, { readonly translation: UiTranslationSchema }>;", bStart, "end of chrome bundles");
const cStart = find((line) => line === "// #region 🔌️I18n Port", bEnd, "i18n port");
const cEnd = find((line) => line === "// #endregion 🐚️ShellI18n", cStart, "end of shell i18n");
const dStart = find((line) => line.startsWith("function normalizeUiLocale("), cEnd, "normalizeUiLocale");
const dEndMarker = find((line) => line === "// #endregion 🔌️I18n Port", dStart, "end of i18n port");
const i18nextImport = find((line) => line === 'import i18next from "i18next";', 0, "i18next import");
const reactI18nextImport = find((line) => line === 'import { I18nextProvider, initReactI18next, useTranslation } from "react-i18next";', 0, "react-i18next import");
if (!(i18nextImport < aStart && reactI18nextImport < aStart)) throw new Error("imports are not above the moved blocks");

const block = (start, end) => lines.slice(start, end + 1);
const moved = {
  storage: block(aStart, aEnd),
  bundles: block(bStart, bEnd),
  port: block(cStart + 1, cEnd),
  locale: block(dStart, dEndMarker - 1),
};

const header = [
  "/** 🌐️ The UI i18n port on its own: the domain-neutral chrome bundles (English and German), the shared",
  " * i18next-backed {@link uiI18n}, product bundle registration, per-shell instances and locale resolution —",
  " * importable without the rest of the React target as `@semio-tech/ui-react/i18n`. i18next and react-i18next stay",
  " * behind the owned {@link UiI18nPort}; the React target barrel re-exports everything here unchanged.",
  " *",
  " * @see ../../../🧱️elements/📚️I18n/🟦️.tsx — the port interface, label shapes and key types",
  " * @see ../🟦️.tsx — the barrel that re-exports this module",
  " */",
  "",
  'import i18next from "i18next";',
  'import { initReactI18next } from "react-i18next";',
  'import { createBrowserStoragePort, ephemeralSet, type ShellLocale, type StoragePort } from "@semio-tech/framework";',
  "import {",
  "  resolveUiLabel,",
  "  type DeepUiTranslationKeys,",
  "  type UiI18nPort,",
  "  type UiLabelPair,",
  "  type UiLabelValue,",
  "  type UiLocale,",
  "  type UiRegisteredTranslationKey,",
  "  type UiRibbonParentEntries,",
  "  type UiTranslateFn,",
  "  type UiTranslationKey,",
  "  type UiTranslationSchema,",
  '} from "../../../🧱️elements/📚️I18n/🟦️.tsx";',
  "",
  "export { resolveUiLabel };",
  "export type { DeepUiTranslationKeys, UiI18nPort, UiLabelPair, UiLabelValue, UiLocale, UiRegisteredTranslationKey, UiTranslateFn, UiTranslationKey, UiTranslationSchema };",
  "",
  "// #region 🌐️LocaleStorage",
];

const moduleLines = [...header, ...moved.storage, "// #endregion 🌐️LocaleStorage", "", ...moved.bundles, "", "// #region 🔌️I18n Port", ...moved.port, "", ...moved.locale, "// #endregion 🔌️I18n Port", ""];

const reexport = [
  "// #region 🔌️I18n Port",
  "// The port lives in `./🌐️i18n/🟦️.ts` (importable alone as `@semio-tech/ui-react/i18n`) and is re-exported here unchanged.",
  "import {",
  "  UI_CHROME_LOCALE_STORAGE_KEY,",
  "  createShellI18nInstance,",
  "  detectShellLocale,",
  "  disposeShellI18nInstance,",
  "  initUiLocaleSync,",
  "  readStoredUiChromeLocale,",
  "  registerUiTranslationBundles,",
  "  setUiLocale,",
  "  uiChromeTranslationBundles,",
  "  uiI18n,",
  "  writeStoredUiChromeLocale,",
  "  type UiTranslationBundlesInput,",
  "  type UiTranslationLocaleCode,",
  '} from "./🌐️i18n/🟦️.ts";',
  "export { UI_CHROME_LOCALE_STORAGE_KEY, createShellI18nInstance, detectShellLocale, disposeShellI18nInstance, initUiLocaleSync, readStoredUiChromeLocale, registerUiTranslationBundles, setUiLocale, uiChromeTranslationBundles, uiI18n, writeStoredUiChromeLocale };",
  "export type { UiTranslationBundlesInput, UiTranslationLocaleCode };",
  "",
];

const kept = [
  ...lines.slice(0, i18nextImport),
  ...lines.slice(i18nextImport + 1, reactI18nextImport),
  'import { I18nextProvider, useTranslation } from "react-i18next";',
  ...lines.slice(reactI18nextImport + 1, aStart),
  ...lines.slice(aEnd + 1, bStart),
  ...lines.slice(bEnd + 1, cStart),
  ...reexport,
  ...lines.slice(cEnd + 1, dStart),
  ...lines.slice(dEndMarker),
];

if (readFileSync(barrel, "utf8") !== original) throw new Error("the barrel changed while extracting; nothing written");
mkdirSync(directory, { recursive: false });
writeFileSync(moduleFile, moduleLines.join(eol));
writeFileSync(barrel, kept.join(eol));
console.log(JSON.stringify({ eol: eol === "\r\n" ? "crlf" : "lf", barrelBefore: lines.length, barrelAfter: kept.length, module: moduleLines.length, blocks: { storage: [aStart + 1, aEnd + 1], bundles: [bStart + 1, bEnd + 1], port: [cStart + 1, cEnd + 1], locale: [dStart + 1, dEndMarker] } }));
