import { readFileSync, writeFileSync } from "fs";
const path = process.argv[2];
let text = readFileSync(path, "utf8");
const old = `export function setControlLabelIdResolver(resolver: (id: string) => string): void {
  _controlLabelIdResolver.current = resolver;
}

/** @emoji 🏷️ Maps shell control ids to i18n keys for inline labels (identity until a product resolver is set). */
export function resolveControlLabelId(id: string): string {
  if (id.startsWith("ui.nav.")) {
    const segment = id.slice("ui.nav.".length);
    if (segment === "back" || segment === "forward" || segment === "up") {
      return _controlLabelIdResolver.current(\`ui.nav.\${segment}\`);
    }
  }
  if (id === "ui.search.toggle") {
    return _controlLabelIdResolver.current("ui.search.toggle");
  }
  if (id === "ui.find.toggle") {
    return _controlLabelIdResolver.current("ui.find.toggle");
  }
  if (id === "ui.fullscreen.toggle") {
    return _controlLabelIdResolver.current("ui.fullscreen.toggle");
  }
  if (id === "ui.mobilePanel.toggle") {
    return _controlLabelIdResolver.current("ui.mobilePanel.toggle");
  }
  if (id.startsWith("ui.panelToggle.")) {
    return _controlLabelIdResolver.current(\`ui.panelToggle.\${id.slice("ui.panelToggle.".length)}\`);
  }
  if (id.startsWith("ui.ribbon.group.")) {
    return _controlLabelIdResolver.current(\`ui.ribbon.parent.\${id.slice("ui.ribbon.group.".length)}\`);
  }
  if (id.startsWith("ui.ribbon.") && id.includes(".group.")) {
    return _controlLabelIdResolver.current(\`ui.ribbon.parent.\${id.slice(id.lastIndexOf(".group.") + ".group.".length)}\`);
  }
  if (id === "ui.windowSearch.suggestions") {
    return _controlLabelIdResolver.current("ui.windowSearch.suggestions");
  }
  if (id === "ui.engagement.actions") {
    return _controlLabelIdResolver.current("ui.engagement.actions");
  }
  if (id === "search-input" || id === "ui.windowSearch.action") {
    return _controlLabelIdResolver.current("ui.windowSearch.action");
  }
  if (id.startsWith("playground.panel.")) {
    return _controlLabelIdResolver.current(\`ui.panelToggle.\${id.slice("playground.panel.".length)}\`);
  }
  return _controlLabelIdResolver.current(id);
}`;

const neu = `export function setControlLabelIdResolver(resolver: (id: string) => string): void {
  _controlLabelIdResolver.current = typeof resolver === "function" ? resolver : (id) => id;
}

/** @emoji 🏷️ Maps shell control ids to i18n keys for inline labels (identity until a product resolver is set). */
export function resolveControlLabelId(id: string): string {
  const resolve = typeof _controlLabelIdResolver.current === "function" ? _controlLabelIdResolver.current : (value: string) => value;
  if (id.startsWith("ui.nav.")) {
    const segment = id.slice("ui.nav.".length);
    if (segment === "back" || segment === "forward" || segment === "up") {
      return resolve(\`ui.nav.\${segment}\`);
    }
  }
  if (id === "ui.search.toggle") {
    return resolve("ui.search.toggle");
  }
  if (id === "ui.find.toggle") {
    return resolve("ui.find.toggle");
  }
  if (id === "ui.fullscreen.toggle") {
    return resolve("ui.fullscreen.toggle");
  }
  if (id === "ui.mobilePanel.toggle") {
    return resolve("ui.mobilePanel.toggle");
  }
  if (id.startsWith("ui.panelToggle.")) {
    return resolve(\`ui.panelToggle.\${id.slice("ui.panelToggle.".length)}\`);
  }
  if (id.startsWith("ui.ribbon.group.")) {
    return resolve(\`ui.ribbon.parent.\${id.slice("ui.ribbon.group.".length)}\`);
  }
  if (id.startsWith("ui.ribbon.") && id.includes(".group.")) {
    return resolve(\`ui.ribbon.parent.\${id.slice(id.lastIndexOf(".group.") + ".group.".length)}\`);
  }
  if (id === "ui.windowSearch.suggestions") {
    return resolve("ui.windowSearch.suggestions");
  }
  if (id === "ui.engagement.actions") {
    return resolve("ui.engagement.actions");
  }
  if (id === "search-input" || id === "ui.windowSearch.action") {
    return resolve("ui.windowSearch.action");
  }
  if (id.startsWith("playground.panel.")) {
    return resolve(\`ui.panelToggle.\${id.slice("playground.panel.".length)}\`);
  }
  return resolve(id);
}`;

if (!text.includes(old)) {
  console.error("OLD BLOCK NOT FOUND");
  const idx = text.indexOf("export function setControlLabelIdResolver");
  console.error(text.slice(idx, idx + 800));
  process.exit(1);
}
writeFileSync(path, text.replace(old, neu));
console.log("patched", path);
