import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { cn, SelectionMarquee, marqueeCoverageFromGesture, screenRectFromPoints, type SelectionMarqueePoint } from "@semio-tech/ui-react";
import { resolveSemanticColorHex } from "@semio-tech/ui-styling";
import type { ComponentSceneHostProps } from "@semio-tech/framework-core";

//#region Types
export type Vec2 = readonly [number, number];

export type NoteCamera = { readonly x: number; readonly y: number; readonly zoom: number };

export interface NoteTextRun {
  readonly text: string;
  readonly bold?: boolean;
  readonly italic?: boolean;
  readonly underline?: boolean;
  readonly link?: string;
}

export interface NoteTextParagraph {
  readonly runs: readonly NoteTextRun[];
}

export interface NoteBlockBase {
  readonly id: string;
  readonly name: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly rotation?: number;
  readonly visible: boolean;
  readonly locked: boolean;
}

export interface NoteTextBlock extends NoteBlockBase {
  readonly kind: "text";
  readonly paragraphs: readonly NoteTextParagraph[];
  readonly fontSize: number;
  readonly fontWeight: "normal" | "bold";
  readonly align: "left" | "center" | "right";
}

export interface NoteImageAsset {
  readonly mime: string;
  readonly data: string;
  readonly width?: number;
  readonly height?: number;
}

export interface NoteImageBlock extends NoteBlockBase {
  readonly kind: "image";
  readonly imageKey: string;
}

export interface NoteTableCell {
  readonly content: string;
}

export interface NoteTableBlock extends NoteBlockBase {
  readonly kind: "table";
  readonly columns: readonly string[];
  readonly rows: readonly (readonly NoteTableCell[])[];
}

export interface NoteMathBlock extends NoteBlockBase {
  readonly kind: "math";
  readonly tex: string;
  readonly displayMode: boolean;
}

export interface NoteInkBlock extends NoteBlockBase {
  readonly kind: "ink";
  readonly points: readonly Vec2[];
  readonly strokeWidth: number;
  readonly color: readonly [number, number, number, number];
}

export interface NoteGroupBlock extends NoteBlockBase {
  readonly kind: "group";
  readonly children: readonly NoteBlockNode[];
}

export type NoteBlockNode = NoteTextBlock | NoteImageBlock | NoteTableBlock | NoteMathBlock | NoteInkBlock | NoteGroupBlock;
export type NoteBlockKind = NoteBlockNode["kind"];

export interface NoteDocument {
  readonly schema: "note.document";
  readonly id: string;
  readonly title?: string;
  readonly camera: NoteCamera;
  readonly blocks: readonly NoteBlockNode[];
  readonly assets?: Readonly<Record<string, NoteImageAsset>>;
  readonly activeTool?: string;
  readonly gridVisible?: boolean;
  readonly gridSpacing?: number;
  readonly gridSubdivisions?: number;
  readonly gridOpacity?: number;
  readonly snapEnabled?: boolean;
  readonly snapGridSpacing?: number;
  readonly pencilWidth?: number;
  readonly eraserRadius?: number;
}

export type NoteResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export interface NoteBounds {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

export type NoteCanvasEvent =
  | { readonly op: "addBlock"; readonly block: NoteBlockNode; readonly parentId?: string | null; readonly index?: number | null }
  | { readonly op: "updateBlock"; readonly blockId: string; readonly block: NoteBlockNode }
  | { readonly op: "removeBlock"; readonly blockId: string }
  | { readonly op: "putAsset"; readonly key: string; readonly asset: NoteImageAsset }
  | { readonly op: "setCamera"; readonly camera: NoteCamera };

type NoteGesturePhase = "begin" | "live" | "commit" | "atomic";

function parseNoteScene(documentJson: string | undefined): NoteDocument | null {
  if (!documentJson) return null;
  try {
    const parsed = JSON.parse(documentJson) as Partial<NoteDocument>;
    if (parsed.schema !== "note.document" || !Array.isArray(parsed.blocks)) return null;
    return parsed as NoteDocument;
  } catch {
    return null;
  }
}

function parseSelectionIds(json: string | undefined): readonly string[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((id): id is string => typeof id === "string") : [];
  } catch {
    return [];
  }
}
//#endregion Types

//#region GeometryHelpers
let noteHostIdCounter = 0;

/** @emoji 🆔 Host-generated ids only need to be unique client-side (Rust re-derives its own on the next round-trip). */
export function createNoteHostId(prefix: string): string {
  noteHostIdCounter += 1;
  return `${prefix}-host-${noteHostIdCounter}`;
}

export function notePositiveMod(value: number, modulus: number): number {
  if (modulus <= 0) return 0;
  return ((value % modulus) + modulus) % modulus;
}

export function noteSnapWorldCoordinate(value: number, spacing: number): number {
  if (spacing <= 0) return value;
  return Math.round(value / spacing) * spacing;
}

export function noteSnapWorldPoint(x: number, y: number, spacing: number): Vec2 {
  return [noteSnapWorldCoordinate(x, spacing), noteSnapWorldCoordinate(y, spacing)];
}

function noteMaybeSnapWorldPoint(doc: NoteDocument, x: number, y: number): Vec2 {
  if (!doc.snapEnabled) return [x, y];
  return noteSnapWorldPoint(x, y, doc.snapGridSpacing ?? 8);
}

export function screenToWorld(camera: NoteCamera, screenX: number, screenY: number): Vec2 {
  return [(screenX - camera.x) / camera.zoom, (screenY - camera.y) / camera.zoom];
}

export function worldToScreen(camera: NoteCamera, worldX: number, worldY: number): { readonly x: number; readonly y: number } {
  return { x: worldX * camera.zoom + camera.x, y: worldY * camera.zoom + camera.y };
}

