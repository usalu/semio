import { join } from "node:path";
const root = process.cwd();
const { repositoryScriptProvidersV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts"));
const { writeScriptProviderMapV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts"));
process.stdout.write(writeScriptProviderMapV1(repositoryScriptProvidersV1()));
