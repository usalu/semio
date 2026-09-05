/** 🧫️ W3 — renders the `🔣️json` parity fixtures from each subset's own `.semio` DSL example asset,
 * using the TypeScript reader+writer pair the io leaves declare. The Rust side asserts the same
 * files, so a disagreement surfaces as a failing Rust test rather than a silent divergence. */
import { block2dToJsonText } from "../../../../../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import { block2dFromDslText } from "../../../../../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️";
import { block5dToJsonText } from "../../../../../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import { block5dFromDslText } from "../../../../../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️";

const BLOCK = "✏️s/🔌️plugins/🧱️block/🗿️artifacts";
const JOBS = [
  { dir: "◻️2d", example: "🎬️hexagonal-cut-concrete-forest-left", asset: "hexagonal-cut-concrete-forest-left", read: block2dFromDslText, write: block2dToJsonText },
  { dir: "◻️2d", example: "➡️hexagonal-cut-concrete-forest-right", asset: "hexagonal-cut-concrete-forest-right", read: block2dFromDslText, write: block2dToJsonText },
  { dir: "🖐️5d", example: "🎬️hexagonal-cut-concrete-forest-left", asset: "hexagonal-cut-concrete-forest-left", read: block5dFromDslText, write: block5dToJsonText },
  { dir: "🖐️5d", example: "🏢️nakagin-capsule", asset: "nakagin-capsule", read: block5dFromDslText, write: block5dToJsonText },
] as const;

for (const job of JOBS) {
  const base = `${BLOCK}/${job.dir}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
  const dsl = await Bun.file(`${base}/📚️examples/${job.example}/🖼️assets/🧪️${job.asset}/🗣️.dsl.semio`).text();
  const json = job.write(job.read(dsl) as never);
  await Bun.write(`${base}/🚪️io/🧪️tests/🧫️fixtures/${job.asset}.json`, json);
  console.log(`${job.dir}/${job.asset}: ${json.length} bytes`);
  console.log(json.slice(0, 400));
}