export function noteBlockBounds(block: NoteBlockNode): NoteBounds {
  if (block.kind === "ink" && block.points.length > 0) {
    let minX = block.points[0]![0];
    let minY = block.points[0]![1];
    let maxX = minX;
    let maxY = minY;
    for (const point of block.points) {
      minX = Math.min(minX, point[0]);
      minY = Math.min(minY, point[1]);
      maxX = Math.max(maxX, point[0]);
      maxY = Math.max(maxY, point[1]);
    }
    return { x: block.x + minX, y: block.y + minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
  }
  return { x: block.x, y: block.y, width: block.width, height: block.height };
}

export function flattenNoteBlocks(blocks: readonly NoteBlockNode[]): NoteBlockNode[] {
  const out: NoteBlockNode[] = [];
  for (const block of blocks) {
    out.push(block);
    if (block.kind === "group") out.push(...flattenNoteBlocks(block.children));
  }
  return out;
}

export function findNoteBlock(doc: NoteDocument, blockId: string): NoteBlockNode | null {
  function visit(node: NoteBlockNode): NoteBlockNode | null {
    if (node.id === blockId) return node;
    if (node.kind === "group") {
      for (const child of node.children) {
        const found = visit(child);
        if (found) return found;
      }
    }
    return null;
  }
  for (const block of doc.blocks) {
    const found = visit(block);
    if (found) return found;
  }
  return null;
}

export function noteSelectionBounds(blocks: readonly NoteBlockNode[], ids: readonly string[]): NoteBounds | null {
  const idSet = new Set(ids);
  const selected = flattenNoteBlocks(blocks).filter((block) => idSet.has(block.id));
  if (!selected.length) return null;
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const block of selected) {
    const bounds = noteBlockBounds(block);
    minX = Math.min(minX, bounds.x);
    minY = Math.min(minY, bounds.y);
    maxX = Math.max(maxX, bounds.x + bounds.width);
    maxY = Math.max(maxY, bounds.y + bounds.height);
  }
  return { x: minX, y: minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
}

function scaleValue(value: number, fromMin: number, fromSize: number, toMin: number, toSize: number): number {
  if (fromSize <= 0) return toMin;
  return toMin + ((value - fromMin) / fromSize) * toSize;
}

export function noteScaleBlockWithinGroup(block: NoteBlockNode, fromBounds: NoteBounds, toBounds: NoteBounds): NoteBlockNode {
  const bounds = noteBlockBounds(block);
  const nextX = scaleValue(bounds.x, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width);
  const nextY = scaleValue(bounds.y, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height);
  const nextWidth = Math.max(8, scaleValue(bounds.x + bounds.width, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width) - nextX);
  const nextHeight = Math.max(8, scaleValue(bounds.y + bounds.height, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height) - nextY);
  if (block.kind === "ink") {
    const scaleX = fromBounds.width > 0 ? toBounds.width / fromBounds.width : 1;
    const scaleY = fromBounds.height > 0 ? toBounds.height / fromBounds.height : 1;
    const points = block.points.map(([px, py]) => [px * scaleX, py * scaleY] as Vec2);
    return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight, points };
  }
  if (block.kind === "group") {
    return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight, children: block.children.map((child) => noteScaleBlockWithinGroup(child, fromBounds, toBounds)) };
  }
  return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight };
}

export function noteResizeBounds(fromBounds: NoteBounds, handle: NoteResizeHandle, dx: number, dy: number, minSize = 8): NoteBounds {
  let { x, y, width, height } = fromBounds;
  if (handle.includes("e")) width = Math.max(minSize, width + dx);
  if (handle.includes("w")) {
    const nextWidth = Math.max(minSize, width - dx);
    x += width - nextWidth;
    width = nextWidth;
  }
  if (handle.includes("s")) height = Math.max(minSize, height + dy);
  if (handle.includes("n")) {
    const nextHeight = Math.max(minSize, height - dy);
    y += height - nextHeight;
    height = nextHeight;
  }
  return { x, y, width, height };
}

export function noteBlocksAtPoint(blocks: readonly NoteBlockNode[], x: number, y: number): NoteBlockNode[] {
  const hits: NoteBlockNode[] = [];
  for (const block of [...flattenNoteBlocks(blocks)].reverse()) {
    const bounds = noteBlockBounds(block);
    if (x >= bounds.x && x <= bounds.x + bounds.width && y >= bounds.y && y <= bounds.y + bounds.height) hits.push(block);
  }
  return hits;
}

export function noteBlocksIntersectingRect(blocks: readonly NoteBlockNode[], rect: NoteBounds): string[] {
  const hits: string[] = [];
  for (const block of flattenNoteBlocks(blocks)) {
    const bounds = noteBlockBounds(block);
    const intersects = bounds.x < rect.x + rect.width && bounds.x + bounds.width > rect.x && bounds.y < rect.y + rect.height && bounds.y + bounds.height > rect.y;
    if (intersects) hits.push(block.id);
  }
  return hits;
}

export function noteTableCellAtPoint(block: NoteTableBlock, localX: number, localY: number): { readonly row: number; readonly col: number } | null {
  const rowCount = block.rows.length + 1;
  const colCount = block.columns.length;
  if (rowCount <= 0 || colCount <= 0) return null;
  const rowHeight = block.height / rowCount;
  const colWidth = block.width / colCount;
  const row = Math.floor(localY / rowHeight) - 1;
  const col = Math.floor(localX / colWidth);
  if (row < 0 || row >= block.rows.length || col < 0 || col >= colCount) return null;
  return { row, col };
}

