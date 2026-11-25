import type { OpenedArtifactV1, OpenedCheckV1, OpenedUiV1 } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🧩️opened/🟦️.ts";
import journey from "../🧫️fixtures/🔣️.json" with { type: "json" };

type Locale = "en" | "de";
type Role = (typeof journey.regions)[number]["role"];
type Result = { status: OpenedCheckV1["status"]; evidence: string[] };

/** 🪟️ Selects the windows of one role through the framework surface attributes, matching the kind id suffix of editor and viewer alike. */
export const regionSelectorV1 = (role: Role): string => `:is([data-surface-window-kind-id$="-${role}"],[data-surface-window-kind-id$=".${role}"])`;

/** 🏷️ The accessible name of a region in a language, read from the fixture. */
export const regionNameV1 = (role: Role, locale: Locale): string => journey.regions.find(row => row.role === role)!.names[locale];

const named = (name: string) => `[aria-label="${name}"]`;
const shown = async (ui: OpenedUiV1, selector: string, attributes: readonly string[] = [], text = false) => (await ui.observe(selector, attributes, text)).filter(row => row.visible);
const pause = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function until<T>(context: OpenedArtifactV1, read: () => Promise<T>, done: (value: T) => boolean, ms: number): Promise<T> {
  const deadline = Date.now() + ms;
  let value = await read();
  while (!done(value) && Date.now() < deadline) { context.signal.throwIfAborted(); await pause(500); value = await read(); }
  return value;
}

const utilisations = async (ui: OpenedUiV1): Promise<string[]> => (await shown(ui, regionSelectorV1("checks"), [], true)).flatMap(row => row.text.match(new RegExp(journey.utilisation.pattern, "gu")) ?? []);

async function regions(ui: OpenedUiV1, roles: readonly Role[], locale: Locale): Promise<{ present: Role[]; missing: Role[]; named: Role[] }> {
  const present: Role[] = [], labelled: Role[] = [];
  for (const role of roles) {
    if ((await shown(ui, regionSelectorV1(role))).length > 0) present.push(role);
    if ((await shown(ui, named(regionNameV1(role, locale)))).length > 0) labelled.push(role);
  }
  return { present, missing: roles.filter(role => !present.includes(role)), named: labelled };
}

async function setLocale(ui: OpenedUiV1, locale: Locale): Promise<string> {
  const settings = `[id="${journey.chrome.settings}"]`, language = `[id="${journey.chrome.language}"][role="combobox"]`, option = `[role="option"][data-value="${locale}"]`;
  for (const selector of [settings, language, option]) {
    if ((await shown(ui, selector)).length === 0) throw Error("Not visible: " + selector);
    await ui.click(selector);
  }
  await ui.press(language, "Escape");
  return (await ui.observe("html", ["lang"], false))[0]?.attributes.lang ?? "";
}

