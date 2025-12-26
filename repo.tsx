// #region Utils

import { ChildProcess, execSync, spawn } from "child_process";
import { readdirSync, readFileSync, renameSync, statSync, unlinkSync, writeFileSync } from "fs";
import { join } from "path";
import sharp from "sharp";

//#region Image Processing
export async function resizeImage(sourcePath: string, targetPathBase: string, targetResolutions: number[]): Promise<void> {
  for (const resolution of targetResolutions) {
    const targetPath = `${targetPathBase}_${resolution}x${resolution}.png`;
    await sharp(sourcePath).resize(resolution, resolution).png().toFile(targetPath);
  }
}
//#endregion Image Processing

//#region File Operations
export function renameFilesByPattern(pattern: RegExp, replacement: string, rootDir: string = "."): void {
  function walk(dir: string): void {
    const files = readdirSync(dir);
    for (const file of files) {
      const filePath = join(dir, file);
      const stat = statSync(filePath);
      if (stat.isDirectory()) {
        walk(filePath);
      } else {
        const newPath = filePath.replace(pattern, replacement);
        if (newPath !== filePath) {
          renameSync(filePath, newPath);
        }
      }
    }
  }
  walk(rootDir);
}

export function deleteFilesByPattern(pattern: string, rootDir: string = "."): void {
  function walk(dir: string): void {
    const files = readdirSync(dir);
    for (const file of files) {
      const filePath = join(dir, file);
      const stat = statSync(filePath);
      if (stat.isDirectory()) {
        walk(filePath);
      } else if (file.match(new RegExp(pattern))) {
        unlinkSync(filePath);
      }
    }
  }
  walk(rootDir);
}
//#endregion File Operations

//#region Process Management
export function stopProcessOnPort(connector: number): void {
  try {
    const output = execSync(`netstat -ano`, { encoding: "utf-8" });
    const lines = output.split("\n");
    for (const line of lines) {
      if (line.includes(`:${connector}`) && line.includes("LISTENING")) {
        const parts = line.trim().split(/\s+/);
        const pid = parts[parts.length - 1];
        if (pid && !isNaN(Number(pid))) {
          execSync(`taskkill /F /PID ${pid}`, { stdio: "ignore" });
          break;
        }
      }
    }
  } catch {
    // Process might not be running
  }
}

export function runProcess(command: string, args: string[], options?: { cwd?: string; onExit?: () => void }): ChildProcess {
  const proc = spawn(command, args, {
    cwd: options?.cwd,
    stdio: "inherit",
    shell: true,
  });

  if (options?.onExit) {
    proc.on("exit", options.onExit);
    process.on("SIGINT", () => {
      proc.kill();
      options.onExit?.();
    });
  }

  return proc;
}
//#endregion Process Management

//#region JSON Utilities
export function unescapeJson(inputPath: string, outputPath: string): void {
  const content = readFileSync(inputPath, "utf-8");
  const unescaped = content.replace(/\\(.)/g, "$1");
  writeFileSync(outputPath, unescaped, "utf-8");
}
//#endregion JSON Utilities

// #endregion Utils

// #region Prettier

import { Box, render, Text } from "ink";
import { dirname } from "path";
import React from "react";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");

function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "error">("running");

  React.useEffect(() => {
    try {
      execSync("npx prettier --ignore-path .prettierignore --write .", {
        cwd: rootDir,
        stdio: "inherit",
      });
      setStatus("success");
      setTimeout(() => process.exit(0), 100);
    } catch (error) {
      setStatus("error");
      setTimeout(() => process.exit(1), 100);
    }
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🎨 Formatting with Prettier...</Text>
      {status !== "running" && (
        <Text>
          {status === "success" ? "✅" : "❌"} Prettier formatting {status === "success" ? "complete" : "failed"}
        </Text>
      )}
    </Box>
  );
}

render(<App />);

// #endregion Prettier

// #region ESLint

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "eslint.json");
const nxArgs = process.argv.slice(2);

function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "error">("running");

  React.useEffect(() => {
    try {
      const output = execSync(["npx", "nx", "run-many", "-t", "lint", "--parallel=1", "--output-style=stream", ...nxArgs].join(" "), {
        cwd: rootDir,
        encoding: "utf-8",
      });

      const report = {
        timestamp: new Date().toISOString(),
        status: "success",
        output: output,
      };
      writeFileSync(reportPath, JSON.stringify(report, null, 2));

      setStatus("success");
      setTimeout(() => process.exit(0), 100);
    } catch (error: any) {
      const stderr = error.stderr?.toString() || "";
      const stdout = error.stdout?.toString() || "";

      const report = {
        timestamp: new Date().toISOString(),
        status: "error",
        stdout: stdout,
        stderr: stderr,
      };
      writeFileSync(reportPath, JSON.stringify(report, null, 2));

      setStatus("error");
      setTimeout(() => process.exit(1), 100);
    }
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🔍 Running ESLint...</Text>
      {status !== "running" && (
        <>
          <Text>
            {status === "success" ? "✅" : "❌"} ESLint check {status === "success" ? "passed" : "failed"}
          </Text>
          <Text dimColor>📝 Report: {reportPath}</Text>
        </>
      )}
    </Box>
  );
}

render(<App />);

// #endregion ESLint

// #region Ruff

import { existsSync } from "fs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const pyEngineDir = join(rootDir, "py", "engine");
const reportPath = join(rootDir, "reports", "ruff.json");

function App() {
  const [status, setStatus] = React.useState<"running" | "skipped" | "success" | "error">("running");

  React.useEffect(() => {
    if (!existsSync(pyEngineDir)) {
      setStatus("skipped");
      setTimeout(() => process.exit(0), 100);
      return;
    }

    try {
      execSync("ruff format .", {
        cwd: pyEngineDir,
        stdio: "inherit",
      });

      execSync("ruff check --fix .", {
        cwd: pyEngineDir,
        stdio: "inherit",
      });

      try {
        const output = execSync("ruff check --output-format=json .", {
          cwd: pyEngineDir,
          encoding: "utf-8",
        });
        const report = {
          timestamp: new Date().toISOString(),
          status: "success",
          issues: JSON.parse(output || "[]"),
        };
        writeFileSync(reportPath, JSON.stringify(report, null, 2));
      } catch (checkError: any) {
        const output = checkError.stdout?.toString() || "[]";
        const report = {
          timestamp: new Date().toISOString(),
          status: "warning",
          issues: JSON.parse(output || "[]"),
        };
        writeFileSync(reportPath, JSON.stringify(report, null, 2));
      }

      setStatus("success");
      setTimeout(() => process.exit(0), 100);
    } catch (error) {
      setStatus("error");
      setTimeout(() => process.exit(1), 100);
    }
  }, []);

  return (
    <Box flexDirection="column">
      {status === "skipped" ? (
        <Text dimColor>⚠️ Python engine directory not found, skipping Ruff</Text>
      ) : (
        <>
          <Text>🐍 Formatting and linting Python with Ruff...</Text>
          {status !== "running" && (
            <>
              <Text>
                {status === "success" ? "✅" : "❌"} Ruff formatting {status === "success" ? "complete" : "failed"}
              </Text>
              <Text dimColor>📝 Report: {reportPath}</Text>
            </>
          )}
        </>
      )}
    </Box>
  );
}

render(<App />);

// #endregion Ruff

// #region TypeScript

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "typescript.json");

function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "error">("running");

  React.useEffect(() => {
    try {
      execSync("npx tsc --noEmit --project tsconfig.json", {
        cwd: rootDir,
        encoding: "utf-8",
      });
      const report = {
        timestamp: new Date().toISOString(),
        status: "success",
        errors: [],
      };
      writeFileSync(reportPath, JSON.stringify(report, null, 2));
      setStatus("success");
      setTimeout(() => process.exit(0), 100);
    } catch (error: any) {
      const stderr = error.stderr?.toString() || "";
      const stdout = error.stdout?.toString() || "";
      const output = stdout || stderr;
      const report = {
        timestamp: new Date().toISOString(),
        status: "error",
        errors: output.split("\n").filter((line: string) => line.trim()),
      };
      writeFileSync(reportPath, JSON.stringify(report, null, 2));
      setStatus("error");
      setTimeout(() => process.exit(1), 100);
    }
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🔍 Running TypeScript compiler check...</Text>
      {status !== "running" && (
        <>
          <Text>
            {status === "success" ? "✅" : "❌"} TypeScript check {status === "success" ? "passed" : "failed"}
          </Text>
          <Text dimColor>📝 Report: {reportPath}</Text>
        </>
      )}
    </Box>
  );
}

render(<App />);

// #endregion TypeScript

// #region I18n

import { glob } from "glob";
import { relative } from "path";

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

// #endregion I18n

// #region Rename Files

import { useApp } from "ink";
import { createInterface } from "readline";

const RED = "\x1B[31m";
const GREEN = "\x1B[32m";
const STRIKETHROUGH = "\x1B[9m";
const RESET = "\x1B[0m";
const INVERSE = "\x1B[7m";
const DIM = "\x1B[2m";

interface FileMatch {
  path: string;
  newPath: string;
  selected: boolean;
  oldFileName: string;
  newFileName: string;
  dirPath: string;
}

function casePreservingReplace(original: string, replacement: string): string {
  if (original === original.toUpperCase()) {
    return replacement.toUpperCase();
  } else if (original[0] === original[0].toUpperCase() && original.slice(1) === original.slice(1).toLowerCase()) {
    return replacement.charAt(0).toUpperCase() + replacement.slice(1).toLowerCase();
  } else {
    return replacement;
  }
}

function findFiles(pattern: string, replacement: string, rootDir: string, ignoreDirs: string[]): FileMatch[] {
  const matches: FileMatch[] = [];

  const regex = new RegExp(pattern, "gi");

  function walk(dir: string): void {
    const files = readdirSync(dir);
    for (const file of files) {
      const filePath = join(dir, file);
      const stat = statSync(filePath);
      if (stat.isDirectory()) {
        if (!ignoreDirs.some((ignored) => file === ignored || filePath.includes(ignored))) {
          walk(filePath);
        }
      } else {
        regex.lastIndex = 0;
        if (regex.test(file)) {
          regex.lastIndex = 0;

          const newFileName = file.replace(regex, (match) => casePreservingReplace(match, replacement));
          const newPath = join(dirname(filePath), newFileName);
          matches.push({
            path: filePath,
            newPath,
            selected: true,
            oldFileName: file,
            newFileName,
            dirPath: relative(rootDir, dirname(filePath)),
          });
        }
      }
    }
  }

  walk(rootDir);
  return matches;
}

function clearScreen(): void {
  process.stdout.write("\x1B[2J\x1B[0f");
}

function formatRename(oldName: string, pattern: string, replacement: string): string {
  const regex = new RegExp(pattern, "gi");
  let result = "";
  let lastIndex = 0;

  regex.lastIndex = 0;
  let match;
  while ((match = regex.exec(oldName)) !== null) {
    result += oldName.slice(lastIndex, match.index);

    const preservedReplacement = casePreservingReplace(match[0], replacement);

    result += `${RED}${STRIKETHROUGH}${match[0]}${RESET}${GREEN}${preservedReplacement}${RESET}`;
    lastIndex = match.index + match[0].length;
  }

  result += oldName.slice(lastIndex);

  return result;
}

function renderList(matches: FileMatch[], cursor: number, pattern: string, replacement: string, pageSize: number = 20): void {
  clearScreen();
  console.log("\x1B[1mRename Files Interactive\x1B[0m");
  console.log("─".repeat(80));
  console.log("Controls: ↑/↓ navigate | Space toggle | a select all | n select none | Enter confirm | q quit");
  console.log("─".repeat(80));

  const selectedCount = matches.filter((m) => m.selected).length;
  console.log(`Selected: ${selectedCount}/${matches.length} files\n`);

  const halfPage = Math.floor(pageSize / 2);
  let startIdx = Math.max(0, cursor - halfPage);
  let endIdx = Math.min(matches.length, startIdx + pageSize);
  if (endIdx - startIdx < pageSize) {
    startIdx = Math.max(0, endIdx - pageSize);
  }

  if (startIdx > 0) {
    console.log(`${DIM}  ... ${startIdx} more above${RESET}`);
  }

  for (let i = startIdx; i < endIdx; i++) {
    const match = matches[i];
    const isCursor = i === cursor;
    const checkbox = match.selected ? "✓" : " ";
    const checkboxColor = match.selected ? GREEN : DIM;

    const formattedName = formatRename(match.oldFileName, pattern, replacement);
    const dirPrefix = match.dirPath ? `${DIM}${match.dirPath}/${RESET}` : "";

    const line = `${checkboxColor}[${checkbox}]${RESET} ${dirPrefix}${formattedName}`;

    if (isCursor) {
      console.log(`${INVERSE}→${RESET} ${line}`);
    } else {
      console.log(`  ${line}`);
    }
  }

  if (endIdx < matches.length) {
    console.log(`${DIM}  ... ${matches.length - endIdx} more below${RESET}`);
  }

  console.log();
  console.log("─".repeat(80));
}

async function interactiveSelect(matches: FileMatch[], pattern: string, replacement: string): Promise<FileMatch[]> {
  return new Promise((resolve) => {
    if (matches.length === 0) {
      console.log("No files found matching the pattern.");
      resolve([]);
      return;
    }

    let cursor = 0;

    const rl = createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    if (process.stdin.isTTY) {
      process.stdin.setRawMode(true);
    }
    process.stdin.resume();

    renderList(matches, cursor, pattern, replacement);

    process.stdin.on("data", (key: Buffer) => {
      const keyStr = key.toString();

      if (keyStr === "\u0003" || keyStr === "q" || keyStr === "Q") {
        if (process.stdin.isTTY) {
          process.stdin.setRawMode(false);
        }
        rl.close();
        console.log("\nCancelled.");
        process.exit(0);
      } else if (keyStr === "\u001B[A") {
        cursor = Math.max(0, cursor - 1);
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === "\u001B[B") {
        cursor = Math.min(matches.length - 1, cursor + 1);
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === " ") {
        matches[cursor].selected = !matches[cursor].selected;
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === "\r" || keyStr === "\n") {
        if (process.stdin.isTTY) {
          process.stdin.setRawMode(false);
        }
        rl.close();
        resolve(matches.filter((m) => m.selected));
      } else if (keyStr === "a" || keyStr === "A") {
        matches.forEach((m) => (m.selected = true));
        renderList(matches, cursor, pattern, replacement);
      } else if (keyStr === "n" || keyStr === "N") {
        matches.forEach((m) => (m.selected = false));
        renderList(matches, cursor, pattern, replacement);
      }
    });
  });
}

