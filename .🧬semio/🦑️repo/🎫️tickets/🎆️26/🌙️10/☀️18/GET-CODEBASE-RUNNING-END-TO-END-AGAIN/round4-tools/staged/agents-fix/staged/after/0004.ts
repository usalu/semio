/** 🛂️ Package descriptor admission: the bounded read of a served plugin descriptor, its owner and app-roster checks, the package receipt identity and the package assets it declares.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import {
  readDescriptorText,
  reportDescriptorProgress,
  PLUGIN_DESCRIPTOR_TEXT_POLICY,
  DescriptorTextLimitError,
  type DescriptorProgress
} from "../📇️descriptor/📥️text/🟦️.ts";
import { readPackageReceiptIdentity, type PackageReceiptIdentity, type PackageReceiptSource } from "../📇️descriptor/📦️package/🟦️.ts";
import type { PluginManifest } from "../../🛂️manifest/🟦️.ts";
import { readPackageAssetDeclarations, type PackageAssetDeclaration } from "../📦️package-assets/🟦️.ts";
import { SemioFaultError } from "../🚨️fault/🟦️.ts";

// #region 🎠️Kernel
//#region 📇️DescriptorAdmission
/** 📇️ Requires a published descriptor with the requested owner before any actor runtime is started. */
/** 📏️ Admission ceiling for one plugin's `🔣️.json`, measured on the response text the parse
 * already needs — so the bound costs nothing, where re-serializing the parsed manifest to measure it cost a
 * full 5.4 MB `JSON.stringify` per plugin per boot and blew the frame budget on `puzzle` alone. */
export const PLUGIN_DESCRIPTOR_CODE_UNIT_CAPACITY = PLUGIN_DESCRIPTOR_TEXT_POLICY.maximumCodeUnits;

/** 🧭️ Attributes package admission failures to their General owner. */
function descriptorFault(pluginId: string, code: string, detail: string): SemioFaultError {
  return new SemioFaultError({ origin: "framework", code, severity: "error", message: `${code}: ${detail}`, scope: { pluginId }, retryable: true });
}

/** 🪪️ The served module's descriptor identity and the digest its explicit receipt source admits. */
export interface PluginPackageDescriptor {
  readonly manifest: PluginManifest;
  readonly packageId: string;
  readonly componentSha256: string;
  /** 📦️ The package assets the descriptor declares (name, exact size, SHA-256) for the kernel to deliver on `instance-open`. */
  readonly assets?: readonly PackageAssetDeclaration[];
}

/** 🪪️ Binds {@link fetchDescriptorManifest} to a required receipt source naming the same plugin and package. */
export async function fetchPackageDescriptor(pluginId: string, moduleUrl: string, receiptSource: PackageReceiptSource, signal?: AbortSignal, onProgress: DescriptorProgress = () => {}): Promise<PluginPackageDescriptor> {
  signal?.throwIfAborted();
  if (!receiptSource || typeof receiptSource.read !== "function") throw Error("Explicit package receipt source required");
  const [{ descriptor, manifest }, identity] = await Promise.all([fetchDescriptorDocument(pluginId, moduleUrl, signal, onProgress), receiptSource.read({ pluginId, moduleUrl, signal, onProgress })]);
  signal?.throwIfAborted();
  let receipt: PackageReceiptIdentity;
  try { receipt = readPackageReceiptIdentity(identity); }
  catch { signal?.throwIfAborted(); throw descriptorFault(pluginId, "plugin.descriptor-invalid", "invalid package receipt identity"); }
  const packageId = "packageId" in descriptor && typeof descriptor.packageId === "string" ? descriptor.packageId : "";
  if (packageId.length === 0 || "hashes" in descriptor || receipt.pluginId !== pluginId || receipt.packageId !== packageId) throw descriptorFault(pluginId, "plugin.descriptor-invalid", "missing package identity");
  let assets: readonly PackageAssetDeclaration[];
  try { assets = readPackageAssetDeclarations(descriptor); }
  catch { throw descriptorFault(pluginId, "plugin.descriptor-invalid", "malformed package asset declaration"); }
  return { manifest, packageId, componentSha256: receipt.componentSha256, ...(assets.length === 0 ? {} : { assets }) };
}

/** 📥️ `onProgress` is called on the response headers and on every streamed chunk — see
 * {@link readDescriptorText} for why this fetch owes its caller a heartbeat at all. */
