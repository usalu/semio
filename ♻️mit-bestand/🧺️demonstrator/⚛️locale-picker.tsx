// #region 🧲️Header
/** 🌐️ The explicit language choice of the landing page: shown while neither a stored choice nor the browser's preferred languages name a shipped locale, and as the switcher in the navbar. */
// #endregion 🧲️Header

import { LanguageChoice, useLabel, type LanguageChoiceOption } from "@semio-tech/ui-react";
import { demonstratorUiLabel } from "./🔨️modules/🌐️locale/⚛️.ts";
import { DEMONSTRATOR_LOCALES, DEMONSTRATOR_LOCALE_ENDONYMS, demonstratorText, type DemonstratorLocale } from "./🔨️modules/🌐️locale/🟦️.ts";

const LOCALE_OPTIONS: readonly LanguageChoiceOption<DemonstratorLocale>[] = DEMONSTRATOR_LOCALES.map((value) => ({ value, endonym: DEMONSTRATOR_LOCALE_ENDONYMS[value] }));

/** 🌐️ Every locale offered in its own words, none preselected, before the page has any language to speak in. */
export function DemonstratorLocalePicker({ onChoose }: { readonly onChoose: (locale: DemonstratorLocale) => void }) {
  return (
    <main aria-labelledby="demonstrator-locale-picker-title" data-slot="demonstrator-locale-picker" className="grid h-full w-full place-items-center">
      <div className="grid justify-items-center gap-double">
        <h1 id="demonstrator-locale-picker-title" className="m-0 text-lg font-semibold">
          {DEMONSTRATOR_LOCALES.map((locale) => (
            <span key={locale} lang={locale} className="block">
              {demonstratorText("demonstrator.locale.choose", locale)}
            </span>
          ))}
        </h1>
        <LanguageChoice label={DEMONSTRATOR_LOCALES.map((locale) => demonstratorText("demonstrator.locale.choose", locale)).join(" / ")} options={LOCALE_OPTIONS} onChoose={onChoose} />
      </div>
    </main>
  );
}

/** 🔤️ The navbar switcher: one radio per locale, each in its own language, grouped under a localized name. */
export function DemonstratorLocaleSwitcher({ locale, onChoose }: { readonly locale: DemonstratorLocale; readonly onChoose: (locale: DemonstratorLocale) => void }) {
  const groupLabel = useLabel(demonstratorUiLabel("demonstrator.landing.language"));
  return <LanguageChoice label={groupLabel} options={LOCALE_OPTIONS} value={locale} onChoose={onChoose} className="pointer-events-auto" />;
}
