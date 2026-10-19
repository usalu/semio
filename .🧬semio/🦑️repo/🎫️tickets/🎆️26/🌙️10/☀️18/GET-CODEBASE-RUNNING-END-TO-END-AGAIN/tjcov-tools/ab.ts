import { toolJobPluginFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobTypedDispatchExact, toolJobWireDispatchExact } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts";
import { readSourceFamilySource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
const R = "/Users/ueli/Documents/semio";
const plugin = toolJobPluginFamilySource(R);
const ab = readSourceFamilySource(R + "/🧰️framework/🔨️modules/🎯️action-bus", { tests: true });
const abProd = readSourceFamilySource(R + "/🧰️framework/🔨️modules/🎯️action-bus");
const checks: Record<string, boolean> = {
  tojbIds: plugin.includes("pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];"),
  six: (plugin.match(/const TOOL_JOB_IDS: &'static \[&'static str\] = Self::TOOL_JOB_IDS;/g) ?? []).length === 6,
  spec: plugin.includes("ToolOperationSpec::new(") && plugin.includes(".tool_jobs\n                .dispatch("),
  admit: ab.includes("pub fn admit_exact_wire") && plugin.includes(".admit_exact_wire("),
  typed: toolJobTypedDispatchExact(ab), typedAlias: ab.includes('bus.dispatch(operation_spec("app", "alias"))'),
  wire: toolJobWireDispatchExact(ab), wireAlias: ab.includes('bus.dispatch_wire("app", "alias"'),
  dup: ab.includes("duplicate_key_inside_one_factory_is_rejected_atomically"),
};
console.log(checks, "plugin", plugin.length, "ab", ab.length, abProd.length, "count6", (plugin.match(/const TOOL_JOB_IDS: &'static \[&'static str\] = Self::TOOL_JOB_IDS;/g) ?? []).length);