export async function fetchDescriptorManifest(pluginId: string, moduleUrl: string, signal?: AbortSignal, onProgress: DescriptorProgress = () => {}): Promise<PluginManifest> {
  const { descriptor, manifest } = await fetchDescriptorDocument(pluginId, moduleUrl, signal, onProgress);
  let assets: readonly PackageAssetDeclaration[];
  try { assets = readPackageAssetDeclarations(descriptor); }
  catch { throw descriptorFault(pluginId, "plugin.descriptor-invalid", "malformed package asset declaration"); }
  if (assets.length > 0) manifestPackageAssets.set(manifest, assets);
  return manifest;
}

const manifestPackageAssets = new WeakMap<PluginManifest, readonly PackageAssetDeclaration[]>();

/** 📦️ The package assets the descriptor behind `manifest` declared when {@link fetchDescriptorManifest} read it, so a loader that
 * registers the manifest with the {@link ActivationRegistry} can pass them on for delivery with `instance-open`; none for a manifest
 * that declared none or was not read by it. */
export function descriptorPackageAssets(manifest: PluginManifest): readonly PackageAssetDeclaration[] {
  return manifestPackageAssets.get(manifest) ?? [];
}

/** 📇️ The JSON descriptor a served plugin module carries beside its entry, read before any actor starts. */
export function pluginDescriptorUrl(moduleUrl: string): string {
  const path = moduleUrl.split(/[?#]/u)[0]!;
  return path.slice(0, path.lastIndexOf("/") + 1) + "🔣️.json";
}

/** 📇️ Reads and admits one served package descriptor: owner, app roster, bounded size. */
/** 📥️ Fetches and admits one descriptor document: a JSON object whose manifest names the requested owner and an app roster, and whose example rows all name the dialect they were authored for. */
async function fetchDescriptorDocument(pluginId: string, moduleUrl: string, signal: AbortSignal | undefined, onProgress: DescriptorProgress): Promise<{ readonly descriptor: object; readonly manifest: PluginManifest }> {
  signal?.throwIfAborted();
  const descriptorUrl = pluginDescriptorUrl(moduleUrl);
  const fault = (code: string, detail: string) => descriptorFault(pluginId, code, detail);
  const response = await fetch(descriptorUrl, signal ? { signal } : undefined);
  try {
    await reportDescriptorProgress(onProgress, signal);
    signal?.throwIfAborted();
    if (!response.ok) throw fault("plugin.descriptor-unavailable", `${descriptorUrl} (HTTP ${response.status})`);
    if (response.headers?.get?.("content-type")?.toLowerCase().includes("text/html")) throw fault("plugin.descriptor-invalid", `${descriptorUrl} returned HTML`);
    let descriptorText: string;
    try { descriptorText = await readDescriptorText(response, onProgress, PLUGIN_DESCRIPTOR_TEXT_POLICY, signal); }
    catch (error) {
      signal?.throwIfAborted();
      if (error instanceof DescriptorTextLimitError) throw fault("plugin.descriptor-oversized", `${descriptorUrl} exceeds its response limit`);
      throw error;
    }
    signal?.throwIfAborted();
    let descriptor: unknown;
    try { descriptor = JSON.parse(descriptorText); }
    catch {
      signal?.throwIfAborted();
      throw fault("plugin.descriptor-invalid", `${descriptorUrl} is not JSON`);
    }
    signal?.throwIfAborted();
    const manifest = descriptor && typeof descriptor === "object" && "manifest" in descriptor ? descriptor.manifest : undefined;
    if (!manifest || typeof manifest !== "object" || !("pluginId" in manifest) || typeof manifest.pluginId !== "string") throw fault("plugin.descriptor-invalid", "missing manifest owner");
    if (manifest.pluginId !== pluginId) throw fault("plugin.descriptor-identity-mismatch", `expected ${pluginId}, received ${manifest.pluginId}`);
    if (!("apps" in manifest) || !Array.isArray(manifest.apps)) throw fault("plugin.descriptor-invalid", "missing app roster");
    if ("examples" in manifest && Array.isArray(manifest.examples)) for (const row of manifest.examples) if (!row?.dialect) throw fault("plugin.descriptor-invalid", `example ${String(row?.id ?? "?")} misses its dialect`);
    return { descriptor: descriptor as object, manifest: manifest as PluginManifest };
  } catch (error) {
    await response.body?.cancel(error).catch(() => {});
    signal?.throwIfAborted();
    throw error;
  }
}
//#endregion 📇️DescriptorAdmission
// #endregion 🎠️Kernel
