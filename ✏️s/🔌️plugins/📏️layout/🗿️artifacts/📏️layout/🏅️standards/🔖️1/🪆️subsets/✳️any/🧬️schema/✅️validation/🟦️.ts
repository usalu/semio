import {VectorSceneCursor} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🔌️ports/🧵️lower/🟦️.ts";
import {parseLayoutDrawingMember} from '../🟦️.ts';
import { binary64Value, binary32Value, type Binary64, type Binary32 } from '@semio-tech/framework/value/ieee754';
import type { LayoutArtifact, Frame, Layer, LayoutBounds, LayoutRect, Page, ParentPage, TextStory, TextStyleRun } from '../🟦️.ts';

class GraphRefusal extends Error {}
const check = (valid: boolean, message: string): void => { if (!valid) throw new GraphRefusal(message); };
const admission = (action: () => void): string | null => { try { action(); return null; } catch (error) { if (error instanceof GraphRefusal) return error.message; throw error; } };
const nonempty = (text: string): boolean => !/^\p{White_Space}*$/u.test(text);
const unique = (ids: Iterable<string>): void => { const seen = new Set<string>(); for (const id of ids) { check(nonempty(id) && !seen.has(id), 'Ids must be non-empty and unique in their collection.'); seen.add(id); } };
const finite = (value: Binary64): boolean => Number.isFinite(binary64Value(value));
const positive = (value: Binary64): boolean => finite(value) && binary64Value(value) > 0;
const nonnegative = (value: Binary64): boolean => finite(value) && binary64Value(value) >= 0;
const boundsValid = (bounds: LayoutBounds, zero: boolean): boolean => [bounds.x, bounds.y, bounds.rotation].every(finite) && (zero ? [bounds.w, bounds.h].every(nonnegative) : [bounds.w, bounds.h].every(positive));
const rectValid = (rect: LayoutRect): boolean => [rect.x, rect.y].every(finite) && [rect.w, rect.h].every(nonnegative);
const colorValid = (color: readonly Binary32[] | null): boolean => color === null || color.every(channel => Number.isFinite(binary32Value(channel)));
const equalIds = (a: readonly string[], b: readonly string[]): boolean => a.length === b.length && a.every((id, index) => id === b[index]);
const localFrames = (document: LayoutArtifact): Frame[] => document.pages.flatMap(page => page.frames);

function content(document: LayoutArtifact, layers: readonly Layer[], frames: readonly Frame[]): void {
  unique(layers.map(layer => layer.id)); unique(frames.map(frame => frame.id));
  const layerIds = new Set(layers.map(layer => layer.id)), stories = new Set(document.stories.map(story => story.id)), links = new Set(document.links.map(link => link.id));
  for (const frame of frames) {
    check(layerIds.has(frame.layerId), 'Every frame must belong to an existing owning layer.');
    check(boundsValid(frame.bounds, false), 'Frame bounds must be finite and have a positive size.');
    if (frame.kind === 'rect') check(colorValid(frame.fill) && colorValid(frame.stroke), 'Frame colors must be finite.');
    if (frame.kind === 'text') {
      check(stories.has(frame.storyId), 'Text frames must name an existing story.');
      check(frame.columns > 0 && rectValid(frame.inset), 'Text columns must be positive and inset sizes finite and non-negative.');
    }
    if (frame.kind === 'image') check(links.has(frame.linkId), 'Image frames must name an existing link.');
  }
}

function threads(source: readonly Frame[]): void {
  const frames = new Map(source.map(frame => [frame.id, frame])), complete = new Set<string>();
  const predecessors=new Set<string>();
  for (const frame of source) if (frame.kind === 'text' && frame.threadNext !== null) {
    const target=frames.get(frame.threadNext);check(target?.kind==='text','Thread targets must be existing text frames in the same content graph.');
    check(target?.kind==='text'&&target.storyId===frame.storyId,'Thread edges must connect frames of the same story.');
    check(!predecessors.has(frame.threadNext),'Text frames must have at most one predecessor.');predecessors.add(frame.threadNext);
  }
  for (const frame of source) {
    const active = new Set<string>(); let current: string | null = frame.id;
    while (current !== null && !complete.has(current)) {
      check(!active.has(current), 'Text threads must not contain cycles.'); active.add(current);
      const held: Frame | undefined = frames.get(current);
      current = held?.kind === 'text' ? held.threadNext : null;
    }
    for (const id of active) complete.add(id);
  }
}

