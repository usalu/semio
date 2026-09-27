"""🔧️ F3 — callers of the asynchronous freshness walk: serve-start pass, Vite activation plugin, activation, staging-root laws.
Idempotent; --dry-run reports only."""
import sys, re
DRY = "--dry-run" in sys.argv
DEV = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev"
changed = {}

def edit(path, pairs):
    t = open(path).read(); o = t
    for label, old, new in pairs:
        if old not in t and new in t: print(f"skip (applied) {label}"); continue
        n = t.count(old)
        if n != 1: raise SystemExit(f"{path.split('/')[-2]} anchor {label}: {n} matches")
        t = t.replace(old, new); print(f"ok {label}")
    if t != o: changed[path] = t

# 1 freshness module
F = f"{DEV}/♻️activation/🔍️freshness/🟦️.ts"
ft = open(F).read()
s = ft.find("/** @emoji 🔎️ Collects one variant's staged-module freshness facts out of the ONE staging root:")
e = ft.find("export async function collectStagedModuleFactsAsync(")
pairs = []
if s >= 0 and e > s:
    doc_end = ft.find("export function collectStagedModuleFacts(options", s)
    doc = ft[s:doc_end]
    new_doc = doc.replace("Read-only — it never builds,\n * never writes, and never blocks a serve;", "Read-only and asynchronous — it never builds,\n * never writes, and never blocks a serve (bounded: {@link SOURCE_FRESHNESS_COMPONENT_CONCURRENCY} components at once, `signal` cancels);")
    pairs.append(("drop sync collect", ft[s:e], new_doc))
pairs += [
    ("rename async collect", '''export async function collectStagedModuleFactsAsync(options: {
  readonly moduleRoot: string;
  readonly installRoot: string;
  readonly receipt?: { readonly plugins: readonly { readonly pluginId: string; readonly artifactSha256: string; readonly sourceContentSha256?: string }[] };
  readonly components: readonly PluginRegistryEntry[];
}): Promise<readonly StagedModuleFacts[]> {''', '''export async function collectStagedModuleFacts(options: {
  readonly moduleRoot: string;
  readonly installRoot: string;
  readonly receipt?: { readonly plugins: readonly { readonly pluginId: string; readonly artifactSha256: string; readonly sourceContentSha256?: string }[] };
  readonly components: readonly PluginRegistryEntry[];
  readonly signal?: AbortSignal;
}): Promise<readonly StagedModuleFacts[]> {'''),
    ("bounded components", '''  return Promise.all(options.components.map(async (target): Promise<StagedModuleFacts> => {''', '''  return mapBoundedV1(options.components, SOURCE_FRESHNESS_COMPONENT_CONCURRENCY, async (target): Promise<StagedModuleFacts> => {'''),
    ("async resolve call", '''    const hashes = await resolveBootSourceContentHashesAsync({
      sourceRoot,
      moduleDirectory,
      receiptSourceContentSha256: receiptSource,
    });''', '''    const hashes = await resolveBootSourceContentHashes({
      sourceRoot,
      moduleDirectory,
      receiptSourceContentSha256: receiptSource,
      signal: options.signal,
    });'''),
    ("close bounded map", '''      installedPackageHash,
    };
  }));
}''', '''      installedPackageHash,
    };
  }, options.signal);
}'''),
    ("serve pass call", '''    reportStagedModuleFreshness(variant, renderer, profile, await collectStagedModuleFactsAsync({''', '''    reportStagedModuleFreshness(variant, renderer, profile, await collectStagedModuleFacts({'''),
    ("import", "resolveBootSourceContentHashes, resolveBootSourceContentHashesAsync, stagedModuleMtime,", "resolveBootSourceContentHashes, SOURCE_FRESHNESS_COMPONENT_CONCURRENCY, mapBoundedV1, stagedModuleMtime,"),
]
edit(F, pairs)

# 2 activation execution
A = f"{DEV}/♻️activation/🏃️execution/🟦️.ts"
edit(A, [("await write", "        const sourceContentSha256 = writeStagedSourceFreshness(moduleDirectory, sourceRoot);", "        const sourceContentSha256 = await writeStagedSourceFreshness(moduleDirectory, sourceRoot, controller.signal);")])

