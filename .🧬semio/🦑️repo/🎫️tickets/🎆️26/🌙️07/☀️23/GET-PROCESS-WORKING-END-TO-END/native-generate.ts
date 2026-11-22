import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {GenerateScript} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import {specificCatalogPublicationV1} from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";
const repoRoot=resolve(import.meta.dir,"../../../../../../..");
const self={repoRoot,publication:specificCatalogPublicationV1(p=>readFileSync(resolve(repoRoot,p),"utf8"))};
(GenerateScript.prototype.run as (this:unknown,s:string[])=>void).call(self,[]);
