import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const mcp = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
const source = readFileSync(join(mcp, "🏠️workspace/🧊️compiled-components/🦀️.rs"), "utf8");
const suite = readFileSync(join(mcp, "🏠️workspace/🧪️tests/🔬️quick/🤖️headless-agent-and-pending-exchanges/🦀️.rs"), "utf8");
const fixture = readFileSync(join(mcp, "🧪️tests/🧫️fixtures/🔣️first-party-codecs.json"), "utf8");
const imports = source.match(/^use semio_os_mcp_(?:foundation::errors|port::channel_port)\b[^\n]*;$/gm)?.join("\n");
const mapper = source.match(/^pub fn activation_fault\([^\n]+\{[\s\S]*?^\}/m)?.[0];
const test = suite.match(/^fn activation_preserves_the_channel_fault_on_the_gateway_wire\(\) \{[\s\S]*?^\}/m)?.[0]
  ?.replace(/^    use semio_os_mcp_plugin_runtime::compiled_components::activation_fault;\n/m, "");
if (!imports || !mapper || !test) throw Error("Missing production activation fault law");

const scratch = resolve(import.meta.dir, "🗑️generated");
const build = join(scratch, "native-mcp/debug/build");
const variants = (pkg: string) => readdirSync(join(build, pkg)).filter(name => existsSync(join(build, pkg, name, "out")));
const extension = process.platform === "win32" ? ".exe" : "";
const input = join(scratch, "mcp-activation-fault.rs");
const output = join(scratch, `mcp-activation-fault${extension}`);
writeFileSync(input, `#![allow(unused_imports)]\n${imports}\n${mapper}\nfn fixture_text(_: &str) -> &'static str { r###"${fixture}"### }\n#[test]\n${test}\n`);

const port = "semio-os-mcp-port";
const portVariant = variants(port).filter(variant => existsSync(join(build, port, variant, "fingerprint/lib-semio_os_mcp_port.json"))).at(-1);
if (!portVariant) throw Error("Missing linked MCP port crate");
const fingerprint = readFileSync(join(build, port, portVariant, "fingerprint/lib-semio_os_mcp_port.json"), "utf8");
const dependencies = ["semio_os_mcp_port", "semio_os_mcp_foundation", "semio_framework_value", "serde_json"].flatMap(crate => {
  const pkg = crate === "serde_json" ? crate : crate.replaceAll("_", "-");
  const variant = crate === "semio_os_mcp_port" ? portVariant : variants(pkg).find(candidate => {
    const value = fingerprint.match(new RegExp(`"${crate}",false,(\\d+)`))?.[1];
    const identity = join(build, pkg, candidate, `fingerprint/lib-${crate}`);
    const littleEndian = value && Buffer.from(BigInt(value).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
    return existsSync(identity) && readFileSync(identity, "utf8").trim() === littleEndian;
  });
  if (!variant) throw Error(`Missing exact linked crate ${crate}`);
  const library = join(pkg, variant, "out", `lib${crate}-${variant}`);
  return ["--extern", `${crate}=${library}.rmeta`, "--extern", `${crate}=${library}.rlib`];
});

const directories = readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory()).flatMap(entry => variants(entry.name).map(variant => join(entry.name, variant, "out")));
const searchPaths = directories.flatMap(path => ["-L", `dependency=${path}`, ...(readdirSync(join(build, path)).some(file => /\.(a|lib)$/.test(file)) ? ["-L", `native=${path}`] : [])]);
const list = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
if (list.exitCode) process.exit(list.exitCode);
const rustup = JSON.parse(list.stdout.toString()).find((item: { id: string }) => item.id === "rustup")?.command;
const toolchain = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/m)?.[1];
if (!rustup || !toolchain) throw Error("Pinned Rust compiler unavailable");
const compiled = Bun.spawnSync([rustup, "run", toolchain, "rustc", "--edition=2024", "--test", input, "-o", output, ...dependencies, ...searchPaths], { cwd: build, stdout: "inherit", stderr: "inherit" });
if (compiled.exitCode) process.exit(compiled.exitCode);
process.exit(Bun.spawnSync([output], { stdout: "inherit", stderr: "inherit" }).exitCode);
