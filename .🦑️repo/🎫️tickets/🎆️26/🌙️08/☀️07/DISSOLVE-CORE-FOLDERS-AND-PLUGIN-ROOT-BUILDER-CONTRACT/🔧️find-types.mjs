import { readFileSync } from "fs";
const lines = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core/🟦️component.ts","utf8").split("\n");
const names = ["UiStackNode","UiTreeNode","UtilityNode","UiSectionNode","UiFieldNode","UiInspectorFieldGroup","StoragePort","PersistedAnchor","PluginUiNode","AppDefinition","ModeDefinition"];
for (const name of names) {
  for (let i=0;i<lines.length;i++) {
    if (new RegExp(`(export )?(type|interface|const) ${name}\\b`).test(lines[i]) || lines[i].includes(`export type ${name}`) || lines[i].includes(`export interface ${name}`)) {
      console.log(`${name} @ ${i+1}: ${lines[i].slice(0,120)}`);
    }
  }
}
// find end of PluginRuntime / where AppDefinition is
for (let i=0;i<lines.length;i++) {
  if (/export type AppDefinition|export interface AppDefinition|export type ModeDefinition|export type WindowKindDefinition|export type PluginManifest|export type PluginContribution/.test(lines[i])) {
    console.log(`${i+1}: ${lines[i].slice(0,140)}`);
  }
}
