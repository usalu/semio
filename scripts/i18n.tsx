#!/usr/bin/env tsx
// #region Header

// scripts/i18n.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

import { readFileSync, writeFileSync } from "fs";
import { glob } from "glob";
import { Box, render, Text } from "ink";
import { dirname, join, relative } from "path";
import React from "react";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const localesDir = join(rootDir, "js", "js", "sketchpad", "locales");
const sketchpadDir = join(rootDir, "js", "js", "sketchpad");
const reportPath = join(rootDir, "reports", "i18n.json");

const CONFIG = {
  localesDir,
  sourceGlob: "js/js/sketchpad/**/*.{ts,tsx}",
  reportPath,
  locales: ["de.json", "en.json"],
  metadataKeys: ["label", "hotkey", "manual", "tutorial", "beginner", "normal", "description"],
};

const PATTERNS = [
  { kind: "id", regex: /\bid\s*=\s*["']([^"']+)["']/g },
  { kind: "id", regex: /\bid\s*=\s*\{["']([^"']+)["']\}/g },
  { kind: "id", regex: /\bid:\s*["']([^"']+)["']/g },
  { kind: "placeholderId", regex: /\bplaceholderId\s*=\s*["']([^"']+)["']/g },
  { kind: "i18nPressed", regex: /\bi18nPressed\s*=\s*["']([^"']+)["']/g },
  { kind: "actionId", regex: /\bactionId\s*=\s*["']([^"']+)["']/g },
  { kind: "actionId", regex: /\bactionId\s*=\s*\{["']([^"']+)["']\}/g },
  { kind: "tooltipId", regex: /\btooltipId\s*=\s*["']([^"']+)["']/g },
  { kind: "tooltipId", regex: /\btooltipId\s*=\s*\{["']([^"']+)["']\}/g },
  { kind: "t", regex: /\bt\(\s*["']([^"']+)["']/g },
  { kind: "t", regex: /\bi18n\.t\(\s*["']([^"']+)["']/g },
  { kind: "useLabel", regex: /\buseLabel\(\s*["']([^"']+)["']/g },
  { kind: "useHotkey", regex: /\buseHotkey\(\s*["']([^"']+)["']/g },
  { kind: "useTooltip", regex: /\buseTooltip\(\s*["']([^"']+)["']/g },
  { kind: "createPanelDefinition", regex: /createPanelDefinition\([^,]+,\s*["']([^"']+)["']/g },
  { kind: "panelToggleTooltip", regex: /semio\.sketchpad\.navbar\.panelToggle\.\$\{[^}]+\}\.\$\{[^}]+\s*\?\s*["']([^"']+)["']\s*:\s*["']([^"']+)["']/g },
];

const GENERIC_SECTION_TERMS = ["title", "label", "name", "section", "info", "data", "item", "content"];

interface Translation {
  [key: string]: string | Translation;
}

interface Issue {
  severity: "error" | "warning";
  key: string;
  message: string;
  location?: string;
}

interface ValidationResult {
  status: "Valid" | "Incomplete" | "Warning" | "Missing";
  details: string;
}

interface FoundId {
  kind: string;
  files: Set<string>;
}

//#region Utilities
function getNestedProperty(obj: Translation, path: string): string | Translation | null {
  const parts = path.split(".");
  let current: any = obj;
  for (const part of parts) {
    if (!current || typeof current !== "object") return null;
    current = current[part];
  }
  return current;
}

function setNestedProperty(obj: Translation, path: string, value: any): boolean {
  const parts = path.split(".");
  let current: any = obj;
  for (let i = 0; i < parts.length - 1; i++) {
    if (!current[parts[i]]) current[parts[i]] = {};
    current = current[parts[i]];
  }
  current[parts[parts.length - 1]] = value;
  return true;
}

function removeNestedProperty(obj: Translation, path: string): boolean {
  const parts = path.split(".");
  let current: any = obj;
  const parents: Array<{ obj: any; key: string }> = [];

  for (let i = 0; i < parts.length - 1; i++) {
    if (!current[parts[i]]) return false;
    parents.push({ obj: current, key: parts[i] });
    current = current[parts[i]];
  }

  const lastKey = parts[parts.length - 1];
  if (!current || !current.hasOwnProperty(lastKey)) return false;

  if (typeof current[lastKey] === "object" && current[lastKey] !== null) {
    const keys = Object.keys(current[lastKey]);
    const hasOnlyMetadata = keys.every((k) => CONFIG.metadataKeys.includes(k));
    if (!hasOnlyMetadata && keys.length > 0) return false;
  }

  delete current[lastKey];

  for (let i = parents.length - 1; i >= 0; i--) {
    const { obj, key } = parents[i];
    if (obj[key] && Object.keys(obj[key]).length === 0) {
      delete obj[key];
    } else {
      break;
    }
  }

  return true;
}

function getAllKeys(obj: Translation, prefix = ""): string[] {
  const keys: string[] = [];
  if (!obj || typeof obj !== "object") return keys;

  for (const prop in obj) {
    const key = prefix ? `${prefix}.${prop}` : prop;
    if (CONFIG.metadataKeys.includes(prop)) continue;
    keys.push(key);
    if (typeof obj[prop] === "object" && obj[prop] !== null) {
      keys.push(...getAllKeys(obj[prop] as Translation, key));
    }
  }
  return keys;
}

function flattenKeys(obj: Translation, prefix: string = ""): string[] {
  const keys: string[] = [];
  for (const [key, value] of Object.entries(obj)) {
    const fullKey = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "object" && value !== null) {
      keys.push(...flattenKeys(value, fullKey));
    } else {
      keys.push(fullKey);
    }
  }
  return keys;
}

function getNestedValue(obj: Translation, path: string): string | Translation | undefined {
  const parts = path.split(".");
  let current: any = obj;
  for (const part of parts) {
    if (current && typeof current === "object" && part in current) {
      current = current[part];
    } else {
      return undefined;
    }
  }
  return current;
}

function setNestedValue(obj: Translation, path: string, value: any): void {
  const parts = path.split(".");
  let current: any = obj;
  for (let i = 0; i < parts.length - 1; i++) {
    const part = parts[i];
    if (!(part in current) || typeof current[part] !== "object") {
      current[part] = {};
    }
    current = current[part];
  }
  current[parts[parts.length - 1]] = value;
}

function deleteNestedValue(obj: Translation, path: string): void {
  const parts = path.split(".");
  let current: any = obj;
  for (let i = 0; i < parts.length - 1; i++) {
    const part = parts[i];
    if (!(part in current) || typeof current[part] !== "object") {
      return;
    }
    current = current[part];
  }
  delete current[parts[parts.length - 1]];
}

function extractReadableName(key: string): string {
  const parts = key.split(".");
  let lastPart = parts[parts.length - 1];

  lastPart = lastPart.replace(/^(label|normal|beginner)$/, "");
  if (!lastPart && parts.length > 1) {
    lastPart = parts[parts.length - 2];
  }

  return lastPart
    .replace(/([A-Z])/g, " $1")
    .replace(/^./, (str) => str.toUpperCase())
    .trim();
}

function generateDefaultLabel(id: string): string {
  const parts = id.split(".");
  const lastPart = parts[parts.length - 1];
  const secondLastPart = parts.length > 1 ? parts[parts.length - 2] : "";

  let label = lastPart;
  if (lastPart === "label" && secondLastPart) {
    label = secondLastPart.replace(/Placeholder$/, "");
  }

  const capitalizedLabel = label
    .replace(/([A-Z])/g, " $1")
    .replace(/^./, (str) => str.toUpperCase())
    .trim();

  return capitalizedLabel || "Label";
}

function checkGenericTerminology(id: string): string | null {
  const parts = id.split(".");
  const lastPart = parts[parts.length - 1];

  if (CONFIG.metadataKeys.includes(lastPart)) return null;

  if (GENERIC_SECTION_TERMS.includes(lastPart)) {
    return `Uses generic term '${lastPart}' - should be more specific (e.g., 'properties', 'metadata', 'details')`;
  }

  return null;
}
//#endregion Utilities

//#region Scanning
async function scanSourceFiles(): Promise<Map<string, FoundId>> {
  const files = await glob(CONFIG.sourceGlob, { cwd: rootDir, absolute: true });
  const foundIds = new Map<string, FoundId>();

  for (const file of files) {
    const content = readFileSync(file, "utf8");
    const relPath = relative(rootDir, file).replace(/\\/g, "/");

    for (const { kind, regex } of PATTERNS) {
      let match;
      const regexCopy = new RegExp(regex.source, regex.flags);
      while ((match = regexCopy.exec(content)) !== null) {
        const id = match[1];
        if (!id.startsWith("semio.sketchpad.") && !id.startsWith("tooltip.") && !id.startsWith("settings.")) continue;

        if (!foundIds.has(id)) {
          foundIds.set(id, { kind, files: new Set() });
        }
        foundIds.get(id)!.files.add(relPath);
      }
    }

    const panelTogglePattern = /semio\.sketchpad\.navbar\.panelToggle\.\$\{[^}]+\}\.\$\{[^}]+\s*\?\s*["']([^"']+)["']\s*:\s*["']([^"']+)["']/g;
    let match;
    while ((match = panelTogglePattern.exec(content)) !== null) {
      const hideId = match[1];
      const showId = match[2];

      if (hideId.startsWith("semio.sketchpad.")) {
        if (!foundIds.has(hideId)) {
          foundIds.set(hideId, { kind: "panelToggleTooltip", files: new Set() });
        }
        foundIds.get(hideId)!.files.add(relPath);
      }

      if (showId.startsWith("semio.sketchpad.")) {
        if (!foundIds.has(showId)) {
          foundIds.set(showId, { kind: "panelToggleTooltip", files: new Set() });
        }
        foundIds.get(showId)!.files.add(relPath);
      }
    }

    const inlineTooltipPattern = /`semio\.sketchpad\.navbar\.panelToggle\.([a-zA-Z0-9_]+)\.(show|hide)`/g;
    while ((match = inlineTooltipPattern.exec(content)) !== null) {
      const panelKey = match[1];
      const action = match[2];
      const id = `semio.sketchpad.navbar.panelToggle.${panelKey}.${action}`;

      if (!foundIds.has(id)) {
        foundIds.set(id, { kind: "inlineTooltip", files: new Set() });
      }
      foundIds.get(id)!.files.add(relPath);
    }
  }

  return foundIds;
}
//#endregion Scanning

//#region Validation
function validateEntry(locale: Translation, id: string, kind: string): ValidationResult {
  const value = getNestedProperty(locale, id);

  if (!value) return { status: "Missing", details: "Key does not exist" };

  if (typeof value === "string") {
    return kind === "placeholderId" ? { status: "Valid", details: "String value" } : { status: "Warning", details: "Expected object with label/hotkey, found string" };
  }

  const issues: string[] = [];
  const hasLabel = (value as any).label != null;

  if (hasLabel) {
    if (typeof (value as any).label === "string") {
      issues.push("label is string (expected object with normal/beginner)");
    } else if (typeof (value as any).label === "object") {
      const hasNormal = (value as any).label.normal != null;
      const hasBeginner = (value as any).label.beginner != null;
      if (!hasNormal && !hasBeginner) issues.push("label object is empty");
      if (hasNormal && (value as any).label.normal === "") issues.push("label.normal is empty string");
      if (hasBeginner && (value as any).label.beginner === "") issues.push("label.beginner is empty string");
    }
  } else {
    issues.push("missing label property");
  }

  if (issues.length > 0) {
    return { status: "Incomplete", details: issues.join(", ") };
  }

  return { status: "Valid", details: "OK" };
}

function validateTranslations(en: Translation, de: Translation, usedIds: Set<string>): Issue[] {
  const issues: Issue[] = [];
  const enKeys = flattenKeys(en);
  const deKeys = flattenKeys(de);

  for (const key of enKeys) {
    if (!deKeys.includes(key)) {
      issues.push({
        severity: "error",
        key,
        message: `Missing German translation for key: ${key}`,
      });
    }
  }

  for (const key of deKeys) {
    if (!enKeys.includes(key)) {
      issues.push({
        severity: "warning",
        key,
        message: `Extra German translation key (not in English): ${key}`,
      });
    }
  }

  for (const key of enKeys) {
    const enValue = getNestedValue(en, key);
    const deValue = getNestedValue(de, key);
    if (typeof enValue === "string" && typeof deValue === "string" && enValue === deValue && enValue !== "") {
      const technicalTerms = [
        "ID",
        "X",
        "Y",
        "Z",
        "U",
        "V",
        "T",
        "HUD",
        "URL",
        "Email",
        "System",
        "Layout",
        "Normal",
        "Desktop",
        "Tablet",
        "Connector",
        "Chat",
        "Kit",
        "Design",
        "Tutorial",
        "Remote",
        "Name",
        "Version",
        "Homepage",
        "Label",
        "Min",
        "Max",
        "Tags",
        "Tag",
        "Concept",
        "Interface",
        "Position",
        "Rotation",
        "Definition",
        "Id",
        "???",
        "??",
        "Beginner",
        "Expert",
        "Developer",
        "User",
        "Mobile",
        "Tablet",
        "Desktop",
      ];

      if (enValue.includes("/") || enValue.includes("\\")) continue;
      if (enValue.includes("Ctrl") || enValue.includes("Alt") || enValue.includes("Shift") || enValue.includes("Meta")) continue;
      if (enValue.length <= 2) continue;
      if (/^[A-Z][a-z]+( [A-Z][a-z]+)*( Id)?$/.test(enValue)) continue;
      if (key.endsWith(".manual")) continue;
      if (enValue.includes("tag1") || enValue.includes("e.g.") || enValue.includes("...")) continue;
      if (!key.startsWith("semio.sketchpad.")) continue;
      if (!technicalTerms.includes(enValue)) {
        issues.push({
          severity: "warning",
          key,
          message: `Incomplete translation (same as English): ${key}`,
        });
      }
    }
  }

  for (const key of enKeys) {
    const baseId = key.replace(/\.(label|beginner|manual|tutorial|hotkey)$/, "");
    if (!usedIds.has(baseId) && !key.includes(".")) {
      issues.push({
        severity: "warning",
        key,
        message: `Unused translation key: ${key}`,
      });
    }
  }

  Array.from(usedIds).forEach((id) => {
    const labelKey = `${id}.label`;
    const labelKeyNormal = `${id}.label.normal`;

    if (!enKeys.includes(labelKey) && !enKeys.includes(labelKeyNormal)) {
      issues.push({
        severity: "error",
        key: labelKey,
        message: `Missing English translation for UI element: ${id}`,
      });
    }
  });

  return issues;
}
//#endregion Validation

//#region Commands
async function validate(): Promise<{
  foundIds: Map<string, FoundId>;
  results: Record<string, Record<string, Array<{ id: string; kind: string; details: string; files: string[] }>>>;
  unusedKeys: Record<string, string[]>;
  locales: Record<string, Translation>;
  genericTermWarnings: Map<string, { warning: string; files: string[] }>;
}> {
  const locales: Record<string, Translation> = {};
  for (const filename of CONFIG.locales) {
    const filePath = join(CONFIG.localesDir, filename);
    locales[filename] = JSON.parse(readFileSync(filePath, "utf8"));
  }

  const foundIds = await scanSourceFiles();
  const results: Record<string, Record<string, Array<{ id: string; kind: string; details: string; files: string[] }>>> = {};
  const genericTermWarnings = new Map<string, { warning: string; files: string[] }>();

  for (const filename of CONFIG.locales) {
    const langCode = filename.replace(".json", "");
    results[langCode] = { Valid: [], Incomplete: [], Warning: [], Missing: [] };

    for (const [id, { kind }] of foundIds) {
      const check = validateEntry(locales[filename], id, kind);
      if (!results[langCode][check.status]) {
        results[langCode][check.status] = [];
      }
      results[langCode][check.status].push({ id, kind, details: check.details, files: Array.from(foundIds.get(id)!.files) });

      const genericWarning = checkGenericTerminology(id);
      if (genericWarning && !genericTermWarnings.has(id)) {
        genericTermWarnings.set(id, { warning: genericWarning, files: Array.from(foundIds.get(id)!.files) });
      }
    }
  }

  const usedKeys = new Set(foundIds.keys());
  const unusedKeys: Record<string, string[]> = {};

  for (const filename of CONFIG.locales) {
    const allKeys = getAllKeys(locales[filename]);
    unusedKeys[filename] = allKeys.filter((key) => {
      if (usedKeys.has(key)) return false;
      for (const part of key.split(".")) {
        if (part.endsWith("Placeholder")) return false;
      }
      return true;
    });
  }

  return { foundIds, results, unusedKeys, locales, genericTermWarnings };
}

async function generateReport(): Promise<void> {
  const { foundIds, results, unusedKeys, genericTermWarnings } = await validate();

  let report = "# i18n Validation Report\n\n";
  report += `Generated: ${new Date().toLocaleString()}\n\n`;
  report += `## Summary\n\nTotal UI elements scanned: **${foundIds.size}**\n\n`;

  for (const filename of CONFIG.locales) {
    const langCode = filename.replace(".json", "");
    const { Valid, Incomplete, Warning, Missing } = results[langCode];
    const total = Valid.length + Incomplete.length + (Warning || []).length + Missing.length;
    const pct = ((Valid.length / total) * 100).toFixed(1);

    report += `\n### ${filename}\n\n`;
    report += `- ✓ Valid: ${Valid.length} / ${total} (${pct}%)\n`;
    report += `- ⚠ Warning: ${(Warning || []).length} / ${total}\n`;
    report += `- ⚠ Incomplete: ${Incomplete.length} / ${total}\n`;
    report += `- ✗ Missing: ${Missing.length} / ${total}\n`;
  }

  report += "\n\n## Generic Terminology Warnings\n\n";
  if (genericTermWarnings && genericTermWarnings.size > 0) {
    report += "The following IDs use generic terminology and should be more specific:\n\n";
    report += "| ID | Warning | Files |\n|---|---|---|\n";
    for (const [id, { warning, files }] of genericTermWarnings) {
      report += `| \`${id}\` | ${warning} | ${files.join(", ")} |\n`;
    }
  } else {
    report += "No generic terminology issues found.\n";
  }

  report += "\n\n## Details\n\n";

  for (const filename of CONFIG.locales) {
    const langCode = filename.replace(".json", "");
    const { Incomplete, Warning, Missing } = results[langCode];

    report += `### ${langCode} Missing Entries\n\n`;
    if (Missing.length === 0) {
      report += "No missing entries.\n";
    } else {
      report += "| ID | Kind | Files | Details |\n|---|---|---|---|\n";
      for (const { id, kind, files, details } of Missing) {
        report += `| \`${id}\` | ${kind} | ${files.join(", ")} | ${details} |\n`;
      }
    }

    report += "\n#### Incomplete Entries\n\n";
    if (Incomplete.length === 0) {
      report += "No incomplete entries.\n";
    } else {
      report += "| ID | Kind | Files | Issues |\n|---|---|---|---|\n";
      for (const { id, kind, files, details } of Incomplete) {
        report += `| \`${id}\` | ${kind} | ${files.join(", ")} | ${details} |\n`;
      }
    }

    report += "\n#### Warnings\n\n";
    if (!Warning || Warning.length === 0) {
      report += "No warnings.\n";
    } else {
      report += "| ID | Kind | Files | Details |\n|---|---|---|---|\n";
      for (const { id, kind, files, details } of Warning) {
        report += `| \`${id}\` | ${kind} | ${files.join(", ")} | ${details} |\n`;
      }
    }

    report += "\n#### Unused Locale Keys\n\n";
    if (unusedKeys[filename].length === 0) {
      report += "No unused keys.\n";
    } else {
      report += "These keys exist in the locale file but are not referenced in the codebase:\n\n";
      for (const key of unusedKeys[filename]) {
        report += `- \`${key}\`\n`;
      }
    }
    report += "\n";
  }

  writeFileSync(CONFIG.reportPath.replace(".json", ".md"), report, "utf8");
}

async function addMissing(): Promise<void> {
  const { foundIds, results, locales } = await validate();
  let totalAdded = 0;

  for (const filename of CONFIG.locales) {
    const langCode = filename.replace(".json", "");
    const missing = results[langCode].Missing;
    let addedCount = 0;

    for (const { id, kind } of missing) {
      const parts = id.split(".");
      const lastPart = parts[parts.length - 1];
      const secondLastPart = parts.length > 1 ? parts[parts.length - 2] : "";

      let value: any;

      if (kind === "placeholderId") {
        const label = secondLastPart
          .replace(/([A-Z])/g, " $1")
          .replace(/Placeholder$/, "")
          .trim();
        const capitalizedLabel = label.charAt(0).toUpperCase() + label.slice(1);

        if (lastPart === "label") {
          value = { label: { normal: capitalizedLabel, beginner: capitalizedLabel } };
        } else {
          value = capitalizedLabel;
        }
      } else {
        const label = generateDefaultLabel(id);

        if (lastPart === "label") {
          value = { label: { normal: label, beginner: label } };
        } else {
          value = { label: { normal: label, beginner: label } };
        }
      }

      if (setNestedProperty(locales[filename], id, value)) {
        addedCount++;
      }
    }

    const filePath = join(CONFIG.localesDir, filename);
    writeFileSync(filePath, JSON.stringify(locales[filename], null, 2) + "\n", "utf8");
    totalAdded += addedCount;
  }
}

async function cleanUnused(): Promise<void> {
  const reportMdPath = CONFIG.reportPath.replace(".json", ".md");
  let reportContent: string;
  try {
    reportContent = readFileSync(reportMdPath, "utf8");
  } catch {
    return;
  }
  const unusedKeysSection = reportContent.match(/#### Unused Locale Keys\s+([\s\S]*?)(?=\n##|\n###|$)/);

  if (!unusedKeysSection) {
    return;
  }

  const unusedKeyMatches = [...unusedKeysSection[1].matchAll(/- `([^`]+)`/g)];
  const unusedKeys = unusedKeyMatches.map((m) => m[1]);

  if (unusedKeys.length === 0) {
    return;
  }

  for (const filename of CONFIG.locales) {
    const filePath = join(CONFIG.localesDir, filename);
    const content = JSON.parse(readFileSync(filePath, "utf8"));

    for (const key of unusedKeys) {
      const parts = key.split(".");
      let hasPlaceholder = false;
      for (const part of parts) {
        if (part.endsWith("Placeholder")) {
          hasPlaceholder = true;
          break;
        }
      }
      if (hasPlaceholder) continue;

      removeNestedProperty(content, key);
    }

    writeFileSync(filePath, JSON.stringify(content, null, 2) + "\n", "utf8");
  }
}

async function fixPlaceholders(): Promise<void> {
  const foundIds = await scanSourceFiles();
  const usedWithLabelSuffix = new Set<string>();
  const usedWithoutLabelSuffix = new Set<string>();

  for (const [id, { kind }] of foundIds.entries()) {
    if (id.includes("Placeholder")) {
      if (id.endsWith(".label")) {
        usedWithLabelSuffix.add(id.replace(".label", ""));
      } else if (kind === "useLabel" || kind === "placeholderId") {
        usedWithoutLabelSuffix.add(id);
      }
    }
  }

  for (const filename of CONFIG.locales) {
    const filePath = join(CONFIG.localesDir, filename);
    const content = JSON.parse(readFileSync(filePath, "utf8"));
    let fixed = false;

    function processPlaceholders(obj: any, currentPath = ""): void {
      if (!obj || typeof obj !== "object") return;

      for (const key in obj) {
        const fullPath = currentPath ? `${currentPath}.${key}` : key;

        if (key.endsWith("Placeholder") && obj[key] && typeof obj[key] === "object" && obj[key].label) {
          const needsTripleNesting = usedWithLabelSuffix.has(fullPath);
          const needsDoubleNesting = usedWithoutLabelSuffix.has(fullPath);

          if (typeof obj[key].label === "string") {
            const stringValue = obj[key].label;
            if (needsTripleNesting) {
              obj[key].label = { label: { normal: stringValue, beginner: stringValue } };
            } else {
              obj[key].label = { normal: stringValue, beginner: stringValue };
            }
            fixed = true;
          } else if (typeof obj[key].label === "object") {
            const hasTripleNesting = obj[key].label.label && typeof obj[key].label.label === "object";
            const hasDoubleNesting = !hasTripleNesting && (obj[key].label.normal || obj[key].label.beginner);

            if (needsDoubleNesting && hasTripleNesting) {
              obj[key].label = obj[key].label.label;
              fixed = true;
            } else if (needsTripleNesting && hasDoubleNesting) {
              const existingValue = obj[key].label;
              obj[key].label = { label: existingValue };
              fixed = true;
            }
          }
        }

        if (typeof obj[key] === "object") {
          processPlaceholders(obj[key], fullPath);
        }
      }
    }

    processPlaceholders(content);

    if (fixed) {
      writeFileSync(filePath, JSON.stringify(content, null, 2) + "\n", "utf8");
    }
  }
}

function runFixMode(en: Translation, de: Translation, usedIds: Set<string>): { enFixed: number; deFixed: number } {
  let enFixed = 0;
  let deFixed = 0;
  const enKeys = flattenKeys(en);
  const deKeys = flattenKeys(de);

  for (const key of enKeys) {
    if (!deKeys.includes(key)) {
      const enValue = getNestedValue(en, key);
      if (typeof enValue === "string") {
        setNestedValue(de, key, enValue);
        deFixed++;
      }
    }
  }

  Array.from(usedIds).forEach((id) => {
    const labelKeyNormal = `${id}.label.normal`;
    const labelKeyBeginner = `${id}.label.beginner`;
    const enKeysFlat = flattenKeys(en);

    if (!enKeysFlat.includes(labelKeyNormal)) {
      const readable = extractReadableName(id);
      setNestedValue(en, labelKeyNormal, readable);
      setNestedValue(de, labelKeyNormal, readable);
      enFixed++;
      deFixed++;
    }

    if (!enKeysFlat.includes(labelKeyBeginner)) {
      const readable = extractReadableName(id);
      setNestedValue(en, labelKeyBeginner, readable);
      setNestedValue(de, labelKeyBeginner, readable);
      enFixed++;
      deFixed++;
    }
  });

  for (const key of deKeys) {
    if (!enKeys.includes(key)) {
      const deValue = getNestedValue(de, key);
      if (typeof deValue === "string") {
        deleteNestedValue(de, key);
        deFixed++;
      }
    }
  }

  const enPath = join(CONFIG.localesDir, "en.json");
  const dePath = join(CONFIG.localesDir, "de.json");
  writeFileSync(enPath, JSON.stringify(en, null, 2) + "\n", "utf8");
  writeFileSync(dePath, JSON.stringify(de, null, 2) + "\n", "utf8");

  return { enFixed, deFixed };
}
//#endregion Commands

//#region App Component
function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "warning" | "error">("running");
  const [errorCount, setErrorCount] = React.useState(0);
  const [warningCount, setWarningCount] = React.useState(0);
  const [fixedEn, setFixedEn] = React.useState(0);
  const [fixedDe, setFixedDe] = React.useState(0);
  const [message, setMessage] = React.useState("");

  React.useEffect(() => {
    const runCommand = async () => {
      const command = process.argv[2] || "validate";
      const fixMode = process.argv.includes("--fix");

      try {
        if (command === "validate") {
          await validate();
          const en = JSON.parse(readFileSync(join(CONFIG.localesDir, "en.json"), "utf8"));
          const de = JSON.parse(readFileSync(join(CONFIG.localesDir, "de.json"), "utf8"));
          const foundIds = await scanSourceFiles();
          const usedIds = new Set(foundIds.keys());
          const issues = validateTranslations(en, de, usedIds);
          const errors = issues.filter((i) => i.severity === "error");
          const warnings = issues.filter((i) => i.severity === "warning");

          const report = {
            timestamp: new Date().toISOString(),
            summary: {
              errors: errors.length,
              warnings: warnings.length,
              total: issues.length,
            },
            errors: errors,
            warnings: warnings,
            status: errors.length > 0 ? "error" : warnings.length > 0 ? "warning" : "success",
          };

          writeFileSync(reportPath, JSON.stringify(report, null, 2), "utf8");
          setErrorCount(errors.length);
          setWarningCount(warnings.length);
          setStatus(errors.length > 0 ? "error" : warnings.length > 0 ? "warning" : "success");
          process.exit(errors.length > 0 ? 1 : 0);
        } else if (command === "add") {
          await addMissing();
          setMessage("Added missing entries");
          setStatus("success");
          process.exit(0);
        } else if (command === "clean") {
          await cleanUnused();
          setMessage("Cleaned unused keys");
          setStatus("success");
          process.exit(0);
        } else if (command === "fix") {
          if (fixMode) {
            const en = JSON.parse(readFileSync(join(CONFIG.localesDir, "en.json"), "utf8"));
            const de = JSON.parse(readFileSync(join(CONFIG.localesDir, "de.json"), "utf8"));
            const foundIds = await scanSourceFiles();
            const usedIds = new Set(foundIds.keys());
            const { enFixed, deFixed } = runFixMode(en, de, usedIds);
            setFixedEn(enFixed);
            setFixedDe(deFixed);
          }
          await addMissing();
          await fixPlaceholders();
          await generateReport();
          await cleanUnused();
          setMessage("Fixed all issues");
          setStatus("success");
          process.exit(0);
        } else if (command === "report") {
          await generateReport();
          setMessage("Generated report");
          setStatus("success");
          process.exit(0);
        } else {
          setMessage(`Unknown command: ${command}. Available: validate, add, clean, fix, report`);
          setStatus("error");
          process.exit(1);
        }
      } catch (error: any) {
        setMessage(error.message || String(error));
        setStatus("error");
        process.exit(1);
      }
    };

    runCommand();
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🔍 Validating i18n translations...</Text>
      {(fixedEn > 0 || fixedDe > 0) && (
        <Text dimColor>
          🔧 Fixed {fixedEn} English entries, {fixedDe} German entries
        </Text>
      )}
      {message && <Text>{message}</Text>}
      {status === "running" && <Text>Loading...</Text>}
      {status === "success" && <Text color="green">✅ i18n validation passed</Text>}
      {status === "warning" && <Text color="yellow">⚠️ i18n validation completed with {warningCount} warnings</Text>}
      {status === "error" && (
        <Text color="red">
          ❌ i18n validation failed with {errorCount} errors, {warningCount} warnings
        </Text>
      )}
      {(status === "success" || status === "warning" || status === "error") && <Text dimColor>📝 Report written to {reportPath}</Text>}
    </Box>
  );
}

const command = process.argv[2];
if (command && !["validate", "add", "clean", "fix", "report"].includes(command)) {
  console.log("Unknown command:", command);
  console.log("Available commands: validate, add, clean, fix, report");
  process.exit(1);
}

render(<App />);
//#endregion App Component