function membership(layerIds: readonly string[], layers: readonly Layer[], frames: readonly Frame[]): void {
  check(equalIds(layerIds, layers.map(layer => layer.id)), 'Layer membership must match the owning layer order.');
  const grouped = new Map<string, string[]>();
  for (const frame of frames) { const ids = grouped.get(frame.layerId) ?? []; ids.push(frame.id); grouped.set(frame.layerId, ids); }
  for (const layer of layers) check(equalIds(layer.objectIds, grouped.get(layer.id) ?? []), 'Layer object membership must match its frames in paint order.');
}

function runs(document: LayoutArtifact, text: string, source: readonly TextStyleRun[]): void {
  const paragraphIds = new Set(document.paragraphStyles.map(style => style.id)), characterIds = new Set(document.characterStyles.map(style => style.id)), boundaries = new Set<bigint>([0n]);
  let bytes = 0n;
  for (const scalar of text) {
    const point = scalar.codePointAt(0)!;
    check(point < 0xd800 || point > 0xdfff, 'Story content must contain valid Unicode scalar values.');
    bytes += point <= 0x7f ? 1n : point <= 0x7ff ? 2n : point <= 0xffff ? 3n : 4n; boundaries.add(bytes);
  }
  for (const run of source) {
    check(run.start <= run.end && boundaries.has(run.start) && boundaries.has(run.end), 'Style runs must stay inside the story on UTF-8 character boundaries.');
    check((run.paragraphStyleId === null || paragraphIds.has(run.paragraphStyleId)) && (run.characterStyleId === null || characterIds.has(run.characterStyleId)), 'Style runs must name existing styles.');
  }
}

function pageContent(document: LayoutArtifact, page: Page): void {
  if(page.backgroundDrawing!==undefined)check(validatePageDrawingChild(page.backgroundDrawing)===null,'Drawing children must have a valid identity, drawing dialect, and admitted owner content.');
  check(positive(page.width) && positive(page.height), 'Pages need positive finite dimensions.');
  check(Object.values(page.margins).every(finite) && page.columns.count > 0 && nonnegative(page.columns.gutter) && page.guides.every(rectValid), 'Page margins must be finite, columns positive, and gutter and guide sizes finite and non-negative.');
  content(document, page.layers, page.frames); membership(page.layerIds, page.layers, page.frames);
  const parent = page.parentPageId === null ? null : document.parentPages.find(parent => parent.id === page.parentPageId);
  check(page.parentPageId === null || parent !== undefined, 'Page parents must exist.');
  unique(page.overrides.map(entry => entry.objectId));
  const inheritedIds = new Set(parent?.frames.map(frame => frame.id) ?? []);
  for (const entry of page.overrides) check(inheritedIds.has(entry.objectId) && (entry.bounds === null || boundsValid(entry.bounds, true)), 'Overrides must name inherited frames and have finite non-negative bounds.');
  check(page.frames.every(frame => !inheritedIds.has(frame.id)), 'Inherited and local frame ids must not collide on a page.');
}

