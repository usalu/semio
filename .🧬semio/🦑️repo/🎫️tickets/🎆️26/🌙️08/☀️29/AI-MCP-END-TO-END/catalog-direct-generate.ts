#!/usr/bin/env repo
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {renderCatalogFiles,publishCatalogFilesV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import {specificCatalogPublicationV1} from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";

/** 🧩️ Renders and publishes the plugin catalog in-process without the resident graph service. */
const repoRoot=resolve(import.meta.dir,"../../../../../../..");
const publication=specificCatalogPublicationV1(path=>readFileSync(resolve(repoRoot,path),"utf8"));
const {files,entries,diagnostics}=renderCatalogFiles(repoRoot,undefined,"exclude",publication);
const outDir=join(repoRoot,publication.outputRoot);
publishCatalogFilesV1(outDir,files);
console.log(`[DEBUG] ${entries.length} entries, ${diagnostics.length} diagnostics -> ${outDir}`);
