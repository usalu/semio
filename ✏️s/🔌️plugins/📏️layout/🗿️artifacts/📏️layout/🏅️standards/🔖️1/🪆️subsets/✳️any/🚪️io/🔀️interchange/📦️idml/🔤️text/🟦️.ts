/** 🔤️ IDML typography and UTF-8 story semantics over owned Layout and XML values. */
import type { ParagraphStyle, CharacterStyle, TextStory, TextStyleRun } from "../../../../🧬️schema/🟦️.ts";
import type { XmlNode } from "../../../../../../../../../../../🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import { binary32, binary64, binary32Value, binary64Value } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type { ParagraphStyle, CharacterStyle, TextStory, TextStyleRun, XmlNode };
export interface TextStyles { paragraphStyles: ParagraphStyle[]; characterStyles: CharacterStyle[] }
export interface ImportedStory extends TextStyles { story: TextStory }
export type TextColor = NonNullable<CharacterStyle["color"]>;
export type TextColors = ReadonlyMap<string, TextColor>;

const noParagraph = "ParagraphStyle/$ID/NormalParagraphStyle";
const noCharacter = "CharacterStyle/$ID/[No character style]";
const reject = (code: string): never => { throw new Error(code); };
const element = (name: string, attrs: [string, string][] = [], children: XmlNode[] = []): XmlNode => ({ kind: "element", name, attrs: attrs.map(([name, value]) => ({ name, value })), children });
const children = (node: XmlNode): XmlNode[] => node.kind === "element" ? node.children : [];
const local = (node: XmlNode): string => node.kind === "element" ? node.name.split(":").at(-1)! : "";
const attr = (node: XmlNode, key: string): string | undefined => node.kind === "element" ? node.attrs.find((attribute) => attribute.name === key)?.value : undefined;
const identity = (node: XmlNode): string => attr(node, "Self") || reject("layout.idml.missing-identity");