function pointToSegmentDistance(px: number, py: number, x1: number, y1: number, x2: number, y2: number): number {
  const dx = x2 - x1;
  const dy = y2 - y1;
  if (dx === 0 && dy === 0) return Math.hypot(px - x1, py - y1);
  const t = Math.max(0, Math.min(1, ((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)));
  return Math.hypot(px - (x1 + t * dx), py - (y1 + t * dy));
}

function inkWorldPoints(block: NoteInkBlock): Vec2[] {
  return block.points.map(([px, py]) => [block.x + px, block.y + py] as Vec2);
}

function inkHitsPoint(block: NoteInkBlock, x: number, y: number, threshold: number): boolean {
  const points = inkWorldPoints(block);
  if (points.length < 2) {
    if (!points[0]) return false;
    return Math.hypot(x - points[0][0], y - points[0][1]) <= threshold;
  }
  for (let index = 1; index < points.length; index += 1) {
    const prev = points[index - 1]!;
    const next = points[index]!;
    if (pointToSegmentDistance(x, y, prev[0], prev[1], next[0], next[1]) <= threshold + block.strokeWidth / 2) return true;
  }
  return false;
}

/** @emoji 🧹 Whole-stroke eraser: returns removeBlock events for every ink stroke under the point. */
export function noteEraseInkStrokeEventsAtPoint(doc: NoteDocument, x: number, y: number, threshold = 8): readonly NoteCanvasEvent[] {
  const hits = flattenNoteBlocks(doc.blocks).filter((block): block is NoteInkBlock => block.kind === "ink" && inkHitsPoint(block, x, y, threshold));
  return hits.map((block) => ({ op: "removeBlock", blockId: block.id }));
}

/** @emoji ✂️ Splits an ink stroke into surviving point-runs after removing points within `radius` of (x, y). */
export function noteEraseInkPointsInBlock(block: NoteInkBlock, x: number, y: number, radius: number): NoteInkBlock[] {
  const keptIndices: number[] = [];
  for (let index = 0; index < block.points.length; index += 1) {
    const point = block.points[index]!;
    if (Math.hypot(block.x + point[0] - x, block.y + point[1] - y) > radius) keptIndices.push(index);
  }
  if (keptIndices.length === block.points.length) return [block];
  if (!keptIndices.length) return [];
  const runs: Vec2[][] = [];
  let current: Vec2[] = [block.points[keptIndices[0]!]!];
  for (let index = 1; index < keptIndices.length; index += 1) {
    if (keptIndices[index]! - keptIndices[index - 1]! > 1) {
      if (current.length >= 2) runs.push(current);
      current = [block.points[keptIndices[index]!]!];
    } else {
      current.push(block.points[keptIndices[index]!]!);
    }
  }
  if (current.length >= 2) runs.push(current);
  return runs.map((points, index) => ({ ...block, id: index === 0 ? block.id : createNoteHostId("ink"), name: index === 0 ? block.name : `${block.name} fragment`, points }));
}

/** @emoji ✂️ Point-eraser events: removeBlock for the original stroke, addBlock for each surviving fragment (skipped if untouched). */
export function noteEraseInkPointEventsNearPoint(doc: NoteDocument, x: number, y: number, radius: number): readonly NoteCanvasEvent[] {
  const events: NoteCanvasEvent[] = [];
  const inkBlocks = flattenNoteBlocks(doc.blocks).filter((block): block is NoteInkBlock => block.kind === "ink");
  for (const block of inkBlocks) {
    const fragments = noteEraseInkPointsInBlock(block, x, y, radius);
    if (fragments.length === 1 && fragments[0] === block) continue;
    events.push({ op: "removeBlock", blockId: block.id });
    for (const fragment of fragments) events.push({ op: "addBlock", block: fragment });
  }
  return events;
}

export function noteTextParagraphsFromPlainText(text: string): readonly NoteTextParagraph[] {
  return text.split(/\n/).map((line) => ({ runs: [{ text: line }] }));
}

export function noteTextPlainText(paragraphs: readonly NoteTextParagraph[]): string {
  return paragraphs.map((paragraph) => paragraph.runs.map((run) => run.text).join("")).join("\n");
}

export function noteParagraphsToHtml(paragraphs: readonly NoteTextParagraph[]): string {
  return paragraphs
    .map((paragraph) => {
      const inner = paragraph.runs
        .map((run) => {
          let text = run.text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
          if (run.link) text = `<a href="${run.link}">${text}</a>`;
          if (run.underline) text = `<u>${text}</u>`;
          if (run.italic) text = `<em>${text}</em>`;
          if (run.bold) text = `<strong>${text}</strong>`;
          return text;
        })
        .join("");
      return `<div>${inner || "<br>"}</div>`;
    })
    .join("");
}

export function noteHtmlToParagraphs(root: HTMLElement): readonly NoteTextParagraph[] {
  const paragraphs: NoteTextParagraph[] = [];
  const children = root.childNodes.length ? [...root.childNodes] : [root];
  for (const child of children) {
    if (child.nodeType === Node.TEXT_NODE) {
      const text = child.textContent ?? "";
      if (text) paragraphs.push({ runs: [{ text }] });
      continue;
    }
    if (!(child instanceof HTMLElement)) continue;
    const tag = child.tagName.toLowerCase();
    if (tag === "br") {
      paragraphs.push({ runs: [{ text: "" }] });
      continue;
    }
    const runs: NoteTextRun[] = [];
    const walk = (node: Node, marks: Partial<NoteTextRun>) => {
      if (node.nodeType === Node.TEXT_NODE) {
        const text = node.textContent ?? "";
        if (text) runs.push({ text, ...marks });
        return;
      }
      if (!(node instanceof HTMLElement)) return;
      const nextMarks = { ...marks };
      const nodeTag = node.tagName.toLowerCase();
      if (nodeTag === "strong" || nodeTag === "b") nextMarks.bold = true;
      if (nodeTag === "em" || nodeTag === "i") nextMarks.italic = true;
      if (nodeTag === "u") nextMarks.underline = true;
      if (nodeTag === "a") nextMarks.link = node.getAttribute("href") ?? undefined;
      for (const childNode of node.childNodes) walk(childNode, nextMarks);
    };
    for (const childNode of child.childNodes) walk(childNode, {});
    if (!runs.length) runs.push({ text: "" });
    paragraphs.push({ runs });
  }
  return paragraphs.length ? paragraphs : [{ runs: [{ text: "" }] }];
}

export function noteImageAssetDataUrl(asset: NoteImageAsset): string {
  if (asset.data.startsWith("data:")) return asset.data;
  if (asset.mime === "image/svg+xml") return `data:image/svg+xml;utf8,${encodeURIComponent(asset.data)}`;
  return `data:${asset.mime};base64,${asset.data}`;
}

export interface NoteClipboardPayload {
  readonly schema: "note.clipboard";
  readonly blocks: readonly NoteBlockNode[];
}

export function noteClipboardPayload(blocks: readonly NoteBlockNode[]): string {
  const payload: NoteClipboardPayload = { schema: "note.clipboard", blocks: [...blocks] };
  return JSON.stringify(payload);
}

export function noteBlocksFromClipboardPayload(json: string): readonly NoteBlockNode[] | null {
  try {
    const parsed = JSON.parse(json) as NoteClipboardPayload;
    if (parsed.schema !== "note.clipboard" || !Array.isArray(parsed.blocks)) return null;
    return parsed.blocks;
  } catch {
    return null;
  }
}

function reidBlockTree(block: NoteBlockNode, renameTop: boolean): NoteBlockNode {
  const id = createNoteHostId(block.kind);
  const name = renameTop ? `${block.name} copy` : block.name;
  if (block.kind === "group") return { ...block, id, name, children: block.children.map((child) => reidBlockTree(child, false)) };
  return { ...block, id, name };
}

export function noteCloneBlocksWithOffset(blocks: readonly NoteBlockNode[], dx: number, dy: number): NoteBlockNode[] {
  return blocks.map((block) => {
    const clone = reidBlockTree(block, false);
    return { ...clone, x: clone.x + dx, y: clone.y + dy };
  });
}

const NOTE_BLOCK_DEFAULT_SIZE: Record<NoteBlockKind, { readonly width: number; readonly height: number }> = {
  text: { width: 280, height: 120 },
  image: { width: 240, height: 160 },
  table: { width: 320, height: 160 },
  math: { width: 200, height: 80 },
  ink: { width: 1, height: 1 },
  group: { width: 280, height: 120 },
};

export function createNoteBlockByKind(kind: NoteBlockKind, x: number, y: number): NoteBlockNode {
  const size = NOTE_BLOCK_DEFAULT_SIZE[kind];
  const base = { id: createNoteHostId(kind), x, y, width: size.width, height: size.height, rotation: 0, visible: true, locked: false };
  if (kind === "image") return { ...base, kind, name: "Image", imageKey: "placeholder" };
  if (kind === "table")
    return {
      ...base,
      kind,
      name: "Table",
      columns: ["A", "B", "C"],
      rows: [
        [{ content: "" }, { content: "" }, { content: "" }],
        [{ content: "" }, { content: "" }, { content: "" }],
      ],
    };
  if (kind === "math") return { ...base, kind, name: "Math", tex: "E = mc^2", displayMode: true };
  if (kind === "ink") return { ...base, kind, name: "Ink", points: [], strokeWidth: 3, color: [0, 0, 0, 1] };
  if (kind === "group") return { ...base, kind, name: "Group", children: [] };
  return { ...base, kind: "text", name: "Text", paragraphs: [{ runs: [{ text: "" }] }], fontSize: 18, fontWeight: "normal", align: "left" };
}

/** @emoji 🖊️ Local pure application of the applyNoteEvents op vocabulary — mirrors note/plugin/rs/lib.rs `apply_note_canvas_event` for optimistic in-gesture rendering. */
export function applyNoteCanvasEventLocal(doc: NoteDocument, event: NoteCanvasEvent): NoteDocument {
  switch (event.op) {
    case "addBlock": {
      const blocks = [...doc.blocks];
      if (!event.parentId) {
        blocks.splice(event.index ?? blocks.length, 0, event.block);
        return { ...doc, blocks };
      }
      return { ...doc, blocks: insertIntoParent(doc.blocks, event.parentId, event.index ?? Number.MAX_SAFE_INTEGER, event.block) };
    }
    case "updateBlock":
      return { ...doc, blocks: updateInTree(doc.blocks, event.blockId, event.block) };
    case "removeBlock":
      return { ...doc, blocks: removeFromTree(doc.blocks, event.blockId) };
    case "putAsset":
      return { ...doc, assets: { ...(doc.assets ?? {}), [event.key]: event.asset } };
    case "setCamera":
      return { ...doc, camera: event.camera };
    default:
      return doc;
  }
}

function insertIntoParent(blocks: readonly NoteBlockNode[], parentId: string, index: number, block: NoteBlockNode): NoteBlockNode[] {
  return blocks.map((node) => {
    if (node.kind !== "group") return node;
    if (node.id === parentId) {
      const children = [...node.children];
      children.splice(Math.min(index, children.length), 0, block);
      return { ...node, children };
    }
    return { ...node, children: insertIntoParent(node.children, parentId, index, block) };
  });
}

function updateInTree(blocks: readonly NoteBlockNode[], blockId: string, nextBlock: NoteBlockNode): NoteBlockNode[] {
  return blocks.map((block) => {
    if (block.id === blockId) return nextBlock;
    if (block.kind === "group") return { ...block, children: updateInTree(block.children, blockId, nextBlock) };
    return block;
  });
}

function removeFromTree(blocks: readonly NoteBlockNode[], blockId: string): NoteBlockNode[] {
  return blocks.filter((block) => block.id !== blockId).map((block) => (block.kind === "group" ? { ...block, children: removeFromTree(block.children, blockId) } : block));
}

function applyEventsLocal(doc: NoteDocument, events: readonly NoteCanvasEvent[]): NoteDocument {
  return events.reduce((acc, event) => applyNoteCanvasEventLocal(acc, event), doc);
}
//#endregion GeometryHelpers

//#region MathRenderer
export interface NoteMathRenderer {
  render(tex: string, displayMode: boolean): string;
}

let noteMathRenderer: NoteMathRenderer = {
  render(tex: string, displayMode: boolean) {
    return `<span class="note-math-fallback">${displayMode ? `$$${tex}$$` : `$${tex}$`}</span>`;
  },
};

/** @emoji ∑ Sets the active note math renderer adapter (defaults to a plain-text fallback until KaTeX loads). */
export function setNoteMathRenderer(renderer: NoteMathRenderer): void {
  noteMathRenderer = renderer;
}

async function ensureKatexMathRenderer(): Promise<void> {
  try {
    const katex = await import("katex");
    await import("katex/dist/katex.min.css");
    setNoteMathRenderer({
      render(tex: string, displayMode: boolean) {
        return katex.default.renderToString(tex, { displayMode, throwOnError: false });
      },
    });
  } catch {
    /* fallback renderer stays active */
  }
}

if (typeof window !== "undefined") void ensureKatexMathRenderer();
//#endregion MathRenderer

//#region BlockViews
function noteTextRunStyle(run: NoteTextRun): React.CSSProperties {
  return { fontWeight: run.bold ? "bold" : undefined, fontStyle: run.italic ? "italic" : undefined, textDecoration: run.underline ? "underline" : undefined };
}

function NoteTextRunView({ run }: { readonly run: NoteTextRun }) {
  if (run.link) {
    return (
      <a href={run.link} className="text-primary underline" style={noteTextRunStyle(run)} onPointerDown={(event) => event.stopPropagation()}>
        {run.text}
      </a>
    );
  }
  return <span style={noteTextRunStyle(run)}>{run.text}</span>;
}

function NoteTextContentView({ block }: { readonly block: NoteTextBlock }) {
  return (
    <div className="text-foreground h-full w-full overflow-auto p-2 whitespace-pre-wrap" style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}>
      {block.paragraphs.map((paragraph, paragraphIndex) => (
        <div key={paragraphIndex}>
          {paragraph.runs.map((run, runIndex) => (
            <NoteTextRunView key={runIndex} run={run} />
          ))}
        </div>
      ))}
    </div>
  );
}

