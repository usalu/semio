import { existsSync } from "node:fs";
import { join } from "node:path";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const configured = join(root, ".venv", process.platform === "win32" ? "Scripts/python.exe" : "bin/python");
const python = existsSync(configured) ? configured : Bun.which("python3") ?? Bun.which("python");
if (!python) throw new Error("Python runtime unavailable");
const program = `import importlib.util,sys
p=sys.argv[1]
s=importlib.util.spec_from_file_location("fixture_case",p)
m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
m.stage0_snapshot_present()
m.stage1_data_layer()
m.stage2_model_parity()
print("[DEBUG] Netz moved-fixture consumers: "+str(len(m.FAILS))+" failures")
raise SystemExit(bool(m.FAILS))`;
const child = Bun.spawn([python, "-B", "-c", program, join(root, "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🐍️.py")], { cwd: root, stdout: "inherit", stderr: "inherit" });
process.exitCode = await child.exited;
