import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const scratch = resolve(import.meta.dir, "🗑️generated");
const responseQuote = process.argv.includes("--guest-response-quote");
const guardReclaim = process.argv.includes("--guard-reclaim-test");
const inputFrontier = process.argv.includes("--input-receipt-frontier");
const inputFrontierWasm = process.argv.includes("--input-receipt-wasm");
const hostBuild = responseQuote || guardReclaim || inputFrontier;
const build = inputFrontierWasm ? join(scratch, "native-guests", "wasm32-wasip2", "debug", "build")
  : join(scratch, hostBuild ? "native-host" : "native-mcp", "debug", "build");
const interpreter = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter");
const runtime = join(interpreter, "🏃️runtime");
const packages = readdirSync(build, { withFileTypes: true })
  .filter(entry => entry.isDirectory() || (hostBuild && entry.isSymbolicLink() && statSync(join(build, entry.name)).isDirectory()))
  .map(entry => entry.name);
const variants = new Map(packages.map(pkg => [pkg, readdirSync(join(build, pkg)).filter(id => existsSync(join(build, pkg, id, "out")))]));
const artifact = (pkg: string, id: string, crate: string) => join(pkg, id, "out", `lib${crate}-${id}`);
const external = (crate: string, path: string) => ["--extern", `${crate}=${path}`];
const fingerprint = (pkg: string, id: string, crate: string) => join(build, pkg, id, "fingerprint", `lib-${crate}`);
const crateName = (crate: string) => crate === "framework_hash" ? "semio_framework_hash" : crate;
const findIdentity = (pkg: string, crate: string, hash: string) => variants.get(pkg)?.find(id => {
  const marker = fingerprint(pkg, id, crate);
  return existsSync(marker) && readFileSync(marker, "utf8").trim() === hash;
});
const packageName = (alias: string) => {
  const crate = crateName(alias);
  return crate.startsWith("semio_") ? crate.replaceAll("_", "-") : crate;
};
const identityFor = (crate: string) => (variants.get(packageName(crate)) ?? []).filter(id => existsSync(join(build, artifact(packageName(crate), id, crate) + ".rlib")))
  .sort((left, right) => statSync(join(build, artifact(packageName(crate), right, crate) + ".rlib")).mtimeMs
    - statSync(join(build, artifact(packageName(crate), left, crate) + ".rlib")).mtimeMs)[0];
