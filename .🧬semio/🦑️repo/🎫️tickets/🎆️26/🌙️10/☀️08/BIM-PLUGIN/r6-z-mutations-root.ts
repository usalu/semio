/**
 * 🪢️ Surgical edits of the leaf mount region of the artifact root `🦀️.rs` (CRLF, shared with peers): remove a whole leaf block, add or
 * remove the mount lines of one test case. Every call re-reads the file, edits in memory and writes it straight back.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, RS } from "./r3-f1-paths.ts";

const rootPath = join(artifact, RS);

function update(change: (lines: string[]) => string[]) {
  const text = readFileSync(rootPath, "utf8");
  const crlf = text.includes("\r\n");
  const lines = change(text.replaceAll("\r\n", "\n").split("\n"));
  writeFileSync(rootPath, lines.join(crlf ? "\r\n" : "\n"));
}

function block(lines: string[], snake: string): [number, number] {
  const open = lines.findIndex((line) => line.trim() === `pub mod ${snake} {`);
  if (open < 0) throw new Error(`no mount block for ${snake}`);
  const indent = lines[open].length - lines[open].trimStart().length;
  let close = open + 1;
  while (!(lines[close].trim() === "}" && lines[close].length - lines[close].trimStart().length === indent)) close++;
  return [open, close];
}

export function removeLeafBlock(snake: string) {
  update((lines) => {
    const [open, close] = block(lines, snake);
    const from = lines[open - 1]?.trim() === '#[path = "."]' ? open - 1 : open;
    return [...lines.slice(0, from), ...lines.slice(close + 1)];
  });
}

export function addMounts(snake: string, mounts: string) {
  update((lines) => {
    const [, close] = block(lines, snake);
    return [...lines.slice(0, close), ...mounts.replace(/\n$/, "").split("\n"), ...lines.slice(close)];
  });
}

export function removeCaseMount(snake: string, testDirName: string) {
  update((lines) => {
    const [open, close] = block(lines, snake);
    const hit = lines.findIndex((line, index) => index > open && index < close && line.includes(`/${testDirName}/`));
    if (hit < 0) throw new Error(`no mount of ${testDirName} in ${snake}`);
    const from = lines[hit - 1].trim() === "#[cfg(test)]" ? hit - 1 : hit;
    return [...lines.slice(0, from), ...lines.slice(hit + 2)];
  });
}

export function hasCaseMount(snake: string, testDirName: string): boolean {
  const lines = readFileSync(rootPath, "utf8").replaceAll("\r\n", "\n").split("\n");
  const [open, close] = block(lines, snake);
  return lines.slice(open, close).some((line) => line.includes(`/${testDirName}/`));
}
