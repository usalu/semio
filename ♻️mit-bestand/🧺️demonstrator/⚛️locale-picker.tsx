// #region 🧲️Header
/** 🌐️ The explicit language choice of the landing page: shown while neither a stored choice nor the browser's preferred languages name a shipped locale, and as the switcher in the navbar. */
// #endregion 🧲️Header

import { cn, useLabel } from "@semio-tech/ui-react";
import { demonstratorUiLabel } from "./🔨️modules/🌐️locale/⚛️.ts";
import { DEMONSTRATOR_LOCALES, DEMONSTRATOR_LOCALE_ENDONYMS, demonstratorText, type DemonstratorLocale } from "./🔨️modules/🌐️locale/🟦️.ts";

/** 🌐️ Every locale offered in its own words, none preselected, before the page has any language to speak in. */
export function DemonstratorLocalePicker({ onChoose }: { readonly onChoose: (locale: DemonstratorLocale) => void }) {
  return (
    <main aria-labelledby="demonstrator-locale-picker-title" data-slot="demonstrator-locale-picker" className="grid h-full w-full place-items-center">
      <div className="grid justify-items-center gap-double">
        <h1 id="demonstrator-locale-picker-title" className="m-0 text-lg font-semibold">
          {DEMONSTRATOR_LOCALES.map((locale, index) => (
            <span key={locale} lang={locale}>
              {index === 0 ? "" : " / "}
              {demonstratorText("demonstrator.locale.choose", locale)}
            </span>
          ))}
        </h1>
        <div className="flex gap-double">
          {DEMONSTRATOR_LOCALES.map((locale) => (
            <button key={locale} type="button" lang={locale} data-locale={locale} onClick={() => onChoose(locale)} className="cursor-pointer rounded-sm border border-border px-double py-single text-sm hover:bg-hover-window">
              {DEMONSTRATOR_LOCALE_ENDONYMS[locale]}
            </button>
          ))}
        </div>
      </div>
    </main>
  );
}

/** 🔤️ The navbar switcher: one pressed-state button per locale, each in its own language, grouped under a localized name. */
export function DemonstratorLocaleSwitcher({ locale, onChoose }: { readonly locale: DemonstratorLocale; readonly onChoose: (locale: DemonstratorLocale) => void }) {
  const groupLabel = useLabel(demonstratorUiLabel("demonstrator.landing.language"));
  return (
    <div role="group" aria-label={groupLabel} data-slot="demonstrator-locale-switcher" className="pointer-events-auto inline-flex items-center gap-tiny">
      {DEMONSTRATOR_LOCALES.map((candidate) => (
        <button
          key={candidate}
          type="button"
          lang={candidate}
          data-locale={candidate}
          aria-pressed={candidate === locale}
          onClick={() => onChoose(candidate)}
          className={cn("cursor-pointer rounded-sm px-single py-half text-xs transition-colors hover:bg-hover-window", candidate === locale ? "font-semibold text-foreground" : "text-muted-foreground hover:text-foreground")}
        >
          {DEMONSTRATOR_LOCALE_ENDONYMS[candidate]}
        </button>
      ))}
    </div>
  );
}
