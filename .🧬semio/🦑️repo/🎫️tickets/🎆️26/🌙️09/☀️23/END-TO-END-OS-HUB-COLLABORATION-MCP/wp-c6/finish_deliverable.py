#!/usr/bin/env python3
from __future__ import annotations
import subprocess, sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
TICKET = next(REPO.joinpath(".🧬semio").rglob("END-TO-END-OS-HUB-COLLABORATION-MCP"))
OUT = TICKET / "🗑️generated" / "wp-c6"
OUT.mkdir(parents=True, exist_ok=True)
(TICKET / "wp-c6").mkdir(exist_ok=True)

fw = next(p for p in REPO.iterdir() if p.name.endswith("framework"))
osroot = fw / "🛍️products" / next(c.name for c in (fw / "🛍️products").iterdir() if "os" in c.name)
ots = osroot / "🟦️.ts"
text = ots.read_text(encoding="utf-8")
for name in ("ephemeralLocalPersistenceBinding", "ephemeralSharedPersistenceBinding"):
    if name not in text:
        raise SystemExit(f"missing helper {name}")

start = text.index("//#region PersistenceDataClassTests")
end = text.index("//#endregion PersistenceDataClassTests") + len("//#endregion PersistenceDataClassTests")
new = """//#region PersistenceDataClassTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("persistence data class routing", () => {
    it("classifies folder, hub, empty bindings, and preview/presence lanes", () => {
      expect(folderPersistenceBinding("/tmp/x").dataClass).toBe("persistedLocalOnly");
      expect(hubPersistenceBinding("http://hub.test", "space-1").dataClass).toBe("persistedShared");
      expect(bindingsDataClass([])).toBe("ephemeralLocalOnly");
      expect(wireLaneDataClass("preview")).toBe("ephemeralShared");
      expect(wireLaneDataClass("presence")).toBe("ephemeralShared");
      expect(ephemeralLocalPersistenceBinding().dataClass).toBe("ephemeralLocalOnly");
      expect(ephemeralSharedPersistenceBinding("preview").dataClass).toBe("ephemeralShared");
    });
  });
}
//#endregion PersistenceDataClassTests"""
if text[start:end] != new:
    ots.write_text(text[:start] + new + text[end:], encoding="utf-8")
    print("patched os.ts")
else:
    print("os.ts already patched")

td = next(c for c in osroot.iterdir() if "test" in c.name.lower())
pdc = td / "persistence-data-class" / "🟦️.ts"
cfg_path = TICKET / "wp-c6" / "vitest.persistence.config.ts"
cfg_path.write_text(
    "import { defineConfig } from \"vitest/config\";\n"
    "import { resolve } from \"node:path\";\n\n"
    f"const os = {osroot.as_posix()!r};\n"
    "const pkg = resolve(os, \"📦️packages/🟦️typescript\");\n\n"
    "export default defineConfig({\n"
    "  root: pkg,\n"
    "  resolve: { alias: { \"@semio-tech/framework-os\": resolve(os, \"🟦️.ts\") } },\n"
    "  test: {\n"
    "    root: pkg,\n"
    "    name: \"@semio-tech/framework-os-persistence-data-class\",\n"
    "    environment: \"node\",\n"
    f"    include: [{pdc.as_posix()!r}],\n"
    "    includeSource: [],\n"
    "    passWithNoTests: false,\n"
    "    testTimeout: 60000,\n"
    "  },\n"
    "});\n",
    encoding="utf-8",
)
print("wrote", cfg_path)

pkg = osroot / "📦️packages" / "🟦️typescript"
log = OUT / "ts-dedicated.log"
proc = subprocess.run(
    ["bunx", "vitest", "run", "--config", str(cfg_path)],
    cwd=str(pkg),
    capture_output=True,
    text=True,
)
log.write_text(proc.stdout + "\n" + proc.stderr, encoding="utf-8")
print("vitest exit", proc.returncode)
print((proc.stdout + proc.stderr)[-1500:])
sys.exit(proc.returncode)