function deps(crate: string, id: string, excluded: string[] = [], selected?: string[]) {
  const file = fingerprint(packageName(crate), id, crate) + ".json";
  const source = readFileSync(file, "utf8");
  return new Map([...source.matchAll(/\[\d+,"([a-z0-9_]+)",false,(\d+)\]/gu)].filter(([, name]) => !excluded.includes(name) && (!selected || selected.includes(name))).map(([, name, digest]) => {
    const hash = Buffer.from(BigInt(digest).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
    const resolved = findIdentity(packageName(name), crateName(name), hash);
    if (!resolved) throw Error(`Missing ${crate} dependency: ${name} ${hash}`);
    return [name, resolved] as const;
  }));
}
function args(dependencies: Map<string, string>, replacements = new Map<string, string>()) {
  return [...dependencies].flatMap(([crate, id]) => {
    const replacement = replacements.get(crate);
    if (replacement) return external(crate, replacement);
    const path = artifact(packageName(crate), id, crateName(crate));
    return [".rmeta", ".rlib", ".dylib", ".so", ".dll"].filter(ext => existsSync(join(build, path + ext))).flatMap(ext => external(crate, path + ext));
  });
}
const rustup = Bun.which("rustup");
const toolchain = (await Bun.file("rust-toolchain.toml").text()).match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if (!rustup || !toolchain) throw Error("Pinned Rust unavailable");
const search = [...variants].flatMap(([pkg, ids]) => ids.flatMap(id => {
  const out = join(pkg, id, "out");
  return ["-L", `dependency=${out}`, ...(readdirSync(join(build, out)).some(name => /\.(a|lib)$/.test(name)) ? ["-L", `native=${out}`] : [])];
}));
function compile(...options: string[]) {
  const result = Bun.spawnSync([rustup!, "run", toolchain!, "rustc", "--edition=2021", ...options, "-L", `dependency=${scratch}`, ...search], {
    cwd: build, env: { ...process.env, SEMIO_BUILD_MODE: "prototype" }, stdout: "inherit", stderr: "inherit",
  });
  if (result.exitCode) process.exit(result.exitCode);
}
const core = "semio_plugin_interpreter";
const imports = "semio_plugin_interpreter_imports";
const owned = "semio_plugin_interpreter_runtime";
if (guardReclaim) {
  const owner = "semio_framework_job_return";
  const caller = "semio_framework_job_testing_caller";
  const callerIds = (variants.get(packageName(caller)) ?? []).filter(id => existsSync(join(build, artifact(packageName(caller), id, caller) + ".rlib")));
  const ownerIds = (variants.get(packageName(owner)) ?? []).filter(id => existsSync(join(build, artifact(packageName(owner), id, owner) + ".rlib")))
    .sort((left, right) => statSync(join(build, artifact(packageName(owner), right, owner) + ".rlib")).mtimeMs
      - statSync(join(build, artifact(packageName(owner), left, owner) + ".rlib")).mtimeMs);
  const owners = ownerIds.map(id => deps(owner, id));
  const match = owners.flatMap(ownerDeps => callerIds.flatMap(callerId => {
    const callerDeps = deps(caller, callerId);
    return [...callerDeps].every(([name, id]) => !ownerDeps.has(name) || ownerDeps.get(name) === id)
      ? [{ ownerDeps, callerId }] : [];
  }))[0];
  if (!match) throw Error("Compatible return owner and caller fixture artifacts are required");
  const group = "🧰️framework/🔨️modules/🧵️job";
  const context = "semio_framework_job_context";
  const turn = "semio_framework_job_native_turn";
  const callerDependencies = deps(caller, match.callerId);
  const contextId = callerDependencies.get(context)!;
  const turnId = callerDependencies.get(turn)!;
  const generated = join(scratch, "guard-rust");
  mkdirSync(generated, { recursive: true });
  const replacements = new Map<string, string>();
  function rebuild(crate: string, source: string, dependencies: Map<string, string>, ...flags: string[]) {
    const output = join(generated, `lib${crate}.rlib`);
    console.error("[DEBUG] Rebuilding guard law dependency", crate);
    compile("--crate-name", crate, "--crate-type=rlib", "-Zthreads=1", resolve(group, source),
      "-o", output, "-L", `dependency=${generated}`, ...flags, ...args(dependencies, replacements));
    replacements.set(crate, output);
  }
  rebuild(context, "⏱️context/📦️packages/🦀️rust/🦀️.rs", deps(context, contextId), "--cfg", 'feature="testkit"');
  rebuild(turn, "🛬️receiving/🎟️native-turn/🦀️.rs", deps(turn, turnId));
  rebuild(caller, "⏱️context/🧪️testing/🎟️caller/🦀️.rs", callerDependencies);
  const dependencies = match.ownerDeps;
  dependencies.set(caller, match.callerId);
  for (const crate of ["serde_json", "semio_framework_trace"]) {
    const id = callerDependencies.get(crate) ?? identityFor(crate);
    if (!id) throw Error(`Missing guard test dependency: ${crate}`);
    dependencies.set(crate, id);
  }
  const source = resolve(group, "📬️outcome/⏸️continuation/📤️owner/🦀️.rs");
  const output = join(generated, "mcp-guard-reclaim-law" + (process.platform === "win32" ? ".exe" : ""));
  compile("--crate-name", owner, "--test", "-Zthreads=1", source, "-o", output,
    "-L", `dependency=${generated}`, ...args(dependencies, replacements));
  const test = "header_guard::tests::full_escrow_reclaims_one_acknowledged_row_before_minting_its_guard";
  process.exit(Bun.spawnSync([output, "--exact", test, "--nocapture"], { stdout: "inherit", stderr: "inherit" }).exitCode ?? 1);
}
if (inputFrontier || inputFrontierWasm) {
  if (inputFrontier && process.platform !== "darwin") throw Error("The low-address native receipt fixture requires Darwin");
  const guest = "semio_plugin_guest_contract";
  const id = identityFor(guest);
  if (!id) throw Error("The linked guest contract is required");
  const dependencies = deps(guest, id, [], ["semio_framework_actor", "semio_framework_value", "serde_json"]);
  dependencies.set(guest, id);
  const output = join(scratch, "mcp-input-receipt-frontier" + (inputFrontierWasm ? ".wasm" : ""));
  const source = join(import.meta.dir, "mcp-input-receipt-frontier.rs");
  process.env.MCP_HOST_DRIVER_POLICY = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/🏛️host-driver-policy.json");
  if (inputFrontierWasm) {
    const macroBuild = join(scratch, "native-guests", "debug", "build");
    const macroSearch = readdirSync(macroBuild).flatMap(pkg => readdirSync(join(macroBuild, pkg))
      .map(id => join(macroBuild, pkg, id, "out"))
      .filter(out => existsSync(out) && readdirSync(out).some(file => /\.(dylib|so|dll)$/.test(file)))
      .flatMap(out => ["-L", `dependency=${out}`]));
    compile(source, "--target=wasm32-wasip2", "-C", "link-arg=-zstack-size=8388608",
      ...macroSearch, ...args(dependencies), "-o", output);
    process.exit(0);
  }
  compile(source, "-C", "link-arg=-Wl,-pagezero_size,0,-segaddr,__TEXT,0x100000000", ...args(dependencies), "-o", output);
  process.exit(Bun.spawnSync([output, ...process.argv.slice(3)], { stdout: "inherit", stderr: "inherit" }).exitCode ?? 1);
}
if (responseQuote) {
  const guest = "semio_plugin_guest_contract";
  const id = identityFor(guest);
  if (!id) throw Error("The linked guest contract is required");
  const dependencies = deps(guest, id);
  dependencies.set(guest, id);
  const trace = "semio_framework_trace";
  const traceId = identityFor(trace);
  if (!traceId) throw Error("The linked physical allocation tracer is required");
  dependencies.set(trace, traceId);
  const input = join(scratch, "mcp-guest-response-quote.rs");
  const output = join(scratch, "mcp-guest-response-quote" + (process.platform === "win32" ? ".exe" : ""));
  const fixture = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/🧪️tests/first-response.json");
  const law = JSON.parse(readFileSync(fixture, "utf8"));
  const { items: expectedItems, work: expectedWork } = law.required;
  const denied = law.cases.filter((row: { accepted: boolean }) => !row.accepted).map((row: { items: number; work: number }) => `(${row.items}, ${row.work})`).join(", ");
  const birthFixture = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🧪️tests/🤝️admission/core-wire-birth.json");
  const dispatchCopies = (JSON.parse(readFileSync(birthFixture, "utf8")).dispatchCopyBytes as number[]).join(", ");
  writeFileSync(input, `use semio_framework_actor::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_job::StepBudget;
use semio_framework_value::{RetainedCloneGrant, ValueRefusalKind};
use semio_plugin_guest_contract::original_actor_context::{original_core_first_response_reserve, OriginalActorGuestEntry};

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

fn main() {
    for length in [144, 152, 184] {
        let (items, demand, work) = original_core_first_response_reserve(length).expect("first response quote");
        println!("[DEBUG] live Core width {length}: items={items}, work={work}, copy={}, capacity={}", demand.copy_bytes, demand.capacity_bytes);
        assert_eq!((items, work), (${expectedItems}, ${expectedWork}));
        assert_eq!(demand.depth, 1);
    }
    let stock = RetainedCloneGrant {
        maximum_items: 128, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24,
        maximum_release_bytes: 1 << 24, maximum_depth: 64,
    };
    let input = RetainedTurnInput {
        operation: 1, generation: 1, epoch: 1, grant: stock,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: u64::MAX, fuel: 10_000 },
    };
    for (items, fuel) in [${denied}] {
        let mut entry = OriginalActorGuestEntry::new();
        let budget = StepBudget::new(fuel, u64::MAX, RetainedCloneGrant { maximum_items: items, ..stock });
        let (outcome, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            entry.dispatch_poll_header_step(input, budget, || Some(1), |_| panic!("unsupplied producer was called"))
        });
        let error = outcome.expect_err("insufficient quote must refuse");
        let kind = if items < ${expectedItems} { ValueRefusalKind::OwnershipLimit } else { ValueRefusalKind::WorkLimit };
        assert_eq!(error.kind, kind);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(entry.terminal_is_empty());
        println!("[DEBUG] live Core admission denied {items} items and {fuel} work without allocating");
    }
    verify_dispatch_baseline();
}

fn verify_dispatch_baseline() {
    use semio_framework_value::RetainedCloneProgress;
    let grant = RetainedCloneGrant {
        maximum_items: 65_536, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 26,
        maximum_release_bytes: 1 << 26, maximum_depth: 64,
    };
    let mut input = RetainedTurnInput {
        operation: 8853, generation: 1, epoch: 1, grant,
        work: OriginalActorTurnWork { clock_binding: 8853, absolute_deadline_us: 10_000_000, fuel: 10_000_000 },
    };
    let clock = || Some(1);
    let budget = |input: RetainedTurnInput| StepBudget::new(input.work.fuel, input.work.absolute_deadline_us, input.grant);
    let advance = |input: &mut RetainedTurnInput, spent, work| {
        let receipt = input.return_original(spent, work).expect("bounded actor receipt");
        input.epoch += 1;
        input.grant = receipt.remaining;
        input.work.fuel = receipt.remaining_fuel;
    };
    let mut guest = OriginalActorGuestEntry::new();
    let mut ready = false;
    for _ in 0..64 {
        let call = guest.admit_step(input, budget(input), clock);
        assert!(call.result.is_ok());
        ready = call.result.is_ok_and(|state| state.ready);
        advance(&mut input, call.receipt_spent, call.work_units_spent);
        if ready { break; }
    }
    assert!(ready);
    let mut previous_start = 0;
    for copied_bytes in [${dispatchCopies}] {
        let mut recipient = RetainedCloneProgress::default();
        let mut observed = None;
        let call = guest.dispatch_step(input, budget(input), &mut recipient, clock, |cx| {
            let start = cx.original_call_start_progress().expect("bound actor baseline").copied_bytes;
            let before = cx.retained_progress().copied_bytes;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() }).expect("paid fixture copy");
            cx.consume_fuel(1);
            observed = Some((start, before, cx.retained_progress().copied_bytes));
            Ok(())
        });
        assert!(call.result.is_ok());
        let (start, before, end) = observed.unwrap();
        assert!(start > previous_start);
        assert_eq!(end - before, copied_bytes);
        assert_eq!(end - start, call.receipt_spent.copied_bytes);
        println!("[DEBUG] live epoch {} starts after {} actor-lifetime copy bytes and returns {} dispatch bytes", input.epoch, start, end - start);
        previous_start = start;
        advance(&mut input, call.receipt_spent, call.work_units_spent);
    }
    for _ in 0..1024 {
        if guest.terminal_is_empty() { break; }
        let call = guest.close_step(input, budget(input));
        assert!(call.result.is_ok());
        advance(&mut input, call.receipt_spent, call.work_units_spent);
    }
    assert!(guest.terminal_is_empty());
}
`);
  compile(input, ...args(dependencies), "-o", output);
  process.exit(Bun.spawnSync([output], { stdout: "inherit", stderr: "inherit" }).exitCode ?? 1);
}
if (process.argv.includes("--dsl-token-test")) {
  const token = "semio_framework_dsl_token";
  const source = resolve("🧰️framework/🔨️modules/🗣️dsl/🔤️token/🦀️.rs");
  const output = join(scratch, "mcp-dsl-token-law");
  const dependencies = deps(token, identityFor(token));
  dependencies.set("serde_json", identityFor("serde_json"));
  compile("--crate-name", token, "--test", source, "-o", output, ...args(dependencies));
  process.exit(Bun.spawnSync([output, "--nocapture"], {stdout: "inherit", stderr: "inherit"}).exitCode ?? 1);
}
if (process.argv.includes("--describe-check")) {
  const crate = "semio_plugin_describe";
  const source = resolve(interpreter, "..", "🖨️describe", "🛂️descriptor-emission", "🦀️.rs");
  const dependencies = deps(crate, identityFor(crate), ["pack"]);
  for (const [name, id] of deps(owned, identityFor(owned))) {
    if (dependencies.has(name)) dependencies.set(name, id);
  }
  const replacements = new Map([[owned, join(scratch, `lib${owned}-prepared_custody.rlib`)]]);
  compile("--crate-name", crate, "--crate-type", "rlib", "--emit", "metadata", source,
    "-o", join(scratch, "mcp-prepared-describe-check.rmeta"), ...args(dependencies, replacements));
  process.exit(0);
}
if (process.argv.includes("--mcp-module")) {
  const workspace = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace");
  const module = join(workspace, "🧊️compiled-components", "🦀️.rs");
  const wrapper = join(scratch, "mcp-compiled-components-check.rs");
  const crate = "semio_os_mcp_plugin_runtime";
  const dependencies = deps(crate, identityFor(crate));
  const needed = new Set(["semio_plugin_host", "semio_os_mcp_foundation", "semio_os_mcp_port", "semio_os_mcp_catalog", "semio_framework_job", "semio_framework_value", "semio_framework_async", "semio_framework_async_sync", "framework_hash"]);
  const replacements = new Map<string, string>();
  const cacheTest = process.argv.includes("--cache-test");
  const execute = process.argv.includes("--execute");
  if (cacheTest) {
    dependencies.set("serde_json", deps(owned, identityFor(owned)).get("serde_json")!);
    needed.add("serde_json");
    needed.add("semio_framework_actor");
  }
  if (process.argv.includes("--current-host")) {
    const host = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host");
    const facade = "semio_plugin_host";
    const guest = "semio_plugin_host_guest_runtime";
    const relay = "semio_plugin_host_post_turn_relay";
    const shard = "semio_plugin_host_shard";
    const executor = "semio_plugin_host_shard_executor";
    const catalog = "semio_os_mcp_catalog";
    const facadeDeps = deps(facade, dependencies.get(facade)!);
    replacements.set(owned, join(scratch, `lib${owned}-prepared_custody.rlib`));
    const metadata = (crate: string, source: string, compileDeps: Map<string, string>) => {
      const output = join(scratch, `lib${crate}-prepared_mcp.${execute ? "rlib" : "rmeta"}`);
      if (!process.argv.includes("--reuse-built") || !existsSync(output)) {
        compile("--crate-name", crate, "--crate-type", "rlib", ...(execute ? ["-Copt-level=1"] : ["--emit", "metadata"]), "-Cmetadata=prepared_mcp",
          source, "-o", output, ...args(compileDeps, replacements));
      }
      replacements.set(crate, output);
    };
    metadata(guest, join(host, "🎭️guest-runtime", "🦀️.rs"), deps(guest, facadeDeps.get(guest)!));
    metadata(shard, join(host, "🧵️shard", "🦀️.rs"), deps(shard, facadeDeps.get(shard)!));
    metadata(relay, join(host, "🔀️post-turn-relay", "🦀️.rs"), deps(relay, facadeDeps.get(relay)!));
    metadata(executor, join(host, "🧵️shard", "🧵️executor", "🦀️.rs"), deps(executor, facadeDeps.get(executor)!));
    metadata(facade, join(host, "📦️packages", "🦀️rust", "🦀️.rs"), facadeDeps);
    const catalogRoot = join(workspace, "..", "📚️catalog-compiler");
    process.env.CARGO_MANIFEST_DIR = join(catalogRoot, "📦️packages", "🦀️rust");
    metadata(catalog, join(catalogRoot, "🦀️.rs"), deps(catalog, dependencies.get(catalog)!));
  }
  await Bun.write(wrapper, `#[path = ${JSON.stringify(module)}]\npub mod compiled_components;\n`);
  const output = join(scratch, execute ? (process.platform === "win32" ? "mcp-compiled-components-check.exe" : "mcp-compiled-components-check") : "mcp-compiled-components-check.rmeta");
  compile("--crate-name", "mcp_compiled_components_check", ...(cacheTest ? ["--test"] : ["--crate-type", "rlib"]), ...(execute ? ["-Cdebuginfo=0", "-Cstrip=symbols"] : ["--emit", "metadata"]), wrapper, "-o", output,
    ...args(new Map([...dependencies].filter(([name]) => needed.has(name))), replacements));
  if (execute) process.exit(Bun.spawnSync([output, "--nocapture"], { env: { ...process.env, TMPDIR: scratch, TMP: scratch, TEMP: scratch }, stdout: "inherit", stderr: "inherit" }).exitCode ?? 1);
  process.exit(0);
}
const direct = deps(owned, identityFor(owned));
const overrides = new Map<string, string>();
function rebuild(crate: string, source: string, dependencies: Map<string, string>) {
  const output = join(scratch, `lib${crate}-prepared_custody.rlib`);
  if (!process.argv.includes("--reuse-built")) {
    compile("--crate-name", crate, "--crate-type", "rlib", "-Copt-level=1", "-Cmetadata=prepared_custody", source, "-o", output, ...args(dependencies, overrides));
  }
  overrides.set(crate, output);
}
rebuild(core, join(interpreter, "🦀️.rs"), deps(core, direct.get(core)!));
rebuild(imports, join(interpreter, "🚪️io", "📞️imports", "🦀️.rs"), deps(imports, direct.get(imports)!));
rebuild(owned, join(runtime, "🦀️.rs"), direct);
const testSource = join(runtime, "🏗️startup", "🧪️tests", "prepared.rs");
const wrapper = join(scratch, "mcp-prepared-startup.rs");
await Bun.write(wrapper, `extern crate semio_framework_value_testing_allocator as _;\n#[path = ${JSON.stringify(testSource)}]\nmod prepared;\n`);
const root = new Map([...direct].filter(([crate]) => ["semio_framework_actor", "semio_framework_job", "semio_framework_trace", "semio_framework_value", "serde_json", "semio_os_kernel"].includes(crate)));
const allocator = "semio_framework_value_testing_allocator";
const trace = "semio_framework_trace";
const compatibleAllocator = variants.get(packageName(allocator))?.find(id => deps(allocator, id).get(trace) === direct.get(trace));
if (!compatibleAllocator) throw Error("Missing allocator matching runtime trace identity");
root.set(allocator, compatibleAllocator);
const testBin = join(scratch, process.platform === "win32" ? "mcp-prepared-startup.exe" : "mcp-prepared-startup");
compile("--test", wrapper, "-o", testBin, ...args(root), ...external(owned, overrides.get(owned)!));
process.exit(Bun.spawnSync([testBin, "--nocapture"], { stdout: "inherit", stderr: "inherit" }).exitCode ?? 1);
