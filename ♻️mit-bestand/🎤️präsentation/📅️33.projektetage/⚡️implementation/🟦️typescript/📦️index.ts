// #region 🧲️Header
/** @emoji 📽️ 33. Projektetage — declarative paper intro via `@semio-tech/animate-present-core`. */
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
} from "@semio-tech/animate-present-core";
import "../🎨️globals.css";
// #endregion 🔌️Adapters

//#region 🔖️spec
// #region 🔌️Adapters
import {
  remapSplitDispositions,
  split,
  splitFigureGrid,
  MEDIA_SCROLL_ORIGIN_TOP_LEFT,
  type Disposition,
  type DispositionPosition,
  type Embodiment,
  type MorphToSlot,
  type IntroSpec,
  type Participant,
  type PresentationMeta,
  type SplitArtifacts,
  unionSourceCrops,
} from "@semio-tech/animate-present-core";
// #endregion 🔌️Adapters

//#region 🔖️Meta
export const presentationMeta: PresentationMeta = {
  id: "projektetage",
  name: "33. Projektetage",
  language: "de",
};

export const introSpec: IntroSpec = {
  id: "projektetage",
  name: "33. Projektetage",
  language: "de",
  title: {
    full: ["Entwerfen mit Bestand"],
    short: "Entwerfen mit Bestand",
  },
  description: {
    full: ["Eine offene Plattform für einen KI-unterstützten, performance-optimierten und integrativen Entwurfsprozess mit wiederverwendeten Baukomponenten"],
    short: "Plattform zum Entwerfen mit wiederverwendete Bauteilen",
  },
  goal: ["Mehr Zeit zum manuellen Entwerfen", "dank Automatisierung!"],
  authors: {
    lines: [
      [
        { name: "Ueli Saluz", marks: ["a", "1", "x"] },
        { name: "Phillipp Geyer", marks: ["a", "1", "x"] },
      ],
      [
        { name: "Kinan Sarakbi", marks: ["a", "2", "y"] },
        { name: "Christoph Gengnagel", marks: ["a", "2", "y"] },
      ],
    ],
  },
  affiliations: {
    steps: [
      [{ mark: "a", name: "Fakultät für Architektur" }],
      [
        { mark: "a", name: "Fakultät für Architektur" },
        { mark: "1", name: "Leibniz Universität Hannover" },
        { mark: "2", name: "Universität der Künste" },
      ],
      [
        { mark: "a", name: "Fakultät für Architektur" },
        {
          mark: "1",
          name: "Leibniz Universität Hannover",
          shortName: "LUH",
          suffix: { mark: "x", name: "Nachhaltige Gebäudesysteme" },
        },
        {
          mark: "2",
          name: "Universität der Künste",
          shortName: "UdK",
          suffix: { mark: "y", name: "Konstruktives Entwerfen" },
        },
      ],
    ],
  },
};
//#endregion 🔖️Meta

//#region 🔖️Catalogue
export const ASSET_CATALOGUE = "/🖼️bauteilbörse.png";
export const ASSET_VIDEO = "./🎥️bauen-mit-bestand.mp4";
export const ASSET_THESIS_PDF = "./📄️bachelor-thesis-ueli-saluz.pdf";
export const ASSET_ZUKUNFT_BAU_ENTWERFEN_MIT_BESTAND = "/🌐️zukunft-bau-entwerfen-mit-bestand.html";

export const CATALOGUE_PARTICIPANT = "catalogue";
export const CATALOGUE_COL1 = "catalogue-col1";
export const CATALOGUE_COL2 = "catalogue-col2";
export const CATALOGUE_COL3 = "catalogue-col3";
export const ZUKUNFT_BAU_PARTICIPANT = "zukunft-bau-entwerfen-mit-bestand";

export const CATALOGUE_EMBODIMENT_FULL = "catalogue--full";
export const CATALOGUE_EMBODIMENT_COL1_CROP = "catalogue-col1--crop";
export const CATALOGUE_EMBODIMENT_COL1_LABEL = "catalogue-col1--label";
export const CATALOGUE_EMBODIMENT_COL2_CROP = "catalogue-col2--crop";
export const CATALOGUE_EMBODIMENT_COL2_LABEL = "catalogue-col2--label";
export const CATALOGUE_EMBODIMENT_COL3_CROP = "catalogue-col3--crop";
export const CATALOGUE_EMBODIMENT_COL3_LABEL = "catalogue-col3--label";
export const ZUKUNFT_BAU_EMBODIMENT = "zukunft-bau-entwerfen-mit-bestand--iframe";

/** @emoji 📐️ `🖼️bauteilbörse.png` pixel width÷height (1222×896). */
export const CATALOGUE_SOURCE_ASPECT = 1222 / 896;

export const CATALOGUE_FRAME = {
  x: 0.127,
  y: 0.1,
  width: 0.746,
  height: 0.75,
};

export const ZUKUNFT_BAU_FRAME = {
  x: 0,
  y: 0,
  width: 1,
  height: 1,
};

export const zukunftBauParticipant: Participant = { id: ZUKUNFT_BAU_PARTICIPANT };

export const zukunftBauEmbodiment: Embodiment = {
  kind: "iframe",
  id: ZUKUNFT_BAU_EMBODIMENT,
  src: ASSET_ZUKUNFT_BAU_ENTWERFEN_MIT_BESTAND,
  title: "Zukunft Bau: Entwerfen mit Bestand",
};

