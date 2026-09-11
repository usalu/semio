/** Language-agnostic assembly mount contract — apps, window kinds, examples. */
export const expected = {
  editorAppId: "s.assembly@1/*#editor",
  viewerAppId: "s.assembly@1/*#viewer",
  windowKindId: "framework.window.tree",
  windowLabel: { en: "Structure", de: "Struktur" },
  examples: [
    { id: "two-room-corridor", label: { en: "Two Rooms And A Corridor", de: "Zwei Räume und ein Korridor" }, minSlots: 3 },
    { id: "wall-roof-facade-strip", label: { en: "Wall And Roof Facade Strip", de: "Wand-Dach-Fassadenstreifen" }, minSlots: 4 },
  ],
} as const;

export function assertMountContract(input: {
  editorAppId: string;
  viewerAppId: string;
  windowKindId: string;
  windowLabel: { en: string; de: string };
  examples: ReadonlyArray<{ id: string; label: { en: string; de: string }; slots: number }>;
}): void {
  if (input.editorAppId !== expected.editorAppId) throw new Error(`editor ${input.editorAppId}`);
  if (input.viewerAppId !== expected.viewerAppId) throw new Error(`viewer ${input.viewerAppId}`);
  if (input.windowKindId !== expected.windowKindId) throw new Error(`window ${input.windowKindId}`);
  if (input.windowLabel.en !== expected.windowLabel.en || input.windowLabel.de !== expected.windowLabel.de) {
    throw new Error(`label ${JSON.stringify(input.windowLabel)}`);
  }
  for (const example of expected.examples) {
    const got = input.examples.find((row) => row.id === example.id);
    if (!got) throw new Error(`missing ${example.id}`);
    if (got.label.en !== example.label.en || got.label.de !== example.label.de) throw new Error(`example label ${example.id}`);
    if (got.slots < example.minSlots) throw new Error(`slots ${example.id}`);
  }
}
