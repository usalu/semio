import json, os
D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup"
TC = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🔣️.json"
BOOT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json"
TOOLS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"
BUN = TOOLS + "/🟦️bun/🧬️schema/🔣️.json"
BAZEL = TOOLS + "/🏗️bazel/🧬️schema/🔣️.json"
WASM = TOOLS + "/🕸️wasm/🧬️schema/🔣️.json"
TECTONIC = "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🔣️.json"
def text(en, de): return {"en": en, "de": de}
def json_pin(file, pointer): return {"format": "json", "file": file, "pointer": pointer}
def rows_pin(tool): return {"format": "rows", "file": BOOT, "pointer": "/definitions/Pins/const/assets", "where": {"tool": tool}, "field": "version"}
def regex_pin(file, pattern, flags=""): return {"format": "regex", "file": file, "pattern": pattern, **({"flags": flags} if flags else {})}
def value_pin(value): return {"format": "value", "value": value}
def probe(arguments, pattern, mode="version"): return {"mode": mode, "arguments": arguments, "pattern": pattern}
def provisioned(family, executable, tool=None): return {"kind": "provisioned", "family": family, "executable": executable, **({"tool": tool} if tool else {})}
def path(names, directories=()): return {"kind": "path", "names": names, "directories": list(directories)}
def dependencies(environment): return {"kind": "script", "script": TOOLS.rsplit("/", 1)[0] + "/📦️dependencies/🏗️native/📜️script.ts", "prepare": ["sync", environment]}
def cargo(id, title_en, title_de, environment, pin, executable, arguments):
    return {"id": id, "summary": text(title_en, title_de), "pin": pin, "probe": probe(arguments, "^" + executable + " (\\d+\\.\\d+\\.\\d+)"), "locate": [provisioned("cargo", executable), path([executable], ["~/.cargo/bin"])], "setup": dependencies(environment)}
