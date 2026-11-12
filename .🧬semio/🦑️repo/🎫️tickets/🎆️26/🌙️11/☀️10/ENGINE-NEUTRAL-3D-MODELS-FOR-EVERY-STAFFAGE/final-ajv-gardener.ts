import { readFileSync } from "node:fs";
import Ajv from "ajv";
const schema = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🔣️.json", "utf8"));
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema, "s");
const v = ajv.compile({ $ref: "s#/$defs/Asset" });
const a = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/kneeling-gardener/🔣️.json", "utf8")).asset;
console.log(v(a), JSON.stringify(v.errors?.slice(0, 6)));