function RenameProgress({ files }: { files: FileMatch[] }) {
  const [completed, setCompleted] = React.useState(0);
  const [errors, setErrors] = React.useState<string[]>([]);
  const { exit } = useApp();

  React.useEffect(() => {
    let current = 0;
    for (const file of files) {
      try {
        renameSync(file.path, file.newPath);
        current++;
        setCompleted(current);
      } catch (error) {
        setErrors((prev) => [...prev, `Failed to rename ${file.path}: ${error}`]);
        current++;
        setCompleted(current);
      }
    }
    setTimeout(() => exit(), 100);
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🔧 Renaming files...</Text>
      {completed > 0 && (
        <Text>
          {completed === files.length && errors.length === 0 ? "✅" : "⚠️"} {completed}/{files.length} files
        </Text>
      )}
      {errors.length > 0 && (
        <Box flexDirection="column" marginTop={1}>
          {errors.slice(0, 5).map((err, i) => (
            <Text key={i} color="red" dimColor>
              {err}
            </Text>
          ))}
          {errors.length > 5 && <Text dimColor>... and {errors.length - 5} more errors</Text>}
        </Box>
      )}
    </Box>
  );
}

async function main(): Promise<void> {
  const args = process.argv.slice(2);

  if (args.length < 2 || args.includes("--help") || args.includes("-h")) {
    const usage = `
Usage: npx tsx scripts/rename-files.tsx <pattern> <replacement> [rootDir] [--ignore dir1,dir2]

Arguments:
  pattern      - Regex pattern to search for in file names (case-insensitive)
  replacement  - String to replace matches with (case is preserved automatically)
  rootDir      - Root directory to search (default: current directory)
  --ignore     - Comma-separated list of directories to ignore

Case Preservation:
  port → connector, Port → Connector, PORT → CONNECTOR

Examples:
  npx tsx scripts/rename-files.tsx "port" "connector" .
  npx tsx scripts/rename-files.tsx "port" "connector" ./src --ignore node_modules,dist

  # Use lookbehind/lookahead to exclude certain matches:
  # Match "port" but not in "export", "import", "report", "portion", "portal", "portable"
  npx tsx scripts/rename-files.tsx "(?<![xXpPeEmMsSoOwWrRnN-])port(?!ion)(?!al)(?!able)" "connector" .

Controls:
  ↑/↓    - Navigate through files
  Space  - Toggle file selection
  a      - Select all
  n      - Select none
  Enter  - Confirm and rename selected files
  q      - Quit without renaming

Display:
  ${RED}${STRIKETHROUGH}old${RESET}${GREEN}new${RESET} - Red strikethrough shows removed text, green shows new text
`;
    render(
      <Box flexDirection="column">
        <Text>{usage}</Text>
      </Box>,
    );
    process.exit(0);
  }

  const pattern = args[0];
  const replacement = args[1];
  const rootDir = args[2] && !args[2].startsWith("--") ? args[2] : ".";

  let ignoreDirs = ["node_modules", ".git", "dist", "build", "__pycache__", ".venv", "bin", "obj"];
  const ignoreIndex = args.indexOf("--ignore");
  if (ignoreIndex !== -1 && args[ignoreIndex + 1]) {
    ignoreDirs = args[ignoreIndex + 1].split(",");
  }

  try {
    new RegExp(pattern);
  } catch (e) {
    render(
      <Box flexDirection="column">
        <Text color="red">Invalid regex pattern: {pattern}</Text>
        <Text color="red" dimColor>
          {(e as Error).message}
        </Text>
      </Box>,
    );
    process.exit(1);
  }

  render(
    <Box flexDirection="column">
      <Text>
        🔍 Searching for files matching "{pattern}" to replace with "{replacement}"...
      </Text>
      <Text dimColor>Root directory: {rootDir}</Text>
      <Text dimColor>Ignoring: {ignoreDirs.join(", ")}</Text>
    </Box>,
  );

  const matches = findFiles(pattern, replacement, rootDir, ignoreDirs);

  if (matches.length === 0) {
    render(
      <Box flexDirection="column">
        <Text>No files found matching the pattern.</Text>
      </Box>,
    );
    process.exit(0);
  }

  const selectedFiles = await interactiveSelect(matches, pattern, replacement);

  if (selectedFiles.length === 0) {
    render(
      <Box flexDirection="column">
        <Text>No files selected. Exiting.</Text>
      </Box>,
    );
    process.exit(0);
  }

  render(<RenameProgress files={selectedFiles} />);
}

main().catch(console.error);

// #endregion Rename Files

// #region Schema

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const reportsDir = join(rootDir, "reports");
const engineeringDir = join(rootDir, "engineering");

const semioTsPath = join(rootDir, "js", "js", "semio.ts");
const enginePyPath = join(rootDir, "py", "engine", "engine.py");
const semioCsPath = join(rootDir, "net", "Semio", "Semio.cs");
const grasshopperCsPath = join(rootDir, "net", "Semio.Grasshopper", "Semio.Grasshopper.cs");
const dataArchPath = join(engineeringDir, "dataarchitecture.pu");
const interfaceArchPath = join(engineeringDir, "interfacearchitecture.txt");
const softwareArchPath = join(engineeringDir, "softwarearchitecture.pu");

// #region Types

interface EntityField {
  name: string;
  type: string;
  optional: boolean;
  description?: string;
}

interface Entity {
  name: string;
  fields: EntityField[];
  hasId: boolean;
  hasDiff: boolean;
  hasDiffs: boolean;
}

interface GrasshopperComponent {
  name: string;
  nickname: string;
  inputs: EntityField[];
  outputs: EntityField[];
}

interface SchemaReport {
  timestamp: string;
  entities: Entity[];
  idTypes: string[];
  weakEntities: string[];
}

interface GrasshopperReport {
  timestamp: string;
  components: GrasshopperComponent[];
  params: string[];
  goos: string[];
}

interface Issue {
  severity: "error" | "warning";
  entity: string;
  field?: string;
  message: string;
  source: string;
}

interface DatabaseField {
  name: string;
  type: string;
  constraints: string[];
}

interface DatabaseEntity {
  name: string;
  fields: DatabaseField[];
}

interface DatabaseRelationship {
  from: string;
  to: string;
  cardinality: string;
  label: string;
}

interface DatabaseReport {
  timestamp: string;
  entities: DatabaseEntity[];
  relationships: DatabaseRelationship[];
}

interface InterfaceField {
  name: string;
  type: string;
  required: boolean;
  isArray: boolean;
  nested?: InterfaceEntity;
}

interface InterfaceEntity {
  name: string;
  fields: InterfaceField[];
}

interface InterfaceReport {
  timestamp: string;
  rootEntity: InterfaceEntity;
}

interface ClassField {
  name: string;
  type: string;
}

interface ClassEntity {
  name: string;
  kind: "class" | "interface" | "enum";
  fields: ClassField[];
  enumValues?: string[];
}

interface ClassRelationship {
  from: string;
  to: string;
  type: string;
  cardinality: string;
}

interface SoftwareReport {
  timestamp: string;
  entities: ClassEntity[];
  relationships: ClassRelationship[];
}

// #endregion Types

const MAIN_ENTITIES = ["Attribute", "Location", "Author", "File", "Folder", "Benchmark", "Quality", "Interface", "Prop", "Model", "Connector", "Tag", "Concept", "Type", "Layer", "Piece", "Group", "Connection", "Stat", "Design", "Kit"];

const WEAK_ENTITIES = ["Coord", "Vec", "Point", "Vector", "Plane", "Range", "Side"];

// #region TypeScript Parser

function parseTypeScriptSchema(): SchemaReport {
  const content = readFileSync(semioTsPath, "utf-8");
  const entities: Entity[] = [];
  const idTypes: string[] = [];
  const weakEntities: string[] = [];

  const idTypeRegex = /export type (\w+Id) = \{ guid: Guid \}/g;
  let match;
  while ((match = idTypeRegex.exec(content)) !== null) {
    idTypes.push(match[1]);
  }

  for (const entityName of [...MAIN_ENTITIES, ...WEAK_ENTITIES]) {
    const schemaRegex = new RegExp(`export const ${entityName}Schema = z\\.object\\(\\{([\\s\\S]*?)\\}\\)`, "m");
    const schemaMatch = content.match(schemaRegex);

    if (schemaMatch) {
      const fieldsStr = schemaMatch[1];
      const fields = parseZodFields(fieldsStr);
      const hasDiff = content.includes(`export const ${entityName}DiffSchema`);
      const hasDiffs = content.includes(`export const ${entityName}sDiffSchema`);
      const hasId = idTypes.includes(`${entityName}Id`);

      const isWeak = WEAK_ENTITIES.includes(entityName);
      if (isWeak) {
        weakEntities.push(entityName);
      }

      entities.push({ name: entityName, fields, hasId, hasDiff, hasDiffs });
    }
  }

  return {
    timestamp: new Date().toISOString(),
    entities,
    idTypes,
    weakEntities,
  };
}

function parseZodFields(fieldsStr: string): EntityField[] {
  const fields: EntityField[] = [];
  const lines = fieldsStr.split("\n");
  for (const line of lines) {
    const fieldMatch = line.match(/^\s*(\w+):\s*(.+?),?\s*$/);
    if (fieldMatch) {
      const name = fieldMatch[1];
      const typeStr = fieldMatch[2];
      const optional = typeStr.includes(".optional()");
      const type = inferZodType(typeStr);
      fields.push({ name, type, optional });
    }
  }
  return fields;
}

function inferZodType(zodStr: string): string {
  if (zodStr.includes("z.string()")) return "string";
  if (zodStr.includes("z.number()")) return "number";
  if (zodStr.includes("z.boolean()")) return "boolean";
  if (zodStr.includes("z.array(")) {
    const innerMatch = zodStr.match(/z\.array\((\w+)Schema/);
    if (innerMatch) return `${innerMatch[1]}[]`;
    return "array";
  }
  const schemaMatch = zodStr.match(/(\w+)Schema/);
  if (schemaMatch) return schemaMatch[1];
  return "unknown";
}

// #endregion TypeScript Parser

// #region Python Parser

function parsePythonSchema(): SchemaReport {
  const content = readFileSync(enginePyPath, "utf-8");
  const entities: Entity[] = [];
  const idTypes: string[] = [];
  const weakEntities: string[] = [];

  const idClassRegex = /class (\w+Id)\([^)]*\):/g;
  let match;
  while ((match = idClassRegex.exec(content)) !== null) {
    if (!match[1].endsWith("IdId")) {
      idTypes.push(match[1]);
    }
  }

  for (const entityName of MAIN_ENTITIES) {
    const fields: EntityField[] = [];

    const fieldClassRegex = new RegExp(`class ${entityName}\\w*Field\\([^)]*\\):([\\s\\S]*?)(?=\\nclass |\\n# )`, "gm");
    while ((match = fieldClassRegex.exec(content)) !== null) {
      const classBody = match[1];
      const classFields = parsePythonSqlmodelFields(classBody);
      for (const f of classFields) {
        if (!fields.some((ef) => ef.name === f.name)) {
          fields.push(f);
        }
      }
    }

    const propsRegex = new RegExp(`class ${entityName}Props\\([^)]*\\):([\\s\\S]*?)(?=\\nclass |\\n# )`, "m");
    const propsMatch = content.match(propsRegex);
    if (propsMatch) {
      const propsFields = parsePythonSqlmodelFields(propsMatch[1]);
      for (const f of propsFields) {
        if (!fields.some((ef) => ef.name === f.name)) {
          fields.push(f);
        }
      }
    }

    const mainClassRegex = new RegExp(`class ${entityName}\\([^)]*(?:TableEntity|table=True)[^)]*\\):`, "m");
    if (mainClassRegex.test(content)) {
      if (!fields.some((f) => f.name === "guid")) {
        fields.unshift({ name: "guid", type: "string", optional: false });
      }

      const hasDiff = content.includes(`class ${entityName}Diff`);
      const hasDiffs = content.includes(`class ${entityName}sDiff`);
      const hasId = idTypes.includes(`${entityName}Id`);

      entities.push({ name: entityName, fields, hasId, hasDiff, hasDiffs });
    }
  }

  for (const weakName of WEAK_ENTITIES) {
    const classRegex = new RegExp(`class ${weakName}\\([^)]*\\):([\\s\\S]*?)(?=\\nclass |\\n# )`, "m");
    const classMatch = content.match(classRegex);
    if (classMatch) {
      const fields = parsePythonSqlmodelFields(classMatch[1]);
      weakEntities.push(weakName);
      entities.push({ name: weakName, fields, hasId: false, hasDiff: false, hasDiffs: false });
    }
  }

  return {
    timestamp: new Date().toISOString(),
    entities,
    idTypes,
    weakEntities,
  };
}

function parsePythonSqlmodelFields(classBody: string): EntityField[] {
  const fields: EntityField[] = [];

  const fieldRegex = /^\s{4}(\w+):\s*([^=\n]+?)(?:\s*=.*)?$/gm;
  let match;
  while ((match = fieldRegex.exec(classBody)) !== null) {
    const name = match[1];
    if (name.startsWith("_") || name === "PLURAL" || name === "pk" || name === "Meta") continue;
    const typeStr = match[2].trim();
    const optional = typeStr.includes("Optional") || typeStr.includes("| None") || typeStr.includes("None |");
    const type = inferPythonType(typeStr);
    fields.push({ name, type, optional });
  }
  return fields;
}

function inferPythonType(typeStr: string): string {
  let type = typeStr
    .replace(/typing\.Optional\[([^\]]+)\]/, "$1")
    .replace(/Optional\[([^\]]+)\]/, "$1")
    .replace(/\s*\|\s*None/, "")
    .replace(/None\s*\|/, "")
    .trim();

  if (type === "str") return "string";
  if (type === "int" || type === "float") return "number";
  if (type === "bool") return "boolean";
  if (type.match(/(?:typing\.)?[Ll]ist\[/)) {
    const inner = type.match(/(?:typing\.)?[Ll]ist\[([^\]]+)\]/);
    if (inner) return `${inferPythonType(inner[1])}[]`;
  }
  return type;
}

// #endregion Python Parser

// #region C# Parser

function parseCSharpSchema(): SchemaReport {
  const content = readFileSync(semioCsPath, "utf-8");
  const entities: Entity[] = [];
  const idTypes: string[] = [];
  const weakEntities: string[] = [];

  const idClassRegex = /public\s+class\s+(\w+Id)\s*:/g;
  let match;
  while ((match = idClassRegex.exec(content)) !== null) {
    idTypes.push(match[1]);
  }

  for (const entityName of MAIN_ENTITIES) {
    const classRegex = new RegExp(`\\[Entity\\([^\\]]+\\)\\]\\s*public\\s+class\\s+${entityName}\\s*:[^{]*\\{`, "m");
    const classStart = content.match(classRegex);

    if (classStart && classStart.index !== undefined) {
      const startIdx = classStart.index + classStart[0].length;
      const body = extractClassBody(content, startIdx);
      const fields = parseCSharpFields(body);

      const hasDiff = content.includes(`public class ${entityName}Diff`) || content.includes(`public class ${entityName}Diff :`);
      const hasDiffs = content.includes(`public class ${entityName}sDiff`) || content.includes(`public class ${entityName}sDiff :`);
      const hasId = idTypes.includes(`${entityName}Id`);

      entities.push({ name: entityName, fields, hasId, hasDiff, hasDiffs });
    }
  }

  for (const weakName of WEAK_ENTITIES) {
    const classRegex = new RegExp(`public\\s+class\\s+${weakName}\\s*:[^{]*\\{`, "m");
    const classStart = content.match(classRegex);

    if (classStart && classStart.index !== undefined) {
      const startIdx = classStart.index + classStart[0].length;
      const body = extractClassBody(content, startIdx);
      const fields = parseCSharpFields(body);
      weakEntities.push(weakName);
      entities.push({ name: weakName, fields, hasId: false, hasDiff: false, hasDiffs: false });
    }
  }

  return {
    timestamp: new Date().toISOString(),
    entities,
    idTypes,
    weakEntities,
  };
}

function extractClassBody(content: string, startIdx: number): string {
  let depth = 1;
  let endIdx = startIdx;
  while (depth > 0 && endIdx < content.length) {
    if (content[endIdx] === "{") depth++;
    if (content[endIdx] === "}") depth--;
    endIdx++;
  }
  return content.substring(startIdx, endIdx - 1);
}

function parseCSharpFields(body: string): EntityField[] {
  const fields: EntityField[] = [];

  const propRegex = /public\s+([\w<>]+)\??\s+(\w+)\s*\{\s*get;\s*set;\s*\}/g;
  let match;
  while ((match = propRegex.exec(body)) !== null) {
    let typeStr = match[1];
    const name = match[2];
    const fullMatch = match[0];
    const optional = fullMatch.includes(typeStr + "?") || body.includes(`${typeStr}? ${name}`);
    typeStr = typeStr.replace(/\?$/, "");
    const type = inferCSharpType(typeStr);
    fields.push({ name, type, optional });
  }
  return fields;
}

function inferCSharpType(typeStr: string): string {
  if (typeStr === "string") return "string";
  if (typeStr === "int" || typeStr === "float" || typeStr === "double") return "number";
  if (typeStr === "bool") return "boolean";
  if (typeStr.startsWith("List<")) {
    const inner = typeStr.match(/List<([^>]+)>/);
    if (inner) return `${inferCSharpType(inner[1])}[]`;
  }
  return typeStr;
}

// #endregion C# Parser

// #region Grasshopper Parser

function parseGrasshopperSchema(): GrasshopperReport {
  const content = readFileSync(grasshopperCsPath, "utf-8");
  const components: GrasshopperComponent[] = [];
  const params: string[] = [];
  const goos: string[] = [];

  const paramRegex = /public\s+class\s+(\w+Param)\s*:/g;
  let match;
  while ((match = paramRegex.exec(content)) !== null) {
    params.push(match[1]);
  }

  const gooRegex = /public\s+class\s+(\w+Goo)\s*:/g;
  while ((match = gooRegex.exec(content)) !== null) {
    goos.push(match[1]);
  }

  const componentRegex = /public\s+class\s+(\w+)Component\s*:[^{]+ModelComponent[^{]*\{[\s\S]*?ModelName\s*=>\s*"(\w+)"[\s\S]*?ModelNickname\s*=>\s*"(\w+)"/g;
  while ((match = componentRegex.exec(content)) !== null) {
    const className = match[1] + "Component";
    const modelName = match[2];
    const nickname = match[3];

    const inputs = extractGrasshopperParams(content, className, "Input");
    const outputs = extractGrasshopperParams(content, className, "Output");

    components.push({
      name: modelName,
      nickname,
      inputs,
      outputs,
    });
  }

  return {
    timestamp: new Date().toISOString(),
    components,
    params,
    goos,
  };
}

function extractGrasshopperParams(content: string, className: string, paramType: "Input" | "Output"): EntityField[] {
  const fields: EntityField[] = [];

  const classRegex = new RegExp(`class\\s+${className}[^{]*\\{([\\s\\S]*?)\\n\\}\\s*\\n`, "m");
  const classMatch = content.match(classRegex);
  if (!classMatch) return fields;

  const classBody = classMatch[1];

  const methodRegex = new RegExp(`RegisterModel${paramType}Params[^{]*\\{([\\s\\S]*?)\\n    \\}`, "m");
  const methodMatch = classBody.match(methodRegex);
  if (!methodMatch) return fields;

  const methodBody = methodMatch[1];

  const textParamRegex = /AddTextParameter\s*\(\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)"/g;
  let match;
  while ((match = textParamRegex.exec(methodBody)) !== null) {
    const name = match[1];
    const nickname = match[2];
    const description = match[3];
    const optional = nickname.includes("?");
    fields.push({ name, type: "string", optional, description });
  }

  const boolParamRegex = /AddBooleanParameter\s*\(\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)"/g;
  while ((match = boolParamRegex.exec(methodBody)) !== null) {
    const name = match[1];
    const nickname = match[2];
    const description = match[3];
    const optional = nickname.includes("?");
    fields.push({ name, type: "boolean", optional, description });
  }

  const typedParamRegex = /AddParameter\s*\(\s*new\s+(\w+)Param\(\)[^,]*,\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)"/g;
  while ((match = typedParamRegex.exec(methodBody)) !== null) {
    const paramType = match[1];
    const name = match[2];
    const nickname = match[3];
    const description = match[4];
    const optional = nickname.includes("?") || nickname.includes("*");
    fields.push({ name, type: paramType, optional, description });
  }

  return fields;
}

// #endregion Grasshopper Parser

// #region Database Architecture Parser

function parseDatabaseArchitecture(): DatabaseReport {
  const content = readFileSync(dataArchPath, "utf-8");
  const entities: DatabaseEntity[] = [];
  const relationships: DatabaseRelationship[] = [];

  const entityRegex = /entity\s+(\w+)\s*\{([^}]+)\}/g;
  let match;
  while ((match = entityRegex.exec(content)) !== null) {
    const entityName = match[1];
    const fieldsStr = match[2];
    const fields = parseDatabaseFields(fieldsStr);
    entities.push({ name: entityName, fields });
  }

  const relRegex = /(\w+)\s+([\|\}o][\|\-][o\|\-\{][\|\-]o[\|\-]?\{?)\s+(\w+)\s*:\s*"([^"]+)"/g;
  while ((match = relRegex.exec(content)) !== null) {
    const from = match[1];
    const cardinality = match[2];
    const to = match[3];
    const label = match[4];
    relationships.push({ from, to, cardinality, label });
  }

  return {
    timestamp: new Date().toISOString(),
    entities,
    relationships,
  };
}

function parseDatabaseFields(fieldsStr: string): DatabaseField[] {
  const fields: DatabaseField[] = [];
  const lines = fieldsStr.split("\n");

  for (const line of lines) {
    const fieldMatch = line.match(/^\s*(\*?)(\w+)\s*:\s*([^\s<]+)(?:\s*<<([^>]+)>>)?/);
    if (fieldMatch) {
      const isRequired = fieldMatch[1] === "*";
      const name = fieldMatch[2];
      const type = fieldMatch[3];
      const constraintStr = fieldMatch[4] || "";

      const constraints: string[] = [];
      if (isRequired) constraints.push("NOT NULL");
      if (constraintStr) {
        constraints.push(...constraintStr.split(",").map((c) => c.trim()));
      }

      fields.push({ name, type, constraints });
    }
  }

  return fields;
}

// #endregion Database Architecture Parser

// #region Interface Architecture Parser

function parseInterfaceArchitecture(): InterfaceReport {
  const content = readFileSync(interfaceArchPath, "utf-8");
  const rootEntity = parseInterfaceEntity(content, 0);

  return {
    timestamp: new Date().toISOString(),
    rootEntity,
  };
}

function parseInterfaceEntity(content: string, startIndex: number): InterfaceEntity {
  const lines = content.split("\n");
  const fields: InterfaceField[] = [];
  let entityName = "Kit";

  for (let i = startIndex; i < lines.length; i++) {
    const line = lines[i];
    if (!line.trim() || line.trim().startsWith("//")) continue;

    const fieldMatch = line.match(/^\s{4}(\w+)\s*:\s*(!|\?|\*|\+)(\w+)(\[)?/);
    if (fieldMatch) {
      const name = fieldMatch[1];
      const requiredMarker = fieldMatch[2];
      const typeName = fieldMatch[3];
      const isArray = fieldMatch[4] !== undefined;
      const required = requiredMarker === "!" || requiredMarker === "+";

      fields.push({
        name,
        type: typeName,
        required,
        isArray,
      });
    }

    if (i === startIndex) {
      const nameMatch = line.match(/^(\w+)\s*:/);
      if (nameMatch) {
        entityName = nameMatch[1];
      }
    }
  }

  return { name: entityName, fields };
}

// #endregion Interface Architecture Parser

// #region Software Architecture Parser

function parseSoftwareArchitecture(): SoftwareReport {
  const content = readFileSync(softwareArchPath, "utf-8");
  const entities: ClassEntity[] = [];
  const relationships: ClassRelationship[] = [];

  const classRegex = /(?:class|interface)\s+(\w+)\s*\{([^}]+)\}/g;
  let match;
  while ((match = classRegex.exec(content)) !== null) {
    const name = match[1];
    const kind = content.substring(match.index - 10, match.index).includes("interface") ? "interface" : "class";
    const fieldsStr = match[2];
    const fields = parseSoftwareFields(fieldsStr);
    entities.push({ name, kind, fields });
  }

  const enumRegex = /class\s+(\w+)\s*\{[^}]*<<enumeration>>[^}]*\}/g;
  while ((match = enumRegex.exec(content)) !== null) {
    const name = match[1];
    const enumValues = match[0].match(/^\s+(\w+)\s*$/gm)?.map((v) => v.trim()) || [];
    entities.push({ name, kind: "enum", fields: [], enumValues });
  }

  const relRegex = /(\w+)\s+([\*o\|\-\.\>]+)\s+"([^"]+)"\s+(\w+)/g;
  while ((match = relRegex.exec(content)) !== null) {
    const from = match[1];
    const cardinalityAndType = match[2];
    const label = match[3];
    const to = match[4];

    let type = "association";
    if (cardinalityAndType.includes("*--")) type = "composition";
    if (cardinalityAndType.includes("o--")) type = "aggregation";
    if (cardinalityAndType.includes("-|>")) type = "inheritance";
    if (cardinalityAndType.includes("..>")) type = "dependency";

    relationships.push({ from, to, type, cardinality: label });
  }

  return {
    timestamp: new Date().toISOString(),
    entities,
    relationships,
  };
}

function parseSoftwareFields(fieldsStr: string): ClassField[] {
  const fields: ClassField[] = [];
  const lines = fieldsStr.split("\n");

  for (const line of lines) {
    const fieldMatch = line.match(/^\s*(\w+)\??:\s*([^\s]+)/);
    if (fieldMatch && !line.includes("<<") && !line.includes("context")) {
      const name = fieldMatch[1];
      const type = fieldMatch[2];
      fields.push({ name, type });
    }
  }

  return fields;
}

// #endregion Software Architecture Parser

// #region Comparison

