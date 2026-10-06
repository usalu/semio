import assert from "node:assert/strict";
import {readFileSync,writeFileSync,renameSync} from "node:fs";
import {join,resolve} from "node:path";
const root=process.cwd(),generated=resolve(import.meta.dir,"../🗑️generated"),engine="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine",wgpu=`${engine}/🎯️targets/🧊️wgpu`;
function edit(path:string,transform:(text:string)=>string):void{const file=join(root,path),before=readFileSync(file,"utf8"),after=transform(before);assert.notEqual(after,before);const staged=join(generated,`entrypoint-stage-${Date.now()}`);writeFileSync(staged,after);assert.equal(readFileSync(file,"utf8"),before);renameSync(staged,file);}
if(process.argv[2]==="test")edit(`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,text=>text+readFileSync(join(import.meta.dir,"📓️test.rs.md"),"utf8"));
if(process.argv[2]==="test-path")for(const path of [`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️native-entrypoint/📓️test.rs.md"])edit(path,text=>text.replace('include_str!("../../../🎯️targets/','include_str!("../../🎯️targets/'));
if(process.argv[2]==="status-test"){
  edit("🧰️framework/🔨️modules/🕸️graph/🧫️fixtures/🎮️renderer-inputs/🔣️.json",text=>{const data=JSON.parse(text);data.nativeProcessStatus={continuous:false,childFailureExit:23};return JSON.stringify(data,null,2)+"\n";});
  edit("🧰️framework/🔨️modules/🕸️graph/🧬️schema/🎮️renderer-inputs/🔣️.json",text=>{const data=JSON.parse(text);data.required.push("nativeProcessStatus");data.properties.nativeProcessStatus={const:{continuous:false,childFailureExit:23}};return JSON.stringify(data,null,2)+"\n";});
  edit("🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️native-prerequisites/🟦️.ts",text=>text+readFileSync(join(import.meta.dir,"📓️status-test.ts.md"),"utf8"));
}
if(process.argv[2]==="status-fix")edit("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs",text=>{const anchor='cache: false, continuous: operation === "run", outputs: [],';assert.ok(text.includes(anchor));return text.replace(anchor,'cache: false, continuous: false, outputs: [],');});
if(process.argv[2]==="status-oracle")for(const path of ["🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️native-prerequisites/🟦️.ts",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️native-entrypoint/📓️status-test.ts.md"])edit(path,text=>text.replace('cwd:workspace,parallel:false,color:false}','cwd:workspace,parallel:false,color:false,__unparsed__:[]}'));
if(process.argv[2]==="fix"){
  edit(`${wgpu}/⌨️native-entrypoint/🦀️.rs`,text=>{const nested='    fn drive_entrypoint<F: std::future::Future>(future: F) -> F::Output {\n        semio_framework_async::block_on(future)\n    }\n';assert.ok(text.includes(nested));return readFileSync(join(import.meta.dir,"📓️driver.rs.md"),"utf8")+text.replace(nested,"").replaceAll("drive_entrypoint(","drive_native_entrypoint(");});
  edit(`${wgpu}/🏗️builder/🦀️.rs`,text=>{const anchor='fn main() {\n';assert.ok(text.includes(anchor));return text.replace(anchor,anchor+'    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {\n        let stack = if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") { "/STACK:8388608" } else { "-Wl,--stack,8388608" };\n        println!("cargo:rustc-link-arg-bin=semio-wgpu-native={stack}");\n    }\n');});
}
if(process.argv[2]==="surface-test")edit(`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,text=>text+readFileSync(join(import.meta.dir,"📓️surface-test.rs.md"),"utf8"));
if(process.argv[2]==="surface-fix"){
  edit("🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs",text=>{
    const signature='    #[cfg(not(target_os = "wasi"))]\n    pub async fn from_window(window: Arc<winit::window::Window>) -> Result<Self, String> {';
    assert.ok(text.includes(signature));
    return text.replace(signature,'    /// 🪟️ Captures the event-thread window surface before asynchronous device preparation.\n    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "wasi")))]\n    pub fn from_window(window: Arc<winit::window::Window>) -> Result<impl std::future::Future<Output = Result<Self, String>> + Send, String> {').replace('Self::from_surface(instance, surface, css_width, css_height, dpr).await\n    }','Ok(Self::from_surface(instance, surface, css_width, css_height, dpr))\n    }');
  });
  edit(`${wgpu}/🪟️winit-app/🦀️.rs`,text=>{
    const anchor='            crate::spawn_app_task(async move {\n                let result = crate::boot_runtime(\n                    window,',ready='                HostUserEvent::RuntimeReady { runtime, presenter } => {\n';
    assert.ok(text.includes(anchor));assert.ok(text.includes(ready));
    return text.replace(anchor,'            let gpu = crate::GpuContext::from_window(window.clone()).expect("native GPU surface capture");\n'+anchor+'\n                    gpu,').replace(ready,ready+'                    crate::log_debug_diagnostic("[TRACE] native renderer boot ready");\n');
  });
  edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>{
    const signature='async fn boot_runtime(\n    window: Arc<Window>,',call='    let mut gpu = GpuContext::from_window(window.clone()).await.map_err(|err| format!("gpu init failed: {err}"))?;';
    assert.ok(text.includes(signature));assert.ok(text.includes(call));
    return text.replace(signature,signature+'\n    gpu: impl std::future::Future<Output = Result<GpuContext, String>> + Send + \'static,').replace(call,'    let mut gpu = gpu.await.map_err(|err| format!("gpu init failed: {err}"))?;');
  });
}
if(process.argv[2]==="live-fix")edit(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️live/📜️script.ts",text=>{
  const smoke='if(renderer==="native-smoke")extra.push(...(node.leaf.args.includes("--")?[]:["--"]),"--smoke");';assert.ok(text.includes(smoke));
  return text.replace(smoke,'if(renderer==="native-smoke"){const target="@semio-tech/framework-os-dev:smoke-draw-native-dev",graph=JSON.parse(readFileSync(join(root,".nx/workspace-data/project-graph.json"),"utf8"));assert.ok(graph.nodes["@semio-tech/framework-os-dev"].data.targets["smoke-draw-native-dev"]);node={leaf:{...node.leaf,args:["nx","run",target,"--","--example",example]}};}').replace('key === "RUSTC_WRAPPER"','key === "RUSTC_WRAPPER" || key === "SEMIO_RUNTIME_DIAGNOSTICS"').replace('    return Boolean(window);','    assert.ok(!text.includes("boot_runtime failed:"),text.slice(-4000));\n    return Boolean(window) && text.includes("native renderer boot ready");').replace('    send({ type: "restart", session_id: id });','    text="";\n    send({ type: "restart", session_id: id });');
});
if(process.argv[2]==="live-verbose")edit(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️live/📜️script.ts",text=>text.replace('args:["nx","run",target,"--","--example",example]','args:["nx","run",target,"--verbose","--output-style=stream","--","--example",example]'));
if(process.argv[2]==="worker-test")edit(`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,text=>text+readFileSync(join(import.meta.dir,"📓️worker-test.rs.md"),"utf8"));
if(process.argv[2]==="worker-fix")edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>{
  const anchor='#[cfg(not(target_arch = "wasm32"))]\nfn spawn_app_task<F>(future: F)',call='KernelPoolFuture::spawn(renderer_worker_pool(), semio_framework_async::Lane::Interactive, future);';
  assert.ok(text.includes(anchor));assert.ok(text.includes(call));
  return text.replace(anchor,readFileSync(join(import.meta.dir,"📓️worker-driver.rs.md"),"utf8")+anchor).replace(call,'KernelPoolFuture::spawn(renderer_worker_pool(), semio_framework_async::Lane::Interactive, drive_app_task(future));');
});
if(process.argv[2]==="tail")for(const name of process.argv.slice(3)){assert.equal(name, name.split(/[\\/]/).at(-1));const output=readFileSync(join(generated,name),"utf8").replaceAll("\0","");console.log(`[DEBUG] ${name}\n${output.slice(-1900)}`);}
if(process.argv[2]==="tree-env"){let node=JSON.parse(readFileSync(join(generated,"command-tree.json"),"utf8"));for(const key of ["dev","draw","draw","wgpu-native","examples","🎬️demo","language","en","terminology","native"])node=node.children.find(child=>child.key===key);console.log(JSON.stringify(node.leaf));}
if(process.argv[2]==="kernel-test")edit(`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,text=>text+readFileSync(join(import.meta.dir,"📓️worker-test.rs.md"),"utf8").replaceAll("native_app_worker_completes_io_without_a_presentation_host","native_kernel_worker_completes_io_without_a_presentation_host").replace("    spawn_app_task(async move {","    let _task=kernel_runtime::KernelPoolFuture::spawn(renderer_worker_pool(),semio_framework_async::Lane::Interactive,async move {"));
if(process.argv[2]==="kernel-fix")edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>{
 const stored='future: Mutex::new(Some(Box::pin(future))), scheduled:';assert.ok(text.includes(stored));
 return text.replace(stored,'future: Mutex::new(Some(Box::pin(crate::drive_app_task(future)))), scheduled:').replace('KernelPoolFuture::spawn(renderer_worker_pool(), semio_framework_async::Lane::Interactive, drive_app_task(future));','KernelPoolFuture::spawn(renderer_worker_pool(), semio_framework_async::Lane::Interactive, future);');
});
if(process.argv[2]==="evidence")for(const name of process.argv.slice(3)){assert.equal(name,name.split(/[\\/]/).at(-1));const output=readFileSync(join(generated,name),"utf8").replaceAll("\0","").replace(/\x1b\[[0-9;?]*[A-Za-z]/g,"");const lines=output.split(/\r?\n/),selected=new Set<number>();lines.forEach((line,index)=>{if(/test result:|panicked at|FAILED|^failures:|error\[|^running \d+ tests?/.test(line))for(let n=Math.max(0,index-1);n<Math.min(lines.length,index+7);n++)selected.add(n);});console.log(`[DEBUG] ${name}\n${[...selected].sort((a,b)=>a-b).map(index=>lines[index]).join("\n").slice(-5500)}`);}
if(process.argv[2]==="kernel-contract"){
 edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>text.replaceAll("drive_app_task","drive_renderer_worker_task").replace("on the application worker before presentation exists","on application and kernel workers before presentation exists"));
 edit(`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,text=>{
  text=text.replace("use super::*;","use super::*;\n\nstatic RENDERER_IO_TEST_OWNER: Mutex<()> = Mutex::new(());");
  for(const name of ["a_contended_renderer_io_poll_wakes_itself_until_the_exact_session_can_register","mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact","headless_entrypoint_drives_retained_io_and_matches_the_json_oracle","native_app_worker_completes_io_without_a_presentation_host","native_kernel_worker_completes_io_without_a_presentation_host"]){const anchor=`fn ${name}() {`;assert.ok(text.includes(anchor));text=text.replace(anchor,anchor+'\n    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());');}
  const anchor="fn native_kernel_worker_completes_io_without_a_presentation_host()";const split=text.indexOf(anchor);assert.ok(split>0);return text.slice(0,split).replace('format!("worker-io-{}.json"','format!("application-worker-io-{}.json"')+text.slice(split).replace('format!("worker-io-{}.json"','format!("kernel-worker-io-{}.json"').replace("application worker must advance","kernel worker must advance");
 });
 edit(`${wgpu}/⌨️native-entrypoint/🧪️tests/🚪️headless.feature`,text=>text+'  Scenario: Boot a kernel before a presentation host exists\n    Given a retained page read submitted directly from the native kernel worker\n    When the worker runs without application or presentation callbacks\n    Then it completes the page read within the worker\'s bounded turns\n    And its result matches the independent filesystem and JSON parser\n');
 edit(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️live/📜️script.ts",text=>{const anchor='    if (!session?.pid) return false;';assert.ok(text.includes(anchor));return text.replace(anchor,'    assert.ok(!text.includes("boot_runtime failed:"),text.slice(-4000));\n    if (!session?.pid || !text.includes("native renderer boot ready")) return false;');});
}
if(process.argv[2]==="component-test"){
 edit(`${wgpu}/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json`,text=>{const data=JSON.parse(text);Object.assign(data,{localComponentMaxBytes:268435456,networkComponentMaxBytes:67108864,localComponentTestBytes:67108865});return JSON.stringify(data,null,2)+"\n";});
 edit(`${wgpu}/⌨️native-entrypoint/🧪️tests/🚪️headless.feature`,text=>text+'  Scenario: Read the unoptimized local development component\n    Given a locally produced component above the network catalog limit\n    And the local development producer admits up to 268435456 bytes\n    When the native kernel reads it in retained pages\n    Then its exact bytes match the independent filesystem and JSON parser\n    And network catalog admission remains bounded to 67108864 bytes\n');
 edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>text.replace('    async fn read_native_component(path:','    pub(super) async fn read_native_component(path:'));
 edit(`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,text=>text+readFileSync(join(import.meta.dir,"📓️component-test.rs.md"),"utf8"));
}
if(process.argv[2]==="component-fix")edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>{
 const anchor='        let bound = semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES;';assert.ok(text.includes(anchor));
 return text.replace(anchor,'        let bound = 256 * 1024 * 1024;').replace('    /// whole-file read refused every real guest (block release is 17.6 MB) and no native shell could\n    /// mount one; the bound is the execution-target component bound the hub itself enforces.','    /// whole-file read refused every real guest. Local unoptimized components use the same\n    /// 256 MiB input ceiling as descriptor emission; network execution-target admission is separate.');
});
if(process.argv[2]==="smoke-prefix")edit(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️live/📜️script.ts",text=>text.replace('const match=plain.match(', 'const match=plain.replace(/@semio-tech\\/framework-os-dev:\\s*/g,"").match('));
if(process.argv[2]==="component-oracle")for(const path of [`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️native-entrypoint/📓️component-test.rs.md"])edit(path,text=>text.replace('serde_json::from_reader(std::fs::File::open(&path).expect("independent file read"))','serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(&path).expect("independent file read")))'));
if(process.argv[2]==="smoke-test"){
 edit(`${wgpu}/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json`,text=>{const data=JSON.parse(text),hash="0".repeat(64);data.emptySmoke={exitCode:1,booted:false,descriptor:{descriptorVersion:1,packageId:"empty-smoke",role:"plugin",manifest:{pluginId:"empty-smoke",label:"Empty Smoke",version:"1.0.0",apps:[],examples:[]},execution:"isolated",executionProtocol:{appChannelVersion:14},hashes:{wasmSha256:hash,coreWasmSha256:hash,descriptorSha256:hash}},runtime:{version:1,variant:"empty-smoke",profile:"dev",modules:[{pluginId:"empty-smoke",wasmSha256:hash,wasmPath:"component.wasm",descriptorPath:"descriptor.json"}]}};return JSON.stringify(data,null,2)+"\n";});
 edit(`${wgpu}/⌨️native-entrypoint/🧪️tests/🚪️headless.feature`,text=>text+'  Scenario: A selected plugin has no bootable application\n    Given a valid local plugin descriptor without applications\n    And an explicit language and terminology\n    When the native smoke boots that selected plugin\n    Then it reports booted false and exits with status 1\n');
 edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>{
  const signature='pub async fn run_smoke(plugin_filter: &str, plugin_modules_root: std::path::PathBuf,services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>) -> i32 {',axes='    let (locale, terminology) = match shell::shell_language_axes() {\n        Ok(axes) => axes,\n        Err(error) => { eprintln!("smoke: {error}"); return 1; }\n    };';assert.ok(text.includes(signature));assert.ok(text.includes(axes));
  return text.replace(axes,'    let (locale, terminology) = axes;').replace(signature,signature+'\n    let axes = match shell::shell_language_axes() {\n        Ok(axes) => axes,\n        Err(error) => { eprintln!("smoke: {error}"); return 1; }\n    };\n    run_smoke_with_axes(plugin_filter,plugin_modules_root,services,axes).await\n}\n\n/// 🧭️ Boots the exact selected native plugin under caller-declared presentation axes.\n#[cfg(not(target_arch = "wasm32"))]\npub(crate) async fn run_smoke_with_axes(plugin_filter: &str, plugin_modules_root: std::path::PathBuf,services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>,axes:(semio_framework_ui_locale::Locale,semio_framework_ui_locale::Terminology)) -> i32 {');
 });
 edit(`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,text=>text+readFileSync(join(import.meta.dir,"📓️smoke-test.rs.md"),"utf8"));
}
if(process.argv[2]==="smoke-fix")edit(`${wgpu}/🧊️renderer/🦀️.rs`,text=>{
 const anchor='    let report = serde_json::json!({\n        "booted": true,',out='            println!("{json}");\n            0';assert.ok(text.includes(anchor));assert.ok(text.includes(out));return text.replace(anchor,'    let booted = plugin_filter.is_empty() || shell.session.is_some();\n    let report = serde_json::json!({\n        "booted": booted,').replace(out,'            println!("{json}");\n            i32::from(!booted)');
});
if(process.argv[2]==="component-upper-test"){
 for(const path of [`${engine}/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs`,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️native-entrypoint/📓️component-test.rs.md"])edit(path,text=>{
  const anchor='    assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual).expect("native JSON pages"),oracle);';assert.ok(text.includes(anchor));return text.replace(anchor,anchor+'\n    drop(actual);\n    let maximum=contract["localComponentMaxBytes"].as_u64().expect("local component ceiling");\n    std::fs::File::create(&path).expect("oversized local input").set_len(maximum+1).expect("bounded sparse input");\n    let refusal=crate::native_entrypoint::drive_native_entrypoint(kernel_runtime::read_native_component(&path)).expect_err("maximum plus one local byte must be refused");\n    assert!(refusal.contains(&format!("exceeds {maximum} bytes")));');
 });
 edit(`${wgpu}/⌨️native-entrypoint/🧪️tests/🚪️headless.feature`,text=>text.replace('    And network catalog admission remains bounded to 67108864 bytes','    And a local input of 268435457 bytes is refused\n    And network catalog admission remains bounded to 67108864 bytes'));
}
