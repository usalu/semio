/** 🔍️ Prints every finding of the zero-touch closure and command laws on the real checkout. */
import { checkZeroTouchClosuresV1, checkZeroTouchCommandsV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
const root = process.cwd();
for (const finding of [...checkZeroTouchClosuresV1(root).findings, ...checkZeroTouchCommandsV1(root).findings]) console.log(JSON.stringify(finding));
