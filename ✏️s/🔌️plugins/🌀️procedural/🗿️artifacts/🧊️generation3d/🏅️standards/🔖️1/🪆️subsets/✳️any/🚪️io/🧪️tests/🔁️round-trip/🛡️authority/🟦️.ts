/** 🧪️ Independent source-token admission for IO authority examples. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import ts from "typescript";
import { inspectRustCompileReferences } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

/** 🔎️ Checks closed source cases independently of the native syntax parser. */
export function testGeneration3dIoAuthorityFixture(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/🛡️authority/🔣️.json", import.meta.url), "utf8"));
  const ids = new Set(fixture.cases.map((entry: { id: string }) => entry.id));
  assert.equal(ids.size, fixture.cases.length);
  for (const entry of fixture.cases) {
    const scanner = ts.createScanner(ts.ScriptTarget.Latest, true, ts.LanguageVariant.Standard, entry.source);
    const identifiers: string[] = [];
    for (let token = scanner.scan(); token !== ts.SyntaxKind.EndOfFileToken; token = scanner.scan()) if (token === ts.SyntaxKind.Identifier) identifiers.push(scanner.getTokenText());
    const prohibited = identifiers.some(identifier => fixture.forbiddenSegments.includes(identifier) || fixture.forbiddenPrefixes.some((prefix: string) => identifier.startsWith(prefix)));
    const sourceImports = inspectRustCompileReferences(entry.source).filter(row => row.kind === "path" || row.kind === "include");
    const importsOwner = sourceImports.some(row => row.path.split(/[\\/]/u).some(segment => fixture.forbiddenSegments.includes(segment.replace(/^[^A-Za-z]+/u, ""))));
    assert.equal(!prohibited && !importsOwner, entry.allowed, entry.id);
  }
  return 1 + fixture.cases.length;
}
