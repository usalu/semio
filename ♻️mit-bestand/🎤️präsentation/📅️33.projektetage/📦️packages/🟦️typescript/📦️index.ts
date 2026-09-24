// #region 🧲️Header
/** @emoji 📽️ 33. Projektetage — declarative paper intro via `@semio-tech/presentation`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import {
  buildResolutionScope,
  collectPresentationSlides,
  countArrangements,
  arrangementRestDispositions,
  expandThoughtSlides,
  loadPresentationFromSlideGlob,
  PRESENTATION_DEFAULT_SLIDE_ASPECT,
  resolveArrangement,
  type Presentation,
  type Slide,
  type SlideFile,
  type Thought,
} from "@semio-tech/presentation";
import "../../🎨️globals.css";
// #endregion 🔌️Adapters

//#region 🔖️spec
import { BAUKOMPONENTEN_ITEMS, CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3, ZUKUNFT_BAU_EMBODIMENT, ZUKUNFT_BAU_FRAME, ZUKUNFT_BAU_PARTICIPANT, columnLabelMorphFrom, inlineColumnLabelPosition, presentationMeta, zukunftBauEmbodiment, zukunftBauParticipant } from "./🔖️spec.ts";
export * from "./🔖️spec.ts";
//#endregion 🔖️spec

//#region 🔖️Deck
const slideModuleLoaders = import.meta.glob<{ default: SlideFile }>("../../🎞️slide/**/*.ts");
const slideModules = Object.fromEntries(await Promise.all(Object.entries(slideModuleLoaders).map(async ([path, loadModule]) => [path, await loadModule()] as const))) as Record<string, { readonly default: SlideFile }>;
const sourceDeck: Presentation = loadPresentationFromSlideGlob(presentationMeta, slideModules);

const CHAPTER_ORDER = ["Einführung", "Recherche", "Bauteilportal", "Entwurfswerkzeug"] as const;

function reorderChapters(presentation: Presentation): Presentation {
  const byName = new Map(presentation.chapters.map((chapter) => [chapter.name, chapter]));
  return {
    ...presentation,
    chapters: CHAPTER_ORDER.map((name) => {
      const chapter = byName.get(name);
      if (!chapter) {
        throw new Error(`reorderChapters: missing chapter "${name}".`);
      }
      return chapter;
    }),
  };
}
const INTRO_TITLE_PARTICIPANT = "title";
const INTRO_TITLE_MORPH_FRAME = { x: 0.05, y: 0.36, width: 0.9, height: 0.28 };

function zukunftBauSlide(id: string, name: string): Slide {
  return {
    arrangement: {
      id,
      name,
      dispositions: [
        {
          participantId: ZUKUNFT_BAU_PARTICIPANT,
          embodimentId: ZUKUNFT_BAU_EMBODIMENT,
          emphasis: "active",
          position: ZUKUNFT_BAU_FRAME,
        },
      ],
    },
  };
}

function addZukunftBauTitleMorph(slide: Slide | undefined): Slide | undefined {
  if (!slide) {
    return slide;
  }
  return {
    ...slide,
    arrangement: {
      ...slide.arrangement,
      dispositions: slide.arrangement.dispositions.map((disposition) =>
        disposition.participantId === INTRO_TITLE_PARTICIPANT
          ? {
              ...disposition,
              position: INTRO_TITLE_MORPH_FRAME,
              morphFrom: [
                ...(disposition.morphFrom ?? []),
                {
                  participantId: ZUKUNFT_BAU_PARTICIPANT,
                  embodimentId: ZUKUNFT_BAU_EMBODIMENT,
                  position: INTRO_TITLE_MORPH_FRAME,
                },
              ],
            }
          : disposition,
      ),
    },
  };
}

function addZukunftBauScope(thought: Thought): Thought {
  const participants = thought.participants?.some((participant) => participant.id === zukunftBauParticipant.id) ? thought.participants : [...(thought.participants ?? []), zukunftBauParticipant];
  const embodiments = thought.embodiments?.some((embodiment) => embodiment.id === zukunftBauEmbodiment.id) ? thought.embodiments : [...(thought.embodiments ?? []), zukunftBauEmbodiment];
  return {
    ...thought,
    participants,
    embodiments,
  };
}

function addZukunftBauBookends(presentation: Presentation): Presentation {
  const firstSlide = {
    ...zukunftBauSlide("zukunft-bau-auftakt", "Zukunft Bau Auftakt"),
    transition: { kind: "morph" as const },
  };
  return {
    ...presentation,
    chapters: presentation.chapters.map((chapter) => ({
      ...chapter,
      sequences: chapter.sequences.map((sequence) => ({
        ...sequence,
        thoughts: sequence.thoughts.map((thought) => {
          const isIntroThought = chapter.name === "Einführung" && sequence.name === "Einleitung" && thought.name === "Einleitung";
          if (!isIntroThought) {
            return thought;
          }
          const scoped = addZukunftBauScope(thought);
          const [titleSlide, ...restSlides] = scoped.slides;
          return {
            ...scoped,
            slides: [firstSlide, ...(titleSlide ? [addZukunftBauTitleMorph(titleSlide)] : []), ...restSlides],
          };
        }),
      })),
    })),
  };
}

export const deck: Presentation = addZukunftBauBookends(reorderChapters(sourceDeck));

function mount(): void {
  const el = document.getElementById("root");
  if (!el) {
    return;
  }
  void Promise.all([import("@semio-tech/animate-js"), import("@semio-tech/ui-react")]).then(([{ mountPresentation }, { DEFAULT_UI_DRIVER }]) => {
    mountPresentation(el, deck, {
      transition: "fade",
      slideNumber: false,
      surfaceChrome: { appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER },
    });
  });
}

if (typeof document !== "undefined" && !import.meta.vitest) {
  mount();
}
//#endregion 🔖️Deck

//#region 🔖️Play
export { presentationPlayAppDefinition as projektetagePlayAppDefinition } from "@semio-tech/presentation";
//#endregion 🔖️Play

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerProjektetageDeckTests } = await import("../../🧪️tests/🧪️projektetage-deck/🟦️.ts");
  await registerProjektetageDeckTests(import.meta.vitest, { BAUKOMPONENTEN_ITEMS, CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3, INTRO_TITLE_MORPH_FRAME, PRESENTATION_DEFAULT_SLIDE_ASPECT, ZUKUNFT_BAU_EMBODIMENT, ZUKUNFT_BAU_FRAME, ZUKUNFT_BAU_PARTICIPANT, arrangementRestDispositions, buildResolutionScope, collectPresentationSlides, columnLabelMorphFrom, countArrangements, deck, expandThoughtSlides, inlineColumnLabelPosition, resolveArrangement }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