tools = [
 {"id": "bun", "summary": text("JavaScript runtime and package manager that runs every repository script.", "JavaScript-Laufzeit und Paketmanager, der jedes Repository-Skript ausführt."), "pin": regex_pin("package.json", "\"packageManager\"\\s*:\\s*\"bun@(\\d+\\.\\d+\\.\\d+)\""), "probe": probe(["--version"], "^(\\d+\\.\\d+\\.\\d+)"), "locate": [provisioned("bun", "bun")], "setup": {"kind": "bun"}},
 {"id": "node", "summary": text("Node.js for the tools that do not run on Bun.", "Node.js für die Werkzeuge, die nicht auf Bun laufen."), "pin": rows_pin("node"), "probe": probe(["--version"], "^v(\\d+\\.\\d+\\.\\d+)"), "locate": [path(["node"], ["~/.local/node/bin"])], "setup": {"kind": "bootstrap"}},
 {"id": "rustup", "summary": text("Rust toolchain manager.", "Verwaltung der Rust-Toolchains."), "pin": rows_pin("rustup"), "probe": probe(["--version"], "^rustup (\\d+\\.\\d+\\.\\d+)"), "locate": [path(["rustup"], ["~/.cargo/bin"])], "setup": {"kind": "bootstrap"}},
 {"id": "rust", "summary": text("Rust nightly with the WebAssembly targets of rust-toolchain.toml.", "Rust-Nightly mit den WebAssembly-Zielen aus rust-toolchain.toml."), "pin": regex_pin("rust-toolchain.toml", "^channel\\s*=\\s*\"([^\"]+)\"", "m"), "probe": probe(["show", "active-toolchain"], "^(nightly-\\d{4}-\\d{2}-\\d{2})"), "locate": [path(["rustup"], ["~/.cargo/bin"])], "setup": {"kind": "rust"}},
 {"id": "go", "summary": text("Go for the repository's Go packages.", "Go für die Go-Pakete des Repositorys."), "pin": json_pin(TC, "/go/tool/version"), "probe": probe(["version"], "go version go(\\d+\\.\\d+\\.\\d+)"), "locate": [provisioned("native", "go", "go")], "setup": {"kind": "native", "tool": "go"}},
 {"id": "uv", "summary": text("uv for the locked Python environments.", "uv für die gesperrten Python-Umgebungen."), "pin": json_pin(TC, "/python/tool/version"), "probe": probe(["--version"], "^uv (\\d+\\.\\d+\\.\\d+)"), "locate": [provisioned("native", "uv", "uv")], "setup": {"kind": "native", "tool": "uv"}},
 {"id": "python-main", "summary": text("Managed Python interpreter of the workspace and the styling package.", "Verwalteter Python-Interpreter des Workspaces und des Styling-Pakets."), "pin": json_pin(TC, "/python/interpreters/main/series"), "probe": probe(["--version"], "^Python (\\d+\\.\\d+\\.\\d+)"), "locate": [{"kind": "python", "interpreter": "main"}], "setup": {"kind": "python", "interpreter": "main"}},
 {"id": "python-oracle", "summary": text("Managed Python interpreter of the energy oracle.", "Verwalteter Python-Interpreter des Energie-Orakels."), "pin": json_pin(TC, "/python/interpreters/oracle/series"), "probe": probe(["--version"], "^Python (\\d+\\.\\d+\\.\\d+)"), "locate": [{"kind": "python", "interpreter": "oracle"}], "setup": {"kind": "python", "interpreter": "oracle"}},
 {"id": "dotnet", "summary": text(".NET SDK of the styling and test packages.", ".NET-SDK der Styling- und Testpakete."), "pin": json_pin(TC, "/dotnet/sdk"), "probe": probe(["--list-sdks"], "^({pin}) \\[", "listed"), "locate": [provisioned("dotnet", "dotnet"), path(["dotnet"], ["$DOTNET_ROOT", "~/.dotnet"])], "setup": {"kind": "dotnet"}},
 {"id": "cmake", "summary": text("CMake for the C++ lane.", "CMake für die C++-Spur."), "pin": json_pin(TC, "/cmake/cmake/version"), "probe": probe(["--version"], "cmake version (\\d+\\.\\d+\\.\\d+)"), "locate": [provisioned("native", "cmake", "cmake")], "setup": {"kind": "native", "tool": "cmake"}},
 {"id": "ninja", "summary": text("Ninja for the C++ lane.", "Ninja für die C++-Spur."), "pin": json_pin(TC, "/cmake/ninja/version"), "probe": probe(["--version"], "^(\\d+\\.\\d+\\.\\d+)"), "locate": [provisioned("native", "ninja", "ninja")], "setup": {"kind": "native", "tool": "ninja"}},
 {"id": "bazel", "summary": text("Bazel build implementation.", "Bazel als Build-Implementierung."), "pin": json_pin(BAZEL, "/definitions/Release/const/version"), "probe": probe(["--version"], "^bazel (\\d+\\.\\d+\\.\\d+)"), "locate": [provisioned("bazel", "bazel")], "setup": {"kind": "bazel"}},
 {"id": "wasm-opt", "summary": text("Binaryen optimizer for WebAssembly components.", "Binaryen-Optimierer für WebAssembly-Komponenten."), "pin": json_pin(WASM, "/definitions/Release/const/version"), "probe": probe(["--version"], "version_?(\\d+)"), "locate": [provisioned("binaryen", "wasm-opt")], "setup": {"kind": "binaryen"}},
 {"id": "tectonic", "summary": text("Tectonic TeX engine for the print pipeline.", "Tectonic-TeX-Engine für die Druck-Pipeline."), "pin": json_pin(TECTONIC, "/version"), "probe": {"mode": "hook", "arguments": [], "pattern": ""}, "locate": [{"kind": "hook"}], "setup": {"kind": "script", "script": "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts", "prepare": ["prepare"], "verify": ["verify"]}},
 cargo("wasm-pack", "wasm-pack builds the browser bindings.", "wasm-pack baut die Browser-Bindings.", "wasm", value_pin("0.15.0"), "wasm-pack", ["--version"]),
 cargo("wasm-bindgen", "wasm-bindgen generates the bindings; it must equal the locked crate.", "wasm-bindgen erzeugt die Bindings; es muss dem gesperrten Crate entsprechen.", "wasm", regex_pin("Cargo.lock", "^name = \"wasm-bindgen\"\\r?\\nversion = \"([^\"]+)\"$", "m"), "wasm-bindgen", ["--version"]),
 cargo("trunk", "Trunk bundles the browser applications.", "Trunk bündelt die Browser-Anwendungen.", "trunk", value_pin("0.21.14"), "trunk", ["--version"]),
 cargo("cargo-nextest", "nextest runs the Rust tests.", "nextest führt die Rust-Tests aus.", "tools", value_pin("0.9.140"), "cargo-nextest", ["--version"]),
 cargo("cargo-llvm-cov", "llvm-cov measures Rust coverage.", "llvm-cov misst die Rust-Abdeckung.", "tools", value_pin("0.8.7"), "cargo-llvm-cov", ["llvm-cov", "--version"]),
 {"id": "chromium", "summary": text("Playwright Chromium for the browser tests.", "Playwright-Chromium für die Browser-Tests."), "pin": {"format": "rows", "package": "playwright-core", "from": "playwright", "file": "browsers.json", "pointer": "/browsers", "where": {"name": "chromium"}, "field": "revision"}, "probe": {"mode": "hook", "arguments": [], "pattern": ""}, "locate": [{"kind": "hook"}], "setup": dependencies("browsers")},
]
messages = {
 "languages": ["en", "de"],
 "texts": {
  "header": text("tool           pin                   found                 status    location", "Werkzeug        Pin                   gefunden              Status    Ort"),
  "ok": text("ok", "ok"),
  "missing": text("missing", "fehlt"),
  "differs": text("differs", "weicht ab"),
  "failed": text("failed", "fehlgeschlagen"),
  "clean": text("All {count} pinned tools match their pins.", "Alle {count} gepinnten Werkzeuge entsprechen ihren Pins."),
  "drift": text("{count} of {total} pinned tools differ from their pins; run `repo tools setup all`.", "{count} von {total} gepinnten Werkzeugen weichen von ihren Pins ab; führe `repo tools setup all` aus."),
  "step": text("Provisioning {tool} {pin}.", "Richte {tool} {pin} ein."),
  "done": text("{tool} {found} is ready.", "{tool} {found} ist bereit."),
  "skipped": text("{tool} {found} already matches its pin.", "{tool} {found} entspricht bereits dem Pin."),
  "manual": text("{tool} {pin} is installed by the bootstrap script, not by this command; run the bootstrap again.", "{tool} {pin} wird vom Bootstrap-Skript installiert, nicht von diesem Befehl; führe das Bootstrap erneut aus."),
  "mismatch": text("{tool} was provisioned but reports {found}, not the pin {pin}.", "{tool} wurde eingerichtet, meldet aber {found} statt des Pins {pin}."),
  "usage": text("{command} takes {choices}.", "{command} erwartet {choices}."),
  "unknown": text("Unknown pinned tool: {tool}.", "Unbekanntes gepinntes Werkzeug: {tool}."),
  "pinfile": text("The pin of {tool} cannot be read from {file}.", "Der Pin von {tool} ist in {file} nicht lesbar."),
  "installer": text("The pinned installer for {tool} is missing for the host {host}.", "Das gepinnte Installationsprogramm für {tool} fehlt für den Host {host}."),
  "runtime": text("Warning: the running Bun {found} is not the pinned Bun {pin}; start the repository tools through the repo launcher, which selects the pinned Bun.", "Warnung: das laufende Bun {found} ist nicht das gepinnte Bun {pin}; starte die Repository-Werkzeuge über den repo-Launcher, der das gepinnte Bun wählt."),
  "cancelled": text("Provisioning was cancelled.", "Die Einrichtung wurde abgebrochen."),
  "progress": text("{tool}: {received} of {bytes} bytes ({percent} %).", "{tool}: {received} von {bytes} Bytes ({percent} %).")
 }
}
document = {"$schema": "./🧬️schema/🔣️.json", "version": 1, "groups": {"all": ["bun", "node", "rustup", "rust", "go", "uv", "python-main", "python-oracle", "dotnet", "cmake", "ninja", "wasm-opt", "tectonic", "wasm-pack", "wasm-bindgen", "trunk", "cargo-nextest", "cargo-llvm-cov", "chromium", "bazel"]}, "tools": tools, "messages": messages}
os.makedirs(D, exist_ok=True)
open(D + "/🔣️.json", "w", encoding="utf8").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
print(len(tools), "tools")