function NoteBlockView({
  block,
  assets,
  selected,
  hovered,
  hidden,
  onPointerDown,
}: {
  readonly block: NoteBlockNode;
  readonly assets?: Readonly<Record<string, NoteImageAsset>>;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly hidden: boolean;
  readonly onPointerDown: (event: React.PointerEvent, blockId: string) => void;
}) {
  if (!block.visible) return null;
  const bounds = noteBlockBounds(block);
  const common = {
    className: cn("bg-background/90 absolute overflow-hidden rounded border shadow-sm", selected && "ring-primary ring-2", hovered && !selected && "ring-primary/60 ring-1", block.locked && "opacity-70", hidden && "pointer-events-none opacity-0"),
    style: {
      left: bounds.x,
      top: bounds.y,
      width: Math.max(8, bounds.width),
      height: Math.max(8, bounds.height),
      transform: block.rotation ? `rotate(${block.rotation}deg)` : undefined,
    },
    onPointerDown: (event: React.PointerEvent) => onPointerDown(event, block.id),
  };
  if (block.kind === "text")
    return (
      <div {...common}>
        <NoteTextContentView block={block} />
      </div>
    );
  if (block.kind === "math") {
    const html = noteMathRenderer.render(block.tex, block.displayMode);
    return (
      <div {...common}>
        <div className="flex h-full w-full items-center justify-center p-2">
          <div className="note-math" dangerouslySetInnerHTML={{ __html: html }} />
        </div>
      </div>
    );
  }
  if (block.kind === "table") {
    return (
      <div {...common}>
        <table className="h-full w-full border-collapse text-sm">
          <thead>
            <tr>
              {block.columns.map((column) => (
                <th key={column} className="border-border border px-2 py-1 text-left font-medium">
                  {column}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {block.rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex} className="border-border border px-2 py-1 align-top">
                    {cell.content}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }
  if (block.kind === "image") {
    const asset = assets?.[block.imageKey];
    const src = asset ? noteImageAssetDataUrl(asset) : null;
    return (
      <div {...common}>
        {src ? <img src={src} alt={block.name} className="h-full w-full object-contain" draggable={false} /> : <div className="bg-muted text-muted-foreground flex h-full w-full items-center justify-center text-xs">{block.imageKey}</div>}
      </div>
    );
  }
  if (block.kind === "ink") {
    if (block.points.length < 2) return null;
    const path = block.points.map((point, index) => `${index === 0 ? "M" : "L"} ${block.x + point[0]} ${block.y + point[1]}`).join(" ");
    const [r, g, b, a] = block.color;
    return (
      <svg className="pointer-events-none absolute inset-0 overflow-visible" style={{ width: "100%", height: "100%" }}>
        <path d={path} fill="none" stroke={`rgba(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)},${a})`} strokeWidth={block.strokeWidth} strokeLinecap="round" strokeLinejoin="round" />
      </svg>
    );
  }
  if (block.kind === "group") {
    return (
      <div {...common}>
        <div className="text-muted-foreground p-1 text-xs">Group · {block.children.length} children</div>
      </div>
    );
  }
  return null;
}

const NOTE_RESIZE_HANDLES: readonly NoteResizeHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const NOTE_RESIZE_CURSOR: Record<NoteResizeHandle, string> = { nw: "nwse-resize", n: "ns-resize", ne: "nesw-resize", e: "ew-resize", se: "nwse-resize", s: "ns-resize", sw: "nesw-resize", w: "ew-resize" };

function NoteSelectionChrome({ camera, bounds, onResizePointerDown }: { readonly camera: NoteCamera; readonly bounds: NoteBounds; readonly onResizePointerDown: (handle: NoteResizeHandle, event: React.PointerEvent) => void }) {
  const topLeft = worldToScreen(camera, bounds.x, bounds.y);
  const width = bounds.width * camera.zoom;
  const height = bounds.height * camera.zoom;
  return (
    <>
      <div className="border-primary pointer-events-none absolute z-20 border" style={{ left: topLeft.x, top: topLeft.y, width, height }} />
      {NOTE_RESIZE_HANDLES.map((handle) => {
        const left = handle.includes("w") ? topLeft.x - 4 : handle.includes("e") ? topLeft.x + width - 4 : topLeft.x + width / 2 - 4;
        const top = handle.includes("n") ? topLeft.y - 4 : handle.includes("s") ? topLeft.y + height - 4 : topLeft.y + height / 2 - 4;
        return <div key={handle} className="border-primary bg-background absolute z-30 h-2 w-2 rounded-sm border" style={{ left, top, cursor: NOTE_RESIZE_CURSOR[handle] }} onPointerDown={(event) => onResizePointerDown(handle, event)} />;
      })}
    </>
  );
}

function NoteViewportGrid({ camera, spacing, subdivisions, opacity, color }: { readonly camera: NoteCamera; readonly spacing: number; readonly subdivisions: number; readonly opacity: number; readonly color: string }) {
  const majorPx = spacing * camera.zoom;
  const minorPx = majorPx / Math.max(1, subdivisions);
  const offsetX = notePositiveMod(camera.x, majorPx);
  const offsetY = notePositiveMod(camera.y, majorPx);
  const patternId = `note-viewport-grid-${spacing}-${subdivisions}`;
  const minorLines: React.ReactNode[] = [];
  for (let index = 1; index < subdivisions; index += 1) {
    const position = index * minorPx;
    minorLines.push(
      <line key={`v-${index}`} x1={position} y1={0} x2={position} y2={majorPx} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
      <line key={`h-${index}`} x1={0} y1={position} x2={majorPx} y2={position} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
    );
  }
  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden>
      <defs>
        <pattern id={patternId} width={majorPx} height={majorPx} patternUnits="userSpaceOnUse" x={offsetX} y={offsetY}>
          {minorLines}
          <path d={`M ${majorPx} 0 L 0 0 0 ${majorPx}`} fill="none" stroke={color} strokeWidth={1} opacity={opacity} />
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill={`url(#${patternId})`} />
    </svg>
  );
}
//#endregion BlockViews

//#region Overlays
function NoteTextEditorOverlay({ block, screenBounds, onCommit, onCancel }: { readonly block: NoteTextBlock; readonly screenBounds: NoteBounds; readonly onCommit: (paragraphs: readonly NoteTextParagraph[]) => void; readonly onCancel: () => void }) {
  const editorRef = useRef<HTMLDivElement | null>(null);
  const applyCommand = (command: string, value?: string) => {
    editorRef.current?.focus();
    document.execCommand(command, false, value);
  };
  useEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;
    editor.focus();
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(editor);
    selection?.removeAllRanges();
    selection?.addRange(range);
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onCancel();
      }
    };
    editor.addEventListener("keydown", onKeyDown);
    return () => editor.removeEventListener("keydown", onKeyDown);
  }, [onCancel]);
  return (
    <div className="absolute z-30" style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}>
      <div className="bg-background/95 mb-1 flex gap-1 rounded border p-1 shadow-sm">
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("bold");
          }}
        >
          B
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs italic"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("italic");
          }}
        >
          I
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs underline"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("underline");
          }}
        >
          U
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            const url = window.prompt("Link URL");
            if (url) applyCommand("createLink", url);
          }}
        >
          Link
        </button>
      </div>
      <div
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        className="text-foreground bg-background h-[calc(100%-2rem)] w-full overflow-auto rounded border p-2 outline-none"
        style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}
        dangerouslySetInnerHTML={{ __html: noteParagraphsToHtml(block.paragraphs) }}
        onBlur={() => {
          if (!editorRef.current) return;
          onCommit(noteHtmlToParagraphs(editorRef.current));
        }}
      />
    </div>
  );
}