function nodes(root: XmlNode, name: string): XmlNode[] {
  const found: XmlNode[] = [];
  const pending = [root];
  while (pending.length) {
    const node = pending.pop()!;
    if (local(node) === name) found.push(node);
    const next = children(node);
    for (let at = next.length - 1; at >= 0; at--) pending.push(next[at]);
  }
  return found;
}
function content(root: XmlNode): string {
  const parts: string[] = [];
  const pending = [root];
  while (pending.length) {
    const node = pending.pop()!;
    if (node.kind === "text" || node.kind === "cData") parts.push(node.text);
    else {
      const next = children(node);
      for (let at = next.length - 1; at >= 0; at--) pending.push(next[at]);
    }
  }
  return parts.join("");
}
function property(node: XmlNode, key: string): string | undefined {
  const properties = children(node).find((child) => local(child) === "Properties");
  const value = properties && children(properties).find((child) => local(child) === key);
  return value ? content(value) : undefined;
}
const numericPattern = /^[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?$/;
function numeric(value: string, code: string): number {
  const parsed = Number(value);
  return numericPattern.test(value) && Number.isFinite(parsed) ? parsed : reject(code);
}
function number(node: XmlNode, key: string): number | undefined {
  const value = attr(node, key) ?? property(node, key);
  if (value === undefined || value === "Auto") return undefined;
  return numeric(value, "layout.idml.style-number");
}
function decimal(value: number): string {
  if (!Number.isFinite(value)) return reject("layout.idml.style-number");
  return Object.is(value, -0) ? "-0" : String(value);
}
function fontStyle(weight: number, italic: boolean): string {
  const face = weight <= 150 ? "Thin" : weight <= 250 ? "ExtraLight" : weight <= 350 ? "Light" : weight <= 450 ? "Regular" : weight <= 550 ? "Medium" : weight <= 650 ? "SemiBold" : weight <= 750 ? "Bold" : weight <= 850 ? "ExtraBold" : "Black";
  return italic ? face + " Italic" : face;
}
function fontWeight(face: string): number {
  for (const [name, weight] of [["ExtraBold", 800], ["SemiBold", 600], ["Bold", 700], ["Black", 900], ["ExtraLight", 200], ["Light", 300], ["Thin", 100], ["Medium", 500]] as const) if (face.includes(name)) return weight;
  return 400;
}
function justification(value: string): string {
  switch (value) {
    case "left": case "LeftAlign": return "LeftAlign";
    case "center": case "CenterAlign": return "CenterAlign";
    case "right": case "RightAlign": return "RightAlign";
    case "justify": case "LeftJustified": case "FullyJustified": return "LeftJustified";
    default: return reject("layout.idml.justification");
  }
}
const propertyNode = (name: string, value: string, type: string): XmlNode => element(name, [["type", type]], [{ kind: "text", text: value }]);

/** 📝️ Maps an owned paragraph style into the semantic IDML style element. */
export function paragraphNode(style: ParagraphStyle): XmlNode {
  return element("ParagraphStyle", [["Self", style.id], ["Name", style.name], ["PointSize", decimal(binary64Value(style.fontSize))], ["FontStyle", fontStyle(style.fontWeight, false)], ["Tracking", decimal(binary64Value(style.tracking))], ["Justification", justification(style.alignment)]], [element("Properties", [], [propertyNode("AppliedFont", style.fontFamily, "string"), propertyNode("Leading", decimal(binary64Value(style.leading)), "unit")])]);
}
/** 🔠️ Maps optional character overrides without inventing inherited values. */
export function characterNode(style: CharacterStyle): XmlNode {
  const attrs: [string, string][] = [["Self", style.id]];
  if (style.name !== null) attrs.push(["Name", style.name]);
  if (style.fontSize !== null) attrs.push(["PointSize", decimal(binary64Value(style.fontSize))]);
  if (style.tracking !== null) attrs.push(["Tracking", decimal(binary64Value(style.tracking))]);
  if (style.fontWeight !== null || style.italic !== null) attrs.push(["FontStyle", fontStyle(style.fontWeight ?? 400, style.italic ?? false)]);
  if (style.color !== null) attrs.push(["FillColor", "Color/style-" + style.id]);
  return element("CharacterStyle", attrs, style.fontFamily === null ? [] : [element("Properties", [], [propertyNode("AppliedFont", style.fontFamily, "string")])]);
}
/** 🎨️ Emits the RGB resource named by characterNode; translucent text is refused explicitly. */
export function characterColor(style: CharacterStyle): XmlNode | null {
  if (style.color === null) return null;
  const channels = style.color.map(binary32Value);
  if (channels.some((channel) => !Number.isFinite(channel) || channel < 0 || channel > 1) || channels[3] !== 1) return reject("layout.idml.text-color-alpha");
  return element("Color", [["Self", "Color/style-" + style.id], ["Name", "style-" + style.id], ["Model", "Process"], ["Space", "RGB"], ["ColorValue", channels.slice(0, 3).map((channel) => decimal(Math.fround(channel * 255))).join(" ")]]);
}
/** 🖍️ Reads owned RGB resources using binary32 capture at the native arithmetic boundaries. */
export function readColors(root: XmlNode): Map<string, TextColor> {
  const colors = new Map<string, TextColor>([["Color/Black", [binary32(0), binary32(0), binary32(0), binary32(1)]]]);
  for (const node of nodes(root, "Color")) {
    if (attr(node, "Space") !== "RGB") continue;
    const values = (attr(node, "ColorValue") ?? "").trim().split(/\s+/).map((value) => numeric(value, "layout.idml.color-value"));
    if (values.length !== 3 || values.some((value) => !Number.isFinite(value) || value < 0 || value > 255)) return reject("layout.idml.color-value");
    colors.set(identity(node), [binary32(Math.fround(values[0]) / 255), binary32(Math.fround(values[1]) / 255), binary32(Math.fround(values[2]) / 255), binary32(1)]);
  }
  return colors;
}
function opacity(source: XmlNode, scope: string): number {
  const setting = children(source).find((child) => local(child) === scope);
  const blend = setting && nodes(setting, "BlendingSetting")[0];
  if (!blend) return 1;
  if (attr(blend, "BlendMode") !== undefined && attr(blend, "BlendMode") !== "Normal") return reject("layout.idml.blend-mode");
  const value = Math.fround(numeric(attr(blend, "Opacity") ?? "100", "layout.idml.opacity"));
  if (value < 0 || value > 100) return reject("layout.idml.opacity");
  return Math.fround(value / 100);
}
function paint(source: XmlNode, colors: TextColors): TextColor | null {
  const id = attr(source, "FillColor");
  if (id === undefined || id === "Swatch/None" || id === "$ID/None") return null;
  const color = colors.get(id) ?? reject("layout.idml.unsupported-color:" + id);
  const alpha = Math.fround(binary32Value(color[3]) * Math.fround(opacity(source, "TransparencySetting") * opacity(source, "FillTransparencySetting")));
  return [color[0], color[1], color[2], binary32(alpha)];
}
function paragraphStyle(source: XmlNode, base: ParagraphStyle | undefined, id: string): ParagraphStyle {
  const size = number(source, "PointSize") ?? (base ? binary64Value(base.fontSize) : 12);
  const justify = justification(attr(source, "Justification") ?? base?.alignment ?? "left");
  const face = attr(source, "FontStyle");
  return { id, name: attr(source, "Name") ?? "Paragraph", fontFamily: property(source, "AppliedFont") ?? attr(source, "AppliedFont") ?? base?.fontFamily ?? "sans-serif", fontSize: binary64(size), fontWeight: face === undefined ? base?.fontWeight ?? 400 : fontWeight(face), leading: binary64(number(source, "Leading") ?? (base ? binary64Value(base.leading) : size * 1.2)), tracking: binary64(number(source, "Tracking") ?? (base ? binary64Value(base.tracking) : 0)), alignment: justify === "CenterAlign" ? "center" : justify === "RightAlign" ? "right" : justify === "LeftJustified" ? "justify" : "left" };
}
function characterStyle(source: XmlNode, base: CharacterStyle | undefined, colors: TextColors, id: string): CharacterStyle {
  const face = attr(source, "FontStyle");
  const size = number(source, "PointSize");
  const tracking = number(source, "Tracking");
  return { id, name: attr(source, "Name") ?? null, fontFamily: property(source, "AppliedFont") ?? attr(source, "AppliedFont") ?? base?.fontFamily ?? null, fontSize: size === undefined ? base?.fontSize ?? null : binary64(size), fontWeight: face === undefined ? base?.fontWeight ?? null : fontWeight(face), italic: face === undefined ? base?.italic ?? null : face.includes("Italic") || face.includes("Oblique"), color: attr(source, "FillColor") === undefined ? base?.color ?? null : paint(source, colors), tracking: tracking === undefined ? base?.tracking ?? null : binary64(tracking) };
}
function basedOn(source: XmlNode): string | undefined {
  const id = property(source, "BasedOn") ?? attr(source, "BasedOn");
  return id === "n" || id === "[No paragraph style]" || id === "[No character style]" ? undefined : id;
}
function resolve<T>(sources: XmlNode[], construct: (node: XmlNode, base: T | undefined, id: string) => T): Map<string, T> {
  const byId = new Map<string, XmlNode>();
  for (const source of sources) { const id = identity(source); if (byId.has(id)) return reject("layout.idml.duplicate-style"); byId.set(id, source); }
  const resolved = new Map<string, T>();
  for (const source of sources) {
    let next: string | undefined = identity(source);
    const chain: string[] = [];
    const visiting = new Set<string>();
    while (next !== undefined && !resolved.has(next)) {
      if (visiting.has(next)) return reject("layout.idml.style-cycle");
      visiting.add(next);
      const current: XmlNode = byId.get(next) ?? reject("layout.idml.missing-base-style");
      chain.push(next);
      next = basedOn(current);
    }
    while (chain.length) { const id = chain.pop()!; const node = byId.get(id)!; const parent = basedOn(node); resolved.set(id, construct(node, parent === undefined ? undefined : resolved.get(parent), id)); }
  }
  return resolved;
}
/** 🧬️ Resolves BasedOn inheritance from a real Styles XML tree in source order. */
export function readStyles(root: XmlNode, colors: TextColors = readColors(root)): TextStyles {
  const paragraphs = nodes(root, "ParagraphStyle");
  const characters = nodes(root, "CharacterStyle");
  const paragraphStyles = resolve(paragraphs, paragraphStyle);
  const characterStyles = resolve<CharacterStyle>(characters, (node, base, id) => characterStyle(node, base, colors, id));
  return { paragraphStyles: paragraphs.filter((node) => identity(node) !== noParagraph || attr(node, "PointSize") !== undefined || property(node, "AppliedFont") !== undefined).map((node) => paragraphStyles.get(identity(node))!), characterStyles: characters.filter((node) => identity(node) !== noCharacter).map((node) => characterStyles.get(identity(node))!) };
}
function inlineId(prefix: string, styles: { id: string }[]): string {
  let index = styles.length + 1;
  while (styles.some((style) => style.id === prefix + index)) index++;
  return prefix + index;
}
function inlineParagraph(node: XmlNode, styles: TextStyles, inherited: string | undefined): string | undefined {
  const direct = attr(node, "AppliedParagraphStyle") ?? inherited;
  const base = styles.paragraphStyles.find((style) => style.id === direct);
  if (!["PointSize", "FontStyle", "Tracking", "Justification", "AppliedFont", "Leading"].some((key) => attr(node, key) !== undefined) && property(node, "AppliedFont") === undefined && property(node, "Leading") === undefined) return base?.id;
  const id = inlineId("inline-paragraph-", styles.paragraphStyles);
  styles.paragraphStyles.push(paragraphStyle(node, base, id));
  return id;
}
function inlineCharacter(node: XmlNode, styles: TextStyles, colors: TextColors, inherited: string | undefined): string | undefined {
  const direct = attr(node, "AppliedCharacterStyle") ?? inherited;
  const base = styles.characterStyles.find((style) => style.id === direct);
  if (!["PointSize", "FontStyle", "Tracking", "FillColor", "AppliedFont"].some((key) => attr(node, key) !== undefined) && property(node, "AppliedFont") === undefined) return base?.id;
  const id = inlineId("inline-character-", styles.characterStyles);
  styles.characterStyles.push(characterStyle(node, base, colors, id));
  return id;
}
interface Utf8Index { positions: Map<number, number>; length: number }
function utf8Positions(value: string): Utf8Index {
  const positions = new Map<number, number>([[0, 0]]);
  let bytes = 0;
  let offset = 0;
  for (const character of value) {
    const scalar = character.codePointAt(0)!;
    if (scalar >= 0xd800 && scalar <= 0xdfff) return reject("layout.idml.story-utf8");
    bytes += scalar < 0x80 ? 1 : scalar < 0x800 ? 2 : scalar < 0x10000 ? 3 : 4;
    offset += character.length;
    positions.set(bytes, offset);
  }
  return { positions, length: bytes };
}
/** 📖️ Reads content, line breaks and inline style ranges without mutating the supplied style collections. */
export function readStory(root: XmlNode, input: TextStyles, colors: TextColors = readColors(root)): ImportedStory {
  const styles: TextStyles = { paragraphStyles: [...input.paragraphStyles], characterStyles: [...input.characterStyles] };
  const story: TextStory = { id: identity(root), content: "", styleRuns: [] };
  let byteLength = 0n;
  const append = (value: string, paragraph: string | undefined, character: string | undefined): void => {
    const start = byteLength;
    byteLength += BigInt(utf8Positions(value).length);
    story.content += value;
    if (start === byteLength || paragraph === undefined && character === undefined) return;
    const previous = story.styleRuns.at(-1);
    if (previous && previous.end === start && previous.paragraphStyleId === (paragraph ?? null) && previous.characterStyleId === (character ?? null)) previous.end = byteLength;
    else story.styleRuns.push({ start, end: byteLength, paragraphStyleId: paragraph ?? null, characterStyleId: character ?? null });
  };
  const pending: { node: XmlNode; paragraph?: string; character?: string }[] = [{ node: root }];
  while (pending.length) {
    const current = pending.pop()!;
    const paragraph = local(current.node) === "ParagraphStyleRange" ? inlineParagraph(current.node, styles, current.paragraph) : current.paragraph;
    const character = local(current.node) === "CharacterStyleRange" ? inlineCharacter(current.node, styles, colors, current.character) : current.character;
    if (local(current.node) === "Content") append(content(current.node), paragraph, character);
    else if (local(current.node) === "Br") append("\n", paragraph, character);
    else { const next = children(current.node); for (let at = next.length - 1; at >= 0; at--) pending.push({ node: next[at], paragraph, character }); }
  }
  return { story, ...styles };
}
function checkedRuns(story: TextStory, index: Utf8Index): void {
  const positions = index.positions;
  const end = BigInt(index.length);
  for (const run of story.styleRuns) if (run.start < 0n || run.start > run.end || run.end > end || !positions.has(Number(run.start)) || !positions.has(Number(run.end))) return reject("layout.idml.style-range");
}
function fragment(story: TextStory, start: number, index: Utf8Index, maximumBytes: number): { end: number; node: XmlNode } {
  const positions = index.positions;
  const length = index.length;
  if (!Number.isSafeInteger(start) || start < 0 || start >= length || !positions.has(start) || !Number.isSafeInteger(maximumBytes) || maximumBytes < 1) return reject("layout.idml.story-cursor");
  let end = Math.min(start + maximumBytes, length);
  while (!positions.has(end) && end > start) end--;
  let owner: TextStyleRun | undefined;
  for (const run of story.styleRuns) {
    const a = Number(run.start), b = Number(run.end);
    if (a > start) end = Math.min(end, a);
    if (b > start) end = Math.min(end, b);
    if (a <= start && b > start) owner = run;
  }
  const begin = positions.get(start)!;
  let node: XmlNode;
  if (story.content[begin] === "\n") { end = start + 1; node = element("Br"); }
  else {
    const value = story.content.slice(begin, positions.get(end));
    const line = value.indexOf("\n");
    if (line >= 0) end = start + utf8Positions(value.slice(0, line)).length;
    if (end <= start) return reject("layout.idml.story-cursor");
    node = element("Content", [], [{ kind: "text", text: story.content.slice(begin, positions.get(end)) }]);
  }
  return { end, node: element("ParagraphStyleRange", [["AppliedParagraphStyle", owner?.paragraphStyleId ?? noParagraph]], [element("CharacterStyleRange", [["AppliedCharacterStyle", owner?.characterStyleId ?? noCharacter]], [node])]) };
}
/** ✂️ Returns one UTF-8 aligned, style-aligned story fragment under a caller-selected byte budget. */
export function storyFragment(story: TextStory, start: number, maximumBytes = 512): { end: number; node: XmlNode } {
  const positions = utf8Positions(story.content);
  checkedRuns(story, positions);
  return fragment(story, start, positions, maximumBytes);
}
/** 🧵️ Yields bounded semantic fragments so callers may report progress or cancel between outputs. */
export function* storyFragments(story: TextStory, maximumBytes = 512): Generator<XmlNode> {
  const positions = utf8Positions(story.content);
  checkedRuns(story, positions);
  const length = positions.length;
  let start = 0;
  while (start < length) { const next = fragment(story, start, positions, maximumBytes); start = next.end; yield next.node; }
}
/** 📜️ Builds the actual Story XML element from owned text and byte-indexed style ranges. */
export function storyNode(story: TextStory): XmlNode { return element("Story", [["Self", story.id]], [...storyFragments(story)]); }
