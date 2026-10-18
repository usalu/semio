import {test} from "bun:test";
import {readFileSync} from "node:fs";
import {readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
export function toolJobRustBlock(source: string, open: number): { body: string; end: number } | undefined {
  let depth = 0;
  let string = false;
  let lineComment = false;
  let blockComment = 0;
  let escaped = false;
  for (let index = open; index < source.length; index += 1) {
    const char = source[index]!;
    const next = source[index + 1] ?? "";
    if (lineComment) {
      if (char === "\n") lineComment = false;
      continue;
    }
    if (blockComment > 0) {
      if (char === "/" && next === "*") {
        blockComment += 1;
        index += 1;
      } else if (char === "*" && next === "/") {
        blockComment -= 1;
        index += 1;
      }
      continue;
    }
    if (string) {
      if (escaped) escaped = false;
      else if (char === "\\") escaped = true;
      else if (char === '"') {
        string = false;
      }
      continue;
    }
    if (char === "/" && next === "/") {
      lineComment = true;
      index += 1;
      continue;
    }
    if (char === "/" && next === "*") {
      blockComment = 1;
      index += 1;
      continue;
    }
    if (char === '"') string = true;
    else if (char === "{") depth += 1;
    else if (char === "}" && --depth === 0) return { body: source.slice(open + 1, index), end: index + 1 };
  }
  return undefined;
}

export function toolJobArtifactRetainedCommandExact(source: string): boolean {
  const checkpointStart = source.indexOf("    fn checkpoint(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {");
  const checkpointOpen = checkpointStart < 0 ? -1 : source.indexOf("{", checkpointStart);
  const checkpoint = checkpointOpen < 0 ? undefined : toolJobRustBlock(source, checkpointOpen);
  const required = [
    "pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;",
    "const ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES: usize = 48;",
    'const ARTIFACT_COMMAND_CHECKPOINT_MAGIC: [u8; 4] = *b"ARC1";',
    "fn encode_artifact_command_checkpoint(",
    "fn decode_artifact_command_checkpoint(",
    "bytes[4] != 3",
    "context_digest != current_context_digest",
    "workspace_identity != current_workspace_identity",
    "fn workspace_identity(&self) -> u64",
    "fn checkpoint(&self, _target: &mut [u8]) -> Result<usize, Fault>",
    "fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault>",
    "CheckpointPages,",
    "CheckpointRetire,",
    "WirePages,",
    "Decode,",
    "Preflight,",
    "Work,",
    "Publish,",
    "checkpoint_input: Option<RetainedToolWireInput>",
    "checkpoint_bytes: [u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES]",
    "checkpoint_pending: bool",
    "context: Option<Arc<ArtifactOwnedToolJobContext<A>>>",
    "pub fn from_wire_with_checkpoint(",
    "pub fn try_new_with_context(",
    "ArtifactRetainedCommandPhase::CheckpointPages =>",
    "ArtifactRetainedCommandPhase::CheckpointRetire =>",
    "checked_add(page.len()).filter(|end| *end <= ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES)",
    "decode_artifact_command_checkpoint(bytes, self.maximum_raw_bytes, input.page_count(), input.declared_bytes(), current_context_digest, current_workspace_identity)",
    "context_digest: self.context.as_ref().map_or(0, |context| context.identity_digest())",
    "workspace_identity: self.work.as_ref().map_or(0, |work| work.workspace_identity())",
    "self.work.as_mut().ok_or_else(|| Fault::from(\"retained-command-checkpoint-work-absent\"))?.restore(checkpoint.work)?",
    "self.raw.clear();",
    "self.raw_page_cursor = 0;",
    "self.checkpoint_pending = true;",
    "StepOutcome::CheckpointReady(Checkpoint",
    "checkpoint.begin_close();",
    "self.checkpoint_input.is_none()",
    "checkpoint_binary_matches_schema_fixture_and_owned_oracle",
    "checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift",
  ];
  return (
    required.every((fragment) => source.includes(fragment)) &&
    checkpoint !== undefined &&
    checkpoint.body.includes("let mut state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES]") &&
    checkpoint.body.includes("let mut work_state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES - ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES]") &&
    checkpoint.body.includes("work.checkpoint(&mut work_state)") &&
    !checkpoint.body.includes("to_vec()") &&
    !source.includes("checkpoint: Vec<u8>") &&
    !source.includes("checkpoint_bytes: Vec<u8>")
  );
}

export function toolJobArtifactRetainedCommandRuntimeLawExact(source: string): boolean {
  return [
    "shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close",
    "bus.dispatch_wire_retained_with_spec(",
    "bytes.get(4) == Some(&3) && bytes.get(5) == Some(&1) && bytes.get(48) == Some(&1)",
    "TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2",
    "semio_framework_job::StepOutcome::Cancelled",
    "original.job.terminal_is_empty()",
    "resumed.job.terminal_is_empty()",
    "cancelled.job.terminal_is_empty()",
  ].every((fragment) => source.includes(fragment));
}




/** 🧪️ Executes tool job artifact retained command policy assertions. */
export function toolJobArtifactRetainedCommandSelfTests(source: string, runtimeLawSource: string): number {
  if (!toolJobArtifactRetainedCommandExact(source)) throw new Error("[verify interactivity tool-jobs] live shared retained command checkpoint was falsely rejected.");
  if (!toolJobArtifactRetainedCommandRuntimeLawExact(runtimeLawSource)) throw new Error("[verify interactivity tool-jobs] live shared retained command runtime replay law was falsely rejected.");
  const mutations: readonly [string, string, string][] = [
    ["resizable work checkpoint", "let mut work_state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES - ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES];", "let mut work_state = Vec::new();"],
    ["resume constructor", "pub fn from_wire_with_checkpoint(", "pub fn from_wire_without_checkpoint("],
    ["checkpoint close owner", "checkpoint.begin_close();", "drop(checkpoint);"],
    ["terminal checkpoint owner", "&& self.checkpoint_input.is_none()", "&& true"],
    ["work cursor restore", ".restore(checkpoint.work)?", ".restore(&[])?"],
    ["context identity restore", "context_digest != current_context_digest", "false"],
    ["workspace identity restore", "workspace_identity != current_workspace_identity", "false"],
    ["checkpoint version", "bytes[4] != 3", "bytes[4] != 2"],
    ["checkpoint maximum", "pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;", "pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = usize::MAX;"],
  ];
  for (const [name, from, to] of mutations) {
    if (!source.includes(from)) throw new Error(`[verify interactivity tool-jobs] shared retained checkpoint mutation source missing: ${name}.`);
    if (toolJobArtifactRetainedCommandExact(source.replace(from, to))) throw new Error(`[verify interactivity tool-jobs] shared retained checkpoint mutation ${name} was falsely accepted.`);
  }
  const runtimeMutations: readonly [string, string, string][] = [
    ["runtime replay cursor", "TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2", "TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 3"],
    ["runtime cancellation", "semio_framework_job::StepOutcome::Cancelled", "semio_framework_job::StepOutcome::Yield"],
  ];
  for (const [name, from, to] of runtimeMutations) {
    if (!runtimeLawSource.includes(from)) throw new Error(`[verify interactivity tool-jobs] shared retained runtime mutation source missing: ${name}.`);
    if (toolJobArtifactRetainedCommandRuntimeLawExact(runtimeLawSource.replaceAll(from, to))) throw new Error(`[verify interactivity tool-jobs] shared retained runtime mutation ${name} was falsely accepted.`);
  }
  return mutations.length + runtimeMutations.length + 2;
}

const root="/Users/ueli/Documents/semio/";

const REQUIRED: string[] = [
    "pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;",
    "const ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES: usize = 48;",
    'const ARTIFACT_COMMAND_CHECKPOINT_MAGIC: [u8; 4] = *b"ARC1";',
    "fn encode_artifact_command_checkpoint(",
    "fn decode_artifact_command_checkpoint(",
    "bytes[4] != 3",
    "context_digest != current_context_digest",
    "workspace_identity != current_workspace_identity",
    "fn workspace_identity(&self) -> u64",
    "fn checkpoint(&self, _target: &mut [u8]) -> Result<usize, Fault>",
    "fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault>",
    "CheckpointPages,",
    "CheckpointRetire,",
    "WirePages,",
    "Decode,",
    "Preflight,",
    "Work,",
    "Publish,",
    "checkpoint_input: Option<RetainedToolWireInput>",
    "checkpoint_bytes: [u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES]",
    "checkpoint_pending: bool",
    "context: Option<Arc<ArtifactOwnedToolJobContext<A>>>",
    "pub fn from_wire_with_checkpoint(",
    "pub fn try_new_with_context(",
    "ArtifactRetainedCommandPhase::CheckpointPages =>",
    "ArtifactRetainedCommandPhase::CheckpointRetire =>",
    "checked_add(page.len()).filter(|end| *end <= ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES)",
    "decode_artifact_command_checkpoint(bytes, self.maximum_raw_bytes, input.page_count(), input.declared_bytes(), current_context_digest, current_workspace_identity)",
    "context_digest: self.context.as_ref().map_or(0, |context| context.identity_digest())",
    "workspace_identity: self.work.as_ref().map_or(0, |work| work.workspace_identity())",
    "self.work.as_mut().ok_or_else(|| Fault::from(\"retained-command-checkpoint-work-absent\"))?.restore(checkpoint.work)?",
    "self.raw.clear();",
    "self.raw_page_cursor = 0;",
    "self.checkpoint_pending = true;",
    "StepOutcome::CheckpointReady(Checkpoint",
    "checkpoint.begin_close();",
    "self.checkpoint_input.is_none()",
    "checkpoint_binary_matches_schema_fixture_and_owned_oracle",
    "checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift",
  ];
const lawSrc = toolJobArtifactRetainedCommandExact.toString();
test("missing required tokens", () => {
  const rcPath=root+"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs";
  const snap=readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c5c8-tools/orig/rc.rs","utf8");
  const tree=readModuleTreeSource(rcPath);
  const required = REQUIRED;
  const miss = (s: string) => required.filter(r => !s.includes(r));
  const a = miss(snap), b = miss(tree);
  console.log("required", required.length, "missing in snapshot", a.length, "missing in tree", b.length, "same", JSON.stringify(a) === JSON.stringify(b));
  console.log(JSON.stringify(a).slice(0, 600));
});
test("exactness on the snapshot, the tree and the root", () => {
  const rcPath=root+"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs";
  const snap=readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c5c8-tools/orig/rc.rs","utf8");
  const tree=readModuleTreeSource(rcPath);
  const plain=readFileSync(rcPath,"utf8");
  console.log("snapshot", toolJobArtifactRetainedCommandExact(snap), "tree", toolJobArtifactRetainedCommandExact(tree), "plain root", toolJobArtifactRetainedCommandExact(plain));
  // which required token is missing in the tree
});
