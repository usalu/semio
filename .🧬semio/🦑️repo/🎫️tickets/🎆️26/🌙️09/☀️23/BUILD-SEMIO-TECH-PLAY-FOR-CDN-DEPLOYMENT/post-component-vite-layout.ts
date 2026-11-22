import { viteStorageLayoutV1 } from "@semio-tech/repo-lib/dev";
import { readFileSync } from "node:fs";
const root = process.argv[2]! + "/.cache/b/p/m", l = viteStorageLayoutV1(root, "release", "play:site");
console.log(l.output); console.log(readFileSync(l.output + "/.semio-artifact.json", "utf8").slice(0, 120));
