/** 🐍️ Uses the existing pinned wheel for a focused comparison while its isolated uv environment is queued. */
import {dirname, resolve} from "node:path";
import {homedir} from "node:os";

const cache = resolve(homedir(), ".cache/uv/archive-v0");
const wheels = await Array.fromAsync(new Bun.Glob("*/pypdfium2_raw/bindings.py").scan({cwd: cache}));
if (wheels.length !== 1) throw Error("Expected one cached PDFium wheel");
const prefix = "import sys\nsys.path.insert(0, " + JSON.stringify(resolve(cache, dirname(dirname(wheels[0]!)))) + ")\n";
const spawn = Bun.spawn.bind(Bun);
Bun.spawn = ((argv: string[], options: Parameters<typeof Bun.spawn>[1]) => {
    if (Array.isArray(argv) && argv[0] === "repo" && argv.includes("pypdfium2==5.15.2")) {
        return spawn(["repo", "tools", "exec", "--light", "python3", "--", "-I", "-c", prefix + argv.at(-1)], options);
    }
    return spawn(argv, options);
}) as typeof Bun.spawn;
