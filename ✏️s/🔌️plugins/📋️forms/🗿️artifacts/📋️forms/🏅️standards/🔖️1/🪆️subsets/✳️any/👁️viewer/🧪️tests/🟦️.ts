import assert from "node:assert/strict";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import metadata from "../../🧩️extensions/🧪️tests/🧫️fixtures/🔣️contribution.json";
import schema from "../../🧩️extensions/🧬️schema/🔣️.json";
import contributionSchema from "../../🧩️extensions/🧬️schema/🔣️contribution.json";
import { extensionRenderPayload } from "../../🧩️extensions/🟦️.ts";
import { parseFormsJsonQuestion, formsValueJsonProjection } from "../../🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";

/** 👓️ Every read-only provider receives authored defaults without an answer mutation lease. */
export function testFormsViewerExtensionInputs(): void {
    const validate = new Ajv().compile(schema);
    const validateContribution = new Ajv().compile(contributionSchema);
    for (const route of fixture.routes) {
        const contribution = { ...metadata.payload, questionKind: route.kind, paramsBodyKey: route.paramsBodyKey, previewBodyKey: route.previewBodyKey };
        assert.equal(validateContribution(contribution), true, JSON.stringify(validateContribution.errors));
        const question = parseFormsJsonQuestion({ ...fixture.question, kind: route.kind });
        const result = extensionRenderPayload(question, {}, fixture.controllerId, { surface: "try" }, false);
        const transport = { ...result, params: formsValueJsonProjection(result.params), authoredParams: formsValueJsonProjection(result.authoredParams) };
        assert.equal(validate(transport), true, JSON.stringify(validate.errors));
        assert.deepEqual(transport, fixture.expected);
    }
}