# 3 vite plugin
V = f"{DEV}/🔌️vite-plugins/🟦️.ts"
edit(V, [
    ("report doc+sig", '''/** @emoji 🔎️ Re-runs the staged-module freshness rule against the receipt the dev server just observed and
 * prints one `[stale]` line per component whose served bytes are behind — the live half of the serve-start
 * pass in `📜️script.ts`. A restage that lands while the server runs therefore retires its own warning
 * without a restart, and one that never lands keeps saying so. */
export function reportActivationFreshness(receipt: ActivationReceipt, options: { readonly moduleRoot: string; readonly installRoot: string; readonly components: readonly ActivationComponentSpec[] }): readonly string[] {
  const activatedRows = new Map(receipt.plugins.map((row) => [row.pluginId, row]));
  const facts = options.components.map((component): StagedModuleFacts => {''', '''/** @emoji 🔎️ Re-runs the staged-module freshness rule against the receipt the dev server just observed and
 * resolves to one `[stale]` line per component whose served bytes are behind — the live half of the serve-start
 * pass in `📜️script.ts`. A restage that lands while the server runs therefore retires its own warning
 * without a restart, and one that never lands keeps saying so. Asynchronous and bounded (the server keeps answering
 * while it walks); `signal` cancels a pass a newer receipt superseded. */
export async function reportActivationFreshness(receipt: ActivationReceipt, options: { readonly moduleRoot: string; readonly installRoot: string; readonly components: readonly ActivationComponentSpec[]; readonly signal?: AbortSignal }): Promise<readonly string[]> {
  const activatedRows = new Map(receipt.plugins.map((row) => [row.pluginId, row]));
  const facts = await mapBoundedV1(options.components, SOURCE_FRESHNESS_COMPONENT_CONCURRENCY, async (component): Promise<StagedModuleFacts> => {'''),
    ("resolve await", '''    const hashes = resolveBootSourceContentHashes({
      sourceRoot: component.sourceRoot,
      moduleDirectory,
      receiptSourceContentSha256: (receiptRow as { sourceContentSha256?: string } | undefined)?.sourceContentSha256,
    });''', '''    const hashes = await resolveBootSourceContentHashes({
      sourceRoot: component.sourceRoot,
      moduleDirectory,
      receiptSourceContentSha256: (receiptRow as { sourceContentSha256?: string } | undefined)?.sourceContentSha256,
      signal: options.signal,
    });'''),
    ("close map", '''      installedPackageHash,
    }
  });
  return stagedModuleReportLines(facts.map(stagedModuleVerdict),''', '''      installedPackageHash,
    }
  }, options.signal);
  return stagedModuleReportLines(facts.map(stagedModuleVerdict),'''),
    ("observer apply", '''      const observer = observeActivationReceipts(options.receiptDirectory, (receipt) => {
        const apply = (): void => {
          staleness = reportActivationFreshness(receipt, options);
          for (const line of staleness) console.warn(line);
          if (previous) {''', '''      let freshness: AbortController | null = null;
      const observer = observeActivationReceipts(options.receiptDirectory, (receipt) => {
        const apply = (): void => {
          freshness?.abort();
          const pass = (freshness = new AbortController());
          void reportActivationFreshness(receipt, { ...options, signal: pass.signal }).then((lines) => {
            if (pass.signal.aborted) return;
            staleness = lines;
            for (const line of lines) console.warn(line);
          }, (error: unknown) => {
            if (!pass.signal.aborted) console.warn(`[stale] freshness check unavailable: ${String(error)}`);
          });
          if (previous) {'''),
    ("dispose abort", '''      dispose = (): void => {
        observer.close();
        unrouteWatch();
      };''', '''      dispose = (): void => {
        freshness?.abort();
        observer.close();
        unrouteWatch();
      };'''),
    ("publishOne await", "        const sourceContentSha256 = writeStagedSourceFreshness(moduleDirectory, component.sourceRoot);", "        const sourceContentSha256 = await writeStagedSourceFreshness(moduleDirectory, component.sourceRoot);"),
    ("import", "publishActivationReceipt, readActivationReceipt, resolveBootSourceContentHashes, stagedModuleMtime,", "publishActivationReceipt, readActivationReceipt, resolveBootSourceContentHashes, SOURCE_FRESHNESS_COMPONENT_CONCURRENCY, mapBoundedV1, stagedModuleMtime,"),
])

# 4 staging-root laws: await the async functions
T = f"{DEV}/🧪️tests/🔌️staging-root/🟦️.ts"
tt = open(T).read(); to = tt
tt = re.sub(r'(?<!await )componentSourceContentHash\(', 'await componentSourceContentHash(', tt)
tt = tt.replace("import {\n  COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES,\n  GENERATED_COMPONENT_OWNER_FILES,\n  UNWATCHED_COMPONENT_SOURCE_DIRECTORIES,\n  healthyPreparedComponents,\n  await componentSourceContentHash,", "import {\n  COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES,\n  GENERATED_COMPONENT_OWNER_FILES,\n  UNWATCHED_COMPONENT_SOURCE_DIRECTORIES,\n  healthyPreparedComponents,\n  componentSourceContentHash,")
tt = re.sub(r'(?<!await )resolveBootSourceContentHashes\(\{', 'await resolveBootSourceContentHashes({', tt)
tt = re.sub(r'(?<!await )writeStagedSourceFreshness\(stagedDirectory', 'await writeStagedSourceFreshness(stagedDirectory', tt)
for name in ["boot freshness: unchanged tree reuses every file digest and stays fresh", "boot freshness: touched-but-identical file rehashes one entry and stays fresh", "boot freshness: edited file is detected as source-changed", "boot freshness: added or removed file is detected as source-changed", "hashes plugin-owner sources stably and ignores declared output directories"]:
    tt = tt.replace(f'it("{name}", () => {{', f'it("{name}", async () => {{')
# the newest/verdict test that calls componentSourceContentHash
idx = tt.find("const before = await componentSourceContentHash(sourceRoot);")
if idx >= 0:
    head = tt.rfind('it("', 0, idx)
    line_end = tt.find("\n", head)
    line = tt[head:line_end]
    if "async () =>" not in line: tt = tt[:head] + line.replace("() => {", "async () => {", 1) + tt[line_end:]
if tt != to: changed[T] = tt; print("ok staging-root awaits")
else: print("skip (applied) staging-root awaits")

if DRY:
    print("dry-run:", len(changed), "files would change")
else:
    for p, t in changed.items(): open(p, "w").write(t)
    print("written", len(changed))
