import { playRuntimeComponentIds } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🟦️.ts";
const { COMPONENT_MODULE_DIRECTORIES } = await import(Bun.resolveSync("@semio-tech/s-dev-catalog-ts/generated/plugins", `${process.argv[2]}/🏢️semio-tech/🎡️play`));
import { moduleDirectoryName } from "@semio-tech/plugin-registry/deployment";
const ids = playRuntimeComponentIds();
const bad = ids.filter(id => !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id));
const dirs = ids.map(id => moduleDirectoryName(id, COMPONENT_MODULE_DIRECTORIES));
console.log("components", ids.length, "bad ids", bad, "directory max units", Math.max(...dirs.map(d => d.length)), "distinct dirs", new Set(dirs).size);
