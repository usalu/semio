import { mountUiRoot } from "@semio-tech/ui-react/runtime";
import { LanguageChoice, bootstrapElementsSurfaceChromeDocument, initUiLocaleSync, readStoredUiChromeAppearance } from "@semio-tech/ui-react";
import { PlayLanding, playStorage } from "../../../../../../../🏢️semio-tech/🎡️play/🏠️play-landing.tsx";
import "../../../../../../../🏢️semio-tech/🎡️play/🎨️globals.css";

/** 🧪️ Border-review harness: the real play landing with inert pane bodies (no wasm engines are built here). */
bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(playStorage));
const params = new URLSearchParams(location.search);
const locale = params.get("locale") === "de" ? "de" : "en";
initUiLocaleSync(locale);
const OPTIONS = [{ value: "en", endonym: "English" }, { value: "de", endonym: "Deutsch" }] as const;
const view = params.get("view") ?? "landing";
mountUiRoot(
  document.getElementById("root")!,
  view === "choice" ? (
    <main className="flex min-h-screen flex-col items-center justify-center gap-single" data-slot="play-language-choice">
      <h1><span lang="en">Choose your language</span> / <span lang="de">Sprache auswählen</span></h1>
      <LanguageChoice label="Language · Sprache" options={OPTIONS as never} onChoose={() => undefined} />
    </main>
  ) : (
    <PlayLanding locale={locale} renderPane={(pane) => <div data-slot="inert-pane" className="size-full">{pane.label}</div>} />
  ),
);
