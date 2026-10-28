/** 🧪️ Portable IDML typography/story laws use an independent XML library port. */
import assert from "node:assert/strict";
import type { XmlNode } from "../../../../../../../../../../../../🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import { binary64, binary32, binary64Value, binary32Value } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import shared from "../../../🧪️fixtures/🔣️.json";
import witness from "./🔣️.json";
import * as text from "../🟦️.ts";

export interface TextXmlOracle { read(source: string): XmlNode; write(node: XmlNode): string }

/** ⚖️ Compares owned semantic outputs with shared neutral witnesses and independent XML roundtrips. */
export function verifyIdmlText(oracle: TextXmlOracle): { cases: number } {
  const source = shared.cases.find((row) => row.id === witness.sharedCase);
  if (!source?.paragraphStyles || !source.characterStyles || !source.styleRuns || !source.story) throw new Error("missing shared text witness");
  const paragraphStyles: text.ParagraphStyle[] = source.paragraphStyles.map((style) => ({ ...style, fontSize: binary64(style.fontSize), leading: binary64(style.leading), tracking: binary64(style.tracking) }));
  const characterStyles: text.CharacterStyle[] = source.characterStyles.map((style) => ({ ...style, fontSize: binary64(style.fontSize), tracking: binary64(style.tracking), color: [binary32(style.color[0]), binary32(style.color[1]), binary32(style.color[2]), binary32(style.color[3])] }));
  const story: text.TextStory = { id: "story-shared", content: source.story, styleRuns: source.styleRuns.map((run) => ({ start: BigInt(run.start), end: BigInt(run.end), paragraphStyleId: run.paragraphStyleId ?? null, characterStyleId: run.characterStyleId ?? null })) };
  const styles = { paragraphStyles, characterStyles };
  const saved = structuredClone(styles);
  const exported: XmlNode = { kind: "element", name: "Styles", attrs: [], children: [...paragraphStyles.map(text.paragraphNode), ...characterStyles.map(text.characterNode)] };
  const graphic: XmlNode = { kind: "element", name: "Graphic", attrs: [], children: characterStyles.map(text.characterColor).filter((node): node is XmlNode => node !== null) };
  const colors = text.readColors(oracle.read(oracle.write(graphic)));
  const imported = text.readStyles(oracle.read(oracle.write(exported)), colors);
  assert.deepEqual(imported, styles);
  const restored = text.readStory(oracle.read(oracle.write(text.storyNode(story))), imported, colors);
  assert.deepEqual(restored.story, story);
  assert.deepEqual(styles, saved);
  const inheritedColors = text.readColors(oracle.read(witness.colorsXml));
  const inherited = text.readStyles(oracle.read(witness.stylesXml), inheritedColors);
  const paragraph = inherited.paragraphStyles.find((style) => style.id === witness.expected.paragraph.id)!;
  const character = inherited.characterStyles.find((style) => style.id === witness.expected.character.id)!;
  assert.deepEqual({ id: paragraph.id, fontFamily: paragraph.fontFamily, fontSize: binary64Value(paragraph.fontSize), leading: binary64Value(paragraph.leading), tracking: binary64Value(paragraph.tracking), alignment: paragraph.alignment }, witness.expected.paragraph);
  assert.deepEqual({ id: character.id, fontFamily: character.fontFamily, fontSize: binary64Value(character.fontSize!), fontWeight: character.fontWeight, italic: character.italic, color: character.color!.map(binary32Value) }, { ...witness.expected.character, color: witness.expected.character.color.map(Math.fround) });
  const inline = text.readStory(oracle.read(witness.storyXml), inherited, inheritedColors);
  assert.equal(inline.story.content, witness.expected.text);
  assert.equal(new TextEncoder().encode(inline.story.content).length, witness.expected.utf8Bytes);
  assert.deepEqual(inline.story.styleRuns, witness.expected.runs.map((run) => ({ ...run, start: BigInt(run.start), end: BigInt(run.end) })));
  assert.equal(binary64Value(inline.paragraphStyles.find((style) => style.id === witness.expected.inlineParagraph.id)!.tracking), witness.expected.inlineParagraph.tracking);
  assert.equal(binary64Value(inline.characterStyles.find((style) => style.id === witness.expected.inlineCharacter.id)!.fontSize!), witness.expected.inlineCharacter.fontSize);
  const longStory: text.TextStory = { id: "long-story", content: witness.longStory.pattern.repeat(witness.longStory.repeat), styleRuns: [] };
  const long = text.readStory(oracle.read(oracle.write(text.storyNode(longStory))), styles, colors);
  assert.equal(long.story.content, longStory.content);
  for (const invalid of witness.invalidStyles) assert.throws(() => text.readStyles(oracle.read(invalid.xml)), { message: invalid.error });
  for (const invalid of witness.invalidRuns) assert.throws(() => text.storyNode({ ...story, styleRuns: [{ start: BigInt(invalid.start), end: BigInt(invalid.end), paragraphStyleId: null, characterStyleId: null }] }), { message: "layout.idml.style-range" });
  assert.throws(() => text.characterColor({ ...characterStyles[0], color: [binary32(0), binary32(0), binary32(0), binary32(0.5)] }), { message: "layout.idml.text-color-alpha" });
  const attributes = text.readStory(oracle.read(witness.inlineAttributeStoryXml), inherited, inheritedColors);
  assert.equal(attributes.paragraphStyles.at(-1)!.fontFamily, witness.expected.inlineAttributes.fontFamily);
  assert.equal(binary64Value(attributes.paragraphStyles.at(-1)!.leading), witness.expected.inlineAttributes.leading);
  assert.equal(attributes.characterStyles.at(-1)!.fontFamily, witness.expected.inlineAttributes.fontFamily);
  const collision = text.readStory(oracle.read(witness.storyXml), { paragraphStyles: [...inherited.paragraphStyles, { ...inherited.paragraphStyles[0], id: "inline-paragraph-4" }], characterStyles: [...inherited.characterStyles, { ...inherited.characterStyles[0], id: "inline-character-4" }] }, inheritedColors);
  assert.equal(collision.story.styleRuns[0].paragraphStyleId, witness.expected.collision.paragraphId);
  assert.equal(collision.story.styleRuns[0].characterStyleId, witness.expected.collision.characterId);
  const fragments = [...text.storyFragments(longStory, witness.longStory.maximumFragmentBytes)];
  assert.ok(fragments.length > 1);
  for (const fragment of fragments) {
    const root: XmlNode = { kind: "element", name: "Story", attrs: [{ name: "Self", value: "fragment" }], children: [fragment] };
    const part = text.readStory(oracle.read(oracle.write(root)), styles, colors).story;
    assert.ok(new TextEncoder().encode(part.content).length <= witness.longStory.maximumFragmentBytes);
  }
  const assertAttributes = (node: XmlNode, expected: Record<string, string>): void => {
    assert.equal(node.kind, "element");
    if (node.kind !== "element") throw new Error("expected semantic XML element");
    const attrs = Object.fromEntries(node.attrs.map((attribute) => [attribute.name, attribute.value]));
    for (const [key, value] of Object.entries(expected)) assert.equal(attrs[key], value);
  };
  const paraXml = oracle.read(oracle.write(text.paragraphNode(paragraphStyles[0])));
  assertAttributes(paraXml, witness.exported.paragraph);
  assertAttributes(oracle.read(oracle.write(text.characterNode(characterStyles[0]))), witness.exported.character);
  assertAttributes(oracle.read(oracle.write(text.characterColor(characterStyles[0])!)), witness.exported.color);
  if (paraXml.kind !== "element") throw new Error("expected paragraph element");
  const props = paraXml.children.find((node) => node.kind === "element" && node.name === "Properties");
  if (props?.kind !== "element") throw new Error("expected paragraph properties");
  for (const [name, expected] of Object.entries(witness.exported.properties)) {
    const entry: XmlNode | undefined = props.children.find((candidate: XmlNode) => candidate.kind === "element" && candidate.name === name);
    if (entry?.kind !== "element") throw new Error("missing paragraph property");
    assert.equal(entry.children.map((child: XmlNode) => child.kind === "text" ? child.text : "").join(""), expected);
  }
  return { cases: 7 + witness.invalidStyles.length + witness.invalidRuns.length + 1 };
}
