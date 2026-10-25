import { readFileSync } from "node:fs";
import { dependencyCargoTestOnlyDependencies } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts";
for (const f of process.argv.slice(2)) console.log(f.slice(-60), [...dependencyCargoTestOnlyDependencies(readFileSync(f, "utf8"))]);
const t = (s: string) => console.log(JSON.stringify([...dependencyCargoTestOnlyDependencies(s)]));
t('[features]\ndefault=["x"]\nx=["testkit"]\ntestkit=["dep:a","dep:b"]\n');
t('[features]\ntestkit=[\n "dep:a",\n "b/f",\n "c?/f"]\nrelease=["b"]\n');
t('[features]\ntesting=["a"]\n');