const FIELD_MAPPINGS: Record<string, Record<string, string[]>> = {
  Attribute: { key: ["name", "key"] }, // TS 'key' = PY 'name'
  Quality: {
    canScale: ["scalable", "can_scale", "Scalable"],
    defaultSiUnit: ["si", "default_si_unit", "SI"],
    defaultImperialUnit: ["imperial", "default_imperial_unit", "Imperial"],
    isMinExcluded: ["min_excluded", "minExcluded", "MinExcluded"],
    isMaxExcluded: ["max_excluded", "maxExcluded", "MaxExcluded"],
    defaultValue: ["default", "default_value", "Default"],
  },
  Layer: {
    path: ["name", "path", "Path", "Name"], // Layer path might be called name in some implementations
    isHidden: ["is_hidden", "hidden", "IsHidden"],
    isLocked: ["is_locked", "locked", "IsLocked"],
  },
  Connector: {
    mandatory: ["is_mandatory", "mandatory", "Mandatory"],
    name: ["id_", "name", "Name"], // TS 'name' = PY 'id_'
  },
  Piece: {
    name: ["id_", "name", "Name"], // TS 'name' = PY 'id_'
    design: ["designPiece", "design_piece", "design", "Design"],
  },
  Prop: {
    quality: ["key", "quality_key", "qualityKey", "quality", "Quality"],
  },
  Stat: {
    quality: ["key", "quality_key", "qualityKey", "quality", "Quality"],
  },
  Model: {
    name: ["id_", "name", "Name"],
  },
  Tag: {
    name: ["id_", "name", "Name"],
  },
  Concept: {
    name: ["id_", "name", "Name"],
  },
  Type: {
    virtual: ["is_virtual", "virtual", "Virtual"],
  },
};

const SKIP_FIELDS = ["createdAt", "createdBy", "updatedAt", "updatedBy", "pk"];

const RELATIONSHIP_FIELDS: Record<string, string[]> = {
  Location: ["attributes"],
  Folder: ["attributes"],
  Benchmark: ["attributes"],
  Quality: ["benchmarks", "attributes"],
  Interface: ["compatibleInterfaces", "attributes"],
  Prop: ["attributes"],
  Model: ["tags", "attributes"],
  Connector: ["props", "attributes"],
  Tag: ["attributes"],
  Concept: ["attributes"],
  Type: ["models", "connectors", "props", "authors", "concepts", "attributes"],
  Layer: ["attributes"],
  Piece: ["props", "attributes"],
  Group: ["pieces", "attributes"],
  Connection: ["attributes"],
  Stat: [],
  Design: ["pieces", "connections", "layers", "groups", "stats", "props", "authors", "concepts", "attributes"],
  Kit: ["types", "designs", "tags", "concepts", "interfaces", "qualities", "files", "folders", "authors", "attributes"],
};

function compareSchemas(ts: SchemaReport, py: SchemaReport, cs: SchemaReport, gh: GrasshopperReport): Issue[] {
  const issues: Issue[] = [];

  for (const tsEntity of ts.entities) {
    if (ts.weakEntities.includes(tsEntity.name)) continue;

    const pyEntity = py.entities.find((e) => e.name === tsEntity.name);
    if (!pyEntity) {
      issues.push({
        severity: "error",
        entity: tsEntity.name,
        message: `Missing entity in Python`,
        source: "py",
      });
    } else {
      for (const tsField of tsEntity.fields) {
        if (SKIP_FIELDS.includes(tsField.name)) continue;

        const relFields = RELATIONSHIP_FIELDS[tsEntity.name] || [];
        if (relFields.includes(tsField.name)) continue;

        const pyFieldName = toSnakeCase(tsField.name);
        const mappings = FIELD_MAPPINGS[tsEntity.name]?.[tsField.name] || [tsField.name, pyFieldName];

        const pyField = pyEntity.fields.find((f) => mappings.includes(f.name) || f.name === pyFieldName);
        if (!pyField) {
          issues.push({
            severity: "error",
            entity: tsEntity.name,
            field: tsField.name,
            message: `Missing field in Python (expected: ${pyFieldName})`,
            source: "py",
          });
        }
      }
    }

    const csEntity = cs.entities.find((e) => e.name === tsEntity.name);
    if (!csEntity) {
      issues.push({
        severity: "error",
        entity: tsEntity.name,
        message: `Missing entity in C#`,
        source: "net",
      });
    } else {
      for (const tsField of tsEntity.fields) {
        if (SKIP_FIELDS.includes(tsField.name)) continue;

        const csFieldName = toPascalCase(tsField.name);
        const mappings = FIELD_MAPPINGS[tsEntity.name]?.[tsField.name] || [tsField.name, csFieldName];

        const csField = csEntity.fields.find((f) => mappings.some((m) => m.toLowerCase() === f.name.toLowerCase()) || f.name.toLowerCase() === tsField.name.toLowerCase());
        if (!csField) {
          issues.push({
            severity: "error",
            entity: tsEntity.name,
            field: tsField.name,
            message: `Missing field in C# (expected: ${csFieldName})`,
            source: "net",
          });
        }
      }
    }

    const ghComponent = gh.components.find((c) => c.name === tsEntity.name || c.name === toPascalCase(tsEntity.name));
    if (!ghComponent) {
      issues.push({
        severity: "warning",
        entity: tsEntity.name,
        message: `Missing Grasshopper component`,
        source: "grasshopper",
      });
    }
  }

  for (const tsId of ts.idTypes) {
    const entityName = tsId.replace("Id", "");
    if (!py.idTypes.includes(tsId)) {
      issues.push({
        severity: "warning",
        entity: entityName,
        message: `Missing ${tsId} in Python`,
        source: "py",
      });
    }
    if (!cs.idTypes.includes(tsId)) {
      issues.push({
        severity: "warning",
        entity: entityName,
        message: `Missing ${tsId} in C#`,
        source: "net",
      });
    }
  }

  for (const entityName of MAIN_ENTITIES) {
    if (!gh.params.includes(`${entityName}Param`)) {
      issues.push({
        severity: "warning",
        entity: entityName,
        message: `Missing ${entityName}Param in Grasshopper`,
        source: "grasshopper",
      });
    }
    if (!gh.goos.includes(`${entityName}Goo`)) {
      issues.push({
        severity: "warning",
        entity: entityName,
        message: `Missing ${entityName}Goo in Grasshopper`,
        source: "grasshopper",
      });
    }
  }

  return issues;
}

function toSnakeCase(str: string): string {
  return str
    .replace(/([A-Z])/g, "_$1")
    .toLowerCase()
    .replace(/^_/, "");
}

function toPascalCase(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

// #endregion Comparison

// #region Main

console.log("📊 Extracting schemas...\n");

const tsReport = parseTypeScriptSchema();
console.log(`TypeScript: ${tsReport.entities.length} entities, ${tsReport.idTypes.length} ID types`);

const pyReport = parsePythonSchema();
console.log(`Python: ${pyReport.entities.length} entities, ${pyReport.idTypes.length} ID types`);

const csReport = parseCSharpSchema();
console.log(`C#: ${csReport.entities.length} entities, ${csReport.idTypes.length} ID types`);

const ghReport = parseGrasshopperSchema();
console.log(`Grasshopper: ${ghReport.components.length} components, ${ghReport.params.length} params, ${ghReport.goos.length} goos`);

const dbReport = parseDatabaseArchitecture();
console.log(`Database Architecture: ${dbReport.entities.length} entities, ${dbReport.relationships.length} relationships`);

const ifaceReport = parseInterfaceArchitecture();
console.log(`Interface Architecture: ${ifaceReport.rootEntity.fields.length} top-level fields`);

const swReport = parseSoftwareArchitecture();
console.log(`Software Architecture: ${swReport.entities.length} entities, ${swReport.relationships.length} relationships`);

writeFileSync(join(reportsDir, "schema-ts.json"), JSON.stringify(tsReport, null, 2));
writeFileSync(join(reportsDir, "schema-py.json"), JSON.stringify(pyReport, null, 2));
writeFileSync(join(reportsDir, "schema-net.json"), JSON.stringify(csReport, null, 2));
writeFileSync(join(reportsDir, "schema-grasshopper.json"), JSON.stringify(ghReport, null, 2));
writeFileSync(join(reportsDir, "schema-database.json"), JSON.stringify(dbReport, null, 2));
writeFileSync(join(reportsDir, "schema-interface.json"), JSON.stringify(ifaceReport, null, 2));
writeFileSync(join(reportsDir, "schema-software.json"), JSON.stringify(swReport, null, 2));

console.log("\n📝 Reports written to reports/schema-*.json");

const issues = compareSchemas(tsReport, pyReport, csReport, ghReport);
const errors = issues.filter((i) => i.severity === "error");
const warnings = issues.filter((i) => i.severity === "warning");

const summaryReport = {
  timestamp: new Date().toISOString(),
  summary: {
    typescript: { entities: tsReport.entities.length, idTypes: tsReport.idTypes.length },
    python: { entities: pyReport.entities.length, idTypes: pyReport.idTypes.length },
    csharp: { entities: csReport.entities.length, idTypes: csReport.idTypes.length },
    grasshopper: {
      components: ghReport.components.length,
      params: ghReport.params.length,
      goos: ghReport.goos.length,
    },
    database: {
      entities: dbReport.entities.length,
      relationships: dbReport.relationships.length,
    },
    interface: {
      topLevelFields: ifaceReport.rootEntity.fields.length,
    },
    software: {
      entities: swReport.entities.length,
      relationships: swReport.relationships.length,
    },
    errors: errors.length,
    warnings: warnings.length,
  },
  errors,
  warnings,
  status: errors.length > 0 ? "error" : warnings.length > 0 ? "warning" : "success",
};

writeFileSync(join(reportsDir, "schema.json"), JSON.stringify(summaryReport, null, 2));

function App() {
  return (
    <Box flexDirection="column">
      <Text>🔍 Validating schemas...</Text>
      <Text>
        {errors.length === 0 ? "✅" : "❌"} {errors.length} errors, {warnings.length} warnings
      </Text>
      {errors.length > 0 && (
        <Box flexDirection="column" marginTop={1}>
          <Text>❌ Errors:</Text>
          {errors.slice(0, 10).map((err, i) => (
            <Text key={i} dimColor>
              {"  "}[{err.source}] {err.entity}
              {err.field ? "." + err.field : ""}: {err.message}
            </Text>
          ))}
          {errors.length > 10 && <Text dimColor> ... and {errors.length - 10} more errors</Text>}
        </Box>
      )}
      {warnings.length > 0 && (
        <Box flexDirection="column" marginTop={1}>
          <Text>⚠️ Warnings:</Text>
          {warnings.slice(0, 5).map((warn, i) => (
            <Text key={i} dimColor>
              {"  "}[{warn.source}] {warn.entity}
              {warn.field ? "." + warn.field : ""}: {warn.message}
            </Text>
          ))}
          {warnings.length > 5 && <Text dimColor> ... and {warnings.length - 5} more warnings</Text>}
        </Box>
      )}
      <Text dimColor>📝 Report: {join(reportsDir, "schema.json")}</Text>
    </Box>
  );
}

render(<App />);
process.exit(errors.length > 0 ? 1 : 0);

// #endregion Main

// #endregion Schema

// #region Issues

import { basename } from "path";
import * as ts from "typescript";

type CodeLanguage = "typescript" | "python" | "csharp" | "meta";
type CodeIssueKind =
  | "comment"
  | "temporary_log"
  | "missing_license_header"
  | "invalid_header_format"
  | "header_filepath_mismatch"
  | "extra_dev_docs"
  | "region_name_missing"
  | "region_mismatch"
  | "region_unclosed"
  | "region_duplicate_sibling"
  | "region_empty"
  | "unreadable_file"
  | "forbidden_import"
  | "forbidden_terminology";
type RegionNode = { name: string; line: number; children: RegionNode[] };
type CodeIssue = { path: string; language: CodeLanguage; kind: CodeIssueKind; line: number; column: number; message: string; reason: string; solution: string; excerpt?: string };
type CodeReport = { timestamp: string; status: "success" | "error"; summary: { filesScanned: number; issues: number; byKind: Record<CodeIssueKind, number>; byLanguage: Record<CodeLanguage, number> }; issues: CodeIssue[] };

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "code.json");
const fix = process.argv.includes("--fix");

type LicenseType = "LGPL" | "AGPL";

const LGPL_LICENSE_LINES = [
  "This program is free software: you can redistribute it and/or modify",
  "it under the terms of the GNU Lesser General Public License as",
  "published by the Free Software Foundation, either version 3 of the",
  "License, or (at your option) any later version.",
  "",
  "This program is distributed in the hope that it will be useful,",
  "but WITHOUT ANY WARRANTY; without even the implied warranty of",
  "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
  "GNU Lesser General Public License for more details.",
  "",
  "You should have received a copy of the GNU Lesser General Public License",
  "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
];

const AGPL_LICENSE_LINES = [
  "This program is free software: you can redistribute it and/or modify",
  "it under the terms of the GNU Affero General Public License as",
  "published by the Free Software Foundation, either version 3 of the",
  "License, or (at your option) any later version.",
  "",
  "This program is distributed in the hope that it will be useful,",
  "but WITHOUT ANY WARRANTY; without even the implied warranty of",
  "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
  "GNU Affero General Public License for more details.",
  "",
  "You should have received a copy of the GNU Affero General Public License",
  "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
];

const CONTRIBUTOR = "Ueli Saluz <ueli@semio-tech.com>";

function getDefaultIssueReason(kind: CodeIssueKind): string {
  if (kind === "comment") return "Code is never documented inline; documentation lives exclusively in the root README.md and AGENTS.md across the four required perspectives (Products, Components, SRS Business Logic/UI/UX, Codebase).";
  if (kind === "temporary_log") return "Only temporary diagnostics prefixed with [DEBUG] are allowed; they must be removed to keep Sketchpad runtime output clean.";
  if (kind === "missing_license_header") return "Every source file must include an SPDX license header and header region per code hygiene requirements.";
  if (kind === "invalid_header_format") return "Header regions must follow the required format so tooling can verify file path, contributor, and license consistently.";
  if (kind === "header_filepath_mismatch") return "Header filepaths must match the actual file path for traceability across the repo.";
  if (kind === "extra_dev_docs") return "Developer documentation is centralized in the root README.md and AGENTS.md; extra docs are forbidden.";
  if (kind === "region_name_missing") return "Regions must be named and nested to keep file structure navigable and consistent.";
  if (kind === "region_mismatch") return "Region blocks must be properly nested and closed with matching named end markers.";
  if (kind === "region_unclosed") return "Every opened region must be closed with a matching named end marker.";
  if (kind === "region_duplicate_sibling") return "Sibling regions must have unique names so region structure stays unambiguous.";
  if (kind === "region_empty") return "Empty regions are forbidden so regions always represent real code content.";
  if (kind === "unreadable_file") return "Unreadable files cannot be scanned for code hygiene requirements.";
  if (kind === "forbidden_import") return "Shared UI elements stay domain-neutral, only elements.tsx may reexport third-party dependencies, and Sketchpad scaffolding must remain decoupled from app internals.";
  return "Shared UI elements must remain domain-neutral and avoid app-specific terminology.";
}

function getDefaultIssueSolution(kind: CodeIssueKind): string {
  if (kind === "comment") return "Remove the inline comment and document the guidance in README.md under Products and Components, plus AGENTS.md under SRS Business Logic/UI/UX and Codebase.";
  if (kind === "temporary_log") return "Remove the temporary log or replace it with a warning or error if it is required.";
  if (kind === "missing_license_header") return "Add the SPDX header and header region using the code fix hook.";
  if (kind === "invalid_header_format") return "Regenerate the header with the fix hook to match the required format.";
  if (kind === "header_filepath_mismatch") return "Update the header filepath or regenerate the header to match the file location.";
  if (kind === "extra_dev_docs") return "Move the content into root README.md and AGENTS.md in the required four documentation sections, then remove the extra file.";
  if (kind === "region_name_missing") return "Add a matching name to the region and endregion markers.";
  if (kind === "region_mismatch") return "Fix the region names and nesting so each endregion matches the open region.";
  if (kind === "region_unclosed") return "Add the matching endregion marker with the same name.";
  if (kind === "region_duplicate_sibling") return "Rename one region or merge them into a single region.";
  if (kind === "region_empty") return "Remove the empty region or move relevant code into it.";
  if (kind === "unreadable_file") return "Fix file permissions or encoding so the scanner can read the file.";
  if (kind === "forbidden_import") return "Move shared logic into elements/shared/semio or app-specific modules, then update the import to the allowed path.";
  return "Replace domain terms with neutral wording or move the string into app-specific modules.";
}

function createIssue(params: { path: string; language: CodeLanguage; kind: CodeIssueKind; line: number; column: number; message: string; excerpt?: string; reason?: string; solution?: string }): CodeIssue {
  return {
    path: params.path,
    language: params.language,
    kind: params.kind,
    line: params.line,
    column: params.column,
    message: params.message,
    reason: params.reason ?? getDefaultIssueReason(params.kind),
    solution: params.solution ?? getDefaultIssueSolution(params.kind),
    excerpt: params.excerpt,
  };
}

function getLicenseType(filepath: string): LicenseType {
  const normalized = filepath.replace(/\\/g, "/");
  if (normalized.startsWith("js/js/")) return "LGPL";
  if (normalized.startsWith("net/Semio/")) return "LGPL";
  if (normalized.startsWith("net/Semio.Grasshopper/")) return "LGPL";
  if (normalized.startsWith("py/engine/")) return "LGPL";
  return "AGPL";
}

function getLicenseLines(licenseType: LicenseType): string[] {
  return licenseType === "LGPL" ? LGPL_LICENSE_LINES : AGPL_LICENSE_LINES;
}

function getLicenseValidationText(licenseType: LicenseType): string {
  return licenseType === "LGPL" ? "GNU Lesser General Public License" : "GNU Affero General Public License";
}

function generateHeader(filepath: string, year: number, language: CodeLanguage): string[] {
  const prefix = language === "python" ? "# " : "// ";
  const regionStart = language === "csharp" ? "#region Header" : language === "python" ? "# region Header" : "// #region Header";
  const regionEnd = language === "csharp" ? "#endregion Header" : language === "python" ? "# endregion Header" : "// #endregion Header";
  const licenseType = getLicenseType(filepath);
  const licenseLines = getLicenseLines(licenseType);
  return [regionStart, "", `${prefix}${filepath}`, "", `${prefix}${year} ${CONTRIBUTOR}`, "", ...licenseLines.map((line) => (line ? `${prefix}${line}` : "")), "", regionEnd];
}

function parseExistingHeader(lines: string[], language: CodeLanguage, filepath: string): { hasHeaderRegion: boolean; headerEndIndex: number; isValidFormat: boolean; existingFilepath: string | null; existingYear: number | null } {
  const prefix = language === "python" ? "#" : "//";
  const regionStartPattern = language === "csharp" ? /^\s*#\s*region\s+Header\s*$/i : language === "python" ? /^\s*#\s*region\s+Header\s*$/i : /^\s*\/\/\s*#region\s+Header\s*$/i;
  const regionEndPattern = language === "csharp" ? /^\s*#\s*endregion\s+Header\s*$/i : language === "python" ? /^\s*#\s*endregion(\s+Header)?\s*$/i : /^\s*\/\/\s*#endregion\s+Header\s*$/i;
  const validRegionEndPattern = language === "csharp" ? /^\s*#\s*endregion\s+Header\s*$/i : language === "python" ? /^\s*#\s*endregion\s+Header\s*$/i : /^\s*\/\/\s*#endregion\s+Header\s*$/i;
  let hasHeaderRegion = false;
  let headerEndIndex = -1;
  let isValidFormat = false;
  let existingFilepath: string | null = null;
  let existingYear: number | null = null;
  const maxSearch = language === "python" ? 60 : 5;
  for (let i = 0; i < Math.min(lines.length, maxSearch); i++) {
    if (regionStartPattern.test(lines[i] ?? "")) {
      hasHeaderRegion = true;
      for (let j = i + 1; j < Math.min(lines.length, i + 30); j++) {
        if (regionEndPattern.test(lines[j] ?? "")) {
          headerEndIndex = j;
          break;
        }
        const line = lines[j] ?? "";
        const filepathMatch = line.match(new RegExp(`^\\s*${prefix}\\s*([a-zA-Z0-9_/.-]+\\.[a-z]+)\\s*$`));
        if (filepathMatch && !existingFilepath) existingFilepath = filepathMatch[1] ?? null;
        const yearMatch = line.match(new RegExp(`^\\s*${prefix}\\s*(\\d{4})(?:[\\s-])`));
        if (yearMatch && !existingYear) existingYear = parseInt(yearMatch[1] ?? "0", 10);
      }
      break;
    }
  }
  if (hasHeaderRegion && headerEndIndex > 0 && existingFilepath && existingYear) {
    const headerContent = lines.slice(0, headerEndIndex + 1).join("\n");
    const expectedLicenseText = getLicenseValidationText(getLicenseType(filepath));
    isValidFormat = headerContent.includes(expectedLicenseText) && headerContent.includes(CONTRIBUTOR) && validRegionEndPattern.test(lines[headerEndIndex] ?? "");
  }
  return { hasHeaderRegion, headerEndIndex, isValidFormat, existingFilepath, existingYear };
}

