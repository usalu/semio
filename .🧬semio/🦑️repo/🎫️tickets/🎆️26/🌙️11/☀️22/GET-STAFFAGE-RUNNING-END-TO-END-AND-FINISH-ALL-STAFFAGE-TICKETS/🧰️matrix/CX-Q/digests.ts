/** 🧬️ The digests of the sample figure at the current code (`digests.ts <out.json>`; with the old-quality preload they are those before the monotone rule) and, for the fixture on disk, which entries differ and in which keys. */
import { readFileSync, writeFileSync } from "node:fs";
import { digestsOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️digests/🟦️.ts";
import { scaledFigure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️volumes/🟦️.ts";
const [out] = process.argv.slice(2);
writeFileSync(out!, JSON.stringify(await digestsOf(scaledFigure()), null, 1) + "\n");
