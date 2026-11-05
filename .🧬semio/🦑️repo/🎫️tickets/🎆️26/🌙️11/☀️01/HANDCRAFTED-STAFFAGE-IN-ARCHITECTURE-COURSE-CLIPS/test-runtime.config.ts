import { mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import base from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts";
const modules = new URL("./🗑️generated/t/node_modules/", import.meta.url);
const alias = ["vitest","ajv","ajv-formats","fflate","polygon-clipping","pdfjs-dist","d3-ease","d3-format","d3-scale","gl-matrix","pngjs","three","jsdom"].map(find => ({ find, replacement: fileURLToPath(new URL(find, modules)) }));
const scratch = fileURLToPath(new URL("./🗑️generated/scratch/", import.meta.url));
mkdirSync(scratch, { recursive: true });
export default { ...base, test: { ...base.test, env: { SEMIO_TEST_ARTIFACT_DIR: scratch, TMPDIR: scratch, TEMP: scratch, TMP: scratch } }, resolve: { ...base.resolve, alias: [...base.resolve.alias, ...alias] } };