/** 🆔️ Admits non-empty authored ids using the native Unicode whitespace definition. */
export function validateId(id: string): string | null { return admission(() => check(nonempty(id), 'Ids must be non-empty.')); }
/** ✒️ Admits referenced styles and exact UTF-8 byte ranges without changing story content. */
export function validateStyleRuns(document: LayoutArtifact, text: string, source: readonly TextStyleRun[]): string | null { return admission(() => runs(document, text, source)); }
/** 📖️ Admits one story against the existing style graph. */
export function validateStory(document: LayoutArtifact, story: TextStory): string | null { return validateId(story.id) ?? validateStyleRuns(document, story.content, story.styleRuns); }
/** 🔗️ Admits a local outgoing thread without revisiting unrelated content. */
export function validateThreadEdge(document: LayoutArtifact, source: string, story: string, next: string | null): string | null {
  return admission(() => {
    check(document.stories.some(held=>held.id===story),'Text frames must name an existing story.');
    const frames = new Map(localFrames(document).map(frame => [frame.id, frame])), active = new Set<string>(); let cursor = next,predecessors=0;
    for(const frame of frames.values())if(frame.kind==='text'&&frame.threadNext!==null){
      if(frame.id!==source&&frame.threadNext===source){check(frame.storyId===story,'Thread edges must connect frames of the same story.');predecessors++;check(predecessors<=1,'Text frames must have at most one predecessor.');}
      check(frame.id===source||next===null||frame.threadNext!==next,'Text frames must have at most one predecessor.');
    }
    while (cursor !== null) {
      check(cursor !== source && !active.has(cursor), 'Text threads must not contain cycles.'); active.add(cursor);
      const frame = frames.get(cursor); check(frame?.kind === 'text', 'Thread targets must be existing text frames in the same content graph.');
      check(frame?.kind==='text'&&frame.storyId===story,'Thread edges must connect frames of the same story.');
      cursor = frame?.kind === 'text' ? frame.threadNext : null;
    }
  });
}
/** ➖️ Refuses deleting targets referenced by surviving local text frames. */
export function validateFrameRemoval(document: LayoutArtifact, removed: Iterable<string>): string | null {
  const ids = new Set(removed);
  return admission(() => check(localFrames(document).every(frame => frame.kind !== 'text' || frame.threadNext === null || ids.has(frame.id) || !ids.has(frame.threadNext)), 'A surviving text frame still points to a removed frame.'));
}
/** ➕️ Admits a new local frame, including parent collisions and its outgoing thread. */
export function validateFrameCreation(document: LayoutArtifact, page: Page, frame: Frame): string | null {
  return admission(() => {
    check(nonempty(frame.id), 'Ids must be non-empty.');
    check(localFrames(document).every(held => held.id !== frame.id), 'Local frame ids must be globally unique.');
    check(!document.parentPages.find(parent => parent.id === page.parentPageId)?.frames.some(held => held.id === frame.id), 'Inherited and local frame ids must not collide on a page.');
    content(document, page.layers, [frame]);
    if (frame.kind === 'text') { const error = validateThreadEdge(document, frame.id, frame.storyId, frame.threadNext); if (error !== null) throw new GraphRefusal(error); }
  });
}
/** 🌱️ Admits new page content and its candidate local thread graph. */
export function validatePageCreation(document: LayoutArtifact, page: Page): string | null {
  return admission(() => {
    check(nonempty(page.id), 'Ids must be non-empty.'); check(document.spreads.some(spread => spread.id === page.spreadId), 'The owning spread must exist.');
    pageContent(document, page);if(page.backgroundDrawing!==undefined)check(pageDrawingIdentity(document,page.id,page.backgroundDrawing),'Page Drawing children must have independent local and target identities.'); const frames = [...localFrames(document), ...page.frames]; unique(frames.map(frame => frame.id)); threads(frames);
  });
}
/** 🧩️ Admits a parent and derives new memberships without modifying the authored input. */
export function normalizeParentContent(document: LayoutArtifact, parent: ParentPage): { parent: ParentPage; error: string | null } {
  const error = admission(() => {
    check(nonempty(parent.id) && nonempty(parent.name) && positive(parent.width) && positive(parent.height), 'Parent pages need an id, a name, and positive finite dimensions.');
    content(document, parent.layers, parent.frames); threads(parent.frames);
    for (const page of document.pages) if (page.parentPageId === parent.id) { const ids = new Set(page.frames.map(frame => frame.id)); check(parent.frames.every(frame => !ids.has(frame.id)), 'Inherited and local frame ids must not collide on a page.'); }
  });
  if (error !== null) return { parent, error };
  const grouped = new Map<string, string[]>(); for (const frame of parent.frames) { const ids = grouped.get(frame.layerId) ?? []; ids.push(frame.id); grouped.set(frame.layerId, ids); }
  return { parent: { ...parent, layerIds: parent.layers.map(layer => layer.id), layers: parent.layers.map(layer => ({ ...layer, objectIds: grouped.get(layer.id) ?? [] })) }, error: null };
}
/** 🌐️ Admits the complete canonical graph before a whole-document semantic decision. */
export function validateDocument(document: LayoutArtifact): string | null {
  const ownership=admission(()=>{unique(Array.from(drawingChildren(document),child=>child.handle.childId));unique(Array.from(drawingChildren(document),child=>child.handle.target.artifactId));});if(ownership)return ownership;
  if(document.backgroundDrawing!==undefined){const error=validateDrawingChild(document.backgroundDrawing);if(error)return error;}
  return admission(() => {
    check(document.schema === 'layout.layout', 'The document schema is unsupported.');
    check(positive(document.grid.baselineGrid) && finite(document.grid.baselineOffset), 'The baseline grid must be positive and finite and its offset finite.');
    for (const rows of [document.paragraphStyles, document.characterStyles, document.stories, document.links, document.parentPages, document.spreads, document.pages]) unique(rows.map(row => row.id));
    unique(localFrames(document).map(frame => frame.id));
    for (const style of document.paragraphStyles) check(nonempty(style.name) && nonempty(style.fontFamily) && positive(style.fontSize) && positive(style.leading) && finite(style.tracking) && style.fontWeight > 0 && ['left', 'center', 'right', 'justify'].includes(style.alignment), 'Paragraph typography must have valid names, sizes, weight, tracking, and alignment.');
    for (const style of document.characterStyles) check((style.fontSize === null || positive(style.fontSize)) && (style.fontWeight === null || style.fontWeight > 0) && (style.tracking === null || finite(style.tracking)) && colorValid(style.color), 'Character typography must have positive sizes and weight and finite tracking and colors.');
    for (const story of document.stories) runs(document, story.content, story.styleRuns);
    for (const parent of document.parentPages) {
      check(nonempty(parent.name) && positive(parent.width) && positive(parent.height), 'Parent pages need a name and positive finite dimensions.');
      content(document, parent.layers, parent.frames); threads(parent.frames); membership(parent.layerIds, parent.layers, parent.frames);
    }
    const pages = new Map(document.pages.map(page => [page.id, page])), spreads = new Map(document.spreads.map(spread => [spread.id, spread]));
    for (const spread of document.spreads) { check(nonempty(spread.name), 'Spreads need a name.'); unique(spread.pageIds); check(spread.pageIds.every(id => pages.get(id)?.spreadId === spread.id), 'Spread members must exist and belong to that spread.'); }
    for (const page of document.pages) { check(spreads.get(page.spreadId)?.pageIds.includes(page.id) === true, 'Every page must be a member of its existing spread.'); pageContent(document, page); }
    threads(localFrames(document));
  });
}

