/** 🧪️ All owning Forms authoring and transport laws share the strict public package gate. */
import{test}from"bun:test";
import{testFormsDocumentContractOracle,testFormsDesignImport,testFormsMutationSchemas}from"../🪪️document/🟦️.ts";
import{testFormsInspectionAuthoring}from"../../../✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️authoring/🟦️.ts";
import{testFormsQuestionPatches}from"../../../✏️editor/❓️questions/🧪️tests/🔬️patches/🟦️.ts";
import{testFormsQuestionPlacement}from"../../../✏️editor/❓️questions/📍️placement/🧪️tests/🟦️.ts";
import{testFormsExtensionInputs}from"../../../🧩️extensions/🧪️tests/🟦️.ts";
import{testFormsVisibility}from"../../../✏️editor/❓️questions/🫥️visibility/🧪️tests/🔬️editing/🟦️.ts";
import{testFormsPersistence}from"../💾️persistence/🟦️.ts";
import{testFormsResponses,testFormsSubmission}from"../../📨️response/🧪️tests/🔬️events/🟦️.ts";
import{testFormsValidation}from"../../✅️validation/🧪️tests/🟦️.ts";
import{testFormsResponseExport}from"../../../🚪️io/📝️text/📸️snapshot/📨️response/📤️export/🧪️tests/🟦️.ts";
import"../🧪️change-block-field/🟦️.ts";
test("testFormsDocumentContractOracle",testFormsDocumentContractOracle);
test("testFormsDesignImport",testFormsDesignImport);
test("testFormsMutationSchemas",testFormsMutationSchemas);
test("testFormsInspectionAuthoring",testFormsInspectionAuthoring);
test("testFormsQuestionPatches",testFormsQuestionPatches);
test("testFormsQuestionPlacement",testFormsQuestionPlacement);
test("testFormsExtensionInputs",testFormsExtensionInputs);
test("testFormsVisibility",testFormsVisibility);
test("testFormsPersistence",testFormsPersistence);
test("testFormsResponses",testFormsResponses);
test("testFormsSubmission",testFormsSubmission);
test("testFormsValidation",testFormsValidation);
test("testFormsResponseExport",testFormsResponseExport);

/** 📋️ The package owner verifies its declared public runtime APIs and private-subpath refusal. */
import{expect}from"bun:test";
import type{FormsSnapshot,FormsDefinition,FormsResponse}from"@semio-tech/forms-js";
export type FormsPublicDocumentContract=Readonly<{snapshot:FormsSnapshot;definition:FormsDefinition;response:FormsResponse}>;
import{createRequire}from"node:module";
import{resolve as resolveFormsOwner}from"node:path";
test("Forms package exports its document and transport APIs by package name",async()=>{
 const packageRoot=resolveFormsOwner(import.meta.dir,"../../../../../../../📦️packages/🟦️typescript"),entry=Bun.resolveSync("@semio-tech/forms-js",packageRoot),owner=await import(entry);
 for(const name of ["parseFormsSnapshot","formsSnapshotToSqliteDatabase","formsSnapshotFromSqliteDatabase","parseFormsJsonArtifact","parseFormsJsonDefinition","parseFormsJsonResponse"])expect(typeof owner[name],name).toBe("function");
 const require=createRequire(resolveFormsOwner(packageRoot,"package.json"));
 expect(()=>require.resolve("@semio-tech/forms-js/private")).toThrow();
});

import { testFormsTryParameterDefaults } from "../../../✏️editor/🎮️commands/🎯️set-try-value/🧪️tests/🟦️.ts";
test("testFormsTryParameterDefaults", testFormsTryParameterDefaults);

import "../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🔤️text-chunks/🧪️tests/🟦️.ts";

import {testFormsConfig} from "../../../🎚️config/🧪️tests/🟦️.ts";
test("shared Forms config matches its independent schema oracle",testFormsConfig);

import { testFormsViewerExtensionInputs } from "../../../👁️viewer/🧪️tests/🟦️.ts";
test("testFormsViewerExtensionInputs", testFormsViewerExtensionInputs);

import { testFormsTryPartialDefaults } from "../../../✏️editor/🎮️commands/🎯️set-try-value/🧪️tests/🟦️.ts";
test("testFormsTryPartialDefaults",testFormsTryPartialDefaults);

import{testFormsQuestionChunkIngress}from"../../../✏️editor/🎮️commands/🩹️patch-questions/🧪️tests/🟦️.ts";
test("canonical question patch ingress preserves bounded authored source",testFormsQuestionChunkIngress);

import { testFormsTryValueTarget } from "../../../✏️editor/🎮️commands/🎯️set-try-value/🧪️tests/🟦️.ts";
test("testFormsTryValueTarget",testFormsTryValueTarget);