function findHeaderRegionStart(lines: string[], language: CodeLanguage): number {
  const regionStartPattern = language === "csharp" ? /^\s*#\s*region\s+Header\s*$/i : language === "python" ? /^\s*#\s*region\s+Header\s*$/i : /^\s*\/\/\s*#region\s+Header\s*$/i;
  const maxSearch = language === "python" ? 60 : 5;
  for (let i = 0; i < Math.min(lines.length, maxSearch); i++) {
    if (regionStartPattern.test(lines[i] ?? "")) return i;
  }
  return -1;
}

function fixHeader(content: string, lines: string[], filepath: string, language: CodeLanguage): string {
  const year = new Date().getFullYear();
  const header = parseExistingHeader(lines, language, filepath);
  const newHeader = generateHeader(filepath, year, language);
  let preambleEndAt = 0;
  if ((lines[0] ?? "").startsWith("#!")) preambleEndAt = 1;
  if (language === "python" && preambleEndAt === 1 && /^\s*#\s*coding[:=]/i.test(lines[1] ?? "")) preambleEndAt = 2;
  if (language === "typescript") {
    while ((lines[preambleEndAt] ?? "").trimStart().startsWith("///") && /<\s*reference\b/i.test(lines[preambleEndAt] ?? "")) preambleEndAt += 1;
  }
  const preamble = lines.slice(0, preambleEndAt);
  if (header.hasHeaderRegion && header.headerEndIndex > 0) {
    const headerStart = findHeaderRegionStart(lines, language);
    const afterHeader = lines.slice(header.headerEndIndex + 1);
    const betweenPreambleAndHeader = lines.slice(preambleEndAt, headerStart).filter((line) => line.trim() !== "");
    if (betweenPreambleAndHeader.length === 0) {
      return [...preamble, ...newHeader, ...afterHeader].join("\n");
    }
    return [...preamble, ...newHeader, "", ...betweenPreambleAndHeader, ...afterHeader].join("\n");
  }
  return [...preamble, ...newHeader, "", ...lines.slice(preambleEndAt)].join("\n");
}

function normalizePath(path: string): string {
  return path.replace(/\\/g, "/");
}

function getTrackedFiles(): string[] {
  return execSync("git ls-files", { cwd: rootDir, encoding: "utf-8" })
    .split(/\r?\n/g)
    .map((line) => line.trim())
    .filter(Boolean);
}

function getUntrackedFiles(): string[] {
  return execSync("git status --porcelain -uall", { cwd: rootDir, encoding: "utf-8" })
    .split(/\r?\n/g)
    .map((line) => line.trim())
    .filter(Boolean)
    .filter((line) => line.startsWith("?? "))
    .map((line) => line.slice("?? ".length).trim())
    .filter(Boolean);
}

function getRepoFiles(): string[] {
  return Array.from(new Set([...getTrackedFiles(), ...getUntrackedFiles()].map(normalizePath))).sort();
}

function getPackageRootDirs(repoFiles: string[]): Set<string> {
  const roots = new Set<string>();
  for (const file of repoFiles) {
    if (file.endsWith("/package.json") || file.endsWith("/pyproject.toml") || file.endsWith(".csproj") || file.endsWith(".fsproj") || file.endsWith("/yak.toml")) roots.add(dirname(file).replace(/\\/g, "/"));
  }
  return roots;
}

function isConfigFile(path: string): boolean {
  const base = basename(path);
  if (base.endsWith(".config.ts") || base.endsWith(".config.tsx")) return true;
  if (base === "vite.config.ts" || base === "vite.config.tsx") return true;
  if (base === "vite.test.config.ts" || base === "vite.test.config.tsx") return true;
  if (base === "vitest.config.ts" || base === "vitest.config.tsx") return true;
  if (base === "eslint.config.ts" || base === "eslint.config.tsx") return true;
  if (base === "playwright.config.ts" || base === "playwright.config.tsx") return true;
  if (base === "tailwind.config.ts" || base === "tailwind.config.tsx") return true;
  if (base === "postcss.config.ts" || base === "postcss.config.tsx") return true;
  return false;
}

function isExcludedFromImportRules(path: string): boolean {
  const normalized = normalizePath(path);
  if (normalized.includes(".storybook/")) return true;
  if (normalized.includes(".stories.")) return true;
  if (normalized.endsWith(".test.ts") || normalized.endsWith(".test.tsx")) return true;
  if (isConfigFile(path)) return true;
  if (normalized.endsWith("/dev.ts")) return true;
  if (normalized.endsWith("/site.tsx")) return true;
  return false;
}

function getLanguage(path: string): CodeLanguage | null {
  if (path.endsWith(".ts") || path.endsWith(".tsx")) return "typescript";
  if (path.endsWith(".py")) return "python";
  if (path.endsWith(".cs")) return "csharp";
  return null;
}

function isExtraDevDoc(path: string, packageRoots: Set<string>): boolean {
  const normalized = normalizePath(path);
  if (normalized === "README.md" || normalized === "AGENTS.md") return false;
  if (normalized.endsWith("/AGENTS.md")) return true;
  if (normalized.endsWith("/README.md") && packageRoots.has(dirname(normalized))) return false;
  return normalized.endsWith("/README.md");
}

function shouldScan(path: string): boolean {
  const normalized = normalizePath(path);
  if (normalized.startsWith("log/")) return false;
  if (normalized.startsWith("reports/")) return false;
  if (normalized.startsWith("temp/")) return false;
  if (normalized.startsWith("test-results/")) return false;
  if (normalized.startsWith("node_modules/")) return false;
  if (normalized.startsWith(".nx/")) return false;
  if (normalized.startsWith(".github/")) return false;
  return true;
}

function splitLines(text: string): string[] {
  return text.replace(/^\uFEFF/, "").split(/\r?\n/g);
}

function getExcerpt(line: string, max = 160): string {
  const trimmed = line.trim();
  return trimmed.length > max ? `${trimmed.slice(0, max)}...` : trimmed;
}

function getLicenseHeaderEndLine(lines: string[], language: CodeLanguage): { hasSpdx: boolean; headerEndLine: number } {
  let start = 0;
  if (language !== "python" && lines[0]?.startsWith("#!")) start = 1;
  if (language === "python" && (lines[0]?.startsWith("#!") || lines[0]?.startsWith("# -*-"))) start = 1;
  if (language === "python" && start === 1 && /^\s*#\s*coding[:=]/i.test(lines[1] ?? "")) start = 2;
  const headerLines: number[] = [];
  let hasSpdx = false;
  const startsWithLineComment = (line: string): boolean => {
    const trimmed = line.trim();
    if (!trimmed) return true;
    if (language === "python") return trimmed.startsWith("#");
    return trimmed.startsWith("//");
  };
  for (let i = start; i < Math.min(lines.length, start + 50); i++) {
    const line = lines[i] ?? "";
    if (!startsWithLineComment(line)) break;
    headerLines.push(i + 1);
    if (line.includes("SPDX-License-Identifier:")) hasSpdx = true;
  }
  if (hasSpdx && headerLines.length > 0) return { hasSpdx: true, headerEndLine: headerLines[headerLines.length - 1] ?? 0 };
  if (language !== "python") {
    const first = lines[start] ?? "";
    const trimmed = first.trim();
    if (trimmed.startsWith("/*")) {
      let endLine = 0;
      let headerText = "";
      for (let i = start; i < Math.min(lines.length, start + 200); i++) {
        headerText += `${lines[i] ?? ""}\n`;
        if ((lines[i] ?? "").includes("*/")) {
          endLine = i + 1;
          break;
        }
      }
      if (headerText.includes("SPDX-License-Identifier:")) return { hasSpdx: true, headerEndLine: endLine || 1 };
    }
  }
  return { hasSpdx: false, headerEndLine: 0 };
}

function parseRegions(lines: string[], language: CodeLanguage, path: string): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const stack: RegionNode[] = [];
  const rootChildren: RegionNode[] = [];
  const regionRegex = language === "csharp" ? /^\s*#\s*region\b(.*)$/ : language === "python" ? /^\s*#\s*region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#\s*endregion\b(.*)$/ : language === "python" ? /^\s*#\s*endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i] ?? "";
    const regionMatch = line.match(regionRegex);
    if (regionMatch) {
      const name = (regionMatch[1] ?? "").trim();
      if (!name) issues.push(createIssue({ path, language, kind: "region_name_missing", line: lineNumber, column: 1, message: "Region is missing a name", excerpt: getExcerpt(line) }));
      const node: RegionNode = { name, line: lineNumber, children: [] };
      const siblings = stack.length > 0 ? (stack[stack.length - 1] as RegionNode).children : rootChildren;
      const duplicate = name !== "" ? siblings.find((s) => s.name === name) : undefined;
      if (duplicate)
        issues.push(createIssue({ path, language, kind: "region_duplicate_sibling", line: lineNumber, column: 1, message: `Duplicate sibling region name "${name}" (first occurrence at line ${duplicate.line})`, excerpt: getExcerpt(line) }));
      siblings.push(node);
      stack.push(node);
      continue;
    }
    const endMatch = line.match(endRegionRegex);
    if (endMatch) {
      const name = (endMatch[1] ?? "").trim();
      if (!name) issues.push(createIssue({ path, language, kind: "region_name_missing", line: lineNumber, column: 1, message: "Endregion is missing a name", excerpt: getExcerpt(line) }));
      const current = stack.pop();
      if (!current) {
        issues.push(createIssue({ path, language, kind: "region_mismatch", line: lineNumber, column: 1, message: "Endregion without matching region", excerpt: getExcerpt(line) }));
        continue;
      }
      if (current.name !== name)
        issues.push(createIssue({ path, language, kind: "region_mismatch", line: lineNumber, column: 1, message: `Region mismatch: opened \"${current.name}\" at line ${current.line}, closed \"${name}\"`, excerpt: getExcerpt(line) }));
    }
  }
  for (const open of stack) {
    issues.push(createIssue({ path, language, kind: "region_unclosed", line: open.line, column: 1, message: `Region \"${open.name}\" not closed`, excerpt: getExcerpt(lines[open.line - 1] ?? "") }));
  }
  return issues;
}

// #region EmptyRegions

function getEmptyRegions(lines: string[], language: CodeLanguage): { regions: { name: string; start: number; end: number }[]; linesToRemove: Set<number> } {
  const regions: { name: string; start: number; end: number }[] = [];
  const linesToRemove = new Set<number>();
  const regionRegex = language === "csharp" ? /^\s*#\s*region\b(.*)$/ : language === "python" ? /^\s*#\s*region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#\s*endregion\b(.*)$/ : language === "python" ? /^\s*#\s*endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
  const stack: { name: string; start: number; hasContent: boolean }[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i] ?? "";
    const regionMatch = line.match(regionRegex);
    if (regionMatch) {
      const name = (regionMatch[1] ?? "").trim();
      stack.push({ name, start: lineNumber, hasContent: false });
      continue;
    }
    if (endRegionRegex.test(line)) {
      const current = stack.pop();
      if (!current) continue;
      if (!current.hasContent) {
        regions.push({ name: current.name, start: current.start, end: lineNumber });
        for (let lineIndex = current.start; lineIndex <= lineNumber; lineIndex++) {
          linesToRemove.add(lineIndex);
        }
      }
      continue;
    }
    if (line.trim() === "") continue;
    for (const entry of stack) {
      entry.hasContent = true;
    }
  }
  return { regions, linesToRemove };
}

// #endregion EmptyRegions

function getHeaderRegionLines(lines: string[], language: CodeLanguage): Set<number> {
  const set = new Set<number>();
  const headerNames = new Set(["header", "headers", "fileheader", "file-header"]);
  const regionRegex = language === "csharp" ? /^\s*#\s*region\b(.*)$/ : language === "python" ? /^\s*#\s*region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#\s*endregion\b(.*)$/ : language === "python" ? /^\s*#\s*endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
  const stack: string[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i] ?? "";
    const startMatch = line.match(regionRegex);
    if (startMatch) {
      const name = (startMatch[1] ?? "").trim().toLowerCase().replace(/\s+/g, "");
      stack.push(name);
      if (stack.some((n) => headerNames.has(n))) set.add(lineNumber);
      continue;
    }
    const endMatch = line.match(endRegionRegex);
    if (endMatch) {
      if (stack.some((n) => headerNames.has(n))) set.add(lineNumber);
      stack.pop();
      continue;
    }
    if (stack.some((n) => headerNames.has(n))) set.add(lineNumber);
  }
  return set;
}

function isTsRegionLineText(textAfterSlashes: string): boolean {
  const t = textAfterSlashes.trim();
  return t.startsWith("#region") || t.startsWith("#endregion");
}

function collectTypescriptComments(content: string, sourceFile: ts.SourceFile): ts.CommentRange[] {
  const seenRanges = new Set<string>();
  const allComments: ts.CommentRange[] = [];
  const addRange = (range: ts.CommentRange): void => {
    const key = `${range.pos}-${range.end}`;
    if (!seenRanges.has(key)) {
      seenRanges.add(key);
      allComments.push(range);
    }
  };
  const collectAt = (pos: number): void => {
    const leading = ts.getLeadingCommentRanges(content, pos);
    if (leading) {
      for (const range of leading) {
        addRange(range);
      }
    }
  };
  collectAt(0);
  const visit = (node: ts.Node): void => {
    collectAt(node.getFullStart());
    const jsDocNodes = ts.getJSDocCommentsAndTags(node);
    for (const doc of jsDocNodes) {
      addRange({ pos: doc.pos, end: doc.end, kind: ts.SyntaxKind.MultiLineCommentTrivia });
    }
    const trailing = ts.getTrailingCommentRanges(content, node.getEnd());
    if (trailing) {
      for (const range of trailing) {
        addRange(range);
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return allComments.sort((a, b) => a.pos - b.pos);
}

function scanTypescriptComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  if (isConfigFile(path)) return issues;
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const headerLines = getHeaderRegionLines(lines, "typescript");
  const allComments = collectTypescriptComments(content, sourceFile);
  for (const comment of allComments) {
    const { line, character } = sourceFile.getLineAndCharacterOfPosition(comment.pos);
    const lineNumber = line + 1;
    const column = character + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const tokenText = content.slice(comment.pos, comment.end);
    if (headerLines.has(lineNumber)) continue;
    if (/todo/i.test(tokenText)) continue;
    if (tokenText.includes("SPDX-License-Identifier:")) continue;
    if (comment.kind === ts.SyntaxKind.SingleLineCommentTrivia && tokenText.startsWith("///") && /<\s*reference\b/i.test(tokenText)) continue;
    const lineText = lines[lineNumber - 1] ?? "";
    if (comment.kind === ts.SyntaxKind.SingleLineCommentTrivia) {
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (isTsRegionLineText(after)) continue;
    }
    issues.push(createIssue({ path, language: "typescript", kind: "comment", line: lineNumber, column, message: comment.kind === ts.SyntaxKind.SingleLineCommentTrivia ? "Line comment found" : "Block comment found", excerpt: getExcerpt(lineText) }));
  }
  return issues;
}

function getTypescriptStringLiteralText(node: ts.Node): string | null {
  if (ts.isStringLiteralLike(node)) return node.text;
  if (ts.isNoSubstitutionTemplateLiteral(node)) return node.text;
  if (ts.isTemplateExpression(node)) return [node.head.text, ...node.templateSpans.map((span) => span.literal.text)].join("");
  if (ts.isBinaryExpression(node) && node.operatorToken.kind === ts.SyntaxKind.PlusToken) {
    const left = getTypescriptStringLiteralText(node.left);
    const right = getTypescriptStringLiteralText(node.right);
    if (left !== null && right !== null) return `${left}${right}`;
  }
  return null;
}
// #region JsJsRules
// #region JsJsRuleConstants
const jsJsRoot = "js/js/";
const sketchpadAppNames = ["Home", "Kit", "Design", "Type", "Quality", "Docs", "Feedback"];
const sketchpadAppPaths = new Set(sketchpadAppNames.map((name) => `js/js/sketchpad/${name}.tsx`));
const sketchpadImportTargets = new Set(["js/js/sketchpad/elements", "js/js/sketchpad/shared", "js/js/sketchpad/Tutorials", "js/js/semio", "js/js/i18n", ...sketchpadAppNames.map((name) => `js/js/sketchpad/${name}`)]);
const appImportTargets = new Set(["js/js/sketchpad/Sketchpad", ...sketchpadImportTargets]);
// #endregion JsJsRuleConstants
// #region JsJsRuleScans
function scanTypescriptForbiddenImports(content: string, lines: string[], path: string): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const normalizedPath = normalizePath(path);
  if (!normalizedPath.startsWith(jsJsRoot)) return issues;
  if (isExcludedFromImportRules(path)) return issues;
  const isElementsFile = normalizedPath.endsWith("/elements.tsx");
  const isSketchpadFile = normalizedPath === "js/js/sketchpad/Sketchpad.tsx";
  const isAppFile = sketchpadAppPaths.has(normalizedPath);
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const addIssue = (node: ts.Node, message: string, reason: string, solution: string): void => {
    const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
    const lineNumber = line + 1;
    issues.push(createIssue({ path, language: "typescript", kind: "forbidden_import", line: lineNumber, column: character + 1, message, reason, solution, excerpt: getExcerpt(lines[lineNumber - 1] ?? "") }));
  };
  const elementsAllowedTargets = new Set(["js/js/i18n", "js/js/semio"]);
  const checkModuleText = (node: ts.Node, moduleText: string): void => {
    if (isElementsFile && moduleText.startsWith(".")) {
      const resolved = normalizePath(join(dirname(normalizedPath), moduleText)).replace(/\.[^.\/]+$/, "");
      if (!elementsAllowedTargets.has(resolved)) {
        addIssue(
          node,
          "Relative imports in elements.tsx must target i18n.ts or semio.ts",
          "elements.tsx is the domain-neutral shared UI library and the only js/js file allowed to import third-party dependencies; it may only import js/js/i18n.ts or js/js/semio.ts.",
          "Move shared functionality into elements.tsx or into js/js/i18n.ts or js/js/semio.ts, then import from the approved path.",
        );
      }
      return;
    }
    if (isSketchpadFile || isAppFile) {
      if (!moduleText.startsWith(".")) return;
      const resolved = normalizePath(join(dirname(normalizedPath), moduleText)).replace(/\.[^.\/]+$/, "");
      if ((isSketchpadFile && !sketchpadImportTargets.has(resolved)) || (isAppFile && !appImportTargets.has(resolved)))
        addIssue(
          node,
          "Relative imports must target elements.tsx, Sketchpad.tsx, semio.ts, or shared.ts",
          "Sketchpad.tsx provides scaffolding and must remain independent of app internals; apps import only shared elements, shared utilities, and core domain modules.",
          "Move shared logic into shared modules (elements/shared/semio) and update the import so Sketchpad.tsx and apps remain decoupled.",
        );
      return;
    }
  };
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteralLike(node.moduleSpecifier)) checkModuleText(node.moduleSpecifier, node.moduleSpecifier.text);
    if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteralLike(node.moduleSpecifier)) checkModuleText(node.moduleSpecifier, node.moduleSpecifier.text);
    if (ts.isImportEqualsDeclaration(node) && ts.isExternalModuleReference(node.moduleReference) && ts.isStringLiteralLike(node.moduleReference.expression)) checkModuleText(node.moduleReference.expression, node.moduleReference.expression.text);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) {
      const arg = node.arguments[0];
      if (arg && ts.isTemplateExpression(arg)) return;
      const argText = getTypescriptStringLiteralText(arg ?? node);
      if (argText !== null) checkModuleText(arg ?? node, argText);
    }
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "require") {
      const argText = getTypescriptStringLiteralText(node.arguments[0] ?? node);
      if (argText !== null) checkModuleText(node.arguments[0] ?? node, argText);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return issues;
}

