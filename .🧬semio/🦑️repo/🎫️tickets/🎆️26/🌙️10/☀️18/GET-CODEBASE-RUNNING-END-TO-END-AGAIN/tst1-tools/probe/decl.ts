import ts from "typescript";
import { normalizationEngineDeclarations } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
const source = normalizationEngineDeclarations("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization");
const syntax = ts.createSourceFile("x.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const names = ["emojiFold", "graphemes", "isEmojiGrapheme", "splitLeadingEmoji", "splitLeadingEmojiIdentity", "matchDirectoryKind", "packageLocation", "fixedSpecificity", "compareFixedSpecificity", "equalFixedSpecificity", "fixedScopeMatches", "matchingFixedContracts", "violation", "mutationDomainOwnerLocation", "mutationFixtureMirrorOwnerPath", "canonicalDirectory", "directoryKindIndex", "directoryKindIndexes", "fixedContractCandidates", "fixedContractLeafIndex", "fixedContractLeafIndexes", "fixedContractLiteralLeaf", "fixedContractSpecificity", "fixedContractSpecificities"];
const have = new Set(syntax.statements.filter((n) => ts.isFunctionDeclaration(n)).map((n) => (n as ts.FunctionDeclaration).name?.text));
console.log("[DEBUG] missing", names.filter((n) => !have.has(n)).join(","));
