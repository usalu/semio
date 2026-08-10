/** 🔺️ Present diff schema — sparse field delta. */
export interface PresentDiff {
  /** @state persistent */ artifact?: PresentArtifact | null;
  /** @state persistent */ schema?: string | null;
  /** @state persistent */ source?: FigureTileSource | null;
  /** @state persistent */ tiles?: PresentTilesDelta | null;
  /** @state shared-ui */ selectedIds?: PresentStringList | null;
  /** @state local-ui */ engagementInput?: string | null;
  /** @state local-ui */ locale?: string | null;
}
export interface PresentStringList { values: string[]; }
export interface PresentTilesDelta { added: FigureTileDraft[]; removed: string[]; patched: PresentTilePatchEntry[]; reordered?: string[] | null; }
export interface PresentTilePatchEntry { id: string; patch: FigureTileDraftPatch; }
export interface FigureTileDraftPatch { name?: string | null; crop?: FigureTileFrame | null; }
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
export interface PresentArtifact {
  schema: string;
  source: FigureTileSource;
  tiles: FigureTileDraft[];
  selectedIds: string[];
  engagementInput: string;
  locale: string;
}
