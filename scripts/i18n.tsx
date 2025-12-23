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

import { readdirSync, readFileSync, writeFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import React from "react";
import { render, Text, Box } from "ink";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const localesDir = join(rootDir, "js", "js", "sketchpad", "locales");
const sketchpadDir = join(rootDir, "js", "js", "sketchpad");
const reportPath = join(rootDir, "reports", "i18n.json");

const FIX_MODE = process.argv.includes("--fix");

interface Translation {
  [key: string]: string | Translation;
}

interface Issue {
  severity: "error" | "warning";
  key: string;
  message: string;
  location?: string;
}

//#region Load Translations
function loadTranslations(lang: string): Translation {
  const path = join(localesDir, `${lang}.json`);
  return JSON.parse(readFileSync(path, "utf-8"));
}

function saveTranslations(lang: string, translations: Translation): void {
  const path = join(localesDir, `${lang}.json`);
  writeFileSync(path, JSON.stringify(translations, null, 2), "utf-8");
}
//#endregion

//#region Find All IDs in Source Files
function findIdsInFile(filePath: string): Set<string> {
  const content = readFileSync(filePath, "utf-8");
  const ids = new Set<string>();

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

function findUsedIds(): Set<string> {
  const usedIds = new Set<string>();
  walkDir(sketchpadDir, (filePath) => {
    const ids = findIdsInFile(filePath);
    ids.forEach((id) => usedIds.add(id));
  });
  return usedIds;
}
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

const germanTranslations: Record<string, string> = {
  Theme: "Design",
  Light: "Hell",
  Dark: "Dunkel",
  System: "System",
  Layout: "Layout",
  Desktop: "Desktop",
  Tablet: "Tablet",
  Mobile: "Mobil",
  Mode: "Modus",
  Developer: "Entwickler",
  User: "Benutzer",
  Expertise: "Erfahrung",
  Beginner: "Anfänger",
  Normal: "Normal",
  Expert: "Experte",
  Settings: "Einstellungen",
  Language: "Sprache",

  Search: "Suche",
  Name: "Name",
  Description: "Beschreibung",
  Icon: "Symbol",
  Image: "Bild",
  Variant: "Variante",
  View: "Ansicht",
  Unit: "Einheit",
  Location: "Standort",
  Longitude: "Längengrad",
  Latitude: "Breitengrad",
  Authors: "Autoren",
  Email: "E-Mail",
  Attributes: "Attribute",
  Value: "Wert",
  Definition: "Definition",
  "Created At": "Erstellt am",
  "Updated At": "Aktualisiert am",
  ID: "ID",
  Type: "Typ",
  Center: "Zentrum",
  X: "X",
  Y: "Y",
  Z: "Z",

  "Fix Piece": "Bauteil fixieren",
  Plane: "Ebene",
  "Plane Origin": "Ebenenursprung",
  "Plane X Axis": "Ebene X-Achse",
  "Plane Y Axis": "Ebene Y-Achse",
  Origin: "Ursprung",
  "X Axis": "X-Achse",
  "Y Axis": "Y-Achse",

  "Connecting Piece ID": "Verbindendes Bauteil-ID",
  "Connecting Connector ID": "Verbindender Connector-ID",
  "Connecting Design Piece ID": "Verbindendes Design-Bauteil-ID",
  "Connected Piece ID": "Verbundenes Bauteil-ID",
  "Connected Connector ID": "Verbundener Connector-ID",
  "Connected Design Piece ID": "Verbundenes Design-Bauteil-ID",
  Gap: "Abstand",
  Shift: "Verschiebung",
  Rise: "Anstieg",
  Rotation: "Rotation",
  Turn: "Drehung",
  Tilt: "Neigung",
  U: "U",
  V: "V",

  Interface: "Schnittstelle",
  Mandatory: "Pflicht",
  Position: "Position",
  Direction: "Richtung",
  "Compatible Interface": "Kompatible Schnittstelle",
  Attribute: "Attribut",

  Cluster: "Gruppieren",
  Expand: "Erweitern",
  Diagram: "Diagramm",
  Scene: "Szene",
  Table: "Tabelle",

  Types: "Typen",
  Designs: "Entwürfe",
  Canvas: "Leinwand",
  Remove: "Entfernen",
  Add: "Hinzufügen",
  "Add Type": "Typ hinzufügen",
  "Add Design": "Entwurf hinzufügen",
  "Add Child": "Kind hinzufügen",
  Pieces: "Bauteile",
  Windows: "Fenster",
  Tools: "Werkzeuge",

  "Select Design": "Entwurf auswählen",
  "Select Variant": "Variante auswählen",
  "Select View": "Ansicht auswählen",
  "Select Type": "Typ auswählen",
  Author: "Autor",
  "Mixed Selection Message": "Gemischte Auswahl",
  "Mixed Values": "Gemischte Werte",
  "Connected Piece Info": "Verbundenes Bauteil Info",
  "Parent Connection": "Elternverbindung",
  "Parent Connections": "Elternverbindungen",
  "Multiple Editing": "Mehrfachbearbeitung",
  "Not Found": "Nicht gefunden",
  Yes: "Ja",
  No: "Nein",
  "Select Only Pieces Or Connections": "Nur Bauteile oder Verbindungen auswählen",

  English: "Englisch",
  German: "Deutsch",
  "Select language...": "Sprache auswählen...",

  "Proximity Connect Distance": "Näherungsverbindungs-Abstand",
  "Grid Size": "Rastergröße",

  Show: "Anzeigen",
  Workbench: "Werkbank",
  HUD: "HUD",
  Stats: "Statistiken",
  Details: "Details",
  Chat: "Chat",
  Toolbar: "Werkzeugleiste",
  Docs: "Dokumentation",
  Overview: "Übersicht",
  Page: "Seite",
  "No Headings": "Keine Überschriften",
  Manual: "Handbuch",
  Tutorial: "Tutorial",

  Version: "Version",
  Filter: "Filter",
  Kind: "Art",
  Temporary: "Temporär",
  Local: "Lokal",
  Remote: "Remote",
  "Sort by Name": "Nach Name sortieren",
  "Toggle Row": "Zeile umschalten",
  "Create Version": "Version erstellen",
  "Hide Kind": "Art ausblenden",
  "Show Temporary": "Temporär anzeigen",
  "Show Local": "Lokal anzeigen",
  "Show Remote": "Remote anzeigen",
  "Sort by Type": "Nach Typ sortieren",
  "Sort by Updated At": "Nach Aktualisierung sortieren",
  "Sort by Created At": "Nach Erstellung sortieren",
  Ascending: "Aufsteigend",
  Descending: "Absteigend",

  Create: "Erstellen",
  "Create Temporary": "Temporär erstellen",
  "Create Local": "Lokal erstellen",
  "Create Remote": "Remote erstellen",
  "Create Kit": "Kit erstellen",
  "Drop here...": "Hier ablegen...",
  "Drag and drop files": "Dateien ziehen und ablegen",
  Placeholder: "Platzhalter",

  "All Compatible": "Alle kompatibel",
  "Compatible Interfaces": "Kompatible Schnittstellen",
  "Multiple Selected": "Mehrere ausgewählt",

  "Default Name": "Standardname",
  "New Version": "Neue Version",
  "Default Version": "Standardversion",
  "Last Updated": "Zuletzt aktualisiert",
  Created: "Erstellt",
  "No Kits": "Keine Kits",
  Loading: "Laden",
  Artifact: "Artefakt",
  "Not Available": "Nicht verfügbar",

  Band: "Leiste",
  "Show Designs": "Entwürfe anzeigen",
  "Show Types": "Typen anzeigen",
  "Show Qualities": "Qualitäten anzeigen",
  "Show Interfaces": "Schnittstellen anzeigen",
  "Show Files": "Dateien anzeigen",
  "Show Folders": "Ordner anzeigen",
  "Show Authors": "Autoren anzeigen",
  Hide: "Ausblenden",
  "Sort by Artifact": "Nach Artefakt sortieren",
  "Create Child": "Kind erstellen",
  "Sort by Kind": "Nach Art sortieren",
  Homepage: "Homepage",
  License: "Lizenz",
  Compatible: "Kompatibel",
  "Create Artifact": "Artefakt erstellen",
  "Create Design": "Entwurf erstellen",
  "Create Type": "Typ erstellen",
  "Create Quality": "Qualität erstellen",
  "Create Interface": "Schnittstelle erstellen",
  "Create File": "Datei erstellen",
  "Create Folder": "Ordner erstellen",
  "Create Author": "Autor erstellen",
  Folder: "Ordner",

  Key: "Schlüssel",
  Formula: "Formel",
  "Default SI Unit": "Standard-SI-Einheit",
  "Default Imperial Unit": "Standard-Imperiale-Einheit",
  "Can Scale": "Skalierbar",
  "Default Value": "Standardwert",
  Min: "Min",
  "Is Min Excluded": "Min ausgeschlossen",
  Max: "Max",
  "Is Max Excluded": "Max ausgeschlossen",
  "Numeric Functions": "Numerische Funktionen",
  "Branching Functions": "Verzweigungsfunktionen",
  "Data Structures": "Datenstrukturen",
  Title: "Titel",
  Functions: "Funktionen",
  Qualities: "Qualitäten",
  "Add (math)": "Addieren",
  Subtract: "Subtrahieren",
  Multiply: "Multiplizieren",
  Divide: "Dividieren",
  If: "Wenn",
  Switch: "Schalter",
  List: "Liste",

  Model: "Modell",
  Models: "Modelle",
  Connector: "Connector",
  Connectors: "Connectors",
  Properties: "Eigenschaften",
  T: "T",

  Back: "Zurück",
  Forward: "Vorwärts",
  Up: "Hoch",
  Navigation: "Navigation",
  Focus: "Fokus",
  "Focus Mode": "Fokusmodus",
  "Panel Toggles": "Panel-Umschalter",
  Fullscreen: "Vollbild",
  "Exit Fullscreen": "Vollbild beenden",
  Open: "Öffnen",
  Input: "Eingabe",
  Right: "Rechts",
  "Search Input": "Sucheingabe",
  "Navigation Buttons": "Navigationsschaltflächen",

  Tags: "Tags",
  Concepts: "Konzepte",
  "Show Tags": "Tags anzeigen",
  "Show Concepts": "Konzepte anzeigen",
  "Create Tag": "Tag erstellen",
  "Create Concept": "Konzept erstellen",
  Point: "Punkt",

  "Choose the color theme for the application": "Wählen Sie das Farbschema für die Anwendung",
  "Choose the color theme": "Farbschema wählen",
  "Use dark color scheme": "Dunkles Farbschema verwenden",
  "Use light color scheme": "Helles Farbschema verwenden",
  "Follow system theme preference": "Systemdesign-Einstellung folgen",
  "Configure the window layout and panel arrangement": "Fensterlayout und Panel-Anordnung konfigurieren",
  "Optimized layout for desktop computers": "Optimiertes Layout für Desktop-Computer",
  "Optimized layout for tablets": "Optimiertes Layout für Tablets",
  "Optimized layout for mobile devices": "Optimiertes Layout für mobile Geräte",
  "Select the user interface mode": "Wählen Sie den Benutzeroberflächenmodus",
  "Choose the interface mode": "Oberflächenmodus wählen",
  "Developer mode with advanced tools and debugging features": "Entwicklermodus mit erweiterten Werkzeugen und Debugging-Funktionen",
  "Standard user mode for regular operations": "Standardbenutzermodus für reguläre Operationen",
  "Select your expertise level to adjust the interface complexity": "Wählen Sie Ihre Erfahrungsstufe um die Komplexität der Oberfläche anzupassen",
  "Choose your expertise level": "Erfahrungsstufe wählen",
  "Show detailed explanations and tutorials": "Detaillierte Erklärungen und Tutorials anzeigen",
  "Show standard tooltips and help": "Standard-Tooltips und Hilfe anzeigen",
  "Show standard tooltips": "Standard-Tooltips anzeigen",
  "Minimal tooltips for experienced users": "Minimale Tooltips für erfahrene Benutzer",
  "Select the language for the application interface": "Wählen Sie die Sprache für die Anwendungsoberfläche",
  "Expertise Level": "Erfahrungsstufe",
  "Choose the layout mode": "Layoutmodus wählen",

  "Color scheme for the interface": "Farbschema für die Oberfläche",
  "Layout mode for the interface": "Layoutmodus für die Oberfläche",
  "Interface mode": "Oberflächenmodus",
  "Your expertise level": "Ihre Erfahrungsstufe",
  "Show detailed help and tutorials": "Detaillierte Hilfe und Tutorials anzeigen",

  "Import Kit": "Kit importieren",

  "Window Library": "Fensterbibliothek",
  "Cluster Menu": "Gruppierungsmenü",
  "Expand Menu": "Erweiterungsmenü",

  "Home App": "Startseite",
  "Kit App": "Kit-App",
  "Design App": "Entwurf-App",
  "Type App": "Typ-App",
  "Quality App": "Qualität-App",

  "Panel Toggle": "Panel-Umschalter",
  "Panel Visibility": "Panel-Sichtbarkeit",

  "e.g., 1.0.0": "z.B. 1.0.0",
  "e.g., MIT, GPL-3.0, Apache-2.0": "z.B. MIT, GPL-3.0, Apache-2.0",
  "e.g., small, medium, large": "z.B. klein, mittel, groß",
  "e.g., front, side, top": "z.B. vorne, seite, oben",
  "e.g., large, small": "z.B. groß, klein",
  "e.g., electrical, mechanical": "z.B. elektrisch, mechanisch",
  "Value...": "Wert...",
  "Unit...": "Einheit...",
  "Definition or URL...": "Definition oder URL...",
  "Select parent type...": "Elterntyp auswählen...",
  "tag1, tag2, tag3": "tag1, tag2, tag3",
  "interface1, interface2": "schnittstelle1, schnittstelle2",
  "Search kits...": "Kits suchen...",
  "Search for content": "Nach Inhalten suchen",
  "Search for kits, designs, types, and more": "Nach Kits, Entwürfen, Typen und mehr suchen",
  "Toggle focus mode to hide distractions": "Fokusmodus umschalten um Ablenkungen auszublenden",
  "Focus on an element in the current view": "Auf ein Element in der aktuellen Ansicht fokussieren",
  "Stop Tutorial": "Tutorial beenden",
  "Click to stop the current tutorial": "Klicken, um das aktuelle Tutorial zu beenden",
  "Previous Step": "Vorheriger Schritt",
  "Go to the previous step in the tutorial": "Zum vorherigen Schritt im Tutorial gehen",
  "Play/Pause": "Abspielen/Pause",
  "Play or pause the tutorial": "Tutorial abspielen oder pausieren",
  "Next Step": "Nächster Schritt",
  "Go to the next step in the tutorial": "Zum nächsten Schritt im Tutorial gehen",
  "Toggle Workbench": "Werkbank umschalten",
  "Toggle the Workbench panel on the left side": "Das Werkbank-Panel auf der linken Seite ein- oder ausblenden",
  "Toggle HUD": "HUD umschalten",
  "Toggle the HUD panel in the middle": "Das HUD-Panel in der Mitte ein- oder ausblenden",
  "Configure application settings": "Anwendungseinstellungen konfigurieren",
  "A description of the kit": "Eine Beschreibung des Kits",
  "A description of the selected kits": "Eine Beschreibung der ausgewählten Kits",
  "A detailed description of what this kit contains and how it should be used.": "Eine detaillierte Beschreibung dessen, was dieses Kit enthält und wie es verwendet werden sollte.",
  "Optional description that explains the purpose of this folder.": "Optionale Beschreibung, die den Zweck dieses Ordners erläutert.",
  "Click to filter artifacts by this name": "Klicken, um Artefakte nach diesem Namen zu filtern",
  "Designs in this kit": "Entwürfe in diesem Kit",
  "Types in this kit": "Typen in diesem Kit",
  "The center position of the piece in the 2D diagram layout.": "Die Zentrumsposition des Bauteils im 2D-Diagramm-Layout.",
  "The 3D placement plane for this piece. Defines position and orientation in 3D space.": "Die 3D-Platzierungsebene für dieses Bauteil. Definiert Position und Ausrichtung im 3D-Raum.",
  "A detailed description of what this design represents and how it should be used.": "Eine detaillierte Beschreibung dessen, was dieser Entwurf darstellt und wie er verwendet werden sollte.",
  "A detailed description of what this type represents and how it should be used.": "Eine detaillierte Beschreibung dessen, was dieser Typ darstellt und wie er verwendet werden sollte.",
  "A description of the connector": "Eine Beschreibung des Connectors",
  "Select the user interface mode: Expert (minimal tooltips), Normal (standard), or Beginner (detailed help)": "Wählen Sie den Benutzeroberflächenmodus: Experte (minimale Tooltips), Normal (Standard) oder Anfänger (detaillierte Hilfe)",
};

function translateToGerman(english: string): string {
  return germanTranslations[english] || english;
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

function runFixMode(en: Translation, de: Translation, usedIds: Set<string>): { enFixed: number; deFixed: number } {
  let enFixed = 0;
  let deFixed = 0;
  const enKeys = flattenKeys(en);
  const deKeys = flattenKeys(de);

  for (const key of enKeys) {
    if (!deKeys.includes(key)) {
      const enValue = getNestedValue(en, key);
      if (typeof enValue === "string") {
        const germanValue = translateToGerman(enValue);
        setNestedValue(de, key, germanValue);
        deFixed++;
      }
    }
  }

  for (const key of enKeys) {
    const enValue = getNestedValue(en, key);
    const deValue = getNestedValue(de, key);
    if (typeof enValue === "string" && typeof deValue === "string" && enValue === deValue && enValue !== "") {
      const technicalTerms = ["ID", "X", "Y", "Z", "U", "V", "T", "HUD", "URL", "Email", "System", "Layout", "Normal", "Desktop", "Tablet", "Connector", "Chat", "Kit", "Design", "Tutorial"];
      if (!technicalTerms.includes(enValue)) {
        const germanValue = translateToGerman(enValue);
        if (germanValue !== enValue) {
          setNestedValue(de, key, germanValue);
          deFixed++;
        }
      }
    }
  }

  Array.from(usedIds).forEach((id) => {
    const labelKeyNormal = `${id}.label.normal`;
    const labelKeyBeginner = `${id}.label.beginner`;
    const enKeys = flattenKeys(en);

    if (!enKeys.includes(labelKeyNormal)) {
      const readable = extractReadableName(id);
      setNestedValue(en, labelKeyNormal, readable);
      setNestedValue(de, labelKeyNormal, translateToGerman(readable));
      enFixed++;
      deFixed++;
    }

    if (!enKeys.includes(labelKeyBeginner)) {
      const readable = extractReadableName(id);
      setNestedValue(en, labelKeyBeginner, readable);
      setNestedValue(de, labelKeyBeginner, translateToGerman(readable));
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

  saveTranslations("en", en);
  saveTranslations("de", de);

  return { enFixed, deFixed };
}
//#endregion

//#region App Component
function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "warning" | "error">("running");
  const [errorCount, setErrorCount] = React.useState(0);
  const [warningCount, setWarningCount] = React.useState(0);
  const [fixedEn, setFixedEn] = React.useState(0);
  const [fixedDe, setFixedDe] = React.useState(0);

  React.useEffect(() => {
    const runValidation = () => {
      const usedIds = findUsedIds();
      let en = loadTranslations("en");
      let de = loadTranslations("de");

      if (FIX_MODE) {
        const { enFixed, deFixed } = runFixMode(en, de, usedIds);
        setFixedEn(enFixed);
        setFixedDe(deFixed);
        en = loadTranslations("en");
        de = loadTranslations("de");
      }

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

      writeFileSync(reportPath, JSON.stringify(report, null, 2), "utf-8");
      setErrorCount(errors.length);
      setWarningCount(warnings.length);
      setStatus(errors.length > 0 ? "error" : warnings.length > 0 ? "warning" : "success");
      process.exit(errors.length > 0 ? 1 : 0);
    };

    runValidation();
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🔍 Validating i18n translations...</Text>
      {FIX_MODE && (
        <Text dimColor>
          🔧 Fixed {fixedEn} English entries, {fixedDe} German entries
        </Text>
      )}
      {status === "running" && <Text>Loading...</Text>}
      {status === "success" && <Text color="green">✅ i18n validation passed</Text>}
      {status === "warning" && (
        <Text color="yellow">
          ⚠️ i18n validation completed with {warningCount} warnings
        </Text>
      )}
      {status === "error" && (
        <Text color="red">
          ❌ i18n validation failed with {errorCount} errors, {warningCount} warnings
        </Text>
      )}
      {(status === "success" || status === "warning" || status === "error") && (
        <Text dimColor>📝 Report written to {reportPath}</Text>
      )}
    </Box>
  );
}

render(<App />);
//#endregion