/** 🧭️ Runs the five editor checks on the EN 1993 editor of side A: windows, outline selection, Properties edit, live utilisation and the en to de switch. */
export async function inspectNormEditorV1(context: OpenedArtifactV1): Promise<readonly OpenedCheckV1[]> {
  const rows = new Map<string, Result>(journey.checks.filter(row => row.phase === "editor").map(row => [row.id, { status: "unexecuted", evidence: [] }]));
  const ui = context.a, all = journey.regions.map(row => row.role), outline = regionSelectorV1("outliner"), properties = regionSelectorV1("properties");
  const run = async (id: string, body: (row: Result) => Promise<boolean>) => {
    const row = rows.get(id)!;
    try { row.status = (await body(row)) ? "passed" : "failed"; } catch (error) { context.signal.throwIfAborted(); row.status = "unexecuted"; row.evidence.push(String(error).slice(0, 500)); }
  };
  await run("editor-windows", async row => {
    const found = await regions(ui, all, context.locale);
    row.evidence.push(JSON.stringify({ locale: context.locale, ...found }));
    return found.missing.length === 0 && found.named.length === all.length;
  });
  const member = `${outline} [role="treeitem"][data-ui-node-key$="${journey.member.idSuffix}"]`;
  await run("select-member", async row => {
    if ((await shown(ui, member)).length === 0) { await ui.click(`[role="combobox"][id="${journey.example.picker}"]`); await ui.click(`[role="option"][data-value="${journey.example.id}"]`); }
    const found = await until(context, () => shown(ui, member, ["aria-selected"]), rows => rows.length > 0, 30_000);
    if (found.length === 0) { row.evidence.push("Member row " + journey.member.idSuffix + " not visible in the outline."); return false; }
    await ui.click(member);
    const selected = await until(context, () => shown(ui, member, ["aria-selected"]), list => list.some(item => item.attributes["aria-selected"] === "true"), 15_000);
    row.evidence.push(JSON.stringify(selected));
    return selected.some(item => item.attributes["aria-selected"] === "true");
  });
  let before: string[] = [];
  await run("edit-value", async row => {
    const field = `${properties} [aria-label*="${journey.edit.field[context.locale]}" i]`;
    before = await utilisations(ui);
    const first = await until(context, () => shown(ui, field, ["value", "aria-valuenow"]), list => list.length > 0, 15_000);
    if (first.length === 0) { row.evidence.push("Field not visible: " + field); return false; }
    for (let step = 0; step < journey.edit.times; step++) await ui.press(field, journey.edit.key);
    await ui.press(field, journey.edit.commit);
    const read = (item?: (typeof first)[number]) => item?.attributes.value ?? item?.attributes["aria-valuenow"] ?? null;
    const changed = (list: typeof first) => list.length === 1 && read(first[0]) !== null && read(list[0]) !== null && read(list[0]) !== read(first[0]);
    const after = await until(context, () => shown(ui, field, ["value", "aria-valuenow"]), changed, 15_000);
    row.evidence.push(JSON.stringify({ before: read(first[0]), after: read(after[0]) }));
    return changed(after);
  });
  await run("checks-utilisation-updates", async row => {
    const after = await until(context, () => utilisations(ui), list => list.length > 0 && list.join("|") !== before.join("|"), journey.utilisation.settleMs);
    row.evidence.push(JSON.stringify({ before, after }));
    return before.length > 0 && after.length > 0 && after.join("|") !== before.join("|");
  });
  await run("language-de", async row => {
    const lang = await setLocale(ui, "de");
    const found = await until(context, () => regions(ui, all, "de"), value => value.named.length === all.length, 15_000);
    row.evidence.push(JSON.stringify({ lang, ...found }));
    const english = await setLocale(ui, "en"), back = await regions(ui, all, "en");
    row.evidence.push(JSON.stringify({ lang: english, ...back }));
    await setLocale(ui, context.locale);
    return /^de(?:-|$)/iu.test(lang) && found.named.length === all.length && /^en(?:-|$)/iu.test(english) && back.named.length === all.length;
  });
  return [...rows].map(([id, row]) => ({ id, ...row }));
}

/** 👁️ Verifies on side B, the read-only viewer, that exactly the 3D model, checks and diagrams windows are shown and the editor-only windows are absent. */
export async function inspectNormViewerV1(context: OpenedArtifactV1): Promise<readonly OpenedCheckV1[]> {
  const id = "viewer-windows", expected = journey.viewerRegions as readonly Role[], absent = journey.regions.map(row => row.role).filter(role => !expected.includes(role));
  try {
    const found = await until(context, () => regions(context.b, expected, context.locale), value => value.missing.length === 0, 30_000);
    const extra: Role[] = [];
    for (const role of absent) if ((await shown(context.b, regionSelectorV1(role))).length > 0) extra.push(role);
    const evidence = JSON.stringify({ ...found, unexpected: extra });
    return [{ id, status: found.missing.length === 0 && found.named.length === expected.length && extra.length === 0 ? "passed" : "failed", evidence: [evidence] }];
  } catch (error) { context.signal.throwIfAborted(); return [{ id, status: "unexecuted", evidence: [String(error).slice(0, 500)] }]; }
}
