import { parseSchemaRecord } from '@semio-tech/framework/schema/record';
import { binary64, binary32, binary64Value, binary32Value, parseBinary64, parseBinary32, type Binary64, type Binary32 } from '@semio-tech/framework/value/ieee754';
import { questionDictionaryNativeJson, questionDictionaryFromNativeJson } from '@semio-tech/framework/value/dictionary/native-json';
import { parseLayoutArtifact, parseLayoutDrawingMember, parsePage, parseParentPage, parseFrame, parseTextStory, parseImageLink, parseParagraphStyle, parseLayer, parseSpread, parseLayoutRect, parsePageOverride, parseTextStyleRun, parseQuestionDictionary } from '../../🟦️.ts';
import * as native from '../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts';
import type { LayoutMutation } from '../🟦️.ts';

type Parser = (value: unknown, at: string) => unknown;
interface Field { parse: Parser; native: (value: unknown, out: boolean) => unknown; missing?: () => unknown }
type Fields = Readonly<Record<string, Field>>;
type Names<T> = T extends unknown ? keyof T : never;
const field = (parse: Parser, convert: Field['native'] = value => value): Field => ({ parse, native: convert });
const string = field((value, at) => { if (typeof value !== 'string') throw new Error(at + ': string required'); return value; });
const boolean = field((value, at) => { if (typeof value !== 'boolean') throw new Error(at + ': boolean required'); return value; });
const index = field((value, at) => { if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < 0) throw new Error(at + ': safe unsigned index required'); return value; });
const u32 = field((value, at) => { const result = index.parse(value, at) as number; if (result > 0xffffffff) throw new Error(at + ': unsigned32 required'); return result; });
const finite = (value: unknown): number => { if (typeof value !== 'number' || !Number.isFinite(value)) throw new Error('Layout native JSON finite number required'); return value; };
const word = field(value => parseBinary64(value), (value, out) => out ? finite(binary64Value(value as Binary64)) : binary64(finite(value)));
const float = field(value => parseBinary32(value), (value, out) => out ? finite(binary32Value(value as Binary32)) : binary32(finite(value)));
const nullable = (inner: Field, required = false): Field => ({ parse: (value, at) => value === null ? null : inner.parse(value, at), native: (value, out) => value === null ? null : inner.native(value, out), ...(required ? {} : { missing: () => null }) });
const list = (inner: Field): Field => field((value, at) => native.array(value).map((item, i) => inner.parse(item, at + '[' + i + ']')), (value, out) => native.array(value).map(item => inner.native(item, out)));
const rgba = field((value, at) => { const result = native.array(value).map(channel => float.parse(channel, at)); if (result.length !== 4) throw new Error(at + ': four channels required'); return result; }, (value, out) => native.rgba(value, out));
const rect = field(parseLayoutRect, (value, out) => native.fields(value, ['x', 'y', 'w', 'h'], out));
const run = field(parseTextStyleRun, native.run);
const frame = field(parseFrame, native.frame);
const page = field(parsePage, (value, out) => native.page(value, out, false));
const parent = field(parseParentPage, (value, out) => native.page(value, out, true));
const story = field(parseTextStory, native.story);
const link = field(parseImageLink, native.image);
const paragraph = field(parseParagraphStyle, native.paragraph);
const layer = field(parseLayer), spread = field(parseSpread);
const override = field(parsePageOverride, (value, out) => { const row = { ...native.row(value) }; if (row.bounds != null) row.bounds = native.fields(row.bounds, ['x', 'y', 'w', 'h', 'rotation'], out); return row; });
const dictionary = field(parseQuestionDictionary, (value, out) => out ? questionDictionaryNativeJson(value as Parameters<typeof questionDictionaryNativeJson>[0]) : questionDictionaryFromNativeJson(value));
const document = field(parseLayoutArtifact, native.artifact);
const pageDrawing = field(parseLayoutDrawingMember, native.pageChild);
const id = { id: string }, target = { pageId: string, frameId: string };
const fields = {
  RenameLayout: { newName: string }, ChangePrintTarget: { newPrintTarget: nullable(string) }, ChangeDataFields: { newFields: nullable(dictionary, true) },
  CreatePage: { page, index: nullable(index) }, DeletePage: id, RenamePage: { ...id, newName: string }, ChangePageWidth: { ...id, newWidth: word }, ChangePageHeight: { ...id, newHeight: word },
  UpdatePageMargins: { ...id, top: word, right: word, bottom: word, left: word }, UpdatePageColumns: { ...id, count: u32, gutter: word }, ReorderPages: { ...id, toIndex: index },
  CreateStory: { story, index: nullable(index) }, DeleteStory: id, EditStory: { ...id, offset: index, delete: index, insert: string },
  CreateLink: { link, index: nullable(index) }, DeleteLink: id, ChangeLinkPath: { ...id, newPath: string },
  CreateFrame: { pageId: string, frame, index: nullable(index), layerId: nullable(string) }, DeleteFrame: target, MoveFrame: { ...target, newX: word, newY: word }, ResizeFrame: { ...target, newWidth: word, newHeight: word }, RotateFrame: { ...target, newRotation: word },
  ChangeFrameFill: { ...target, newFill: nullable(rgba) }, ChangeFrameStroke: { ...target, newStroke: nullable(rgba) }, ChangeFrameWrapMode: { ...target, newWrapMode: string }, ChangeFrameColumns: { ...target, newColumns: u32 },
  UpdateGrid: { baselineGrid: word, baselineOffset: word, snapToBaseline: boolean }, SetFrameFlags: { ...target, locked: nullable(boolean), visible: nullable(boolean) },
  UpdateParagraphStyle: { ...id, name: string, fontFamily: string, fontSize: word, fontWeight: u32, leading: word, tracking: word, alignment: string },
  UpdateTextFrame: { ...target, storyId: string, threadNext: nullable(string), insetX: word, insetY: word, insetWidth: word, insetHeight: word },
  UpdateLayer: { pageId: string, layerId: string, name: string, visible: boolean, locked: boolean },
  CreateCharacterStyle: { ...id, name: nullable(string), index: nullable(index) }, DeleteCharacterStyle: id,
  UpdateCharacterStyle: { ...id, name: nullable(string), fontFamily: nullable(string), fontSize: nullable(word), fontWeight: nullable(u32), italic: nullable(boolean), color: nullable(rgba), tracking: nullable(word) },
  UpdateParentPage: { ...id, name: string, width: word, height: word }, UpdateSpread: { ...id, name: string }, SetPageParent: { ...id, parentPageId: nullable(string) },
  SetPageGuides: { ...id, guides: list(rect) }, SetStoryRuns: { ...id, runs: list(run) }, UpdateLink: { ...id, width: u32, height: u32, dpi: u32, colorProfile: nullable(string) }, SetPageOverrides: { ...id, overrides: list(override) },
  CreateLayer: { pageId: string, ...id, name: string, index: nullable(index), remove: { ...boolean, missing: () => false } }, SetFrameLayer: { ...target, layerId: string }, ReorderFrame: { ...target, forward: boolean },
  DragFrames: { pageId: string, targets: list(string), dx: word, dy: word }, RotateFrames: { pageId: string, targets: list(string), pivotX: word, pivotY: word, angle: word }, ScaleFrames: { pageId: string, targets: list(string), pivotX: word, pivotY: word, sx: word, sy: word },
  CreateParagraphStyle: { style: paragraph, index: nullable(index) }, DeleteParagraphStyle: id, CreateParentPage: { page: parent, index: nullable(index) }, DeleteParentPage: id,
  CreateSpread: { spread, index: nullable(index) }, DeleteSpread: id, SetPageSpread: { ...id, spreadId: string, index: nullable(index) }, ReorderLayer: { pageId: string, layerId: string, toIndex: index },
  SetParentPageContent: { ...id, layers: list(layer), frames: list(frame) }, ReplaceDocument: { document },
  SetPageBackgroundDrawing: { pageId: string, drawing: nullable(pageDrawing, true) },
} satisfies Record<Names<LayoutMutation>, Fields>;

