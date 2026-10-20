/** 🔶️ The result phase of transformTestClosure. */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { identifierSet, parseRustSourceTree, useLeafNames, type RustModuleNode } from "../../🌳️module-tree/🟦️.ts";
import { absolutizeSuperPaths, declaredNames, widenHostItems } from "../🟢️shared/🟦️.ts";
import type { TestMount, TestTransformResult } from "../🟢️shared/🟦️.ts";

/** 🔷 Builds result, overlay, readText, setText of transformTestClosure. */
export function useResultState({ rootSource, rootFile }: { readonly rootSource: string; readonly rootFile: string }) {
  const result: TestTransformResult = { rootSource, edits: new Map(), creates: new Map(), deletes: new Set(), notes: [] };
  const overlay = new Map<string, string>();
  const readText = (path: string): string | null => overlay.get(path) ?? (result.deletes.has(path) ? null : result.creates.get(path) ?? (existsSync(path) ? readFileSync(path, "utf8") : null));
  const setText = (path: string, text: string): void => {
    overlay.set(path, text);
    if (path === rootFile) result.rootSource = text;
    else if (result.creates.has(path)) result.creates.set(path, text);
    else result.edits.set(path, text);
  };
  const rootEdits: { start: number; end: number; text: string }[] = [];
  const hostEdits = new Map<string, { start: number; end: number; text: string }[]>();
  const sameEdit = (left: { start: number; end: number; text: string }, right: { start: number; end: number; text: string }): boolean => left.start === right.start && left.end === right.end && left.text === right.text;
  const queueEdit = (host: string, edit: { start: number; end: number; text: string }): void => {
    if (host === rootFile) { if (!rootEdits.some((queued) => sameEdit(queued, edit))) rootEdits.push(edit); }
    else if (!(hostEdits.get(host) ?? []).some((queued) => sameEdit(queued, edit))) hostEdits.set(host, [...(hostEdits.get(host) ?? []), edit]);
  };
  const flushEdits = (): void => {
    const apply = (host: string, edits: { start: number; end: number; text: string }[]): void => {
      let text = readText(host) ?? "";
      for (const edit of edits.sort((a, b) => b.start - a.start)) text = text.slice(0, edit.start) + edit.text + text.slice(edit.end);
      setText(host, text);
    };
    if (rootEdits.length) apply(rootFile, rootEdits.splice(0));
    for (const [host, edits] of hostEdits) apply(host, edits);
    hostEdits.clear();
  };
  const removalOf = (host: string, node: RustModuleNode): { start: number; end: number; text: string } => {
    const text = readText(host) ?? "";
    let end = node.end;
    while (text[end] === " " || text[end] === "\t") end += 1;
    if (text[end] === "\n") end += 1;
    let start = node.start;
    const lineStart = text.lastIndexOf("\n", start - 1) + 1;
    if (/^[ \t]*$/u.test(text.slice(lineStart, start))) start = lineStart;
    return { start, end, text: "" };
  };
  const orphanedDocRemoval = (host: string, node: RustModuleNode): { start: number; end: number; text: string }[] => {
    const text = readText(host) ?? "";
    const lines: { start: number; end: number; text: string }[] = [];
    let cursor = text.lastIndexOf("\n", node.start - 1);
    const doc: { start: number; end: number }[] = [];
    while (cursor >= 0) {
      const lineStart = text.lastIndexOf("\n", cursor - 1) + 1;
      const line = text.slice(lineStart, cursor).trim();
      if (line.startsWith("///")) doc.push({ start: lineStart, end: cursor + 1 });
      else if (doc.length || !(line === "" || line.startsWith("//"))) break;
      else if (line.startsWith("//!")) break;
      cursor = lineStart - 1;
    }
    for (const range of doc) lines.push({ start: range.start, end: range.end, text: "" });
    return lines;
  };
  let sourceCache: [string, string][] | null = null;
  const artifactSources = (): [string, string][] => {
    if (sourceCache) return sourceCache;
    const out: [string, string][] = [];
    const walk = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) { if (!/^(node_modules|target|dist)$/u.test(entry.name)) walk(path); }
        else if (entry.name.endsWith(".rs")) out.push([path, readText(path) ?? ""]);
      }
    };
    walk(dirname(rootFile));
    return (sourceCache = out);
  };
  const subtreeText = (file: string, seen = new Set<string>()): string => {
    if (seen.has(file)) return "";
    seen.add(file);
    const text = readText(file) ?? "";
    let out = text;
    for (const node of parseRustSourceTree(text).modules) if (!node.inline && node.pathAttribute !== null) out += `\n${subtreeText(resolve(dirname(file), node.pathAttribute), seen)}`;
    return out;
  };
  const retargetPrivateImportPaths = (body: string, mount: TestMount, carry: readonly string[]): string => {
    const hostPath = mount.ownerPath.filter((segment) => segment !== "component").join("::");
    if (!hostPath || !carry.length) return body;
    let out = body;
    for (const name of new Set(carry.flatMap((statement) => useLeafNames(statement)))) {
      out = out.replace(new RegExp(`\\buse\\s+crate::${hostPath}::${name}\\s*;`, "gu"), "");
      out = out.replace(new RegExp(`\\bcrate::${hostPath}::${name}(?![A-Za-z0-9_])`, "gu"), name);
    }
    return out;
  };
  const carryIntoHostGlobModules = (body: string, mount: TestMount, carry: readonly string[]): string => {
    const hostPath = mount.ownerPath.filter((segment) => segment !== "component").join("::");
    if (!hostPath || !carry.length) return body;
    const glob = new RegExp(`^use\\s+crate::${hostPath}::\\*\\s*;$`, "u");
    const edits: { at: number; text: string }[] = [];
    const visit = (nodes: readonly RustModuleNode[]): void => {
      for (const node of nodes) {
        if (!node.inline) continue;
        for (const item of node.items) if (item.kind === "use" && glob.test(body.slice(item.declStart, item.end).trim())) edits.push({ at: item.end, text: `\n${carry.join("\n")}` });
        visit(node.children);
      }
    };
    visit(parseRustSourceTree(body).modules);
    let out = body;
    for (const edit of edits.sort((a, b) => b.at - a.at)) out = out.slice(0, edit.at) + edit.text + out.slice(edit.at);
    return out;
  };
  const hostGlobCarry = (mount: TestMount, body: string): string[] | null => {
    const hostPath = mount.ownerPath.filter((segment) => segment !== "component").join("::");
    if (!hostPath || !new RegExp(`\\buse\\s+crate::${hostPath}::\\*\\s*;`, "u").test(body)) return [];
    const hostText = readText(mount.hostFile) ?? "";
    const carry: string[] = [];
    for (const item of parseRustSourceTree(hostText).items) {
      if (item.kind !== "use" || item.visibility !== "private") continue;
      const statement = hostText.slice(item.declStart, item.end).trim();
      if (/^use\s+super::/u.test(statement) && !statement.includes("{self") && !/\*\s*;\s*$/u.test(statement)) {
        const absolute = absolutizeSuperPaths(statement, mount.ownerPath);
        if (absolute === null) return null;
        carry.push(absolute);
        continue;
      }
      const childHead = /^use\s+([A-Za-z_][A-Za-z0-9_]*)::/u.exec(statement)?.[1];
      if (childHead && parseRustSourceTree(hostText).modules.some((node) => node.name === childHead)) {
        carry.push(statement.replace(/^use\s+/u, `use crate::${hostPath}::`));
        continue;
      }
      if (/^use\s+(?:self|super)::/u.test(statement) || (statement.includes("{self") && !/^use\s+(?:crate|[A-Za-z_][A-Za-z0-9_]*)::/u.test(statement)) || /\bsuper::/u.test(statement)) return null;
      carry.push(statement);
    }
    return carry;
  };
  const widenFor = (mount: TestMount, items: readonly string[], body: string): string[] | null => {
    if (!items.length) return [];
    const hostText = readText(mount.hostFile) ?? "";
    const edits = widenHostItems(hostText, items);
    if (!edits) return null;
    for (const edit of edits) queueEdit(mount.hostFile, edit);
    const bodyNames = identifierSet(body);
    for (const node of parseRustSourceTree(hostText).modules) {
      if (!items.includes(node.name) || node.inline || node.pathAttribute === null) continue;
      const childFile = resolve(dirname(mount.hostFile), node.pathAttribute);
      const childText = readText(childFile);
      if (childText === null) continue;
      const restrictedNames = parseRustSourceTree(childText).items.filter((item) => item.name && item.kind !== "use" && item.kind !== "impl" && item.kind !== "macro" && item.visibility !== "pub" && bodyNames.has(item.name)).map((item) => item.name!);
      for (const edit of widenHostItems(childText, restrictedNames) ?? []) queueEdit(childFile, edit);
    }
    const bound = declaredNames(body, true);
    const imported = [...declaredNames(body)].filter((name) => !bound.has(name));
    const fresh = items.filter((name) => !imported.includes(name));
    return fresh.length ? [`use crate::${mount.ownerPath.filter((segment) => segment !== "component").join("::")}::{${fresh.join(", ")}};`] : [];
  };
  return { result, readText, setText, subtreeText, artifactSources, hostGlobCarry, widenFor, queueEdit, removalOf, orphanedDocRemoval, retargetPrivateImportPaths, flushEdits, overlay, carryIntoHostGlobModules };
}
