import {strict as assert} from "node:assert";
import {readFile} from "node:fs/promises";
import {join} from "node:path";
import {portableMsvcInventoryV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/📦️inventory/🟦️.ts";

const bytes=await readFile(join(import.meta.dir,"🗑️generated","provider-manifest-v1.json")),observed=portableMsvcInventoryV1(JSON.parse(bytes.toString("utf8")));assert.equal(observed.text,bytes.toString("utf8"));assert.equal(observed.sha256,"69cfa0b1437d32de989abad528e77ef5497bd0771e2d5b281d6385786520b6db");console.log(JSON.stringify({files:observed.files,bytes:observed.bytes,sha256:observed.sha256,readinessAuthority:false}));
