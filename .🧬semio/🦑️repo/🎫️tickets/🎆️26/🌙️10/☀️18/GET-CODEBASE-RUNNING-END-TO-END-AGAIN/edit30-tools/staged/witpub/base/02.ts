/** 🧾️ The input witness: one small file whose content is the digest of a test's whole declared source set, so Bazel keys and links one input where it would link every source as a runfile; an edit changes the witness exactly when a source's content changes. */
import { createHash } from "node:crypto";

/** 🏭️ The self-contained program the witness action runs under the pinned Bun toolchain: it reads an output path and the exec-root paths of the declared sources from a Bazel parameter file, hashes every file's content with bounded concurrency and writes `{version, files, sha256}` where `sha256` covers the sorted `path NUL size NUL content-sha256 LF` rows. */
export const BAZEL_WITNESS_RUNNER = String.raw`import{createHash}from"node:crypto";
import{mkdirSync,readFileSync,renameSync,writeFileSync}from"node:fs";
import{readFile}from"node:fs/promises";
import{dirname}from"node:path";
const parameters=(process.argv[2]??"").replace(/^@/u,""),[output,...paths]=readFileSync(parameters,"utf8").split("\n").filter(Boolean);
if(!output)throw new Error("[witness] output path required");
paths.sort();
const rows=new Array(paths.length);let next=0;
await Promise.all(Array.from({length:Math.min(64,Math.max(1,paths.length))},async()=>{for(;;){const index=next++;if(index>=paths.length)return;const path=paths[index],bytes=await readFile(path);rows[index]=path+"\0"+bytes.length+"\0"+createHash("sha256").update(bytes).digest("hex")+"\n";}}));
const outer=createHash("sha256");for(const row of rows)outer.update(row);
mkdirSync(dirname(output),{recursive:true});
const temporary=output+"."+process.pid+".tmp";writeFileSync(temporary,JSON.stringify({version:1,files:paths.length,sha256:outer.digest("hex")})+"\n");renameSync(temporary,output);
`;

/** 🧬️ The Starlark rule that runs the witness program over a source set; it is local, uncached remotely only in execution and never sandboxed, because staging every source into a sandbox would cost what the witness saves. */
export const BAZEL_WITNESS_STARLARK = `
def _witness_impl(ctx):
    output = ctx.actions.declare_file(ctx.label.name + ".witness")
    request = ctx.actions.args()
    request.use_param_file("@%s", use_always = True)
    request.set_param_file_format("multiline")
    request.add(output.path)
    sources = depset(transitive = [src[DefaultInfo].files for src in ctx.attr.srcs])
    request.add_all(sources)
    ctx.actions.run(executable = ctx.attr.runtime, arguments = [ctx.file.runner.path, request], inputs = depset([ctx.file.runner], transitive = [sources]), outputs = [output], mnemonic = "RepoInputsWitness", progress_message = "Witnessing the inputs of " + ctx.label.name, execution_requirements = {"no-sandbox": "1", "no-remote-exec": "1"}, use_default_shell_env = True, env = {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"})
    return [DefaultInfo(files = depset([output]))]

repo_inputs_witness = rule(implementation = _witness_impl, attrs = {"srcs": attr.label_list(allow_files = True), "runner": attr.label(allow_single_file = True), "runtime": attr.string(mandatory = True)})
`;

/** 🧮️ The reference digest of a set of files, for tests and tools that must reproduce a witness without Bazel. */
export function bazelWitnessDigestV1(files: readonly Readonly<{ path: string; bytes: Uint8Array }>[]): { version: 1; files: number; sha256: string } {
  const sorted = [...files].sort((left, right) => (left.path < right.path ? -1 : left.path > right.path ? 1 : 0));
  const outer = createHash("sha256");
  for (const file of sorted) outer.update(`${file.path}\0${file.bytes.length}\0${createHash("sha256").update(file.bytes).digest("hex")}\n`);
  return { version: 1, files: sorted.length, sha256: outer.digest("hex") };
}
