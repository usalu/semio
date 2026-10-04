/** 🔭️ Preview of the quiz's pets glue (work packages I and C3): the quiz's own cards, its preferences panel with what the
 * learner allows and "Play with the pets", a quiz page's marked task list, a classification in its topic, and the glue
 * (`QuizPetsProvider`, `QuizPets`) with the real pet layer, in a real browser and without a proctor. The buttons of the
 * first card move the step, so the scene, the quiet of a run and the names under the choice can be watched. The cast is
 * the sample menagerie of the pets product's schema conformance vectors, with one more scene for the quiz.
 * @see ./vite.config.ts — how to start it */
import "../../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎨️.css";
import { StrictMode, useLayoutEffect, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";
import { DEFAULT_UI_DRIVER, UI_AVAILABLE_HEIGHT, bootstrapElementsSurfaceChromeDocument, useElementsSurfaceChrome } from "@semio-tech/ui-react/chrome";
import type { Menagerie } from "@semio-tech/pets";
import type { CatalogView, RunView, SheetClassificationTask } from "@semio-tech/quiz";
import { BodyButton, CardIcon, ClassificationTaskView, PreferencesPanelCard, QuizCard, applyLocale, petScene, quizText, textScale, type QuizPreferences, type QuizStep } from "@semio-tech/quiz-react";
import { PetTopic, QuizPets, QuizPetsProvider, petProp, type QuizPetsSource } from "../../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx";
import vectors from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";

const text = (en: string, de: string) => ({ en, de });
const sample = vectors.menagerie as unknown as Menagerie;
const SAMPLE: Menagerie = { ...sample, casts: [...sample.casts, { scene: "heating", core: ["floaty"], rotation: ["hoppy"] }] };
const CATALOG: CatalogView = {
  id: "preview",
  title: text("Preview", "Vorschau"),
  introduction: { title: text("Welcome", "Willkommen"), paragraphs: [] },
  quizzes: [{ id: "heating", emoji: "🔥", title: text("Heating", "Heizen"), description: text("Warmth.", "Wärme."), tasks: [] }],
  badges: [],
};
const TASK: SheetClassificationTask = {
  kind: "classification",
  id: "u-values",
  title: text("U-values", "U-Werte"),
  prompt: text("Assign each part.", "Ordne jedes Bauteil zu."),
  categories: [{ id: "good", label: text("Good", "Gut") }],
  items: [
    { id: "wall-geg", label: text("Wall (GEG)", "Wand (GEG)") },
    { id: "single-glazing", label: text("Single glazing", "Einfachverglasung") },
  ],
};
const RUNS = { r1: { run: "r1", quiz: "heating" } as RunView };
const STEPS: readonly { readonly label: string; readonly step: QuizStep }[] = [
  { label: "home", step: { screen: "home" } },
  { label: "quiz page", step: { screen: "home", page: "heating" } },
  { label: "run", step: { screen: "run", run: "r1" } },
  { label: "results", step: { screen: "results", run: "r1" } },
];
const source: QuizPetsSource = async () => SAMPLE;

function Preview(): ReactElement {
  const [preferences, setPreferences] = useState<QuizPreferences>({ theme: "system", textSize: "normal", showCursors: true, others: "submitted", animateIcons: true, iconsChosen: false, pets: "calm", petsLiveliness: "calm", petsChosen: false, petsPlay: true, petsMischief: true, challenge: "medium" });
  const [step, setStep] = useState<QuizStep>({ screen: "home" });
  const locale = preferences.locale ?? "en";
  const words = quizText(locale);
  useElementsSurfaceChrome({ appearance: preferences.theme, device: "desktop", driver: DEFAULT_UI_DRIVER, browserDefaults: "native" });
  useLayoutEffect(() => {
    void applyLocale(locale);
    document.documentElement.style.setProperty("--quiz-text-scale", String(textScale(preferences.textSize)));
  }, [locale, preferences.textSize]);
  return (
    <QuizPetsProvider source={source} choice={preferences.pets} chosen={preferences.petsChosen} play={preferences.petsPlay} mischief={preferences.petsMischief} state={{ step, runs: RUNS, catalog: CATALOG }} locale={locale}>
      <div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale} data-pets={preferences.pets}>
        <main id="quiz-main" className="flex min-h-0 flex-1 flex-col overflow-auto p-double">
          <div className="quiz-page quiz-pair mx-auto w-full max-w-6xl">
            <div className="flex flex-col gap-double">
              <QuizCard id="steps" card="steps" topic="heating" icon={<CardIcon icon="info" />} title={`Step: ${step.screen} · scene: ${petScene(step, RUNS, CATALOG)}`}>
                <p className="m-0 text-sm">A card whose top edge carries pets; its text and buttons are kept clear.</p>
                <div className="flex flex-wrap gap-single">
                  {STEPS.map((entry) => (
                    <BodyButton key={entry.label} onClick={() => setStep(entry.step)}>
                      {entry.label}
                    </BodyButton>
                  ))}
                </div>
              </QuizCard>
              <QuizCard id="tasks" card="quiz-tasks" icon={<CardIcon icon="list" />} title="Tasks in this quiz">
                <ol role="list" className="m-0 flex list-none flex-col gap-single p-0">
                  {["u-values", "heating-load-and-demand"].map((task, index) => (
                    <li key={task} data-pet-prop={petProp("heating", task)} className="flex min-w-0 items-center gap-double border border-normal px-double py-single text-sm">
                      {index + 1}. {task}
                    </li>
                  ))}
                </ol>
              </QuizCard>
              <QuizCard id="task" card="task" icon={<CardIcon icon="info" />} title="U-values">
                <PetTopic topic={petProp("heating", TASK.id)}>
                  <ClassificationTaskView task={TASK} answer={undefined} onAnswer={() => undefined} text={words} locale={locale} />
                </PetTopic>
              </QuizCard>
            </div>
            <PreferencesPanelCard preferences={preferences} locale={locale} text={words} onChange={setPreferences} />
          </div>
        </main>
        <QuizPets />
      </div>
    </QuizPetsProvider>
  );
}

bootstrapElementsSurfaceChromeDocument("system");
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Preview />
  </StrictMode>,
);