function variant(value: unknown, at: string): { name: keyof typeof fields; payload: Record<string, unknown>; shape: Fields } {
  const root = parseSchemaRecord(value, Object.keys(fields), at), names = Object.keys(root);
  if (names.length !== 1) throw new Error(at + ': exactly one semantic mutation tag required');
  const name = names[0] as keyof typeof fields, shape: Fields = fields[name];
  return { name, payload: parseSchemaRecord(root[name], Object.keys(shape), at + '.' + name), shape };
}

/** 🧬️ Parses only the complete canonical command model with exact owned words and unsigned indices. */
export function parseLayoutMutation(value: unknown, at = '$'): LayoutMutation {
  const { name, payload, shape } = variant(value, at), result: Record<string, unknown> = {};
  for (const [key, item] of Object.entries(shape)) { if (!Object.hasOwn(payload, key)) throw new Error(at + '.' + name + ': missing ' + key); result[key] = item.parse(payload[key], at + '.' + name + '.' + key); }
  return { [name]: result } as unknown as LayoutMutation;
}
/** 📥️ Converts the finite native JSON facet and declared defaults into the canonical command model. */
export function layoutMutationFromNativeJson(value: unknown): LayoutMutation {
  const { name, payload, shape } = variant(value, '$'), result: Record<string, unknown> = {};
  for (const [key, item] of Object.entries(shape)) {
    if (!Object.hasOwn(payload, key) && item.missing === undefined) throw new Error('Layout native JSON missing ' + name + '.' + key);
    result[key] = item.native(Object.hasOwn(payload, key) ? payload[key] : item.missing!(), false);
  }
  return parseLayoutMutation({ [name]: result });
}
/** 📤️ Emits native JSON through an explicit finite projection without weakening canonical words. */
export function layoutMutationNativeJson(value: LayoutMutation): unknown {
  const { name, payload, shape } = variant(parseLayoutMutation(value), '$'), result: Record<string, unknown> = {};
  for (const [key, item] of Object.entries(shape)) result[key] = item.native(payload[key], true);
  return { [name]: result };
}
