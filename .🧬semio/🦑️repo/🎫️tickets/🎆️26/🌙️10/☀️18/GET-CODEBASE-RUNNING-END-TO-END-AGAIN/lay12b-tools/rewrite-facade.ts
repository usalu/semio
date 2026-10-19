import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const root = process.cwd();
const UI = "🧰️framework/🔨️modules/🖱️ui";
const M = "🧰️framework/🔨️modules/🛂️manifest";
const T = {
  contract: M + "/🏷️ui-contract-and-canvas-types/🟦️.ts",
  onboarding: "🧰️framework/🔨️modules/🎓️onboarding/🟦️.ts",
  actionTypes: M + "/🎬️action-types-and-semantics/🟦️.ts",
  definitions: M + "/🔘️definitions-action-ids-and-shell-brand/🟦️.ts",
  argRes: "🧰️framework/🔨️modules/🧩️action-argument-resolution/🟦️.ts",
  pick: UI + "/🔨️modules/🎯️canvas-pick-targets/🟦️.ts",
} as const;
const AXES = "@semio-tech/framework/manifest/generated/ui-axes";
const groups: Record<string, string[]> = {
  contract: ["UiNumberLimits", "UiNumberScale", "InputKind", "TreeWindowRowExtent", "TreePresentation", "SliderAppearance", "CanvasPickRequest", "CanvasPickTarget", "CanvasHoverFocus", "CANVAS_HOVER_SOURCE_CANVAS", "CANVAS_HOVER_SOURCE_PICK_MENU"],
  onboarding: ["IntroductionDefinition", "IntroductionLogo", "IntroductionStepDefinition", "IntroductionDemonstration", "IntroductionPoint", "IntroductionPlacement", "IntroductionGesture", "IntroductionCursor", "IntroductionKeyModifier", "TutorialCameraState", "TutorialOverlayRect", "TutorialGestureCue", "TutorialCameraKeyframe", "TutorialDefinition", "TutorialUiSnapshot", "TutorialUiChange", "TutorialEvent", "TutorialDocumentEvent"],
  actionTypes: ["ActionArgDef", "argControl"],
  definitions: ["DialogChoice", "DialogDefinition", "dialogChoiceArgs"],
  argRes: ["actionArgRequiresChoice", "effectiveActionArgs", "invalidActionChoiceArgs", "unresolvedActionArgs", "unresolvedDialogChoiceArgs"],
  pick: ["canvasPickTargetKey", "sortCanvasPickTargetsGeneralFirst", "pickMostSpecificCanvasTarget", "canvasHoverFocusFromTarget"],
  axes: ["ShellLocale", "ShellTerminology"],
};
const owner = new Map<string, string>();
for (const [g, names] of Object.entries(groups)) for (const n of names) owner.set(n, g);
const files: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { if (e.name === "node_modules") continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(ts|tsx)$/u.test(e.name)) files.push(p); } };
walk(join(root, UI));
const stmt = /^(import|export)(\s+type)?\s*\{([^}]*)\}\s*from\s*"@semio-tech\/framework";?[ \t]*$/gmu;
const spec = (from: string, group: string): string => {
  if (group === "axes") return AXES;
  const r = relative(dirname(from), join(root, T[group as keyof typeof T])).split("\\").join("/");
  return r.startsWith(".") ? r : "./" + r;
};
const unmapped: string[] = [];
let changed = 0;
for (const f of files) {
  const src = readFileSync(f, "utf8");
  if (!src.includes('"@semio-tech/framework"')) continue;
  const out = src.replace(stmt, (all, kw: string, typeOnly: string | undefined, list: string) => {
    const items = list.split(",").map((x) => x.trim()).filter(Boolean);
    const by = new Map<string, string[]>();
    const keep: string[] = [];
    for (const item of items) {
      const name = item.replace(/^type\s+/u, "").replace(/\s+as\s+.*$/u, "");
      const g = owner.get(name);
      if (!g) { keep.push(item); unmapped.push(`${relative(root, f)}: ${name}`); continue; }
      (by.get(g) ?? by.set(g, []).get(g)!).push(item);
    }
    const lines: string[] = [];
    for (const [g, names] of by) lines.push(`${kw}${typeOnly ?? ""} { ${names.join(", ")} } from "${spec(f, g)}";`);
    if (keep.length) lines.push(`${kw}${typeOnly ?? ""} { ${keep.join(", ")} } from "@semio-tech/framework";`);
    changed++;
    return lines.join("\n");
  });
  if (out !== src) writeFileSync(f, out);
}
console.log("statements rewritten", changed);
console.log("unmapped", unmapped);
