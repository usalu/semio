/** 🔎️ Measures whether a taxonomy path pattern may wildcard the staging profile segment. */
import { taxonomyPathPatternMatches } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const base = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist";
const pattern = `${base}/*/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`;
for (const path of [
  `${base}/dev/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`,
  `${base}/release/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`,
  `${base}/dev/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js.bak`,
  `${base}/dev/🔌️plugin-modules/🪞️other/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`,
  `${base}/dev/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/nested/🪟️preview2-shim/cli.js`,
]) console.log(taxonomyPathPatternMatches(path, pattern), path.slice(base.length));