/** @emoji 🏷️ Grid keys of all 3×5 catalogue tiles → semantic participant ids. */
export const CATALOGUE_TILE_SEMANTIC_KEYS = {
  "tile-r0-c0": "Struktur 1",
  "tile-r0-c1": "Struktur 2",
  "tile-r0-c2": "Flächen",
  "tile-r0-c3": "Elemente 1",
  "tile-r0-c4": "Elemente 2",
  "tile-r1-c0": "Rippenplatte 1",
  "tile-r1-c1": "Rippenplatte 2",
  "tile-r1-c2": "Rippenplatte 3",
  "tile-r1-c3": "Rippenplatte 4",
  "tile-r1-c4": "Rippenplatte 5",
  "tile-r2-c0": "Rippenplatte 6",
  "tile-r2-c1": "Unterzug 1",
  "tile-r2-c2": "Unterzug 2",
  "tile-r2-c3": "Unterzug 3",
  "tile-r2-c4": "Stütze",
} as const;

/** @emoji 🧩️ Applies semantic participant ids to split template artifacts. */
export function catalogueSplitWithSemanticKeys(artifacts: SplitArtifacts): SplitArtifacts {
  const keyMap = CATALOGUE_TILE_SEMANTIC_KEYS;
  const remapId = (gridKey: string): string => keyMap[gridKey as keyof typeof keyMap] ?? gridKey;
  const participants = artifacts.participants.map((participant) => ({
    id: remapId(participant.id),
  }));
  const embodiments = artifacts.embodiments.map((embodiment) => {
    const gridKey = embodiment.id.replace(/-figure$/, "");
    const semantic = remapId(gridKey);
    return { ...embodiment, id: `${semantic}-figure` };
  });
  const dispositions = artifacts.dispositions.map((disposition) => {
    const semantic = remapId(disposition.participantId);
    return {
      ...disposition,
      participantId: semantic,
      embodimentId: `${semantic}-figure`,
    };
  });
  return { participants, embodiments, dispositions };
}

const CATALOGUE_SPLIT_RAW = split({
  source: ASSET_CATALOGUE,
  rows: 3,
  columns: 5,
  frame: CATALOGUE_FRAME,
  gap: 0,
  sourceAspect: CATALOGUE_SOURCE_ASPECT,
});

export const CATALOGUE_SPLIT = catalogueSplitWithSemanticKeys(CATALOGUE_SPLIT_RAW);

/** @emoji 📐️ Union of normalized figure crops for participant ids. */
export function unionTileCropForParticipants(artifacts: SplitArtifacts, participantIds: readonly string[]): DispositionPosition {
  const crops = artifacts.dispositions
    .filter((disposition) => participantIds.includes(disposition.participantId))
    .map((disposition) => {
      const embodiment = artifacts.embodiments.find((entry) => entry.id === disposition.embodimentId);
      return embodiment?.crop;
    })
    .filter((crop): crop is DispositionPosition => crop !== undefined);
  return unionSourceCrops(crops);
}