function scanTypescriptForbiddenTerminology(content: string, lines: string[], path: string): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const normalizedPath = normalizePath(path);
  if (!normalizedPath.startsWith(jsJsRoot) || !normalizedPath.endsWith("/elements.tsx")) return issues;
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const forbiddenRegex = /\b(kit|design|type|connector|connection|docs|feedback)\b/i;
  const allowedPatterns = [/@dnd-kit/, /\/docs\//, /semio\.sketchpad\.docs\./, /^type$/, /^id$/];
  const isAllowed = (text: string): boolean => allowedPatterns.some((p) => p.test(text));
  const addIssue = (node: ts.Node, term: string): void => {
    const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
    const lineNumber = line + 1;
    issues.push(
      createIssue({
        path,
        language: "typescript",
        kind: "forbidden_terminology",
        line: lineNumber,
        column: character + 1,
        message: `Forbidden terminology \"${term}\"`,
        reason: "Shared UI elements must stay domain-neutral and avoid app-specific terminology.",
        solution: "Replace the term with neutral wording or move the string into app-specific modules.",
        excerpt: getExcerpt(lines[lineNumber - 1] ?? ""),
      }),
    );
  };
  const visit = (node: ts.Node): void => {
    if (ts.isJsxText(node)) {
      const match = node.text.match(forbiddenRegex);
      if (match && !isAllowed(node.text)) addIssue(node, match[0]);
    }
    const literalText = getTypescriptStringLiteralText(node);
    if (literalText !== null) {
      const match = literalText.match(forbiddenRegex);
      if (match && !isAllowed(literalText)) addIssue(node, match[0]);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return issues;
}
// #endregion JsJsRuleScans
// #endregion JsJsRules

function scanTypescriptTemporaryLogs(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const methods = new Set(["log", "debug", "info", "warn", "error"]);
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node)) {
      const expression = node.expression;
      if (ts.isPropertyAccessExpression(expression) && ts.isIdentifier(expression.expression) && expression.expression.text === "console" && methods.has(expression.name.text)) {
        const debugArg = node.arguments
          .map((arg) => getTypescriptStringLiteralText(arg))
          .filter((text): text is string => text !== null)
          .some((text) => text.includes("[DEBUG]"));
        if (debugArg) {
          const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
          const lineNumber = line + 1;
          if (lineNumber > licenseHeaderEndLine)
            issues.push(createIssue({ path, language: "typescript", kind: "temporary_log", line: lineNumber, column: character + 1, message: "Temporary [DEBUG] console log found", excerpt: getExcerpt(lines[lineNumber - 1] ?? "") }));
        }
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return issues;
}

function stripTypescriptComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): string {
  if (isConfigFile(path)) return content;
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const headerLines = getHeaderRegionLines(lines, "typescript");
  const allComments = collectTypescriptComments(content, sourceFile);
  const removals: { start: number; end: number; insertSpace: boolean }[] = [];
  for (const comment of allComments) {
    const { line } = sourceFile.getLineAndCharacterOfPosition(comment.pos);
    const lineNumber = line + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const tokenText = content.slice(comment.pos, comment.end);
    if (headerLines.has(lineNumber)) continue;
    if (/todo/i.test(tokenText)) continue;
    if (tokenText.includes("SPDX-License-Identifier:")) continue;
    if (comment.kind === ts.SyntaxKind.SingleLineCommentTrivia) {
      const lineText = lines[lineNumber - 1] ?? "";
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (isTsRegionLineText(after)) continue;
      if (tokenText.startsWith("///") && /<\s*reference\b/i.test(tokenText)) continue;
    }
    const prev = comment.pos > 0 ? (content[comment.pos - 1] ?? "") : "";
    const next = comment.end < content.length ? (content[comment.end] ?? "") : "";
    const insertSpace = /[A-Za-z0-9_$]/.test(prev) && /[A-Za-z0-9_$]/.test(next);
    removals.push({ start: comment.pos, end: comment.end, insertSpace });
  }
  if (removals.length === 0) return content;
  let out = "";
  let index = 0;
  for (const removal of removals) {
    if (removal.start < index) continue;
    out += content.slice(index, removal.start);
    if (removal.insertSpace) out += " ";
    index = removal.end;
  }
  out += content.slice(index);
  return out;
}

function scanTsOrCsharpComments(content: string, lines: string[], language: CodeLanguage, path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  let inSingle = false;
  let inDouble = false;
  let inTemplate = false;
  let inBlockComment = false;
  let line = 1;
  let col = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    if (ch === "\n") {
      line += 1;
      col = 1;
      continue;
    }
    if (inBlockComment) {
      if (ch === "*" && next === "/") {
        inBlockComment = false;
        i += 1;
        col += 2;
        continue;
      }
      col += 1;
      continue;
    }
    if (!inDouble && !inTemplate && ch === "'" && !inSingle) {
      inSingle = true;
      col += 1;
      continue;
    }
    if (inSingle) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "'") inSingle = false;
      col += 1;
      continue;
    }
    if (!inSingle && !inTemplate && ch === '"' && !inDouble) {
      inDouble = true;
      col += 1;
      continue;
    }
    if (inDouble) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === '"') inDouble = false;
      col += 1;
      continue;
    }
    if (language === "typescript" && !inSingle && !inDouble && ch === "`" && !inTemplate) {
      inTemplate = true;
      col += 1;
      continue;
    }
    if (inTemplate) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "`") inTemplate = false;
      col += 1;
      continue;
    }
    if (ch === "/" && next === "/") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (line <= licenseHeaderEndLine) {
        col += 2;
        continue;
      }
      if (language === "typescript" && isTsRegionLineText(after)) {
        col += 2;
        continue;
      }
      if (after.includes("SPDX-License-Identifier:")) {
        col += 2;
        continue;
      }
      issues.push(createIssue({ path, language, kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) }));
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      line += 1;
      col = 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      if (line <= licenseHeaderEndLine) {
        inBlockComment = true;
        i += 1;
        col += 2;
        continue;
      }
      const lineText = lines[line - 1] ?? "";
      issues.push(createIssue({ path, language, kind: "comment", line, column: col, message: "Block comment found", excerpt: getExcerpt(lineText) }));
      inBlockComment = true;
      i += 1;
      col += 2;
      continue;
    }
    col += 1;
  }
  return issues;
}

function isPythonRegionLine(lineText: string): boolean {
  return /^\s*#\s*(?:end)?region\b/i.test(lineText);
}

function scanPythonComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  if (isConfigFile(path)) return issues;
  const headerLines = getHeaderRegionLines(lines, "python");
  let inSingle = false;
  let inDouble = false;
  let inTripleSingle = false;
  let inTripleDouble = false;
  let line = 1;
  let col = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      line += 1;
      col = 1;
      continue;
    }
    if (inTripleSingle) {
      if (ch === "'" && next === "'" && next2 === "'") {
        inTripleSingle = false;
        i += 2;
        col += 3;
        continue;
      }
      col += 1;
      continue;
    }
    if (inTripleDouble) {
      if (ch === '"' && next === '"' && next2 === '"') {
        inTripleDouble = false;
        i += 2;
        col += 3;
        continue;
      }
      col += 1;
      continue;
    }
    if (inSingle) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "'") inSingle = false;
      col += 1;
      continue;
    }
    if (inDouble) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === '"') inDouble = false;
      col += 1;
      continue;
    }
    if (ch === "'" && next === "'" && next2 === "'") {
      inTripleSingle = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === '"' && next === '"' && next2 === '"') {
      inTripleDouble = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "'") {
      inSingle = true;
      col += 1;
      continue;
    }
    if (ch === '"') {
      inDouble = true;
      col += 1;
      continue;
    }
    if (ch === "#") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(lineText.indexOf("#") + 1).trim();
      if (line <= licenseHeaderEndLine) {
        col += 1;
        continue;
      }
      if (headerLines.has(line)) {
        col += 1;
        continue;
      }
      if (isPythonRegionLine(lineText)) {
        col += 1;
        continue;
      }
      if (after.includes("SPDX-License-Identifier:")) {
        col += 1;
        continue;
      }
      if (/todo/i.test(after)) {
        col += 1;
        continue;
      }
      issues.push(createIssue({ path, language: "python", kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) }));
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      line += 1;
      col = 1;
      continue;
    }
    col += 1;
  }
  return issues;
}

function scanPythonTemporaryLogs(lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const lineText = lines[i] ?? "";
    if (!lineText.includes("[DEBUG]")) continue;
    if (!/\b(print|logging\.(debug|info|warning|error)|logger\.(debug|info|warning|error))\s*\(/.test(lineText)) continue;
    issues.push(createIssue({ path, language: "python", kind: "temporary_log", line: lineNumber, column: 1, message: "Temporary [DEBUG] log found", excerpt: getExcerpt(lineText) }));
  }
  return issues;
}

function scanCsharpComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  if (isConfigFile(path)) return issues;
  const headerLines = getHeaderRegionLines(lines, "csharp");
  let inChar = false;
  let inString = false;
  let inVerbatimString = false;
  let inBlockComment = false;
  let line = 1;
  let col = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      line += 1;
      col = 1;
      continue;
    }
    if (inBlockComment) {
      if (ch === "*" && next === "/") {
        inBlockComment = false;
        i += 1;
        col += 2;
        continue;
      }
      col += 1;
      continue;
    }
    if (inChar) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "'") inChar = false;
      col += 1;
      continue;
    }
    if (inVerbatimString) {
      if (ch === '"' && next === '"') {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === '"') inVerbatimString = false;
      col += 1;
      continue;
    }
    if (inString) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === '"') inString = false;
      col += 1;
      continue;
    }
    if (ch === "'" && !inChar) {
      inChar = true;
      col += 1;
      continue;
    }
    if (ch === "$" && next === "@" && next2 === '"') {
      inVerbatimString = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "@" && next === "$" && next2 === '"') {
      inVerbatimString = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "@" && next === '"') {
      inVerbatimString = true;
      i += 1;
      col += 2;
      continue;
    }
    if (ch === "$" && next === '"') {
      inString = true;
      i += 1;
      col += 2;
      continue;
    }
    if (ch === '"') {
      inString = true;
      col += 1;
      continue;
    }
    if (ch === "/" && next === "/") {
      const lineText = lines[line - 1] ?? "";
      if (line <= licenseHeaderEndLine) {
        col += 2;
        continue;
      }
      if (headerLines.has(line)) {
        col += 2;
        continue;
      }
      if (/todo/i.test(lineText)) {
        col += 2;
        continue;
      }
      issues.push(createIssue({ path, language: "csharp", kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) }));
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      line += 1;
      col = 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      const lineText = lines[line - 1] ?? "";
      if (line <= licenseHeaderEndLine) {
        inBlockComment = true;
        i += 1;
        col += 2;
        continue;
      }
      const endIndex = content.indexOf("*/", i + 2);
      const blockText = endIndex === -1 ? content.slice(i) : content.slice(i, endIndex + 2);
      if (headerLines.has(line) || /todo/i.test(blockText)) {
        inBlockComment = true;
        i += 1;
        col += 2;
        continue;
      }
      issues.push(createIssue({ path, language: "csharp", kind: "comment", line, column: col, message: "Block comment found", excerpt: getExcerpt(lineText) }));
      inBlockComment = true;
      i += 1;
      col += 2;
      continue;
    }
    col += 1;
  }
  return issues;
}

function scanCsharpTemporaryLogs(lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const regex = /\b(?:System\.)?Console\.(?:WriteLine|Write|Error\.WriteLine|Error\.Write)\s*\(/;
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const lineText = lines[i] ?? "";
    if (!regex.test(lineText)) continue;
    if (!lineText.includes("[DEBUG]")) continue;
    issues.push(createIssue({ path, language: "csharp", kind: "temporary_log", line: lineNumber, column: 1, message: "Temporary [DEBUG] console log found", excerpt: getExcerpt(lineText) }));
  }
  return issues;
}

function stripCsharpComments(content: string, lines: string[], licenseHeaderEndLine: number): string {
  const headerLines = getHeaderRegionLines(lines, "csharp");
  let out = "";
  let inChar = false;
  let inString = false;
  let inVerbatimString = false;
  let inBlockCommentKeep = false;
  let inBlockCommentDrop = false;
  let line = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      out += "\n";
      line += 1;
      continue;
    }
    if (inBlockCommentKeep) {
      out += ch;
      if (ch === "*" && next === "/") {
        out += "/";
        i += 1;
        inBlockCommentKeep = false;
      }
      continue;
    }
    if (inBlockCommentDrop) {
      if (ch === "*" && next === "/") {
        i += 1;
        inBlockCommentDrop = false;
      }
      continue;
    }
    if (inChar) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "'") inChar = false;
      continue;
    }
    if (inVerbatimString) {
      out += ch;
      if (ch === '"' && next === '"') {
        out += '"';
        i += 1;
        continue;
      }
      if (ch === '"') inVerbatimString = false;
      continue;
    }
    if (inString) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === '"') inString = false;
      continue;
    }
    if (ch === "'") {
      inChar = true;
      out += ch;
      continue;
    }
    if (ch === "$" && next === "@" && next2 === '"') {
      out += '$@"';
      i += 2;
      inVerbatimString = true;
      continue;
    }
    if (ch === "@" && next === "$" && next2 === '"') {
      out += '@$"';
      i += 2;
      inVerbatimString = true;
      continue;
    }
    if (ch === "@" && next === '"') {
      out += '@"';
      i += 1;
      inVerbatimString = true;
      continue;
    }
    if (ch === "$" && next === '"') {
      out += '$"';
      i += 1;
      inString = true;
      continue;
    }
    if (ch === '"') {
      out += '"';
      inString = true;
      continue;
    }
    if (ch === "/" && next === "/") {
      const startIndex = i;
      if (line <= licenseHeaderEndLine) {
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      if (headerLines.has(line) || /todo/i.test(lines[line - 1] ?? "")) {
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      i -= 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      const endIndex = content.indexOf("*/", i + 2);
      const blockText = endIndex === -1 ? content.slice(i) : content.slice(i, endIndex + 2);
      if (line <= licenseHeaderEndLine || headerLines.has(line) || /todo/i.test(blockText)) {
        out += "/*";
        i += 1;
        inBlockCommentKeep = true;
        continue;
      }
      i += 1;
      inBlockCommentDrop = true;
      continue;
    }
    out += ch;
  }
  return out;
}

function stripTsOrCsharpComments(content: string, lines: string[], language: CodeLanguage, licenseHeaderEndLine: number): string {
  let out = "";
  let inSingle = false;
  let inDouble = false;
  let inTemplate = false;
  let inBlockCommentKeep = false;
  let inBlockCommentDrop = false;
  let line = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    if (ch === "\n") {
      out += "\n";
      line += 1;
      continue;
    }
    if (inBlockCommentKeep) {
      out += ch;
      if (ch === "*" && next === "/") {
        out += "/";
        i += 1;
        inBlockCommentKeep = false;
      }
      continue;
    }
    if (inBlockCommentDrop) {
      if (ch === "*" && next === "/") {
        i += 1;
        inBlockCommentDrop = false;
      }
      continue;
    }
    if (!inDouble && !inTemplate && ch === "'" && !inSingle) {
      inSingle = true;
      out += ch;
      continue;
    }
    if (inSingle) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "'") inSingle = false;
      continue;
    }
    if (!inSingle && !inTemplate && ch === '"' && !inDouble) {
      inDouble = true;
      out += ch;
      continue;
    }
    if (inDouble) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === '"') inDouble = false;
      continue;
    }
    if (language === "typescript" && !inSingle && !inDouble && ch === "`" && !inTemplate) {
      inTemplate = true;
      out += ch;
      continue;
    }
    if (inTemplate) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "`") inTemplate = false;
      continue;
    }
    if (ch === "/" && next === "/") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      const keep = line <= licenseHeaderEndLine || after.includes("SPDX-License-Identifier:") || (language === "typescript" && isTsRegionLineText(after));
      if (keep) {
        const startIndex = i;
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      i -= 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      if (line <= licenseHeaderEndLine) {
        out += "/*";
        i += 1;
        inBlockCommentKeep = true;
        continue;
      }
      i += 1;
      inBlockCommentDrop = true;
      continue;
    }
    out += ch;
  }
  return out;
}

function stripPythonComments(content: string, lines: string[], licenseHeaderEndLine: number): string {
  const headerLines = getHeaderRegionLines(lines, "python");
  let out = "";
  let inSingle = false;
  let inDouble = false;
  let inTripleSingle = false;
  let inTripleDouble = false;
  let line = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      out += "\n";
      line += 1;
      continue;
    }
    if (inTripleSingle) {
      out += ch;
      if (ch === "'" && next === "'" && next2 === "'") {
        out += "''";
        i += 2;
        inTripleSingle = false;
      }
      continue;
    }
    if (inTripleDouble) {
      out += ch;
      if (ch === '"' && next === '"' && next2 === '"') {
        out += '""';
        i += 2;
        inTripleDouble = false;
      }
      continue;
    }
    if (inSingle) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "'") inSingle = false;
      continue;
    }
    if (inDouble) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === '"') inDouble = false;
      continue;
    }
    if (ch === "'" && next === "'" && next2 === "'") {
      out += "'''";
      i += 2;
      inTripleSingle = true;
      continue;
    }
    if (ch === '"' && next === '"' && next2 === '"') {
      out += '"""';
      i += 2;
      inTripleDouble = true;
      continue;
    }
    if (ch === "'") {
      inSingle = true;
      out += ch;
      continue;
    }
    if (ch === '"') {
      inDouble = true;
      out += ch;
      continue;
    }
    if (ch === "#") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(lineText.indexOf("#") + 1).trim();
      const keep = line <= licenseHeaderEndLine || headerLines.has(line) || after.includes("SPDX-License-Identifier:") || isPythonRegionLine(lineText) || /todo/i.test(after) || lineText.trimStart().startsWith("#!");
      if (keep) {
        const startIndex = i;
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      i -= 1;
      continue;
    }
    out += ch;
  }
  return out;
}

function writeReport(report: CodeReport): void {
  writeFileSync(reportPath, JSON.stringify(report, null, 2));
}

