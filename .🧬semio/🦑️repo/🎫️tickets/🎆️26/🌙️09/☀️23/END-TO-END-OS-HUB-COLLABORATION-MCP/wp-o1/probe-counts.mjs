
import { readdirSync, readFileSync, mkdtempSync, copyFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
const dep = await import('file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%93%87%EF%B8%8Fregistry/%F0%9F%93%A6%EF%B8%8Fdeployment/%F0%9F%9F%A6%EF%B8%8F.ts');
const act = await import('file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%A7%91%E2%80%8D%F0%9F%92%BBdev/%E2%99%BB%EF%B8%8Factivation/%F0%9F%9F%A6%EF%B8%8F.ts');
const inst = await import('file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%8F%AA%EF%B8%8Fstore/%F0%9F%93%A5%EF%B8%8Finstallation/%F0%9F%9F%A6%EF%B8%8F.ts');
const mat = await import('file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%8C%90%EF%B8%8Fbrowser-bundle/%F0%9F%8F%97%EF%B8%8Fmaterialization/%F0%9F%9F%A6%EF%B8%8F.ts');
const { moduleIdForDirectoryName } = dep;
const pluginOutRoot = act.pluginModulesRoot("dev");
const repoRoot = "/Users/ueli/Documents/semio";
const extRoot = inst.defaultExtensionInstallRoot(repoRoot);
function census(root) {
  let components = 0, imports = 0;
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory() || !moduleIdForDirectoryName(entry.name)) continue;
    for (const filename of readdirSync(join(root, entry.name)).filter((n) => n.endsWith("_component.js"))) {
      components++;
      const source = readFileSync(join(root, entry.name, filename), "utf8");
      for (const line of source.split("\n")) {
        if (line.includes("import") && line.includes("preview2-shim")) imports++;
      }
    }
  }
  return { components, imports };
}
const tmp = mkdtempSync(join(tmpdir(), "semio-cli-"));
const expectedCli = join(tmp, "cli.js");
copyFileSync(join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser/cli.js"), expectedCli);
mat.patchPreview2ShimGuestLogClassification(expectedCli);
mat.patchPreview2ShimGuestLogLineRelease(expectedCli);
const stagedCli = join(pluginOutRoot, '🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim', "cli.js");
const match = readFileSync(stagedCli).equals(readFileSync(expectedCli));
console.log(JSON.stringify({ pluginOutRoot, extRoot, plugin: census(pluginOutRoot), ext: census(extRoot), cliPatchedMatch: match }, null, 2));
rmSync(tmp, { recursive: true, force: true });