function NoteTableCellEditorOverlay({
  block,
  row,
  col,
  screenBounds,
  onCommit,
  onCancel,
}: {
  readonly block: NoteTableBlock;
  readonly row: number;
  readonly col: number;
  readonly screenBounds: NoteBounds;
  readonly onCommit: (content: string, advance?: boolean) => void;
  readonly onCancel: () => void;
}) {
  const inputRef = useRef<HTMLInputElement | null>(null);
  useEffect(() => {
    inputRef.current?.focus();
    inputRef.current?.select();
  }, []);
  return (
    <input
      ref={inputRef}
      className="bg-background ring-primary absolute z-30 rounded border px-2 py-1 text-sm ring-2 outline-none"
      style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}
      defaultValue={block.rows[row]?.[col]?.content ?? ""}
      onKeyDown={(event) => {
        if (event.key === "Escape") {
          event.preventDefault();
          onCancel();
        }
        if (event.key === "Enter" || event.key === "Tab") {
          event.preventDefault();
          onCommit(event.currentTarget.value, true);
        }
      }}
      onBlur={(event) => onCommit(event.currentTarget.value)}
    />
  );
}
//#endregion Overlays

//#region DragState
type NoteDragState =
  | { readonly kind: "pan"; readonly startX: number; readonly startY: number; readonly camera: NoteCamera }
  | { readonly kind: "move"; readonly origins: Readonly<Record<string, { readonly x: number; readonly y: number }>>; readonly startX: number; readonly startY: number }
  | { readonly kind: "marquee"; readonly start: SelectionMarqueePoint }
  | { readonly kind: "ink"; readonly blockId: string }
  | { readonly kind: "eraser"; readonly mode: "eraserStroke" | "eraserPoint" }
  | { readonly kind: "resize"; readonly handle: NoteResizeHandle; readonly fromBounds: NoteBounds; readonly startX: number; readonly startY: number; readonly selectedIds: readonly string[] };

type NoteTextEditState = { readonly blockId: string; readonly created?: boolean };
type NoteTableEditState = { readonly blockId: string; readonly row: number; readonly col: number };

const NOTE_MARQUEE_THRESHOLD_PX = 4;
//#endregion DragState

