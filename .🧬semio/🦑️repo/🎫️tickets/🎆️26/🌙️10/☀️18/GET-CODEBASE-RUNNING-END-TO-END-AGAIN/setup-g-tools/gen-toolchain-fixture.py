"""Generates the toolchain probe scenarios (expected values authored by hand below, never computed by the code under test)."""
import json, sys
out = sys.argv[1]
P = lambda i, f, l, h: {"id": i, "found": f, "location": l, "how": h}
def sc(id, platform, arch, env, home, files, directories, commands, expected):
    return {"id": id, "platform": platform, "arch": arch, "env": env, "home": home, "files": files, "directories": directories, "commands": commands, "expected": expected}
XS = lambda status, stdout: {"program": "/usr/bin/xcode-select", "arguments": ["-p"], "status": status, "stdout": stdout}
VS_X64 = "Microsoft.VisualStudio.Component.VC.Tools.x86.x64"
VS_ARM = "Microsoft.VisualStudio.Component.VC.Tools.ARM64"
VSW = "C:\\Program Files (x86)\\Microsoft Visual Studio\\Installer\\vswhere.exe"
VSQ = lambda comp, stdout, status=0: {"program": VSW, "arguments": ["-latest", "-products", "*", "-requires", comp, "-property", "installationPath"], "status": status, "stdout": stdout}
BT = "C:\\Program Files\\Microsoft Visual Studio\\2022\\BuildTools"
scenarios = [
 sc("linux-all-on-path", "linux", "x64", {"PATH": "/usr/local/bin:/usr/bin:/home/dev/.cargo/bin"}, "/home/dev", ["/usr/bin/cc", "/home/dev/.cargo/bin/rustc"], [], [], [P("cc", True, "/usr/bin/cc", "path"), P("rust", True, "/home/dev/.cargo/bin/rustc", "path")]),
 sc("linux-rust-only-in-cargo-home", "linux", "arm64", {"PATH": "/usr/bin"}, "/home/dev", ["/usr/bin/cc", "/home/dev/.cargo/bin/rustup"], [], [], [P("cc", True, "/usr/bin/cc", "path"), P("rust", True, "/home/dev/.cargo/bin/rustup", "cargo-home")]),
 sc("linux-cargo-home-variable", "linux", "x64", {"PATH": "/usr/bin", "CARGO_HOME": "/opt/cargo"}, "/home/dev", ["/usr/bin/cc", "/opt/cargo/bin/rustc", "/home/dev/.cargo/bin/rustc"], [], [], [P("cc", True, "/usr/bin/cc", "path"), P("rust", True, "/opt/cargo/bin/rustc", "cargo-home")]),
 sc("linux-named-compiler-wins", "linux", "x64", {"PATH": "/usr/bin", "SEMIO_RUSTC": "/opt/rust/bin/rustc"}, "/home/dev", ["/usr/bin/cc", "/usr/bin/rustc", "/opt/rust/bin/rustc"], [], [], [P("cc", True, "/usr/bin/cc", "path"), P("rust", True, "/opt/rust/bin/rustc", "environment")]),
 sc("linux-named-compiler-missing-is-not-replaced", "linux", "x64", {"PATH": "/usr/bin", "SEMIO_RUSTC": "/opt/rust/bin/rustc"}, "/home/dev", ["/usr/bin/cc", "/usr/bin/rustc"], [], [], [P("cc", True, "/usr/bin/cc", "path"), P("rust", False, None, "none")]),
 sc("linux-nothing-installed", "linux", "x64", {"PATH": "/usr/bin::/bin"}, "/home/dev", [], [], [], [P("cc", False, None, "none"), P("rust", False, None, "none")]),
 sc("linux-compiler-before-manager-in-one-directory", "linux", "x64", {"PATH": "/usr/bin"}, "/home/dev", ["/usr/bin/cc", "/usr/bin/rustc", "/usr/bin/rustup"], [], [], [P("cc", True, "/usr/bin/cc", "path"), P("rust", True, "/usr/bin/rustc", "path")]),
 sc("darwin-ready", "darwin", "arm64", {"PATH": "/usr/bin:/Users/dev/.cargo/bin"}, "/Users/dev", ["/usr/bin/cc", "/Users/dev/.cargo/bin/rustc"], ["/Library/Developer/CommandLineTools"], [XS(0, "/Library/Developer/CommandLineTools\n")], [P("cc", True, "/usr/bin/cc", "path"), P("developer-tools", True, "/Library/Developer/CommandLineTools", "developer-directory"), P("rust", True, "/Users/dev/.cargo/bin/rustc", "path")]),
 sc("darwin-shim-without-developer-tools", "darwin", "x64", {"PATH": "/usr/bin"}, "/Users/dev", ["/usr/bin/cc"], [], [XS(2, "")], [P("cc", True, "/usr/bin/cc", "path"), P("developer-tools", False, None, "none"), P("rust", False, None, "none")]),
 sc("darwin-developer-dir-variable", "darwin", "arm64", {"PATH": "/usr/bin", "DEVELOPER_DIR": "/Applications/Xcode.app/Contents/Developer"}, "/Users/dev", ["/usr/bin/cc"], ["/Applications/Xcode.app/Contents/Developer"], [], [P("cc", True, "/usr/bin/cc", "path"), P("developer-tools", True, "/Applications/Xcode.app/Contents/Developer", "environment"), P("rust", False, None, "none")]),
 sc("darwin-stale-developer-path", "darwin", "arm64", {"PATH": "/usr/bin"}, "/Users/dev", ["/usr/bin/cc"], [], [XS(0, "/Library/Developer/CommandLineTools\n")], [P("cc", True, "/usr/bin/cc", "path"), P("developer-tools", False, None, "none"), P("rust", False, None, "none")]),
 sc("darwin-selector-prints-nothing", "darwin", "arm64", {"PATH": "/usr/bin"}, "/Users/dev", ["/usr/bin/cc"], ["/Library/Developer/CommandLineTools"], [XS(0, "")], [P("cc", True, "/usr/bin/cc", "path"), P("developer-tools", False, None, "none"), P("rust", False, None, "none")]),
 sc("windows-linker-on-path-case-insensitive-variable", "win32", "x64", {"Path": "C:\\Windows\\System32;C:\\VS\\VC\\bin;C:\\Users\\dev\\.cargo\\bin"}, "C:\\Users\\dev", ["C:\\VS\\VC\\bin\\link.exe", "C:\\Users\\dev\\.cargo\\bin\\rustc.exe"], [], [], [P("msvc-linker", True, "C:\\VS\\VC\\bin\\link.exe", "path"), P("rust", True, "C:\\Users\\dev\\.cargo\\bin\\rustc.exe", "path")]),
 sc("windows-build-tools-found-by-vswhere", "win32", "x64", {"Path": "C:\\Windows\\System32", "ProgramFiles(x86)": "C:\\Program Files (x86)"}, "C:\\Users\\dev", [VSW, "C:\\Users\\dev\\.cargo\\bin\\rustup.exe"], [BT + "\\VC\\Tools\\MSVC"], [VSQ(VS_X64, BT + "\r\n")], [P("msvc-linker", True, BT, "installer"), P("rust", True, "C:\\Users\\dev\\.cargo\\bin\\rustup.exe", "cargo-home")]),
 sc("windows-arm64-asks-for-the-arm64-component", "win32", "arm64", {"Path": "C:\\Windows\\System32", "ProgramFiles(x86)": "C:\\Program Files (x86)"}, "C:\\Users\\dev", [VSW], [BT + "\\VC\\Tools\\MSVC"], [VSQ(VS_X64, BT + "\r\n"), VSQ(VS_ARM, BT + "\r\n")], [P("msvc-linker", True, BT, "installer"), P("rust", False, None, "none")]),
 sc("windows-arm64-component-missing", "win32", "arm64", {"Path": "C:\\Windows\\System32", "ProgramFiles(x86)": "C:\\Program Files (x86)"}, "C:\\Users\\dev", [VSW], [BT + "\\VC\\Tools\\MSVC"], [VSQ(VS_X64, BT + "\r\n")], [P("msvc-linker", False, None, "none"), P("rust", False, None, "none")]),
 sc("windows-install-without-c-tools", "win32", "x64", {"Path": "C:\\Windows\\System32", "ProgramFiles(x86)": "C:\\Program Files (x86)"}, "C:\\Users\\dev", [VSW], [], [VSQ(VS_X64, BT + "\r\n")], [P("msvc-linker", False, None, "none"), P("rust", False, None, "none")]),
 sc("windows-vswhere-finds-nothing", "win32", "x64", {"Path": "C:\\Windows\\System32", "ProgramFiles(x86)": "C:\\Program Files (x86)"}, "C:\\Users\\dev", [VSW], [], [VSQ(VS_X64, "")], [P("msvc-linker", False, None, "none"), P("rust", False, None, "none")]),
 sc("windows-vswhere-fails", "win32", "x64", {"Path": "C:\\Windows\\System32", "ProgramFiles(x86)": "C:\\Program Files (x86)"}, "C:\\Users\\dev", [VSW], [BT + "\\VC\\Tools\\MSVC"], [VSQ(VS_X64, BT + "\r\n", 1)], [P("msvc-linker", False, None, "none"), P("rust", False, None, "none")]),
 sc("windows-vswhere-only-on-path", "win32", "x64", {"Path": "C:\\Tools;C:\\Windows\\System32"}, "C:\\Users\\dev", ["C:\\Tools\\vswhere.exe"], [BT + "\\VC\\Tools\\MSVC"], [{"program": "C:\\Tools\\vswhere.exe", "arguments": ["-latest", "-products", "*", "-requires", VS_X64, "-property", "installationPath"], "status": 0, "stdout": BT + "\r\n"}], [P("msvc-linker", True, BT, "installer"), P("rust", False, None, "none")]),
 sc("windows-installation-variable", "win32", "x64", {"Path": "C:\\Windows\\System32", "VSINSTALLDIR": BT}, "C:\\Users\\dev", [], [BT + "\\VC\\Tools\\MSVC"], [], [P("msvc-linker", True, BT, "environment"), P("rust", False, None, "none")]),
 sc("windows-nothing-installed", "win32", "x64", {"Path": "C:\\Windows\\System32"}, "C:\\Users\\dev", [], [], [], [P("msvc-linker", False, None, "none"), P("rust", False, None, "none")]),
 sc("windows-cargo-home-variable-any-case", "win32", "x64", {"Path": "C:\\Windows\\System32", "cargo_home": "D:\\cargo"}, "C:\\Users\\dev", ["D:\\cargo\\bin\\rustc.exe"], [], [], [P("msvc-linker", False, None, "none"), P("rust", True, "D:\\cargo\\bin\\rustc.exe", "cargo-home")]),
]
json.dump({"scenarios": scenarios}, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(out, "a").write("\n")
print(len(scenarios))
