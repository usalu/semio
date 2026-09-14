#!/usr/bin/env bun
/**
 * 🏗️ Copies real committed `🎫️ticket.json` documents into the codec case fixture.
 *
 * The vectors are REAL documents, byte for byte, so the case cannot pass against a shape that only
 * the two implementations believe in. Re-run after adding a source; the fixture is regenerated in
 * full and the `source` member records where each document came from.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const TICKETS = join(REPO_ROOT, ".🧬semio", "🦑️repo", "🎫️tickets");
const TARGET = join(REPO_ROOT, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🎫️tickets", "🧪️tests", "📄️ticket-document-codec", "🧫️fixtures", "📄️documents.json");

/** 📄️ The committed documents the codec case reads, one per behaviour worth pinning. */
const SOURCES = [
  "🎆️25/🌙️11/☀️24/LOG-SYSTEM",
  "🎆️26/🌙️01/☀️19/ADD-SH-LANGUAGE-TO-REPO-BINARY",
  "🎆️26/🌙️04/☀️26/KIT-COMMAND-SCOPE-REFACTOR",
  "🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END",
  "🎆️26/🌙️09/☀️05/WINDOWS-CHECKOUT-ILLEGAL-FILENAMES",
  "🎆️26/🌙️09/☀️05/CONFINE-DWG-CODECS-TO-ARTIFACT-I-O",
];

/** 🚫️ Documents a conforming decoder must refuse rather than default. */
const REFUSED = ['{"title":"no status"}', '{"title":"bad status","status":"OPEN"}', '{"title":"bad status","status":""}', "[]", "{"];

const documents = SOURCES.map((source) => ({ source, text: readFileSync(join(TICKETS, ...source.split("/"), "🎫️ticket.json"), "utf8") }));
writeFileSync(TARGET, `${JSON.stringify({ documents, refused: REFUSED }, null, 2)}\n`, "utf8");
console.log(`wrote ${documents.length} documents to ${TARGET}`);
