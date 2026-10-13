"""🧩️ Adds the helper declarations introduced by the taxonomy/graph performance work to every introspective test closure
(a string-array literal of production declaration names) that names a function now depending on them.
argv: <test files...>; prints each file and the names it added."""
import re, sys

DEPENDENCIES = {
    "generatorPathCompare": ["utf8Compare"],
    "sourceAdmissionByteCompare": ["utf8Compare"],
    "assertLexicalInputOutsideOpaque": ["lexicalInputOutsideOpaque", "noFollowAncestors"],
    "assertNoFollowAncestors": ["noFollowAncestors"],
    "referenceCoordinateRoots": ["createLexicalInputSession", "lexicalInputOutsideOpaque", "noFollowAncestors"],
    "incomingReferenceObservation": ["createLexicalInputSession", "lexicalInputOutsideOpaque", "noFollowAncestors"],
    "incomingReferenceScan": ["createLexicalInputSession", "lexicalInputOutsideOpaque", "noFollowAncestors"],
    "incomingReferenceSnapshot": ["createLexicalInputSession", "lexicalInputOutsideOpaque", "noFollowAncestors"],
    "rustReferenceGraph": ["createLexicalInputSession", "lexicalInputOutsideOpaque", "noFollowAncestors"],
    "resolveReferencePath": ["referenceLeaf", "referenceLeafNames", "referenceLeafNameSets"],
    "lexicalOpaqueReferenceTarget": ["opaqueLeafNames", "opaqueLeafNameLists"],
    "unprovenRustReferenceTargets": ["exactPathsByLeaf", "exactPathsByLeafs"],
    "historicalDocumentEvidence": ["historicalLeafExpressions", "fixedContractCandidates", "fixedContractLeafIndex", "fixedContractLeafIndexes", "fixedContractLiteralLeaf"],
    "matchingFixedContracts": ["fixedContractCandidates", "fixedContractLeafIndex", "fixedContractLeafIndexes", "fixedContractLiteralLeaf", "fixedContractSpecificity", "fixedContractSpecificities"],
    "typescriptCollectionSyntax": ["TYPESCRIPT_COLLECTION_OPERATOR"],
    "referenceTokens": ["markdownReferenceDefinitionTokens", "MARKDOWN_WHITESPACE_RUN", "MARKDOWN_TEXT_RUN", "MARKDOWN_LINE_TERMINATOR"],
    "tomlTokens": ["TOML_TABLE_HEADER"],
    "matchDirectoryKind": ["directoryKindIndex", "directoryKindIndexes"],
    "packageLocation": ["directoryKindIndex", "directoryKindIndexes"],
    "projectionSourceAt": ["memberNamesByFold", "memberNameFolds"],
    "canonicalProjectedMemberName": ["memberNamesByFold", "memberNameFolds"],
    "projectionCatalogEntryForSubset": ["entrySubtree", "entrySubtreeIndexes"],
    "projectionBundleProblem": ["entrySubtree", "entrySubtreeIndexes"],
    "artifactProjectionAuthorityNodes": ["entrySubtree", "entrySubtreeIndexes"],
    "artifactProjectionDirectoryMappings": ["entrySubtree", "entrySubtreeIndexes"],
}
ARRAY = re.compile(r'\[(\s*"[^"\n]+"(?:\s*,\s*"[^"\n]+")+\s*)\]')

for path in sys.argv[1:]:
    text = open(path, encoding="utf8").read()
    added = []
    def patch(match):
        names = re.findall(r'"([^"\n]+)"', match.group(1))
        missing = []
        for name in names:
            for dependency in DEPENDENCIES.get(name, []):
                if dependency not in names and dependency not in missing:
                    missing.append(dependency)
        if not missing:
            return match.group(0)
        added.extend(missing)
        return "[" + match.group(1).rstrip() + ", " + ", ".join(f'"{name}"' for name in missing) + "]"
    patched = ARRAY.sub(patch, text)
    if patched != text:
        open(path, "w", encoding="utf8").write(patched)
        print(path, sorted(set(added)))
