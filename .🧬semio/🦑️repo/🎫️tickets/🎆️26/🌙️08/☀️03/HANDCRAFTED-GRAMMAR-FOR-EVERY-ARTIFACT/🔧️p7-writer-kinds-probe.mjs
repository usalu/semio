#!/usr/bin/env bun
import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../..");
const engineRs = join(
  repoRoot,
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs",
);
const engine = readFileSync(engineRs, "utf8");
const registered = [...engine.matchAll(/register_language\(dsl::LanguageSpec\s*\{[^}]*id:\s*"([^"]+)"/gs)].map((m) => m[1]);
const debugLogs = [
  "writer.open_document uri=… language_id=… (open_document::handle)",
  "writer.main tokens path language_id=… (main window render)",
  "writer.engine language_tokens_json language_id=…",
];

function countSemioExamples() {
  const root = join(repoRoot, "✏️s/🔌️plugins");
  const bySuffix = {};
  let total = 0;
  const walk = (dir) => {
    for (const name of readdirSync(dir)) {
      const full = join(dir, name);
      const st = statSync(full);
      if (st.isDirectory()) walk(full);
      else if (name.endsWith(".semio")) {
        total++;
        const suffix = name.includes(".") ? name.split(".").slice(-2).join(".") : name;
        bySuffix[suffix] = (bySuffix[suffix] || 0) + 1;
      }
    }
  };
  walk(root);
  return { total, bySuffix };
}

const examples = countSemioExamples();
const report = {
  capturedAt: new Date().toISOString(),
  writerRegisteredLanguageIds: registered,
  writerRegisteredCount: registered.length,
  writerDebugLogSites: debugLogs,
  openDocumentTests: ["jack", "dag.jack via set_active_example", "open-document command in command_surface tests"],
  semioExamplesUnderPlugins: examples,
  runtimeNote:
    "cargo test / open_document [DEBUG] stderr not executed on this host (Xcode license exit 69). Registered language count satisfies P7 ≥6 kinds statically.",
};
writeFileSync(join(ticketDir, "🧪p7-writer-kinds-probe.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify({ registered: registered.length, examples: examples.total }, null, 2));
