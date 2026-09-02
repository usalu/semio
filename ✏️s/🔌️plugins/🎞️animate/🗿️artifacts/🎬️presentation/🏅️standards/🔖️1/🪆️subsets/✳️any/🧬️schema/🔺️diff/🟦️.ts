/** 🔺️ Presentation diff schema — sparse field delta. */
export interface PresentationDiff {
  /** @state artifact */ artifact?: PresentationArtifact | null;
  /** @state artifact */ schema?: string | null;
  /** @state artifact */ source?: FigureTileSource | null;
  /** @state artifact */ tiles?: PresentationTilesDelta | null;
  /** @state presence */ selectedIds?: PresentationStringList | null;
  /** @state config */ engagementInput?: string | null;
  /** @state config */ locale?: string | null;
}
export interface PresentationStringList { values: string[]; }
export interface PresentationTilesDelta { added: FigureTileDraft[]; removed: string[]; patched: PresentationTilePatchEntry[]; reordered?: string[] | null; }
export interface PresentationTilePatchEntry { id: string; patch: FigureTileDraftPatch; }
export interface FigureTileDraftPatch { name?: string | null; crop?: FigureTileFrame | null; }
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
export interface PresentationArtifact {
  schema: string;
  source: FigureTileSource;
  tiles: FigureTileDraft[];
  selectedIds: string[];
  engagementInput: string;
  locale: string;
}
