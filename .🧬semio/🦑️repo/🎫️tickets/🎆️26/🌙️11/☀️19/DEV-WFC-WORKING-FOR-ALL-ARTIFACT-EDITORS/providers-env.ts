import { repositoryScriptProvidersV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import { writeScriptProviderMapV1, SCRIPT_PROVIDERS_ENV } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts";
console.log(JSON.stringify({ name: SCRIPT_PROVIDERS_ENV, value: writeScriptProviderMapV1(repositoryScriptProvidersV1()) }));
