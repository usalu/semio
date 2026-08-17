import { readdirSync, readFileSync, writeFileSync, mkdirSync, existsSync, copyFileSync } from "fs";
import { join, dirname, relative } from "path";

const repo = process.cwd();
const paths = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
const ticket = paths[0];
const el = paths[1];
const barrel = paths[2];

function child(dir, pred) {
  return readdirSync(dir).find((n) => pred(n));
}
const core = join(el, child(el, (n) => n.includes("core")));
const portsFile = join(core, child(core, (n) => n.includes("Ports")), child(join(core, child(core, (n) => n.includes("Ports"))), (n) => n.endsWith(".tsx")));
const cnFile = join(core, child(core, (n) => n.includes("ClassNames")), child(join(core, child(core, (n) => n.includes("ClassNames"))), (n) => n.endsWith(".tsx")));
const agDir = join(el, "ActionGroup");
const compName = child(agDir, (n) => n.endsWith("component.tsx"));
const storyName = child(join(el, "Select"), (n) => n.includes("story") && n.endsWith(".tsx")) || "story.tsx";

const rel = (fromDir, toFile) => {
  let r = relative(fromDir, toFile).replaceAll("\\", "/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};

const bgDir = join(el, "ButtonGroup");
mkdirSync(bgDir, { recursive: true });
const btnDir = join(el, "Button");
mkdirSync(btnDir, { recursive: true });

const bgComp = join(bgDir, compName);
const btnComp = join(btnDir, compName);
const barrelRelFromBg = rel(bgDir, barrel);
const portsRelFromBg = rel(bgDir, portsFile);
const cnRelFromBg = rel(bgDir, cnFile);
const bgRelFromBtn = rel(btnDir, bgComp);
const portsRelFromBtn = rel(btnDir, portsFile);
const cnRelFromBtn = rel(btnDir, cnFile);
const barrelRelFromBtn = rel(btnDir, barrel);

const header = (elementPath) => `// #region ️Header
//  framework/ui/elements/${elementPath}/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion Header
`.replaceAll("Header", "\u{1F5C4}\u{FE0F}Header").replace("// #region \u{1F5C4}\u{FE0F}Header", "// #region \u{1F5C4}\u{FE0F}Header");

// Match ActionGroup header emoji exactly by copying first 6 lines and rewriting path line
const agHeaderLines = readFileSync(join(agDir, compName), "utf8").split("\n").slice(0, 6);
function makeHeader(elementName) {
  return [
    agHeaderLines[0],
    agHeaderLines[1].replace(/elements\/ActionGroup/, `elements/${elementName}`),
    ...agHeaderLines.slice(2),
  ].join("\n");
}

const bgBody = `${makeHeader("ButtonGroup")}

// #region Adapters
import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva } from "class-variance-authority";
import { reactHostPort } from "${portsRelFromBg}";
import { cn } from "${cnRelFromBg}";
// W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import {
  chromeControlItemClass,
  chromeControlGroupClass,
  borderElementClass,
  useLevel,
  type Level,
  Label,
  type ControlIcon,
  useControlInlineText,
  useControlAccessibleLabel,
  ControlHotkeyBadge,
  renderControlIcon,
} from "${barrelRelFromBg}";
// #endregion Adapters

// #region ButtonGroup
// Grouped control buttons with shared level context.
// Consumers MUST provide ButtonGroupItem children.

/**
 * buttonGroupItemVariants holds the data fields for a buttonGroupItemVariants record.
 **/
const buttonGroupItemVariants = cva(cn(chromeControlItemClass, "aspect-square"), {
  variants: {
    variant: {
      default: "",
      ghost: "border-transparent bg-transparent",
      outline: \`border \${borderElementClass}\`,
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

/**
 * ButtonGroupContext holds the data fields for a ButtonGroupContext record.
 **/
const ButtonGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ButtonGroupProps holds the data fields for a ButtonGroupProps record.
 **/
interface ButtonGroupProps extends Omit<React.ComponentProps<"div">, "id"> {
  detailPanelWidthMode?: "fit" | "fill";
  id?: string;
  showLabel?: boolean;
  children: React.ReactNode;
}

/**
 * ButtonGroup holds the data fields for a ButtonGroup record.
 **/
function ButtonGroup({ className, detailPanelWidthMode = "fit", id, showLabel, children, ...props }: ButtonGroupProps) {
  const level = useLevel();
  const buttonGroupContextValue = reactHostPort.useMemo(() => ({ level }), [level]);
  const buttonGroupElement = (
    <ButtonGroupContext.Provider value={buttonGroupContextValue}>
      <div
        data-slot="button-group"
        data-detail-panel-control={detailPanelWidthMode}
        id={id}
        data-level={level}
        className={cn(chromeControlGroupClass, detailPanelWidthMode === "fill" ? "w-full min-w-0" : "", "group/button-group", className)}
        {...props}
      >
        {children}
      </div>
    </ButtonGroupContext.Provider>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={\`\${id}-label\`}>
        {buttonGroupElement}
      </Label>
    );
  }

  return buttonGroupElement;
}

/**
 * ButtonGroupItem holds the data fields for a ButtonGroupItem record.
 **/
function ButtonGroupItem({
  className,
  children,
  id,
  icon,
  text,
  asChild = false,
  ...props
}: React.ComponentProps<"button"> & {
  id?: string;
  icon: ControlIcon;
  text?: string;
  asChild?: boolean;
}) {
  const context = reactHostPort.useContext(ButtonGroupContext);
  const level = context.level ?? "base";
  const Comp = asChild ? Slot : "button";
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const buttonGroupItemElement = (
    <Comp
      data-slot="button-group-item"
      id={id}
      aria-label={ariaLabel}
      title={ariaLabel}
      data-level={context.level || level}
      className={cn(
        buttonGroupItemVariants(),
        inlineText ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
        inlineText && "flex items-center gap-single py-single px-double w-auto aspect-auto",
        className,
      )}
      {...(props as any)}
    >
      {children}
      {inlineText ? (
        <span data-slot="inline-label" className={cn("min-w-0 text-xs whitespace-nowrap", /\\bjustify-between\\b/.test(String(className ?? "")) && "flex-1 truncate")}>
          {inlineText}
        </span>
      ) : null}
      <ControlHotkeyBadge id={id} allowInline={Boolean(inlineText)} />
      {renderControlIcon(icon)}
    </Comp>
  );

  return buttonGroupItemElement;
}

export { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
export type { ButtonGroupProps };
// #endregion ButtonGroup
`;

// Fix region emoji markers to match repo style by copying from ActionGroup markers
function stampRegions(text, regionName, regionEmojiName) {
  // Use same connector/adapters markers as ActionGroup
  const ag = readFileSync(join(agDir, compName), "utf8");
  const adaptersOpen = ag.match(/^\/\/ #region .+$/m)[0]; // first is header
  const lines = ag.split("\n");
  const regionOpens = lines.filter((l) => l.startsWith("// #region "));
  const regionCloses = lines.filter((l) => l.startsWith("// #endregion "));
  const headerOpen = regionOpens[0];
  const headerClose = regionCloses[0];
  const adaptersOpenL = regionOpens.find((l) => l.includes("Adapters") || /🔌/.test(l) || l.includes("Adapter"));
  const adaptersCloseL = regionCloses.find((l) => l.includes("Adapters") || /🔌/.test(l) || l.includes("Adapter"));
  // Main region in ActionGroup
  const mainOpen = regionOpens.find((l) => l.includes("ActionGroup") || l.includes("Action"));
  const mainClose = regionCloses.find((l) => l.includes("ActionGroup") || l.includes("Action"));
  // Fallback: just keep ASCII regions if detection fails - but try harder
  const aOpen = lines.find((l) => l.startsWith("// #region ") && l.includes("Adapters"))
    || lines.find((l) => /^\/\/ #region /.test(l) && !l.includes("Header") && lines.indexOf(l) < 30);
  // Get exact lines by index from ActionGroup
  const agAdaptersStart = lines.findIndex((l) => /Adapters/.test(l) && l.startsWith("// #region"));
  const agAdaptersEnd = lines.findIndex((l) => /Adapters/.test(l) && l.startsWith("// #endregion"));
  const agMainStart = lines.findIndex((l) => /ActionGroup/.test(l) && l.startsWith("// #region"));
  const agMainEnd = lines.findIndex((l, i) => i > agMainStart && /ActionGroup/.test(l) && l.startsWith("// #endregion"));
  
  let out = text;
  out = out.replace("// #region Adapters", lines[agAdaptersStart]);
  out = out.replace("// #endregion Adapters", lines[agAdaptersEnd]);
  // interim marker
  const interim = lines.find((l) => l.includes("W3-interim"));
  out = out.replace("// W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.", interim);
  out = out.replace(`// #region ButtonGroup`, lines[agMainStart].replace("ActionGroup", "ButtonGroup").replace(/🌩️/g, "\u{1F6E1}\u{FE0F}").replace(/ActionGroup/, "ButtonGroup"));
  // Simpler: construct main region line with shield emoji like inventory ButtonGroup wasn't region-named
  // Use Input Components style - just put a fitting emoji. Look at Toggle for group-ish
  return { out, lines, agAdaptersStart, agAdaptersEnd, agMainStart, interim };
}

// Build final ButtonGroup file with exact markers from ActionGroup
const agLines = readFileSync(join(agDir, compName), "utf8").split("\n");
const adaptersOpen = agLines.find((l) => l.startsWith("// #region ") && l.includes("Adapters"));
const adaptersClose = agLines.find((l) => l.startsWith("// #endregion ") && l.includes("Adapters"));
const interimLine = agLines.find((l) => l.includes("W3-interim") && l.includes("remaining symbols"));
const actionMainOpen = agLines.find((l) => l.startsWith("// #region ") && l.includes("ActionGroup"));
const actionMainClose = agLines.find((l) => l.startsWith("// #endregion ") && l.includes("ActionGroup"));
// Replace ActionGroup name in region tags, keep emoji from ActionGroup (lightning)
const bgMainOpen = actionMainOpen.replaceAll("ActionGroup", "ButtonGroup");
const bgMainClose = actionMainClose.replaceAll("ActionGroup", "ButtonGroup");

const bgFile = `${makeHeader("ButtonGroup")}

${adaptersOpen}
import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva } from "class-variance-authority";
import { reactHostPort } from "${portsRelFromBg}";
import { cn } from "${cnRelFromBg}";
${interimLine}
import {
  chromeControlItemClass,
  chromeControlGroupClass,
  borderElementClass,
  useLevel,
  type Level,
  Label,
  type ControlIcon,
  useControlInlineText,
  useControlAccessibleLabel,
  ControlHotkeyBadge,
  renderControlIcon,
} from "${barrelRelFromBg}";
${adaptersClose}

${bgMainOpen}
// Grouped control buttons with shared level context.
// Consumers MUST provide ButtonGroupItem children.

/**
 * buttonGroupItemVariants holds the data fields for a buttonGroupItemVariants record.
 **/
const buttonGroupItemVariants = cva(cn(chromeControlItemClass, "aspect-square"), {
  variants: {
    variant: {
      default: "",
      ghost: "border-transparent bg-transparent",
      outline: \`border \${borderElementClass}\`,
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

/**
 * ButtonGroupContext holds the data fields for a ButtonGroupContext record.
 **/
const ButtonGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ButtonGroupProps holds the data fields for a ButtonGroupProps record.
 **/
interface ButtonGroupProps extends Omit<React.ComponentProps<"div">, "id"> {
  detailPanelWidthMode?: "fit" | "fill";
  id?: string;
  showLabel?: boolean;
  children: React.ReactNode;
}

/**
 * ButtonGroup holds the data fields for a ButtonGroup record.
 **/
function ButtonGroup({ className, detailPanelWidthMode = "fit", id, showLabel, children, ...props }: ButtonGroupProps) {
  const level = useLevel();
  const buttonGroupContextValue = reactHostPort.useMemo(() => ({ level }), [level]);
  const buttonGroupElement = (
    <ButtonGroupContext.Provider value={buttonGroupContextValue}>
      <div
        data-slot="button-group"
        data-detail-panel-control={detailPanelWidthMode}
        id={id}
        data-level={level}
        className={cn(chromeControlGroupClass, detailPanelWidthMode === "fill" ? "w-full min-w-0" : "", "group/button-group", className)}
        {...props}
      >
        {children}
      </div>
    </ButtonGroupContext.Provider>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={\`\${id}-label\`}>
        {buttonGroupElement}
      </Label>
    );
  }

  return buttonGroupElement;
}

/**
 * ButtonGroupItem holds the data fields for a ButtonGroupItem record.
 **/
function ButtonGroupItem({
  className,
  children,
  id,
  icon,
  text,
  asChild = false,
  ...props
}: React.ComponentProps<"button"> & {
  id?: string;
  icon: ControlIcon;
  text?: string;
  asChild?: boolean;
}) {
  const context = reactHostPort.useContext(ButtonGroupContext);
  const level = context.level ?? "base";
  const Comp = asChild ? Slot : "button";
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const buttonGroupItemElement = (
    <Comp
      data-slot="button-group-item"
      id={id}
      aria-label={ariaLabel}
      title={ariaLabel}
      data-level={context.level || level}
      className={cn(
        buttonGroupItemVariants(),
        inlineText ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
        inlineText && "flex items-center gap-single py-single px-double w-auto aspect-auto",
        className,
      )}
      {...(props as any)}
    >
      {children}
      {inlineText ? (
        <span data-slot="inline-label" className={cn("min-w-0 text-xs whitespace-nowrap", /\\bjustify-between\\b/.test(String(className ?? "")) && "flex-1 truncate")}>
          {inlineText}
        </span>
      ) : null}
      <ControlHotkeyBadge id={id} allowInline={Boolean(inlineText)} />
      {renderControlIcon(icon)}
    </Comp>
  );

  return buttonGroupItemElement;
}

export { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
export type { ButtonGroupProps };
${bgMainClose}
`;

writeFileSync(bgComp, bgFile);
console.log("WROTE", bgComp);

// Button file - imports ButtonGroup from sibling
const btnMainOpen = actionMainOpen.replaceAll("ActionGroup", "Button");
const btnMainClose = actionMainClose.replaceAll("ActionGroup", "Button");

const btnFile = `${makeHeader("Button")}

${adaptersOpen}
import * as React from "react";
import { type VariantProps } from "class-variance-authority";
${interimLine}
import { type ControlIcon, type ElementProps } from "${barrelRelFromBtn}";
import { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants } from "${bgRelFromBtn}";
${adaptersClose}

${btnMainOpen}
// Single-item Button and cycling Button built on ButtonGroup.
// Consumers MUST provide an icon for each Button.

/**
 * ButtonProps holds the data fields for a ButtonProps record.
 **/
type ButtonProps = React.ComponentProps<"button"> &
  VariantProps<typeof buttonGroupItemVariants> & {
    asChild?: boolean;
    id?: string;
    icon: ControlIcon;
    text?: string;
    children?: React.ReactNode;
  };

/**
 * ButtonCycleItem holds the data fields for a ButtonCycleItem record.
 **/
interface ButtonCycleItem<T extends string> {
  value: T;
  label: string;
  icon: ControlIcon;
  text?: string;
  id?: string;
}

/**
 * ButtonCycleProps holds the data fields for a ButtonCycleProps record.
 **/
interface ButtonCycleProps<T extends string> extends Omit<React.ComponentProps<"button">, "children" | "id">, ElementProps {
  value?: T;
  onValueChange?: (value: T) => void;
  items: ButtonCycleItem<T>[];
  showLabel?: boolean;
}

/**
 **/
function Button({ className, asChild = false, id, icon, text, children, ...props }: ButtonProps) {
  return (
    <ButtonGroup className={className}>
      <ButtonGroupItem id={id} asChild={asChild} icon={icon} text={text} {...props}>
        {children}
      </ButtonGroupItem>
    </ButtonGroup>
  );
}

/**
 * ButtonCycle holds the data fields for a ButtonCycle record.
 **/
function ButtonCycle<T extends string = string>({ className, id, showLabel, value, onValueChange, items, ...props }: ButtonCycleProps<T>) {
  const currentIndex = items.findIndex((item) => item.value === value);
  const currentItem = currentIndex >= 0 ? items[currentIndex] : items[0];
  const cycleText = typeof currentItem?.text === "string" ? currentItem.text : typeof currentItem?.label === "string" ? currentItem.label : undefined;

  const handleCycle = () => {
    const nextIndex = (currentIndex + 1) % items.length;
    if (onValueChange) onValueChange(items[nextIndex].value);
  };

  return (
    <ButtonGroup id={id} showLabel={showLabel} className={className}>
      <ButtonGroupItem id={id} onClick={handleCycle} icon={currentItem.icon} text={cycleText} {...props} />
    </ButtonGroup>
  );
}

export { Button, ButtonCycle };
export type { ButtonCycleProps, ButtonProps };
${btnMainClose}
`;

writeFileSync(btnComp, btnFile);
console.log("WROTE", btnComp);

// Now patch barrel: re-read, replace cluster with import-then-export regions
const barrelText = readFileSync(barrel, "utf8");
const startMarker = "/**\n * buttonGroupItemVariants holds the data fields for a buttonGroupItemVariants record.";
const endMarker = "export type { ButtonCycleProps, ButtonProps };";
const startIdx = barrelText.indexOf(startMarker);
const endIdx = barrelText.indexOf(endMarker);
if (startIdx < 0 || endIdx < 0) {
  console.error("FAILED to locate Button cluster", { startIdx, endIdx });
  process.exit(1);
}
const endPos = endIdx + endMarker.length;
const before = barrelText.slice(0, startIdx);
const after = barrelText.slice(endPos);
// relative from barrel dir to leaves
const barrelDir = dirname(barrel);
const bgImportPath = rel(barrelDir, bgComp);
const btnImportPath = rel(barrelDir, btnComp);

const replacement = `// #region ButtonGroup
import { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants } from "${bgImportPath}";
export { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
// #endregion ButtonGroup

// #region Button
import { Button, ButtonCycle, type ButtonCycleProps, type ButtonProps } from "${btnImportPath}";
export { Button, ButtonCycle, type ButtonCycleProps, type ButtonProps };
// #endregion Button
`;

// Use ActionGroup region emoji style for barrel regions - look at how ActionGroup is in barrel
const barrelAg = barrelText.match(/\/\/ #region .*ActionGroup[\s\S]*?\/\/ #endregion .*ActionGroup/);
let finalReplacement = replacement;
if (barrelAg) {
  const open = barrelAg[0].split("\n")[0];
  const close = barrelAg[0].split("\n").filter((l) => l.startsWith("// #endregion"))[0];
  finalReplacement = `${open.replace("ActionGroup", "ButtonGroup").replace(/🌩️/g, "🌩️")}
import { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants } from "${bgImportPath}";
export { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
${close.replace("ActionGroup", "ButtonGroup")}

${open.replace("ActionGroup", "Button")}
import { Button, ButtonCycle, type ButtonCycleProps, type ButtonProps } from "${btnImportPath}";
export { Button, ButtonCycle, type ButtonCycleProps, type ButtonProps };
${close.replace("ActionGroup", "Button")}
`;
}

const newBarrel = before + finalReplacement + after.replace(/^\n+/, "\n");
// Verify we didn't leave old export
if (newBarrel.includes("function ButtonGroup(") || newBarrel.includes("function Button(")) {
  console.error("OLD DEFS STILL PRESENT");
  process.exit(1);
}
writeFileSync(barrel, newBarrel);
console.log("PATCHED barrel", { startIdx, endPos, newLines: newBarrel.split("\n").length });

// region balance
const lines = newBarrel.split("\n");
let o = 0, c = 0;
for (const l of lines) {
  if (/^\/\/\s*#region\b/.test(l)) o++;
  if (/^\/\/\s*#endregion\b/.test(l)) c++;
}
console.log({ opens: o, closes: c, balanced: o === c });
console.log("bg import path", bgImportPath);
console.log("btn import path", btnImportPath);
console.log("paths ok", { portsRelFromBg, cnRelFromBg, barrelRelFromBg, bgRelFromBtn });
