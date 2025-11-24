#!/usr/bin/env tsx
import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const localesDir = join(rootDir, "js", "js", "sketchpad", "locales");
const sketchpadDir = join(rootDir, "js", "js", "sketchpad");
const reportPath = join(rootDir, "agents", "i18n.md");

interface Translation {
  [key: string]: string | Translation;
}

interface Issue {
  severity: "error" | "warning";
  key: string;
  message: string;
  location?: string;
}

const issues: Issue[] = [];

//#region Load Translations
function loadTranslations(lang: string): Translation {
  const path = join(localesDir, `${lang}.json`);
  return JSON.parse(readFileSync(path, "utf-8"));
}

const en = loadTranslations("en");
const de = loadTranslations("de");
//#endregion

//#region Find All IDs in Source Files
function findIdsInFile(filePath: string): Set<string> {
  const content = readFileSync(filePath, "utf-8");
  const ids = new Set<string>();
  
  // Match id="..." or id='...' or id={...}
  const idPattern = /id\s*=\s*["'`{]([^"'`}]+)["'`}]/g;
  let match;
  while ((match = idPattern.exec(content)) !== null) {
    const id = match[1].trim();
    if (id.startsWith("semio.sketchpad.")) {
      ids.add(id);
    }
  }
  
  return ids;
}

function walkDir(dir: string, callback: (filePath: string) => void): void {
  const files = readdirSync(dir, { withFileTypes: true });
  for (const file of files) {
    const filePath = join(dir, file.name);
    if (file.isDirectory()) {
      walkDir(filePath, callback);
    } else if (file.name.endsWith(".tsx") || file.name.endsWith(".ts")) {
      callback(filePath);
    }
  }
}

const usedIds = new Set<string>();
walkDir(sketchpadDir, filePath => {
  const ids = findIdsInFile(filePath);
  ids.forEach(id => usedIds.add(id));
});
//#endregion

//#region Validate Translations
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

const enKeys = flattenKeys(en);
const deKeys = flattenKeys(de);

// Check for missing keys in German
for (const key of enKeys) {
  if (!deKeys.includes(key)) {
    issues.push({
      severity: "error",
      key,
      message: `Missing German translation for key: ${key}`,
    });
  }
}

// Check for extra keys in German
for (const key of deKeys) {
  if (!enKeys.includes(key)) {
    issues.push({
      severity: "warning",
      key,
      message: `Extra German translation key (not in English): ${key}`,
    });
  }
}

// Check for incomplete translations (same as English)
for (const key of enKeys) {
  const enValue = getNestedValue(en, key);
  const deValue = getNestedValue(de, key);
  if (typeof enValue === "string" && typeof deValue === "string" && enValue === deValue) {
    issues.push({
      severity: "warning",
      key,
      message: `Incomplete translation (same as English): ${key}`,
    });
  }
}

// Check for unused keys
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

// Check for missing translations for used IDs
Array.from(usedIds).forEach(id => {
  const labelKey = `${id}.label`;
  if (!enKeys.includes(labelKey)) {
    issues.push({
      severity: "error",
      key: labelKey,
      message: `Missing English translation for UI element: ${id}`,
    });
  }
});
//#endregion

//#region Generate Report
let report = "# i18n Validation Report\n\n";
report += `Generated: ${new Date().toISOString()}\n\n`;

const errors = issues.filter(i => i.severity === "error");
const warnings = issues.filter(i => i.severity === "warning");

report += `## Summary\n\n`;
report += `- **Errors**: ${errors.length}\n`;
report += `- **Warnings**: ${warnings.length}\n`;
report += `- **Total Issues**: ${issues.length}\n\n`;

if (errors.length > 0) {
  report += `## Errors\n\n`;
  for (const issue of errors) {
    report += `- **${issue.key}**: ${issue.message}\n`;
  }
  report += `\n`;
}

if (warnings.length > 0) {
  report += `## Warnings\n\n`;
  for (const issue of warnings) {
    report += `- **${issue.key}**: ${issue.message}\n`;
  }
  report += `\n`;
}

if (issues.length === 0) {
  report += `## ✅ All i18n checks passed!\n\n`;
}

writeFileSync(reportPath, report, "utf-8");
console.log(`📝 Report written to ${reportPath}`);
console.log(`\n${errors.length} errors, ${warnings.length} warnings`);

if (errors.length > 0) {
  process.exit(1);
}
//#endregion