//#region NoteCanvasHost
export function NoteCanvasHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.noteCanvas;
  const rootRef = useRef<HTMLDivElement | null>(null);
  const gestureActiveRef = useRef(false);
  const rafRef = useRef<number | null>(null);
  const pendingLiveEventsRef = useRef<readonly NoteCanvasEvent[] | null>(null);
  const [draftDoc, setDraftDoc] = useState<NoteDocument | null>(null);
  const [dragState, setDragState] = useState<NoteDragState | null>(null);
  const [marqueePoints, setMarqueePoints] = useState<readonly SelectionMarqueePoint[]>([]);
  const [textEdit, setTextEdit] = useState<NoteTextEditState | null>(null);
  const [tableEdit, setTableEdit] = useState<NoteTableEditState | null>(null);

  const sceneDoc = useMemo(() => parseNoteScene(scene?.documentJson), [scene?.documentJson]);
  const doc = draftDoc ?? sceneDoc;
  const selectedIds = useMemo(() => parseSelectionIds(scene?.selectionJson), [scene?.selectionJson]);
  const selectedSet = useMemo(() => new Set(selectedIds), [selectedIds]);
  const hoveredId = scene?.hoveredId ?? null;
  const isNavigator = scene?.viewMode === "navigator";
  const interactive = scene?.interactive ?? false;

  useEffect(() => {
    if (!gestureActiveRef.current) setDraftDoc(null);
  }, [scene?.documentJson]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (!node.controllerId) return;
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const flushPendingLive = useCallback(() => {
    if (rafRef.current != null) {
      cancelAnimationFrame(rafRef.current);
      rafRef.current = null;
    }
    pendingLiveEventsRef.current = null;
  }, []);

  const beginGesture = useCallback(
    (events: readonly NoteCanvasEvent[], selectIds?: readonly string[]) => {
      gestureActiveRef.current = true;
      setDraftDoc((current) => applyEventsLocal(current ?? sceneDoc ?? { schema: "note.document", id: "empty", camera: { x: 0, y: 0, zoom: 1 }, blocks: [] }, events));
      dispatch("applyNoteEvents", { eventsJson: JSON.stringify(events), phase: "begin", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch, sceneDoc],
  );

  const liveGesture = useCallback(
    (events: readonly NoteCanvasEvent[]) => {
      setDraftDoc((current) => (current ? applyEventsLocal(current, events) : current));
      pendingLiveEventsRef.current = events;
      if (rafRef.current == null) {
        rafRef.current = requestAnimationFrame(() => {
          rafRef.current = null;
          const pending = pendingLiveEventsRef.current;
          pendingLiveEventsRef.current = null;
          if (pending) dispatch("applyNoteEvents", { eventsJson: JSON.stringify(pending), phase: "live" });
        });
      }
    },
    [dispatch],
  );

  const commitGesture = useCallback(
    (events: readonly NoteCanvasEvent[], selectIds?: readonly string[]) => {
      flushPendingLive();
      gestureActiveRef.current = false;
      dispatch("applyNoteEvents", { eventsJson: JSON.stringify(events), phase: "commit", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch, flushPendingLive],
  );

  const atomicGesture = useCallback(
    (events: readonly NoteCanvasEvent[], selectIds?: readonly string[]) => {
      dispatch("applyNoteEvents", { eventsJson: JSON.stringify(events), phase: "atomic", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch],
  );

  const selectionBounds = useMemo(() => (doc ? noteSelectionBounds(doc.blocks, selectedIds) : null), [doc, selectedIds]);
  const tool = doc?.activeTool ?? "selectDirect";
  const showResizeHandles = !isNavigator && (tool === "selectDirect" || tool === "selectMarquee") && Boolean(selectionBounds) && selectedIds.length > 0;

  const beginMove = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      if (!rootRef.current || !doc) return;
      const block = findNoteBlock(doc, blockId);
      if (!block || block.locked) return;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const moveIds = selectedSet.has(blockId) ? selectedIds : [blockId];
      const origins: Record<string, { x: number; y: number }> = {};
      for (const id of moveIds) {
        const entry = findNoteBlock(doc, id);
        if (entry) origins[id] = { x: entry.x, y: entry.y };
      }
      setDragState({ kind: "move", origins, startX: screenX, startY: screenY });
    },
    [doc, selectedIds, selectedSet],
  );

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      rootRef.current.focus();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (tool === "pan" || event.button === 1 || (tool === "selectDirect" && event.altKey)) {
        setDragState({ kind: "pan", startX: screenX, startY: screenY, camera });
        return;
      }
      if (tool === "eraserStroke" || tool === "eraserPoint") {
        setDragState({ kind: "eraser", mode: tool });
        const events = tool === "eraserStroke" ? noteEraseInkStrokeEventsAtPoint(doc, worldX, worldY) : noteEraseInkPointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) beginGesture(events);
        return;
      }
      if (tool === "selectMarquee") {
        setDragState({ kind: "marquee", start: { x: screenX, y: screenY } });
        setMarqueePoints([{ x: screenX, y: screenY }]);
        return;
      }
      if (tool === "pencil") {
        const block = createNoteBlockByKind("ink", worldX, worldY);
        beginGesture([{ op: "addBlock", block }], [block.id]);
        setDragState({ kind: "ink", blockId: block.id });
        return;
      }
      if (tool === "text" || tool === "image" || tool === "table" || tool === "math") {
        const [placeX, placeY] = noteMaybeSnapWorldPoint(doc, worldX, worldY);
        const block = createNoteBlockByKind(tool, placeX, placeY);
        atomicGesture([{ op: "addBlock", block }], [block.id]);
        if (tool === "text") setTextEdit({ blockId: block.id, created: true });
        return;
      }
      const hits = noteBlocksAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (!top || top.locked) {
        if (tool === "selectDirect") dispatch("setSelection", { ids: [] });
        return;
      }
      if (tool === "selectDirect") {
        const nextSelection = event.shiftKey ? [...new Set([...selectedIds, top.id])] : [top.id];
        dispatch("setSelection", { ids: nextSelection });
        beginMove(event, top.id);
      }
    },
    [atomicGesture, beginGesture, beginMove, dispatch, doc, interactive, isNavigator, selectedIds, tool],
  );

  const handleBlockPointerDown = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      event.stopPropagation();
      if (!rootRef.current || !doc || !interactive) return;
      const block = findNoteBlock(doc, blockId);
      if (!block || block.locked) return;
      const nextSelection = event.shiftKey ? [...new Set([...selectedIds, blockId])] : [blockId];
      dispatch("setSelection", { ids: nextSelection });
      if (tool === "selectDirect" || tool === "selectMarquee") beginMove(event, blockId);
    },
    [beginMove, dispatch, doc, interactive, selectedIds, tool],
  );

  const handleResizePointerDown = useCallback(
    (handle: NoteResizeHandle, event: React.PointerEvent) => {
      event.stopPropagation();
      if (!rootRef.current || !selectionBounds) return;
      const rect = rootRef.current.getBoundingClientRect();
      setDragState({ kind: "resize", handle, fromBounds: selectionBounds, startX: event.clientX - rect.left, startY: event.clientY - rect.top, selectedIds: [...selectedIds] });
    },
    [selectedIds, selectionBounds],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (!dragState) {
        if (!interactive) return;
        const hits = noteBlocksAtPoint(doc.blocks, worldX, worldY);
        const top = hits[0] ?? null;
        dispatch("setHover", { id: top?.id ?? null });
        return;
      }
      if (dragState.kind === "pan") {
        const nextCamera = { ...dragState.camera, x: dragState.camera.x + (screenX - dragState.startX), y: dragState.camera.y + (screenY - dragState.startY) };
        setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
        dispatch("setCamera", { camera: nextCamera });
        return;
      }
      if (dragState.kind === "move") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const events: NoteCanvasEvent[] = [];
        for (const [blockId, origin] of Object.entries(dragState.origins)) {
          const block = findNoteBlock(doc, blockId);
          if (!block) continue;
          events.push({ op: "updateBlock", blockId, block: { ...block, x: origin.x + dx, y: origin.y + dy } });
        }
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "marquee") {
        setMarqueePoints([dragState.start, { x: screenX, y: screenY }]);
        return;
      }
      if (dragState.kind === "ink") {
        const block = findNoteBlock(doc, dragState.blockId);
        if (!block || block.kind !== "ink") return;
        const localX = worldX - block.x;
        const localY = worldY - block.y;
        liveGesture([{ op: "updateBlock", blockId: block.id, block: { ...block, points: [...block.points, [localX, localY]] } }]);
        return;
      }
      if (dragState.kind === "eraser") {
        const events = dragState.mode === "eraserStroke" ? noteEraseInkStrokeEventsAtPoint(doc, worldX, worldY) : noteEraseInkPointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "resize") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const toBounds = noteResizeBounds(dragState.fromBounds, dragState.handle, dx, dy);
        const events: NoteCanvasEvent[] = [];
        for (const blockId of dragState.selectedIds) {
          const block = findNoteBlock(doc, blockId);
          if (!block) continue;
          events.push({ op: "updateBlock", blockId, block: noteScaleBlockWithinGroup(block, dragState.fromBounds, toBounds) });
        }
        if (events.length) liveGesture(events);
      }
    },
    [dispatch, doc, dragState, interactive, liveGesture],
  );

  const handlePointerUp = useCallback(() => {
    if (!doc) {
      setDragState(null);
      setMarqueePoints([]);
      return;
    }
    if (dragState?.kind === "move") {
      const events: NoteCanvasEvent[] = [];
      for (const blockId of Object.keys(dragState.origins)) {
        const block = findNoteBlock(doc, blockId);
        if (!block) continue;
        if (doc.snapEnabled) {
          const spacing = doc.snapGridSpacing ?? 8;
          const [x, y] = noteSnapWorldPoint(block.x, block.y, spacing);
          events.push({ op: "updateBlock", blockId, block: { ...block, x, y } });
        } else {
          events.push({ op: "updateBlock", blockId, block });
        }
      }
      commitGesture(events);
    } else if (dragState?.kind === "ink") {
      const block = findNoteBlock(doc, dragState.blockId);
      if (block) commitGesture([{ op: "updateBlock", blockId: block.id, block }]);
      else commitGesture([]);
    } else if (dragState?.kind === "resize") {
      const events: NoteCanvasEvent[] = [];
      for (const blockId of dragState.selectedIds) {
        const block = findNoteBlock(doc, blockId);
        if (block) events.push({ op: "updateBlock", blockId, block });
      }
      commitGesture(events);
    } else if (dragState?.kind === "eraser") {
      commitGesture([]);
    } else if (dragState?.kind === "pan") {
      flushPendingLive();
      gestureActiveRef.current = false;
    }
    if (dragState?.kind === "marquee" && marqueePoints.length >= 2 && rootRef.current) {
      const screenRect = screenRectFromPoints(marqueePoints);
      if (screenRect) {
        const camera = doc.camera;
        const worldRect = { x: (screenRect.x - camera.x) / camera.zoom, y: (screenRect.y - camera.y) / camera.zoom, width: screenRect.width / camera.zoom, height: screenRect.height / camera.zoom };
        dispatch("setSelection", { ids: noteBlocksIntersectingRect(doc.blocks, worldRect) });
      }
    }
    setDragState(null);
    setMarqueePoints([]);
  }, [commitGesture, dispatch, doc, dragState, flushPendingLive, marqueePoints]);

  const handleWheel = useCallback(
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator) return;
      event.preventDefault();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const zoomFactor = event.deltaY < 0 ? 1.08 : 0.92;
      const nextZoom = Math.min(8, Math.max(0.1, camera.zoom * zoomFactor));
      const worldX = (screenX - camera.x) / camera.zoom;
      const worldY = (screenY - camera.y) / camera.zoom;
      const nextCamera = { x: screenX - worldX * nextZoom, y: screenY - worldY * nextZoom, zoom: nextZoom };
      setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
      dispatch("setCamera", { camera: nextCamera });
    },
    [dispatch, doc, isNavigator],
  );

  const handleDoubleClick = useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      const hits = noteBlocksAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (top?.kind === "text" && !top.locked) {
        setTableEdit(null);
        setTextEdit({ blockId: top.id });
        dispatch("setSelection", { ids: [top.id] });
        return;
      }
      if (top?.kind === "table" && !top.locked) {
        const cell = noteTableCellAtPoint(top, worldX - top.x, worldY - top.y);
        if (!cell) return;
        setTextEdit(null);
        setTableEdit({ blockId: top.id, row: cell.row, col: cell.col });
        dispatch("setSelection", { ids: [top.id] });
        return;
      }
      if (top) return;
      const [placeX, placeY] = noteMaybeSnapWorldPoint(doc, worldX, worldY);
      const block = createNoteBlockByKind("text", placeX, placeY);
      atomicGesture([{ op: "addBlock", block }], [block.id]);
      setTextEdit({ blockId: block.id, created: true });
    },
    [atomicGesture, dispatch, doc, interactive, isNavigator],
  );

  const commitTextEdit = useCallback(
    (blockId: string, paragraphs: readonly NoteTextParagraph[], created?: boolean) => {
      if (!doc) {
        setTextEdit(null);
        return;
      }
      const block = findNoteBlock(doc, blockId);
      if (!block || block.kind !== "text") {
        setTextEdit(null);
        return;
      }
      const plain = noteTextPlainText(paragraphs).trim();
      if (!plain && created) {
        atomicGesture([{ op: "removeBlock", blockId }]);
        dispatch("setSelection", { ids: [] });
      } else {
        atomicGesture([{ op: "updateBlock", blockId, block: { ...block, paragraphs } }]);
      }
      setTextEdit(null);
    },
    [atomicGesture, dispatch, doc],
  );

  const commitTableEdit = useCallback(
    (blockId: string, row: number, col: number, content: string, advance?: boolean) => {
      if (!doc) {
        setTableEdit(null);
        return;
      }
      const block = findNoteBlock(doc, blockId);
      if (!block || block.kind !== "table") {
        setTableEdit(null);
        return;
      }
      const rows = block.rows.map((entry, rowIndex) => (rowIndex === row ? entry.map((cell, colIndex) => (colIndex === col ? { content } : cell)) : entry));
      atomicGesture([{ op: "updateBlock", blockId, block: { ...block, rows } }]);
      if (advance) {
        const nextCol = col + 1 < block.columns.length ? col + 1 : 0;
        const nextRow = col + 1 < block.columns.length ? row : row + 1;
        if (nextRow < block.rows.length) setTableEdit({ blockId, row: nextRow, col: nextCol });
        else setTableEdit(null);
        return;
      }
      setTableEdit(null);
    },
    [atomicGesture, doc],
  );

  const pasteImageAsset = useCallback(
    (dataUrl: string, mime: string, worldX: number, worldY: number) => {
      const assetKey = `asset-${createNoteHostId("image")}`;
      const imageBlock = createNoteBlockByKind("image", worldX - 120, worldY - 80);
      if (imageBlock.kind !== "image") return;
      atomicGesture(
        [
          { op: "putAsset", key: assetKey, asset: { mime, data: dataUrl } },
          { op: "addBlock", block: { ...imageBlock, imageKey: assetKey } },
        ],
        [imageBlock.id],
      );
    },
    [atomicGesture],
  );

  const handleCopy = useCallback(
    (event: React.ClipboardEvent<HTMLDivElement>) => {
      if (!doc) return;
      if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
      if (!selectedIds.length) return;
      const blocks = selectedIds.map((id) => findNoteBlock(doc, id)).filter((block): block is NoteBlockNode => Boolean(block));
      if (!blocks.length) return;
      event.preventDefault();
      event.clipboardData.setData("text/plain", noteClipboardPayload(blocks));
    },
    [doc, selectedIds, textEdit],
  );

  const handlePaste = useCallback(
    (event: React.ClipboardEvent<HTMLDivElement>) => {
      if (!doc || !rootRef.current) return;
      if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
      event.preventDefault();
      const rect = rootRef.current.getBoundingClientRect();
      const [worldX, worldY] = noteMaybeSnapWorldPoint(doc, ...screenToWorld(doc.camera, rect.width / 2, rect.height / 2));
      for (const item of event.clipboardData.items) {
        if (item.type.startsWith("image/")) {
          const file = item.getAsFile();
          if (!file) continue;
          const reader = new FileReader();
          reader.onload = () => {
            if (typeof reader.result === "string") pasteImageAsset(reader.result, file.type, worldX, worldY);
          };
          reader.readAsDataURL(file);
          return;
        }
      }
      const text = event.clipboardData.getData("text/plain");
      const clipboardBlocks = noteBlocksFromClipboardPayload(text);
      if (clipboardBlocks) {
        const clones = noteCloneBlocksWithOffset(clipboardBlocks, worldX, worldY);
        atomicGesture(
          clones.map((block) => ({ op: "addBlock", block }) as const),
          clones.map((block) => block.id),
        );
        return;
      }
      if (text.trim().startsWith("<svg")) {
        const assetKey = `asset-${createNoteHostId("image")}`;
        const imageBlock = createNoteBlockByKind("image", worldX - 120, worldY - 80);
        if (imageBlock.kind !== "image") return;
        atomicGesture(
          [
            { op: "putAsset", key: assetKey, asset: { mime: "image/svg+xml", data: text.trim() } },
            { op: "addBlock", block: { ...imageBlock, imageKey: assetKey } },
          ],
          [imageBlock.id],
        );
        return;
      }
      if (text.trim()) {
        const block = createNoteBlockByKind("text", worldX, worldY);
        const seeded: NoteTextBlock = { ...(block as NoteTextBlock), paragraphs: noteTextParagraphsFromPlainText(text.trim()) };
        atomicGesture([{ op: "addBlock", block: seeded }], [seeded.id]);
      }
    },
    [atomicGesture, doc, pasteImageAsset, textEdit],
  );

  if (!scene || !doc) return <div className="text-muted-foreground p-2 text-xs">No note scene</div>;

  const camera = doc.camera;
  const visibleBlocks = flattenNoteBlocks(doc.blocks);
  const gridColor = resolveSemanticColorHex("border");
  const gridSpacing = doc.gridSpacing ?? 32;
  const gridSubdivisions = doc.gridSubdivisions ?? 4;
  const gridOpacity = doc.gridOpacity ?? 0.35;
  const scale = isNavigator ? Math.min(0.2, 1 / Math.max(camera.zoom, 1)) : camera.zoom;
  const editingTextBlock = textEdit ? (findNoteBlock(doc, textEdit.blockId) as NoteTextBlock | null) : null;
  const editingTableBlock = tableEdit ? (findNoteBlock(doc, tableEdit.blockId) as NoteTableBlock | null) : null;

  return (
    <div
      ref={rootRef}
      tabIndex={0}
      data-surface-id={node.surfaceId}
      className={cn("bg-muted/20 relative h-full w-full touch-none overflow-hidden outline-none")}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerLeave={handlePointerUp}
      onWheel={handleWheel}
      onDoubleClick={handleDoubleClick}
      onCopy={handleCopy}
      onPaste={handlePaste}
    >
      {doc.gridVisible !== false && !isNavigator ? <NoteViewportGrid camera={camera} spacing={gridSpacing} subdivisions={gridSubdivisions} opacity={gridOpacity} color={gridColor} /> : null}
      <div className="absolute origin-top-left" style={{ transform: `translate(${camera.x}px, ${camera.y}px) scale(${scale})`, width: isNavigator ? 4000 : undefined, height: isNavigator ? 3000 : undefined }}>
        {visibleBlocks.map((block) => (
          <NoteBlockView key={block.id} block={block} assets={doc.assets} selected={selectedIds.includes(block.id)} hovered={hoveredId === block.id} hidden={textEdit?.blockId === block.id} onPointerDown={handleBlockPointerDown} />
        ))}
      </div>
      {showResizeHandles && selectionBounds ? <NoteSelectionChrome camera={camera} bounds={selectionBounds} onResizePointerDown={handleResizePointerDown} /> : null}
      {editingTextBlock && textEdit?.blockId === editingTextBlock.id ? (
        <NoteTextEditorOverlay
          block={editingTextBlock}
          screenBounds={{
            x: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).x,
            y: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).y,
            width: editingTextBlock.width * camera.zoom,
            height: editingTextBlock.height * camera.zoom,
          }}
          onCommit={(paragraphs) => commitTextEdit(editingTextBlock.id, paragraphs, textEdit.created)}
          onCancel={() => {
            if (textEdit.created) atomicGesture([{ op: "removeBlock", blockId: editingTextBlock.id }]);
            setTextEdit(null);
          }}
        />
      ) : null}
      {editingTableBlock && tableEdit
        ? (() => {
            const rowHeight = editingTableBlock.height / (editingTableBlock.rows.length + 1);
            const colWidth = editingTableBlock.width / editingTableBlock.columns.length;
            const cellX = editingTableBlock.x + tableEdit.col * colWidth;
            const cellY = editingTableBlock.y + (tableEdit.row + 1) * rowHeight;
            const screen = worldToScreen(camera, cellX, cellY);
            return (
              <NoteTableCellEditorOverlay
                block={editingTableBlock}
                row={tableEdit.row}
                col={tableEdit.col}
                screenBounds={{ x: screen.x, y: screen.y, width: colWidth * camera.zoom, height: rowHeight * camera.zoom }}
                onCommit={(content, advance) => commitTableEdit(editingTableBlock.id, tableEdit.row, tableEdit.col, content, advance)}
                onCancel={() => setTableEdit(null)}
              />
            );
          })()
        : null}
      {marqueePoints.length >= 2 ? (
        <SelectionMarquee
          shape="rect"
          coverage={marqueeCoverageFromGesture({ method: "rectangle", startX: marqueePoints[0]!.x, endX: marqueePoints[marqueePoints.length - 1]!.x, path: marqueePoints })}
          rect={screenRectFromPoints(marqueePoints) ?? { x: 0, y: 0, width: 0, height: 0 }}
        />
      ) : null}
    </div>
  );
}
//#endregion NoteCanvasHost
