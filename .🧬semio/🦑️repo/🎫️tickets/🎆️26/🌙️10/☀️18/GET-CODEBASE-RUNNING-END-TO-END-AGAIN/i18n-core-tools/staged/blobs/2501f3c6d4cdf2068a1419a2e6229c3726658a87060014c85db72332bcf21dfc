/** ⚠️ Plugin-crate and target scope for `verify rust-warnings`.
 * @see 📜️script.ts
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GlobV1 } from "../../../../../../../🔨️modules/⚙️runtime/🌐️glob/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../🔨️modules/⚙️runtime/📄️parse/🟦️.ts";
import {refuseVerifyV1} from "../📖️texts/🟦️.ts";

/** 🦀️ Shipping plugins, extensions, and their artifact libraries from the repository's package catalog. */
function pluginCrateNames(root: string, includeSupport = false): string[] {
  const workspace = (runtimeParseTomlV1(readFileSync(join(root, "Cargo.toml"), "utf8")) as { workspace: { members: string[]; exclude?: string[] } }).workspace;
  const excluded = (workspace.exclude ?? []).map((pattern) => new GlobV1(pattern));
  const manifests = workspace.members.flatMap((member) => {
    const paths = [...new GlobV1(`${member}/Cargo.toml`).scanSync({ cwd: root, onlyFiles: true })];
    if (!paths.length) throw refuseVerifyV1("missing","rust-warnings.member-manifest-missing-for",{member:String(member)});
    return paths.filter((path) => !excluded.some((pattern) => pattern.match(path.slice(0, -"/Cargo.toml".length))));
  });
  const names = manifests.flatMap((path) => {
    const manifest = runtimeParseTomlV1(readFileSync(join(root, path), "utf8"));
    const pkg = (manifest as { package?: { name?: string; metadata?: { semio?: { role?: string } } } }).package;
    const name = pkg?.name;
    if (!name) throw refuseVerifyV1("missing","rust-warnings.package-name-missing-in",{path:String(path)});
    return ["plugin", "extension"].includes(pkg?.metadata?.semio?.role ?? "") || name.startsWith("semio-s-artifact-") || name.startsWith("semio-framework-artifact-") || (includeSupport && name.startsWith("semio-s-plugin-")) ? [name] : [];
  });
  return [...new Set(names)].sort();
}

/** 🎯️ Native test surfaces, WASI component libraries, and browser actor/renderer libraries. */
export function rustWarningTargetScope(root: string, target: string | undefined): { packages: string[]; scopeArgs: string[]; targetArgs: string[]; packageArgs: Record<string, string[]> } {
  const renderer = "semio-framework-os-renderer-wgpu";
  if (target === "wasm32-wasip2") return { packages: pluginCrateNames(root), scopeArgs: ["--lib"], targetArgs: ["--target", target], packageArgs: {} };
  if (target === "wasm32-unknown-unknown") return { packages: ["semio-framework-actor", renderer], scopeArgs: ["--lib"], targetArgs: ["--target", target], packageArgs: {} };
  if (target && target !== "native") throw refuseVerifyV1("invalid","rust-warnings.unknown-target",{target:String(target)});
  return { packages: ["semio-framework-actor", "semio-framework", "semio-framework-os-kernel", renderer, ...pluginCrateNames(root, true)], scopeArgs: ["--all-targets"], targetArgs: [], packageArgs: { [renderer]: ["--features", "native-bin"], "semio-framework-os-kernel": ["--features", "native-bin"] } };
}