/** @emoji 📐️ Bounding box of slide positions for participant ids. */
export function unionTilePositionForParticipants(artifacts: SplitArtifacts, participantIds: readonly string[]): DispositionPosition {
  const positions = artifacts.dispositions
    .filter((disposition) => participantIds.includes(disposition.participantId))
    .map((disposition) => disposition.position)
    .filter((position): position is DispositionPosition => position !== undefined);
  if (positions.length === 0) {
    throw new Error("unionTilePositionForParticipants: no positions matched.");
  }
  let minX = 1;
  let minY = 1;
  let maxX = 0;
  let maxY = 0;
  for (const position of positions) {
    minX = Math.min(minX, position.x);
    minY = Math.min(minY, position.y);
    maxX = Math.max(maxX, position.x + position.width);
    maxY = Math.max(maxY, position.y + position.height);
  }
  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** @emoji 📐️ Ten catalogue tiles (5–14) as three separated columns (2×3 | 1×3 | 1×1). */
export function catalogueFocusColumnTiles(): readonly { readonly participantId: string; readonly position: DispositionPosition }[] {
  const rowGap = 0.014;
  const innerGap = 0.01;
  const columnGap = 0.05;
  const col1Width = 0.44;
  const col2Width = 0.2;
  const col3Width = 0.2;
  const blockWidth = col1Width + columnGap + col2Width + columnGap + col3Width;
  const layout: DispositionPosition = {
    x: (1 - blockWidth) / 2,
    y: 0.11,
    width: blockWidth,
    height: 0.78,
  };
  const col1X = layout.x;
  const col2X = col1X + col1Width + columnGap;
  const col3X = col2X + col2Width + columnGap;
  const rowHeight = (layout.height - rowGap * 2) / 3;
  const cellW1 = (col1Width - innerGap) / 2;
  const col3Height = layout.height;
  const rowY = (row: number): number => layout.y + row * (rowHeight + rowGap);

  const placements: readonly { readonly gridKey: string; readonly position: DispositionPosition }[] = [
    { gridKey: "tile-r1-c0", position: { x: col1X, y: rowY(0), width: cellW1, height: rowHeight } },
    { gridKey: "tile-r1-c1", position: { x: col1X + cellW1 + innerGap, y: rowY(0), width: cellW1, height: rowHeight } },
    { gridKey: "tile-r1-c2", position: { x: col1X, y: rowY(1), width: cellW1, height: rowHeight } },
    { gridKey: "tile-r1-c3", position: { x: col1X + cellW1 + innerGap, y: rowY(1), width: cellW1, height: rowHeight } },
    { gridKey: "tile-r1-c4", position: { x: col1X, y: rowY(2), width: cellW1, height: rowHeight } },
    { gridKey: "tile-r2-c0", position: { x: col1X + cellW1 + innerGap, y: rowY(2), width: cellW1, height: rowHeight } },
    { gridKey: "tile-r2-c1", position: { x: col2X, y: rowY(0), width: col2Width, height: rowHeight } },
    { gridKey: "tile-r2-c2", position: { x: col2X, y: rowY(1), width: col2Width, height: rowHeight } },
    { gridKey: "tile-r2-c3", position: { x: col2X, y: rowY(2), width: col2Width, height: rowHeight } },
    { gridKey: "tile-r2-c4", position: { x: col3X, y: layout.y, width: col3Width, height: col3Height } },
  ];

  return placements.map(({ gridKey, position }) => {
    const semantic = CATALOGUE_TILE_SEMANTIC_KEYS[gridKey as keyof typeof CATALOGUE_TILE_SEMANTIC_KEYS] ?? gridKey;
    return { participantId: semantic, position };
  });
}

export const CATALOGUE_FOCUS_TILES = catalogueFocusColumnTiles();

export const CATALOGUE_COLUMN_TILE_KEYS = {
  col1: ["Rippenplatte 1", "Rippenplatte 2", "Rippenplatte 3", "Rippenplatte 4", "Rippenplatte 5", "Rippenplatte 6"],
  col2: ["Unterzug 1", "Unterzug 2", "Unterzug 3"],
  col3: ["Stütze"],
} as const;

export const CATALOGUE_COLUMN_LABELS: Record<keyof typeof CATALOGUE_COLUMN_TILE_KEYS, string> = {
  col1: "Rippenplatte",
  col2: "Unterzug",
  col3: "Stütze",
};

export const CATALOGUE_LABEL_INLINE_FRAME = { x: 0.1, y: 0.44, width: 0.8, height: 0.12 };
export const CATALOGUE_LABEL_INLINE_GAP = 0.03;

/** @emoji 📐️ One of three equal inline label slots on the Bauteilbeschriftungen row. */
export function inlineColumnLabelPosition(columnIndex: 0 | 1 | 2): DispositionPosition {
  const gap = CATALOGUE_LABEL_INLINE_GAP;
  const colWidth = (CATALOGUE_LABEL_INLINE_FRAME.width - gap * 2) / 3;
  return {
    x: CATALOGUE_LABEL_INLINE_FRAME.x + columnIndex * (colWidth + gap),
    y: CATALOGUE_LABEL_INLINE_FRAME.y,
    width: colWidth,
    height: CATALOGUE_LABEL_INLINE_FRAME.height,
  };
}

/** @emoji 📐️ Focus-slide dispositions for catalogue tile participants. */
export function catalogueFocusDispositions(): readonly Disposition[] {
  const focusTiles = catalogueFocusColumnTiles();
  const positions = Object.fromEntries(focusTiles.map((tile) => [tile.participantId, tile.position]));
  return remapSplitDispositions(
    CATALOGUE_SPLIT.dispositions.filter((disposition) => focusTiles.some((tile) => tile.participantId === disposition.participantId)),
    positions,
  );
}

/** @emoji 🔀️ One-to-many morphTo slots: catalogue figure into focus tiles at grid positions on the catalogue slide. */
export function catalogueFocusMorphTo(): readonly MorphToSlot[] {
  return catalogueFocusColumnTiles().map((tile) => {
    const splitDisposition = CATALOGUE_SPLIT.dispositions.find((disposition) => disposition.participantId === tile.participantId);
    const position = splitDisposition?.position;
    if (!position) {
      throw new Error(`catalogueFocusMorphTo: no grid position for "${tile.participantId}".`);
    }
    return {
      participantId: tile.participantId,
      embodimentId: `${tile.participantId}-figure`,
      position,
    };
  });
}

/** @emoji 🔀️ Many-to-one morphFrom slots: focus tiles (source figure) into one column label disposition. */
export function columnLabelMorphFrom(column: keyof typeof CATALOGUE_COLUMN_TILE_KEYS, labelPosition: DispositionPosition): Disposition["morphFrom"] {
  return CATALOGUE_COLUMN_TILE_KEYS[column].map((participantId) => ({
    participantId,
    embodimentId: `${participantId}-figure`,
    position: labelPosition,
  }));
}

export const mediaParticipants: Participant[] = [zukunftBauParticipant, { id: CATALOGUE_PARTICIPANT }, { id: CATALOGUE_COL1 }, { id: CATALOGUE_COL2 }, { id: CATALOGUE_COL3 }, { id: "demo-video" }, { id: "thesis" }, ...CATALOGUE_SPLIT.participants];

export const mediaEmbodiments: Embodiment[] = [
  zukunftBauEmbodiment,
  {
    kind: "figure",
    id: CATALOGUE_EMBODIMENT_FULL,
    src: ASSET_CATALOGUE,
    alt: "Komponentenkatalog",
    crop: { x: 0, y: 0, width: 1, height: 1 },
    sourceAspect: CATALOGUE_SOURCE_ASPECT,
  },
  {
    kind: "figure",
    id: CATALOGUE_EMBODIMENT_COL1_CROP,
    src: ASSET_CATALOGUE,
    alt: CATALOGUE_COLUMN_LABELS.col1,
    crop: unionTileCropForParticipants(CATALOGUE_SPLIT, CATALOGUE_COLUMN_TILE_KEYS.col1),
    sourceAspect: CATALOGUE_SOURCE_ASPECT,
  },
  {
    kind: "text",
    id: CATALOGUE_EMBODIMENT_COL1_LABEL,
    lines: [CATALOGUE_COLUMN_LABELS.col1],
    level: "heading",
    morphRoot: "heading-line",
  },
  {
    kind: "figure",
    id: CATALOGUE_EMBODIMENT_COL2_CROP,
    src: ASSET_CATALOGUE,
    alt: CATALOGUE_COLUMN_LABELS.col2,
    crop: unionTileCropForParticipants(CATALOGUE_SPLIT, CATALOGUE_COLUMN_TILE_KEYS.col2),
    sourceAspect: CATALOGUE_SOURCE_ASPECT,
  },
  {
    kind: "text",
    id: CATALOGUE_EMBODIMENT_COL2_LABEL,
    lines: [CATALOGUE_COLUMN_LABELS.col2],
    level: "heading",
    morphRoot: "heading-line",
  },
  {
    kind: "figure",
    id: CATALOGUE_EMBODIMENT_COL3_CROP,
    src: ASSET_CATALOGUE,
    alt: CATALOGUE_COLUMN_LABELS.col3,
    crop: unionTileCropForParticipants(CATALOGUE_SPLIT, CATALOGUE_COLUMN_TILE_KEYS.col3),
    sourceAspect: CATALOGUE_SOURCE_ASPECT,
  },
  {
    kind: "text",
    id: CATALOGUE_EMBODIMENT_COL3_LABEL,
    lines: [CATALOGUE_COLUMN_LABELS.col3],
    level: "heading",
    morphRoot: "heading-line",
  },
  {
    kind: "video",
    id: "demo-video--clip",
    src: ASSET_VIDEO,
    muted: true,
    controls: true,
  },
  {
    kind: "pdf",
    id: "thesis--doc",
    src: ASSET_THESIS_PDF,
    page: 1,
    pages: [1, 12, 25, 35, 42, 43, 51],
    alt: "Bachelorarbeit Ueli Saluz",
  },
  ...CATALOGUE_SPLIT.embodiments,
];
//#endregion 🔖️Catalogue

//#region 🔖️Baukomponenten
export const BAUKOMPONENTEN_FRAME = { x: 0.04, y: 0.06, width: 0.92, height: 0.88 };
export const BAUKOMPONENTEN_GAP = 0.012;

export const BAUKOMPONENTEN_ITEMS = [
  { id: "betondeckenplatten", src: "/🖼️bauteilbörse-betondeckenplatten.png", kind: "figure", alt: "Betondeckenplatten" },
  { id: "gipsplatten", src: "/🖼️bauteilbörse-gipsplatten.png", kind: "figure", alt: "Gipsplatten" },
  { id: "holzbalken-2", src: "/🖼️bauteilbörse-holzbalken-2.png", kind: "figure", alt: "Holzbalken" },
  { id: "holzbalken", src: "/🖼️bauteilbörse-holzbalken.png", kind: "figure", alt: "Holzbalken" },
  { id: "metallprofile", src: "/🖼️bauteilbörse-metallprofile.png", kind: "figure", alt: "Metallprofile" },
  { id: "stahltragwerk", src: "./📄️bauteilbörse-stahltragwerk.pdf", kind: "pdf", alt: "Stahltragwerk" },
  { id: "träger-hea", src: "/🖼️bauteilbörse-träger-hea.png", kind: "figure", alt: "Träger HEA" },
  { id: "träger-ipe", src: "/🖼️bauteilbörse-träger-ipe.png", kind: "figure", alt: "Träger IPE" },
  { id: "trennwand-glas", src: "/🖼️bauteilbörse-trennwand-glas.png", kind: "figure", alt: "Trennwand Glas" },
] as const;

/** @emoji 🧩️ Participants, embodiments, and grid dispositions for the Baukomponenten 3×3 slide. */
export function baukomponentenGridArtifacts(): {
  readonly participants: Participant[];
  readonly embodiments: Embodiment[];
  readonly dispositions: Disposition[];
} {
  const cells = splitFigureGrid({
    rows: 3,
    columns: 3,
    frame: BAUKOMPONENTEN_FRAME,
    gap: BAUKOMPONENTEN_GAP,
  });
  const participants: Participant[] = [];
  const embodiments: Embodiment[] = [];
  const dispositions: Disposition[] = [];
  for (const [index, item] of BAUKOMPONENTEN_ITEMS.entries()) {
    const position = cells[index]?.position;
    if (!position) {
      throw new Error(`baukomponentenGridArtifacts: missing grid cell for index ${index}.`);
    }
    participants.push({ id: item.id });
    if (item.kind === "figure") {
      const embodimentId = `${item.id}--figure`;
      embodiments.push({
        kind: "figure",
        id: embodimentId,
        src: item.src,
        alt: item.alt,
        scrollOrigin: MEDIA_SCROLL_ORIGIN_TOP_LEFT,
      });
      dispositions.push({
        participantId: item.id,
        embodimentId,
        emphasis: "active",
        position,
      });
      continue;
    }
    const embodimentId = `${item.id}--doc`;
    embodiments.push({
      kind: "pdf",
      id: embodimentId,
      src: item.src,
      page: 1,
      alt: item.alt,
      scrollOrigin: MEDIA_SCROLL_ORIGIN_TOP_LEFT,
    });
    dispositions.push({
      participantId: item.id,
      embodimentId,
      emphasis: "active",
      position,
    });
  }
  return { participants, embodiments, dispositions };
}
//#endregion 🔖️Baukomponenten
//#endregion 🔖️spec

//#region 🔖️Deck
const slideModuleLoaders = import.meta.glob<{ default: SlideFile }>("../slide/**/*.ts");
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
  void Promise.all([import("@semio-tech/animate-present-renderer-react"), import("@semio-tech/ui-react")]).then(([{ mountPresentation }, { DEFAULT_UI_DRIVER }]) => {
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
export { presentationPlayAppDefinition as projektetagePlayAppDefinition } from "@semio-tech/animate-present-core";
//#endregion 🔖️Play

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  function bauteilboersenThought(presentation: Presentation): Thought | undefined {
    return presentation.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
      ?.thoughts.find((thought) => thought.name === "Bauteilbörsen");
  }

  describe("projektetage deck", () => {
    it("declares intro plus expanded render slides", () => {
      expect(countArrangements(deck)).toBeGreaterThanOrEqual(15);
      expect(deck.language).toBe("de");
    });

    it("uses German bookmark names on intro and catalogue slides", () => {
      const introSlide = collectPresentationSlides(deck)[0];
      expect(introSlide).toEqual({
        h: 0,
        v: 0,
        chapter: "Einführung",
        sequence: "Einleitung",
        thought: "Einleitung",
        slide: "Zukunft Bau Auftakt",
      });
      const titleSlide = collectPresentationSlides(deck)[1];
      expect(titleSlide).toEqual({
        h: 0,
        v: 1,
        chapter: "Einführung",
        sequence: "Einleitung",
        thought: "Einleitung",
        slide: "Titel",
      });
      const catalogueSlide = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
      expect(catalogueSlide).toMatchObject({
        h: 2,
        v: 1,
        chapter: "Bauteilportal",
        sequence: "Ausgangslage",
        thought: "Bauteilbörsen",
        slide: "Bauteilkatalog",
      });
    });

    it("opens with the Zukunft Bau Entwerfen mit Bestand image and closes with Abschluss", () => {
      const slides = collectPresentationSlides(deck);
      expect(slides.at(0)?.slide).toBe("Zukunft Bau Auftakt");
      expect(slides.at(-1)?.slide).toBe("Abschluss");
      const firstThought = deck.chapters[0]?.sequences[0]?.thoughts[0];
      const lastThought = deck.chapters.at(-1)?.sequences.at(-1)?.thoughts.at(-1);
      expect(firstThought?.slides[0]?.arrangement.dispositions[0]).toMatchObject({
        participantId: ZUKUNFT_BAU_PARTICIPANT,
        embodimentId: ZUKUNFT_BAU_EMBODIMENT,
        position: ZUKUNFT_BAU_FRAME,
      });
      expect(firstThought?.slides[1]?.arrangement.dispositions[0]?.morphFrom).toEqual([
        {
          participantId: ZUKUNFT_BAU_PARTICIPANT,
          embodimentId: ZUKUNFT_BAU_EMBODIMENT,
          position: INTRO_TITLE_MORPH_FRAME,
        },
      ]);
      expect(firstThought?.slides[1]?.arrangement.dispositions[0]?.position).toEqual(INTRO_TITLE_MORPH_FRAME);
      expect(firstThought?.slides[0]?.transition).toEqual({ kind: "morph" });
      expect(
        expandThoughtSlides(firstThought!)
          .slice(0, 2)
          .map((slide) => slide.autoAnimateId),
      ).toEqual(["einleitung--m0", "einleitung--m0"]);
      const abschluss = lastThought?.slides.at(-1);
      expect(abschluss?.arrangement.name).toBe("Abschluss");
      expect(abschluss?.arrangement.dispositions).toHaveLength(2);
      expect(lastThought?.embodiments?.find((entry) => entry.id === "abschluss-heading--text")).toMatchObject({
        kind: "text",
        lines: ["Vielen Dank für Ihre Aufmerksamkeit!"],
        level: "heading",
      });
      expect(lastThought?.embodiments?.find((entry) => entry.id === "abschluss-sponsorship--text")).toMatchObject({
        kind: "text",
        level: "body",
      });
    });

    it("assembles the catalogue with morphTo targets for the focus slide", () => {
      const bauteilboersen = bauteilboersenThought(deck);
      const catalogue = bauteilboersen?.slides.find((slide) => slide.arrangement.id === "catalogue");
      expect(catalogue?.arrangement.dispositions).toHaveLength(1);
      expect(catalogue?.arrangement.dispositions[0]?.morphTo).toHaveLength(10);
      expect(catalogue?.arrangement.settleBeforeMorphTo).toBeUndefined();
    });

    it("names all ten catalogue morph targets semantically", () => {
      const bauteilboersen = bauteilboersenThought(deck);
      const catalogue = bauteilboersen?.slides.find((slide) => slide.arrangement.id === "catalogue");
      expect(catalogue?.arrangement.dispositions[0]?.morphTo?.map((slot) => slot.participantId)).toEqual([
        "Rippenplatte 1",
        "Rippenplatte 2",
        "Rippenplatte 3",
        "Rippenplatte 4",
        "Rippenplatte 5",
        "Rippenplatte 6",
        "Unterzug 1",
        "Unterzug 2",
        "Unterzug 3",
        "Stütze",
      ]);
    });

    it("focuses ten catalogue tile participants for column morph", () => {
      const bauteilboersen = bauteilboersenThought(deck);
      const focus = bauteilboersen?.slides.find((slide) => slide.arrangement.id === "catalogue-focus");
      const dispositions = focus?.arrangement.dispositions ?? [];
      expect(dispositions).toHaveLength(10);
      expect(dispositions.map((disposition) => disposition.participantId)).toEqual(["Rippenplatte 1", "Rippenplatte 2", "Rippenplatte 3", "Rippenplatte 4", "Rippenplatte 5", "Rippenplatte 6", "Unterzug 1", "Unterzug 2", "Unterzug 3", "Stütze"]);
    });

    it("assigns one auto-animate run across catalogue, focus, and labels", () => {
      const bauteilboersen = bauteilboersenThought(deck);
      expect(bauteilboersen).toBeDefined();
      const expanded = expandThoughtSlides(bauteilboersen!);
      const morphIds = expanded.filter((slide) => ["catalogue", "catalogue-focus", "catalogue-labels"].includes(slide.id)).map((slide) => slide.autoAnimateId);
      expect(morphIds).toHaveLength(3);
      expect(new Set(morphIds).size).toBe(1);
      expect(morphIds[0]).toBeTruthy();
    });

    it("expands ten morph sources and three resting labels on Bauteilbeschriftungen", () => {
      const bauteilboersen = bauteilboersenThought(deck);
      expect(bauteilboersen).toBeDefined();
      const expanded = expandThoughtSlides(bauteilboersen!);
      const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
      expect(labelSlide).toBeDefined();
      expect(arrangementRestDispositions(labelSlide!.arrangement)).toHaveLength(3);
      const morphFromSlots = labelSlide!.arrangement.dispositions.flatMap((disposition) => disposition.morphFrom ?? []);
      expect(morphFromSlots).toHaveLength(10);
    });

    it("morphs tile figures into label positions before column text appears", () => {
      const labelPosition = inlineColumnLabelPosition(0);
      const slots = columnLabelMorphFrom("col1", labelPosition);
      expect(slots?.every((slot) => slot.position === labelPosition)).toBe(true);
      expect(slots?.every((slot) => slot.embodimentId.endsWith("-figure"))).toBe(true);
    });

    it("morphs each column participant into inline label dispositions on one row", () => {
      const bauteilboersen = bauteilboersenThought(deck);
      expect(bauteilboersen).toBeDefined();
      const expanded = expandThoughtSlides(bauteilboersen!);
      const focusSlide = expanded.find((slide) => slide.id === "catalogue-focus");
      const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
      const labelDispositions = labelSlide ? arrangementRestDispositions(labelSlide.arrangement) : [];
      expect(labelDispositions).toHaveLength(3);
      expect(labelDispositions.map((disposition) => disposition.participantId)).toEqual([CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3]);
      expect(labelDispositions.every((disposition) => disposition.embodimentId.endsWith("--label"))).toBe(true);
      const yPositions = labelDispositions.map((disposition) => disposition.position?.y);
      expect(new Set(yPositions).size).toBe(1);
      const scope = buildResolutionScope([bauteilboersen!]);
      const resolved = resolveArrangement(scope, labelSlide!.arrangement);
      expect(resolved.filter((entry) => entry.embodiment.kind === "text")).toHaveLength(3);
    });

    it("loads every slide from slide/<chapter>/<sequence>/<thought>/<slide>.ts paths", () => {
      expect(deck.chapters).toHaveLength(4);
      expect(deck.chapters.map((chapter) => chapter.name)).toEqual(["Einführung", "Recherche", "Bauteilportal", "Entwurfswerkzeug"]);
      const einführung = deck.chapters.find((chapter) => chapter.name === "Einführung");
      expect(einführung?.sequences.map((sequence) => sequence.name)).toEqual(["Einleitung"]);
      expect(einführung?.sequences[0]?.thoughts.map((thought) => thought.name)).toEqual(["Einleitung"]);
      expect(einführung?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Zukunft Bau Auftakt", "Titel", "Beschreibung", "Ziel", "Autoren", "Fakultät", "Universitäten", "Lehrstühle"]);
      const recherche = deck.chapters.find((chapter) => chapter.name === "Recherche");
      expect(recherche?.sequences.map((sequence) => sequence.name)).toEqual(["Recherche"]);
      expect(recherche?.sequences[0]?.thoughts.map((thought) => thought.name)).toEqual(["Gedanke Schweiz"]);
      expect(recherche?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Überblick", "Zoom In 1", "Zoom In 2", "Zoom In 3"]);
      const bauteilportal = deck.chapters.find((chapter) => chapter.name === "Bauteilportal");
      expect(bauteilportal?.sequences.map((sequence) => sequence.name)).toEqual(["Ausgangslage", "Systematik"]);
      expect(bauteilportal?.sequences[0]?.thoughts.map((thought) => thought.name)).toEqual(["Bauteilbörsen", "Typologien"]);
      expect(bauteilportal?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Baukomponenten", "Bauteilkatalog", "Bauteilarten", "Bauteilbeschriftungen"]);
      expect(bauteilportal?.sequences[0]?.thoughts[1]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Typologien", "Baum", "Katalog"]);
      expect(bauteilportal?.sequences[1]?.name).toBe("Systematik");
      expect(bauteilportal?.sequences[1]?.thoughts.map((thought) => thought.name)).toEqual(["Eingabeprozess", "Generatoren"]);
      expect(bauteilportal?.sequences[1]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual([
        "Eingabearten",
        "Eingabeoberfläche",
        "Eingabeoberfläche Annotiert",
        "Manuelles Prüfen",
        "Import Besipiel",
        "Import Verarbeitung",
        "Output",
      ]);
      expect(bauteilportal?.sequences[1]?.thoughts[1]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Bauteillogik", "Modelle", "Konnektivität"]);
      const entwurfswerkzeug = deck.chapters.find((chapter) => chapter.name === "Entwurfswerkzeug");
      expect(entwurfswerkzeug?.sequences.map((sequence) => sequence.name)).toEqual(["Benutzeroberfläche"]);
      expect(entwurfswerkzeug?.sequences[0]?.thoughts[0]?.name).toBe("Benutzeroberfläche");
      expect(entwurfswerkzeug?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Katalog", "Filter", "Detail", "Puzzle", "Abschluss"]);
    });

    it("embeds the 3D puzzle iframe full-frame on Puzzle", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Entwurfswerkzeug")
        ?.sequences.find((sequence) => sequence.name === "Benutzeroberfläche")
        ?.thoughts.find((entry) => entry.name === "Benutzeroberfläche");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Puzzle");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]).toMatchObject({
        participantId: "puzzle-3d",
        embodimentId: "puzzle-3d--iframe",
        position: { x: 0, y: 0, width: 1, height: 1 },
      });
      expect(thought?.embodiments?.find((entry) => entry.id === "puzzle-3d--iframe")).toMatchObject({
        kind: "iframe",
        src: "https://v4.3d.puzzle.semio-tech.com/",
      });
    });

    it("lays out each entwurfswerkzeug catalog step as a single centered figure", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Entwurfswerkzeug")
        ?.sequences.find((sequence) => sequence.name === "Benutzeroberfläche")
        ?.thoughts.find((entry) => entry.name === "Benutzeroberfläche");
      for (const item of [
        {
          slide: "Katalog",
          participantId: "entwurfswerkzeug-katalog",
          src: "/🖼️entwurfswerkzeug-🖼️katalog.png",
          sourceAspect: 688 / 1948,
        },
        {
          slide: "Filter",
          participantId: "entwurfswerkzeug-filter",
          src: "/🖼️entwurfswerkzeug-filter.png",
          sourceAspect: 674 / 1948,
        },
        {
          slide: "Detail",
          participantId: "entwurfswerkzeug-detail",
          src: "/🖼️entwurfswerkzeug-detail.png",
          sourceAspect: 674 / 1948,
        },
      ]) {
        const slide = thought?.slides.find((entry) => entry.arrangement.name === item.slide);
        expect(slide?.arrangement.dispositions).toHaveLength(1);
        expect(slide?.arrangement.dispositions[0]).toMatchObject({
          participantId: item.participantId,
          embodimentId: `${item.participantId}--figure`,
        });
        expect(thought?.embodiments?.find((entry) => entry.id === `${item.participantId}--figure`)).toMatchObject({
          kind: "figure",
          src: item.src,
          sourceAspect: item.sourceAspect,
        });
      }
    });

    it("embeds the procedural bauteillogik iframe full-frame on Bauteillogik", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Generatoren");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Bauteillogik");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]).toMatchObject({
        participantId: "procedural-bauteillogik",
        embodimentId: "procedural-bauteillogik--iframe",
        position: { x: 0, y: 0, width: 1, height: 1 },
      });
      expect(thought?.embodiments?.find((entry) => entry.id === "procedural-bauteillogik--iframe")).toMatchObject({
        kind: "iframe",
        src: "https://v4.procedural.semio-tech.com/",
      });
    });

    it("embeds the CAD modelle iframe full-frame on Modelle", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Generatoren");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Modelle");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]).toMatchObject({
        participantId: "cad-modelle",
        embodimentId: "cad-modelle--iframe",
        position: { x: 0, y: 0, width: 1, height: 1 },
      });
      expect(thought?.embodiments?.find((entry) => entry.id === "cad-modelle--iframe")).toMatchObject({
        kind: "iframe",
        src: "https://v4.cad.semio-tech.com/",
      });
    });

    it("lays out Generatoren Konnektivität as 50/50 figure and connector table", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Generatoren");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Konnektivität");
      expect(slide?.arrangement.dispositions).toHaveLength(2);
      const [figure, table] = slide?.arrangement.dispositions ?? [];
      expect(figure?.participantId).toBe("konnektivität-beispiel-3d");
      expect(table?.participantId).toBe("konnektivität-beispiel-tabelle");
      expect(figure?.position?.x ?? 0).toBeLessThan(0.5);
      expect((figure?.position?.x ?? 0) + (figure?.position?.width ?? 0)).toBeLessThanOrEqual(0.5);
      expect(table?.position?.x ?? 0).toBeGreaterThanOrEqual(0.5);
      expect(table?.position?.y).toBe(0.06);
      expect(table?.position?.height).toBeCloseTo(0.88, 10);
      const figureEmbodiment = thought?.embodiments?.find((entry) => entry.id === "konnektivität-beispiel-3d--figure");
      expect(figureEmbodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️konnektivität-beispiel-3d.png",
      });
      const tableEmbodiment = thought?.embodiments?.find((entry) => entry.id === "konnektivität-beispiel-tabelle--markdown");
      expect(tableEmbodiment).toMatchObject({
        kind: "markdown",
        src: "/📄️konnektivität-beispiel-tabelle.md",
      });
    });

    it("shows the eingabearten figure on Eingabearten", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Eingabearten");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-eingabearten");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-eingabearten--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️eingabeprozess-eingabearten.png",
        alt: "Eingabearten im Eingabeprozess",
        sourceAspect: 3586 / 1346,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(3586 / 1346, 10);
    });

    it("shows the eingabeoberfläche figure on Eingabeoberfläche", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Eingabeoberfläche");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-eingabeoberfläche");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-eingabeoberfläche--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️eingabeprozess-eingabeoberfläche.png",
        alt: "Eingabeoberfläche im Eingabeprozess",
        sourceAspect: 2130 / 1670,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(2130 / 1670, 10);
    });

    it("shows the eingabeoberfläche annotiert figure on Eingabeoberfläche Annotiert", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Eingabeoberfläche Annotiert");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-eingabeoberfläche-annotiert");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-eingabeoberfläche-annotiert--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️eingabeprozess-eingabeoberfläche-annotiert.png",
        alt: "Annotierte Eingabeoberfläche im Eingabeprozess",
        sourceAspect: 746 / 659,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(746 / 659, 10);
    });

    it("shows the manuelles prüfen figure on Manuelles Prüfen", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Manuelles Prüfen");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-manuelles-prüfen");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-manuelles-prüfen--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️eingabeprozess-formular.png",
        alt: "Manuelles Prüfen im Eingabeprozess",
        sourceAspect: 860 / 1183,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(860 / 1183, 10);
    });

    it("lays out three bauteilbörse figures in a 1×3 grid on Import Besipiel", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Import Besipiel");
      expect(slide?.arrangement.dispositions).toHaveLength(3);
      expect(slide?.arrangement.dispositions.map((disposition) => disposition.participantId)).toEqual(["import-besipiel-holzbalken", "import-besipiel-rippenplatte", "import-besipiel-träger-heb"]);
      for (const [index, item] of [
        { id: "import-besipiel-holzbalken", src: "/🖼️bauteilbörse-holzbalken.png", alt: "Holzbalken" },
        { id: "import-besipiel-rippenplatte", src: "/🖼️bauteilbörse-rippenplatte.png", alt: "Rippenplatte" },
        { id: "import-besipiel-träger-heb", src: "/🖼️bauteilbörse-träger-heb.png", alt: "Träger HEB" },
      ].entries()) {
        const disposition = slide?.arrangement.dispositions[index];
        expect(disposition?.embodimentId).toBe(`${item.id}--figure`);
        const embodiment = thought?.embodiments?.find((entry) => entry.id === `${item.id}--figure`);
        expect(embodiment).toMatchObject({
          kind: "figure",
          src: item.src,
          alt: item.alt,
          scrollOrigin: { x: 0, y: 0 },
        });
      }
      const [left, middle, right] = slide?.arrangement.dispositions.map((disposition) => disposition.position) ?? [];
      expect(left?.x ?? 0).toBeLessThan(middle?.x ?? 0);
      expect((middle?.x ?? 0) + (middle?.width ?? 0)).toBeLessThanOrEqual((right?.x ?? 0) + 0.001);
    });

    it("shows the import verarbeitung figure on Import Verarbeitung", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Import Verarbeitung");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-import-verarbeitung");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-import-verarbeitung--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️import-verarbeitung.png",
        alt: "Import Verarbeitung im Eingabeprozess",
        sourceAspect: 1278 / 1288,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(1278 / 1288, 10);
    });

    it("shows the eingabeprozess output json on Output", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Systematik")
        ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Output");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-output");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-output--json");
      expect(embodiment).toMatchObject({
        kind: "json",
        src: "/🔣️eingabeprozess-output.json",
        title: "Eingabeprozess Output",
      });
      expect(slide?.arrangement.dispositions[0]?.position).toEqual({
        x: 0.04,
        y: 0.06,
        width: 0.92,
        height: 0.88,
      });
    });

    it("shows the typologien figure on Typologien", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
        ?.thoughts.find((entry) => entry.name === "Typologien");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Typologien");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("typologien");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "typologien--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️typologien.png",
        alt: "Typologien-Katalog",
        sourceAspect: 984 / 1448,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(984 / 1448, 10);
    });

    it("shows the typology tree figure on Baum", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
        ?.thoughts.find((entry) => entry.name === "Typologien");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Baum");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("typologien-baum");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "typologien-baum--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️typologienbaum.png",
        alt: "Generator-Typologiebaum",
        sourceAspect: 1536 / 1024,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(1536 / 1024, 10);
    });

    it("shows the typologien katalog figure on Katalog", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
        ?.thoughts.find((entry) => entry.name === "Typologien");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Katalog");
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]?.participantId).toBe("typologien-katalog");
      const embodiment = thought?.embodiments?.find((entry) => entry.id === "typologien-katalog--figure");
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: "/🖼️katalog.png",
        alt: "Typologien-Katalog",
        sourceAspect: 1264 / 713,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(1264 / 713, 10);
    });

    it("shows the recherche schweiz figures on Gedanke Schweiz", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Recherche")
        ?.sequences.find((sequence) => sequence.name === "Recherche")
        ?.thoughts.find((entry) => entry.name === "Gedanke Schweiz");
      expect(thought?.slides.map((slide) => slide.arrangement.name)).toEqual(["Überblick", "Zoom In 1", "Zoom In 2", "Zoom In 3"]);
      for (const item of [
        {
          slide: "Überblick",
          participantId: "recherche-schweiz-überblick",
          src: "/🖼️recherche-schweiz-überblick.png",
          sourceAspect: 1987 / 1015,
        },
        {
          slide: "Zoom In 1",
          participantId: "recherche-schweiz-zoom-in-1",
          src: "/🖼️recherche-schweiz-zoom-in-1.png",
          sourceAspect: 1984 / 1014,
        },
        {
          slide: "Zoom In 2",
          participantId: "recherche-schweiz-zoom-in-2",
          src: "/🖼️recherche-schweiz-zoom-in-2.png",
          sourceAspect: 1988 / 1018,
        },
        {
          slide: "Zoom In 3",
          participantId: "recherche-schweiz-zoom-in-3",
          src: "/🖼️recherche-schweiz-zoom-in-3.png",
          sourceAspect: 1981 / 1017,
        },
      ]) {
        const slide = thought?.slides.find((entry) => entry.arrangement.name === item.slide);
        expect(slide?.arrangement.dispositions).toHaveLength(1);
        expect(slide?.arrangement.dispositions[0]).toMatchObject({
          participantId: item.participantId,
          embodimentId: `${item.participantId}--figure`,
        });
        expect(thought?.embodiments?.find((entry) => entry.id === `${item.participantId}--figure`)).toMatchObject({
          kind: "figure",
          src: item.src,
          sourceAspect: item.sourceAspect,
        });
        const position = slide?.arrangement.dispositions[0]?.position;
        expect(position).toBeDefined();
        expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(item.sourceAspect, 10);
      }
    });

    it("lays out nine bauteilbörse figures in a 3×3 grid on Baukomponenten", () => {
      const thought = deck.chapters
        .find((chapter) => chapter.name === "Bauteilportal")
        ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
        ?.thoughts.find((entry) => entry.name === "Bauteilbörsen");
      const slide = thought?.slides.find((entry) => entry.arrangement.name === "Baukomponenten");
      expect(slide?.arrangement.dispositions).toHaveLength(9);
      expect(slide?.arrangement.dispositions.map((disposition) => disposition.participantId)).toEqual([
        "betondeckenplatten",
        "gipsplatten",
        "holzbalken-2",
        "holzbalken",
        "metallprofile",
        "stahltragwerk",
        "träger-hea",
        "träger-ipe",
        "trennwand-glas",
      ]);
      const gridEmbodimentIds = BAUKOMPONENTEN_ITEMS.map((item) => (item.kind === "figure" ? `${item.id}--figure` : `${item.id}--doc`));
      const gridEmbodiments = thought?.embodiments?.filter((embodiment) => gridEmbodimentIds.includes(embodiment.id)) ?? [];
      const kinds = gridEmbodiments.map((embodiment) => embodiment.kind);
      expect(kinds.filter((kind) => kind === "figure")).toHaveLength(8);
      expect(kinds).toContain("pdf");
      for (const embodiment of gridEmbodiments) {
        if (embodiment.kind === "figure" || embodiment.kind === "pdf") {
          expect(embodiment.scrollOrigin).toEqual({ x: 0, y: 0 });
        }
      }
    });
  });
}
//#endregion 🧪️Tests