function run(): void {
  const issues: CodeIssue[] = [];
  let filesScanned = 0;
  const byKind: Record<CodeIssueKind, number> = {
    comment: 0,
    temporary_log: 0,
    missing_license_header: 0,
    invalid_header_format: 0,
    header_filepath_mismatch: 0,
    extra_dev_docs: 0,
    region_name_missing: 0,
    region_mismatch: 0,
    region_unclosed: 0,
    region_duplicate_sibling: 0,
    region_empty: 0,
    unreadable_file: 0,
    forbidden_import: 0,
    forbidden_terminology: 0,
  };
  const byLanguage: Record<CodeLanguage, number> = { typescript: 0, python: 0, csharp: 0, meta: 0 };
  const repoFiles = getRepoFiles().filter(shouldScan);
  const packageRoots = getPackageRootDirs(repoFiles);
  for (const docPath of repoFiles.filter((file) => isExtraDevDoc(file, packageRoots))) {
    const absolute = join(rootDir, docPath);
    if (fix) {
      try {
        if (existsSync(absolute)) unlinkSync(absolute);
      } catch {
        issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra dev doc detected but could not be deleted" }));
        byKind.extra_dev_docs += 1;
        byLanguage.meta += 1;
      }
      continue;
    }
    issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra README.md/AGENTS.md outside repo root" }));
    byKind.extra_dev_docs += 1;
    byLanguage.meta += 1;
  }
  for (const path of repoFiles) {
    const language = getLanguage(path);
    if (!language) continue;
    filesScanned += 1;
    byLanguage[language] += 1;
    const absolute = join(rootDir, path);
    let content = "";
    try {
      content = readFileSync(absolute, "utf-8");
    } catch {
      const issue = createIssue({ path, language, kind: "unreadable_file", line: 1, column: 1, message: "Failed to read file" });
      issues.push(issue);
      byKind[issue.kind] += 1;
      continue;
    }
    let lines = splitLines(content);
    const headerInfo = parseExistingHeader(lines, language, path);
    if (fix && !isConfigFile(path)) {
      const needsHeaderFix = !headerInfo.isValidFormat || headerInfo.existingFilepath !== path;
      if (needsHeaderFix) {
        content = fixHeader(content, lines, path, language);
        writeFileSync(absolute, content, "utf-8");
        lines = splitLines(content);
      }
    }
    let license = getLicenseHeaderEndLine(lines, language);
    if (fix && !isConfigFile(path)) {
      const updated =
        language === "python"
          ? stripPythonComments(content, lines, license.headerEndLine)
          : language === "typescript"
            ? stripTypescriptComments(content, lines, path, license.headerEndLine)
            : stripCsharpComments(content, lines, license.headerEndLine);
      if (updated !== content) {
        writeFileSync(absolute, updated, "utf-8");
        content = updated;
      }
      lines = splitLines(content);
      license = getLicenseHeaderEndLine(lines, language);
    }
    if (!isConfigFile(path)) {
      const updatedHeaderInfo = parseExistingHeader(lines, language, path);
      const expectedLicense = getLicenseType(path) === "LGPL" ? "LGPL-3.0" : "AGPL-3.0";
      if (!updatedHeaderInfo.isValidFormat) {
        const issue = createIssue({ path, language, kind: "invalid_header_format", line: 1, column: 1, message: `Header does not follow the required format (#region Header, filepath, contributor, ${expectedLicense})` });
        issues.push(issue);
        byKind[issue.kind] += 1;
      } else if (updatedHeaderInfo.existingFilepath !== path) {
        const issue = createIssue({ path, language, kind: "header_filepath_mismatch", line: 1, column: 1, message: `Header filepath mismatch: expected "${path}", found "${updatedHeaderInfo.existingFilepath}"` });
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
    }
    // #region EmptyRegionsUsage
    let emptyRegions = getEmptyRegions(lines, language);
    if (fix && !isConfigFile(path) && emptyRegions.regions.length > 0) {
      const updatedLines = lines.filter((_, index) => !emptyRegions.linesToRemove.has(index + 1));
      const updatedContent = updatedLines.join("\n");
      writeFileSync(absolute, updatedContent, "utf-8");
      content = updatedContent;
      lines = updatedLines;
      license = getLicenseHeaderEndLine(lines, language);
      emptyRegions = getEmptyRegions(lines, language);
    }
    for (const region of emptyRegions.regions) {
      const issue = createIssue({ path, language, kind: "region_empty", line: region.start, column: 1, message: `Empty region "${region.name}"`, excerpt: getExcerpt(lines[region.start - 1] ?? "") });
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    // #endregion EmptyRegionsUsage
    for (const issue of parseRegions(lines, language, path)) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    const tempLogIssues =
      language === "python"
        ? scanPythonTemporaryLogs(lines, path, license.headerEndLine)
        : language === "typescript"
          ? scanTypescriptTemporaryLogs(content, lines, path, license.headerEndLine)
          : scanCsharpTemporaryLogs(lines, path, license.headerEndLine);
    for (const issue of tempLogIssues) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    const commentIssues =
      language === "python"
        ? scanPythonComments(content, lines, path, license.headerEndLine)
        : language === "typescript"
          ? scanTypescriptComments(content, lines, path, license.headerEndLine)
          : scanCsharpComments(content, lines, path, license.headerEndLine);
    for (const issue of commentIssues) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    if (language === "typescript") {
      const forbiddenImportIssues = scanTypescriptForbiddenImports(content, lines, path);
      for (const issue of forbiddenImportIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      const forbiddenTerminologyIssues = scanTypescriptForbiddenTerminology(content, lines, path);
      for (const issue of forbiddenTerminologyIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
    }
  }
  const report: CodeReport = { timestamp: new Date().toISOString(), status: issues.length === 0 ? "success" : "error", summary: { filesScanned, issues: issues.length, byKind, byLanguage }, issues };
  writeReport(report);
  if (fix) process.exit(issues.some((issue) => issue.kind === "unreadable_file") ? 1 : 0);
  if (report.status === "success") process.exit(0);
  process.exit(1);
}

function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "error">("running");
  const [report, setReport] = React.useState<CodeReport | null>(null);

  React.useEffect(() => {
    const issues: CodeIssue[] = [];
    let filesScanned = 0;
    const byKind: Record<CodeIssueKind, number> = {
      comment: 0,
      temporary_log: 0,
      missing_license_header: 0,
      invalid_header_format: 0,
      header_filepath_mismatch: 0,
      extra_dev_docs: 0,
      region_name_missing: 0,
      region_mismatch: 0,
      region_unclosed: 0,
      region_duplicate_sibling: 0,
      region_empty: 0,
      unreadable_file: 0,
      forbidden_import: 0,
      forbidden_terminology: 0,
    };
    const byLanguage: Record<CodeLanguage, number> = { typescript: 0, python: 0, csharp: 0, meta: 0 };
    const repoFiles = getRepoFiles().filter(shouldScan);
    const packageRoots = getPackageRootDirs(repoFiles);
    for (const docPath of repoFiles.filter((file) => isExtraDevDoc(file, packageRoots))) {
      const absolute = join(rootDir, docPath);
      if (fix) {
        try {
          if (existsSync(absolute)) unlinkSync(absolute);
        } catch {
          issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra dev doc detected but could not be deleted" }));
          byKind.extra_dev_docs += 1;
          byLanguage.meta += 1;
        }
        continue;
      }
      issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra README.md/AGENTS.md outside repo root" }));
      byKind.extra_dev_docs += 1;
      byLanguage.meta += 1;
    }
    for (const path of repoFiles) {
      const language = getLanguage(path);
      if (!language) continue;
      filesScanned += 1;
      byLanguage[language] += 1;
      const absolute = join(rootDir, path);
      let content = "";
      try {
        content = readFileSync(absolute, "utf-8");
      } catch {
        const issue = createIssue({ path, language, kind: "unreadable_file", line: 1, column: 1, message: "Failed to read file" });
        issues.push(issue);
        byKind[issue.kind] += 1;
        continue;
      }
      let lines = splitLines(content);
      const headerInfo = parseExistingHeader(lines, language, path);
      if (fix && !isConfigFile(path)) {
        const needsHeaderFix = !headerInfo.isValidFormat || headerInfo.existingFilepath !== path;
        if (needsHeaderFix) {
          content = fixHeader(content, lines, path, language);
          writeFileSync(absolute, content, "utf-8");
          lines = splitLines(content);
        }
      }
      let license = getLicenseHeaderEndLine(lines, language);
      if (fix && !isConfigFile(path)) {
        const updated =
          language === "python"
            ? stripPythonComments(content, lines, license.headerEndLine)
            : language === "typescript"
              ? stripTypescriptComments(content, lines, path, license.headerEndLine)
              : stripCsharpComments(content, lines, license.headerEndLine);
        if (updated !== content) {
          writeFileSync(absolute, updated, "utf-8");
          content = updated;
        }
        lines = splitLines(content);
        license = getLicenseHeaderEndLine(lines, language);
      }
      if (!isConfigFile(path)) {
        const updatedHeaderInfo = parseExistingHeader(lines, language, path);
        const expectedLicense = getLicenseType(path) === "LGPL" ? "LGPL-3.0" : "AGPL-3.0";
        if (!updatedHeaderInfo.isValidFormat) {
          const issue = createIssue({ path, language, kind: "invalid_header_format", line: 1, column: 1, message: `Header does not follow the required format (#region Header, filepath, contributor, ${expectedLicense})` });
          issues.push(issue);
          byKind[issue.kind] += 1;
        } else if (updatedHeaderInfo.existingFilepath !== path) {
          const issue = createIssue({ path, language, kind: "header_filepath_mismatch", line: 1, column: 1, message: `Header filepath mismatch: expected "${path}", found "${updatedHeaderInfo.existingFilepath}"` });
          issues.push(issue);
          byKind[issue.kind] += 1;
        }
      }
      let emptyRegions = getEmptyRegions(lines, language);
      if (fix && !isConfigFile(path) && emptyRegions.regions.length > 0) {
        const updatedLines = lines.filter((_, index) => !emptyRegions.linesToRemove.has(index + 1));
        const updatedContent = updatedLines.join("\n");
        writeFileSync(absolute, updatedContent, "utf-8");
        content = updatedContent;
        lines = updatedLines;
        license = getLicenseHeaderEndLine(lines, language);
        emptyRegions = getEmptyRegions(lines, language);
      }
      for (const region of emptyRegions.regions) {
        const issue = createIssue({ path, language, kind: "region_empty", line: region.start, column: 1, message: `Empty region "${region.name}"`, excerpt: getExcerpt(lines[region.start - 1] ?? "") });
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      for (const issue of parseRegions(lines, language, path)) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      const tempLogIssues =
        language === "python"
          ? scanPythonTemporaryLogs(lines, path, license.headerEndLine)
          : language === "typescript"
            ? scanTypescriptTemporaryLogs(content, lines, path, license.headerEndLine)
            : scanCsharpTemporaryLogs(lines, path, license.headerEndLine);
      for (const issue of tempLogIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      const commentIssues =
        language === "python"
          ? scanPythonComments(content, lines, path, license.headerEndLine)
          : language === "typescript"
            ? scanTypescriptComments(content, lines, path, license.headerEndLine)
            : scanCsharpComments(content, lines, path, license.headerEndLine);
      for (const issue of commentIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      if (language === "typescript") {
        const forbiddenImportIssues = scanTypescriptForbiddenImports(content, lines, path);
        for (const issue of forbiddenImportIssues) {
          issues.push(issue);
          byKind[issue.kind] += 1;
        }
        const forbiddenTerminologyIssues = scanTypescriptForbiddenTerminology(content, lines, path);
        for (const issue of forbiddenTerminologyIssues) {
          issues.push(issue);
          byKind[issue.kind] += 1;
        }
      }
    }
    const finalReport: CodeReport = { timestamp: new Date().toISOString(), status: issues.length === 0 ? "success" : "error", summary: { filesScanned, issues: issues.length, byKind, byLanguage }, issues };
    writeReport(finalReport);
    setReport(finalReport);
    setStatus(finalReport.status === "success" ? "success" : "error");
    const exitCode = fix ? (issues.some((issue) => issue.kind === "unreadable_file") ? 1 : 0) : finalReport.status === "success" ? 0 : 1;
    setTimeout(() => process.exit(exitCode), 100);
  }, []);

  return (
    <Box flexDirection="column">
      <Text>{fix ? "🔧 Fixing codebase..." : "🔍 Analyzing codebase..."}</Text>
      {report && (
        <>
          <Text>
            {status === "success" ? "✅" : "❌"} {report.summary.filesScanned} files, {report.summary.issues} issues
          </Text>
          <Text dimColor>📝 Report: {reportPath}</Text>
        </>
      )}
    </Box>
  );
}

render(<App />);

// #endregion Issues

// #region Ticket

import { mkdirSync } from "fs";
import matter from "gray-matter";

//#region Types
interface Lines {
  added: number;
  removed: number;
}

interface FileEntry {
  path: string;
  lines: Lines;
}

interface Files {
  updated: FileEntry[];
  created: FileEntry[];
  removed: FileEntry[];
}

interface IterationDate {
  started: string;
  ended?: string;
}

interface Iteration {
  prompt: string;
  date: IterationDate;
  model: string;
  author: string;
  commit?: string;
  files: Files;
  lines: Lines;
}

interface TicketDate {
  created: string;
  finished?: string;
}

interface TicketFrontmatter {
  slug: string;
  summary?: string;
  prompt: string;
  status: "open" | "finished";
  author: string;
  date: TicketDate;
  commit?: string;
  model?: string;
  iterations: Iteration[];
  files?: Files;
  lines?: Lines;
}

interface Ticket {
  frontmatter: TicketFrontmatter;
  content: string;
  path: string;
}

interface TicketCreateInput {
  slug: string;
  prompt: string;
  content?: string;
}

interface IterationStartInput {
  prompt: string;
  model: string;
  files: { updated?: string[]; created?: string[]; removed?: string[] };
}

interface IterationFinishInput {
  files: { updated?: string[]; created?: string[]; removed?: string[] };
}

interface ListOptions {
  year?: number;
  month?: number;
  day?: number;
  slug?: string;
}

interface SearchOptions extends ListOptions {
  query?: string;
  limit?: number;
}
//#endregion Types

//#region Exports
export type { FileEntry, Files, Iteration, IterationDate, IterationFinishInput, IterationStartInput, Lines, ListOptions, SearchOptions, Ticket, TicketCreateInput, TicketDate, TicketFrontmatter };

export { createTicket, deleteTicket, finishIteration, finishTicket, listTickets, readTicket, reopenTicket, searchTickets, startIteration };
//#endregion Exports

//#region Configuration
const LOG_ROOT = join(process.cwd(), "log");

export enum Model {
  COMPOSER_1 = "composer-1",
  CLAUDE_SONNET_4_5 = "claude-sonnet-4-5",
  CLAUDE_OPUS_4_5 = "claude-opus-4-5",
  GPT_5_1_CODEX_MAX = "gpt-5.1-codex-max",
  GPT_5_2_CODEX = "gpt-5.2-codex",
}

function getGitConfig(key: string): string {
  try {
    return execSync(`git config --get ${key}`, { encoding: "utf-8" }).trim();
  } catch {
    return "";
  }
}

function getGitAuthor(): string {
  const name = getGitConfig("user.name");
  const email = getGitConfig("user.email");
  if (name && email) return `${name} <${email}>`;
  if (name) return name;
  if (email) return email;
  return "Unknown";
}

function getGitHead(): string {
  return execSync("git rev-parse HEAD", { encoding: "utf-8" }).trim();
}
//#endregion Configuration

//#region Path Utilities
function getTicketPath(year: number, month: number, day: number, slug: string): string {
  const monthStr = month.toString().padStart(2, "0");
  const dayStr = day.toString().padStart(2, "0");
  return join(LOG_ROOT, "tickets", year.toString(), monthStr, dayStr, `${slug}.md`);
}

function parseTicketPath(path: string): { year: number; month: number; day: number; slug: string } | null {
  const relativePath = path.replace(LOG_ROOT, "").replace(/\\/g, "/");
  const match = relativePath.match(/^\/tickets\/(\d{4})\/(\d{2})\/(\d{2})\/(.+)\.md$/);
  if (!match) return null;
  return {
    year: parseInt(match[1]),
    month: parseInt(match[2]),
    day: parseInt(match[3]),
    slug: match[4],
  };
}

function ensureDirectoryExists(filePath: string): void {
  const dir = dirname(filePath);
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
}
//#endregion Path Utilities

//#region Git Stats
function getGitStatusPorcelain(): string {
  return execSync("git status --porcelain", { encoding: "utf-8" });
}

function parseGitPath(rawPath: string): string {
  const trimmed = rawPath.trim();
  if (!trimmed) return "";
  if (trimmed.startsWith('"') && trimmed.endsWith('"')) {
    try {
      return JSON.parse(trimmed);
    } catch {
      return trimmed.slice(1, -1);
    }
  }
  return trimmed;
}

function getUntrackedFiles(): Set<string> {
  const files = new Set<string>();
  for (const line of getGitStatusPorcelain().split(/\r?\n/)) {
    if (!line.startsWith("?? ")) continue;
    const path = parseGitPath(line.slice(3));
    if (path) files.add(path);
  }
  return files;
}

function quoteGitPath(path: string): string {
  return '"' + path.replaceAll('"', '\\"') + '"';
}

function parseGitNumstatOutput(output: string): Lines {
  let added = 0;
  let removed = 0;
  for (const line of output.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const parts = trimmed.split("\t");
    if (parts.length < 2) continue;
    const a = parts[0] === "-" ? 0 : parseInt(parts[0]);
    const r = parts[1] === "-" ? 0 : parseInt(parts[1]);
    if (!Number.isNaN(a)) added += a;
    if (!Number.isNaN(r)) removed += r;
  }
  return { added, removed };
}

function computeGitLinesForFile(filePath: string, base?: string): Lines {
  const untracked = getUntrackedFiles();
  const nullPath = process.platform === "win32" ? "NUL" : "/dev/null";
  if (untracked.has(filePath)) {
    try {
      const output = execSync(`git diff --no-index --numstat -- ${nullPath} ${quoteGitPath(filePath)}`, { encoding: "utf-8" });
      return parseGitNumstatOutput(output);
    } catch (error: any) {
      if (error && error.stdout) {
        const stdout = Buffer.isBuffer(error.stdout) ? error.stdout.toString("utf-8") : String(error.stdout);
        return parseGitNumstatOutput(stdout);
      }
      return { added: 0, removed: 0 };
    }
  }
  const ref = base || "HEAD";
  try {
    const output = execSync(`git diff --numstat ${ref} -- ${quoteGitPath(filePath)}`, { encoding: "utf-8" });
    return parseGitNumstatOutput(output);
  } catch {
    return { added: 0, removed: 0 };
  }
}

function computeGitLinesForFiles(filePaths: string[], base?: string): Lines {
  let added = 0;
  let removed = 0;
  for (const filePath of filePaths) {
    const lines = computeGitLinesForFile(filePath, base);
    added += lines.added;
    removed += lines.removed;
  }
  return { added, removed };
}

function buildFilesWithLines(input: { updated?: string[]; created?: string[]; removed?: string[] }, base?: string): Files {
  const updated: FileEntry[] = (input.updated || []).map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const created: FileEntry[] = (input.created || []).map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const removed: FileEntry[] = (input.removed || []).map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  return { updated, created, removed };
}

function computeTotalLines(files: Files): Lines {
  let added = 0;
  let removed = 0;
  for (const f of files.updated) {
    added += f.lines.added;
    removed += f.lines.removed;
  }
  for (const f of files.created) {
    added += f.lines.added;
    removed += f.lines.removed;
  }
  for (const f of files.removed) {
    added += f.lines.added;
    removed += f.lines.removed;
  }
  return { added, removed };
}
//#endregion Git Stats

//#region Validation
function requireFiles(files: { updated?: string[]; created?: string[]; removed?: string[] }): void {
  const hasAny = Boolean((files.updated && files.updated.length) || (files.created && files.created.length) || (files.removed && files.removed.length));
  if (!hasAny) throw new Error("Missing required files: provide at least one of --file=, --file-created=, --file-removed=");
}

function validateModel(model: string): string {
  const values = Object.values(Model);
  if (!values.includes(model as any)) throw new Error(`Unknown model: ${model}. Add it to the Model enum in scripts/log.ts.`);
  return model;
}
//#endregion Validation

//#region Serialization
function sanitizeForMatter(value: any): any {
  if (value === undefined) return undefined;
  if (value === null) return null;
  if (Array.isArray(value)) return value.map((v) => sanitizeForMatter(v)).filter((v) => v !== undefined);
  if (typeof value === "object") {
    const out: any = {};
    for (const [key, v] of Object.entries(value)) {
      const sanitized = sanitizeForMatter(v);
      if (sanitized !== undefined) out[key] = sanitized;
    }
    return out;
  }
  return value;
}

function serializeFileEntry(entry: FileEntry): any {
  return { [entry.path]: { lines: entry.lines } };
}

function serializeFiles(files: Files): any {
  return {
    updated: files.updated.map(serializeFileEntry),
    created: files.created.map((f) => f.path),
    removed: files.removed.map((f) => f.path),
  };
}

function serializeIteration(iteration: Iteration): any {
  return {
    prompt: iteration.prompt,
    date: iteration.date,
    model: iteration.model,
    author: iteration.author,
    commit: iteration.commit,
    files: serializeFiles(iteration.files),
    lines: iteration.lines,
  };
}

function serializeFrontmatter(fm: TicketFrontmatter): any {
  const result: any = {
    slug: fm.slug,
    prompt: fm.prompt,
    status: fm.status,
    author: fm.author,
    date: fm.date,
  };
  if (fm.summary) result.summary = fm.summary;
  if (fm.commit) result.commit = fm.commit;
  if (fm.model) result.model = fm.model;
  result.iterations = fm.iterations.map(serializeIteration);
  if (fm.files) result.files = serializeFiles(fm.files);
  if (fm.lines) result.lines = fm.lines;
  return result;
}

function deserializeFileEntry(entry: any): FileEntry {
  if (typeof entry === "string") return { path: entry, lines: { added: 0, removed: 0 } };
  const keys = Object.keys(entry);
  if (keys.length === 1) {
    const path = keys[0];
    const value = entry[path];
    return { path, lines: value?.lines || { added: 0, removed: 0 } };
  }
  return { path: entry.path || "", lines: entry.lines || { added: 0, removed: 0 } };
}

function deserializeFiles(raw: any): Files {
  if (!raw) return { updated: [], created: [], removed: [] };
  const updated = (raw.updated || []).map(deserializeFileEntry);
  const created = (raw.created || []).map((e: any) => (typeof e === "string" ? { path: e, lines: { added: 0, removed: 0 } } : deserializeFileEntry(e)));
  const removed = (raw.removed || []).map((e: any) => (typeof e === "string" ? { path: e, lines: { added: 0, removed: 0 } } : deserializeFileEntry(e)));
  return { updated, created, removed };
}

function deserializeIteration(raw: any): Iteration {
  const date: IterationDate = typeof raw.date === "string" ? { started: raw.date } : { started: raw.date?.started || "", ended: raw.date?.ended };
  return {
    prompt: raw.prompt || "",
    date,
    model: raw.model || "unknown",
    author: raw.author || "Unknown",
    commit: raw.commit,
    files: deserializeFiles(raw.files),
    lines: raw.lines || { added: 0, removed: 0 },
  };
}

function deserializeFrontmatter(raw: any): TicketFrontmatter {
  const date: TicketDate = typeof raw.date === "string" ? { created: raw.date } : { created: raw.date?.created || new Date().toISOString(), finished: raw.date?.finished };
  const summary = typeof raw.summary === "string" && raw.summary.trim() ? raw.summary : undefined;
  const prompt = typeof raw.prompt === "string" ? raw.prompt : "";
  if (!prompt.trim()) throw new Error(`Missing required ticket prompt for: ${raw.slug || "UNKNOWN"}. Add 'prompt' to the ticket frontmatter.`);
  const status: "open" | "finished" = raw.status || "open";
  if (status === "finished" && !summary) throw new Error(`Missing required ticket summary for finished ticket: ${raw.slug || "UNKNOWN"}.`);
  return {
    slug: raw.slug || "UNKNOWN",
    summary,
    prompt,
    status,
    author: raw.author || "Unknown",
    date,
    commit: raw.commit,
    model: raw.model,
    iterations: (raw.iterations || []).map(deserializeIteration),
    files: raw.files ? deserializeFiles(raw.files) : undefined,
    lines: raw.lines,
  };
}
//#endregion Serialization

//#region CRUD Operations
function createTicket(input: TicketCreateInput): Ticket {
  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth() + 1;
  const day = now.getDate();
  const slug = input.slug.toUpperCase();
  const ticketPath = getTicketPath(year, month, day, slug);
  if (existsSync(ticketPath)) throw new Error(`Ticket already exists: ${ticketPath}`);
  const frontmatter: TicketFrontmatter = {
    slug,
    prompt: input.prompt,
    status: "open",
    author: getGitAuthor(),
    date: { created: now.toISOString() },
    iterations: [],
  };
  const content = input.content || "# Previously\n\n# Plan\n\n# Changes\n";
  const fileContent = matter.stringify(content, sanitizeForMatter(serializeFrontmatter(frontmatter)));
  ensureDirectoryExists(ticketPath);
  writeFileSync(ticketPath, fileContent, "utf-8");
  return { frontmatter, content, path: ticketPath };
}

function migrateTicketPromptsFromFirstIteration(): { migrated: number; skipped: number } {
  let migrated = 0;
  let skipped = 0;
  const ticketPaths = listTicketPaths();
  for (const ticketPath of ticketPaths) {
    const fileContent = readFileSync(ticketPath, "utf-8");
    const parsed = matter(fileContent);
    const data: any = parsed.data || {};
    const currentPrompt = typeof data.prompt === "string" ? data.prompt.trim() : "";
    if (currentPrompt) {
      skipped++;
      continue;
    }
    const iterations = Array.isArray(data.iterations) ? data.iterations : [];
    const firstIterationPrompt = typeof iterations[0]?.prompt === "string" ? iterations[0].prompt.trim() : "";
    const nextPrompt = firstIterationPrompt || (typeof data.summary === "string" ? data.summary.trim() : "") || (typeof data.slug === "string" ? data.slug.trim() : "") || "UNKNOWN";
    if (!nextPrompt.trim()) throw new Error(`Cannot migrate ticket prompt (empty fallback): ${ticketPath}`);
    const nextData: any = {
      slug: data.slug,
      summary: data.summary,
      prompt: nextPrompt,
      status: data.status,
      author: data.author,
      date: data.date,
      commit: data.commit,
      model: data.model,
      iterations: data.iterations,
      files: data.files,
      lines: data.lines,
    };
    const rewritten = matter.stringify(parsed.content, sanitizeForMatter(nextData));
    writeFileSync(ticketPath, rewritten, "utf-8");
    migrated++;
  }
  return { migrated, skipped };
}

function readTicket(year: number, month: number, day: number, slug: string): Ticket {
  const ticketPath = getTicketPath(year, month, day, slug);
  if (!existsSync(ticketPath)) throw new Error(`Ticket not found: ${ticketPath}`);
  const fileContent = readFileSync(ticketPath, "utf-8");
  const parsed = matter(fileContent);
  return {
    frontmatter: deserializeFrontmatter(parsed.data),
    content: parsed.content,
    path: ticketPath,
  };
}

function writeTicket(ticket: Ticket): void {
  const fileContent = matter.stringify(ticket.content, sanitizeForMatter(serializeFrontmatter(ticket.frontmatter)));
  writeFileSync(ticket.path, fileContent, "utf-8");
}

function startIteration(year: number, month: number, day: number, slug: string, input: IterationStartInput): Ticket {
  requireFiles(input.files);
  const ticket = readTicket(year, month, day, slug);
  const iterations = ticket.frontmatter.iterations;
  if (iterations.length > 0) {
    const last = iterations[iterations.length - 1];
    if (!last.date.ended) throw new Error(`Cannot start a new iteration while the latest iteration is unfinished (started: ${last.date.started})`);
  }
  const now = new Date().toISOString();
  const files = buildFilesWithLines(input.files);
  const lines = computeTotalLines(files);
  const newIteration: Iteration = {
    prompt: input.prompt,
    date: { started: now },
    model: input.model,
    author: getGitAuthor(),
    files,
    lines,
  };
  ticket.frontmatter.iterations.push(newIteration);
  writeTicket(ticket);
  return ticket;
}

function finishIteration(year: number, month: number, day: number, slug: string, input: IterationFinishInput): Ticket {
  requireFiles(input.files);
  const ticket = readTicket(year, month, day, slug);
  const iterations = ticket.frontmatter.iterations;
  if (iterations.length === 0) throw new Error(`No iterations found for ticket: ${slug}`);
  const lastIteration = iterations[iterations.length - 1];
  if (lastIteration.date.ended) throw new Error(`Latest iteration already finished at: ${lastIteration.date.ended}`);
  const now = new Date().toISOString();
  const commit = getGitHead();
  const files = buildFilesWithLines(input.files);
  const lines = computeTotalLines(files);
  lastIteration.date.ended = now;
  lastIteration.commit = commit;
  lastIteration.files = files;
  lastIteration.lines = lines;
  writeTicket(ticket);
  return ticket;
}

function finishTicket(year: number, month: number, day: number, slug: string, summary: string): Ticket {
  if (!summary || !summary.trim()) throw new Error("Missing required ticket summary.");
  const ticket = readTicket(year, month, day, slug);
  const iterations = ticket.frontmatter.iterations;
  if (iterations.length === 0) throw new Error(`Cannot finish ticket without any iterations: ${slug}`);
  const last = iterations[iterations.length - 1];
  if (!last.date.ended) throw new Error(`Cannot finish ticket while the latest iteration is unfinished (started: ${last.date.started})`);
  const updatedPaths = new Set<string>();
  const createdPaths = new Set<string>();
  const removedPaths = new Set<string>();
  for (const iteration of iterations) {
    for (const f of iteration.files.updated) updatedPaths.add(f.path);
    for (const f of iteration.files.created) createdPaths.add(f.path);
    for (const f of iteration.files.removed) removedPaths.add(f.path);
  }
  const allPaths = [...updatedPaths, ...createdPaths, ...removedPaths];
  const base = getGitHead();
  const updated: FileEntry[] = [...updatedPaths].sort().map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const created: FileEntry[] = [...createdPaths].sort().map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const removed: FileEntry[] = [...removedPaths].sort().map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const files: Files = { updated, created, removed };
  const lines = computeGitLinesForFiles(allPaths, base);
  const now = new Date().toISOString();
  ticket.frontmatter.status = "finished";
  ticket.frontmatter.summary = summary;
  ticket.frontmatter.date.finished = now;
  ticket.frontmatter.commit = getGitHead();
  ticket.frontmatter.model = last.model;
  ticket.frontmatter.files = files;
  ticket.frontmatter.lines = lines;
  writeTicket(ticket);
  return ticket;
}

function reopenTicket(year: number, month: number, day: number, slug: string): Ticket {
  const ticket = readTicket(year, month, day, slug);
  if (ticket.frontmatter.status === "open") throw new Error(`Ticket is already open: ${slug}`);
  ticket.frontmatter.status = "open";
  delete ticket.frontmatter.date.finished;
  delete ticket.frontmatter.commit;
  delete ticket.frontmatter.model;
  delete ticket.frontmatter.files;
  delete ticket.frontmatter.lines;
  writeTicket(ticket);
  return ticket;
}

function addPlanToTicket(year: number, month: number, day: number, slug: string, planFilePath: string): Ticket {
  if (!existsSync(planFilePath)) throw new Error(`Plan file not found: ${planFilePath}`);
  const planContent = readFileSync(planFilePath, "utf-8").trim();
  const ticket = readTicket(year, month, day, slug);
  const content = ticket.content;
  const planHeaderRegex = /^# Plan\s*$/m;
  const match = content.match(planHeaderRegex);
  if (!match) throw new Error(`No "# Plan" section found in ticket: ${slug}`);
  const planHeaderIndex = match.index!;
  const afterPlanHeader = planHeaderIndex + match[0].length;
  const nextSectionRegex = /^# /m;
  const remainingContent = content.slice(afterPlanHeader);
  const nextSectionMatch = remainingContent.match(nextSectionRegex);
  let newContent: string;
  if (nextSectionMatch) {
    const nextSectionIndex = afterPlanHeader + nextSectionMatch.index!;
    newContent = content.slice(0, afterPlanHeader) + "\n\n" + planContent + "\n\n" + content.slice(nextSectionIndex);
  } else {
    newContent = content.slice(0, afterPlanHeader) + "\n\n" + planContent + "\n";
  }
  ticket.content = newContent;
  writeTicket(ticket);
  return ticket;
}

function deleteTicket(year: number, month: number, day: number, slug: string): void {
  const ticketPath = getTicketPath(year, month, day, slug);
  if (!existsSync(ticketPath)) throw new Error(`Ticket not found: ${ticketPath}`);
  unlinkSync(ticketPath);
}

function listTickets(options: ListOptions = {}): Ticket[] {
  const tickets: Ticket[] = [];
  const invalidPaths: string[] = [];
  const paths = listTicketPaths(options);
  for (const path of paths) {
    try {
      const fileContent = readFileSync(path, "utf-8");
      const matterParsed = matter(fileContent);
      tickets.push({
        frontmatter: deserializeFrontmatter(matterParsed.data),
        content: matterParsed.content,
        path,
      });
    } catch {
      invalidPaths.push(path);
    }
  }
  if (invalidPaths.length > 0) {
    const head = invalidPaths.slice(0, 10).join("\n");
    throw new Error(`Failed to parse ${invalidPaths.length} ticket(s). First 10:\n${head}`);
  }
  return tickets.sort((a, b) => new Date(b.frontmatter.date.created).getTime() - new Date(a.frontmatter.date.created).getTime());
}

function listTicketPaths(options: ListOptions = {}): string[] {
  const paths: string[] = [];
  function walk(dir: string): void {
    if (!existsSync(dir)) return;
    const entries = readdirSync(dir);
    for (const entry of entries) {
      const fullPath = join(dir, entry);
      const stat = statSync(fullPath);
      if (stat.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (!entry.endsWith(".md") || entry === "prompts.md") continue;
      const parsed = parseTicketPath(fullPath);
      if (!parsed) continue;
      if (options.year !== undefined && parsed.year !== options.year) continue;
      if (options.month !== undefined && parsed.month !== options.month) continue;
      if (options.day !== undefined && parsed.day !== options.day) continue;
      if (options.slug !== undefined && parsed.slug !== options.slug) continue;
      paths.push(fullPath);
    }
  }
  walk(join(LOG_ROOT, "tickets"));
  return paths;
}

function searchTickets(options: SearchOptions = {}): Ticket[] {
  const allTickets = listTickets({ year: options.year, month: options.month, day: options.day, slug: options.slug });
  if (!options.query) return options.limit ? allTickets.slice(0, options.limit) : allTickets;
  const query = options.query.toLowerCase();
  const matchedTickets = allTickets.filter((ticket) => {
    const slugMatch = ticket.frontmatter.slug.toLowerCase().includes(query);
    const summaryMatch = (ticket.frontmatter.summary || "").toLowerCase().includes(query);
    const contentMatch = ticket.content.toLowerCase().includes(query);
    const authorMatch = ticket.frontmatter.author.toLowerCase().includes(query);
    return slugMatch || summaryMatch || contentMatch || authorMatch;
  });
  return options.limit ? matchedTickets.slice(0, options.limit) : matchedTickets;
}
//#endregion CRUD Operations

//#region Lookup
function findLatestTicketBySlug(slug: string): { year: number; month: number; day: number; slug: string } {
  const normalizedSlug = slug.toUpperCase();
  const tickets = listTickets({ slug: normalizedSlug });
  const latest = tickets[0];
  if (!latest) throw new Error(`No ticket found for slug: ${normalizedSlug}`);
  const parsed = parseTicketPath(latest.path);
  if (!parsed) throw new Error(`Failed to parse ticket path: ${latest.path}`);
  return parsed;
}
//#endregion Lookup

//#region CLI
function printUsage(): void {
  const usage = `
Usage: tsx scripts/log.ts <command> [options]

Commands:
  ticket create <slug>                 Create a ticket (no iterations)
                                       Required: --prompt="..."
  ticket iteration start <slug>        Start a new iteration on a ticket
                                       Required: --model=MODEL --prompt="..." and at least one file flag
  ticket iteration finish <slug>       Finish the latest iteration
                                       Required: at least one file flag
  ticket finish <slug>                 Finish the ticket (requires latest iteration finished)
                                       Required: --summary="..."
  ticket reopen <slug>                 Reopen a finished ticket (removes total files/lines)
  ticket plan <slug>                   Add a plan to the ticket from a markdown file
                                       Required: --plan=path/to/plan.md
  ticket migrate prompts               Backfill missing ticket prompts from the first iteration prompt
  ticket read <year> <month> <day> <slug>     Read a ticket
  ticket delete <year> <month> <day> <slug>   Delete a ticket
  ticket list [year] [month] [day]            List tickets (optionally filtered)
  ticket search [query] [--limit=N]           Search tickets
                                             Optional: --year=YYYY --month=MM --day=DD --limit=N
  models                               List available model enum values

File Flags:
  --file=PATH              Add file to updated list
  --file-created=PATH      Add file to created list
  --file-removed=PATH      Add file to removed list

Ticket Schema:
  slug, summary, prompt, status, author, date{created, finished}, commit, model,
  iterations[{prompt, date{started, ended}, model, author, commit, files{...}, lines{...}}],
  files{updated[{path, lines}], created[path], removed[path]}, lines{added, removed}

  - author, commit, date, lines: derived from git (forbidden to set manually)
  - model, files: must be set manually
  - when ticket is finished: files and lines are computed from git

Workflow:
  1. Create a ticket: ticket create <slug> --prompt="..."
  2. Start iteration: ticket iteration start <slug> --model=MODEL --prompt="..." --file=...
  3. Finish iteration: ticket iteration finish <slug> --file=...
  4. Finish ticket: ticket finish <slug> --summary="..."

Examples:
  tsx scripts/log.ts ticket create MY-TASK --prompt="User request..."
  tsx scripts/log.ts ticket iteration start MY-TASK --model=${Model.CLAUDE_OPUS_4_5} --prompt="User request..." --file=scripts/log.ts
  tsx scripts/log.ts ticket iteration finish MY-TASK --file=scripts/log.ts --file=README.md
  tsx scripts/log.ts ticket finish MY-TASK --summary="Implement new feature"
  tsx scripts/log.ts ticket reopen MY-TASK
  tsx scripts/log.ts ticket plan MY-TASK --plan=docs/plan.md
  tsx scripts/log.ts ticket read 2025 12 16 MY-TASK
  tsx scripts/log.ts ticket list 2025 12
  tsx scripts/log.ts ticket search "drag drop"
  tsx scripts/log.ts ticket migrate prompts
  tsx scripts/log.ts models
`;
  render(
    <Box flexDirection="column">
      <Text>{usage}</Text>
    </Box>,
  );
}

function parseFlag(args: string[], name: string): string | undefined {
  const prefix = `--${name}=`;
  const match = args.find((arg) => arg.startsWith(prefix));
  if (!match) return undefined;
  return match.slice(prefix.length);
}

function parseFlags(args: string[], name: string): string[] {
  const prefix = `--${name}=`;
  const values: string[] = [];
  for (const arg of args) {
    if (!arg.startsWith(prefix)) continue;
    values.push(arg.slice(prefix.length));
  }
  return values;
}

function requireFlag(args: string[], name: string): string {
  const value = parseFlag(args, name);
  if (!value) throw new Error(`Missing required flag: --${name}=`);
  return value;
}

function parseFilesFromFlags(flags: string[]): { updated?: string[]; created?: string[]; removed?: string[] } {
  const filesUpdated = parseFlags(flags, "file");
  const filesCreated = parseFlags(flags, "file-created");
  const filesRemoved = parseFlags(flags, "file-removed");
  return {
    updated: filesUpdated.length > 0 ? filesUpdated : undefined,
    created: filesCreated.length > 0 ? filesCreated : undefined,
    removed: filesRemoved.length > 0 ? filesRemoved : undefined,
  };
}

const args = process.argv.slice(2);
const command = args[0];
try {
  switch (command) {
    case "ticket": {
      const [, sub, ...rest] = args;
      if (!sub) {
        render(
          <Box flexDirection="column">
            <Text color="red">Error: Missing ticket command</Text>
          </Box>,
        );
        printUsage();
        process.exit(1);
      }
      if (sub === "create") {
        const [slug, ...flags] = rest;
        if (!slug) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        for (const flag of flags) {
          if (!flag.startsWith("--")) throw new Error(`Unexpected argument: ${flag}. Summary is set on ticket finish via --summary=.`);
        }
        const prompt = requireFlag(flags, "prompt");
        const ticket = createTicket({ slug, prompt });
        render(
          <Box flexDirection="column">
            <Text>✅ Created ticket: {ticket.path}</Text>
          </Box>,
        );
        break;
      }

      if (sub === "migrate") {
        const [migrationCommand] = rest;
        if (migrationCommand === "prompts") {
          const result = migrateTicketPromptsFromFirstIteration();
          render(
            <Box flexDirection="column">
              <Text>
                ✅ Migrated ticket prompts: {result.migrated} (skipped: {result.skipped})
              </Text>
            </Box>,
          );
          break;
        }
        render(
          <Box flexDirection="column">
            <Text color="red">Error: Unknown migrate command</Text>
          </Box>,
        );
        printUsage();
        process.exit(1);
        break;
      }
      if (sub === "iteration") {
        const [iterationCommand, slugArg, ...flags] = rest;
        if (iterationCommand === "start") {
          if (!slugArg) {
            render(
              <Box flexDirection="column">
                <Text color="red">Error: Missing slug</Text>
              </Box>,
            );
            printUsage();
            process.exit(1);
          }
          const model = validateModel(requireFlag(flags, "model"));
          const prompt = requireFlag(flags, "prompt");
          const files = parseFilesFromFlags(flags);
          const latest = findLatestTicketBySlug(slugArg);
          const ticket = startIteration(latest.year, latest.month, latest.day, latest.slug, { prompt, model, files });
          render(
            <Box flexDirection="column">
              <Text>
                ✅ Started iteration {ticket.frontmatter.iterations.length} for ticket: {ticket.path}
              </Text>
            </Box>,
          );
          break;
        }
        if (iterationCommand === "finish") {
          if (!slugArg) {
            render(
              <Box flexDirection="column">
                <Text color="red">Error: Missing slug</Text>
              </Box>,
            );
            printUsage();
            process.exit(1);
          }
          const files = parseFilesFromFlags(flags);
          const latest = findLatestTicketBySlug(slugArg);
          const ticket = finishIteration(latest.year, latest.month, latest.day, latest.slug, { files });
          const lastIteration = ticket.frontmatter.iterations[ticket.frontmatter.iterations.length - 1];
          render(
            <Box flexDirection="column">
              <Text>✅ Finished iteration for ticket: {ticket.path}</Text>
              <Text dimColor>Commit: {lastIteration.commit || "none"}</Text>
              <Text dimColor>
                Lines: +{lastIteration.lines.added} -{lastIteration.lines.removed}
              </Text>
            </Box>,
          );
          break;
        }
        render(
          <Box flexDirection="column">
            <Text color="red">Error: Unknown iteration command</Text>
          </Box>,
        );
        printUsage();
        process.exit(1);
        break;
      }
      if (sub === "finish") {
        const [slugArg, ...flags] = rest;
        if (!slugArg) {
          console.error("Error: Missing slug");
          printUsage();
          process.exit(1);
        }
        for (const flag of flags) {
          if (!flag.startsWith("--")) throw new Error(`Unexpected argument: ${flag}`);
        }
        const summary = requireFlag(flags, "summary");
        const latest = findLatestTicketBySlug(slugArg);
        const ticket = finishTicket(latest.year, latest.month, latest.day, latest.slug, summary);
        render(
          <Box flexDirection="column">
            <Text>✅ Finished ticket: {ticket.path}</Text>
            <Text dimColor>Commit: {ticket.frontmatter.commit}</Text>
            {ticket.frontmatter.lines && (
              <Text dimColor>
                Lines: +{ticket.frontmatter.lines.added} -{ticket.frontmatter.lines.removed}
              </Text>
            )}
          </Box>,
        );
        break;
      }
      if (sub === "reopen") {
        const [slugArg] = rest;
        if (!slugArg) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        const latest = findLatestTicketBySlug(slugArg);
        const ticket = reopenTicket(latest.year, latest.month, latest.day, latest.slug);
        render(
          <Box flexDirection="column">
            <Text>✅ Reopened ticket: {ticket.path}</Text>
            <Text dimColor>Status: {ticket.frontmatter.status}</Text>
            <Text dimColor>Iterations preserved: {ticket.frontmatter.iterations.length}</Text>
          </Box>,
        );
        break;
      }
      if (sub === "plan") {
        const [slugArg, ...flags] = rest;
        if (!slugArg) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        const planPath = requireFlag(flags, "plan");
        const latest = findLatestTicketBySlug(slugArg);
        const ticket = addPlanToTicket(latest.year, latest.month, latest.day, latest.slug, planPath);
        render(
          <Box flexDirection="column">
            <Text>✅ Added plan to ticket: {ticket.path}</Text>
          </Box>,
        );
        break;
      }
      if (sub === "read") {
        const [year, month, day, slug] = rest;
        if (!year || !month || !day || !slug) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing year, month, day, or slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
        render(
          <Box flexDirection="column">
            <Text>📄 Path: {ticket.path}</Text>
            <Text>Slug: {ticket.frontmatter.slug}</Text>
            <Text>Summary: {ticket.frontmatter.summary || ""}</Text>
            <Text>Status: {ticket.frontmatter.status}</Text>
            <Text>Author: {ticket.frontmatter.author}</Text>
            <Text>Created: {ticket.frontmatter.date.created}</Text>
            {ticket.frontmatter.date.finished && <Text>Finished: {ticket.frontmatter.date.finished}</Text>}
            {ticket.frontmatter.commit && <Text>Commit: {ticket.frontmatter.commit}</Text>}
            {ticket.frontmatter.model && <Text>Model: {ticket.frontmatter.model}</Text>}
            <Text>Iterations: {ticket.frontmatter.iterations.length}</Text>
            {ticket.frontmatter.iterations.map((it, i) => {
              const totalFiles = it.files.updated.length + it.files.created.length + it.files.removed.length;
              return (
                <Box key={i} flexDirection="column" marginTop={1}>
                  <Text dimColor>
                    {" "}
                    [{i + 1}] {it.date.started}
                  </Text>
                  <Text dimColor> Model: {it.model}</Text>
                  <Text dimColor> Author: {it.author}</Text>
                  {it.date.ended && <Text dimColor> Ended: {it.date.ended}</Text>}
                  {it.commit && <Text dimColor> Commit: {it.commit.substring(0, 8)}</Text>}
                  <Text dimColor>
                    {" "}
                    Lines: +{it.lines.added} -{it.lines.removed}
                  </Text>
                  <Text dimColor>
                    {" "}
                    Files: {totalFiles} ({it.files.updated.length} updated, {it.files.created.length} created, {it.files.removed.length} removed)
                  </Text>
                  <Text dimColor>
                    {" "}
                    Prompt: {it.prompt.substring(0, 80)}
                    {it.prompt.length > 80 ? "..." : ""}
                  </Text>
                </Box>
              );
            })}
            {ticket.frontmatter.files && (
              <Box marginTop={1}>
                <Text>Total Files: {ticket.frontmatter.files.updated.length + ticket.frontmatter.files.created.length + ticket.frontmatter.files.removed.length}</Text>
              </Box>
            )}
            {ticket.frontmatter.lines && (
              <Text>
                Total Lines: +{ticket.frontmatter.lines.added} -{ticket.frontmatter.lines.removed}
              </Text>
            )}
            <Box marginTop={1}>
              <Text>Content:</Text>
            </Box>
            <Text dimColor>{ticket.content}</Text>
          </Box>,
        );
        break;
      }
      if (sub === "delete") {
        const [year, month, day, slug] = rest;
        if (!year || !month || !day || !slug) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing year, month, day, or slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        deleteTicket(parseInt(year), parseInt(month), parseInt(day), slug);
        render(
          <Box flexDirection="column">
            <Text>
              ✅ Deleted ticket: {year}/{month}/{day}/{slug}
            </Text>
          </Box>,
        );
        break;
      }
      if (sub === "list") {
        const [year, month, day] = rest;
        const options: ListOptions = {};
        if (year) options.year = parseInt(year);
        if (month) options.month = parseInt(month);
        if (day) options.day = parseInt(day);
        const tickets = listTickets(options);
        render(
          <Box flexDirection="column">
            <Text>Found {tickets.length} ticket(s):</Text>
            {tickets.map((ticket) => {
              const parsed = parseTicketPath(ticket.path);
              if (!parsed) return null;
              return (
                <Box key={ticket.path} flexDirection="column" marginTop={1}>
                  <Text>
                    {parsed.year}-{String(parsed.month).padStart(2, "0")}-{String(parsed.day).padStart(2, "0")} {parsed.slug}
                  </Text>
                  <Text dimColor> Summary: {ticket.frontmatter.summary}</Text>
                  <Text dimColor> Status: {ticket.frontmatter.status}</Text>
                  <Text dimColor> Author: {ticket.frontmatter.author}</Text>
                  {ticket.frontmatter.model && <Text dimColor> Model: {ticket.frontmatter.model}</Text>}
                  <Text dimColor> Iterations: {ticket.frontmatter.iterations.length}</Text>
                </Box>
              );
            })}
          </Box>,
        );
        break;
      }
      if (sub === "search") {
        const options: SearchOptions = {};
        let query = "";
        for (const arg of rest) {
          if (arg.startsWith("--year=")) options.year = parseInt(arg.split("=")[1]);
          else if (arg.startsWith("--month=")) options.month = parseInt(arg.split("=")[1]);
          else if (arg.startsWith("--day=")) options.day = parseInt(arg.split("=")[1]);
          else if (arg.startsWith("--limit=")) options.limit = parseInt(arg.split("=")[1]);
          else if (!arg.startsWith("--")) query = arg;
        }
        if (query) options.query = query;
        const tickets = searchTickets(options);
        const limitText = options.limit ? ` (showing first ${options.limit})` : "";
        render(
          <Box flexDirection="column">
            <Text>
              Found {tickets.length} ticket(s){limitText}:
            </Text>
            {tickets.map((ticket) => {
              const parsed = parseTicketPath(ticket.path);
              if (!parsed) return null;
              return (
                <Box key={ticket.path} flexDirection="column" marginTop={1}>
                  <Text>
                    {parsed.year}-{String(parsed.month).padStart(2, "0")}-{String(parsed.day).padStart(2, "0")} {parsed.slug}
                  </Text>
                  <Text dimColor> Summary: {ticket.frontmatter.summary}</Text>
                  <Text dimColor> Status: {ticket.frontmatter.status}</Text>
                  <Text dimColor> Author: {ticket.frontmatter.author}</Text>
                  {ticket.frontmatter.model && <Text dimColor> Model: {ticket.frontmatter.model}</Text>}
                  {options.query && <Text dimColor> Preview: {ticket.content.substring(0, 200).replace(/\n/g, " ")}...</Text>}
                </Box>
              );
            })}
          </Box>,
        );
        break;
      }
      render(
        <Box flexDirection="column">
          <Text color="red">Error: Unknown ticket command</Text>
        </Box>,
      );
      printUsage();
      process.exit(1);
      break;
    }
    case "models": {
      render(
        <Box flexDirection="column">
          <Text>Available models:</Text>
          {Object.values(Model).map((model) => (
            <Text key={model} dimColor>
              {" "}
              {model}
            </Text>
          ))}
        </Box>,
      );
      break;
    }
    default:
      printUsage();
      process.exit(1);
  }
} catch (error) {
  render(
    <Box flexDirection="column">
      <Text color="red">Error: {error instanceof Error ? error.message : String(error)}</Text>
    </Box>,
  );
  process.exit(1);
}
//#endregion CLI

// #endregion Ticket

// #region Update Metabolism
// #region Imports

import InvalidKit from "@semio/assets/semio/kit_invalid.json";
import { applyKitDiff, Attribute, Author, Concept, Design, exportKit, Folder, getKitDiff, guid, importKit, Interface, inverseKitDiff, Kit, File as KitFile, Quality, serializeValidationResult, Tag, Type, validateSemioKit } from "@semio/js/semio";
import { promises as fs } from "fs";

// #endregion Imports

// #region Constants

const __dirname = dirname(fileURLToPath(import.meta.url));
const INCLUDE_FOLDERS = ["representations", "icons", "images"];

// #endregion Constants

// #region File Collection

function collectFiles(dir: string, basePath: string = ""): Map<string, Blob> {
  const files = new Map<string, Blob>();
  const entries = readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = join(dir, entry.name);
    const relativePath = basePath ? `${basePath}/${entry.name}` : entry.name;

    if (entry.isDirectory()) {
      if (entry.name === ".semio" || entry.name === ".git") continue;

      if (!basePath && !INCLUDE_FOLDERS.includes(entry.name)) continue;

      const subFiles = collectFiles(fullPath, relativePath);
      Array.from(subFiles.entries()).forEach(([path, blob]) => {
        files.set(path, blob);
      });
    } else {
      if (!basePath) continue;

      const buffer = readFileSync(fullPath);
      const blob = new Blob([buffer]);
      files.set(relativePath, blob);
    }
  }

  return files;
}

// #endregion File Collection

// #region Regenerate Zip

async function regenerateZip(setMessage: (msg: string) => void): Promise<{ fileCount: number; size: string; copiedCount: number }> {
  setMessage("Regenerating metabolism.zip...");
  const kitPath = join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
  const kitJson = readFileSync(kitPath, "utf-8");
  const kit = JSON.parse(kitJson) as Kit;

  const metabolismDir = join(__dirname, "..", "examples", "metabolism");
  const files = collectFiles(metabolismDir);
  const fileCount = files.size;

  setMessage("Exporting kit...");
  const zipBlob = await exportKit(kit, files);
  const buffer = Buffer.from(await zipBlob.arrayBuffer());

  const outputPath = join(__dirname, "..", "assets", "semio", "metabolism.zip");
  writeFileSync(outputPath, buffer);
  const size = (buffer.length / 1024).toFixed(2);

  setMessage("Copying to public folders...");
  const publicPaths = [join(__dirname, "..", "js", "js", "public", "metabolism.zip"), join(__dirname, "..", "js", "play", "public", "metabolism.zip")];

  let copied = 0;
  for (const publicPath of publicPaths) {
    const publicDir = join(publicPath, "..");
    if (!existsSync(publicDir)) {
      mkdirSync(publicDir, { recursive: true });
    }
    writeFileSync(publicPath, buffer);
    copied++;
  }

  setMessage("Validating import...");
  const { kit: imported } = await importKit(buffer);
  const tambourAfter = imported.types?.find((t) => t.name === "Tambour");

  return { fileCount, size, copiedCount: copied };
}

// #endregion Regenerate Zip

// #region Generate Diff

async function generateDiff(setMessage: (msg: string) => void): Promise<void> {
  setMessage("Loading Metabolism kit...");
  const kitPath = join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
  const kitJson = await fs.readFile(kitPath, "utf-8");
  let original: Kit = JSON.parse(kitJson);

  setMessage("Removing flattened designs...");
  original.designs = original.designs?.filter((d) => !d.parent);

  setMessage("Creating comprehensive diff...");

  let modified: Kit = JSON.parse(JSON.stringify(original));

  modified.name = "Metabolism Modified";
  modified.version = "r25.08-1";
  modified.description = "Modified version for comprehensive diff testing";
  modified.icon = "modified-icon.svg";
  modified.image = "modified-image.png";
  modified.homepage = "https://modified.example.com";
  modified.license = "MIT-Modified";

  if (modified.types && modified.types.length > 0) {
    modified.types.splice(0, 1);

    if (modified.types.length > 0) {
      modified.types[0].name = modified.types[0].name + " Modified";
      modified.types[0].description = "Updated description";
      modified.types[0].connectors = modified.types[0].connectors || [];
      modified.types[0].connectors.push({
        guid: guid(),
        name: "new-connector",
        point: { x: 1, y: 1, z: 1 },
        direction: { x: 0, y: 1, z: 0 },
        t: 0.5,
        mandatory: true,
      });
    }

    const newType: Type = {
      guid: guid(),
      name: "New Test Type",
      virtual: true,
      unit: "mm",
      description: "A new type added for testing",
      createdAt: new Date() as any,
      updatedAt: new Date() as any,
      connectors: [
        {
          guid: guid(),
          name: "test-connector",
          point: { x: 0, y: 0, z: 0 },
          direction: { x: 0, y: 0, z: 1 },
          t: 0,
          mandatory: false,
        },
      ],
    };
    modified.types.push(newType);
  }

  if (modified.designs && modified.designs.length > 0) {
    modified.designs.splice(0, 1);

    if (modified.designs.length > 0) {
      modified.designs[0].name = modified.designs[0].name + " Modified";
      modified.designs[0].description = "Updated design description";
      modified.designs[0].pieces = modified.designs[0].pieces || [];
      modified.designs[0].pieces.push({
        guid: guid(),
        type: original.types?.[3]?.guid ? { guid: original.types[3].guid } : undefined,
        plane: {
          origin: { x: 5, y: 5, z: 5 },
          xAxis: { x: 1, y: 0, z: 0 },
          yAxis: { x: 0, y: 1, z: 0 },
        },
        scale: 1.5,
      });
    }

    const newDesign: Design = {
      guid: guid(),
      name: "New Test Design",
      unit: "mm",
      description: "A new design added for testing",
      createdAt: new Date() as any,
      updatedAt: new Date() as any,
      pieces: [
        {
          guid: guid(),
          type: original.types?.[2]?.guid ? { guid: original.types[2].guid } : undefined,
          plane: {
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 },
          },
          scale: 1.0,
        },
      ],
    };
    modified.designs.push(newDesign);
  }

  if (modified.tags && modified.tags.length > 0) {
    const removedTag = modified.tags.splice(0, 1)[0];

    if (modified.tags.length > 0) {
      modified.tags[0].name = modified.tags[0].name + " Modified";
      modified.tags[0].description = "Updated tag description";
    }

    const newTag: Tag = {
      guid: guid(),
      name: "New Test Tag",
      description: "A new tag for testing",
      icon: "test-tag-icon.svg",
    };
    modified.tags.push(newTag);
  }

  if (modified.concepts && modified.concepts.length > 0) {
    modified.concepts.splice(0, 1);

    if (modified.concepts.length > 0) {
      modified.concepts[0].name = modified.concepts[0].name + " Modified";
      modified.concepts[0].description = "Updated concept description";
    }

    const newConcept: Concept = {
      guid: guid(),
      name: "New Test Concept",
      description: "A new concept for testing",
      icon: "test-concept-icon.svg",
    };
    modified.concepts.push(newConcept);
  }

  if (modified.interfaces && modified.interfaces.length > 0) {
    modified.interfaces.splice(0, 1);

    if (modified.interfaces.length > 0) {
      modified.interfaces[0].name = modified.interfaces[0].name + " Modified";
      modified.interfaces[0].description = "Updated interface description";
    }

    const newInterface: Interface = {
      guid: guid(),
      name: "New Test Interface",
      description: "A new interface for testing",
      icon: "test-interface-icon.svg",
    };
    modified.interfaces.push(newInterface);
  } else {
    modified.interfaces = [
      {
        guid: guid(),
        name: "Test Interface",
        description: "A new interface for testing",
        icon: "test-icon.svg",
      },
    ];
  }

  if (modified.qualities && modified.qualities.length > 0) {
    modified.qualities.splice(0, 1);

    if (modified.qualities.length > 0) {
      modified.qualities[0].name = modified.qualities[0].name + " Modified";
      modified.qualities[0].description = "Updated quality description";
      modified.qualities[0].defaultValue = 99;
    }

    const newQuality: Quality = {
      guid: guid(),
      key: "test.quality",
      name: "Test Quality",
      kind: 1,
      defaultSiUnit: "m",
      defaultImperialUnit: "ft",
      defaultValue: 10,
      canScale: true,
      description: "A new quality for testing",
    };
    modified.qualities.push(newQuality);
  }

  if (modified.files && modified.files.length > 0) {
    modified.files.splice(0, 1);

    if (modified.files.length > 0) {
      modified.files[0].name = "updated-" + modified.files[0].name;
    }

    const newFile: KitFile = {
      guid: guid(),
      name: "new-file.txt",
      createdAt: new Date() as any,
      updatedAt: new Date() as any,
    };
    modified.files.push(newFile);
  }

  if (modified.folders && modified.folders.length > 0) {
    modified.folders.splice(0, 1);

    if (modified.folders.length > 0) {
      modified.folders[0].name = modified.folders[0].name + " Modified";
      modified.folders[0].description = "Updated folder description";
    }

    const newFolder: Folder = {
      guid: guid(),
      name: "test-folder",
      description: "A new folder for testing",
      createdAt: new Date() as any,
      updatedAt: new Date() as any,
    };
    modified.folders.push(newFolder);
  } else {
    modified.folders = [
      {
        guid: guid(),
        name: "test-folder",
        description: "A new folder for testing",
        createdAt: new Date() as any,
        updatedAt: new Date() as any,
      },
    ];
  }

  if (modified.authors && modified.authors.length > 1) {
    modified.authors[0].name = modified.authors[0].name + " Modified";
    modified.authors[0].email = "modified@example.com";

    modified.authors.splice(1, 1);

    const newAuthor: Author = {
      guid: guid(),
      name: "Test Author",
      email: "test@example.com",
    };
    modified.authors.push(newAuthor);
  }

  if (modified.attributes && modified.attributes.length > 0) {
    modified.attributes.splice(0, 1);

    if (modified.attributes.length > 0) {
      modified.attributes[0].value = "modified-value";
      modified.attributes[0].definition = "Updated attribute definition";
    }

    const newAttribute: Attribute = {
      guid: guid(),
      key: "test.attribute",
      value: "test value",
      definition: "A test attribute",
    };
    modified.attributes.push(newAttribute);
  } else {
    modified.attributes = [
      {
        guid: guid(),
        key: "test.attribute",
        value: "test value",
        definition: "A test attribute",
      },
    ];
  }

  setMessage("Computing diff...");
  const diff = getKitDiff(original, modified);

  setMessage("Computing inverse diff...");
  const inverseDiff = inverseKitDiff(original, diff);

  setMessage("Applying forward diff...");
  const diffed = applyKitDiff(original, diff);

  setMessage("Writing diff files...");
  const outputDir = join(__dirname, "..", "assets", "semio");

  await fs.writeFile(join(outputDir, "diff_kit_metabolism.json"), JSON.stringify(diff, null, 2), "utf-8");
  await fs.writeFile(join(outputDir, "diff_kit_metabolism_inverted.json"), JSON.stringify(inverseDiff, null, 2), "utf-8");
  await fs.writeFile(join(outputDir, "kit_metabolism_diffed.json"), JSON.stringify(diffed, null, 2), "utf-8");
}

// #endregion Generate Diff

// #region Generate Validation

function generateValidation(setMessage: (msg: string) => void): number {
  setMessage("Generating validation.json...");
  const kit = InvalidKit as unknown as Kit;
  const result = validateSemioKit(kit);
  const json = serializeValidationResult(result);

  const outputPath = join(__dirname, "..", "assets", "semio", "validation.json");
  writeFileSync(outputPath, json + "\n");

  return result.issues.length;
}

// #endregion Generate Validation

// #region App

function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "error">("running");
  const [message, setMessage] = React.useState<string>("");
  const [fileCount, setFileCount] = React.useState<number>(0);
  const [size, setSize] = React.useState<string>("");
  const [copiedCount, setCopiedCount] = React.useState<number>(0);
  const [issueCount, setIssueCount] = React.useState<number>(0);

  React.useEffect(() => {
    (async () => {
      try {
        const zipResult = await regenerateZip(setMessage);
        setFileCount(zipResult.fileCount);
        setSize(zipResult.size);
        setCopiedCount(zipResult.copiedCount);

        await generateDiff(setMessage);

        const validationIssues = generateValidation(setMessage);
        setIssueCount(validationIssues);

        setStatus("success");
        setTimeout(() => process.exit(0), 100);
      } catch (error) {
        setStatus("error");
        setMessage(error instanceof Error ? error.message : String(error));
        setTimeout(() => process.exit(1), 100);
      }
    })();
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🔧 Updating Metabolism assets...</Text>
      {message && <Text dimColor>{message}</Text>}
      {status === "success" && (
        <>
          <Text>
            ✅ Exported {fileCount} files ({size} KB)
          </Text>
          <Text>
            ✅ Copied to {copiedCount} public folder{copiedCount !== 1 ? "s" : ""}
          </Text>
          <Text>✅ Generated diff files</Text>
          <Text>✅ Generated validation.json ({issueCount} issues)</Text>
        </>
      )}
      {status === "error" && <Text>❌ {message}</Text>}
    </Box>
  );
}

render(<App />);

// #endregion App

// #endregion Update Metabolism
