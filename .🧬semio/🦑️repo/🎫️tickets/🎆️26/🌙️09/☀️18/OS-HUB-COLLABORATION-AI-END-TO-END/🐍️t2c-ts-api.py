#!/usr/bin/env python3
"""🩹️ Rewrites the two TypeScript-compiler-API idioms the repo-product contract suites got wrong:
`statement.modifiers` (no such member on `ts.Statement` — `canHaveModifiers`/`getModifiers` is the
public reader) and `source.parseDiagnostics` (a real member the public `SourceFile` type omits).

Every replacement is an exact literal with an asserted occurrence count, so a peer edit that moved a
line makes the script fail instead of matching somewhere else.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parents[6]
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"

MODIFIERS_DESTRUCTURED = "statement.modifiers?.some(({ kind }) => kind === ts.SyntaxKind.ExportKeyword)"
MODIFIERS_NAMED = "statement.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)"
MODIFIERS_FIX = "(ts.canHaveModifiers(statement) && ts.getModifiers(statement)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword))"

EDITS: tuple[tuple[str, tuple[tuple[str, str, int], ...]], ...] = (
    (
        f"{LIB}/🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts",
        (
            (f"const exported = {MODIFIERS_DESTRUCTURED};", f"const exported = {MODIFIERS_FIX};", 1),
            ("source.parseDiagnostics", "parsedDiagnostics(source)", 2),
        ),
    ),
    (
        f"{LIB}/🧪️tests/📱️app-verification-source-ownership/🟦️.ts",
        (
            (f"const exported = {MODIFIERS_DESTRUCTURED};", f"const exported = {MODIFIERS_FIX};", 1),
            ("source.parseDiagnostics", "parsedDiagnostics(source)", 2),
        ),
    ),
    (
        f"{LIB}/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts",
        ((f"if (!{MODIFIERS_DESTRUCTURED}) continue;", f"if (!{MODIFIERS_FIX}) continue;", 1),),
    ),
    (
        f"{LIB}/🧪️tests/🧱️framework-source-topology/🟦️.ts",
        ((f"if (!{MODIFIERS_DESTRUCTURED}) continue;", f"if (!{MODIFIERS_FIX}) continue;", 1),),
    ),
    (
        f"{LIB}/🧪️tests/🧱️root-artifact-dependency-source/🟦️.ts",
        ((f"if (!{MODIFIERS_DESTRUCTURED}) continue;", f"if (!{MODIFIERS_FIX}) continue;", 1),),
    ),
    (
        f"{LIB}/🧪️tests/🧱️root-taxonomy-workflow-source/🟦️.ts",
        ((f"if (!{MODIFIERS_NAMED}) continue;", f"if (!{MODIFIERS_FIX}) continue;", 1),),
    ),
    (
        f"{LIB}/🧪️tests/🦑️repo-source-ownership/🟦️.ts",
        (
            ("parsed.parseDiagnostics", "parsedDiagnostics(parsed)", 1),
            ("loadCatalogTaxonomy(repoRoot)", "loadCatalogTaxonomy()", 2),
        ),
    ),
    (
        f"{LIB}/🧪️tests/🧱️manifestless-source-closure/🟦️.ts",
        (("parsed.parseDiagnostics", "parsedDiagnostics(parsed)", 1),),
    ),
    (
        f"{LIB}/⚡️caching/🧪️tests/🧱️command-source/🟦️.ts",
        (("getWorkspaceRoot(import.meta.url)", "getWorkspaceRoot()", 1),),
    ),
    (
        f"{LIB}/🧪️tests/🧱️cargo-transaction-command-source/🟦️.ts",
        (("getWorkspaceRoot(import.meta.url)", "getWorkspaceRoot()", 1),),
    ),
)


HELPER = (
    "/** 🧾️ Reads the parse diagnostics every `createSourceFile` result carries and the public `SourceFile` type omits. */\n"
    "const parsedDiagnostics = (source: ts.SourceFile): readonly ts.Diagnostic[] =>\n"
    "  (source as ts.SourceFile & { readonly parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics;\n"
)


def insert_helper(text: str) -> str:
    lines = text.split("\n")
    last_import = max(index for index, line in enumerate(lines) if line.startswith("import "))
    return "\n".join([*lines[: last_import + 1], "", HELPER.rstrip("\n"), *lines[last_import + 1 :]])


def main() -> int:
    failures = 0
    for relative, edits in EDITS:
        path = ROOT / relative
        text = path.read_text(encoding="utf8")
        if any("parsedDiagnostics(" in replacement for _, replacement, _ in edits) and "const parsedDiagnostics" not in text:
            text = insert_helper(text)
        for needle, replacement, expected in edits:
            found = text.count(needle)
            if found != expected:
                print(f"MISMATCH {relative}: {needle!r} × {found}, expected {expected}", file=sys.stderr)
                failures += 1
                continue
            text = text.replace(needle, replacement)
        path.write_text(text, encoding="utf8")
        print(f"ok {relative}")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
