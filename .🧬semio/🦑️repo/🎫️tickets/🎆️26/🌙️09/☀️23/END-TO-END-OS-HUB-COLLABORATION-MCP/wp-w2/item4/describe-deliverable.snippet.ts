/** @emoji 🛂️ The ONE describe route of every plugin and extension component (the inferred Nx `describe` target, which
 * `dependsOn` `component-dev`): reads the exact bytes `component-dev` staged at `<crate>/dist/component-dev/<crate>.wasm`,
 * extracts its core with jco and re-emits `🛂️.descriptor.semio` + `🔣️.json` at the owner root (`<owner>/📦️packages/🦀️rust`
 * is the crate). No build happens here, so the committed descriptor, the `materialize-dev` staging (which reads the same
 * deliverable) and every consumer that resolves `dist/component-dev` describe one build by construction. */
export function describeComponentDeliverable(repoRoot: string, manifest: string, control: DescriptorEmissionControlV1 = {}): number {
  const manifestPath = resolve(repoRoot, manifest);
  try {
    const cargo = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(manifestPath, "utf8")) as { package?: { name?: string; metadata?: { component?: { package?: string }; semio?: { role?: string } } } };
    const packageName = cargo.package?.name;
    if (!packageName || !cargo.package?.metadata?.component?.package || !["plugin", "extension"].includes(cargo.package.metadata.semio?.role ?? "")) throw new Error(`${manifest} is not a plugin or extension component manifest`);
    const crateRoot = dirname(manifestPath);
    const deliverableRoot = join(crateRoot, "dist");
    const component = join(deliverableRoot, "component-dev", `${packageName.replace(/-/g, "_")}.wasm`);
    if (!existsSync(component)) throw new Error(`no component-dev deliverable at ${relative(repoRoot, component)}; describe runs through Nx, which builds component-dev first`);
    const scratch = mkdtempSync(join(deliverableRoot, ".semio-describe-core-"));
    try {
      const core = extractPluginCore(repoRoot, component, scratch, packageName.replace(/-/g, "_"));
      const receipt = emitOwnerDescriptorPairV1(repoRoot, { rawComponentPath: component, extractedCorePath: core, ownerRoot: resolve(crateRoot, "..", ".."), artifactRoot: deliverableRoot }, control);
      console.log(`described ${receipt.pluginId} (${receipt.role} ${receipt.packageId}@${receipt.version}) from ${relative(repoRoot, component)} -> ${relative(repoRoot, receipt.ownerRoot)} (wasm=${receipt.rawSha256} core=${receipt.coreSha256} descriptor=${receipt.descriptorSha256})`);
      return 0;
    } finally {
      rmSync(scratch, { recursive: true, force: true });
    }
  } catch (error) {
    console.error(`describe ${manifest} failed: ${(error as Error).message}`);
    return 1;
  }
}
