import { join } from "node:path";
import { readFileSync } from "node:fs";
import { toolJobOwnerSourceText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const W = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(join(W, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧫️fixtures/🎚️scalar-config-cohort.json"), "utf8"));
for (const owner of fixture.owners) {
  const source = toolJobOwnerSourceText(W, owner.file);
  const witnesses = [
      `const ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;`,
      `const ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;`,
      "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()",
      "request.authority.actor().len() > 64", "self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()",
      `_config_text_bytes(request.base.get()) > ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES`,
      `grant.maximum_bytes < ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.closing`,
      "if self.checkpoint.cursor != 0", "inverse: vec![inverse]", "protocol::UndoPolicy::ExactBaseOnly", "authority.prepare_one_item(edit, std::sync::Arc::new(next))?",
      "if !base.return_to_registry()", "self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some()",
  ];
  console.log(owner.id, "bytes", source.length, "missing:", witnesses.filter((w) => !source.includes(w)));
}