/** 🧩️ Validates only newly supplied composed Drawing content and its exact owned dialect. */
export function validateDrawingChild(child:import('../🟦️.ts').LayoutDrawingMember):string|null{try{const h=child.handle,d=h.target.dialect;if(!nonempty(h.childId)||!nonempty(h.target.artifactId)||d.artifactKind!=='s.draw.drawing'||d.standard!=='1'||d.subset!=='*')return 'Drawing member references must have independent identities and the genuine Draw dialect.';parseLayoutDrawingMember(child);return null;}catch{return 'Drawing member reference is invalid.';}}

/** 🖼️ Page paint retains a finite positive authored viewport independently of document Content fit. */
export function validatePageDrawingChild(child:import('../🟦️.ts').LayoutDrawingMember):string|null{return validateDrawingChild(child);}

function* drawingChildren(document:LayoutArtifact):Iterable<import('../🟦️.ts').LayoutDrawingMember>{if(document.backgroundDrawing!==undefined)yield document.backgroundDrawing;for(const page of document.pages)if(page.backgroundDrawing!==undefined)yield page.backgroundDrawing;}
function pageDrawingIdentity(document:LayoutArtifact,pageId:string,child:import('../🟦️.ts').LayoutDrawingMember):boolean{const aliases=(held:typeof child)=>held.handle.childId===child.handle.childId||held.handle.target.artifactId===child.handle.target.artifactId;if(document.backgroundDrawing!==undefined&&aliases(document.backgroundDrawing))return false;for(const page of document.pages)if(page.id!==pageId&&page.backgroundDrawing!==undefined&&aliases(page.backgroundDrawing))return false;return true;}
/** 🪪️ Validates only a new Page child and its identity against existing owned child slots. */
export function validatePageDrawingAssignment(document:LayoutArtifact,pageId:string,child:import('../🟦️.ts').LayoutDrawingMember):string|null{return validatePageDrawingChild(child)??(pageDrawingIdentity(document,pageId,child)?null:'Page Drawing children must have independent local and target identities.');}

export {SnapshotGraphRead,FinalGraphSourceBindings} from './👁️graph/🟦️.ts';
export type {LayoutGraphRead,ContentOwner,ParagraphRead,CharacterRead,StoryRead,ParentRead,SpreadRead,PageRead,FrameRead,GraphField} from './👁️graph/🟦️.ts';
