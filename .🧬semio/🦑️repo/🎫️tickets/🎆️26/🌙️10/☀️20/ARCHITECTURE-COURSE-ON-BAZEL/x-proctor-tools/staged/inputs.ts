import { relative, resolve } from "node:path";
import { proctorSourceDirectories } from "../../../../../../../../../🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts";
import { proctorSourceGlobs } from "../../../../../../../../../🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts";

const repository = resolve(process.argv[2] ?? process.cwd());
const roots = (await proctorSourceDirectories(repository)).map(root => relative(repository, root).split("\\").join("/"));
console.log(JSON.stringify({ roots, globs: proctorSourceGlobs(roots) }));
