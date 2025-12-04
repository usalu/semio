#!/usr/bin/env node
/**
 * i18n Management Script
 *
 * This script validates, fixes, and manages i18n locale files.
 *
 * Commands:
 *   node i18n.mjs validate       - Validate locale files and generate report
 *   node i18n.mjs add            - Add missing i18n entries
 *   node i18n.mjs clean          - Remove unused keys
 *   node i18n.mjs fix            - Run full fix: add missing + clean unused
 *   node i18n.mjs report         - Generate markdown report
 */

import fs from "fs";
import globPkg from "glob";
const { glob } = globPkg;
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG = {
  localesDir: path.join(__dirname, "../js/js/sketchpad/locales"),
  sourceGlob: "js/js/sketchpad/**/*.{ts,tsx}",
  reportPath: path.join(__dirname, "../reports/i18n.json"),
  locales: ["de.json", "en.json"],
  metadataKeys: ["label", "hotkey", "manual", "tutorial", "beginner", "normal", "description"],
};

// Detection patterns for different usage types
const PATTERNS = [
  // id patterns - JSX attributes and object properties
  { kind: "id", regex: /\bid\s*=\s*["']([^"']+)["']/g },
  { kind: "id", regex: /\bid\s*=\s*\{["']([^"']+)["']\}/g },
  { kind: "id", regex: /\bid:\s*["']([^"']+)["']/g },

  // Other attributes
  { kind: "placeholderId", regex: /\bplaceholderId\s*=\s*["']([^"']+)["']/g },
  { kind: "i18nPressed", regex: /\bi18nPressed\s*=\s*["']([^"']+)["']/g },
  { kind: "actionId", regex: /\bactionId\s*=\s*["']([^"']+)["']/g },
  { kind: "actionId", regex: /\bactionId\s*=\s*\{["']([^"']+)["']\}/g },
  { kind: "tooltipId", regex: /\btooltipId\s*=\s*["']([^"']+)["']/g },
  { kind: "tooltipId", regex: /\btooltipId\s*=\s*\{["']([^"']+)["']\}/g },

  // Function calls
  { kind: "t", regex: /\bt\(\s*["']([^"']+)["']/g },
  { kind: "t", regex: /\bi18n\.t\(\s*["']([^"']+)["']/g },
  { kind: "useLabel", regex: /\buseLabel\(\s*["']([^"']+)["']/g },
  { kind: "useHotkey", regex: /\buseHotkey\(\s*["']([^"']+)["']/g },
  { kind: "useTooltip", regex: /\buseTooltip\(\s*["']([^"']+)["']/g },

  // Panel definitions and dynamic tooltip patterns
  { kind: "createPanelDefinition", regex: /createPanelDefinition\([^,]+,\s*["']([^"']+)["']/g },
  { kind: "panelToggleTooltip", regex: /semio\.sketchpad\.navbar\.panelToggle\.\$\{[^}]+\}\.\$\{[^}]+\s*\?\s*["']([^"']+)["']\s*:\s*["']([^"']+)["']/g },
];

// Utility functions
function getNestedProperty(obj, path) {
  const parts = path.split(".");
  let current = obj;
  for (const part of parts) {
    if (!current || typeof current !== "object") return null;
    current = current[part];
  }
  return current;
}

function setNestedProperty(obj, path, value) {
  const parts = path.split(".");
  let current = obj;
  for (let i = 0; i < parts.length - 1; i++) {
    if (!current[parts[i]]) current[parts[i]] = {};
    current = current[parts[i]];
  }
  current[parts[parts.length - 1]] = value;
  return true;
}

function removeNestedProperty(obj, path) {
  const parts = path.split(".");
  let current = obj;
  const parents = [];

  for (let i = 0; i < parts.length - 1; i++) {
    if (!current[parts[i]]) return false;
    parents.push({ obj: current, key: parts[i] });
    current = current[parts[i]];
  }

  const lastKey = parts[parts.length - 1];
  if (!current || !current.hasOwnProperty(lastKey)) return false;

  // Don't remove if it has non-metadata children
  if (typeof current[lastKey] === "object" && current[lastKey] !== null) {
    const keys = Object.keys(current[lastKey]);
    const hasOnlyMetadata = keys.every((k) => CONFIG.metadataKeys.includes(k));
    if (!hasOnlyMetadata && keys.length > 0) return false;
  }

  delete current[lastKey];

  // Clean up empty parent objects
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

function getAllKeys(obj, prefix = "") {
  const keys = [];
  if (!obj || typeof obj !== "object") return keys;

  for (const prop in obj) {
    const key = prefix ? `${prefix}.${prop}` : prop;
    if (CONFIG.metadataKeys.includes(prop)) continue;
    keys.push(key);
    if (typeof obj[prop] === "object" && obj[prop] !== null) {
      keys.push(...getAllKeys(obj[prop], key));
    }
  }
  return keys;
}

// ID Scanner
async function scanSourceFiles() {
  const globResult = glob(CONFIG.sourceGlob, { cwd: path.join(__dirname, ".."), absolute: true });
  const files = await new Promise((resolve, reject) => {
    const matches = [];
    globResult.on('match', (match) => matches.push(match));
    globResult.on('end', () => resolve(matches));
    globResult.on('error', reject);
  });
  const foundIds = new Map();

  for (const file of files) {
    const content = fs.readFileSync(file, "utf8");
    const relPath = path.relative(path.join(__dirname, ".."), file).replace(/\\/g, "/");

    for (const { kind, regex } of PATTERNS) {
      let match;
      while ((match = regex.exec(content)) !== null) {
        const id = match[1];
        // Accept IDs that start with semio.sketchpad., tooltip., or settings.
        if (!id.startsWith("semio.sketchpad.") && !id.startsWith("tooltip.") && !id.startsWith("settings.")) continue;

        if (!foundIds.has(id)) {
          foundIds.set(id, { kind, files: new Set() });
        }
        foundIds.get(id).files.add(relPath);
      }
    }

    // Detect dynamic panel toggle patterns like semio.sketchpad.navbar.panelToggle.{key}.{show|hide}
    const panelTogglePattern = /semio\.sketchpad\.navbar\.panelToggle\.\$\{[^}]+\}\.\$\{[^}]+\s*\?\s*["']([^"']+)["']\s*:\s*["']([^"']+)["']/g;
    let match;
    while ((match = panelTogglePattern.exec(content)) !== null) {
      // match[1] is 'hide', match[2] is 'show'
      const hideId = match[1];
      const showId = match[2];

      if (hideId.startsWith("semio.sketchpad.")) {
        if (!foundIds.has(hideId)) {
          foundIds.set(hideId, { kind: "panelToggleTooltip", files: new Set() });
        }
        foundIds.get(hideId).files.add(relPath);
      }

      if (showId.startsWith("semio.sketchpad.")) {
        if (!foundIds.has(showId)) {
          foundIds.set(showId, { kind: "panelToggleTooltip", files: new Set() });
        }
        foundIds.get(showId).files.add(relPath);
      }
    }

    // Detect inline tooltip construction patterns for panel toggles
    const inlineTooltipPattern = /`semio\.sketchpad\.navbar\.panelToggle\.([a-zA-Z0-9_]+)\.(show|hide)`/g;
    while ((match = inlineTooltipPattern.exec(content)) !== null) {
      const panelKey = match[1];
      const action = match[2];
      const id = `semio.sketchpad.navbar.panelToggle.${panelKey}.${action}`;

      if (!foundIds.has(id)) {
        foundIds.set(id, { kind: "inlineTooltip", files: new Set() });
      }
      foundIds.get(id).files.add(relPath);
    }
  }

  return foundIds;
}

// List of generic/non-specific terms that should not be used in section IDs
const GENERIC_SECTION_TERMS = ["title", "label", "name", "section", "info", "data", "item", "content"];

// Check if an ID uses generic terminology (returns warning message or null)
function checkGenericTerminology(id) {
  const parts = id.split(".");
  const lastPart = parts[parts.length - 1];

  // Skip metadata keys
  if (CONFIG.metadataKeys.includes(lastPart)) return null;

  // Check if the last part is a generic term
  if (GENERIC_SECTION_TERMS.includes(lastPart)) {
    return `Uses generic term '${lastPart}' - should be more specific (e.g., 'properties', 'metadata', 'details')`;
  }

  return null;
}

// Validation
function validateEntry(locale, id, kind) {
  const value = getNestedProperty(locale, id);

  if (!value) return { status: "Missing", details: "Key does not exist" };

  if (typeof value === "string") {
    return kind === "placeholderId" ? { status: "Valid", details: "String value" } : { status: "Warning", details: "Expected object with label/hotkey, found string" };
  }

  const issues = [];
  const hasLabel = value.label != null;

  if (hasLabel) {
    if (typeof value.label === "string") {
      issues.push("label is string (expected object with normal/beginner)");
    } else if (typeof value.label === "object") {
      const hasNormal = value.label.normal != null;
      const hasBeginner = value.label.beginner != null;
      if (!hasNormal && !hasBeginner) issues.push("label object is empty");
      if (hasNormal && value.label.normal === "") issues.push("label.normal is empty string");
      if (hasBeginner && value.label.beginner === "") issues.push("label.beginner is empty string");
    }
  } else {
    issues.push("missing label property");
  }

  if (issues.length > 0) {
    return { status: "Incomplete", details: issues.join(", ") };
  }

  return { status: "Valid", details: "OK" };
}

// Generate default label from key
function generateDefaultLabel(id) {
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

// Commands
async function validate() {
  console.log("🔍 Validating i18n setup...\n");

  console.log("📂 Loading locale files...");
  const locales = {};
  for (const filename of CONFIG.locales) {
    const filePath = path.join(CONFIG.localesDir, filename);
    locales[filename] = JSON.parse(fs.readFileSync(filePath, "utf8"));
    console.log(`  ✓ Loaded ${filename}`);
  }

  console.log("\n🔎 Scanning source files for UI element IDs...");
  const foundIds = await scanSourceFiles();
  console.log(`  ✓ Found ${foundIds.size} unique UI element IDs`);

  console.log("\n🔍 Validating locale entries...");
  const results = {};
  const genericTermWarnings = new Map();

  for (const filename of CONFIG.locales) {
    const langCode = filename.replace(".json", "");
    results[langCode] = { Valid: [], Incomplete: [], Warning: [], Missing: [] };

    for (const [id, { kind }] of foundIds) {
      const check = validateEntry(locales[filename], id, kind);
      if (!results[langCode][check.status]) {
        results[langCode][check.status] = [];
      }
      results[langCode][check.status].push({ id, kind, details: check.details, files: Array.from(foundIds.get(id).files) });

      // Check for generic terminology
      const genericWarning = checkGenericTerminology(id);
      if (genericWarning && !genericTermWarnings.has(id)) {
        genericTermWarnings.set(id, { warning: genericWarning, files: Array.from(foundIds.get(id).files) });
      }
    }

    const { Valid, Incomplete, Warning, Missing } = results[langCode];
    const total = Valid.length + Incomplete.length + Warning.length + Missing.length;

    console.log(`  ${filename}:`);
    console.log(`    ✓ Valid:      ${Valid.length} / ${total}`);
    if (Warning.length > 0) console.log(`    ⚠ Warning:   ${Warning.length} / ${total}`);
    if (Incomplete.length > 0) console.log(`    ⚠ Incomplete: ${Incomplete.length} / ${total}`);
    if (Missing.length > 0) console.log(`    ✗ Missing:    ${Missing.length} / ${total}`);
  }

  // Report generic terminology warnings
  if (genericTermWarnings.size > 0) {
    console.log(`\n⚠️  Generic Terminology Warnings: ${genericTermWarnings.size}`);
    for (const [id, { warning }] of genericTermWarnings) {
      console.log(`    ${id}: ${warning}`);
    }
  }

  console.log("\n🔍 Checking for unused locale keys...");
  const usedKeys = new Set(foundIds.keys());
  const unusedKeys = {};

  for (const filename of CONFIG.locales) {
    const allKeys = getAllKeys(locales[filename]);
    unusedKeys[filename] = allKeys.filter((key) => {
      if (usedKeys.has(key)) return false;
      // Keep placeholder parent keys
      for (const part of key.split(".")) {
        if (part.endsWith("Placeholder")) return false;
      }
      return true;
    });
    console.log(`  ${filename}: ${unusedKeys[filename].length} potentially unused keys`);
  }

  return { foundIds, results, unusedKeys, locales, genericTermWarnings };
}

async function generateReport() {
  console.log("\n📝 Generating report...");
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

  fs.writeFileSync(CONFIG.reportPath, report, "utf8");
  console.log(`  ✓ Report saved to ${path.relative(path.join(__dirname, ".."), CONFIG.reportPath)}`);
}

async function addMissing() {
  console.log("➕ Adding missing i18n entries...\n");

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

      let value;

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

    const filePath = path.join(CONFIG.localesDir, filename);
    fs.writeFileSync(filePath, JSON.stringify(locales[filename], null, 2) + "\n", "utf8");
    console.log(`  ✓ Added ${addedCount} entries to ${filename}`);
    totalAdded += addedCount;
  }

  console.log(`\n✅ Added ${totalAdded} total entries!`);
}

async function cleanUnused() {
  console.log("🧹 Cleaning unused i18n keys...\n");

  const report = fs.readFileSync(CONFIG.reportPath, "utf8");
  const unusedKeysSection = report.match(/#### Unused Locale Keys\s+([\s\S]*?)(?=\n##|\n###|$)/);

  if (!unusedKeysSection) {
    console.log("✅ No unused keys to remove");
    return;
  }

  const unusedKeyMatches = [...unusedKeysSection[1].matchAll(/- `([^`]+)`/g)];
  const unusedKeys = unusedKeyMatches.map((m) => m[1]);

  if (unusedKeys.length === 0) {
    console.log("✅ No unused keys to remove");
    return;
  }

  console.log(`Found ${unusedKeys.length} unused keys to remove`);

  for (const filename of CONFIG.locales) {
    const filePath = path.join(CONFIG.localesDir, filename);
    const content = JSON.parse(fs.readFileSync(filePath, "utf8"));
    let removedCount = 0;

    for (const key of unusedKeys) {
      // Don't remove keys under *Placeholder entries
      const parts = key.split(".");
      let hasPlaceholder = false;
      for (const part of parts) {
        if (part.endsWith("Placeholder")) {
          hasPlaceholder = true;
          break;
        }
      }
      if (hasPlaceholder) continue;

      if (removeNestedProperty(content, key)) {
        removedCount++;
      }
    }

    fs.writeFileSync(filePath, JSON.stringify(content, null, 2) + "\n", "utf8");
    console.log(`  ✓ Removed ${removedCount} keys from ${filename}`);
  }

  console.log("\n✅ Cleanup complete!");
}

async function fixPlaceholders() {
  console.log("🔧 Fixing placeholder structures...\n");

  const foundIds = await scanSourceFiles();
  const usedWithLabelSuffix = new Set();
  const usedWithoutLabelSuffix = new Set();

  // Categorize placeholder usage
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
    const filePath = path.join(CONFIG.localesDir, filename);
    const content = JSON.parse(fs.readFileSync(filePath, "utf8"));
    let fixed = false;

    function processPlaceholders(obj, currentPath = "") {
      if (!obj || typeof obj !== "object") return;

      for (const key in obj) {
        const fullPath = currentPath ? `${currentPath}.${key}` : key;

        if (key.endsWith("Placeholder") && obj[key] && typeof obj[key] === "object" && obj[key].label) {
          const needsTripleNesting = usedWithLabelSuffix.has(fullPath);
          const needsDoubleNesting = usedWithoutLabelSuffix.has(fullPath);

          if (typeof obj[key].label === "string") {
            // String -> needs wrapping
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
              // Unwrap triple to double
              obj[key].label = obj[key].label.label;
              fixed = true;
            } else if (needsTripleNesting && hasDoubleNesting) {
              // Wrap double to triple
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
      fs.writeFileSync(filePath, JSON.stringify(content, null, 2) + "\n", "utf8");
      console.log(`  ✓ Fixed ${filename}`);
    }
  }

  console.log("✅ Placeholder structures fixed!");
}

// Main
const command = process.argv[2] || "validate";

try {
  switch (command) {
    case "validate":
      await validate();
      break;
    case "add":
      await addMissing();
      break;
    case "clean":
      await cleanUnused();
      break;
    case "fix":
      await addMissing();
      await fixPlaceholders();
      await cleanUnused();
      await generateReport();
      break;
    case "report":
      await generateReport();
      break;
    default:
      console.log("Unknown command:", command);
      console.log("Available commands: validate, add, clean, fix, report");
      process.exit(1);
  }
} catch (error) {
  console.error("Error:", error);
  process.exit(1);
}
